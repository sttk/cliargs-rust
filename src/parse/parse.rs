// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::parse_args;
use crate::Cmd;
use crate::Error;

impl<'a> Cmd<'a> {
    pub fn parse(&mut self) -> Result<(), Error<'a>> {
        let collect_args = |arg| {
            self.args.push(arg);
        };

        let collect_opts = |name, option| {
            let vec = self.opts.entry(name).or_insert_with(|| Vec::new());
            if let Some(arg) = option {
                vec.push(arg);
            }
            Ok::<(), Error<'a>>(())
        };

        let take_args = |_arg: &str| false;

        if !self._arg_refs.is_empty() {
            match parse_args(&self._arg_refs[1..], collect_args, collect_opts, take_args) {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_of_cmd {
    use crate::{Cmd, Error, OptionError};

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
                Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar { option })) => {
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
                Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar { option })) => {
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
                Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar { option })) => {
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
                Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar { option })) => {
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
                Err(Error::InvalidOption(OptionError::OptionContainsInvalidChar { option })) => {
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