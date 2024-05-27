// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::cmp;
use std::error;
use std::ffi;
use std::fmt;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    OsArgsContainInvalidUnicode { index: usize, os_arg: ffi::OsString },
    InvalidOption(OptionError<'a>),
    InvalidConfig(ConfigError<'a>),
    FailToParse(ParseError<'a>),
}

#[derive(Debug, PartialEq)]
pub enum OptionError<'a> {
    OptionContainsInvalidChar { option: &'a str },
    UnconfiguredOption { option: &'a str },
    OptionNeedsArg { option: &'a str, store_key: &'a str },
    OptionTakesNoArg { option: &'a str, store_key: &'a str },
    OptionIsNotMultiArgs { option: &'a str, store_key: &'a str },
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::OsArgsContainInvalidUnicode { index, os_arg } => write!(
                f,
                "The command line arguments contain invalid unicode (index: {}, argument: \"{}\")",
                index,
                String::from_utf8_lossy(os_arg.as_encoded_bytes()).escape_debug(),
            ),
            Error::InvalidOption::<'_>(err) => return write!(f, "{err}"),
            Error::InvalidConfig::<'_>(err) => return write!(f, "{err}"),
            Error::FailToParse::<'_>(err) => return write!(f, "{err}"),
        }
    }
}

impl error::Error for Error<'_> {}

impl<'a> OptionError<'a> {
    pub fn option(&self) -> &'a str {
        return match self {
            OptionError::OptionContainsInvalidChar { option } => option,
            OptionError::UnconfiguredOption { option } => option,
            OptionError::OptionNeedsArg { option, .. } => option,
            OptionError::OptionTakesNoArg { option, .. } => option,
            OptionError::OptionIsNotMultiArgs { option, .. } => option,
        };
    }
}

impl fmt::Display for OptionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            OptionError::OptionContainsInvalidChar { option } => write!(
                f,
                "The option contains invalid characters (option: \"{}\")",
                option.escape_debug(),
            ),
            OptionError::UnconfiguredOption { option } => write!(
                f,
                "The option is not specified in configurations (option: \"{}\")",
                option.escape_debug(),
            ),
            OptionError::OptionNeedsArg { option, .. } => write!(
                f,
                "The option needs argument(s) (option: \"{}\")",
                option.escape_debug(),
            ),
            OptionError::OptionTakesNoArg { option, .. } => write!(
                f,
                "The option takes no argument (option: \"{}\")",
                option.escape_debug(),
            ),
            OptionError::OptionIsNotMultiArgs { option, .. } => write!(
                f,
                "The option cannot have multiple arguments (option: \"{}\")",
                option.escape_debug(),
            ),
        }
    }
}

impl error::Error for OptionError<'_> {}

#[derive(Debug, PartialEq)]
pub enum ConfigError<'a> {
    StoreKeyIsDuplicated { store_key: &'a str },
    ConfigIsMultiArgsButHasNoArg { store_key: &'a str },
    ConfigHasDefaultsButHasNoArg { store_key: &'a str },
    OptionNameIsDuplicated { store_key: &'a str, name: &'a str },
}

impl<'a> ConfigError<'a> {
    pub fn store_key(&self) -> &'a str {
        return match self {
            ConfigError::StoreKeyIsDuplicated { store_key } => store_key,
            ConfigError::ConfigIsMultiArgsButHasNoArg { store_key } => store_key,
            ConfigError::ConfigHasDefaultsButHasNoArg { store_key } => store_key,
            ConfigError::OptionNameIsDuplicated { store_key, .. } => store_key,
        };
    }
}

impl fmt::Display for ConfigError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ConfigError::StoreKeyIsDuplicated { store_key } => write!(
                f,
                "The store key is duplicated (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            ConfigError::ConfigIsMultiArgsButHasNoArg { store_key } => write!(
                f,
                "The configuration is specified both having multiple arguments and having no argument (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            ConfigError::ConfigHasDefaultsButHasNoArg { store_key } => write!(
                f,
                "The configuration is specified both default argument(s) and having no argument (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            ConfigError::OptionNameIsDuplicated { store_key, name } => write!(
                f,
                "The option name in the configuration is duplicated (store_key: \"{}\", name: \"{}\")",
                store_key.escape_debug(),
                name.escape_debug()
            ),
        }
    }
}

