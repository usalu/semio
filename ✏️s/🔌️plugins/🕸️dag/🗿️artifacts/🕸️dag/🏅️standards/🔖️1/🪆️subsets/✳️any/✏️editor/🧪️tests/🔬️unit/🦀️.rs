pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::new_app_with_registry as framework_new_app_with_registry;
    use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type DagApp = VcsArtifactApp<EditorApp<DagPlayApp>>;
    
    /// 🧪️ An app instance using its declared tool catalog and concrete factories.
    pub async fn new_app() -> DagApp {
        new_app_with_registry().await
    }
    
    /// ✏️ Adapts `create_dag_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`'s framework test context signature (contract §2.5 gap 3,
    /// not yet updated for the `AppDefinition`-returning convention) still expects.
    pub fn dag_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_dag_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> DagApp {
        framework_new_app_with_registry::<EditorApp<DagPlayApp>>(dag_app_manifest_for_tests).await
    }
    
    pub async fn render(app: &mut DagApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
    }
}

use super::*;

//#region 🧪️RetainedConfigOracle
#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_snapshot_input() {
    let base = DagConfig::default();
    let mut expected = serde_json::to_value(&base).expect("JSON oracle base");
    expected["cameraX"] = serde_json::json!(1.0);
    expected["cameraY"] = serde_json::json!(2.0);
    expected["cameraZoom"] = serde_json::json!(3.0);
    let (post, inverse, _) = prepare_dag_config(&base, DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 })).expect("bounded config candidate");
    assert_eq!(serde_json::to_value(post).expect("JSON oracle post"), expected);
    assert!(matches!(&inverse[0], DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x, y, zoom }) if (*x, *y, *zoom) == (base.camera_x, base.camera_y, base.camera_zoom)));
    assert!(dag_config_footprint(&DagConfigMutation::ReplaceConfig(crate::editor::dag::config::ReplaceConfig { config: base })).is_err());
    assert_eq!(DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024, 4_096);
}
//#endregion 🧪️RetainedConfigOracle
use crate::editor::dag::unit_tests::context::{new_app_with_registry, DagApp};
use semio_framework_plugin::PluginApp;

//#region 🔖️CommandSurface
/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<DagCommand> {
    vec![
        DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: Some(10.0), y: None }),
        DagCommand::RemoveNode(remove_node::RemoveNode { node_id: "n1".into() }),
        DagCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        DagCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
            operations: vec![
                node_graph_edit::DagNodeGraphEditOp::SetFixture { fixture_json: "{}".into() },
                node_graph_edit::DagNodeGraphEditOp::DeleteSelection,
                node_graph_edit::DagNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            ],
        }),
        DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        DagCommand::Disconnect(disconnect::Disconnect { edge_id: "e1".into() }),
        DagCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: "n1".into(), value: "renamed".into() }),
        DagCommand::Reorganize(reorganize::Reorganize {}),
        DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() }),
        DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 } }),
        DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
    ]
}

/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
    let commands = every_command();
    assert_eq!(commands.len(), 12, "every DagCommand row must be covered by every_command()");
    let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
    let expectations: Vec<(&str, DagCommand)> = vec![
        ("add-node", DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: Some(10.0), y: None })),
        ("remove-node", DagCommand::RemoveNode(remove_node::RemoveNode { node_id: "n1".into() })),
        ("delete-selection", DagCommand::DeleteSelection(delete_selection::DeleteSelection {})),
        ("node-graph-edit", DagCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: Vec::new() })),
        ("connect-media-ports", DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() })),
        ("disconnect", DagCommand::Disconnect(disconnect::Disconnect { edge_id: "e1".into() })),
        ("move-media-node", DagCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 })),
        ("rename-dag-node", DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: "n1".into(), value: "renamed".into() })),
        ("reorganize", DagCommand::Reorganize(reorganize::Reorganize {})),
        ("patch-dag-nodes", DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["n1".into()], field: "value".into(), value: "5".into() })),
        ("node-graph-viewport", DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } })),
        ("graph-pointer-down", DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {})),
    ];
    for (expected_keyword, command) in expectations {
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
    }
}

