use crate::app_data::AppData;

#[actix_web::get("/")]
pub async fn page_landing(data: actix_web::web::Data<AppData>) -> impl actix_web::Responder {
    let mut ctx = tera::Context::new();
    let rendered = data.tmpl.render("landing.html", &ctx).unwrap();
    actix_web::HttpResponse::Ok().body(rendered)
}
