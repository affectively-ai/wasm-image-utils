/// Base64 encoding/decoding operations

use base64::{Engine as _, engine::general_purpose};

/// Decode base64 string to bytes
///
/// Returns `Ok(Vec<u8>)` on success, or `Err(String)` with a description
/// of the decoding failure.
pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    general_purpose::STANDARD
        .decode(trimmed)
        .map_err(|e| format!("base64 decode error: {e}"))
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
        let decoded = base64_decode(&encoded).expect("should decode");
        assert_eq!(original, decoded.as_slice());
    }

    #[test]
    fn test_base64_decode_empty() {
        let decoded = base64_decode("").expect("empty input should succeed");
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base64_decode_whitespace() {
        let decoded = base64_decode("  ").expect("whitespace-only should succeed as empty");
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base64_decode_invalid() {
        let result = base64_decode("!!!not-valid-base64!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_decode_with_padding() {
        // "AA==" decodes to a single zero byte
        let decoded = base64_decode("AA==").expect("padded input should decode");
        assert_eq!(decoded, vec![0u8]);
    }
}
