use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check() {
    let address = spawn_app();
    let url = format!("{}/v1/health-check", &address);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request ");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscriber_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let url = format!("{app_address}/v1/subscriptions");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=usrsula_le_guin%40gmail.com";
    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    dbg!(&response.status());
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscriber_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let url = format!("{app_address}/v1/subscriptions");
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        let response = client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        let http_code = response.status().as_u16();

        assert_eq!(
            400, http_code,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
