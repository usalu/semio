//! 🧪️ Runtime law: the macro-installed document-lane preparation (`snapshot_details_editor_support!`) reaches `Prepared` through the real sealer, publishes the exact post snapshot and an inverse that restores the base.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_stdio_epw::editor::epw::EpwEditor;
use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::blank_epw_snapshot;
use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::mutations::set_location::SetLocation;
use semio_s_artifact_stdio_epw::{apply_mutation, EpwMutation};

#[test]
fn macro_installed_preparation_publishes_the_post_snapshot_and_a_restoring_inverse() {
    let base = blank_epw_snapshot();
    let mut location = base.location.clone();
    location.city = "Zürich".into();
    location.country = "CHE".into();
    let mutation = EpwMutation::SetLocation(SetLocation { location });
    let factory = <EpwEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the EPW document lane declares a preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
    let mut expected = base.clone();
    apply_mutation(&mut expected, &mutation);
    assert_eq!(*law.post, expected);
    assert_eq!(law.post.location.city, "Zürich");
    let mut restored = (*law.post).clone();
    for row in &law.inverse {
        apply_mutation(&mut restored, row);
    }
    assert_eq!(restored, base);
}
