//! 🧪️ `apply-filter` laws: the committed quintets are a print of this leaf (and of the ONE shared pixel engine the
//! TypeScript twin runs on the shared pixel corpus); a filter's inverse restores the exact prior image; a filter edited in
//! history re-derives its pixels — and its downstream — from its base; the store folds it like the leaf.

use super::*;
use crate::mutations::paint_stroke::{paint_stroke, RasterBrush, RasterStrokePoint};
use crate::mutations::{apply_raster_mutation, inverse_raster_mutation};
use crate::standards::v1::subsets::any::schema::find_layer;
use crate::{RasterLayerNode, SemioImageSnapshot, RASTER_DOCUMENT_SCHEMA};
use protocol::{Mutation, MutationDiff};

//#region 🧫️Cases
fn base(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"filter","title":"Filter","layers":[{{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":6,"height":4,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the filter base decodes")
}

/// 🧱️ A half-opaque red wall down column 2 of the blank layer.
fn walled() -> RasterMutation {
    paint_stroke("paint", "pixels", "brush", RasterBrush { size: 1.0, hardness: 1.0, opacity: 1.0, color: vec![0.8, 0.2, 0.1, 0.5] }, vec![RasterStrokePoint { x: 2.5, y: 0.0 }, RasterStrokePoint { x: 2.5, y: 4.0 }])
}

/// ✂️ `filter` clipped to the selection runs `(start, length, coverage)`.
fn clipped(filter: RasterMutation, spans: &[(u32, u32, u32)]) -> RasterMutation {
    let RasterMutation::ApplyFilter(filter) = filter else { unreachable!("an image filter") };
    RasterMutation::ApplyFilter(ApplyFilter { selection: Some(spans.iter().map(|(start, length, coverage)| RasterSelectionSpan { start: *start, length: *length, coverage: *coverage }).collect()), ..filter })
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

/// 🧱️ The blank base with the wall painted.
fn walled_base() -> RasterSnapshot {
    let blank = base(false);
    let walled_document = apply_raster_mutation(&blank, &walled()).expect("the wall paints");
    retire(blank);
    walled_document
}

/// 🧫️ Every committed scenario: its directory, its before-document and its filter.
fn cases() -> Vec<(&'static str, RasterSnapshot, RasterMutation)> {
    vec![
        ("🌈️inverts", walled_base(), apply_filter("paint", "invert", 0.0)),
        ("🔆️brightens", walled_base(), apply_filter("paint", "brightness", 0.5)),
        ("🌫️blurs", walled_base(), apply_filter("paint", "blur", 1.0)),
        ("↔️flips", walled_base(), apply_filter("paint", "flipHorizontal", 0.0)),
        ("⚠️unchanged", base(false), apply_filter("paint", "grayscale", 0.0)),
        ("🔒️locked", base(true), apply_filter("paint", "invert", 0.0)),
        ("🚫️rejects", base(false), apply_filter("ghost", "invert", 0.0)),
        ("✂️clips", walled_base(), clipped(apply_filter("paint", "brightness", 1.0), &[(0, 3, 255), (6, 3, 128)])),
    ]
}

fn json(value: &impl semio_framework_value::ToValue) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("a value prints as JSON")
}

/// 🎯️ The committed outcome of one filter: applied, a warned no-op, or a refusal naming its code and target.
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
    let after = MutationDiff::apply(outcome.diff(), before).expect("every committed filter diff applies");
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
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌈️apply-filter")
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

//#region 🌈️Filter
fn pixels(document: &RasterSnapshot) -> Vec<u8> {
    let Some(RasterLayerNode::Pixel { image_key: Some(key), .. }) = find_layer(&document.layers, "paint") else { panic!("the layer shows an image") };
    document.assets.get(key).and_then(|child| child.local_owner::<SemioImageSnapshot>()).expect("the image is materialized").frames[0].rgba8.clone()
}

/// 🌈️ The filtered image is exactly what the ONE shared pixel engine computes: the leaf adds no pixel math.
#[test]
fn a_filter_runs_exactly_what_the_shared_engine_does() {
    let before = walled_base();
    for (filter, amount, operation) in [("brightness", 0.5, PixelOperation::Brightness(0.5)), ("blur", 1.0, PixelOperation::Blur(1)), ("flipHorizontal", 0.0, PixelOperation::FlipHorizontal), ("posterize", 4.0, PixelOperation::Posterize(4))] {
        let after = apply_raster_mutation(&before, &apply_filter("paint", filter, amount)).expect("the filter applies");
        let mut job = PixelEditJob::new(RasterImage { width: 6, height: 4, pixels: pixels(&before) }, operation, None).expect("the engine admits the filter");
        while !job.advance(4096).expect("the engine advances").done {}
        assert_eq!(pixels(&after), job.into_result().expect("the engine completes").pixels, "{filter}");
        retire(after);
    }
    retire(before);
}

/// ↩️ Filtering, then undoing the filter, restores the exact prior image — the same child, the same pixels.
#[test]
fn the_inverse_restores_the_exact_prior_image() {
    let before = walled_base();
    let filter = apply_filter("paint", "contrast", 0.75);
    let filtered = apply_raster_mutation(&before, &filter).expect("the filter applies");
    let mut restored = filtered.clone();
    for undo in inverse_raster_mutation(&before, &filter).expect("valid retained mutation inverse fixture") {
        let next = apply_raster_mutation(&restored, &undo).expect("every undo step applies");
        retire(std::mem::replace(&mut restored, next));
    }
    assert_eq!(restored, before, "undoing the filter lands on the walled document with the filtered image released");
    for document in [before, filtered, restored] {
        retire(document);
    }
}

/// ⏪️ A filter whose kind and amount are edited in history re-derives its pixels from its base, and the downstream filter
/// replays on the edited one: the fold of the edited log equals replaying the edited filter, then the downstream one.
#[test]
fn an_edited_filter_rederives_itself_and_its_downstream() {
    let first = apply_filter("paint", "brightness", 0.25);
    let edited = apply_filter("paint", "saturation", -1.0);
    let downstream = apply_filter("paint", "invert", 0.0);
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
    assert_ne!(original, rewritten, "the edited filter changes the head");
    let walled_document = walled_base();
    let mut stepwise = apply_raster_mutation(&walled_document, &edited).expect("the edited filter applies on its base");
    let next = apply_raster_mutation(&stepwise, &downstream).expect("the downstream filter replays on the edited one");
    retire(std::mem::replace(&mut stepwise, next));
    assert_eq!(rewritten, stepwise, "the edited log folds exactly like replaying the edited filter and its downstream");
    for document in [walled_document, original, rewritten, stepwise] {
        retire(document);
    }
}

/// 🏷️ A filter row reads in English and German from the leaf, with its amount when it takes one.
#[test]
fn the_filter_labels_itself_in_english_and_german() {
    for (mutation, en, de) in [
        (apply_filter("paint", "blur", 3.0), "Blur 3 on layer paint", "Weichzeichnen 3 auf Ebene paint"),
        (apply_filter("paint", "invert", 0.0), "Invert on layer paint", "Invertieren auf Ebene paint"),
    ] {
        let RasterMutation::ApplyFilter(filter) = mutation else { unreachable!() };
        let label = protocol::MutationKind::<RasterSnapshot, RasterMutation>::label(&filter);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), en);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), de);
    }
}

