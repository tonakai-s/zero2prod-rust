use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod_rust::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let connection = PgPool::connect(
            &configuration.database.connection_string()
        ).await
        .expect("Failed to connect to Postgres");
    run(listener, connection)?.await
}