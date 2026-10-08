//! 🧪️ `transform-image` laws: the committed quintets are a print of this leaf (and of the ONE shared pixel engine); the
//! layer keeps its place and scale (a crop keeps its window, a quarter turn swaps the display scale); a transform's
//! inverse restores the exact prior image, extent and placement; a transform edited in history re-derives itself and its
//! downstream; the store folds it like the leaf.

use super::*;
use crate::mutations::paint_stroke::{paint_stroke, RasterBrush, RasterStrokePoint};
use crate::mutations::{apply_raster_mutation, inverse_raster_mutation};
use crate::standards::v1::subsets::any::schema::find_layer;
use crate::{RasterLayerNode, SemioImageSnapshot, RASTER_DOCUMENT_SCHEMA};
use protocol::Mutation;

//#region 🧫️Cases
fn base(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"transform","title":"Transform","layers":[{{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":null,"width":6,"height":4,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the transform base decodes")
}

/// 🧱️ A red wall down column 2 of the blank layer, so every grid change moves something visible.
fn walled() -> RasterMutation {
    paint_stroke("paint", "pixels", "brush", RasterBrush { size: 1.0, hardness: 1.0, opacity: 1.0, color: vec![1.0, 0.0, 0.0, 1.0] }, vec![RasterStrokePoint { x: 2.5, y: 0.0 }, RasterStrokePoint { x: 2.5, y: 4.0 }])
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

fn walled_base() -> RasterSnapshot {
    let blank = base(false);
    let walled_document = apply_raster_mutation(&blank, &walled()).expect("the wall paints");
    retire(blank);
    walled_document
}

/// 🧫️ Every committed scenario: its directory, its before-document and its transform.
fn cases() -> Vec<(&'static str, RasterSnapshot, RasterMutation)> {
    vec![
        ("↪️rotates-right", walled_base(), rotate_image("paint", true)),
        ("↩️rotates-left", walled_base(), rotate_image("paint", false)),
        ("📏️resizes", walled_base(), resize_image("paint", 12, 8, false)),
        ("🫧️smooths", walled_base(), resize_image("paint", 3, 2, true)),
        ("✂️crops", walled_base(), crop_image("paint", 1, 1, 3, 2)),
        ("📍️outside", walled_base(), crop_image("paint", 4, 0, 3, 2)),
        ("🔒️locked", base(true), rotate_image("paint", true)),
        ("🚫️rejects", base(false), rotate_image("ghost", true)),
    ]
}

fn json(value: &impl semio_framework_value::ToValue) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("a value prints as JSON")
}

/// 🎯️ The committed outcome of one transform: applied, or a refusal naming its code and target.
fn outcome_json(outcome: &protocol::MutationOutcome<RasterDiff>) -> serde_json::Value {
    let refusal = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    if let Some(message) = refusal {
        return serde_json::json!({ "status": "rejected", "code": message.code.0, "path": message.target });
    }
    let messages: Vec<serde_json::Value> = outcome.messages().iter().map(|message| serde_json::json!({ "level": json(&message.level), "code": message.code.0 })).collect();
    match messages.is_empty() {
        true => serde_json::json!({ "status": "applied" }),
        false => serde_json::json!({ "status": "applied", "messages": messages }),
    }
}

/// 🖨️ The five files of one scenario as this leaf computes them: before, after, mutation, diff, outcome.
fn quintet(before: &RasterSnapshot, mutation: &RasterMutation) -> [(&'static str, serde_json::Value); 5] {
    let outcome = mutation.diff(before);
    let after = protocol::apply_diff(outcome.diff(), before).expect("every committed transform diff applies");
    let files = [
        ("📸️snapshot/⬅️before/🔣️.json", json(before)),
        ("📸️snapshot/➡️after/🔣️.json", json(&after)),
        ("🦠️mutation/🔣️.json", json(mutation)),
        ("🔺️diff/🔣️.json", json(outcome.diff())),
        ("🎯️outcome/🔣️.json", outcome_json(&outcome)),
    ];
    retire(after);
    files
}

fn fixture_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🔄️transform-image")
}

/// 🧫️ Regenerates the committed quintets from this leaf. Ignored by default: it WRITES into the source tree, which is
/// the point — the committed files are a print of this table and never hand-edited.
#[test]
#[ignore]
fn emit_committed_fixtures() {
    for (case, before, mutation) in cases() {
        for (file, value) in quintet(&before, &mutation) {
            let path = fixture_root().join(case).join(file);
            std::fs::create_dir_all(path.parent().expect("a fixture file has a parent")).expect("fixture directory");
            std::fs::write(&path, serde_json::to_string_pretty(&value).expect("fixture JSON") + "\n").expect("fixture write");
        }
        retire(before);
    }
}

/// 🧾️ Every committed quintet is exactly what this leaf computes from its committed before-document and mutation.
#[test]
fn the_committed_quintets_are_a_print_of_the_leaf() {
    for (case, before, mutation) in cases() {
        let committed_before: RasterSnapshot = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(fixture_root().join(case).join("📸️snapshot/⬅️before/🔣️.json")).expect("the committed before exists"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed before decodes");
        let committed_mutation: RasterMutation = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(fixture_root().join(case).join("🦠️mutation/🔣️.json")).expect("the committed mutation exists"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed mutation decodes");
        assert_eq!(committed_mutation, mutation, "{case}: the committed mutation is the table's");
        for (file, value) in quintet(&committed_before, &committed_mutation) {
            let committed: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(fixture_root().join(case).join(file)).expect("the committed file exists")).expect("the committed file is JSON");
            assert_eq!(value, committed, "{case}/{file}");
        }
        retire(committed_before);
        retire(before);
    }
}
//#endregion 🧫️Cases

//#region 🔄️Transform
fn image(document: &RasterSnapshot) -> (Vec<u8>, u32, u32) {
    let Some(RasterLayerNode::Pixel { image_key: Some(key), .. }) = find_layer(&document.layers, "paint") else { panic!("the layer shows an image") };
    let image = document.assets.get(key).and_then(|child| child.local_owner::<SemioImageSnapshot>()).expect("the image is materialized");
    (image.frames[0].rgba8.clone(), image.width, image.height)
}

fn grid(document: &RasterSnapshot) -> (Option<u32>, Option<u32>, RasterTransform) {
    let Some(RasterLayerNode::Pixel { width, height, transform, .. }) = find_layer(&document.layers, "paint") else { panic!("a pixel layer") };
    (*width, *height, transform.clone())
}

/// 🔄️ The transformed image is exactly what the ONE shared pixel engine computes: the leaf adds no pixel math.
#[test]
fn a_transform_runs_exactly_what_the_shared_engine_does() {
    let before = walled_base();
    let (pixels, width, height) = image(&before);
    for (mutation, operation) in [
        (rotate_image("paint", true), PixelOperation::RotateClockwise),
        (rotate_image("paint", false), PixelOperation::RotateCounterclockwise),
        (resize_image("paint", 12, 8, true), PixelOperation::Resize { width: 12, height: 8, bilinear: true }),
        (crop_image("paint", 1, 1, 3, 2), PixelOperation::Crop { x: 1, y: 1, width: 3, height: 2 }),
    ] {
        let after = apply_raster_mutation(&before, &mutation).expect("the transform applies");
        let mut job = PixelEditJob::new(RasterImage { width, height, pixels: pixels.clone() }, operation, None).expect("the engine admits the transform");
        while !job.advance(4096).expect("the engine advances").done {}
        let expected = job.into_result().expect("the engine completes");
        assert_eq!(image(&after), (expected.pixels, expected.width, expected.height), "{mutation:?}");
        retire(after);
    }
    retire(before);
}

/// 📐️ The layer keeps its place and scale: the display extent becomes the pixel extent, a crop keeps its window's centre
/// where it was, and a quarter turn of a non-uniformly displayed image swaps the display scale onto the turned axes.
#[test]
fn the_layer_keeps_its_place_and_scale() {
    let before = walled_base();
    let cropped = apply_raster_mutation(&before, &crop_image("paint", 1, 1, 3, 2)).expect("the crop applies");
    assert_eq!(grid(&cropped), (Some(3), Some(2), RasterTransform { x: -0.5, y: 0.0, a: 1.0, b: 0.0, c: 0.0, d: 1.0 }));
    let rotated = apply_raster_mutation(&before, &rotate_image("paint", true)).expect("the turn applies");
    assert_eq!(grid(&rotated), (Some(4), Some(6), RasterTransform { x: 0.0, y: 0.0, a: 1.0, b: 0.0, c: 0.0, d: 1.0 }));
    let stretched = apply_raster_mutation(&before, &RasterMutation::ResizeLayer(crate::mutations::resize_layer::ResizeLayer { layer_id: "paint".into(), new_width: 12, new_height: 4 })).expect("the display stretches");
    let turned = apply_raster_mutation(&stretched, &rotate_image("paint", true)).expect("the stretched turn applies");
    assert_eq!(grid(&turned), (Some(4), Some(6), RasterTransform { x: 0.0, y: 0.0, a: 1.0, b: 0.0, c: 0.0, d: 2.0 }), "the doubled width scale moves onto the turned image's height");
    for document in [before, cropped, rotated, stretched, turned] {
        retire(document);
    }
}

/// ↩️ Transforming, then undoing the transform, restores the exact prior image, extent and placement.
#[test]
fn the_inverse_restores_the_exact_prior_image_extent_and_placement() {
    let before = walled_base();
    for mutation in [rotate_image("paint", true), resize_image("paint", 12, 8, false), crop_image("paint", 1, 1, 3, 2)] {
        let transformed = apply_raster_mutation(&before, &mutation).expect("the transform applies");
        let mut restored = transformed.clone();
        for undo in inverse_raster_mutation(&before, &mutation).expect("valid retained mutation inverse fixture") {
            let next = apply_raster_mutation(&restored, &undo).expect("every undo step applies");
            retire(std::mem::replace(&mut restored, next));
        }
        assert_eq!(restored, before, "undoing {mutation:?} lands on the walled document");
        retire(transformed);
        retire(restored);
    }
    retire(before);
}

/// ⏪️ A transform edited in history re-derives itself from its base, and the downstream transform replays on it.
#[test]
fn an_edited_transform_rederives_itself_and_its_downstream() {
    let first = rotate_image("paint", true);
    let edited = resize_image("paint", 3, 2, false);
    let downstream = crop_image("paint", 0, 0, 2, 2);
    let fold = |log: &[&RasterMutation]| {
        let mut document = walled_base();
        for mutation in log {
            let next = apply_raster_mutation(&document, mutation).expect("the log folds");
            retire(std::mem::replace(&mut document, next));
        }
        document
    };
    let original = fold(&[&first, &downstream]);
    let rewritten = fold(&[&edited, &downstream]);
    assert_ne!(original, rewritten, "the edited transform changes the head");
    let walled_document = walled_base();
    let mut stepwise = apply_raster_mutation(&walled_document, &edited).expect("the edited transform applies on its base");
    let next = apply_raster_mutation(&stepwise, &downstream).expect("the downstream crop replays on the edited one");
    retire(std::mem::replace(&mut stepwise, next));
    assert_eq!(rewritten, stepwise);
    for document in [walled_document, original, rewritten, stepwise] {
        retire(document);
    }
}

/// 🏷️ A transform row reads in English and German from the leaf.
#[test]
fn the_transform_labels_itself_in_english_and_german() {
    for (mutation, en, de) in [
        (rotate_image("paint", false), "Rotate layer paint left", "Ebene paint nach links drehen"),
        (crop_image("paint", 1, 2, 3, 4), "Crop layer paint to 3 × 4 at (1, 2)", "Ebene paint auf 3 × 4 bei (1, 2) zuschneiden"),
    ] {
        let RasterMutation::TransformImage(transform) = mutation else { unreachable!() };
        let label = protocol::MutationKind::<RasterSnapshot, RasterMutation>::label(&transform);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), en);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), de);
    }
}

