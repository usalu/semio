//! 🧪️ Laws of the raster menu filter: a filter is ONE edit holding ONE `apply-filter` leaf on the layer's pixels with the
//! session selection (a flip mirrors the whole image); a filter the layer or the session cannot take leaves zero trace.

use semio_s_artifact_raster_raster_editor::config::RasterConfig;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;

fn receive_persistent_fixture(value:semio_framework_value::DslValue,grant:semio_framework_value::RetainedCloneGrant)->semio_s_artifact_raster_raster_model::RasterSelectionSpans{
 let mut caller=OriginalWorkFixture::new();let output=caller.receive_native(value,grant);caller.finish();output
}
use semio_framework_value::ToValue;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_plugin::{ArtifactView, ConfigView};
use semio_s_artifact_raster_raster_editor::commands::apply_filter::*;
use semio_s_artifact_raster_raster_editor::selection::RasterPixelSelection;
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_plugin::HistoryView;

fn document(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"filter","title":"Filter","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"width":4,"height":3,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the filter document decodes")
}

fn selected() -> RasterConfig {
    RasterConfig { pixel_selection: Some(RasterPixelSelection { layer_id: "ink".into(), target: "pixels".into(), width: 4, height: 3, spans: receive_persistent_fixture(vec![semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan { start: 1, length: 2, coverage: 255 }].to_value(),OriginalWorkFixture::stock()) }), ..RasterConfig::default() }
}

fn filter(name: &str, amount: f64) -> ApplyFilter {
    ApplyFilter { layer_id: "ink".into(), filter: name.into(), amount }
}

fn retire(snapshot: RasterSnapshot) {
    semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(snapshot);
}

#[test]
fn a_filter_is_one_leaf_on_the_pixels_with_the_session_selection() {
    let document = document(false);
    let session=selected();
    let RasterMutation::ApplyFilter(leaf) = apply_filter_leaf(&filter("blur", 2.0), &document, &session).expect("the filter builds") else { panic!("an image filter") };
    assert_eq!((leaf.layer_id.as_str(), leaf.filter.as_str(), leaf.amount), ("ink", "blur", 2.0));
    assert_eq!(leaf.selection.as_ref().map(|selection|selection.iter().copied().collect::<Vec<_>>()), Some(vec![RasterSelectionSpan { start: 1, length: 2, coverage: 255 }]));
    assert!(std::ptr::eq(leaf.selection.as_ref().unwrap().get(0).unwrap(),session.pixel_selection.as_ref().unwrap().spans.get(0).unwrap()));
    let RasterMutation::ApplyFilter(flip) = apply_filter_leaf(&filter("flipVertical", 0.0), &document, &session).expect("the flip builds") else { panic!("an image filter") };
    assert_eq!(flip.selection, None, "a flip mirrors the whole image");
    let mut caller=OriginalWorkFixture::new();caller.retire_owned((leaf,session),OriginalWorkFixture::stock());caller.finish();
    retire(document);
}

#[test]
fn a_filter_the_layer_or_the_leaf_cannot_take_is_refused() {
    for (case, document, payload) in [
        ("locked", document(true), filter("invert", 0.0)),
        ("missing", document(false), ApplyFilter { layer_id: "ghost".into(), ..filter("invert", 0.0) }),
        ("unknown filter", document(false), filter("emboss", 0.0)),
        ("amount out of range", document(false), filter("brightness", 2.0)),
        ("fractional radius", document(false), filter("blur", 0.5)),
    ] {
        assert!(apply_filter_leaf(&payload, &document, &RasterConfig::default()).is_err(), "{case} must be refused");
        retire(document);
    }
    let document = document(false);
    let masking = RasterConfig { paint_target: "mask".into(), ..RasterConfig::default() };
    assert!(apply_filter_leaf(&filter("invert", 0.0), &document, &masking).is_err(), "a session painting the mask is refused");
    retire(document);
}

#[test]
fn the_handler_publishes_one_plain_edit() {
    let document = document(false);
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let session = RasterConfig::default();
    let cfg = ConfigView { snapshot: &session, window: None };
    let emit = handle(&filter("invert", 0.0), &doc, &cfg).expect("the filter publishes");
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(matches!(emit.artifact_mutations[0], RasterMutation::ApplyFilter(_)));
    assert!(handle(&filter("emboss", 0.0), &doc, &cfg).is_err(), "a refused filter publishes nothing");
    retire(document);
}
