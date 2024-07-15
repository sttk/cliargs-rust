#[cfg(test)]
mod tests_of_opt_cfg {
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
}
