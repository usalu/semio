//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle2dMutation`'s binary wire form, plus the `DocumentEnvelope`/`DocumentStore` aliases every
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::puzzle2d::op::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `Puzzle2dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle2dMutation, protocol::ProtocolError> {
    Puzzle2dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle2dEnvelope = DocumentEnvelope<Puzzle2dSnapshot, Puzzle2dMutation>;
pub type Puzzle2dStore = DocumentStore<Puzzle2dSnapshot, Puzzle2dMutation>;
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
                mutations: vec![Puzzle2dMutation::SetNode {
                    index: 0,
                    node: Puzzle2dNode { id: "n1".into(), node_kind: None, shape: None, x: 0.0, y: 0.0, radius: None, width: None, height: None, text: None, icon_kind: None, root: None, scale: None, visible: None, locked: None, handles: Vec::new() },
                }],
                description: None,
            })
            .expect("apply");
        let projection = store.snapshot().expect("projection");
        assert_eq!(projection.nodes.len(), 1);
        assert_eq!(projection.nodes[0].id, "n1");
    }
}
//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
mod wire_format_guard {
    //! 🔒️ The permanent byte-level regression guard for this artifact's spr codec, frozen from the
    //! pre-consolidation `📡️protocol` crate (master ticket
    //! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, TEMPLATE.md §0.4/§7).
    use super::*;
    use crate::artifacts::puzzle2d as puzzle_2d;
    use protocol::OpText;
    use serde_json::json;

    fn ops() -> Vec<Puzzle2dMutation> {
        let node: puzzle_2d::Puzzle2dNode = serde_json::from_value(json!({"id":"n1","nodeKind":"Base","shape":"circle","x":1.5,"y":-2.25,"radius":3.0,"text":"hi","iconKind":"base","root":true,"scale":2.0,"visible":true,"locked":false,"handles":[]})).unwrap();
        let edge: puzzle_2d::Puzzle2dEdge = serde_json::from_value(json!({"id":"e1","source":"n1:h0","target":"n2:h0","edgeKind":"wire.link","sourceTip":"none","targetTip":"arrow","visible":true,"locked":false})).unwrap();
        let meta: puzzle_2d::Puzzle2dMeta = serde_json::from_value(json!({"manifestId":"nakagin","kindCompatibility":[{"source":"a","target":"b","bidirectional":true,"specificity":"handle"}]})).unwrap();
        let document = Puzzle2dSnapshot::default();
        vec![
            Puzzle2dMutation::SetNode { index: 0, node },
            Puzzle2dMutation::RemoveNode { id: "n1".into() },
            Puzzle2dMutation::SetEdge { index: 1, edge },
            Puzzle2dMutation::RemoveEdge { id: "e1".into() },
            Puzzle2dMutation::SetMeta { meta },
            Puzzle2dMutation::SetDocument { snapshot: document },
        ]
    }

    /// 🔒️ The exact `print_op | byte-length | hex` of every operation row, captured from the
    /// pre-consolidation `📡️protocol` crate BEFORE this plugin was merged into one crate. A
    /// round-trip law is self-consistent and would happily pass on a silently changed format;
    /// only these frozen bytes prove the wire did not move.
    const PRE_MIGRATION_OPERATION_WIRE: &[&str] = &[
        "setNode index=0 node { id=n1 node-kind=Base shape=circle x=1.5 y=-2.25 radius=3 text=hi icon-kind=base root=true scale=2 visible=true locked=false handles=[ ] } | 98 | 0100050442617365046261736506636972636c65026869026e3102000400010e0d0d0006040106000206020305000000000000f83f040500000000000002c0050500000000000008400806030906010a020b0500000000000000400c020d010e0c00",
        "removeNode id=n1 | 10 | 010101026e3101000600",
        "setEdge index=1 edge { id=e1 source=\"n1:h0\" target=\"n2:h0\" edge-kind=wire.link source-tip=none target-tip=arrow visible=true locked=false } | 69 | 010206056172726f77026531056e313a6830056e323a6830046e6f6e6509776972652e6c696e6b02000401010e0d0800060101060202060303060504060405060006020701",
        "removeEdge id=e1 | 10 | 01030102653101000600",
        "setMeta meta { manifest-id=nakagin kind-compatibility [bidirectional:BOOL specificity:ENUM source:TEXT target:TEXT] { true handle a b } } | 43 | 01040301610162076e616b6167696e01000e0d020006020114010400000101010006030200050003000501",
        "setDocument snapshot { schema=puzzle.2d.fixture camera { x=0 y=0 zoom=1 } meta { kind-compatibility [bidirectional:BOOL specificity:ENUM source:TEXT target:TEXT] { } } nodes [id:TEXT node-kind:TEXT shape:TEXT x:NUM y:NUM radius:NUM width:NUM height:NUM text:TEXT icon-kind:TEXT root:BOOL scale:NUM visible:BOOL locked:BOOL handles:LIST] { } edges [id:TEXT source:REF target:REF edge-kind:TEXT source-tip:TEXT target-tip:TEXT visible:BOOL locked:BOOL] { } } | 160 | 0105011170757a7a6c652e32642e6669787475726501000e0d05000600010e0d0300050000000000000000010500000000000000000205000000000000f03f0214000f0000050100050200050300040400040500040600040700040800050900050a00010b00040c00010d00010e000003140008000005010005020005030005040005050005060001070001040e0d0101140004000001010006020005030005",
    ];

    /// ⚖️ Every operation row still prints and encodes to its pre-migration bytes, and still
    /// decodes back to the same value.
    #[test]
    fn operation_rows_keep_their_pre_migration_wire_bytes() {
        let operations = ops();
        assert_eq!(operations.len(), PRE_MIGRATION_OPERATION_WIRE.len(), "every operation variant must be covered by the frozen wire table");
        for (operation, expected) in operations.iter().zip(PRE_MIGRATION_OPERATION_WIRE) {
            let bytes = encode_op(operation).expect("encode");
            let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            assert_eq!(&format!("{} | {} | {hex}", operation.print_op(), bytes.len()), expected);
            assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🔒️WireFormatGuard
