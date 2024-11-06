use aws_sdk_s3::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;

pub struct AwsHandler {
    client: Client,
}

impl AwsHandler {
    pub async fn new(profile: &str) -> Result<Self, Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .profile_name(profile)
            .build();
        let config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(credentials_provider)
            .load()
            .await;
        let client = Client::new(&config);

        Ok(AwsHandler { client })
    }

    pub async fn list_buckets(&self) -> Result<(), Error> {
        let resp = self.client.list_buckets().send().await?;

        println!("Buckets:");
        for bucket in resp.buckets().unwrap_or_default() {
            println!("  {}", bucket.name().unwrap_or_default());
        }

        Ok(())
    }
}
