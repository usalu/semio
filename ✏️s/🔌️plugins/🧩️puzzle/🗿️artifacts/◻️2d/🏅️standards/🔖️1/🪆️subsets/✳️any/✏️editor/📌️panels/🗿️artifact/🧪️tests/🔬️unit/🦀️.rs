use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_nodes_section() {
    let mut app = concrete_forest_app();
    let json = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(json.contains("puzzle2d-play-document.nodes"));
    assert!(json.contains("seed-left-001"));
}

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_english_and_german_and_reuse() {
    let mut app = concrete_forest_app();
    let english = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
    let german_view = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let german = render_body_with_view(&mut app, PUZZLE2D_PLAY_BODY_LAYERS, &german_view);
    assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
    let reuse_view = semio_framework_plugin::ViewModel { terminology: semio_framework_plugin::Terminology::Reuse, ..Default::default() };
    let reuse = render_body_with_view(&mut app, PUZZLE2D_PLAY_BODY_LAYERS, &reuse_view);
    assert!(reuse.contains("Building components"));
}

//#region 🪟️WindowLaws
const RETIREMENT_DRAIN_STEPS: usize = 4096;
/// 🏗️ Two windows' worth of rows in each section — past `TREE_WINDOW_DEFAULT_ROWS` and past
/// `UI_BUILT_CHILDREN_MAX`, so every law below measures a document that cannot fit one paint.
const SCALE_NODES: usize = 200;
const SCALE_EDGES: usize = 199;

/// ♻️ Stands in for the reactor's own one-page-per-turn retirement pump: a released tree hands its
/// child backings and its `UiValue` pages back, and only closing them returns the process-wide
/// admission credit the next build needs.
fn drain_retired_ui_owners() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_ui_value_page_one() {
            break;
        }
    }
}

fn labels() -> &'static Puzzle2dLabels {
    crate::editor::puzzle2d::terminology::puzzle2d_labels(&semio_framework_plugin::ViewModel::default())
}

/// 🏗️ A synthetic fixture of `nodes` nodes and `edges` edges — the shape Nakagin has (180/179),
/// without depending on that asset's contents.
fn scaled_scene(nodes: usize, edges: usize) -> Puzzle2dScene {
    let node_values: Vec<Value> = (0..nodes).map(|index| serde_json::json!({ "id": format!("node-{index}"), "text": format!("Node {index}"), "nodeKind": "capsule", "x": index as f64, "y": 0.0 })).collect();
    let edge_values: Vec<Value> = (0..edges).map(|index| serde_json::json!({ "id": format!("edge-{index}"), "source": format!("node-{index}"), "target": format!("node-{}", index + 1), "edgeKind": "link" })).collect();
    Puzzle2dScene {
        fixture: serde_json::json!({ "schema": crate::editor::puzzle2d::PUZZLE2D_FIXTURE_SCHEMA, "nodes": node_values, "edges": edge_values }),
        runtime: crate::editor::puzzle2d::config::Puzzle2dPlayRuntime::default(),
        active_utility: String::new(),
        interaction: crate::editor::puzzle2d::Puzzle2dInteractionSnapshot::default(),
    }
}

/// 🪟️ The window a container published, if any.
fn window_of(node: &BuiltNode) -> Option<ui::TreeWindow> {
    match &node.component {
        ui::Component::TreeSection(props) => props.window,
        ui::Component::TreeItem(props) => props.window,
        _ => None,
    }
}

fn granularity_of(node: &BuiltNode) -> Option<String> {
    match &node.component {
        ui::Component::TreeItem(props) => props.granularity.as_ref().map(|value| value.as_str().to_string()),
        _ => None,
    }
}

fn child_of<'a>(node: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    node.children.iter().find(|child| child.key.as_str() == key).unwrap_or_else(|| panic!("container {key} must exist"))
}

