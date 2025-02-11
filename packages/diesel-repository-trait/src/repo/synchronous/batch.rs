use diesel::result::Error;

/// Trait for retrieving records by a batch of IDs.
pub trait FindByIdBatch<T, ID> {
    /// Given a slice of IDs, returns a vector of matching records.
    fn find_by_id_batch(&self, ids: &[ID]) -> Result<Vec<T>, Error>;
}

/// Trait for batch insertion of multiple new records.
pub trait SaveBatch<T, NewRecord> {
    /// Inserts multiple records at once.
    fn save_batch(&self, new_records: &[NewRecord]) -> Result<Vec<T>, Error>;
}

/// Trait for batch updating multiple records.
pub trait UpdateBatch<T, UpdateRecord> {
    /// Updates a batch of records.
    fn update_batch(&self, update_records: &[UpdateRecord]) -> Result<Vec<T>, Error>;
}

/// Trait for batch deletion of records by their IDs.
pub trait DeleteBatch<ID> {
    /// Deletes multiple records given a slice of IDs.
    fn delete_batch(&self, ids: &[ID]) -> Result<(), Error>;
}
