//! 💾️ Generic binary framing for the visible glTF mutation aggregate.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");

use crate::artifacts::gltf::schema::mutations::GltfMutation;

const BINARY_MARKER: u8 = 0x47;
const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "GLTF mutation aggregate", offset, detail: detail.into() }
}

impl protocol::OpBinary for GltfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = serde_json::to_vec(self).map_err(|error| malformed(0, error.to_string()))?;
        if payload.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("GLTF mutation payload"));
        }
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        writer.write_u8(BINARY_MARKER);
        writer.write_varint_u64(payload.len() as u64);
        writer.write_bytes(&payload);
        Ok(writer.into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = dsl::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|error| malformed(0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed(0, format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let marker = reader.read_u8().map_err(|error| malformed(1, error.to_string()))?;
        if marker != BINARY_MARKER {
            return Err(malformed(1, "unknown aggregate marker"));
        }
        let payload_len = usize::try_from(reader.read_varint_u64().map_err(|error| malformed(2, error.to_string()))?).map_err(|_| malformed(2, "payload length exceeds usize"))?;
        if payload_len > GLTF_MUTATION_MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("GLTF mutation payload"));
        }
        let payload = reader.read_bytes(payload_len).map_err(|error| malformed(2, error.to_string()))?;
        if reader.remaining() != 0 {
            return Err(malformed((bytes.len() - reader.remaining()) as u64, "trailing bytes"));
        }
        serde_json::from_slice(payload).map_err(|error| malformed(2, error.to_string()))
    }
}
