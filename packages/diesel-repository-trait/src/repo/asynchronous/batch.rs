use async_trait::async_trait;
use diesel::result::Error;

/// Trait for retrieving records by a batch of IDs.
#[async_trait]
pub trait FindByIdBatch<T, ID> {
    /// Given a slice of IDs, returns a vector of matching records.
    async fn find_by_id_batch(&self, ids: &[ID]) -> Result<Vec<T>, Error>;
}

/// Trait for batch insertion of multiple new records.
#[async_trait]
pub trait SaveBatch<T, NewRecord> {
    /// Inserts multiple records at once and returns the created records.
    async fn save_batch(&self, new_records: &[NewRecord]) -> Result<Vec<T>, Error>;
}

/// Trait for batch updating multiple records.
#[async_trait]
pub trait UpdateBatch<T, UpdateRecord> {
    /// Updates a batch of records and returns the updated records.
    async fn update_batch(&self, update_records: &[UpdateRecord]) -> Result<Vec<T>, Error>;
}

/// Trait for batch deletion of records by their IDs.
#[async_trait]
pub trait DeleteBatch<ID> {
    /// Deletes multiple records given a slice of IDs.
    async fn delete_batch(&self, ids: &[ID]) -> Result<(), Error>;
}
