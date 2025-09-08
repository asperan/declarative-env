use hierrorchy::{error_leaf, error_node};
use std::{error::Error, str::FromStr};
use syn::{parse::Parse, Error as SynError, Ident, LitStr, Token};

pub struct MacroConfig {
    path: String,
    format: AcceptedFormat,
}

impl MacroConfig {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn format(&self) -> AcceptedFormat {
        self.format
    }
}

impl Parse for MacroConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut macro_config_builder = MacroConfigBuilder::new();
        while !input.is_empty() {
            let keyword: Ident = input.parse()?;
            match keyword
                .to_string()
                .parse::<MacroConfigKeyword>()
                .map_err(|it| SynError::new(keyword.span(), it))?
            {
                MacroConfigKeyword::Path => {
                    let _: Token![=] = input.parse()?;
                    let value: LitStr = input.parse()?;
                    if !input.is_empty() {
                        let _: Token![,] = input.parse()?;
                    }
                    macro_config_builder.set_path(value.value());
                }
                MacroConfigKeyword::Format => {
                    let _: Token![=] = input.parse()?;
                    let value: LitStr = input.parse()?;
                    if !input.is_empty() {
                        let _: Token![,] = input.parse()?;
                    }
                    macro_config_builder.set_format(
                        value
                            .value()
                            .parse::<AcceptedFormat>()
                            .map_err(|it| SynError::new(value.span(), it))?,
                    );
                }
            }
        }
        macro_config_builder
            .build()
            .map_err(|it| SynError::new(input.span(), it))
    }
}

struct MacroConfigBuilder {
    path: Option<String>,
    format: Option<AcceptedFormat>,
}

impl MacroConfigBuilder {
    pub fn new() -> Self {
        MacroConfigBuilder {
            path: None,
            format: None,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn set_format(&mut self, format: AcceptedFormat) {
        self.format = Some(format);
    }

    pub fn build(&self) -> Result<MacroConfig, MacroConfigBuilderError> {
        if self.path.is_none() {
            return Err(MissingRequiredConfigurationError {
                keyword: String::from("path"),
            }
            .into());
        }
        Ok(MacroConfig {
            path: self
                .path
                .as_ref()
                .expect("path existence is already checked")
                .clone(),
            format: self.format.unwrap_or(AcceptedFormat::Hjson),
        })
    }
}

error_node! {
    pub type MacroConfigBuilderError<MissingRequiredConfigurationError> = "failed to build macro configuration"
}

#[error_leaf(format!("the required configuration '{}' is missing", self.keyword))]
pub struct MissingRequiredConfigurationError {
    keyword: String,
}

#[derive(Debug, Clone, Copy)]
enum MacroConfigKeyword {
    Path,
    Format,
}

impl FromStr for MacroConfigKeyword {
    type Err = UnknownOptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "path" => Ok(Self::Path),
            "format" => Ok(Self::Format),
            _ => Err(UnknownOptionError {
                keyword: s.to_string(),
            }),
        }
    }
}

#[error_leaf(format!("unknown option '{}'", self.keyword))]
pub struct UnknownOptionError {
    keyword: String,
}

#[derive(Debug, Clone, Copy)]
pub enum AcceptedFormat {
    Hjson,
}

impl FromStr for AcceptedFormat {
    type Err = InvalidFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hjson" => Ok(Self::Hjson),
            _ => Err(InvalidFormatError {
                format: s.to_string(),
            }),
        }
    }
}

#[error_leaf(format!("invalid format '{}'", self.format))]
pub struct InvalidFormatError {
    format: String,
}
