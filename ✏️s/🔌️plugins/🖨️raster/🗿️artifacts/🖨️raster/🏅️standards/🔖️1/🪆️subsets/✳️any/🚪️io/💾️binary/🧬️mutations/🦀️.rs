//! ⚖️ Raster artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::op::RasterMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `RasterMutation` to its binary command form.
pub async fn encode_op(operation: &RasterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RasterMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<RasterMutation, protocol::ProtocolError> {
    RasterMutation::decode_op(bytes)
}

//#region 🔖️RetainedStoreInitialization

//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::io::text::mutations::{RasterMutationDsl,raster_mutation_to_dsl,raster_mutation_from_dsl};

impl protocol::OpBinary for RasterMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}

impl protocol::OpBinary for RasterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        raster_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(raster_mutation_from_dsl(RasterMutationDsl::decode_op(bytes)?))
    }
}
}
