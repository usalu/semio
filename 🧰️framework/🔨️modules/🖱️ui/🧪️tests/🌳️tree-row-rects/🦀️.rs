//! 🌳️ LAW: a tree row is laid out, painted and hit-tested on ONE row metric, and the rectangles a
//! pointer hits are the rectangles the layout published — never a second geometry.
//!
//! The defect: on `?plugin=generation3d&mode=generate` the Generations window's `Add Generation`
//! row mounted as a keyed `Stack` with no arena children of its own (a row's icon, label,
//! description and actions live inline on the owning `Tree`'s spec and are drawn by
//! `paint::retained_tree_node_step`), so `mounted_layout` measured it as a bare vertical stack —
//! `padding_standard * 2 = 6.4 px`, published at `[3.2, 16, 308.992, 6.4]` — while the painter
//! stepped its own private cursor by a hardcoded `24.0`. `events::hit_test` reads the published
//! rects, so five clicks on the painted label dispatched nothing
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-tree-row-hit-test-2026-09-12.md`).
//!
//! Oracle: `🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json`; its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts`, which pins the same rows against the
//! React `Tree`'s own row metric (`treeRowHeightPx`).

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::{ActionDescriptor, UiNode, UiPresence, UiState, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::engine::UiSurfaceToken;
use crate::wgpu::events::hit_test;
use crate::wgpu::flex::{LayoutJobStage, LayoutJobStep};
use crate::wgpu::mounted_layout::{MountedLayoutIdentity, MountedLayoutJob};
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{NodeKey, UiTree};
use crate::wgpu::Label;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🌳️tree-row-rects/🔣️.json")).expect("tree row rect fixture")
}

fn case<'a>(law: &'a Value, name: &str) -> &'a Value {
    law["cases"].as_array().expect("cases").iter().find(|entry| entry["name"] == name).unwrap_or_else(|| panic!("fixture case {name}"))
}

/// 🖱️ Every fixture row carries an activate action, exactly as the live `Add Generation` row does —
/// `events::is_plain_stack_container` refuses a row with no interaction of its own as a hit target,
/// so a row without one could never be clicked whatever its rect said.
fn activate(id: &str) -> ActionDescriptor {
    ActionDescriptor { controller_id: "law".to_string(), action: format!("activate.{id}"), args: None }
}

fn item(value: &Value) -> UiTreeItemNode {
    let id = value["id"].as_str().expect("item id");
    let mut node = UiTreeItemNode::base(id, Label::data(id));
    node.action = Some(activate(id));
    if value["hidden"].as_bool().unwrap_or(false) {
        node.presence = UiPresence { state: UiState::Hidden, ..UiPresence::default() };
    }
    if let Some(children) = value["items"].as_array() {
        node.items = Some(children.iter().map(item).collect());
    }
    if let Some(open) = value["defaultOpen"].as_bool() {
        node.default_open = Some(open);
    }
    node
}

fn section(value: &Value) -> UiTreeSectionNode {
    UiTreeSectionNode {
        id: value["id"].as_str().expect("section id").to_string(),
        label: value["label"].as_str().map(Label::data),
        default_open: Some(true),
        presence: UiPresence::default(),
        items: value["items"].as_array().map(|items| items.iter().map(item).collect()).unwrap_or_default(),
    }
}

fn tree_node(value: &Value) -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: value["sections"].as_array().expect("sections").iter().map(section).collect(),
        presence: UiPresence::default(),
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
}

/// 📐️ Mounts the case's authored tree and runs the REAL retained layout end to end — admit, shape,
/// measure, arrange, publish — so every rect asserted below is one `events::hit_test` reads.
fn laid_out(case: &Value) -> (UiTree, NodeId) {
    let width = case["treeWidth"].as_f64().expect("treeWidth") as f32;
    let height = case["treeHeight"].as_f64().expect("treeHeight") as f32;
    let mut tree = UiTree::new();
    tree.apply_tree(&tree_node(&case["tree"]));
    let root = tree.root.unwrap_or_else(|| panic!("tree row law root"));
    let identity = MountedLayoutIdentity { surface: UiSurfaceToken::new(1, 1), generation: 5, revision: 7, theme_revision: 11, viewport_revision: 13 };
    let mut job = MountedLayoutJob::try_new(&tree, root, identity, Theme::default(), width, height).unwrap_or_else(|fault| panic!("tree row law job: {fault:?}"));
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview = 0;
    while !job.is_admitted() {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(5), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview);
        assert!(!matches!(job.admit_one(&tree, &mut cx), LayoutJobStep::Fault(_)));
    }
    while job.stage() != LayoutJobStage::PublishResults {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(5), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview);
        let _ = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
    }
    let published = job.identity();
    loop {
        match job.publish_one(&mut tree, published) {
            LayoutJobStep::Complete => break,
            LayoutJobStep::Fault(fault) => panic!("tree row law publish: {fault}"),
            _ => {}
        }
    }
    (tree, root)
}

