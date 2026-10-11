//! 🧪️ Laws of the raster paint tool: a released stroke is ONE transaction holding ONE `paint-stroke` leaf built from the
//! session brush, target, mask value and selection; a stroke the layer cannot take leaves zero trace; two releases are
//! two transactions; a streamed stroke is one transaction of every sample, previewed through the one rasterizer, ended
//! with zero trace by a cancel, and deaf to the late ticks of a closed press.

use super::*;
use crate::editor::raster::config::RasterPixelSelection;
use semio_framework_plugin::{AppOperationContext, HistoryView};

fn document(locked: bool, visible: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"paint","title":"Paint","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":{visible},"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":4,"height":3,"imageKey":null,"locked":{locked}}},{{"kind":"group","id":"folder","name":"Folder","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"children":[]}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the paint document decodes")
}

fn config(target: &str) -> RasterConfig {
    RasterConfig { brush_size: 4.0, brush_opacity: 0.5, brush_color: "#ff8000".into(), brush_hardness: 0.25, paint_target: target.into(), mask_value: 51, ..RasterConfig::default() }
}

fn stroke(layer: &str, tool: &str) -> PaintStroke {
    PaintStroke { layer_id: layer.into(), tool: tool.into(), xs: vec![0.5, 3.5], ys: vec![1.5, 1.5], phase: None, reason: None, gesture: None }
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
    let selected = RasterConfig { pixel_selection: Some(RasterPixelSelection { layer_id: "ink".into(), target: "pixels".into(), width: 4, height: 3, spans: vec![crate::mutations::paint_stroke::RasterSelectionSpan { start: 1, length: 2, coverage: 255 }, crate::mutations::paint_stroke::RasterSelectionSpan { start: 6, length: 3, coverage: 128 }] }), ..config("pixels") };
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

//#region 🌊️Stream
fn tick(phase: &str, gesture: &str, xs: &[f64], ys: &[f64]) -> PaintStroke {
    PaintStroke { layer_id: "ink".into(), tool: "brush".into(), xs: xs.to_vec(), ys: ys.to_vec(), phase: Some(phase.into()), reason: None, gesture: Some(gesture.into()) }
}

fn operation(seed: &str) -> AppOperationContext {
    AppOperationContext { app_instance_id: 1, parent_document_id: "raster-paint".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: seed.into() }
}

#[test]
fn a_streamed_stroke_is_one_transaction_holding_every_sample_in_order() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&document, &history, operation("seed-stream"));
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    let (first, open) = handle_in_window(&tick("stream", "press-1", &[0.5, 1.5], &[0.5, 0.5]), &doc, &cfg, &RasterCompositeWindowTransient::default()).expect("the first tick opens the stroke");
    assert!(first.artifact_mutations.is_empty() && first.transaction.is_none(), "a tick publishes nothing on the document");
    let opened = open.stroke.as_deref().map(|state| (state.gesture.clone(), state.transaction.clone())).expect("the stroke is in flight");
    assert_eq!(opened.0, "press-1");
    let (_, open) = handle_in_window(&tick("stream", "press-1", &[2.5, 2.5], &[1.5, 1.5]), &doc, &cfg, &open).expect("a tick extends the stroke");
    assert_eq!(open.stroke.as_deref().map(|state| &state.transaction), Some(&opened.1), "every tick names the transaction the first one opened");
    let (committed, closed) = handle_in_window(&tick("commit", "press-1", &[3.5], &[2.5]), &doc, &cfg, &open).expect("the release commits");
    assert_eq!(committed.transaction.as_ref(), Some(&opened.1), "the release commits the transaction the ticks streamed");
    let once = paint_stroke_leaf(&PaintStroke { xs: vec![0.5, 1.5, 2.5, 3.5], ys: vec![0.5, 0.5, 1.5, 2.5], ..stroke("ink", "brush") }, &document, &session).expect("the whole stroke paints");
    assert_eq!(committed.artifact_mutations, vec![once], "the streamed stroke is the one-shot stroke of the same samples, a repeated sample once");
    assert_eq!(closed, RasterCompositeWindowTransient { stroke: None, closed: Some("press-1".into()) });
    let (late, unchanged) = handle_in_window(&tick("stream", "press-1", &[0.5], &[2.5]), &doc, &cfg, &closed).expect("a late tick is dropped");
    assert!(late.artifact_mutations.is_empty());
    assert_eq!(unchanged, closed, "a late tick of a closed press leaves zero trace");
    retire(document);
}

