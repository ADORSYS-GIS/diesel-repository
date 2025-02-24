// Bring in the macros and traits:
use diesel::result::Error;
use diesel::{table, AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, Selectable};
use diesel_repository::{Count, FindAll, FindByQuery, Repo};
use std::sync::Arc;

table! {
    accounts {
        id -> Text,
        sub -> Text,
        name -> Text
    }
}

// Define a dummy DB pool type for async (usually this would be defined in your app).
pub mod db {
    // In your app, you’d likely have:
    // pub type DbPool = diesel_async::pooled_connection::bb8::Pool<
    //     diesel_async::AsyncConnectionManager<diesel_async::AsyncPgConnection>
    // >;
    //
    // For testing, we create a stub.
    #[allow(dead_code)]
    pub struct DummyPool;
    pub type DbPool = DummyPool;
}

// An entity with a derive macro.
#[derive(
    Debug, Eq, PartialEq, Queryable, Identifiable, Selectable, Insertable, AsChangeset, Repo,
)]
#[repository(pool = "db::DbPool")]
#[diesel(table_name = crate::accounts)]
#[crud_repo(find_all, find_one, insert, update, delete)]
#[paging_repo(find_all)]
#[repo_table_name("crate::accounts")]
#[id_type("String")]
pub struct Account {
    pub id: String,
    pub sub: String,
    pub name: String,
}

// For async tests we use Tokio.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a dummy pool. (Replace with your real pool creation.)
    let pool = Arc::new(dummy_pool());
    let repo = AccountAsyncRepo::new(pool);

    // The following calls will trigger the unimplemented!() (as placeholders)
    // You can later replace these with actual implementations.
    // For example:
    let _result = repo.find_all().await?;
    let _paged = repo.find_all_paging(1, 10).await?;

    println!("Async test run completed.");

    Ok(())
}

// Dummy pool creation function.
fn dummy_pool() -> db::DbPool {
    // In a real async setup, you’d initialize your diesel_async pool.
    // For testing, we simply return an instance of our dummy type.
    db::DummyPool
}

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct Miaou {
    pool: Arc<Pool<AsyncPgConnection>>,
}

pub struct Waff;

impl FindAll<Waff> for Miaou {
    async fn find_all(&self) -> Result<Vec<Waff>, Error> {
        let mut conn = self.pool.get().await?;
        let res = accounts::table.load(&mut conn).await?;
        Ok(res)
    }
}
