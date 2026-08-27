//! 📝️ Generic text framing for the visible glTF mutation aggregate.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");

use crate::artifacts::gltf::schema::mutations::GltfMutation;

const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if value.len() % 2 != 0 || value.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES * 2 {
        return Err("GLTF mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        let payload = serde_json::to_vec(self).expect("GltfMutation serialization is infallible");
        format!("gltf-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let payload = line.strip_prefix("gltf-mutation payload=").ok_or_else(|| text_error("expected canonical GLTF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        serde_json::from_slice(&bytes).map_err(|error| text_error(error.to_string()))
    }
}