/// 🧯️ A payload the schema refuses is a Fatal invariant breach naming the field, and moves nothing.
#[test]
fn a_payload_outside_its_bounds_is_an_invariant_breach() {
    let document = walled_base();
    let shaped = |operation: &str, x: u32, y: u32, width: u32, height: u32, bilinear: bool| RasterMutation::TransformImage(TransformImage { layer_id: "paint".into(), operation: operation.into(), x, y, width, height, bilinear });
    for (field, transform) in [
        ("operation", shaped("skew", 0, 0, 0, 0, false)),
        ("width", shaped("resize", 0, 0, 0, 4, false)),
        ("height", shaped("crop", 0, 0, 2, 16_385, false)),
        ("x", shaped("resize", 1, 0, 2, 2, false)),
        ("x", shaped("rotateClockwise", 0, 1, 0, 0, false)),
        ("width", shaped("rotateCounterclockwise", 0, 0, 2, 0, false)),
        ("bilinear", shaped("crop", 0, 0, 2, 2, true)),
    ] {
        let outcome = transform.diff(&document);
        assert_eq!(outcome.diff(), &RasterDiff::default());
        let message = outcome.messages().first().expect("a breach raises");
        assert_eq!((message.level, message.code.0.as_str(), message.target.as_slice()), (semio_framework_diagnostic::Severity::Fatal, "mutation.invariant", [field.to_string()].as_slice()), "{transform:?}");
        assert!(transform.inverse(&document).expect("valid retained mutation inverse fixture").is_empty());
    }
    retire(document);
}
//#endregion 🔄️Transform

//#region 🏪️Store
/// 🏪️ The store folds a transform through the artifact's own retained owners exactly like the leaf's diff.
#[semio_framework_async_macros::async_test]
async fn the_store_folds_a_transform_like_the_leaf() {
    let transform = crop_image("paint", 1, 0, 4, 3);
    let mut store = crate::mutations::RasterStore::new(store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "transform-image", walled_base(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(crate::host::owned::raster_document_store_owners());
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![transform.clone()], transaction: None }).await.expect("the transform applies");
    let before = walled_base();
    let expected = apply_raster_mutation(&before, &transform).expect("the leaf applies");
    let projected = store.snapshot().expect("the store projects");
    assert_eq!(projected, expected);
    retire(projected);
    retire(expected);
    retire(before);
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🏪️Store
