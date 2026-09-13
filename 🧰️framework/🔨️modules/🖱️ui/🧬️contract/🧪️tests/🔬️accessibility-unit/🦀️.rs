use super::*;

fn ui_text(value: &str) -> crate::UiText {
    crate::UiText::try_from_str(value).expect("bounded fixture text")
}

#[test]
fn default_spec_serializes_to_empty_object() {
    let json = serde_json::to_value(AccessibilitySpec::default()).expect("serialize");
    assert_eq!(json, serde_json::json!({}));
}

#[test]
fn liveness_field_omitted_at_default() {
    let json = serde_json::to_value(AccessibilitySpec { live: Liveness::Assertive, ..AccessibilitySpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "live": "assertive" }));
}

#[test]
fn hidden_field_omitted_at_default() {
    let json = serde_json::to_value(AccessibilitySpec { hidden: true, ..AccessibilitySpec::default() }).expect("serialize");
    assert_eq!(json, serde_json::json!({ "hidden": true }));
}

//#region ♿️ProjectionLaws

const PROJECTION_FIXTURE: &str = include_str!("../../🧫️fixtures/♿️accessibility-projection.json");

/// ♿️ The role a `Component` implies, for every component the contract declares — answered here by
/// `accessibility_role` and by the TypeScript `uiAccessibilityRoleV1` twin over the SAME fixture, so
/// no renderer can invent a role vocabulary of its own.
#[test]
fn every_component_implies_the_role_the_shared_fixture_declares() {
    let fixture: serde_json::Value = serde_json::from_str(PROJECTION_FIXTURE).expect("♿️ the projection fixture parses");
    let rows = fixture["roles"].as_array().expect("fixture role rows");
    let mut seen = std::collections::BTreeSet::new();
    for row in rows {
        let id = row["id"].as_str().expect("fixture row id");
        let component: crate::Component = serde_json::from_value(row["component"].clone()).unwrap_or_else(|error| panic!("{id}: fixture component deserializes: {error}"));
        let activatable = row["activatable"].as_bool().expect("fixture activatable");
        assert_eq!(accessibility_role(&component, activatable), row["role"].as_str().expect("fixture role"), "{id}: implied role");
        assert_eq!(accessibility_is_focusable(&component, activatable), row["focusable"].as_bool().expect("fixture focusable"), "{id}: focusability");
        seen.insert(row["component"]["type"].as_str().expect("fixture component type tag"));
    }
    assert_eq!(seen.len(), 18, "the fixture covers every one of the contract's 18 components");
    eprintln!("[DEBUG] ui contract accessibility: {} role rows over all {} components", rows.len(), seen.len());
}

/// ♿️ The per-node projection of a real published snapshot, field for field.
#[test]
fn every_published_record_projects_the_way_the_shared_fixture_declares() {
    let fixture: serde_json::Value = serde_json::from_str(PROJECTION_FIXTURE).expect("♿️ the projection fixture parses");
    let snapshot: crate::UiSnapshot = serde_json::from_value(fixture["document"].clone()).expect("♿️ the fixture document is a valid snapshot");
    let expected = fixture["expected"].as_array().expect("fixture expectation");
    let mut announced = Vec::new();
    for row in expected {
        let node_id = crate::UiNodeId(row["nodeId"].as_u64().expect("fixture node id"));
        let record = snapshot.nodes.iter().find(|record| record.id == node_id).unwrap_or_else(|| panic!("the fixture document declares node {node_id:?}"));
        let depth = row["depth"].as_u64().expect("fixture depth") as usize;
        let node = accessibility_projection_node(record, depth);
        assert_eq!(node.key, row["key"].as_str().expect("fixture key"));
        assert_eq!(node.role, row["role"].as_str().expect("fixture role"), "{} role", node.key);
        assert_eq!(node.label.as_deref(), row["label"].as_str(), "{} label", node.key);
        assert_eq!(node.description.as_deref(), row["description"].as_str(), "{} description", node.key);
        assert_eq!(node.live, row["live"].as_str().expect("fixture live"), "{} live", node.key);
        assert_eq!(node.shortcut.as_deref(), row["shortcut"].as_str(), "{} shortcut", node.key);
        assert_eq!(node.hidden, row["hidden"].as_bool().expect("fixture hidden"), "{} hidden", node.key);
        assert_eq!(node.disabled, row["disabled"].as_bool().expect("fixture disabled"), "{} disabled", node.key);
        assert_eq!(node.focusable, row["focusable"].as_bool().expect("fixture focusable"), "{} focusable", node.key);
        assert_eq!(node.actionable, row["actionable"].as_bool().expect("fixture actionable"), "{} actionable", node.key);
        assert!(!node.focused, "the pure projection stamps no live focus — only a renderer's own walk does");
        if !node.hidden && (node.focusable || node.actionable) && node.label.is_some() {
            announced.push(node.node_id);
        }
    }
    let declared: Vec<u64> = fixture["announced"].as_array().expect("fixture announced").iter().map(|id| id.as_u64().expect("fixture announced id")).collect();
    assert_eq!(announced, declared, "exactly the reachable, named nodes");
    eprintln!("[DEBUG] ui contract accessibility: {} projected nodes, {} reachable by name", expected.len(), announced.len());
}

/// ♿️ A hidden node is KEPT, carrying `hidden`, never dropped — a consumer that dropped it would
/// disagree with the DOM renderer (which renders the element and marks it `aria-hidden`) about what
/// exists on the surface at all.
#[test]
fn a_hidden_node_is_marked_not_dropped() {
    let fixture: serde_json::Value = serde_json::from_str(PROJECTION_FIXTURE).expect("♿️ the projection fixture parses");
    let snapshot: crate::UiSnapshot = serde_json::from_value(fixture["document"].clone()).expect("♿️ snapshot");
    let ornament = snapshot.nodes.iter().find(|record| record.accessibility.hidden).expect("the fixture declares a decorative node");
    let node = accessibility_projection_node(ornament, 1);
    assert!(node.hidden);
    assert_eq!(node.role, "img", "a hidden node still carries the role it would have had");
    eprintln!("[DEBUG] ui contract accessibility: the decorative node projects as hidden, not absent");
}

//#endregion ♿️ProjectionLaws

#[test]
fn shortcut_roundtrips() {
    let spec = AccessibilitySpec { shortcut: Some(ui_text("Ctrl+S")), live: Liveness::Polite, hidden: false, ..AccessibilitySpec::default() };
    let json = serde_json::to_string(&spec).expect("serialize");
    let back: AccessibilitySpec = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(spec, back);
}
