//! ⚖️ Block 5D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::block5d::schema::mutations::text::Block5dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block5dMutation` to its binary command form.
pub fn encode_op(operation: &Block5dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block5dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block5dMutation, protocol::ProtocolError> {
    Block5dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block5d::{Block5dSnapshot, BLOCK_5D_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    #[semio_framework_async_macros::async_test]
    async fn block5d_document_vcs_replays_granular_operations() {
        use crate::artifacts::block5d::schema::mutations::{self as m, Block5dStore};

        let mut store = Block5dStore::new(create_document_envelope(BLOCK_5D_SCHEMA, "block5d", Block5dSnapshot::default(), None)).expect("valid initial state");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_part_kind("p1".into())], description: None }).expect("apply");
        let projection = store.snapshot().expect("snapshot");
        assert_eq!(projection.part_kind.name, "p1");
    }

    #[semio_framework_async_macros::async_test]
    async fn block5d_operation_binary_round_trips() {
        let operation = crate::artifacts::block5d::schema::mutations::delete_grip("g0".into());
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
