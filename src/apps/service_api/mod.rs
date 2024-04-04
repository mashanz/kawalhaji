#[actix_web::get("/")]
pub async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}

#[actix_web::post("/echo")]
pub async fn echo(req_body: String) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hey there!")
}
