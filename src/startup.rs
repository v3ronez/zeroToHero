use actix_web::{App, HttpServer, dev::Server, web};
use std::{io::Error, net::TcpListener};

use crate::routes::{health_check, subscription};

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/v1")
                .route("/health-check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscription)),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
