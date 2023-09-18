# strings
Rust strings stuff

## Table of Contents
1. [Prerequisites](#Prerequisites)
2. [Install Cargo commands](#install-cargo-commands)
3. [Running examples](#running-examples)
4. [Running included Cargo.toml commands](#running-included-cargo.toml-commands)
   
## Prerequisites

### Install Cargo commands

#### Rust formatter

To install:
```shell
rustup component add rustfmt
```

To run:
```shell
cargo fmt
```

#### Install cargo-cmd

To install:
```shell
cargo install cargo-cmd
```

To run:
```shell
cargo cmd <command>
```

## Running examples

### getenv-sys
You can run this example with:
```shell
cargo run --bin getenv-sys
```

### list-cwd
You can run this example with:
```shell
cargo run --bin list-cwd
```

### print-file
You can run this example with:
```shell
cargo run --bin print-file
```

### simple
You can run this example with:
```shell
cargo run --bin simple
```

### arc-strings
You can run this example with:
```shell
cargo run --bin arc-strings
```

### latin1-string
You can run this example with:
```shell
cargo run --bin latin1-string
```

## Running included Cargo.toml commands

### Check Rust files formatting

Run:
```shell
cargo cmd fmt-check
```

### Fix Rust files formatting

Run:
```shell
cargo cmd fmt-fix
```
