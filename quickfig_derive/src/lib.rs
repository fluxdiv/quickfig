// extern crate proc_macro;
#![allow(unused, dead_code)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse_macro_input, Attribute, Data, DeriveInput, Ident, Lit, Meta, Type, LitStr, punctuated::Punctuated, Token
};
use anyhow::Result;
use quickfig_core::{
    FieldMarker,
    // Field,
    Field2,
    ConfigFields,
};

// quickfig_derive
// https://doc.rust-lang.org/book/ch20-05-macros.html

#[proc_macro_derive(ConfigFields, attributes(keys))]
pub fn config_field_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_config_field_macro(&ast)
}

// fn meta_to_field(meta: ParseNestedMeta<'_>) -> Result<FieldMarker, syn::Error> {
//     if let Some(ident) = meta.path.get_ident() {
//         let ty: syn::Type = match syn::parse(ident.to_token_stream().into()) {
//             Ok(t) => t,
//             Err(_e) => {
//                 return Err(meta.error("Failure parsing ident"));
//             }
//         };
//         // println!("ty: {:#?}", ty);
//         if let syn::Type::Path(type_path) = ty {
//             if let Some(at) = FieldMarker::from_type_path(&type_path) {
//                 return Ok(at);
//             } else {
//                 return Err(meta.error("Unsupported type. Available types are String, bool, char, u8..u128, i8..i128, f32..f64"));
//             }
//         } else {
//             return Err(meta.error("ty is not Type::Path"));
//         }
//     } else {
//         return Err(meta.error("Must be singular type"));
//     }
// }

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
                        // Some(ident) if ident.eq("must_be") => {
                        //     let mut count = 0;
                        //     attr.parse_nested_meta(|meta| {
                        //         if count != 0 {
                        //             // This does cause LSP hints, but on enum itself
                        //             // I think using span here is the way
                        //             // panic!();
                        //             return Err(meta.error("Must be 1 param"));
                        //         } else {count += 1;}
                        //
                        //         let allowed: FieldMarker = meta_to_field(meta)?;
                        //         this_variant.add_type(allowed);
                        //         // let astr = format!("{:#?}", allowed);
                        //         Ok(())
                        //     }).unwrap();
                        // },
                        // Some(ident) if ident.eq("any_of") => {
                        //     attr.parse_nested_meta(|meta| {
                        //         let allowed: FieldMarker = meta_to_field(meta)?;
                        //         this_variant.add_type(allowed);
                        //         // let astr = format!("{:#?}", allowed);
                        //         Ok(())
                        //     }).unwrap();
                        // },
                        Some(ident) if ident.eq("keys") => {
                            let keys: Punctuated<LitStr, Token![,]> = attr
                                .parse_args_with(Punctuated::parse_terminated)
                                .expect("Failed to parse keys attribute");

                            for key in keys {
                                this_variant.add_key(key.value());
                                // println!("Got key: {}", key.value());
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

    // Note to self: ALWAYS use full path for -EVERYTHING- in interpolated tokenstream

    let mut match_arms: Vec<quote::__private::TokenStream> = Vec::new();

    // For each variant in the user's enum with derive(ConfigFields)

    // let mut fields = Vec<Field2>;
    
    // Handling Keys
    // - If the variant has no #[keys("id", "ID")], 
    //   then the key is simply the variant name,
    //   and there is only 1 Field2 to create.
    //   - let key = #var_name;
    //   - let value: impl Deserialize = config.get_at_str(#var_name);
    //   - let f: Field2 = Field2 { key, value };
    //   - fields.push(f);
    // - If it does have #[keys("id", "ID")],
    //   then I need to make a Field2 for each key.

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
                // for key in field_keys, if self.has_key create Field2 then push
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

        impl quickfig::core::ConfigFields for #name {
            fn hello_macro() {
                println!("Hello, Macro! my name is {}!", stringify!(#name));
                println!();
            }
        }

        trait #trait_ident<S> 
            where
                S: serde::de::DeserializeOwned + quickfig::core::config_types::DeserializedConfig,
        {
            type CF: quickfig::core::ConfigFields;
            fn get<'a>(&'a self, user_enum: Self::CF) -> 
            std::option::Option<std::vec::Vec<quickfig::core::Field2<'a, S>>>;
        }

        impl #trait_ident<quickfig::core::config_types::JSON> for quickfig::core::Config<quickfig::core::config_types::JSON> {
            type CF = #name;

            fn get<'a>(&'a self, user_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig::core::Field2<'a, quickfig::core::config_types::JSON>>> {
                
                // TODO
                // Each arm in this match statement returns Option<Vec<Field>>
                // and UNLESS they have #[non-exhaustive] on their enum, I dont 
                // think I need to handle the _ arm / default None return
                match user_enum {
                    #(#match_arms)*,
                }

                return None;
            }
        }

        impl #trait_ident<quickfig::core::config_types::TOML> for quickfig::core::Config<quickfig::core::config_types::TOML> {
            type CF = #name;

            fn get<'a>(&'a self, user_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig::core::Field2<'a, quickfig::core::config_types::TOML>>> {
                
                match user_enum {
                    #(#match_arms)*,
                }

                return None;
            }
        }
    };

    impl_gen.into()
}

