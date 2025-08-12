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
    AllowedType,
    AllowedTypeWrapper,
    AT, ATW,
    ConfigFields,
};

// quickfig_derive
// https://doc.rust-lang.org/book/ch20-05-macros.html

#[proc_macro_derive(ConfigFields, attributes(must_be, any_of, keys))]
pub fn config_field_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_config_field_macro(&ast)
}

fn meta_to_allowedtype(meta: ParseNestedMeta<'_>) -> Result<AllowedType, syn::Error> {
    if let Some(ident) = meta.path.get_ident() {
        let ty: syn::Type = match syn::parse(ident.to_token_stream().into()) {
            Ok(t) => t,
            Err(_e) => {
                return Err(meta.error("Failure parsing ident"));
            }
        };
        // println!("ty: {:#?}", ty);
        if let syn::Type::Path(type_path) = ty {
            if let Some(at) = AllowedType::from_type_path(&type_path) {
                return Ok(at);
            } else {
                return Err(meta.error("Unsupported type. Available types are String, bool, char, u8..u128, i8..i128, f32..f64"));
            }
        } else {
            return Err(meta.error("ty is not Type::Path"));
        }
    } else {
        return Err(meta.error("Must be singular type"));
    }
}

struct VariantDefinition {
    ident: Ident,
    allowed_types: Vec<AllowedType>,
    keys: Vec<String>
}

impl VariantDefinition {
    fn new(ident: Ident) -> Self {
        Self {ident, allowed_types: vec![], keys: vec![]}
    }
    fn add_type(&mut self, ty: AllowedType) {
        self.allowed_types.push(ty);
    }
    fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }
}

