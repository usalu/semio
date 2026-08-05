//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle2dOperation`'s binary wire form, plus the `DocumentEnvelope`/`DocumentStore` aliases every
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

use crate::artifacts::puzzle2d::op::Puzzle2dOperation;
use crate::artifacts::puzzle2d::Puzzle2dProjection;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `Puzzle2dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle2dOperation, protocol::ProtocolError> {
    Puzzle2dOperation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle2dEnvelope = DocumentEnvelope<Puzzle2dProjection, Puzzle2dOperation>;
pub type Puzzle2dStore = DocumentStore<Puzzle2dProjection, Puzzle2dOperation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle2d_document_vcs_replays_granular_operations() {
        use crate::artifacts::puzzle2d::engine::empty_puzzle2d_projection;
        use crate::artifacts::puzzle2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", empty_puzzle2d_projection(), None));
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

//#region 🧪️WireBaselineDump
#[cfg(test)]
mod wire_baseline_dump {
    use super::*;
    use crate::artifacts::puzzle2d as puzzle_2d;
    use protocol::OpText;
    use serde_json::json;

    fn ops() -> Vec<Puzzle2dOperation> {
        let node: puzzle_2d::Puzzle2dNode = serde_json::from_value(json!({"id":"n1","nodeKind":"Base","shape":"circle","x":1.5,"y":-2.25,"radius":3.0,"text":"hi","iconKind":"base","root":true,"scale":2.0,"visible":true,"locked":false,"handles":[]})).unwrap();
        let edge: puzzle_2d::Puzzle2dEdge = serde_json::from_value(json!({"id":"e1","source":"n1:h0","target":"n2:h0","edgeKind":"wire.link","sourceTip":"none","targetTip":"arrow","visible":true,"locked":false})).unwrap();
        let meta: puzzle_2d::Puzzle2dMeta = serde_json::from_value(json!({"manifestId":"nakagin","kindCompatibility":[{"source":"a","target":"b","bidirectional":true,"specificity":"handle"}]})).unwrap();
        let document = Puzzle2dProjection::default();
        vec![
            Puzzle2dOperation::SetNode { index: 0, node },
            Puzzle2dOperation::RemoveNode { id: "n1".into() },
            Puzzle2dOperation::SetEdge { index: 1, edge },
            Puzzle2dOperation::RemoveEdge { id: "e1".into() },
            Puzzle2dOperation::SetMeta { meta },
            Puzzle2dOperation::SetDocument { document },
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
