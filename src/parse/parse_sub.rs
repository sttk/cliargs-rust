// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::parse_args;
use crate::errors::InvalidOption;
use crate::Cmd;

impl<'b, 'a> Cmd<'a> {
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

        if !self._leaked_strs.is_empty() {
            if let Some(idx) = parse_args(
                &self._leaked_strs[1..],
                collect_args,
                collect_opts,
                take_opt_args,
                true,
            )? {
                return Ok(Some(Cmd::with_strings(
                    self._leaked_strs[idx + 1..]
                        .into_iter()
                        .map(|s| s.to_string()),
                )));
            }
        }

        Ok(None)
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
}
