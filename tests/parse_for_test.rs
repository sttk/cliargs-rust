#[cfg(test)]
mod tests_of_parse_for {
    use cliargs;

    #[derive(cliargs::OptStore)]
    struct MyOptions {
        #[opt(cfg = "f,b,foo-bar", desc = "The FooBar flag")]
        foo_bar: bool,
    }

    #[test]
    fn make_cfgs_for_my_options() {
        let mut my_options = MyOptions::with_defaults();
        assert_eq!(my_options.foo_bar, false);

        let cfgs = cliargs::OptCfg::make_cfgs_for(&mut my_options);
        assert_eq!(cfgs.len(), 1);

        let cfg = &cfgs[0];
        assert_eq!(cfg.store_key, "foo_bar");
        assert_eq!(
            cfg.names,
            vec!["f".to_string(), "b".to_string(), "foo-bar".to_string()]
        );
        assert_eq!(cfg.has_arg, false);
        assert_eq!(cfg.is_array, false);
        assert_eq!(cfg.defaults, None);
        assert_eq!(cfg.desc, "The FooBar flag".to_string());
        assert_eq!(cfg.arg_in_help, "".to_string());
    }

    #[test]
    fn parse_for_my_options() {
        let mut my_options = MyOptions::with_defaults();
        assert_eq!(my_options.foo_bar, false);

        let mut cmd = cliargs::Cmd::with_strings([
            "/path/to/app".to_string(),
            "-f".to_string(),
            "abc".to_string(),
        ]);
        let result = cmd.parse_for(&mut my_options);
        assert!(result.is_ok());

        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), ["abc"]);
        assert_eq!(cmd.has_opt("foo_bar"), true);
        assert_eq!(cmd.opt_arg("foo_bar"), None);
        assert_eq!(cmd.opt_args("foo_bar"), Some(&[] as &[&str]));

        assert_eq!(cmd.opt_cfgs().len(), 1);
        assert_eq!(cmd.opt_cfgs()[0].store_key, "foo_bar");
        assert_eq!(
            cmd.opt_cfgs()[0].names,
            ["f".to_string(), "b".to_string(), "foo-bar".to_string()]
        );
        assert_eq!(cmd.opt_cfgs()[0].has_arg, false);
        assert_eq!(cmd.opt_cfgs()[0].is_array, false);
        assert_eq!(cmd.opt_cfgs()[0].defaults, None);
        assert_eq!(cmd.opt_cfgs()[0].desc, "The FooBar flag".to_string());
        assert_eq!(cmd.opt_cfgs()[0].arg_in_help, "".to_string());
    }
}

mod tests_of_parse_until_sub_cmd_for {
    use cliargs::validators::*;
    use cliargs::Cmd;

    #[derive(cliargs::OptStore)]
    struct Options1 {
        #[opt(cfg = "foo-bar")]
        fooBar: u32,
        v: bool,
    }

    #[derive(cliargs::OptStore)]
    struct Options2 {
        qux: bool,
        q: String,
    }

    #[test]
    fn it_should_parse_command_line_arguments_containing_subcommand() {
        let mut cmd = Cmd::with_strings([
            "/path/to/app".to_string(),
            "--foo-bar=123".to_string(),
            "-v".to_string(),
            "baz".to_string(),
            "--qux".to_string(),
            "corge".to_string(),
            "-q=ABC".to_string(),
        ]);

        let mut options1 = Options1::with_defaults();
        let mut options2 = Options2::with_defaults();

        if let Some(mut sub_cmd) = cmd.parse_until_sub_cmd_for(&mut options1).unwrap() {
            let _ = sub_cmd.parse_for(&mut options2).unwrap();

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("fooBar"), true);
            assert_eq!(cmd.opt_arg("fooBar"), Some("123"));
            assert_eq!(cmd.opt_args("fooBar"), Some(&["123"] as &[&str]));
            assert_eq!(cmd.has_opt("v"), true);
            assert_eq!(cmd.opt_arg("v"), None);
            assert_eq!(cmd.opt_args("v"), Some(&[] as &[&str]));

            assert_eq!(cmd.has_opt("foo-bar"), false);
            assert_eq!(cmd.opt_arg("foo-bar"), None);
            assert_eq!(cmd.opt_args("foo-bar"), None);
            assert_eq!(cmd.has_opt("f"), false);
            assert_eq!(cmd.opt_arg("f"), None);
            assert_eq!(cmd.opt_args("f"), None);

            assert_eq!(sub_cmd.name(), "baz");
            assert_eq!(sub_cmd.args(), &["corge"]);
            assert_eq!(sub_cmd.has_opt("qux"), true);
            assert_eq!(sub_cmd.opt_arg("qux"), None);
            assert_eq!(sub_cmd.opt_args("qux"), Some(&[] as &[&str]));
            assert_eq!(sub_cmd.has_opt("q"), true);
            assert_eq!(sub_cmd.opt_arg("q"), Some("ABC"));
            assert_eq!(sub_cmd.opt_args("q"), Some(&["ABC"] as &[&str]));
        } else {
            assert!(false);
        }

        assert_eq!(options1.fooBar, 123);
        assert_eq!(options1.v, true);
        assert_eq!(options2.qux, true);
        assert_eq!(options2.q, "ABC");
    }
}
