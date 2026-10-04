//! 🧪️ Laws of the raster grid change: a rotation, resize or crop is ONE edit holding ONE `transform-image` leaf on the
//! layer's pixels; a change the layer, the session or the leaf cannot take leaves zero trace.

use super::*;
use semio_framework_plugin::HistoryView;

fn document(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"grid","title":"Grid","layers":[{{"kind":"pixel","id":"ink","name":"Ink","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"width":4,"height":3,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the grid document decodes")
}

fn change(operation: &str, x: u32, y: u32, width: u32, height: u32, bilinear: bool) -> TransformImage {
    TransformImage { layer_id: "ink".into(), operation: operation.into(), x, y, width, height, bilinear }
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

#[test]
fn a_grid_change_is_one_leaf_naming_the_operation_and_its_extent() {
    let document = document(false);
    for payload in [change("rotateClockwise", 0, 0, 0, 0, false), change("resize", 0, 0, 8, 6, true), change("crop", 1, 1, 2, 2, false)] {
        let RasterMutation::TransformImage(leaf) = transform_image_leaf(&payload, &document, &RasterConfig::default()).expect("the grid change builds") else { panic!("an image transform") };
        assert_eq!((leaf.layer_id.as_str(), leaf.operation.as_str(), leaf.x, leaf.y, leaf.width, leaf.height, leaf.bilinear), ("ink", payload.operation.as_str(), payload.x, payload.y, payload.width, payload.height, payload.bilinear));
    }
    retire(document);
}

#[test]
fn a_grid_change_the_layer_the_session_or_the_leaf_cannot_take_is_refused() {
    for (case, document, payload) in [
        ("locked", document(true), change("rotateClockwise", 0, 0, 0, 0, false)),
        ("missing", document(false), TransformImage { layer_id: "ghost".into(), ..change("rotateClockwise", 0, 0, 0, 0, false) }),
        ("unknown operation", document(false), change("shear", 0, 0, 0, 0, false)),
        ("empty resize", document(false), change("resize", 0, 0, 0, 6, false)),
        ("oversized crop", document(false), change("crop", 0, 0, 16_385, 2, false)),
        ("turn with an extent", document(false), change("rotateCounterclockwise", 0, 0, 2, 2, false)),
        ("resize with an origin", document(false), change("resize", 1, 0, 2, 2, false)),
        ("sampled crop", document(false), change("crop", 0, 0, 2, 2, true)),
    ] {
        assert!(transform_image_leaf(&payload, &document, &RasterConfig::default()).is_err(), "{case} must be refused");
        retire(document);
    }
    let document = document(false);
    let masking = RasterConfig { paint_target: "mask".into(), ..RasterConfig::default() };
    assert!(transform_image_leaf(&change("rotateClockwise", 0, 0, 0, 0, false), &document, &masking).is_err(), "a session painting the mask is refused");
    retire(document);
}

#[test]
fn the_handler_publishes_one_plain_edit() {
    let document = document(false);
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let session = RasterConfig::default();
    let cfg = ConfigView { snapshot: &session, window: None };
    let emit = handle(&change("rotateClockwise", 0, 0, 0, 0, false), &doc, &cfg).expect("the grid change publishes");
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(matches!(emit.artifact_mutations[0], RasterMutation::TransformImage(_)));
    assert!(handle(&change("shear", 0, 0, 0, 0, false), &doc, &cfg).is_err(), "a refused grid change publishes nothing");
    retire(document);
}
