// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::parse_args;
use crate::errors::InvalidOption;
use crate::Cmd;
use crate::OptCfg;
use std::collections::HashMap;

impl<'b, 'a> Cmd<'a> {
    /// Parses command line arguments with option configurations.
    ///
    /// This method divides command line arguments to command arguments and options.
    /// And an option consists of a name and an option argument.
    /// Options are divided to long format options and short format options.
    /// About long/short format options, since they are same with `parse` method, see the comment
    /// of that method.
    ///
    /// This method allows only options declared in option configurations, basically.
    /// An option configuration has fields: `store_key`, `names`, `has_arg`, `is_array`,
    /// `defaults`, `desc`, `arg_in_help`, and `validator`.
    ///
    /// When an option matches one of the `names` in the option configurations, the option is
    /// registered into [Cmd] with `store_key`.
    /// If both `has_arg` and `is_array` is false, the optioin can have only one option argument,
    /// otherwise the option cannot have option arguments.
    /// If `defaults` field is specified and no option value is given in command line arguments,
    /// the value of `defaults` is set as the option arguments.
    ///
    /// If options not declared in option configurations are given in command line arguments, this
    /// method basicaly returns [InvalidOption::UnconfiguredOption] error.
    /// However, if you want to allow other options, add an option configuration of which
    /// `store_key` or the first element of `names` is "*".
    ///
    /// The ownership of the vector of option configurations which is passed as an argument of
    /// this method is moved to this method and set into this [Cmd] instance.
    /// It can be retrieved with its method: [Cmd::opt_cfgs].
    ///
    /// ```
    /// use cliargs::{Cmd, OptCfg};
    /// use cliargs::OptCfgParam::{names, has_arg, defaults, validator, desc, arg_in_help};
    /// use cliargs::validators::validate_number;
    /// use cliargs::errors::InvalidOption;
    ///
    /// let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
    /// let opt_cfgs = vec![
    ///     OptCfg::with([
    ///         names(&["foo-bar"]),
    ///         desc("This is description of foo-bar."),
    ///     ]),
    ///     OptCfg::with([
    ///         names(&["baz", "z"]),
    ///         has_arg(true),
    ///         defaults(&["1"]),
    ///         desc("This is description of baz."),
    ///         arg_in_help("<num>"),
    ///         validator(validate_number::<u32>),
    ///     ]),
    /// ];
    ///
    /// match cmd.parse_with(opt_cfgs) {
    ///     Ok(_) => { /* ... */ },
    ///     Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
    ///     Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
    ///     Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
    ///     Err(err) => panic!("Invalid option: {}", err.option()),
    /// }
    /// ```
    pub fn parse_with(&mut self, opt_cfgs: Vec<OptCfg>) -> Result<(), InvalidOption> {
        let result = self.parse_args_with(&opt_cfgs, false, self.is_after_end_opt);
        self.cfgs = opt_cfgs;
        result?;
        Ok(())
    }

    /// Parses command line arguments with option configurations but stops parsing when
    /// encountering first command argument.
    ///
    /// This method creates and returns a new [Cmd] instance that holds the command line arguments
    /// starting from the first command argument.
    ///
    /// This method parses command line arguments in the same way as the [Cmd::parse_with] method,
    /// except that it only parses the command line arguments before the first command argument.
    ///
    /// The ownership of the vector of option configurations which is passed as an argument of
    /// this method is moved to this method and set into this [Cmd] instance.
    /// It can be retrieved with its method: [Cmd::opt_cfgs].
    ///
    /// ```
    /// use cliargs::{Cmd, OptCfg};
    /// use cliargs::OptCfgParam::{names, has_arg, defaults, validator, desc, arg_in_help};
    /// use cliargs::validators::validate_number;
    /// use cliargs::errors::InvalidOption;
    ///
    /// let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
    /// let opt_cfgs = vec![
    ///     OptCfg::with([
    ///         names(&["foo-bar"]),
    ///         desc("This is description of foo-bar."),
    ///     ]),
    ///     OptCfg::with([
    ///         names(&["baz", "z"]),
    ///         has_arg(true),
    ///         defaults(&["1"]),
    ///         desc("This is description of baz."),
    ///         arg_in_help("<num>"),
    ///         validator(validate_number::<u32>),
    ///     ]),
    /// ];
    ///
    /// match cmd.parse_until_sub_cmd_with(opt_cfgs) {
    ///     Ok(Some(mut sub_cmd)) => {
    ///         let sub_cmd_name = sub_cmd.name();
    ///         match sub_cmd.parse() {
    ///             Ok(_) => { /* ... */ },
    ///             Err(err) => panic!("Invalid option: {}", err.option()),
    ///         }
    ///     },
    ///     Ok(None) => { /* ... */ },
    ///     Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
    ///     Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
    ///     Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
    ///     Err(err) => panic!("Invalid option: {}", err.option()),
    /// }
    /// ```
    pub fn parse_until_sub_cmd_with(
        &mut self,
        opt_cfgs: Vec<OptCfg>,
    ) -> Result<Option<Cmd<'b>>, InvalidOption> {
        match self.parse_args_with(&opt_cfgs, true, self.is_after_end_opt) {
            Ok(Some((idx, is_after_end_opt))) => {
                self.cfgs = opt_cfgs;
                return Ok(Some(self.sub_cmd(idx, is_after_end_opt)));
            }
            Ok(None) => {
                self.cfgs = opt_cfgs;
                return Ok(None);
            }
            Err(err) => {
                self.cfgs = opt_cfgs;
                return Err(err);
            }
        }
    }

    fn parse_args_with(
        &mut self,
        opt_cfgs: &Vec<OptCfg>,
        until_1st_arg: bool,
        is_after_end_opt: bool,
    ) -> Result<Option<(usize, bool)>, InvalidOption> {
        let mut cfg_map = HashMap::<&str, usize>::new();
        let mut opt_map = HashMap::<&str, ()>::new();

        const ANY_OPT: &str = "*";
        let mut has_any_opt = is_after_end_opt;

        for (i, cfg) in opt_cfgs.iter().enumerate() {
            let names: Vec<&String> = cfg.names.iter().filter(|nm| !nm.is_empty()).collect();

            let store_key: &str = if cfg.store_key.is_empty() && !names.is_empty() {
                &names[0]
            } else {
                &cfg.store_key
            };

            if store_key.is_empty() {
                continue;
            }

            if store_key == ANY_OPT {
                has_any_opt = true;
                continue;
            }

            let first_name = if names.is_empty() {
                store_key
            } else {
                &names[0]
            };

            if opt_map.contains_key(store_key) {
                return Err(InvalidOption::StoreKeyIsDuplicated {
                    store_key: store_key.to_string(),
                    name: first_name.to_string(),
                });
            }
            opt_map.insert(store_key, ());

            if !cfg.has_arg {
                if cfg.is_array {
                    return Err(InvalidOption::ConfigIsArrayButHasNoArg {
                        store_key: store_key.to_string(),
                        name: first_name.to_string(),
                    });
                }
                if let Some(vec) = &cfg.defaults {
                    if !vec.is_empty() {
                        return Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                            store_key: store_key.to_string(),
                            name: first_name.to_string(),
                        });
                    }
                }
            }

            if names.is_empty() {
                cfg_map.insert(first_name, i);
            } else {
                for name in names.iter() {
                    if cfg_map.contains_key(name.as_str()) {
                        return Err(InvalidOption::OptionNameIsDuplicated {
                            store_key: store_key.to_string(),
                            name: name.to_string(),
                        });
                    }
                    cfg_map.insert(name, i);
                }
            }
        }

        if self._num_of_args == 0 {
            return Ok(None);
        }

        let take_opt_args = |opt: &str| {
            if let Some(i) = cfg_map.get(opt) {
                return opt_cfgs[*i].has_arg;
            }
            false
        };

        let collect_args = |arg| {
            self.args.push(arg);
        };

        let mut str_refs: Vec<&'a str> = Vec::with_capacity(opt_cfgs.len());

        let collect_opts = |name: &'a str, arg_op: Option<&'a str>| {
            if let Some(i) = cfg_map.get(name) {
                let cfg = &opt_cfgs[*i];

                let store_key: &str = if !cfg.store_key.is_empty() {
                    &cfg.store_key
                } else {
                    if let Some(name) = cfg.names.iter().find(|nm| !nm.is_empty()) {
                        name
                    } else {
                        ""
                    }
                };

                if let Some(arg) = arg_op {
                    if !cfg.has_arg {
                        return Err(InvalidOption::OptionTakesNoArg {
                            option: name.to_string(),
                            store_key: store_key.to_string(),
                        });
                    }

                    if let Some(vec) = self.opts.get_mut(store_key) {
                        if !vec.is_empty() {
                            if !cfg.is_array {
                                return Err(InvalidOption::OptionIsNotArray {
                                    option: name.to_string(),
                                    store_key: store_key.to_string(),
                                });
                            }
                        }

                        (cfg.validator)(store_key, name, arg)?;
                        vec.push(arg);
                    } else {
                        (cfg.validator)(store_key, name, arg)?;

                        let string = String::from(store_key);
                        let str: &'a str = string.leak();
                        str_refs.push(str);
                        self.opts.insert(str, vec![arg]);
                    }
                } else {
                    if cfg.has_arg {
                        return Err(InvalidOption::OptionNeedsArg {
                            option: name.to_string(),
                            store_key: store_key.to_string(),
                        });
                    }

                    if let None = self.opts.get_mut(store_key) {
                        let string = String::from(store_key);
                        let str: &'a str = string.leak();
                        str_refs.push(str);
                        self.opts.insert(str, vec![]);
                    }
                }

                Ok(())
            } else {
                if !has_any_opt {
                    return Err(InvalidOption::UnconfiguredOption {
                        option: String::from(name),
                    });
                }

                if let Some(arg) = arg_op {
                    if let Some(vec) = self.opts.get_mut(name) {
                        vec.push(arg);
                    } else {
                        self.opts.insert(name, vec![arg]);
                    }
                } else {
                    self.opts.insert(name, Vec::with_capacity(0));
                }

                Ok(())
            }
        };

        let result = parse_args(
            &self._leaked_strs[1..(self._num_of_args)],
            collect_args,
            collect_opts,
            take_opt_args,
            until_1st_arg,
            is_after_end_opt,
        );

        for str_ref in str_refs {
            self._leaked_strs.push(str_ref);
        }

        for cfg in opt_cfgs.iter() {
            let store_key: &str = if !cfg.store_key.is_empty() {
                &cfg.store_key
            } else {
                if let Some(name) = cfg.names.iter().find(|nm| !nm.is_empty()) {
                    name
                } else {
                    ""
                }
            };

            if store_key.is_empty() {
                continue;
            }

            if store_key == ANY_OPT {
                continue;
            }

            if let None = self.opts.get_mut(store_key) {
                if let Some(def_vec) = &cfg.defaults {
                    let string = String::from(store_key);
                    let key: &'a str = string.leak();
                    self._leaked_strs.push(key);
                    let vec = self.opts.entry(key).or_insert(Vec::new());

                    for def_val in def_vec.iter() {
                        let string = String::from(def_val);
                        let arg: &'a str = string.leak();
                        self._leaked_strs.push(arg);
                        vec.push(arg);
                    }
                }
            }
        }

        if let Some((idx, is_after_end_opt)) = result? {
            return Ok(Some((idx + 1, is_after_end_opt))); // +1, because _parse_args parses from 1
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests_of_parse_with {
    use super::*;
    use crate::OptCfgParam::*;

    #[test]
    fn zero_cfg_and_zero_arg() {
        let opt_cfgs = vec![];

        let mut cmd = Cmd::with_strings(["app".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 0);
    }

    #[test]
    fn zero_cfg_and_one_command_arg() {
        let opt_cfgs = vec![];

        let mut cmd = Cmd::with_strings(["/path/to/app".to_string(), "foo-bar".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &["foo-bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 0);
    }

    #[test]
    fn zero_cfg_and_one_long_opt() {
        let opt_cfgs = vec![];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "--foo-bar".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => assert!(false),
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 0);
    }

    #[test]
    fn zero_cfg_and_one_short_opt() {
        let opt_cfgs = vec![];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-f".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => assert!(false),
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "f");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 0);
    }

    #[test]
    fn one_cfg_and_zero_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_and_one_cmd_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "foo-bar".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &["foo-bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_and_one_long_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "--foo-bar".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_and_one_short_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_and_one_different_long_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "--bar-foo".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "bar-foo");
            }
            Err(_) => {}
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.has_opt("bar-foo"), false);
        assert_eq!(cmd.opt_arg("bar-foo"), None);
        assert_eq!(cmd.opt_args("bar-foo"), None);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_and_one_different_short_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-b".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "b");
            }
            Err(_) => {}
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.has_opt("b"), false);
        assert_eq!(cmd.opt_arg("b"), None);
        assert_eq!(cmd.opt_args("b"), None);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn any_opt_cfg_and_one_different_long_opt() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"])]),
            OptCfg::with([names(&["*"])]),
        ];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "--bar-foo".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.has_opt("bar-foo"), true);
        assert_eq!(cmd.opt_arg("bar-foo"), None);
        assert_eq!(cmd.opt_args("bar-foo"), Some(&[] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["*".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn any_opt_cfg_and_one_different_short_opt() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"])]), OptCfg::with([names(&["*"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-b".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(true),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.has_opt("b"), true);
        assert_eq!(cmd.opt_arg("b"), None);
        assert_eq!(cmd.opt_args("b"), Some(&[] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["*".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_arg_and_one_long_opt_has_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"]), has_arg(true)])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "ABC".to_string(),
        ]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("ABC"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_arg_and_one_short_opt_has_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"]), has_arg(true)])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string(), "ABC".to_string()]);

        match cmd.parse_with(opt_cfgs) {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), Some("ABC"));
        assert_eq!(cmd.opt_args("f"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_arg_but_one_long_opt_has_no_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"]), has_arg(true)])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo-bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo-bar");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionNeedsArg {
                store_key: sk,
                option,
            }) => {
                assert_eq!(sk, "foo-bar");
                assert_eq!(option, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_arg_but_one_short_opt_has_no_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"]), has_arg(true)])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(err) => {
                assert_eq!(err.option(), "f");
                match err {
                    InvalidOption::OptionNeedsArg {
                        store_key: sk,
                        option,
                    } => {
                        assert_eq!(sk, "f");
                        assert_eq!(option, "f");
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_no_arg_but_one_long_opt_has_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "ABC".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &["ABC"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo-bar=ABC".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(err) => {
                assert_eq!(err.option(), "foo-bar");
                match err {
                    InvalidOption::OptionTakesNoArg {
                        store_key: sk,
                        option,
                    } => {
                        assert_eq!(sk, "foo-bar");
                        assert_eq!(option, "foo-bar");
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "--foo-bar".to_string(), "".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &[""] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo-bar=".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(err) => {
                assert_eq!(err.option(), "foo-bar");
                match err {
                    InvalidOption::OptionTakesNoArg {
                        store_key: sk,
                        option,
                    } => {
                        assert_eq!(sk, "foo-bar");
                        assert_eq!(option, "foo-bar");
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_no_arg_but_one_short_opt_has_arg() {
        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string(), "ABC".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &["ABC"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f=ABC".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(err) => {
                assert_eq!(err.option(), "f");
                match err {
                    InvalidOption::OptionTakesNoArg {
                        store_key: sk,
                        option,
                    } => {
                        assert_eq!(sk, "f");
                        assert_eq!(option, "f");
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string(), "".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &[""] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());

        let opt_cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f=".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(err) => {
                assert_eq!(err.option(), "f");
                match err {
                    InvalidOption::OptionTakesNoArg {
                        store_key: sk,
                        option,
                    } => {
                        assert_eq!(sk, "f");
                        assert_eq!(option, "f");
                    }
                    _ => {}
                }
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_no_arg_but_is_array() {
        let opt_cfgs = vec![OptCfg::with([
            names(&["foo-bar"]),
            has_arg(false),
            is_array(true),
        ])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo-bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo-bar");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::ConfigIsArrayButHasNoArg {
                store_key: sk,
                name,
            }) => {
                assert_eq!(sk, "foo-bar");
                assert_eq!(name, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_is_array_and_opt_has_no_arg() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true), is_array(true)]),
            OptCfg::with([names(&["f"]), has_arg(true), is_array(true)]),
        ];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo-bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo-bar");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionNeedsArg {
                store_key: sk,
                option,
            }) => {
                assert_eq!(sk, "foo-bar");
                assert_eq!(option, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());

        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true), is_array(true)]),
            OptCfg::with([names(&["f"]), has_arg(true), is_array(true)]),
        ];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-f".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "f");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionNeedsArg {
                store_key: sk,
                option,
            }) => {
                assert_eq!(sk, "f");
                assert_eq!(option, "f");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_is_array_and_opt_has_one_arg() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true), is_array(true)]),
            OptCfg::with([names(&["f"]), has_arg(true), is_array(true)]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "ABC".to_string(),
            "-f".to_string(),
            "DEF".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("ABC"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), Some("DEF"));
        assert_eq!(cmd.opt_args("f"), Some(&["DEF"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_is_array_and_opt_has_multiple_args() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true), is_array(true)]),
            OptCfg::with([names(&["f"]), has_arg(true), is_array(true)]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "ABC".to_string(),
            "-f".to_string(),
            "DEF".to_string(),
            "--foo-bar".to_string(),
            "GHI".to_string(),
            "-f".to_string(),
            "JKL".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("ABC"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["ABC", "GHI"] as &[&str]));
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), Some("DEF"));
        assert_eq!(cmd.opt_args("f"), Some(&["DEF", "JKL"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_has_name_and_aliase_and_arg_matches_them() {
        let opt_cfgs = vec![OptCfg::with([
            names(&["foo-bar", "f"]),
            has_arg(true),
            is_array(true),
        ])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "ABC".to_string(),
            "-f".to_string(),
            "DEF".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("ABC"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["ABC", "DEF"] as &[&str]));
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(
            cmd.cfgs[0].names,
            vec!["foo-bar".to_string(), "f".to_string()]
        );
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_is_not_array_but_opts_are_multiple() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true)]),
            OptCfg::with([names(&["f"]), has_arg(true)]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar=ABC".to_string(),
            "--foo-bar".to_string(),
            "DEF".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo-bar");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionIsNotArray {
                store_key: sk,
                option,
            }) => {
                assert_eq!(sk, "foo-bar");
                assert_eq!(option, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("ABC"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"]), has_arg(true)]),
            OptCfg::with([names(&["f"]), has_arg(true)]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "-f=ABC".to_string(),
            "-f".to_string(),
            "DEF".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "f");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionIsNotArray {
                store_key: sk,
                option,
            }) => {
                assert_eq!(sk, "f");
                assert_eq!(option, "f");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), true);
        assert_eq!(cmd.opt_arg("f"), Some("ABC"));
        assert_eq!(cmd.opt_args("f"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn specify_defaults() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["bar"]), has_arg(true), defaults(&["A"])]),
            OptCfg::with([
                names(&["baz"]),
                has_arg(true),
                is_array(true),
                defaults(&["B"]),
            ]),
        ];

        let mut cmd = Cmd::with_strings(["app".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.opt_arg("foo"), None);
        assert_eq!(cmd.opt_args("foo"), None);
        assert_eq!(cmd.has_opt("bar"), true);
        assert_eq!(cmd.opt_arg("bar"), Some("A"));
        assert_eq!(cmd.opt_args("bar"), Some(&["A"] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), Some("B"));
        assert_eq!(cmd.opt_args("baz"), Some(&["B"] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, Some(vec!["A".to_string()]));
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["baz".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, Some(vec!["B".to_string()]));
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn one_cfg_requires_no_arg_but_has_defaults() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"]), defaults(&["A"])])];

        let mut cmd = Cmd::with_strings(["app".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo-bar");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::ConfigHasDefaultsButHasNoArg {
                store_key: sk,
                name,
            }) => {
                assert_eq!(sk, "foo-bar");
                assert_eq!(name, "foo-bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, Some(vec!["A".to_string()]));
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn multiple_args() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo-bar"])]),
            OptCfg::with([names(&["baz", "z"]), has_arg(true), is_array(true)]),
            OptCfg::with([names(&["corge"]), has_arg(true), defaults(&["99"])]),
            OptCfg::with([names(&["*"])]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo-bar".to_string(),
            "qux".to_string(),
            "--baz".to_string(),
            "1".to_string(),
            "-z=2".to_string(),
            "-X".to_string(),
            "quux".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), Some("1"));
        assert_eq!(cmd.opt_args("baz"), Some(&["1", "2"] as &[&str]));
        assert_eq!(cmd.has_opt("X"), true);
        assert_eq!(cmd.opt_arg("X"), None);
        assert_eq!(cmd.opt_args("X"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("corge"), true);
        assert_eq!(cmd.opt_arg("corge"), Some("99"));
        assert_eq!(cmd.opt_args("corge"), Some(&["99"] as &[&str]));
        assert_eq!(cmd.args(), &["qux", "quux"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 4);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["baz".to_string(), "z".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, true);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[2].store_key, "".to_string());
        assert_eq!(cmd.cfgs[2].names, vec!["corge".to_string()]);
        assert_eq!(cmd.cfgs[2].has_arg, true);
        assert_eq!(cmd.cfgs[2].is_array, false);
        assert_eq!(cmd.cfgs[2].defaults, Some(vec!["99".to_string()]));
        assert_eq!(cmd.cfgs[2].desc, "".to_string());
        assert_eq!(cmd.cfgs[2].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[3].store_key, "".to_string());
        assert_eq!(cmd.cfgs[3].names, vec!["*".to_string()]);
        assert_eq!(cmd.cfgs[3].has_arg, false);
        assert_eq!(cmd.cfgs[3].is_array, false);
        assert_eq!(cmd.cfgs[3].defaults, None);
        assert_eq!(cmd.cfgs[3].desc, "".to_string());
        assert_eq!(cmd.cfgs[3].arg_in_help, "".to_string());
    }

    #[test]
    fn parse_all_args_even_if_error() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo", "f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "-ef".to_string(), "bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "e");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "e");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("e"), false);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.opt_arg("foo"), None);
        assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &["bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn parse_all_args_even_if_short_option_value_is_error() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["e"])]),
            OptCfg::with([names(&["foo", "f"])]),
        ];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "-ef=123".to_string(), "bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "f");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionTakesNoArg {
                store_key: sk,
                option,
            }) => {
                assert_eq!(option, "f");
                assert_eq!(sk, "foo");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("e"), true);
        assert_eq!(cmd.opt_arg("e"), None);
        assert_eq!(cmd.opt_args("e"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.args(), &["bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["e".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn parse_all_args_even_if_long_option_value_is_error() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["e"])]),
            OptCfg::with([names(&["foo", "f"])]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo=123".to_string(),
            "-e".to_string(),
            "bar".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "foo");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionTakesNoArg {
                store_key: sk,
                option,
            }) => {
                assert_eq!(option, "foo");
                assert_eq!(sk, "foo");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("e"), true);
        assert_eq!(cmd.opt_arg("e"), None);
        assert_eq!(cmd.opt_args("e"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.args(), &["bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["e".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn ignore_cfg_if_names_is_empty() {
        let opt_cfgs = vec![OptCfg::with([names(&[])]), OptCfg::with([names(&["foo"])])];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "--foo".to_string(), "bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.args(), &["bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, Vec::<String>::new());
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["foo".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn option_name_is_duplicated() {
        let opt_cfgs = vec![
            OptCfg::with([names(&["foo", "f"])]),
            OptCfg::with([names(&["bar", "f"])]),
        ];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "--foo".to_string(), "--bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => {
                assert_eq!(err.option(), "f");
            }
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::OptionNameIsDuplicated {
                store_key: sk,
                name,
            }) => {
                assert_eq!(name, "f");
                assert_eq!(sk, "bar");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.has_opt("bar"), false);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["bar".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn use_store_key() {
        let opt_cfgs = vec![OptCfg::with([store_key("FooBar"), names(&["f", "foo"])])];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "--foo".to_string(), "bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("FooBar"), true);
        assert_eq!(cmd.opt_arg("FooBar"), None);
        assert_eq!(cmd.opt_args("FooBar"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.args(), &["bar"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "FooBar".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string(), "foo".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn use_store_key_as_option_name_if_names_is_empty() {
        let opt_cfgs = vec![OptCfg::with([store_key("FooBar")])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--FooBar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err);
                assert!(false);
            }
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("FooBar"), true);
        assert_eq!(cmd.opt_arg("FooBar"), None);
        assert_eq!(cmd.opt_args("FooBar"), Some(&[] as &[&str]));
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "FooBar".to_string());
        assert_eq!(cmd.cfgs[0].names, Vec::<String>::new());
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn store_key_is_duplicated() {
        let opt_cfgs = vec![
            OptCfg::with([store_key("FooBar"), names(&["f", "foo"])]),
            OptCfg::with([store_key("FooBar"), names(&["b", "bar"])]),
        ];

        let mut cmd =
            Cmd::with_strings(["app".to_string(), "--foo".to_string(), "bar".to_string()]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => assert!(false),
            Err(ref err) => assert_eq!(err.option(), "b"),
        }
        match result {
            Ok(()) => assert!(false),
            Err(InvalidOption::StoreKeyIsDuplicated {
                store_key: sk,
                name,
            }) => {
                assert_eq!(sk, "FooBar");
                assert_eq!(name, "b");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("FooBar"), false);
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.has_opt("bar"), false);
        assert_eq!(cmd.has_opt("b"), false);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "FooBar".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["f".to_string(), "foo".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "FooBar".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["b".to_string(), "bar".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }

    #[test]
    fn accept_all_options_if_store_key_is_asterisk() {
        let opt_cfgs = vec![OptCfg::with([store_key("*")])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo".to_string(),
            "--bar".to_string(),
            "baz".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.has_opt("bar"), true);
        assert_eq!(cmd.args(), &["baz"] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "*".to_string());
        assert_eq!(cmd.cfgs[0].names, Vec::<String>::new());
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn accept_unconfigured_option_even_if_it_matches_store_key() {
        let opt_cfgs = vec![
            OptCfg::with([
                store_key("Bar"),
                names(&["foo", "f"]),
                has_arg(true),
                is_array(true),
            ]),
            OptCfg::with([store_key("*")]),
        ];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo".to_string(),
            "1".to_string(),
            "-f=2".to_string(),
            "--Bar=3".to_string(),
        ]);

        let result = cmd.parse_with(opt_cfgs);
        match result {
            Ok(()) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("Bar"), true);
        assert_eq!(cmd.opt_arg("Bar"), Some("1"));
        assert_eq!(cmd.opt_args("Bar"), Some(&["1", "2", "3"] as &[&str]));
        assert_eq!(cmd.has_opt("foo"), false);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.cfgs.len(), 2);
        assert_eq!(cmd.cfgs[0].store_key, "Bar".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, true);
        assert_eq!(cmd.cfgs[0].is_array, true);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "*".to_string());
        assert_eq!(cmd.cfgs[1].names, Vec::<String>::new());
        assert_eq!(cmd.cfgs[1].has_arg, false);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
    }
}

#[cfg(test)]
mod tests_of_parse_util_sub_cmd_with {
    use super::*;
    use crate::OptCfgParam::*;

    #[test]
    fn get_sub_cmd() {
        let opt_cfgs1 = vec![OptCfg::with([names(&["foo", "f"])])];

        let opt_cfgs2 = vec![OptCfg::with([names(&["bar", "b"])])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo".to_string(),
            "sub".to_string(),
            "--bar".to_string(),
        ]);

        match cmd.parse_until_sub_cmd_with(opt_cfgs1) {
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(sub_cmd.name(), "sub");
                assert_eq!(sub_cmd.args(), &[] as &[&str]);
                assert_eq!(sub_cmd.has_opt("bar"), false);
                assert_eq!(sub_cmd.opt_arg("fbar"), None);
                assert_eq!(sub_cmd.opt_args("bar"), None);

                let _ = sub_cmd.parse_with(opt_cfgs2);
                assert_eq!(sub_cmd.name(), "sub");
                assert_eq!(sub_cmd.args(), &[] as &[&str]);
                assert_eq!(sub_cmd.has_opt("bar"), true);
                assert_eq!(sub_cmd.opt_arg("bar"), None);
                assert_eq!(sub_cmd.opt_args("bar"), Some(&[] as &[&str]));

                assert_eq!(sub_cmd.cfgs.len(), 1);
                assert_eq!(sub_cmd.cfgs[0].store_key, "".to_string());
                assert_eq!(
                    sub_cmd.cfgs[0].names,
                    vec!["bar".to_string(), "b".to_string()]
                );
                assert_eq!(sub_cmd.cfgs[0].has_arg, false);
                assert_eq!(sub_cmd.cfgs[0].is_array, false);
                assert_eq!(sub_cmd.cfgs[0].defaults, None);
                assert_eq!(sub_cmd.cfgs[0].desc, "".to_string());
                assert_eq!(sub_cmd.cfgs[0].arg_in_help, "".to_string());
            }
            Ok(None) => assert!(false),
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.opt_arg("foo"), None);
        assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn no_sub_cmd() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo", "f"])])];

        let mut cmd = Cmd::with_strings(["app".to_string(), "--foo".to_string()]);

        match cmd.parse_until_sub_cmd_with(opt_cfgs) {
            Ok(Some(_)) => assert!(false),
            Ok(None) => {}
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.opt_arg("foo"), None);
        assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo".to_string(), "f".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn fail_to_parse() {
        let opt_cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "--bar-foo".to_string()]);

        match cmd.parse_until_sub_cmd_with(opt_cfgs) {
            Ok(_) => assert!(false),
            Err(InvalidOption::UnconfiguredOption { option }) => {
                assert_eq!(option, "bar-foo");
            }
            Err(_) => {}
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("foo-bar"), false);
        assert_eq!(cmd.opt_arg("foo-bar"), None);
        assert_eq!(cmd.opt_args("foo-bar"), None);
        assert_eq!(cmd.has_opt("f"), false);
        assert_eq!(cmd.opt_arg("f"), None);
        assert_eq!(cmd.opt_args("f"), None);
        assert_eq!(cmd.args(), &[] as &[&str]);

        assert_eq!(cmd.has_opt("bar-foo"), false);
        assert_eq!(cmd.opt_arg("bar-foo"), None);
        assert_eq!(cmd.opt_args("bar-foo"), None);

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "".to_string());
        assert_eq!(cmd.cfgs[0].names, vec!["foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }

    #[test]
    fn should_parse_with_end_opt_mark() {
        let opt_cfgs0 = vec![OptCfg::with([names(&["foo"])])];
        let opt_cfgs1 = vec![OptCfg::with([names(&["bar"])])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--foo".to_string(),
            "sub".to_string(),
            "--".to_string(),
            "bar".to_string(),
            "-@".to_string(),
        ]);

        match cmd.parse_until_sub_cmd_with(opt_cfgs0) {
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
                assert_eq!(cmd.has_opt("foo"), true);
                assert_eq!(cmd.opt_arg("foo"), None);
                assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));
                assert_eq!(cmd.has_opt("bar"), false);
                assert_eq!(cmd.opt_arg("bar"), None);
                assert_eq!(cmd.opt_args("bar"), None);

                match sub_cmd.parse_with(opt_cfgs1) {
                    Ok(_) => {
                        assert_eq!(sub_cmd.name(), "sub");
                        assert_eq!(sub_cmd.args(), &["bar", "-@"] as &[&str]);
                        assert_eq!(sub_cmd.has_opt("foo"), false);
                        assert_eq!(sub_cmd.opt_arg("foo"), None);
                        assert_eq!(sub_cmd.opt_args("foo"), None);
                        assert_eq!(sub_cmd.has_opt("bar"), false);
                        assert_eq!(sub_cmd.opt_arg("bar"), None);
                        assert_eq!(sub_cmd.opt_args("bar"), None);
                    }
                    Err(_) => assert!(false),
                }
            }
            Ok(None) => assert!(false),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn should_parse_after_end_opt_mark() {
        let opt_cfgs0 = vec![OptCfg::with([names(&["foo"])])];
        let opt_cfgs1 = vec![OptCfg::with([names(&["bar"])])];

        let mut cmd = Cmd::with_strings([
            "app".to_string(),
            "--".to_string(),
            "--foo".to_string(),
            "sub".to_string(),
            "bar".to_string(),
            "-@".to_string(),
        ]);

        match cmd.parse_until_sub_cmd_with(opt_cfgs0) {
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
                assert_eq!(cmd.has_opt("foo"), false);
                assert_eq!(cmd.opt_arg("foo"), None);
                assert_eq!(cmd.opt_args("foo"), None);
                assert_eq!(cmd.has_opt("bar"), false);
                assert_eq!(cmd.opt_arg("bar"), None);
                assert_eq!(cmd.opt_args("bar"), None);

                match sub_cmd.parse_with(opt_cfgs1) {
                    Ok(_) => {
                        assert_eq!(sub_cmd.name(), "--foo");
                        assert_eq!(sub_cmd.args(), &["sub", "bar", "-@"] as &[&str]);
                        assert_eq!(sub_cmd.has_opt("foo"), false);
                        assert_eq!(sub_cmd.opt_arg("foo"), None);
                        assert_eq!(sub_cmd.opt_args("foo"), None);
                        assert_eq!(sub_cmd.has_opt("bar"), false);
                        assert_eq!(sub_cmd.opt_arg("bar"), None);
                        assert_eq!(sub_cmd.opt_args("bar"), None);
                    }
                    Err(_) => assert!(false),
                }
            }
            Ok(None) => assert!(false),
            Err(_) => assert!(false),
        }
    }
}
