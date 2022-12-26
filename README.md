# Quadtris

A Modern Tetris game built with Rust using [Macroquad](https://macroquad.rs/).

Features:

- [x] [SRS (Super Rotation System)](https://harddrop.com/wiki/SRS)
- [ ] T-spin and All Clear detection
- [ ] 40 Lines and Marathon mode
- [ ] Sound effects

Try online [here](https://quadtris.chocole.top)

# Build

Native build

```bash
cargo run --release
```

WebAssembly build

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown

# host locally with a web server
cp target/wasm32-unknown-unknown/release/quadtris.wasm web/
cd web
cp -r ../res .
npx serve .
```

# See also

- [NullpoMino](https://github.com/nullpomino/nullpomino) is an open-source action puzzle game that works on the Java platform. It has a wide variety of single-player modes and netplay to allow players to compete over the Internet or LAN.
