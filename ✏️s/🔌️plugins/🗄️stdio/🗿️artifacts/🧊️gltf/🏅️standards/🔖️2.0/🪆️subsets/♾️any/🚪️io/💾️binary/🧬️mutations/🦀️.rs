//! 💾️ Generic binary framing for the visible glTF mutation aggregate.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::schema::mutations::GltfMutation;

const BINARY_MARKER: u8 = 0x47;
const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

fn malformed(offset: u64, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "GLTF mutation aggregate", offset, detail: detail.into() }
}

impl protocol::OpBinary for GltfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
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
        let text = std::str::from_utf8(payload).map_err(|error| malformed(2, error.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(2, error.to_string()))
    }
}

mod node_name_protobuf {
use crate::standards::v2_0::subsets::any::schema::mutations::change_node_name::*;
use crate::standards::v2_0::subsets::any::io::text::mutations::{FacadeResult, GltfChangeNodeNameFacadeError, facade_error};
struct FacadeProtobufReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> FacadeProtobufReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn done(&self) -> bool {
        self.position == self.bytes.len()
    }

    fn varint(&mut self, path: &str) -> FacadeResult<u64> {
        let start = self.position;
        let mut value = 0_u64;
        for shift in 0..10 {
            let byte = match self.bytes.get(self.position) {
                Some(byte) => *byte,
                None => return facade_error("truncated", path),
            };
            self.position += 1;
            if shift == 9 && byte > 1 {
                return facade_error("varint", path);
            }
            value |= u64::from(byte & 0x7f) << (shift * 7);
            if byte & 0x80 == 0 {
                let width = if value == 0 { 1 } else { (64 - value.leading_zeros()).div_ceil(7) as usize };
                if self.position - start != width {
                    return facade_error("nonminimal", path);
                }
                return Ok(value);
            }
        }
        facade_error("varint", path)
    }

    fn key(&mut self, path: &str) -> FacadeResult<(u32, u8)> {
        let key = self.varint(path)?;
        let field = u32::try_from(key >> 3).map_err(|_| GltfChangeNodeNameFacadeError { code: "field", path: path.to_string() })?;
        if field == 0 {
            return facade_error("field", path);
        }
        Ok((field, (key & 7) as u8))
    }

    fn bytes(&mut self, path: &str) -> FacadeResult<&'a [u8]> {
        let length = self.varint(path)?;
        let length = usize::try_from(length).map_err(|_| GltfChangeNodeNameFacadeError { code: "length", path: path.to_string() })?;
        let end = self.position.checked_add(length).ok_or_else(|| GltfChangeNodeNameFacadeError { code: "length", path: path.to_string() })?;
        let bytes = match self.bytes.get(self.position..end) {
            Some(bytes) => bytes,
            None => return facade_error("truncated", path),
        };
        self.position = end;
        Ok(bytes)
    }

    fn message(&mut self, path: &str) -> FacadeResult<FacadeProtobufReader<'a>> {
        Ok(FacadeProtobufReader::new(self.bytes(path)?))
    }
}

fn protobuf_optional(reader: &mut FacadeProtobufReader<'_>, path: &str) -> FacadeResult<Option<String>> {
    let (field, wire) = reader.key(path)?;
    let value = match (field, wire) {
        (1, 2) => match String::from_utf8(reader.bytes(path)?.to_vec()) {
            Ok(value) => Some(value),
            Err(_) => return facade_error("utf8", path),
        },
        (2, 2) => {
            if !reader.message(path)?.done() {
                return facade_error("absent", path);
            }
            None
        }
        (1 | 2, _) => return facade_error("wire", path),
        _ => return facade_error("unknown", path),
    };
    if !reader.done() {
        return facade_error("duplicate", path);
    }
    Ok(value)
}

fn protobuf_apply(reader: &mut FacadeProtobufReader<'_>, path: &str) -> FacadeResult<ChangeNodeNameMutation> {
    let mut node = None;
    let mut value = None;
    while !reader.done() {
        let (field, wire) = reader.key(path)?;
        match field {
            1 if wire == 0 && node.is_none() => node = Some(u32::try_from(reader.varint(path)?).map_err(|_| GltfChangeNodeNameFacadeError { code: "node", path: path.to_string() })?),
            2 if wire == 2 && value.is_none() => {
                let mut nullable = reader.message(path)?;
                value = Some(protobuf_optional(&mut nullable, path)?);
            }
            1 | 2 if matches!(wire, 0 | 2) => return facade_error("duplicate", path),
            1 | 2 => return facade_error("wire", path),
            _ => return facade_error("unknown", path),
        }
    }
    match (node, value) {
        (Some(node), Some(value)) => Ok(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node, value })),
        (None, _) => facade_error("node", path),
        (_, None) => facade_error("nullable", path),
    }
}

pub fn decode_gltf_change_node_name_protobuf(bytes: &[u8]) -> FacadeResult<ChangeNodeNameMutation> {
    let mut reader = FacadeProtobufReader::new(bytes);
    let (field, wire) = reader.key("protobuf.phase")?;
    if field != 1 {
        return facade_error("phase", "protobuf.phase");
    }
    if wire != 2 {
        return facade_error("wire", "protobuf.phase");
    }
    let mut phase = reader.message("protobuf.phase")?;
    let mutation = protobuf_apply(&mut phase, "protobuf.apply")?;
    if !phase.done() {
        return facade_error("duplicate", "protobuf.phase");
    }
    if !reader.done() {
        return facade_error("duplicate", "protobuf.phase");
    }
    Ok(mutation)
}

}
pub use node_name_protobuf::decode_gltf_change_node_name_protobuf;
