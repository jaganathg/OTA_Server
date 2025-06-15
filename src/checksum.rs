use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::io::AsyncReadExt;

pub async fn calculate_file_checksum<P: AsRef<Path>>(file_path: P) -> Result<String, std::io::Error> {
    let mut file = tokio::fs::File::open(file_path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8KB chunks
    
    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

pub fn _verify_checksum(data: &[u8], expected_checksum: &str) -> bool {
    if !expected_checksum.starts_with("sha256:") {
        return false;
    }
    
    let expected = &expected_checksum[7..]; // Remove "sha256:" prefix
    let mut hasher = Sha256::new();
    hasher.update(data);
    let calculated = format!("{:x}", hasher.finalize());
    
    calculated == expected
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verify_checksum() {
        let data = b"hello world";
        let checksum = "sha256:b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert!(_verify_checksum(data, checksum));
    }
}