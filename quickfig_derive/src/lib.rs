// extern crate proc_macro;
#![allow(unused, dead_code)]
use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta, parse_macro_input, Attribute, Data, DeriveInput, Ident, Lit, Meta, Type
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

// hello_macro_derive will be called when a user does `#[derive(ConfigFields)]` on a type
// `ConfigFields` in `proc_macro_derive(ConfigFields)` just specifies the name, I can change that,
// but it needs to match the trait name in the other lib
//
// note that the output for our derive macro is also a TokenStream. The returned TokenStream is added to the code that our crate users write, so when they compile their crate, they’ll get the extra functionality that we provide in the modified TokenStream.
// ==
// Note that the output is a TokenStream. The returned TokenStream is added to the code
// that crate users write. Basically an updated version of the Rust code they used the
// macro #[derive(ConfigFields)] on
#[proc_macro_derive(ConfigFields, attributes(must_be, any_of))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}


fn meta_to_allowedtype(meta: ParseNestedMeta<'_>) -> Result<AllowedType, syn::Error> {
    if let Some(ident) = meta.path.get_ident() {
        let ty: syn::Type = match syn::parse(ident.to_token_stream().into()) {
            Ok(t) => t,
            Err(_e) => {
                return Err(meta.error("Failure parsing ident"));
            }
        };
        if let syn::Type::Path(type_path) = ty {
            let at = AllowedType::from_type_path(&type_path);
            if let Some(at) = AllowedType::from_type_path(&type_path) {
                return Ok(at);
            } else {
                return Err(meta.error("Unsupported type"));
            }
        } else {
            return Err(meta.error("ty is not Type::Path"));
        }
    } else {
        return Err(meta.error("Must be singular type"));
    }
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // This is the type that `ConfigFields` macro is derived on aka users enum
    let name = &ast.ident;
    // let x: &Data = &ast.data;

    let mut return_types_list: Vec<AllowedType> = vec![];

    // Version 1) The API should be like 
    // // For #[must_be(A)]
    // Config.get(ConfigFields::Name) -> AllowedTypeWrapper
    // // For #[any_of(A, B, C)]
    // Config.get(ConfigFields::Age) -> Vec<AllowedTypeWrapper>
    // Then user can call `AllowedTypeWrapper.get_string()` which returns
    // Option<String>, based on if I was able to extract the String or not
    // Version 2)
    // Return for both gets monomorphisized into Vec<AllowedTypeWrapper>

    // will be like [ (Name, [String, u32])   ,   (Age, [u32]) ]
    // let mut var_attr_pairs: Vec<(Ident, Vec<String>)> = vec![];
    let mut vars: Vec<Ident> = vec![];
    let mut allowedtypes_lists: Vec<Vec<AllowedType>> = vec![];

    match &ast.data {
        Data::Enum(data_enum) => {
            // enum variants on the user's Enum
            for variant in data_enum.variants.iter() {
                println!("-------------------------------------");
                let variant_name = &variant.ident;
                println!("- START VARIANT: {} -", variant_name);

                vars.push(variant_name.clone());
                let mut this_vars_allowedtypes: Vec<AllowedType> = vec![];

                for attr in &variant.attrs {

                    println!("--- START ATTR ---");
                    let attr_name = attr.path();

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
                                this_vars_allowedtypes.push(allowed);
                                // let astr = format!("{:#?}", allowed);
                                Ok(())
                            });
                        },
                        Some(ident) if ident.eq("any_of") => {
                            let pres = attr.parse_nested_meta(|meta| {
                                let allowed: AllowedType = meta_to_allowedtype(meta)?;
                                this_vars_allowedtypes.push(allowed);
                                // let astr = format!("{:#?}", allowed);
                                Ok(())
                            });
                        },
                        _ => {
                            // Some(f) and None and _ don't cause LSP hints on panic
                            todo!()
                            // panic!("All enum variants must be given an attribute that identifies the type they expect");
                        }
                    };
                };
                allowedtypes_lists.push(this_vars_allowedtypes);
            }
        },
        Data::Struct(_data_struct) => {
            println!("is struct");
        },
        Data::Union(_data_union) => {
            println!("is union");
        },
    };

    println!("ast done");

    // Note to self: ALWAYS use full path for -EVERYTHING- in interpolated tokenstream

    // HERE 08/05
    // I have the way below to construct/iterate through each variant & its attributes
    // Now, figure out how to use them to read from Config and return correct value

    // Holds each match arm as token stream
    let mut match_arms: Vec<quote::__private::TokenStream> = Vec::new();

    for (var, at_list) in vars.iter().zip(allowedtypes_lists.iter()) {

        // I also need to know if it's any_of or must_be, right? or is that handled
        // so right now, if `must_be` gets more than 1 type it errors, and
        // they return vec<ATW> either way, so I dont need that info here

        let at_actions: Vec<quote::__private::TokenStream> = at_list.iter()
            .map(|allowed_type| {
                let at_str = format!("{:#?}", allowed_type);
                let at_ident = Ident::new(
                    &format!("{:?}", allowed_type),
                    Span::call_site()
                );

                quote! {
                    // only push if some (if parsing was successful)
                    if let Some(at) = self.parse_allowed_type(
                        stringify!(#var),
                        quickfig_core::AllowedType::#at_ident
                    ) {
                        at_wrappers.push(at);
                    }
                }
            })
            .collect();

        // ---------------------------------------
        // whole match arm -----------------------


        let arm = quote! {
            #name::#var => {
                println!("variant is {}", stringify!(#var));

                // Should error or differentiate between non-existent key
                // and unable to parse key into given types

                if !self.has_key(stringify!(#var)) {
                    // Should this be an error?
                    println!("key not found: {}", stringify!(#var));
                } else {
                    // Config file has the key associated with this variant's name
                    println!("key found: {}", stringify!(#var));

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
        };

        match_arms.push(arm);
    }

    let impl_gen = quote! {

        impl ConfigFields for #name {
            fn hello_macro() {
                println!("Hello, Macro! my name is {}!", stringify!(#name));
                println!();
            }
        }

        // Move this to library and import it, then make .get public, and make
        // the methods on Config itself private
        // nvm orphan rules
        trait QuickFigReservedTraitName {
            type CF: quickfig_core::ConfigFields;
            fn get(&self, their_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig_core::ATW>>;
        }

        impl QuickFigReservedTraitName for quickfig_core::Config<quickfig_core::config_types::JSON> {
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

        // impl QuickFigReservedTraitName for quickfig_core::Config<quickfig_core::config_types::TOML> {
        //     type CF = #name;
        //
        //     fn get(&self, their_enum: Self::CF) -> std::option::Option<std::vec::Vec<quickfig_core::AT>> {
        //
        //         match their_enum {
        //             #(#name::#var_idents => {
        //                 println!("variant is {}", stringify!(#var_idents));
        //             })*
        //         }
        //         // todo!()
        //         return None;
        //     }
        // }
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