impl error::Error for ConfigError<'_> {}

#[derive(Debug)]
pub enum ParseError<'a> {
    InvalidInt {
        option: &'a str,
        field: &'a str,
        input: &'a str,
        bit_size: u8,
        cause: Box<dyn error::Error + 'a>,
    },
    InvalidUint {
        option: &'a str,
        field: &'a str,
        input: &'a str,
        bit_size: u8,
        cause: Box<dyn error::Error + 'a>,
    },
    InvalidFloat {
        option: &'a str,
        field: &'a str,
        input: &'a str,
        bit_size: u8,
        cause: Box<dyn error::Error + 'a>,
    },
}

impl<'a> ParseError<'a> {
    pub fn option(&self) -> &'a str {
        return match self {
            ParseError::InvalidInt { option, .. } => option,
            ParseError::InvalidUint { option, .. } => option,
            ParseError::InvalidFloat { option, .. } => option,
        };
    }

    pub fn field(&self) -> &'a str {
        return match self {
            ParseError::InvalidInt { field, .. } => field,
            ParseError::InvalidUint { field, .. } => field,
            ParseError::InvalidFloat { field, .. } => field,
        };
    }

    pub fn input(&self) -> &'a str {
        return match self {
            ParseError::InvalidInt { input, .. } => input,
            ParseError::InvalidUint { input, .. } => input,
            ParseError::InvalidFloat { input, .. } => input,
        };
    }
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParseError::InvalidInt { option, input, .. } => write!(
                f,
                "The option arguments is invalid as an integer (option: \"{}\", argument: \"{}\")",
                option.escape_debug(),
                input.escape_debug(),
            ),
            ParseError::InvalidUint { option, input, .. } => write!(
                f,
                "The option arguments is invalid as an unsigned integer (option: \"{}\", argument: \"{}\")",
                option.escape_debug(),
                input.escape_debug(),
            ),
            ParseError::InvalidFloat { option, input, .. } => write!(
                f,
                "The option arguments is invalid as a floating point number (option: \"{}\", argument: \"{}\")",
                option.escape_debug(),
                input.escape_debug(),
            ),
        }
    }
}

impl error::Error for ParseError<'_> {}

