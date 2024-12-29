use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub trait IS3Service {
    fn upload_to_s3(
        &self,
        csv_file_path: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}
