//! ⚖️ Playbook artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for PlaybookMutation` is implemented directly in the shared `playbook` kernel
//! crate; see `🗿️artifacts/📖️playbook/🦀️component.rs` for why. This component only adds the thin
//! artifact-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.
//!
//! The app's typed `PlaybookCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/📖️playbook/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::playbook::op::PlaybookMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `PlaybookMutation` to its binary state-patch form.
pub fn encode_op(operation: &PlaybookMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `PlaybookMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<PlaybookMutation, protocol::ProtocolError> {
    PlaybookMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = PlaybookMutation::UpdatePlaybook { title: Some("Renamed".into()) };
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
