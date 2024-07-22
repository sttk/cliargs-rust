# [cliargs_derive][repo-url] [![crate.io][crateio-img]][crateio-url] [![doc.rs][docrs-img]][docrs-url] [![CI Status][ci-img]][ci-url] [![MIT License][mit-img]][mit-url]

This derive macro is attached to a struct which holds command line option values, and automatically
implements
its method to generate `OptCfg`s from its fields, and other methods.

This macro automatically implements the method to generates a vector of `OptCfg` from the field
definitions and `opt` field attributes, and this also implements the method that instantiates
the struct using the default values specified in `opt` field attributes, and implements the
method to updates the field values with the values from the passed `HashMap`.

The `opt` field attribute can have the following pairs of name and value: one is `cfg` to
specify `names` and `defaults` of `OptCfg` struct, another is `desc` to specify `desc` of
`OptCfg` struct, and yet another is `arg` to specify `arg_in_help` of `OptCfg` struct.

The format of `cfg` is like `cfg="f,foo=123"`.
The left side of the equal sign is the option name(s), and the right side is the default
value(s).
If there is no equal sign, it is determined that only the option name is specified.
If you want to specify multiple option names, separate them with commas.
If you want to specify multiple default values, separate them with commas and round them with
square brackets, like `[1,2,3]`.
If you want to use your favorite carachter as a separator, you can use it by putting it on the
left side of the open square bracket, like `/[1/2/3]`.

The following code is a sample of a option store struct.

```rust
extern crate cliargs_derive;
use cliargs_derive::OptStore;

#[derive(OptStore)]
struct MyOptions {
    #[opt(cfg="f,foo-bar", desc="The description of foo-bar.")]
    foo_bar: bool,

    #[opt(cfg="b", desc="The description of baz.", arg="text")]
    baz: String,

    #[opt(cfg="q=[1,2,3]", desc="The description of qux.", arg="num...")]
    qux: Vec<u8>,
}
```

## License

Copyright (C) 2024 Takayuki Sato

This program is free software under MIT License.<br>
See the file LICENSE in this distribution for more details.


[repo-url]: https://github.com/sttk/cliargs-rust
[crateio-img]: https://img.shields.io/badge/crate.io-ver.0.1.0-fc8d62?logo=rust
[crateio-url]: https://crates.io/crates/cliargs_derive
[docrs-img]: https://img.shields.io/badge/doc.rs-cliargs-66c2a5?logo=docs.rs
[docrs-url]: https://docs.rs/cliargs_derive
[ci-img]: https://github.com/sttk/cliargs-rust/actions/workflows/rust.yml/badge.svg?branch=main
[ci-url]: https://github.com/sttk/cliargs-rust/actions
[mit-img]: https://img.shields.io/badge/license-MIT-green.svg
[mit-url]: https://opensource.org/licenses/MIT
