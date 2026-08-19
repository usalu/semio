//! 🔀️ DAG play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window renders
//! in `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table: `handle` →
//! `DagCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DagMutation, DagConfigMutation>, Fault>`, the exact signature `ArtifactEditor::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

use crate::editor::dag::commands::{connect_media_ports, delete_selection, disconnect, move_media_node, node_graph_edit, reorganize};
use crate::editor::dag::commands::set_locale;
use crate::editor::dag::commands::{add_node, patch_dag_nodes, remove_node, rename_dag_node};
use crate::editor::dag::commands::{graph_pointer_down, node_graph_viewport};
use crate::editor::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::editor::dag::modes::edit;
use crate::editor::dag::modes::edit::windows::{compiled, main};
use crate::editor::dag::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::dag::terminology::{dag_play_labels, is_de_locale};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::{DagSnapshot, DAG_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, AppActionRegistry, ConfigView, ContextMenuItemSpec, ContextMenuRequest, ArtifactEditor, ArtifactView, DomainTopology, Editor, Emit, Fault, GranularityDefinition,
    HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode,
};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const DAG_PLAY_APP_ID: &str = "dag-play";
/// 🕹️ The `graph` interaction domain id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// node/edge selection + transitive hover over the DAG's own edge-derived parent links.
pub const DAG_PLAY_INTERACTION_DOMAIN: &str = "graph";
pub use main::{DAG_PLAY_BODY_MAIN, DAG_PLAY_WINDOW_MAIN};
pub use compiled::{DAG_PLAY_BODY_COMPILED, DAG_PLAY_WINDOW_COMPILED};
pub use document_panel::DAG_PLAY_BODY_DOCUMENT;
pub use catalogue_panel::DAG_PLAY_BODY_CATALOGUE;
pub use inspection_panel::DAG_PLAY_BODY_INSPECTOR;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub async fn dag_action(action: &str, args: Option<Value>) -> ActionDescriptor {
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
    pub enum DagCommand for DagSnapshot, DagMutation, DagConfig, DagConfigMutation {
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
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "graphPointerDown" as "graph-pointer-down" => graph_pointer_down::GraphPointerDown,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
async fn dag_context_menu_items(registry: &AppActionRegistry, labels: &crate::editor::dag::terminology::DagPlayLabels, is_de: bool, selected: &[String], request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

    let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), selected, &[]);
    let hit_edge_id = request.surface.as_ref().and_then(|target| target.hits.iter().find(|hit| hit.domain == "edge")).map(|hit| hit.id.clone());

    // 🗂️ Grouped disclosure: `addNode`/`reorganize` stay top-level (the most frequent verbs);
    // `renameDagNode` joins them only for a single-node selection; `disconnect` folds into the
    // "transfer" taxonomy group when an edge is hit — `organize_context_menu` (applied automatically at
    // the `VcsArtifactApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
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
/// 🧪️ Unit struct — every former `DagPlayRuntime`/`ViewModel.locale` field now lives in [`DagConfig`],
/// written through [`DagConfigMutation`]s.
#[derive(Default)]
pub struct DagPlayApp;

impl ArtifactEditor for DagPlayApp {
    type Snapshot = DagSnapshot;
    type Mutation = DagMutation;
    type Config = DagConfig;
    type ConfigMutation = DagConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::dag::presence::DagPresence;
    type PresenceMutation = crate::editor::dag::presence::DagPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = DagCommand;

    const DIALECT: Dialect = crate::artifacts::dag::DAG_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DAG_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::dag::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> DagSnapshot {
        crate::artifacts::dag::default_snapshot()
    }

    // 🎞️ No `whole_document_operation` override: whole-document replace is not an in-history
    // mutation any more (the old whole-snapshot-replacement variant is gone with no replacement —
    // see the mutations facet report). The trait default (`None`) applies, so the generic
    // `document:in` media importer correctly reports `MediaError::NotImplemented`; a real
    // whole-document load goes through `store::ArtifactStore::reset` instead.

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &DagCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit` read the `graph` interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)` signature
    /// is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    async fn handle(command: &DagCommand, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<DagMutation, DagConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            DagCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            DagCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `render` carries no `InteractionView` (`ArtifactEditor`'s breaking pass only added it to
    /// `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the main
    /// node-graph canvas and the inspector both degrade to "nothing selected" until a future wave
    /// threads interaction into render; the document tree instead binds `interaction_domain("graph")`
    /// so the framework's own post-render stamp paints its selection/hover, no app code needed.
    /// Flagged as a discovered framework gap, not worked around here (matches `space`'s identical gap).
    async fn render(body_key: &str, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let camera = dag_config_camera(config);
        let labels = dag_play_labels(config);
        match body_key {
            DAG_PLAY_BODY_MAIN => main::render(document, &camera, labels),
            DAG_PLAY_BODY_COMPILED => compiled::render(document, &camera),
            DAG_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            DAG_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            DAG_PLAY_BODY_INSPECTOR => inspection_panel::render(document, &[], labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render`), so the
    /// selection-dependent rows below always take the "nothing selected" branch — `request.surface`'s
    /// own click-carried selection (independent of `graph`'s live state) still drives the menu.
    async fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let labels = dag_play_labels(cfg.snapshot);
        let is_de = is_de_locale(cfg.snapshot);
        dag_context_menu_items(registry, labels, is_de, &[], request)
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every node's parent is the source of its first
    /// incoming edge (`None` for a root with no incoming edge), and every edge is registered as a
    /// sibling child of that same source node — so hovering/selecting a node transitively covers its
    /// downstream nodes AND edges (the DAG's actual data-flow direction), while `validate_state` prunes
    /// a deleted node/edge id out of `graph`'s selection the moment it disappears from the document. A
    /// join (a node with multiple incoming edges) picks its FIRST incoming edge's source as the single
    /// parent — `TopologyNode` has one parent slot, so a true multi-parent DAG only gets one branch of
    /// its transitive closure; a documented approximation, matching `PathDelimited`'s own precedent.
    async fn interaction_topology(doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> InteractionTopology {
        let document = doc.snapshot;
        let nodes = document.nodes();
        let edges = document.edges();
        // 🧵️ `DagFixtureEdge.source`/`.target` are "nodeId@portId" endpoint strings (defaulting to the
        // "out" port when bare) — `split_endpoint` peels the node id back off before it can be matched
        // against a plain `DagNodeSpec.id`.
        let node_id_of = |endpoint: &str| crate::artifacts::dag::schema::split_endpoint(endpoint).0;
        let mut ordered = Vec::with_capacity(nodes.len() + edges.len());
        for node in &nodes {
            let parent = edges.iter().find(|edge| node_id_of(&edge.target) == node.id).map(|edge| node_id_of(&edge.source));
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent });
        }
        for edge in &edges {
            ordered.push(TopologyNode { id: edge.id.clone(), granularity: "edge".into(), parent: Some(node_id_of(&edge.source)) });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(DAG_PLAY_INTERACTION_DOMAIN.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }
}
//#endregion 🔖️DagPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_dag_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::dag::DAG_DIALECT)
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
            .action_with(ActionDefinition::new_catalog("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("removeNode", LocalizedLabel::native("Remove Node", "Knoten entfernen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"), ActionKind::Mutation).with_category("selection"))
            .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::new_catalog("disconnect", LocalizedLabel::native("Disconnect", "Trennen"), ActionKind::Mutation).with_category("transfer"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("renameDagNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"), ActionKind::Mutation).with_category("actions"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .mutation("patchDagNodes", LocalizedLabel::native("Patch Nodes", "Knoten patchen"))
            // 👁️ Ephemeral view state — camera/viewport. Selection/hover no longer declared here: the
            // framework auto-injects interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity for every domain declared via `.interaction(...)`
            // below (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — never declare those
            // actions yourself).
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
            // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
            // one `graph` domain over the node graph, node/edge granularities, `HierarchyProvider::Topology`
            // from the DAG's own edges (see `DagPlayApp::interaction_topology`), transitive HOVER only (a
            // hovered node lights up everything downstream, a nice "what does this feed?" highlight) —
            // selection stays NON-transitive: a downstream node is a dependent, not a structural child (no
            // AST-style containment), so clicking one node must not silently drag every node it feeds into
            // the selection (and so `deleteSelection` never cascade-deletes downstream nodes the user never
            // clicked). `nodeGraphSelect`'s old marquee behavior is now the framework's own
            // `SelectionMethod::Rectangle` method, no app geometry needed.
            .interaction(InteractionDefinition {
                id: DAG_PLAY_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Graph", "Graph"),
                granularities: vec![
                    GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "box".into() },
                    GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "git-commit-horizontal".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .window_kind_interactions(DAG_PLAY_WINDOW_MAIN, vec![InteractionRef::new(DAG_PLAY_INTERACTION_DOMAIN)])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // dag has no user-visible config defaults to expose, so `config_spec()` stays the trait
            // default `ConfigSpec::empty()`; declaring it explicitly here still keeps this app's typed
            // channel surface consistent with `shooting_ui::create_shooting_app`'s convention.
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example_source(...)`/`.workflow(...)` on this builder, so the app-level
            // `crate::examples::art_dag_demo` example registration and the no-op `.workflow("dag", …)`
            // call are dropped here (reported in the migration report, not silently lost). The
            // subset's own `📚️examples/🎬️demo` facet (`crate::artifacts::dag::examples::demo`,
            // real content, pre-existing) is the modern, role-agnostic replacement surface for this.
            .config(DagPlayApp::config_spec())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as framework_new_app, new_app_with_registry as framework_new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type DagApp = VcsArtifactApp<EditorApp<DagPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> DagApp {
        framework_new_app::<EditorApp<DagPlayApp>>()
    }

    /// ✏️ Adapts `create_dag_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`'s framework testkit signature (contract §2.5 gap 3,
    /// not yet updated for the `AppDefinition`-returning convention) still expects.
    pub async fn dag_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_dag_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> DagApp {
        framework_new_app_with_registry::<EditorApp<DagPlayApp>>(dag_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut DagApp, command: DagCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut DagApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::dag::testkit::{new_app_with_registry, DagApp};
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<DagCommand> {
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
            DagCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.5 }),
            DagCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            DagCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 13, "every DagCommand row must be covered by every_command()");
        let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
        keywords.sort();
        keywords.dedup();
        assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[test]
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
    #[test]
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

    #[test]
    async fn every_declared_action_is_registered() {
        let definition = create_dag_app();
        for command in ["addNode", "removeNode", "deleteSelection", "nodeGraphEdit", "connectMediaPorts", "disconnect", "moveMediaNode", "renameDagNode", "reorganize", "patchDagNodes", "nodeGraphViewport", "graphPointerDown"] {
            assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
        }
    }

    /// 🕹️ `graph` is declared once, node/edge granularities, `Topology` hierarchy, scoped to the main
    /// window — the framework auto-injects the six interaction actions for it (never app-declared).
    #[test]
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
    #[test]
    async fn interaction_topology_covers_every_node_and_edge_via_their_edges() {
        let mut app: DagApp = new_app_with_registry();
        let snapshot = app.snapshot().expect("snapshot");
        let node_id = snapshot.nodes().first().expect("seed node").id.clone();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = DagConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
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
    #[test]
    async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuHit, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

        let mut app: DagApp = new_app_with_registry();
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
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::dag::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "dag.play.nope").contains("Unknown body"));
    }

    #[test]
    async fn whole_document_operation_is_not_supported_as_an_in_history_mutation() {
        let replacement = crate::artifacts::dag::default_snapshot();
        assert!(DagPlayApp::whole_document_operation(replacement).is_none(), "whole-document replace goes through ArtifactStore::reset, never a mutation");
    }

    /// 🧬️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<DagPlayApp>, (bool, bool)>(
            "mem://dag-convergence",
            DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }),
            DagCommand::AddNode(add_node::AddNode { kind: "slider".into(), x: None, y: None }),
            |app| {
                let projection = app.snapshot().expect("projection");
                let nodes = projection.nodes();
                (nodes.iter().any(|node| matches!(node.kind, infinite_board_port_directed_dag::DagNodeKind::Note { .. })), nodes.iter().any(|node| matches!(node.kind, infinite_board_port_directed_dag::DagNodeKind::Slider { .. })))
            },
        );
    }

    #[test]
    async fn ingest_operations_is_idempotent_for_dag() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<semio_framework_plugin::EditorApp<DagPlayApp>, usize>(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), |app| app.snapshot().expect("projection").nodes().len());
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
