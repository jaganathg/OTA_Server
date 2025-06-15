use crate::metadata::{KernelInfo, VersionHistory};
use crate::checksum::calculate_file_checksum;
use std::path::PathBuf;

pub struct MetadataManager {
    kernels_dir: PathBuf,
    metadata_dir: PathBuf,
}

impl MetadataManager {
    pub fn new(kernels_dir: String, metadata_dir: String) -> Self {
        Self {
            kernels_dir: PathBuf::from(kernels_dir),
            metadata_dir: PathBuf::from(metadata_dir),
        }
    }
    
    pub async fn add_kernel(
        &self,
        version: String,
        kernel_file: String,
        description: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let kernel_path = self.kernels_dir.join(&kernel_file);
        
        if !kernel_path.exists() {
            return Err(format!("Kernel file not found: {}", kernel_file).into());
        }
        
        // Calculate file size and checksum
        let metadata = tokio::fs::metadata(&kernel_path).await?;
        let file_size = metadata.len();
        let checksum = calculate_file_checksum(&kernel_path).await?;
        
        // Create kernel info
        let kernel_info = KernelInfo::new(
            version.clone(),
            kernel_file,
            file_size,
            checksum,
            description,
        );
        
        // Update latest.json
        self.update_latest(&kernel_info).await?;
        
        // Update version history
        self.update_history(&kernel_info).await?;
        
        Ok(())
    }
    
    async fn update_latest(&self, kernel_info: &KernelInfo) -> Result<(), Box<dyn std::error::Error>> {
        let latest_path = self.metadata_dir.join("latest.json");
        let json = serde_json::to_string_pretty(kernel_info)?;
        tokio::fs::write(latest_path, json).await?;
        Ok(())
    }
    
    async fn update_history(&self, kernel_info: &KernelInfo) -> Result<(), Box<dyn std::error::Error>> {
        let history_path = self.metadata_dir.join("version-history.json");
        
        let mut history = if history_path.exists() {
            let content = tokio::fs::read_to_string(&history_path).await?;
            serde_json::from_str::<VersionHistory>(&content).unwrap_or_else(|_| {
                VersionHistory {
                    versions: Vec::new(),
                    latest: kernel_info.version.clone(),
                }
            })
        } else {
            VersionHistory {
                versions: Vec::new(),
                latest: kernel_info.version.clone(),
            }
        };
        
        // Add new version or update existing
        if let Some(pos) = history.versions.iter().position(|v| v.version == kernel_info.version) {
            history.versions[pos] = kernel_info.clone();
        } else {
            history.versions.push(kernel_info.clone());
        }
        
        history.latest = kernel_info.version.clone();
        
        let json = serde_json::to_string_pretty(&history)?;
        tokio::fs::write(history_path, json).await?;
        Ok(())
    }

    pub async fn list_versions(&self) -> Result<VersionHistory, Box<dyn std::error::Error>> {
        let history_path = self.metadata_dir.join("version-history.json");
        
        if history_path.exists() {
            let content = tokio::fs::read_to_string(&history_path).await?;
            let history = serde_json::from_str::<VersionHistory>(&content)?;
            Ok(history)
        } else {
            Ok(VersionHistory {
                versions: Vec::new(),
                latest: "none".to_string(),
            })
        }
    }
}