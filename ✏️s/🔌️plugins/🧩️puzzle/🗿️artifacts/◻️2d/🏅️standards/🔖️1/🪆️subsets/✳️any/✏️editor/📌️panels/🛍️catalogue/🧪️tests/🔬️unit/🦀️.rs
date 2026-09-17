use super::*;

//#region 🪟️WindowLaws
const RETIREMENT_DRAIN_STEPS: usize = 4096;
/// 🏗️ Past `TREE_WINDOW_DEFAULT_ROWS` and past `UI_BUILT_CHILDREN_MAX`, so the catalogue cannot fit
/// one paint — the shape a manifest-wide kind catalogue really has.
const SCALE_KINDS: usize = 200;

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

/// 🏗️ A synthetic fixture whose `meta.kindCatalogs.nodes` carries `kinds` rows.
fn scaled_scene(kinds: usize) -> Puzzle2dScene {
    let rows: Vec<Value> = (0..kinds).map(|index| json!({ "id": format!("kind-{index}"), "name": format!("Kind {index}") })).collect();
    Puzzle2dScene {
        fixture: json!({ "schema": crate::editor::puzzle2d::PUZZLE2D_FIXTURE_SCHEMA, "nodes": [], "edges": [], "meta": { "kindCatalogs": { "nodes": rows, "handles": [], "edges": [] } } }),
        runtime: crate::editor::puzzle2d::config::Puzzle2dPlayRuntime::default(),
        active_utility: String::new(),
        interaction: crate::editor::puzzle2d::Puzzle2dInteractionSnapshot::default(),
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

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> semio_framework_plugin::TreeWindowRequest {
    semio_framework_plugin::TreeWindowRequest { body_key: PUZZLE2D_PLAY_BODY_CATALOGUE.into(), node_key: node_key.into(), open, offset, rows }
}

/// 🪟️ (a)+(b) An oversized catalogue stamps every section's full extent, materialises no more than
/// its slice, closes the sections the author closed, and carries no `+N` row.
#[test]
fn the_catalogue_stamps_every_section_and_never_pages() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_KINDS);
    let windows = TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("an oversized catalogue must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    assert_eq!(window_of(nodes).expect("the nodes section must stamp its window").total as usize, SCALE_KINDS);
    assert!(!nodes.children.is_empty(), "the open nodes section must materialise its first window");
    assert!(nodes.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX);
    for section in [HANDLES_SECTION, EDGES_SECTION] {
        let container = child_of(&tree, section);
        assert_eq!(container.children.len(), 1, "an empty section shows exactly its placeholder row");
    }
    let body = body_json(tree);
    assert!(!body.contains(".more"), "a windowed catalogue carries no continuation key");
    assert!(!body.contains("\"+"), "a windowed catalogue carries no `+N` label");
    drain_retired_ui_owners();
}

/// 🪟️ (c) A host window materialises exactly `[offset, offset + rows)`; every catalogue row keeps
/// its own `addNode` binding, because a catalogue row is not a pick target.
#[test]
fn a_catalogue_window_materialises_its_slice_with_row_bindings_intact() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_KINDS);
    let (offset, rows) = (40u32, 9u32);
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![request(NODES_SECTION, Some(true), offset, rows)], ..Default::default() };
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_CATALOGUE);
    let tree = render(&scene, labels(), &windows).expect("a windowed catalogue must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    assert_eq!(window_of(nodes).expect("stamped window").offset, offset);
    assert_eq!(nodes.children.len(), rows as usize);
    let keys: Vec<String> = nodes.children.iter().map(|child| child.key.as_str().to_string()).collect();
    let expected: Vec<String> = (offset..offset + rows).map(|index| format!("{NODES_SECTION}.kind-{index}")).collect();
    assert_eq!(keys, expected);
    for row in nodes.children.iter() {
        assert_eq!(row.bindings.len(), 1, "catalogue row {} keeps its own addNode binding", row.key.as_str());
    }
    drop(tree);
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws
