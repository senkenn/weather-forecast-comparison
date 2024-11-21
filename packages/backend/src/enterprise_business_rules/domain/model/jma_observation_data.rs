use crate::enterprise_business_rules::domain::entity::jma_observation_data::JmaObservationData;

impl JmaObservationData {
    pub fn create_csv_data(&self) -> String {
        // TODO: implement create_csv_data method
        // let csv_file_path = "output.csv";
        // let mut wtr = csv::Writer::from_path(csv_file_path)?;
        // wtr.write_record(&["timestamp", "temperature", "humidity"])?;
        // for data in weather_data.jma_past_data {
        //     wtr.write_record(&[
        //         data.timestamp,
        //         data.temperature.to_string(),
        //         data.humidity.to_string(),
        //     ])?;
        // }
        // wtr.flush()?;

        String::from("csv_data")
    }

    pub fn upload_to_s3(&self) -> bool {
        // TODO: implement upload_to_s3 method
        println!("Uploading to S3...");
        // let region = Region::new("us-east-1");
        // let credentials =
        //     Credentials::new("your-access-key", "your-secret-key", None, None, "static");
        // let config = Config::builder()
        //     .region(region)
        //     .credentials_provider(credentials)
        //     .build();
        // let client = Client::from_conf(config);

        // let body = ByteStream::from_path(Path::new(csv_file_path)).await?;
        // client
        //     .put_object()
        //     .bucket("your-bucket-name")
        //     .key("output.csv")
        //     .body(body)
        //     .send()
        //     .await?;

        true
    }
}
