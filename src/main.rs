use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(&address).expect("Failed to bind port");

    println!("Actix is running on: {}", address);

    run(listener)?.await?;

    Ok(())
}
