use darling::{FromAttributes, FromDeriveInput};
use syn::DeriveInput;

#[derive(Debug, PartialEq, Eq, Clone, FromDeriveInput)]
#[darling(attributes(repository), supports(struct_named))]
pub struct RepoOpts {
    pub ident: syn::Ident,

    pub attrs: Vec<syn::Attribute>,

    /// Expecting something like: #[repository(pool = "db::DbPool")]
    #[darling(default)]
    pub pool: Option<syn::Path>,

    #[darling(default)]
    pub table_name: Option<syn::Path>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    pub repo: RepoOpts,
    pub repo_type: RepoTypeOpts,
    pub crud_repo: CrudRepoOpts,
    pub paging_repo: PagingRepoOpts,
    pub batch_repo: BatchRepoOpts,
}

impl FromDeriveInput for Options {
    fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
        let opts = RepoOpts::from_derive_input(&input)?;
        let repo_type = RepoTypeOpts::from_attributes(&input.attrs)?;
        let crud_repo = CrudRepoOpts::from_attributes(&input.attrs)?;
        let paging_repo = PagingRepoOpts::from_attributes(&input.attrs)?;
        let batch_repo = BatchRepoOpts::from_attributes(&input.attrs)?;

        Ok(Self {
            repo: opts,
            repo_type,
            crud_repo,
            paging_repo,
            batch_repo,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, FromAttributes)]
#[darling(default, attributes(repo_type))]
pub struct RepoTypeOpts {
    pub id_type: Option<syn::Ident>,

    pub new_type: Option<syn::Ident>,

    pub update_type: Option<syn::Ident>,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, FromAttributes)]
#[darling(default, attributes(crud_repo))]
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
    pub count: bool,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, FromAttributes)]
#[darling(default, attributes(paging_repo))]
pub struct PagingRepoOpts {
    #[darling(default)]
    pub find_query: bool,

    #[darling(default)]
    pub find_all: bool,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, FromAttributes)]
#[darling(default, attributes(batch_repo))]
pub struct BatchRepoOpts {
    #[darling(default)]
    pub find: bool,

    #[darling(default)]
    pub save: bool,

    #[darling(default)]
    pub update: bool,

    #[darling(default)]
    pub delete: bool,
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let input: syn::DeriveInput = syn::parse_quote! {
        #[repository(pool = db::DbPool, table_name = crate::accounts)]
        #[repo_type(id_type = String)]
        #[crud_repo(find_one, find_one_query, find_all)]
        #[paging_repo(find_all)]
        struct Account {
            id: String,
            sub: String,
            name: String,
        }
    };

    let Options {
        repo,
        repo_type,
        crud_repo,
        paging_repo,
        ..
    } = Options::from_derive_input(&input)?;

    assert_eq!(
        repo.ident,
        syn::Ident::new("Account", proc_macro2::Span::call_site())
    );
    match repo.pool {
        None => panic!("pool should not be empty"),
        Some(v) => {
            assert_eq!(
                v,
                syn::parse_quote! {
                    db::DbPool
                }
            );
        }
    }
    match repo.table_name {
        None => panic!("table should not be empty"),
        Some(v) => {
            let x: syn::Path = syn::parse_quote! {
                crate::accounts
            };
            assert_eq!(v, x);
        }
    }
    match repo_type.id_type {
        None => panic!("id_type should not be empty"),
        Some(v) => {
            let challenge: syn::Ident = syn::parse_quote! {
                String
            };
            assert_eq!(v, challenge);
        }
    }

    assert_eq!(crud_repo.find_one, true);
    assert_eq!(crud_repo.find_one_query, true);
    assert_eq!(crud_repo.find_all, true);

    assert_eq!(paging_repo.find_all, true);

    Ok(())
}
