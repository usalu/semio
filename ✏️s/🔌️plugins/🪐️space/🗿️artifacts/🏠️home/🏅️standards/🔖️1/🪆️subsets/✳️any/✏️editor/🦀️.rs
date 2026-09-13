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

use crate::SHomeSnapshot;
use crate::editor::home::commands::apply_directory_event_page;
use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space};
use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, fold_directory_events, manage_space, presence_heartbeat, rename_space, share_space};
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
pub const S_HOME_CONTROLLER_ID: &str = "s.space.home@1/*#editor";
//#endregion 🔖️Constants

fn require_session_identity(view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<&semio_framework_plugin::ViewSessionIdentity, Fault> {
    view_state.and_then(crate::home_session_identity).ok_or_else(|| Fault::from("s.home.session-identity-required"))
}

//#region 🔖️HomeCommand
app_commands! {
    /// 🎯️ `HomeApp::Command` — the SOLE dispatch surface for the Home launcher's own behavior, one
    /// variant per action declared in `create_home_app`'s manifest.
    pub enum HomeCommand for SHomeSnapshot, crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation, HomeConfig, HomeConfigMutation {
        "applyDirectoryEventPage" as "apply-directory-event-page" => apply_directory_event_page::ApplyDirectoryEventPage,
        "createStudio" as "create-studio" => create_studio::CreateStudio,
        "bindSpaceFile" as "bind-space-file" => bind_space_file::BindSpaceFile,
        "importSpace" as "import-space" => import_space::ImportSpace,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "deleteVirtualFileSystemNode" as "delete-vfs-node" => delete_virtual_file_system_node::DeleteVirtualFileSystemNode,
        "goHome" as "go-home" => go_home::GoHome,
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: Home = a real table of
        // every space, fed by the event-sourced hub directory read model (contract §C1/§C6).
        "createSpace" as "create-space" => create_space::CreateSpace,
        "deleteSpace" as "delete-space" => delete_space::DeleteSpace,
        "renameSpace" as "rename-space" => rename_space::RenameSpace,
        "shareSpace" as "share-space" => share_space::ShareSpace,
        "manageSpace" as "manage-space" => manage_space::ManageSpace,
        "copyInviteLink" as "copy-invite-link" => copy_invite_link::CopyInviteLink,
        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
    }
}
//#endregion 🔖️HomeCommand

//#region 🧵️RetainedCommands
const HOME_RETAINED_TOOL_IDS: &[&str] = &[
    "openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "presenceHeartbeat",
];
const HOME_RETAINED_PAYLOAD_SCHEMA: &str = "space.home.tool-command.v1";
const HOME_RETAINED_RAW_BYTES: usize = 128 * 1024;
const HOME_RETAINED_SCALAR_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;
const HOME_RETAINED_WORK_ITEMS: usize = 1;
const HOME_CONFIG_BASE_BYTES: usize = 4 * 1024 * 1024;
const HOME_CONFIG_STEP_BYTES: usize = 16 * 1024 * 1024;
const HOME_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "openSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "navigateVirtualFileSystemNode", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "goHome", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "createSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "deleteSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "shareSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "manageSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "copyInviteLink", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn home_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(HOME_RETAINED_RAW_BYTES, 256, 1, HOME_CONFIG_STEP_BYTES, 7_500, 1, 1)
}

fn home_retained_extent(command: &HomeCommand, _snapshot: &SHomeSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let admitted = match command {
        HomeCommand::OpenSpace(payload) => payload.space_id.len(),
        HomeCommand::NavigateVirtualFileSystemNode(payload) => payload.node_id.len(),
        HomeCommand::GoHome(_) | HomeCommand::PresenceHeartbeat(_) => 0,
        HomeCommand::CreateSpace(payload) => payload.name.len().saturating_add(payload.kind.len()).saturating_add(payload.visibility.len()),
        HomeCommand::DeleteSpace(payload) => payload.space_id.len(),
        HomeCommand::ShareSpace(payload) => payload.space_id.len().saturating_add(payload.email.len()).saturating_add(payload.role.len()),
        HomeCommand::ManageSpace(payload) => payload.space_id.len(),
        HomeCommand::CopyInviteLink(payload) => payload.space_id.len().saturating_add(payload.role.len()),
        HomeCommand::ApplyDirectoryEventPage(_)
        | HomeCommand::CreateStudio(_)
        | HomeCommand::BindSpaceFile(_)
        | HomeCommand::ImportSpace(_)
        | HomeCommand::DeleteVirtualFileSystemNode(_)
        | HomeCommand::RenameSpace(_)
        | HomeCommand::FoldDirectoryEvents(_) => return None,
    };
    (admitted <= HOME_RETAINED_SCALAR_BYTES).then_some(HOME_RETAINED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "ArtifactCommandReducer requires the eight operation, document, configuration, history, and interaction inputs.")]
