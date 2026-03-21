use actix_web::{HttpRequest, HttpResponse, Responder, get};
use darkicewolf50_actix_setup::log_incoming_proxy;
use serde::{Deserialize, Serialize};
use serde_saphyr;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct Sponsor {
    #[serde(rename = "SponsorName")]
    sponsor_name: String,
    #[serde(rename = "LogoUrl")]
    logo_url: Option<String>,
    #[serde(rename = "Url")]
    sponsor_site_url: Option<String>,
    #[serde(rename = "DescriptionAboutSponsor")]
    description_about_sponsor: Option<String>,
}

/// Gets the current sponsors with all of thier details, like sponsor website, tier, logo, etc. where it is displayed on the website
///
/// # Params
///
/// - req - The incoming request, inclluding the headers.
///
/// # Returns
///
/// - 200 response with the sponsor details.
///
/// # Example
///
/// ```rust
/// use ucalg_baja_cloud::sponsors::get_sponsors;
/// use actix_web::{test, App};
/// use serde_json::Value;
///
/// #[actix_web::test]
/// async fn test_get_sponsors() {
///     let app = test::init_service(
///         App::new().service(get_sponsors)
///     ).await;
///
///     let req = test::TestRequest::get()
///         .uri("/sponsors")
///         .to_request();
///
///     let resp: Value = test::call_and_read_body_json(&app, req).await;
///
///     // We don't assert exact contents because the YAML file may vary,
///     // but we do guarantee a JSON object is returned.
///     assert!(resp.is_object());
/// }
/// ```
///
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>
/// semi-permanent email, do not need to respond but try to be a good alumni
#[get("/sponsors")]
pub async fn get_sponsors(req: HttpRequest) -> impl Responder {
    log_incoming_proxy("GET", "/sponsors", &req);
    let sponsor_database_path = PathBuf::from(
        #[cfg(debug_assertions)]
        "./Database/sponsorship.yaml",
        #[cfg(not(debug_assertions))]
        "/sponsors/sponsorship.yaml",
    );

    let yaml = fs::read_to_string(sponsor_database_path).unwrap_or_else(|_| "".to_string());

    let yaml: HashMap<String, Vec<Sponsor>> =
        serde_saphyr::from_str(&yaml).unwrap_or_else(|_| HashMap::from([("".to_string(), vec![])]));

    HttpResponse::Ok().json(yaml)
}
