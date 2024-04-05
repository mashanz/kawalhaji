#[actix_web::get("/laporan")]
pub async fn page_laporan() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Page Laporan")
}
