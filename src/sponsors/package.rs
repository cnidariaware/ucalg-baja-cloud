use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder, get};
use darkicewolf50_actix_setup::log_incoming_proxy;

#[get("/sponsor-package")]
pub async fn get_sponsor_package(req: HttpRequest) -> impl Responder {
    log_incoming_proxy("GET", "/sponsor-package", &req);
    let package_path = PathBuf::from(
        #[cfg(debug_assertions)]
        "./Database/UCalgary Baja Sponsorship Package (3)_compressed.pdf",
        #[cfg(not(debug_assertions))]
        "/sponsors/sponsor-package.pdf",
    );
    match NamedFile::open(package_path) {
        Ok(package) => package
            .use_last_modified(true)
            .prefer_utf8(true)
            .into_response(&req),
        Err(e) => {
            println!("Failed to open sponsorship package\nError: {e}");
            HttpResponse::NotFound().body("Sponsor pakcage not found")
        }
    }
}
