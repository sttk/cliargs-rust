# [cliargs-rust][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

A library to parse command line arguments and print the help for Rust application.

This library provides the following functionalities:

- Supports [POSIX][posix-args] & [GNU][gnu-args] like short and long options.
    - This library supports `--` option.
    - This library doesn't support numeric short option.
    - This library supports not `-ofoo` but `-o=foo` as an alternative to `-o foo` for short option.
- Supports parsing with option configurations.
- Supports parsing with option configurations made from struct fields and attributes, and setting the option values to them.
- Is able to parse command line arguments including sub commands. *(To be added)*
- Generates help text from option configurations. *(To be added)*

## Install

In `Cargo.toml`, write this crate as a dependency.

```toml
[dependencies]
cliargs = "0.3.0"
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

let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
match cmd.parse() {
    Ok(_) => { /* ... */ },
    Err(InvalidOption::OptionContainsInvalidChar { option }) => {
        panic!("Option contains invalid character: {option}");
    },
    Err(err) => panic!("Invalid option: {}", err.option()),
}
```

### Parses with configurations

The `Cmd` struct has the method `parse_with` which parses command line arguments with configurations.
This method takes an array of option configurations: `OptCfg`, and divides command line arguments to options and command arguments according to this configurations..

An option configuration has fields: `store_key`, `names`, `has_arg`, `is_array`, `defaults`, `desc`, `arg_in_help`, and `validator`.

`store_key` field is specified the key name to store the option value to the option map in the `Cmd` instance.
If this field is not specified, the first element of `names` field is used instead.

`names` field is a string array and specified the option names, that are both long options and short options.
The order of elements in this field is used in a help text.
If you want to prioritize the output of short option name first in the help text, like `-f, --foo-bar`, but use the long option name as the key in the option map, write `store_key` and `names` fields as follows: `OptCfg::with(&[store_key("foo-bar"), names(&["f", "foo-bar"])])`.

`has_arg` field indicates the option requires one or more values.
`is_array` field indicates the option can have multiple values.
`defaults` field is an array of string which is used as default one or more option arguments if the option is not specified.
`desc` is a description of the option for help text.
`arg_n_help` field is a text which is output after option name and aliases as an option value in help text.

`validator` field is to set a function pointer which validates an option argument.
This crate provides the validator `cliargs::validators::validate_number<T>` which validates whether an option argument is valid format as a number.

The ownership of the vector of option configurations which is passed as an argument of this method
is moved to this method and set to the public field `cfgs` of `Cmd` instance.
If you want to access the option configurations after parsing, get them from this field.

```
use cliargs::{Cmd, OptCfg};
use cliargs::OptCfgParam::{names, has_arg, defaults, validator, desc, arg_in_help};
use cliargs::validators::validate_number;
use cliargs::errors::InvalidOption;

let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
let opt_cfgs = vec![
    OptCfg::with(&[
        names(&["foo-bar"]),
        desc("This is description of foo-bar."),
    ]),
    OptCfg::with(&[
        names(&["baz", "z"]),
        has_arg(true),
        defaults(&["1"]),
        desc("This is description of baz."),
        arg_in_help("<num>"),
        validator(validate_number::<u32>),
    ]),
];

match cmd.parse_with(opt_cfgs) {
    Ok(_) => { /* ... */ },
    Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
    Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
    Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
    Err(err) => panic!("Invalid option: {}", err.option()),
}
```

### Parse for a OptStore struct

The `Cmd` struct has the method `parse_for` which parses command line arguments and set their
option values to the option store which is passed as an argument.

This method divides command line arguments to command arguments and options, then sets
each option value to a curresponding field of the option store.

Within this method, a vector of `OptCfg` is made from the fields of the option store.
This `OptCfg` vector is set to the public field `cfgs` of the `Cmd` instance.
If you want to access this option configurations, get them from this field.

An option configuration corresponding to each field of an option store is determined by
its type and `opt` field attribute.
If the type is bool, the option takes no argument.
If the type is integer, floating point number or string, the option can takes single option
argument, therefore it can appear once in command line arguments.
If the type is a vector, the option can takes multiple option arguments, therefore it can
appear multiple times in command line arguments.

A `opt` field attribute can have the following pairs of name and value: one is `cfg` to
specify `names` and `defaults` fields of `OptCfg` struct, another is `desc` to specify
`desc` field, and yet another is `arg` to specify `arg_in_help` field.

The format of `cfg` is like `cfg="f,foo=123"`.
The left side of the equal sign is the option name(s), and the right side is the default
value(s).
If there is no equal sign, it is determined that only the option name is specified.
If you want to specify multiple option names, separate them with commas.
If you want to specify multiple default values, separate them with commas and round them
with square brackets, like `[1,2,3]`.
If you want to use your favorite carachter as a separator, you can use it by putting it on
the left side of the open square bracket, like `/[1/2/3]`.

NOTE: A default value of empty string array option in a field attribute is `[]`, like
`#[opt(cfg="=[]")]`, but it doesn't represent an array which contains only one empty
string.
If you want to specify an array which contains only one emtpy string, write nothing after
`=` symbol, like `#[opt(cfg="=")]`.

```
use cliargs::Cmd;
use cliargs::errors::InvalidOption;

#[derive(cliargs::OptStore)]
struct MyOptions {
    #[opt(cfg = "f,foo-bar", desc="The description of foo_bar.")]
    foo_bar: bool,
    #[opt(cfg = "b,baz", desc="The description of baz.", arg="<s>")]
    baz: String,
}
let mut my_options = MyOptions::with_defaults();

let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
match cmd.parse_for(&mut my_options) {
    Ok(_) => { /* ... */ },
    Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
    Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
    Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
    Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
    Err(err) => panic!("Invalid option: {}", err.option()),
}

let opt_cfgs = &cmd.cfgs;
```

## Supporting Rust versions

This crate supports Rust 1.74.1 or later.

```
% cargo msrv --no-check-feedback
Fetching index
Determining the Minimum Supported Rust Version (MSRV) for toolchain x86_64-apple-darwin
Using check command cargo check
   Finished The MSRV is: 1.74.1   █████████████████████████████████████████████████████████ 00:00:04
```


## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/cliargs-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.3.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/cliargs
[docrs-img]: https://img.shields.io/badge/doc.rs-cliargs-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/cliargs
[ci-img]: https://github.com/sttk/cliargs-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/cliargs-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT

[posix-args]: https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html#Argument-Syntax
[gnu-args]: https://www.gnu.org/prep/standards/html_node/Command_002dLine-Interfaces.html
