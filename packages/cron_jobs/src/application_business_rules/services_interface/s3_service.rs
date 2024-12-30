use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait IS3Service {
    async fn upload_to_s3(&self, csv_file_path: String) -> Result<()>;
}
