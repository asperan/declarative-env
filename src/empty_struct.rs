use syn::{parse::Parse, Ident, Token, Visibility};

pub struct EmptyStruct {
    visibility: Visibility,
    struct_name: String,
}

impl EmptyStruct {
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn struct_name(&self) -> &str {
        &self.struct_name
    }
}

impl Parse for EmptyStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_visibility: Visibility = input.parse()?;
        let _struct_keyword: Token![struct] = input.parse()?;
        let struct_name: Ident = input.parse()?;
        let _semicolon: Token![;] = input.parse()?;

        Ok(EmptyStruct {
            visibility: struct_visibility,
            struct_name: struct_name.to_string(),
        })
    }
}
