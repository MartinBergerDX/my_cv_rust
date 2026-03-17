#![cfg(feature = "ssr")]

use my_cv_rust::{
    configuration::{EmailClientSettings, Settings},
    email_client::EmailClient,
    http::build_app_router,
    rate_limit::IpRateLimiter,
    state::AppState,
};
use secrecy::Secret;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use wiremock::{
    matchers::{header_exists, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn contact_form_is_rate_limited_per_ip() {
    // Arrange: mock email provider
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails"))
        .and(header_exists("Authorization"))
        .respond_with(ResponseTemplate::new(200))
        // Only the first 2 requests should send an email (3rd is rate-limited).
        .expect(2)
        .mount(&mock_server)
        .await;

    // Arrange: build AppState with a strict limiter (2 requests / 60s)
    let leptos_conf = leptos::config::get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = leptos_conf.leptos_options;

    let settings = Settings {
        email_client: EmailClientSettings {
            base_url: mock_server.uri(),
            sender_email: "cv@example.com".into(),
            authorization_token: Secret::new("test-token".into()),
            timeout_milliseconds: 200,
        },
        contact_recipient_email: "me@example.com".into(),
    };

    let email_client = EmailClient::new(
        settings.email_client.base_url.clone(),
        settings.email_client.sender_email.clone(),
        settings.email_client.authorization_token.clone(),
        settings.email_client.timeout(),
    );

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        settings,
        email_client: Arc::new(email_client),
        contact_rate_limiter: Arc::new(IpRateLimiter::new(Duration::from_secs(60), 2)),
    };

    let app = build_app_router(app_state);

    // Arrange: run server on ephemeral port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    // Act: send 3 contact submissions from the same IP (127.0.0.1).
    //
    // Leptos server functions default to `input = PostUrl`, which expects
    // `application/x-www-form-urlencoded`. For a single argument named `input`,
    // the encoding is nested like: `input[name]=...&input[email]=...` (serde_qs).
    #[derive(Serialize)]
    struct Input<'a> {
        name: &'a str,
        email: &'a str,
        subject: &'a str,
        message: &'a str,
    }
    #[derive(Serialize)]
    struct Args<'a> {
        input: Input<'a>,
    }
    let body = serde_qs::to_string(&Args {
        input: Input {
            name: "Alice",
            email: "alice@example.com",
            subject: "Hi",
            message: "Hello!",
        },
    })
    .unwrap();

    let r1 = client
        .post(format!("{}/api/submit_contact", base))
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body.clone())
        .send()
        .await
        .unwrap();
    if !r1.status().is_success() {
        let status = r1.status();
        let text = r1.text().await.unwrap_or_default();
        panic!("r1 failed: status={status}, body={text}");
    }

    let r2 = client
        .post(format!("{}/api/submit_contact", base))
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body.clone())
        .send()
        .await
        .unwrap();
    assert!(r2.status().is_success());

    let r3 = client
        .post(format!("{}/api/submit_contact", base))
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body.clone())
        .send()
        .await
        .unwrap();
    assert!(r3.status().is_success());

    // Assert: 3rd response indicates rate limiting
    let json: serde_json::Value = r3.json().await.unwrap();
    assert_eq!(json["success"], false);
    assert!(json["message"]
        .as_str()
        .unwrap_or_default()
        .to_lowercase()
        .contains("too many"));
}

