pub mod app_data;
pub mod apps;
pub mod embed;
pub mod tera_lib;

use crate::app_data::AppData;
pub fn run() -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(move || {
        // Create Tera
        let terax = embed::load_templates().unwrap();

        // Create App
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(AppData {
                tmpl: terax.clone(),
            }))
            .service(apps::service_api::hello)
            .service(apps::service_api::echo)
            .service(apps::service_auth::page_login)
            .service(apps::service_landing::page_landing)
            .service(apps::service_laporan::page_laporan)
            .service(apps::service_thread::page_thread)
            .route(
                "/hey",
                actix_web::web::get().to(apps::service_api::manual_hello),
            )
            .default_service(actix_web::web::to(apps::page_404))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    Ok(server)
}
