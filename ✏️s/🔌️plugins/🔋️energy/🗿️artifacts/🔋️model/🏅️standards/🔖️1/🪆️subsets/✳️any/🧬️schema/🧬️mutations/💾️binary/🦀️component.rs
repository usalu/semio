//! ⚖️ EnergyModel artifact — state-patch-representation wire codec + laws.

use crate::artifacts::model::schema::mutations::text::EnergyModelMutation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes an `EnergyModelMutation` to its binary state-patch form.
pub async fn encode_op(operation: &EnergyModelMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `EnergyModelMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<EnergyModelMutation, protocol::ProtocolError> {
    EnergyModelMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::model::mutations::replace_model;

    #[test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: "{}".to_string() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    async fn replace_model_round_trips() {
        let operation = EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: r#"{"name":"demo"}"#.to_string() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
//#endregion 🧪️Tests
