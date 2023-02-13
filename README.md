# Quadtris

A Modern Tetris game built with Rust using [Macroquad](https://macroquad.rs/).

Try online [here](https://quadtris.chocole.top)

Features:

- [x] [SRS (Super Rotation System)](https://harddrop.com/wiki/SRS)
  - [x] 180 Rotation Wallkicks with [TETR.IO Wallkick tables](https://twitter.com/tetriogame/status/1271572187309375491)
- [x] [DAS, ARR](https://harddrop.com/wiki/ARR), Soft Drop Gravity and Keybind customization
- [ ] Visual effects
  - [ ] Global Lighting
  - [ ] Gamefield Frame
  - [ ] Different Materials for Moving/Freezing/Finalized Blocks
  - [ ] Clear effects
- [ ] Game Modes
  - [ ] 40 Lines Sprint
  - [ ] Marathon
- [x] Sound effects
- [ ] Gameplay Polishing
  - [ ] Guideline lock delay
  - [ ] T-spin and All Clear detection
  - [x] [ARE (spawn delay)](https://harddrop.com/wiki/ARE)
  - [x] [IRS](https://harddrop.com/wiki/IRS#IRS)
  - [ ] Line clear delay


[output.webm](https://user-images.githubusercontent.com/29040696/209563673-65858886-696c-4b2f-95ce-931978e69607.webm)

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
