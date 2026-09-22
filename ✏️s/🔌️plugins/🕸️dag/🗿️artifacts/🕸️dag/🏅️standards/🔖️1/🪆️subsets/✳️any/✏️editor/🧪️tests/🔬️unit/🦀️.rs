pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members as framework_new_app_with_registry;
    use semio_framework_plugin::{EditorApp, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type DagApp = VcsArtifactApp<EditorApp<DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    
    /// 🧪️ An app instance using its declared tool catalog and concrete factories, bound to the live
    /// runtime instance `meta("local")` addresses. Binding is mandatory now that every document verb
    /// is classified `Migrated`: an unbound wrapper answers every typed dispatch
    /// `interactive-job.live-instance: typed command does not belong to the mounted live app instance`.
    pub async fn new_app() -> DagApp {
        let mut app = new_app_with_registry().await;
        semio_framework::io::resolve_ready(app.bind_instance_id(1));
        app
    }
    
    /// ✏️ Adapts `create_dag_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`'s framework test context signature (contract §2.5 gap 3,
    /// not yet updated for the `AppDefinition`-returning convention) still expects.
    pub fn dag_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_dag_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> DagApp {
        framework_new_app_with_registry::<EditorApp<DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(dag_app_manifest_for_tests).await
    }
    
    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does, draining
    /// EVERY result page — on a mounted app `dispatch_typed` only QUEUES the operation, so a test
    /// reading `app.snapshot()` straight afterwards would observe the pre-dispatch document.
    pub async fn settle(app: &mut DagApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("local").instance_id).await.expect("settle the typed operation");
    }

    /// 🎛️ Dispatches one typed command and settles its publication.
    pub async fn dispatch(app: &mut DagApp, command: DagCommand) -> semio_framework_plugin::InvocationResult {
        let result = app.dispatch_typed(command, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("dispatch");
        settle(app).await;
        result
    }

    /// 🧹️ Closes every store the wrapper opened. A live `ArtifactStore` asserts in `Drop` unless it
    /// was driven to its terminal-empty shallow shell, so every fixture that mounts an app must end here.
    pub fn close(app: &mut DagApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(app);
    }

    /// 🖼️ Projects one rendered tree through the retained page transport — a bare `serde_json` of
    /// `tree.root` cannot see `BuiltChildren`, whose rows only exist on the retained transport.
    pub async fn render(app: &mut DagApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
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
                node_graph_edit::DagNodeGraphEditOp::SetHostSnapshot { host_snapshot_json: "{}".into() },
                node_graph_edit::DagNodeGraphEditOp::DeleteSelection,
                node_graph_edit::DagNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            ],
        }),
        DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        DagCommand::Disconnect(disconnect::Disconnect { edge_id: "e1".into() }),
        DagCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        DagCommand::RenameDagNode(rename_dag_node::RenameDagNode { old_id: "n1".into(), value: "renamed".into() }),
        DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() }),
        DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.5 } }),
        DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
    ]
}

/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
    let commands = every_command();
    assert_eq!(commands.len(), 11, "every DagCommand row must be covered by every_command()");
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
        ("patch-dag-nodes", DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["n1".into()], field: "value".into(), value: "5".into() })),
        ("node-graph-viewport", DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } })),
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
    for body in [DAG_PLAY_BODY_ARTIFACT, DAG_PLAY_BODY_CATALOGUE, DAG_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("graph.dag"), "artifact kind missing from the manifest");
}

// 🚧️ SDK GAP (contract §2.4): `Editor::builder(...)`/`.build_definition()` returns a bare
// `AppDefinition` with no `.examples` slot — the old `manifest_includes_the_demo_example` test
// (asserting `create_dag_app().examples` contained "demo") no longer applies; the app-level example
// registration this test guarded is dropped along with `.example_source(...)` (see the doc comment
// on `create_dag_app`'s `.build_definition()` call), not silently — reported in the migration report.

