//! 🔀️ DAG play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window renders
//! in `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `DagCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DagOperation, DagConfigOperation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

use crate::apps::dag::commands::graph::{connect_media_ports, delete_selection, disconnect, move_media_node, node_graph_edit, reorganize};
use crate::apps::dag::commands::locale::set_locale;
use crate::apps::dag::commands::nodes::{add_node, patch_dag_nodes, remove_node, rename_dag_node};
use crate::apps::dag::commands::selection::{graph_pointer_down, node_graph_hover, node_graph_select, node_graph_viewport, select_node, set_selection};
use crate::apps::dag::config::{dag_config_camera, DagConfig, DagConfigOperation};
use crate::apps::dag::modes::edit;
use crate::apps::dag::modes::edit::windows::{compiled, main};
use crate::apps::dag::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::dag::terminology::{dag_play_labels, is_de_locale};
use crate::artifacts::dag::op::DagOperation;
use crate::artifacts::dag::{DagDocument, DAG_DOCUMENT_SCHEMA};
use infinite_board_port_directed_dag::default_dag_document;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, App, AppActionRegistry, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, UiNode,
};
use serde_json::Value;

//#region 🔖️Constants
pub const DAG_PLAY_APP_ID: &str = "dag-play";
pub use main::{DAG_PLAY_BODY_MAIN, DAG_PLAY_WINDOW_MAIN};
pub use compiled::{DAG_PLAY_BODY_COMPILED, DAG_PLAY_WINDOW_COMPILED};
pub use document_panel::DAG_PLAY_BODY_DOCUMENT;
pub use catalogue_panel::DAG_PLAY_BODY_CATALOGUE;
pub use inspection_panel::DAG_PLAY_BODY_INSPECTOR;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn dag_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionFactory::new(DAG_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `DagPlayApp::Command` — the SOLE dispatch surface for dag's own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(keyword = ..)]` the codec uses) — genuinely different vocabularies for every row except
    /// where they happen to coincide (e.g. `"reorganize" as "reorganize"`). `"setLocale" as "locale"` is
    /// the row that proves it. **Row order is the binary variant ordinal: appending is safe, reordering
    /// is a wire-format break.**
    pub enum DagCommand for DagDocument, DagOperation, DagConfig, DagConfigOperation {
        "addNode" as "add-node" => add_node::AddNode,
        "removeNode" as "remove-node" => remove_node::RemoveNode,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "disconnect" as "disconnect" => disconnect::Disconnect,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "renameDagNode" as "rename-dag-node" => rename_dag_node::RenameDagNode,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "patchDagNodes" as "patch-dag-nodes" => patch_dag_nodes::PatchDagNodes,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectNode" as "select-node" => select_node::SelectNode,
        "nodeGraphSelect" as "node-graph-select" => node_graph_select::NodeGraphSelect,
        "nodeGraphHover" as "node-graph-hover" => node_graph_hover::NodeGraphHover,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "graphPointerDown" as "graph-pointer-down" => graph_pointer_down::GraphPointerDown,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
fn dag_context_menu_items(registry: &AppActionRegistry, labels: &crate::apps::dag::terminology::DagPlayLabels, is_de: bool, selected: &[String], request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

    let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), selected, &[]);
    let hit_edge_id = request.surface.as_ref().and_then(|target| target.hits.iter().find(|hit| hit.domain == "edge")).map(|hit| hit.id.clone());

    // 🗂️ Grouped disclosure: `addNode`/`reorganize` stay top-level (the most frequent verbs);
    // `renameDagNode` joins them only for a single-node selection; `disconnect` folds into the
    // "transfer" taxonomy group when an edge is hit — `organize_context_menu` (applied automatically at
    // the `VcsDocumentApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
    // inserts the pre-destructive separator itself, so no `.separator()` call is needed ahead of the
    // `deleteSelection`/`nodeGraphEdit` destructive row below.
    let mut menu = Menu::of(registry).action_args("addNode", serde_json::json!({ "kind": "computation" })).action("reorganize");
    if nodes.len() == 1 {
        menu = menu.action("renameDagNode");
    }
    if let Some(edge_id) = hit_edge_id {
        menu = menu.group("transfer", |m| m.action_args("disconnect", serde_json::json!({ "edgeId": edge_id })));
    }
    if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
        menu = menu.item(spec);
    }
    menu.build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️DagPlayApp
