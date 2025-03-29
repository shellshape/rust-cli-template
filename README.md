# Rust CLI Template

This is a simple Rust CLI template which I am using for my projects like those in my [shellshape](https://github.com/shellshape) collection.

This template is based on [Clap](https://crates.io/crates/clap) in combination with [Figment](https://crates.io/crates/figment) and [dirs](https://crates.io/crates/dirs) for parsing configuration files. Also, [anyhow](https://crates.io/crates/anyhow) is used for basic error handling.

Feel free to use this template as basis for your own projects!

## Usage

First of all, you need [cargo-generate](https://github.com/cargo-generate/cargo-generate) to be installed.
```bash
cargo install cargo-generate
```

Now, you can simply use the following command to bootstrap your project with this template.
```bash
cargo generate --git https://github.com/shellshape/rust-cli-template template
```