//! 🧪️ Runtime law: the document-lane `mutation_apply` preparation reaches `Prepared` through the real sealer, publishes the exact post snapshot and an inverse that restores the base.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_stdio_txt::editor::txt::TxtEditor;
use semio_s_artifact_stdio_txt::schema::mutations::SetLineMutation;
use semio_s_artifact_stdio_txt::{apply_mutation, TxtMutation, TxtSnapshot};

#[test]
fn mutation_apply_preparation_publishes_the_post_snapshot_and_a_restoring_inverse() {
    let base = TxtSnapshot::from_body("alpha\nbeta\n");
    let mutation = TxtMutation::SetLine(SetLineMutation { index: 1, text: "gamma".into() });
    let factory = <TxtEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the TXT document lane declares a preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
    let mut expected = base.clone();
    apply_mutation(&mut expected, &mutation);
    assert_eq!(*law.post, expected);
    assert_eq!(law.post.to_body(), "alpha\ngamma\n");
    let mut restored = (*law.post).clone();
    for row in &law.inverse {
        apply_mutation(&mut restored, row);
    }
    assert_eq!(restored, base);
}
