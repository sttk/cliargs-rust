// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::error;
use std::fmt;

/// The enum type for errors of options or option arguments.
///
/// This enum type has `option()` method, which makes it possible to handle option-related errors
/// in a unified manner.
#[derive(Debug, PartialEq)]
pub enum InvalidOption {
    /// Indicates that the name of an option is using invalid characters.
    /// This error occurs if the name contains symbols or starts with a symbol or number.
    OptionContainsInvalidChar {
        /// The option name that caused this error.
        option: String,
    },

    /// Indicates that the option with the specified name does not exist in the option
    /// configurations.
    UnconfiguredOption {
        /// The option name that caused this error.
        option: String,
    },

    /// Indicates that the option requires arguments in the configuration, but no argument is
    /// specified.
    OptionNeedsArg {
        /// The option name that caused this error.
        option: String,

        /// The store key of the specified option in the configuration.
        store_key: String,
    },

    /// Indicates that the option is not suppoesed to take an argument in the configuration, but
    /// an argument is specified.
    OptionTakesNoArg {
        /// The option name that caused this error.
        option: String,

        /// The store key of the specified option in the configuration.
        store_key: String,
    },

    /// Indicates that the option is supposed to take one argument in the configuration, but
    /// multiple arguments are specified.
    OptionIsNotArray {
        /// The option name that caused this error.
        option: String,

        /// The store key of the specified option in the configuration.
        store_key: String,
    },

    /// Indicates that there are duplicated store keys among multiple configurations.
    StoreKeyIsDuplicated {
        /// The store key that caused this error.
        store_key: String,

        /// The first name of the option configuration.
        name: String,
    },

    /// Indicates that an option configuration contradicts that the option can take multiple
    /// arguments (`.is_array == true`) though it does not take option arguments
    /// (`.has_arg == false`).
    ConfigIsArrayButHasNoArg {
        /// The store key of the option configuration that caused this error.
        store_key: String,

        /// The first name of the option configuration.
        name: String,
    },

    /// Indicates that an option configuration contradicts that the default arguments (`.defaults`)
    /// is not empty though it does not take option arguments (`.has_arg == false`).
    ConfigHasDefaultsButHasNoArg {
        /// The store key of the option configuration that caused this error.
        store_key: String,

        /// The first name of the option configuration.
        name: String,
    },

    /// Indicates that there are duplicated opton names among the option configurations.
    OptionNameIsDuplicated {
        /// The store key of the option configuration that caused this error.
        store_key: String,

        /// The duplicated name of the option configuration.
        name: String,
    },

    /// Indicates that the option argument is invalidated by the validator in the option
    /// configuration.
    OptionArgIsInvalid {
        /// The store key of the option configuration that caused this error.
        store_key: String,

        /// The option name that caused this error.
        option: String,

        /// The option argument that was validated.
        opt_arg: String,

        /// The details for the invalidation.
        details: String,
    },
}

impl InvalidOption {
    /// Returns the name of the option that caused the error.
    pub fn option(&self) -> &str {
        match self {
            InvalidOption::OptionContainsInvalidChar { option } => option,
            InvalidOption::UnconfiguredOption { option } => option,
            InvalidOption::OptionNeedsArg { option, .. } => option,
            InvalidOption::OptionTakesNoArg { option, .. } => option,
            InvalidOption::OptionIsNotArray { option, .. } => option,
            InvalidOption::StoreKeyIsDuplicated { name, .. } => name,
            InvalidOption::ConfigIsArrayButHasNoArg { name, .. } => name,
            InvalidOption::ConfigHasDefaultsButHasNoArg { name, .. } => name,
            InvalidOption::OptionNameIsDuplicated { name, .. } => name,
            InvalidOption::OptionArgIsInvalid { option, .. } => option,
        }
    }
}

