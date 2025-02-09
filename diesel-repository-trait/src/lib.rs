mod model;
mod repo;

pub use model::{Count, Paged};

use diesel::QueryableByName;
#[cfg(feature = "sync")]
pub use repo::synchronous::*;

#[cfg(feature = "async")]
pub use repo::asynchronous::*;
