//! 🎛️ S Studio app — `ArtifactApp` impl, command dispatch, manifest (constitutional: ui + general,
//! merged at app level).
//!
//! 🕳️ Deviation from the usual "general"/"ui" split: the Studio app has no document type of its own —
//! its `ArtifactApp::Snapshot`/`Mutation` are `semio_framework_os::{WorkflowSnapshot, WorkflowMutation}`,
//! owned entirely by `framework/product/os/core/rs` (outside this plugin). There is therefore no
//! `🗿️artifacts/🪐️space` node anywhere in this crate — this file carries what would otherwise be the
//! artifact facade's constants (manifest/panel identifiers shared by `engine` config defaults and this
//! file's manifest/render dispatch), plus every DocumentHelpers used by 2+ command modules (a helper
//! that takes `&SpaceConfig` stays app-level no matter how many consumers it has, per the per-app
//! recipe's DocumentHelpers placement rule — artifacts must never depend on apps, and there is no
//! artifact here regardless).
//!
//! WIRING + DISPATCH ONLY beyond that: every command's real body lives in its own
//! `🎮️commands/<group>/🦀️.rs` payload module (see `app_commands!` below).

use crate::engine::space::commands::node_graph_edit;
use crate::engine::space::commands::node_graph_viewport;
use crate::engine::space::commands::presence_heartbeat;
use crate::engine::space::commands::{add_parameter, bind_parameter_field, patch_parameter, remove_parameter, unbind_parameter_field};
use crate::engine::space::commands::{close_focused_instance, open_instance};
use crate::engine::space::commands::{compiled_dag_engagement_input, compiled_dag_engagement_submit, workflow_engagement_input, workflow_engagement_submit};
use crate::engine::space::commands::{connect_media_ports, disconnect_media_edge};
use crate::engine::space::commands::{
    copy_app_instance, delete_selection, duplicate_app_instance, move_media_node, paste_app_instance, patch_app_instances, patch_media_nodes, remove_app_instance, rename_app_instance, reorganize_workflow, spawn_app,
};
use crate::engine::space::commands::{export_media, import_media, import_media_payload};
use crate::engine::space::commands::{export_studio_dsl, export_studio_pack, import_space_pack, import_space_pack_payload, open_space, set_active_example};
use crate::engine::space::commands::{go_home, navigate_virtual_file_system_node, set_active_panel_tab, set_app_registrations};
use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use crate::engine::space::presence::{SpacePresence, SpacePresenceMutation};
use crate::engine::space::terminology::SStudioLabels;
use crate::parse_demo_space_document;
use semio_framework::{DslValue, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_os::workflow::ConnectPorts;
use semio_framework_os::{create_os_id, empty_workflow_snapshot, MediaContract, WorkflowEdge, WorkflowMutation, WorkflowSnapshot, S_WORKFLOW_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    app::InteractionView, app_commands, create_default_layout, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppOperationContext, ArtifactApp, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry,
    ArtifactView, CommandDefinition, ConfigView, DomainTopology, DraftView, Effect, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget,
    InteractionTopology, InteractiveJobClassification, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, WindowLayout, CLEAR_SELECTION_ACTION_ID,
    INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
/// 🪪️ Canonical surface id (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1: every
/// `App::builder(...)` id must parse via `semio_framework::parse_surface_app_id` as
/// `<artifact_kind>@<standard>/<subset>#<role>`) — was the bare `"studio"`, which stopped parsing the
/// moment that migration's validation landed in `PluginBuilder::document_app`'s `build_definition`.
pub const S_PLAY_APP_ID: &str = "s.space.studio@1/*#editor";
pub const S_PLAY_CONTROLLER_ID: &str = "s.space.studio@1/*#editor";
pub const S_PLAY_CATALOGUE_TAB_ID: &str = "s-play-catalogue";
pub const S_PLAY_PARAMETERS_TAB_ID: &str = "s-play-parameters";
pub const S_PLAY_INSPECTOR_TAB_ID: &str = "s-play-inspector";
pub const S_PLAY_CATALOGUE_BODY_KEY: &str = "s.play.catalogue";
pub const S_PLAY_PARAMETERS_BODY_KEY: &str = "s.play.parameters";
pub const S_PLAY_INSPECTOR_BODY_KEY: &str = "s.play.inspector";
pub const S_PLAY_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
pub const S_STUDIO_EXAMPLES: &[(&str, &str)] = &[("demo", "Demo Studio")];
/// 🕹️ The `graph` interaction domain id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// one domain over the workflow node graph, shared by both the "instance" and "media-node" granularities.
pub const S_PLAY_INTERACTION_DOMAIN: &str = "graph";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
pub(crate) fn s_play_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(S_PLAY_CONTROLLER_ID).action(action, args)
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

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `graph` domain — replaces the deleted
/// `selectInstance`/`nodeGraphSelect`/`setMediaNodeSelection`/`setAppInstanceSelection` action builders
/// every measure/document row used to construct by hand.
pub(crate) async fn space_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = pack::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    ActionDescriptor {
        controller_id: S_PLAY_CONTROLLER_ID.into(),
        action: INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(DslValue::object(vec![
            ("domainId".to_string(), DslValue::String(S_PLAY_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), DslValue::String(targets)),
            ("merge".to_string(), DslValue::String("replace".into())),
            ("method".to_string(), DslValue::String("pick".into())),
        ])),
    }
}

/// @emoji 🤝️ Resolves the source/target ports for a proposed connect and negotiates their wire contract
/// via `engine::negotiate_media_connect`, converting a rejection into a `Notify` effect — shared by
/// `connections::connect_media_ports` and the `graph_edit::node_graph_edit`/`"connect"` fixture edit.
pub(crate) async fn negotiate_connect_or_notify(projection: &WorkflowSnapshot, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, Effect> {
    crate::engine::space::engine::negotiate_media_connect(projection, source_node_id, source_port_id, target_node_id, target_port_id).await.map_err(|reason| Effect::Notify { message: reason })
}

pub(crate) async fn connect_edge_operation(source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str, contract: MediaContract) -> WorkflowMutation {
    WorkflowMutation::ConnectPorts(ConnectPorts {
        edge: WorkflowEdge { id: create_os_id("edge"), source_node_id: source_node_id.into(), source_port_id: source_port_id.into(), target_node_id: target_node_id.into(), target_port_id: target_port_id.into(), contract },
    })
}

/// @emoji 🔎️ First selected node — the fallback target for actions that implicitly operate on "the"
/// current selection (rename/remove/open) when no explicit node id is supplied. `selected` is the
/// `graph` domain's live selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no
/// longer a deleted `SpaceConfig` field.
pub(crate) async fn primary_selected_node_id(selected: &[String], config: &SpaceConfig) -> Option<String> {
    selected.first().cloned().or_else(|| config.active_node_id.clone())
}

/// 🔧️ Small pure fold applying a batch of `SpaceConfigMutation`s onto a snapshot — used where a
/// command handler needs the POST-command config (not the pre-command `cfg.snapshot`) to build a
/// derived side value (the presence broadcast) in the very same call, without reaching back into a
/// store this pure function doesn't own.
#[cfg(test)]
pub(crate) async fn apply_config_mutations(config: &SpaceConfig, operations: &[SpaceConfigMutation]) -> SpaceConfig {
    use protocol::Mutation;
    operations.iter().fold(config.clone(), |acc, operation| operation.diff(&acc).diff().clone())
}

pub(crate) async fn config_space_id(config: &SpaceConfig) -> String {
    config.space_id.clone().unwrap_or_else(|| "default".into())
}

/// 🫀️ Remote presence is host-owned. Until `ArtifactHost` threads the typed `PresenceView` into render,
/// Space shows no remote peers instead of retaining cross-instance state in a process-global registry.
pub(crate) async fn presence_peers_json(_app: &SpaceApp, _config: &SpaceConfig) -> String {
    "[]".into()
}

/// 🖱️ On-demand space workflow context menu from hit-test and selection snapshot.
async fn space_workflow_context_menu_items(
    registry: &semio_framework_plugin::AppActionRegistry,
    labels: &SStudioLabels,
    is_de: bool,
    surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>,
    selected_node_ids: &[String],
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, selection_domains_from_surface, ContextMenuItemSpec, Menu};

    let hits: &[semio_framework_plugin::ContextMenuHit] = surface.map_or(&[], |target| target.hits.as_slice());
    let (nodes, _) = selection_domains_from_surface(surface, selected_node_ids, &[]);
    let hit_node = hits.iter().find(|hit| hit.domain == "node").map(|hit| hit.id.as_str());
    let mut menu = Menu::of(registry);
    if hits.is_empty() {
        // 🗂️ Empty-canvas menu: paste/select-all stay top-level (the two most frequent verbs here),
        // reorganize is a rarer layout action so it moves into its own taxonomy group.
        menu = menu
            .item(ContextMenuItemSpec { id: "paste-instance".into(), label: Some(labels.context_paste.into()), icon: Some("clipboard".into()), action: Some("pasteAppInstance".into()), ..Default::default() })
            
            .item(ContextMenuItemSpec { id: "select-all".into(), label: Some(labels.context_select_all.into()), icon: Some("maximize-2".into()), action: Some(SELECT_ALL_ACTION_ID.into()), ..Default::default() })
            
            .group("transform", |m| m.item(ContextMenuItemSpec { id: "reorganize".into(), label: Some(labels.context_reorganize.into()), icon: Some("layout-grid".into()), action: Some("reorganizeWorkflow".into()), ..Default::default() }))
            ;
    }
    if hit_node.is_some() || !nodes.is_empty() {
        // 🗂️ Node menu: open/duplicate stay top-level (the two most frequent verbs); copy moves into
        // "transfer" (clipboard), rename into "settings" (identity/label editing), remove stays a
        // trailing destructive leaf — `organize_context_menu` (run automatically at the
        // `VcsArtifactApp::context_menu` funnel) inserts the pre-destructive separator itself.
        menu = menu
            .item(ContextMenuItemSpec { id: "open-instance".into(), label: Some(labels.context_open_instance.into()), icon: Some("external-link".into()), action: Some("openInstance".into()), ..Default::default() })
            
            .item(ContextMenuItemSpec { id: "duplicate-instance".into(), label: Some(labels.context_duplicate.into()), icon: Some("copy".into()), action: Some("duplicateAppInstance".into()), ..Default::default() })
            
            .group("transfer", |m| m.item(ContextMenuItemSpec { id: "copy-instance".into(), label: Some(labels.context_copy.into()), icon: Some("clipboard-copy".into()), action: Some("copyAppInstance".into()), ..Default::default() }))
            
            .group("settings", |m| m.item(ContextMenuItemSpec { id: "rename-instance".into(), label: Some(labels.context_rename_label.into()), icon: Some("edit-3".into()), action: Some("renameAppInstance".into()), ..Default::default() }))
            ;
        if !nodes.is_empty() {
            menu = menu
                .group("selection", |m| {
                    m.item(ContextMenuItemSpec { id: "clear-selection".into(), label: Some(labels.context_clear_selection.into()), icon: Some("square-dashed".into()), action: Some(CLEAR_SELECTION_ACTION_ID.into()), ..Default::default() })
                })
                ;
        }
        let phrase = selection_count_phrase(is_de, &[(nodes.len().max(if hit_node.is_some() && nodes.is_empty() { 1 } else { 0 }), if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" })]);
        let remove_label = if phrase.is_empty() { labels.context_remove.as_str().to_string() } else { format!("{} ({phrase})", labels.context_remove.as_str()) };
        // 🎯️ Destructive tail always comes last — kept unconditionally after the "selection" group so
        // remove-instance is the final row regardless of whether clear-selection was appended above.
        menu = menu.item(ContextMenuItemSpec { id: "remove-instance".into(), label: Some(remove_label), icon: Some("trash".into()), action: Some("removeAppInstance".into()), destructive: Some(true), ..Default::default() });
    }
    menu.build()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️SpaceCommand
app_commands! {
    /// 🎯️ `SpaceApp::Command` — the SOLE dispatch surface for the studio app's own behavior, one
    /// variant per action declared in `create_space_app`'s manifest.
    pub enum SpaceCommand for WorkflowSnapshot, WorkflowMutation, SpaceConfig, SpaceConfigMutation {
        // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
        "patchParameter" as "patch-parameter" => patch_parameter::PatchParameter,
        "addParameter" as "add-parameter" => add_parameter::AddParameter,
        "removeParameter" as "remove-parameter" => remove_parameter::RemoveParameter,
        "spawnApp" as "spawn-app" => spawn_app::SpawnApp,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "disconnectMediaEdge" as "disconnect-media-edge" => disconnect_media_edge::DisconnectMediaEdge,
        "removeAppInstance" as "remove-app-instance" => remove_app_instance::RemoveAppInstance,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "copyAppInstance" as "copy-app-instance" => copy_app_instance::CopyAppInstance,
        "duplicateAppInstance" as "duplicate-app-instance" => duplicate_app_instance::DuplicateAppInstance,
        "pasteAppInstance" as "paste-app-instance" => paste_app_instance::PasteAppInstance,
        "renameAppInstance" as "rename-app-instance" => rename_app_instance::RenameAppInstance,
        "patchMediaNodes" as "patch-media-nodes" => patch_media_nodes::PatchMediaNodes,
        "patchAppInstances" as "patch-app-instances" => patch_app_instances::PatchAppInstances,
        "bindParameterField" as "bind-parameter-field" => bind_parameter_field::BindParameterField,
        "unbindParameterField" as "unbind-parameter-field" => unbind_parameter_field::UnbindParameterField,
        "reorganizeWorkflow" as "reorganize-workflow" => reorganize_workflow::ReorganizeWorkflow,
        "workflowEngagementSubmit" as "workflow-engagement-submit" => workflow_engagement_submit::WorkflowEngagementSubmit,
        "compiledDagEngagementSubmit" as "compiled-dag-engagement-submit" => compiled_dag_engagement_submit::CompiledDagEngagementSubmit,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,

        // 👁️ Config-only — emit `config_mutations`, never document operations.
        "setActivePanelTab" as "active-panel-tab" => set_active_panel_tab::SetActivePanelTab,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "workflowEngagementInput" as "workflow-engagement-input" => workflow_engagement_input::WorkflowEngagementInput,
        "compiledDagEngagementInput" as "compiled-dag-engagement-input" => compiled_dag_engagement_input::CompiledDagEngagementInput,

        // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "exportMedia" as "export-media" => export_media::ExportMedia,
        "importMedia" as "import-media" => import_media::ImportMedia,
        "importMediaPayload" as "import-media-payload" => import_media_payload::ImportMediaPayload,
        "exportStudioPack" as "export-studio-pack" => export_studio_pack::ExportStudioPack,
        "exportStudioDsl" as "export-studio-dsl" => export_studio_dsl::ExportStudioDsl,
        "importSpacePack" as "import-space-pack" => import_space_pack::ImportSpacePack,
        "importSpacePackPayload" as "import-space-pack-payload" => import_space_pack_payload::ImportSpacePackPayload,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "openInstance" as "open-instance" => open_instance::OpenInstance,
        "closeFocusedInstance" as "close-focused-instance" => close_focused_instance::CloseFocusedInstance,
        "goHome" as "go-home" => go_home::GoHome,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "setAppRegistrations" as "set-app-registrations" => set_app_registrations::SetAppRegistrations,
    }
}
//#endregion 🔖️SpaceCommand

//#region 🔖️SpaceApp
/// 🧪️ Unit app instance — config lives in `SpaceConfig`; remote presence stays unavailable until
/// `ArtifactHost` supplies its typed instance-owned presence view.
#[derive(Default, Clone, Copy)]
pub struct SpaceApp;

//#region 🧵️RetainedCommands
const SPACE_BOUNDED_TOOL_IDS: &[&str] = &[
    "setActivePanelTab",
    "nodeGraphViewport",
    "presenceHeartbeat",
    "workflowEngagementInput",
    "compiledDagEngagementInput",
    "closeFocusedInstance",
    "setActiveExample",
    "importSpacePack",
    "goHome",
    "navigateVirtualFileSystemNode",
    "spawnApp",
    "openSpace",
    "openInstance",
    "importSpacePackPayload",
    "setAppRegistrations",
];
/// 🧵️ Still batch-only, honestly: every id here is a workflow-graph or media edit the shell never
/// dispatches on its own (no `🏛️ShellHost`/`🕸️NodeGraph` call site, verified by id), and each needs
/// its own reducer/extent review before it can claim a bounded first step.
#[cfg(test)]
const SPACE_BATCH_ONLY_TOOL_IDS: &[&str] = &[
    "patchParameter",
    "addParameter",
    "removeParameter",
    "moveMediaNode",
    "connectMediaPorts",
    "disconnectMediaEdge",
    "removeAppInstance",
    "deleteSelection",
    "copyAppInstance",
    "duplicateAppInstance",
    "pasteAppInstance",
    "renameAppInstance",
    "patchMediaNodes",
    "patchAppInstances",
    "bindParameterField",
    "unbindParameterField",
    "reorganizeWorkflow",
    "workflowEngagementSubmit",
    "compiledDagEngagementSubmit",
    "nodeGraphEdit",
    "exportMedia",
    "importMedia",
    "importMediaPayload",
    "exportStudioPack",
    "exportStudioDsl",
];
const SPACE_RETAINED_PAYLOAD_SCHEMA: &str = "os.workflow.space.tool-command.v1";
/// 📦️ `setAppRegistrations` carries the whole live catalog as one JSON argument and
/// `importSpacePackPayload` a whole base64 `.pack` data URL, so the studio's wire ceiling is sized for
/// a catalog, not for a click.
const SPACE_BOUNDED_RAW_BYTES: usize = 4 * 1024 * 1024;
const SPACE_BOUNDED_WORK_ITEMS: usize = 1;

/// 🧾️ The single contract this app's factory publishes AND its proof rows carry —
/// `validate_tool_job_rows` joins the two by exact equality, so both sides must read it from here.
const SPACE_BOUNDED_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

fn space_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SPACE_BOUNDED_RAW_BYTES, 64, SPACE_BOUNDED_WORK_ITEMS as u64, SPACE_BOUNDED_OUTPUT_BYTES, 7_500)
}

fn space_bounded_extent(command: &SpaceCommand, _snapshot: &WorkflowSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SPACE_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(SPACE_BOUNDED_WORK_ITEMS)
}

fn space_bounded_reduce(
    command: &SpaceCommand,
    snapshot: &WorkflowSnapshot,
    config: &SpaceConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<SpaceApp>>,
    operation: &AppOperationContext,
) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, NoDraftMutation>, Fault> {
    if !SPACE_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.route"), "the bounded Space reducer rejects document, registry, payload, and graph routes"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    // 🕹️ `openInstance` with no explicit node falls back to the live `graph` selection, which the
    // macro-generated 3-arg `dispatch` cannot see — the retained path reads it off the same
    // `InteractionState` `SpaceApp::handle` passes to `open_instance::apply`.
    if let SpaceCommand::OpenInstance(payload) = command {
        let selected = interaction.selection.get(S_PLAY_INTERACTION_DOMAIN).map_or_else(Vec::new, |selection| selection.ids.clone());
        return Ok(crate::engine::space::engine::resolve_future(open_instance::open_with_selection(payload, &doc, config, &selected)));
    }
    command.dispatch(&doc, &ConfigView { snapshot: config })
}

pub struct SpaceCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SpaceCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SPACE_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SpaceCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<SpaceApp>;
    type Job = ArtifactRetainedCommandJob<SpaceApp>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SPACE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        space_bounded_contract()
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
        if input.declared_bytes() > SPACE_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Space command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SpaceCommandJobFactory {
    type Owner = SpaceApp;
    const TOOL_IDS: &'static [&'static str] = SPACE_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = S_WORKFLOW_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActivePanelTab", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "workflowEngagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "compiledDagEngagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "closeFocusedInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSpacePack", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "goHome", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "navigateVirtualFileSystemNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "spawnApp", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openSpace", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openInstance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSpacePackPayload", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setAppRegistrations", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
/// 📏️ Sized for a real session, not for a single toggle: `openSpace` publishes a space id, a focus
/// reset, a clipboard reset and an active node in one turn, and a studio's clipboard/collapsed sets
/// are node-id lists.
const SPACE_CONFIG_MAXIMUM_BYTES: usize = 65_536;
const SPACE_CONFIG_MAXIMUM_ITEMS: usize = 256;
const SPACE_CONFIG_TEXT_BYTES: usize = 8_192;
const SPACE_CONFIG_METADATA_BYTES: usize = 64;

struct SpaceConfigPreparationFactory;

struct SpaceConfigPreparation {
    base: Option<store::SnapshotRead<SpaceConfig>>,
    mutation: Option<SpaceConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(SpaceConfig, SpaceConfigMutation, SpaceConfigMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<SpaceConfig, SpaceConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

fn space_config_bytes(config: &SpaceConfig) -> Result<usize, String> {
    let items = config.camera.len().saturating_add(config.collapsed_node_ids.len()).saturating_add(config.preview_off_node_ids.len()).saturating_add(config.clipboard_node_ids.len());
    if items > SPACE_CONFIG_MAXIMUM_ITEMS {
        return Err("Space Config exceeds its retained item envelope".into());
    }
    let mut bytes = 0usize;
    for value in config.camera.keys().chain(config.collapsed_node_ids.iter()).chain(config.preview_off_node_ids.iter()).chain(config.clipboard_node_ids.iter()) {
        bytes = bytes.saturating_add(value.len());
    }
    for value in [&config.active_node_id, &config.focused_node_id, &config.pending_import_node_id, &config.pending_import_format, &config.space_id, &config.client_id, &config.client_name] {
        bytes = bytes.saturating_add(value.as_ref().map_or(0, String::len));
    }
    for value in [&config.workflow_engagement_input, &config.compiled_dag_engagement_input, &config.active_panel_tab] {
        bytes = bytes.saturating_add(value.len());
    }
    if bytes > SPACE_CONFIG_TEXT_BYTES {
        return Err("Space Config exceeds its encoded text envelope".into());
    }
    let bytes = bytes.saturating_add(size_of::<SpaceConfig>()).saturating_add(items.saturating_mul(128));
    if bytes > SPACE_CONFIG_MAXIMUM_BYTES {
        return Err("Space Config exceeds its retained byte envelope".into());
    }
    Ok(bytes)
}

fn space_config_mutation_bytes(mutation: &SpaceConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        SpaceConfigMutation::SetActivePanelTab { tab_id } => tab_id.len(),
        SpaceConfigMutation::SetCamera { window_id, .. } => window_id.len(),
        SpaceConfigMutation::SetClient { client_id, client_name } => client_id.as_ref().map_or(0, String::len).saturating_add(client_name.as_ref().map_or(0, String::len)),
        SpaceConfigMutation::SetWorkflowEngagementInput { value } | SpaceConfigMutation::SetCompiledDagEngagementInput { value } => value.len(),
        SpaceConfigMutation::SetActiveNode { node_id } | SpaceConfigMutation::SetFocusedNode { node_id } => node_id.as_ref().map_or(0, String::len),
        SpaceConfigMutation::SetSpaceId { space_id } => space_id.as_ref().map_or(0, String::len),
        SpaceConfigMutation::SetClipboard { node_ids } => node_ids.iter().map(String::len).sum(),
        _ => return Err("Space Config preparation rejects a non-retained mutation".into()),
    };
    if bytes > SPACE_CONFIG_TEXT_BYTES {
        return Err("Space Config mutation exceeds its encoded text envelope".into());
    }
    let bytes = bytes.saturating_add(size_of::<SpaceConfigMutation>());
    if bytes > SPACE_CONFIG_MAXIMUM_BYTES {
        return Err("Space Config mutation exceeds its retained byte envelope".into());
    }
    Ok(bytes)
}

fn prepare_space_config(base: &SpaceConfig, mutation: SpaceConfigMutation) -> Result<(SpaceConfig, SpaceConfigMutation, SpaceConfigMutation), String> {
    space_config_bytes(base)?;
    space_config_mutation_bytes(&mutation)?;
    let mut post = base.clone();
    let inverse = match &mutation {
        SpaceConfigMutation::SetActivePanelTab { tab_id } => {
            post.active_panel_tab = tab_id.clone();
            SpaceConfigMutation::SetActivePanelTab { tab_id: base.active_panel_tab.clone() }
        }
        SpaceConfigMutation::SetCamera { window_id, camera } => {
            post.camera.insert(window_id.clone(), *camera);
            base.camera.get(window_id).map_or_else(|| SpaceConfigMutation::Snapshot { config: base.clone() }, |camera| SpaceConfigMutation::SetCamera { window_id: window_id.clone(), camera: *camera })
        }
        SpaceConfigMutation::SetClient { client_id, client_name } => {
            post.client_id = client_id.clone();
            post.client_name = client_name.clone();
            SpaceConfigMutation::SetClient { client_id: base.client_id.clone(), client_name: base.client_name.clone() }
        }
        SpaceConfigMutation::SetWorkflowEngagementInput { value } => {
            post.workflow_engagement_input = value.clone();
            SpaceConfigMutation::SetWorkflowEngagementInput { value: base.workflow_engagement_input.clone() }
        }
        SpaceConfigMutation::SetCompiledDagEngagementInput { value } => {
            post.compiled_dag_engagement_input = value.clone();
            SpaceConfigMutation::SetCompiledDagEngagementInput { value: base.compiled_dag_engagement_input.clone() }
        }
        SpaceConfigMutation::SetFocusedNode { node_id } => {
            post.focused_node_id = node_id.clone();
            SpaceConfigMutation::SetFocusedNode { node_id: base.focused_node_id.clone() }
        }
        SpaceConfigMutation::SetActiveNode { node_id } => {
            post.active_node_id = node_id.clone();
            SpaceConfigMutation::SetActiveNode { node_id: base.active_node_id.clone() }
        }
        SpaceConfigMutation::SetSpaceId { space_id } => {
            post.space_id = space_id.clone();
            SpaceConfigMutation::SetSpaceId { space_id: base.space_id.clone() }
        }
        SpaceConfigMutation::SetClipboard { node_ids } => {
            post.clipboard_node_ids = node_ids.clone();
            SpaceConfigMutation::SetClipboard { node_ids: base.clipboard_node_ids.clone() }
        }
        _ => return Err("Space Config preparation rejects a non-retained mutation".into()),
    };
    space_config_bytes(&post)?;
    Ok((post, inverse, mutation))
}

fn space_config_edit(forward: SpaceConfigMutation, inverse: SpaceConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<SpaceConfigMutation> {
    let id = format!("space-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse: vec![inverse],
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

impl store::ArtifactStoreOneItemPreparationFactory<SpaceConfig, SpaceConfigMutation> for SpaceConfigPreparationFactory {
    fn preflight(&self, mutation: &SpaceConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > SPACE_CONFIG_METADATA_BYTES) {
            return Err("Space Config preparation rejects its lane or description envelope".into());
        }
        space_config_mutation_bytes(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: SPACE_CONFIG_MAXIMUM_BYTES * 4 + 1_024 })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<SpaceConfig, SpaceConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SpaceConfig, SpaceConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<SpaceConfig, SpaceConfigMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SPACE_CONFIG_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SpaceConfigPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<SpaceConfig, SpaceConfigMutation> for SpaceConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Space Config preparation lost its exact base".to_string())?.get();
            let mutation = self.mutation.as_ref().ok_or_else(|| "Space Config preparation lost its mutation".to_string())?;
            space_config_bytes(base)?;
            space_config_mutation_bytes(mutation)?;
            let bytes = SPACE_CONFIG_MAXIMUM_BYTES * 4 + 1_024;
            if grant.maximum_bytes < bytes {
                return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            self.candidate = Some(prepare_space_config(base, self.mutation.take().ok_or_else(|| "Space Config preparation lost its mutation".to_string())?)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Space Config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Space Config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(space_config_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<SpaceConfig, SpaceConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<SpaceConfig, SpaceConfigMutation>> {
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
        if let Some(mutation) = self.mutation.as_ref() {
            let bytes = space_config_mutation_bytes(mutation)?;
            if grant.maximum_bytes < bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
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
                return Err("Space Config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

impl ArtifactApp for SpaceApp {
    const DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.space.studio", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
    type Snapshot = WorkflowSnapshot;
    type Mutation = WorkflowMutation;
    type Config = SpaceConfig;
    type ConfigMutation = SpaceConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SpacePresence;
    type PresenceMutation = SpacePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = SpaceCommand;

    const APP_ID: &'static str = S_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = S_WORKFLOW_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: SpaceApp,
        owner_file: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs",
        controller: "s.space.studio@1/*#editor",
        document_schema: "os.workflow",
        factory: "SpaceCommandJobFactory",
        factory_type: SpaceCommandJobFactory,
        contract: space_bounded_contract(),
        tools: [
            "setActivePanelTab",
            "nodeGraphViewport",
            "presenceHeartbeat",
            "workflowEngagementInput",
            "compiledDagEngagementInput",
            "closeFocusedInstance",
            "setActiveExample",
            "importSpacePack",
            "goHome",
            "navigateVirtualFileSystemNode",
            "spawnApp",
            "openSpace",
            "openInstance",
            "importSpacePackPayload",
            "setAppRegistrations",
        ]
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::space_retained_store_preparation::<Self::Snapshot, Self::Mutation>("space-studio-artifact-retained", SPACE_BOUNDED_OUTPUT_BYTES)
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SpaceCommandJobFactory::new(&controller))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(SpaceConfigPreparationFactory))
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !SPACE_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.tool-mismatch"), "Space command does not match its exact registered tool"));
        }
        if space_bounded_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.retained.extent"), "Space bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, space_bounded_reduce, space_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            SpaceCommand::command_id,
            SPACE_BOUNDED_RAW_BYTES,
            SPACE_BOUNDED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn initial_snapshot() -> WorkflowSnapshot {
        empty_workflow_snapshot().await
    }

    async fn command_id(command: &SpaceCommand) -> &'static str {
        command.command_id()
    }

    /// 🪪️ `s.space.space`'s config+presence schema descriptor (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `register_document_app` registers it the
    /// moment this type is bound to the plugin, completing the app-schema declaration for `🪐️space`.
    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::engine::space::config::schema::app_schema_descriptor().await)
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `SpaceCommand` until every call site speaks OpBinary.
    async fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<SpaceCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_f64);
        let string_vec = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
        let json_field = |key: &str| args.and_then(|value| value.get(key)).map(|raw| raw.as_str().map_or_else(|| pack::json::to_string(&pack::json::from_dsl_value(raw)), str::to_string));
        let node_id = || str_field("nodeId").or_else(|| str_field("node_id")).or_else(|| str_field("instanceId")).or_else(|| str_field("instance_id"));
        match action {
            "patchParameter" => Ok(SpaceCommand::PatchParameter(patch_parameter::PatchParameter {
                parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value: json_field("value").unwrap_or_else(|| "null".into()),
            })),
            "addParameter" => Ok(SpaceCommand::AddParameter(add_parameter::AddParameter { name: str_field("name").unwrap_or_else(|| "Parameter".into()), kind: str_field("kind").or_else(|| str_field("type")).unwrap_or_else(|| "numeric".into()) })),
            "removeParameter" => Ok(SpaceCommand::RemoveParameter(remove_parameter::RemoveParameter { parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default() })),
            "spawnApp" => Ok(SpaceCommand::SpawnApp(spawn_app::SpawnApp {
                plugin_id: str_field("pluginId").or_else(|| str_field("plugin_id")).unwrap_or_default(),
                app_id: str_field("appId").or_else(|| str_field("app_id")).unwrap_or_default(),
                x: f64_field("x").unwrap_or(0.0),
                y: f64_field("y").unwrap_or(0.0),
            })),
            "moveMediaNode" => Ok(SpaceCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: node_id().unwrap_or_default(), x: f64_field("x").unwrap_or(0.0), y: f64_field("y").unwrap_or(0.0) })),
            "connectMediaPorts" => Ok(SpaceCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_field("sourceNodeId").or_else(|| str_field("source_node_id")).unwrap_or_default(),
                source_port_id: str_field("sourcePortId").or_else(|| str_field("source_port_id")).unwrap_or_default(),
                target_node_id: str_field("targetNodeId").or_else(|| str_field("target_node_id")).unwrap_or_default(),
                target_port_id: str_field("targetPortId").or_else(|| str_field("target_port_id")).unwrap_or_default(),
            })),
            "disconnectMediaEdge" => Ok(SpaceCommand::DisconnectMediaEdge(disconnect_media_edge::DisconnectMediaEdge { edge_id: str_field("edgeId").or_else(|| str_field("edge_id")).unwrap_or_default() })),
            "removeAppInstance" => Ok(SpaceCommand::RemoveAppInstance(remove_app_instance::RemoveAppInstance { node_id: node_id(), surface_contexts: Default::default() })),
            "deleteSelection" => Ok(SpaceCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "copyAppInstance" => Ok(SpaceCommand::CopyAppInstance(copy_app_instance::CopyAppInstance {})),
            "duplicateAppInstance" => Ok(SpaceCommand::DuplicateAppInstance(duplicate_app_instance::DuplicateAppInstance {})),
            "pasteAppInstance" => Ok(SpaceCommand::PasteAppInstance(paste_app_instance::PasteAppInstance {})),
            "renameAppInstance" => Ok(SpaceCommand::RenameAppInstance(rename_app_instance::RenameAppInstance { label: str_field("label").or_else(|| str_field("name")), surface_contexts: Default::default() })),
            "patchMediaNodes" => {
                let ids = string_vec("nodeIds");
                Ok(SpaceCommand::PatchMediaNodes(patch_media_nodes::PatchMediaNodes {
                    node_ids: if ids.is_empty() { node_id().map(|id| vec![id]).unwrap_or_default() } else { ids },
                    field: str_field("field").unwrap_or_default(),
                    axis: str_field("axis"),
                    value: json_field("value").unwrap_or_else(|| "null".into()),
                }))
            }
            "patchAppInstances" => {
                let ids = string_vec("nodeIds");
                Ok(SpaceCommand::PatchAppInstances(patch_app_instances::PatchAppInstances {
                    node_ids: if ids.is_empty() { node_id().map(|id| vec![id]).unwrap_or_default() } else { ids },
                    field: str_field("field").unwrap_or_default(),
                    value: json_field("value").unwrap_or_else(|| "null".into()),
                }))
            }
            "bindParameterField" => Ok(SpaceCommand::BindParameterField(bind_parameter_field::BindParameterField {
                node_id: node_id().unwrap_or_default(),
                field_path: str_field("fieldPath").or_else(|| str_field("field_path")).unwrap_or_default(),
                parameter_id: str_field("parameterId").or_else(|| str_field("parameter_id")).unwrap_or_default(),
            })),
            "unbindParameterField" => {
                Ok(SpaceCommand::UnbindParameterField(unbind_parameter_field::UnbindParameterField { node_id: node_id().unwrap_or_default(), field_path: str_field("fieldPath").or_else(|| str_field("field_path")).unwrap_or_default() }))
            }
            "reorganizeWorkflow" => Ok(SpaceCommand::ReorganizeWorkflow(reorganize_workflow::ReorganizeWorkflow {})),
            "workflowEngagementSubmit" => Ok(SpaceCommand::WorkflowEngagementSubmit(workflow_engagement_submit::WorkflowEngagementSubmit { value: str_field("value") })),
            "compiledDagEngagementSubmit" => Ok(SpaceCommand::CompiledDagEngagementSubmit(compiled_dag_engagement_submit::CompiledDagEngagementSubmit {})),
            "nodeGraphEdit" => Ok(SpaceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: json_field("operations").or_else(|| json_field("operationsJson")).unwrap_or_else(|| "[]".into()) })),
            "setActivePanelTab" => Ok(SpaceCommand::SetActivePanelTab(set_active_panel_tab::SetActivePanelTab { tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default() })),
            "nodeGraphViewport" => Ok(SpaceCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: json_field("viewport").or_else(|| json_field("viewportJson")).unwrap_or_else(|| "{}".into()) })),
            "presenceHeartbeat" => Ok(SpaceCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat { client_id: str_field("clientId").or_else(|| str_field("client_id")).unwrap_or_default(), name: str_field("name").unwrap_or_default() })),
            "workflowEngagementInput" => Ok(SpaceCommand::WorkflowEngagementInput(workflow_engagement_input::WorkflowEngagementInput { value: str_field("value").unwrap_or_default() })),
            "compiledDagEngagementInput" => Ok(SpaceCommand::CompiledDagEngagementInput(compiled_dag_engagement_input::CompiledDagEngagementInput { value: str_field("value").unwrap_or_default() })),
            "setActiveExample" => Ok(SpaceCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").or_else(|| str_field("example_id")).unwrap_or_default() })),
            "exportMedia" => Ok(SpaceCommand::ExportMedia(export_media::ExportMedia { node_id: node_id().unwrap_or_default(), format: str_field("format").unwrap_or_default() })),
            "importMedia" => Ok(SpaceCommand::ImportMedia(import_media::ImportMedia { node_id: node_id().unwrap_or_default(), format: str_field("format").unwrap_or_default() })),
            "importMediaPayload" => Ok(SpaceCommand::ImportMediaPayload(import_media_payload::ImportMediaPayload { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "exportStudioPack" => Ok(SpaceCommand::ExportStudioPack(export_studio_pack::ExportStudioPack {})),
            "exportStudioDsl" => Ok(SpaceCommand::ExportStudioDsl(export_studio_dsl::ExportStudioDsl {})),
            "importSpacePack" => Ok(SpaceCommand::ImportSpacePack(import_space_pack::ImportSpacePack {})),
            "importSpacePackPayload" => Ok(SpaceCommand::ImportSpacePackPayload(import_space_pack_payload::ImportSpacePackPayload { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "openSpace" => Ok(SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "openInstance" => Ok(SpaceCommand::OpenInstance(open_instance::OpenInstance { node_id: node_id() })),
            "closeFocusedInstance" => Ok(SpaceCommand::CloseFocusedInstance(close_focused_instance::CloseFocusedInstance {})),
            "goHome" => Ok(SpaceCommand::GoHome(go_home::GoHome {})),
            "navigateVirtualFileSystemNode" => Ok(SpaceCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).or_else(|| str_field("nodeId")).or_else(|| str_field("node_id")).unwrap_or_default(),
            })),
            "setAppRegistrations" => Ok(SpaceCommand::SetAppRegistrations(set_app_registrations::SetAppRegistrations { json: json_field("json").unwrap_or_else(|| "[]".into()) })),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.unhandled-action"), format!("space: unhandled action id {other}"))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`reorganizeWorkflow`/`copyAppInstance`/
    /// `duplicateAppInstance`/`removeAppInstance`/`renameAppInstance`/`openInstance` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose per-row
    /// `$module::handle(payload, doc, cfg)` signature is framework-fixed and has no `interaction`
    /// slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    async fn handle(
        command: &SpaceCommand,
        doc: &ArtifactView<'_, WorkflowSnapshot>,
        cfg: &ConfigView<'_, SpaceConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WorkflowMutation, SpaceConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SpaceCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::ReorganizeWorkflow(payload) => reorganize_workflow::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::CopyAppInstance(payload) => copy_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::DuplicateAppInstance(payload) => duplicate_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::RemoveAppInstance(payload) => remove_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::RenameAppInstance(payload) => rename_app_instance::apply(payload, doc, cfg, interaction).await,
            SpaceCommand::OpenInstance(payload) => open_instance::apply(payload, doc, cfg, interaction).await,
            _ => command.dispatch(doc, cfg),
        }
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every workflow node is registered at both the
    /// "instance" and "media-node" granularities (a node IS the app instance now, see the kernel
    /// `🔁️workflow` crate's `🔖️InstanceIdentity` doc) so `selectAll`/range-selection behave correctly
    /// under either granularity; no real parent/child structure exists in the node graph, so every
    /// node is a root.
    async fn interaction_topology(doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> InteractionTopology {
        let mut ordered = Vec::new();
        for node in &doc.snapshot.graph.nodes {
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "instance".into(), parent: None });
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "media-node".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(S_PLAY_INTERACTION_DOMAIN.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let projection = doc.snapshot;
        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels::<SStudioLabels>(view_state);
        match body_key {
            crate::engine::space::modes::main::windows::workflow::S_PLAY_BODY_WORKFLOW => {
                crate::engine::space::modes::main::windows::workflow::render(&SpaceApp, projection, config).await.map(semio_framework_plugin::built_to_component_tree)
            }
            crate::engine::space::modes::main::windows::media_vfs::S_PLAY_BODY_MEDIA_VFS => crate::engine::space::modes::main::windows::media_vfs::render(projection, view_state).await.map(semio_framework_plugin::built_to_component_tree),
            crate::engine::space::modes::main::windows::compiled_dag::S_PLAY_BODY_COMPILED_DAG => crate::engine::space::modes::main::windows::compiled_dag::render(projection).await.map(semio_framework_plugin::built_to_component_tree),
            S_PLAY_CATALOGUE_BODY_KEY => crate::engine::space::panels::catalogue::build_catalogue_tree(labels, view_state.locale).await.map(semio_framework_plugin::built_to_component_tree),
            // 🧬️ `parameters`/`inspection` are now ported to the contract `BuiltNode` DSL (SEMANTIC-
            // UI-CONTRACT-AND-RENDERER-FAMILY, 26/08/20) — both `render` fns are U1-sync (the contract
            // builder's own sync-only ruling), so no `.await` here, only the `map` bridge into `ComponentTree`.
            S_PLAY_PARAMETERS_BODY_KEY => crate::engine::space::panels::parameters::render(projection, labels).map(semio_framework_plugin::built_to_component_tree),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // inspector degrades to its "no selection" default until a future wave threads interaction
            // into render. Flagged as a discovered framework gap, not worked around here.
            S_PLAY_INSPECTOR_BODY_KEY => crate::engine::space::panels::inspection::render(projection, &[], labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    async fn window_measures(doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        HashMap::from([(crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), crate::engine::space::modes::main::windows::workflow::window_measures(cfg.snapshot, &doc.snapshot.graph.nodes, view_state).await)])
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` — see ticket 26/08/14's
    /// w3b-summary.md), so the selection-dependent rows below always take the "nothing selected"
    /// branch rather than reading a stale/wrong selection.
    async fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, WorkflowSnapshot>,
        _cfg: &ConfigView<'_, SpaceConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let labels = semio_framework_plugin::resolve_labels::<SStudioLabels>(view_state);
        let is_de = matches!(view_state.locale, semio_framework_plugin::Locale::De);
        space_workflow_context_menu_items(registry, labels, is_de, request.surface.as_ref(), &[]).await
    }
}
//#endregion 🔖️SpaceApp

//#region 🔖️SpaceManifest
async fn space_play_layout() -> WindowLayout {
    create_default_layout(
        &[
            crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(),
            crate::engine::space::modes::main::windows::media_vfs::S_PLAY_WINDOW_MEDIA_VFS.into(),
            crate::engine::space::modes::main::windows::compiled_dag::S_PLAY_WINDOW_COMPILED_DAG.into(),
        ],
        "row",
        Some(&[40.0, 30.0, 30.0]),
        Some(&["Workflow".into(), "Media VFS".into(), "Compiled DAG".into()]),
    )
}

pub async fn create_space_app() -> App {
    use crate::engine::space::modes::main::windows::{compiled_dag, media_vfs, workflow};
    let builder = App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space")).await.document(["semio", "s", "studio"])
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setAppRegistrations", LocalizedLabel::native("Set App Registrations", "App-Registrierungen festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("App Registrations", "App-Registrierungen"))]) }).await
        .icon_id("s").await
        .mode_def(crate::engine::space::modes::main::definition().await).await
        .default_mode_id("main").await
        .window_kind_def(workflow::definition().await).await
        .window_kind_def(media_vfs::definition().await).await
        .window_kind_def(compiled_dag::definition().await).await
        .panel_tab_def(crate::engine::space::panels::catalogue::definition().await).await
        .panel_tab_def(crate::engine::space::panels::parameters::definition().await).await
        .panel_tab_def(crate::engine::space::panels::inspection::definition().await).await
        .default_layout(space_play_layout().await).await
        .mutation("patchParameter", LocalizedLabel::native("Patch Parameter", "Parameter aktualisieren")).await
        .mutation("addParameter", LocalizedLabel::native("Add Parameter", "Parameter hinzufügen")).await
        .mutation("removeParameter", LocalizedLabel::native("Remove Parameter", "Parameter entfernen")).await
        .mutation("spawnApp", LocalizedLabel::native("Spawn App", "App erzeugen")).await
        .mutation("moveMediaNode", LocalizedLabel::native("Move Media Node", "Medienknoten verschieben")).await
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Media Ports", "Medien-Ports verbinden")).await
        .mutation("disconnectMediaEdge", LocalizedLabel::native("Disconnect Media Edge", "Medienverbindung trennen")).await
        .action_with(ActionDefinition::bounded_catalog("removeAppInstance", LocalizedLabel::native("Remove App Instance", "App-Instanz entfernen"), ActionKind::Mutation).with_category("selection")).await
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen")).await
        .action_with(ActionDefinition::bounded_catalog("copyAppInstance", LocalizedLabel::native("Copy App Instance", "App-Instanz kopieren"), ActionKind::Mutation).with_category("transfer")).await
        .action_with(ActionDefinition::bounded_catalog("duplicateAppInstance", LocalizedLabel::native("Duplicate App Instance", "App-Instanz duplizieren"), ActionKind::Mutation).with_category("create")).await
        .action_with(ActionDefinition::bounded_catalog("pasteAppInstance", LocalizedLabel::native("Paste App Instance", "App-Instanz einfügen"), ActionKind::Mutation).with_category("transfer")).await
        .action_with(ActionDefinition::bounded_catalog("renameAppInstance", LocalizedLabel::native("Rename App Instance", "App-Instanz umbenennen"), ActionKind::Mutation).with_category("settings")).await
        .mutation("patchMediaNodes", LocalizedLabel::native("Patch Media Nodes", "Medienknoten aktualisieren")).await
        .mutation("patchAppInstances", LocalizedLabel::native("Patch App Instances", "App-Instanzen aktualisieren")).await
        .mutation("bindParameterField", LocalizedLabel::native("Bind Parameter Field", "Parameterfeld verknüpfen")).await
        .mutation("unbindParameterField", LocalizedLabel::native("Unbind Parameter Field", "Parameterfeld lösen")).await
        .action_with(ActionDefinition::bounded_catalog("reorganizeWorkflow", LocalizedLabel::native("Reorganize Workflow", "Workflow neu anordnen"), ActionKind::Mutation).with_category("transform")).await
        .mutation("workflowEngagementSubmit", LocalizedLabel::native("Workflow Engagement Submit", "Workflow-Eingabe bestätigen")).await
        .mutation("compiledDagEngagementSubmit", LocalizedLabel::native("Compiled DAG Engagement Submit", "Kompilierter-DAG-Eingabe bestätigen")).await
        .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Workflow", "Workflow bearbeiten")).await
        // 🕹️ Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
        // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
        // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
        .view_action("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen")).await
        .view_action("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansichtsfenster festlegen")).await
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Anwesenheits-Heartbeat")).await
        .view_action("workflowEngagementInput", LocalizedLabel::native("Workflow Engagement Input", "Workflow-Eingabe")).await
        .view_action("compiledDagEngagementInput", LocalizedLabel::native("Compiled DAG Engagement Input", "Kompilierter-DAG-Eingabe")).await
        .shell_action("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen")).await
        .shell_action("exportMedia", LocalizedLabel::native("Export Media", "Medien exportieren")).await
        .shell_action("importMedia", LocalizedLabel::native("Import Media", "Medien importieren")).await
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importMediaPayload", LocalizedLabel::native("Import Media Payload", "Medien-Payload importieren"), ActionKind::Shell) }).await
        .shell_action("exportStudioPack", LocalizedLabel::native("Export Studio Pack", "Studio-Paket exportieren")).await
        .shell_action("exportStudioDsl", LocalizedLabel::native("Export Studio DSL", "Studio-DSL exportieren")).await
        .shell_action("importSpacePack", LocalizedLabel::native("Import Studio Pack", "Studio-Paket importieren")).await
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSpacePackPayload", LocalizedLabel::native("Import Studio Pack Payload", "Studio-Paket-Payload importieren"), ActionKind::Shell) }).await
        .shell_action("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen")).await
        .action_with(ActionDefinition::bounded_catalog("openInstance", LocalizedLabel::native("Open Instance", "Instanz öffnen"), ActionKind::Shell).with_category("open")).await
        .shell_action("closeFocusedInstance", LocalizedLabel::native("Close Focused Instance", "Fokussierte Instanz schließen")).await
        .shell_action("goHome", LocalizedLabel::native("Go Home", "Zur Startseite")).await
        .shell_action("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren")).await
        .action_interactive_job("patchParameter", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("addParameter", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("removeParameter", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("spawnApp", InteractiveJobClassification::Migrated).await
        .action_interactive_job("moveMediaNode", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("connectMediaPorts", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("disconnectMediaEdge", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("removeAppInstance", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("deleteSelection", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("copyAppInstance", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("duplicateAppInstance", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("pasteAppInstance", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("renameAppInstance", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("patchMediaNodes", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("patchAppInstances", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("bindParameterField", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("unbindParameterField", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("reorganizeWorkflow", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("workflowEngagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("compiledDagEngagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("setActivePanelTab", InteractiveJobClassification::Migrated).await
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated).await
        .action_interactive_job("presenceHeartbeat", InteractiveJobClassification::Migrated).await
        .action_interactive_job("workflowEngagementInput", InteractiveJobClassification::Migrated).await
        .action_interactive_job("compiledDagEngagementInput", InteractiveJobClassification::Migrated).await
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated).await
        .action_interactive_job("exportMedia", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("importMedia", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("importMediaPayload", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("exportStudioPack", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("exportStudioDsl", InteractiveJobClassification::BatchOnlyPendingRewrite).await
        .action_interactive_job("importSpacePack", InteractiveJobClassification::Migrated).await
        .action_interactive_job("importSpacePackPayload", InteractiveJobClassification::Migrated).await
        .action_interactive_job("openSpace", InteractiveJobClassification::Migrated).await
        .action_interactive_job("openInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("closeFocusedInstance", InteractiveJobClassification::Migrated).await
        .action_interactive_job("goHome", InteractiveJobClassification::Migrated).await
        .action_interactive_job("navigateVirtualFileSystemNode", InteractiveJobClassification::Migrated).await
        .action_interactive_job("setAppRegistrations", InteractiveJobClassification::Migrated).await
        // 📝️ Staged argument form for parameter creation (spawnApp/exportMedia stay context/registry-driven).
        .action_args("addParameter", vec![
            ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value(&"Parameter"),
            ActionArgDef::select("type", LocalizedLabel::native("Type", "Typ"), vec![
                ActionArgOption::new("numeric", LocalizedLabel::native("Numeric", "Numerisch")),
                ActionArgOption::new("categorical", LocalizedLabel::native("Categorical", "Kategorisch")),
                ActionArgOption::new("toggle", LocalizedLabel::native("Toggle", "Schalter")),
                ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
            ]).default_value(&"numeric"),
        ]).await
        // 📇️ Per-window action scoping — the Workflow (NodeGraph) window owns all graph/instance/
        // parameter editing plus the per-instance media import/export; the Media VFS
        // (VirtualFileSystem) window only navigates the media file tree; the read-only Compiled DAG
        // window only drives its own engagement. Navigation, panel-tab, presence, example and generic
        // node-graph view actions stay unscoped orphans and appear on every window.
        .window_kind_action_refs(workflow::S_PLAY_WINDOW_WORKFLOW, vec![
            "patchParameter".into(), "addParameter".into(), "removeParameter".into(),
            "spawnApp".into(), "moveMediaNode".into(), "connectMediaPorts".into(), "disconnectMediaEdge".into(),
            "removeAppInstance".into(), "deleteSelection".into(), "copyAppInstance".into(),
            "duplicateAppInstance".into(), "pasteAppInstance".into(), "renameAppInstance".into(),
            "patchMediaNodes".into(), "patchAppInstances".into(), "bindParameterField".into(),
            "unbindParameterField".into(), "reorganizeWorkflow".into(), "workflowEngagementSubmit".into(),
            "workflowEngagementInput".into(), "nodeGraphEdit".into(), "exportMedia".into(),
            "importMedia".into(), "importMediaPayload".into(),
        ]).await
        .window_kind_action_refs(media_vfs::S_PLAY_WINDOW_MEDIA_VFS, vec![
            "navigateVirtualFileSystemNode".into(),
        ]).await
        .window_kind_action_refs(compiled_dag::S_PLAY_WINDOW_COMPILED_DAG, vec![
            "compiledDagEngagementSubmit".into(), "compiledDagEngagementInput".into(),
        ]).await
        // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // one domain over the workflow node graph, "instance"/"media-node" granularities (a node IS
        // the app instance now — see the kernel `🔁️workflow` crate's `🔖️InstanceIdentity` doc),
        // `HierarchyProvider::Topology` (see `SpaceApp::interaction_topology` above) — the node graph
        // itself is flat (no parent/child structure), so the topology exists purely so a deleted
        // node's id auto-prunes out of `graph`'s selection.
        .interaction(InteractionDefinition {
            id: S_PLAY_INTERACTION_DOMAIN.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "instance".into(), label: LocalizedLabel::native("Instance", "Instanz"), icon_id: "box".into() },
                GranularityDefinition { id: "media-node".into(), label: LocalizedLabel::native("Media Node", "Medienknoten"), icon_id: "circle".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick],
                merges: vec![MergeMode::Replace],
                transitive: false,
                broadcast: true,
            },
        }).await
        .window_kind_interactions(workflow::S_PLAY_WINDOW_WORKFLOW, vec![InteractionRef::new(S_PLAY_INTERACTION_DOMAIN)]).await
        .keybinding("mod+z", "undo").await
        .keybinding("mod+shift+z", "redo").await
        .keybinding("mod+s", "commitCheckpoint").await;
    let definition = builder.build_definition();
    let app = App { definition, examples: Vec::new() };
    let mut app = app.workflow("s", "S Studio", "studio").await;
    for (id, label) in S_STUDIO_EXAMPLES {
        // 🚧️ `OsWorkflowArtifactDocument` (= `BackboneDocument<WorkflowSnapshot, WorkflowMutation>`)
        // is deeply framework-owned (`ArtifactVcs`/`ArtifactCursor`/`Edit`/`Conflict`/...) and still
        // only derives `Serialize`, not `ToValue` — bridging the whole envelope is out of this
        // ticket's plugin-slice scope (tracked with the in-flight `BackboneDocument: ToValue`
        // framework work). The example payload only ever needed the bare `WorkflowSnapshot` anyway
        // (`parse_demo_space_document`'s own doc: "the fixture holds only the `WorkflowSnapshot`
        // payload"), which already derives `ToValue` — read it straight off `.vcs.initial_snapshot`.
        let snapshot = parse_demo_space_document().await.vcs.initial_snapshot;
        let document_value = dsl::to_dsl_value(&snapshot).expect("serialize demo studio document");
        let json = pack::json_to_string_pretty(&pack::json_from_dsl_value(&document_value));
        // 📊️ `label` is sourced from `S_STUDIO_EXAMPLES` — no per-locale split is available at the
        // source, so it is genuine runtime data here, not compile-checked native copy.
        app = app.example(*id, LocalizedLabel::data(*label), json, "file-text").await;
    }
    app
}
//#endregion 🔖️SpaceManifest

//#region 🧪️Testkit
/// 🧪️ Shared test harness — every command-group test file needs this, added here first per the
/// per-app recipe so other nodes don't each re-derive it.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
