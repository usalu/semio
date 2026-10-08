//! 💾️ Framing and direct binary registry for PngMutation.
use crate::schema::mutations::PngMutation;

pub type PngMutationPayloadEncoder = fn(&PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

pub struct Entry {
    pub tag: u8,
    pub encode: PngMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<PngMutation, protocol::ProtocolError>,
}

pub const REGISTRY: &[Entry] = &[
    crate::standards::v1_2::subsets::any::io::binary::mutations::change_gamma::CODEC,
    crate::standards::v1_2::subsets::any::io::binary::mutations::patch_pixels::CODEC,
    crate::standards::v1_2::subsets::any::io::binary::mutations::paint_native_samples::CODEC,
    crate::standards::v1_2::subsets::any::io::binary::mutations::set_gamma::CODEC,
    crate::standards::v1_2::subsets::any::io::binary::mutations::replace_samples::CODEC,
];

impl protocol::OpBinary for PngMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let (tag, payload) = REGISTRY.iter().find_map(|entry| (entry.encode)(self).map(|payload| (entry.tag, payload))).expect("every PNG mutation has a direct binary owner");
        let mut result = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        result.extend(payload?);
        Ok(result)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() < 2 || bytes[0] != store::pack_rt::OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "mutation frame", offset: 0, detail: "expected format byte and direct tag".into() });
        }
        let entry = REGISTRY.iter().find(|entry| entry.tag == bytes[1]).ok_or_else(|| protocol::ProtocolError::Malformed { what: "mutation tag", offset: 1, detail: format!("unknown tag {}", bytes[1]) })?;
        (entry.decode)(&bytes[2..])
    }
}

#[path = "🩹️patch-pixels/🦀️.rs"]
pub mod patch_pixels;

#[path = "🌗️change-gamma/🦀️.rs"]
pub mod change_gamma;

#[path = "🎨️paint-native-samples/🦀️.rs"]
pub mod paint_native_samples;

#[path = "🎚️set-gamma/🦀️.rs"]
pub mod set_gamma;

#[path = "🧩️replace-samples/🦀️.rs"]
pub mod replace_samples;
