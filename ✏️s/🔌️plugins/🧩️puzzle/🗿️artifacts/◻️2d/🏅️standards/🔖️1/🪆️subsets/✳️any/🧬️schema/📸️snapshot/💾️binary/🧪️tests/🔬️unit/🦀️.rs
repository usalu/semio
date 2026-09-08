use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::text as dsl;

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
    use crate::standards::v1::subsets::any::schema::mutations::binary::Puzzle2dStore;
    use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle2dMutation;
    use crate::{Puzzle2dNode, PUZZLE_2D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = semio_framework::io::resolve_ready(Puzzle2dStore::new(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None))).expect("store");
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_node(node, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
}
//#endregion 🔖️CommandEnvelopeTests
