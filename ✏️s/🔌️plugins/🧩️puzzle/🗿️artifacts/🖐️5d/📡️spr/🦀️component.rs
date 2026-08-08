//! 📡️ Puzzle 5d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle5dMutation`'s binary wire form, plus the `DocumentEnvelope`/`DocumentStore` aliases every
//! puzzle-5d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::puzzle5d::op::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `Puzzle5dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle5dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle5dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle5dMutation, protocol::ProtocolError> {
    Puzzle5dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle5dEnvelope = DocumentEnvelope<Puzzle5dSnapshot, Puzzle5dMutation>;
pub type Puzzle5dStore = DocumentStore<Puzzle5dSnapshot, Puzzle5dMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_document_vcs_replays_granular_operations() {
        use crate::artifacts::puzzle5d::engine::empty_puzzle5d_projection;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE_5D_SCHEMA};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                mutations: vec![Puzzle5dMutation::SetPart { index: 0, part: Puzzle5dPart { id: "p1".into(), part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() } }],
                description: None,
            })
            .expect("apply");
        let projection = store.snapshot().expect("projection");
        assert_eq!(projection.parts.len(), 1);
        assert_eq!(projection.parts[0].id, "p1");
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
    use crate::artifacts::puzzle5d as puzzle_5d;
    use protocol::OpText;
    use serde_json::json;

    fn ops() -> Vec<Puzzle5dMutation> {
        let part: puzzle_5d::Puzzle5dPart = serde_json::from_value(json!({"id":"p1","partKind":"Capsule","2d":{"x":1.0,"y":2.0,"shape":"circle","radius":3.0,"text":"t","iconKind":"i","hidden":false,"locked":false},"3d":{"origin":[1.0,2.0,3.0],"meshUrl":"/m.glb","orientation":[0.0,0.0,0.0,1.0],"scale":[2.0,3.0,4.0],"label":"L"},"grips":[{"id":"g0","gripKind":"k","2d":{"angle":0.5,"gripKind":"k","radius":3.0},"3d":{"position":[0.0,0.0,0.0],"direction":[0.0,0.0,1.0],"radius":3.0,"label":"g"}}]})).unwrap();
        let fastener: puzzle_5d::Puzzle5dFastener = serde_json::from_value(json!({"id":"f1","source":"p1:g0","target":"p2:g0","fastenerKind":"fk","gap":1.0,"shift":2.0,"rise":3.0,"rotation":4.0,"turn":5.0,"tilt":6.0})).unwrap();
        let meta: puzzle_5d::Puzzle5dMeta = serde_json::from_value(json!({"description":"a scene"})).unwrap();
        let document = Puzzle5dSnapshot::default();
        vec![
            Puzzle5dMutation::SetPart { index: 0, part },
            Puzzle5dMutation::RemovePart { id: "p1".into() },
            Puzzle5dMutation::SetFastener { index: 1, fastener },
            Puzzle5dMutation::RemoveFastener { id: "f1".into() },
            Puzzle5dMutation::SetMeta { meta },
            Puzzle5dMutation::SetDocument { snapshot: document },
        ]
    }

    /// 🔒️ The exact `print_op | byte-length | hex` of every operation row, captured from the
    /// pre-consolidation `📡️protocol` crate BEFORE this plugin was merged into one crate. A
    /// round-trip law is self-consistent and would happily pass on a silently changed format;
    /// only these frozen bytes prove the wire did not move.
    const PRE_MIGRATION_OPERATION_WIRE: &[&str] = &[
        "setPart index=0 part { id=p1 part-kind=Capsule part-2d=x=1 y=2 shape=circle radius=3 text=t icon-kind=i hidden=false locked=false part-3d=origin=@1,2,3 mesh-url=\"/m.glb\" orientation=0,0,0,1 scale=2,3,4 label=L grips=[ id=g0 grip-kind=k grip-2d=angle=0.5rad grip-kind=k radius=3 grip-3d=position=@0,0,0 direction=^0,0,1 radius=3 label=g ] } | 306 | 01000a062f6d2e676c620743617073756c65014c06636972636c6501670267300169016b027031017402000400010e0d05000608010601020d080005000000000000f03f010500000000000000400206030305000000000000084006060907060608010901030d05001503000000000000f03f00000000000000400000000000000840010600021504000000000000000000000000000000000000000000000000000000000000f03f031503000000000000004000000000000008400000000000001040040602040c010d04000605010607020d030005000000000000e03f01060702050000000000000840030d0400150300000000000000000000000000000000000000000000000001150300000000000000000000000000000000000000000000f03f02050000000000000840030604",
        "removePart id=p1 | 10 | 01010102703101000600",
        "setFastener index=1 fastener { id=f1 source=\"p1:g0\" target=\"p2:g0\" fastener-kind=fk gap=1 shift=2 rise=3 rotation=4 turn=5 tilt=6 } | 101 | 01020402663102666b0570313a67300570323a673002000401010e0d0a0006000106020206030306010405000000000000f03f0505000000000000004006050000000000000840070500000000000010400805000000000000144009050000000000001840",
        "removeFastener id=f1 | 10 | 01030102663101000600",
        "setMeta meta { description=\"a scene\" } | 19 | 0104010761207363656e6501000e0d01000600",
        "setDocument snapshot { schema=puzzle.5d domain=architecture meta { description=\"\" } kind-compatibility [source:REF target:REF bidirectional:BOOL] { } parts [id:TEXT part-kind:REF part-2d:REC part-3d:REC grips:LIST] { } fasteners [id:TEXT source:TEXT target:TEXT fastener-kind:REF gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM] { } } | 111 | 010503000c6172636869746563747572650970757a7a6c652e356401000e0d06000602010601030e0d0100060005140003000005010005020001061400050000050100050200000300000400000714000a000005010005020005030005040004050004060004070004080004090004",
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
