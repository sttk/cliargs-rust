// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

mod errors;
mod option_field;
mod scalar_field;
mod util;
mod vector_field;

use errors::OptStoreErr;
use util::identify_field_type;

use proc_macro::TokenStream;
use std::collections::HashMap;

/// Is attached to a struct which holds command line option values, and automatically implements
/// its method to generate `OptCfg`s from its fields, and other methods.
///
/// This macro automatically implements the method to generates a vector of `OptCfg` from the field
/// definitions and `opt` field attributes, and this also implements the method that instantiates
/// the struct using the default values specified in `opt` field attributes, and implements the
/// method to updates the field values with the values from the passed `HashMap1.
///
/// The `opt` field attribute can have the following pairs of name and value: one is `cfg` to
/// specify `names` and `defaults` of `OptCfg` struct, another is `desc` to specify `desc` of
/// `OptCfg` struct, and yet another is `arg` to specify `arg_in_help` of `OptCfg` struct.
///
/// The format of `cfg` is like `cfg="f,foo=123"`.
/// The left side of the equal sign is the option name(s), and the right side is the default
/// value(s).
/// If there is no equal sign, it is determined that only the option name is specified.
/// If you want to specify multiple option names, separate them with commas.
/// If you want to specify multiple default values, separate them with commas and round them with
/// square brackets, like `[1,2,3]`.
/// If you want to use your favorite carachter as a separator, you can use it by putting it on the
/// left side of the open square bracket, like `/[1/2/3]`.
///
/// The following code is a sample of a option store struct.
///
/// ```rust
/// extern crate cliargs_derive;
/// use cliargs_derive::OptStore;
///
/// #[derive(OptStore)]
/// struct MyOptions {
///     #[opt(cfg="f,foo-bar", desc="The description of foo-bar.")]
///     foo_bar: bool,
///
///     #[opt(cfg="b", desc="The description of baz.", arg="text")]
///     baz: String,
///
///     #[opt(cfg="q=[1,2,3]", desc="The description of qux.", arg="num...")]
///     qux: Vec<u8>,
/// }
/// ```
#[proc_macro_derive(OptStore, attributes(opt))]
pub fn opt_store_derive(input: TokenStream) -> TokenStream {
    let input = &syn::parse_macro_input!(input as syn::DeriveInput);

    match generate_opt_store_impl(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_opt_store_impl(input: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let (impl_generics, _, where_clause) = &input.generics.split_for_impl();

    let struct_data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => return Err(OptStoreErr::MustPutOnStruct.at(input.ident.span())),
    };

    let struct_span = input.ident.span();

    let mut cfg_vec = Vec::<proc_macro2::TokenStream>::new();
    let mut init_vec = Vec::<proc_macro2::TokenStream>::new();
    let mut set_vec = Vec::<proc_macro2::TokenStream>::new();
    for field in &struct_data.fields {
        collect_impl_for_field(
            field,
            &mut cfg_vec,
            &mut init_vec,
            &mut set_vec,
            struct_span,
        )?;
    }

    let expanded = quote::quote! {
        #[automatically_derived]
        impl #impl_generics #struct_name #where_clause {
            pub fn with_defaults() -> #struct_name {
                #struct_name {
                    #(#init_vec),*
                }
            }
        }

        #[automatically_derived]
        impl #impl_generics cliargs::OptStore for  #struct_name #where_clause {
            fn make_opt_cfgs(&self) -> Vec<cliargs::OptCfg> {
                vec![
                    #(#cfg_vec),*
                ]
            }

            fn set_field_values(&mut self, m: &std::collections::HashMap<&str, Vec<&str>>) -> Result<(), cliargs::errors::InvalidOption> {
                #(#set_vec)*
                Ok(())
            }
        }
    };

    //println!("{}", expanded);
    Ok(expanded.into())
}

fn collect_impl_for_field(
    field: &syn::Field,
    cfg_vec: &mut Vec<proc_macro2::TokenStream>,
    init_vec: &mut Vec<proc_macro2::TokenStream>,
    set_vec: &mut Vec<proc_macro2::TokenStream>,
    struct_span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    let field_name = match field.ident.as_ref() {
        Some(ident) => ident,
        None => return Err(OptStoreErr::MustNotPutOnTuple.at(struct_span)),
    };
    let field_span = field_name.span();

    let mut attr_map = HashMap::<String, String>::new();
    let mut attr_span: Option<proc_macro2::Span> = None;
    for attr in &field.attrs {
        if attr.path().is_ident("opt") {
            let span = attr.path().get_ident().unwrap().span();
            attr_span = Some(span);

            collect_impl_for_field_attr(attr, &mut attr_map, span)?;
        }
    }

    if let Some((ty_ident, in_vec, in_opt)) = identify_field_type(&field.ty) {
        if in_opt {
            return option_field::collect_impl(
                field_name, ty_ident, cfg_vec, init_vec, set_vec, &attr_map, attr_span, field_span,
            );
        } else if in_vec {
            return vector_field::collect_impl(
                field_name, ty_ident, cfg_vec, init_vec, set_vec, &attr_map, attr_span, field_span,
            );
        } else {
            return scalar_field::collect_impl(
                field_name, ty_ident, cfg_vec, init_vec, set_vec, &attr_map, attr_span, field_span,
            );
        }
    }

    Err(OptStoreErr::BadFieldType.at(field_span))
}

fn collect_impl_for_field_attr(
    attr: &syn::Attribute,
    attr_map: &mut HashMap<String, String>,
    attr_span: proc_macro2::Span,
) -> Result<(), syn::Error> {
    let nested = attr.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    )?;
    for meta in nested {
        match meta {
            syn::Meta::NameValue(meta) => {
                if meta.path.is_ident("cfg") {
                    match meta.value {
                        syn::Expr::Lit(lit) => match lit.lit {
                            syn::Lit::Str(s) => {
                                let value = s.value();
                                match &value.split_once("=") {
                                    Some((lhs, rhs)) => {
                                        attr_map.insert("names".to_string(), lhs.to_string());
                                        attr_map.insert("defaults".to_string(), rhs.to_string());
                                    }
                                    None => {
                                        attr_map.insert("names".to_string(), value);
                                    }
                                }
                            }
                            _ => return Err(OptStoreErr::BadAttrMetaValueCfg.at(attr_span)),
                        },
                        _ => return Err(OptStoreErr::BadAttrMetaValueCfg.at(attr_span)),
                    }
                } else if meta.path.is_ident("desc") {
                    match meta.value {
                        syn::Expr::Lit(lit) => match lit.lit {
                            syn::Lit::Str(s) => {
                                attr_map.insert("desc".to_string(), s.value());
                            }
                            _ => return Err(OptStoreErr::BadAttrMetaValueDesc.at(attr_span)),
                        },
                        _ => return Err(OptStoreErr::BadAttrMetaValueDesc.at(attr_span)),
                    }
                } else if meta.path.is_ident("arg") {
                    match meta.value {
                        syn::Expr::Lit(lit) => match lit.lit {
                            syn::Lit::Str(s) => {
                                attr_map.insert("arg".to_string(), s.value());
                            }
                            _ => return Err(OptStoreErr::BadAttrMetaValueArg.at(attr_span)),
                        },
                        _ => return Err(OptStoreErr::BadAttrMetaValueArg.at(attr_span)),
                    }
                } else {
                    return Err(OptStoreErr::BadAttrMetaName.at(attr_span));
                }
            }
            _ => return Err(OptStoreErr::BadAttrMetaName.at(attr_span)),
        }
    }

    Ok(())
}
