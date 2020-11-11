use base_server::config::get_config;
use base_server::startup::run;
use base_server::telemetry;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

/// Initialises tracing, database and web server
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application.port);

    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        // `connect_with` instead of `connect`
        .connect_with(config.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
