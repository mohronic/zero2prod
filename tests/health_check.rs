use std::net::TcpListener;
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscripe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();
    let body = "name=john%20doe&email=john_doe%40gmail.com";

    let respone = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, respone.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    
    assert_eq!(saved.email, "john_doe@gmail.com");
    assert_eq!(saved.name, "john doe");
}

#[actix_rt::test]
async fn subscripe_returns_400_for_missing_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john_doe%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (body, error_message) in test_cases {
        let respone = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            respone.status().as_u16(),
            "The API did not fail with 400 bad request when payload was {}.",
            error_message
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
