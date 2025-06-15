use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
}