use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelInfo {
    pub version: String,
    pub kernel_file: String,
    pub file_size: u64,
    pub checksum: String,
    pub release_date: DateTime<Utc>,
    pub description: String,
    pub download_url: String,
}

// Client-facing structure that exactly matches what the OTA client expects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientKernelInfo {
    pub latest_version: String,
    pub kernel_file: String,
    pub file_size: u64,
    pub checksum: String,
    pub release_date: String, // Client expects string, not DateTime
    pub description: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub latest_version: String,
    pub kernel_info: KernelInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistory {
    pub versions: Vec<KernelInfo>,
    pub latest: String,
}

impl KernelInfo {
    pub fn new(
        version: String,
        kernel_file: String,
        file_size: u64,
        checksum: String,
        description: String,
    ) -> Self {
        Self {
            version: version.clone(),
            kernel_file: kernel_file.clone(),
            file_size,
            checksum,
            release_date: Utc::now(),
            description,
            download_url: format!("/kernels/{}", kernel_file),
        }
    }

    // Convert to client-facing format that exactly matches client expectations
    pub fn to_client_format(&self) -> ClientKernelInfo {
        ClientKernelInfo {
            latest_version: self.version.clone(),
            kernel_file: self.kernel_file.clone(),
            file_size: self.file_size,
            checksum: self.checksum.clone(),
            release_date: self.release_date.to_rfc3339(), // Convert DateTime to string
            description: self.description.clone(),
            download_url: self.download_url.clone(),
        }
    }
}
