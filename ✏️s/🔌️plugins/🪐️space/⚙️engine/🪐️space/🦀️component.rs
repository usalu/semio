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
//! `🎮️commands/<group>/🦀️component.rs` payload module (see `app_commands!` below).

use crate::engine::space::commands::{connect_media_ports, disconnect_media_edge};
use crate::engine::space::commands::{compiled_dag_engagement_input, compiled_dag_engagement_submit, workflow_engagement_input, workflow_engagement_submit};
use crate::engine::space::commands::node_graph_edit;
use crate::engine::space::commands::{close_focused_instance, open_instance};
use crate::engine::space::commands::{export_media, import_media, import_media_payload};
use crate::engine::space::commands::{go_home, navigate_virtual_file_system_node, set_active_panel_tab, set_app_registrations};
use crate::engine::space::commands::{copy_app_instance, delete_selection, duplicate_app_instance, move_media_node, patch_app_instances, patch_media_nodes, paste_app_instance, remove_app_instance, rename_app_instance, reorganize_workflow, spawn_app};
use crate::engine::space::commands::{add_parameter, bind_parameter_field, patch_parameter, remove_parameter, unbind_parameter_field};
use crate::engine::space::commands::presence_heartbeat;
use crate::engine::space::commands::node_graph_viewport;
use crate::engine::space::commands::{export_studio_dsl, export_studio_pack, import_space_pack, import_space_pack_payload, open_space, set_active_example};
use crate::engine::space::config::SpaceConfig;
use crate::engine::space::presence::{SpacePresence, SpacePresenceMutation};
use crate::engine::space::terminology::SStudioLabels;
use crate::parse_demo_space_document;
use semio_framework_os::{create_os_id, empty_workflow_snapshot, MediaContract, WorkflowSnapshot, WorkflowEdge, WorkflowMutation, S_WORKFLOW_SCHEMA};
use semio_framework_plugin::{
    app::InteractionView, NoDraft, NoDraftMutation, DraftView, app_commands, create_default_layout, host_now_ms, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, CommandDefinition, ConfigView, ArtifactApp, ArtifactView,
    DomainTopology, Emit, Fault, FaultOrigin, GranularityDefinition, HierarchyProvider, HostEffect, HoverSpec, InteractionDefinition, InteractionRef, InteractionTarget, InteractionTopology, Label, LocalizedLabel, MergeMode, SelectionMethod,
    SelectionMode, SelectionSpec, TopologyNode, UiNode, WindowLayout, CLEAR_SELECTION_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID,
};
use store::EngineHandles;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

//#region 🔖️Constants
/// 🪪️ Canonical surface id (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1: every
/// `App::builder(...)` id must parse via `semio_framework::parse_surface_app_id` as
/// `<artifact_kind>@<standard>/<subset>#<role>`) — was the bare `"studio"`, which stopped parsing the
/// moment that migration's validation landed in `PluginBuilder::document_app`'s `build_definition`.
pub const S_PLAY_APP_ID: &str = "s.space.studio@1/*#editor";
pub const S_PLAY_CONTROLLER_ID: &str = "s-play";
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
pub(crate) fn s_play_action(action: &str, args: Option<Value>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(S_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `graph` domain — replaces the deleted
/// `selectInstance`/`nodeGraphSelect`/`setMediaNodeSelection`/`setAppInstanceSelection` action builders
/// every measure/document row used to construct by hand.
pub(crate) fn space_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    s_play_action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": S_PLAY_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

/// @emoji 🤝️ Resolves the source/target ports for a proposed connect and negotiates their wire contract
/// via `engine::negotiate_media_connect`, converting a rejection into a `Notify` effect — shared by
/// `connections::connect_media_ports` and the `graph_edit::node_graph_edit`/`"connect"` fixture edit.
pub(crate) fn negotiate_connect_or_notify(projection: &WorkflowSnapshot, source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str) -> Result<MediaContract, HostEffect> {
    crate::engine::space::engine::negotiate_media_connect(projection, source_node_id, source_port_id, target_node_id, target_port_id).map_err(|reason| HostEffect::Notify { message: reason })
}

pub(crate) fn connect_edge_operation(source_node_id: &str, source_port_id: &str, target_node_id: &str, target_port_id: &str, contract: MediaContract) -> WorkflowMutation {
    WorkflowMutation::ConnectPorts {
        edge: WorkflowEdge { id: create_os_id("edge"), source_node_id: source_node_id.into(), source_port_id: source_port_id.into(), target_node_id: target_node_id.into(), target_port_id: target_port_id.into(), contract },
    }
}

/// @emoji 🔎️ First selected node — the fallback target for actions that implicitly operate on "the"
/// current selection (rename/remove/open) when no explicit node id is supplied. `selected` is the
/// `graph` domain's live selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no
/// longer a deleted `SpaceConfig` field.
pub(crate) fn primary_selected_node_id(selected: &[String], config: &SpaceConfig) -> Option<String> {
    selected.first().cloned().or_else(|| config.active_node_id.clone())
}

/// 🔧️ Small pure fold applying a batch of `SpaceConfigMutation`s onto a snapshot — used where a
/// command handler needs the POST-command config (not the pre-command `cfg.snapshot`) to build a
/// derived side value (the presence broadcast) in the very same call, without reaching back into a
/// store this pure function doesn't own.
pub(crate) fn apply_config_mutations(config: &SpaceConfig, operations: &[crate::engine::space::config::SpaceConfigMutation]) -> SpaceConfig {
    use protocol::Mutation;
    operations.iter().fold(config.clone(), |acc, operation| operation.diff(&acc).diff().clone())
}

// 🫀️ The shared `presence:` backbone-URI hack was deleted from os-core — presence now flows through
// the semio_hub's duplex `PresencePeer`/`Presence` frames via `framework/sync`'s `ArtifactEvent::Presence`
// for migrated apps. `s` isn't wired onto `ArtifactHost` yet, so it keeps this tiny self-contained
// in-memory heartbeat map until then — same upsert/prune/exclude-self semantics as before, just owned
// locally instead of delegated to a shared cross-process mechanism.
#[derive(Clone)]
struct SPresencePeerLocal {
    client_id: String,
    name: String,
    selection: Vec<String>,
    updated_at_ms: f64,
}

const S_PRESENCE_STALE_MS: f64 = 15_000.0;

fn presence_refresh_needed(operations: &[crate::engine::space::config::SpaceConfigMutation]) -> bool {
    use crate::engine::space::config::SpaceConfigMutation;
    operations.iter().any(|operation| matches!(operation, SpaceConfigMutation::SetClient { .. } | SpaceConfigMutation::Snapshot { .. }))
}

pub(crate) fn config_space_id(config: &SpaceConfig) -> String {
    config.space_id.clone().unwrap_or_else(|| "default".into())
}

fn shared_presence_peers() -> Arc<Mutex<HashMap<String, HashMap<String, SPresencePeerLocal>>>> {
    static REGISTRY: OnceLock<Arc<Mutex<HashMap<String, HashMap<String, SPresencePeerLocal>>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))).clone()
}

