use diesel::QueryableByName;
use diesel::sql_types::BigInt;

#[derive(Debug)]
pub struct Paged<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
}

#[derive(QueryableByName, Debug)]
pub struct Count {
    #[sql_type = "BigInt"]
    pub count: i64,
}