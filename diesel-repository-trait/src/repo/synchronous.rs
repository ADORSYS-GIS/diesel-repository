use diesel::result::Error;

pub trait FindAllRepo<T> {
    fn find_all(&self) -> Result<Vec<T>, Error>;
}

pub trait FindOneRepo<T> {
    fn find_one(&self, id: i32) -> Result<Option<T>, Error>;
}

pub trait FindAllPagingRepo<T> {
    fn find_all_paging(&self, page: i64, per_page: i64) -> Result<Vec<T>, Error>;
}

// --- Tests for the traits ---
#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;
    
    struct DummyRepo;

    mod sync_tests {
        use super::*;
        
        impl FindAllRepo<u32> for DummyRepo {
            fn find_all(&self) -> Result<Vec<u32>, Error> {
                Ok(vec![1, 2, 3])
            }
        }

        #[test]
        fn test_find_all_sync() {
            let repo = DummyRepo;
            let res = repo.find_all();
            assert_eq!(res.unwrap(), vec![1, 2, 3]);
        }
    }
}
