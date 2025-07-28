use async_trait::async_trait;

#[async_trait]
pub trait DbProvider: Send + Sync {
    async fn execute_query(
        &self,
        query: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn debug_tables(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
