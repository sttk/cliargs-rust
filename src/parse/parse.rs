// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::parse_args;
use crate::errors::InvalidOption;
use crate::Cmd;

impl<'b, 'a> Cmd<'a> {
    /// Parses command line arguments without configurations.
    ///
    /// This method divides command line arguments into options and command arguments based on
    /// simple rules that are almost the same as POSIX & GNU:
    /// arguments staring with `-` or `--` are treated as options, and others are treated as command
    /// arguments.
    /// If an `=` is found within an option, the part before the `=` is treated as the option name,
    /// and the part after the `=` is treated as the option argument.
    /// Options starting with `--` are long options and option starting with `-` are short options.
    /// Multiple short options can be concatenated into a single command line argument.
    /// If an argument is exactly `--`, all subsequent arguments are treated as command arguments.
    ///
    /// Since the results of parsing are stored into this `Cmd` instance, this method returns a
    /// [Result] which contains an unit value (`()`) if succeeding, or a `errors::InvalidOption`
    /// if failing.
    ///
    /// ```rust
    /// use cliargs::Cmd;
    /// use cliargs::errors::InvalidOption;
    ///
    /// let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
    ///
    /// match cmd.parse() {
    ///     Ok(_) => { /* ... */ },
    ///     Err(InvalidOption::OptionContainsInvalidChar { option }) => {
    ///         panic!("Option contains invalid character: {option}");
    ///     },
    ///     Err(err) => panic!("Invalid option: {}", err.option()),
    /// }
    /// ```
    pub fn parse(&mut self) -> Result<(), InvalidOption> {
        let collect_args = |arg| {
            self.args.push(arg);
        };

        let collect_opts = |name, option| {
            let vec = self.opts.entry(name).or_insert_with(|| Vec::new());
            if let Some(arg) = option {
                vec.push(arg);
            }
            Ok(())
        };

        let take_opt_args = |_arg: &str| false;

        if self._num_of_args > 0 {
            match parse_args(
                &self._leaked_strs[1..(self._num_of_args)],
                collect_args,
                collect_opts,
                take_opt_args,
                false,
                self.is_after_end_opt,
            ) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    /// Parses command line arguments without configurations but stops parsing when encountering
    /// first command argument.
    ///
    /// This method creates and returns a new [Cmd] instance that holds the command line arguments
    /// starting from the first command argument.
    ///
    /// This method parses command line arguments in the same way as the [Cmd::parse] method, except
    /// that it only parses the command line arguments before the first command argument.
    ///
    /// ```rust
    /// use cliargs::Cmd;
    /// use cliargs::errors::InvalidOption;
    ///
    /// let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
    ///
    /// match cmd.parse_until_sub_cmd() {
    ///     Ok(Some(mut sub_cmd)) => {
    ///         let sub_cmd_name = sub_cmd.name();
    ///         match sub_cmd.parse() {
    ///             Ok(_) => { /* ... */ },
    ///             Err(err) => panic!("Invalid option: {}", err.option()),
    ///         }
    ///     },
    ///     Ok(None) => { /* ... */ },
    ///     Err(InvalidOption::OptionContainsInvalidChar { option }) => {
    ///         panic!("Option contains invalid character: {option}");
    ///     },
    ///     Err(err) => panic!("Invalid option: {}", err.option()),
    /// }
    /// ```
    pub fn parse_until_sub_cmd(&mut self) -> Result<Option<Cmd<'b>>, InvalidOption> {
        let collect_args = |_arg| {};

        let collect_opts = |name, option| {
            let vec = self.opts.entry(name).or_insert_with(|| Vec::new());
            if let Some(arg) = option {
                vec.push(arg);
            }
            Ok(())
        };

        let take_opt_args = |_arg: &str| false;

        if self._num_of_args > 0 {
            if let Some((idx, is_after_end_opt)) = parse_args(
                &self._leaked_strs[1..(self._num_of_args)],
                collect_args,
                collect_opts,
                take_opt_args,
                true,
                self.is_after_end_opt,
            )? {
                return Ok(Some(self.sub_cmd(idx + 1, is_after_end_opt))); // +1, because parse_args parses from 1.
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests_of_cmd {
    use crate::errors::InvalidOption;
    use crate::Cmd;

    mod tests_of_parse {
        use super::*;

        #[test]
        fn should_parse_zero_arg() {
            let mut cmd = Cmd::with_strings(["/path/to/app".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_opt_with_no_arg() {
            let mut cmd = Cmd::with_strings(["/path/to/app".to_string(), "abcd".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), ["abcd"]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_long_opt() {
            let mut cmd = Cmd::with_strings(["/path/to/app".to_string(), "--silent".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), true);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), Some(&[] as &[&str]));
        }

        #[test]
        fn should_parse_one_long_opt_with_arg() {
            let mut cmd =
                Cmd::with_strings(["/path/to/app".to_string(), "--alphabet=ABC".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), true);
            assert_eq!(cmd.opt_arg("alphabet"), Some("ABC"));
            assert_eq!(cmd.opt_args("alphabet"), Some(&["ABC"] as &[&str]));
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_short_opt() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-s".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_short_opt_with_arg() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-a=123".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), Some("123"));
            assert_eq!(cmd.opt_args("a"), Some(&["123"] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_arg_by_multiple_short_opts() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-sa".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_one_arg_by_multiple_short_opts_with_arg() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-sa=123".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), Some("123"));
            assert_eq!(cmd.opt_args("a"), Some(&["123"] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_long_opt_name_including_hyphen_marks() {
            let mut cmd = Cmd::with_strings(["app".to_string(), "--aaa-bbb-ccc=123".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("aaa-bbb-ccc"), true);
            assert_eq!(cmd.opt_arg("aaa-bbb-ccc"), Some("123"));
            assert_eq!(cmd.opt_args("aaa-bbb-ccc"), Some(&["123"] as &[&str]));
        }

        #[test]
        fn should_parse_opts_and_arg_including_equal_marks() {
            let mut cmd = Cmd::with_strings(["app".to_string(), "-sa=b=c".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), Some("b=c"));
            assert_eq!(cmd.opt_args("a"), Some(&["b=c"] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_opts_with_args_including_marks() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-sa=1,2-3".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), Some("1,2-3"));
            assert_eq!(cmd.opt_args("a"), Some(&["1,2-3"] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_but_fail_because_of_illegal_long_opt_including_invalid_char() {
            let mut cmd = Cmd::with_strings([
                "path/to/app".to_string(),
                "-s".to_string(),
                "--abc%def".to_string(),
                "-a".to_string(),
            ]);
            match cmd.parse() {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "abc%def");
                }
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_but_fail_because_of_illegal_long_opt_of_which_first_char_is_number() {
            let mut cmd = Cmd::with_strings(["app".to_string(), "--1abc".to_string()]);
            match cmd.parse() {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "1abc");
                }
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_but_fail_because_of_illegal_long_opt_of_which_first_char_is_hyphen() {
            let mut cmd = Cmd::with_strings(["app".to_string(), "---aaa=123".to_string()]);
            match cmd.parse() {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "-aaa=123");
                }
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_but_fail_because_of_illegal_char_in_short_opt() {
            let mut cmd = Cmd::with_strings([
                "path/to/app".to_string(),
                "-s".to_string(),
                "--alphabet".to_string(),
                "-a@".to_string(),
            ]);
            match cmd.parse() {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "@");
                }
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("alphabet"), true);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_with_end_opt_mark() {
            let mut cmd = Cmd::with_strings([
                "path/to/app".to_string(),
                "-s".to_string(),
                "--".to_string(),
                "-a".to_string(),
                "-s@".to_string(),
                "--".to_string(),
                "xxx".to_string(),
            ]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["-a", "-s@", "--", "xxx"] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), true);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_single_hyphen() {
            let mut cmd = Cmd::with_strings(["path/to/app".to_string(), "-".to_string()]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["-"] as &[&str]);
            assert_eq!(cmd.has_opt("a"), false);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), None);
            assert_eq!(cmd.has_opt("alphabet"), false);
            assert_eq!(cmd.opt_arg("alphabet"), None);
            assert_eq!(cmd.opt_args("alphabet"), None);
            assert_eq!(cmd.has_opt("s"), false);
            assert_eq!(cmd.opt_arg("s"), None);
            assert_eq!(cmd.opt_args("s"), None);
            assert_eq!(cmd.has_opt("silent"), false);
            assert_eq!(cmd.opt_arg("silent"), None);
            assert_eq!(cmd.opt_args("silent"), None);
        }

        #[test]
        fn should_parse_multiple_args() {
            let mut cmd = Cmd::with_strings([
                "app".to_string(),
                "--foo-bar".to_string(),
                "-a".to_string(),
                "--baz".to_string(),
                "-bc=3".to_string(),
                "qux".to_string(),
                "-c=4".to_string(),
                "quux".to_string(),
            ]);
            match cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["qux", "quux"] as &[&str]);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.opt_arg("a"), None);
            assert_eq!(cmd.opt_args("a"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("b"), true);
            assert_eq!(cmd.opt_arg("b"), None);
            assert_eq!(cmd.opt_args("b"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("c"), true);
            assert_eq!(cmd.opt_arg("c"), Some("3"));
            assert_eq!(cmd.opt_args("c"), Some(&["3", "4"] as &[&str]));
            assert_eq!(cmd.has_opt("foo-bar"), true);
            assert_eq!(cmd.opt_arg("foo-bar"), None);
            assert_eq!(cmd.opt_args("foo-bar"), Some(&[] as &[&str]));
            assert_eq!(cmd.has_opt("baz"), true);
            assert_eq!(cmd.opt_arg("baz"), None);
            assert_eq!(cmd.opt_args("baz"), Some(&[] as &[&str]));
        }

        #[test]
        fn should_parse_all_args_even_if_failed() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--1".to_string(),
                "-b2ar".to_string(),
                "--3".to_string(),
                "baz".to_string(),
            ]);
            match cmd.parse() {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                    assert_eq!(option, "1");
                }
                Err(_) => assert!(false),
            }

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &["baz"] as &[&str]);
            assert_eq!(cmd.has_opt("foo"), true);
            assert_eq!(cmd.has_opt("b"), true);
            assert_eq!(cmd.has_opt("a"), true);
            assert_eq!(cmd.has_opt("r"), true);
            assert_eq!(cmd.has_opt("1"), false);
            assert_eq!(cmd.has_opt("2"), false);
            assert_eq!(cmd.has_opt("3"), false);
        }
    }
}

#[cfg(test)]
mod tests_of_parse_until_sub_cmd {
    use super::*;

    #[test]
    fn test_if_command_line_arguments_contains_no_command_argument_and_option() {
        let ui_args = vec!["/path/to/app".to_string()];
        let mut cmd = Cmd::with_strings(ui_args);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => {}
            Ok(Some(_)) => assert!(false),
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
    }

    #[test]
    fn test_if_command_line_arguments_contains_only_command_arguments() {
        let ui_args = vec![
            "/path/to/app".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ];
        let mut cmd = Cmd::with_strings(ui_args);

        match cmd.parse_until_sub_cmd() {
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(sub_cmd.name(), "foo");
                assert_eq!(sub_cmd.args(), &[] as &[&str]);

                match sub_cmd.parse() {
                    Ok(_) => {}
                    Err(_) => assert!(false),
                }

                assert_eq!(sub_cmd.name(), "foo");
                assert_eq!(sub_cmd.args(), &["bar"]);
            }
            Ok(None) => assert!(false),
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);

        //

        let f = || {
            let ui_args = vec![
                "/path/to/app".to_string(),
                "foo".to_string(),
                "bar".to_string(),
            ];
            let mut cmd = Cmd::with_strings(ui_args);

            if let Some(mut sub_cmd) = cmd.parse_until_sub_cmd()? {
                assert_eq!(sub_cmd.name(), "foo");
                assert_eq!(sub_cmd.args(), &[] as &[&str]);

                match sub_cmd.parse() {
                    Ok(_) => {}
                    Err(_) => assert!(false),
                }

                assert_eq!(sub_cmd.name(), "foo");
                assert_eq!(sub_cmd.args(), &["bar"]);

                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
            } else {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
            }

            Ok::<(), InvalidOption>(())
        };
        let _ = f();
    }

    #[test]
    fn test_if_command_line_arguments_contains_only_command_options() {
        let ui_args = vec![
            "/path/to/app".to_string(),
            "--foo".to_string(),
            "-b".to_string(),
        ];
        let mut cmd = Cmd::with_strings(ui_args);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => {}
            Ok(Some(_)) => assert!(false),
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.has_opt("b"), true);
        assert_eq!(cmd.opt_arg("foo"), None);
        assert_eq!(cmd.opt_arg("b"), None);
    }

    #[test]
    fn test_if_command_line_arguments_contains_both_command_arguments_and_options() {
        let ui_args = vec![
            "/path/to/app".to_string(),
            "--foo=123".to_string(),
            "bar".to_string(),
            "--baz".to_string(),
            "-q=ABC".to_string(),
            "quux".to_string(),
        ];
        let mut cmd = Cmd::with_strings(ui_args);

        if let Some(mut sub_cmd) = cmd.parse_until_sub_cmd().unwrap() {
            assert_eq!(sub_cmd.name(), "bar");
            assert_eq!(sub_cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("baz"), false);
            assert_eq!(cmd.opt_arg("baz"), None);
            assert_eq!(cmd.has_opt("q"), false);
            assert_eq!(cmd.opt_arg("q"), None);

            match sub_cmd.parse() {
                Ok(_) => {}
                Err(_) => assert!(false),
            }

            assert_eq!(sub_cmd.name(), "bar");
            assert_eq!(sub_cmd.args(), &["quux"]);
            assert_eq!(sub_cmd.has_opt("baz"), true);
            assert_eq!(sub_cmd.opt_arg("baz"), None);
            assert_eq!(sub_cmd.has_opt("q"), true);
            assert_eq!(sub_cmd.opt_arg("q"), Some("ABC"));
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("foo"), true);
        assert_eq!(cmd.opt_arg("foo"), Some("123"));
    }

    #[test]
    fn test_if_fail_to_parse() {
        let ui_args = vec![
            "/path/to/app".to_string(),
            "--f#o".to_string(),
            "bar".to_string(),
        ];
        let mut cmd = Cmd::with_strings(ui_args);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => assert!(false),
            Ok(Some(_)) => assert!(false),
            Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                assert_eq!(option, "f#o");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("f#o"), false);
        assert_eq!(cmd.opt_arg("f#o"), None);
    }

    #[test] // for the fix of issue #39
    fn test_if_sub_command_is_like_path() {
        let ui_args = vec![
            "/path/to/app".to_string(),
            "--foo-bar".to_string(),
            "path/to/bar".to_string(),
            "--baz".to_string(),
            "qux".to_string(),
        ];
        let mut cmd = Cmd::with_strings(ui_args);

        if let Some(mut sub_cmd) = cmd.parse_until_sub_cmd().unwrap() {
            sub_cmd.parse().unwrap();

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("foo-bar"), true);

            assert_eq!(sub_cmd.name(), "path/to/bar");
            assert_eq!(sub_cmd.args(), &["qux"]);
            assert_eq!(sub_cmd.has_opt("baz"), true);
        }
    }

    #[test]
    fn should_parse_single_hyphen() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-a".to_string(),
            "-".to_string(),
            "b".to_string(),
            "-".to_string(),
        ]);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => assert!(false),
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
                assert_eq!(cmd.has_opt("a"), true);

                match sub_cmd.parse() {
                    Ok(_) => {
                        assert_eq!(sub_cmd.name(), "-");
                        assert_eq!(sub_cmd.args(), &["b", "-"]);
                        assert_eq!(sub_cmd.has_opt("a"), false);
                    }
                    Err(_) => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn should_parse_single_hyphen_but_error() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-a".to_string(),
            "-@".to_string(),
            "-".to_string(),
            "b".to_string(),
            "-".to_string(),
        ]);

        match cmd.parse_until_sub_cmd() {
            Ok(_) => assert!(false),
            Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                assert_eq!(option, "@");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
        assert_eq!(cmd.has_opt("a"), true);
    }

    #[test]
    fn should_parse_with_end_opt_mark() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "sub".to_string(),
            "--".to_string(),
            "-a".to_string(),
            "-s@".to_string(),
            "--".to_string(),
            "xxx".to_string(),
        ]);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => assert!(false),
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
                assert_eq!(cmd.has_opt("a"), false);
                assert_eq!(cmd.opt_arg("a"), None);
                assert_eq!(cmd.opt_args("a"), None);
                assert_eq!(cmd.has_opt("alphabet"), false);
                assert_eq!(cmd.opt_arg("alphabet"), None);
                assert_eq!(cmd.opt_args("alphabet"), None);
                assert_eq!(cmd.has_opt("s"), false);
                assert_eq!(cmd.opt_arg("s"), None);
                assert_eq!(cmd.opt_args("s"), None);
                assert_eq!(cmd.has_opt("silent"), false);
                assert_eq!(cmd.opt_arg("silent"), None);
                assert_eq!(cmd.opt_args("silent"), None);

                match sub_cmd.parse() {
                    Err(_) => assert!(false),
                    Ok(_) => {
                        assert_eq!(sub_cmd.name(), "sub");
                        assert_eq!(sub_cmd.args(), &["-a", "-s@", "--", "xxx"]);
                        assert_eq!(sub_cmd.has_opt("a"), false);
                        assert_eq!(sub_cmd.opt_arg("a"), None);
                        assert_eq!(sub_cmd.opt_args("a"), None);
                        assert_eq!(sub_cmd.has_opt("alphabet"), false);
                        assert_eq!(sub_cmd.opt_arg("alphabet"), None);
                        assert_eq!(sub_cmd.opt_args("alphabet"), None);
                        assert_eq!(sub_cmd.has_opt("s"), false);
                        assert_eq!(sub_cmd.opt_arg("s"), None);
                        assert_eq!(sub_cmd.opt_args("s"), None);
                        assert_eq!(sub_cmd.has_opt("silent"), false);
                        assert_eq!(sub_cmd.opt_arg("silent"), None);
                        assert_eq!(sub_cmd.opt_args("silent"), None);
                    }
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn should_parse_after_end_opt_mark() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-s".to_string(),
            "--".to_string(),
            "-a".to_string(),
            "-s@".to_string(),
            "--".to_string(),
            "xxx".to_string(),
        ]);

        match cmd.parse_until_sub_cmd() {
            Ok(None) => assert!(false),
            Ok(Some(mut sub_cmd)) => {
                assert_eq!(cmd.name(), "app");
                assert_eq!(cmd.args(), &[] as &[&str]);
                assert_eq!(cmd.has_opt("a"), false);
                assert_eq!(cmd.opt_arg("a"), None);
                assert_eq!(cmd.opt_args("a"), None);
                assert_eq!(cmd.has_opt("alphabet"), false);
                assert_eq!(cmd.opt_arg("alphabet"), None);
                assert_eq!(cmd.opt_args("alphabet"), None);
                assert_eq!(cmd.has_opt("s"), true);
                assert_eq!(cmd.opt_arg("s"), None);
                assert_eq!(cmd.opt_args("s"), Some(&[] as &[&str]));
                assert_eq!(cmd.has_opt("silent"), false);
                assert_eq!(cmd.opt_arg("silent"), None);
                assert_eq!(cmd.opt_args("silent"), None);

                match sub_cmd.parse() {
                    Err(_) => assert!(false),
                    Ok(_) => {
                        assert_eq!(sub_cmd.name(), "-a");
                        assert_eq!(sub_cmd.args(), &["-s@", "--", "xxx"]);
                        assert_eq!(sub_cmd.has_opt("a"), false);
                        assert_eq!(sub_cmd.opt_arg("a"), None);
                        assert_eq!(sub_cmd.opt_args("a"), None);
                        assert_eq!(sub_cmd.has_opt("alphabet"), false);
                        assert_eq!(sub_cmd.opt_arg("alphabet"), None);
                        assert_eq!(sub_cmd.opt_args("alphabet"), None);
                        assert_eq!(sub_cmd.has_opt("s"), false);
                        assert_eq!(sub_cmd.opt_arg("s"), None);
                        assert_eq!(sub_cmd.opt_args("s"), None);
                        assert_eq!(sub_cmd.has_opt("silent"), false);
                        assert_eq!(sub_cmd.opt_arg("silent"), None);
                        assert_eq!(sub_cmd.opt_args("silent"), None);
                    }
                }
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn should_parse_after_end_opt_mark_but_error() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-@".to_string(),
            "--".to_string(),
            "-a".to_string(),
            "-s@".to_string(),
            "--".to_string(),
            "xxx".to_string(),
        ]);

        match cmd.parse_until_sub_cmd() {
            Ok(_) => assert!(false),
            Err(InvalidOption::OptionContainsInvalidChar { option }) => {
                assert_eq!(option, "@");
            }
            Err(_) => assert!(false),
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), &[] as &[&str]);
    }
}
