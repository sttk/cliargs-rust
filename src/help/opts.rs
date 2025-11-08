// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::OptCfg;

#[allow(clippy::single_component_path_imports)]
use linebreak;

pub fn create_opts_help(cfgs: &[OptCfg], indent_ref: &mut usize) -> Vec<(usize, String)> {
    let mut ret = Vec::<(usize, String)>::with_capacity(cfgs.len());
    let indent = *indent_ref;

    const ANY_OPT: &str = "*";

    if indent > 0 {
        for cfg in cfgs {
            let store_key: &str = if !cfg.store_key.is_empty() {
                &cfg.store_key
            } else if let Some(name) = cfg.names.iter().find(|nm| !nm.is_empty()) {
                name
            } else {
                ""
            };

            if store_key.is_empty() {
                continue;
            }

            if store_key == ANY_OPT {
                continue;
            }

            let (first_indent, mut text) = make_opt_title(cfg);

            let width = first_indent + linebreak::text_width(&text);

            if !cfg.desc.is_empty() {
                if width + 2 > indent {
                    text.push('\n');
                    text.push_str(&" ".repeat(indent));
                    text.push_str(&cfg.desc);
                } else {
                    text.push_str(&" ".repeat(indent - width));
                    text.push_str(&cfg.desc);
                }
            }

            ret.push((first_indent, text));
        }
    } else {
        let mut widths = Vec::<usize>::with_capacity(ret.len());
        let mut indent = 0;

        for cfg in cfgs {
            let store_key: &str = if !cfg.store_key.is_empty() {
                &cfg.store_key
            } else if let Some(name) = cfg.names.iter().find(|nm| !nm.is_empty()) {
                name
            } else {
                ""
            };

            if store_key.is_empty() {
                continue;
            }

            if store_key == ANY_OPT {
                continue;
            }

            let (first_indent, text) = make_opt_title(cfg);

            let width = first_indent + linebreak::text_width(&text);
            if indent < width {
                indent = width;
            }

            ret.push((first_indent, text));
            widths.push(width);
        }

        indent += 2;
        *indent_ref = indent;

        let mut i = 0;
        for cfg in cfgs {
            let store_key: &str = if !cfg.store_key.is_empty() {
                &cfg.store_key
            } else if let Some(name) = cfg.names.iter().find(|nm| !nm.is_empty()) {
                name
            } else {
                ""
            };

            if store_key.is_empty() {
                continue;
            }

            if store_key == ANY_OPT {
                continue;
            }

            if !cfg.desc.is_empty() {
                ret[i].1.push_str(&" ".repeat(indent - widths[i]));
                ret[i].1.push_str(&cfg.desc);
            }

            i += 1;
        }
    }

    ret
}

fn make_opt_title(cfg: &OptCfg) -> (usize, String) {
    let mut head_spaces = 0;
    let mut last_spaces = 0;
    let mut title = String::from("");
    let mut use_store_key = true;

    let n = cfg.names.len();

    for (i, name) in cfg.names.iter().enumerate() {
        match name.len() {
            0 => {
                if title.is_empty() {
                    head_spaces += 4;
                } else if i != n - 1 {
                    last_spaces += 4;
                } else {
                    last_spaces += 2;
                }
            }
            1 => {
                if last_spaces > 0 {
                    title.push(',');
                    title.push_str(&" ".repeat(last_spaces - 1));
                }
                last_spaces = 0;
                title.push('-');
                title.push_str(name);
                if i != n - 1 {
                    last_spaces += 2;
                }
                use_store_key = false;
            }
            _ => {
                if last_spaces > 0 {
                    title.push(',');
                    title.push_str(&" ".repeat(last_spaces - 1));
                }
                last_spaces = 0;
                title.push_str("--");
                title.push_str(name);
                if i != n - 1 {
                    last_spaces += 2;
                }
                use_store_key = false;
            }
        }
    }

    if use_store_key {
        match cfg.store_key.len() {
            0 => {}
            1 => {
                if last_spaces > 0 {
                    title.push(',');
                    title.push_str(&" ".repeat(last_spaces - 1));
                }
                title.push('-');
                title.push_str(&cfg.store_key);
            }
            _ => {
                if last_spaces > 0 {
                    title.push(',');
                    title.push_str(&" ".repeat(last_spaces - 1));
                }
                title.push_str("--");
                title.push_str(&cfg.store_key);
            }
        }
    }

    if !cfg.arg_in_help.is_empty() {
        title.push(' ');
        title.push_str(&cfg.arg_in_help);
    }

    (head_spaces, title)
}

#[cfg(test)]
mod tests_of_make_opt_title {
    use super::*;
    use crate::OptCfgParam::*;

