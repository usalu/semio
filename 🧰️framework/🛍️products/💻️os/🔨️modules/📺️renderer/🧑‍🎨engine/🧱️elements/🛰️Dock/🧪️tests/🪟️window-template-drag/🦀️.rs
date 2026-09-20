//! 🪟️ LAW: a new window template lands through the same tab/split/root-split zones as a dock drag.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json")).expect("window lifecycle fixture")
}

fn side(value: &str) -> DockSide {
    match value {
        "left" => DockSide::Left,
        "right" => DockSide::Right,
        "top" => DockSide::Top,
        _ => DockSide::Bottom,
    }
}

fn root_kind(node: &DockNode) -> &'static str {
    match node {
        DockNode::Stack { .. } => "stack",
        DockNode::Row(_) => "row",
        DockNode::Column(_) => "column",
    }
}

#[test]
fn new_window_template_drop_matches_every_neutral_zone_vector() {
    for case in fixture()["dropCases"].as_array().expect("drop cases") {
        let initial = case["initialWindows"].as_array().map(|ids| ids.iter().map(|id| DockStackTab::new(id.as_str().unwrap())).collect()).unwrap_or_else(|| vec![DockStackTab::new("main")]);
        let mut dock = DockState::default();
        dock.root = DockNode::Stack { windows: initial, active: case["initialWindows"].as_array().map_or_else(|| "main".to_string(), |_| String::new()) };
        let drag = DockDragPayload { kind: DockDragKind::NewWindow, window_id: "world-2".into(), window_kind_id: "world".into(), template_id: Some("top".into()), source_path: Vec::new(), tab_index: 0, ghost_label: "Top".into() };
        let zone = match case["zone"]["kind"].as_str().unwrap() {
            "tab" => DockDropZone::Tab { stack_path: Vec::new(), corner: WindowStackCorner::TopLeft, index: case["zone"]["index"].as_u64().unwrap() as usize },
            "split" => DockDropZone::Split { stack_path: Vec::new(), side: side(case["zone"]["side"].as_str().unwrap()) },
            _ => DockDropZone::RootSplit { side: side(case["zone"]["side"].as_str().unwrap()) },
        };
        assert!(dock.apply_drop(&drag, &zone), "{}", case["name"]);
        assert_eq!(root_kind(&dock.root), case["expectedRootKind"].as_str().unwrap());
        for (id, path) in case["expectedPaths"].as_object().unwrap() {
            let expected: Vec<usize> = path.as_array().unwrap().iter().map(|part| part.as_u64().unwrap() as usize).collect();
            assert_eq!(find_stack_path(&dock.root, id, &mut Vec::new()), Some(expected), "{}: {id}", case["name"]);
        }
        assert_eq!(dock.window_template_id("world-2"), Some("top"));
        assert_eq!(dock.render_view(Some(&drag)).root, dock.root, "a palette drag never docks out an existing source");
    }
}
