// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::errors::OptStoreErr;
use super::util::collect_impl_of_first_number;
use super::util::parse_defaults;
use std::collections::HashMap;

pub fn collect_impl(
    field_name: &syn::Ident,
    type_ident: &syn::Ident,
    cfg_vec: &mut Vec<proc_macro2::TokenStream>,
    init_vec: &mut Vec<proc_macro2::TokenStream>,
    set_vec: &mut Vec<proc_macro2::TokenStream>,
    attr_map: &HashMap<String, String>,
    attr_span: Option<proc_macro2::Span>,
    field_span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    match type_ident.to_string().as_str() {
        "String" => for_string(
            field_name, cfg_vec, init_vec, set_vec, attr_map, attr_span, field_span,
        ),
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "f32"
        | "f64" => for_number(
            field_name, type_ident, cfg_vec, init_vec, set_vec, attr_map, attr_span, field_span,
        ),
        _ => Err(OptStoreErr::BadFieldType.at(field_span)),
    }
}

fn for_string(
    field_name: &syn::Ident,
    cfg_vec: &mut Vec<proc_macro2::TokenStream>,
    init_vec: &mut Vec<proc_macro2::TokenStream>,
    set_vec: &mut Vec<proc_macro2::TokenStream>,
    attr_map: &HashMap<String, String>,
    _attr_span: Option<proc_macro2::Span>,
    _field_span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    let empty = String::with_capacity(0);

    let desc = attr_map.get("desc").or(Some(&empty));
    let arg = attr_map.get("arg").or(Some(&empty));
    let store_key = field_name.to_string();

    let names = attr_map.get("names").or(Some(&empty)).unwrap();
    let names: Vec<&str> = if names.is_empty() {
        Vec::<&str>::new()
    } else {
        names.split(",").map(|s| s.trim()).collect()
    };

    let default_str: proc_macro2::TokenStream;
    let default_vec: proc_macro2::TokenStream;
    match parse_defaults(attr_map) {
        Some(vec) => {
            let s = if vec.is_empty() { "" } else { &vec[0] };
            default_str = quote::quote! { Some(#s.to_string()) };
            default_vec = quote::quote! { Some(vec![#(#vec.to_string()),*]) };
        }
        None => {
            default_str = quote::quote! { None };
            default_vec = quote::quote! { None };
        }
    }

    init_vec.push(quote::quote! { #field_name: #default_str });

    set_vec.push(quote::quote! {{
        if let Some(v) = m.get(#store_key) {
            if ! v.is_empty() {
                self.#field_name = Some(v[0].to_string());
            }
        }
    }});

    cfg_vec.push(quote::quote! {
        cliargs::OptCfg {
          store_key: #store_key.to_string(),
          names: vec![#(#names.to_string()),*],
          has_arg: true,
          is_array: false,
          defaults: #default_vec,
          desc: #desc.to_string(),
          arg_in_help: #arg.to_string(),
          validator: |_, _, _| Ok(()),
        }
    });

    Ok(())
}

fn for_number(
    field_name: &syn::Ident,
    field_type: &syn::Ident,
    cfg_vec: &mut Vec<proc_macro2::TokenStream>,
    init_vec: &mut Vec<proc_macro2::TokenStream>,
    set_vec: &mut Vec<proc_macro2::TokenStream>,
    attr_map: &HashMap<String, String>,
    attr_span: Option<proc_macro2::Span>,
    _field_span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    let empty = String::with_capacity(0);

    let desc = attr_map.get("desc").or(Some(&empty));
    let arg = attr_map.get("arg").or(Some(&empty));
    let store_key = field_name.to_string();

    let names = attr_map.get("names").or(Some(&empty)).unwrap();
    let names: Vec<&str> = if names.is_empty() {
        Vec::<&str>::new()
    } else {
        names.split(",").map(|s| s.trim()).collect()
    };

    let name = match names.iter().find(|&&x| !x.is_empty()) {
        Some(s) => String::from(*s),
        None => store_key.clone(),
    };

    let default_num: proc_macro2::TokenStream;
    let default_vec: proc_macro2::TokenStream;
    match parse_defaults(attr_map) {
        Some(vec) => {
            let span = attr_span.unwrap();
            let n = collect_impl_of_first_number(&vec, field_name, field_type, span)?;
            default_num = quote::quote! { Some(#n) };
            default_vec = quote::quote! { Some(vec![#(#vec.to_string()),*]) };
        }
        None => {
            default_num = quote::quote! { None };
            default_vec = quote::quote! { None };
        }
    }

    init_vec.push(quote::quote! { #field_name: #default_num });

    set_vec.push(quote::quote! {{
        match m.get(#store_key) {
            Some(v) => {
                if ! v.is_empty() {
                    match v[0].parse::<#field_type>() {
                        Ok(n) => { self.#field_name = Some(n); },
                        Err(err) => return Err(cliargs::errors::InvalidOption::OptionArgIsInvalid {
                            store_key: #store_key.to_string(),
                            option: #name.to_string(),
                            opt_arg: v[0].to_string(),
                            details: format!("{}", err),
                        }),
                    }
                }
            },
            None => {},
        }
    }});

    cfg_vec.push(quote::quote! {
        cliargs::OptCfg {
          store_key: #store_key.to_string(),
          names: vec![#(#names.to_string()),*],
          has_arg: true,
          is_array: false,
          defaults: #default_vec,
          desc: #desc.to_string(),
          arg_in_help: #arg.to_string(),
          validator: cliargs::validators::validate_number::<#field_type>,
        }
    });

    Ok(())
}
