// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

pub mod validators;

use crate::errors::InvalidOption;
use std::fmt;

/// Represents an option configuration for how to parse command line arguments.
///
/// And this is also used when creating the help text for command line arguments.
pub struct OptCfg {
    /// Is the key to store option value(s) in the option map in a `Cmd` instance.
    /// If this key is not specified or empty, the first element of the `names` field is used
    /// instead.
    pub store_key: String,

    /// Is the vector for specifying the option name and the aliases.
    /// The order of the `names` in this array are used in a help text.
    pub names: Vec<String>,

    /// Is the flag which allow the option to take option arguments.
    pub has_arg: bool,

    /// Is the flag which allow the option to take multiple option arguments.
    pub is_array: bool,

    /// Is the `Option` of the vector to specify default value(s) for when the comand option is not
    /// given in command line arguments.
    /// If this value is `None`, the default value(s) is not specified.
    pub defaults: Option<Vec<String>>,

    /// Is the string field to set the description of the option which is used in a help text.
    pub desc: String,

    /// Is the field to set a display string of the option argument(s) in a help text.
    /// An example of the display is like: `-o, --option <value>`.
    pub arg_in_help: String,

    /// Is the function pointer to validate the option argument(s).
    /// If the option argument is invalid, this funciton returns a
    /// `InvalidOption::OptionArgIsInvalid` instance.
    pub validator: fn(store_key: &str, name: &str, arg: &str) -> Result<(), InvalidOption>,
}

impl fmt::Debug for OptCfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("OptCfg")
            .field("store_key", &self.store_key)
            .field("names", &self.names)
            .field("has_arg", &self.has_arg)
            .field("is_array", &self.is_array)
            .field("defaults", &self.defaults)
            .field("desc", &self.desc)
            .field("arg_in_help", &self.arg_in_help)
            .finish()
    }
}

impl OptCfg {
    /// Creates a `OptCfg` instance in a manner similar to named parameters.
    ///
    /// ```rust
    ///   use cliargs::OptCfg;
    ///   use cliargs::OptCfgParam::{names, has_arg, desc};
    ///
    ///   let cfg = OptCfg::with(&[
    ///       names(&["foo-bar", "f"]),
    ///       has_arg(true),
    ///       desc("this is foo-bar option."),
    ///   ]);
    /// ```
    pub fn with(params: &[OptCfgParam]) -> OptCfg {
        let empty_string = String::from("");
        let empty_vec = Vec::with_capacity(0);

        let mut init = OptCfgInit {
            store_key: &empty_string,
            names: &empty_vec,
            has_arg: false,
            is_array: false,
            defaults: None,
            desc: &empty_string,
            arg_in_help: &empty_string,
            validator: |_, _, _| Ok(()),
        };

        for param in params.iter() {
            init.edit(param);
        }

        OptCfg {
            store_key: init.store_key.to_string(),
            names: init.names.iter().map(|s| s.to_string()).collect(),
            has_arg: init.has_arg,
            is_array: init.is_array,
            defaults: if let Some(sl) = init.defaults {
                Some(sl.iter().map(|s| s.to_string()).collect())
            } else {
                None
            },
            desc: init.desc.to_string(),
            arg_in_help: init.arg_in_help.to_string(),
            validator: init.validator,
        }
    }
}

struct OptCfgInit<'a> {
    store_key: &'a str,
    names: &'a [&'a str],
    has_arg: bool,
    is_array: bool,
    defaults: Option<&'a [&'a str]>,
    desc: &'a str,
    arg_in_help: &'a str,
    validator: fn(store_key: &str, name: &str, arg: &str) -> Result<(), InvalidOption>,
}

