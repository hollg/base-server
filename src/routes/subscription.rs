use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

/// The form data submitted to add a subscriber to the DB
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// Adds a new subscription from form data
#[tracing::instrument(
    name = "Adding a new subscription", skip(form, pool),
    fields(
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscription(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    insert_subscriber(&pool, &form)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

/// Generates an ID and adds `FormData` to the DB
#[tracing::instrument(name = "Saving new user details in the database", skip(form, pool))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO subscription (id, email, name) VALUES ($1, $2, $3)",
        Uuid::new_v4(),
        form.email,
        form.name
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
