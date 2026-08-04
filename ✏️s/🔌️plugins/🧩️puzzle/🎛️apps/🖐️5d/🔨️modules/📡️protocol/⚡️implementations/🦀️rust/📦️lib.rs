//! ⚖️ Puzzle 5d app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use puzzle_5d_op::Puzzle5dOperation;

/// 📦️ Encodes a `Puzzle5dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle5dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle5dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle5dOperation, protocol::ProtocolError> {
    Puzzle5dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_document_vcs_replays_granular_operations() {
        use puzzle_5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE_5D_SCHEMA};
        use puzzle_5d_op::Puzzle5dStore;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", puzzle_5d_engine::empty_puzzle5d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Puzzle5dOperation::SetPart { index: 0, part: Puzzle5dPart { id: "p1".into(), part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.parts.len(), 1);
        assert_eq!(projection.parts[0].id, "p1");
    }
}
//#endregion 🧪️Tests