impl fmt::Display for InvalidOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            InvalidOption::OptionContainsInvalidChar { option } => write!(
                f,
                "The option contains invalid character (option: \"{}\")",
                option.escape_debug(),
            ),
            InvalidOption::UnconfiguredOption { option } => write!(
                f,
                "The option is not specified in configurations (option: \"{}\")",
                option.escape_debug(),
            ),
            InvalidOption::OptionNeedsArg { option, .. } => write!(
                f,
                "The option needs argument(s) (option: \"{}\")",
                option.escape_debug(),
            ),
            InvalidOption::OptionTakesNoArg { option, .. } => write!(
                f,
                "The option takes no argument (option: \"{}\")",
                option.escape_debug(),
            ),
            InvalidOption::OptionIsNotArray { option, .. } => write!(
                f,
                "The option cannot have multiple arguments (option: \"{}\")",
                option.escape_debug(),
            ),
            InvalidOption::OptionArgIsInvalid {
                option,
                opt_arg,
                details,
                ..
            } => write!(
                f,
                "The option argument \"{}\" is invalid because: {} (option: \"{}\")",
                opt_arg.escape_debug(),
                details.escape_debug(),
                option.escape_debug(),
            ),
            _ => write!(
                f,
                "The option configuration is invalid (option: \"{}\")",
                self.option(),
            ),
        }
    }
}

impl error::Error for InvalidOption {}

#[cfg(test)]
mod tests_of_invalid_option {
    use super::*;

