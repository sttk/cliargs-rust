// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use crate::errors::InvalidOption;
use crate::Cmd;
use crate::OptCfg;
use std::collections::HashMap;

/// Requires the method to make a [OptCfg] vector from the struct instance which implements this
/// trait.
pub trait OptStore {
    /// Makes a vector of [OptCfg] struct instances from the field definitions and `opt` field
    /// attributes of the struct instance.
    fn make_opt_cfgs(&self) -> Vec<OptCfg>;

    /// Sets the values in the argument map to the fields in this struct
    /// The key in the map is a store key of [OptCfg].
    fn set_field_values(&mut self, m: &HashMap<&str, Vec<&str>>) -> Result<(), InvalidOption>;
}

impl OptCfg {
    /// Makes a vector of [OptCfg] struct instances from the field definitions and `opt` field
    /// attributes of the struct instnace of which type is `T`.
    ///
    /// One [OptCfg] struct instance is made for each field.
    /// The field name is set to `store_key`.
    /// If the field's data type is `bool`, `has_arg` is set to `false`, otherwise, it is set to
    /// `true`.
    /// If the field is a vector type, `is_array` is set to `true`; otherwise, it is set to
    /// `false`.
    ///
    /// Additionally, `names`, `defaults`, `desc`, and `arg_in_help` are set with extracted from
    /// the `opt` attribute attached to the field.
    ///
    /// For `validator`, if the field's data type is numeric, it is set to the `validate_number`
    /// function pointer corresponding to the data type.
    pub fn make_cfgs_for<T: OptStore>(opt_store: &mut T) -> Vec<OptCfg> {
        opt_store.make_opt_cfgs()
    }
}

impl Cmd<'_> {
    /// Parses command line arguments and set their option values to the option store which is
    /// passed as an argument.
    ///
    /// This method divides command line arguments to command arguments and options, then sets
    /// each option value to a curresponding field of the option store.
    ///
    /// Within this method, a vector of [OptCfg] is made from the fields of the option store.
    /// This [OptCfg] vector is set to the public field `cfgs` of the [Cmd] instance.
    /// If you want to access this option configurations, get them from this field.
    ///
    /// An option configuration corresponding to each field of an option store is determined by
    /// its type and `opt` field attribute.
    /// If the type is bool, the option takes no argument.
    /// If the type is integer, floating point number or string, the option can takes single option
    /// argument, therefore it can appear once in command line arguments.
    /// If the type is a vector, the option can takes multiple option arguments, therefore it can
    /// appear multiple times in command line arguments.
    ///
    /// A `opt` field attribute can have the following pairs of name and value: one is `cfg` to
    /// specify `names` and `defaults` fields of [OptCfg] struct, another is `desc` to specify
    /// `desc` field, and yet another is `arg` to specify `arg_in_help` field.
    ///
    /// The format of `cfg` is like `cfg="f,foo=123"`.
    /// The left side of the equal sign is the option name(s), and the right side is the default
    /// value(s).
    /// If there is no equal sign, it is determined that only the option name is specified.
    /// If you want to specify multiple option names, separate them with commas.
    /// If you want to specify multiple default values, separate them with commas and round them
    /// with square brackets, like `[1,2,3]`.
    /// If you want to use your favorite carachter as a separator, you can use it by putting it on
    /// the left side of the open square bracket, like `/[1/2/3]`.
    ///
    /// NOTE: A default value of empty string array option in a field attribute is `[]`, like
    /// `#[opt(cfg="=[]")]`, but it doesn't represent an array which contains only one empty
    /// string.
    /// If you want to specify an array which contains only one emtpy string, write nothing after
    /// `=` symbol, like `#[opt(cfg="=")]`.
    ///
    /// ```
    /// use cliargs::Cmd;
    /// use cliargs::errors::InvalidOption;
    ///
    /// #[derive(cliargs::OptStore)]
    /// struct MyOptions {
    ///     #[opt(cfg = "f,foo-bar", desc="The description of foo_bar.")]
    ///     foo_bar: bool,
    ///     #[opt(cfg = "b,baz", desc="The description of baz.", arg="<s>")]
    ///     baz: String,
    /// }
    /// let mut my_options = MyOptions::with_defaults();
    ///
    /// let mut cmd = Cmd::with_strings(vec![ /* ... */ ]);
    /// match cmd.parse_for(&mut my_options) {
    ///     Ok(_) => { /* ... */ },
    ///     Err(InvalidOption::OptionContainsInvalidChar { option }) => { /* ... */ },
    ///     Err(InvalidOption::UnconfiguredOption { option }) => { /* ... */ },
    ///     Err(InvalidOption::OptionNeedsArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionTakesNoArg { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionIsNotArray { option, .. }) => { /* ... */ },
    ///     Err(InvalidOption::OptionArgIsInvalid { option, opt_arg, details, .. }) => { /* ... */ },
    ///     Err(err) => panic!("Invalid option: {}", err.option()),
    /// }
    ///
    /// let opt_cfgs = &cmd.cfgs;
    /// ```
    pub fn parse_for<T: OptStore>(&mut self, opt_store: &mut T) -> Result<(), InvalidOption> {
        let cfgs = opt_store.make_opt_cfgs();
        match self.parse_with(cfgs) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        opt_store.set_field_values(&self.opts)
    }
}

#[cfg(test)]
mod tests_of_make_cfgs_for {
    use super::*;
    use crate as cliargs;
    extern crate cliargs_derive;
    pub use cliargs_derive::OptStore;

    //#[derive(OptStore)]
    //enum MyEnum {}  // -> error

    //#[derive(OptStore)]
    //struct MyTuple(u8, u8); // -> error

    //#[derive(OptStore)]
    //struct MyStruct {
    //    foo: std::time::Duration, // -> error
    //}

    //#[derive(OptStore)]
    //struct WithAttrOptions {
    //    //#[opt(cfg)] // -> error
    //    //#[opt(aaa = "a")] // -> error
    //    //#[opt(cfg=aaa)] // -> error
    //    //#[opt(cfg = 123)] // -> error
    //    //#[opt(desc = 123)] // -> error
    //    //#[opt(arg = 123)] // -> error
    //    b_val: bool,
    //}

