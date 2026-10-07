//! 🧪️ `paint-stroke` laws: the committed quintets are a print of this leaf (and of the ONE shared rasterizer the TypeScript
//! twin verifies on the shared pixel corpus); a stroke's inverse restores the exact prior image; a stroke edited in history
//! re-derives its pixels — and its downstream — from its base; the store folds it exactly like the leaf.

use super::*;
use crate::mutations::{apply_raster_mutation, inverse_raster_mutation};
use crate::{RasterSnapshot, RASTER_DOCUMENT_SCHEMA};
use protocol::{Mutation, MutationDiff};

//#region 🧫️Cases
fn base(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"stroke","title":"Stroke","layers":[{{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":6,"height":4,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the stroke base decodes")
}

fn brush(size: f64, hardness: f64, opacity: f64, color: [f64; 4]) -> RasterBrush {
    RasterBrush { size, hardness, opacity, color: color.to_vec() }
}

fn points(points: &[(f64, f64)]) -> Vec<RasterStrokePoint> {
    points.iter().map(|(x, y)| RasterStrokePoint { x: *x, y: *y }).collect()
}

/// ✂️ `stroke` clipped to the selection runs `(start, length, coverage)`.
fn clipped(stroke: RasterMutation, spans: &[(u32, u32, u32)]) -> RasterMutation {
    let RasterMutation::PaintStroke(stroke) = stroke else { unreachable!("a paint stroke") };
    RasterMutation::PaintStroke(PaintStroke { selection: Some(spans.iter().map(|(start, length, coverage)| RasterSelectionSpan { start: *start, length: *length, coverage: *coverage }).collect()), ..stroke })
}

/// 🧫️ Every committed scenario: its directory, its before-document and its stroke.
fn cases() -> Vec<(&'static str, RasterSnapshot, RasterMutation)> {
    vec![
        ("🖌️paints", base(false), paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [1.0, 0.0, 0.0, 1.0]), points(&[(0.5, 0.5), (3.5, 1.5)]))),
        ("🎭️masks", base(false), paint_stroke("paint", "mask", "brush", brush(1.5, 0.5, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0), (4.0, 2.0)]))),
        ("⚠️misses", base(false), paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [1.0, 1.0, 1.0, 1.0]), points(&[(40.0, 40.0)]))),
        ("🔒️locked", base(true), paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [1.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0)]))),
        ("🚫️rejects", base(false), paint_stroke("ghost", "pixels", "eraser", brush(4.0, 0.25, 0.5, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0), (2.0, 3.0)]))),
        ("✂️clips", base(false), clipped(paint_stroke("paint", "pixels", "brush", brush(3.0, 1.0, 1.0, [0.0, 0.5, 1.0, 1.0]), points(&[(0.0, 2.0), (6.0, 2.0)])), &[(6, 3, 255), (12, 6, 128)])),
    ]
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

fn json(value: &impl semio_framework_value::ToValue) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("a value prints as JSON")
}

/// 🎯️ The committed outcome of one stroke: applied, a warned no-op, or a refusal naming its code and target.
fn outcome_json(outcome: &protocol::MutationOutcome<RasterDiff>) -> serde_json::Value {
    let refusal = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    if let Some(message) = refusal {
        return serde_json::json!({ "status": "rejected", "code": message.code.0, "path": message.target });
    }
    let messages: Vec<serde_json::Value> = outcome.messages().iter().map(|message| serde_json::json!({ "level": json(&message.level), "code": message.code.0 })).collect();
    match (outcome.diff() == &RasterDiff::default(), messages.is_empty()) {
        (true, false) => serde_json::json!({ "status": "no-op", "messages": messages }),
        (_, true) => serde_json::json!({ "status": "applied" }),
        (false, false) => serde_json::json!({ "status": "applied", "messages": messages }),
    }
}

