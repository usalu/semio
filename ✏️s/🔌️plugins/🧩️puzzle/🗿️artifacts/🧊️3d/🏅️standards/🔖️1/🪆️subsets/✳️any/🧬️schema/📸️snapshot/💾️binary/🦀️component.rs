//! 🎒️ Puzzle 3d artifact — the binary document surface and its laws: `encode`/`decode` over the
//! typed `Puzzle3dSnapshot`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Puzzle3dSnapshot` to its binary pack form.
pub fn encode(document: &Puzzle3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle3dSnapshot, PackError> {
    <Puzzle3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT).expect("parse concrete-forest example");
        semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Puzzle3dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this
    /// file's existing dsl/pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::puzzle3d::op::Puzzle3dMutation;
        use crate::artifacts::puzzle3d::spr::Puzzle3dStore;
        use crate::artifacts::puzzle3d::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand};

        let mut store = Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None));
        let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Puzzle3dMutation::SetObject { index: 0, object }], description: None }).expect("apply");
        let edit: &Edit<Puzzle3dMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
