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

        assert_eq!(cmd.cfgs.len(), 1);
        assert_eq!(cmd.cfgs[0].store_key, "foo_bar");
        assert_eq!(cmd.cfgs[0].names, ["f".to_string(), "b".to_string(), "foo-bar".to_string()]);
        assert_eq!(cmd.cfgs[0].has_arg, false);
        assert_eq!(cmd.cfgs[0].is_array, false);
        assert_eq!(cmd.cfgs[0].defaults, None);
        assert_eq!(cmd.cfgs[0].desc, "The FooBar flag".to_string());
        assert_eq!(cmd.cfgs[0].arg_in_help, "".to_string());
    }
}
