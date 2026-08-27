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

use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::commands::set_locale;
use crate::editor::dag::commands::{add_node, patch_dag_nodes, remove_node, rename_dag_node};
use crate::editor::dag::commands::{connect_media_ports, delete_selection, disconnect, move_media_node, node_graph_edit, reorganize};
use crate::editor::dag::commands::{graph_pointer_down, node_graph_viewport};
use crate::editor::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::editor::dag::modes::edit;
use crate::editor::dag::modes::edit::windows::{compiled, main};
use crate::editor::dag::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::dag::terminology::{dag_play_labels, is_de_locale};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionFactory, ActionKind, AppActionRegistry, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest,
    ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DomainTopology, DraftView, Editor, EditorApp, Emit, Fault,
    GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode,
};
use serde_json::Value;
use store::EngineHandles;

//#region 🔖️Constants
pub const DAG_PLAY_APP_ID: &str = "dag-play";
/// 🕹️ The `graph` interaction domain id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// node/edge selection + transitive hover over the DAG's own edge-derived parent links.
pub const DAG_PLAY_INTERACTION_DOMAIN: &str = "graph";
pub use catalogue_panel::DAG_PLAY_BODY_CATALOGUE;
pub use compiled::{DAG_PLAY_BODY_COMPILED, DAG_PLAY_WINDOW_COMPILED};
pub use document_panel::DAG_PLAY_BODY_DOCUMENT;
pub use inspection_panel::DAG_PLAY_BODY_INSPECTOR;
pub use main::{DAG_PLAY_BODY_MAIN, DAG_PLAY_WINDOW_MAIN};

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn dag_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(DAG_PLAY_APP_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
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

//#region 🧵️RetainedConfigCommands
const DAG_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["nodeGraphViewport", "setLocale"];
const DAG_RETAINED_COMMAND_SCHEMA: &str = "dag.dag/v1.tool-command.v1";
const DAG_RETAINED_RAW_BYTES: usize = 8_192;

fn dag_retained_config_reduce(
    command: &DagCommand,
    snapshot: &DagSnapshot,
    config: &DagConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<DagMutation, DagConfigMutation, NoDraftMutation>, Fault> {
    if !DAG_RETAINED_CONFIG_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("dag-retained-config-route-mismatch"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

fn dag_retained_config_extent(command: &DagCommand, _snapshot: &DagSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    DAG_RETAINED_CONFIG_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

struct DagConfigCommandJobFactory { keys: Vec<ToolFactoryKey> }

impl DagConfigCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DAG_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DagConfigCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<DagPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<DagPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { DAG_RETAINED_COMMAND_SCHEMA }
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(DAG_RETAINED_RAW_BYTES, 64, 1, 8_192, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > DAG_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded DAG Config command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DagConfigCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DagPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DAG_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = "dag.dag";
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedConfigCommands

//#region 📬️ConfigStorePreparation
const DAG_CONFIG_STORE_MAXIMUM_BYTES: usize = 768;
const DAG_CONFIG_TEXT_BYTES: usize = 96;
const DAG_CONFIG_METADATA_BYTES: usize = 64;

struct DagConfigPreparationFactory;
struct DagConfigPreparation {
    base: Option<store::SnapshotRead<DagConfig>>,
    mutation: Option<DagConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(DagConfig, Vec<DagConfigMutation>, DagConfigMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<DagConfig, DagConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

fn dag_config_footprint(mutation: &DagConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = match mutation {
        DagConfigMutation::Snapshot { .. } => return Err("DAG Config preparation rejects whole-snapshot input".into()),
        DagConfigMutation::SetLocale { value } => value.len(),
        DagConfigMutation::SetCamera { .. } => 0,
    };
    if retained_bytes > DAG_CONFIG_TEXT_BYTES { return Err("DAG Config mutation exceeds its fixed preparation envelope".into()); }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024 })
}

fn prepare_dag_config(base: &DagConfig, mutation: DagConfigMutation) -> Result<(DagConfig, Vec<DagConfigMutation>, DagConfigMutation), String> {
    dag_config_footprint(&mutation)?;
    if base.locale.len() > DAG_CONFIG_TEXT_BYTES { return Err("DAG Config base exceeds its fixed preparation envelope".into()); }
    let mut post = base.clone();
    let inverse = match &mutation {
        DagConfigMutation::Snapshot { .. } => return Err("DAG Config preparation rejects whole-snapshot input".into()),
        DagConfigMutation::SetCamera { x, y, zoom } => {
            post.camera_x = *x; post.camera_y = *y; post.camera_zoom = *zoom;
            DagConfigMutation::SetCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom }
        }
        DagConfigMutation::SetLocale { value } => { post.locale = value.clone(); DagConfigMutation::SetLocale { value: base.locale.clone() } }
    };
    Ok((post, vec![inverse], mutation))
}

fn dag_config_edit(forward: DagConfigMutation, inverse: Vec<DagConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<DagConfigMutation> {
    let id = format!("dag-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<DagConfig, DagConfigMutation> for DagConfigPreparationFactory {
    fn preflight(&self, mutation: &DagConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > DAG_CONFIG_METADATA_BYTES) {
            return Err("DAG Config preparation rejected its lane or description envelope".into());
        }
        dag_config_footprint(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<DagConfig, DagConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<DagConfig, DagConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<DagConfig, DagConfigMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err() || request.operation != request.authority.operation() || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > DAG_CONFIG_METADATA_BYTES { return Err(request); }
        Ok(Box::new(DagConfigPreparation { base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None, checkpoint: Default::default(), retained_bytes: 0, cancelled: false, closing: false }))
    }
}

impl store::ArtifactStoreOneItemPreparation<DagConfig, DagConfigMutation> for DagConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "DAG Config preparation lost its exact base root".to_string())?.get();
            if base.locale.len() > DAG_CONFIG_TEXT_BYTES { return Err("DAG Config base exceeds its fixed preparation envelope".into()); }
            let bytes = DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024;
            if grant.maximum_bytes < bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.mutation.take().ok_or_else(|| "DAG Config preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_dag_config(base, mutation)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "DAG Config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "DAG Config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(dag_config_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<DagConfig, DagConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<DagConfig, DagConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.is_some() || self.candidate.is_some() {
            if grant.maximum_bytes < self.retained_bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            if self.prepared.take().is_none() { self.candidate = None; }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.mutation.is_some() {
            if grant.maximum_bytes < DAG_CONFIG_STORE_MAXIMUM_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: DAG_CONFIG_STORE_MAXIMUM_BYTES });
        }
        if let Some(description) = self.description.as_ref() {
            let bytes = description.len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("DAG Config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

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
    const DOCUMENT_SCHEMA: &'static str = "dag.dag";

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<DagPlayApp>,
        owner_file: "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "dag-play",
        document_schema: "dag.dag",
        factory: "DagConfigCommandJobFactory",
        factory_type: DagConfigCommandJobFactory,
        tools: {
            "nodeGraphViewport" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 8_192, 7_500),
            "setLocale" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 8_192, 7_500),
        }
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(DagConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(DagConfigCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !DAG_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if request.command.command_id() != request.tool_id { return Err(Fault::from("dag-retained-command-tool-mismatch")); }
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command, request.snapshot, request.config, request.history, request.interaction_state, request.interaction_hover, request.context,
            operation_context, request.completion, DagCommand::command_id, DAG_RETAINED_RAW_BYTES, 1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, dag_retained_config_reduce, dag_retained_config_extent)),
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

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
    async fn handle(
        command: &DagCommand,
        doc: &ArtifactView<'_, DagSnapshot>,
        cfg: &ConfigView<'_, DagConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<DagMutation, DagConfigMutation, Self::DraftMutation>, Fault> {
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
    async fn render(body_key: &str, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
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
            .action_with(ActionDefinition::bounded_catalog("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Mutation).with_category("create"))
            .mutation("removeNode", LocalizedLabel::native("Remove Node", "Knoten entfernen"))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .action_with(ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"), ActionKind::Mutation).with_category("selection"))
            .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
            .action_with(ActionDefinition::bounded_catalog("disconnect", LocalizedLabel::native("Disconnect", "Trennen"), ActionKind::Mutation).with_category("transfer"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::bounded_catalog("renameDagNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"), ActionKind::Mutation).with_category("actions"))
            .action_with(ActionDefinition::bounded_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
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
            .action_interactive_job("addNode", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("removeNode", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("connectMediaPorts", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("disconnect", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("moveMediaNode", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("renameDagNode", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchDagNodes", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("nodeGraphViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("graphPointerDown", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setLocale", semio_framework_plugin::InteractiveJobClassification::Migrated)
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

    //#region 🧪️RetainedConfigOracle
    #[test]
    fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
        let base = DagConfig::default();
        let mut expected = serde_json::to_value(&base).expect("JSON oracle base");
        expected["locale"] = serde_json::json!("de-DE");
        let (post, inverse, _) = prepare_dag_config(&base, DagConfigMutation::SetLocale { value: "de-DE".into() }).expect("bounded config candidate");
        assert_eq!(serde_json::to_value(post).expect("JSON oracle post"), expected);
        assert!(matches!(&inverse[0], DagConfigMutation::SetLocale { value } if value == &base.locale));
        assert!(dag_config_footprint(&DagConfigMutation::SetLocale { value: "x".repeat(DAG_CONFIG_TEXT_BYTES) }).is_ok());
        assert!(dag_config_footprint(&DagConfigMutation::SetLocale { value: "x".repeat(DAG_CONFIG_TEXT_BYTES + 1) }).is_err());
        assert!(dag_config_footprint(&DagConfigMutation::Snapshot { config: base }).is_err());
        assert_eq!(DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024, 4_096);
    }
    //#endregion 🧪️RetainedConfigOracle
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
    #[semio_framework_async_macros::async_test]
    async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
        let commands = every_command();
        assert_eq!(commands.len(), 13, "every DagCommand row must be covered by every_command()");
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
    #[semio_framework_async_macros::async_test]
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
    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::dag::testkit::{new_app, render};
        let mut app = new_app();
        assert!(render(&mut app, "dag.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn whole_document_operation_is_not_supported_as_an_in_history_mutation() {
        let replacement = crate::artifacts::dag::default_snapshot();
        assert!(DagPlayApp::whole_document_operation(replacement).is_none(), "whole-document replace goes through ArtifactStore::reset, never a mutation");
    }

    /// 🧬️ Two instances apply DISJOINT edits (A adds a note node, B adds a slider node) and converge to
    /// contain BOTH via a `MemoryBackbone` — impossible with whole-document snapshots.
    #[semio_framework_async_macros::async_test]
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

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent_for_dag() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<semio_framework_plugin::EditorApp<DagPlayApp>, usize>(DagCommand::AddNode(add_node::AddNode { kind: "note".into(), x: None, y: None }), |app| {
            app.snapshot().expect("projection").nodes().len()
        });
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
