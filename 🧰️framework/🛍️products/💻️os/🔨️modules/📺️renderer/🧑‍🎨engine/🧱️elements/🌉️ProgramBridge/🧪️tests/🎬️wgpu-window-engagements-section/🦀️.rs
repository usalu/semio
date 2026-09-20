//! 🎬️ The canonical engagements wire is the guest's reserved retained section, decoded against a
//! language-neutral JSON fixture. `serde_json::Value` is the independent structural oracle.

use super::*;
use super::window_measures_section_tests::section_document;

pub(crate) fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🎬️window-engagements/🔣️.json")).expect("window engagements fixture parses")
}

pub(crate) fn engagements_section_document(section: &serde_json::Value, generation: u64) -> UiDocumentLease {
    section_document(section, generation, semio_framework::UiRefreshSection::Engagements)
}

#[test]
fn window_engagements_section_round_trips_the_actual_camel_case_schema() {
    let fixture = fixture();
    let mut document = engagements_section_document(&fixture["section"], 31);
    let engagements = window_engagements_from_section(&document).expect("engagements section decodes");
    assert_eq!(serde_json::to_value(&engagements).expect("engagements serialize"), fixture["section"], "serde_json oracle preserves the authored section");
    assert_eq!(engagements["pane-top"].input.as_ref().and_then(|input| input.id.as_deref()), Some("engagement-input"));
    assert_eq!(engagements["pane-perspective"].possible_engagements.as_ref().map(Vec::len), Some(1));
    while !document.close_step() {}
}

#[test]
fn window_engagements_section_refuses_a_non_map_payload() {
    let mut document = engagements_section_document(&serde_json::json!(["not", "a", "map"]), 32);
    let error = window_engagements_from_section(&document).expect_err("a non-map payload must refuse");
    assert!(error.starts_with("window engagements section parse"), "{error}");
    while !document.close_step() {}
}
