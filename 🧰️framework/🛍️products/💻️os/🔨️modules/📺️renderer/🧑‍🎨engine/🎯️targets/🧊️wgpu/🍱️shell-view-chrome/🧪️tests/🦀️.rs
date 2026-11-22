//! 🍱️ Unit tests of shell-view-chrome; the work package moves the tests of its sources here.

use super::*;
use semio_renderer_wgpu_shell_dock::{UTILITY_CATEGORIES, UTILITY_CATEGORY_ORDER, UtilityRibbonSegment, UtilityTreeDirection, build_utility_ribbon_segments, dedupe_utility_nodes_by_id, find_pressed_utility_leaf_id, find_utility_group_path, group_utility_nodes_by_category, window_utility_bar_nodes, reconcile_utility_path, utility_ribbon_row_keys, utility_tree_selection};
use semio_ui_declaration::utilities::UtilityCategory;
use serde_json::{Value, json};

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../🧱️elements/🎛️UtilityTree/🧪️tests/🧫️fixtures/🌳️utility-tree/🔣️.json")).expect("utility tree fixture")
}

fn nodes(value: &Value) -> Vec<UtilityNode> {
    serde_json::from_value(value.clone()).expect("utility nodes")
}

fn strings(values: &Value) -> Vec<String> {
    values.as_array().expect("array").iter().map(|value| value.as_str().expect("string").to_string()).collect()
}

fn summary(node: &UtilityNode) -> Value {
    let wire = serde_json::to_value(node).expect("node wire");
    let mut out = serde_json::Map::new();
    for key in ["id", "kind", "iconId", "text", "order", "category"] {
        if let Some(value) = wire.get(key) {
            out.insert(key.into(), value.clone());
        }
    }
    if let Some(children) = wire.get("children").and_then(Value::as_array) {
        out.insert("childIds".into(), Value::Array(children.iter().map(|child| child["id"].clone()).collect()));
    }
    Value::Object(out)
}

/// 🗂️ LAW: nodes bucket into ordered category collections exactly as React's `groupUtilityNodesByCategory`.
#[test]
fn utility_category_grouping_replays_the_fixture() {
    for case in fixture()["grouping"].as_array().expect("grouping") {
        let categories: Vec<UtilityCategory> = if case["categories"].is_null() { UTILITY_CATEGORY_ORDER.to_vec() } else { serde_json::from_value(case["categories"].clone()).expect("categories") };
        let grouped: Vec<Value> = group_utility_nodes_by_category(&nodes(&case["nodes"]), &categories).iter().map(summary).collect();
        assert_eq!(Value::Array(grouped), case["expected"], "{}", case["name"]);
    }
    assert_eq!(UTILITY_CATEGORIES, [UtilityCategory::Selection, UtilityCategory::Utilities]);
}

#[test]
fn utility_nodes_dedupe_by_first_id() {
    let fixture = fixture();
    let lists: Vec<Vec<UtilityNode>> = fixture["dedupe"]["lists"].as_array().expect("lists").iter().map(nodes).collect();
    let ids: Vec<Value> = dedupe_utility_nodes_by_id(&lists).iter().map(|node| summary(node)["id"].clone()).collect();
    assert_eq!(Value::Array(ids), fixture["dedupe"]["expected"]);
}

/// 🎀️ LAW: segments, active-path reconciliation, pressed-leaf lookups and row buckets follow React's ribbon.
#[test]
fn ribbon_segments_paths_and_row_buckets_replay_the_fixture() {
    let fixture = fixture();
    let tree = nodes(&fixture["tree"]);
    for case in fixture["segments"].as_array().expect("segments") {
        let projected: Vec<Value> = build_utility_ribbon_segments(&tree, &strings(&case["path"]), 0)
            .iter()
            .map(|segment| match segment {
                UtilityRibbonSegment::Picker { collections, depth } => json!({ "kind": "picker", "depth": depth, "ids": collections.iter().map(|node| summary(node)["id"].clone()).collect::<Vec<_>>() }),
                UtilityRibbonSegment::Utilities { items, depth } => json!({ "kind": "utilities", "depth": depth, "ids": items.iter().map(|node| summary(node)["id"].clone()).collect::<Vec<_>>() }),
            })
            .collect();
        assert_eq!(Value::Array(projected), case["expected"], "segments for {}", case["path"]);
    }
    for case in fixture["reconcile"].as_array().expect("reconcile") {
        assert_eq!(reconcile_utility_path(&tree, &strings(&case["path"])), strings(&case["expected"]), "reconcile {}", case["path"]);
    }
    assert_eq!(find_pressed_utility_leaf_id(&tree).as_deref(), fixture["pressed"]["leafId"].as_str());
    for (target, expected) in fixture["pressed"]["groupPaths"].as_object().expect("group paths") {
        assert_eq!(find_utility_group_path(&tree, target, &[]).map(|path| json!(path)).unwrap_or(Value::Null), *expected, "group path of {target}");
    }
    let expected = &fixture["pressed"]["selection"];
    let selection = utility_tree_selection(&tree, expected["treeId"].as_str().expect("tree id")).expect("pressed selection");
    assert_eq!((selection.active_utility_id.as_str(), selection.window_id.as_str(), selection.generation), (expected["activeUtilityId"].as_str().expect("id"), expected["windowId"].as_str().expect("window"), expected["generation"].as_u64()));
    for case in fixture["rows"].as_array().expect("rows") {
        let direction = UtilityTreeDirection::from_tag(case["direction"].as_str().expect("direction")).expect("known direction");
        let keys = utility_ribbon_row_keys(&tree, &strings(&case["path"]), direction, case["utilityOptions"].as_bool().expect("options"), case["selectionRenderer"].as_bool().expect("renderer"));
        assert_eq!(keys, strings(&case["expected"]), "rows {} {}", case["direction"], case["path"]);
    }
}

/// 🪟️ LAW: the window bar unwraps selection/utilities buckets and Select groups and drops history and sync, as React's `utilityBarNode`.
#[test]
fn window_utility_bar_replays_the_fixture() {
    let fixture = fixture();
    let bar: Vec<Value> = window_utility_bar_nodes(&nodes(&fixture["windowBar"]["nodes"])).iter().map(|node| summary(node)["id"].clone()).collect();
    assert_eq!(Value::Array(bar), fixture["windowBar"]["expected"]);
}
