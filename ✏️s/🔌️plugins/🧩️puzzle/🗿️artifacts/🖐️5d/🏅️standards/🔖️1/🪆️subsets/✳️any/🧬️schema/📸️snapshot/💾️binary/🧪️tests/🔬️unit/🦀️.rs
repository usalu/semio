
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
    use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dMutation;
    use crate::standards::v1::subsets::any::schema::mutations::binary::Puzzle5dStore;
    use crate::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(crate::PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None))).expect("store");
    let part = Puzzle5dPart { id: "p1".into(), anchor: Puzzle5dPartAnchor::Fixed, part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
}
//#endregion 🔖️CommandEnvelopeTests
