//! 🧪️ Laws of the raster bucket tool: a click is ONE transaction holding ONE `fill-region` leaf built from the session
//! colour, target, mask value and selection, its seed the clicked pixel; a click the layer cannot take leaves zero trace.

use super::*;
use crate::editor::raster::config::RasterPixelSelection;
use crate::mutations::paint_stroke::RasterSelectionSpan;
use semio_framework_plugin::{AppOperationContext, HistoryView};

fn document(locked: bool, visible: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"fill","title":"Fill","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":{visible},"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":4,"height":3,"imageKey":null,"locked":{locked}}},{{"kind":"group","id":"folder","name":"Folder","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"children":[]}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the fill document decodes")
}

fn config(target: &str) -> RasterConfig {
    RasterConfig { brush_color: "#ff8000".into(), paint_target: target.into(), mask_value: 51, ..RasterConfig::default() }
}

fn click(layer: &str, x: f64, y: f64) -> FillRegion {
    FillRegion { layer_id: layer.into(), x, y }
}

fn leaf(mutation: RasterMutation) -> FillRegionLeaf {
    let RasterMutation::FillRegion(leaf) = mutation else { panic!("a region fill") };
    leaf
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

//#region 🪣️Leaf
#[test]
fn a_click_fills_with_the_session_colour_and_tolerance_from_the_clicked_pixel() {
    let document = document(false, true);
    let built = leaf(fill_region_leaf(&click("ink", 2.75, 1.25), &document, &RasterConfig { fill_tolerance: 40, ..config("pixels") }).expect("the click fills"));
    assert_eq!((built.layer_id.as_str(), built.target.as_str()), ("ink", "pixels"));
    assert_eq!(built.seed, RasterSeed { x: 2, y: 1 });
    assert_eq!(built.tolerance, 40);
    assert_eq!(built.color, vec![1.0, 128.0 / 255.0, 0.0, 1.0]);
    assert_eq!(built.selection, None);
    retire(document);
}

#[test]
fn a_mask_fill_fills_the_session_mask_value_as_its_grey_level() {
    let document = document(false, true);
    let built = leaf(fill_region_leaf(&click("ink", 0.0, 0.0), &document, &config("mask")).expect("the mask fill fills"));
    assert_eq!(built.target, "mask");
    assert_eq!(built.color, vec![0.2, 0.2, 0.2, 1.0]);
    retire(document);
}

#[test]
fn the_session_selection_clips_only_fills_on_its_own_layer_and_target() {
    let document = document(false, true);
    let selected = RasterConfig { pixel_selection: Some(RasterPixelSelection { layer_id: "ink".into(), target: "pixels".into(), width: 4, height: 3, spans: vec![crate::RasterSelectionSpan { start: 1, length: 2, coverage: 255 }, crate::RasterSelectionSpan { start: 6, length: 3, coverage: 128 }] }), ..config("pixels") };
    let built = leaf(fill_region_leaf(&click("ink", 1.0, 0.0), &document, &selected).expect("the clipped fill fills"));
    assert_eq!(built.selection, Some(vec![RasterSelectionSpan { start: 1, length: 2, coverage: 255 }, RasterSelectionSpan { start: 6, length: 3, coverage: 128 }]));
    retire(document);
}

#[test]
fn a_click_the_layer_cannot_take_is_refused() {
    for (case, document, payload, session) in [
        ("hidden", document(false, false), click("ink", 1.0, 1.0), config("pixels")),
        ("locked", document(true, true), click("ink", 1.0, 1.0), config("pixels")),
        ("missing", document(false, true), click("ghost", 1.0, 1.0), config("pixels")),
        ("a group holds no pixels", document(false, true), click("folder", 1.0, 1.0), config("pixels")),
        ("a group without a mask", document(false, true), click("folder", 1.0, 1.0), config("mask")),
        ("off the grid", document(false, true), click("ink", -1.0, 1.0), config("pixels")),
        ("not a number", document(false, true), click("ink", f64::NAN, 1.0), config("pixels")),
        ("tolerance past 255", document(false, true), click("ink", 1.0, 1.0), RasterConfig { fill_tolerance: 256, ..config("pixels") }),
    ] {
        assert!(fill_region_leaf(&payload, &document, &session).is_err(), "{case} must be refused");
        retire(document);
    }
}
//#endregion 🪣️Leaf

//#region 🛠️Tool
#[test]
fn the_handler_publishes_one_edit_stamped_with_the_bucket_transaction() {
    let document = document(false, true);
    let history = HistoryView::empty();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "raster-fill".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: "authoring-seed-fill".into() };
    let doc = ArtifactView::with_operation(&document, &history, operation);
    let session = config("pixels");
    let cfg = ConfigView { snapshot: &session, window: None };
    let emit = handle(&click("ink", 1.0, 1.0), &doc, &cfg).expect("the click publishes");
    let transaction = emit.transaction.as_ref().expect("the click is a tool transaction");
    assert_eq!(transaction.tool, RASTER_FILL_TOOL_ID);
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(matches!(emit.artifact_mutations[0], RasterMutation::FillRegion(_)));
    assert!(handle(&click("ghost", 1.0, 1.0), &doc, &cfg).is_err(), "a refused click publishes nothing");
    retire(document);
}
//#endregion 🛠️Tool
