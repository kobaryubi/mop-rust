# mop-rust
## Build
```
cargo build
```

## Run
```
cargo run
cargo run --bin <package-name>
    cargo run --bin mop-rust
    cargo run --bin audio_capturer
cargo run --example <example-name>
    cargo run --example audio_capturer
```

## Test
```
cargo test
```

## Build documentation
```
cargo doc
```

## Publish a library to crates.io
```
cargo publish
```

## Setup
### Download Rustup and install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Generate a new project
```
cargo new mop-rust --bin
```

## Regular work
### Get the latest version of Rust 
```
rustup update
```

## Words
```
Rustup: the Rust installer and version management tool
Cargo: the Rust build tool and package manager

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
    A binary or library
```

## Tools
```
Rustfmt: rustup component add rustfmt
    rustup component add rustfmt
    cargo fmt
```

## Crates
### cpal
```
CPAL - Cross-Platform Audio Library
    Low-level library for audio input and output in pure Rust.
```