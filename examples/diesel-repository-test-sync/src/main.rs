// Bring in the macros and traits.
use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use diesel_repository::Repo;
use std::sync::Arc;

table! {
    accounts {
        id -> Text,
        sub -> Text,
        name -> Text
    }
}

// Define a dummy DB pool type for synchronous use.
pub mod db {
    // Typically, you’d define:
    // pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
    // For testing, we create a stub.
    #[allow(dead_code)]
    pub struct DummyPool;
    pub type DbPool = DummyPool;
}

// Define an entity with a derive macro.
#[derive(
    Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Insertable, AsChangeset, Repo,
)]
#[diesel(table_name = crate::accounts)]
#[repository(pool = "db::DbPool", table_name = "crate::accounts")]
#[repo_type(id_type = String)]
#[crud_repo(find, find_query)]
#[paging_repo(find_all)]
pub struct Account {
    pub id: String,
    pub sub: String,
    pub name: String,
}

fn main() -> anyhow::Result<()> {
    // Create a dummy pool (replace with your actual pool creation logic).
    let pool = Arc::new(dummy_pool());
    let repo = AccountRepo::new(pool);
    println!("Sync test run completed.");

    let _result: Vec<Account> = repo.find_all()?;
    let _paged = repo.find_all_paging(1, 10)?;

    Ok(())
}

// Dummy pool creation function.
fn dummy_pool() -> db::DbPool {
    // In a real setup, you would create and configure your diesel pool.
    // For testing, simply return an instance of our dummy pool.
    db::DummyPool
}
