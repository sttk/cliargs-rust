// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::errors::InvalidOption;
use std::fmt;
use std::ops;
use std::str;

/// Validates an option argument string whether it is valid as a number value of the specified
/// type.
///
/// If the option argument is invalid, this funciton returns a `InvalidOption::OptionArgIsInvalid`
/// instance.
pub fn validate_number<T>(store_key: &str, option: &str, opt_arg: &str) -> Result<(), InvalidOption>
where
    T: str::FromStr
        + ops::Add
        + ops::Div
        + ops::Mul
        + ops::Sub
        + ops::Rem
        + Copy
        + PartialEq
        + PartialOrd,
    <T as str::FromStr>::Err: fmt::Display,
{
    match opt_arg.parse::<T>() {
        Ok(_) => Ok(()),
        Err(err) => Err(InvalidOption::OptionArgIsInvalid {
            store_key: store_key.to_string(),
            option: option.to_string(),
            opt_arg: opt_arg.to_string(),
            details: format!("{}", err),
        }),
    }
}

#[cfg(test)]
mod tests_of_validators {
    use super::*;

    mod test_of_validate_number {
        use super::*;

        #[test]
        fn should_validate_i8() {
            assert_eq!(validate_number::<i8>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<i8>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(validate_number::<i8>("FooBar", "foo-bar", "-123"), Ok(()));

            match validate_number::<i8>("FooBar", "foo-bar", "128") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "128");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i8>("FooBar", "foo-bar", "-129") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-129");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i8>("FooBar", "foo-bar", "1e1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "1e1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i8>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i8>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_i16() {
            assert_eq!(validate_number::<i16>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<i16>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(validate_number::<i16>("FooBar", "foo-bar", "-123"), Ok(()));

            match validate_number::<i16>("FooBar", "foo-bar", "32768") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "32768");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i16>("FooBar", "foo-bar", "-32769") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-32769");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i16>("FooBar", "foo-bar", "1e3") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "1e3");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i16>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i16>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_i32() {
            assert_eq!(validate_number::<i32>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<i32>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(validate_number::<i32>("FooBar", "foo-bar", "-123"), Ok(()));

            match validate_number::<i32>("FooBar", "foo-bar", "2147483648") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "2147483648");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i32>("FooBar", "foo-bar", "-2147483649") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-2147483649");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i32>("FooBar", "foo-bar", "1e+3") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "1e+3");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i32>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i32>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_i64() {
            assert_eq!(validate_number::<i64>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<i64>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(validate_number::<i64>("FooBar", "foo-bar", "-123"), Ok(()));

            match validate_number::<i64>("FooBar", "foo-bar", "9223372036854775808") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "9223372036854775808");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i64>("FooBar", "foo-bar", "-9223372036854775809") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-9223372036854775809");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i64>("FooBar", "foo-bar", "100e-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "100e-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i64>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i64>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_i128() {
            assert_eq!(validate_number::<i128>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<i128>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(validate_number::<i128>("FooBar", "foo-bar", "-123"), Ok(()));

            match validate_number::<i128>(
                "FooBar",
                "foo-bar",
                "170141183460469231731687303715884105728",
            ) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "170141183460469231731687303715884105728");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i128>(
                "FooBar",
                "foo-bar",
                "-170141183460469231731687303715884105729",
            ) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-170141183460469231731687303715884105729");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i128>("FooBar", "foo-bar", "1E+3") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "1E+3");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i128>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<i128>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_isize() {
            assert_eq!(validate_number::<isize>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<isize>("FooBar", "foo-bar", "123"), Ok(()));
            assert_eq!(
                validate_number::<isize>("FooBar", "foo-bar", "-123"),
                Ok(())
            );

            match validate_number::<i64>("FooBar", "foo-bar", "9223372036854775808") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "9223372036854775808");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<isize>("FooBar", "foo-bar", "-9223372036854775809") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-9223372036854775809");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<isize>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<isize>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_u8() {
            assert_eq!(validate_number::<u8>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<u8>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<u8>("FooBar", "foo-bar", "256") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "256");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u8>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u8>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u8>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_u16() {
            assert_eq!(validate_number::<u16>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<u16>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<u16>("FooBar", "foo-bar", "65536") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "65536");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u16>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u16>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u16>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_u32() {
            assert_eq!(validate_number::<u32>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<u32>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<u32>("FooBar", "foo-bar", "4294967296") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "4294967296");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u32>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u32>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u32>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_u64() {
            assert_eq!(validate_number::<u64>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<u64>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<u64>("FooBar", "foo-bar", "18446744073709551616") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "18446744073709551616");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u64>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u64>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u64>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_u128() {
            assert_eq!(validate_number::<u128>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<u128>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<u128>(
                "FooBar",
                "foo-bar",
                "340282366920938463463374607431768211456",
            ) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "340282366920938463463374607431768211456");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u128>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u128>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<u128>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_usize() {
            assert_eq!(validate_number::<usize>("FooBar", "foo-bar", "0"), Ok(()));
            assert_eq!(validate_number::<usize>("FooBar", "foo-bar", "123"), Ok(()));

            match validate_number::<usize>("FooBar", "foo-bar", "18446744073709551616") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "18446744073709551616");
                    assert_eq!(details, "number too large to fit in target type");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<usize>("FooBar", "foo-bar", "-1") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "-1");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<usize>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<usize>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_f32() {
            assert_eq!(validate_number::<f32>("FooBar", "foo-bar", "0.0"), Ok(()));
            assert_eq!(validate_number::<f32>("FooBar", "foo-bar", "1.23"), Ok(()));

            // floating point number literal exceeding f32::MAX is valid as f32::INFINITY.

            match validate_number::<f32>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid float literal");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<f32>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid float literal");
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn should_validate_f64() {
            assert_eq!(validate_number::<f64>("FooBar", "foo-bar", "0.0"), Ok(()));
            assert_eq!(validate_number::<f64>("FooBar", "foo-bar", "1.23"), Ok(()));

            // floating point number literal exceeding f64::MAX is valid as f64::INFINITY.

            match validate_number::<f64>("FooBar", "foo-bar", "0x0a") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "0x0a");
                    assert_eq!(details, "invalid float literal");
                }
                Err(_) => assert!(false),
            }
            match validate_number::<f64>("FooBar", "foo-bar", "abc") {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "FooBar");
                    assert_eq!(option, "foo-bar");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid float literal");
                }
                Err(_) => assert!(false),
            }
        }
    }
}
