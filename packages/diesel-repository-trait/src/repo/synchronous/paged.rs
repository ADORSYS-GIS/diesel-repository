use crate::Paged;
use diesel::result::Error;

/// Trait for executing a paged query using a Diesel query builder.
pub trait FindByQueryPaged<T> {
    /// Executes the query with pagination.
    ///
    /// - `query`: A Diesel query builder instance.
    /// - `page`: The page number (1-indexed).
    /// - `per_page`: The number of records per page.
    ///
    /// Returns a [`Paged<T>`] with the items and paging metadata.
    fn find_by_query_paged<Q: diesel::QueryDsl>(
        &self,
        query: Q,
        page: i64,
        per_page: i64,
    ) -> Result<Paged<T>, Error>;
}

/// Trait for retrieving all records in a paginated form.
pub trait FindAllPaged<T> {
    /// Returns all records paginated.
    fn find_all_paged(&self, page: i64, per_page: i64) -> Result<Paged<T>, Error>;
}
