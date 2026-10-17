//! 🧫️ Block 5d's committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is
//! the shared `semio_s_mutation_vectors`.

use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn committed_vectors_hold() {
    let root = fixture_path!("../../../../🧪️tests/🧫️fixtures/🧬️mutations");
    semio_s_mutation_vectors::assert_committed_vectors::<semio_s_artifact_block_5d_model::Block5dSnapshot, semio_s_artifact_block_5d_mutations::Block5dMutation>(&root, || {}).await;
}
