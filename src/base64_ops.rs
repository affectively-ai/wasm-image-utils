/// Base64 encoding/decoding operations

use base64::{Engine as _, engine::general_purpose};

/// Decode base64 string to bytes
pub fn base64_decode(input: &str) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(input)
        .unwrap_or_default()
}

/// Encode bytes to base64 string
pub fn base64_encode(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let original = b"Hello, World!";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded);
        assert_eq!(original, decoded.as_slice());
    }
}
