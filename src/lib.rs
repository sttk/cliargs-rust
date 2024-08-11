// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

//! This crate is a library to parse command line arguments.
//!
//! This crate provides the following functionalities:
//!
//! - Supports [POSIX][posix] & [GNU][gnu] like short and long options.
//!     - This crate supports `--` option.
//!     - This library doesn't support numeric short option.
//!     - This library supports not `-ofoo` but `-o=foo` as an alternative to
//!       `-o foo` for short option.
//! - Supports parsing with option configurations.
//! - Supports parsing with option configurations made from struct fields and attributes, and
//!   setting the option values to them.
//! - Supports parsing command line arguments including sub commands.
//! - Generates help text from option configurations.
//!
//! [posix]: https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html#Argument-Syntax
//! [gnu]: https://www.gnu.org/prep/standards/html_node/Command_002dLine-Interfaces.html
//!
//! ## Install
//!
//! In `Cargo.toml`, write this crate as a dependency.
//!
//! ```toml
//! [dependencies]
//! cliargs = "0.5.0"
//! ```
//!
//! ## Usage
//!
//! This crate provides the `Cmd` strcut to parse command line arguments.
//! The usage of this `Cmd` struct is as follows:
//!
//! ### Creates a `Cmd` instance
//!
//! The `Cmd::new` function creates a `Cmd` instance with original command line
//! arguments.
//! This function uses `std::env::arg_os` and `OsString#into_string` to read
//! command line arguments in order to avoid `panic!` call that the user cannot
//! control.
//!
//! ```rust
//! use cliargs::Cmd;
//! use cliargs::errors::InvalidOsArg;
//!
//! let cmd = match Cmd::new() {
//!     Ok(cmd) => cmd,
//!     Err(InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
//!         panic!("Invalid Unicode data: {:?} (index: {})", os_arg, index);
//!     }
//! };
//! ```
//!
//! ### Creates a `Cmd` instance with the specified `String` array
//!
//! The `Cmd::with_strings` function creates a `Cmd` instance with the
//! specified `String` array.
//!
//! ```rust
//! use cliargs::Cmd;
//! use std::env;
//!
//! let cmd = Cmd::with_strings(env::args());
//! ```
//!
//! ### Creates a `Cmd` instance with the specified `OsString` array.
//!
//! ```rust
//! use cliargs::Cmd;
//! use cliargs::errors::InvalidOsArg;
//! use std::env;
//!
//! let cmd = match Cmd::with_os_strings(env::args_os()) {
//!     Ok(cmd) => cmd,
//!     Err(InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
//!         panic!("Invalid Unicode data: {:?} (index: {})", os_arg, index);
//!     }
//! };
//! ```
//!
//! ## Parses without configurations
//!
//! The `Cmd` struct has the method which parses command line arguments without
//! configurations.
//! This method automatically divides command line arguments to options and
//! command arguments.
//!
//! Command line arguments starts with `-` or `--` are options, and others are
//! command arguments.
//! If you want to specify a value to an option, follows `"="` and the value
//! after the option, like `foo=123`.
//!
//! All command line arguments after `--` are command arguments, even they
//! starts with `-` or `--`.
//!
//! ```rust
//! use cliargs::Cmd;
//! use cliargs::errors::InvalidOption;
//!
//! let mut cmd = Cmd::with_strings([ /* ... */ ]);
//! match cmd.parse() {
//!     Ok(_) => { /* ... */ },
//!     Err(InvalidOption::OptionContainsInvalidChar { option }) => {
//!         panic!("Option contains invalid character: {option}");
//!     },
//!     Err(err) => panic!("Invalid option: {}", err.option()),
//! }
//! ```
//!
//! ## Parses with configurations
//!
//! The `Cmd` struct has the method `parse_with` which parses command line
//! arguments with configurations.
//! This method takes an array of option configurations: `OptCfg`, and divides
//! command line arguments to options and command arguments according to this
//! configurations..
//!
//! An option configuration has fields: `store_key`, `names`, `has_arg`,
//! `is_array`, `defaults`, `desc`, `arg_in_help`, and `validator`.
//!
//! `store_key` field is specified the key name to store the option value to
//! the option map in the `Cmd` instance.
//! If this field is not specified, the first element of `names` field is used
//! instead.
//!
//! `names` field is a string array and specified the option names, that are
//! both long options and short options.
//! The order of elements in this field is used in a help text.
//! If you want to prioritize the output of short option name first in the help
//! text, like `-f, --foo-bar`, but use the long option name as the key in the
//! option map, write `store_key` and `names` fields as follows:
//! `OptCfg::with([store_key("foo-bar"), names(&["f", "foo-bar"])])`.
//!
//! `has_arg` field indicates the option requires one or more values.
//! `is_array` field indicates the option can have multiple values.
//! `defaults` field is an array of string which is used as default one or more
//! option arguments if the option is not specified.
//! `desc` is a description of the option for help text.
//! `arg_n_help` field is a text which is output after option name and aliases
//! as an option value in help text.
//!
//! `validator` field is to set a function pointer which validates an option
//! argument.
//! This crate provides the validator `cliargs::validators::validate_number<T>`
//! which validates whether an option argument is valid format as a number.
//!
//! In addition,the help printing for an array of [OptCfg] is generated with [Help].
//!
//! ```rust
//! use cliargs::{Cmd, OptCfg};
//! use cliargs::OptCfgParam::{names, has_arg, defaults, validator, desc, arg_in_help};
//! use cliargs::validators::validate_number;
//! use cliargs::errors::InvalidOption;
//! use cliargs::Help;
//!
//! let mut cmd = Cmd::with_strings([ /* ... */ ]);
//! let opt_cfgs = vec![
//!     OptCfg {
//!         store_key: "foo_bar".to_string(),
//!         names: vec!["foo-bar".to_string(), "f".to_string()],
//!         has_arg: true,
//!         is_array: false,
//!         defaults: Some(vec![123.to_string()]),
//!         desc: "This is description of foo-bar.".to_string(),
//!         arg_in_help: "<num>".to_string(),
//!         validator: validate_number::<u64>,
//!     },
//!     OptCfg::with([
//!         names(&["baz", "z"]),
//!         has_arg(true),
//!         defaults(&["1"]),
//!         desc("This is description of baz."),
//!         arg_in_help("<num>"),
//!         validator(validate_number::<u64>),
//!     ]),
//! ];
//!
//! match cmd.parse_with(opt_cfgs) {
//!     Ok(_) => { /* ... */ },
//!     Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
//!     Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
//!     Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
//!     Err(err) => panic!("Invalid option: {}", err.option()),
//! }
//!
//! let opt_cfgs = cmd.opt_cfgs();
//!
//! let mut help = Help::new();
//! help.add_text("This is the usage description.".to_string());
//! help.add_opts_with_margins(opt_cfgs, 2, 0);
//! help.print();
//!
//! // (stdout)
//! // This is the usage description.
//! //   --foo-bar, -f    This is description of foo-bar.
//! //   --bar, -z <num>  This is description of baz.
//! ```
//!
//! ## Parse for a OptStore struct
//!
//! The [Cmd] struct has the method parse_for which parses command line arguments and set their option values to
//! the option store which is passed as an argument.
//!
//! This method divides command line arguments to command arguments and options, then sets each option value to a
//! curresponding field of the option store.
//!
//! Within this method, a vector of [OptCfg] is made from the fields of the option store. This [OptCfg] vector is
//! set to the public field cfgs of the [Cmd] instance. If you want to access this option configurations, get them
//! from this field.
//! An option configuration corresponding to each field of an option store is determined by its type and opt field
//! attribute.
//! If the type is bool, the option takes no argument. If the type is integer, floating point number or string, the
//! option can takes single option argument, therefore it can appear once in command line arguments.
//! If the type is a vector, the option can takes multiple option arguments, therefore it can appear multiple times
//! in command line arguments.
//!
//! A `opt` field attribute can have the following pairs of name and value: one is `cfg` to specify `names` and
//! `defaults` fields of [OptCfg] struct, another is `desc` to specify `desc` field, and yet another is `arg` to
//! specify `arg_in_help` field.
//!
//! The format of `cfg` is like `cfg="f,foo=123"`. The left side of the equal sign is the option name(s), and the
//! right side is the default value(s).
//! If there is no equal sign, it is determined that only the option name is specified.
//! If you want to specify multiple option names, separate them with commas.
//! If you want to specify multiple default values, separate them with commas and round them with square brackets,
//! like `[1,2,3]`.
//! If you want to use your favorite carachter as a separator, you can use it by putting it on the left side of the
//! open square bracket, like `/[1/2/3]`.
//!
//! NOTE: A default value of empty string array option in a field attribute is `[]`, like `#[opt(cfg="=[]")]`, but
//! it doesn't represent an array which contains only one empty string.
//! If you want to specify an array which contains only one emtpy string, write nothing after `=` symbol, like
//! `#[opt(cfg="=")]`.
//!
//! ```rust
//! use cliargs::Cmd;
//! use cliargs::errors::InvalidOption;
//! use cliargs::Help;
//!
//! #[derive(cliargs::OptStore)]
//! struct MyOptions {
//!     #[opt(cfg = "f,foo-bar", desc="The description of foo_bar.")]
//!     foo_bar: bool,
//!     #[opt(cfg = "b,baz", desc="The description of baz.", arg="<s>")]
//!     baz: String,
//! }
//! let mut my_options = MyOptions::with_defaults();
//!
//! let mut cmd = Cmd::with_strings([ /* ... */ ]);
//! match cmd.parse_for(&mut my_options) {
//!     Ok(_) => { /* ... */ },
//!     Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
//!     Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
//!     Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
//!     Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
//!     Err(err) => panic!("Invalid option: {}", err.option()),
//! }
//!
//! let opt_cfgs = cmd.opt_cfgs();
//!
//! let mut help = Help::new();
//! help.add_text("This is the usage description.".to_string());
//! help.add_opts_with_margins(opt_cfgs, 2, 0);
//! help.print();
//!
//! // (stdout)
//! // This is the usage description.
//! //   -f, --foo-bar  This is description of foo_bar.
//! //   -z, --baz <s>  This is description of baz.
//! ```
//!
//! ## Parse command line arguments including sub command
//!
//! This crate provides methods [Cmd::parse_until_sub_cmd], [Cmd::parse_until_sub_cmd_with], and
//! [Cmd::parse_until_sub_cmd_for] for parsing command line arguments including sub commands.
//!
//! These methods correspond to [Cmd::parse], [Cmd::parse_with], and [Cmd::parse_for],
//! respectively, and behave the same except that they stop parsing before the first command
//! argument (= sub command) and
//! return a [Cmd] instance containing the arguments starting from the the sub command.
//!
//! The folowing is an example code using [Cmd::parse_until_sub_cmd]:
//!
//! ```rust
//! use cliargs::Cmd;
//! use cliargs::errors::InvalidOption;
//!
//! let mut cmd = Cmd::with_strings([ /* ... */ ]);
//!
//! match cmd.parse_until_sub_cmd() {
//!     Ok(Some(mut sub_cmd)) => {
//!         let sub_cmd_name = sub_cmd.name();
//!         match sub_cmd.parse() {
//!             Ok(_) => { /* ... */ },
//!             Err(err) => panic!("Invalid option: {}", err.option()),
//!         }
//!     },
//!     Ok(None) => { /* ... */ },
//!     Err(InvalidOption::OptionContainsInvalidChar { option }) => {
//!         panic!("Option contains invalid character: {option}");
//!     },
//!     Err(err) => panic!("Invalid option: {}", err.option()),
//! }
//! ```