    mod tests_when_no_attr {
        use super::*;
        use std::collections::HashMap;

        #[derive(OptStore)]
        struct NoAttrOptions {
            b_val: bool,
            s_val: String,
            i8_val: i8,
            i16_val: i16,
            i32_val: i32,
            i64_val: i64,
            i128_val: i128,
            u8_val: u8,
            u16_val: u16,
            u32_val: u32,
            u64_val: u64,
            u128_val: u128,
            f32_val: f32,
            f64_val: f64,
            s_arr: Vec<String>,
            i8_arr: Vec<i8>,
            i16_arr: Vec<i16>,
            i32_arr: Vec<i32>,
            i64_arr: Vec<i64>,
            i128_arr: Vec<i128>,
            u8_arr: Vec<u8>,
            u16_arr: Vec<u16>,
            u32_arr: Vec<u32>,
            u64_arr: Vec<u64>,
            u128_arr: Vec<u128>,
            f32_arr: Vec<f32>,
            f64_arr: Vec<f64>,
            s_opt: Option<String>,
            i8_opt: Option<i8>,
            i16_opt: Option<i16>,
            i32_opt: Option<i32>,
            i64_opt: Option<i64>,
            i128_opt: Option<i128>,
            u8_opt: Option<u8>,
            u16_opt: Option<u16>,
            u32_opt: Option<u32>,
            u64_opt: Option<u64>,
            u128_opt: Option<u128>,
            f32_opt: Option<f32>,
            f64_opt: Option<f64>,
        }

        #[test]
        fn test_create_instance_with_defaults() {
            let store = NoAttrOptions::with_defaults();
            assert_eq!(store.b_val, false);
            assert_eq!(store.s_val, "".to_string());
            assert_eq!(store.i8_val, 0);
            assert_eq!(store.i16_val, 0);
            assert_eq!(store.i32_val, 0);
            assert_eq!(store.i64_val, 0);
            assert_eq!(store.i128_val, 0);
            assert_eq!(store.u8_val, 0);
            assert_eq!(store.u16_val, 0);
            assert_eq!(store.u32_val, 0);
            assert_eq!(store.u64_val, 0);
            assert_eq!(store.u128_val, 0);
            assert_eq!(store.f32_val, 0.0);
            assert_eq!(store.f64_val, 0.0);
            assert_eq!(store.s_arr, Vec::<String>::new());
            assert_eq!(store.i8_arr, Vec::<i8>::new());
            assert_eq!(store.i16_arr, Vec::<i16>::new());
            assert_eq!(store.i32_arr, Vec::<i32>::new());
            assert_eq!(store.i64_arr, Vec::<i64>::new());
            assert_eq!(store.i128_arr, Vec::<i128>::new());
            assert_eq!(store.u8_arr, Vec::<u8>::new());
            assert_eq!(store.u16_arr, Vec::<u16>::new());
            assert_eq!(store.u32_arr, Vec::<u32>::new());
            assert_eq!(store.u64_arr, Vec::<u64>::new());
            assert_eq!(store.u128_arr, Vec::<u128>::new());
            assert_eq!(store.f32_arr, Vec::<f32>::new());
            assert_eq!(store.f64_arr, Vec::<f64>::new());
            assert_eq!(store.s_opt, None);
            assert_eq!(store.i8_opt, None);
            assert_eq!(store.i16_opt, None);
            assert_eq!(store.i32_opt, None);
            assert_eq!(store.i64_opt, None);
            assert_eq!(store.i128_opt, None);
            assert_eq!(store.u8_opt, None);
            assert_eq!(store.u16_opt, None);
            assert_eq!(store.u32_opt, None);
            assert_eq!(store.u64_opt, None);
            assert_eq!(store.u128_opt, None);
            assert_eq!(store.f32_opt, None);
            assert_eq!(store.f64_opt, None);
        }

