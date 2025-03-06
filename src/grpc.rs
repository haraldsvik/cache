use tonic::{Request, Response, Status};
use crate::Cache;

pub mod cache {
    tonic::include_proto!("cache");
}

use cache::{
    cache_service_server::CacheService,
    LookupRequest as GrpcLookupRequest,
    LookupResponse as GrpcLookupResponse,
};

pub struct CacheGrpcService {
    cache: Cache,
}

impl CacheGrpcService {
    pub fn new(cache: Cache) -> Self {
        Self { cache }
    }
}

#[tonic::async_trait]
impl CacheService for CacheGrpcService {
    async fn lookup(
        &self,
        request: Request<GrpcLookupRequest>,
    ) -> Result<Response<GrpcLookupResponse>, Status> {
        let request = request.into_inner();
        let mut found = std::collections::HashMap::new();
        let mut missing = Vec::new();

        // Process each key
        for key in request.keys {
            // Validate that the key is numeric
            if !key.chars().all(|c| c.is_ascii_digit()) {
                missing.push(key);
                continue;
            }

            if let Some(value) = self.cache.get(&key) {
                found.insert(key.clone(), value.clone());
            } else {
                missing.push(key);
            }
        }

        // If we have missing keys, forward them to the upstream service
        if !missing.is_empty() {
            tracing::warn!("Found {} missing keys, forwarding to upstream service", missing.len());
            // TODO: Implement upstream service call here
        }

        Ok(Response::new(GrpcLookupResponse { found, missing }))
    }
} 