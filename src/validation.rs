/// Image validation operations

/// Validate image size from base64 string
/// 
/// Estimates the binary size from base64 encoding.
/// Base64 is approximately 33% larger than binary data.
pub fn validate_image_size(base64: &str, max_size_mb: f64) -> bool {
    // Estimate size: base64 is ~33% larger than binary
    let estimated_size = (base64.len() as f64 * 3.0) / 4.0;
    let max_size_bytes = max_size_mb * 1024.0 * 1024.0;
    estimated_size <= max_size_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_image_size() {
        // Create a base64 string that represents ~1MB
        let large_base64 = "A".repeat(1_400_000); // ~1.05MB when decoded
        assert!(!validate_image_size(&large_base64, 1.0)); // Should fail for 1MB limit
        assert!(validate_image_size(&large_base64, 2.0)); // Should pass for 2MB limit
    }

    #[test]
    fn test_validate_small_image() {
        let small_base64 = "iVBORw0KGgo="; // Small PNG
        assert!(validate_image_size(small_base64, 1.0)); // Should pass
    }
}
