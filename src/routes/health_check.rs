use actix_web::{HttpRequest, HttpResponse};

#[tracing::instrument(name = "Checking application health", skip(_req))]
pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
