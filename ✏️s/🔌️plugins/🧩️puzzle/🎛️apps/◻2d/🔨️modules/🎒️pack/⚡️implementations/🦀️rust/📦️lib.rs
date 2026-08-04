//! 📦️ Puzzle 2d app — binary document surface + laws (constitutional: pack).

use puzzle_2d::Puzzle2dProjection;
use store::PackError;

/// 📦️ Encodes a `Puzzle2dProjection` to its binary pack form.
pub fn encode(document: &Puzzle2dProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle2dProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle2dProjection, PackError> {
    <Puzzle2dProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = puzzle_2d_dsl::parse_dsl(puzzle_2d_dsl::PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete-forest example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle2dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use puzzle_2d::Puzzle2dNode;
        use puzzle_2d_op::{Puzzle2dOperation, Puzzle2dStore};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle2dStore::new(create_document_envelope(puzzle_2d::PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dProjection::default(), None));
        let node = Puzzle2dNode { id: "n1".into(), node_kind: None, shape: None, x: 0.0, y: 0.0, radius: None, width: None, height: None, text: None, icon_kind: None, root: None, scale: None, visible: None, locked: None, handles: Vec::new() };
        store.dispatch(DocumentCommand::Apply { operations: vec![Puzzle2dOperation::SetNode { index: 0, node }], description: None }).expect("apply");
        let edit: &Edit<Puzzle2dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<Puzzle2dProjection, Puzzle2dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