/// 🧯️ A payload the schema refuses is a Fatal invariant breach naming the field, and moves nothing.
#[test]
fn a_payload_outside_its_bounds_is_an_invariant_breach() {
    let document = walled_base();
    for (field, filter) in [
        ("filter", apply_filter("paint", "emboss", 0.0)),
        ("amount", apply_filter("paint", "brightness", 1.5)),
        ("amount", apply_filter("paint", "blur", 1.5)),
        ("amount", apply_filter("paint", "invert", 1.0)),
        ("amount", apply_filter("paint", "gamma", f64::NAN)),
        ("selection", clipped(apply_filter("paint", "flipVertical", 0.0), &[(0, 2, 255)])),
        ("selection", clipped(apply_filter("paint", "invert", 0.0), &[(4, 2, 255), (5, 1, 255)])),
    ] {
        let outcome = filter.diff(&document);
        assert_eq!(outcome.diff(), &RasterDiff::default());
        let message = outcome.messages().first().expect("a breach raises");
        assert_eq!((message.level, message.code.0.as_str(), message.target.as_slice()), (semio_framework_diagnostic::Severity::Fatal, "mutation.invariant", [field.to_string()].as_slice()), "{filter:?}");
        assert!(filter.inverse(&document).expect("valid retained mutation inverse fixture").is_empty());
    }
    retire(document);
}
//#endregion 🌈️Filter

//#region 🏪️Store
/// 🏪️ The store folds a filter through the artifact's own retained owners exactly like the leaf's diff.
#[semio_framework_async_macros::async_test]
async fn the_store_folds_a_filter_like_the_leaf() {
    let filter = apply_filter("paint", "threshold", 64.0);
    let mut store = crate::mutations::RasterStore::new(store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "apply-filter", walled_base(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(crate::host::owned::raster_document_store_owners());
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![filter.clone()], transaction: None }).await.expect("the filter applies");
    let before = walled_base();
    let expected = apply_raster_mutation(&before, &filter).expect("the leaf applies");
    let projected = store.snapshot().expect("the store projects");
    assert_eq!(projected, expected);
    retire(projected);
    retire(expected);
    retire(before);
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🏪️Store
