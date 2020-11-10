use actix_web::{HttpRequest, HttpResponse};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    println!("In health_check!");
    HttpResponse::Ok().finish()
}
