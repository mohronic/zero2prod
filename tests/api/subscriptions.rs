use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscripe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=john%20doe&email=john_doe%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let respone = app.post_subscriptions(body.into()).await;

    assert_eq!(200, respone.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = spawn_app().await;
    let body = "name=john%20doe&email=john_doe%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let respone = app.post_subscriptions(body.into()).await;

    assert_eq!(200, respone.status().as_u16());

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "john_doe@gmail.com");
    assert_eq!(saved.name, "john doe");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john_doe%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (body, error_message) in test_cases {
        let respone = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            respone.status().as_u16(),
            "The API did not fail with 400 bad request when payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=john_doe%40gmail.com", "empty name"),
        ("name=john%20doe&email=", "empty email"),
        (
            "name=john%20doe&email=definitely_not_an_email",
            "invalid email",
        ),
    ];

    for (body, error_message) in test_cases {
        let respone = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            respone.status().as_u16(),
            "The API did not fail with 400 bad request when payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=test%20person&email=test%40email.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = "name=test%20person&email=test%40email.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn subscribe_fails_if_there_is_a_fatal_database_error() {
    let app = spawn_app().await;
    let body = "name=test%20person&email=test%40email.com";

    // sqlx::query!("ALTER TABLE subscription_tokens DROP COLUMN subscription_token;")
    //     .execute(&app.db_pool)
    //     .await
    //     .unwrap();

    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;")
        .execute(&app.db_pool)
        .await
        .unwrap();

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(response.status().as_u16(), 500);
}
