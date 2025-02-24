use async_trait::async_trait;
use diesel::result::Error;
use crate::Paged;

/// Trait for executing a paged query using a Diesel query builder.
#[async_trait]
pub trait FindByQueryPaged<T> {
    /// Executes the query with pagination.
    ///
    /// - `query`: A Diesel query builder instance.
    /// - `page`: The page number (1-indexed).
    /// - `per_page`: The number of records per page.
    ///
    /// Returns a [`Paged<T>`] containing the items and pagination metadata.
    async fn find_by_query_paged<Q: diesel::QueryDsl>(
        &self,
        query: Q,
        page: i64,
        per_page: i64,
    ) -> Result<Paged<T>, Error>;
}

/// Trait for retrieving all records in a paginated form.
#[async_trait]
pub trait FindAllPaged<T> {
    /// Asynchronously returns all records paged.
    async fn find_all_paged(&self, page: i64, per_page: i64) -> Result<Paged<T>, Error>;
}
