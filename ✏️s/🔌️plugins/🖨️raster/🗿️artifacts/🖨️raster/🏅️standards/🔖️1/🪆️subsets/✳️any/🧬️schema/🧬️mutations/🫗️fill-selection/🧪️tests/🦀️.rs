//! 🧪️ `fill-selection` laws: the committed quintets are a print of this leaf (and of the ONE shared fill engine the region
//! fill uses); a fill's inverse restores the exact prior image; a fill edited in history re-derives itself and its
//! downstream; the store folds it like the leaf.

use super::*;
use crate::mutations::{inverse_raster_mutation};
use crate::standards::v1::subsets::any::io::text::mutations::apply_raster_mutation;
use crate::standards::v1::subsets::any::schema::find_layer;
use crate::{RasterLayerNode, SemioImageSnapshot, RASTER_DOCUMENT_SCHEMA};
use protocol::Mutation;
use semio_framework_pixels::editing::PixelOperation;

//#region 🧫️Cases
fn base(locked: bool) -> RasterSnapshot {
    let identity = r#"{"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":0.0,"d":1.0}"#;
    semio_framework_pack_json::from_json_str(&format!(
        r#"{{"schema":"s.raster.raster","id":"fill","title":"Fill","layers":[{{"kind":"pixel","id":"paint","name":"Paint","visible":true,"opacity":1.0,"blendMode":"normal","transform":{identity},"mask":{{"enabled":true,"linked":true,"invert":false,"width":null,"height":null,"imageKey":null,"transform":{identity}}},"width":6,"height":4,"imageKey":null,"locked":{locked}}}]}}"#
    ), semio_framework_pack_json::JsonMemberPolicy::Reject)
    .expect("the fill base decodes")
}

fn clipped(fill: RasterMutation, spans: &[(u32, u32, u32)]) -> RasterMutation {
    let RasterMutation::FillSelection(fill) = fill else { unreachable!("a selection fill") };
    RasterMutation::FillSelection(FillSelection { selection: Some(spans.iter().map(|(start, length, coverage)| RasterSelectionSpan { start: *start, length: *length, coverage: *coverage }).collect()), ..fill })
}

fn retire(snapshot: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(snapshot);
}

/// 🧫️ Every committed scenario: its directory, its before-document and its fill.
fn cases() -> Vec<(&'static str, RasterSnapshot, RasterMutation)> {
    vec![
        ("🫗️fills", base(false), fill_selection("paint", "pixels", [0.0, 0.5, 1.0, 1.0])),
        ("✂️clips", base(false), clipped(fill_selection("paint", "pixels", [1.0, 1.0, 0.0, 1.0]), &[(6, 3, 255), (12, 6, 128)])),
        ("🎭️masks", base(false), clipped(fill_selection("paint", "mask", [0.25, 0.25, 0.25, 0.5]), &[(0, 3, 255)])),
        ("⚠️unchanged", base(false), fill_selection("paint", "pixels", [0.0, 0.0, 0.0, 0.0])),
        ("📍️outside", base(false), clipped(fill_selection("paint", "pixels", [1.0, 0.0, 0.0, 1.0]), &[(20, 8, 255)])),
        ("🔒️locked", base(true), fill_selection("paint", "pixels", [1.0, 0.0, 0.0, 1.0])),
        ("🚫️rejects", base(false), fill_selection("ghost", "pixels", [1.0, 0.0, 0.0, 1.0])),
    ]
}

fn json(value: &impl semio_framework_value::ToValue) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("a value prints as JSON")
}

/// 🎯️ The committed outcome of one fill: applied, a warned no-op, or a refusal naming its code and target.
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
    let after = protocol::apply_diff(outcome.diff(), before).expect("every committed fill diff applies");
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
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🫗️fill-selection")
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

//#region 🫗️Fill
fn pixels(document: &RasterSnapshot) -> Vec<u8> {
    let Some(RasterLayerNode::Pixel { image_key: Some(key), .. }) = find_layer(&document.layers, "paint") else { panic!("the layer shows an image") };
    document.assets.get(key).and_then(|child| child.local_owner::<SemioImageSnapshot>()).expect("the image is materialized").frames[0].rgba8.clone()
}

/// 🫗️ The filled image is exactly what the ONE shared fill engine computes over the selection's coverage.
#[test]
fn a_fill_fills_exactly_what_the_shared_engine_does() {
    let before = base(false);
    let fill = clipped(fill_selection("paint", "pixels", [0.0, 1.0, 0.0, 1.0]), &[(1, 4, 255), (9, 2, 64)]);
    let after = apply_raster_mutation(&before, &fill).expect("the fill applies");
    let mut expected = RasterImage { width: 6, height: 4, pixels: vec![0; 6 * 4 * 4] };
    let mut coverage = vec![0u8; 24];
    coverage[1..5].fill(255);
    coverage[9..11].fill(64);
    assert!(fill_in_place(&mut expected, &PixelOperation::Fill([0, 255, 0, 255]), &coverage).expect("the engine fills"));
    assert_eq!(pixels(&after), expected.pixels);
    retire(after);
    retire(before);
}

