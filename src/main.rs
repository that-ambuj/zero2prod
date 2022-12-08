use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind port");

    println!("Actix is running on: {}", address);

    run(listener, connection_pool)?.await?;

    Ok(())
}
