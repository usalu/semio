//! 🔁️ A puzzle3d store replays granular operations into its projection.

use semio_s_artifact_puzzle_3d_host::close_puzzle3d_store;

use semio_s_artifact_puzzle_3d_host::puzzle3d_store;

#[semio_framework_async_macros::async_test]
async fn puzzle3d_document_vcs_replays_granular_operations() {
    use semio_s_artifact_puzzle_3d_model::empty_puzzle3d_snapshot;
    use {semio_s_artifact_puzzle_3d_model::Puzzle3dObject, semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA};
    use store::os_store::document_materialize::create_document_envelope;
    use store::os_store::store_schema::ArtifactCommand;

    let mut store = (puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_snapshot(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).await.expect("store");
    (store.dispatch(ArtifactCommand::Apply {
        mutations: vec![semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object(
            Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false },
            None,
        )],
        transaction: None,
    }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).await
    .expect("apply");
    let projection = store.snapshot().expect("projection");
    assert_eq!(projection.objects.len(), 1);
    assert_eq!(projection.objects[0].id, "o1");
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
