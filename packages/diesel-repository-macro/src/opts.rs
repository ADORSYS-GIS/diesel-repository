use darling::{FromDeriveInput, FromMeta};

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(repository, repo_type, crud_repo, paging_repo),
    supports(struct_named)
)]
pub struct RepoOpts {
    pub ident: syn::Ident,

    #[darling(default)]
    pub repository: RepositoryOpts,

    #[darling(default)]
    pub repo_type: RepoTypeOpts,

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
#[darling(default)]
pub struct RepositoryOpts {
    /// Expecting something like: #[repository(pool = "db::DbPool")]
    #[darling(default)]
    pub pool: Option<syn::Path>,

    #[darling(default)]
    pub table_name: Option<syn::Path>,
}

#[derive(Debug, Default, FromMeta)]
pub struct CrudRepoOpts {
    #[darling(default)]
    pub find_one: bool,

    #[darling(default)]
    pub find_one_query: bool,

    #[darling(default)]
    pub find_query: bool,

    #[darling(default)]
    pub find_all: bool,

    #[darling(default)]
    pub save: bool,

    #[darling(default)]
    pub update: bool,

    #[darling(default)]
    pub replace: bool,

    #[darling(default)]
    pub delete: bool,

    #[darling(default)]
    pub count: bool
}

#[derive(Debug, Default, FromMeta)]
pub struct PagingRepoOpts {
    #[darling(default)]
    pub find_query: bool,

    #[darling(default)]
    pub find_all: bool
}

#[derive(Debug, Default, FromMeta)]
pub struct BatchRepoOpts {

    #[darling(default)]
    pub find: bool,
    
    #[darling(default)]
    pub save: bool,
    
    #[darling(default)]
    pub update: bool,
    
    #[darling(default)]
    pub delete: bool
}
