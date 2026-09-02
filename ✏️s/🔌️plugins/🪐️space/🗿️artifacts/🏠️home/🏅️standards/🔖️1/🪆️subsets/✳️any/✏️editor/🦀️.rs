//! 🏠️ S Home launcher editor — `ArtifactEditor` impl, command dispatch, manifest (constitutional: ui).
//!
//! WIRING + DISPATCH ONLY: every command's real body lives in its own `🎮️commands/<group>/🦀️.rs`
//! payload module (see `app_commands!` below). The catalog/draft/backbone document-helper functions this
//! file used to hold (`catalog_port`, `resolve_studio_document`, `list_all_space_catalog_entries`, …)
//! moved to the PLUGIN ROOT `🦀️.rs` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! W2 packet P7) — they are genuinely shared by 3 surfaces now (this editor, the new `👁️viewer`, and the
//! sibling `🪐️space` studio app's own commands), and a viewer file can never import through `::editor::`
//! (`policyViewerPurityBreaches`), so the shared code cannot live here anymore. Reach it as `crate::X`
//! from any module in this crate.

use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::commands::set_active_panel_tab;
use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space};
use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, fold_directory_events, presence_heartbeat, rename_space, set_client, share_space};
use crate::editor::home::commands::{delete_virtual_file_system_node, go_home, navigate_virtual_file_system_node};
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::editor::home::presence::{HomePresence, HomePresenceMutation};
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::{app_commands, create_tab_stack_layout, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ComponentTree, ConfigView, DraftView, DslValue, Editor, EditorApp, Emit, Fault, FaultOrigin, Label, LocalizedLabel, NoDraft, NoDraftMutation, PluginAssemblyError, UiAssemblyResult};
use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionRef, DialogDefinition};
use store::EngineHandles;

//#region 🔖️Constants
pub const S_HOME_CONTROLLER_ID: &str = "s-home";
//#endregion 🔖️Constants

//#region 🔖️HomeCommand
app_commands! {
    /// 🎯️ `HomeApp::Command` — the SOLE dispatch surface for the Home launcher's own behavior, one
    /// variant per action declared in `create_home_app`'s manifest.
    pub enum HomeCommand for SHomeSnapshot, crate::artifacts::home::op::SHomeMutation, HomeConfig, crate::editor::home::config::HomeConfigMutation {
        "createStudio" as "create-studio" => create_studio::CreateStudio,
        "bindSpaceFile" as "bind-space-file" => bind_space_file::BindSpaceFile,
        "importSpace" as "import-space" => import_space::ImportSpace,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "deleteVirtualFileSystemNode" as "delete-vfs-node" => delete_virtual_file_system_node::DeleteVirtualFileSystemNode,
        "goHome" as "go-home" => go_home::GoHome,
        "setActivePanelTab" as "active-panel-tab" => set_active_panel_tab::SetActivePanelTab,
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: Home = a real table of
        // every space, fed by the event-sourced hub directory read model (contract §C1/§C6).
        "createSpace" as "create-space" => create_space::CreateSpace,
        "deleteSpace" as "delete-space" => delete_space::DeleteSpace,
        "renameSpace" as "rename-space" => rename_space::RenameSpace,
        "shareSpace" as "share-space" => share_space::ShareSpace,
        "copyInviteLink" as "copy-invite-link" => copy_invite_link::CopyInviteLink,
        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "setClient" as "set-client" => set_client::SetClient,
    }
}
//#endregion 🔖️HomeCommand