/// ↩️ Filling, then undoing the fill, restores the exact prior document.
#[test]
fn the_inverse_restores_the_exact_prior_image() {
    let before = apply_raster_mutation(&base(false), &fill_selection("paint", "pixels", [1.0, 0.0, 0.0, 1.0])).expect("a first fill");
    let fill = clipped(fill_selection("paint", "pixels", [0.0, 0.0, 1.0, 1.0]), &[(0, 6, 255)]);
    let filled = apply_raster_mutation(&before, &fill).expect("the fill applies");
    let mut restored = filled.clone();
    for undo in inverse_raster_mutation(&before, &fill).expect("valid retained mutation inverse fixture") {
        let next = apply_raster_mutation(&restored, &undo).expect("every undo step applies");
        retire(std::mem::replace(&mut restored, next));
    }
    assert_eq!(restored, before);
    for document in [before, filled, restored] {
        retire(document);
    }
}

/// ⏪️ A fill whose colour and selection are edited in history re-derives itself and its downstream from its base.
#[test]
fn an_edited_fill_rederives_itself_and_its_downstream() {
    let first = fill_selection("paint", "pixels", [1.0, 0.0, 0.0, 1.0]);
    let edited = clipped(fill_selection("paint", "pixels", [0.0, 1.0, 0.0, 0.5]), &[(0, 12, 255)]);
    let downstream = clipped(fill_selection("paint", "pixels", [0.0, 0.0, 1.0, 1.0]), &[(10, 4, 255)]);
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
    assert_ne!(original, rewritten);
    let start = base(false);
    let mut stepwise = apply_raster_mutation(&start, &edited).expect("the edited fill applies on its base");
    let next = apply_raster_mutation(&stepwise, &downstream).expect("the downstream fill replays on it");
    retire(std::mem::replace(&mut stepwise, next));
    assert_eq!(rewritten, stepwise);
    for document in [start, original, rewritten, stepwise] {
        retire(document);
    }
}

/// 🏷️ A fill row reads in English and German from the leaf.
#[test]
fn the_fill_labels_itself_in_english_and_german() {
    for (mutation, en, de) in [
        (fill_selection("paint", "mask", [1.0, 1.0, 1.0, 1.0]), "Fill the mask of layer paint", "Maske von Ebene paint füllen"),
        (clipped(fill_selection("paint", "pixels", [1.0, 1.0, 1.0, 1.0]), &[(0, 1, 255)]), "Fill the selection on layer paint", "Auswahl auf Ebene paint füllen"),
    ] {
        let RasterMutation::FillSelection(fill) = mutation else { unreachable!() };
        let label = protocol::MutationKind::<RasterSnapshot, RasterMutation>::label(&fill);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), en);
        assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), de);
    }
}

/// 🧯️ A payload the schema refuses is a Fatal invariant breach naming the field, and moves nothing.
#[test]
fn a_payload_outside_its_bounds_is_an_invariant_breach() {
    let document = base(false);
    for (field, fill) in [
        ("target", fill_selection("paint", "canvas", [0.0, 0.0, 0.0, 1.0])),
        ("color", fill_selection("paint", "pixels", [0.0, 0.0, 2.0, 1.0])),
        ("selection", clipped(fill_selection("paint", "pixels", [0.0, 0.0, 0.0, 1.0]), &[(4, 2, 255), (5, 1, 255)])),
    ] {
        let outcome = fill.diff(&document);
        assert_eq!(outcome.diff(), &RasterDiff::default());
        let message = outcome.messages().first().expect("a breach raises");
        assert_eq!((message.level, message.code.0.as_str(), message.target.as_slice()), (semio_framework_diagnostic::Severity::Fatal, "mutation.invariant", [field.to_string()].as_slice()));
        assert!(fill.inverse(&document).expect("valid retained mutation inverse fixture").is_empty());
    }
    retire(document);
}
//#endregion 🫗️Fill

//#region 🏪️Store
/// 🏪️ The store folds a fill through the artifact's own retained owners exactly like the leaf's diff.
#[semio_framework_async_macros::async_test]
async fn the_store_folds_a_fill_like_the_leaf() {
    let fill = clipped(fill_selection("paint", "mask", [0.5, 0.5, 0.5, 1.0]), &[(2, 5, 200)]);
    let mut store = crate::mutations::RasterStore::new(store::create_document_envelope(RASTER_DOCUMENT_SCHEMA, "fill-selection", base(false), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(crate::host::owned::raster_document_store_owners());
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![fill.clone()], transaction: None }).await.expect("the fill applies");
    let before = base(false);
    let expected = apply_raster_mutation(&before, &fill).expect("the leaf applies");
    let projected = store.snapshot().expect("the store projects");
    assert_eq!(projected, expected);
    retire(projected);
    retire(expected);
    retire(before);
    store::os_store::test_support::close_plain_test_store(&mut store);
}
//#endregion 🏪️Store
