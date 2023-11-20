# Trying Bevy (Rust game engine)


```bash
cargo add bevy
```


## Optimization

Install lld - a faster linker

```bash
sudo pacman -S lld
```


Add to files:

*Cargo.toml*
```toml
[dependencies]
bevy = { version = "0.12.0", features = ["dynamic_linking"] }

# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

*rust-toolchain.toml*
```toml
[toolchain]
channel = "nightly"
```

Run
```bash
cargo run
```

It will take a while, since it's compiling a whole game engine.  
Subsequent compiles will be iterative and faster.

