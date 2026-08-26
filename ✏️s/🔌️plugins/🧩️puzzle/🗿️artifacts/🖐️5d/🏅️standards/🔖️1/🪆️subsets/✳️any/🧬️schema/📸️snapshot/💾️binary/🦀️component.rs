//! 🎒️ Puzzle 5d artifact — the binary document surface and its laws: `encode`/`decode` over the
//! typed `Puzzle5dSnapshot`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Puzzle5dSnapshot` to its binary pack form.
pub fn encode(document: &Puzzle5dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle5dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle5dSnapshot, PackError> {
    <Puzzle5dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_representative_document() {
        let document = Puzzle5dSnapshot::default();
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle5d::op::Puzzle5dMutation;
        use crate::artifacts::puzzle5d::spr::Puzzle5dStore;
        use crate::artifacts::puzzle5d::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(crate::artifacts::puzzle5d::PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None))).expect("store");
        let part = Puzzle5dPart { id: "p1".into(), anchor: Puzzle5dPartAnchor::Fixed, part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
        semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle5d::mutations::create_part(part, None)], description: None })).expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
