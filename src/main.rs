use std::{io::Result, net::TcpListener};

use zero2prod::{configuration, startup::run};

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let port = configuration.application_port;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).expect("Error to bind the 8000 port");
    run(listener)?.await
}
