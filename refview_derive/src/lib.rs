/* Copyright (C) 2022 Dylan Staatz - All Rights Reserved. */

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::punctuated::Pair;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Index, Token};

#[proc_macro_derive(RefView)]
pub fn derive_ref_view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let view_name_str = format!("{}Viewer", name);
    let view_name = Ident::new(&view_name_str, name.span());

    // Generate fields for RefView struct
    let struct_fields = struct_fields(&input.data, None);

    // Generate RefView by referencing fields
    let ref_fields = ref_fields(&input.data, None);

    let expanded = quote! {
        // A generated struct to be used as the associated type
        pub struct #view_name <'a> #struct_fields

        // The generated impl.
        impl<'a> refview::RefView<'a> for #name {
            type Viewer = #view_name <'a>;
            fn view(&'a self) -> Self::Viewer
            where
                Self::Viewer: 'a
            {
                #view_name #ref_fields
            }
        }
    };

    // println!("{}", expanded);

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(RefViewMut)]
pub fn derive_ref_view_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let view_name_str = format!("{}ViewerMut", name);
    let view_name = Ident::new(&view_name_str, name.span());

    // Generate fields for RefViewMut struct
    let struct_fields = struct_fields(&input.data, Some(<Token![mut]>::default()));

    // Generate RefViewMut by referencing fields
    let ref_fields = ref_fields(&input.data, Some(<Token![mut]>::default()));

    let expanded = quote! {
        // A generated struct to be used as the associated type
        pub struct #view_name <'a> #struct_fields

        // The generated impl.
        impl<'a> refview::RefViewMut<'a> for #name {
            type ViewerMut = #view_name <'a>;
            fn view_mut(&'a mut self) -> Self::ViewerMut
            where
                Self::ViewerMut: 'a
            {
                #view_name #ref_fields
            }
        }
    };

    // println!("{}", expanded);

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Generate fields for struct
fn struct_fields(data: &Data, mutability: Option<Token![mut]>) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let temp = fields.named.pairs().map(|pair| match pair {
                        Pair::Punctuated(field, comma) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            let ty = &field.ty;
                            quote_spanned! {field.span()=>
                                pub #name #colon &'a #mutability #ty #comma
                            }
                        }
                        Pair::End(field) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            let ty = &field.ty;
                            quote_spanned! {field.span()=>
                                pub #name #colon &'a #mutability #ty
                            }
                        }
                    });
                    quote! {
                        { #(#temp)* }
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let temp = fields.unnamed.pairs().map(|pair| match pair {
                        Pair::Punctuated(field, comma) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            let ty = &field.ty;
                            quote_spanned! {field.span()=>
                                pub #name #colon &'a #mutability #ty #comma
                            }
                        }
                        Pair::End(field) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            let ty = &field.ty;
                            quote_spanned! {field.span()=>
                                pub #name #colon &'a #mutability #ty
                            }
                        }
                    });
                    quote! {
                        ( #(#temp)* );
                    }
                }
                Fields::Unit => {
                    // Unit structs have no fields to view
                    // TODO: Generate error?
                    quote! {}
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate RefView by referencing fields
fn ref_fields(data: &Data, mutability: Option<Token![mut]>) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let temp = fields.named.pairs().map(|pair| match pair {
                        Pair::Punctuated(field, comma) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            quote_spanned! {field.span()=>
                                #name #colon & #mutability self.#name #comma
                            }
                        }
                        Pair::End(field) => {
                            let name = &field.ident;
                            let colon = &field.colon_token;
                            quote_spanned! {field.span()=>
                                #name #colon & #mutability self.#name
                            }
                        }
                    });
                    quote! {
                        { #(#temp)* }
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let temp = fields
                        .unnamed
                        .pairs()
                        .enumerate()
                        .map(|(i, pair)| match pair {
                            Pair::Punctuated(field, comma) => {
                                let index = Index::from(i);
                                let name = &field.ident;
                                let colon = &field.colon_token;
                                quote_spanned! {field.span()=>
                                    #name #colon & #mutability self.#index #comma
                                }
                            }
                            Pair::End(field) => {
                                let index = Index::from(i);
                                let name = &field.ident;
                                let colon = &field.colon_token;
                                quote_spanned! {field.span()=>
                                    #name #colon & #mutability self.#index
                                }
                            }
                        });
                    quote! {
                        ( #(#temp)* )
                    }
                }
                Fields::Unit => {
                    // Unit structs have no fields to view
                    // TODO: Generate error?
                    quote! {}
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
