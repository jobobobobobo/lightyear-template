An opinionated lightyear starter project.

Replace all instances of `mygame` in the names of folders and files with the name of your game. 

# Usage

```
cargo run server
cargo run client -c 1
```

# Crates

### client

### server

### assets
Preloads assets during a managed loading state. Allows for postprocessing loaded GLTFs. Example adds colliders to loaded GLTF.

### common 
Contains all shared gameplay logic between client and server.

### launcher
Non-bevy management code for configuring the client and server apps before running them. Supports native and wasm.

### protocol
Lightyear protocol code. Separate from `common` to save on compile time.

### render
Shared logic between client and headed server. Anything that the headless server can't run goes here.


## Configuration

Configuration can be modified in `crates/mygame-launcher/options` and extended in `crates/mygame-launcher/launch_options.rs`.

## Running the WASM client

```
trunk --config ./crates/mygame-launcher/Trunk.toml serve
```

Navigate to `127.0.0.1:8080?client_id=42`

## Certificate instructions for development

From the template root...

```
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -keyout ./mygame-launcher/web/certs/key.pem -out ./mygame-launcher/web/certs/cert.pem -days 14 -nodes -subj "/CN=localhost" -addext "subjectAltName=DNS:localhost,IP:127.0.0.1"
```

## Notes

- `HostServer` mode - client and server in the same `App` - is unsupported. When launching the client and server on the same machine, the server will be launched in its own `App` on a separate thread.
