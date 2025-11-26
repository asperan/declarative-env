use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::token::Struct as SynStruct;
use syn::Ident as SynIdent;

use crate::{EmptyStruct, EnvVariableDeclarations};

pub struct DeclarativeEnvGenerator {
    empty_struct: EmptyStruct,
    variable_declarations: EnvVariableDeclarations,
}

impl DeclarativeEnvGenerator {
    pub fn new(
        empty_struct: EmptyStruct,
        variable_declarations: EnvVariableDeclarations,
    ) -> DeclarativeEnvGenerator {
        DeclarativeEnvGenerator {
            empty_struct,
            variable_declarations,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let mut result = TokenStream2::new();
        result.extend(self.create_struct_def());
        result.extend(self.create_struct_impl());
        result.extend(self.requiredenvnotfounderror_code());
        result.into()
    }

    fn create_struct_def(&self) -> TokenStream2 {
        let mut token_stream: TokenStream2 = TokenStream2::new();
        token_stream.extend(quote! { #[derive(Debug, Clone)] });
        token_stream.extend(self.empty_struct.visibility().to_token_stream());
        token_stream.extend(SynStruct::default().to_token_stream());
        token_stream.extend(
            SynIdent::new(self.empty_struct.struct_name(), Span2::call_site()).to_token_stream(),
        );

        let mut fields = TokenStream2::new();
        for it in self.variable_declarations.as_ref() {
            let variable_name = SynIdent::new(it.name(), Span2::call_site());
            let variable_type = it.rust_type();
            fields.extend(quote! { #variable_name: #variable_type, });
        }
        token_stream.extend(quote! { { #fields } });
        token_stream
    }

    fn create_struct_impl(&self) -> TokenStream2 {
        let mut token_stream: TokenStream2 = TokenStream2::new();
        let struct_name = SynIdent::new(self.empty_struct.struct_name(), Span2::call_site());
        let requiredenvnotfounderror_struct_ident = self.requiredenvnotfounderror_struct_ident();
        let mut functions: TokenStream2 = TokenStream2::new();
        let mut variable_reads = TokenStream2::new();
        for it in self.variable_declarations.as_ref() {
            let var_name = SynIdent::new(it.name(), Span2::call_site());
            let var_name_str = it.name();
            let var_type = it.rust_type();
            let default_behaviour = match &it.default_value() {
                Some(v) => {
                    quote! {
                        #v.parse::<#var_type>()?
                    }
                }
                None => quote! {
                    return Err(#requiredenvnotfounderror_struct_ident::new(#var_name_str).into())
                },
            };
            variable_reads.extend(quote! {
                let #var_name = match std::env::var(#var_name_str) {
                    Ok(v) => v.parse::<#var_type>()?,
                    Err(std::env::VarError::NotPresent) => #default_behaviour,
                    Err(e) => return Err(e.into()),
                };
            });
        }
        let all_vars = self
            .variable_declarations
            .iter()
            .map(|it| SynIdent::new(it.name(), Span2::call_site()));
        functions.extend(quote! {
            pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
                #variable_reads
                Ok(#struct_name { #(#all_vars),* })
            }
        });
        for it in self.variable_declarations.as_ref() {
            let fn_name = SynIdent::new(it.name(), Span2::call_site());
            let fn_return_type = it.rust_type().to_struct_return_type();
            let self_ref = it.rust_type().to_struct_self_caller();
            functions.extend(quote! {
                pub fn #fn_name(&self) -> #fn_return_type {
                    #self_ref.#fn_name
                }
            });
        }
        token_stream.extend(quote! {
            impl #struct_name {
                #functions
            }
        });
        token_stream
    }

    fn requiredenvnotfounderror_struct_ident(&self) -> SynIdent {
        format_ident!(
            "{}RequiredEnvNotFoundError",
            self.empty_struct.struct_name()
        )
    }

    fn requiredenvnotfounderror_code(&self) -> TokenStream2 {
        let error_struct_name = self.requiredenvnotfounderror_struct_ident();
        quote! {
            #[derive(Debug)]
            pub struct #error_struct_name {
                key: String,
            }

            impl #error_struct_name {
                fn new(key: &str) -> Self {
                    Self { key: key.to_string() }
                }
            }

            impl std::fmt::Display for #error_struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                    f,
                    "failed to load configuration from env: variable {} not found and no default value specified",
                    self.key,
                    )
                }
            }

            impl std::error::Error for #error_struct_name {}
        }
    }
}
