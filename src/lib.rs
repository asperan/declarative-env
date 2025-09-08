//! This crate contains a macro for generating a struct which reads the environmental variables
//! defined in the configured file.
//!
//! This allows to have a single point of definition for the env configuration, thus avoiding a
//! possible incoherence between the documentation and the implementation.
//!
//! # Usage
//! The macro must be attached to an empty struct definition:
//! ```no_run
//!    #[declarative_env!(path = "./example.hjson", format = "hjson")]
//!    struct MyConfig;
//! ```
//! > Note: the struct can have any visibility.
//!
//! The macro creates a struct with the same name of the empty struct, but it injects as fields all
//! the variables defined in the `path` keyword.
//!
//! For example, the definition file:
//! ```hjson
//! SERVER_PORT: {
//!    type: u16
//!    description: The port the server will listen on
//!    default: 8080
//! }
//! ```
//! generates the struct
//! ```
//! struct MyConfig {
//!    SERVER_PORT: u16,
//! }
//! ```
//!
//! The path of the definition file is relative to the cargo manifest dir (the one with
//! `Cargo.toml`).
//!
//! The supported formats are specified in the `AcceptedFormat` enum; formats are the lowercase
//! version of the enum variants, so `AcceptedFormat::Hjson` is selected with `format = "hjson"`.
//!
//! ## Env Variable Definition
//! The env variable definition file contains a map of objects, where the key is the name of the
//! env variable, and the value contains the metadata of the variable:
//! - the **type** (accepted types are defined in `AcceptedRustType`).
//! - the **description**, unused in generated code, required for documentation purpose.
//! - the **default** value, optional (it must be parsable as the defiend **type**).
//!
//! ### Accepted formats
//! This crate supports the following formats for the definition file:
//! - hjson
//!
//! ### Accepted types
//! This crate supports the following types for the variables defined:
//! - iX (X = {8, 16, 32, 64, 128})
//! - uX (X = {8, 16, 32, 64, 128})
//! - bool
//! - String
//! - f32,f64
#![deny(missing_docs)]

mod accepted_rust_type;
mod declarative_env_generator;
mod empty_struct;
mod macro_config;
mod variable_declarations;

use std::path::PathBuf;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Error as SynError};

use crate::{
    declarative_env_generator::DeclarativeEnvGenerator, empty_struct::EmptyStruct,
    macro_config::MacroConfig, variable_declarations::EnvVariableDeclarations,
};

extern crate proc_macro;

/// This is the entrypoint for the crate.
#[proc_macro_attribute]
pub fn declarative_env(attr: TokenStream, item: TokenStream) -> TokenStream {
    let macro_config = parse_macro_input!(attr as MacroConfig);
    let file_content = match read_env_definition_file(macro_config.path()) {
        Ok(c) => c,
        Err(e) => return e.to_compile_error().into(),
    };

    let empty_struct = parse_macro_input!(item as EmptyStruct);
    let variable_configs: EnvVariableDeclarations = match macro_config.format() {
        macro_config::AcceptedFormat::Hjson => match deser_hjson::from_str(&file_content) {
            Ok(v) => v,
            Err(e) => {
                return SynError::new(Span::call_site(), e)
                    .to_compile_error()
                    .into()
            }
        },
    };

    DeclarativeEnvGenerator::new(empty_struct, variable_configs).generate()
}

#[doc(hidden)]
fn read_env_definition_file(path: &str) -> Result<String, SynError> {
    let cargo_manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").map_err(|it| SynError::new(Span::call_site(), it))?;
    let config_file_path = PathBuf::from(cargo_manifest_dir).join(path);
    std::fs::read_to_string(config_file_path).map_err(|it| SynError::new(Span::call_site(), it))
}
