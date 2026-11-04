//! ♿️ Every interactive control carries an accessible name, driven by the shared `🧪️tests/🧫️fixtures/♿️control-names/🔣️.json` fixture.

use super::*;

fn record(id: u64, component: serde_json::Value, label: Option<&str>) -> crate::UiNodeRecord {
    let mut node = serde_json::json!({ "id": id, "key": format!("#{id}"), "component": component, "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {} });
    if let Some(label) = label {
        node["accessibility"]["label"] = label.into();
    }
    serde_json::from_value(node).expect("fixture node")
}

#[test]
fn every_fixture_case_agrees_with_the_unnamed_control_rule() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/♿️control-names/🔣️.json")).expect("control name fixture");
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["case"].as_str().unwrap();
        let node = record(1, case["component"].clone(), case["label"].as_str());
        let parent = case["parent"].as_object().map(|parent| {
            let mut props = serde_json::json!({ "type": "container", "role": parent["role"] });
            if let Some(label) = parent.get("label") {
                props["label"] = label.clone();
            }
            record(0, props, None)
        });
        assert_eq!(control_is_unnamed(&node, parent.as_ref()), case["unnamed"].as_bool().unwrap(), "{id}");
    }
}

#[test]
fn the_document_walk_reports_each_unnamed_control_once_and_accepts_named_ones() {
    let limits = UiDocumentLimits::default();
    for (name, label, expected) in [("unnamed", None, 1usize), ("labelled", Some("Width"), 0)] {
        let mut state = crate::UiSnapshotState::new(crate::SurfaceId::try_from("surface").unwrap());
        let mut root = record(0, serde_json::json!({ "type": "container", "role": "group" }), None);
        root.children.try_push(crate::UiNodeId(1)).expect("one child");
        state.root = Some(crate::UiNodeId(0));
        state.nodes.try_insert(root).expect("root");
        state.nodes.try_insert(record(1, serde_json::json!({ "type": "input", "value": "x" }), label)).expect("control");
        let violations = validate_state(&state, &limits).err().map(|found| found.iter().filter(|violation| matches!(violation, UiContractViolation::UnnamedControl { .. })).count()).unwrap_or(0);
        assert_eq!(violations, expected, "{name}");
    }
}