pub(crate) fn presence_peers_json(_app: &SpaceApp, config: &SpaceConfig) -> String {
    let space_id = config_space_id(config);
    let self_client_id = config.client_id.clone().unwrap_or_default();
    let now_ms = host_now_ms();
    let peers: Vec<Value> = shared_presence_peers()
        .lock()
        .ok()
        .and_then(|registry| registry.get(&space_id).cloned())
        .unwrap_or_default()
        .into_values()
        .filter(|peer| peer.client_id != self_client_id && now_ms - peer.updated_at_ms <= S_PRESENCE_STALE_MS)
        .map(|peer| json!({ "clientId": peer.client_id, "name": peer.name, "selectionCount": peer.selection.len() }))
        .collect();
    serde_json::to_string(&peers).unwrap_or_else(|_| "[]".into())
}

pub(crate) fn publish_presence(_app: &SpaceApp, config: &SpaceConfig, selected_node_ids: &[String]) {
    let (Some(client_id), Some(client_name)) = (&config.client_id, &config.client_name) else {
        return;
    };
    let space_id = config_space_id(config);
    let now_ms = host_now_ms();
    if let Ok(mut registry) = shared_presence_peers().lock() {
        let peers = registry.entry(space_id).or_default();
        peers.retain(|_, entry| now_ms - entry.updated_at_ms <= S_PRESENCE_STALE_MS);
        peers.insert(client_id.clone(), SPresencePeerLocal { client_id: client_id.clone(), name: client_name.clone(), selection: selected_node_ids.to_vec(), updated_at_ms: now_ms });
    }
}

