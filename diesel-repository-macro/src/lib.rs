extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, ItemStruct};

/// The derive macro for `Repository`.
#[proc_macro_derive(Repository, attributes(repository))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    // Parse the input entity.
    let input_ast = parse_macro_input!(input as DeriveInput);
    let entity_ident = input_ast.ident;

    // Look for an attribute like #[repository(pool = "crate::db::DbPool")].
    let mut pool_path: Option<syn::Path> = None;
    for attr in input_ast.attrs.iter() {
        if attr.path.is_ident("repository") {
            let meta = attr.parse_meta().expect("Failed to parse repository meta");
            if let syn::Meta::NameValue(nv) = meta {
                if nv.path.is_ident("pool") {
                    if let syn::Lit::Str(lit_str) = nv.lit {
                        pool_path = Some(
                            syn::parse_str(&lit_str.value()).expect("Invalid pool path provided"),
                        );
                    }
                }
            } else if let syn::Meta::List(nv) = meta {
                for x in nv.nested {
                    match x {
                        syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                            if nv.path.is_ident("pool") {
                                if let syn::Lit::Str(lit_str) = nv.lit {
                                    pool_path = Some(
                                        syn::parse_str(&lit_str.value())
                                            .expect("Invalid pool path provided"),
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    let pool_path = pool_path.expect("Missing #[repository(pool = \"...\")] attribute");

    // Create a repository struct name, e.g. AppleRepository.
    let repo_ident = Ident::new(&format!("{}Repository", entity_ident), entity_ident.span());

    let expanded = quote! {
        pub struct #repo_ident {
            pub pool: std::sync::Arc<#pool_path>,
        }

        impl #repo_ident {
            pub fn new(pool: std::sync::Arc<#pool_path>) -> Self {
                Self { pool }
            }
        }
    };

    TokenStream::from(expanded)
}

/// The attribute macro for `crud_repo`.
#[proc_macro_attribute]
pub fn crud_repo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(item as ItemStruct);
    let entity_ident = &input_ast.ident;
    let repo_ident = Ident::new(&format!("{}Repository", entity_ident), entity_ident.span());

    let expanded = quote! {
        #input_ast

        impl diesel_repository::FindAllRepo<#entity_ident> for #repo_ident {
            fn find_all(&self) -> Result<Vec<#entity_ident>, diesel::result::Error> {
                unimplemented!("Sync find_all not implemented")
            }
        }

        impl diesel_repository::FindOneRepo<#entity_ident> for #repo_ident {
            fn find_one(&self, id: i32) -> Result<Option<#entity_ident>, diesel::result::Error> {
                unimplemented!("Sync find_one not implemented")
            }
        }
    };

    TokenStream::from(expanded)
}

/// The attribute macro for `paging_repo`.
#[proc_macro_attribute]
pub fn paging_repo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(item as ItemStruct);
    let entity_ident = &input_ast.ident;
    let repo_ident = Ident::new(&format!("{}Repository", entity_ident), entity_ident.span());

    let expanded = quote! {
        #input_ast

        impl diesel_repository::FindAllPagingRepo<#entity_ident> for #repo_ident {
            fn find_all_paging(&self, page: i64, per_page: i64) -> Result<Vec<#entity_ident>, diesel::result::Error> {
                unimplemented!("Sync paging find_all not implemented")
            }
        }
    };

    TokenStream::from(expanded)
}