/// Enums for errors that can occur when parsing command line arguments.
pub mod errors;

mod opt_cfg;
pub use opt_cfg::OptCfg;
pub use opt_cfg::OptCfgParam;

mod help;
pub use help::Help;
pub use help::HelpIter;

/// Function pointers for validating an option argument.
pub use opt_cfg::validators;

mod parse;
pub use parse::OptStore;

extern crate cliargs_derive;
pub use cliargs_derive::OptStore;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::path;

/// Parses command line arguments and stores them.
///
/// The results of parsing are stored by separating into command name, command arguments, options,
/// and option arguments.
///
/// These values are retrieved as string slices with the same lifetime as this `Cmd` instance.
/// Therefore, if you want to use those values for a longer period, it is needed to convert them
/// to [String]s.
pub struct Cmd<'a> {
    name: &'a str,
    args: Vec<&'a str>,
    opts: HashMap<&'a str, Vec<&'a str>>,
    cfgs: Vec<OptCfg>,

    _leaked_strs: Vec<&'a str>,
    _num_of_args: usize,
}

impl<'a> Drop for Cmd<'a> {
    fn drop(&mut self) {
        for str in &self._leaked_strs {
            let boxed = unsafe { Box::from_raw(*str as *const str as *mut str) };
            mem::drop(boxed);
        }
    }
}

