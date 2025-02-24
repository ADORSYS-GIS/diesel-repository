use darling::{FromDeriveInput, FromMeta};
use syn::LitStr;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(repository, repo_type, crud_repo, paging_repo),
    supports(struct_named)
)]
pub struct RepoOpts {
    pub ident: syn::Ident,

    #[darling(default)]
    pub repo_type: RepoTypeOpts,

    #[darling(default)]
    pub repository: RepositoryOpts,

    #[darling(default)]
    pub crud_repo: Option<CrudRepoOpts>,

    #[darling(default)]
    pub paging_repo: Option<PagingRepoOpts>,

    #[darling(default)]
    pub batch_repo: Option<BatchRepoOpts>,
}

#[derive(Debug, Default, FromMeta)]
pub struct RepoTypeOpts {
    #[darling(default)]
    pub id_type: Option<syn::Ident>,
    
    #[darling(default)]
    pub new_type: Option<syn::Ident>,
    
    #[darling(default)]
    pub update_type: Option<syn::Ident>,
}

#[derive(Debug, Default, FromMeta)]
pub struct RepositoryOpts {
    /// Expecting something like: #[repository(pool = "db::DbPool")]
    pub pool: Option<syn::Path>,
    pub table_name: Option<syn::Path>,
}

#[derive(Debug, Default, FromMeta)]
pub struct CrudRepoOpts {
    /// Expecting a list of method names, e.g.
    /// #[crud_repo(find_all, find_one, insert, update, delete)]
    #[darling(default)]
    pub methods: Vec<LitStr>,
}

#[derive(Debug, Default, FromMeta)]
pub struct PagingRepoOpts {
    /// Expecting a list of paging methods, e.g.
    /// #[paging_repo(find_all)]
    #[darling(default)]
    pub methods: Vec<LitStr>,
}

#[derive(Debug, Default, FromMeta)]
pub struct BatchRepoOpts {
    /// Expecting a list of paging methods, e.g.
    /// #[paging_repo(find_all)]
    #[darling(default)]
    pub methods: Vec<LitStr>,
}
