// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

mod parse_;
mod parse_with;

mod parse_for;
pub use parse_for::make_opt_cfgs_for;
pub use parse_for::OptStore;

use crate::errors::InvalidOption;

fn parse_args<'a, F1, F2, F3>(
    args: &[&'a str],
    mut collect_args: F1,
    mut collect_opts: F2,
    take_opt_args: F3,
    until_1st_arg: bool,
    is_after_end_opt: bool,
) -> Result<Option<(usize, bool)>, InvalidOption>
where
    F1: FnMut(&'a str),
    F2: FnMut(&'a str, Option<&'a str>) -> Result<(), InvalidOption>,
    F3: Fn(&str) -> bool,
{
    let mut is_non_opt = is_after_end_opt;
    let mut prev_opt_taking_args = "";
    let mut first_err: Option<InvalidOption> = None;

    'L0: for (i_arg, arg) in args.iter().enumerate() {
        if is_non_opt {
            if until_1st_arg {
                if let Some(err) = first_err {
                    return Err(err);
                }
                return Ok(Some((i_arg, is_non_opt)));
            }
            collect_args(arg);
        } else if !prev_opt_taking_args.is_empty() {
            match collect_opts(prev_opt_taking_args, Some(arg)) {
                Err(err) => {
                    prev_opt_taking_args = "";
                    if first_err.is_none() {
                        first_err = Some(err);
                    }
                    continue 'L0;
                }
                Ok(_) => {
                    prev_opt_taking_args = "";
                }
            }
        } else if let Some(arg) = arg.strip_prefix("--") {
            if arg.is_empty() {
                is_non_opt = true;
                continue 'L0;
            }

            let mut i = 0;

            for ch in arg.chars() {
                if i > 0 {
                    if ch == '=' {
                        if let Err(err) = collect_opts(&arg[0..i], Some(&arg[i + 1..])) {
                            if first_err.is_none() {
                                first_err = Some(err);
                            }
                            continue 'L0;
                        }
                        break;
                    }
                    if !is_allowed_character(ch) {
                        if first_err.is_none() {
                            first_err = Some(InvalidOption::OptionContainsInvalidChar {
                                option: String::from(arg),
                            });
                        }
                        continue 'L0;
                    }
                } else if !is_allowed_first_character(ch) {
                    if first_err.is_none() {
                        first_err = Some(InvalidOption::OptionContainsInvalidChar {
                            option: String::from(arg),
                        });
                    }
                    continue 'L0;
                }
                i += 1;
            }

            if i == arg.len() {
                if take_opt_args(arg) && i_arg < args.len() - 1 {
                    prev_opt_taking_args = arg;
                    continue 'L0;
                }
                if let Err(err) = collect_opts(arg, None) {
                    if first_err.is_none() {
                        first_err = Some(err);
                    }
                    continue 'L0;
                }
            }
        } else if let Some(arg) = arg.strip_prefix("-") {
            if arg.is_empty() {
                if until_1st_arg {
                    if let Some(err) = first_err {
                        return Err(err);
                    }
                    return Ok(Some((i_arg, is_non_opt)));
                }
                collect_args("-");
                continue 'L0;
            }

            let mut name: &str = "";
            let mut i = 0;

            for ch in arg.chars() {
                if i > 0 {
                    if ch == '=' {
                        if !name.is_empty() {
                            if let Err(err) = collect_opts(name, Some(&arg[i + 1..])) {
                                if first_err.is_none() {
                                    first_err = Some(err);
                                }
                            }
                        }
                        continue 'L0;
                    }
                    if !name.is_empty() {
                        if let Err(err) = collect_opts(name, None) {
                            if first_err.is_none() {
                                first_err = Some(err);
                            }
                        }
                    }
                }
                if !is_allowed_first_character(ch) {
                    if first_err.is_none() {
                        first_err = Some(InvalidOption::OptionContainsInvalidChar {
                            option: String::from(&arg[i..i + 1]),
                        });
                    }
                    name = "";
                } else {
                    name = &arg[i..i + 1];
                }
                i += 1;
            }

            if i == arg.len() && !name.is_empty() {
                if take_opt_args(name) && i_arg < args.len() - 1 {
                    prev_opt_taking_args = name;
                } else if let Err(err) = collect_opts(name, None) {
                    if first_err.is_none() {
                        first_err = Some(err);
                    }
                    continue 'L0;
                }
            }
        } else {
            if until_1st_arg {
                if let Some(err) = first_err {
                    return Err(err);
                }
                return Ok(Some((i_arg, is_non_opt)));
            }
            collect_args(arg);
        }
    }

    match first_err {
        Some(err) => Err(err),
        None => Ok(None),
    }
}

#[inline]
fn is_allowed_character(ch: char) -> bool {
    ch == '-' || ch.is_ascii_alphabetic() || ch.is_ascii_digit()
}

#[inline]
fn is_allowed_first_character(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}
