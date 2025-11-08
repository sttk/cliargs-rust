#[cfg(test)]
mod tests_of_parse {
    use cliargs;
    use std::ffi;

    #[test]
    fn it_should_parse_command_line_arguments() {
        let mut cmd = cliargs::Cmd::new().unwrap();
        match cmd.parse() {
            Ok(_) => {}
            Err(_) => assert!(false),
        }
        println!("cmd = {cmd:?}");
        assert!(cmd.name().starts_with("parse_test-"));
        assert!(cmd.opt_cfgs().is_empty());
    }

    #[test]
    fn it_should_parse_strings_as_command_line_arguments() {
        let mut cmd = cliargs::Cmd::with_strings([
            "/path/to/app".to_string(),
            "--foo-bar=123".to_string(),
            "bar".to_string(),
            "--baz".to_string(),
            "qux".to_string(),
        ]);
        match cmd.parse() {
            Ok(_) => {}
            Err(_) => assert!(false),
        }
        println!("cmd = {cmd:?}");
        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), ["bar", "qux"]);
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("123"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["123"] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), None);
        assert_eq!(cmd.opt_args("baz"), Some(&[] as &[&str]));
        assert!(cmd.opt_cfgs().is_empty());
    }

    #[test]
    fn it_should_parse_os_strings_as_command_line_arguments() {
        let mut cmd = cliargs::Cmd::with_os_strings([
            ffi::OsString::from("/path/to/app"),
            ffi::OsString::from("--foo-bar=123"),
            ffi::OsString::from("bar"),
            ffi::OsString::from("--baz"),
            ffi::OsString::from("qux"),
        ])
        .unwrap();
        match cmd.parse() {
            Ok(_) => {}
            Err(_) => assert!(false),
        }
        println!("cmd = {cmd:?}");
        assert_eq!(cmd.name(), "app");
        assert_eq!(cmd.args(), ["bar", "qux"]);
        assert_eq!(cmd.has_opt("foo-bar"), true);
        assert_eq!(cmd.opt_arg("foo-bar"), Some("123"));
        assert_eq!(cmd.opt_args("foo-bar"), Some(&["123"] as &[&str]));
        assert_eq!(cmd.has_opt("baz"), true);
        assert_eq!(cmd.opt_arg("baz"), None);
        assert_eq!(cmd.opt_args("baz"), Some(&[] as &[&str]));
        assert!(cmd.opt_cfgs().is_empty());
    }
}

mod tests_of_parse_until_sub_cmd {
    use cliargs;

    #[test]
    fn it_should_parse_command_line_arguments_containing_subcommand() {
        let mut cmd = cliargs::Cmd::with_strings([
            "/path/to/app".to_string(),
            "--foo-bar=123".to_string(),
            "-v".to_string(),
            "baz".to_string(),
            "--qux".to_string(),
            "corge".to_string(),
            "-q=ABC".to_string(),
        ]);

        if let Some(mut sub_cmd) = cmd.parse_until_sub_cmd().unwrap() {
            let _ = sub_cmd.parse().unwrap();

            assert_eq!(cmd.name(), "app");
            assert_eq!(cmd.args(), &[] as &[&str]);
            assert_eq!(cmd.has_opt("foo-bar"), true);
            assert_eq!(cmd.opt_arg("foo-bar"), Some("123"));
            assert_eq!(cmd.opt_args("foo-bar"), Some(&["123"] as &[&str]));
            assert_eq!(cmd.has_opt("v"), true);
            assert_eq!(cmd.opt_arg("v"), None);
            assert_eq!(cmd.opt_args("v"), Some(&[] as &[&str]));

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
    }
}

#[cfg(test)]
mod tests_of_errors {
    use cliargs;
    use std::ffi;

    #[cfg(not(windows))] // Because basically OsStr is valid WTF8 and OsString is valid WTF16 on windows
    #[test]
    fn it_should_parse_but_fail_because_command_line_arguments_contain_invalid_unicode() {
        let bad_arg = b"bar\xFF";
        let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_arg) };
        let bad_os_string = bad_os_str.to_os_string();

        match cliargs::Cmd::with_os_strings([
            ffi::OsString::from("/path/to/app"),
            ffi::OsString::from("--foo-bar=123"),
            bad_os_string.clone(),
            ffi::OsString::from("--baz"),
            ffi::OsString::from("qux"),
        ]) {
            Ok(_) => assert!(false),
            Err(cliargs::errors::InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                assert_eq!(index, 2);
                assert_eq!(os_arg, bad_os_string);
            }
        }
    }

    #[test]
    fn it_should_parse_but_fail_because_option_contains_invalid_char() {
        let mut cmd = cliargs::Cmd::with_strings([
            "/path/to/app".to_string(),
            "--foo-bar=123".to_string(),
            "--b@z".to_string(),
            "qux".to_string(),
        ]);
        match cmd.parse() {
            Ok(_) => assert!(false),
            Err(cliargs::errors::InvalidOption::OptionContainsInvalidChar { option }) => {
                assert_eq!(option, "b@z");
            }
            Err(_) => assert!(false),
        }
    }
}

#[rustversion::since(1.91)]
#[test]
fn compile_error_check() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_errors/*.rs");
}

#[rustversion::before(1.91)]
#[test]
fn compile_error_check() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_errors/less_than_1_90/*.rs");
}