impl<'a> OptCfgInit<'a> {
    fn edit(&mut self, param: &'a OptCfgParam) {
        match param {
            OptCfgParam::store_key(s) => self.store_key = s,
            OptCfgParam::names(v) => self.names = v,
            OptCfgParam::has_arg(b) => self.has_arg = *b,
            OptCfgParam::is_array(b) => self.is_array = *b,
            OptCfgParam::defaults(v) => self.defaults = Some(v),
            OptCfgParam::desc(s) => self.desc = s,
            OptCfgParam::arg_in_help(s) => self.arg_in_help = s,
            OptCfgParam::validator(f) => self.validator = *f,
        }
    }
}

/// Enables to create a `OptCfg` instance in a manner similar to named parameters.
#[allow(non_camel_case_types)]
pub enum OptCfgParam<'a> {
    /// Holds the value for `OptCfg#store_key`.
    store_key(&'a str),

    /// Holds the value for `OptCfg#names`.
    names(&'a [&'a str]),

    /// Holds the value for `OptCfg#has_arg`.
    has_arg(bool),

    /// Holds the value for `OptCfg#is_array`.
    is_array(bool),

    /// Holds the value for `OptCfg#defaults`.
    defaults(&'a [&'a str]),

    /// Holds the value for `OptCfg#desc`.
    desc(&'a str),

    /// Holds the value for `OptCfg#arg_in_help`.
    arg_in_help(&'a str),

    /// Holds the value for `OptCfg#validator`.
    validator(fn(&str, &str, &str) -> Result<(), InvalidOption>),
}

#[cfg(test)]
mod tests_of_opt_cfg {
    use super::*;

    mod tests_of_named_param {
        use super::*;

        #[test]
        fn test_of_store_key() {
            let cfg = OptCfg::with(&[OptCfgParam::store_key("fooBar")]);

            assert_eq!(cfg.store_key, "fooBar");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_names() {
            let cfg = OptCfg::with(&[OptCfgParam::names(&["foo-bar", "f"])]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, vec!["foo-bar".to_string(), "f".to_string()]);
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_has_arg() {
            let cfg = OptCfg::with(&[OptCfgParam::has_arg(true)]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_is_array() {
            let cfg = OptCfg::with(&[OptCfgParam::is_array(true)]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_defaults() {
            let cfg = OptCfg::with(&[OptCfgParam::defaults(&["123", "456"])]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(
                cfg.defaults,
                Some(vec!["123".to_string(), "456".to_string()])
            );
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_desc() {
            let cfg = OptCfg::with(&[OptCfgParam::desc("description")]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "description");
            assert_eq!(cfg.arg_in_help, "");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_arg_in_help() {
            let cfg = OptCfg::with(&[OptCfgParam::arg_in_help("<num>")]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "<num>");

            assert_eq!((cfg.validator)("a", "b", "c"), Ok(()));
        }

        #[test]
        fn test_of_validator() {
            let cfg = OptCfg::with(&[OptCfgParam::validator(|key, name, arg| {
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key: key.to_string(),
                    option: name.to_string(),
                    opt_arg: arg.to_string(),
                    details: "fail to parse integer".to_string(),
                })
            })]);

            assert_eq!(cfg.store_key, "");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "");
            assert_eq!(cfg.arg_in_help, "");

            match (cfg.validator)("a", "b", "c") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "a");
                    assert_eq!(option, "b");
                    assert_eq!(opt_arg, "c");
                    assert_eq!(details, "fail to parse integer");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn test_of_debug() {
            let cfg = OptCfg {
                store_key: "fooBar".to_string(),
                names: vec!["foo-bar".to_string(), "baz".to_string()],
                has_arg: true,
                is_array: true,
                defaults: Some(vec![123.to_string(), 456.to_string()]),
                desc: "option description".to_string(),
                arg_in_help: "<num>".to_string(),
                validator: |_, _, _| Ok(()),
            };

            assert_eq!(
                format!("{cfg:?}"),
                "OptCfg { store_key: \"fooBar\", names: [\"foo-bar\", \"baz\"], has_arg: true, \
                 is_array: true, defaults: Some([\"123\", \"456\"]), desc: \"option description\", \
                 arg_in_help: \"<num>\" }"
            );
        }
    }
}
