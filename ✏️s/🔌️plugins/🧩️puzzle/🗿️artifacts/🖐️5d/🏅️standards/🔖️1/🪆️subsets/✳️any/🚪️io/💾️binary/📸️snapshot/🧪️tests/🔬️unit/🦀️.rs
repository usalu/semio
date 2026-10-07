
use crate::standards::v1::subsets::any::io::binary::snapshot::*;

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
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
    use crate::host::owned::{close_puzzle5d_store,puzzle5d_store};
    use crate::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = (puzzle5d_store(create_document_envelope(crate::PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("store");
    let part = Puzzle5dPart { id: "p1".into(), anchor: Puzzle5dPartAnchor::Fixed, part_kind: None, part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
    (store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None)], transaction: None })).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    (semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone()))).await;
    close_puzzle5d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests
