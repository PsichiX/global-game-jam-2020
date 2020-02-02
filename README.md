# global-game-jam-2020
Global Game Jam 2020

## Installation
1. Make sure that you have latest `node.js` with `npm` tools installed (https://nodejs.org/)
1. Make sure that you have latest `wasm-pack` toolset installed (https://rustwasm.github.io/wasm-pack/installer/)
1. Make sure that you have latest `oxygengine-ignite` application installed (from project repository releases)
1. go to `ggj20-game` and type `npm install` to install all NodeJS dependencies

## Building for development and production
- Launch live development with hot reloading (app will be automatically
  recompiled in background):
```bash
npm start
```
- Build production distribution (will be available in `/dist` folder):
  with debug symbols:
  ```bash
  npm run build
  ```
  optimized release mode:
  ```bash
  OXY_RELEASE=1 npm run build
  ```
- Build crate without of running dev env:
```bash
cargo build
```

## Asset licences
Music from http://ncs.io/FearlessID
- "Fearless" by TULE

Music from https://filmmusic.io
- "Fearless First" by Kevin MacLeod (https://incompetech.com)
- "Surf Shimmy" by Kevin MacLeod (https://incompetech.com)
- "District Four" by Kevin MacLeod (https://incompetech.com)
- "Werq" by Kevin MacLeod (https://incompetech.com)
License: CC BY (http://creativecommons.org/licenses/by/4.0/)
