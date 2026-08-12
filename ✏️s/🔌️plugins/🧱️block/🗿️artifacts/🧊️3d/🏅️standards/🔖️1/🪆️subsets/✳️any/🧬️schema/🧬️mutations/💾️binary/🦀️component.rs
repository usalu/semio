//! ⚖️ Block 3D artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::block3d::schema::mutations::text::Block3dMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `Block3dMutation` to its binary command form.
pub fn encode_op(operation: &Block3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Block3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Block3dMutation, protocol::ProtocolError> {
    Block3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block3d::{Block3dSnapshot, BLOCK_3D_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};

    #[test]
    fn block3d_document_vcs_replays_granular_operations() {
        use crate::artifacts::block3d::schema::mutations::{self as m, Block3dStore};

        let mut store = Block3dStore::new(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", Block3dSnapshot::default(), None));
        store
            .dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_object_kind("o1".into())], description: None })
            .expect("apply");
        let projection = store.snapshot().expect("snapshot");
        assert_eq!(projection.object_kind.name, "o1");
    }

    #[test]
    fn block3d_operation_binary_round_trips() {
        let operation = crate::artifacts::block3d::schema::mutations::delete_vortex("v0".into());
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }
}
//#endregion 🧪️Tests
