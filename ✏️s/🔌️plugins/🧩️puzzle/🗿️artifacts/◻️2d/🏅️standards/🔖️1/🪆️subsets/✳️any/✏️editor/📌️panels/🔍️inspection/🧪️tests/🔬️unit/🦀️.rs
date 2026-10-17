use crate::editor::puzzle2d::config::PUZZLE2D_PLAY_BODY_PROPERTIES;
use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::snapshot::{Puzzle2dInteractionSnapshot, Puzzle2dScene, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA};
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::TreeWindows;
use semio_framework_pack_json::Value;
#[allow(unused_imports)]
use crate::editor::puzzle2d::{snapshot::board_snapshot_edges, snapshot::board_snapshot_nodes, snapshot::snapshot_target_regions, snapshot::puzzle_extension_id, interaction::ui_label, snapshot::PUZZLE2D_PLAY_CONTROLLER_ID};
#[allow(unused_imports)]
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Component, NumberStepperProps, Trigger};
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_desc;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_with_action;
#[allow(unused_imports)]
use semio_framework_plugin::ui_node_list;
#[allow(unused_imports)]
use semio_framework_plugin::ActionFactory;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTreeBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::PluginAssemblyError;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::UiFixedList;
#[allow(unused_imports)]
use semio_framework_plugin::UiListBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::UiMapBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::UiText;
#[allow(unused_imports)]
use semio_framework_plugin::UiValue;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
#[allow(unused_imports)]
use semio_framework_ui_contract as ui;
use crate::editor::puzzle2d::panels::inspection::{IDS_SECTION};
use crate::editor::puzzle2d::panels::inspection::*;

//#region 🪟️WindowLaws
const RETIREMENT_DRAIN_STEPS: usize = 4096;
/// 🏗️ A whole-board selection — far past what the deleted `IDS_ROWS = 8` truncation ever showed.
const SCALE_IDS: usize = 120;

fn drain_retired_ui_owners() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if semio_framework_ui_contract::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if semio_framework_ui_contract::close_ui_value_page_one() {
            break;
        }
    }
}

fn labels() -> &'static Puzzle2dLabels {
    crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))
}

/// 🏗️ A document of `ids` nodes with every one of them selected.
fn selected_scene(ids: usize) -> Puzzle2dScene {
    let nodes: Vec<Value> = (0..ids).map(|index| semio_framework_pack_json::json!({ "id": format!("node-{index}"), "text": format!("Node {index}"), "nodeKind": "capsule" })).collect();
    Puzzle2dScene {
        board_snapshot: semio_framework_pack_json::json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": nodes, "edges": [] }),
        runtime: crate::editor::puzzle2d::config::Puzzle2dPlayRuntime::default(),
        active_utility: String::new(),
        interaction: Puzzle2dInteractionSnapshot { granularity: crate::editor::puzzle2d::snapshot::PUZZLE2D_GRANULARITY_NODE.into(), selected: (0..ids).map(|index| format!("node-{index}")).collect(), ..Default::default() },
    }
}

fn window_of(node: &BuiltNode) -> Option<semio_framework_ui_contract::TreeWindow> {
    match &node.component {
        semio_framework_ui_contract::Component::TreeSection(props) => props.window,
        semio_framework_ui_contract::Component::TreeItem(props) => props.window,
        _ => None,
    }
}

/// 🧾️ The body as JSON. A built tree's children travel as retained pages, so the projection helper —
/// not `serde_json` on the root — is what walks and retires them.
fn body_json(tree: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("retire the rendered body")
}

fn child_of<'a>(node: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    node.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("container {key} must exist"))
}

fn request(open: Option<bool>, offset: u32, rows: u32) -> semio_framework_plugin::TreeWindowRequest {
    semio_framework_plugin::TreeWindowRequest { body_key: PUZZLE2D_PLAY_BODY_PROPERTIES.into(), node_key: IDS_SECTION.into(), open, offset, rows }
}

/// 🪟️ (a) The selected-id list is a windowed section stamping the whole selection, never a static
/// `+N` row.
#[test]
fn the_inspector_ids_section_stamps_the_whole_selection() {
    drain_retired_ui_owners();
    let scene = selected_scene(SCALE_IDS);
    let windows = TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("a whole-board selection must be admitted");
    let ids = child_of(&tree, IDS_SECTION);
    assert_eq!(window_of(ids).expect("the ids section must stamp its window").total as usize, SCALE_IDS);
    assert!(!ids.children.is_empty() && ids.children.len() <= SCALE_IDS);
    let body = body_json(tree);
    assert!(!body.contains(".more"), "a windowed inspector carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed inspector carries no `+N` label");
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed ids section stamps its total and materialises nothing.
#[test]
fn a_closed_inspector_ids_section_builds_no_row() {
    drain_retired_ui_owners();
    let scene = selected_scene(SCALE_IDS);
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![request(Some(false), 0, 16)], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_PROPERTIES);
    let tree = render(&scene, labels(), &windows).expect("a closed ids section must be admitted");
    let ids = child_of(&tree, IDS_SECTION);
    assert_eq!(window_of(ids).expect("a closed container still stamps its window").total as usize, SCALE_IDS);
    assert_eq!(ids.children.len(), 0);
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`, keyed by raw id.
#[test]
fn an_inspector_ids_window_materialises_exactly_its_slice() {
    drain_retired_ui_owners();
    let scene = selected_scene(SCALE_IDS);
    let (offset, rows) = (33u32, 7u32);
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![request(Some(true), offset, rows)], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_PROPERTIES);
    let tree = render(&scene, labels(), &windows).expect("a windowed ids section must be admitted");
    let ids = child_of(&tree, IDS_SECTION);
    assert_eq!(window_of(ids).expect("stamped window").offset, offset);
    let keys: Vec<String> = ids.children.iter().map(|child| child.key.as_str().to_string()).collect();
    let expected: Vec<String> = (offset..offset + rows).map(|index| format!("node-{index}")).collect();
    assert_eq!(keys, expected);
    drop(tree);
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws
