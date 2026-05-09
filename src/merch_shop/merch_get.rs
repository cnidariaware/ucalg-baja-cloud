use std::{fs, path::PathBuf};

use actix_web::{HttpRequest, HttpResponse, Responder, get};
use darkicewolf50_actix_setup::log_incoming_proxy;

use crate::utils::{ArcVec, merch::MerchItem};


/// Gets all of the merch items available, with all of the assciated detials,
/// like size, colour images, etc. that are displayed on the merch shop.
///
/// # Params
///
/// - req - The incoming request, inclluding the headers.
///
/// # Returns
///
/// - 200 response with merch details .
///
///  # Example
///
/// ```rust
/// use ucalg_baja_cloud::merch_shop::get_merch;
/// use actix_web::{test, App};
/// use serde_json::Value;
///
/// #[actix_web::test]
/// async fn test_get_merch() {
///     let app = test::init_service(
///         App::new().service(get_merch)
///     ).await;
///
///     let req = test::TestRequest::get()
///         .uri("/merch")
///         .to_request();
///
///     let resp: Value = test::call_and_read_body_json(&app, req).await;
///
///     // Response should always be valid JSON (an array, possibly empty)
///     assert!(resp.is_array());
/// }
/// ```
///
/// # Author (s)
///
/// - Name <brock@darkicewolf50.dev>
/// semi-permanent email, do not need to respond but try to be a good alumni
#[get("/merch")]
pub async fn get_merch(req: HttpRequest) -> impl Responder {
    log_incoming_proxy("GET", "/shop/merch", &req);
    let sponsor_get_path = PathBuf::from(
        #[cfg(debug_assertions)]
        "./Database/merch.yaml",
        #[cfg(not(debug_assertions))]
        "/Shop/merch.yaml",
    );

    let yaml = fs::read_to_string(sponsor_get_path).unwrap_or_else(|_| "".to_string());

    let yaml: ArcVec<MerchItem> = serde_saphyr::from_str(&yaml)
        .unwrap_or_else(|_| vec![])
        .into();

    HttpResponse::Ok().json(yaml)
}