/// ⚖️ The row whose `Option` fields make `None`/`Some` distinct wire cases (`AddNode` is the only
/// `DagCommand` row with `Option` fields), pinned to the exact bytes captured from the pre-merge
/// `dag_protocol` crate (this ticket's `🧪️wire-baseline-before.txt`, row 1). A regression here is a
/// real format break, not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(DagCommand, &str, &str); 1] = [(DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: Some(10.0), y: None }), "add-node add-node kind=slider x=10", "01000106736c696465720200060001050000000000002440")];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_dag_app()).expect("app definition json");
    assert!(json.contains(DAG_PLAY_WINDOW_MAIN), "main window kind missing from the manifest: {json}");
    assert!(json.contains(DAG_PLAY_WINDOW_COMPILED), "compiled window kind missing from the manifest: {json}");
    assert!(json.contains(edit::DAG_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [DAG_PLAY_BODY_DOCUMENT, DAG_PLAY_BODY_CATALOGUE, DAG_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("graph.dag"), "artifact kind missing from the manifest");
}

// 🚧️ SDK GAP (contract §2.4): `Editor::builder(...)`/`.build_definition()` returns a bare
// `AppDefinition` with no `.examples` slot — the old `manifest_includes_the_demo_example` test
// (asserting `create_dag_app().examples` contained "demo") no longer applies; the app-level example
// registration this test guarded is dropped along with `.example_source(...)` (see the doc comment
// on `create_dag_app`'s `.build_definition()` call), not silently — reported in the migration report.

#[semio_framework_async_macros::async_test]
async fn every_declared_action_is_registered() {
    let definition = create_dag_app();
    for command in ["addNode", "removeNode", "deleteSelection", "nodeGraphEdit", "connectMediaPorts", "disconnect", "moveMediaNode", "renameDagNode", "reorganize", "patchDagNodes", "nodeGraphViewport", "graphPointerDown"] {
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
    }
}

/// 🕹️ `graph` is declared once, node/edge granularities, `Topology` hierarchy, scoped to the main
/// window — the framework auto-injects the six interaction actions for it (never app-declared).
#[semio_framework_async_macros::async_test]
async fn declares_the_graph_interaction_domain_scoped_to_the_main_window() {
    let definition = create_dag_app();
    let interaction = definition.interactions.iter().find(|def| def.id == DAG_PLAY_INTERACTION_DOMAIN).expect("graph domain declared");
    assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec!["node", "edge"]);
    assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
    assert!(interaction.hover.transitive, "hovering a node must cover its downstream descendants");
    assert!(!interaction.selection.transitive, "selection must NOT cascade into downstream nodes — a dependent is not a structural child");
    let main_window = definition.window_kinds.iter().find(|window| window.id == DAG_PLAY_WINDOW_MAIN).expect("main window declared");
    assert!(main_window.interactions.contains(&InteractionRef::new(DAG_PLAY_INTERACTION_DOMAIN)));
}

/// 🌳️ `interaction_topology` derives every node's parent from its first incoming edge's source, and
/// registers every edge as a sibling child of that same source — enough structure for
/// `validate_state` to prune a stale selection the moment `removeNode`/`disconnect` deletes its
/// target, and for transitive hover to cover a node's downstream nodes and edges.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_covers_every_node_and_edge_via_their_edges() {
    let app: DagApp = new_app_with_registry().await;
    let snapshot = app.snapshot().expect("snapshot");
    let node_id = snapshot.nodes().first().expect("seed node").id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = DagConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let topology = DagPlayApp::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(DAG_PLAY_INTERACTION_DOMAIN).expect("graph domain topology present");
    assert!(domain.ordered.iter().any(|node| node.id == node_id && node.granularity == "node"), "every seed node is registered");
    assert_eq!(domain.ordered.iter().filter(|node| node.granularity == "edge").count(), snapshot.edges().len(), "every seed edge is registered");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️ContextMenu
/// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small even with a large
/// selection, and the known `deleteSelection` destructive row (dispatched via `nodeGraphEdit` —
/// `NodeGraphDeleteDispatch::ViaNodeGraphEdit`) is always last, either as a top-level leaf or as the
/// tail of its group.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    use semio_framework_plugin::{ContextMenuHit, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

    let mut app: DagApp = new_app_with_registry().await;
    let node_ids: Vec<String> = app.snapshot().expect("projection").nodes().iter().map(|node| node.id.clone()).collect();
    // 🕹️ The click-carried `request.surface.selection` drives the menu directly —
    // `dag_context_menu_items`'s own `selected` fallback param is always `&[]` now (`render`/
    // `context_menu` carry no `InteractionView`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![ContextMenuHit { domain: "node".into(), id: node_ids[0].clone(), label: None }],
            selection: vec![ContextMenuSelectionGroup { domain: "node".into(), ids: node_ids.clone() }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection (via nodeGraphEdit) must be last: {menu:?}");
}
//#endregion 🔖️ContextMenu

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::dag::unit_tests::context::{new_app, render};
    let mut app = new_app().await;
    assert!(render(&mut app, "dag.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn whole_document_operation_is_not_supported_as_an_in_history_mutation() {
    let replacement = crate::default_snapshot();
    assert!(DagPlayApp::whole_document_operation(replacement).is_none(), "whole-document replace goes through ArtifactStore::reset, never a mutation");
}

/// 🧬️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
/// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    semio_framework_plugin::artifact_app_laws::assert_two_instances_converge::<EditorApp<DagPlayApp>, (bool, bool)>(
        "mem://dag-convergence",
        DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }),
        DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }),
        |app| {
            let projection = app.snapshot().expect("projection");
            let nodes = projection.nodes();
            (nodes.iter().any(|node| matches!(node.kind, semio_framework_artifact_infinite_dag::DagNodeKind::Note { .. })), nodes.iter().any(|node| matches!(node.kind, semio_framework_artifact_infinite_dag::DagNodeKind::Slider { .. })))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_dag() {
    semio_framework_plugin::artifact_app_laws::assert_ingest_idempotent::<EditorApp<DagPlayApp>, usize>(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), |app| app.snapshot().expect("projection").nodes().len()).await;
}
//#endregion 🔖️CrossCutting
