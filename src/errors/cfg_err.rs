// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InvalidConfig {
    StoreKeyIsDuplicated { store_key: String },
    ConfigIsMultiArgsButHasNoArg { store_key: String },
    ConfigHasDefaultsButHasNoArg { store_key: String },
    OptionNameIsDuplicated { store_key: String, name: String },
}

impl InvalidConfig {
    pub fn store_key(&self) -> &str {
        return match self {
            InvalidConfig::StoreKeyIsDuplicated { store_key } => store_key,
            InvalidConfig::ConfigIsMultiArgsButHasNoArg { store_key } => store_key,
            InvalidConfig::ConfigHasDefaultsButHasNoArg { store_key } => store_key,
            InvalidConfig::OptionNameIsDuplicated { store_key, .. } => store_key,
        };
    }
}

impl fmt::Display for InvalidConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            InvalidConfig::StoreKeyIsDuplicated { store_key } => write!(
                f,
                "The store key is duplicated (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            InvalidConfig::ConfigIsMultiArgsButHasNoArg { store_key } => write!(
                f,
                "The configuration is specified both having multiple arguments and having no argument (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            InvalidConfig::ConfigHasDefaultsButHasNoArg { store_key } => write!(
                f,
                "The configuration is specified both default argument(s) and having no argument (store_key: \"{}\")",
                store_key.escape_debug()
            ),
            InvalidConfig::OptionNameIsDuplicated { store_key, name } => write!(
                f,
                "The option name in the configuration is duplicated (store_key: \"{}\", name: \"{}\")",
                store_key.escape_debug(),
                name.escape_debug()
            ),
        }
    }
}

impl error::Error for InvalidConfig {}

#[cfg(test)]
mod tests_of_invalid_config {
    use super::*;

    mod store_key_is_duplicated {
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.store_key(), "fooBar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidConfig::StoreKeyIsDuplicated { store_key }) => {
                    assert_eq!(store_key, "fooBar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "StoreKeyIsDuplicated { store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::StoreKeyIsDuplicated {
                store_key: "fooBar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The store key is duplicated (store_key: \"fooBar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidConfig> {
                Err(InvalidConfig::StoreKeyIsDuplicated {
                    store_key: "fooBar".to_string(),
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
                    if let Some(cfg_err) = err.downcast_ref::<InvalidConfig>() {
                        assert_eq!(cfg_err.store_key(), "fooBar");
                        match cfg_err {
                            InvalidConfig::StoreKeyIsDuplicated { store_key } => {
                                assert_eq!(*store_key, "fooBar");
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
        use super::*;

        #[test]
        fn should_create_and_handle() {
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigIsMultiArgsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => assert_eq!(err.store_key(), "fooBar"),
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidConfig::ConfigIsMultiArgsButHasNoArg { store_key }) => {
                    assert_eq!(store_key, "fooBar");
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigIsMultiArgsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "ConfigIsMultiArgsButHasNoArg { store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigIsMultiArgsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The configuration is specified both having multiple arguments and having no argument (store_key: \"fooBar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidConfig> {
                Err(InvalidConfig::ConfigIsMultiArgsButHasNoArg {
                    store_key: "fooBar".to_string(),
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
                    if let Some(cfg_err) = err.downcast_ref::<InvalidConfig>() {
                        assert_eq!(cfg_err.store_key(), "fooBar");
                        match cfg_err {
                            InvalidConfig::ConfigIsMultiArgsButHasNoArg { store_key } => {
                                assert_eq!(*store_key, "fooBar");
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
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(err.store_key(), "fooBar");
                }
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidConfig::ConfigHasDefaultsButHasNoArg { store_key }) => {
                    assert_eq!(store_key, "fooBar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "ConfigHasDefaultsButHasNoArg { store_key: \"fooBar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidConfig> =
                Err(InvalidConfig::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
                });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The configuration is specified both default argument(s) and having no argument (store_key: \"fooBar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidConfig> {
                Err(InvalidConfig::ConfigHasDefaultsButHasNoArg {
                    store_key: "fooBar".to_string(),
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
                    if let Some(cfg_err) = err.downcast_ref::<InvalidConfig>() {
                        assert_eq!(cfg_err.store_key(), "fooBar");
                        match cfg_err {
                            InvalidConfig::ConfigHasDefaultsButHasNoArg { store_key } => {
                                assert_eq!(*store_key, "fooBar");
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
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(err.store_key(), "fooBar");
                }
            }
            match result {
                Ok(_) => assert!(false),
                Err(InvalidConfig::OptionNameIsDuplicated { store_key, name }) => {
                    assert_eq!(store_key, "fooBar");
                    assert_eq!(name, "foo-bar");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_write_for_debug() {
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    println!("{err}");
                    assert_eq!(
                        format!("{err:?}"),
                        "OptionNameIsDuplicated { store_key: \"fooBar\", name: \"foo-bar\" }",
                    );
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let result: Result<(), InvalidConfig> = Err(InvalidConfig::OptionNameIsDuplicated {
                store_key: "fooBar".to_string(),
                name: "foo-bar".to_string(),
            });
            match result {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err}");
                    assert_eq!(
                        format!("{err}"),
                        "The option name in the configuration is duplicated (store_key: \"fooBar\", name: \"foo-bar\")",
                    );
                }
            }
        }

        #[test]
        fn should_handle_as_std_error() {
            fn returns_error() -> Result<(), InvalidConfig> {
                Err(InvalidConfig::OptionNameIsDuplicated {
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
                    //println!("{err}");
                    //println!("{err:?}");
                    if let Some(cfg_err) = err.downcast_ref::<InvalidConfig>() {
                        assert_eq!(cfg_err.store_key(), "fooBar");
                        match cfg_err {
                            InvalidConfig::OptionNameIsDuplicated { store_key, name } => {
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
}
