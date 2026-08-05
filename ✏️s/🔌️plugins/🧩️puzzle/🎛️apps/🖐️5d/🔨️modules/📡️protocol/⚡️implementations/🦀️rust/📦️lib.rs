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

//#region 🧪️WireBaselineDump
#[cfg(test)]
mod wire_baseline_dump {
    use super::*;
    use protocol::OpText;
    use serde_json::json;

    fn ops() -> Vec<Puzzle5dOperation> {
        let part: puzzle_5d::Puzzle5dPart = serde_json::from_value(json!({"id":"p1","partKind":"Capsule","2d":{"x":1.0,"y":2.0,"shape":"circle","radius":3.0,"text":"t","iconKind":"i","hidden":false,"locked":false},"3d":{"origin":[1.0,2.0,3.0],"meshUrl":"/m.glb","orientation":[0.0,0.0,0.0,1.0],"scale":[2.0,3.0,4.0],"label":"L"},"grips":[{"id":"g0","gripKind":"k","2d":{"angle":0.5,"gripKind":"k","radius":3.0},"3d":{"position":[0.0,0.0,0.0],"direction":[0.0,0.0,1.0],"radius":3.0,"label":"g"}}]})).unwrap();
        let fastener: puzzle_5d::Puzzle5dFastener = serde_json::from_value(json!({"id":"f1","source":"p1:g0","target":"p2:g0","fastenerKind":"fk","gap":1.0,"shift":2.0,"rise":3.0,"rotation":4.0,"turn":5.0,"tilt":6.0})).unwrap();
        let meta: puzzle_5d::Puzzle5dMeta = serde_json::from_value(json!({"description":"a scene"})).unwrap();
        let document = puzzle_5d::Puzzle5dProjection::default();
        vec![
            Puzzle5dOperation::SetPart { index: 0, part },
            Puzzle5dOperation::RemovePart { id: "p1".into() },
            Puzzle5dOperation::SetFastener { index: 1, fastener },
            Puzzle5dOperation::RemoveFastener { id: "f1".into() },
            Puzzle5dOperation::SetMeta { meta },
            Puzzle5dOperation::SetDocument { document },
        ]
    }

    #[test]
    fn debug_wire_dump() {
        for operation in ops() {
            let text = operation.print_op();
            let bytes = encode_op(&operation).expect("encode");
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[WIRE] {text} | {} | {hex}", bytes.len());
            assert_eq!(decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🧪️WireBaselineDump
