//! ⚖️ Puzzle 2d app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use puzzle_2d_op::Puzzle2dOperation;

/// 📦️ Encodes a `Puzzle2dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle2dOperation, protocol::ProtocolError> {
    Puzzle2dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle2d_document_vcs_replays_granular_operations() {
        use puzzle_2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use puzzle_2d_op::Puzzle2dStore;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", puzzle_2d_engine::empty_puzzle2d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Puzzle2dOperation::SetNode {
                    index: 0,
                    node: Puzzle2dNode { id: "n1".into(), node_kind: None, shape: None, x: 0.0, y: 0.0, radius: None, width: None, height: None, text: None, icon_kind: None, root: None, scale: None, visible: None, locked: None, handles: Vec::new() },
                }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.nodes.len(), 1);
        assert_eq!(projection.nodes[0].id, "n1");
    }
}
//#endregion 🧪️Tests
