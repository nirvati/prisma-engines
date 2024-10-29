use base64::Engine;

pub fn string_to_base64(str: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(str.as_bytes())
}
