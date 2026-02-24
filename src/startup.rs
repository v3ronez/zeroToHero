use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self, Data},
};
use sqlx::PgConnection;
use std::{io::Error, net::TcpListener, sync::Arc};

use crate::routes::{health_check, subscription};

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, Error> {
    // Web::Data::new ->> Arc<T>
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/v1")
                    .route("/health-check", web::get().to(health_check))
                    .route("/subscriptions", web::post().to(subscription)),
            )
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
