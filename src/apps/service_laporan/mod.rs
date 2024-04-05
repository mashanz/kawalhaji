use crate::app_data::AppData;

#[actix_web::get("/laporan")]
pub async fn page_laporan(data: actix_web::web::Data<AppData>) -> impl actix_web::Responder {
    let mut ctx = tera::Context::new();
    let rendered = data.tmpl.render("laporan.html", &ctx).unwrap();
    actix_web::HttpResponse::Ok().body(rendered)
}
