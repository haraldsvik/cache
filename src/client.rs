use tonic::transport::Channel;

pub mod cache {
    tonic::include_proto!("cache");
}

use cache::{
    cache_service_client::CacheServiceClient,
    LookupRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connecting to our gRPC server
    let mut client = CacheServiceClient::connect("http://[::1]:50051").await?;

    // Create a test request with some keys
    let request = tonic::Request::new(LookupRequest {
        keys: vec![
            "80268025748".to_string(),    // Should be found if it exists in mock_data.txt
            "999999".to_string(),   // Should be missing
            "abc123".to_string(),   // Should be skipped (non-numeric)
        ],
    });

    // Send the request and get the response
    let response = client.lookup(request).await?;
    let response = response.into_inner();

    // Print the results
    println!("Found keys:");
    for (key, value) in response.found {
        println!("  {}: {}", key, value);
    }

    println!("\nMissing keys:");
    for key in response.missing {
        println!("  {}", key);
    }

    Ok(())
} 