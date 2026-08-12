//! ⚖️ Block 2D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::block2d::schema::mutations::text::Block2dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block2dMutation` to its binary command form.
pub fn encode_op(operation: &Block2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block2dMutation, protocol::ProtocolError> {
    Block2dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::{Block2dSnapshot, BLOCK_2D_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    #[test]
    fn block2d_document_vcs_replays_granular_operations() {
        use crate::artifacts::block2d::schema::mutations::{self as m, Block2dStore};

        let mut store = Block2dStore::new(create_document_envelope(BLOCK_2D_SCHEMA, "block2d", Block2dSnapshot::default(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_node_kind("n1".into())], description: None }).expect("apply");
        let projection = store.snapshot().expect("snapshot");
        assert_eq!(projection.node_kind.name, "n1");
    }

    #[test]
    fn block2d_operation_binary_round_trips() {
        let operation = crate::artifacts::block2d::schema::mutations::delete_handle("h0".into());
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
