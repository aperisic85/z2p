//! tests/health_check.rs

use z2p::main;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}

fn spawn_app() -> std::io::Result<()> {
    let server = z2p::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