//#region 🧵️RetainedCommands
const HOME_RETAINED_TOOL_IDS: &[&str] = &[
    "openSpace", "navigateVirtualFileSystemNode", "goHome", "setActivePanelTab", "createSpace", "deleteSpace", "shareSpace", "copyInviteLink", "presenceHeartbeat", "setClient",
];
const HOME_RETAINED_PAYLOAD_SCHEMA: &str = "space.home.tool-command.v1";
const HOME_RETAINED_RAW_BYTES: usize = 8_192;
const HOME_RETAINED_WORK_ITEMS: usize = 1;
const HOME_CONFIG_VALUE_BYTES: usize = 512;
const HOME_CONFIG_BASE_BYTES: usize = 512;
const HOME_CONFIG_STEP_BYTES: usize = 4_096;
const HOME_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "openSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "navigateVirtualFileSystemNode", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "goHome", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setActivePanelTab", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "createSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "deleteSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "shareSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "copyInviteLink", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setClient", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn home_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(HOME_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500, 1, 1)
}

fn home_retained_extent(command: &HomeCommand, _snapshot: &SHomeSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let admitted = match command {
        HomeCommand::OpenSpace(payload) => payload.space_id.len(),
        HomeCommand::NavigateVirtualFileSystemNode(payload) => payload.node_id.len(),
        HomeCommand::GoHome(_) | HomeCommand::PresenceHeartbeat(_) => 0,
        HomeCommand::SetActivePanelTab(payload) => payload.tab_id.len(),
        HomeCommand::CreateSpace(payload) => payload.name.len().saturating_add(payload.kind.len()).saturating_add(payload.visibility.len()),
        HomeCommand::DeleteSpace(payload) => payload.space_id.len(),
        HomeCommand::ShareSpace(payload) => payload.space_id.len().saturating_add(payload.email.len()).saturating_add(payload.role.len()),
        HomeCommand::CopyInviteLink(payload) => payload.space_id.len().saturating_add(payload.role.len()),
        HomeCommand::SetClient(payload) => payload.client_id.len().saturating_add(payload.client_name.len()),
        _ => return None,
    };
    let limit = if matches!(command, HomeCommand::SetActivePanelTab(_) | HomeCommand::SetClient(_)) { HOME_CONFIG_VALUE_BYTES } else { HOME_RETAINED_RAW_BYTES };
    (admitted <= limit).then_some(HOME_RETAINED_WORK_ITEMS)
}