fn child_by_key(tree: &UiTree, parent: NodeId, key: &str) -> NodeId {
    tree.children(parent).find(|child| matches!(tree.node(*child).map(|node| &node.key), Some(NodeKey::Explicit(found)) if found == key)).unwrap_or_else(|| panic!("row {key}"))
}

fn node_at_path(tree: &UiTree, root: NodeId, path: &Value) -> NodeId {
    let mut current = root;
    for key in path.as_array().expect("path") {
        current = child_by_key(tree, current, key.as_str().expect("path segment"));
    }
    current
}

fn close(left: f32, right: f64) -> bool {
    (f64::from(left) - right).abs() <= 1e-4
}

#[test]
fn the_row_metric_is_the_one_the_fixture_was_pinned_against() {
    let law = law();
    let metrics = &law["metrics"];
    let theme = Theme::default();
    assert!(close(theme.tree_row_height, metrics["rowHeightPx"].as_f64().expect("rowHeightPx")));
    assert!(close(theme.tree_row_height, metrics["uiSpacingCompactPx"].as_f64().expect("uiSpacingCompactPx") * metrics["treeRowUiSpacing"].as_f64().expect("treeRowUiSpacing")));
    // 🌳️ The immediate-mode chrome twin reads the same token, so this target has ONE row pitch.
    assert!(close(crate::wgpu::widgets::TREE_ROW_HEIGHT, metrics["rowHeightPx"].as_f64().expect("rowHeightPx")));
    let row_metrics = crate::wgpu::layout::TreeRowMetrics::from_theme(&theme);
    assert_eq!(row_metrics.row_height, theme.tree_row_height);
    assert_eq!(row_metrics.header_height, theme.tree_row_height);
}

#[test]
fn every_fixture_row_is_published_at_the_rect_the_painter_draws_it_at() {
    let law = law();
    for entry in law["cases"].as_array().expect("cases") {
        let (tree, root) = laid_out(entry);
        let name = entry["name"].as_str().expect("case name");
        let root_layout = tree.mounted_layout(root).unwrap_or_else(|| panic!("{name} root layout"));
        assert!(close(root_layout.3, entry["treeHeight"].as_f64().expect("treeHeight")), "{name} tree height {}", root_layout.3);
        for rect in entry["rects"].as_array().expect("rects") {
            let id = node_at_path(&tree, root, &rect["path"]);
            let (x, y, width, height) = tree.mounted_layout(id).unwrap_or_else(|| panic!("{name} row layout"));
            let expected = (rect["x"].as_f64().expect("x"), rect["y"].as_f64().expect("y"), rect["width"].as_f64().expect("width"), rect["height"].as_f64().expect("height"));
            assert!(close(x, expected.0) && close(y, expected.1) && close(width, expected.2) && close(height, expected.3), "{name} {:?}: got ({x}, {y}, {width}, {height}), want {expected:?}", rect["path"]);
        }
        println!("[DEBUG] tree-row-rects case {name}: {} rows pinned", entry["rects"].as_array().map_or(0, Vec::len));
    }
}

#[test]
fn a_pointer_aimed_at_a_painted_row_hits_that_row() {
    let law = law();
    for probe in law["hitTests"].as_array().expect("hitTests") {
        let entry = case(&law, probe["case"].as_str().expect("case"));
        let (tree, root) = laid_out(entry);
        let x = probe["point"]["x"].as_f64().expect("x") as f32;
        let y = probe["point"]["y"].as_f64().expect("y") as f32;
        let hit = hit_test(&tree, root, x, y).unwrap_or_else(|| panic!("no hit at ({x}, {y}) — {}", probe["why"]));
        let key = match tree.node(hit).map(|node| &node.key) {
            Some(NodeKey::Explicit(key)) => key.clone(),
            other => panic!("hit an unkeyed node: {other:?}"),
        };
        let expects = probe["expects"].as_str().expect("expects");
        let want = expects.rsplit('/').next().expect("expected row id");
        assert_eq!(key, want, "hit at ({x}, {y}) — {}", probe["why"]);
        println!("[DEBUG] tree-row-rects hit ({x}, {y}) -> {key}");
    }
}

#[test]
fn the_defect_geometry_is_refused() {
    // ⚖️ The exact rect the live target published for `Add Generation`: a row measured as its own
    // padding. A row is never shorter than the metric its label is painted on.
    let law = law();
    let entry = case(&law, "two-sections-stack-in-order");
    let (tree, root) = laid_out(entry);
    let actions = child_by_key(&tree, root, "actions");
    let add = child_by_key(&tree, actions, "add-generation");
    let (_, _, _, height) = tree.mounted_layout(add).expect("add-generation layout");
    assert!(height >= Theme::default().tree_row_height, "row measured {height}, the defect published 6.4");
}
