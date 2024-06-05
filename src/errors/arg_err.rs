// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use std::error;
use std::ffi;
use std::fmt;

/// The enum type for errors of `OsString` arguments.
///
/// The variants of this enum indicates errors that can occur when operating
/// command line arguments represented by `OsString`.
#[derive(Debug, PartialEq)]
pub enum InvalidOsArg {
    /// The enum variant which indicates that at least one `OsString` value in
    /// the command line arguments is invalid Unicode.
    OsArgsContainInvalidUnicode {
        /// The index of the invalid argument.
        /// The argument of which index is zero is the command path.
        index: usize,
        /// The `OsString` value of the invalid argument.
        os_arg: ffi::OsString,
    },
}

impl fmt::Display for InvalidOsArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg } => write!(
                f,
                "The command line arguments contains invalid unicode (index: {}, arguments: \"{}\")",
                index,
                String::from_utf8_lossy(os_arg.as_encoded_bytes()).escape_debug(),
            ),
        }
    }
}

impl error::Error for InvalidOsArg {}

#[cfg(not(windows))] // Because basically OsStr is valid WTF8 and OsString is valid WTF16 on Windows
#[cfg(test)]
mod tests_of_invalid_os_arg {
    use super::*;

    mod tests_of_os_args_contain_invalid_unicode {
        use super::*;
        use std::error;
        use std::ffi;

        #[test]
        fn should_create_and_handles() {
            let bad_input = b"Hello \xF0\x90\x80World";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), InvalidOsArg> = Err(InvalidOsArg::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });

            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(format!("{err}"), "The command line arguments contains invalid unicode (index: 12, arguments: \"Hello �World\")");
                    assert_eq!(format!("{err:?}"), "OsArgsContainInvalidUnicode { index: 12, os_arg: \"Hello \\xF0\\x90\\x80World\" }");
                }
            }

            match result {
                Ok(_) => assert!(false),
                Err(ref err) => match err {
                    InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg } => {
                        assert_eq!(*index, 12);
                        assert_eq!(format!("{os_arg:?}"), "\"Hello \\xF0\\x90\\x80World\"");
                    }
                },
            }

            match result {
                Ok(_) => assert!(false),
                Err(InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg }) => {
                    assert_eq!(index, 12);
                    assert_eq!(format!("{os_arg:?}"), "\"Hello \\xF0\\x90\\x80World\"");
                }
            }
        }

        #[test]
        fn should_write_for_debug() {
            let bad_input = b"Hello \xF0\x90\x80World";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), InvalidOsArg> = Err(InvalidOsArg::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });

            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(format!("{err:?}"), "OsArgsContainInvalidUnicode { index: 12, os_arg: \"Hello \\xF0\\x90\\x80World\" }");
                }
            }
        }

        #[test]
        fn should_write_for_display() {
            let bad_input = b"Hello \xF0\x90\x80World";
            let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
            let bad_os_string = bad_os_str.to_os_string();

            let result: Result<(), InvalidOsArg> = Err(InvalidOsArg::OsArgsContainInvalidUnicode {
                index: 12,
                os_arg: bad_os_string.clone(),
            });

            match result {
                Ok(_) => assert!(false),
                Err(ref err) => {
                    assert_eq!(format!("{err}"), "The command line arguments contains invalid unicode (index: 12, arguments: \"Hello �World\")");
                }
            }
        }

        #[test]
        fn should_handle_as_dyn_std_error() {
            fn returns_error() -> Result<(), InvalidOsArg> {
                let bad_input = b"Hello \xF0\x90\x80World";
                let bad_os_str = unsafe { ffi::OsStr::from_encoded_bytes_unchecked(bad_input) };
                let bad_os_string = bad_os_str.to_os_string();

                Err(InvalidOsArg::OsArgsContainInvalidUnicode {
                    index: 12,
                    os_arg: bad_os_string.clone(),
                })
            }

            fn returns_dyn_std_error() -> Result<(), Box<dyn error::Error>> {
                returns_error()?;
                Ok(())
            }

            match returns_dyn_std_error() {
                Ok(_) => assert!(false),
                Err(err) => {
                    //println!("{err:?}");
                    if let Some(os_arg_err) = err.downcast_ref::<InvalidOsArg>() {
                        match os_arg_err {
                            InvalidOsArg::OsArgsContainInvalidUnicode { index, os_arg } => {
                                assert_eq!(*index, 12);
                                assert_eq!(
                                    format!("{:?}", os_arg),
                                    "\"Hello \\xF0\\x90\\x80World\""
                                );
                            }
                        }
                    } else {
                        assert!(false);
                    }
                }
            }
        }
    }
}
