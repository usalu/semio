//! 📡️ Puzzle 5d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle5dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! puzzle-5d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::puzzle5d::schema::mutations::text::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle5dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle5dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle5dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle5dMutation, protocol::ProtocolError> {
    Puzzle5dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle5dEnvelope = ArtifactEnvelope<Puzzle5dSnapshot, Puzzle5dMutation>;
pub type Puzzle5dStore = ArtifactStore<Puzzle5dSnapshot, Puzzle5dMutation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_document_vcs_replays_granular_operations() {
        use crate::artifacts::puzzle5d::mutations::create_part;
        use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::empty_puzzle5d_snapshot;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, PUZZLE_5D_SCHEMA};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_snapshot(), None))).expect("store");
        semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![create_part(Puzzle5dPart { id: "p1".into(), ..Default::default() }, None)], description: None })).expect("apply");
        let projection = store.snapshot().expect("projection");
        assert_eq!(projection.parts.len(), 1);
        assert_eq!(projection.parts[0].id, "p1");
    }
}
//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
mod wire_format_guard {
    //! 🔒️ Byte-level `OpBinary` round-trip guard for the semantic-mutations-overhaul vocabulary
    //! (ticket `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`). The pre-overhaul whole-record-upsert /
    //! whole-document-replace wire bytes this guard used to freeze no longer exist — that
    //! vocabulary is banned outright, not preserved — so this now asserts the NEW operations'
    //! `OpText`/`OpBinary` round-trip instead of pinning byte literals for a wire shape this ticket
    //! deliberately changed.
    use super::*;
    use crate::artifacts::puzzle5d::mutations::{change_description, change_domain, connect_grips, create_part, delete_part, disconnect_grips};
    use crate::artifacts::puzzle5d::Puzzle5dPart;
    use protocol::OpText;

    fn ops() -> Vec<Puzzle5dMutation> {
        let part = Puzzle5dPart { id: "p1".into(), part_kind: Some("Capsule".into()), ..Default::default() };
        vec![
            create_part(part, Some(0)),
            delete_part("p1".into()),
            connect_grips("f1".into(), "p1:g0".into(), "p2:g0".into(), Some("fk".into()), 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 0.0),
            disconnect_grips("f1".into()),
            change_domain("architecture".into()),
            change_description("a scene".into()),
        ]
    }

    /// ⚖️ Every operation still prints, parses, encodes, and decodes back to an equal value.
    #[test]
    fn operations_round_trip_text_and_binary() {
        let operations = ops();
        assert!(!operations.is_empty());
        for operation in &operations {
            let line = operation.print_op();
            assert_eq!(&Puzzle5dMutation::parse_op(&line).expect("parse_op"), operation);
            let bytes = encode_op(operation).expect("encode");
            assert_eq!(&decode_op(&bytes).expect("decode"), operation);
        }
    }
}
//#endregion 🔒️WireFormatGuard
