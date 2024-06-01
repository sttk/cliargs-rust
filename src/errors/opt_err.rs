// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InvalidOption {
    OptionContainsInvalidChar { option: String },
    UnconfiguredOption { option: String },
    OptionNeedsArg { option: String, store_key: String },
    OptionTakesNoArg { option: String, store_key: String },
    OptionIsNotMultiArgs { option: String, store_key: String },
}

impl InvalidOption {
    pub fn option(&self) -> &str {
        return match self {
            InvalidOption::OptionContainsInvalidChar { option } => &option,
            InvalidOption::UnconfiguredOption { option } => &option,
            InvalidOption::OptionNeedsArg { option, .. } => &option,
            InvalidOption::OptionTakesNoArg { option, .. } => &option,
            InvalidOption::OptionIsNotMultiArgs { option, .. } => &option,
        };
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
            InvalidOption::OptionIsNotMultiArgs { option, .. } => write!(
                f,
                "The option cannot have multiple arguments (option: \"{}\")",
                option.escape_debug(),
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

    mod tests_of_option_is_not_multi_args {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotMultiArgs {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.option(), "foo-bar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionIsNotMultiArgs { option, store_key }) => {
                    assert_eq!(option, "foo-bar");
                    assert_eq!(store_key, "fooBar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotMultiArgs {
                option: "foo-bar".to_string(),
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionIsNotMultiArgs { option: \"foo-bar\", store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidOption> = Err(InvalidOption::OptionIsNotMultiArgs {
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
                Err(InvalidOption::OptionIsNotMultiArgs {
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
                            InvalidOption::OptionIsNotMultiArgs { option, store_key } => {
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
}
