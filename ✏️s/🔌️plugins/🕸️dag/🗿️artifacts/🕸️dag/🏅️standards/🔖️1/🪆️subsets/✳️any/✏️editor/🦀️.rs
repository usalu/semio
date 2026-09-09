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

use crate::editor::dag::commands::{add_node, patch_dag_nodes, remove_node, rename_dag_node};
use crate::editor::dag::commands::{connect_media_ports, delete_selection, disconnect, move_media_node, node_graph_edit, reorganize};
use crate::editor::dag::commands::{graph_pointer_down, node_graph_viewport};
use crate::editor::dag::config::{dag_config_camera, DagConfig, DagConfigMutation};
use crate::editor::dag::modes::edit;
use crate::editor::dag::modes::edit::windows::{compiled, main};
use crate::editor::dag::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::dag::terminology::{dag_play_labels, is_de_locale};
use crate::op::DagMutation;
use crate::DagSnapshot;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionFactory, ActionKind, AppActionRegistry, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DomainTopology, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition,
    InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode,
};
use store::EngineHandles;

//#region 🔖️Constants
pub const DAG_PLAY_APP_ID: &str = "s.dag.dag@1/*#editor";
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
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
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
    }
}
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
fn dag_context_menu_items(registry: &AppActionRegistry, labels: &crate::editor::dag::terminology::DagPlayLabels, is_de: bool, selected: &[String], request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

    let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), selected, &[]);
    let hit_edge_id = request.surface.as_ref().and_then(|target| target.hits.iter().find(|hit| hit.domain == "edge")).map(|hit| hit.id.clone());

    // 🗂️ Grouped disclosure: `addNode`/`reorganize` stay top-level (the most frequent verbs);
    // `renameDagNode` joins them only for a single-node selection; `disconnect` folds into the
    // "transfer" taxonomy group when an edge is hit — `organize_context_menu` (applied automatically at
    // the `VcsArtifactApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
    // inserts the pre-destructive separator itself, so no `.separator()` call is needed ahead of the
    // `deleteSelection`/`nodeGraphEdit` destructive row below.
    let mut menu = Menu::of(registry).action_args("addNode", dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String("computation".to_string()))])).action("reorganize");
    if nodes.len() == 1 {
        menu = menu.action("renameDagNode");
    }
    if let Some(edge_id) = hit_edge_id {
        menu = menu.group("transfer", |m| m.action_args("disconnect", dsl::DslValue::object([("edgeId".to_string(), dsl::DslValue::String(edge_id))])));
    }
    if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
        menu = menu.item(spec);
    }
    menu.build()
}
//#endregion 🔖️ContextMenu

//#region 🔖️DagPlayApp
/// 🧪️ Unit struct whose editor-owned camera state lives in [`DagConfig`] and whose host-owned
/// preferences arrive through the canonical [`ViewModel`].
#[derive(Default)]
pub struct DagPlayApp;

//#region 🧵️RetainedConfigCommands
const DAG_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["nodeGraphViewport"];
const DAG_RETAINED_COMMAND_SCHEMA: &str = "dag.dag/v1.tool-command.v1";
const DAG_RETAINED_RAW_BYTES: usize = 8_192;

fn dag_retained_config_reduce(
    command: &DagCommand,
    snapshot: &DagSnapshot,
    config: &DagConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<DagPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<DagMutation, DagConfigMutation, NoDraftMutation>, Fault> {
    if !DAG_RETAINED_CONFIG_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("dag-retained-config-route-mismatch"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

fn dag_retained_config_extent(command: &DagCommand, _snapshot: &DagSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    DAG_RETAINED_CONFIG_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

struct DagConfigCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl DagConfigCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DAG_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DagConfigCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<DagPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<DagPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        DAG_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(DAG_RETAINED_RAW_BYTES, 64, 1, 8_192, 7_500)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

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
    type Owner = EditorApp<DagPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DAG_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = "dag.dag";
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[ArtifactToolPublicationLane::Config] }];
}
//#endregion 🧵️RetainedConfigCommands

//#region 📬️ConfigStorePreparation
const DAG_CONFIG_STORE_MAXIMUM_BYTES: usize = 768;
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
    match mutation {
        DagConfigMutation::ReplaceConfig(crate::editor::dag::config::ReplaceConfig { .. }) => return Err("DAG Config preparation rejects whole-snapshot input".into()),
        DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { .. }) => {}
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024 })
}

fn prepare_dag_config(base: &DagConfig, mutation: DagConfigMutation) -> Result<(DagConfig, Vec<DagConfigMutation>, DagConfigMutation), String> {
    dag_config_footprint(&mutation)?;
    let mut post = base.clone();
    let inverse = match &mutation {
        DagConfigMutation::ReplaceConfig(crate::editor::dag::config::ReplaceConfig { .. }) => return Err("DAG Config preparation rejects whole-snapshot input".into()),
        DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x, y, zoom }) => {
            post.camera_x = *x;
            post.camera_y = *y;
            post.camera_zoom = *zoom;
            DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })
        }
    };
    Ok((post, vec![inverse], mutation))
}

