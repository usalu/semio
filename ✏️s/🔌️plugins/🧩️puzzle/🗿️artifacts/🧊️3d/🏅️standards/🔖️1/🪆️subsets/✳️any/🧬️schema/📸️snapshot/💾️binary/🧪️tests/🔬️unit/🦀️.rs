use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::text as dsl;

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
    use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle3d_store, puzzle3d_store};
    use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle3dMutation;
    use crate::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = semio_framework::io::resolve_ready(puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None))).expect("store");
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_object(object, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests
