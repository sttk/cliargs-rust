#[cfg(test)]
mod tests_of_parse_with {
    use cliargs::validators::*;
    use cliargs::OptCfgParam::*;
    use cliargs::{Cmd, OptCfg};

    #[test]
    fn it_should_parse_command_line_arguments_with_option_configurations() {
        let opt_cfgs = vec![
            OptCfg::with(&[store_key("fooBar"), names(&["foo-bar", "f"])]),
            OptCfg::with(&[names(&["baz", "b"]), has_arg(true)]),
            OptCfg::with(&[
                names(&["qux", "q"]),
                has_arg(true),
                is_array(true),
                validator(validate_number::<u32>),
            ]),
        ];

        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-f".to_string(),
            "-b".to_string(),
            "ABC".to_string(),
            "--qux=123".to_string(),
            "-q".to_string(),
            "456".to_string(),
        ]);

        if let Err(err) = cmd.parse_with(opt_cfgs) {
            println!("{:?}", err);
            assert!(false);
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("fooBar"), true);
        assert_eq!(cmd.opt_arg("fooBar"), None);
        assert_eq!(cmd.opt_args("fooBar"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), Some("ABC"));
        assert_eq!(cmd.opt_args("baz"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.has_opt("qux"), true);
        assert_eq!(cmd.opt_arg("qux"), Some("123"));
        assert_eq!(cmd.opt_args("qux"), Some(&["123", "456"] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 3);
        assert_eq!(cmd.cfgs[0].store_key, "fooBar".to_string());
        assert_eq!(
            cmd.cfgs[0].names,
            vec!["foo-bar".to_string(), "f".to_string()]
        );
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["baz".to_string(), "b".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[2].store_key, "".to_string());
        assert_eq!(cmd.cfgs[2].names, vec!["qux".to_string(), "q".to_string()]);
        assert_eq!(cmd.cfgs[2].has_arg, true);
        assert_eq!(cmd.cfgs[2].is_array, true);
        assert_eq!(cmd.cfgs[2].defaults, None);
        assert_eq!(cmd.cfgs[2].desc, "".to_string());
        assert_eq!(cmd.cfgs[2].arg_in_help, "".to_string());
    }
}

#[cfg(test)]
mod tests_of_errors {
    use cliargs::errors::InvalidOption;
    use cliargs::validators::*;
    use cliargs::OptCfgParam::*;
    use cliargs::{Cmd, OptCfg};

    #[test]
    fn it_should_parse_but_fail_if_the_option_does_not_match_configuration() {
        let opt_cfgs = vec![
            OptCfg::with(&[store_key("fooBar"), names(&["foo-bar", "f"])]),
            OptCfg::with(&[names(&["baz", "b"]), has_arg(true)]),
            OptCfg::with(&[
                names(&["qux", "q"]),
                has_arg(true),
                is_array(true),
                validator(validate_number::<u32>),
            ]),
        ];

        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-f=aaa".to_string(),
            "-b".to_string(),
            "ABC".to_string(),
            "--qux=123".to_string(),
            "-q".to_string(),
            "456".to_string(),
        ]);

        if let Err(InvalidOption::OptionTakesNoArg {
            store_key: sk,
            option,
        }) = cmd.parse_with(opt_cfgs)
        {
            assert_eq!(sk, "fooBar");
            assert_eq!(option, "f");
        } else {
            assert!(false);
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("fooBar"), false);
        assert_eq!(cmd.opt_arg("fooBar"), None);
        assert_eq!(cmd.opt_args("fooBar"), None);
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), Some("ABC"));
        assert_eq!(cmd.opt_args("baz"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.has_opt("qux"), true);
        assert_eq!(cmd.opt_arg("qux"), Some("123"));
        assert_eq!(cmd.opt_args("qux"), Some(&["123", "456"] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 3);
        assert_eq!(cmd.cfgs[0].store_key, "fooBar".to_string());
        assert_eq!(
            cmd.cfgs[0].names,
            vec!["foo-bar".to_string(), "f".to_string()]
        );
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["baz".to_string(), "b".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[2].store_key, "".to_string());
        assert_eq!(cmd.cfgs[2].names, vec!["qux".to_string(), "q".to_string()]);
        assert_eq!(cmd.cfgs[2].has_arg, true);
        assert_eq!(cmd.cfgs[2].is_array, true);
        assert_eq!(cmd.cfgs[2].defaults, None);
        assert_eq!(cmd.cfgs[2].desc, "".to_string());
        assert_eq!(cmd.cfgs[2].arg_in_help, "".to_string());
    }

    #[test]
    fn it_should_parse_but_fail_if_the_option_argument_is_invalid() {
        let opt_cfgs = vec![
            OptCfg::with(&[store_key("fooBar"), names(&["foo-bar", "f"])]),
            OptCfg::with(&[names(&["baz", "b"]), has_arg(true)]),
            OptCfg::with(&[
                names(&["qux", "q"]),
                has_arg(true),
                is_array(true),
                validator(validate_number::<u32>),
            ]),
        ];

        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "-f".to_string(),
            "-b".to_string(),
            "ABC".to_string(),
            "--qux=123".to_string(),
            "-q".to_string(),
            "xxx".to_string(),
        ]);

        if let Err(InvalidOption::OptionArgIsInvalid {
            store_key: sk,
            option,
            opt_arg,
            details,
        }) = cmd.parse_with(opt_cfgs)
        {
            assert_eq!(sk, "qux");
            assert_eq!(option, "q");
            assert_eq!(opt_arg, "xxx");
            assert_eq!(details, "invalid digit found in string");
        } else {
            assert!(false);
        }

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.has_opt("fooBar"), true);
        assert_eq!(cmd.opt_arg("fooBar"), None);
        assert_eq!(cmd.opt_args("fooBar"), Some(&[] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), Some("ABC"));
        assert_eq!(cmd.opt_args("baz"), Some(&["ABC"] as &[&str]));
        assert_eq!(cmd.has_opt("qux"), true);
        assert_eq!(cmd.opt_arg("qux"), Some("123"));
        assert_eq!(cmd.opt_args("qux"), Some(&["123"] as &[&str]));

        assert_eq!(cmd.cfgs.len(), 3);
        assert_eq!(cmd.cfgs[0].store_key, "fooBar".to_string());
        assert_eq!(
            cmd.cfgs[0].names,
            vec!["foo-bar".to_string(), "f".to_string()]
        );
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[1].store_key, "".to_string());
        assert_eq!(cmd.cfgs[1].names, vec!["baz".to_string(), "b".to_string()]);
        assert_eq!(cmd.cfgs[1].has_arg, true);
        assert_eq!(cmd.cfgs[1].is_array, false);
        assert_eq!(cmd.cfgs[1].defaults, None);
        assert_eq!(cmd.cfgs[1].desc, "".to_string());
        assert_eq!(cmd.cfgs[1].arg_in_help, "".to_string());
        assert_eq!(cmd.cfgs[2].store_key, "".to_string());
        assert_eq!(cmd.cfgs[2].names, vec!["qux".to_string(), "q".to_string()]);
        assert_eq!(cmd.cfgs[2].has_arg, true);
        assert_eq!(cmd.cfgs[2].is_array, true);
        assert_eq!(cmd.cfgs[2].defaults, None);
        assert_eq!(cmd.cfgs[2].desc, "".to_string());
        assert_eq!(cmd.cfgs[2].arg_in_help, "".to_string());
    }
}