#[test]
fn the_window_previews_its_open_stroke_through_the_one_rasterizer() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&document, &history, operation("seed-preview"));
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    let (_, open) = handle_in_window(&tick("stream", "press-1", &[0.5, 3.5], &[1.5, 1.5]), &doc, &cfg, &RasterCompositeWindowTransient::default()).expect("the tick opens the stroke");
    let preview = raster_stroke_preview(&document, &open).expect("the open stroke previews");
    let leaf: RasterMutation = semio_framework_value::FromValue::from_value(open.stroke.as_deref().expect("in flight").stroke.clone()).expect("the provisional leaf decodes");
    let painted = crate::standards::v1::subsets::any::io::text::mutations::apply_raster_mutation(&document, &leaf).expect("the provisional leaf paints");
    assert_eq!(preview, painted, "the preview is the provisional leaf through the one rasterizer");
    assert_ne!(preview, document, "the preview shows a stroke the document does not hold");
    assert!(raster_stroke_preview(&document, &RasterCompositeWindowTransient::default()).is_none(), "a window at rest previews nothing");
    for snapshot in [preview, painted, document] {
        retire(snapshot);
    }
}

#[test]
fn a_cancel_or_another_press_ends_the_open_stroke_with_zero_trace() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&document, &history, operation("seed-cancel"));
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    let rest = RasterCompositeWindowTransient::default();
    let (_, open) = handle_in_window(&tick("stream", "press-2", &[0.5], &[0.5]), &doc, &cfg, &rest).expect("the tick opens the stroke");
    let (aborted, closed) = handle_in_window(&PaintStroke { reason: Some("blur".into()), ..tick("abort", "press-2", &[], &[]) }, &doc, &cfg, &open).expect("the cancel ends the stroke");
    assert!(aborted.artifact_mutations.is_empty() && aborted.transaction.is_none());
    assert_eq!(closed, RasterCompositeWindowTransient { stroke: None, closed: Some("press-2".into()) });
    let (late, unchanged) = handle_in_window(&tick("commit", "press-2", &[1.5], &[0.5]), &doc, &cfg, &closed).expect("a late release is dropped");
    assert!(late.artifact_mutations.is_empty());
    assert_eq!(unchanged, closed);
    let (_, open) = handle_in_window(&tick("stream", "press-3", &[0.5], &[0.5]), &doc, &cfg, &rest).expect("the tick opens the stroke");
    let (_, host) = handle_in_window(&PaintStroke { layer_id: String::new(), tool: String::new(), xs: Vec::new(), ys: Vec::new(), phase: Some("abort".into()), reason: Some("captureLost".into()), gesture: None }, &doc, &cfg, &open).expect("a host fact ends the open stroke");
    assert_eq!(host, RasterCompositeWindowTransient { stroke: None, closed: Some("press-3".into()) }, "a host abort closes whatever press is open");
    let (_, first) = handle_in_window(&tick("stream", "press-4", &[0.5], &[0.5]), &doc, &cfg, &rest).expect("the tick opens the stroke");
    let next = ArtifactView::with_operation(&document, &history, operation("seed-cancel-next"));
    let (_, second) = handle_in_window(&tick("stream", "press-5", &[3.5], &[2.5]), &next, &cfg, &first).expect("another press takes over");
    let state = second.stroke.as_deref().expect("the new press is in flight");
    assert_eq!(state.gesture, "press-5");
    assert_ne!(Some(&state.transaction), first.stroke.as_deref().map(|state| &state.transaction), "the interrupted press's transaction is gone");
    let RasterMutation::PaintStroke(leaf) = semio_framework_value::FromValue::from_value(state.stroke.clone()).expect("the leaf decodes") else { panic!("a paint stroke") };
    assert_eq!(leaf.points, vec![RasterStrokePoint { x: 3.5, y: 2.5 }], "the new press holds only its own samples");
    retire(document);
}

#[test]
fn streamed_phases_need_a_press_and_a_window() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&document, &history, operation("seed-phase"));
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    assert!(handle(&tick("stream", "press-6", &[0.5], &[0.5]), &doc, &cfg).is_err(), "a stream without a window is refused");
    assert!(handle_in_window(&PaintStroke { gesture: None, ..tick("stream", "press-6", &[0.5], &[0.5]) }, &doc, &cfg, &RasterCompositeWindowTransient::default()).is_err(), "a stream without a press is refused");
    assert!(handle_in_window(&tick("smear", "press-6", &[0.5], &[0.5]), &doc, &cfg, &RasterCompositeWindowTransient::default()).is_err(), "an unknown phase is refused");
    assert!(handle(&PaintStroke { phase: Some("abort".into()), ..stroke("ink", "brush") }, &doc, &cfg).expect("an abort without a window ends nothing").artifact_mutations.is_empty());
    retire(document);
}
//#endregion 🌊️Stream
