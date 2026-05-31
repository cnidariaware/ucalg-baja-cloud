use utoipa::OpenApi;

pub mod merch_shop;
pub mod database;
pub mod sponsors;
pub mod utils;

// move below at some point
// this is tech debt i dont want to feal with

#[derive(OpenApi)]
#[openapi(paths(
    darkicewolf50_actix_setup::swagger_docs::health_check_swagger,
    crate::merch_shop::merch_swagger::_get_merch,
    crate::merch_shop::merch_swagger::_recieve_order
))]
pub struct ApiDoc;


