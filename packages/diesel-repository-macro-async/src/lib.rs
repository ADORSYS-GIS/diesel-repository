extern crate proc_macro;

use proc_macro::TokenStream;

/// The derive macro for `Repo`.
#[proc_macro_derive(
    Repo,
    attributes(repository, diesel, repo_type, crud_repo, paging_repo,)
)]
pub fn repository_derive(_input: TokenStream) -> TokenStream {
    todo!("Implement Repository derive macro")
}
