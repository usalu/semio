//! ⚖️ Block 3D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::block3d::op::Block3dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block3dOperation` to its binary command form.
pub fn encode_op(operation: &Block3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dOperation, protocol::ProtocolError> {
    Block3dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block3d::{Block3dDefinition, BLOCK_3D_SCHEMA};
    use crate::BlockKindIdentity;
    use store::{create_document_envelope, DocumentCommand};

    #[test]
    fn block3d_document_vcs_replays_granular_operations() {
        use crate::artifacts::block3d::op::Block3dStore;

        let mut store = Block3dStore::new(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", Block3dDefinition::default(), None));
        store
            .dispatch(DocumentCommand::Apply { operations: vec![Block3dOperation::SetObjectKind { object_kind: BlockKindIdentity { id: "o1".into(), name: "o1".into(), label: "O1".into(), ..Default::default() } }], description: None })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.object_kind.id, "o1");
    }

    #[test]
    fn block3d_operation_binary_round_trips() {
        let operation = Block3dOperation::RemoveVortex { id: "v0".into() };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
