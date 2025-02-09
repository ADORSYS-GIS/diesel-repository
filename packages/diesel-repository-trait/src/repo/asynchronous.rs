use diesel::result::Error;

use async_trait::async_trait;

#[async_trait]
pub trait FindAllRepo<T> {
    async fn find_all(&self) -> Result<Vec<T>, Error>;
}

#[async_trait]
pub trait FindOneRepo<T> {
    async fn find_one(&self, id: i32) -> Result<Option<T>, Error>;
}

#[async_trait]
pub trait FindAllPagingRepo<T> {
    async fn find_all_paging(&self, page: i64, per_page: i64) -> Result<Vec<T>, Error>;
}

// --- Tests for the traits ---
#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;

    struct DummyRepo;

    mod async_tests {
        use super::*;
        #[async_trait::async_trait]
        impl FindAllRepo<u32> for DummyRepo {
            async fn find_all(&self) -> Result<Vec<u32>, Error> {
                Ok(vec![1, 2, 3])
            }
        }

        #[tokio::test]
        async fn test_find_all_async() {
            let repo = DummyRepo;
            let res = repo.find_all().await;
            assert_eq!(res.unwrap(), vec![1, 2, 3]);
        }
    }
}
