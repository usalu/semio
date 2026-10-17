//! ⚖️ Real-store authoring: a raster document edit authors, applies, undoes and redoes through the raster editor's own catalog and the runtime's canonical operation wire.
use semio_framework_plugin::{authoring_law::author_through_document_store, EditorApp};
use semio_framework_value::ToValue;
use semio_s_artifact_raster_raster_editor::RasterPlayApp;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterTransform};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_mutations_layers::create_layer::CreateLayer;

fn assert_row_is_the_serde_document(row: &[u8], mutation: &RasterMutation) {
    assert!(row.starts_with(b"op"), "the canonical wire carries its operation header");
    let wire: serde_json::Value = serde_json::from_slice(&row[2..]).expect("the wire body is JSON an independent parser accepts");
    assert_eq!(wire, serde_json::to_value(mutation.to_value()).expect("independent serde_json document"));
}

/// 🖨️ Creating a layer publishes one forward row equal to the independent serde_json document, and its inverse deletes the layer again.
#[semio_framework_async_macros::async_test]
async fn a_raster_document_edit_authors_applies_undoes_and_redoes_through_its_real_store() {
    let layer = RasterLayerNode::Pixel { id: "layer-1".into(), name: "Background".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, width: Some(4), height: Some(4), image_key: None };
    let mutation = RasterMutation::CreateLayer(CreateLayer { parent_id: None, index: 0, layer: Box::new(layer) });
    let law = author_through_document_store::<EditorApp<RasterPlayApp>>(mutation.clone()).await;
    assert_eq!(law.applied.layers.len(), 1);
    assert!(law.undone.layers.is_empty(), "undo deletes the created layer");
    assert_eq!(law.redone.layers, law.applied.layers, "redo restores the created layer");
    assert_eq!(law.forward_rows.len(), 1);
    assert_row_is_the_serde_document(&law.forward_rows[0], &mutation);
    assert_eq!(law.inverse_rows.len(), 1, "creating one layer is undone by exactly one row");
    assert!(law.inverse_rows[0].starts_with(b"op"));
}
