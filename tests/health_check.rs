//! tests/health_check.rs
use std::net::TcpListener;
use z2p::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1")?;
    //get port assigned by OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    format!("https://127.0.0.1:{}", port);
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
