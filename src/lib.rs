use wasm_bindgen::prelude::*;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod validation;

use validation::validate_image_size;

/// Validate image size from base64 string
/// 
/// # Arguments
/// * `base64` - Base64-encoded image string
/// * `max_size_mb` - Maximum size in megabytes
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
/// Base64 string without data URI prefix
#[wasm_bindgen]
pub fn extract_base64_from_data_uri(data_uri: &str) -> String {
    if let Some(index) = data_uri.find("base64,") {
        data_uri[index + 7..].to_string()
    } else {
        data_uri.to_string()
    }
}

/// Get MIME type from data URI
/// 
/// # Arguments
/// * `data_uri` - Data URI string
/// 
/// # Returns
/// MIME type string (defaults to "image/jpeg")
#[wasm_bindgen]
pub fn get_mime_type_from_data_uri(data_uri: &str) -> String {
    if let Some(start) = data_uri.find("data:") {
        if let Some(end) = data_uri[start + 5..].find(';') {
            return data_uri[start + 5..start + 5 + end].to_string();
        } else if let Some(end) = data_uri[start + 5..].find(',') {
            return data_uri[start + 5..start + 5 + end].to_string();
        }
    }
    "image/jpeg".to_string()
}

/// Calculate estimated size of base64 string in bytes
/// 
/// # Arguments
/// * `base64` - Base64-encoded string
/// 
/// # Returns
/// Estimated size in bytes
#[wasm_bindgen]
pub fn estimate_base64_size(base64: &str) -> f64 {
    // Base64 is ~33% larger than binary: (length * 3) / 4
    (base64.len() as f64 * 3.0) / 4.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_base64_from_data_uri() {
        let data_uri = "data:image/jpeg;base64,/9j/4AAQSkZJRg==";
        let result = extract_base64_from_data_uri(data_uri);
        assert_eq!(result, "/9j/4AAQSkZJRg==");
    }

    #[test]
    fn test_get_mime_type_from_data_uri() {
        let data_uri = "data:image/png;base64,iVBORw0KGgo=";
        let result = get_mime_type_from_data_uri(data_uri);
        assert_eq!(result, "image/png");
    }

    #[test]
    fn test_estimate_base64_size() {
        let base64 = "AAAA"; // 4 chars = 3 bytes
        let size = estimate_base64_size(base64);
        assert!((size - 3.0).abs() < 0.1);
    }
}
