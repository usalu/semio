//! ⚖️ Block 2D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::block2d::op::Block2dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block2dOperation` to its binary command form.
pub fn encode_op(operation: &Block2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block2dOperation, protocol::ProtocolError> {
    Block2dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::{Block2dDefinition, BLOCK_2D_SCHEMA};
    use crate::core::BlockKindIdentity;
    use store::{create_document_envelope, DocumentCommand};

    #[test]
    fn block2d_document_vcs_replays_granular_operations() {
        use crate::artifacts::block2d::op::Block2dStore;

        let mut store = Block2dStore::new(create_document_envelope(BLOCK_2D_SCHEMA, "block2d", Block2dDefinition::default(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Block2dOperation::SetNodeKind { node_kind: BlockKindIdentity { id: "n1".into(), name: "n1".into(), label: "N1".into(), ..Default::default() } }], description: None }).expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.node_kind.id, "n1");
    }

    #[test]
    fn block2d_operation_binary_round_trips() {
        let operation = Block2dOperation::RemoveHandle { id: "h0".into() };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
