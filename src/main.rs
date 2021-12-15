use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to create Postgres connection pool");
    
    // PgPoolOptions::new()
    //     .connect_timeout(std::time::Duration::from_secs(2))
    //     .connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to Postgres");
        
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind address");
    run(listener, connection_pool)?.await
}