    mod tests_of_option_contains_invalid_char {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionContainsInvalidChar {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "foo-bar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionContainsInvalidChar {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionContainsInvalidChar { option: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionContainsInvalidChar {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option contains invalid character (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionContainsInvalidChar {
                    option: "b@z".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    //println!("{err:?}");
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "b@z");
                        match opt_err {
                            InvalidOption::OptionContainsInvalidChar { option } => {
                                assert_eq!(*option, "b@z");
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

    mod tests_of_unconfigured_option {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::UnconfiguredOption {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::UnconfiguredOption { option }) => {
                    assert_eq!(option, "foo-bar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::UnconfiguredOption {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "UnconfiguredOption { option: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::UnconfiguredOption {
                option: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option is not specified in configurations (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionContainsInvalidChar {
                    option: "b@z".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    println!("{err:?}");
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "b@z");
                        match opt_err {
                            InvalidOption::OptionContainsInvalidChar { option } => {
                                assert_eq!(*option, "b@z");
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

    mod tests_of_option_needs_arg {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNeedsArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionNeedsArg { option, store_key }) => {
                    assert_eq!(option, "foo-bar");
                    assert_eq!(store_key, "fooBar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNeedsArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionNeedsArg { option: \"foo-bar\", store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNeedsArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option needs argument(s) (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionNeedsArg {
                    option: "b@z".to_string(),
                    store_key: "BAZ".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    println!("{err:?}");
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "b@z");
                        match opt_err {
                            InvalidOption::OptionNeedsArg { option, store_key } => {
                                assert_eq!(*option, "b@z");
                                assert_eq!(*store_key, "BAZ");
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

    mod tests_of_option_takes_no_arg {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionTakesNoArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionTakesNoArg { option, store_key }) => {
                    assert_eq!(option, "foo-bar");
                    assert_eq!(store_key, "fooBar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionTakesNoArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionTakesNoArg { option: \"foo-bar\", store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionTakesNoArg {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option takes no argument (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionTakesNoArg {
                    option: "b@z".to_string(),
                    store_key: "BAZ".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    println!("{err:?}");
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "b@z");
                        match opt_err {
                            InvalidOption::OptionTakesNoArg { option, store_key } => {
                                assert_eq!(*option, "b@z");
                                assert_eq!(*store_key, "BAZ");
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

    mod tests_of_option_is_not_array {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotArray {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionIsNotArray { option, store_key }) => {
                    assert_eq!(option, "foo-bar");
                    assert_eq!(store_key, "fooBar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotArray {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionIsNotArray { option: \"foo-bar\", store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotArray {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option cannot have multiple arguments (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionIsNotArray {
                    option: "b@z".to_string(),
                    store_key: "BAZ".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    println!("{err:?}");
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "b@z");
                        match opt_err {
                            InvalidOption::OptionIsNotArray { option, store_key } => {
                                assert_eq!(*option, "b@z");
                                assert_eq!(*store_key, "BAZ");
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

    mod store_key_is_duplicated {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::StoreKeyIsDuplicated { store_key, name }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(name, "foo-bar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err:?}"),
                        "StoreKeyIsDuplicated { store_key: \"fooBar\", name: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err}"),
                        "The option configuration is invalid (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::StoreKeyIsDuplicated {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "foo-bar");
                        match opt_err {
                            InvalidOption::StoreKeyIsDuplicated { store_key, name } => {
                                assert_eq!(*store_key, "fooBar");
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

    mod config_is_array_but_has_no_arg {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::ConfigIsArrayButHasNoArg {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::ConfigIsArrayButHasNoArg { store_key, name }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(name, "foo-bar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::ConfigIsArrayButHasNoArg {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err:?}"),
                        "ConfigIsArrayButHasNoArg { store_key: \"fooBar\", name: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::ConfigIsArrayButHasNoArg {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err}"),
                        "The option configuration is invalid (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::ConfigIsArrayButHasNoArg {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "foo-bar");
                        match opt_err {
                            InvalidOption::ConfigIsArrayButHasNoArg { store_key, name } => {
                                assert_eq!(*store_key, "fooBar");
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

    mod config_has_defaults_but_has_no_arg {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> =
                Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(err.option(), "foo-bar");
                }
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::ConfigHasDefaultsButHasNoArg { store_key, name }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(name, "foo-bar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> =
                Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err:?}"),
                        "ConfigHasDefaultsButHasNoArg { store_key: \"fooBar\", name: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> =
                Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err}"),
                        "The option configuration is invalid (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "foo-bar");
                        match opt_err {
                            InvalidOption::ConfigHasDefaultsButHasNoArg { store_key, name } => {
                                assert_eq!(*store_key, "fooBar");
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

    mod option_name_is_duplicated {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(err.option(), "foo-bar");
                }
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionNameIsDuplicated { store_key, name }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(name, "foo-bar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionNameIsDuplicated { store_key: \"fooBar\", name: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err}"),
                        "The option configuration is invalid (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionNameIsDuplicated {
                    store_key: "fooBar".to_string(),
                    name: "foo-bar".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "foo-bar");
                        match opt_err {
                            InvalidOption::OptionNameIsDuplicated { store_key, name } => {
                                assert_eq!(*store_key, "fooBar");
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

    mod option_arg_is_invalid {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionArgIsInvalid {
                store_key: "fooBar".to_string(),
                option: "foo-bar".to_string(),
                opt_arg: "x123".to_string(),
                details: "illegal number format.".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(err.option(), "foo-bar");
                }
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "x123");
                    assert_eq!(details, "illegal number format.");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionArgIsInvalid {
                store_key: "fooBar".to_string(),
                option: "foo-bar".to_string(),
                opt_arg: "x123".to_string(),
                details: "illegal number format.".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionArgIsInvalid { store_key: \"fooBar\", option: \"foo-bar\", \
                         opt_arg: \"x123\", details: \"illegal number format.\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionArgIsInvalid {
                store_key: "fooBar".to_string(),
                option: "foo-bar".to_string(),
                opt_arg: "x123".to_string(),
                details: "illegal number format.".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(
                        format!("{err}"),
                        "The option argument \"x123\" is invalid because: illegal number format. \
                         (option: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidOption> {
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key: "fooBar".to_string(),
                    option: "foo-bar".to_string(),
                    opt_arg: "x123".to_string(),
                    details: "illegal number format.".to_string(),
                })
            }
            fn returns_dyn_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }
            match returns_dyn_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    if let Some(opt_err) = err.downcast_ref::<InvalidOption>() {
                        assert_eq!(opt_err.option(), "foo-bar");
                        match opt_err {
                            InvalidOption::OptionArgIsInvalid {
                                store_key,
                                option,
                                opt_arg,
                                details,
                            } => {
                                assert_eq!(*store_key, "fooBar");
                                assert_eq!(*option, "foo-bar");
                                assert_eq!(*opt_arg, "x123");
                                assert_eq!(*details, "illegal number format.");
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