/// 🪟️ Every verb this app declares must be DISPATCHABLE from a window kind. Since ticket
/// 26/09/18 slice DS1 an app-level `.action_with(...)` is no longer cloned into every
/// `WindowKindDefinition.actions` (the copy made a package descriptor grow as
/// `apps × window kinds × actions`); the union a shell actually offers is
/// `semio_framework::window_kind_actions` — the same predicate the plugin host itself uses
/// (`🔌️plugin/🦀️.rs:7520`). Reading `window.actions` alone now sees only the rows a window
/// claims for itself, so this law reads through the resolver instead.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_is_registered() {
    let definition = create_dag_app();
    let dispatchable: Vec<String> = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window).into_iter().map(|action| action.id.clone())).collect();
    for command in ["addNode", "removeNode", "deleteSelection", "nodeGraphEdit", "connectMediaPorts", "disconnect", "moveMediaNode", "renameDagNode", "patchDagNodes", "nodeGraphViewport", "graphPointerDown"] {
        assert!(dispatchable.iter().any(|id| id == command), "registry declares {command}; dispatchable: {dispatchable:?}");
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
    let mut app: DagApp = new_app_with_registry().await;
    let snapshot = app.snapshot().expect("snapshot");
    let node_id = snapshot.nodes().first().expect("seed node").id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = DagConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let topology = DagPlayApp::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(DAG_PLAY_INTERACTION_DOMAIN).expect("graph domain topology present");
    let registered_node = domain.ordered.iter().any(|node| node.id == node_id && node.granularity == "node");
    let registered_edges = domain.ordered.iter().filter(|node| node.granularity == "edge").count();
    let seed_edges = snapshot.edges().len();
    crate::editor::dag::unit_tests::context::close(&mut app);
    assert!(registered_node, "every seed node is registered");
    assert_eq!(registered_edges, seed_edges, "every seed edge is registered");
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
    let ordered = last_is_destructive_leaf || last_is_group_ending_in_destructive;
    let rendered = format!("{menu:?}");
    crate::editor::dag::unit_tests::context::close(&mut app);
    assert!(ordered, "known destructive deleteSelection (via nodeGraphEdit) must be last: {rendered}");
}
//#endregion 🔖️ContextMenu

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::dag::unit_tests::context::{close, new_app, render};
    let mut app = new_app().await;
    let json = render(&mut app, "dag.play.nope").await;
    close(&mut app);
    assert!(json.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn whole_document_operation_is_not_supported_as_an_in_history_mutation() {
    let replacement = crate::default_snapshot();
    assert!(DagPlayApp::whole_document_operation(replacement).is_none(), "whole-document replace goes through ArtifactStore::reset, never a mutation");
}

/// 🧬️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
/// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
/// `artifact_app_laws::assert_two_registered_instances_converge` replayed over THIS crate's harness: the
/// registry-less law faults `interactive-job.catalog-authority` at construction, and the registered twin
/// builds member-less apps, whose genesis refuses dag's derived `s.stdio.semio@v1/graph` child ("not
/// declared by this app's member roster"). Same law, same shape: settle each edit, fold both ways, commit
/// a checkpoint on A, fold it on B, and the probe must agree after each exchange.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    use crate::editor::dag::unit_tests::context::{new_app, DagApp};
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, settle_registered_typed_operation};
    use store::MemoryBackbone;
    fn probe(app: &DagApp) -> (bool, bool) {
        let projection = app.snapshot().expect("projection");
        let nodes = projection.nodes();
        (nodes.iter().any(|node| matches!(node.kind, semio_framework_artifact_infinite_dag::DagNodeKind::Note { .. })), nodes.iter().any(|node| matches!(node.kind, semio_framework_artifact_infinite_dag::DagNodeKind::Slider { .. })))
    }
    let mut instance_a = new_app().await;
    let mut instance_b = new_app().await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://dag-convergence", "mem://dag-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    let genesis = probe(&instance_a);
    let receiver = meta("actor-a").instance_id;
    instance_a.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), &meta("actor-a")).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut instance_a, receiver).await.expect("a's edit publishes");
    instance_b.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }), &meta("actor-b")).await.expect("b applies its edit");
    settle_registered_typed_operation(&mut instance_b, receiver).await.expect("b's edit publishes");
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");
    assert_eq!(probe(&instance_a), probe(&instance_b), "both instances must converge on the same snapshot");
    assert_eq!(probe(&instance_a), (true, true), "each instance holds both disjoint edits");
    let admitted = instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("a commits a checkpoint");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut instance_a, admitted).await.expect("a's checkpoint commit settles");
    settle_registered_typed_operation(&mut instance_a, receiver).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone().await.expect("b folds a's checkpoint");
    assert_eq!(probe(&instance_a), probe(&instance_b), "a replicated checkpoint keeps both instances converged");
    assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close_registered_fixture_app(&mut instance_a);
    close_registered_fixture_app(&mut instance_b);
}

