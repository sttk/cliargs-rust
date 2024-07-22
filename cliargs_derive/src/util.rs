// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use super::errors::OptStoreErr;
use std::collections::HashMap;

fn delve_type(ty: &syn::Type) -> Option<(&syn::Ident, Option<&syn::Type>)> {
    if let syn::Type::Path(ty_path) = ty {
        if let Some(scalar_type) = ty_path.path.get_ident() {
            return Some((scalar_type, None));
        }

        let segs = &ty_path.path.segments;
        if segs.len() == 1 {
            if let Some(seg) = segs.first() {
                let composite_type = &seg.ident;
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let Some(arg) = args.args.first() {
                            if let syn::GenericArgument::Type(t) = arg {
                                return Some((composite_type, Some(t)));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

// The elements of result tuple are:
//   - the first is the `Ident` of element scalar type.
//   - the second is the flag if stored in Vec<>.
//   - the third is the flag if stored in Option<>..
pub fn identify_field_type(ty: &syn::Type) -> Option<(&syn::Ident, bool, bool)> {
    if let Some((ty_ident, child_type)) = delve_type(&ty) {
        let field_type = ty_ident.to_string();
        if let Some(ty) = child_type {
            if let Some((ty_ident, child_type)) = delve_type(&ty) {
                if child_type.is_none() {
                    if field_type == "Vec" {
                        return Some((ty_ident, true, false));
                    } else if field_type == "Option" {
                        return Some((ty_ident, false, true));
                    }
                }
            }
        } else {
            return Some((ty_ident, false, false));
        }
    }

    None
}

pub fn parse_defaults(attr_map: &HashMap<String, String>) -> Option<Vec<String>> {
    match attr_map.get("defaults") {
        Some(s) => {
            let mut vec = vec![s.to_string()];
            if s.ends_with("]") {
                let n = s.len();
                if s.starts_with("[") {
                    let s = &s[1..n - 1];
                    if s.is_empty() {
                        vec = Vec::with_capacity(0);
                    } else {
                        vec = s.split(",").map(|s| s.to_string()).collect();
                    }
                } else {
                    let mut chars = s[..n - 1].chars();
                    if let Some(sep) = chars.next() {
                        if let Some(open_par) = chars.next() {
                            if open_par == '[' {
                                let s = chars.as_str();
                                if s.is_empty() {
                                    vec = Vec::with_capacity(0);
                                } else {
                                    vec = s.split(sep).map(|s| s.to_string()).collect();
                                }
                            }
                        }
                    }
                }
            }
            Some(vec)
        }
        None => None,
    }
}

pub fn collect_impl_of_first_number(
    arr: &[String],
    fld: &syn::Ident,
    ty: &syn::Ident,
    span: proc_macro2::Span,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match ty.to_string().as_str() {
        "i8" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0i8 });
            } else if let Ok(n) = arr[0].parse::<i8>() {
                return Ok(quote::quote! { #n });
            }
        }
        "i16" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0i16 });
            } else if let Ok(n) = arr[0].parse::<i16>() {
                return Ok(quote::quote! { #n });
            }
        }
        "i32" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0i32 });
            } else if let Ok(n) = arr[0].parse::<i32>() {
                return Ok(quote::quote! { #n });
            }
        }
        "i64" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0i64 });
            } else if let Ok(n) = arr[0].parse::<i64>() {
                return Ok(quote::quote! { #n });
            }
        }
        "i128" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0i128 });
            } else if let Ok(n) = arr[0].parse::<i128>() {
                return Ok(quote::quote! { #n });
            }
        }
        "u8" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0u8 });
            } else if let Ok(n) = arr[0].parse::<u8>() {
                return Ok(quote::quote! { #n });
            }
        }
        "u16" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0u16 });
            } else if let Ok(n) = arr[0].parse::<u16>() {
                return Ok(quote::quote! { #n });
            }
        }
        "u32" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0u32 });
            } else if let Ok(n) = arr[0].parse::<u32>() {
                return Ok(quote::quote! { #n });
            }
        }
        "u64" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0u64 });
            } else if let Ok(n) = arr[0].parse::<u64>() {
                return Ok(quote::quote! { #n });
            }
        }
        "u128" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0u128 });
            } else if let Ok(n) = arr[0].parse::<u128>() {
                return Ok(quote::quote! { #n });
            }
        }
        "f32" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0f32 });
            } else if let Ok(n) = arr[0].parse::<f32>() {
                return Ok(quote::quote! { #n });
            }
        }
        "f64" => {
            if arr.is_empty() {
                return Ok(quote::quote! { 0f64 });
            } else if let Ok(n) = arr[0].parse::<f64>() {
                return Ok(quote::quote! { #n });
            }
        }
        _ => {}
    }

    return Err(OptStoreErr::InvalidNumberFormat(fld.to_string(), ty.to_string()).at(span));
}

pub fn collect_impl_of_all_numbers(
    arr: &[String],
    fld: &syn::Ident,
    ty: &syn::Ident,
    span: proc_macro2::Span,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut vec = Vec::new();
    let mut is_err = false;

    match ty.to_string().as_str() {
        "i8" => {
            for string in arr {
                if let Ok(n) = string.parse::<i8>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "i16" => {
            for string in arr {
                if let Ok(n) = string.parse::<i16>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "i32" => {
            for string in arr {
                if let Ok(n) = string.parse::<i32>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "i64" => {
            for string in arr {
                if let Ok(n) = string.parse::<i64>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "i128" => {
            for string in arr {
                if let Ok(n) = string.parse::<i128>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "u8" => {
            for string in arr {
                if let Ok(n) = string.parse::<u8>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "u16" => {
            for string in arr {
                if let Ok(n) = string.parse::<u16>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "u32" => {
            for string in arr {
                if let Ok(n) = string.parse::<u32>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "u64" => {
            for string in arr {
                if let Ok(n) = string.parse::<u64>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "u128" => {
            for string in arr {
                if let Ok(n) = string.parse::<u128>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "f32" => {
            for string in arr {
                if let Ok(n) = string.parse::<f32>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        "f64" => {
            for string in arr {
                if let Ok(n) = string.parse::<f64>() {
                    vec.push(quote::quote! { #n });
                } else {
                    is_err = true;
                    break;
                }
            }
        }
        _ => {}
    }

    if is_err {
        return Err(OptStoreErr::InvalidNumberFormat(fld.to_string(), ty.to_string()).at(span));
    } else {
        Ok(quote::quote! { vec![#(#vec),*] })
    }
}
