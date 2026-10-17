//! 🧪️ Aggregated unit tests of the `panels` leaves, one module per leaf.

mod document {
use crate::ProcedureScene;
use semio_framework_plugin::UiText;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use super::super::document::*;
use crate::editor::procedure::unit_tests::context::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_steps() {
    let mut app = imperative_app().await;
    assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_ARTIFACT).await.contains("imperative-play-document.steps"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(IMPERATIVE_PLAY_BODY_ARTIFACT));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use crate::editor::procedure::terminology::imperative_labels;
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, TreeWindows, ViewModel, INTERACTION_SELECT_ACTION_ID};

const STEPS_SECTION: &str = "imperative-play-document.steps";
const OVERSIZED: usize = 200;

/// 📜️ A procedure scene past one node's fixed child capacity — the exact program that used to make
/// `render()` return `Err("ui.fixed-capacity", "imperative step admission failed")`.
fn oversized_procedure(count: usize) -> crate::ProcedureScene {
    let path = imperative_engine::Path {
        steps: (0..count).map(|index| imperative_engine::Step { id: format!("step-{index:03}"), kind: "log.print".to_string(), params: Default::default(), bodies: Default::default() }).collect(),
    };
    crate::ProcedureScene { path, seed: Default::default() }
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: IMPERATIVE_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn window_law_node<'a>(root: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    fn walk<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| walk(child, key))
    }
    walk(root, key).unwrap_or_else(|| panic!("no node keyed {key}"))
}

fn window_law_extent(node: &BuiltNode) -> (u32, u32) {
    let window = match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => None,
    };
    let window = window.unwrap_or_else(|| panic!("{} stamps no window", node.key.as_str()));
    (window.total, window.offset)
}

fn window_law_keys(node: &BuiltNode) -> Vec<String> {
    node.children.iter().map(|child| child.key.as_str().to_string()).collect()
}

fn window_law_no_continuation(node: &BuiltNode) {
    assert!(!node.key.as_str().ends_with(".more"), "{} is a continuation row", node.key.as_str());
    if let Component::TreeItem(props) = &node.component {
        assert!(!props.label.0.as_str().starts_with('+'), "{} carries a +N label", node.key.as_str());
    }
    for child in node.children.iter() {
        window_law_no_continuation(child);
    }
}

fn window_law_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }
}

