use std::ops::Deref;

use serde::{de::Visitor, Deserialize};

use crate::accepted_rust_type::AcceptedRustType;

#[derive(Debug)]
pub struct EnvVariableSpec {
    name: String,
    rust_type: AcceptedRustType,
    description: String,
    default_value: Option<String>,
}

impl EnvVariableSpec {
    fn from_name_and_fields(name: String, fields: VariableConfiguration) -> EnvVariableSpec {
        EnvVariableSpec {
            name,
            rust_type: fields.rust_type,
            description: fields.description,
            default_value: fields.default_value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rust_type(&self) -> AcceptedRustType {
        self.rust_type
    }

    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }
}

#[derive(Debug)]
pub struct EnvVariableDeclarations {
    declarations: Vec<EnvVariableSpec>,
}

impl Deref for EnvVariableDeclarations {
    type Target = Vec<EnvVariableSpec>;

    fn deref(&self) -> &Self::Target {
        &self.declarations
    }
}

impl AsRef<Vec<EnvVariableSpec>> for EnvVariableDeclarations {
    fn as_ref(&self) -> &Vec<EnvVariableSpec> {
        &self.declarations
    }
}

impl<'de> Deserialize<'de> for EnvVariableDeclarations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(EnvVariableDeclarations {
            declarations: deserializer
                .deserialize_map(VariableDeclarationVisitor::new())?
                .into_iter()
                .map(|it| EnvVariableSpec::from_name_and_fields(it.0, it.1))
                .collect(),
        })
    }
}

struct VariableDeclarationVisitor {}

impl VariableDeclarationVisitor {
    fn new() -> Self {
        VariableDeclarationVisitor {}
    }
}

impl<'de> Visitor<'de> for VariableDeclarationVisitor {
    type Value = Vec<(String, VariableConfiguration)>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Vec::new())
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut values = Vec::with_capacity(core::cmp::min(map.size_hint().unwrap_or(0), 128));

        while let Some((key, value)) = map.next_entry()? {
            values.push((key, value));
        }

        Ok(values)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct VariableConfiguration {
    #[serde(rename(deserialize = "type"))]
    rust_type: AcceptedRustType,
    // This field is used to require a description for a variable, and for future uses in case
    // it will be possible to add documentation comments to a generated token stream.
    #[allow(dead_code)]
    description: String,
    #[serde(rename(deserialize = "default"))]
    default_value: Option<String>,
}
