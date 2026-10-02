//! 🧪️ Laws of the raster paint tool: a released stroke is ONE transaction holding ONE `paint-stroke` leaf built from the
//! session brush, target, mask value and selection; a stroke the layer cannot take leaves zero trace; two releases are
//! two transactions; a mounted release lands exactly the leaf.

use super::*;
use crate::editor::raster::config::RasterPixelSelection;
use semio_framework_plugin::{AppOperationContext, HistoryView};

fn document(locked: bool, visible: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    dsl::json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"paint","title":"Paint","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":{visible},"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":4,"height":3,"imageKey":null,"locked":{locked}}},{{"kind":"group","id":"folder","name":"Folder","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"children":[]}}]}}"#
    ))
    .expect("the paint document decodes")
}

fn config(target: &str) -> RasterConfig {
    RasterConfig { brush_size: 4.0, brush_opacity: 0.5, brush_color: "#ff8000".into(), brush_hardness: 0.25, paint_target: target.into(), mask_value: 51, ..RasterConfig::default() }
}

fn stroke(layer: &str, tool: &str) -> PaintStroke {
    PaintStroke { layer_id: layer.into(), tool: tool.into(), xs: vec![0.5, 3.5], ys: vec![1.5, 1.5] }
}

fn leaf(mutation: RasterMutation) -> PaintStrokeLeaf {
    let RasterMutation::PaintStroke(leaf) = mutation else { panic!("a paint stroke") };
    leaf
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

//#region 🎨️Leaf
#[test]
fn a_released_stroke_paints_with_the_session_brush() {
    let document = document(false, true);
    let built = leaf(paint_stroke_leaf(&stroke("ink", "brush"), &document, &config("pixels")).expect("the stroke paints"));
    assert_eq!((built.layer_id.as_str(), built.target.as_str(), built.tool.as_str()), ("ink", "pixels", "brush"));
    assert_eq!(built.brush, RasterBrush { size: 4.0, hardness: 0.25, opacity: 0.5, color: vec![1.0, 128.0 / 255.0, 0.0, 1.0] });
    assert_eq!(built.points, vec![RasterStrokePoint { x: 0.5, y: 1.5 }, RasterStrokePoint { x: 3.5, y: 1.5 }]);
    assert_eq!(built.selection, None);
    retire(document);
}

#[test]
fn a_mask_stroke_paints_the_session_mask_value_as_its_grey_level() {
    let document = document(false, true);
    let built = leaf(paint_stroke_leaf(&stroke("ink", "eraser"), &document, &config("mask")).expect("the mask stroke paints"));
    assert_eq!((built.target.as_str(), built.tool.as_str()), ("mask", "eraser"));
    assert_eq!(built.brush.color, vec![0.2, 0.2, 0.2, 1.0]);
    retire(document);
}

#[test]
fn the_session_selection_clips_only_strokes_on_its_own_layer_and_target() {
    let document = document(false, true);
    let selected = RasterConfig { pixel_selection: Some(RasterPixelSelection { layer_id: "ink".into(), target: "pixels".into(), width: 4, height: 3, spans: "[[1,2,255],[6,3,128]]".into() }), ..config("pixels") };
    let built = leaf(paint_stroke_leaf(&stroke("ink", "brush"), &document, &selected).expect("the clipped stroke paints"));
    assert_eq!(built.selection, Some(vec![RasterSelectionSpan { start: 1, length: 2, coverage: 255 }, RasterSelectionSpan { start: 6, length: 3, coverage: 128 }]));
    let elsewhere = RasterConfig { pixel_selection: Some(RasterPixelSelection { layer_id: "other".into(), ..selected.pixel_selection.clone().expect("a selection") }), ..selected };
    assert_eq!(leaf(paint_stroke_leaf(&stroke("ink", "brush"), &document, &elsewhere).expect("the stroke paints")).selection, None);
    retire(document);
}

#[test]
fn a_stroke_the_layer_cannot_take_is_refused() {
    let open = document(false, true);
    for (case, document, payload, target) in [
        ("hidden", document(false, false), stroke("ink", "brush"), "pixels"),
        ("locked", document(true, true), stroke("ink", "brush"), "pixels"),
        ("missing", document(false, true), stroke("ghost", "brush"), "pixels"),
        ("a group holds no pixels", document(false, true), stroke("folder", "brush"), "pixels"),
        ("a group without a mask", document(false, true), stroke("folder", "brush"), "mask"),
        ("unknown tool", document(false, true), stroke("ink", "smudge"), "pixels"),
        ("no samples", document(false, true), PaintStroke { xs: Vec::new(), ys: Vec::new(), ..stroke("ink", "brush") }, "pixels"),
        ("ragged columns", document(false, true), PaintStroke { ys: vec![1.0], ..stroke("ink", "brush") }, "pixels"),
    ] {
        assert!(paint_stroke_leaf(&payload, &document, &config(target)).is_err(), "{case} must be refused");
        retire(document);
    }
    retire(open);
}
//#endregion 🎨️Leaf

//#region 🛠️Tool
#[test]
fn one_release_is_one_transaction_of_one_leaf_and_two_are_two() {
    let document = document(false, true);
    let mutation = paint_stroke_leaf(&stroke("ink", "brush"), &document, &config("pixels")).expect("the stroke paints");
    let (first, mutations) = raster_tool_commit(RASTER_PAINT_TOOL_ID, "seed-one", mutation.clone()).expect("the release commits");
    assert!(first.id.starts_with("tx-"), "{first:?}");
    assert_eq!(first.tool, RASTER_PAINT_TOOL_ID);
    assert_eq!(mutations, vec![mutation.clone()]);
    let (second, _) = raster_tool_commit(RASTER_PAINT_TOOL_ID, "seed-two", mutation).expect("the second release commits");
    assert_ne!(first.id, second.id);
    retire(document);
}

#[test]
fn the_handler_publishes_one_edit_stamped_with_the_transaction() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "raster-paint".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: "authoring-seed-paint".into() };
    let doc = ArtifactView::with_operation(&document, &history, operation);
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    let emit = handle(&stroke("ink", "brush"), &doc, &cfg).expect("the release publishes");
    let transaction = emit.transaction.as_ref().expect("the release is a tool transaction");
    assert_eq!(transaction.tool, RASTER_PAINT_TOOL_ID);
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(matches!(emit.artifact_mutations[0], RasterMutation::PaintStroke(_)));
    assert!(handle(&stroke("ghost", "brush"), &doc, &cfg).is_err(), "a refused stroke publishes nothing");
    retire(document);
}
//#endregion 🛠️Tool
