use super::*;

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
    crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::default())
}

/// 🏗️ A document of `ids` nodes with every one of them selected.
fn selected_scene(ids: usize) -> Puzzle2dScene {
    let nodes: Vec<Value> = (0..ids).map(|index| serde_json::json!({ "id": format!("node-{index}"), "text": format!("Node {index}"), "nodeKind": "capsule" })).collect();
    Puzzle2dScene {
        fixture: serde_json::json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": nodes, "edges": [] }),
        runtime: crate::editor::puzzle2d::config::Puzzle2dPlayRuntime::default(),
        active_utility: String::new(),
        interaction: Puzzle2dInteractionSnapshot { granularity: crate::editor::puzzle2d::PUZZLE2D_GRANULARITY_NODE.into(), selected: (0..ids).map(|index| format!("node-{index}")).collect(), hovered: Vec::new() },
    }
}

fn window_of(node: &BuiltNode) -> Option<semio_framework_ui_contract::TreeWindow> {
    match &node.component {
        semio_framework_ui_contract::Component::TreeSection(props) => props.window,
        semio_framework_ui_contract::Component::TreeItem(props) => props.window,
        _ => None,
    }
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
    let windows = semio_framework_plugin::TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("a whole-board selection must be admitted");
    let ids = child_of(&tree, IDS_SECTION);
    assert_eq!(window_of(ids).expect("the ids section must stamp its window").total as usize, SCALE_IDS);
    assert!(!ids.children.is_empty() && ids.children.len() <= SCALE_IDS);
    let body = serde_json::to_string(&tree).expect("serialize the inspector body");
    assert!(!body.contains(".more"), "a windowed inspector carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed inspector carries no `+N` label");
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed ids section stamps its total and materialises nothing.
#[test]
fn a_closed_inspector_ids_section_builds_no_row() {
    drain_retired_ui_owners();
    let scene = selected_scene(SCALE_IDS);
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![request(Some(false), 0, 16)], ..Default::default() };
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_PROPERTIES);
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
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![request(Some(true), offset, rows)], ..Default::default() };
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_PROPERTIES);
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
