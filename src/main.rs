use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use ucalg_baja_cloud::ApiDoc;
use ucalg_baja_cloud::merch_shop;
use ucalg_baja_cloud::sponsors::get_sponsors;
use ucalg_baja_cloud::utils::database::{self, SpreadSheet};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    {
        println!("Local Debug mode active!");
        println!("Running at http://localhost:6526");
    }

    #[cfg(not(debug_assertions))]
    {
        println!("Running on port 6526");
    }

    let database: web::Data<SpreadSheet> = web::Data::new(database::Database::new());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // Allow any origin — or use `.allowed_origin("http://localhost:8080")` to restrict
                    .allow_any_origin()
                    // Allow common methods
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    // Allow any headers
                    .allow_any_header(), // Optionally enable sending cookies, etc.
                                         //.supports_credentials()
            )
            .app_data(database.clone())
            // GET /
            .service(darkicewolf50_actix_setup::health_check_proxy_swagger)
            // GET /sponsors
            .service(get_sponsors)
            .service(
                web::scope("/shop")
                    // POST /recieve_order
                    .service(merch_shop::recieve_order)
                    // GET /merch
                    .service(merch_shop::get_merch),
            )
            // GET /swagger/
            // swagger/OpenAPI docs
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 6526))?
    .run()
    .await
}
