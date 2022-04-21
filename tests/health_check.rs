#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::tests]
    async fn health_check_succeeds(){
        let response  = health_check().await;

        assert!(response.status().is_ok())
    }
}