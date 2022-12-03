use actix_web::{dev::Server, HttpServer, App};
use std::net::TcpListener;

use crate::routes::subscribe;
use crate::routes::health_check;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
        .listen(listener)?
        .run();

    Ok(server)
}