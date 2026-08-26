//! 🎒️ Puzzle 2d artifact — the binary document surface and its laws: `encode`/`decode` over the
//! typed `Puzzle2dSnapshot`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Puzzle2dSnapshot` to its binary pack form.
pub fn encode(document: &Puzzle2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle2dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle2dSnapshot, PackError> {
    <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete-forest example");
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle2dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle2d::op::Puzzle2dMutation;
        use crate::artifacts::puzzle2d::spr::Puzzle2dStore;
        use crate::artifacts::puzzle2d::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = semio_framework::io::resolve_ready(Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None))).expect("store");
        let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
        semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::puzzle2d::mutations::create_node(node, None)], description: None })).expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