    #[test]
    fn test_when_cfg_has_only_one_long_name() {
        let cfg = OptCfg::with([names(&["foo-bar"])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 0);
        assert_eq!(title, "--foo-bar");
    }

    #[test]
    fn test_when_cfg_has_only_one_short_name() {
        let cfg = OptCfg::with([names(&["f"])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 0);
        assert_eq!(title, "-f");
    }

    #[test]
    fn test_when_cfg_has_multiple_names() {
        let cfg = OptCfg::with([names(&["f", "b", "foo-bar"])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 0);
        assert_eq!(title, "-f, -b, --foo-bar");
    }

    #[test]
    fn test_when_cfg_has_no_name_but_store_key() {
        let cfg = OptCfg::with([store_key("Foo_Bar")]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 0);
        assert_eq!(title, "--Foo_Bar");
    }

    #[test]
    fn test_when_cfg_names_contain_emtpy_name() {
        let cfg = OptCfg::with([names(&["", "f"])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 4);
        assert_eq!(title, "-f");

        let cfg = OptCfg::with([names(&["", "f", "", "b", ""])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 4);
        assert_eq!(title, "-f,     -b");
    }

    #[test]
    fn test_when_cfg_names_are_all_empty_and_has_store_key() {
        let cfg = OptCfg::with([store_key("FooBar"), names(&["", ""])]);
        let (indent, title) = make_opt_title(&cfg);
        assert_eq!(indent, 8);
        assert_eq!(title, "--FooBar");
    }
}

#[cfg(test)]
mod tests_of_create_opts_help {
    use super::*;
    use crate::OptCfgParam::*;

    #[test]
    fn test_when_cfg_has_only_one_long_name() {
        let cfgs = vec![OptCfg::with([names(&["foo-bar"])])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "--foo-bar");
        assert_eq!(indent, 11);
    }

    #[test]
    fn test_when_cfg_has_only_one_long_name_and_desc() {
        let cfgs = vec![OptCfg::with([
            names(&["foo-bar"]),
            desc("The description of foo-bar."),
        ])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "--foo-bar  The description of foo-bar.");
        assert_eq!(indent, 11);
    }

    #[test]
    fn test_when_cfg_has_only_one_long_name_and_desc_and_arg_in_help() {
        let cfgs = vec![OptCfg::with([
            names(&["foo-bar"]),
            desc("The description of foo-bar."),
            arg_in_help("<num>"),
        ])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "--foo-bar <num>  The description of foo-bar.");
        assert_eq!(indent, 17);
    }

    #[test]
    fn test_when_cfg_has_only_one_short_name() {
        let cfgs = vec![OptCfg::with([names(&["f"])])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "-f");
        assert_eq!(indent, 4);
    }

    #[test]
    fn test_when_cfg_has_only_one_short_name_and_desc() {
        let cfgs = vec![OptCfg::with([names(&["f"]), desc("The description of f.")])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "-f  The description of f.");
        assert_eq!(indent, 4);
    }

    #[test]
    fn test_when_cfg_has_only_one_short_name_and_desc_and_arg_in_help() {
        let cfgs = vec![OptCfg::with([
            names(&["f"]),
            desc("The description of f."),
            arg_in_help("<n>"),
        ])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "-f <n>  The description of f.");
        assert_eq!(indent, 8);
    }

    #[test]
    fn test_when_indent_is_positive_and_longer_than_title() {
        let cfgs = vec![OptCfg::with([
            names(&["foo-bar"]),
            desc("The description of foo-bar."),
            arg_in_help("<num>"),
        ])];

        let mut indent = 19;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(text, "--foo-bar <num>    The description of foo-bar.");
        assert_eq!(indent, 19);
    }

    #[test]
    fn test_when_indent_is_positive_and_shorter_than_title() {
        let cfgs = vec![OptCfg::with([
            names(&["foo-bar"]),
            desc("The description of foo-bar."),
            arg_in_help("<num>"),
        ])];

        let mut indent = 16;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(
            text,
            "--foo-bar <num>\n                The description of foo-bar."
        );
        assert_eq!(indent, 16);

        let mut indent = 10;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 0);
        assert_eq!(
            text,
            "--foo-bar <num>\n          The description of foo-bar."
        );
        assert_eq!(indent, 10);
    }

    #[test]
    fn test_when_names_contains_empty_strings() {
        let cfgs = vec![OptCfg::with([
            names(&["", "", "f", "", "foo-bar", ""]),
            desc("The description of foo-bar."),
            arg_in_help("<num>"),
        ])];

        let mut indent = 0;
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 8);
        assert_eq!(text, "-f,     --foo-bar <num>  The description of foo-bar.");
        assert_eq!(indent, 8 + 25);

        let mut indent = 35; // longer than title width
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 8);
        assert_eq!(
            text,
            "-f,     --foo-bar <num>    The description of foo-bar."
        );
        assert_eq!(indent, 35);

        let mut indent = 33; // equal to title width
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 8);
        assert_eq!(text, "-f,     --foo-bar <num>  The description of foo-bar.");
        assert_eq!(indent, 33);

        let mut indent = 32; // shorter than title width
        let vec = create_opts_help(&cfgs, &mut indent);

        assert_eq!(vec.len(), 1);
        let (first_indent, text) = &vec[0];
        assert_eq!(*first_indent, 8);
        assert_eq!(
            text,
            &("-f,     --foo-bar <num>\n".to_string()
                + &" ".repeat(32)
                + "The description of foo-bar.")
        );
        assert_eq!(indent, 32);
    }
}
