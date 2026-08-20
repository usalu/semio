//! 💾️ GLTF mutation binary transport is the generic descriptor envelope codec.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");

use crate::artifacts::gltf::schema::modules::mutation_dispatch::{validate_gltf_mutation_envelope, GltfMutation, GltfMutationEnvelope, GltfMutationPhase, GLTF_MUTATION_MAX_COMMAND_ID_BYTES, GLTF_MUTATION_MAX_PAYLOAD_BYTES};

const BINARY_MARKER: u8 = 0x47;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "GLTF mutation envelope", offset, detail: detail.into() }
}

impl protocol::OpBinary for GltfMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let envelope = self.envelope();
        validate_gltf_mutation_envelope(envelope).map_err(|error| malformed(0, error.to_string()))?;
        let mut writer = dsl::ByteWriter::new().await;
        writer.write_u8(store::pack_rt::OP_BINARY_FORMAT).await;
        writer.write_u8(BINARY_MARKER).await;
        writer.write_u8(envelope.phase.binary_tag()).await;
        writer.write_varint_u64(envelope.command_id.len() as u64).await;
        writer.write_bytes(envelope.command_id.as_bytes()).await;
        writer.write_varint_u64(envelope.version as u64).await;
        writer.write_varint_u64(envelope.payload.len() as u64).await;
        writer.write_bytes(&envelope.payload).await;
        Ok(writer.into_bytes().await)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = dsl::ByteReader::new(bytes).await;
        let format = reader.read_u8().await.map_err(|error| malformed(0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed(0, format!("expected format {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let marker = reader.read_u8().await.map_err(|error| malformed(1, error.to_string()))?;
        if marker != BINARY_MARKER {
            return Err(malformed(1, "unknown envelope marker"));
        }
        let phase = GltfMutationPhase::from_binary_tag(reader.read_u8().await.map_err(|error| malformed(2, error.to_string()))?).map_err(|error| malformed(2, error.to_string()))?;
        let id_len = usize::try_from(reader.read_varint_u64().await.map_err(|error| malformed(3, error.to_string()))?).map_err(|_| malformed(3, "command id length exceeds usize"))?;
        if id_len == 0 || id_len > GLTF_MUTATION_MAX_COMMAND_ID_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("GLTF mutation command id"));
        }
        let command_id = std::str::from_utf8(reader.read_bytes(id_len).await.map_err(|error| malformed(3, error.to_string()))?).map_err(|error| malformed(3, error.to_string()))?.into();
        let version = u32::try_from(reader.read_varint_u64().await.map_err(|error| malformed(3, error.to_string()))?).map_err(|_| malformed(3, "version exceeds u32"))?;
        let payload_len = usize::try_from(reader.read_varint_u64().await.map_err(|error| malformed(3, error.to_string()))?).map_err(|_| malformed(3, "payload length exceeds usize"))?;
        if payload_len > GLTF_MUTATION_MAX_PAYLOAD_BYTES {
            return Err(protocol::ProtocolError::LimitExceeded("GLTF mutation payload"));
        }
        let payload = reader.read_bytes(payload_len).await.map_err(|error| malformed(3, error.to_string()))?.to_vec();
        if reader.remaining().await != 0 {
            return Err(malformed((bytes.len() - reader.remaining().await) as u64, "trailing bytes"));
        }
        let envelope = GltfMutationEnvelope { command_id, version, phase, payload };
        validate_gltf_mutation_envelope(&envelope).map_err(|error| malformed(0, error.to_string()))?;
        GltfMutation::from_transport(envelope).map_err(|error| malformed(0, error.to_string()))
    }
}
