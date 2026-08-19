//! ⚖️ S Space index artifact — binary command protocol surface + laws (constitutional: spr).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::text::SSpaceMutation;
use protocol::OpBinary;

/// 📦️ Encodes an `SSpaceMutation` to its binary command form.
pub async fn encode_op(operation: &SSpaceMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `SSpaceMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<SSpaceMutation, protocol::ProtocolError> {
    SSpaceMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::touch_artifact;

    #[test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = touch_artifact("artifact-1".into(), 7, "user:1".into());
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
