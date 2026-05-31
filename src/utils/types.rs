use std::sync::Arc;

// for types that have no strong correlation with one thing

pub type ArcString = Arc<str>;
pub type ArcVec<T> = Arc<[T]>;

pub type BajaResult<T> = Result<T, BajaError>;

use serde::{Deserialize, Serialize};

// type of error
// Attached as body of HttpResponse
#[derive(Debug, Serialize, Deserialize)]
pub enum BajaError {
    Success, // remove later in place for more specifc types of errors
    Error(String)
}