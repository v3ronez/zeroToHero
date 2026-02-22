use std::{io::Error, net::TcpListener};

use actix_web::{
    App, HttpResponse, HttpServer,
    dev::Server,
    web::{self, Form},
};

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

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize, Debug)]
struct SubscriptionForm {
    email: String,
    name: String,
}

async fn subscription(_form: Form<SubscriptionForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
