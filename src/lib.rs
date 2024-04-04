pub mod apps;

pub fn run() -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(apps::service_api::hello)
            .service(apps::service_api::echo)
            .route(
                "/hey",
                actix_web::web::get().to(apps::service_api::manual_hello),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    Ok(server)
}
