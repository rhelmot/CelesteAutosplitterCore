Celeste Autosplitter
====================

This is an autosplitter for Celeste, built for projects that use the livesplit core wasm autosplitter format.
This is hardcoded right now for use with mono, and probably just for Linux as well.
It won't work with Everest without a lot of development, since Everest uses dotnet core and not mono.

In order to use it, compile it (instructions below) and point your timer at the autosplitter file. I use the livesplit OBS plugin.
Since it uses ptrace, you'll have to either configure your timer to launch Celeste (ideal) or run your timer as root.

Compiling
---------

- Install the rust compiler
- Install the rust compiler plugin for building wasm files
- `cargo build --release --target wasm32-unknown-unknown`
- The result file should be `target/release/celeste_autosplitter.wasm`