/// 🧪️ Unit struct — every former `DagPlayRuntime`/`ViewState.locale` field now lives in [`DagConfig`],
/// written through [`DagConfigOperation`]s.
#[derive(Default)]
pub struct DagPlayApp;

impl DocumentApp for DagPlayApp {
    type Projection = DagDocument;
    type Operation = DagOperation;
    type Config = DagConfig;
    type ConfigOperation = DagConfigOperation;
    type Command = DagCommand;

    fn app_id(&self) -> &str {
        DAG_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        DAG_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> DagDocument {
        default_dag_document()
    }

    fn whole_document_operation(&self, projection: DagDocument) -> Option<DagOperation> {
        Some(DagOperation::SetDocument { document: projection })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(&self, command: &DagCommand) -> &str {
        command.command_id()
    }

    fn handle(&self, command: &DagCommand, doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagOperation, DagConfigOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let selected = &config.selected_node_ids;
        let camera = dag_config_camera(config);
        let labels = dag_play_labels(config);
        match body_key {
            DAG_PLAY_BODY_MAIN => main::render(document, &camera, selected, labels),
            DAG_PLAY_BODY_COMPILED => compiled::render(document, &camera),
            DAG_PLAY_BODY_DOCUMENT => document_panel::render(document, selected, labels),
            DAG_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            DAG_PLAY_BODY_INSPECTOR => inspection_panel::render(document, selected, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn context_menu(&self, request: &ContextMenuRequest, _doc: &DocumentView<'_, DagDocument>, cfg: &ConfigView<'_, DagConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let labels = dag_play_labels(cfg.projection);
        let is_de = is_de_locale(cfg.projection);
        let selected = &cfg.projection.selected_node_ids;
        dag_context_menu_items(registry, labels, is_de, selected, request)
    }
}
//#endregion 🔖️DagPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_dag_app() -> App {
    App::from_builder(
        App::builder(DAG_PLAY_APP_ID, LocalizedLabel::native("DAG", "DAG"))
            .document(["semio", "mathematical", "graph", "port", "directed", "dag"])
            .artifact_kind(crate::artifacts::dag::artifact_kind())
            .icon_id("dag")
            .mode_def(edit::definition())
            .default_mode_id(edit::DAG_PLAY_MODE_EDIT)
            .window_kind_def(main::definition())
            .window_kind_def(compiled::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            // 🗂️ Referenced by `dag_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Operation).with_category("create"))
            .operation("removeNode", LocalizedLabel::native("Remove Node", "Knoten entfernen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"), ActionKind::Operation).with_category("selection"))
            .operation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::new_catalog("disconnect", LocalizedLabel::native("Disconnect", "Trennen"), ActionKind::Operation).with_category("transfer"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("renameDagNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"), ActionKind::Operation).with_category("actions"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .operation("patchDagNodes", LocalizedLabel::native("Patch Nodes", "Knoten patchen"))
            // 👁️ Ephemeral view state — selection and camera/viewport.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"))
            .view_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Knotengraph auswählen"))
            .view_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Knotengraph-Hover"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
            .keybinding("delete,backspace", "deleteSelection")
            // 📝️ Staged argument form for the panel-visible create action.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Typ"), vec![
                    ActionArgOption::new("computation", LocalizedLabel::native("Computation", "Berechnung")),
                    ActionArgOption::new("slider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("select", LocalizedLabel::native("Select", "Auswahl")),
                    ActionArgOption::new("screen", LocalizedLabel::native("Screen", "Bildschirm")),
                    ActionArgOption::new("note", LocalizedLabel::native("Note", "Notiz")),
                    ActionArgOption::new("preview", LocalizedLabel::native("Preview", "Vorschau")),
                ]).default_value("computation"),
            ])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // dag has no user-visible config defaults to expose, so `config_spec()` stays the trait
            // default `ConfigSpec::empty()`; declaring it explicitly here still keeps this app's typed
            // channel surface consistent with `shooting_ui::create_shooting_app`'s convention.
            .config(DagPlayApp.config_spec()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), serde_json::to_string(&default_dag_document()).expect("default DAG document has no non-string map keys or non-finite floats, so JSON serialization is infallible"), "cylinder")
    .workflow("dag", "DAG", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as framework_new_app, new_app_with_registry as framework_new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type DagApp = VcsDocumentApp<DagPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> DagApp {
        framework_new_app::<DagPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry() -> DagApp {
        framework_new_app_with_registry::<DagPlayApp>(create_dag_app)
    }

    pub fn dispatch(app: &mut DagApp, command: DagCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut DagApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::testkit::{new_app_with_registry, DagApp};
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
            DagCommand::SetSelection(set_selection::SetSelection { ids: vec!["n1".into()] }),
            DagCommand::SelectNode(select_node::SelectNode { node_id: "n1".into() }),
            DagCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec!["n1".into(), "n2".into()] }),
            DagCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {}),
            DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.5 }),
            DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            DagCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 17, "every DagCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
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
            ("set-selection", DagCommand::SetSelection(set_selection::SetSelection { ids: vec!["n1".into()] })),
            ("select-node", DagCommand::SelectNode(select_node::SelectNode { node_id: "n1".into() })),
            ("node-graph-select", DagCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec!["n1".into()] })),
            ("node-graph-hover", DagCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {})),
            ("node-graph-viewport", DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.0 })),
            ("graph-pointer-down", DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {})),
            ("locale", DagCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })),
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
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(DagCommand, &str, &str); 1] = [(DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: Some(10.0), y: None }), "add-node kind=slider x=10", "01000106736c696465720200060001050000000000002440")];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_dag_app().definition).expect("app definition json");
        assert!(json.contains(DAG_PLAY_WINDOW_MAIN), "main window kind missing from the manifest: {json}");
        assert!(json.contains(DAG_PLAY_WINDOW_COMPILED), "compiled window kind missing from the manifest: {json}");
        assert!(json.contains(edit::DAG_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [DAG_PLAY_BODY_DOCUMENT, DAG_PLAY_BODY_CATALOGUE, DAG_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("graph.dag"), "artifact kind missing from the manifest");
    }

    #[test]
    fn manifest_includes_the_demo_example() {
        let app = create_dag_app();
        assert!(app.examples.iter().any(|example| example.id == "demo"), "demo example missing from the manifest");
    }

    #[test]
    fn every_declared_action_is_registered_and_set_selection_is_a_view_action() {
        let definition = create_dag_app().definition;
        for command in [
            "addNode", "removeNode", "deleteSelection", "nodeGraphEdit", "connectMediaPorts", "disconnect", "moveMediaNode", "renameDagNode", "reorganize", "patchDagNodes", "setSelection", "selectNode", "nodeGraphSelect", "nodeGraphHover",
            "nodeGraphViewport", "graphPointerDown",
        ] {
            assert!(definition.actions.iter().any(|action| action.id == command), "registry declares {command}");
        }
        let mut app: DagApp = new_app_with_registry();
        let result = app.dispatch_typed(DagCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() }), &semio_framework_plugin::testkit::meta("local")).expect("setSelection");
        assert!(result.operations.is_empty(), "setSelection (View) emits no operations even under registry enforcement");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️ContextMenu
    /// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small even with a large
    /// selection, and the known `deleteSelection` destructive row (dispatched via `nodeGraphEdit` —
    /// `NodeGraphDeleteDispatch::ViaNodeGraphEdit`) is always last, either as a top-level leaf or as the
    /// tail of its group.
    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuHit, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

        let mut app: DagApp = new_app_with_registry();
        let node_ids: Vec<String> = app.projection().expect("projection").nodes.iter().map(|node| node.id.clone()).collect();
        app.dispatch_typed(DagCommand::SetSelection(set_selection::SetSelection { ids: node_ids.clone() }), &semio_framework_plugin::testkit::meta("local")).expect("setSelection");
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
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection (via nodeGraphEdit) must be last: {menu:?}");
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️CrossCutting
    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::dag::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "dag.play.nope").contains("Unknown body"));
    }

    #[test]
    fn whole_document_operation_replaces_the_projection() {
        let app = DagPlayApp;
        let replacement = default_dag_document();
        let operation = app.whole_document_operation(replacement.clone()).expect("whole document operation");
        assert_eq!(operation, DagOperation::SetDocument { document: replacement });
    }

    /// 🧬️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<DagPlayApp, (bool, bool)>(
            "mem://dag-convergence",
            DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }),
            DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }),
            |app| {
                let projection = app.projection().expect("projection");
                (projection.nodes.iter().any(|node| matches!(node.kind, infinite_board_port_directed_dag::DagNodeKind::Note { .. })), projection.nodes.iter().any(|node| matches!(node.kind, infinite_board_port_directed_dag::DagNodeKind::Slider { .. })))
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_dag() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<DagPlayApp, usize>(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), |app| app.projection().expect("projection").nodes.len());
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
