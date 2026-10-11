//! 🧪️ Runtime law: the config-lane `config_apply` preparation reaches `Prepared` through the real sealer, publishes the exact post config and an inverse that restores the base.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor;

type Config = <TiffAnyEditor as ArtifactEditor>::Config;
type ConfigMutation = <TiffAnyEditor as ArtifactEditor>::ConfigMutation;

#[test]
fn config_apply_preparation_publishes_the_post_config_and_a_restoring_inverse() {
    let base = Config::default();
    let mutation = ConfigMutation::from_value(DslValue::object([("kind".into(), DslValue::String("set-selected-ifd".into())), ("selected_ifd".into(), DslValue::uint(2))])).expect("a config mutation decodes");
    let factory = <TiffAnyEditor as ArtifactEditor>::build_config_store_one_item_preparation_factory().expect("the TIFF config lane declares a preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
    assert_ne!(*law.post, base);
    assert_eq!(law.inverse.len(), 1);
    assert_eq!(law.inverse[0].to_value(), ConfigMutation::from_value(DslValue::object([("kind".into(), DslValue::String("set-selected-ifd".into())), ("selected_ifd".into(), DslValue::uint(0))])).expect("the restoring row decodes").to_value());
}
