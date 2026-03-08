use wasm_bindgen::prelude::*;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

pub mod base64_ops;
mod validation;

use validation::validate_image_size;

/// Validate image size from base64 string
///
/// # Arguments
/// * `base64` - Base64-encoded image string
/// * `max_size_mb` - Maximum size in megabytes (must be positive and finite)
///
/// # Returns
/// `true` if image size is within limit, `false` otherwise
#[wasm_bindgen]
pub fn validate_image_size_wasm(base64: &str, max_size_mb: f64) -> bool {
    validate_image_size(base64, max_size_mb)
}

/// Extract base64 string from data URI
///
/// # Arguments
/// * `data_uri` - Data URI string (e.g., "data:image/jpeg;base64,...")
///
/// # Returns
/// Base64 string without data URI prefix, trimmed of whitespace.
/// Returns the trimmed input as-is if no "base64," marker is found.
#[wasm_bindgen]
pub fn extract_base64_from_data_uri(data_uri: &str) -> String {
    let trimmed = data_uri.trim();
    if let Some(index) = trimmed.find("base64,") {
        let start = index + 7;
        if start >= trimmed.len() {
            return String::new();
        }
        trimmed[start..].trim().to_string()
    } else {
        trimmed.to_string()
    }
}

/// Get MIME type from data URI
///
/// # Arguments
/// * `data_uri` - Data URI string
///
/// # Returns
/// MIME type string extracted from the data URI, or an empty string
/// if the data URI is malformed or does not contain a recognizable MIME type.
#[wasm_bindgen]
pub fn get_mime_type_from_data_uri(data_uri: &str) -> String {
    let trimmed = data_uri.trim();

    // Strip the "data:" prefix; bail if absent
    let after_data = match trimmed.strip_prefix("data:") {
        Some(rest) => rest,
        None => return String::new(),
    };

    // The MIME type is everything before the first ';' or ','
    let mime = if let Some(end) = after_data.find(|c| c == ';' || c == ',') {
        &after_data[..end]
    } else {
        // No delimiter found -- just "data:<something>" with no payload marker
        after_data
    };

    let mime = mime.trim();
    if mime.is_empty() {
        return String::new();
    }
    mime.to_string()
}

/// Calculate estimated decoded size of a base64 string in bytes,
/// accounting for padding characters.
///
/// # Arguments
/// * `base64` - Base64-encoded string
///
/// # Returns
/// Estimated decoded size in bytes
#[wasm_bindgen]
pub fn estimate_base64_size(base64: &str) -> f64 {
    let len = base64.len();
    if len == 0 {
        return 0.0;
    }
    let padding = base64
        .as_bytes()
        .iter()
        .rev()
        .take_while(|&&b| b == b'=')
        .count();
    ((len as f64 * 3.0) / 4.0) - padding as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- extract_base64_from_data_uri -----------------------------------------

    #[test]
    fn test_extract_base64_from_data_uri() {
        let data_uri = "data:image/jpeg;base64,/9j/4AAQSkZJRg==";
        let result = extract_base64_from_data_uri(data_uri);
        assert_eq!(result, "/9j/4AAQSkZJRg==");
    }

    #[test]
    fn test_extract_base64_trims_whitespace() {
        let data_uri = "  data:image/jpeg;base64,/9j/4AAQ==\n  ";
        let result = extract_base64_from_data_uri(data_uri);
        assert_eq!(result, "/9j/4AAQ==");
    }

    #[test]
    fn test_extract_base64_empty_payload() {
        // "base64," at the very end with nothing after it
        let data_uri = "data:image/jpeg;base64,";
        let result = extract_base64_from_data_uri(data_uri);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_base64_no_marker() {
        let result = extract_base64_from_data_uri("plain text");
        assert_eq!(result, "plain text");
    }

    // -- get_mime_type_from_data_uri ------------------------------------------

    #[test]
    fn test_get_mime_type_from_data_uri() {
        let data_uri = "data:image/png;base64,iVBORw0KGgo=";
        let result = get_mime_type_from_data_uri(data_uri);
        assert_eq!(result, "image/png");
    }

    #[test]
    fn test_get_mime_type_no_semicolon() {
        // data URI with comma but no semicolon
        let result = get_mime_type_from_data_uri("data:image/gif,rawdata");
        assert_eq!(result, "image/gif");
    }

    #[test]
    fn test_get_mime_type_no_data_prefix() {
        let result = get_mime_type_from_data_uri("garbage input");
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_mime_type_empty_mime() {
        // "data:;base64,..." has an empty MIME portion
        let result = get_mime_type_from_data_uri("data:;base64,abc");
        assert_eq!(result, "");
    }

    #[test]
    fn test_get_mime_type_trimmed() {
        let result = get_mime_type_from_data_uri("  data:image/webp;base64,abc  ");
        assert_eq!(result, "image/webp");
    }

    // -- estimate_base64_size -------------------------------------------------

    #[test]
    fn test_estimate_base64_size_no_padding() {
        let base64 = "AAAA"; // 4 chars, 0 padding = 3 bytes
        let size = estimate_base64_size(base64);
        assert!((size - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_estimate_base64_size_with_padding() {
        // "AA==" → 1 decoded byte
        assert!((estimate_base64_size("AA==") - 1.0).abs() < 0.1);
        // "AAA=" → 2 decoded bytes
        assert!((estimate_base64_size("AAA=") - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_estimate_base64_size_empty() {
        assert!((estimate_base64_size("") - 0.0).abs() < 0.01);
    }
}
