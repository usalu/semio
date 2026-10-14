//! ✉️ The CW7 command-envelope law over a real puzzle3d store: an applied edit round-trips through its mutation envelopes.

use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_s_artifact_puzzle_3d_model::Puzzle3dObject;

/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `Puzzle3dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside the
/// snapshot codecs' own dsl/pack round-trip laws (same pattern as `dag`'s own
/// `command_envelope_round_trip_holds_for_an_applied_operation`).
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use semio_s_artifact_puzzle_3d_host::{close_puzzle3d_store, puzzle3d_store};
    use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
    use semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA;
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand};

    let mut store = (puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("store");
    let object = Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    (store.dispatch(ArtifactCommand::Apply { mutations: vec![semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(object, None)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    (semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone()))).await;
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