fn view_with(requests: Vec<semio_framework_plugin::TreeWindowRequest>) -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel { tree_windows: requests, ..Default::default() }
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> semio_framework_plugin::TreeWindowRequest {
    semio_framework_plugin::TreeWindowRequest { body_key: PUZZLE2D_PLAY_BODY_LAYERS.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🪟️ (a) An oversized document: every container stamps its FULL extent and materialises no more
/// than its slice, and the body carries neither a `.more` key nor a `+N` label.
#[test]
fn the_outliner_stamps_every_container_and_never_pages() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_NODES, SCALE_EDGES);
    let windows = semio_framework_plugin::TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("an oversized outliner must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    let edges = child_of(&tree, EDGES_SECTION);
    assert_eq!(window_of(nodes).expect("the nodes section must stamp its window").total as usize, SCALE_NODES);
    assert_eq!(window_of(edges).expect("the edges section must stamp its window").total as usize, SCALE_EDGES);
    assert!(nodes.children.len() <= SCALE_NODES, "a section may never materialise more than its extent");
    assert!(nodes.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX, "a built node may not exceed the children contract");
    assert!(!nodes.children.is_empty(), "the open nodes section must materialise its first window");
    let body = serde_json::to_string(&tree).expect("serialize the outliner body");
    assert!(!body.contains(".more"), "a windowed body carries no continuation key: {body:.400}");
    assert!(!body.contains("\"+"), "a windowed body carries no `+N` label: {body:.400}");
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed container stamps its total and materialises nothing — both the author default
/// (`edges`) and an explicit host close (`nodes`).
#[test]
fn a_closed_outliner_container_stamps_its_total_and_builds_no_row() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_NODES, SCALE_EDGES);
    let view = view_with(vec![request(NODES_SECTION, Some(false), 0, 32)]);
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("a closed outliner must be admitted");
    for (section, total) in [(NODES_SECTION, SCALE_NODES), (EDGES_SECTION, SCALE_EDGES)] {
        let container = child_of(&tree, section);
        assert_eq!(window_of(container).expect("a closed container still stamps its window").total as usize, total);
        assert_eq!(container.children.len(), 0, "a closed container materialises no child");
    }
    drop(tree);
    drain_retired_ui_owners();
}

/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`, keyed by raw entity id.
#[test]
fn a_host_window_materialises_exactly_its_slice_keyed_by_raw_id() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_NODES, SCALE_EDGES);
    let (offset, rows) = (64u32, 12u32);
    let view = view_with(vec![request(NODES_SECTION, Some(true), offset, rows), request(EDGES_SECTION, Some(true), 0, 5)]);
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("a windowed outliner must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    assert_eq!(window_of(nodes).expect("stamped window").offset, offset);
    assert_eq!(nodes.children.len(), rows as usize);
    let keys: Vec<String> = nodes.children.iter().map(|child| child.key.as_str().to_string()).collect();
    let expected: Vec<String> = (offset..offset + rows).map(|index| format!("node-{index}")).collect();
    assert_eq!(keys, expected, "the window must be exactly [offset, offset + rows) keyed by raw id");
    let edges = child_of(&tree, EDGES_SECTION);
    assert_eq!(edges.children.len(), 5);
    assert_eq!(edges.children.iter().next().expect("first edge row").key.as_str(), "edge-0");
    drop(tree);
    drain_retired_ui_owners();
}

/// 🎯️ (d) Pick rows carry a granularity and no binding of their own; the tree root carries exactly
/// one `interactionSelect` binding for the whole domain.
#[test]
fn outliner_pick_rows_are_domain_bound_without_a_per_row_binding() {
    drain_retired_ui_owners();
    let scene = scaled_scene(24, 12);
    let view = view_with(vec![request(NODES_SECTION, Some(true), 0, 24), request(EDGES_SECTION, Some(true), 0, 12)]);
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("a domain-bound outliner must be admitted");
    assert_eq!(tree.bindings.len(), 1, "the tree root carries exactly one interactionSelect binding");
    assert_eq!(tree.bindings.iter().next().expect("root binding").action.as_str(), semio_framework_plugin::INTERACTION_SELECT_ACTION_ID);
    for (section, granularity) in [(NODES_SECTION, PUZZLE2D_GRANULARITY_NODE), (EDGES_SECTION, PUZZLE2D_GRANULARITY_EDGE)] {
        for row in child_of(&tree, section).children.iter() {
            assert_eq!(granularity_of(row).as_deref(), Some(granularity), "row {} must declare its granularity", row.key.as_str());
            assert_eq!(row.bindings.len(), 0, "row {} must carry no per-row binding", row.key.as_str());
        }
    }
    drop(tree);
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws
