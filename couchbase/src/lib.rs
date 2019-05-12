#![doc(html_root_url = "https://docs.rs/couchbase/1.0.0-alpha.2")]

mod bucket;
mod cluster;
mod collection;
mod instance;
mod util;

pub mod error;
pub mod options;
pub mod result;
pub mod subdoc;

pub use crate::bucket::Bucket;
pub use crate::cluster::Cluster;
pub use crate::collection::Collection;
pub use crate::error::CouchbaseError;
