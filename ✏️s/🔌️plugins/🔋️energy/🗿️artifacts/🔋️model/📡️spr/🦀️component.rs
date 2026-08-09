//! ⚖️ EnergyModel artifact — state-patch-representation wire codec + laws.

use crate::artifacts::model::op::EnergyModelMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

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
    use crate::artifacts::model::EnergyModelSnapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = EnergyModelMutation::NoMutation;
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn set_snapshot_round_trips() {
        let operation = EnergyModelMutation::SetSnapshot {
            snapshot: EnergyModelSnapshot::default(),
        };
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
//#endregion 🧪️Tests