/// 🖨️ The five files of one scenario as this leaf computes them: before, after, mutation, diff, outcome.
fn quintet(before: &RasterSnapshot, mutation: &RasterMutation) -> [(&'static str, serde_json::Value); 5] {
    let outcome = mutation.diff(before);
    let after = MutationDiff::apply(outcome.diff(), before).expect("every committed stroke diff applies");
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
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🖌️paint-stroke")
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

//#region 🎨️Raster
/// 🎨️ The painted image is exactly what the ONE shared rasterizer paints into a blank layer: the leaf adds no pixel math.
#[test]
fn a_stroke_paints_exactly_what_the_shared_rasterizer_paints() {
    let before = base(false);
    let stroke = paint_stroke("paint", "pixels", "brush", brush(2.5, 0.4, 0.75, [0.2, 0.6, 1.0, 1.0]), points(&[(0.25, 0.75), (5.5, 3.25), (2.0, 0.5)]));
    let after = apply_raster_mutation(&before, &stroke).expect("the stroke applies");
    let Some(RasterLayerNode::Pixel { image_key: Some(key), .. }) = find_layer(&after.layers, "paint") else { panic!("the layer points at the painted image") };
    let image = after.assets.get(key).and_then(|child| child.local_owner::<SemioImageSnapshot>()).expect("the painted image is materialized");
    let mut expected = RasterImage { width: 6, height: 4, pixels: vec![0; 6 * 4 * 4] };
    let RasterMutation::PaintStroke(payload) = &stroke else { unreachable!() };
    paint_stroke_in_place(&mut expected, &operation(payload), None).expect("the rasterizer paints");
    assert_eq!((image.width, image.height), (6, 4));
    assert_eq!(image.frames[0].rgba8, expected.pixels);
    assert!(key.starts_with("pixels-paint-"), "{key}");
    retire(after);
    retire(before);
}

/// ↩️ Painting, then erasing over the painted image, then undoing the erase restores the painted image exactly — the same
/// child id, the same pixels — and undoing the paint restores the blank layer with no asset left behind.
#[test]
fn the_inverse_restores_the_exact_prior_image() {
    let blank = base(false);
    let paint = paint_stroke("paint", "pixels", "brush", brush(3.0, 1.0, 1.0, [1.0, 0.5, 0.0, 1.0]), points(&[(0.5, 0.5), (5.5, 3.5)]));
    let painted = apply_raster_mutation(&blank, &paint).expect("the paint applies");
    let erase = paint_stroke("paint", "pixels", "eraser", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(3.0, 0.0), (3.0, 4.0)]));
    let erased = apply_raster_mutation(&painted, &erase).expect("the erase applies");
    assert_eq!(erased.assets.len(), 1, "the image the erase replaced left the pool");
    let mut restored = erased.clone();
    for undo in inverse_raster_mutation(&painted, &erase).expect("valid retained mutation inverse fixture") {
        let next = apply_raster_mutation(&restored, &undo).expect("every undo step applies");
        retire(std::mem::replace(&mut restored, next));
    }
    assert_eq!(restored, painted, "undoing the erase lands on the painted document");
    let key = |document: &RasterSnapshot| match find_layer(&document.layers, "paint") {
        Some(RasterLayerNode::Pixel { image_key: Some(key), .. }) => key.clone(),
        _ => panic!("the layer shows an image"),
    };
    let pixels = |document: &RasterSnapshot| document.assets.get(&key(document)).and_then(|child| child.local_owner::<SemioImageSnapshot>()).expect("materialized").frames[0].rgba8.clone();
    assert_eq!(pixels(&restored), pixels(&painted), "the restored image holds the painted pixels");
    let mut unpainted = painted.clone();
    for undo in inverse_raster_mutation(&blank, &paint).expect("valid retained mutation inverse fixture") {
        let next = apply_raster_mutation(&unpainted, &undo).expect("every undo step applies");
        retire(std::mem::replace(&mut unpainted, next));
    }
    assert_eq!(unpainted, blank, "undoing the paint restores the blank layer with an empty pool");
    for document in [blank, painted, erased, restored, unpainted] {
        retire(document);
    }
}

