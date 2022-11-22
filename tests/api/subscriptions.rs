use crate::helpers::spawn_app;

#[tokio::test]
async fn subscripe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=john%20doe&email=john_doe%40gmail.com";

    let respone = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, respone.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "john_doe@gmail.com");
    assert_eq!(saved.name, "john doe");
}

#[tokio::test]
async fn subscripe_returns_400_for_missing_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john_doe%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (body, error_message) in test_cases {
        let respone = client
            .post(&format!("{}/subscriptions", &app.address))
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

#[tokio::test]
async fn subscripe_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=john_doe%40gmail.com", "empty name"),
        ("name=john%20doe&email=", "empty email"),
        (
            "name=john%20doe&email=definitely_not_an_email",
            "invalid email",
        ),
    ];

    for (body, error_message) in test_cases {
        let respone = client
            .post(&format!("{}/subscriptions", &app.address))
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