impl fmt::Debug for Cmd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cmd")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("opts", &self.opts)
            .finish()
    }
}

impl<'b, 'a> Cmd<'a> {
    /// Creates a `Cmd` instance with command line arguments obtained from [std::env::args_os].
    ///
    /// Since [std::env::args_os] returns a vector of [OsString] and they can contain invalid
    /// unicode data, the return value of this funciton is [Result] of `Cmd` or
    /// `errors::InvalidOsArg`.
    pub fn new() -> Result<Cmd<'a>, errors::InvalidOsArg> {
        Self::with_os_strings(env::args_os())
    }

    /// Creates a `Cmd` instance with the specified iterator of [OsString]s.
    ///
    /// [OsString]s can contain invalid unicode data, the return value of this function
    /// is [Result] of `Cmd` or `errors::InvalidOsArg`.
    pub fn with_os_strings(
        osargs: impl IntoIterator<Item = OsString>,
    ) -> Result<Cmd<'a>, errors::InvalidOsArg> {
        let osarg_iter = osargs.into_iter();
        let (size, _) = osarg_iter.size_hint();
        let mut _leaked_strs = Vec::with_capacity(size);

        let cmd_name_start: usize;

        let mut enm = osarg_iter.enumerate();
        if let Some((idx, osarg)) = enm.next() {
            // The first element is the command path.
            let path = path::Path::new(&osarg);
            let base_len = if let Some(base_os) = path.file_name() {
                if let Some(base_str) = base_os.to_str() {
                    base_str.len()
                } else {
                    0
                }
            } else {
                0
            };
            match osarg.into_string() {
                Ok(string) => {
                    let str: &'a str = string.leak();
                    _leaked_strs.push(str);
                    cmd_name_start = str.len() - base_len;
                }
                Err(osstring) => {
                    return Err(errors::InvalidOsArg::OsArgsContainInvalidUnicode {
                        index: idx,
                        os_arg: osstring,
                    });
                }
            }

            // The elements from the second one onward are the arguments.
            for (idx, osarg) in enm {
                match osarg.into_string() {
                    Ok(string) => {
                        let str: &'a str = string.leak();
                        _leaked_strs.push(str);
                    }
                    Err(osstring) => {
                        for str in _leaked_strs {
                            let boxed = unsafe { Box::from_raw(str as *const str as *mut str) };
                            mem::drop(boxed);
                        }
                        return Err(errors::InvalidOsArg::OsArgsContainInvalidUnicode {
                            index: idx,
                            os_arg: osstring,
                        });
                    }
                }
            }
        } else {
            _leaked_strs.push("");
            cmd_name_start = 0;
        }

        let _num_of_args = _leaked_strs.len();

        Ok(Cmd {
            name: &_leaked_strs[0][cmd_name_start..],
            args: Vec::new(),
            opts: HashMap::new(),
            cfgs: Vec::new(),
            _leaked_strs,
            _num_of_args,
        })
    }

    /// Creates a `Cmd` instance with the specified iterator of [String]s.
    pub fn with_strings(args: impl IntoIterator<Item = String>) -> Cmd<'a> {
        let arg_iter = args.into_iter();
        let (size, _) = arg_iter.size_hint();
        let mut _leaked_strs = Vec::with_capacity(size);

        for arg in arg_iter {
            let str: &'a str = arg.leak();
            _leaked_strs.push(str);
        }

        let cmd_name_start: usize;

        if _leaked_strs.len() > 0 {
            let path = path::Path::new(_leaked_strs[0]);
            let mut base_len = 0;
            if let Some(base_os) = path.file_name() {
                if let Some(base_str) = base_os.to_str() {
                    base_len = base_str.len();
                }
            }
            cmd_name_start = _leaked_strs[0].len() - base_len;
        } else {
            _leaked_strs.push("");
            cmd_name_start = 0;
        };

        let _num_of_args = _leaked_strs.len();

        Cmd {
            name: &_leaked_strs[0][cmd_name_start..],
            args: Vec::new(),
            opts: HashMap::new(),
            cfgs: Vec::new(),
            _leaked_strs,
            _num_of_args,
        }
    }

    fn sub_cmd(&'a self, from_index: usize) -> Cmd<'b> {
        Cmd::with_strings(
            self._leaked_strs[from_index..(self._num_of_args)]
                .into_iter()
                .map(|s| s.to_string()),
        )
    }

    /// Returns the command name.
    ///
    /// This name is base name extracted from the command path string slice, which is the first
    /// element of the command line arguments.
    pub fn name(&'a self) -> &'a str {
        self.name
    }

    /// Returns the command arguments.
    ///
    /// These arguments are retrieved as string slices in an array.
    pub fn args(&'a self) -> &'a [&'a str] {
        &self.args
    }

    /// Checks whether an option with the specified name exists.
    pub fn has_opt(&self, name: &str) -> bool {
        self.opts.contains_key(name)
    }

    /// Returns the option argument with the specified name.
    ///
    /// If the option has multiple arguments, this method returns the first argument.
    ///
    /// Since the option may not be specified in the command line arguments,
    /// the return value of this method  is an [Option] of an option argument or [None].
    pub fn opt_arg(&'a self, name: &str) -> Option<&'a str> {
        if let Some(opt_vec) = self.opts.get(name) {
            if opt_vec.len() > 0 {
                return Some(opt_vec[0]);
            }
        }
        None
    }

    /// Returns the option arguments with the specified name.
    ///
    /// If the option has one or multiple arguments, this method returns an array of the arguments.
    ///
    /// Since the option may not be specified in the command line arguments, the return value of
    /// this method is an [Option] of option arguments or [None].
    pub fn opt_args(&'a self, name: &str) -> Option<&'a [&'a str]> {
        match self.opts.get(name) {
            Some(vec) => Some(&vec),
            None => None,
        }
    }

    /// Retrieves the option configurations which was used to parse command line arguments.
    pub fn opt_cfgs(&'a self) -> &[OptCfg] {
        &self.cfgs
    }
}

#[cfg(test)]
mod tests_of_cmd {
    use super::Cmd;

    mod tests_of_new {
        use super::Cmd;

        #[test]
        fn should_create_a_new_instance() {
            let cmd = Cmd::new().unwrap();
            println!("cmd = {cmd:?}");
            println!("cmd._leaked_strs = {:?}", cmd._leaked_strs);
            assert!(cmd.name().starts_with("cliargs-"));
            assert!(cmd._leaked_strs.len() > 0);
        }
    }

    mod tests_of_with_strings {
        use super::Cmd;

        #[test]
        fn should_create_a_new_instance() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "bar".to_string(),
            ]);

            cmd.args.push(cmd._leaked_strs[2]);
            cmd.opts
                .insert(&cmd._leaked_strs[1][2..], Vec::with_capacity(0));

            println!("cmd = {cmd:?}");
            println!("cmd._leaked_strs = {:?}", cmd._leaked_strs);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_absolute_path() {
            let cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_relative_path() {
            let cmd = Cmd::with_strings([
                "../path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_base_name_only() {
            let cmd = Cmd::with_strings([
                "app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            let cmd = Cmd::with_strings([]);
            assert_eq!(cmd.name(), "");
        }
    }

    mod tests_of_with_os_strings {
        use super::Cmd;
        use std::ffi;

        #[test]
        fn should_create_a_new_instance() {
            let cmd = Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("bar_baz"),
                ffi::OsString::from("qux"),
            ])
            .unwrap();

            assert_eq!(cmd.name(), "app");
        }

        #[cfg(not(windows))] // Because OsStr is valid WTF8 and OsString is valid WTF16 on Windows
        #[test]
        fn should_fail_because_os_args_contain_invalid_unicode() {
            let bad_arg = b"bar\xFFbaz";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_arg) };
            let bad_os_string = bad_os_str.to_os_string();

            match Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                bad_os_string.clone(),
                ffi::OsString::from("qux"),
            ]) {
                Ok(_) => assert!(false),
                Err(crate::errors::InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                    assert_eq!(index, 2);
                    assert_eq!(os_arg, bad_os_string);
                }
            }
        }

        #[cfg(not(windows))] // Because OsStr is valid WTF8 and OsString is valid WTF16 on Windows
        #[test]
        fn should_fail_because_command_name_contains_invalid_unicode() {
            let bad_arg = b"bar\xFFbaz";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_arg) };
            let bad_os_string = bad_os_str.to_os_string();

            match Cmd::with_os_strings([
                bad_os_string.clone(),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("qux"),
            ]) {
                Ok(_) => assert!(false),
                Err(crate::errors::InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                    assert_eq!(index, 0);
                    assert_eq!(os_arg, bad_os_string);
                }
            }
        }

        #[test]
        fn should_get_command_name_from_absolute_path() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_from_relative_path() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("../path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_from_base_name_only() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            if let Ok(cmd) = Cmd::with_os_strings([]) {
                assert_eq!(cmd.name(), "");
            } else {
                assert!(false);
            }
        }
    }

    mod tests_of_getters {
        use super::Cmd;

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            let cmd = Cmd::with_strings([]);

            assert_eq!(cmd.name(), "");
        }

        #[test]
        fn should_get_command_arguments() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._leaked_strs[6]);
            cmd.args.push(cmd._leaked_strs[7]);
            cmd.opts
                .insert(&cmd._leaked_strs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._leaked_strs[2][2..],
                vec![&cmd._leaked_strs[3], &cmd._leaked_strs[5]],
            );

            assert_eq!(cmd.args(), ["quux", "corge"]);
        }

        #[test]
        fn should_check_option_is_specified() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._leaked_strs[6]);
            cmd.args.push(cmd._leaked_strs[7]);
            cmd.opts
                .insert(&cmd._leaked_strs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._leaked_strs[2][2..],
                vec![&cmd._leaked_strs[3], &cmd._leaked_strs[5]],
            );

            assert_eq!(cmd.has_opt("foo"), true);
            assert_eq!(cmd.has_opt("bar"), true);
            assert_eq!(cmd.has_opt("baz"), false);
        }

        #[test]
        fn should_get_single_option_argument() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._leaked_strs[6]);
            cmd.args.push(cmd._leaked_strs[7]);
            cmd.opts
                .insert(&cmd._leaked_strs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._leaked_strs[2][2..],
                vec![&cmd._leaked_strs[3], &cmd._leaked_strs[5]],
            );

            assert_eq!(cmd.opt_arg("foo"), None);
            assert_eq!(cmd.opt_arg("bar"), Some("baz"));
            assert_eq!(cmd.opt_arg("baz"), None);
        }

        #[test]
        fn should_get_multiple_option_arguments() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._leaked_strs[6]);
            cmd.args.push(cmd._leaked_strs[7]);
            cmd.opts
                .insert(&cmd._leaked_strs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._leaked_strs[2][2..],
                vec![&cmd._leaked_strs[3], &cmd._leaked_strs[5]],
            );

            assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));
            assert_eq!(cmd.opt_args("bar"), Some(&["baz", "qux"] as &[&str]));
            assert_eq!(cmd.opt_args("baz"), None);
        }
    }

    mod tests_of_moving_cmd {
        use crate::Cmd;
        use crate::OptCfg;
        use crate::OptCfgParam::*;

        #[test]
        fn should_move_by_passing_a_parameter() {
            fn move_cmd(cmd: Cmd) {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &["baz", "qux", "quux", "corge"]);
                assert_eq!(cmd.opt_args("foo").unwrap(), &Vec::<&str>::new());
                assert_eq!(cmd.opt_args("bar").unwrap(), &["ABC", "DEF"]);
                assert_eq!(
                    cmd._leaked_strs,
                    &[
                        "/path/to/app",
                        "--foo",
                        "--bar=ABC",
                        "baz",
                        "--bar=DEF",
                        "qux",
                        "quux",
                        "corge",
                        "foo",
                        "bar",
                    ]
                );
                assert_eq!(cmd.opt_cfgs().len(), 2);
                assert_eq!(cmd.opt_cfgs()[0].store_key, "");
                assert_eq!(cmd.opt_cfgs()[0].names, &["foo"]);
                assert_eq!(cmd.opt_cfgs()[0].has_arg, false);
                assert_eq!(cmd.opt_cfgs()[0].is_array, false);
                assert_eq!(cmd.opt_cfgs()[0].defaults, None);
                assert_eq!(cmd.opt_cfgs()[0].desc, "");
                assert_eq!(cmd.opt_cfgs()[0].arg_in_help, "");
                assert_eq!(cmd.opt_cfgs()[1].store_key, "");
                assert_eq!(cmd.opt_cfgs()[1].names, &["bar"]);
                assert_eq!(cmd.opt_cfgs()[1].has_arg, true);
                assert_eq!(cmd.opt_cfgs()[1].is_array, true);
                assert_eq!(cmd.opt_cfgs()[1].defaults, None);
                assert_eq!(cmd.opt_cfgs()[1].desc, "");
                assert_eq!(cmd.opt_cfgs()[1].arg_in_help, "");
            }

            let cfgs = vec![
                OptCfg::with([names(&["foo"])]),
                OptCfg::with([names(&["bar"]), has_arg(true), is_array(true)]),
            ];

            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar=ABC".to_string(),
                "baz".to_string(),
                "--bar=DEF".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            let _ = cmd.parse_with(cfgs);

            move_cmd(cmd);
        }

        #[test]
        fn should_move_by_returning() {
            fn move_cmd() -> Cmd<'static> {
                let cfgs = vec![
                    OptCfg::with([names(&["foo"])]),
                    OptCfg::with([names(&["bar"]), has_arg(true), is_array(true)]),
                ];

                let mut cmd = Cmd::with_strings([
                    "/path/to/app".to_string(),
                    "--foo".to_string(),
                    "--bar=ABC".to_string(),
                    "baz".to_string(),
                    "--bar=DEF".to_string(),
                    "qux".to_string(),
                    "quux".to_string(),
                    "corge".to_string(),
                ]);
                let _ = cmd.parse_with(cfgs);
                cmd
            }

            let cmd = move_cmd();
            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["baz", "qux", "quux", "corge"]);
            assert_eq!(cmd.opt_args("foo").unwrap(), &Vec::<&str>::new());
            assert_eq!(cmd.opt_args("bar").unwrap(), &["ABC", "DEF"]);
            assert_eq!(
                cmd._leaked_strs,
                &[
                    "/path/to/app",
                    "--foo",
                    "--bar=ABC",
                    "baz",
                    "--bar=DEF",
                    "qux",
                    "quux",
                    "corge",
                    "foo",
                    "bar",
                ]
            );
            assert_eq!(cmd.opt_cfgs().len(), 2);
            assert_eq!(cmd.opt_cfgs()[0].store_key, "");
            assert_eq!(cmd.opt_cfgs()[0].names, &["foo"]);
            assert_eq!(cmd.opt_cfgs()[0].has_arg, false);
            assert_eq!(cmd.opt_cfgs()[0].is_array, false);
            assert_eq!(cmd.opt_cfgs()[0].defaults, None);
            assert_eq!(cmd.opt_cfgs()[0].desc, "");
            assert_eq!(cmd.opt_cfgs()[0].arg_in_help, "");
            assert_eq!(cmd.opt_cfgs()[1].store_key, "");
            assert_eq!(cmd.opt_cfgs()[1].names, &["bar"]);
            assert_eq!(cmd.opt_cfgs()[1].has_arg, true);
            assert_eq!(cmd.opt_cfgs()[1].is_array, true);
            assert_eq!(cmd.opt_cfgs()[1].defaults, None);
            assert_eq!(cmd.opt_cfgs()[1].desc, "");
            assert_eq!(cmd.opt_cfgs()[1].arg_in_help, "");
        }

        #[test]
        fn should_move_by_mem_replace() {
            fn move_cmd() -> Cmd<'static> {
                let cfgs = vec![
                    OptCfg::with([names(&["foo"])]),
                    OptCfg::with([names(&["bar"]), has_arg(true), is_array(true)]),
                ];

                let mut cmd = Cmd::with_strings([
                    "/path/to/app".to_string(),
                    "--foo".to_string(),
                    "--bar=ABC".to_string(),
                    "baz".to_string(),
                    "--bar=DEF".to_string(),
                    "qux".to_string(),
                    "quux".to_string(),
                    "corge".to_string(),
                ]);
                let _ = cmd.parse_with(cfgs);

                let mut cmd1 = Cmd::with_strings([]);
                let _ = std::mem::replace(&mut cmd1, cmd);
                cmd1
            }

            let cmd = move_cmd();
            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["baz", "qux", "quux", "corge"]);
            assert_eq!(cmd.opt_args("foo").unwrap(), &Vec::<&str>::new());
            assert_eq!(cmd.opt_args("bar").unwrap(), &["ABC", "DEF"]);
            assert_eq!(
                cmd._leaked_strs,
                &[
                    "/path/to/app",
                    "--foo",
                    "--bar=ABC",
                    "baz",
                    "--bar=DEF",
                    "qux",
                    "quux",
                    "corge",
                    "foo",
                    "bar",
                ]
            );
            assert_eq!(cmd.opt_cfgs().len(), 2);
            assert_eq!(cmd.opt_cfgs()[0].store_key, "");
            assert_eq!(cmd.opt_cfgs()[0].names, &["foo"]);
            assert_eq!(cmd.opt_cfgs()[0].has_arg, false);
            assert_eq!(cmd.opt_cfgs()[0].is_array, false);
            assert_eq!(cmd.opt_cfgs()[0].defaults, None);
            assert_eq!(cmd.opt_cfgs()[0].desc, "");
            assert_eq!(cmd.opt_cfgs()[0].arg_in_help, "");
            assert_eq!(cmd.opt_cfgs()[1].store_key, "");
            assert_eq!(cmd.opt_cfgs()[1].names, &["bar"]);
            assert_eq!(cmd.opt_cfgs()[1].has_arg, true);
            assert_eq!(cmd.opt_cfgs()[1].is_array, true);
            assert_eq!(cmd.opt_cfgs()[1].defaults, None);
            assert_eq!(cmd.opt_cfgs()[1].desc, "");
            assert_eq!(cmd.opt_cfgs()[1].arg_in_help, "");
        }

        #[test]
        fn should_move_by_mem_swap() {
            fn move_cmd() -> Cmd<'static> {
                let cfgs = vec![
                    OptCfg::with([names(&["foo"])]),
                    OptCfg::with([names(&["bar"]), has_arg(true), is_array(true)]),
                ];

                let mut cmd = Cmd::with_strings([
                    "/path/to/app".to_string(),
                    "--foo".to_string(),
                    "--bar=ABC".to_string(),
                    "baz".to_string(),
                    "--bar=DEF".to_string(),
                    "qux".to_string(),
                    "quux".to_string(),
                    "corge".to_string(),
                ]);
                let _ = cmd.parse_with(cfgs);

                let mut cmd1 = Cmd::with_strings([]);
                let _ = std::mem::swap(&mut cmd1, &mut cmd);
                cmd1
            }

            let cmd = move_cmd();
            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["baz", "qux", "quux", "corge"]);
            assert_eq!(cmd.opt_args("foo").unwrap(), &Vec::<&str>::new());
            assert_eq!(cmd.opt_args("bar").unwrap(), &["ABC", "DEF"]);
            assert_eq!(
                cmd._leaked_strs,
                &[
                    "/path/to/app",
                    "--foo",
                    "--bar=ABC",
                    "baz",
                    "--bar=DEF",
                    "qux",
                    "quux",
                    "corge",
                    "foo",
                    "bar",
                ]
            );
            assert_eq!(cmd.opt_cfgs().len(), 2);
            assert_eq!(cmd.opt_cfgs()[0].store_key, "");
            assert_eq!(cmd.opt_cfgs()[0].names, &["foo"]);
            assert_eq!(cmd.opt_cfgs()[0].has_arg, false);
            assert_eq!(cmd.opt_cfgs()[0].is_array, false);
            assert_eq!(cmd.opt_cfgs()[0].defaults, None);
            assert_eq!(cmd.opt_cfgs()[0].desc, "");
            assert_eq!(cmd.opt_cfgs()[0].arg_in_help, "");
            assert_eq!(cmd.opt_cfgs()[1].store_key, "");
            assert_eq!(cmd.opt_cfgs()[1].names, &["bar"]);
            assert_eq!(cmd.opt_cfgs()[1].has_arg, true);
            assert_eq!(cmd.opt_cfgs()[1].is_array, true);
            assert_eq!(cmd.opt_cfgs()[1].defaults, None);
            assert_eq!(cmd.opt_cfgs()[1].desc, "");
            assert_eq!(cmd.opt_cfgs()[1].arg_in_help, "");
        }
    }
}
