use warp::{Filter, Reply, Rejection};
use std::path::PathBuf;
use crate::metadata::{KernelInfo, VersionResponse};
use crate::config::ServerConfig;
use crate::checksum::calculate_file_checksum;

// Health check endpoint
pub fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "healthy"})))
}

// Version info endpoint
pub fn version(
    config: ServerConfig,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("version")
        .and(warp::get())
        .and(warp::any().map(move || config.clone()))
        .and_then(get_latest_version)
}

// Kernel file serving endpoint
pub fn kernels(
    config: ServerConfig,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("kernels")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_kernel_file)
}

async fn get_latest_version(config: ServerConfig) -> Result<Box<dyn Reply>, Rejection> {
    let metadata_path = PathBuf::from(&config.paths.metadata_dir).join("latest.json");
    
    match tokio::fs::read_to_string(&metadata_path).await {
        Ok(content) => {
            match serde_json::from_str::<KernelInfo>(&content) {
                Ok(kernel_info) => {
                    let response = VersionResponse {
                        latest_version: kernel_info.version.clone(),
                        kernel_info,
                    };
                    Ok(Box::new(warp::reply::json(&response)))
                }
                Err(_) => {
                    let error_response = serde_json::json!({"error": "Invalid metadata format"});
                    Ok(Box::new(warp::reply::with_status(
                        warp::reply::json(&error_response),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            }
        }
        Err(_) => {
            let error_response = serde_json::json!({"error": "No version information available"});
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::NOT_FOUND,
            )))
        }
    }
}

async fn serve_kernel_file(
    filename: String,
    config: ServerConfig,
) -> Result<Box<dyn Reply>, Rejection> {
    let file_path = PathBuf::from(&config.paths.kernels_dir).join(&filename);
    
    if !file_path.exists() {
        let error_response = serde_json::json!({"error": "File not found"});
        return Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&error_response),
            warp::http::StatusCode::NOT_FOUND,
        )));
    }
    
    // Calculate checksum for verification
    let checksum = match calculate_file_checksum(&file_path).await {
        Ok(hash) => hash,
        Err(_) => {
            let error_response = serde_json::json!({"error": "Error calculating checksum"});
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    };
    
    match tokio::fs::read(&file_path).await {
        Ok(contents) => {
            Ok(Box::new(warp::reply::with_header(
                warp::reply::with_header(
                    contents,
                    "content-type",
                    "application/octet-stream",
                ),
                "x-checksum",
                checksum,
            )))
        }
        Err(_) => {
            let error_response = serde_json::json!({"error": "Error reading file"});
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
    }
}