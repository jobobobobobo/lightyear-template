{
  description = "Cross compiling a rust program for windows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, crane, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        winToolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        winCraneLib = (crane.mkLib pkgs).overrideToolchain winToolchain;

        gnuToolchain = with fenix.packages.${system};
          combine [
            minimal.rustc
            minimal.cargo
            targets.x86_64-unknown-linux-gnu.latest.rust-std
          ];

        gnuCraneLib = (crane.mkLib pkgs).overrideToolchain gnuToolchain;

        client = winCraneLib.buildPackage {
          src = winCraneLib.cleanCargoSource ./.;

          strictDeps = true;
          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu";

          # fixes issues related to libring
          TARGET_CC =
            "${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/${pkgs.pkgsCross.mingwW64.stdenv.cc.targetPrefix}cc";

          #fixes issues related to openssl
          OPENSSL_DIR = "${pkgs.openssl.dev}";
          OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

          depsBuildBuild = with pkgs; [
            pkgsCross.mingwW64.stdenv.cc
            pkgsCross.mingwW64.windows.pthreads
          ];
        };

        server = gnuCraneLib.buildPackage {
          src = gnuCraneLib.cleanCargoSource ./.;

          strictDeps = true;
          doCheck = false;

          CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";

        };
      in {
        packages = {
          inherit client;
          inherit server;
          default = client;
        };

        devShells.default = gnuCraneLib.devShell {
          inputsFrom = [ client server ];
          packages = [ pkgs.rust-analyzer pkgs.rustfmt ];
        };

        checks = {
          inherit client;
          inherit server;
        };
      });
}
