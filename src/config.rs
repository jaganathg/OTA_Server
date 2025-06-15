use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: Server,
    pub paths: Paths,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paths {
    pub kernels_dir: String,
    pub metadata_dir: String,
}

impl ServerConfig {
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: ServerConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub async fn ensure_directories(&self) -> Result<(), std::io::Error> {
        tokio::fs::create_dir_all(&self.paths.kernels_dir).await?;
        tokio::fs::create_dir_all(&self.paths.metadata_dir).await?;
        Ok(())
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: Server {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            paths: Paths {
                kernels_dir: "./kernels".to_string(),
                metadata_dir: "./metadata".to_string(),
            },
        }
    }
}