use serde::{Deserialize, Serialize};

// type of error
// Attached as body of HttpResponse
#[derive(Debug, Serialize, Deserialize)]
pub enum BajaError {
    Success,
    Error(String)
}