//! 📡️ Deterministic binary I/O for one glTF inference leaf.

use std::fmt;

use super::text::{self, GltfInferenceLeafEnvelope};

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

//#region 📨️Envelope
pub const GLTF_INFERENCE_LEAF_BINARY_MAGIC: [u8; 8] = [0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0c];
pub const GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH: usize = 40;
pub const GLTF_INFERENCE_LEAF_BINARY_FORMAT_MAJOR: u16 = 1;
pub const GLTF_INFERENCE_LEAF_BINARY_FORMAT_MINOR: u16 = 0;
pub const GLTF_INFERENCE_LEAF_BINARY_SCHEMA_VERSION: u32 = 1;
pub const GLTF_INFERENCE_LEAF_BINARY_FLAGS: u32 = 1;
pub const GLTF_INFERENCE_LEAF_BINARY_SCHEMA_CRC32: u32 = 0xcbd1_08c3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GltfInferenceBinaryError {
    Payload(text::GltfInferenceTextError),
    TooShort { actual: usize },
    Magic { actual: [u8; 8] },
    FormatMajor { actual: u16 },
    FormatMinor { actual: u16 },
    SchemaVersion { actual: u32 },
    Flags { actual: u32 },
    SchemaCrc32 { actual: u32 },
    PayloadLengthOverflow { declared: u64 },
    LengthMismatch { declared: u64, actual: usize },
    PayloadChecksum { declared: u32, actual: u32 },
    HeaderChecksum { declared: u32, actual: u32 },
}

impl fmt::Display for GltfInferenceBinaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payload(error) => write!(formatter, "invalid canonical leaf payload: {error}"),
            Self::TooShort { actual } => write!(formatter, "binary envelope is shorter than 40 bytes: {actual}"),
            Self::Magic { actual } => write!(formatter, "invalid binary magic: {actual:02x?}"),
            Self::FormatMajor { actual } => write!(formatter, "unsupported binary format major: {actual}"),
            Self::FormatMinor { actual } => write!(formatter, "unsupported binary format minor: {actual}"),
            Self::SchemaVersion { actual } => write!(formatter, "unsupported inference leaf schema version: {actual}"),
            Self::Flags { actual } => write!(formatter, "unsupported binary flags: {actual:#010x}"),
            Self::SchemaCrc32 { actual } => write!(formatter, "inference leaf schema CRC mismatch: {actual:08x}"),
            Self::PayloadLengthOverflow { declared } => write!(formatter, "payload length does not fit this platform: {declared}"),
            Self::LengthMismatch { declared, actual } => write!(formatter, "binary length mismatch: declared payload {declared}, total bytes {actual}"),
            Self::PayloadChecksum { declared, actual } => write!(formatter, "payload CRC mismatch: declared {declared:08x}, got {actual:08x}"),
            Self::HeaderChecksum { declared, actual } => write!(formatter, "header CRC mismatch: declared {declared:08x}, got {actual:08x}"),
        }
    }
}

impl std::error::Error for GltfInferenceBinaryError {}
impl From<text::GltfInferenceTextError> for GltfInferenceBinaryError {
    fn from(error: text::GltfInferenceTextError) -> Self {
        Self::Payload(error)
    }
}

pub fn encode_gltf_inference_leaf_binary(value: &GltfInferenceLeafEnvelope) -> Result<Vec<u8>, GltfInferenceBinaryError> {
    let payload = text::canonical_json_bytes(value)?;
    let length = u64::try_from(payload.len()).map_err(|_| GltfInferenceBinaryError::PayloadLengthOverflow { declared: u64::MAX })?;
    let mut output = Vec::with_capacity(GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH + payload.len());
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_MAGIC);
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_FORMAT_MAJOR.to_le_bytes());
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_FORMAT_MINOR.to_le_bytes());
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_SCHEMA_VERSION.to_le_bytes());
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_FLAGS.to_le_bytes());
    output.extend_from_slice(&GLTF_INFERENCE_LEAF_BINARY_SCHEMA_CRC32.to_le_bytes());
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(&text::crc32_iso_hdlc(&payload).to_le_bytes());
    output.extend_from_slice(&text::crc32_iso_hdlc(&output).to_le_bytes());
    output.extend_from_slice(&payload);
    Ok(output)
}

