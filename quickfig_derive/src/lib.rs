// extern crate proc_macro;
#![allow(unused, dead_code)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse_macro_input, Attribute, Data, DeriveInput, Ident, Lit, Meta, Type, LitStr, punctuated::Punctuated, Token
};

// https://doc.rust-lang.org/book/ch20-05-macros.html

#[proc_macro_derive(ConfigFields, attributes(keys))]
pub fn config_field_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_config_field_macro(&ast)
}

struct VariantDefinition {
    ident: Ident,
    keys: Vec<String>
}

impl VariantDefinition {
    fn new(ident: Ident) -> Self {
        Self {ident, keys: vec![]}
    }
    fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }
}

fn impl_config_field_macro(ast: &syn::DeriveInput) -> TokenStream {
    // This is the type that `ConfigFields` macro is derived on aka users enum
    let name = &ast.ident;
    let user_enum_name = &ast.ident;
    let mut variant_defs: Vec<VariantDefinition> = vec![];

    match &ast.data {
        Data::Enum(data_enum) => {
            // enum variants on the user's Enum
            for variant in data_enum.variants.iter() {
                // println!("-------------------------------------");
                // let variant_name = &variant.ident;
                // println!("- START VARIANT: {} -", variant_name);
                let mut this_variant = VariantDefinition::new(variant.ident.clone());

                for attr in &variant.attrs {
                    // println!("--- START ATTR: {:#?} ---", "");
                    // let attr_name = attr.path();
                    match attr.path().get_ident() {
                        Some(ident) if ident.eq("keys") => {
                            let keys: Punctuated<LitStr, Token![,]> = attr
                                .parse_args_with(Punctuated::parse_terminated)
                                .expect("Failed to parse keys attribute");

                            for key in keys {
                                this_variant.add_key(key.value());
                            }
                        },
                        _ => {
                            // Some(f) and None and _ don't cause LSP hints on panic
                            todo!()
                        }
                    };
                };

                variant_defs.push(this_variant);
            }
        },
        Data::Struct(_data_struct) => {
            unimplemented!("ConfigFields can only be derived on Enums")
        },
        Data::Union(_data_union) => {
            unimplemented!("ConfigFields can only be derived on Enums")
        },
    };

    // NOTE: ALWAYS use full path for EVERYTHING in interpolated tokenstream

    let mut match_arms: Vec<quote::__private::TokenStream> = Vec::new();

    for variant in variant_defs.into_iter() {
        let var_name = variant.ident;
        let var_keys = variant.keys;

        let field_keys: Vec<String> = match var_keys.is_empty() {
            true => { 
                // no keys on variant, use variant name
                vec![var_name.to_string()]
            },
            false => {
                // variant had keys attr, use them
                var_keys
            }
        };

        let key_actions: Vec<quote::__private::TokenStream> = field_keys.iter()
            .map(|key| {
                // for key in field_keys, if self.has_key create Field then push
                quote! {
                    if !self.has_key(#key) {
                        // println!("key not found: {}", #key);
                    } else {
                        if let Some(field) = self.create_field(#key) {
                            return_fields.push(field);
                        }
                    }
                }
            })
            .collect();
        
        let match_arm = quote! {
            #user_enum_name::#var_name => {
                let mut return_fields = vec![];
                #(#key_actions)*
        
                // if empty (none of the keys existed) return None
                if return_fields.is_empty() {
                    return None;
                } else {
                    return Some(return_fields);
                }
            }
        };

        match_arms.push(match_arm);
    }

    // Generate unique ident so user can derive on multiple types
    let trait_name = format!("QuickfigConfigTrait{}", name);
    let trait_ident = syn::Ident::new(&trait_name, name.span());

    let impl_gen = quote! {

        impl ::quickfig::core::ConfigFields for #name {}

        trait #trait_ident<S> 
            where
                S: ::quickfig::serde::de::DeserializeOwned + ::quickfig::core::config_types::DeserializedConfig,
        {
            type CF: ::quickfig::core::ConfigFields;
            fn get<'a>(&'a self, user_enum: Self::CF) -> 
            std::option::Option<std::vec::Vec<::quickfig::core::Field<'a, S>>>;
        }

        impl #trait_ident<::quickfig::core::config_types::JSON> for ::quickfig::core::Config<::quickfig::core::config_types::JSON> {
            type CF = #name;

            fn get<'a>(&'a self, user_enum: Self::CF) -> std::option::Option<std::vec::Vec<::quickfig::core::Field<'a, ::quickfig::core::config_types::JSON>>> {
                
                match user_enum {
                    #(#match_arms)*,
                }
            }
        }

        impl #trait_ident<::quickfig::core::config_types::TOML> for ::quickfig::core::Config<::quickfig::core::config_types::TOML> {
            type CF = #name;

            fn get<'a>(&'a self, user_enum: Self::CF) -> std::option::Option<std::vec::Vec<::quickfig::core::Field<'a, ::quickfig::core::config_types::TOML>>> {
                
                match user_enum {
                    #(#match_arms)*,
                }
            }
        }
    };

    impl_gen.into()
}

