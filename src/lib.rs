// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

pub mod errors;

mod parse;

use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::mem;
use std::path;

pub struct Cmd<'a> {
    name: &'a str,
    args: Vec<&'a str>,
    opts: HashMap<&'a str, Vec<&'a str>>,

    _arg_refs: Vec<&'a str>,
}

impl<'a> Drop for Cmd<'a> {
    fn drop(&mut self) {
        for str in &self._arg_refs {
            let boxed = unsafe { Box::from_raw(*str as *const str as *mut str) };
            mem::drop(boxed);
        }
    }
}

impl fmt::Debug for Cmd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cmd")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("opts", &self.opts)
            .finish()
    }
}

impl<'a> Cmd<'a> {
    pub fn new() -> Result<Cmd<'a>, errors::InvalidOsArg> {
        Self::with_os_strings(env::args_os())
    }

    pub fn with_os_strings(
        osargs: impl IntoIterator<Item = OsString>,
    ) -> Result<Cmd<'a>, errors::InvalidOsArg> {
        let osarg_iter = osargs.into_iter();
        let (size, _) = osarg_iter.size_hint();
        let mut _arg_refs = Vec::with_capacity(size);

        let cmd_name_start: usize;

        let mut enm = osarg_iter.enumerate();
        if let Some((idx, osarg)) = enm.next() {
            // The first element is the command path.
            let path = path::Path::new(&osarg);
            let base_len = if let Some(base_os) = path.file_name() {
                if let Some(base_str) = base_os.to_str() {
                    base_str.len()
                } else {
                    0
                }
            } else {
                0
            };
            match osarg.into_string() {
                Ok(string) => {
                    let str: &'a str = string.leak();
                    _arg_refs.push(str);
                    cmd_name_start = str.len() - base_len;
                }
                Err(osstring) => {
                    return Err(errors::InvalidOsArg::OsArgsContainInvalidUnicode {
                        index: idx,
                        os_arg: osstring,
                    });
                }
            }

            // The elements from the second one onward are the arguments.
            for (idx, osarg) in enm {
                match osarg.into_string() {
                    Ok(string) => {
                        let str: &'a str = string.leak();
                        _arg_refs.push(str);
                    }
                    Err(osstring) => {
                        for str in _arg_refs {
                            let boxed = unsafe { Box::from_raw(str as *const str as *mut str) };
                            mem::drop(boxed);
                        }
                        return Err(errors::InvalidOsArg::OsArgsContainInvalidUnicode {
                            index: idx,
                            os_arg: osstring,
                        });
                    }
                }
            }
        } else {
            _arg_refs.push("");
            cmd_name_start = 0;
        }

        Ok(Cmd {
            name: &_arg_refs[0][cmd_name_start..],
            args: Vec::new(),
            opts: HashMap::new(),
            _arg_refs,
        })
    }

    pub fn with_strings(args: impl IntoIterator<Item = String>) -> Cmd<'a> {
        let arg_iter = args.into_iter();
        let (size, _) = arg_iter.size_hint();
        let mut _arg_refs = Vec::with_capacity(size);

        for arg in arg_iter {
            let str: &'a str = arg.leak();
            _arg_refs.push(str);
        }

        let cmd_name_start: usize;

        if _arg_refs.len() > 0 {
            let path = path::Path::new(_arg_refs[0]);
            let mut base_len = 0;
            if let Some(base_os) = path.file_name() {
                if let Some(base_str) = base_os.to_str() {
                    base_len = base_str.len();
                }
            }
            cmd_name_start = _arg_refs[0].len() - base_len;
        } else {
            _arg_refs.push("");
            cmd_name_start = 0;
        };

        Cmd {
            name: &_arg_refs[0][cmd_name_start..],
            args: Vec::new(),
            opts: HashMap::new(),
            _arg_refs,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn args(&self) -> &[&'a str] {
        &self.args
    }

    pub fn has_opt(&self, name: &str) -> bool {
        self.opts.contains_key(name)
    }

    pub fn opt_arg(&self, name: &str) -> Option<&str> {
        if let Some(opt_vec) = self.opts.get(name) {
            if opt_vec.len() > 0 {
                return Some(opt_vec[0]);
            }
        }
        None
    }

    pub fn opt_args(&self, name: &str) -> Option<&[&'a str]> {
        match self.opts.get(name) {
            Some(vec) => Some(&vec),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests_of_cmd {
    use super::Cmd;

    mod tests_of_new {
        use super::Cmd;

        #[test]
        fn should_create_a_new_instance() {
            let cmd = Cmd::new().unwrap();
            println!("cmd = {cmd:?}");
            println!("cmd._arg_refs = {:?}", cmd._arg_refs);
            assert!(cmd.name().starts_with("cliargs-"));
            assert!(cmd._arg_refs.len() > 0);
        }
    }

    mod tests_of_with_strings {
        use super::Cmd;

        #[test]
        fn should_create_a_new_instance() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "bar".to_string(),
            ]);

            cmd.args.push(cmd._arg_refs[2]);
            cmd.opts
                .insert(&cmd._arg_refs[1][2..], Vec::with_capacity(0));

            println!("cmd = {cmd:?}");
            println!("cmd._arg_refs = {:?}", cmd._arg_refs);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_absolute_path() {
            let cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_relative_path() {
            let cmd = Cmd::with_strings([
                "../path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_from_base_name_only() {
            let cmd = Cmd::with_strings([
                "app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);
            assert_eq!(cmd.name(), "app");
        }

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            let cmd = Cmd::with_strings([]);
            assert_eq!(cmd.name(), "");
        }
    }

    mod tests_of_with_os_strings {
        use super::Cmd;
        use std::ffi;

        #[test]
        fn should_create_a_new_instance() {
            let cmd = Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("bar_baz"),
                ffi::OsString::from("qux"),
            ])
            .unwrap();

            assert_eq!(cmd.name(), "app");
        }

        #[cfg(not(windows))] // Because basically OsStr is valid WTF8 and OsString is valid WTF16 on windows
        #[test]
        fn should_fail_because_os_args_contain_invalid_unicode() {
            let bad_arg = b"bar\xFFbaz";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_arg) };
            let bad_os_string = bad_os_str.to_os_string();

            match Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                bad_os_string.clone(),
                ffi::OsString::from("qux"),
            ]) {
                Ok(_) => assert!(false),
                Err(crate::errors::InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                    assert_eq!(index, 2);
                    assert_eq!(os_arg, bad_os_string);
                }
            }
        }

        #[cfg(not(windows))] // Because basically OsStr is valid WTF8 and OsString is valid WTF16 on windows
        #[test]
        fn should_fail_because_command_name_contains_invalid_unicode() {
            let bad_arg = b"bar\xFFbaz";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_arg) };
            let bad_os_string = bad_os_str.to_os_string();

            match Cmd::with_os_strings([
                bad_os_string.clone(),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("qux"),
            ]) {
                Ok(_) => assert!(false),
                Err(crate::errors::InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                    assert_eq!(index, 0);
                    assert_eq!(os_arg, bad_os_string);
                }
            }
        }

        #[test]
        fn should_get_command_name_from_absolute_path() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("/path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_from_relative_path() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("../path/to/app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_from_base_name_only() {
            if let Ok(cmd) = Cmd::with_os_strings([
                ffi::OsString::from("app"),
                ffi::OsString::from("--foo"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("baz"),
                ffi::OsString::from("--bar"),
                ffi::OsString::from("qux"),
                ffi::OsString::from("quux"),
                ffi::OsString::from("corge"),
            ]) {
                assert_eq!(cmd.name(), "app");
            } else {
                assert!(false);
            }
        }

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            if let Ok(cmd) = Cmd::with_os_strings([]) {
                assert_eq!(cmd.name(), "");
            } else {
                assert!(false);
            }
        }
    }

    mod tests_of_getters {
        use super::Cmd;

        #[test]
        fn should_get_command_name_when_command_line_arguments_is_empty() {
            let cmd = Cmd::with_strings([]);

            assert_eq!(cmd.name(), "");
        }

        #[test]
        fn should_get_command_arguments() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._arg_refs[6]);
            cmd.args.push(cmd._arg_refs[7]);
            cmd.opts
                .insert(&cmd._arg_refs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._arg_refs[2][2..],
                vec![&cmd._arg_refs[3], &cmd._arg_refs[5]],
            );

            assert_eq!(cmd.args(), ["quux", "corge"]);
        }

        #[test]
        fn should_check_option_is_specified() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._arg_refs[6]);
            cmd.args.push(cmd._arg_refs[7]);
            cmd.opts
                .insert(&cmd._arg_refs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._arg_refs[2][2..],
                vec![&cmd._arg_refs[3], &cmd._arg_refs[5]],
            );

            assert_eq!(cmd.has_opt("foo"), true);
            assert_eq!(cmd.has_opt("bar"), true);
            assert_eq!(cmd.has_opt("baz"), false);
        }

        #[test]
        fn should_get_single_option_argument() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._arg_refs[6]);
            cmd.args.push(cmd._arg_refs[7]);
            cmd.opts
                .insert(&cmd._arg_refs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._arg_refs[2][2..],
                vec![&cmd._arg_refs[3], &cmd._arg_refs[5]],
            );

            assert_eq!(cmd.opt_arg("foo"), None);
            assert_eq!(cmd.opt_arg("bar"), Some("baz"));
            assert_eq!(cmd.opt_arg("baz"), None);
        }

        #[test]
        fn should_get_multiple_option_arguments() {
            let mut cmd = Cmd::with_strings([
                "/path/to/app".to_string(),
                "--foo".to_string(),
                "--bar".to_string(),
                "baz".to_string(),
                "--bar".to_string(),
                "qux".to_string(),
                "quux".to_string(),
                "corge".to_string(),
            ]);

            cmd.args.push(cmd._arg_refs[6]);
            cmd.args.push(cmd._arg_refs[7]);
            cmd.opts
                .insert(&cmd._arg_refs[1][2..], Vec::with_capacity(0));
            cmd.opts.insert(
                &cmd._arg_refs[2][2..],
                vec![&cmd._arg_refs[3], &cmd._arg_refs[5]],
            );

            assert_eq!(cmd.opt_args("foo"), Some(&[] as &[&str]));
            assert_eq!(cmd.opt_args("bar"), Some(&["baz", "qux"] as &[&str]));
            assert_eq!(cmd.opt_args("baz"), None);
        }
    }
}
