//! 📏️ The window-measures wire home on both bridge backends: the guest's reserved
//! `framework.section.measures` surface, built here by the REAL producer
//! (`semio_framework_plugin::app::paged_text_carrier`), published as a retained document and read back
//! by [`window_measures_from_section`]. `serde_json` is the independent oracle for the round trip.
//!
//! Fixture: `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`.

use super::*;

pub(crate) fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/📏️window-measures/🔣️.json")).expect("window measures fixture parses")
}

fn flatten(node: &ui_contract::BuiltNode, next: &mut u64, records: &mut Vec<ui_contract::UiNodeRecord>) -> ui_contract::UiNodeId {
    let id = ui_contract::UiNodeId(*next);
    *next += 1;
    let slot = records.len();
    records.push(ui_contract::UiNodeRecord {
        id,
        key: node.key.clone(),
        component: node.component.credited_clone().expect("carrier component credit"),
        layout: node.layout.clone(),
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        transition: None,
        accessibility: node.accessibility.clone(),
        bindings: ui_contract::UiNodeBindings::default(),
        menu: None,
        children: ui_contract::UiNodeChildren::default(),
    });
    for child in node.children.iter() {
        let child_id = flatten(child, next, records);
        records[slot].children.try_push(child_id).expect("carrier children fit the document");
    }
    id
}

pub(crate) fn section_document(section: &serde_json::Value, generation: u64, refresh_section: semio_framework::UiRefreshSection) -> UiDocumentLease {
    let body_key = refresh_section.body_key();
    let carrier = semio_framework_plugin::app::paged_text_carrier(body_key, &serde_json::to_string(section).expect("section serializes")).expect("real producer builds the carrier");
    let mut records = Vec::new();
    let root = flatten(&carrier, &mut 1, &mut records);
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(1), root: Some(root), layout_epoch: 0 };
    UiDocumentLease::try_publish(SurfaceId::try_from(body_key).expect("reserved surface id"), identity, records).expect("reserved section publishes")
}

/// 📃️ The reserved measures surface exactly as the guest publishes it for `section`.
pub(crate) fn measures_section_document(section: &serde_json::Value, generation: u64) -> UiDocumentLease {
    section_document(section, generation, semio_framework::UiRefreshSection::Measures)
}

#[test]
fn window_measures_section_round_trips_every_window_instance() {
    let fixture = fixture();
    let mut document = measures_section_document(&fixture["section"], 1);
    let measures = window_measures_from_section(&document).expect("measures section decodes");
    assert_eq!(serde_json::to_value(&measures).expect("measures serialize"), fixture["section"], "serde_json oracle: the decoded section is the authored one");
    assert!(matches!(measures["puzzle3d-main"][0], WindowMeasure::Number { value, .. } if value == 12.0), "the puzzle 3d fill count reaches the wgpu shell");
    while !document.close_step() {}
}

#[test]
fn window_measures_section_refuses_a_payload_that_is_not_a_measures_map() {
    let mut document = measures_section_document(&serde_json::json!(["not", "a", "map"]), 2);
    let error = window_measures_from_section(&document).expect_err("a non-map payload must refuse");
    assert!(error.starts_with("window measures section parse"), "{error}");
    while !document.close_step() {}
}