/// 🖱️ On-demand space workflow context menu from hit-test and selection snapshot.
fn space_workflow_context_menu_items(
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
            .group("transform", |m| m.item(ContextMenuItemSpec { id: "reorganize".into(), label: Some(labels.context_reorganize.into()), icon: Some("layout-grid".into()), action: Some("reorganizeWorkflow".into()), ..Default::default() }));
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
            .group("settings", |m| m.item(ContextMenuItemSpec { id: "rename-instance".into(), label: Some(labels.context_rename_label.into()), icon: Some("edit-3".into()), action: Some("renameAppInstance".into()), ..Default::default() }));
        if !nodes.is_empty() {
            menu = menu.group("selection", |m| {
                m.item(ContextMenuItemSpec { id: "clear-selection".into(), label: Some(labels.context_clear_selection.into()), icon: Some("square-dashed".into()), action: Some(CLEAR_SELECTION_ACTION_ID.into()), ..Default::default() })
            });
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
    pub enum SpaceCommand for WorkflowSnapshot, WorkflowMutation, SpaceConfig, crate::engine::space::config::SpaceConfigMutation {
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
/// 🧪️ App instance — config lives in `SpaceConfig` via `SpaceConfigMutation`s; ephemeral presence
/// heartbeats stay on the app instance until ArtifactHost presence wiring lands.
#[derive(Default, Clone, Copy)]
pub struct SpaceApp;

impl ArtifactApp for SpaceApp {
    type Snapshot = WorkflowSnapshot;
    type Mutation = WorkflowMutation;
    type Config = SpaceConfig;
    type ConfigMutation = crate::engine::space::config::SpaceConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SpacePresence;
    type PresenceMutation = SpacePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = SpaceCommand;

    const APP_ID: &'static str = S_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = S_WORKFLOW_SCHEMA;

    fn initial_snapshot() -> WorkflowSnapshot {
        empty_workflow_snapshot()
    }

    fn command_id(command: &SpaceCommand) -> &'static str {
        command.command_id()
    }

    /// 🪪️ `s.space.space`'s config+presence schema descriptor (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `register_document_app` registers it the
    /// moment this type is bound to the plugin, completing the app-schema declaration for `🪐️space`.
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::engine::space::config::schema::app_schema_descriptor())
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `SpaceCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<SpaceCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(|raw| raw.as_f64().or_else(|| raw.as_i64().map(|n| n as f64)).or_else(|| raw.as_u64().map(|n| n as f64)));
        let string_vec = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_array).map(|items| items.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
        let json_field = |key: &str| args.and_then(|value| value.get(key)).map(|raw| if let Some(text) = raw.as_str() { text.to_string() } else { raw.to_string() });
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
            "removeAppInstance" => Ok(SpaceCommand::RemoveAppInstance(remove_app_instance::RemoveAppInstance { node_id: node_id() })),
            "deleteSelection" => Ok(SpaceCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "copyAppInstance" => Ok(SpaceCommand::CopyAppInstance(copy_app_instance::CopyAppInstance {})),
            "duplicateAppInstance" => Ok(SpaceCommand::DuplicateAppInstance(duplicate_app_instance::DuplicateAppInstance {})),
            "pasteAppInstance" => Ok(SpaceCommand::PasteAppInstance(paste_app_instance::PasteAppInstance {})),
            "renameAppInstance" => Ok(SpaceCommand::RenameAppInstance(rename_app_instance::RenameAppInstance { label: str_field("label").or_else(|| str_field("name")) })),
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
            "unbindParameterField" => Ok(SpaceCommand::UnbindParameterField(unbind_parameter_field::UnbindParameterField { node_id: node_id().unwrap_or_default(), field_path: str_field("fieldPath").or_else(|| str_field("field_path")).unwrap_or_default() })),
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
            other => Err(Fault::new(FaultOrigin::App, semio_framework_plugin::FaultCode::new("s.space.unhandled-action"), format!("space: unhandled action id {other}"))),
        }
    }

    /// 🕹️ `deleteSelection`/`nodeGraphEdit`/`reorganizeWorkflow`/`copyAppInstance`/
    /// `duplicateAppInstance`/`removeAppInstance`/`renameAppInstance`/`openInstance` read the `graph`
    /// interaction domain directly (bypassing the `app_commands!`-generated `dispatch`, whose per-row
    /// `$module::handle(payload, doc, cfg)` signature is framework-fixed and has no `interaction`
    /// slot) — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(command: &SpaceCommand, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<WorkflowMutation, crate::engine::space::config::SpaceConfigMutation, Self::DraftMutation>, Fault> {
        let emit = match command {
            SpaceCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            SpaceCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, interaction),
            SpaceCommand::ReorganizeWorkflow(payload) => reorganize_workflow::apply(payload, doc, cfg, interaction),
            SpaceCommand::CopyAppInstance(payload) => copy_app_instance::apply(payload, doc, cfg, interaction),
            SpaceCommand::DuplicateAppInstance(payload) => duplicate_app_instance::apply(payload, doc, cfg, interaction),
            SpaceCommand::RemoveAppInstance(payload) => remove_app_instance::apply(payload, doc, cfg, interaction),
            SpaceCommand::RenameAppInstance(payload) => rename_app_instance::apply(payload, doc, cfg, interaction),
            SpaceCommand::OpenInstance(payload) => open_instance::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }?;
        if presence_refresh_needed(&emit.config_mutations) {
            let next_config = apply_config_mutations(cfg.snapshot, &emit.config_mutations);
            publish_presence(&SpaceApp::default(), &next_config, &interaction.selection(S_PLAY_INTERACTION_DOMAIN).ids);
        }
        Ok(emit)
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology` — every workflow node is registered at both the
    /// "instance" and "media-node" granularities (a node IS the app instance now, see the kernel
    /// `🔁️workflow` crate's `🔖️InstanceIdentity` doc) so `selectAll`/range-selection behave correctly
    /// under either granularity; no real parent/child structure exists in the node graph, so every
    /// node is a root.
    fn interaction_topology(doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> InteractionTopology {
        let mut ordered = Vec::new();
        for node in &doc.snapshot.graph.nodes {
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "instance".into(), parent: None });
            ordered.push(TopologyNode { id: node.id.clone(), granularity: "media-node".into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(S_PLAY_INTERACTION_DOMAIN.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> UiNode {
        let projection = doc.snapshot;
        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&config.locale);
        // 🪟 `VcsArtifactApp::render` appends `:{windowInstanceId}` when `view_state.window_id` is set —
        // strip it so Space body keys still match.
        let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
        match base_body_key {
            crate::engine::space::modes::main::windows::workflow::S_PLAY_BODY_WORKFLOW => crate::engine::space::modes::main::windows::workflow::render(&SpaceApp::default(), projection, config),
            crate::engine::space::modes::main::windows::media_vfs::S_PLAY_BODY_MEDIA_VFS => crate::engine::space::modes::main::windows::media_vfs::render(projection, &config.locale),
            crate::engine::space::modes::main::windows::compiled_dag::S_PLAY_BODY_COMPILED_DAG => crate::engine::space::modes::main::windows::compiled_dag::render(projection),
            S_PLAY_CATALOGUE_BODY_KEY => crate::engine::space::panels::catalogue::build_catalogue_tree(labels, semio_framework_plugin::locale_from_str(&config.locale)),
            S_PLAY_PARAMETERS_BODY_KEY => crate::engine::space::panels::parameters::render(projection, labels),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // inspector degrades to its "no selection" default until a future wave threads interaction
            // into render. Flagged as a discovered framework gap, not worked around here.
            S_PLAY_INSPECTOR_BODY_KEY => crate::engine::space::panels::inspection::render(projection, &[], labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        HashMap::from([(crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW.into(), crate::engine::space::modes::main::windows::workflow::window_measures(cfg.snapshot, &doc.snapshot.graph.nodes))])
    }

    /// 🕹️ `context_menu` carries no `InteractionView` (same gap as `render` — see ticket 26/08/14's
    /// w3b-summary.md), so the selection-dependent rows below always take the "nothing selected"
    /// branch rather than reading a stale/wrong selection.
    fn context_menu(request: &semio_framework_plugin::ContextMenuRequest, _doc: &ArtifactView<'_, WorkflowSnapshot>, cfg: &ConfigView<'_, SpaceConfig>, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&cfg.snapshot.locale);
        let is_de = cfg.snapshot.locale.starts_with("de");
        space_workflow_context_menu_items(registry, labels, is_de, request.surface.as_ref(), &[])
    }
}
//#endregion 🔖️SpaceApp

//#region 🔖️SpaceManifest
fn space_play_layout() -> WindowLayout {
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

pub fn create_space_app() -> App {
    use crate::engine::space::modes::main::windows::{compiled_dag, media_vfs, workflow};
    let builder = App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space")).document(["semio", "s", "studio"])
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("setAppRegistrations", LocalizedLabel::native("Set App Registrations", "App-Registrierungen festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("App Registrations", "App-Registrierungen"))]) })
        .icon_id("s")
        .mode_def(crate::engine::space::modes::main::definition())
        .default_mode_id("main")
        .window_kind_def(workflow::definition())
        .window_kind_def(media_vfs::definition())
        .window_kind_def(compiled_dag::definition())
        .panel_tab_def(crate::engine::space::panels::catalogue::definition())
        .panel_tab_def(crate::engine::space::panels::parameters::definition())
        .panel_tab_def(crate::engine::space::panels::inspection::definition())
        .default_layout(space_play_layout())
        .mutation("patchParameter", LocalizedLabel::native("Patch Parameter", "Parameter aktualisieren"))
        .mutation("addParameter", LocalizedLabel::native("Add Parameter", "Parameter hinzufügen"))
        .mutation("removeParameter", LocalizedLabel::native("Remove Parameter", "Parameter entfernen"))
        .mutation("spawnApp", LocalizedLabel::native("Spawn App", "App erzeugen"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Media Node", "Medienknoten verschieben"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Media Ports", "Medien-Ports verbinden"))
        .mutation("disconnectMediaEdge", LocalizedLabel::native("Disconnect Media Edge", "Medienverbindung trennen"))
        .action_with(ActionDefinition::new_catalog("removeAppInstance", LocalizedLabel::native("Remove App Instance", "App-Instanz entfernen"), ActionKind::Mutation).with_category("selection"))
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
        .action_with(ActionDefinition::new_catalog("copyAppInstance", LocalizedLabel::native("Copy App Instance", "App-Instanz kopieren"), ActionKind::Mutation).with_category("transfer"))
        .action_with(ActionDefinition::new_catalog("duplicateAppInstance", LocalizedLabel::native("Duplicate App Instance", "App-Instanz duplizieren"), ActionKind::Mutation).with_category("create"))
        .action_with(ActionDefinition::new_catalog("pasteAppInstance", LocalizedLabel::native("Paste App Instance", "App-Instanz einfügen"), ActionKind::Mutation).with_category("transfer"))
        .action_with(ActionDefinition::new_catalog("renameAppInstance", LocalizedLabel::native("Rename App Instance", "App-Instanz umbenennen"), ActionKind::Mutation).with_category("settings"))
        .mutation("patchMediaNodes", LocalizedLabel::native("Patch Media Nodes", "Medienknoten aktualisieren"))
        .mutation("patchAppInstances", LocalizedLabel::native("Patch App Instances", "App-Instanzen aktualisieren"))
        .mutation("bindParameterField", LocalizedLabel::native("Bind Parameter Field", "Parameterfeld verknüpfen"))
        .mutation("unbindParameterField", LocalizedLabel::native("Unbind Parameter Field", "Parameterfeld lösen"))
        .action_with(ActionDefinition::new_catalog("reorganizeWorkflow", LocalizedLabel::native("Reorganize Workflow", "Workflow neu anordnen"), ActionKind::Mutation).with_category("transform"))
        .mutation("workflowEngagementSubmit", LocalizedLabel::native("Workflow Engagement Submit", "Workflow-Eingabe bestätigen"))
        .mutation("compiledDagEngagementSubmit", LocalizedLabel::native("Compiled DAG Engagement Submit", "Kompilierter-DAG-Eingabe bestätigen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Workflow", "Workflow bearbeiten"))
        // 🕹️ Selection/hover are the framework's `graph` interaction domain now (`.interaction(...)`
        // below) — the six framework verbs (`interactionSelect`/`interactionHover`/`clearSelection`/
        // `selectAll`/`setSelectionMode`/`setInteractionGranularity`) auto-inject.
        .view_action("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen"))
        .view_action("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansichtsfenster festlegen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Anwesenheits-Heartbeat"))
        .view_action("workflowEngagementInput", LocalizedLabel::native("Workflow Engagement Input", "Workflow-Eingabe"))
        .view_action("compiledDagEngagementInput", LocalizedLabel::native("Compiled DAG Engagement Input", "Kompilierter-DAG-Eingabe"))
        .shell_action("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
        .shell_action("exportMedia", LocalizedLabel::native("Export Media", "Medien exportieren"))
        .shell_action("importMedia", LocalizedLabel::native("Import Media", "Medien importieren"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importMediaPayload", LocalizedLabel::native("Import Media Payload", "Medien-Payload importieren"), ActionKind::Shell) })
        .shell_action("exportStudioPack", LocalizedLabel::native("Export Studio Pack", "Studio-Paket exportieren"))
        .shell_action("exportStudioDsl", LocalizedLabel::native("Export Studio DSL", "Studio-DSL exportieren"))
        .shell_action("importSpacePack", LocalizedLabel::native("Import Studio Pack", "Studio-Paket importieren"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importSpacePackPayload", LocalizedLabel::native("Import Studio Pack Payload", "Studio-Paket-Payload importieren"), ActionKind::Shell) })
        .shell_action("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"))
        .action_with(ActionDefinition::new_catalog("openInstance", LocalizedLabel::native("Open Instance", "Instanz öffnen"), ActionKind::Shell).with_category("open"))
        .shell_action("closeFocusedInstance", LocalizedLabel::native("Close Focused Instance", "Fokussierte Instanz schließen"))
        .shell_action("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"))
        .shell_action("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"))
        // 📝️ Staged argument form for parameter creation (spawnApp/exportMedia stay context/registry-driven).
        .action_args("addParameter", vec![
            ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).default_value("Parameter"),
            ActionArgDef::select("type", LocalizedLabel::native("Type", "Typ"), vec![
                ActionArgOption::new("numeric", LocalizedLabel::native("Numeric", "Numerisch")),
                ActionArgOption::new("categorical", LocalizedLabel::native("Categorical", "Kategorisch")),
                ActionArgOption::new("toggle", LocalizedLabel::native("Toggle", "Schalter")),
                ActionArgOption::new("text", LocalizedLabel::native("Text", "Text")),
            ]).default_value("numeric"),
        ])
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
        ])
        .window_kind_action_refs(media_vfs::S_PLAY_WINDOW_MEDIA_VFS, vec![
            "navigateVirtualFileSystemNode".into(),
        ])
        .window_kind_action_refs(compiled_dag::S_PLAY_WINDOW_COMPILED_DAG, vec![
            "compiledDagEngagementSubmit".into(), "compiledDagEngagementInput".into(),
        ])
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
        })
        .window_kind_interactions(workflow::S_PLAY_WINDOW_WORKFLOW, vec![InteractionRef::new(S_PLAY_INTERACTION_DOMAIN)])
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .keybinding("mod+s", "commitCheckpoint");
    let definition = builder.build_definition();
    let mut app = App { definition, examples: Vec::new() };
    app.definition.controller_id = S_PLAY_CONTROLLER_ID.into();
    let mut app = app.workflow("s", "S Studio", "studio");
    for (id, label) in S_STUDIO_EXAMPLES {
        let json = serde_json::to_string_pretty(&parse_demo_space_document()).expect("serialize demo studio document");
        // 📊️ `label` is sourced from `S_STUDIO_EXAMPLES` — no per-locale split is available at the
        // source, so it is genuine runtime data here, not compile-checked native copy.
        app = app.example(*id, LocalizedLabel::data(*label), json, "file-text");
    }
    app
}
//#endregion 🔖️SpaceManifest

//#region 🧪️Testkit
/// 🧪️ Shared test harness — every command-group test file needs this, added here first per the
/// per-app recipe so other nodes don't each re-derive it.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_os::{MediaPortDirection, MediaPortSpec, MediaType, WorkflowMediaPort, WorkflowNode};
    use semio_framework_os::{apply_workflow_operation, register_app_io, ArtifactPresentation, MediaClass, MediaForm, PortMultiplicity};
    use semio_framework_plugin::{App, AppIo, HistoryView, LocalizedLabel, SurfaceKind};

    pub(crate) fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    pub type SpaceVcsApp = semio_framework_plugin::VcsArtifactApp<SpaceApp>;

    /// 🕹️ A fresh, real `VcsArtifactApp<SpaceApp>` — the only way a downstream crate can obtain a
    /// genuine `InteractionView` (its fields are framework-crate-private; ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so any test exercising the `graph`
    /// domain's real selection/hover state must go through this, not `studio_emit`.
    pub(crate) fn app() -> SpaceVcsApp {
        semio_framework_plugin::testkit::new_app::<SpaceApp>()
    }

    /// 🕹️ Registry-backed counterpart of `app()` — carries the manifest's real `AppActionRegistry`
    /// (including the declared `graph` interaction domain), needed by any test that dispatches a
    /// framework interaction verb (`interactionSelect`/`interactionHover`/…) via `handle_action`, which
    /// faults with "undeclared interaction domain" against the bare, registry-less `app()`.
    pub(crate) fn app_with_registry() -> SpaceVcsApp {
        semio_framework_plugin::testkit::new_app_with_registry::<SpaceApp>(create_space_app)
    }

    pub(crate) fn dispatch(app: &mut SpaceVcsApp, command: SpaceCommand) -> semio_framework_plugin::InvocationResult {
        app.dispatch_typed(command, &semio_framework_plugin::testkit::meta("local")).expect("dispatch")
    }

    /// 🕹️ Routes through `SpaceCommand::dispatch` (the `app_commands!`-generated, framework-fixed
    /// 3-arg path — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), NOT
    /// `SpaceApp::handle` (which now needs a real `InteractionView`, only obtainable through a full
    /// `VcsArtifactApp` dispatch — see `testkit::app`/`dispatch` below for that path). The 7 commands
    /// that read live selection (`deleteSelection`/`nodeGraphEdit`/`reorganizeWorkflow`/
    /// `copyAppInstance`/`duplicateAppInstance`/`removeAppInstance`/`renameAppInstance`) fall back to
    /// treating the selection as empty here — exactly the same degradation `SpaceApp::render`'s own
    /// selection-dependent branches already carry — so this helper stays usable for every OTHER
    /// command's non-selection-dependent behavior unchanged.
    pub(crate) fn studio_emit(projection: &WorkflowSnapshot, config: &SpaceConfig, command: &SpaceCommand) -> Result<Emit<WorkflowMutation, crate::engine::space::config::SpaceConfigMutation>, Fault> {
        let history = empty_history();
        let doc = ArtifactView::new(projection, &history);
        let cfg = ConfigView { snapshot: config };
        command.dispatch(&doc, &cfg)
    }

    /// 📽️ Folds studio document operations onto a projection the way the store would (minus history).
    pub(crate) fn apply_mutations(projection: &WorkflowSnapshot, operations: &[WorkflowMutation]) -> WorkflowSnapshot {
        operations.iter().fold(projection.clone(), |current, operation| apply_workflow_operation(&current, operation))
    }

    /// 📽️ Folds studio config operations onto a config snapshot the way the store would.
    pub(crate) fn apply_config(config: &SpaceConfig, operations: &[crate::engine::space::config::SpaceConfigMutation]) -> SpaceConfig {
        apply_config_mutations(config, operations)
    }

    /// 🪪️ Canonical surface id for a synthetic test-registry app (ticket
    /// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — every `App::builder(...)` id must parse
    /// via `semio_framework::parse_surface_app_id`, so this mirrors `surface_app_id` over a throwaway
    /// `s.<slug>@1/*` dialect. Shared by `seed_app` and every command test module that dispatches
    /// `SpawnApp`/looks the registration back up, so both sides agree on the same string.
    pub(crate) fn test_surface_id(slug: &str) -> String {
        semio_framework::surface_app_id(&semio_framework::ArtifactDialect { artifact_kind: format!("s.{slug}"), standard: "1".into(), subset: "*".into() }, semio_framework::AppRole::Editor)
    }

    fn seed_app(plugin_id: &str, app_id: &str, label: &str, document: &[&str], document_schema: &str, ports: Vec<MediaPortSpec>) {
        let surface_id = test_surface_id(app_id);
        let definition = App::builder(surface_id, LocalizedLabel::data(label))
            .document(document.iter().map(|segment| segment.to_string()))
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind("main", LocalizedLabel::native("Main", "Hauptansicht"), format!("{app_id}.main"), SurfaceKind::Canvas2d, "square-pen")
            .io(AppIo::from_document(document_schema, MediaType { class: MediaClass::Data, form: MediaForm::Value }, ArtifactPresentation { id: app_id.into(), name: label.into(), dimension: String::new(), component_kind: app_id.into() }).with_ports(ports))
            .build_definition();
        register_app_io(plugin_id, &definition);
    }

    pub(crate) fn seed_draw_plugin() {
        seed_app("draw", "draw", "Draw", &["semio", "draw"], "draw.document", Vec::new());
    }

    pub(crate) fn seed_multi_port_plugins() {
        let puzzle_ports = vec![
            MediaPortSpec { id: "in-a".into(), label: "In A".into(), direction: MediaPortDirection::In, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
            MediaPortSpec { id: "out-a".into(), label: "Out A".into(), direction: MediaPortDirection::Out, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
            MediaPortSpec { id: "out-b".into(), label: "Out B".into(), direction: MediaPortDirection::Out, media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, kind_id: Some("topology".into()), required: false, multiplicity: PortMultiplicity::One },
        ];
        seed_app("puzzle.5d", "puzzle5d", "Puzzle 5D", &["semio", "puzzle", "5d"], "puzzle5d.document", puzzle_ports);

        let shooting_ports = vec![MediaPortSpec { id: "scene-in".into(), label: "Scene".into(), direction: MediaPortDirection::In, media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, kind_id: Some("2d.shooting".into()), required: true, multiplicity: PortMultiplicity::One }];
        seed_app("shooting", "shooting", "Shooting", &["semio", "shooting"], "shooting.document", shooting_ports);
    }

    pub(crate) fn test_node(id: &str, inputs: Vec<WorkflowMediaPort>, outputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode { id: id.into(), plugin_id: "test".into(), app_id: "test".into(), label: id.into(), yields: String::new(), artifact_ref: format!("artifacts/{id}"), config_ref: format!("config/{id}"), x: 0.0, y: 0.0, width: 1.0, height: 1.0, inputs, outputs }
    }

    pub(crate) fn test_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, media_type: MediaType, kind_id: &str) -> WorkflowMediaPort {
        let dir_word = match direction {
            MediaPortDirection::In => "in",
            MediaPortDirection::Out => "out",
        };
        WorkflowMediaPort { id: format!("{node_id}:{spec_id}:{dir_word}"), spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type, kind_id: Some(kind_id.into()), required: false, multiplicity: PortMultiplicity::One } }
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::testkit::{empty_history, studio_emit};
    use crate::demo_space_projection;
    use semio_framework_plugin::testkit as plugin_testkit;
    use semio_framework_plugin::{PluginApp, VcsArtifactApp};

    #[test]
    fn initial_snapshot_is_empty_not_demo() {
        let app = SpaceApp::default();
        assert!(SpaceApp::initial_snapshot().graph.nodes.is_empty());
    }

    #[test]
    fn demo_document_has_instances_and_edges() {
        let projection = demo_space_projection();
        assert!(projection.graph.nodes.len() >= 5);
        assert!(!projection.graph.edges.is_empty());
        assert!(semio_framework_os::validate_workflow(&projection.graph).ok);
    }

    #[test]
    fn space_window_kind_actions_scope_editing_to_workflow() {
        let definition = create_space_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
        };
        let graph = resolve(crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW);
        let vfs = resolve(crate::engine::space::modes::main::windows::media_vfs::S_PLAY_WINDOW_MEDIA_VFS);
        let dag = resolve(crate::engine::space::modes::main::windows::compiled_dag::S_PLAY_WINDOW_COMPILED_DAG);
        for graph_operation in ["spawnApp", "connectMediaPorts", "removeAppInstance", "exportMedia", "addParameter"] {
            assert!(graph.contains(&graph_operation.to_string()), "Workflow must expose {graph_operation}");
            assert!(!vfs.contains(&graph_operation.to_string()), "Media VFS must NOT expose {graph_operation}");
            assert!(!dag.contains(&graph_operation.to_string()), "Compiled DAG must NOT expose {graph_operation}");
        }
        assert!(vfs.contains(&"navigateVirtualFileSystemNode".to_string()));
        assert!(!graph.contains(&"navigateVirtualFileSystemNode".to_string()));
        assert!(dag.contains(&"compiledDagEngagementSubmit".to_string()));
        assert!(!graph.contains(&"compiledDagEngagementSubmit".to_string()));
        // 🌐️ Global navigation/utility actions stay orphans on every window.
        for shared in ["setActiveExample", "goHome"] {
            assert!(graph.contains(&shared.to_string()) && vfs.contains(&shared.to_string()) && dag.contains(&shared.to_string()), "{shared} stays global");
        }
    }

    #[test]
    fn space_manifest_uses_studio_app_id() {
        let app = create_space_app();
        assert_eq!(app.definition.id, S_PLAY_APP_ID);
        assert_eq!(app.definition.controller_id, "s-play");
    }

    /// 🪪️ `ArtifactStore::dispatch_inner`'s `CommitCheckpoint` arm (🏪️store `🦀️component.rs`) rejects an
    /// empty checkpoint (`VcsError::ValidationFailed("cannot create an empty checkpoint")`) — a
    /// freshly-constructed `VcsArtifactApp` has no uncommitted edits at all (no `genesis()` on
    /// `SpaceApp`, empty `initial_snapshot`), so this must spawn a real edit before committing, exactly
    /// like the sibling `checkout_checkpoint_restores_projection` below.
    #[test]
    fn commit_checkpoint_round_trips_projection() {
        use crate::engine::space::commands::spawn_app;
        use serde_json::json;
        testkit::seed_draw_plugin();
        let mut app = VcsArtifactApp::new(SpaceApp::default());
        app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw"), x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).expect("spawn");
        let before = app.snapshot().expect("projection").graph.nodes.len();
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "snapshot" })), &plugin_testkit::meta("local")).expect("commit");
        assert_eq!(app.snapshot().expect("projection").graph.nodes.len(), before);
    }

    #[test]
    fn checkout_checkpoint_restores_projection() {
        use crate::engine::space::commands::spawn_app;
        use serde_json::json;
        testkit::seed_draw_plugin();
        let mut app = VcsArtifactApp::new(SpaceApp::default());
        let before = app.snapshot().expect("projection").graph.nodes.len();
        app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw"), x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).expect("spawn");
        app.handle_action("commitCheckpoint", Some(&json!({ "message": "after-first-spawn" })), &plugin_testkit::meta("local")).expect("commit");
        let after_first = app.snapshot().expect("projection").graph.nodes.len();
        assert!(after_first > before);
        let files = app.document_pack().expect("document pack");
        let parsed: store::ParsedDocumentText<WorkflowSnapshot, WorkflowMutation> = store::parse_document_pack(&files.pack, &files.spr).expect("parse document pack");
        let checkpoint_id = parsed.envelope.vcs.checkpoints[0].id.clone();
        app.dispatch_typed(SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw"), x: 80.0, y: 80.0 }), &plugin_testkit::meta("local")).expect("spawn2");
        assert!(app.snapshot().expect("projection").graph.nodes.len() > after_first);
        app.handle_action("checkoutCheckpoint", Some(&json!({ "checkpointId": checkpoint_id })), &plugin_testkit::meta("local")).expect("checkout");
        assert_eq!(app.snapshot().expect("projection").graph.nodes.len(), after_first);
    }

    /// 🧪️ The definitional proof: two independent instances start from `SpaceApp::initial_snapshot()`
    /// (genuinely EMPTY — `paired_apps`/`new_app::<A>()` never seed the bundled demo projection; this
    /// test previously assumed otherwise and dispatched a rename against a node id that could never
    /// exist in either instance, which the missing-target guard correctly rejected once the crate could
    /// finally link and run this test for the first time), apply DISJOINT edits (A spawns a "draw"
    /// instance, B spawns a "shooting" instance from a different plugin), and exchanging operations
    /// over a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    ///
    /// 🚧️ BLOCKED (2026-08-17, lane 2-G): fails with `Fault { code: "module.vcs", message:
    /// "validation failed: change ... has an invalid edit reference ..." }` inside `pump b`
    /// (`assert_two_instances_converge`'s own `instance_b.handle_action("commitCheckpoint", ...)`,
    /// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5852`, itself calling into
    /// `🏪️store/🦀️component.rs`'s `validate_durable_history`/`CommitCheckpoint` handling). Confirmed via
    /// two independent isolated (`--test-threads=1`) reproductions that this is NOT specific to
    /// "shooting" or to using two different plugins: substituting a second `draw` spawn (different
    /// position) at B reproduces the identical fault. This is therefore a genuine, pre-existing
    /// framework bug in the backbone-relay + checkpoint path for CREATE-type mutations (`AddNode`),
    /// never exercised before this lane: the ORIGINAL test always failed earlier, at `instance_b`'s own
    /// `dispatch_typed` (missing-target, since it renamed a node id that never existed), so this
    /// checkpoint code path was never reached by any test until the canonical-surface-id fix let this
    /// suite compile and run for the first time. `🏪️store/**` and `🔌️plugin/**` are both under
    /// `🧰️framework/**` — forbidden to this lane. Left failing per the brief's explicit instruction
    /// ("never delete/#\[ignore\] to force green; leave failing + sharedFileRequest"); see this lane's
    /// `📓️w2-g-report.md` for the sharedFileRequest.
    #[test]
    fn two_instances_converge_on_disjoint_edits_via_backbone() {
        use crate::engine::space::commands::spawn_app;
        testkit::seed_draw_plugin();
        testkit::seed_multi_port_plugins();
        plugin_testkit::assert_two_instances_converge::<SpaceApp, (usize, usize)>(
            "mem://s-studio-convergence",
            SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "draw".into(), app_id: testkit::test_surface_id("draw"), x: 80.0, y: 80.0 }),
            SpaceCommand::SpawnApp(spawn_app::SpawnApp { plugin_id: "shooting".into(), app_id: testkit::test_surface_id("shooting"), x: 300.0, y: 100.0 }),
            move |app| {
                let projection = app.snapshot().expect("projection");
                let draw_count = projection.graph.nodes.iter().filter(|node| node.plugin_id == "draw").count();
                let shooting_count = projection.graph.nodes.iter().filter(|node| node.plugin_id == "shooting").count();
                (draw_count, shooting_count)
            },
        );
    }

    #[test]
    fn space_declares_expected_actions_and_examples() {
        let studio = create_space_app();
        let workflow = studio.definition.window_kinds.iter().find(|window| window.id == crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW).expect("workflow window");
        assert!(workflow.actions.iter().any(|action| action.id == "spawnApp"));
        assert!(workflow.actions.iter().any(|action| action.id == "reorganizeWorkflow"));
        let registrations = studio.definition.commands.iter().find(|command| command.id == "setAppRegistrations").expect("host registration command");
        assert!(!registrations.in_palette);
        assert_eq!(registrations.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), vec!["json"]);
        assert_eq!(studio.examples.len(), S_STUDIO_EXAMPLES.len());
    }

    #[test]
    fn space_labels_resolve_native_english_by_default() {
        let projection = demo_space_projection();
        let history = empty_history();
        let doc = ArtifactView::new(&projection, &history);
        let config = SpaceConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let app = SpaceApp::default();
        let catalogue_json = serde_json::to_string(&SpaceApp::render(S_PLAY_CATALOGUE_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(catalogue_json.contains("\"Apps\""));

        let parameters_json = serde_json::to_string(&SpaceApp::render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(parameters_json.contains("Add Parameter"));
        assert!(parameters_json.contains("\"Name\""));
        assert!(parameters_json.contains("\"Remove\""));
        assert!(!parameters_json.contains("Parameter hinzufügen"));
    }

    #[test]
    fn space_labels_resolve_native_german_locale() {
        let projection = demo_space_projection();
        let history = empty_history();
        let doc = ArtifactView::new(&projection, &history);
        let config = SpaceConfig { locale: "de".into(), ..SpaceConfig::default() };
        let cfg = ConfigView { snapshot: &config };
        let app = SpaceApp::default();
        let parameters_json = serde_json::to_string(&SpaceApp::render(S_PLAY_PARAMETERS_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(parameters_json.contains("Parameter hinzufügen"));
        assert!(parameters_json.contains("\"Entfernen\""));
        assert!(!parameters_json.contains("Add Parameter"));

        let inspector_json = serde_json::to_string(&SpaceApp::render(S_PLAY_INSPECTOR_BODY_KEY, &doc, &cfg)).unwrap();
        assert!(inspector_json.contains("Wähle Workflow-Knoten im Arbeitsbereich aus."));
    }

    /// 🗂️ Grouped-disclosure context menu: at most 9 top-level rows (leaves+groups combined) and the
    /// destructive `removeAppInstance` row is always the final top-level entry.
    #[test]
    fn space_workflow_context_menu_stays_within_budget_with_destructive_tail() {
        let registry = semio_framework_plugin::AppActionRegistry::from_definition(&create_space_app().definition);
        let labels = semio_framework_plugin::resolve_labels_for_locale::<SStudioLabels>(&SpaceConfig::default().locale);
        let selected_node_ids = vec!["node-1".to_string()];
        let items = space_workflow_context_menu_items(&registry, labels, false, None, &selected_node_ids);
        assert!(items.len() <= 9, "top-level context menu rows must stay within budget: {} rows", items.len());
        let last = items.last().expect("non-empty menu");
        assert_eq!(last.id, "remove-instance");
        assert_eq!(last.destructive, Some(true), "removeAppInstance must be the last, destructive top-level row");
    }

    // 🌉️ Keeps `studio_emit`/`empty_history` imports exercised at this module's own level too (every
    // command-group file also imports them directly from `testkit`).
    #[test]
    fn testkit_studio_emit_smoke_test() {
        let projection = demo_space_projection();
        let config = SpaceConfig::default();
        let _ = empty_history();
        let _ = studio_emit(&projection, &config, &SpaceCommand::GoHome(go_home::GoHome {})).expect("handle");
    }
}
//#endregion 🧪️Tests
