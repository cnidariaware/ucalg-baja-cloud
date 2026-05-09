use std::sync::Arc;

// for types that have no strong correlation with one thing

pub type ArcString = Arc<str>;
pub type ArcVec<T> = Arc<[T]>;