        #[test]
        fn test_make_opt_cfgs_for_opt_store() {
            let mut store = NoAttrOptions::with_defaults();
            let cfgs = cliargs::OptCfg::make_cfgs_for(&mut store);
            assert_eq!(cfgs.len(), 40);

            let cfg = &cfgs[0];
            assert_eq!(cfg.store_key, "b_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[1];
            assert_eq!(cfg.store_key, "s_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[2];
            assert_eq!(cfg.store_key, "i8_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[3];
            assert_eq!(cfg.store_key, "i16_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[4];
            assert_eq!(cfg.store_key, "i32_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[5];
            assert_eq!(cfg.store_key, "i64_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[6];
            assert_eq!(cfg.store_key, "i128_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[7];
            assert_eq!(cfg.store_key, "u8_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[8];
            assert_eq!(cfg.store_key, "u16_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[9];
            assert_eq!(cfg.store_key, "u32_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[10];
            assert_eq!(cfg.store_key, "u64_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[11];
            assert_eq!(cfg.store_key, "u128_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[12];
            assert_eq!(cfg.store_key, "f32_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[13];
            assert_eq!(cfg.store_key, "f64_val");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[14];
            assert_eq!(cfg.store_key, "s_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[15];
            assert_eq!(cfg.store_key, "i8_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[16];
            assert_eq!(cfg.store_key, "i16_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[17];
            assert_eq!(cfg.store_key, "i32_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[18];
            assert_eq!(cfg.store_key, "i64_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[19];
            assert_eq!(cfg.store_key, "i128_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[20];
            assert_eq!(cfg.store_key, "u8_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[21];
            assert_eq!(cfg.store_key, "u16_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[22];
            assert_eq!(cfg.store_key, "u32_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[23];
            assert_eq!(cfg.store_key, "u64_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[24];
            assert_eq!(cfg.store_key, "u128_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[25];
            assert_eq!(cfg.store_key, "f32_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[26];
            assert_eq!(cfg.store_key, "f64_arr");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[27];
            assert_eq!(cfg.store_key, "s_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[28];
            assert_eq!(cfg.store_key, "i8_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[29];
            assert_eq!(cfg.store_key, "i16_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[30];
            assert_eq!(cfg.store_key, "i32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[31];
            assert_eq!(cfg.store_key, "i64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[32];
            assert_eq!(cfg.store_key, "i128_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[33];
            assert_eq!(cfg.store_key, "u8_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[34];
            assert_eq!(cfg.store_key, "u16_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[35];
            assert_eq!(cfg.store_key, "u32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[36];
            assert_eq!(cfg.store_key, "u64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[37];
            assert_eq!(cfg.store_key, "u128_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[38];
            assert_eq!(cfg.store_key, "f32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[39];
            assert_eq!(cfg.store_key, "f64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());
        }

        #[test]
        fn tests_set_field_values() {
            let mut store = NoAttrOptions::with_defaults();

            let mut m = HashMap::<&str, Vec<&str>>::new();
            m.insert("b_val", vec![]);
            m.insert("s_val", vec!["ABC"]);
            m.insert("i8_val", vec!["-111"]);
            m.insert("i16_val", vec!["-123"]);
            m.insert("i32_val", vec!["-123"]);
            m.insert("i64_val", vec!["-123"]);
            m.insert("i128_val", vec!["-123"]);
            m.insert("u8_val", vec!["123"]);
            m.insert("u16_val", vec!["123"]);
            m.insert("u32_val", vec!["123"]);
            m.insert("u64_val", vec!["123"]);
            m.insert("u128_val", vec!["123"]);
            m.insert("f32_val", vec!["0.12"]);
            m.insert("f64_val", vec!["3.45"]);
            m.insert("s_arr", vec!["A", "B", "C"]);
            m.insert("i8_arr", vec!["-1", "-2", "-3"]);
            m.insert("i16_arr", vec!["-1", "-2", "-3"]);
            m.insert("i32_arr", vec!["-1", "-2", "-3"]);
            m.insert("i64_arr", vec!["-1", "-2", "-3"]);
            m.insert("i128_arr", vec!["-1", "-2", "-3"]);
            m.insert("u8_arr", vec!["1", "2", "3"]);
            m.insert("u16_arr", vec!["1", "2", "3"]);
            m.insert("u32_arr", vec!["1", "2", "3"]);
            m.insert("u64_arr", vec!["1", "2", "3"]);
            m.insert("u128_arr", vec!["1", "2", "3"]);
            m.insert("f32_arr", vec!["0.1", "0.2", "0.3"]);
            m.insert("f64_arr", vec!["0.1", "0.2", "0.3"]);
            m.insert("s_opt", vec!["abc"]);
            m.insert("i8_opt", vec!["-45"]);
            m.insert("i16_opt", vec!["-45"]);
            m.insert("i32_opt", vec!["-45"]);
            m.insert("i64_opt", vec!["-45"]);
            m.insert("i128_opt", vec!["-45"]);
            m.insert("u8_opt", vec!["45"]);
            m.insert("u16_opt", vec!["45"]);
            m.insert("u32_opt", vec!["45"]);
            m.insert("u64_opt", vec!["45"]);
            m.insert("u128_opt", vec!["45"]);
            m.insert("f32_opt", vec!["4.5"]);
            m.insert("f64_opt", vec!["4.5"]);

            match store.set_field_values(&m) {
                Ok(_) => {
                    assert_eq!(store.b_val, true);
                    assert_eq!(store.s_val, "ABC".to_string());
                    assert_eq!(store.i8_val, -111);
                    assert_eq!(store.i16_val, -123);
                    assert_eq!(store.i32_val, -123);
                    assert_eq!(store.i64_val, -123);
                    assert_eq!(store.i128_val, -123);
                    assert_eq!(store.u8_val, 123);
                    assert_eq!(store.u16_val, 123);
                    assert_eq!(store.u32_val, 123);
                    assert_eq!(store.u64_val, 123);
                    assert_eq!(store.u128_val, 123);
                    assert_eq!(store.f32_val, 0.12);
                    assert_eq!(store.f64_val, 3.45);
                    assert_eq!(
                        store.s_arr,
                        vec!["A".to_string(), "B".to_string(), "C".to_string(),]
                    );
                    assert_eq!(store.i8_arr, vec![-1, -2, -3]);
                    assert_eq!(store.i16_arr, vec![-1, -2, -3]);
                    assert_eq!(store.i32_arr, vec![-1, -2, -3]);
                    assert_eq!(store.i64_arr, vec![-1, -2, -3]);
                    assert_eq!(store.i128_arr, vec![-1, -2, -3]);
                    assert_eq!(store.u8_arr, vec![1, 2, 3]);
                    assert_eq!(store.u16_arr, vec![1, 2, 3]);
                    assert_eq!(store.u32_arr, vec![1, 2, 3]);
                    assert_eq!(store.u64_arr, vec![1, 2, 3]);
                    assert_eq!(store.u128_arr, vec![1, 2, 3]);
                    assert_eq!(store.f32_arr, vec![0.1, 0.2, 0.3]);
                    assert_eq!(store.f64_arr, vec![0.1, 0.2, 0.3]);
                    assert_eq!(store.s_opt, Some("abc".to_string()));
                    assert_eq!(store.i8_opt, Some(-45));
                    assert_eq!(store.i16_opt, Some(-45));
                    assert_eq!(store.i32_opt, Some(-45));
                    assert_eq!(store.i64_opt, Some(-45));
                    assert_eq!(store.i128_opt, Some(-45));
                    assert_eq!(store.u8_opt, Some(45));
                    assert_eq!(store.u16_opt, Some(45));
                    assert_eq!(store.u32_opt, Some(45));
                    assert_eq!(store.u64_opt, Some(45));
                    assert_eq!(store.u128_opt, Some(45));
                    assert_eq!(store.f32_opt, Some(4.5));
                    assert_eq!(store.f64_opt, Some(4.5));
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn tests_set_field_values_if_map_is_empty() {
            let mut store = NoAttrOptions::with_defaults();

            let m = HashMap::<&str, Vec<&str>>::new();
            match store.set_field_values(&m) {
                Ok(_) => {
                    assert_eq!(store.b_val, false);
                    assert_eq!(store.s_val, "".to_string());
                    assert_eq!(store.i8_val, 0);
                    assert_eq!(store.i16_val, 0);
                    assert_eq!(store.i32_val, 0);
                    assert_eq!(store.i64_val, 0);
                    assert_eq!(store.i128_val, 0);
                    assert_eq!(store.u8_val, 0);
                    assert_eq!(store.u16_val, 0);
                    assert_eq!(store.u32_val, 0);
                    assert_eq!(store.u64_val, 0);
                    assert_eq!(store.u128_val, 0);
                    assert_eq!(store.f32_val, 0.0);
                    assert_eq!(store.f64_val, 0.0);
                    assert_eq!(store.s_arr, Vec::<String>::new());
                    assert_eq!(store.i8_arr, Vec::<i8>::new());
                    assert_eq!(store.i16_arr, Vec::<i16>::new());
                    assert_eq!(store.i32_arr, Vec::<i32>::new());
                    assert_eq!(store.i64_arr, Vec::<i64>::new());
                    assert_eq!(store.i128_arr, Vec::<i128>::new());
                    assert_eq!(store.u8_arr, Vec::<u8>::new());
                    assert_eq!(store.u16_arr, Vec::<u16>::new());
                    assert_eq!(store.u32_arr, Vec::<u32>::new());
                    assert_eq!(store.u64_arr, Vec::<u64>::new());
                    assert_eq!(store.u128_arr, Vec::<u128>::new());
                    assert_eq!(store.f32_arr, Vec::<f32>::new());
                    assert_eq!(store.f64_arr, Vec::<f64>::new());
                    assert_eq!(store.s_opt, None);
                    assert_eq!(store.i8_opt, None);
                    assert_eq!(store.i16_opt, None);
                    assert_eq!(store.i32_opt, None);
                    assert_eq!(store.i64_opt, None);
                    assert_eq!(store.i128_opt, None);
                    assert_eq!(store.u8_opt, None);
                    assert_eq!(store.u16_opt, None);
                    assert_eq!(store.u32_opt, None);
                    assert_eq!(store.u64_opt, None);
                    assert_eq!(store.u128_opt, None);
                    assert_eq!(store.f32_opt, None);
                    assert_eq!(store.f64_opt, None);
                }
                Err(_) => assert!(false),
            }
        }

        #[test]
        fn tests_set_field_values_if_number_format_error() {
            let mut store = NoAttrOptions::with_defaults();

            let mut m = HashMap::<&str, Vec<&str>>::new();
            m.insert("i8_val", vec!["-1024"]);

            match store.set_field_values(&m) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "i8_val");
                    assert_eq!(option, "i8_val");
                    assert_eq!(opt_arg, "-1024");
                    assert_eq!(details, "number too small to fit in target type");
                }
                Err(_) => assert!(false),
            }

            let mut m = HashMap::<&str, Vec<&str>>::new();
            m.insert("i8_arr", vec!["abc"]);

            match store.set_field_values(&m) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "i8_arr");
                    assert_eq!(option, "i8_arr");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }

            let mut m = HashMap::<&str, Vec<&str>>::new();
            m.insert("i8_opt", vec!["abc"]);

            match store.set_field_values(&m) {
                Ok(_) => assert!(false),
                Err(InvalidOption::OptionArgIsInvalid {
                    store_key,
                    option,
                    opt_arg,
                    details,
                }) => {
                    assert_eq!(store_key, "i8_opt");
                    assert_eq!(option, "i8_opt");
                    assert_eq!(opt_arg, "abc");
                    assert_eq!(details, "invalid digit found in string");
                }
                Err(_) => assert!(false),
            }
        }
    }

    mod tests_when_with_attr {
        use super::*;

        #[derive(OptStore)]
        struct WithAttrOptions {
            #[opt(cfg = "b", desc = "The description of b_val")]
            b_val: bool,

            #[opt(cfg = "s,sss=ABC", desc = "The description of s_val", arg = "txt")]
            s_val: String,

            #[opt(cfg = "i8=-12", desc = "The description of i8_val", arg = "<n>")]
            i8_val: i8,

            #[opt(cfg = "i16=-123", desc = "The description of i16_val", arg = "<n>")]
            i16_val: i16,

            #[opt(cfg = "i32=-234", desc = "The description of i32_val", arg = "<n>")]
            i32_val: i32,

            #[opt(cfg = "i64=-345", desc = "The description of i64_val", arg = "<n>")]
            i64_val: i64,

            #[opt(cfg = "i128=-456", desc = "The description of i128_val", arg = "<n>")]
            i128_val: i128,

            #[opt(cfg = "u8=123", desc = "The description of u8_val", arg = "<n>")]
            u8_val: u8,

            #[opt(cfg = "u16=234", desc = "The description of u16_val", arg = "<n>")]
            u16_val: u16,

            #[opt(cfg = "u32=345", desc = "The description of u32_val", arg = "<n>")]
            u32_val: u32,

            #[opt(cfg = "u64=456", desc = "The description of u64_val", arg = "<n>")]
            u64_val: u64,

            #[opt(cfg = "u128=567", desc = "The description of u128_val", arg = "<n>")]
            u128_val: u128,

            #[opt(cfg = "f32=0.678", desc = "The description of f32_val", arg = "<n>")]
            f32_val: f32,

            #[opt(cfg = "f64=7.89", desc = "The description of f64_val", arg = "<n>")]
            f64_val: f64,

            #[opt(cfg = "ss=[A,B,C]", desc = "The description of s_arr", arg = "<s>")]
            s_arr: Vec<String>,

            #[opt(cfg = "ii8=[-1,2,-3]", desc = "The description of i8_arr", arg = "<n>")]
            i8_arr: Vec<i8>,

            #[opt(
                cfg = "ii16=[2,-3,4]",
                desc = "The description of i16_arr",
                arg = "<n>"
            )]
            i16_arr: Vec<i16>,

            #[opt(
                cfg = "ii32=[-3,4,-5]",
                desc = "The description of i32_arr",
                arg = "<n>"
            )]
            i32_arr: Vec<i32>,

            #[opt(
                cfg = "ii64=[4,-5,6]",
                desc = "The description of i64_arr",
                arg = "<n>"
            )]
            i64_arr: Vec<i64>,

            #[opt(
                cfg = "ii128=[-5,6,-7]",
                desc = "The description of i128_arr",
                arg = "<n>"
            )]
            i128_arr: Vec<i128>,

            #[opt(cfg = "uu8=[1,2,3]", desc = "The description of u8_arr", arg = "<n>")]
            u8_arr: Vec<u8>,

            #[opt(cfg = "uu16=[2,3,4]", desc = "The description of u16_arr", arg = "<n>")]
            u16_arr: Vec<u16>,

            #[opt(cfg = "uu32=[3,4,5]", desc = "The description of u32_arr", arg = "<n>")]
            u32_arr: Vec<u32>,

            #[opt(cfg = "uu64=[4,5,6]", desc = "The description of u64_arr", arg = "<n>")]
            u64_arr: Vec<u64>,

            #[opt(
                cfg = "uu128=[5,6,7]",
                desc = "The description of u128_arr",
                arg = "<n>"
            )]
            u128_arr: Vec<u128>,

            #[opt(
                cfg = "ff32=[0.6,0.7]",
                desc = "The description of f32_arr",
                arg = "<n>"
            )]
            f32_arr: Vec<f32>,

            #[opt(
                cfg = "ff64=[0.7,0.8]",
                desc = "The description of f64_arr",
                arg = "<n>"
            )]
            f64_arr: Vec<f64>,

            #[opt(cfg = "=ABC", desc = "The description of s_opt", arg = "<s>")]
            s_opt: Option<String>,

            #[opt(cfg = "=-12", desc = "The description of i8_opt", arg = "<n>")]
            i8_opt: Option<i8>,

            #[opt(cfg = "=-234", desc = "The description of i16_opt", arg = "<n>")]
            i16_opt: Option<i16>,

            #[opt(cfg = "=-345", desc = "The description of i32_opt", arg = "<n>")]
            i32_opt: Option<i32>,

            #[opt(cfg = "=-456", desc = "The description of i64_opt", arg = "<n>")]
            i64_opt: Option<i64>,

            #[opt(cfg = "=-567", desc = "The description of i128_opt", arg = "<n>")]
            i128_opt: Option<i128>,

            #[opt(cfg = "=123", desc = "The description of u8_opt", arg = "<n>")]
            u8_opt: Option<u8>,

            #[opt(cfg = "=234", desc = "The description of u16_opt", arg = "<n>")]
            u16_opt: Option<u16>,

            #[opt(cfg = "=345", desc = "The description of u32_opt", arg = "<n>")]
            u32_opt: Option<u32>,

            #[opt(cfg = "=456", desc = "The description of u64_opt", arg = "<n>")]
            u64_opt: Option<u64>,

            #[opt(cfg = "=567", desc = "The description of u128_opt", arg = "<n>")]
            u128_opt: Option<u128>,

            #[opt(cfg = "=0.1", desc = "The description of f32_opt", arg = "<n>")]
            f32_opt: Option<f32>,

            #[opt(cfg = "=1.2", desc = "The description of f64_opt", arg = "<n>")]
            f64_opt: Option<f64>,
        }

        #[test]
        fn test_create_instance_with_defaults() {
            let store = WithAttrOptions::with_defaults();
            assert_eq!(store.b_val, false);
            assert_eq!(store.s_val, "ABC".to_string());
            assert_eq!(store.i8_val, -12);
            assert_eq!(store.i16_val, -123);
            assert_eq!(store.i32_val, -234);
            assert_eq!(store.i64_val, -345);
            assert_eq!(store.i128_val, -456);
            assert_eq!(store.u8_val, 123);
            assert_eq!(store.u16_val, 234);
            assert_eq!(store.u32_val, 345);
            assert_eq!(store.u64_val, 456);
            assert_eq!(store.u128_val, 567);
            assert_eq!(store.f32_val, 0.678);
            assert_eq!(store.f64_val, 7.89);
            assert_eq!(
                store.s_arr,
                vec!["A".to_string(), "B".to_string(), "C".to_string()]
            );
            assert_eq!(store.i8_arr, vec![-1, 2, -3]);
            assert_eq!(store.i16_arr, vec![2, -3, 4]);
            assert_eq!(store.i32_arr, vec![-3, 4, -5]);
            assert_eq!(store.i64_arr, vec![4, -5, 6]);
            assert_eq!(store.i128_arr, vec![-5, 6, -7]);
            assert_eq!(store.u8_arr, vec![1, 2, 3]);
            assert_eq!(store.u16_arr, vec![2, 3, 4]);
            assert_eq!(store.u32_arr, vec![3, 4, 5]);
            assert_eq!(store.u64_arr, vec![4, 5, 6]);
            assert_eq!(store.u128_arr, vec![5, 6, 7]);
            assert_eq!(store.f32_arr, vec![0.6, 0.7]);
            assert_eq!(store.f64_arr, vec![0.7, 0.8]);
            assert_eq!(store.s_opt, Some("ABC".to_string()));
            assert_eq!(store.i8_opt, Some(-12));
            assert_eq!(store.i16_opt, Some(-234));
            assert_eq!(store.i32_opt, Some(-345));
            assert_eq!(store.i64_opt, Some(-456));
            assert_eq!(store.i128_opt, Some(-567));
            assert_eq!(store.u8_opt, Some(123));
            assert_eq!(store.u16_opt, Some(234));
            assert_eq!(store.u32_opt, Some(345));
            assert_eq!(store.u64_opt, Some(456));
            assert_eq!(store.u128_opt, Some(567));
            assert_eq!(store.f32_opt, Some(0.1));
            assert_eq!(store.f64_opt, Some(1.2));
        }

        #[test]
        fn test_make_opt_cfgs_for_store() {
            let mut store = WithAttrOptions::with_defaults();
            let cfgs = cliargs::OptCfg::make_cfgs_for(&mut store);
            assert_eq!(cfgs.len(), 40);

            let cfg = &cfgs[0];
            assert_eq!(cfg.store_key, "b_val");
            assert_eq!(cfg.names, vec!["b"]);
            assert_eq!(cfg.has_arg, false);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, None);
            assert_eq!(cfg.desc, "The description of b_val".to_string());
            assert_eq!(cfg.arg_in_help, "".to_string());

            let cfg = &cfgs[1];
            assert_eq!(cfg.store_key, "s_val");
            assert_eq!(cfg.names, vec!["s".to_string(), "sss".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["ABC".to_string()]));
            assert_eq!(cfg.desc, "The description of s_val".to_string());
            assert_eq!(cfg.arg_in_help, "txt".to_string());

            let cfg = &cfgs[2];
            assert_eq!(cfg.store_key, "i8_val");
            assert_eq!(cfg.names, vec!["i8"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-12".to_string()]));
            assert_eq!(cfg.desc, "The description of i8_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[3];
            assert_eq!(cfg.store_key, "i16_val");
            assert_eq!(cfg.names, vec!["i16"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-123".to_string()]));
            assert_eq!(cfg.desc, "The description of i16_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[4];
            assert_eq!(cfg.store_key, "i32_val");
            assert_eq!(cfg.names, vec!["i32"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-234".to_string()]));
            assert_eq!(cfg.desc, "The description of i32_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[5];
            assert_eq!(cfg.store_key, "i64_val");
            assert_eq!(cfg.names, vec!["i64"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-345".to_string()]));
            assert_eq!(cfg.desc, "The description of i64_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[6];
            assert_eq!(cfg.store_key, "i128_val");
            assert_eq!(cfg.names, vec!["i128"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-456".to_string()]));
            assert_eq!(cfg.desc, "The description of i128_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[7];
            assert_eq!(cfg.store_key, "u8_val");
            assert_eq!(cfg.names, vec!["u8"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["123".to_string()]));
            assert_eq!(cfg.desc, "The description of u8_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[8];
            assert_eq!(cfg.store_key, "u16_val");
            assert_eq!(cfg.names, vec!["u16"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["234".to_string()]));
            assert_eq!(cfg.desc, "The description of u16_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[9];
            assert_eq!(cfg.store_key, "u32_val");
            assert_eq!(cfg.names, vec!["u32"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["345".to_string()]));
            assert_eq!(cfg.desc, "The description of u32_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[10];
            assert_eq!(cfg.store_key, "u64_val");
            assert_eq!(cfg.names, vec!["u64"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["456".to_string()]));
            assert_eq!(cfg.desc, "The description of u64_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[11];
            assert_eq!(cfg.store_key, "u128_val");
            assert_eq!(cfg.names, vec!["u128"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["567".to_string()]));
            assert_eq!(cfg.desc, "The description of u128_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[12];
            assert_eq!(cfg.store_key, "f32_val");
            assert_eq!(cfg.names, vec!["f32"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["0.678".to_string()]));
            assert_eq!(cfg.desc, "The description of f32_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[13];
            assert_eq!(cfg.store_key, "f64_val");
            assert_eq!(cfg.names, vec!["f64"]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["7.89".to_string()]));
            assert_eq!(cfg.desc, "The description of f64_val".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[14];
            assert_eq!(cfg.store_key, "s_arr");
            assert_eq!(cfg.names, vec!["ss".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["A".to_string(), "B".to_string(), "C".to_string()])
            );
            assert_eq!(cfg.desc, "The description of s_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<s>".to_string());

            let cfg = &cfgs[15];
            assert_eq!(cfg.store_key, "i8_arr");
            assert_eq!(cfg.names, vec!["ii8".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["-1".to_string(), "2".to_string(), "-3".to_string()])
            );
            assert_eq!(cfg.desc, "The description of i8_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[16];
            assert_eq!(cfg.store_key, "i16_arr");
            assert_eq!(cfg.names, vec!["ii16".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["2".to_string(), "-3".to_string(), "4".to_string()])
            );
            assert_eq!(cfg.desc, "The description of i16_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[17];
            assert_eq!(cfg.store_key, "i32_arr");
            assert_eq!(cfg.names, vec!["ii32".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["-3".to_string(), "4".to_string(), "-5".to_string()])
            );
            assert_eq!(cfg.desc, "The description of i32_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[18];
            assert_eq!(cfg.store_key, "i64_arr");
            assert_eq!(cfg.names, vec!["ii64".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["4".to_string(), "-5".to_string(), "6".to_string()])
            );
            assert_eq!(cfg.desc, "The description of i64_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[19];
            assert_eq!(cfg.store_key, "i128_arr");
            assert_eq!(cfg.names, vec!["ii128".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["-5".to_string(), "6".to_string(), "-7".to_string()])
            );
            assert_eq!(cfg.desc, "The description of i128_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[20];
            assert_eq!(cfg.store_key, "u8_arr");
            assert_eq!(cfg.names, vec!["uu8".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["1".to_string(), "2".to_string(), "3".to_string()])
            );
            assert_eq!(cfg.desc, "The description of u8_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[21];
            assert_eq!(cfg.store_key, "u16_arr");
            assert_eq!(cfg.names, vec!["uu16".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["2".to_string(), "3".to_string(), "4".to_string()])
            );
            assert_eq!(cfg.desc, "The description of u16_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[22];
            assert_eq!(cfg.store_key, "u32_arr");
            assert_eq!(cfg.names, vec!["uu32".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["3".to_string(), "4".to_string(), "5".to_string()])
            );
            assert_eq!(cfg.desc, "The description of u32_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[23];
            assert_eq!(cfg.store_key, "u64_arr");
            assert_eq!(cfg.names, vec!["uu64".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["4".to_string(), "5".to_string(), "6".to_string()])
            );
            assert_eq!(cfg.desc, "The description of u64_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[24];
            assert_eq!(cfg.store_key, "u128_arr");
            assert_eq!(cfg.names, vec!["uu128".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["5".to_string(), "6".to_string(), "7".to_string()])
            );
            assert_eq!(cfg.desc, "The description of u128_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[25];
            assert_eq!(cfg.store_key, "f32_arr");
            assert_eq!(cfg.names, vec!["ff32".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["0.6".to_string(), "0.7".to_string()])
            );
            assert_eq!(cfg.desc, "The description of f32_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[26];
            assert_eq!(cfg.store_key, "f64_arr");
            assert_eq!(cfg.names, vec!["ff64".to_string()]);
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, true);
            assert_eq!(
                cfg.defaults,
                Some(vec!["0.7".to_string(), "0.8".to_string()])
            );
            assert_eq!(cfg.desc, "The description of f64_arr".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[27];
            assert_eq!(cfg.store_key, "s_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["ABC".to_string()]));
            assert_eq!(cfg.desc, "The description of s_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<s>".to_string());

            let cfg = &cfgs[28];
            assert_eq!(cfg.store_key, "i8_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-12".to_string()]));
            assert_eq!(cfg.desc, "The description of i8_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[29];
            assert_eq!(cfg.store_key, "i16_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-234".to_string()]));
            assert_eq!(cfg.desc, "The description of i16_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[30];
            assert_eq!(cfg.store_key, "i32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-345".to_string()]));
            assert_eq!(cfg.desc, "The description of i32_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[31];
            assert_eq!(cfg.store_key, "i64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-456".to_string()]));
            assert_eq!(cfg.desc, "The description of i64_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[32];
            assert_eq!(cfg.store_key, "i128_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["-567".to_string()]));
            assert_eq!(cfg.desc, "The description of i128_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[33];
            assert_eq!(cfg.store_key, "u8_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["123".to_string()]));
            assert_eq!(cfg.desc, "The description of u8_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[34];
            assert_eq!(cfg.store_key, "u16_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["234".to_string()]));
            assert_eq!(cfg.desc, "The description of u16_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[35];
            assert_eq!(cfg.store_key, "u32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["345".to_string()]));
            assert_eq!(cfg.desc, "The description of u32_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[36];
            assert_eq!(cfg.store_key, "u64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["456".to_string()]));
            assert_eq!(cfg.desc, "The description of u64_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[37];
            assert_eq!(cfg.store_key, "u128_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["567".to_string()]));
            assert_eq!(cfg.desc, "The description of u128_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[38];
            assert_eq!(cfg.store_key, "f32_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["0.1".to_string()]));
            assert_eq!(cfg.desc, "The description of f32_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());

            let cfg = &cfgs[39];
            assert_eq!(cfg.store_key, "f64_opt");
            assert_eq!(cfg.names, Vec::<String>::new());
            assert_eq!(cfg.has_arg, true);
            assert_eq!(cfg.is_array, false);
            assert_eq!(cfg.defaults, Some(vec!["1.2".to_string()]));
            assert_eq!(cfg.desc, "The description of f64_opt".to_string());
            assert_eq!(cfg.arg_in_help, "<n>".to_string());
        }

        #[test]
        fn test_defaults_for_string_array_field() {
            #[derive(OptStore)]
            struct MyOptions {
                #[opt(cfg = "=[]")]
                empty: Vec<String>,
                #[opt(cfg = "=[a]")]
                single_str: Vec<String>,
                #[opt(cfg = "=[a,b]")]
                multiple_str: Vec<String>,
                #[opt(cfg = "=")]
                empty_str: Vec<String>,
            }
            let mut store = MyOptions::with_defaults();
            let cfgs = cliargs::OptCfg::make_cfgs_for(&mut store);
            assert_eq!(cfgs.len(), 4);
            assert_eq!(cfgs[0].store_key, "empty".to_string());
            assert_eq!(cfgs[0].defaults, Some(Vec::<String>::new()));
            assert_eq!(cfgs[1].store_key, "single_str".to_string());
            assert_eq!(cfgs[1].defaults, Some(vec!["a".to_string()]));
            assert_eq!(cfgs[2].store_key, "multiple_str".to_string());
            assert_eq!(
                cfgs[2].defaults,
                Some(vec!["a".to_string(), "b".to_string()])
            );
            assert_eq!(cfgs[3].store_key, "empty_str".to_string());
            assert_eq!(cfgs[3].defaults, Some(vec!["".to_string()]));
        }
    }
}

#[cfg(test)]
mod tests_of_parse_for {
    use crate as cliargs;

    #[derive(cliargs::OptStore, Debug)]
    struct MyOptions {
        #[opt(cfg = "b-val")]
        b_val: bool,

        #[opt(cfg = "s-val")]
        s_val: String,

        #[opt(cfg = "i8-val")]
        i8_val: i8,

        #[opt(cfg = "i16-val")]
        i16_val: i16,

        #[opt(cfg = "i32-val")]
        i32_val: i32,

        #[opt(cfg = "i64-val")]
        i64_val: i64,

        #[opt(cfg = "i128-val")]
        i128_val: i128,

        #[opt(cfg = "u8-val")]
        u8_val: u8,

        #[opt(cfg = "u16-val")]
        u16_val: u16,

        #[opt(cfg = "u32-val")]
        u32_val: u32,

        #[opt(cfg = "u64-val")]
        u64_val: u64,

        #[opt(cfg = "u128-val")]
        u128_val: u128,

        #[opt(cfg = "f32-val")]
        f32_val: f32,

        #[opt(cfg = "f64-val")]
        f64_val: f64,

        #[opt(cfg = "s-arr")]
        s_arr: Vec<String>,

        #[opt(cfg = "i8-arr")]
        i8_arr: Vec<i8>,

        #[opt(cfg = "i16-arr")]
        i16_arr: Vec<i16>,

        #[opt(cfg = "i32-arr")]
        i32_arr: Vec<i32>,

        #[opt(cfg = "i64-arr")]
        i64_arr: Vec<i64>,

        #[opt(cfg = "i128-arr")]
        i128_arr: Vec<i128>,

        #[opt(cfg = "u8-arr")]
        u8_arr: Vec<u8>,

        #[opt(cfg = "u16-arr")]
        u16_arr: Vec<u16>,

        #[opt(cfg = "u32-arr")]
        u32_arr: Vec<u32>,

        #[opt(cfg = "u64-arr")]
        u64_arr: Vec<u64>,

        #[opt(cfg = "u128-arr")]
        u128_arr: Vec<u128>,

        #[opt(cfg = "f32-arr")]
        f32_arr: Vec<f32>,

        #[opt(cfg = "f64-arr")]
        f64_arr: Vec<f64>,

        #[opt(cfg = "s-opt")]
        s_opt: Option<String>,

        #[opt(cfg = "i8-opt")]
        i8_opt: Option<i8>,

        #[opt(cfg = "i16-opt")]
        i16_opt: Option<i16>,

        #[opt(cfg = "i32-opt")]
        i32_opt: Option<i32>,

        #[opt(cfg = "i64-opt")]
        i64_opt: Option<i64>,

        #[opt(cfg = "i128-opt")]
        i128_opt: Option<i128>,

        #[opt(cfg = "u8-opt")]
        u8_opt: Option<u8>,

        #[opt(cfg = "u16-opt")]
        u16_opt: Option<u16>,

        #[opt(cfg = "u32-opt")]
        u32_opt: Option<u32>,

        #[opt(cfg = "u64-opt")]
        u64_opt: Option<u64>,

        #[opt(cfg = "u128-opt")]
        u128_opt: Option<u128>,

        #[opt(cfg = "f32-opt")]
        f32_opt: Option<f32>,

        #[opt(cfg = "f64-opt")]
        f64_opt: Option<f64>,
    }

    #[test]
    fn test_parse_for_my_options() {
        let mut store = MyOptions::with_defaults();
        let mut cmd = cliargs::Cmd::with_strings([
            "/path/to/command".to_string(),
            "--b-val".to_string(),
            "--s-val".to_string(),
            "abcd".to_string(),
            "--i8-val=1".to_string(),
            "--i16-val=-12".to_string(),
            "--i32-val=123".to_string(),
            "--i64-val=-1234".to_string(),
            "--i128-val=12345".to_string(),
            "--u8-val=1".to_string(),
            "--u16-val=12".to_string(),
            "--u32-val=123".to_string(),
            "--u64-val=1234".to_string(),
            "--u128-val=12345".to_string(),
            "--f32-val=1.23".to_string(),
            "--f64-val=-4.56".to_string(),
            "--s-arr=a".to_string(),
            "--s-arr".to_string(),
            "b".to_string(),
            "--s-arr=c".to_string(),
            "--i8-arr=-1".to_string(),
            "--i8-arr=-2".to_string(),
            "--i16-arr=-1".to_string(),
            "--i16-arr=-2".to_string(),
            "--i32-arr=-1".to_string(),
            "--i32-arr=-2".to_string(),
            "--i64-arr=-1".to_string(),
            "--i64-arr=-2".to_string(),
            "--i128-arr=-1".to_string(),
            "--i128-arr=-2".to_string(),
            "--u8-arr=3".to_string(),
            "--u8-arr=4".to_string(),
            "--u16-arr=3".to_string(),
            "--u16-arr=4".to_string(),
            "--u32-arr=3".to_string(),
            "--u32-arr=4".to_string(),
            "--u64-arr=3".to_string(),
            "--u64-arr=4".to_string(),
            "--u128-arr=3".to_string(),
            "--u128-arr=4".to_string(),
            "--f32-arr=0.1".to_string(),
            "--f32-arr=-0.2".to_string(),
            "--f64-arr=-0.3".to_string(),
            "--f64-arr=0.4".to_string(),
            "--s-opt=aaa".to_string(),
            "--i8-opt=-1".to_string(),
            "--i16-opt=-2".to_string(),
            "--i32-opt=-3".to_string(),
            "--i64-opt=-4".to_string(),
            "--i128-opt=-5".to_string(),
            "--u8-opt=1".to_string(),
            "--u16-opt=2".to_string(),
            "--u32-opt=3".to_string(),
            "--u64-opt=4".to_string(),
            "--u128-opt=5".to_string(),
            "--f32-opt=-0.1".to_string(),
            "--f64-opt=2.3".to_string(),
        ]);

        assert_eq!(cmd.parse_for(&mut store), Ok(()));

        assert_eq!(store.b_val, true);
        assert_eq!(store.s_val, "abcd".to_string());
        assert_eq!(store.i8_val, 1);
        assert_eq!(store.i16_val, -12);
        assert_eq!(store.i32_val, 123);
        assert_eq!(store.i64_val, -1234);
        assert_eq!(store.i128_val, 12345);
        assert_eq!(store.u8_val, 1);
        assert_eq!(store.u16_val, 12);
        assert_eq!(store.u32_val, 123);
        assert_eq!(store.u64_val, 1234);
        assert_eq!(store.u128_val, 12345);
        assert_eq!(store.f32_val, 1.23);
        assert_eq!(store.f64_val, -4.56);
        assert_eq!(
            store.s_arr,
            vec!["a".to_string(), "b".to_string(), "c".to_string(),]
        );
        assert_eq!(store.i8_arr, vec![-1, -2]);
        assert_eq!(store.i16_arr, vec![-1, -2]);
        assert_eq!(store.i32_arr, vec![-1, -2]);
        assert_eq!(store.i64_arr, vec![-1, -2]);
        assert_eq!(store.i128_arr, vec![-1, -2]);
        assert_eq!(store.u8_arr, vec![3, 4]);
        assert_eq!(store.u16_arr, vec![3, 4]);
        assert_eq!(store.u32_arr, vec![3, 4]);
        assert_eq!(store.u64_arr, vec![3, 4]);
        assert_eq!(store.u128_arr, vec![3, 4]);
        assert_eq!(store.f32_arr, vec![0.1, -0.2]);
        assert_eq!(store.f64_arr, vec![-0.3, 0.4]);
        assert_eq!(store.s_opt, Some("aaa".to_string()));
        assert_eq!(store.i8_opt, Some(-1));
        assert_eq!(store.i16_opt, Some(-2));
        assert_eq!(store.i32_opt, Some(-3));
        assert_eq!(store.i64_opt, Some(-4));
        assert_eq!(store.i128_opt, Some(-5));
        assert_eq!(store.u8_opt, Some(1));
        assert_eq!(store.u16_opt, Some(2));
        assert_eq!(store.u32_opt, Some(3));
        assert_eq!(store.u64_opt, Some(4));
        assert_eq!(store.u128_opt, Some(5));
        assert_eq!(store.f32_opt, Some(-0.1));
        assert_eq!(store.f64_opt, Some(2.3));
    }
}
