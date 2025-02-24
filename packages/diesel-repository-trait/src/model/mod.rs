use diesel::sql_types::BigInt;
use diesel::QueryableByName;

/// A simple structure to hold paginated results.
pub struct Paged<T> {
    /// The records for the current page.
    pub items: Vec<T>,
    /// The total number of records matching the query.
    pub total_count: i64,
    /// The current page number (e.g. 1-indexed).
    pub page: i64,
    /// The number of records per page.
    pub per_page: i64,
}

#[derive(QueryableByName, Debug)]
pub struct ViewCount {
    #[sql_type = "BigInt"]
    pub count: i64,
}
