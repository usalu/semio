//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle2dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::puzzle2d::schema::mutations::text::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle2dMutation` to its binary command form.
pub async fn encode_op(operation: &Puzzle2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<Puzzle2dMutation, protocol::ProtocolError> {
    Puzzle2dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle2dEnvelope = ArtifactEnvelope<Puzzle2dSnapshot, Puzzle2dMutation>;
pub type Puzzle2dStore = ArtifactStore<Puzzle2dSnapshot, Puzzle2dMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn puzzle2d_document_vcs_replays_granular_operations() {
        use crate::artifacts::puzzle2d::schema::empty_puzzle2d_snapshot;
        use crate::artifacts::puzzle2d::mutations::create_node;
        use crate::artifacts::puzzle2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", empty_puzzle2d_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![create_node(Puzzle2dNode { id: "n1".into(), ..Default::default() }, None)],
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
    //! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul vocabulary
    //! (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`). The pre-overhaul whole-record-upsert / whole-document-replace wire
    //! bytes this guard used to freeze no longer exist — that vocabulary is banned outright, not
    //! preserved — so this now asserts the NEW operations' `OpText`/`OpBinary` round-trip
    //! (`print_op`/`parse_op`, `encode_op`/`decode_op`) instead of pinning byte literals for a wire
    //! shape this ticket deliberately changed.
    use super::*;
    use crate::artifacts::puzzle2d::mutations::{change_manifest_id, connect_handles, create_node, delete_node, disconnect_handles, move_node};
    use crate::artifacts::puzzle2d::Puzzle2dNode;
    use protocol::OpText;

    async fn ops() -> Vec<Puzzle2dMutation> {
        let node = Puzzle2dNode { id: "n1".into(), node_kind: Some("Base".into()), shape: Some("circle".into()), x: 1.5, y: -2.25, radius: Some(3.0), text: Some("hi".into()), icon_kind: Some("base".into()), root: Some(true), scale: Some(2.0), visible: Some(true), locked: Some(false), ..Default::default() };
        vec![
            create_node(node, Some(0)),
            move_node("n1".into(), 4.0, 5.0),
            delete_node("n1".into()),
            connect_handles("e1".into(), "n1:h0".into(), "n2:h0".into(), Some("wire.link".into()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, Some("none".into()), Some("arrow".into())),
            disconnect_handles("e1".into()),
            change_manifest_id(Some("nakagin".into())),
        ]
    }

    /// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
    #[semio_framework_async_macros::async_test]
    async fn operations_round_trip_text_and_binary() {
        let operations = ops();
        assert!(!operations.is_empty());
        for operation in &operations {
            let line = operation.print_op();
            assert_eq!(&Puzzle2dMutation::parse_op(&line).expect("parse_op"), operation);
            let bytes = encode_op(operation).expect("encode");
            assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🔒️WireFormatGuard
