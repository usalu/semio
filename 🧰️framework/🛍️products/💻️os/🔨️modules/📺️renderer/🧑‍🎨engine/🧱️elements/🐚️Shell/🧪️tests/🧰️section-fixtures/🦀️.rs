//! 🧰️ Fixtures the Shell tests share with the `program-bridge` and `canvas-presence` elements: the reserved window-measures and window-engagements section documents built by the real producer, and the canvas-presence roster rows.
//!
//! Moved out of `cfg(test)` helper modules of the carved crates (`window_measures_section_tests`, `window_engagements_section_tests`, `canvas_presence::tests`), which a dependency cannot expose to a dependent's test build.

use super::*;
use replication::{PresenceDomain, PresenceInteraction, PresenceViewKind};
use serde_json::Value;

pub fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/📏️window-measures/🔣️.json")).expect("window measures fixture parses")
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

pub fn section_document(section: &serde_json::Value, generation: u64, refresh_section: semio_framework::UiRefreshSection) -> UiDocumentLease {
    let body_key = refresh_section.body_key();
    let carrier = semio_framework_plugin::app::paged_text_carrier(body_key, &serde_json::to_string(section).expect("section serializes")).expect("real producer builds the carrier");
    let mut records = Vec::new();
    let root = flatten(&carrier, &mut 1, &mut records);
    let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(1), root: Some(root), layout_epoch: 0 };
    UiDocumentLease::try_publish(SurfaceId::try_from(body_key).expect("reserved surface id"), identity, records).expect("reserved section publishes")
}

/// 📃️ The reserved measures surface exactly as the guest publishes it for `section`.
pub fn measures_section_document(section: &serde_json::Value, generation: u64) -> UiDocumentLease {
    section_document(section, generation, semio_framework::UiRefreshSection::Measures)
}

pub fn engagements_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🎬️window-engagements/🔣️.json")).expect("window engagements fixture parses")
}

pub fn engagements_section_document(section: &serde_json::Value, generation: u64) -> UiDocumentLease {
    section_document(section, generation, semio_framework::UiRefreshSection::Engagements)
}

/// 🔢️ A fixture value with every number as `f64`, so an `f32` screen coordinate compares to its JSON integer.
pub fn numeric(value: &Value) -> Value {
    match value {
        Value::Number(number) => serde_json::Number::from_f64(number.as_f64().expect("finite number")).map_or(Value::Null, Value::Number),
        Value::Array(items) => Value::Array(items.iter().map(numeric).collect()),
        Value::Object(fields) => Value::Object(fields.iter().map(|(key, field)| (key.clone(), numeric(field))).collect()),
        other => other.clone(),
    }
}


fn numbers(value: &Value) -> Vec<f64> {
    value.as_array().expect("number array").iter().map(|number| number.as_f64().expect("number")).collect()
}

fn window_view(value: &Value) -> PresenceWindowView {
    let kind = &value["kind"];
    let size = numbers(&value["size"]);
    PresenceWindowView {
        window_id: value["windowId"].as_str().expect("window id").to_string(),
        space: value["space"].as_str().expect("space").to_string(),
        kind: PresenceViewKind::Canvas { x: kind["x"].as_f64().expect("x"), y: kind["y"].as_f64().expect("y"), zoom: kind["zoom"].as_f64().expect("zoom") },
        size: [size[0], size[1]],
        pointer: value.get("pointer").map(|pointer| {
            let pointer = numbers(pointer);
            [pointer[0], pointer[1], pointer[2]]
        }),
        ray_origin: None,
    }
}

/// 👥️ One fixture roster row as the wire's `PresencePeer` — shared with the shell's board-presence law.
pub fn peer(value: &Value) -> PresencePeer {
    let strings = |value: &Value| value.as_array().expect("id array").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>();
    PresencePeer {
        actor: value["actor"].as_str().expect("actor").to_string(),
        connected_at_ms: value["connectedAtMs"].as_i64().expect("connected at"),
        label: value.get("label").and_then(Value::as_str).map(str::to_string),
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: value.get("interaction").map(|interaction| PresenceInteraction {
            app_id: interaction["app_id"].as_str().expect("app id").to_string(),
            domains: interaction["domains"]
                .as_array()
                .expect("domains")
                .iter()
                .map(|domain| PresenceDomain {
                    domain: domain["domain"].as_str().expect("domain").to_string(),
                    granularity: domain["granularity"].as_str().expect("granularity").to_string(),
                    selected: strings(&domain["selected"]),
                    hovered: strings(&domain["hovered"]),
                })
                .collect(),
        }),
        color: value.get("color").and_then(Value::as_u64).map(|color| color as u8),
        surface: None,
        views: value["views"].as_array().expect("views").iter().map(window_view).collect(),
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: value.get("activeTool").and_then(Value::as_str).map(str::to_string),
        history_edit: None,
        typing: Vec::new(),
    }
}