pub fn decode_gltf_inference_leaf_binary(input: &[u8]) -> Result<GltfInferenceLeafEnvelope, GltfInferenceBinaryError> {
    if input.len() < GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH {
        return Err(GltfInferenceBinaryError::TooShort { actual: input.len() });
    }
    let actual_magic: [u8; 8] = input[0..8].try_into().expect("fixed header slice");
    if actual_magic != GLTF_INFERENCE_LEAF_BINARY_MAGIC {
        return Err(GltfInferenceBinaryError::Magic { actual: actual_magic });
    }
    let format_major = read_u16(input, 8);
    if format_major != GLTF_INFERENCE_LEAF_BINARY_FORMAT_MAJOR {
        return Err(GltfInferenceBinaryError::FormatMajor { actual: format_major });
    }
    let format_minor = read_u16(input, 10);
    if format_minor != GLTF_INFERENCE_LEAF_BINARY_FORMAT_MINOR {
        return Err(GltfInferenceBinaryError::FormatMinor { actual: format_minor });
    }
    let schema_version = read_u32(input, 12);
    if schema_version != GLTF_INFERENCE_LEAF_BINARY_SCHEMA_VERSION {
        return Err(GltfInferenceBinaryError::SchemaVersion { actual: schema_version });
    }
    let flags = read_u32(input, 16);
    if flags != GLTF_INFERENCE_LEAF_BINARY_FLAGS {
        return Err(GltfInferenceBinaryError::Flags { actual: flags });
    }
    let schema_crc = read_u32(input, 20);
    if schema_crc != GLTF_INFERENCE_LEAF_BINARY_SCHEMA_CRC32 {
        return Err(GltfInferenceBinaryError::SchemaCrc32 { actual: schema_crc });
    }
    let declared_length = read_u64(input, 24);
    let payload_length = usize::try_from(declared_length).map_err(|_| GltfInferenceBinaryError::PayloadLengthOverflow { declared: declared_length })?;
    let expected_length = GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH.checked_add(payload_length).ok_or(GltfInferenceBinaryError::PayloadLengthOverflow { declared: declared_length })?;
    if expected_length != input.len() {
        return Err(GltfInferenceBinaryError::LengthMismatch { declared: declared_length, actual: input.len() });
    }
    let declared_header_crc = read_u32(input, 36);
    let actual_header_crc = text::crc32_iso_hdlc(&input[..36]);
    if declared_header_crc != actual_header_crc {
        return Err(GltfInferenceBinaryError::HeaderChecksum { declared: declared_header_crc, actual: actual_header_crc });
    }
    let payload = &input[GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH..];
    let declared_payload_crc = read_u32(input, 32);
    let actual_payload_crc = text::crc32_iso_hdlc(payload);
    if declared_payload_crc != actual_payload_crc {
        return Err(GltfInferenceBinaryError::PayloadChecksum { declared: declared_payload_crc, actual: actual_payload_crc });
    }
    let encoded = std::str::from_utf8(payload).map_err(|error| GltfInferenceBinaryError::Payload(text::GltfInferenceTextError::Json(error.to_string())))?;
    text::decode_gltf_inference_leaf_text(&format!(
        "schema {}\nversion 1\nlength {}\nchecksum {:08x}\n{encoded}",
        serde_json::from_slice::<GltfInferenceLeafEnvelope>(payload).map_err(|error| GltfInferenceBinaryError::Payload(text::GltfInferenceTextError::Json(error.to_string())))?.id,
        payload.len(),
        text::crc32_iso_hdlc(payload)
    ))
    .map_err(Into::into)
}

fn read_u16(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(input[offset..offset + 2].try_into().expect("fixed header slice"))
}
fn read_u32(input: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(input[offset..offset + 4].try_into().expect("fixed header slice"))
}
fn read_u64(input: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(input[offset..offset + 8].try_into().expect("fixed header slice"))
}
//#endregion 📨️Envelope

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_leaf_roundtrip() {
        let value = GltfInferenceLeafEnvelope {
            id: "s.stdio.gltf.inference.overall-size.v1".into(),
            algorithm_version: 1,
            policy_hash: "policy".into(),
            dependency_hashes: vec!["geometry:1".into()],
            cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2".into(),
            validity: "valid".into(),
            quality: "exact".into(),
            diagnostic_ids: Vec::new(),
            provenance: vec!["scene-world".into()],
            value: serde_json::json!(1.0),
        };
        let encoded = encode_gltf_inference_leaf_binary(&value).unwrap();
        assert_eq!(decode_gltf_inference_leaf_binary(&encoded).unwrap(), value);
    }
}