/// 🔁️ `artifact_app_laws::assert_ingest_idempotent` over THIS crate's registered, bound harness: the
/// registry-less law faults `interactive-job.catalog-authority`, and the registered twin binds no live
/// instance (`interactive-job.live-instance`). Same law, same shape — a sender on a memory backbone, its
/// envelopes replayed twice onto a fresh receiver.
#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_dag() {
    use crate::editor::dag::unit_tests::context::new_app;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, settle_registered_typed_operation};
    use store::{Backbone, BackboneMessage, MemoryBackbone};
    let mut sender = new_app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://dag-idempotent", "mem://dag-idempotent").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach sender");
    let genesis = sender.snapshot().expect("projection").nodes().len();
    sender.dispatch_typed(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), &meta("local")).await.expect("apply command");
    settle_registered_typed_operation(&mut sender, meta("local").instance_id).await.expect("the add publishes");
    assert_eq!(sender.snapshot().expect("projection").nodes().len(), genesis + 1, "the sender applied its edit");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the add reached the backbone");
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = new_app().await;
    receiver.ingest_operations(&operations).await.expect("ingest once");
    let once = receiver.snapshot().expect("projection").nodes().len();
    assert_eq!(once, genesis + 1, "the replayed add applies");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("projection").nodes().len(), once, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("sender releases its backbone");
    close_registered_fixture_app(&mut sender);
    close_registered_fixture_app(&mut receiver);
}
//#endregion 🔖️CrossCutting

//#region 🧵️RetainedToolCatalog
/// 🧾️ The exact three-way join the guest's own `interactive-job.catalog-authority` check performs at
/// boot: every retained id is declared `Migrated`, carries a publication contract, and is served by
/// the ONE registered factory type the proofs are bound to. dag shipped two factories over one owner
/// (`DagConfigCommandJobFactory` for `nodeGraphViewport`, a second for the document verbs) and
/// trapped the guest with `typed_join=false` before it could paint a single window — ticket
/// 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice B2b.
#[test]
fn every_retained_tool_id_is_migrated_contracted_and_served_by_one_factory() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    assert_eq!(DAG_RETAINED_TOOL_IDS.len(), DAG_RETAINED_PUBLICATION_CONTRACTS.len());
    for tool_id in DAG_RETAINED_TOOL_IDS {
        assert!(DAG_RETAINED_PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id), "{tool_id} has no publication contract");
    }
    assert_eq!(<DagRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, DAG_RETAINED_TOOL_IDS, "the proofs bind one factory type, so it must serve every retained id");
    let definition = create_dag_app();
    for window in definition.window_kinds.iter() {
        for action in window.actions.iter().filter(|action| DAG_RETAINED_TOOL_IDS.contains(&action.id.as_str())) {
            assert_eq!(action.semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{} is retained but not Migrated", action.id);
        }
    }
}

/// 🌉️ The `{action,args}` bridge resolves every id a shell can express as a flat `{action, args}`
/// pair, including the example picker's `setActiveExample`, for which dag declared no command at all
/// until this slice. The five canvas-gesture verbs carry payloads no such pair can state
/// (edge/port/patch structures), so the bridge faults them `dag.unhandled-action` on purpose rather
/// than inventing a lossy decoding — they stay reachable only through the typed command channel.
#[test]
fn command_from_action_resolves_every_flat_verb_and_names_the_gesture_only_ones() {
    use semio_framework_plugin::ArtifactEditor;
    const GESTURE_ONLY: &[&str] = &["nodeGraphEdit", "connectMediaPorts", "moveMediaNode", "renameDagNode", "patchDagNodes"];
    for tool_id in DAG_RETAINED_TOOL_IDS.iter().filter(|tool_id| !GESTURE_ONLY.contains(tool_id)) {
        let command = DagPlayApp::command_from_action(tool_id, None).unwrap_or_else(|error| panic!("{tool_id} has no bridge: {error:?}"));
        assert_eq!(command.command_id(), *tool_id);
    }
    for tool_id in GESTURE_ONLY {
        let error = DagPlayApp::command_from_action(tool_id, None).expect_err("a gesture-only verb must not be silently mis-decoded");
        assert_eq!(error.code, semio_framework_plugin::FaultCode::new("dag.unhandled-action"));
    }
    assert!(DagPlayApp::command_from_action("thereIsNoSuchVerb", None).is_err());
}
//#endregion 🧵️RetainedToolCatalog