impl cmp::PartialEq for ParseError<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }
        match (self, other) {
            (
                ParseError::InvalidInt {
                    option: o1,
                    field: f1,
                    input: i1,
                    bit_size: b1,
                    ..
                },
                ParseError::InvalidInt {
                    option: o2,
                    field: f2,
                    input: i2,
                    bit_size: b2,
                    ..
                },
            ) => {
                return o1 == o2 && f1 == f2 && i1 == i2 && b1 == b2;
            }
            (
                ParseError::InvalidUint {
                    option: o1,
                    field: f1,
                    input: i1,
                    bit_size: b1,
                    ..
                },
                ParseError::InvalidUint {
                    option: o2,
                    field: f2,
                    input: i2,
                    bit_size: b2,
                    ..
                },
            ) => {
                return o1 == o2 && f1 == f2 && i1 == i2 && b1 == b2;
            }
            (
                ParseError::InvalidFloat {
                    option: o1,
                    field: f1,
                    input: i1,
                    bit_size: b1,
                    ..
                },
                ParseError::InvalidFloat {
                    option: o2,
                    field: f2,
                    input: i2,
                    bit_size: b2,
                    ..
                },
            ) => {
                return o1 == o2 && f1 == f2 && i1 == i2 && b1 == b2;
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests_of_cliargs_error {

    #[cfg(not(windows))] // Because basically OsStr is valid WTF8 and OsString is valid WTF16 on windows
    mod tests_of_os_args_contain_invalid_unicode {
        use crate::Error;
        use std::error;
        use std::ffi;

        #[test]
        fn should_create_and_handle() {
            let bad_input = b"Hello \xF0\x90\x80World";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), Error> = Err(Error::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => match err {
                    Error::OsArgsContainInvalidUnicode { index, os_arg } => {
                        assert_eq!(*index, 12);
                        assert_eq!(os_arg, &bad_os_string);
                    }
                    _ => assert!(false),
                },
            }
        }

        #[test]
        fn should_write_for_debug() {
            let bad_input = b"Hello \xF0\x90\x80World (\"\\\")";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), Error> = Err(Error::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    println!("{err:?}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OsArgsContainInvalidUnicode { index: 12, os_arg: \"Hello \\xF0\\x90\\x80World (\\\"\\\\\\\")\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let bad_input = b"Hello \xF0\x90\x80World (\"\\\")";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), Error> = Err(Error::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The command line arguments contain invalid unicode (index: 12, argument: \"Hello \u{fffd}World (\\\"\\\\\\\")\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_an_error() {
            let bad_input = b"Hello \xF0\x90\x80World (\"\\\")";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), Box<dyn error::Error>> =
                Err(Box::new(Error::OsArgsContainInvalidUnicode {
                    index: 12,
                    os_arg: bad_os_string.clone(),
                }));
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    println!("{err}");
                    println!("{err:?}");
                    if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                        match cliargs_err {
                            Error::OsArgsContainInvalidUnicode { index, os_arg } => {
                                assert_eq!(*index, 12);
                                assert_eq!(os_arg, &bad_os_string);
                            }
                            _ => assert!(false),
                        }
                    } else {
                        assert!(false);
                    }
                }
            }
        }
    }

    mod tests_of_invalid_option {
        mod option_contains_invalid_char {
            use crate::Error;
            use crate::OptionError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> = Err(Error::InvalidOption(
                    OptionError::OptionContainsInvalidChar { option: "foo-bar" },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidOption(err) => {
                            assert_eq!(err.option(), "foo-bar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar {
                        option,
                    })) => {
                        assert_eq!(option, "foo-bar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> = Err(Error::InvalidOption(
                    OptionError::OptionContainsInvalidChar { option: "foo-bar" },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidOption(OptionContainsInvalidChar { option: \"foo-bar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> = Err(Error::InvalidOption(
                    OptionError::OptionContainsInvalidChar { option: "foo-bar" },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option contains invalid characters (option: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> =
                    Err(Box::new(Error::InvalidOption(
                        OptionError::OptionContainsInvalidChar { option: "foo-bar" },
                    )));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(err2) => {
                                    assert_eq!(err2.option(), "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(OptionError::OptionContainsInvalidChar {
                                    option,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod unconfigured_option {
            use crate::Error;
            use crate::OptionError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::UnconfiguredOption {
                        option: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidOption(err) => {
                            assert_eq!(err.option(), "foo-bar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidOption(OptionError::UnconfiguredOption { option })) => {
                        assert_eq!(option, "foo-bar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::UnconfiguredOption {
                        option: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidOption(UnconfiguredOption { option: \"foo-bar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::UnconfiguredOption {
                        option: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option is not specified in configurations (option: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidOption(OptionError::UnconfiguredOption { option: "foo-bar" }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(err2) => {
                                    assert_eq!(err2.option(), "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(OptionError::UnconfiguredOption {
                                    option,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod option_needs_arg {
            use crate::Error;
            use crate::OptionError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionNeedsArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidOption(err) => {
                            assert_eq!(err.option(), "foo-bar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidOption(OptionError::OptionNeedsArg {
                        option,
                        store_key,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionNeedsArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidOption(OptionNeedsArg { option: \"foo-bar\", store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionNeedsArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option needs argument(s) (option: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidOption(OptionError::OptionNeedsArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(err2) => {
                                    assert_eq!(err2.option(), "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(OptionError::OptionNeedsArg {
                                    option,
                                    store_key,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod option_takes_no_arg {
            use crate::Error;
            use crate::OptionError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionTakesNoArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidOption(err) => {
                            assert_eq!(err.option(), "foo-bar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidOption(OptionError::OptionTakesNoArg {
                        option,
                        store_key,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionTakesNoArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidOption(OptionTakesNoArg { option: \"foo-bar\", store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionTakesNoArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option takes no argument (option: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidOption(OptionError::OptionTakesNoArg {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(err2) => {
                                    assert_eq!(err2.option(), "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(OptionError::OptionTakesNoArg {
                                    option,
                                    store_key,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod option_is_not_multi_args {
            use crate::Error;
            use crate::OptionError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidOption(err) => {
                            assert_eq!(err.option(), "foo-bar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                        option,
                        store_key,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidOption(OptionIsNotMultiArgs { option: \"foo-bar\", store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option cannot have multiple arguments (option: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                        option: "foo-bar",
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(err2) => {
                                    assert_eq!(err2.option(), "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidOption(OptionError::OptionIsNotMultiArgs {
                                    option,
                                    store_key,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }
    }

    mod tests_of_invalid_config {
        mod store_key_is_duplicated {
            use crate::ConfigError;
            use crate::Error;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated {
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidConfig(err) => {
                            assert_eq!(err.store_key(), "FooBar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated { store_key })) => {
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated {
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidConfig(StoreKeyIsDuplicated { store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated {
                        store_key: "FooBar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The store key is duplicated (store_key: \"FooBar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated {
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(err2) => {
                                    assert_eq!(err2.store_key(), "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(ConfigError::StoreKeyIsDuplicated {
                                    store_key,
                                }) => {
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod config_is_multi_args_but_has_no_arg {
            use crate::ConfigError;
            use crate::Error;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigIsMultiArgsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidConfig(err) => {
                            assert_eq!(err.store_key(), "FooBar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidConfig(ConfigError::ConfigIsMultiArgsButHasNoArg {
                        store_key,
                    })) => {
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigIsMultiArgsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidConfig(ConfigIsMultiArgsButHasNoArg { store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigIsMultiArgsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The configuration is specified both having multiple arguments and having no argument (store_key: \"FooBar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidConfig(ConfigError::ConfigIsMultiArgsButHasNoArg {
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(err2) => {
                                    assert_eq!(err2.store_key(), "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(
                                    ConfigError::ConfigIsMultiArgsButHasNoArg { store_key },
                                ) => {
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod config_has_defaults_but_has_no_arg {
            use crate::ConfigError;
            use crate::Error;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigHasDefaultsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidConfig(err) => {
                            assert_eq!(err.store_key(), "FooBar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidConfig(ConfigError::ConfigHasDefaultsButHasNoArg {
                        store_key,
                    })) => {
                        assert_eq!(store_key, "FooBar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigHasDefaultsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidConfig(ConfigHasDefaultsButHasNoArg { store_key: \"FooBar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> = Err(Error::InvalidConfig(
                    ConfigError::ConfigHasDefaultsButHasNoArg {
                        store_key: "FooBar",
                    },
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The configuration is specified both default argument(s) and having no argument (store_key: \"FooBar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidConfig(ConfigError::ConfigHasDefaultsButHasNoArg {
                        store_key: "FooBar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(err2) => {
                                    assert_eq!(err2.store_key(), "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(
                                    ConfigError::ConfigHasDefaultsButHasNoArg { store_key },
                                ) => {
                                    assert_eq!(*store_key, "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }

        mod option_name_is_duplicated {
            use crate::ConfigError;
            use crate::Error;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                        store_key: "FooBar",
                        name: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::InvalidConfig(err) => {
                            assert_eq!(err.store_key(), "FooBar");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                        store_key,
                        name,
                    })) => {
                        assert_eq!(store_key, "FooBar");
                        assert_eq!(name, "foo-bar");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                        store_key: "FooBar",
                        name: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "InvalidConfig(OptionNameIsDuplicated { store_key: \"FooBar\", name: \"foo-bar\" })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let result: Result<(), Error> =
                    Err(Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                        store_key: "FooBar",
                        name: "foo-bar",
                    }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option name in the configuration is duplicated (store_key: \"FooBar\", name: \"foo-bar\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let result: Result<(), Box<dyn error::Error>> = Err(Box::new(
                    Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                        store_key: "FooBar",
                        name: "foo-bar",
                    }),
                ));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(err2) => {
                                    assert_eq!(err2.store_key(), "FooBar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::InvalidConfig(ConfigError::OptionNameIsDuplicated {
                                    store_key,
                                    name,
                                }) => {
                                    assert_eq!(*store_key, "FooBar");
                                    assert_eq!(*name, "foo-bar");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
            }
        }
    }

    mod tests_of_fail_to_parse {
        mod invalid_int {
            use crate::Error;
            use crate::ParseError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let err0 = match "x1234".parse::<i32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidInt {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::FailToParse(err) => {
                            assert_eq!(err.option(), "foo-bar");
                            assert_eq!(err.field(), "FooBar");
                            assert_eq!(err.input(), "x1234");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::FailToParse(ParseError::InvalidInt {
                        option,
                        field,
                        input,
                        bit_size,
                        cause,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(field, "FooBar");
                        assert_eq!(input, "x1234");
                        assert_eq!(bit_size, 32);
                        assert_eq!(format!("{cause}"), "invalid digit found in string");
                        assert_eq!(format!("{cause:?}"), "ParseIntError { kind: InvalidDigit }");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let err0 = match "x1234".parse::<i32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidInt {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "FailToParse(InvalidInt { option: \"foo-bar\", field: \"FooBar\", input: \"x1234\", bit_size: 32, cause: ParseIntError { kind: InvalidDigit } })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let err0 = match "x1234".parse::<i32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidInt {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option arguments is invalid as an integer (option: \"foo-bar\", argument: \"x1234\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let err0 = match "x1234".parse::<i32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Box<dyn error::Error>> =
                    Err(Box::new(Error::FailToParse(ParseError::InvalidInt {
                        option: "foo-bar",
                        field: "FooBar",
                        input: "x1234",
                        bit_size: 32,
                        cause: Box::new(err0),
                    })));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(parse_err) => {
                                    assert_eq!(parse_err.option(), "foo-bar");
                                    assert_eq!(parse_err.field(), "FooBar");
                                    assert_eq!(parse_err.input(), "x1234");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(ParseError::InvalidInt {
                                    option,
                                    field,
                                    input,
                                    bit_size,
                                    cause,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*field, "FooBar");
                                    assert_eq!(*input, "x1234");
                                    assert_eq!(*bit_size, 32);
                                    assert_eq!(format!("{cause}"), "invalid digit found in string");
                                }
                                _ => assert!(false),
                            }
                        }
                    }
                }
            }
        }

        mod invalid_uint {
            use crate::Error;
            use crate::ParseError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let err0 = match "-1234".parse::<u32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidUint {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "-1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::FailToParse(err) => {
                            assert_eq!(err.option(), "foo-bar");
                            assert_eq!(err.field(), "FooBar");
                            assert_eq!(err.input(), "-1234");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::FailToParse(ParseError::InvalidUint {
                        option,
                        field,
                        input,
                        bit_size,
                        cause,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(field, "FooBar");
                        assert_eq!(input, "-1234");
                        assert_eq!(bit_size, 32);
                        assert_eq!(format!("{cause}"), "invalid digit found in string");
                        assert_eq!(format!("{cause:?}"), "ParseIntError { kind: InvalidDigit }");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let err0 = match "-1234".parse::<u32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidUint {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "-1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "FailToParse(InvalidUint { option: \"foo-bar\", field: \"FooBar\", input: \"-1234\", bit_size: 32, cause: ParseIntError { kind: InvalidDigit } })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let err0 = match "-1234".parse::<u32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidUint {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "-1234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(
                            format!("{err}"),
                            "The option arguments is invalid as an unsigned integer (option: \"foo-bar\", argument: \"-1234\")",
                        );
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let err0 = match "-1234".parse::<u32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Box<dyn error::Error>> =
                    Err(Box::new(Error::FailToParse(ParseError::InvalidUint {
                        option: "foo-bar",
                        field: "FooBar",
                        input: "-1234",
                        bit_size: 32,
                        cause: Box::new(err0),
                    })));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(parse_err) => {
                                    assert_eq!(parse_err.option(), "foo-bar");
                                    assert_eq!(parse_err.field(), "FooBar");
                                    assert_eq!(parse_err.input(), "-1234");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(ParseError::InvalidUint {
                                    option,
                                    field,
                                    input,
                                    bit_size,
                                    cause,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*field, "FooBar");
                                    assert_eq!(*input, "-1234");
                                    assert_eq!(*bit_size, 32);
                                    assert_eq!(format!("{cause}"), "invalid digit found in string");
                                }
                                _ => assert!(false),
                            }
                        }
                    }
                }
            }
        }

        mod invalid_float {
            use crate::Error;
            use crate::ParseError;
            use std::error;

            #[test]
            fn should_create_and_handle() {
                let err0 = match "x1.234".parse::<f32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidFloat {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1.234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => match err {
                        Error::FailToParse(err) => {
                            assert_eq!(err.option(), "foo-bar");
                            assert_eq!(err.field(), "FooBar");
                            assert_eq!(err.input(), "x1.234");
                        }
                        _ => assert!(false),
                    },
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(Error::FailToParse(ParseError::InvalidFloat {
                        option,
                        field,
                        input,
                        bit_size,
                        cause,
                    })) => {
                        assert_eq!(option, "foo-bar");
                        assert_eq!(field, "FooBar");
                        assert_eq!(input, "x1.234");
                        assert_eq!(bit_size, 32);
                        assert_eq!(format!("{cause}"), "invalid float literal");
                        assert_eq!(format!("{cause:?}"), "ParseFloatError { kind: Invalid }");
                    }
                    _ => assert!(false),
                }
            }

            #[test]
            fn should_write_for_debug() {
                let err0 = match "x1.234".parse::<f32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidFloat {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1.234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err:?}");
                        assert_eq!(
                            format!("{err:?}"),
                            "FailToParse(InvalidFloat { option: \"foo-bar\", field: \"FooBar\", input: \"x1.234\", bit_size: 32, cause: ParseFloatError { kind: Invalid } })",
                        );
                    }
                }
            }

            #[test]
            fn should_write_for_display() {
                let err0 = match "x1.234".parse::<f32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Error> = Err(Error::FailToParse(ParseError::InvalidFloat {
                    option: "foo-bar",
                    field: "FooBar",
                    input: "x1.234",
                    bit_size: 32,
                    cause: Box::new(err0),
                }));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        assert_eq!(format!("{err}"), "The option arguments is invalid as a floating point number (option: \"foo-bar\", argument: \"x1.234\")");
                    }
                }
            }

            #[test]
            fn should_handle_as_an_error() {
                let err0 = match "x1.234".parse::<f32>() {
                    Ok(_) => panic!(),
                    Err(err) => err,
                };

                let result: Result<(), Box<dyn error::Error>> =
                    Err(Box::new(Error::FailToParse(ParseError::InvalidFloat {
                        option: "foo-bar",
                        field: "FooBar",
                        input: "x1.234",
                        bit_size: 32,
                        cause: Box::new(err0),
                    })));
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        println!("{err}");
                        println!("{err:?}");
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(parse_err) => {
                                    assert_eq!(parse_err.option(), "foo-bar");
                                    assert_eq!(parse_err.field(), "FooBar");
                                    assert_eq!(parse_err.input(), "x1.234");
                                }
                                _ => assert!(false),
                            }
                        } else {
                            assert!(false);
                        }
                    }
                }
                match result {
                    Ok(_) => assert!(false),
                    Err(ref err) => {
                        if let Some(cliargs_err) = err.downcast_ref::<Error>() {
                            match cliargs_err {
                                Error::FailToParse(ParseError::InvalidFloat {
                                    option,
                                    field,
                                    input,
                                    bit_size,
                                    cause,
                                }) => {
                                    assert_eq!(*option, "foo-bar");
                                    assert_eq!(*field, "FooBar");
                                    assert_eq!(*input, "x1.234");
                                    assert_eq!(*bit_size, 32);
                                    assert_eq!(format!("{cause}"), "invalid float literal");
                                }
                                _ => assert!(false),
                            }
                        }
                    }
                }
            }
        }
    }
}
