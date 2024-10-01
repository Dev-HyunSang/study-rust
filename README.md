# The Rust Programming Language

## Installation

```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# or
$ brew install rustc
$ xcode-select --install
```

### Update and Delete

```shell
$ rustup update # update
$ rustup self uninstall # delete
```

## Using Cargo

```shell
$ cargo --version

# Create Project
$ cargo new hello_cargo
$ cd hello_cargo

# Build
$ cargo build
$ ./target/debug/hello_cargo # create binary

# Run
$ cargo run

# check (code)
$ cargo check
```