fn home_retained_reduce(
    command: &HomeCommand,
    snapshot: &SHomeSnapshot,
    config: &HomeConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<HomeApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
    require_session_identity(context.and_then(|context| context.view_state.as_ref()))?;
    if home_retained_extent(command, snapshot, _interaction).is_none() {
        return Err(Fault::from("space-home-retained-route-mismatch"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

pub struct HomeRetainedCommandJobFactory { keys: Vec<ToolFactoryKey> }

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
    const DOCUMENT_SCHEMA: &'static str = crate::S_HOME_DOCUMENT_SCHEMA;
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
    config
        .directory_json
        .len()
        .saturating_add(config.directory_session_binding_sha256.len())
        .saturating_add(config.directory_receipt_sha256.len())
        .saturating_add(size_of_val(&config.directory_authorization_generation))
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

#[cfg(test)]
struct HomeConfigByteCounter { bytes: usize }

#[cfg(test)]
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
        let (mutation_bytes, maximum_bytes) = match mutation {
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 }
                if *authorization_generation > 0
                    && directory_json.len() <= HOME_CONFIG_BASE_BYTES
                    && crate::editor::home::config::directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) =>
            {
                (directory_json.len().saturating_add(session_binding_sha256.len()).saturating_add(receipt_sha256.len()).saturating_add(8), HOME_CONFIG_BASE_BYTES + 136)
            }
            HomeConfigMutation::FoldDirectoryEvent { event_json } => (event_json.len(), HOME_CONFIG_BASE_BYTES),
            _ => return Err("Space Home config preparation rejects non-retained mutations".into()),
        };
        if lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Space Home config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 3, retained_bytes: HOME_CONFIG_STEP_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<HomeConfig, HomeConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<HomeConfig, HomeConfigMutation>> {
        let (mutation_bytes, maximum_bytes) = match &request.mutation {
            HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 }
                if *authorization_generation > 0
                    && directory_json.len() <= HOME_CONFIG_BASE_BYTES
                    && crate::editor::home::config::directory_projection_state_is_valid(directory_json, session_binding_sha256, *authorization_generation, receipt_sha256) =>
            {
                (directory_json.len().saturating_add(session_binding_sha256.len()).saturating_add(receipt_sha256.len()).saturating_add(8), HOME_CONFIG_BASE_BYTES + 136)
            }
            HomeConfigMutation::FoldDirectoryEvent { event_json } => (event_json.len(), HOME_CONFIG_BASE_BYTES),
            _ => return Err(request),
        };
        if request.lane != store::HistoryLane::Document || mutation_bytes > maximum_bytes || request.description.as_ref().is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
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
                HomeConfigMutation::ReplaceDirectoryProjection { directory_json, session_binding_sha256, authorization_generation, receipt_sha256 } => HomeConfigMutation::ReplaceDirectoryProjection {
                    directory_json: std::mem::replace(&mut post.directory_json, directory_json.clone()),
                    session_binding_sha256: std::mem::replace(&mut post.directory_session_binding_sha256, session_binding_sha256.clone()),
                    authorization_generation: std::mem::replace(&mut post.directory_authorization_generation, *authorization_generation),
                    receipt_sha256: std::mem::replace(&mut post.directory_receipt_sha256, receipt_sha256.clone()),
                },
                // ⚙️ The fold is not a field replacement — its post state is the mutation's own diff,
                // and its declared inverse is the exact pre-fold snapshot.
                HomeConfigMutation::FoldDirectoryEvent { .. } => {
                    let diff = ::protocol::Mutation::diff(&mutation, base).into_parts().0;
                    post = ::protocol::MutationDiff::apply(&diff, base).map_err(|_| "Space Home config fold could not apply its own diff".to_string())?;
                    HomeConfigMutation::Snapshot { config: base.clone() }
                }
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
                return Err("Space Home config publication exceeds its complete retained envelope".into());
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
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
    type Config = HomeConfig;
    type ConfigMutation = HomeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = HomePresence;
    type PresenceMutation = HomePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = HomeCommand;

    const DIALECT: Dialect = crate::HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::S_HOME_DOCUMENT_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(HomeConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<HomeApp>,
        owner_file: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.space.home@1/*#editor",
        document_schema: "s.home",
        factory: "HomeRetainedCommandJobFactory",
        factory_type: HomeRetainedCommandJobFactory,
        contract: home_retained_contract(),
        tools: ["openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "presenceHeartbeat"]
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::space_retained_store_preparation::<Self::Snapshot, Self::Mutation>("space-home-artifact-retained", HOME_RETAINED_RAW_BYTES)
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
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
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
    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        match action {
            "applyDirectoryEventPage" => Ok(HomeCommand::ApplyDirectoryEventPage(apply_directory_event_page::ApplyDirectoryEventPage {
                page_json: str_field("pageJson").ok_or_else(|| Fault::from("s.home.directory-event-page-input-missing"))?,
            })),
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
            "manageSpace" => Ok(HomeCommand::ManageSpace(manage_space::ManageSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "shareSpace" => {
                Ok(HomeCommand::ShareSpace(share_space::ShareSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(), email: str_field("email").unwrap_or_default(), role: str_field("role").unwrap_or_default() }))
            }
            "copyInviteLink" => Ok(HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
                ttl_secs: args.and_then(|value| value.get("ttlSecs")).and_then(DslValue::as_f64).map_or(0, |n| n as u64),
            })),
            "foldDirectoryEvents" => {
                Ok(HomeCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents { events_json: args.and_then(|value| value.get("eventsJson")).and_then(DslValue::as_str).map_or_else(|| "[]".into(), str::to_string) }))
            }
            "presenceHeartbeat" => Ok(HomeCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {})),
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
        _interaction: &InteractionView<'_>, view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation, HomeConfigMutation, Self::DraftMutation>, Fault> {
        let identity = require_session_identity(view_state)?;
        match command {
            HomeCommand::CreateStudio(payload) => create_studio::handle_with_identity(payload, doc, cfg, identity),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn render(body_key: &str, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        crate::home_session_identity(view_state).ok_or_else(|| PluginAssemblyError::new("s.home.session-identity-required", "current host session identity is required"))?;
        let root = match body_key {
            crate::editor::home::modes::explore::windows::main::S_HOME_BODY => crate::editor::home::modes::explore::windows::main::render(cfg.snapshot, view_state)?,
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
    
    Editor::builder(crate::HOME_DIALECT)
        .document(["semio", "s", "home"])
        .icon_id("home")
        .mode_def(crate::editor::home::modes::explore::definition())
        .default_mode_id("explore")
        .window_kind_def(crate::editor::home::modes::explore::windows::main::definition())
        .default_layout(create_tab_stack_layout(&[crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW.into()], Some(&["Studios".into()])))
        .mutation("createStudio", LocalizedLabel::native("Create Studio", "Studio erstellen"))
        .shell_action("bindSpaceFile", LocalizedLabel::native("Bind Studio File", "Studio-Datei verknüpfen"))
        .mutation("importSpace", LocalizedLabel::native("Import Studio", "Studio importieren"))
        .action_with(semio_framework_plugin::ActionDefinition::new("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"), semio_framework_plugin::ActionKind::Shell, "folder-open"))
        .action_with(semio_framework_plugin::ActionDefinition::new("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"), semio_framework_plugin::ActionKind::Shell, "folder"))
        .mutation("deleteVirtualFileSystemNode", LocalizedLabel::native("Delete File System Node", "Dateisystemknoten löschen"))
        .action_with(semio_framework_plugin::ActionDefinition::new("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"), semio_framework_plugin::ActionKind::Shell, "home"))
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
                    .default_value(&"atelier"),
                    ActionArgDef::select(
                        "visibility",
                        LocalizedLabel::native("Visibility", "Sichtbarkeit"),
                        vec![ActionArgOption::new("private", LocalizedLabel::native("Private", "Privat")), ActionArgOption::new("public", LocalizedLabel::native("Public", "Öffentlich"))],
                    )
                    .default_value(&"private"),
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
                    .default_value(&"spectator"),
                ])
                .submit_label(LocalizedLabel::native("Share", "Teilen")),
        )
        .shell_action("manageSpace", LocalizedLabel::native("Manage Space", "Space verwalten"))
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))
        .view_action("applyDirectoryEventPage", LocalizedLabel::native("Apply Directory Event Page", "Verzeichnis-Ereignisseite anwenden"))
        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse einspielen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        .action_interactive_job("createStudio", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("bindSpaceFile", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("importSpace", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("openSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("navigateVirtualFileSystemNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteVirtualFileSystemNode", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("goHome", InteractiveJobClassification::Migrated)
        .action_interactive_job("createSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("renameSpace", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("shareSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("manageSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("foldDirectoryEvents", InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("presenceHeartbeat", InteractiveJobClassification::Migrated)
        .window_kind_action_refs(crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW, vec![
            "createStudio".into(),
            "bindSpaceFile".into(),
            "importSpace".into(),
            "openSpace".into(),
            "navigateVirtualFileSystemNode".into(),
            "deleteVirtualFileSystemNode".into(),
            "goHome".into(),
            "createSpace".into(),
            "deleteSpace".into(),
            "renameSpace".into(),
            "shareSpace".into(),
            "manageSpace".into(),
            "copyInviteLink".into(),
        ])
        .keybinding("mod+n", "createStudio")
        .keybinding("mod+o", "importSpace")
        .build_definition()
}
//#endregion 🔖️HomeManifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