fn dag_config_edit(forward: DagConfigMutation, inverse: Vec<DagConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<DagConfigMutation> {
    let id = format!("dag-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<DagConfig, DagConfigMutation> for DagConfigPreparationFactory {
    fn preflight(&self, mutation: &DagConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > DAG_CONFIG_METADATA_BYTES) {
            return Err("DAG Config preparation rejected its lane or description envelope".into());
        }
        dag_config_footprint(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<DagConfig, DagConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<DagConfig, DagConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<DagConfig, DagConfigMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > DAG_CONFIG_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(DagConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: Default::default(),
            retained_bytes: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<DagConfig, DagConfigMutation> for DagConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "DAG Config preparation lost its exact base root".to_string())?.get();
            let bytes = DAG_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024;
            if grant.maximum_bytes < bytes {
                return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            let mutation = self.mutation.take().ok_or_else(|| "DAG Config preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_dag_config(base, mutation)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "DAG Config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "DAG Config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(dag_config_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<DagConfig, DagConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<DagConfig, DagConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.is_some() || self.candidate.is_some() {
            if grant.maximum_bytes < self.retained_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            if self.prepared.take().is_none() {
                self.candidate = None;
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.mutation.is_some() {
            if grant.maximum_bytes < DAG_CONFIG_STORE_MAXIMUM_BYTES {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: DAG_CONFIG_STORE_MAXIMUM_BYTES });
        }
        if let Some(description) = self.description.as_ref() {
            let bytes = description.len();
            if grant.maximum_bytes < bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("DAG Config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
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

    const DIALECT: Dialect = crate::DAG_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "dag.dag";

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<DagPlayApp>,
        owner_file: "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.dag.dag@1/*#editor",
        document_schema: "dag.dag",
        factory: "DagConfigCommandJobFactory",
        factory_type: DagConfigCommandJobFactory,
        tools: {
            "nodeGraphViewport" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 8_192, 7_500),
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
        if !DAG_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("dag-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            DagCommand::command_id,
            DAG_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, dag_retained_config_reduce, dag_retained_config_extent)),
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::dag::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> DagSnapshot {
        crate::default_snapshot()
    }

    // 🎞️ No `whole_document_operation` override: whole-document replace is not an in-history
    // mutation any more (the old whole-snapshot-replacement variant is gone with no replacement —
    // see the mutations facet report). The trait default (`None`) applies, so the generic
    // `document:in` media importer correctly reports `MediaError::NotImplemented`; a real
    // whole-document load goes through `store::ArtifactStore::reset` instead.

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &DagCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit` read the `graph` interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)` signature
    /// is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &DagCommand,
        doc: &ArtifactView<'_, DagSnapshot>,
        cfg: &ConfigView<'_, DagConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
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
    fn render(body_key: &str, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let camera = dag_config_camera(config);
        let labels = dag_play_labels(view_state);
        let node = match body_key {
            DAG_PLAY_BODY_MAIN => main::render(document, &camera, labels),
            DAG_PLAY_BODY_COMPILED => compiled::render(document, &camera),
            DAG_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            DAG_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            DAG_PLAY_BODY_INSPECTOR => inspection_panel::render(document, &[], labels),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🕹️ `context_menu` carries no `InteractionView` either (same gap as `render`), so the
    /// selection-dependent rows below always take the "nothing selected" branch — `request.surface`'s
    /// own click-carried selection (independent of `graph`'s live state) still drives the menu.
    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let labels = dag_play_labels(view_state);
        let is_de = is_de_locale(view_state);
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
    fn interaction_topology(doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> InteractionTopology {
        let document = doc.snapshot;
        let nodes = document.nodes();
        let edges = document.edges();
        // 🧵️ `DagFixtureEdge.source`/`.target` are "nodeId@portId" endpoint strings (defaulting to the
        // "out" port when bare) — `split_endpoint` peels the node id back off before it can be matched
        // against a plain `DagNodeSpec.id`.
        let node_id_of = |endpoint: &str| crate::schema::split_endpoint(endpoint).0;
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
pub fn create_dag_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::DAG_DIALECT)
            .document(["semio", "mathematical", "graph", "port", "directed", "dag"])
            .artifact_kind(crate::artifact_kind())
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
            .action_with(ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
            .mutation("patchDagNodes", LocalizedLabel::native("Patch Nodes", "Knoten patchen"))
            // 👁️ Ephemeral view state — camera/viewport. Selection/hover no longer declared here: the
            // framework auto-injects interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity for every domain declared via `.interaction(...)`
            // below (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM — never declare those
            // actions yourself).
            .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
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
                ]).default_value(&"computation"),
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
            // `crate::examples::demo` example registration and the no-op `.workflow("dag", …)`
            // call are dropped here (reported in the migration report, not silently lost). The
            // subset's own `📚️examples/🎬️demo` facet (`crate::examples::demo`,
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
            .config(DagPlayApp::config_spec())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
