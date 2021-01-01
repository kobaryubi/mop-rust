# mop-rust
## Build
```
cargo build
```

## Run
```
cargo run
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
```

## Tools
```
Rustfmt: rustup component add rustfmt
    rustup component add rustfmt
    cargo fmt
```