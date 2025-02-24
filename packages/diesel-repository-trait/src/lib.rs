mod model;
mod repo;

pub use model::{ViewCount, Paged};

#[cfg(not(feature = "async"))]
pub use repo::synchronous::*;

#[cfg(feature = "async")]
pub use repo::asynchronous::*;
