# [cliargs-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

A library to parse command line arguments and print the help for Rust application.

This library provides the following functionalities:

- Supports [POSIX][posix-args] & [GNU][gnu-args] like short and long options.
    - This library supports `--` option.
    - This library doesn't support numeric short option.
    - This library supports not `-ofoo` but `-o=foo` as an alternative to `-o foo` for short option.
- Supports parsing with option configurations. *(To be added)*
- Supports parsing with an object which stores option values and has annotations of fields. *(To be added)*
- Is able to parse command line arguments including sub commands. *(To be added)*
- Generates help text from option configurations. *(To be added)*

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
cliargs = "0.0.0"
```

## Usage

This crate provides the `Cmd` struct to parse command line arguments.
The usage of this `Cmd` struct is as follows:

### Creates a `Cmd` instance

The `Cmd::new` function  creates a `Cmd` instance with original command line arguments.
This function uses `std::env::args_os` and `OsString#into_string` to read command line arguments in order to avoid `panic!` call that the user cannot control.

```
use cliargs::Cmd;
use cliargs::errors::InvalidOsArg;

let cmd = match Cmd::new() {
    Ok(cmd) => cmd,
    Err(InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
        panic!("Invalid Unicode data: {:?} (index: {})", os_arg, index);
    }
};
```

### Creates a `Cmd` instance with the specified `String` array

The `Cmd::with_strings` function creates a `Cmd` instance with the specified `String` array.

```
use cliargs::Cmd;
use std::env;

let cmd = Cmd::with_strings(env::args());
```

### Creates a `Cmd` instance with the specified `OsString` array

The `Cmd::with_os_strings` function creates a `Cmd` instance with the specified `OsString` array.

```
use cliargs::Cmd;
use cliargs::errors::InvalidOsArg;
use std::env;

let cmd = match Cmd::with_os_strings(env::args_os()) {
    Ok(cmd) => cmd,
    Err(InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
        panic!("Invalid Unicode data: {:?} (index: {})", os_arg, index);
    }
};
```

### Parses without configurations

The `Cmd` struct has the method which parses command line arguments without configurations.
This method automatically divides command line arguments to command arguments, options, and option arguments.

Command line arguments starts with `-` or `--` are options, and others are command arguments.
If you want to specify a value to an option, follows `"="` and the value after the option, like `foo=123`.

All command line arguments after `--` are command arguments, even they starts with `-` or `--`.

```
use cliargs::Cmd;
use cliargs::errors::InvalidOption;

let cmd = Cmd::with_strings(vec![ /* ... */ ]);
match cmd.parse() {
    Ok(_) => { /* ... */ },
    Err(InvalidOption::OptionContainsInvalidChar { option }) => {
        panic!("Option contains invalid character: {option}");
    },
    Err(errr) => panic!("Invalid option: {}", err.option()),
}
```

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
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.0.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/cliargs
[docrs-img]: https://img.shields.io/badge/doc.rs-cliargs-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/cliargs
[ci-img]: https://github.com/sttk/cliargs-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/cliargs-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT

[posix-args]: https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html#Argument-Syntax
[gnu-args]: https://www.gnu.org/prep/standards/html_node/Command_002dLine-Interfaces.html
