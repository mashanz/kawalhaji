#[actix_web::get("/laporan/1")]
pub async fn page_thread() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Laporan Thread")
}
