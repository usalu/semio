//! 🧫️ Block 3d's committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is
//! the shared `semio_s_mutation_vectors`.

use semio_framework_test_fixtures::fixture_path;

/// 🚪️ The door/hatch vortex-kind catalog every committed block-3d vector decodes against.
fn vortex_kind_catalog() {
    semio_s_artifact_block_3d_model::validate_vortex_kind_catalog(&[
        semio_s_artifact_block_3d_model::Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() },
        semio_s_artifact_block_3d_model::Block3dVortexKind { id: "hatch".into(), name: "hatch".into(), label: "Hatch".into(), color: "hsl(37 52% 48%)".into(), default_cable_kind: "cable.bus".into() },
    ]);
}

#[semio_framework_async_macros::async_test]
async fn committed_vectors_hold() {
    let root = fixture_path!("../../../../🧪️tests/🧫️fixtures/🧬️mutations");
    semio_s_mutation_vectors::assert_committed_vectors::<semio_s_artifact_block_3d_model::Block3dSnapshot, semio_s_artifact_block_3d_mutations::Block3dMutation>(&root, vortex_kind_catalog).await;
}
