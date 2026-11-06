//! 🧪️ Laws of the raster selection fill: a fill is ONE edit holding ONE `fill-selection` leaf on the session target
//! with the session colour and selection; a fill the layer or the session cannot take leaves zero trace.

use semio_s_artifact_raster_raster_editor::config::RasterConfig;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;

fn receive_persistent_fixture(value:semio_framework_value::DslValue,grant:semio_framework_value::RetainedCloneGrant)->semio_s_artifact_raster_raster_model::RasterSelectionSpans{
 let mut caller=OriginalWorkFixture::new();let output=caller.receive_native(value,grant);caller.finish();output
}
use semio_framework_value::ToValue;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_plugin::{ArtifactView, ConfigView};
use semio_s_artifact_raster_raster_editor::commands::fill_selection::*;
use semio_s_artifact_raster_raster_editor::selection::RasterPixelSelection;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_plugin::HistoryView;

fn document(locked: bool, masked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    let mask = if masked { format!(r#"{{"enabled":true,"invert":false,"linked":true,"transform":{identity},"imageKey":null,"width":null,"height":null}}"#) } else { "null".to_string() };
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"fill","title":"Fill","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{mask},"width":4,"height":3,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the fill document decodes")
}

fn selected(target: &str) -> RasterConfig {
    RasterConfig { paint_target: target.into(), pixel_selection: Some(RasterPixelSelection { layer_id: "ink".into(), target: target.into(), width: 4, height: 3, spans: receive_persistent_fixture(vec![semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan { start: 1, length: 2, coverage: 255 }].to_value(),OriginalWorkFixture::stock()) }), ..RasterConfig::default() }
}

fn fill() -> FillSelection {
    FillSelection { layer_id: "ink".into() }
}

fn retire(snapshot: RasterSnapshot) {
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot);
}

#[test]
fn a_pixel_fill_is_one_leaf_with_the_session_colour_and_selection() {
    let document = document(false, false);
    let session = RasterConfig { brush_color: "#ff8000".into(), ..selected("pixels") };
    let RasterMutation::FillSelection(leaf) = fill_selection_leaf(&fill(), &document, &session).expect("the fill builds") else { panic!("a selection fill") };
    assert_eq!((leaf.layer_id.as_str(), leaf.target.as_str()), ("ink", "pixels"));
    assert_eq!(leaf.color, vec![1.0, 128.0 / 255.0, 0.0, 1.0]);
    assert_eq!(leaf.selection.as_ref().map(|selection|selection.iter().copied().collect::<Vec<_>>()), Some(vec![RasterSelectionSpan { start: 1, length: 2, coverage: 255 }]));
    assert!(std::ptr::eq(leaf.selection.as_ref().unwrap().get(0).unwrap(),session.pixel_selection.as_ref().unwrap().spans.get(0).unwrap()));
    let mut caller=OriginalWorkFixture::new();caller.retire_owned((leaf,session),OriginalWorkFixture::stock());caller.finish();
    retire(document);
}

#[test]
fn a_mask_fill_is_the_mask_value_at_the_brush_opacity() {
    let document = document(false, true);
    let session = RasterConfig { mask_value: 51, brush_opacity: 0.5, ..selected("mask") };
    let RasterMutation::FillSelection(leaf) = fill_selection_leaf(&fill(), &document, &session).expect("the mask fill builds") else { panic!("a selection fill") };
    assert_eq!(leaf.target, "mask");
    assert_eq!(leaf.color, vec![0.2, 0.2, 0.2, 0.5]);
    assert_eq!(leaf.selection.as_ref().map(|selection|selection.iter().copied().collect::<Vec<_>>()), Some(vec![RasterSelectionSpan { start: 1, length: 2, coverage: 255 }]));
    let mut caller=OriginalWorkFixture::new();caller.retire_owned(leaf,OriginalWorkFixture::stock());caller.finish();
    let mut whole=session;let selection=whole.pixel_selection.take();
    let RasterMutation::FillSelection(leaf) = fill_selection_leaf(&fill(), &document, &whole).expect("the whole-mask fill builds") else { panic!("a selection fill") };
    assert_eq!(leaf.selection, None, "without a selection the whole image fills");
    let mut caller=OriginalWorkFixture::new();caller.retire_owned((selection,whole),OriginalWorkFixture::stock());caller.finish();
    retire(document);
}

#[test]
fn a_fill_the_layer_or_the_session_cannot_take_is_refused() {
    for (case, document, session) in [
        ("locked", document(true, false), selected("pixels")),
        ("no mask", document(false, false), selected("mask")),
        ("invalid colour", document(false, false), RasterConfig { brush_color: "orange".into(), ..selected("pixels") }),
    ] {
        assert!(fill_selection_leaf(&fill(), &document, &session).is_err(), "{case} must be refused");
        let mut caller=OriginalWorkFixture::new();caller.retire_owned(session,OriginalWorkFixture::stock());caller.finish();
        retire(document);
    }
    let document = document(false, false);
    assert!(fill_selection_leaf(&FillSelection { layer_id: "ghost".into() }, &document, &RasterConfig::default()).is_err(), "a missing layer is refused");
    retire(document);
}

#[test]
fn the_handler_publishes_one_plain_edit() {
    let document = document(false, false);
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let session = RasterConfig::default();
    let cfg = ConfigView { snapshot: &session, window: None };
    let emit = handle(&fill(), &doc, &cfg).expect("the fill publishes");
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(matches!(emit.artifact_mutations[0], RasterMutation::FillSelection(_)));
    assert!(handle(&FillSelection { layer_id: "ghost".into() }, &doc, &cfg).is_err(), "a refused fill publishes nothing");
    retire(document);
}
