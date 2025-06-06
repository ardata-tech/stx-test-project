#[cfg(test)]
mod tests {
    use anyhow::Result;
    use w3io_partner_space_and_time::{SxT, SxTUser};

    #[tokio::test]
    #[ignore] // Remove this to run the test with valid credentials
    async fn test_authentication() -> Result<()> {
        // Requires valid .env file with credentials
        let user = SxTUser::load()?;
        let authenticated = user.authenticate().await?;
        assert!(authenticated.api_key_login.is_some());
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Remove this to run the test with valid credentials
    async fn test_query_execution() -> Result<()> {
        let sxt = SxT::new()?;
        let sxt = sxt.authenticate().await?;
        
        let result = sxt
            .execute_query::<serde_json::Value>("SELECT 1 as test".to_string())
            .await?;
        
        assert!(!result.is_empty());
        Ok(())
    }
}