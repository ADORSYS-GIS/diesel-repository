extern crate proc_macro;

use diesel_repository_macro_core::{FromDeriveInput, Options};
use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use syn::{parse_macro_input, DeriveInput};

#[cfg(not(feature = "async"))]
use diesel_repository_macro_sync::derive;

#[cfg(feature = "async")]
use diesel_repository_macro_async::derive;

/// The derive macro for `repo`.
#[proc_macro_error]
#[proc_macro_derive(Repo, attributes(repository, repo_type, crud_repo, paging_repo))]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match Options::from_derive_input(&input) {
        Ok(r) => r,
        Err(e) => abort!(input, "Failed to parse #[derive(Repo)] attributes: {}", e),
    };

    derive(opts)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
