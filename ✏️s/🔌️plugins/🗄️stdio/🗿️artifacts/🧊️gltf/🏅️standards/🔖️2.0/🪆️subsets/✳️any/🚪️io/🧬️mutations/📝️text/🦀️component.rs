//! 📝️ GLTF mutation text transport is the generic descriptor envelope codec.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{validate_gltf_mutation_envelope, GltfMutation, GltfMutationEnvelope, GltfMutationPhase, GLTF_MUTATION_MAX_PAYLOAD_BYTES};

async fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

async fn decode_hex(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 || text.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES * 2 {
        return Err("GLTF mutation text payload exceeds its budget".into());
    }
    async fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .map(|pair| match (nibble(pair[0]), nibble(pair[1])) {
            (Some(high), Some(low)) => Ok((high << 4) | low),
            _ => Err("GLTF mutation payloadHex must be lowercase hexadecimal".into()),
        })
        .collect()
}

async fn text_error(detail: impl Into<String>) -> store::TextError {
    store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1))
}

async fn parse_field<'a>(field: &'a str, name: &str) -> Result<&'a str, store::TextError> {
    field.strip_prefix(name).ok_or_else(|| text_error(format!("expected {name}")))
}

impl protocol::OpText for GltfMutation {
    async fn print_op(&self) -> String {
        let envelope = self.envelope();
        let phase = match envelope.phase {
            GltfMutationPhase::Mutation => "mutation",
            GltfMutationPhase::Inverse => "inverse",
            GltfMutationPhase::Diff => unreachable!("GltfMutation cannot carry a diff envelope"),
        };
        let payload = (!envelope.payload.is_empty()).then(|| encode_hex(&envelope.payload)).unwrap_or_else(|| "-".into());
        format!("gltf-mutation commandId={} version={} phase={phase} payload={payload}", encode_hex(envelope.command_id.as_bytes()), envelope.version)
    }

    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let fields: Vec<_> = line.split_ascii_whitespace().collect();
        if fields.len() != 5 || fields[0] != "gltf-mutation" {
            return Err(text_error("expected canonical GLTF mutation envelope"));
        }
        let command_id = decode_hex(parse_field(fields[1], "commandId=")?).map_err(text_error)?;
        let command_id = String::from_utf8(command_id).map_err(|error| text_error(error.to_string()))?;
        let version = parse_field(fields[2], "version=")?.parse().map_err(|error| text_error(format!("invalid mutation version: {error}")))?;
        let phase = match parse_field(fields[3], "phase=")? {
            "mutation" => GltfMutationPhase::Mutation,
            "inverse" => GltfMutationPhase::Inverse,
            _ => return Err(text_error("unknown GLTF mutation phase")),
        };
        let payload_hex = parse_field(fields[4], "payload=")?;
        let payload = if payload_hex == "-" { Vec::new() } else { decode_hex(payload_hex).map_err(text_error)? };
        let envelope = GltfMutationEnvelope { command_id, version, phase, payload };
        validate_gltf_mutation_envelope(&envelope).map_err(|error| text_error(error.to_string()))?;
        GltfMutation::from_transport(envelope).map_err(|error| text_error(error.to_string()))
    }
}
