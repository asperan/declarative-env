use std::{fmt::Display, str::FromStr};

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum AcceptedRustType {
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    String,
    bool,
    f32,
    f64,
}

impl AcceptedRustType {
    pub fn to_struct_return_type(self) -> TokenStream2 {
        let str_type = match self {
            AcceptedRustType::i8 => "i8",
            AcceptedRustType::i16 => "i16",
            AcceptedRustType::i32 => "i32",
            AcceptedRustType::i64 => "i64",
            AcceptedRustType::i128 => "i128",
            AcceptedRustType::u8 => "u8",
            AcceptedRustType::u16 => "u16",
            AcceptedRustType::u32 => "u32",
            AcceptedRustType::u64 => "u64",
            AcceptedRustType::u128 => "u128",
            AcceptedRustType::f32 => "f32",
            AcceptedRustType::f64 => "f64",
            AcceptedRustType::bool => "bool",
            AcceptedRustType::String => "&str",
        };
        TokenStream2::from_str(str_type)
            .expect("AcceptedRustType::toStructReturnType always create a valid TokenStream")
    }

    pub fn to_struct_self_caller(self) -> TokenStream2 {
        let self_str = match self {
            AcceptedRustType::String => "&self",
            AcceptedRustType::i8
            | AcceptedRustType::i16
            | AcceptedRustType::i32
            | AcceptedRustType::i64
            | AcceptedRustType::i128
            | AcceptedRustType::u8
            | AcceptedRustType::u16
            | AcceptedRustType::u32
            | AcceptedRustType::u64
            | AcceptedRustType::u128
            | AcceptedRustType::f32
            | AcceptedRustType::f64
            | AcceptedRustType::bool => "self",
        };
        TokenStream2::from_str(self_str)
            .expect("AcceptedRustType::toStructSelfCaller returns a valid self usage")
    }
}

impl Display for AcceptedRustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AcceptedRustType::i8 => "i8",
                AcceptedRustType::i16 => "i16",
                AcceptedRustType::i32 => "i32",
                AcceptedRustType::i64 => "i64",
                AcceptedRustType::i128 => "i128",
                AcceptedRustType::u8 => "u8",
                AcceptedRustType::u16 => "u16",
                AcceptedRustType::u32 => "u32",
                AcceptedRustType::u64 => "u64",
                AcceptedRustType::u128 => "u128",
                AcceptedRustType::f32 => "f32",
                AcceptedRustType::f64 => "f64",
                AcceptedRustType::String => "String",
                AcceptedRustType::bool => "bool",
            }
        )
    }
}

impl ToTokens for AcceptedRustType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(
            TokenStream2::from_str(&self.to_string())
                .expect("AcceptedRustType contains valid Rust types"),
        );
    }
}
