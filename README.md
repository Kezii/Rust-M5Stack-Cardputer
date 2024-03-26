# Rust on M5Stack Cardputer
<a href="https://github.com/Kezii/Rust-M5Stack-Cardputer/actions"><img alt="actions" src="https://github.com/Kezii/Rust-M5Stack-Cardputer/actions/workflows/rust.yml/badge.svg"></a>

### Implemented drivers
- [x] display
- [x] keyboard
- [x] speaker (needs testing, #2)
- [ ] sd card
- [ ] battery
- [ ] microphone
- [ ] rgb led

### Utilities
- [x] double buffering
- [x] keyboard input events (typing, shift/fn etc)
- [ ] sd card filesystem ?
- [ ] app loader ?

# Getting started

First, install cargo or rustup using your (Linux) package manager, or from the official website

https://doc.rust-lang.org/cargo/getting-started/installation.html

Then, install the toolchain for esp32, following their official book

https://esp-rs.github.io/book/installation/riscv-and-xtensa.html

# Examples

## 3d graphics demo
https://github.com/Kezii/Rust-M5Stack-Cardputer/assets/3357750/658bd303-03c5-4dc2-9c49-75a98b667719


Interactive 3d graphics demo based on [embedded-gfx](https://github.com/Kezii/embedded-gfx)

Use (orange) arrow keys to move, EASD to look around

```
cargo run --release --bin graphics
```

## Terminal emulator with rink-core

![terminal](https://github.com/Kezii/Rust-M5Stack-Cardputer/assets/3357750/90585aa0-dfcb-4bc8-bd9d-3e5204a807f0)

Terminal emulator with rink-core built-in, with a reduced set of (gnu) units so it fits on the ram

You can use it as a simple units-aware calculator

```
cargo run --release --bin rink
```

## ESP-NOW remote keyboard

Sends key events over ESP-NOW, using the Cardputer as a remote keyboard

An ESP-NOW receiver with a wifi access point is required

```
cargo run --release --bin espnow_remote
```


# Credits
Upstream display driver

https://github.com/almindor/st7789
