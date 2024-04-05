pub mod service_api;
pub mod service_auth;
pub mod service_landing;
pub mod service_laporan;
pub mod service_thread;

pub async fn page_404() -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().body("404: Halaman tidak ditemukan")
}
