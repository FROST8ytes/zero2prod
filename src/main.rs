use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("[+] Configuration\n\tApplication port: {}\n\tDatabase:\n\t\tHost: {}\n\t\tPort: {}\n\t\tUsername: {}\n\t\tPassword: {}\n\t\tDatabase name: {}\n", configuration.application_port, configuration.database.host, configuration.database.port, configuration.database.username, configuration.database.password, configuration.database.database_name);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!(
        "[*] Listening on http://{}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );

    run(listener, connection_pool)?.await
}
