extern crate proc_macro;
mod derive;
mod opts;

use crate::derive::derive;
use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// The derive macro for `repo`.
#[proc_macro_error]
#[proc_macro_derive(
    Repo,
    attributes(repository, diesel, repo_type, crud_repo, paging_repo,)
)]
pub fn repository_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
