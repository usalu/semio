use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_nodes_section() {
    let mut app = concrete_forest_app();
    let json = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
    assert!(json.contains("puzzle2d-play-document.nodes"));
    assert!(json.contains("seed-left-001"));
    close_app(&mut app);
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
    close_app(&mut app);
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

/// 🧾️ The body as JSON. A built tree's children travel as retained pages, so the projection helper —
/// not `serde_json` on the root — is what walks and retires them.
fn body_json(tree: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("retire the rendered body")
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
    let windows = TreeWindows::unhosted();
    let tree = render(&scene, labels(), &windows).expect("an oversized outliner must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    let edges = child_of(&tree, EDGES_SECTION);
    assert_eq!(window_of(nodes).expect("the nodes section must stamp its window").total as usize, SCALE_NODES);
    assert_eq!(window_of(edges).expect("the edges section must stamp its window").total as usize, SCALE_EDGES);
    assert!(nodes.children.len() <= SCALE_NODES, "a section may never materialise more than its extent");
    assert!(nodes.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX, "a built node may not exceed the children contract");
    assert!(!nodes.children.is_empty(), "the open nodes section must materialise its first window");
    let body = body_json(tree);
    assert!(!body.contains(".more"), "a windowed body carries no continuation key: {body:.400}");
    assert!(!body.contains("\"+"), "a windowed body carries no `+N` label: {body:.400}");
    drain_retired_ui_owners();
}

/// 🪟️ (b) A closed container stamps its total and materialises nothing — both the author default
/// (`edges`) and an explicit host close (`nodes`).
#[test]
fn a_closed_outliner_container_stamps_its_total_and_builds_no_row() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_NODES, SCALE_EDGES);
    let view = view_with(vec![request(NODES_SECTION, Some(false), 0, 32)]);
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
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
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
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
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("a domain-bound outliner must be admitted");
    assert_eq!(tree.bindings.len(), 1, "the tree root carries exactly one interactionSelect binding");
    assert_eq!(tree.bindings.iter().next().expect("root binding").action.name.as_str(), semio_framework_plugin::INTERACTION_SELECT_ACTION_ID);
    for (section, granularity) in [(NODES_SECTION, PUZZLE2D_GRANULARITY_NODE), (EDGES_SECTION, PUZZLE2D_GRANULARITY_EDGE)] {
        for row in child_of(&tree, section).children.iter() {
            assert_eq!(granularity_of(row).as_deref(), Some(granularity), "row {} must declare its granularity", row.key.as_str());
            assert_eq!(row.bindings.len(), 0, "row {} must carry no per-row binding", row.key.as_str());
        }
    }
    drop(tree);
    drain_retired_ui_owners();
}

/// 🧾️ Every node record one body spends — the tree root, every section and every row counts once,
/// exactly as `SurfaceReconcileLimits::max_nodes` counts them.
fn node_records(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(node_records).sum::<usize>()
}

/// 🔑️ Every node key in one body. A windowed container is addressed by its authored key, so two
/// containers (or two rows) sharing one key inside a body make the host's `TreeWindowRequest`
/// ambiguous — the SDK refuses it.
fn node_keys(node: &BuiltNode, keys: &mut Vec<String>) {
    keys.push(node.key.as_str().to_string());
    for child in node.children.iter() {
        node_keys(child, keys);
    }
}

/// 🧾️ The whole-document law: one container holds more entries than a whole `UI_DOCUMENT_NODES` arena
/// AND every container is open at once asking for more rows than the arena could hold. Per-container
/// clamps alone do not save this body — only the SDK's body-wide node ledger does. Every container
/// must still stamp its FULL total, the body must reconcile inside `UI_DOCUMENT_NODES` records, and
/// nothing may be closed off with a `+N`.
#[test]
fn a_whole_open_document_stamps_every_total_and_stays_inside_the_body_node_ceiling() {
    drain_retired_ui_owners();
    let (nodes_len, edges_len) = (ui::UI_DOCUMENT_NODES + 72, ui::UI_DOCUMENT_NODES + 40);
    let scene = scaled_scene(nodes_len, edges_len);
    let view = view_with(vec![request(NODES_SECTION, Some(true), 0, 512), request(EDGES_SECTION, Some(true), 0, 512)]);
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("a fully open oversized outliner must be admitted");
    for (section, total) in [(NODES_SECTION, nodes_len), (EDGES_SECTION, edges_len)] {
        let container = child_of(&tree, section);
        assert_eq!(window_of(container).expect("every container stamps its window").total as usize, total, "container {section} must stamp its FULL total however few rows it could afford");
    }
    let records = node_records(&tree);

    // 🔑️ One body, one key per node: a duplicate would make a host window request ambiguous.
    let mut keys = Vec::new();
    node_keys(&tree, &mut keys);
    let mut unique = keys.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), keys.len(), "every node key in this body is unique ({} of {} distinct)", unique.len(), keys.len());
    assert!(records <= ui::UI_DOCUMENT_NODES, "the whole open document must reconcile inside one surface arena, spent {records} of {}", ui::UI_DOCUMENT_NODES);
    let body = body_json(tree);
    assert!(!body.contains(".more"), "no continuation row closes an exhausted container: {body:.400}");
    assert!(!body.contains("\"+"), "and no `+N` label either: {body:.400}");
    drain_retired_ui_owners();
}
//#endregion 🪟️WindowLaws

