//! 🧰️ Hexadecimal payload codec of the PDF 1.4 text mutation payloads.

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("Invalid hexadecimal payload".into());
    }
    text.as_bytes().as_chunks::<2>().0.iter().map(|pair| u8::from_str_radix(std::str::from_utf8(pair).map_err(|error| error.to_string())?, 16).map_err(|error| error.to_string())).collect()
}
