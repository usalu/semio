//! ⚖️ EnergyModel artifact — the binary operation surface (`spr`) and its laws. Tags are the
//! aggregate's own variant ordinals, emitted by `dsl::variants_binary` from the `dsl::DslEnum`
//! derive: there is no hand-maintained tag registry to collide on.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::text::EnergyModelMutation;
use protocol::OpBinary;

/// 📦️ Encodes an `EnergyModelMutation` to its binary state-patch form.
pub fn encode_op(operation: &EnergyModelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `EnergyModelMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<EnergyModelMutation, protocol::ProtocolError> {
    EnergyModelMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn every_kind_round_trips_through_this_codec() {
        for operation in crate::mutations::wire_probes() {
            let bytes = encode_op(&operation).expect("encode");
            assert_eq!(decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🧪️Tests
