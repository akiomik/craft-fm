craft-fm
========

[![CI](https://github.com/akiomik/craft-fm/actions/workflows/ci.yml/badge.svg)](https://github.com/akiomik/craft-fm/actions/workflows/ci.yml)

wasm + Web Audio API

## Requirements

- [`npm`](https://docs.npmjs.com/)
- [`cargo`](https://doc.rust-lang.org/stable/cargo/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- [`chromedriver`](https://chromedriver.chromium.org/downloads) (testing)

## Setup

```sh
npm install
```

## Launch local server

```sh
npm run serve
```

## Testing

```sh
cargo test
wasm-pack test --headless --chrome
```
