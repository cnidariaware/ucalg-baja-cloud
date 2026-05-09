use std::sync::Arc;

// use actix_web::HttpResponse;
// use actix_web::{Responder, get, web};
// use serde::{Deserialize, Serialize};
// use serde_json::json;

use utoipa::OpenApi;

pub mod merch_shop;
pub mod sponsors;
pub mod utils;


#[derive(OpenApi)]
#[openapi(paths(
    darkicewolf50_actix_setup::swagger_docs::health_check_swagger,
    crate::merch_shop::merch_swagger::_get_merch,
    crate::merch_shop::merch_swagger::_recieve_order
))]
pub struct ApiDoc;

pub type ArcString = Arc<str>;
pub type ArcVec<T> = Arc<[T]>;
