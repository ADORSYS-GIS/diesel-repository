use async_trait::async_trait;
use diesel::result::Error;

/// Trait for retrieving a record by its identifier.
#[async_trait]
pub trait FindById<T, ID> {
    /// Asynchronously returns the record for the given ID.
    async fn find_by_id(&self, id: ID) -> Result<T, Error>;
}

/// Trait for retrieving a single record using a Diesel query.
/// Useful when you expect the query to return exactly one row.
#[async_trait]
pub trait FindOneByQuery<T> {
    /// Executes the query and returns one record.
    async fn find_one_by_query<Q: diesel::QueryDsl>(&self, query: Q) -> Result<T, Error>;
}

/// Trait for retrieving multiple records using a Diesel query.
#[async_trait]
pub trait FindByQuery<T> {
    /// Executes the query and returns a vector of matching records.
    async fn find_by_query<Q: diesel::QueryDsl>(&self, query: Q) -> Result<Vec<T>, Error>;
}

/// Trait for retrieving all records from a table.
#[async_trait]
pub trait FindAll<T> {
    /// Asynchronously returns all records.
    async fn find_all(&self) -> Result<Vec<T>, Error>;
}

/// Trait for inserting a new record into the database.
#[async_trait]
pub trait Save<T, NewRecord> {
    /// Inserts a new record and returns the created record (including any generated fields).
    async fn save(&self, new_record: NewRecord) -> Result<T, Error>;
}

/// Trait for updating an existing record.
#[async_trait]
pub trait Update<T, UpdateRecord> {
    /// Updates an existing record and returns the updated version.
    async fn update(&self, update_record: UpdateRecord) -> Result<T, Error>;
}

/// Trait for a “replace” operation (upsert).
#[async_trait]
pub trait Replace<T, NewRecord> {
    /// Replaces a record if it exists or inserts it if it does not.
    async fn replace(&self, new_record: NewRecord) -> Result<T, Error>;
}

/// Trait for deleting a record by its identifier.
#[async_trait]
pub trait Delete<ID> {
    /// Deletes the record with the given ID.
    async fn delete(&self, id: ID) -> Result<(), Error>;
}

/// Trait for counting records matching a query.
#[async_trait]
pub trait Count {
    /// Returns the count of records that match the provided query.
    async fn count<Q: diesel::QueryDsl>(&self, query: Q) -> Result<i64, Error>;
}