/// ⏪️ A stroke whose brush is edited in history re-derives its pixels from its base, and the downstream stroke replays on
/// the edited one: the fold of the edited log equals painting the edited stroke, then the downstream one.
#[test]
fn an_edited_stroke_rederives_itself_and_its_downstream() {
    let blank = base(false);
    let first = paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [1.0, 0.0, 0.0, 1.0]), points(&[(0.5, 1.5), (5.5, 1.5)]));
    let edited = paint_stroke("paint", "pixels", "brush", brush(3.0, 1.0, 1.0, [0.0, 0.0, 1.0, 1.0]), points(&[(0.5, 1.5), (5.5, 1.5)]));
    let downstream = paint_stroke("paint", "pixels", "eraser", brush(1.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(2.5, 0.0), (2.5, 4.0)]));
    let fold = |log: &[&RasterMutation]| {
        let mut document = base(false);
        for mutation in log {
            let next = apply_raster_mutation(&document, mutation).expect("the log folds");
            retire(std::mem::replace(&mut document, next));
        }
        document
    };
    let original = fold(&[&first, &downstream]);
    let rewritten = fold(&[&edited, &downstream]);
    assert_ne!(original, rewritten, "the edited brush changes the head");
    let mut stepwise = apply_raster_mutation(&blank, &edited).expect("the edited stroke applies on its base");
    let next = apply_raster_mutation(&stepwise, &downstream).expect("the downstream stroke replays on the edited one");
    retire(std::mem::replace(&mut stepwise, next));
    assert_eq!(rewritten, stepwise, "the edited log folds exactly like replaying the edited stroke and its downstream");
    let again = fold(&[&edited, &downstream]);
    assert_eq!(again, rewritten, "replay is deterministic");
    for document in [blank, original, rewritten, stepwise, again] {
        retire(document);
    }
}

/// 🏷️ A stroke row reads in English and German from the leaf.
#[test]
fn the_stroke_labels_itself_in_english_and_german() {
    let RasterMutation::PaintStroke(stroke) = paint_stroke("paint", "mask", "eraser", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)])) else { unreachable!() };
    let label = protocol::MutationKind::<RasterSnapshot, RasterMutation>::label(&stroke);
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Mask eraser stroke of 3 points on layer paint");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "Masken-Radierstrich mit 3 Punkten auf Ebene paint");
}

/// 🧯️ A payload the schema refuses is a Fatal invariant breach naming the field, and moves nothing.
#[test]
fn a_payload_outside_its_bounds_is_an_invariant_breach() {
    let document = base(false);
    for (field, stroke) in [
        ("target", paint_stroke("paint", "canvas", "brush", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0)]))),
        ("tool", paint_stroke("paint", "pixels", "smudge", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0)]))),
        ("points", paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), Vec::new())),
        ("brush", paint_stroke("paint", "pixels", "brush", brush(0.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0)]))),
        ("brush", paint_stroke("paint", "pixels", "brush", RasterBrush { size: 2.0, hardness: 1.0, opacity: 1.0, color: vec![1.0, 0.0, 0.0] }, points(&[(1.0, 1.0)]))),
        ("selection", clipped(paint_stroke("paint", "pixels", "brush", brush(2.0, 1.0, 1.0, [0.0, 0.0, 0.0, 1.0]), points(&[(1.0, 1.0)])), &[(4, 2, 255), (5, 1, 255)])),
    ] {
        let outcome = stroke.diff(&document);
        assert_eq!(outcome.diff(), &RasterDiff::default());
        let message = outcome.messages().first().expect("a breach raises");
        assert_eq!((message.level, message.code.0.as_str(), message.target.as_slice()), (semio_framework_diagnostic::Severity::Fatal, "mutation.invariant", [field.to_string()].as_slice()));
        assert!(stroke.inverse(&document).expect("valid retained mutation inverse fixture").is_empty());
    }
    retire(document);
}
//#endregion 🎨️Raster

//#region 🏪️Store
/// 🏪️ The store folds a stroke through the artifact's own retained owners exactly like the leaf's diff.
#[semio_framework_async_macros::async_test]
async fn the_store_folds_a_stroke_like_the_leaf() {
    let stroke = paint_stroke("paint", "pixels", "brush", brush(2.0, 0.5, 0.8, [0.1, 0.9, 0.3, 1.0]), points(&[(1.0, 1.0), (4.5, 2.5)]));
    let mut store = crate::mutations::RasterStore::new(store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "paint-stroke", base(false), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(crate::host::owned::raster_document_store_owners());
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![stroke.clone()], transaction: None }).await.expect("the stroke applies");
    let before = base(false);
    let expected = apply_raster_mutation(&before, &stroke).expect("the leaf applies");
    let projected = store.snapshot().expect("the store projects");
    assert_eq!(projected, expected);
    retire(projected);
    retire(expected);
    retire(before);
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🏪️Store
