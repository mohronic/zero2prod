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
async fn subscripe_returns_400_for_invalid_data() {
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

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(&body["TextBody"].as_str().unwrap());

    assert_eq!(html_link, text_link);
}
