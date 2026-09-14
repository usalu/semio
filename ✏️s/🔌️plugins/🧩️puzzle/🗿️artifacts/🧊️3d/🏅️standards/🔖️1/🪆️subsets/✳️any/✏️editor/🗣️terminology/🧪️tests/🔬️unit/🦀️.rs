
use super::*;

#[test]
fn labels_resolve_every_host_locale_and_terminology_axis() {
    for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Native), (Locale::De, Terminology::Reuse)] {
        let view_state = semio_framework_plugin::ViewModel { locale, terminology, ..Default::default() };
        assert!(!puzzle3d_labels(&view_state).expect("admitted host axis").objects.as_str().is_empty());
    }
}

#[test]
fn every_authored_locale_tag_including_its_region_form_is_admitted() {
    for (tag, expected) in [("en", Locale::En), ("en-US", Locale::En), ("de", Locale::De), ("de-DE", Locale::De)] {
        for (terminology_tag, expected_terminology) in [("native", Terminology::Native), ("reuse", Terminology::Reuse)] {
            assert_eq!(puzzle3d_label_axes(tag, terminology_tag), Some((expected, expected_terminology)));
        }
    }
}

#[test]
fn an_unauthored_locale_or_terminology_fails_closed_instead_of_defaulting() {
    for tag in ["fr", "en_US", "EN", "de-AT", "", "e", "den"] {
        assert!(puzzle3d_label_axes(tag, "native").is_none(), "{tag} must not resolve a label set");
    }
    for terminology_tag in ["", "Native", "reuse-de", "brand"] {
        assert!(puzzle3d_label_axes("en", terminology_tag).is_none(), "{terminology_tag} must not resolve a label set");
    }
}

/// 🧾️ The language-neutral fill run vocabulary the definition must spell exactly: `$defs.Puzzle3dFillRun`.
const PUZZLE3D_SCHEMA: &str = include_str!("../../../../🧬️schema/🔣️.json");

#[test]
fn the_fill_run_vocabulary_equals_the_schema_table_and_is_authored_in_english_and_german() {
    let schema: serde_json::Value = serde_json::from_str(PUZZLE3D_SCHEMA).expect("schema parses");
    let table = &schema["$defs"]["Puzzle3dFillRun"]["x-semio-toolRun"];
    let stages: Vec<String> = puzzle3d_fill_run_stages().into_iter().map(|stage| stage.id).collect();
    assert_eq!(serde_json::json!(stages), table["stages"]);
    let counters: Vec<String> = puzzle3d_fill_run_counters().into_iter().map(|counter| counter.id).collect();
    assert_eq!(serde_json::json!(counters), table["counters"]);
    let reasons: Vec<serde_json::Value> = puzzle3d_fill_run_reasons().iter().map(|reason| serde_json::json!({ "code": reason.code, "id": reason.id, "verdict": serde_json::to_value(reason.verdict).expect("verdict serializes") })).collect();
    assert_eq!(serde_json::json!(reasons), table["reasons"]);
    let labels = puzzle3d_fill_run_stages().into_iter().map(|stage| stage.label).chain(puzzle3d_fill_run_counters().into_iter().map(|counter| counter.label)).chain(puzzle3d_fill_run_reasons().into_iter().map(|reason| reason.template)).chain([puzzle3d_fill_run_unit()]);
    for label in labels {
        let (en, de) = (label.resolve(Terminology::Native, Locale::En), label.resolve(Terminology::Native, Locale::De));
        assert!(!en.is_empty() && !de.is_empty() && en != de, "every fill run label is authored in both languages: {en} / {de}");
    }
}

/// 🧾️ The read-only brush suggestions vocabulary equals `$defs.Puzzle3dBrushSuggestionsRun` and is authored in both languages.
#[test]
fn the_brush_suggestions_run_vocabulary_equals_the_schema_table_and_is_authored_in_english_and_german() {
    let schema: serde_json::Value = serde_json::from_str(PUZZLE3D_SCHEMA).expect("schema parses");
    let table = &schema["$defs"]["Puzzle3dBrushSuggestionsRun"]["x-semio-toolRun"];
    let stages: Vec<String> = puzzle3d_brush_suggestions_run_stages().into_iter().map(|stage| stage.id).collect();
    assert_eq!(serde_json::json!(stages), table["stages"]);
    let counters: Vec<String> = puzzle3d_brush_suggestions_run_counters().into_iter().map(|counter| counter.id).collect();
    assert_eq!(serde_json::json!(counters), table["counters"]);
    let reasons: Vec<serde_json::Value> = puzzle3d_brush_suggestions_run_reasons().iter().map(|reason| serde_json::json!({ "code": reason.code, "id": reason.id, "verdict": serde_json::to_value(reason.verdict).expect("verdict serializes") })).collect();
    assert_eq!(serde_json::json!(reasons), table["reasons"]);
    let labels = puzzle3d_brush_suggestions_run_stages().into_iter().map(|stage| stage.label).chain(puzzle3d_brush_suggestions_run_counters().into_iter().map(|counter| counter.label)).chain(puzzle3d_brush_suggestions_run_reasons().into_iter().map(|reason| reason.template)).chain([puzzle3d_brush_suggestions_run_unit()]);
    for label in labels {
        let (en, de) = (label.resolve(Terminology::Native, Locale::En), label.resolve(Terminology::Native, Locale::De));
        assert!(!en.is_empty() && !de.is_empty() && en != de, "every brush suggestions label is authored in both languages: {en} / {de}");
    }
}
