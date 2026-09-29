//! 🎯️ Neutral whole-stack drops address the tree presented after source extraction.

use super::*;
use serde_json::{json, Value};

fn stack_projections(node: &DockNode, path: &mut DockPath, out: &mut Vec<Value>) {
    match node {
        DockNode::Stack { windows, .. } => out.push(json!({ "path": path, "windows": dock_tab_ids(windows) })),
        DockNode::Row(children) | DockNode::Column(children) => {
            for (index, (child, _)) in children.iter().enumerate() {
                path.push(index);
                stack_projections(child, path, out);
                path.pop();
            }
        }
    }
}

#[test]
fn committed_stack_drops_land_on_every_neutral_visible_destination() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧫️fixtures/🎯️stack-drop-destination/🔣️.json")).unwrap();
    for law in fixture["cases"].as_array().unwrap() {
        let initial = law["initialStacks"].as_array().unwrap();
        let source_index = law["sourceIndex"].as_u64().unwrap() as usize;
        let source = &initial[source_index];
        let children = initial.iter().map(|stack| (DockNode::Stack { windows: stack["windows"].as_array().unwrap().iter().map(|id| DockStackTab::new(id.as_str().unwrap())).collect(), active: stack["active"].as_str().unwrap().into() }, 1.0)).collect();
        let mut dock = DockState::default();
        dock.root = if law["axis"] == "row" { DockNode::Row(children) } else { DockNode::Column(children) };
        let before = dock.root.clone();
        let active = source["active"].as_str().unwrap();
        let kind = if law["dragKind"] == "stack" { DockDragKind::Stack } else { DockDragKind::Tab };
        let drag = DockDragPayload { kind, window_id: active.into(), window_kind_id: active.into(), template_id: None, source_path: vec![source_index], tab_index: source["windows"].as_array().unwrap().iter().position(|id| id == active).unwrap(), ghost_label: active.into() };
        let path = law["zone"]["path"].as_array().unwrap().iter().map(|part| part.as_u64().unwrap() as usize).collect();
        let zone = if law["zone"]["kind"] == "tab" {
            DockDropZone::Tab { stack_path: path, corner: WindowStackCorner::TopLeft, index: law["zone"]["index"].as_u64().unwrap() as usize }
        } else {
            let side = match law["zone"]["side"].as_str().unwrap() { "left" => DockSide::Left, "right" => DockSide::Right, "top" => DockSide::Top, _ => DockSide::Bottom };
            DockDropZone::Split { stack_path: path, side }
        };
        assert!(!dock.render_view(Some(&drag)).collect_window_ids().contains(&active.to_string()));
        assert_eq!(dock.root, before, "{}: preview must be reversible", law["id"]);
        assert_eq!(dock.apply_drop(&drag, &zone), law["accepted"].as_bool().unwrap(), "{}", law["id"]);
        let mut actual = Vec::new();
        stack_projections(&dock.root, &mut Vec::new(), &mut actual);
        assert_eq!(json!(actual), law["expectedStacks"], "{}", law["id"]);
        if law["accepted"] == false { assert_eq!(dock.root, before); }
        println!("[DEBUG] WGPU stack drop {}: {}", law["id"], json!(actual));
    }
}