fn home_retained_reduce(
    command: &HomeCommand,
    snapshot: &SHomeSnapshot,
    config: &HomeConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<crate::artifacts::home::op::SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
    if home_retained_extent(command, snapshot, _interaction).is_none() {
        return Err(Fault::from("space-home-retained-route-mismatch"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct HomeRetainedCommandJobFactory { keys: Vec<ToolFactoryKey> }

impl HomeRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: HOME_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for HomeRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<HomeApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<HomeApp>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { HOME_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { home_retained_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > HOME_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Space Home retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for HomeRetainedCommandJobFactory {
    type Owner = EditorApp<HomeApp>;
    const TOOL_IDS: &'static [&'static str] = HOME_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::home::S_HOME_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = HOME_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
struct HomeConfigPreparationFactory;

struct HomeConfigPreparation {
    base: Option<store::SnapshotRead<HomeConfig>>,
    mutation: Option<HomeConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(HomeConfig, HomeConfigMutation, HomeConfigMutation)>,
    sealed_candidate: Option<(HomeConfig, protocol::Edit<HomeConfigMutation>)>,
    serialized_bytes: Option<usize>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn home_config_retained_bytes(config: &HomeConfig) -> usize {
    config.active_panel_tab.len().saturating_add(config.locale.len()).saturating_add(config.directory_json.len()).saturating_add(config.client_id.len()).saturating_add(config.client_name.len())
}

fn home_config_edit(forward: HomeConfigMutation, inverse: HomeConfigMutation, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<HomeConfigMutation> {
    let id = format!("space-home-retained-{}-{}", authority.operation().0, authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![forward], inverse: vec![inverse],
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
        }],
        description, coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
    }
}

struct HomeConfigByteCounter { bytes: usize }

impl std::io::Write for HomeConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > HOME_CONFIG_STEP_BYTES { return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)); }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn home_config_edit_bytes(edit: &protocol::Edit<HomeConfigMutation>) -> Result<usize, String> {
    let bytes = pack::to_json_string(&dsl::ToValue::to_value(edit)).len();
    if bytes > HOME_CONFIG_STEP_BYTES {
        return Err("Space Home config edit exceeds its serialized byte envelope".to_string());
    }
    Ok(bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<HomeConfig, HomeConfigMutation> for HomeConfigPreparationFactory {
    fn preflight(&self, mutation: &HomeConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            HomeConfigMutation::SetActivePanelTab { tab_id } => tab_id.len(),
            HomeConfigMutation::SetClient { client_id, client_name } => client_id.len().saturating_add(client_name.len()),
            _ => return Err("Space Home config preparation rejects non-retained mutations".into()),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > HOME_CONFIG_VALUE_BYTES || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Space Home config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: HOME_CONFIG_STEP_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>> {
        let mutation_bytes = match &request.mutation {
            HomeConfigMutation::SetActivePanelTab { tab_id } => tab_id.len(),
            HomeConfigMutation::SetClient { client_id, client_name } => client_id.len().saturating_add(client_name.len()),
            _ => return Err(request),
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > HOME_CONFIG_VALUE_BYTES || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(HomeConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation> for HomeConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < HOME_CONFIG_STEP_BYTES || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() && self.sealed_candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Space Home config preparation lost its exact base root".to_string())?.get();
            let base_bytes = home_config_retained_bytes(base);
            if base_bytes > HOME_CONFIG_BASE_BYTES { return Err("Space Home config base exceeds retained byte capacity".into()); }
            let mutation = self.mutation.take().ok_or_else(|| "Space Home config preparation lost its mutation owner".to_string())?;
            let mut post = base.clone();
            let inverse = match &mutation {
                HomeConfigMutation::SetActivePanelTab { tab_id } => HomeConfigMutation::SetActivePanelTab { tab_id: std::mem::replace(&mut post.active_panel_tab, tab_id.clone()) },
                HomeConfigMutation::SetClient { client_id, client_name } => HomeConfigMutation::SetClient {
                    client_id: std::mem::replace(&mut post.client_id, client_id.clone()),
                    client_name: std::mem::replace(&mut post.client_name, client_name.clone()),
                },
                _ => return Err("Space Home config preparation received a non-retained mutation".into()),
            };
            self.candidate = Some((post, inverse, mutation));
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if self.sealed_candidate.is_none() {
            let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Space Home config preparation lost its candidate".to_string())?;
            let authority = self.authority.as_ref().ok_or_else(|| "Space Home config preparation lost its Store authority".to_string())?;
            self.sealed_candidate = Some((post, home_config_edit(forward, inverse, self.description.take(), authority)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.sealed_candidate.as_ref().ok_or_else(|| "Space Home config preparation lost its semantic edit".to_string())?;
            let bytes = home_config_edit_bytes(edit)?;
            if bytes.saturating_add(home_config_retained_bytes(post)).saturating_add(512) > HOME_CONFIG_STEP_BYTES {
                return Err("Space Home config publication exceeds the 4096-byte complete envelope".into());
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        let (post, edit) = self.sealed_candidate.take().ok_or_else(|| "Space Home config preparation lost its validated edit".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Space Home config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<HomeConfig, HomeConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if (self.prepared.is_some() || self.sealed_candidate.is_some() || self.candidate.is_some() || self.mutation.is_some() || self.description.is_some()) && grant.maximum_bytes < HOME_CONFIG_STEP_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.take().is_some() || self.sealed_candidate.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: HOME_CONFIG_STEP_BYTES }); }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Space Home config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            let bytes = authority.actor().len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.sealed_candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigStorePreparation

//#region 🔖️HomeApp
/// 🧪️ Unit struct — the Home launcher holds catalog bootstrap ports plus per-session studio port
/// bindings for folder/file-backed studios.
#[derive(Default, Clone, Copy)]
pub struct HomeApp;

impl ArtifactEditor for HomeApp {
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::artifacts::home::op::SHomeMutation;
    type Config = HomeConfig;
    type ConfigMutation = crate::editor::home::config::HomeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = HomePresence;
    type PresenceMutation = HomePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = HomeCommand;

    const DIALECT: Dialect = crate::artifacts::home::HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::home::S_HOME_DOCUMENT_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(HomeConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<HomeApp>,
        owner_file: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s-home",
        document_schema: "s.home",
        factory: "HomeRetainedCommandJobFactory",
        factory_type: HomeRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 65_536, 7_500),
        tools: ["openSpace", "navigateVirtualFileSystemNode", "goHome", "setActivePanelTab", "createSpace", "deleteSpace", "shareSpace", "copyInviteLink", "presenceHeartbeat", "setClient"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(HomeRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !HOME_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("space-home-command-tool-mismatch"));
        }
        if home_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("space-home-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, home_retained_reduce, home_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            HomeCommand::command_id,
            HOME_RETAINED_RAW_BYTES,
            HOME_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> SHomeSnapshot {
        SHomeSnapshot::default()
    }

    fn command_id(command: &HomeCommand) -> &'static str {
        command.command_id()
    }

    /// 🪪️ `s.space.home`'s config+presence schema descriptor (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `register_document_app` registers it the
    /// moment this type is bound to the plugin, completing the app-schema declaration for `🪐️space`.
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        match action {
            "createStudio" => Ok(HomeCommand::CreateStudio(create_studio::CreateStudio {
                name: str_field("name").unwrap_or_else(|| "Untitled".into()),
                kind: str_field("kind").unwrap_or_else(|| "catalog".into()),
                folder_path: str_field("folderPath").or_else(|| str_field("folder_path")),
            })),
            "bindSpaceFile" => Ok(HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                file_path: str_field("filePath").or_else(|| str_field("file_path")).unwrap_or_default(),
            })),
            "importSpace" => Ok(HomeCommand::ImportSpace(import_space::ImportSpace { dsl: str_field("dsl").or_else(|| str_field("payload")) })),
            "openSpace" => Ok(HomeCommand::OpenSpace(open_space::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "navigateVirtualFileSystemNode" => Ok(HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode {
                node_id: str_field("nodeId").or_else(|| str_field("node_id")).or_else(|| str_field("spaceId")).or_else(|| str_field("space_id")).unwrap_or_default(),
            })),
            "deleteVirtualFileSystemNode" => Ok(HomeCommand::DeleteVirtualFileSystemNode(delete_virtual_file_system_node::DeleteVirtualFileSystemNode {
                node_id: str_field("nodeId").or_else(|| str_field("node_id")).or_else(|| str_field("spaceId").map(|id| format!("studio:{id}"))).or_else(|| str_field("space_id").map(|id| format!("studio:{id}"))).unwrap_or_default(),
            })),
            "goHome" => Ok(HomeCommand::GoHome(go_home::GoHome {})),
            "setActivePanelTab" => Ok(HomeCommand::SetActivePanelTab(set_active_panel_tab::SetActivePanelTab { tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default() })),
            "createSpace" => Ok(HomeCommand::CreateSpace(create_space::CreateSpace {
                name: str_field("name").unwrap_or_default(),
                kind: str_field("kind").or_else(|| str_field("spaceKind")).unwrap_or_default(),
                visibility: str_field("visibility").unwrap_or_default(),
            })),
            "deleteSpace" => Ok(HomeCommand::DeleteSpace(delete_space::DeleteSpace {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                confirmed: args.and_then(|value| value.get("confirmed")).and_then(DslValue::as_bool).unwrap_or(false),
            })),
            "renameSpace" => Ok(HomeCommand::RenameSpace(rename_space::RenameSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(), name: str_field("name").unwrap_or_default() })),
            "shareSpace" => {
                Ok(HomeCommand::ShareSpace(share_space::ShareSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(), email: str_field("email").unwrap_or_default(), role: str_field("role").unwrap_or_default() }))
            }
            "copyInviteLink" => Ok(HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
                ttl_secs: args.and_then(|value| value.get("ttlSecs")).and_then(DslValue::as_f64).map(|n| n as u64).unwrap_or(0),
            })),
            "foldDirectoryEvents" => {
                Ok(HomeCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents { events_json: args.and_then(|value| value.get("eventsJson")).and_then(DslValue::as_str).map(str::to_string).unwrap_or_else(|| "[]".into()) }))
            }
            "presenceHeartbeat" => Ok(HomeCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {})),
            "setClient" => Ok(HomeCommand::SetClient(set_client::SetClient {
                client_id: str_field("clientId").or_else(|| str_field("client_id")).unwrap_or_default(),
                client_name: str_field("clientName").or_else(|| str_field("client_name")).unwrap_or_default(),
            })),
            other => Err(Fault::new(FaultOrigin::App, "s.home.unhandled-action", format!("home: unhandled action id {other}"))),
        }
    }

    /// 🕹️ Home declares NO interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// its VFS rows (`🏠️main` window) render through `build_virtual_file_system_scene`, a
    /// `UiNode::ComponentScene` the framework's `stamp_and_cache_interaction_ui` post-pass never walks
    /// (that pass only stamps `UiNode::Tree`), and every row-scoped command (`navigateVirtualFileSystemNode`,
    /// `deleteVirtualFileSystemNode`) already takes an explicit `node_id` argument from the click event
    /// rather than reading a stored selection — there was no bespoke selection/hover config, mutation, or
    /// command here to delete. `_interaction` is accepted (trait-required) and unused.
    fn handle(
        command: &HomeCommand,
        doc: &ArtifactView<'_, SHomeSnapshot>,
        cfg: &ConfigView<'_, HomeConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<crate::artifacts::home::op::SHomeMutation, crate::editor::home::config::HomeConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> UiAssemblyResult<ComponentTree> {
        // 🪟 `VcsArtifactApp::render` appends `:{windowInstanceId}` when `view_state.window_id` is set —
        // strip it so Home's single body key still matches.
        let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
        let root = match base_body_key {
            crate::editor::home::modes::explore::windows::main::S_HOME_BODY => crate::editor::home::modes::explore::windows::main::render(cfg.snapshot)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| PluginAssemblyError::new("s.home.render.unknown-body", "unknown body key text admission failed"))?,
        };
        Ok(ComponentTree { root })
    }
}
//#endregion 🔖️HomeApp

//#region 🔖️HomeManifest
/// 🧱️ The manifest stitch: one call per taxonomy node. `.example(...)`/`.workflow(...)` do not exist on
/// `EditorBuilder` (contract §2.4, W0-F gap 4) — `create_home_app` never called either, so nothing is
/// dropped here (unlike other W2 packets that had to note a loss).
pub async fn create_home_app() -> semio_framework_plugin::AppDefinition {
    let mut definition = Editor::builder(crate::artifacts::home::HOME_DIALECT)
        .document(["semio", "s", "home"])
        .icon_id("home")
        .mode_def(crate::editor::home::modes::explore::definition())
        .default_mode_id("explore")
        .window_kind_def(crate::editor::home::modes::explore::windows::main::definition())
        .default_layout(create_tab_stack_layout(&[crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW.into()], Some(&["Studios".into()])))
        .mutation("createStudio", LocalizedLabel::native("Create Studio", "Studio erstellen"))
        .shell_action("bindSpaceFile", LocalizedLabel::native("Bind Studio File", "Studio-Datei verknüpfen"))
        .mutation("importSpace", LocalizedLabel::native("Import Studio", "Studio importieren"))
        .shell_action("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"))
        .shell_action("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"))
        .mutation("deleteVirtualFileSystemNode", LocalizedLabel::native("Delete File System Node", "Dateisystemknoten löschen"))
        .shell_action("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"))
        .view_action("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen"))
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the overview table's
        // row-scoped actions. Every one of these is a pure `Effect` relay (contract §C6) — never a
        // document mutation — so each is `.shell_action`, matching `openSpace`/`goHome` above, not
        // `.mutation`. `createSpace`/`deleteSpace`/`renameSpace`/`shareSpace` are each their own dialog's
        // submit action too (`DialogDefinition::new(id, …, ActionRef::new(id))`, the same self-
        // referencing shape `PluginBuilder`'s own `declaring_dialog_appends_to_definition` test uses).
        .shell_action("createSpace", LocalizedLabel::native("Create Space", "Space erstellen"))
        .dialog(
            DialogDefinition::new("createSpace", LocalizedLabel::native("Create Space", "Space erstellen"), ActionRef::new("createSpace"))
                .args(vec![
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                    ActionArgDef::select(
                        "kind",
                        LocalizedLabel::native("Kind", "Art"),
                        vec![ActionArgOption::new("atelier", LocalizedLabel::native("Atelier", "Atelier")), ActionArgOption::new("studio", LocalizedLabel::native("Studio", "Studio"))],
                    )
                    .default_value("atelier"),
                    ActionArgDef::select(
                        "visibility",
                        LocalizedLabel::native("Visibility", "Sichtbarkeit"),
                        vec![ActionArgOption::new("private", LocalizedLabel::native("Private", "Privat")), ActionArgOption::new("public", LocalizedLabel::native("Public", "Öffentlich"))],
                    )
                    .default_value("private"),
                ])
                .submit_label(LocalizedLabel::native("Create", "Erstellen")),
        )
        .shell_action("deleteSpace", LocalizedLabel::native("Delete Space", "Space löschen"))
        .dialog(
            DialogDefinition::new("deleteSpace", LocalizedLabel::native("Delete Space?", "Space löschen?"), ActionRef::new("deleteSpace"))
                .body(LocalizedLabel::native("This cannot be undone.", "Dies kann nicht rückgängig gemacht werden."))
                .submit_label(LocalizedLabel::native("Delete", "Löschen")),
        )
        .shell_action("renameSpace", LocalizedLabel::native("Rename Space", "Space umbenennen"))
        .dialog(
            DialogDefinition::new("renameSpace", LocalizedLabel::native("Rename Space", "Space umbenennen"), ActionRef::new("renameSpace"))
                .args(vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required()])
                .submit_label(LocalizedLabel::native("Rename", "Umbenennen")),
        )
        .shell_action("shareSpace", LocalizedLabel::native("Share Space", "Space teilen"))
        .dialog(
            DialogDefinition::new("shareSpace", LocalizedLabel::native("Share Space", "Space teilen"), ActionRef::new("shareSpace"))
                .args(vec![
                    ActionArgDef::text("email", LocalizedLabel::native("Email", "E-Mail")).required(),
                    ActionArgDef::select(
                        "role",
                        LocalizedLabel::native("Role", "Rolle"),
                        vec![ActionArgOption::new("author", LocalizedLabel::native("Author", "Autor")), ActionArgOption::new("spectator", LocalizedLabel::native("Spectator", "Betrachter"))],
                    )
                    .default_value("spectator"),
                ])
                .submit_label(LocalizedLabel::native("Share", "Teilen")),
        )
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))
        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse einspielen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        .view_action("setClient", LocalizedLabel::native("Set Client", "Client setzen"))
        .action_interactive_job("createStudio", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("bindSpaceFile", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("importSpace", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("openSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("navigateVirtualFileSystemNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteVirtualFileSystemNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("goHome", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActivePanelTab", InteractiveJobClassification::Migrated)
        .action_interactive_job("createSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("renameSpace", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("shareSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("foldDirectoryEvents", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("presenceHeartbeat", InteractiveJobClassification::Migrated)
        .action_interactive_job("setClient", InteractiveJobClassification::Migrated)
        .window_kind_action_refs(crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW, vec![
            "createStudio".into(),
            "bindSpaceFile".into(),
            "importSpace".into(),
            "openSpace".into(),
            "navigateVirtualFileSystemNode".into(),
            "deleteVirtualFileSystemNode".into(),
            "goHome".into(),
            "setActivePanelTab".into(),
            "createSpace".into(),
            "deleteSpace".into(),
            "renameSpace".into(),
            "shareSpace".into(),
            "copyInviteLink".into(),
        ])
        .keybinding("mod+n", "createStudio")
        .keybinding("mod+o", "importSpace")
        .build_definition();
    definition.controller_id = S_HOME_CONTROLLER_ID.into();
    definition
}
//#endregion 🔖️HomeManifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::EditorApp;

    pub type HomeEditorApp = semio_framework_plugin::VcsArtifactApp<EditorApp<HomeApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn new_app() -> HomeEditorApp {
        semio_framework_plugin::testkit::new_app::<EditorApp<HomeApp>>()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️RetainedCommandEnvelope
    #[test]
    fn retained_command_fixture_matches_exact_routes_and_serde_json_boundaries() {
        use store::ArtifactStoreOneItemPreparationFactory as _;
        let fixture: pack::JsonValue = pack::parse_json(include_str!("🧪️fixtures/🎯️retained-command-limits.json")).expect("language-neutral retained fixture");
        let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
        assert_eq!(migrated, HOME_RETAINED_TOOL_IDS);
        assert_eq!(HOME_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
        assert_eq!(fixture["limits"]["configValueBytes"].as_u64(), Some(HOME_CONFIG_VALUE_BYTES as u64));
        assert_eq!(fixture["limits"]["storeStepBytes"].as_u64(), Some(HOME_CONFIG_STEP_BYTES as u64));
        let factory = HomeConfigPreparationFactory;
        for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
            let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
            let mutation = HomeConfigMutation::SetActivePanelTab { tab_id: value };
            let encoded = serde_json::to_vec(&mutation).expect("third-party JSON encode");
            let decoded: HomeConfigMutation = serde_json::from_slice(&encoded).expect("third-party JSON decode");
            assert_eq!(decoded, mutation);
            assert_eq!(factory.preflight(&decoded, None, store::HistoryLane::Document).is_ok(), case["accepted"].as_bool().expect("admission oracle"));
        }
    }

    #[test]
    fn retained_config_cancel_and_cleanup_respect_the_production_grant() {
        use std::io::Write as _;
        use store::ArtifactStoreOneItemPreparation as _;
        let value = "x".repeat(HOME_CONFIG_VALUE_BYTES);
        let mut preparation = HomeConfigPreparation {
            base: None, mutation: Some(HomeConfigMutation::SetActivePanelTab { tab_id: value }), description: None, authority: None, candidate: None, sealed_candidate: None, serialized_bytes: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 };
        preparation.cancel();
        assert!(matches!(preparation.advance(grant).expect("cancelled step"), store::ArtifactStoreOneItemPreparationStep::Blocked));
        preparation.begin_close();
        assert!(matches!(preparation.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).expect("undersized close"), store::SnapshotRetirementStep::Blocked));
        assert!(matches!(preparation.close_step(grant).expect("bounded close"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 }));
        assert!(matches!(preparation.close_step(grant).expect("terminal close"), store::SnapshotRetirementStep::Complete));
        assert!(preparation.terminal_is_empty());
        let mut counter = HomeConfigByteCounter { bytes: 0 };
        assert_eq!(counter.write(&[0; 4_096]).expect("maximum serialized envelope"), 4_096);
        assert!(counter.write(&[0]).is_err());
    }
    //#endregion 🧪️RetainedCommandEnvelope

    use semio_framework_os::{create_backbone_document, empty_space_snapshot, load_os_space_document, seed_os_space_catalog_if_empty, LocalStorageBackbonePort, OsSpaceDocument, SpaceKind, SpaceVisibility, S_SPACE_SCHEMA};
    use std::sync::Arc;

    async fn empty_history() -> semio_framework_plugin::HistoryView {
        semio_framework_plugin::HistoryView::empty()
    }

    #[semio_framework_async_macros::async_test]
    async fn home_manifest_derives_the_canonical_surface_id() {
        let definition = create_home_app();
        assert_eq!(definition.id, semio_framework::surface_app_id(&HomeApp::DIALECT.into(), semio_framework::AppRole::Editor));
        assert_eq!(definition.controller_id, "s-home");
    }

    #[semio_framework_async_macros::async_test]
    async fn home_declares_create_space_action() {
        let definition = create_home_app();
        let main = definition.window_kinds.iter().find(|window| window.id == crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW).expect("home main window");
        assert!(main.actions.iter().any(|action| action.id == "createStudio"));
    }

    #[semio_framework_async_macros::async_test]
    async fn space_document_persists_through_backbone_port() {
        // 🕳️ `parse_demo_space_document()` yields a `workflow::WorkflowSnapshot` (the demo fixture's own
        // artifact content), not a `space::SpaceSnapshot`-backed catalog entry
        // `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest persistence
        // path specifically, so it mints its own manifest instead.
        // 🧬️ O1 — the concrete `store::BackbonePorts` enum, not `dyn OsBackbonePort`: `seed_os_space_
        // catalog_if_empty`/`load_os_space_document` both take `Arc<dyn OsBackbonePort>` BY VALUE, so
        // `Arc<BackbonePorts>` unsizes at each call site with no trait-object variable needed here.
        let port: Arc<store::BackbonePorts> = Arc::new(store::BackbonePorts::LocalStorage(LocalStorageBackbonePort::default()));
        let projection = empty_space_snapshot("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
        let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
        let _ = seed_os_space_catalog_if_empty(demo, port.clone()).expect("seed");
        let loaded = load_os_space_document("persist-test", port.clone()).expect("load");
        assert_eq!(loaded.id, "persist-test");
        assert_eq!(loaded.name, "Persist Test");
    }

    /// 🧪️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the pre-ticket version of
    /// these two tests asserted on the VFS scene's ALWAYS-present `emptyMessage` field, which happened
    /// to make them incidentally immune to `crate::list_all_space_catalog_entries()`'s process-global
    /// catalog singleton being polluted by other tests in this same test binary. The new table render
    /// has no such structural field (`TableView` carries no message), so these are rewritten to fold a
    /// KNOWN directory event (deterministic, independent of the global catalog) and assert on the
    /// locale-correct COLUMN HEADERS instead — the real thing "labels resolve to the right locale" means
    /// for a table.
    async fn config_with_one_folded_space(locale: &str) -> HomeConfig {
        let event_json = pack::json!({
            "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
            "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Fixture", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
            "recordedAtMs": 1000
        })
        .to_string();
        let base = HomeConfig { locale: locale.into(), ..HomeConfig::default() };
        protocol::Mutation::diff(&crate::editor::home::config::HomeConfigMutation::FoldDirectoryEvent { event_json }, &base).diff().clone()
    }

    #[semio_framework_async_macros::async_test]
    async fn home_labels_resolve_native_english_by_default() {
        let history = empty_history();
        let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = ArtifactView::new(&home_doc, &history);
        let config = config_with_one_folded_space("en-US");
        let cfg = ConfigView { snapshot: &config };
        let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        let json = pack::to_json_string(&home_node);
        assert!(json.contains("Updated"), "English column header must resolve: {json}");
        assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn home_labels_resolve_native_german_locale() {
        let history = empty_history();
        let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = ArtifactView::new(&home_doc, &history);
        let config = config_with_one_folded_space("de");
        let cfg = ConfigView { snapshot: &config };
        let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        let json = pack::to_json_string(&home_node);
        assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
        assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
    }
}
//#endregion 🧪️Tests