fn impl_config_field_macro(ast: &syn::DeriveInput) -> TokenStream {
    // This is the type that `ConfigFields` macro is derived on aka users enum
    let name = &ast.ident;
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
                        Some(ident) if ident.eq("must_be") => {
                            let mut count = 0;
                            attr.parse_nested_meta(|meta| {
                                if count != 0 {
                                    // This does cause LSP hints, but on enum itself
                                    // I think using span here is the way
                                    // panic!();
                                    return Err(meta.error("Must be 1 param"));
                                } else {count += 1;}

                                let allowed: AllowedType = meta_to_allowedtype(meta)?;
                                this_variant.add_type(allowed);
                                // let astr = format!("{:#?}", allowed);
                                Ok(())
                            }).unwrap();
                        },
                        Some(ident) if ident.eq("any_of") => {
                            attr.parse_nested_meta(|meta| {
                                let allowed: AllowedType = meta_to_allowedtype(meta)?;
                                this_variant.add_type(allowed);
                                // let astr = format!("{:#?}", allowed);
                                Ok(())
                            }).unwrap();
                        },
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

    for variant in variant_defs.into_iter() {
        let var_name = variant.ident;
        let var_types = variant.allowed_types;
        let var_keys = variant.keys;

        let at_actions: Vec<quote::__private::TokenStream> = var_types.iter()
            .map(|allowed_type| {
                let at_ident = Ident::new(
                    &format!("{:?}", allowed_type),
                    Span::call_site()
                );
                quote! {
                    // only push if some (if parsing was successful)
                    if let Some(at) = self.parse_allowed_type(
                        stringify!(#var_name),
                        quickfig_core::AllowedType::#at_ident
                    ) {
                        at_wrappers.push(at);
                    }
                }
            })
            .collect();

        let key_actions: Vec<quote::__private::TokenStream> = var_keys.iter()
            .map(|key| {
                let at_actions_for_key: Vec<quote::__private::TokenStream> = var_types
                    .iter()
                    .map(|allowed_type| {
                        let at_ident = Ident::new(
                            &format!("{:?}", allowed_type),
                            Span::call_site()
                        );
                        quote! {
                            // only push if some (if parsing was successful)
                            if let Some(at) = self.parse_allowed_type(
                                #key,
                                quickfig_core::AllowedType::#at_ident
                            ) {
                                at_wrappers.push(at);
                            }
                        }
                    })
                    .collect();

                quote! {
                    if !self.has_key(#key) {
                        println!("key not found: {}", stringify!(#key));
                    } else {
                        println!("key found: {}", stringify!(#key));
                        #(#at_actions_for_key)*
                    }
                }
            })
            .collect();



        // I should definitely be able to unify this in a simpler way right
        let key_branch = if var_keys.is_empty() {
            // keys is empty, proceed as normal using variant name as key
            quote! {
                // Should error or differentiate between non-existent key
                // and unable to parse key into given types
                if !self.has_key(stringify!(#var_name)) {
                    // Should this be an error?
                    println!("key not found: {}", stringify!(#var_name));
                } else {
                    // Config file has the key associated with this variant's name
                    println!("key found: {}", stringify!(#var_name));
                    // declare this here, then have each iter loop push to it
                    let mut at_wrappers: Vec<quickfig_core::ATW> = vec![];
                    // for each allowed type in list, try to parse &Value
                    // self.parse_allowed_type(at: AllowedType) -> Option<ATWrapper>
                    // in each iteration I push to at_wrappers, but only if some
                    #(#at_actions)*

                    // if empty (no parse success), return None
                    // is this API/return schema kind of confusing though?
                    if at_wrappers.is_empty() {
                        return None;
                    } else {
                        return Some(at_wrappers);
                    }
                }
            }
        } else {
            // this variants keys wasn't empty, for key in keys do at_action using key instead of #var
            quote!{
                let mut at_wrappers: Vec<quickfig_core::ATW> = vec![];

                #(#key_actions)*

                if at_wrappers.is_empty() {
                    return None;
                } else {
                    return Some(at_wrappers);
                }
            }
        };

        let match_arm = quote! {
            #name::#var_name => {
                // println!("variant is {}", stringify!(#var_name));
                #key_branch
            }
        };

        match_arms.push(match_arm);
    }

    // Generate unique ident so user can derive on multiple types
    let trait_name = format!("QuickfigConfigTrait{}", name);
    let trait_ident = syn::Ident::new(&trait_name, name.span());

    let impl_gen = quote! {

        impl ConfigFields for #name {
            fn hello_macro() {
                println!("Hello, Macro! my name is {}!", stringify!(#name));
                println!();
            }
        }

        trait #trait_ident {
            type CF: quickfig_core::ConfigFields;
            fn get(&self, their_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig_core::ATW>>;
        }

        impl #trait_ident for quickfig_core::Config<quickfig_core::config_types::JSON> {
            type CF = #name;

            fn get(&self, their_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig_core::ATW>> {
                
                // Each arm in this match statement returns Option<Vec<ATW>>
                // and UNLESS they have #[non-exhaustive] on their enum, I dont 
                // think I need to handle the _ arm / default None return
                match their_enum {
                    #(#match_arms)*,
                }

                return None;
            }
        }

        impl #trait_ident for quickfig_core::Config<quickfig_core::config_types::TOML> {
            type CF = #name;

            fn get(&self, their_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig_core::ATW>> {
                
                match their_enum {
                    #(#match_arms)*,
                }

                return None;
            }
        }
    };

    impl_gen.into()
}



// #[proc_macro_derive(MyMacro, attributes(some_attribute))]
// pub fn my_macro(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let enum_data = match input.data {
//         syn::Data::Enum(e) => e,
//         _ => {
//             return syn::Error::new_spanned(input.ident, "MyMacro can only be derived on enums")
//                 .to_compile_error()
//                 .into();
//         }
//     };
//     for variant in enum_data.variants {
//         for attr in variant.attrs {
//             if attr.path().is_ident("some_attribute") {
//                 println!("attr in variant.attrs");
//                 println!("{:#?}", attr);
//                 match attr.meta {
//                     Meta::List(meta_list) => todo!(),
//                     Meta::Path(path) => todo!(),
//                     Meta::NameValue(meta_name_value) => todo!(),
//                 }
//             }
//         }
//     }
//
//     TokenStream::new()
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
