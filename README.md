# Trying Bevy (Rust Game Engine)

Code following [Bevy Book](https://bevyengine.org/learn/book/getting-started/)


## Add Bevy

Add `bevy` as a dependency

```bash
cargo add bevy
```


## Optimization

Install a faster linker and add configuration files for improving speed.


### Install lld

Install `lld` - a faster linker

```bash
sudo pacman -S lld
```


### Add to files

**Cargo.toml**
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

**rust-toolchain.toml**
```toml
[toolchain]
channel = "nightly"
```

**config.toml**
```toml
# NOTE: For maximum performance, build using a nightly compiler
# If you are using rust stable, remove the "-Zshare-generics=y" below.

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]

[target.x86_64-apple-darwin]
rustflags = [
    "-C",
    "link-arg=-fuse-ld=/usr/local/opt/llvm/bin/ld64.lld",
    "-Zshare-generics=y",
]

[target.aarch64-apple-darwin]
rustflags = [
    "-C",
    "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld",
    "-Zshare-generics=y",
]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=n"]
```


## Running
Run
```bash
cargo run
```

It will take a while, since it's compiling a whole game engine.  
Subsequent compiles will be iterative and faster.