#[semio_framework_async_macros::async_test]
async fn an_oversized_procedure_stamps_the_full_total_and_materialises_at_most_its_slice() {
    let document = oversized_procedure(OVERSIZED);
    let tree = render(&document, imperative_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::unhosted()).expect("the document tree builds");
    let steps = window_law_node(&tree, STEPS_SECTION);
    assert_eq!(window_law_extent(steps), (OVERSIZED as u32, 0));
    assert!(steps.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", steps.children.len());
    window_law_no_continuation(&tree);
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let document = oversized_procedure(OVERSIZED);
    let view = window_law_view(vec![window_law_request(STEPS_SECTION, Some(false), 0, 32)]);
    let tree = render(&document, imperative_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::for_body(&view, IMPERATIVE_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let steps = window_law_node(&tree, STEPS_SECTION);
    assert_eq!(window_law_extent(steps), (OVERSIZED as u32, 0));
    assert_eq!(steps.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_canonical_step_row_id() {
    let document = oversized_procedure(OVERSIZED);
    let view = window_law_view(vec![window_law_request(STEPS_SECTION, Some(true), 70, 5)]);
    let tree = render(&document, imperative_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::for_body(&view, IMPERATIVE_PLAY_BODY_ARTIFACT)).expect("the document tree builds");
    let steps = window_law_node(&tree, STEPS_SECTION);
    assert_eq!(window_law_extent(steps), (OVERSIZED as u32, 70));
    assert_eq!(window_law_keys(steps), (70..75).map(|index| step_row_id(&format!("step-{index:03}"))).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn pick_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let document = oversized_procedure(3);
    let tree = render(&document, imperative_labels(&ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    for row in window_law_node(&tree, STEPS_SECTION).children.iter() {
        let Component::TreeItem(props) = &row.component else { panic!("{} is not a tree row", row.key.as_str()) };
        assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("step"));
        assert!(row.bindings.is_empty(), "{} must not pay for its own activate binding", row.key.as_str());
    }
}

//#endregion 🪟️WindowLaws
}

mod inspection {
use super::super::inspection::*;
use crate::editor::procedure::unit_tests::context::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn inspection_shows_step_count_summary() {
    let mut app = imperative_app().await;
    assert!(render_body(&mut app, IMPERATIVE_PLAY_BODY_INSPECTOR).await.contains("imperative-play-inspector.steps"));
}
}

mod inspection_2 {
use semio_framework_test_fixtures::fixture_text;

use crate::editor::procedure::terminology::ImperativeLabels;
use super::super::inspection::*;

fn project(node: semio_framework_plugin::BuiltNode) -> serde_json::Value {
    let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire semantic tree");
    serde_json::from_str(&text).expect("independent UI oracle")
}

#[test]
fn imperative_semantic_panels_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️panels.json")).expect("neutral UI vectors");
    let document = crate::procedure_derivable_scene(&crate::ProcedureSnapshot::default()).expect("the default procedure document derives its scene");
    for row in vectors["cases"].as_array().expect("locales") {
        let labels = semio_framework_plugin::resolve_labels::<ImperativeLabels>(&semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) });
        let tree = project(crate::editor::procedure::panels::document::render(&document, labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("document"));
        assert_eq!(tree["children"][0]["component"]["label"], row["document"]);
        let catalogue = crate::editor::procedure::panels::catalogue::render(labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("catalogue");
        let actions = catalogue.children[0].children.iter().map(|node| serde_json::to_value(&node.bindings[0].args).expect("independent action oracle")).collect::<Vec<_>>();
        let tree = project(catalogue);
        assert_eq!(tree["children"][0]["component"]["label"], row["catalogue"]);
        for (index, kind) in vectors["actionKinds"].as_array().expect("actions").iter().enumerate() {
            assert_eq!(actions[index]["kind"], *kind);
        }
        let tree = project(render(&document, labels).expect("inspector"));
        assert_eq!(tree["children"][0]["component"]["label"], row["inspection"]);
        assert_eq!(tree["children"][0]["children"][0]["component"]["label"], row["steps"]);
        let node = crate::editor::procedure::modes::edit::windows::main::render(&document, &vectors["runOutput"].to_string(), labels).expect("table");
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("table surface") };
        let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(props).expect("packed table");
        // 📊️ `TableWindowKit::render` emits the renderer table contract both hosts read: `columnsJson`
        // as `{id, label}` records and `rowsJson` as `{id, "<column index>": cell}` records. The
        // committed oracle spells what the USER reads — the locale-resolved column labels and the cell
        // text in column order — so the records are projected back to that, rather than the oracle being
        // rewritten into the renderer's keying.
        let columns: Vec<serde_json::Value> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.columns_json).expect("columns oracle").into_iter().map(|column| column["label"].clone()).collect();
        assert_eq!(serde_json::Value::Array(columns.clone()), row["columns"]);
        let rows: Vec<serde_json::Value> = serde_json::from_str::<Vec<serde_json::Value>>(&scene.rows_json)
            .expect("rows oracle")
            .into_iter()
            .map(|record| serde_json::Value::Array((0..columns.len()).map(|column| record[column.to_string().as_str()].clone()).collect()))
            .collect();
        assert_eq!(serde_json::Value::Array(rows), vectors["rows"]);
        project(node);
    }
    for node in [crate::editor::procedure::modes::edit::windows::script::render(&document).expect("editor text"), crate::viewer::procedure::modes::view::windows::script::render(&document).expect("viewer text")] {
        let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("text surface") };
        let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("packed text");
        assert_eq!(scene.language.as_deref(), Some("imperative"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(scene.settings_json.as_deref().expect("settings")).expect("settings oracle"), vectors["settings"]);
        project(node);
    }
}
}
