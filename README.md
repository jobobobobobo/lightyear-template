An opinionated lightyear starter project.

Replace all instances of `mygame` in the names of folders and files with the name of your game. 

# Usage

```
cargo run server
cargo run client -c 1
```

# Crates

```
client
server

assets  - Preloads assets during a managed loading state. Allows for postprocessing loaded GLTFs, example adds colliders to loaded GLTF.
common  - All shared gameplay logic between client and server
launcher - Non-bevy management code for configuring the client and server apps before running them. Supports native and wasm.
protocol - Lightyear protocol
render - Shared logic between client and headed server. Anything that the headless server can't run goes here.
```

# Configuration

Configuration can be modified in `crates/mygame-launcher/options` and extended in `crates/mygame-launcher/launch_options.rs`.


# Notes

- `HostServer` mode - client and server in the same `App` - is unsupported. When launching the client and server on the same machine, the server will be launched in its own `App` on a separate thread.
