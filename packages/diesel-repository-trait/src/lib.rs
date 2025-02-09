mod model;
mod repo;

pub use model::{Count, Paged};

#[cfg(not(feature = "async"))]
pub use repo::synchronous::*;

#[cfg(feature = "async")]
pub use repo::asynchronous::*;
