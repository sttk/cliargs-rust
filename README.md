# [cliargs-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

A library to parse command line arguments and print the help for Rust application.

This library provides the following functionalities:

- Supports [POSIX][posix-args] & [GNU][gnu-args] like short and long options.
    - This library supports `--` option.
    - This library doesn't support numeric short option.
    - This library supports not `-ofoo` but `-o=foo` as an alternative to `-o foo` for short option.
- Supports parsing with option configurations.
- Supports parsing with an object which stores option values and has annotations of fields.
- Is able to parse command line arguments including sub commands.
- Generates help text from option configurations.

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
cliargs = "0.1.0"
```

## Usage

### Parse without configurations


### Parse with configurations


### Parse for a option store


### Parse command line arguments including sub commands


## Supporting Rust versions

This crate supports Rust 1.74.1 or later.

```
% cargo msrv --no-check-feedback
Fetching index
Determining the Minimum Supported Rust Version (MSRV) for toolchain x86_64-apple-darwin
Using check command cargo check
   Finished The MSRV is: 1.74.1   █████████████████████████████████████ 00:00:02
```


## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/cliargs-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.1.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/cliargs
[docrs-img]: https://img.shields.io/badge/doc.rs-cliargs-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/cliargs
[ci-img]: https://github.com/sttk/cliargs-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/cliargs-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