//#region 🙈️RowFlagLaws
fn row_actions_of(node: &BuiltNode) -> Vec<(String, bool)> {
    match &node.component {
        ui::Component::TreeItem(props) => props
            .row_actions
            .iter()
            .map(|action| {
                let asked = match action.action.args.as_ref() {
                    Some(semio_framework_plugin::UiValue::Map(map)) => map.iter().find(|(key, _)| key.as_str() == "value").and_then(|(_, value)| match value {
                        semio_framework_plugin::UiValue::Bool(value) => Some(value),
                        _ => None,
                    }),
                    _ => None,
                };
                (action.icon.as_str().to_string(), asked.expect("a row toggle always names the state it asks for"))
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn flagged_scene(hidden: bool, locked: bool) -> Puzzle2dScene {
    let mut scene = scaled_scene(1, 0);
    scene.fixture["nodes"][0]["hidden"] = serde_json::json!(hidden);
    scene.fixture["nodes"][0]["locked"] = serde_json::json!(locked);
    scene
}

/// 🙈️ An outliner node row carries hide + lock toggles whose `value` is ALWAYS the inverse of the
/// row's current state — the alternating-toggle law puzzle3d's own outliner once broke by hardcoding
/// `true`, which made "Show"/"Unlock" re-apply the state the row was already in.
#[test]
fn outliner_node_rows_toggle_hide_and_lock_to_the_inverse_state() {
    drain_retired_ui_owners();
    for (hidden, locked) in [(false, false), (true, false), (false, true), (true, true)] {
        let scene = flagged_scene(hidden, locked);
        let view = view_with(vec![request(NODES_SECTION, Some(true), 0, 4)]);
        let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
        let tree = render(&scene, labels(), &windows).expect("a flagged outliner must be admitted");
        let row = child_of(child_of(&tree, NODES_SECTION), "node-0");
        let actions = row_actions_of(row);
        assert_eq!(actions.len(), 2, "a node row carries exactly the hide and lock toggles");
        assert_eq!(actions[0], (if hidden { "eye-off" } else { "eye" }.to_string(), !hidden), "the hide toggle asks for the inverse of hidden={hidden}");
        assert_eq!(actions[1], (if locked { "lock" } else { "lock-open" }.to_string(), !locked), "the lock toggle asks for the inverse of locked={locked}");
        drop(tree);
        drain_retired_ui_owners();
    }
}

/// 🔗️ Edge rows carry NO inline toggles — mirroring puzzle3d's attraction rows, and halving the
/// per-document argument-arena cost on a Nakagin-scale board.
#[test]
fn outliner_edge_rows_carry_no_inline_toggles() {
    drain_retired_ui_owners();
    let scene = scaled_scene(4, 3);
    let view = view_with(vec![request(EDGES_SECTION, Some(true), 0, 3)]);
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("an outliner with edges must be admitted");
    for row in child_of(&tree, EDGES_SECTION).children.iter() {
        assert!(row_actions_of(row).is_empty(), "edge row {} must carry no inline toggle", row.key.as_str());
    }
    drop(tree);
    drain_retired_ui_owners();
}

/// 🏷️ A Nakagin-scale document still materialises its first window with toggles attached — the
/// graceful-degradation law: a row the argument arena cannot afford keeps the row and drops only its
/// toggles, so a section can never end at zero rows.
#[test]
fn an_oversized_outliner_still_materialises_rows_when_toggles_cannot_be_afforded() {
    drain_retired_ui_owners();
    let scene = scaled_scene(SCALE_NODES, SCALE_EDGES);
    let view = view_with(vec![request(NODES_SECTION, Some(true), 0, 128)]);
    let windows = TreeWindows::for_body(&view, PUZZLE2D_PLAY_BODY_LAYERS);
    let tree = render(&scene, labels(), &windows).expect("an oversized outliner must be admitted");
    let nodes = child_of(&tree, NODES_SECTION);
    assert!(!nodes.children.is_empty(), "the open nodes section must materialise rows whatever the arena can afford");
    assert_eq!(window_of(nodes).expect("stamped window").total as usize, SCALE_NODES);
    drop(tree);
    drain_retired_ui_owners();
}
//#endregion 🙈️RowFlagLaws
