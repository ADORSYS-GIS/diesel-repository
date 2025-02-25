use crate::opts::RepoOpts;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use proc_macro_error2::abort;
use quote::quote;
use syn::DeriveInput;

pub fn derive(input: &DeriveInput) -> syn::Result<TokenStream> {
    let opts = RepoOpts::from_derive_input(input)?;
    let struct_name = opts.ident;
    let id_type = opts
        .repo_type
        .id_type
        .unwrap_or_else(|| abort!(struct_name, "Missing 'id_type' attribute in #[repo_type]"));

    let new_type = match opts.repo_type.new_type {
        None => struct_name.clone(),
        Some(v) => v,
    };

    let update_type = match opts.repo_type.update_type {
        None => struct_name.clone(),
        Some(v) => v,
    };

    // Generate repository type name: e.g. `AccountRepo` for struct `Account`
    let repo_name = syn::Ident::new(&format!("{}Repo", struct_name), struct_name.span());

    // Ensure the repository attribute provided a pool type.
    let pool_type = match opts.repository.pool {
        Some(path) => path,
        None => abort!(struct_name, "Missing 'pool' attribute in #[repository]"),
    };

    let diesel_table = match opts.repository.table_name {
        Some(path) => path,
        None => abort!(
            struct_name,
            "Missing 'table_name' attribute in #[repository]"
        ),
    };

    let mut crud_methods = TokenStream::new();
    if let Some(crud) = opts.crud_repo {
        if crud.find_one {
            let m = quote! {
                impl diesel_repository::FindById<#struct_name, #id_type> for #repo_name {
                    pub fn find_by_id(&self) -> Result<#struct_name, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_by_id] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.find_one_query {
            let m = quote! {
                impl diesel_repository::FindOneByQuery<#struct_name> for #repo_name {
                    pub fn find_one_by_query<Q: diesel::QueryDsl>(&self, query: Q) -> Result<#struct_name, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_one_by_query] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.find_query {
            let m = quote! {
                impl diesel_repository::FindByQuery<#struct_name> for #repo_name {
                    pub fn find_by_query<Q: diesel::QueryDsl>(&self, query: Q) -> Result<Vec<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_by_query] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.find_all {
            let m = quote! {
                impl diesel_repository::FindAll<#struct_name> for #repo_name {
                    pub fn find_all(&self) -> Result<Vec<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_all] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.save {
            let m = quote! {
                impl diesel_repository::Save<#struct_name, #new_type> for #repo_name {
                    pub fn save(&self, new_record: #new_type) -> Result<#struct_name, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[save] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.update {
            let m = quote! {
                impl diesel_repository::Update<#struct_name, #update_type> for #repo_name {
                    pub fn update(&self, update_record: #update_type) -> Result<#struct_name, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[update] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.replace {
            let m = quote! {
                impl diesel_repository::Replace<#struct_name, #struct_name> for #repo_name {
                    pub fn replace(&self, update_record: #struct_name) -> Result<#struct_name, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[replace] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(m);
        }
        if crud.delete {
            let method_fn = quote! {
                impl diesel_repository::Delete<#id_type> for #repo_name {
                    pub fn delete(&self, id: #id_type) -> Result<(), diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[delete] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(method_fn);
        }
        if crud.count {
            let method_fn = quote! {
                impl diesel_repository::Count for #repo_name {
                    pub fn count<Q: diesel::QueryDsl>(&self, query: Q) -> Result<i64, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[delete] is not implemented yet")
                    }
                }
            };
            crud_methods.extend(method_fn);
        }
    }

    let mut paging_methods = TokenStream::new();
    if let Some(paging) = opts.paging_repo {
        if paging.find_query {
            let m = quote! {
                impl diesel_repository::FindByQueryPaged<#struct_name, #id_type> for #repo_name {
                    pub fn find_by_query_paged<Q: diesel::QueryDsl>(
                            &self,
                            query: Q,
                            page: i64,
                            per_page: i64) -> Result<diesel_repository::Paged<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_query] is not implemented yet")
                    }
                }
            };
            paging_methods.extend(m);
        }
        if paging.find_all {
            let m = quote! {
                impl diesel_repository::FindAllPaged<#struct_name> for #repo_name {
                    pub fn find_all_paged<Q: diesel::QueryDsl>(&self, page: i64, per_page: i64) -> Result<diesel_repository::Paged<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find_all_paged] is not implemented yet")
                    }
                }
            };
            paging_methods.extend(m);
        }
    }

    let mut batch_methods = TokenStream::new();
    if let Some(batch) = opts.batch_repo {
        if batch.find {
            let m = quote! {
                impl diesel_repository::FindByIdBatch<#struct_name, #id_type> for #repo_name {
                    pub fn find_by_id_batch(&self, ids: &[#id_type]) -> Result<Vec<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[find] is not implemented yet")
                    }
                }
            };
            batch_methods.extend(m);
        }
        if batch.save {
            let m = quote! {
                impl diesel_repository::SaveBatch<#struct_name, #new_type> for #repo_name {
                    pub fn save_batch(&self, new_records: &[new_type]) -> Result<Vec<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[save] is not implemented yet")
                    }
                }
            };
            batch_methods.extend(m);
        }
        if batch.update {
            let m = quote! {
                impl diesel_repository::UpdateBatch<#struct_name, #update_type> for #repo_name {
                    pub fn update_batch(&self, new_records: &[update_type]) -> Result<Vec<#struct_name>, diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[update] is not implemented yet")
                    }
                }
            };
            batch_methods.extend(m);
        }
        if batch.delete {
            let m = quote! {
                impl diesel_repository::DeleteBatch<#id_type> for #repo_name {
                    pub fn delete_batch(&self, new_records: &[#id_type]) -> Result<(), diesel::result::Error> {
                        use diesel::prelude::*;
                        use #diesel_table::dsl::*;

                        // Stub implementation: Replace with real query logic.
                        unimplemented!("[delete] is not implemented yet")
                    }
                }
            };
            batch_methods.extend(m);
        }
    }

    // Assemble the final repository implementation.
    let expanded = quote! {
        pub struct #repo_name {
            pool: std::sync::Arc<#pool_type>,
        }

        impl #repo_name {
            pub fn new(pool: std::sync::Arc<#pool_type>) -> Self {
                Self { pool }
            }
        }

        #crud_methods

        #paging_methods

        #batch_methods
    };

    Ok(expanded)
}
