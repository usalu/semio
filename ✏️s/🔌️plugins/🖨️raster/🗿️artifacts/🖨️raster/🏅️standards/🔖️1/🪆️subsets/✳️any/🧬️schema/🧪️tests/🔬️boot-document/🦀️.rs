use super::*;
use crate::standards::v1::subsets::any::schema::mutations::binary::test_support::retire_raster_snapshot;

/// 📄️ The boot document is the committed Semio-logo carrier read through the artifact's own text
/// codec — not the empty scaffold, and not a Rust restatement of the `.dsl.semio` bytes.
#[semio_framework_async_macros::async_test]
async fn default_document_boots_on_the_semio_demo_carrier() {
    let document = default_raster_document();
    assert_ne!(document, empty_raster_document(), "the boot document must not fall back to the empty scaffold");
    assert_eq!(document.id, "semio-demo");
    assert_eq!(document.title.as_deref(), Some("Semio Raster Demo"));
    assert_eq!(document.layers.len(), 2, "the Semio logo carries a backdrop pixel layer and a brighten adjustment layer");
    assert_eq!(layer_node_id(&document.layers[0]), "backdrop");
    assert!(matches!(document.layers[1], RasterLayerNode::Adjustment { .. }), "the second layer is the brighten adjustment");
    retire_raster_snapshot(document);
}

/// 📚️ `raster_example_document` resolves exactly the ids this subset registers, and nothing else.
#[semio_framework_async_macros::async_test]
async fn only_a_registered_example_id_resolves_to_a_document() {
    let registered = raster_example_document(crate::examples::art_raster_demo::ID).expect("the demo example id resolves");
    let expected = default_raster_document();
    assert_eq!(registered, expected);
    assert!(raster_example_document("not-a-real-example").is_none());
    retire_raster_snapshot(registered);
    retire_raster_snapshot(expected);
}
