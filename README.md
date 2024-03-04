# Rust on M5Stack Cardputer

### Implemented drivers
- [x] display
- [x] keyboard
- [ ] sd card
- [ ] battery
- [ ] speaker
- [ ] microphone
- [ ] rgb led

### Utilities
- [x] double buffering
- [x] keyboard input events (typing, shift/fn etc)
- [ ] sd card os ?
- [ ] app loader ?


## examples

### 3d graphics demo
![gfx](https://github.com/Kezii/Rust-M5Stack-Cardputer/assets/3357750/4ad54248-a363-4d34-b510-83186cdd3fb8)


https://github.com/Kezii/Rust-M5Stack-Cardputer/assets/3357750/ea274a64-5811-4846-b3f5-e94eeebd0635


Interactive 3d graphics demo
Use (orange) arrow keys to move, EASD to look around

```
cargo run --release --bin graphics
```

### Terminal emulator with rink-core

![terminal](https://github.com/Kezii/Rust-M5Stack-Cardputer/assets/3357750/90585aa0-dfcb-4bc8-bd9d-3e5204a807f0)

Terminal emulator with rink-core built-in, with a reduced set of (gnu) units so it fits on the ram
You can use it as a simple units-aware calculator

```
cargo run --release --bin terminal
```

# Credits
Upstream display driver

https://github.com/almindor/st7789
