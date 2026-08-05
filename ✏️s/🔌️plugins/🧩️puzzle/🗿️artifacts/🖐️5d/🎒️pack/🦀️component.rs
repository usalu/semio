//! 🎒️ Puzzle 5d artifact — the binary document surface and its laws: `encode`/`decode` over the
//! typed `Puzzle5dProjection`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.

use crate::artifacts::puzzle5d::Puzzle5dProjection;
use store::PackError;

/// 📦️ Encodes a `Puzzle5dProjection` to its binary pack form.
pub fn encode(document: &Puzzle5dProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle5dProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle5dProjection, PackError> {
    <Puzzle5dProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_representative_document() {
        let document = Puzzle5dProjection::default();
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle5dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle5d::op::Puzzle5dOperation;
        use crate::artifacts::puzzle5d::spr::Puzzle5dStore;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle5dStore::new(create_document_envelope(crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dProjection::default(), None));
        let part = Puzzle5dPart { id: "p1".into(), part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
        store.dispatch(DocumentCommand::Apply { operations: vec![Puzzle5dOperation::SetPart { index: 0, part }], description: None }).expect("apply");
        let edit: &Edit<Puzzle5dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<Puzzle5dProjection, Puzzle5dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
