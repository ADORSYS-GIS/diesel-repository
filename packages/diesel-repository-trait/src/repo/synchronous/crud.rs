use diesel::result::Error;

/// Trait for retrieving a record by its identifier.
pub trait FindById<T, ID> {
    /// Returns the record for the given ID.
    fn find_by_id(&self, id: ID) -> Result<T, Error>;
}

/// Trait for retrieving a single record using a Diesel query.
/// This is useful when you expect the query to return exactly one row.
pub trait FindOneByQuery<T, Q> {
    /// Executes the query and returns one record.
    fn find_one_by_query(&self, query: Q) -> Result<T, Error>;
}

/// Trait for retrieving multiple records using a Diesel query.
pub trait FindByQuery<T, Q> {
    /// Executes the query and returns a vector of matching records.
    fn find_by_query(&self, query: Q) -> Result<Vec<T>, Error>;
}

/// Trait for retrieving all records from a table.
pub trait FindAll<T> {
    /// Returns all records.
    fn find_all(&self) -> Result<Vec<T>, Error>;
}

/// Trait for inserting a new record into the database.
pub trait Save<T, NewRecord> {
    /// Inserts a new record and returns the created record (with any generated fields).
    fn save(&self, new_record: NewRecord) -> Result<T, Error>;
}

/// Trait for updating an existing record.
pub trait Update<T, UpdateRecord> {
    /// Updates an existing record and returns the updated version.
    fn update(&self, update_record: UpdateRecord) -> Result<T, Error>;
}

/// Trait for a “replace” operation (upsert).
pub trait Replace<T, NewRecord> {
    /// Replaces a record if it exists or inserts it if not.
    fn replace(&self, new_record: NewRecord) -> Result<T, Error>;
}

/// Trait for deleting a record by its identifier.
pub trait Delete<ID> {
    /// Deletes the record with the given ID.
    fn delete(&self, id: ID) -> Result<(), Error>;
}

/// (Optional) Trait for counting records matching a query.
pub trait Count<Q> {
    /// Returns the count of records that match the provided query.
    fn count(&self, query: Q) -> Result<i64, Error>;
}
