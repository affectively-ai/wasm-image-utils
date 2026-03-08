/// Image validation operations

/// Validate image size from base64 string
///
/// Estimates the binary size from base64 encoding, accounting for padding.
/// Returns `false` if `max_size_mb` is not a positive finite number.
pub fn validate_image_size(base64: &str, max_size_mb: f64) -> bool {
    if !max_size_mb.is_finite() || max_size_mb <= 0.0 {
        return false;
    }
    let estimated_size = estimate_decoded_size(base64);
    let max_size_bytes = max_size_mb * 1024.0 * 1024.0;
    estimated_size <= max_size_bytes
}

/// Estimate the decoded byte size of a base64 string, accounting for padding.
fn estimate_decoded_size(base64: &str) -> f64 {
    let len = base64.len();
    if len == 0 {
        return 0.0;
    }
    let padding = base64.as_bytes().iter().rev().take_while(|&&b| b == b'=').count();
    ((len as f64 * 3.0) / 4.0) - padding as f64
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

    #[test]
    fn test_validate_negative_max_size() {
        assert!(!validate_image_size("AAAA", -1.0));
    }

    #[test]
    fn test_validate_zero_max_size() {
        assert!(!validate_image_size("AAAA", 0.0));
    }

    #[test]
    fn test_validate_nan_max_size() {
        assert!(!validate_image_size("AAAA", f64::NAN));
    }

    #[test]
    fn test_validate_infinity_max_size() {
        assert!(!validate_image_size("AAAA", f64::INFINITY));
    }

    #[test]
    fn test_estimate_decoded_size_with_padding() {
        // "AA==" is 4 chars with 2 padding = 1 byte decoded
        assert!((estimate_decoded_size("AA==") - 1.0).abs() < 0.01);
        // "AAA=" is 4 chars with 1 padding = 2 bytes decoded
        assert!((estimate_decoded_size("AAA=") - 2.0).abs() < 0.01);
        // "AAAA" is 4 chars with 0 padding = 3 bytes decoded
        assert!((estimate_decoded_size("AAAA") - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_estimate_decoded_size_empty() {
        assert!((estimate_decoded_size("") - 0.0).abs() < 0.01);
    }
}
