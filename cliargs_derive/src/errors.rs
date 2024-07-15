// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

pub enum OptStoreErr {
    MustPutOnStruct,
    MustNotPutOnTuple,
    BadFieldType,
    BadAttrMetaName,
    BadAttrMetaValueCfg,
    BadAttrMetaValueDesc,
    BadAttrMetaValueArg,
    MustNotHasDefaults(String),
    InvalidNumberFormat(String, String),
}

impl OptStoreErr {
    fn msg(&self) -> String {
        match self {
            OptStoreErr::MustPutOnStruct => String::from("must be attached to a struct"),
            OptStoreErr::MustNotPutOnTuple => String::from("must not be attached to a tuple"),
            OptStoreErr::BadFieldType => String::from(
                "accept only bool, primitive number types, String, \
                 Vec<> of primitive number, Vec<> of String, \
                 Option<> primitive number, Option<> of String",
            ),
            OptStoreErr::BadAttrMetaName => String::from(
                "field attribute `opt` accepts only cfg=\"...\", \
                 desc=\"...\", and arg=\"...\"",
            ),
            OptStoreErr::BadAttrMetaValueCfg => {
                String::from("`cfg` in field attributes `opt` must be a string literal")
            }
            OptStoreErr::BadAttrMetaValueDesc => {
                String::from("`desc` in field attributes `opt` must be a string literal")
            }
            OptStoreErr::BadAttrMetaValueArg => {
                String::from("`arg` in field attributes `opt` must be a string literal")
            }
            OptStoreErr::MustNotHasDefaults(fld) => {
                format!("`{fld}` is bool, so the default value cannot be specified")
            }
            OptStoreErr::InvalidNumberFormat(fld, ty) => {
                format!("`{fld}` is {ty}, but the default value is invalid format")
            }
        }
    }

    pub fn at(&self, span: proc_macro2::Span) -> syn::Error {
        const PREFIX: &str = "[derive macro `OptStore`]";
        syn::Error::new(span, format!("{PREFIX} {}", self.msg()))
    }
}
