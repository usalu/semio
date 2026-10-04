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
use crate::editor::home::commands::{apply_directory_event_page, apply_local_catalog_document};
use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space, persist_locally, promote_to_hub_space};
use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, manage_space, presence_heartbeat, rename_space, share_space};
use crate::editor::home::commands::{delete_virtual_file_system_node, go_home, navigate_virtual_file_system_node};
use crate::editor::home::config::{home_retained_contract, HomeConfig, HomeConfigMutation, HomeConfigPreparationFactory};
use crate::editor::home::transient::{HomeTransient, HomeTransientMutation, HomeTransientRetirementFactory};
use crate::editor::home::presence::{HomePresence, HomePresenceMutation};
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app_commands;
use semio_framework_plugin::create_tab_stack_layout;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ComponentTree;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultOrigin;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionRef, DialogDefinition};
use semio_framework_2d::compute::EngineHandles;

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
        "applyLocalCatalogDocument" as "apply-local-catalog-document" => apply_local_catalog_document::ApplyLocalCatalogDocument,
        "createStudio" as "create-studio" => create_studio::CreateStudio,
        "promoteToHubSpace" as "promote-to-hub-space" => promote_to_hub_space::PromoteToHubSpace,
        "persistLocally" as "persist-locally" => persist_locally::PersistLocally,
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
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
    }
}
//#endregion 🔖️HomeCommand

//#region 🧵️RetainedCommands
const HOME_RETAINED_TOOL_IDS: &[&str] = &[
    "applyDirectoryEventPage", "applyLocalCatalogDocument", "createStudio", "bindSpaceFile", "importSpace", "openSpace", "navigateVirtualFileSystemNode", "deleteVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "renameSpace", "shareSpace", "manageSpace", "copyInviteLink", "promoteToHubSpace", "persistLocally", "presenceHeartbeat",
];
const HOME_RETAINED_PAYLOAD_SCHEMA: &str = "space.home.tool-command.v1";
const HOME_RETAINED_RAW_BYTES: usize = crate::editor::home::config::HOME_DIRECTORY_PAGE_BYTES;
const HOME_RETAINED_SCALAR_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;
const HOME_RETAINED_WORK_ITEMS: usize = 1;
const HOME_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    // 🛣️ The typed-operation terminal refuses any emit whose store lane is absent from this contract ("typed-operation
    // emitted a store lane absent from its exact factory publication contract"), so every writing route names its lane
    // and only the pure-`Effect` relays are `HostOnly`. `applyDirectoryEventPage` folds one page into the TRANSIENT
    // directory projection (derived hub state: no history, no persistence); the catalog routes bump the catalog
    // generation in the ARTIFACT store; the tombstone lands in the CONFIG store.
    ArtifactToolPublicationContract { tool_id: "applyDirectoryEventPage", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "applyLocalCatalogDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "createStudio", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "bindSpaceFile", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "importSpace", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "openSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "navigateVirtualFileSystemNode", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "deleteVirtualFileSystemNode", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "goHome", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "createSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "promoteToHubSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "persistLocally", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "renameSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "shareSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "manageSpace", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "copyInviteLink", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

/// 📏️ Each retained route's admitted extent AND the ceiling it is judged against. Two ceilings, not
/// one: a public invocation's scalars are capped at [`HOME_RETAINED_SCALAR_BYTES`], but one sealed
/// directory page is a `HostOnly` machine payload whose only real bound is the retained wire budget
/// [`HOME_RETAINED_RAW_BYTES`] — the hub pages it with `hasMore`, so a page is bounded by
/// construction and capping it at 4 KiB would refuse ordinary pages of a dozen spaces.
fn home_retained_extent(command: &HomeCommand, _snapshot: &SHomeSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let (admitted, ceiling) = match command {
        HomeCommand::OpenSpace(payload) => (payload.space_id.len(), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::NavigateVirtualFileSystemNode(payload) => (payload.node_id.len(), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::GoHome(_) | HomeCommand::PresenceHeartbeat(_) => (0, HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::CreateSpace(payload) => (payload.name.len().saturating_add(payload.kind.len()).saturating_add(payload.visibility.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::DeleteSpace(payload) => (payload.space_id.len(), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::RenameSpace(payload) => (payload.space_id.len().saturating_add(payload.name.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::ShareSpace(payload) => (payload.space_id.len().saturating_add(payload.email.len()).saturating_add(payload.role.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::ManageSpace(payload) => (payload.space_id.len(), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::CopyInviteLink(payload) => (payload.space_id.len().saturating_add(payload.role.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::PromoteToHubSpace(payload) => (payload.space_id.len().saturating_add(payload.name.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::PersistLocally(payload) => (payload.space_id.len().saturating_add(payload.folder_path.as_ref().map_or(0, String::len)), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::CreateStudio(payload) => (payload.name.len().saturating_add(payload.kind.len()).saturating_add(payload.folder_path.as_ref().map_or(0, String::len)), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::ApplyDirectoryEventPage(payload) => (payload.page_json.len(), HOME_RETAINED_RAW_BYTES),
        HomeCommand::ApplyLocalCatalogDocument(payload) => (payload.document_id.len().saturating_add(payload.pack.len()).saturating_add(payload.spr.len()), HOME_RETAINED_RAW_BYTES),
        HomeCommand::BindSpaceFile(payload) => (payload.space_id.len().saturating_add(payload.file_path.len()), HOME_RETAINED_SCALAR_BYTES),
        HomeCommand::ImportSpace(payload) => (payload.dsl.as_ref().map_or(0, String::len), HOME_RETAINED_RAW_BYTES),
        HomeCommand::DeleteVirtualFileSystemNode(payload) => (payload.node_id.len(), HOME_RETAINED_SCALAR_BYTES),
    };
    (admitted <= ceiling).then_some(HOME_RETAINED_WORK_ITEMS)
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
    let identity = require_session_identity(context.and_then(|context| context.view_state.as_ref()))?;
    if home_retained_extent(command, snapshot, _interaction).is_none() {
        return Err(Fault::from("space-home-retained-route-mismatch"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let row = |space_id: &str| context.and_then(|context| context.transient.directory().space(space_id));
    // 🪪️ `createStudio` needs the signed-in human (it names the studio's owner); rename/share/remove read the ONE folded
    // directory row they need from the projection this job captured at dispatch — a page folded meanwhile publishes a new
    // root and never mixes into it.
    match command {
        HomeCommand::CreateStudio(payload) => create_studio::handle_with_identity(payload, &doc, &cfg, identity),
        HomeCommand::RenameSpace(payload) => rename_space::handle_with_row(payload, &doc, &cfg, row(&payload.space_id)),
        HomeCommand::ShareSpace(payload) => share_space::handle_with_row(payload, &doc, &cfg, row(&payload.space_id)),
        HomeCommand::DeleteVirtualFileSystemNode(payload) => delete_virtual_file_system_node::handle_with_row(payload, &doc, &cfg, row(delete_virtual_file_system_node::local_studio_id(&payload.node_id))),
        _ => command.dispatch(&doc, &cfg),
    }
}

/// 📄️ The sealed page an editor command carries, for the shared page route.
fn home_directory_page_json(command: &HomeCommand) -> Option<&str> {
    match command {
        HomeCommand::ApplyDirectoryEventPage(payload) => Some(&payload.page_json),
        _ => None,
    }
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
//#region 💾️CatalogWork
/// 💾️ The IO-owning retained work of the four Home commands that write the local studio catalog (`importSpace`,
/// `bindSpaceFile`, `persistLocally`, and the host's `applyLocalCatalogDocument` re-hydration) — their IO never runs inside a `handle`. Stage `validate` reads only (identity, payload, target);
/// stage `commit` performs the one catalog write and answers the `change-catalog-generation` event. The retained job
/// checkpoints after `validate` (the checkpoint is the stage), so a cancellation between the stages writes nothing and
/// a job restored from its checkpoint resumes at `commit`, whose write refuses by name if its target changed meanwhile.
pub struct HomeCatalogWork {
    tool_id: &'static str,
    validated: bool,
    committed: bool,
}

impl HomeCatalogWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, validated: false, committed: false }
    }

    /// 🪜️ One stage for `command` against `doc`: `validate` (reads only) or `commit` (the one catalog write).
    fn advance(&mut self, command: &HomeCommand, doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<HomeApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.committed {
            return Err(Fault::from("space-home-catalog-work-repeated"));
        }
        let validated_step = ArtifactCommandWorkStep::Progress { stage: HOME_CATALOG_VALIDATED_STAGE, preview: HOME_CATALOG_VALIDATED_PREVIEW.as_bytes() };
        match (command, self.validated) {
            (HomeCommand::ImportSpace(import_space::ImportSpace { dsl: None }), _) => {
                self.committed = true;
                Ok(ArtifactCommandWorkStep::Complete(import_space::file_request()))
            }
            (HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some(dsl) }), false) => {
                import_space::validate(dsl)?;
                self.validated = true;
                Ok(validated_step)
            }
            (HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some(dsl) }), true) => {
                self.committed = true;
                import_space::commit(dsl, doc).map(ArtifactCommandWorkStep::Complete)
            }
            (HomeCommand::ApplyLocalCatalogDocument(payload), false) => {
                apply_local_catalog_document::validate(payload)?;
                self.validated = true;
                Ok(validated_step)
            }
            (HomeCommand::ApplyLocalCatalogDocument(payload), true) => {
                self.committed = true;
                apply_local_catalog_document::commit(payload, doc).map(ArtifactCommandWorkStep::Complete)
            }
            (HomeCommand::BindSpaceFile(payload), false) => {
                bind_space_file::validate(payload)?;
                self.validated = true;
                Ok(validated_step)
            }
            (HomeCommand::BindSpaceFile(payload), true) => {
                self.committed = true;
                bind_space_file::commit(payload, doc).map(ArtifactCommandWorkStep::Complete)
            }
            (HomeCommand::PersistLocally(payload), validated) => match (payload.folder_path.as_deref().map(str::trim), validated) {
                (None | Some(""), _) => {
                    self.committed = true;
                    Ok(ArtifactCommandWorkStep::Complete(persist_locally::folder_dialog(&payload.space_id)))
                }
                (Some(folder_path), false) => {
                    persist_locally::validate(&payload.space_id, folder_path)?;
                    self.validated = true;
                    Ok(validated_step)
                }
                (Some(folder_path), true) => {
                    self.committed = true;
                    persist_locally::commit(&payload.space_id, folder_path, doc).map(ArtifactCommandWorkStep::Complete)
                }
            },
            _ => Err(Fault::from("space-home-catalog-work-command-mismatch")),
        }
    }
}

const HOME_CATALOG_VALIDATED_STAGE: &str = "space-home.catalog.validated";
const HOME_CATALOG_VALIDATED_PREVIEW: &str = "{\"en\":\"Studio checked, writing the catalog\",\"de\":\"Studio geprüft, Katalog wird geschrieben\"}";

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<HomeApp>> for HomeCatalogWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &HomeCommand, snapshot: &SHomeSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<HomeApp>>>) -> Option<usize> {
        home_retained_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<HomeApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<HomeApp>>, Fault> {
        require_session_identity(input.context.and_then(|context| context.view_state.as_ref()))?;
        self.advance(input.command, &ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone()))
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        let slot = target.first_mut().ok_or_else(|| Fault::from("space-home-catalog-work-checkpoint-capacity"))?;
        *slot = u8::from(self.validated);
        Ok(1)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.validated = match checkpoint {
            [] | [0] => false,
            [1] => true,
            _ => return Err(Fault::from("space-home-catalog-work-checkpoint-invalid")),
        };
        Ok(())
    }
}
//#endregion 💾️CatalogWork
//#endregion 🧵️RetainedCommands

//#region 🔖️HomeApp
/// 🧪️ Unit struct — the Home launcher holds catalog bootstrap ports plus per-session studio port
/// bindings for folder/file-backed studios.
#[derive(Default, Clone, Copy)]
pub struct HomeApp;

impl ArtifactEditor for HomeApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
    type Config = HomeConfig;
    type ConfigMutation = HomeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = HomePresence;
    type PresenceMutation = HomePresenceMutation;
    type Transient = HomeTransient;
    type TransientMutation = HomeTransientMutation;
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
        artifact_schema: "s.home",
        factory: "HomeRetainedCommandJobFactory",
        factory_type: HomeRetainedCommandJobFactory,
        contract: home_retained_contract(),
        tools: ["applyDirectoryEventPage", "applyLocalCatalogDocument", "createStudio", "bindSpaceFile", "importSpace", "openSpace", "navigateVirtualFileSystemNode", "deleteVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "renameSpace", "shareSpace", "manageSpace", "copyInviteLink", "promoteToHubSpace", "persistLocally", "presenceHeartbeat"]
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// ♻️ Home OWNS a config store — the shared `HomeConfigPreparationFactory` (config module), and every directory
    /// projection lands in it — so closing an instance needs that lane's owners AND its disposer, one
    /// declaration in two halves. Unlike the viewer wrapper, `EditorApp` installs no default
    /// (`🔌️plugin/🦀️.rs:32854` forwards `E`'s answer unchanged), so an editor that omits either half
    /// answers `interactive-job.close-owned-disposer-missing` and then `artifact store has no
    /// owner-supplied bounded disposer` on every close. Found by ticket 26/09/18 S4 the moment a
    /// `createStudio` dispatch got far enough to reach the close ladder.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    /// 🫧️ The transient lane carries the folded hub directory: one bounded page item per publication
    /// ([`semio_framework_plugin::bounded_transient_preparation_factory`] prices the encoded page, never the root), and
    /// displaced or disposed roots retire in grants measured by the projection itself, never by encoding it.
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::transient_store_disposer::<Self::Transient, Self::TransientMutation>(std::sync::Arc::new(HomeTransientRetirementFactory)))
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(std::sync::Arc::new(HomeTransientRetirementFactory))
    }

    /// 👤️ Home reads its OWN presence root on the `createStudio` path (the studio it mints names the
    /// signed-in human as its owner), and `PresenceStore::local_read` fails closed with
    /// `presence local read requires a live exact local retirement owner` unless this factory is
    /// installed. Home declared none, so `createStudio` was refused the moment the store fold
    /// contract above stopped refusing it first — the same class S4 cured for Home's config, draft
    /// and transient disposers, on the one lane it missed (ticket 26/09/18 S10 §1.5).
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::home::presence::HomePresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::home::presence::HomePresenceRetirementFactory))
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
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "space home command does not match its exact registered tool"));
        }
        if home_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.too-large"), "space home command payload exceeds its bounded retained extent"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = match request.command.as_ref() {
            HomeCommand::ApplyDirectoryEventPage(_) => Box::new(apply_directory_event_page::HomeDirectoryPageWork::<EditorApp<Self>>::new(tool_id, home_directory_page_json)),
            HomeCommand::ImportSpace(_) | HomeCommand::BindSpaceFile(_) | HomeCommand::PersistLocally(_) | HomeCommand::ApplyLocalCatalogDocument(_) => Box::new(HomeCatalogWork::new(tool_id)),
            _ => Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, home_retained_reduce, home_retained_extent)),
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
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
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str).map(str::to_string);
        match action {
            "applyDirectoryEventPage" => Ok(HomeCommand::ApplyDirectoryEventPage(apply_directory_event_page::ApplyDirectoryEventPage {
                page_json: str_field("pageJson").ok_or_else(|| Fault::from("s.home.directory-event-page-input-missing"))?,
            })),
            "applyLocalCatalogDocument" => Ok(HomeCommand::ApplyLocalCatalogDocument(apply_local_catalog_document::ApplyLocalCatalogDocument {
                document_id: str_field("documentId").unwrap_or_default(),
                pack: str_field("pack").unwrap_or_default(),
                spr: str_field("spr").unwrap_or_default(),
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
            "importSpace" => {
                let chunk_count = args.and_then(|value| value.get(semio_framework::kernel::IMPORT_ARGUMENT_CHUNK_COUNT)).and_then(DslValue::as_f64).unwrap_or(1.0);
                if chunk_count > 1.0 {
                    return Err(Fault::new(FaultOrigin::App, "s.home.import-space.oversized", format!("a studio manifest must fit one import chunk of {} bytes", semio_framework::kernel::IMPORT_CHUNK_BYTES)));
                }
                Ok(HomeCommand::ImportSpace(import_space::ImportSpace { dsl: str_field("dsl").or_else(|| str_field(semio_framework::kernel::IMPORT_ARGUMENT_PAYLOAD)) }))
            }
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
            "promoteToHubSpace" => Ok(HomeCommand::PromoteToHubSpace(promote_to_hub_space::PromoteToHubSpace {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                name: str_field("name").unwrap_or_default(),
            })),
            "persistLocally" => Ok(HomeCommand::PersistLocally(persist_locally::PersistLocally {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                folder_path: str_field("folderPath").or_else(|| str_field("folder_path")),
            })),
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

    /// 🪪️ RENDERING is never gated on the signed-in human; only MUTATING is (see `handle` above, which
    /// keeps `require_session_identity` — a signed-out human genuinely cannot create a studio). This
    /// gate made the product's landing window refuse to publish for every signed-out visitor, which is
    /// the ordinary first paint of a hub-configured shell; the window's own render now answers an empty
    /// space table instead (ticket 26/09/18, S2 §3.4).
    fn render(body_key: &str, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, cfg.snapshot, HomeTransient::default().directory(), view_state)
    }

    /// 🫧️ The host-facing render: the framework hands the app-local transient in here, so the table lists the folded hub
    /// directory; the bare `render` above has no transient and lists the local rows only.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        _doc: &ArtifactView<'_, SHomeSnapshot>,
        cfg: &ConfigView<'_, HomeConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, HomeTransient>,
        _interaction: &InteractionView<'_>,
    ) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, cfg.snapshot, transient.snapshot.directory(), view_state)
    }
}
//#endregion 🔖️HomeApp

/// 🖼️ One Home body from the config and the folded directory projection.
pub fn render_body(body_key: &str, config: &HomeConfig, directory: &crate::editor::home::transient::HomeDirectoryProjection, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
    let root = match body_key {
        crate::editor::home::modes::explore::windows::main::S_HOME_BODY => crate::editor::home::modes::explore::windows::main::render(config, directory, view_state)?,
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("s.home.render.unknown-body", "unknown body key text admission failed"))?,
    };
    Ok(ComponentTree { root })
}

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
        .mutation("deleteVirtualFileSystemNode", LocalizedLabel::native("Remove Studio from Home", "Studio aus Home entfernen"))
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
        .shell_action("promoteToHubSpace", LocalizedLabel::native("Promote to hub", "Zum Hub hochstufen"))
        .shell_action("persistLocally", LocalizedLabel::native("Persist locally", "Lokal speichern"))
        .dialog(
            DialogDefinition::new("persistLocally", LocalizedLabel::native("Persist locally", "Lokal speichern"), ActionRef::new("persistLocally"))
                .body(LocalizedLabel::native("Choose a folder to keep this ephemeral studio on disk.", "Wählen Sie einen Ordner, um dieses flüchtige Studio dauerhaft lokal zu speichern."))
                .args(vec![ActionArgDef::text("folderPath", LocalizedLabel::native("Folder path", "Ordnerpfad")).required()])
                .submit_label(LocalizedLabel::native("Persist", "Speichern")),
        )
        .dialog(
            DialogDefinition::new("ephemeralShareBlocked", LocalizedLabel::native("Sharing unavailable", "Teilen nicht verfügbar"), ActionRef::new("promoteToHubSpace"))
                .body(LocalizedLabel::native(
                    "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.",
                    "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.",
                ))
                .submit_label(LocalizedLabel::native("Promote to hub", "Zum Hub hochstufen")),
        )

        .view_action("applyDirectoryEventPage", LocalizedLabel::native("Apply Directory Event Page", "Verzeichnis-Ereignisseite anwenden"))
        .view_action("applyLocalCatalogDocument", LocalizedLabel::native("Show Studio Kept on This Device", "Auf diesem Gerät gespeichertes Studio anzeigen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        .action_interactive_job("createStudio", InteractiveJobClassification::Migrated)
        .action_interactive_job("bindSpaceFile", InteractiveJobClassification::Migrated)
        .action_interactive_job("importSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("openSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("navigateVirtualFileSystemNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteVirtualFileSystemNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("goHome", InteractiveJobClassification::Migrated)
        .action_interactive_job("createSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteSpace", InteractiveJobClassification::Migrated)
        .action_destructive("deleteSpace")
        .action_interactive_job("renameSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("shareSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("manageSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("promoteToHubSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("persistLocally", InteractiveJobClassification::Migrated)
        .action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::Migrated)
        .action_interactive_job("applyLocalCatalogDocument", InteractiveJobClassification::Migrated)
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
            "promoteToHubSpace".into(),
            "persistLocally".into(),
        ])
        .keybinding("mod+n", "createStudio")
        .keybinding("mod+o", "importSpace")
        .action_describe("createStudio", LocalizedLabel::native("Creates a new studio with the given name and kind, either a temporary local-only one or one kept in a folder when a folder path is given; sharing stays off until it is promoted.", "Erstellt ein neues Studio mit dem angegebenen Namen und der Art, entweder temporär und nur lokal oder mit Ordnerpfad in einem Ordner gespeichert; Teilen bleibt gesperrt, bis es hochgestuft wird."))
        .action_describe("bindSpaceFile", LocalizedLabel::native("Binds a local studio to a file on disk by path, so the studio's events are persisted to and read from that file; a host without a filesystem refuses it.", "Verknüpft ein lokales Studio anhand eines Pfads mit einer Datei auf dem Datenträger, sodass seine Ereignisse in diese Datei geschrieben und daraus gelesen werden; ein Host ohne Dateisystem lehnt dies ab."))
        .action_describe("importSpace", LocalizedLabel::native("Imports a studio from the given .os DSL text, or opens the host's file picker for an .os file when no text is given.", "Importiert ein Studio aus dem angegebenen .os-DSL-Text oder öffnet ohne Text die Dateiauswahl des Hosts für eine .os-Datei."))
        .action_describe("openSpace", LocalizedLabel::native("Opens the space or studio with the given id, navigating the shell to it.", "Öffnet den Space oder das Studio mit der angegebenen Id und navigiert die Shell dorthin."))
        .action_describe("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigates the shell to the space behind one node of the Home file tree.", "Navigiert die Shell zum Space hinter einem Knoten des Home-Dateibaums."))
        .action_describe("deleteVirtualFileSystemNode", LocalizedLabel::native("Removes the local studio behind one node of the Home file tree from Home; the studio and its history stay intact, and undoing the removal lists it again.", "Entfernt das lokale Studio hinter einem Knoten des Home-Dateibaums aus Home; das Studio und sein Verlauf bleiben erhalten, und das Rückgängigmachen listet es wieder."))
        .action_describe("goHome", LocalizedLabel::native("Navigates the shell back to the Home launcher.", "Navigiert die Shell zurück zum Home-Starter."))
        .action_describe("createSpace", LocalizedLabel::native("Creates a new shared space on the hub with the given name; without a name it opens the Create Space dialog.", "Erstellt auf dem Hub einen neuen geteilten Space mit dem angegebenen Namen; ohne Namen öffnet es den Dialog Space erstellen."))
        .action_describe("deleteSpace", LocalizedLabel::native("Deletes one space on the hub for every member; the first call opens a confirmation dialog, and only the confirmed call deletes it.", "Löscht einen Space auf dem Hub für alle Mitglieder; der erste Aufruf öffnet einen Bestätigungsdialog, erst der bestätigte Aufruf löscht."))
        .action_describe("renameSpace", LocalizedLabel::native("Renames one space on the hub; without a new name it opens the Rename Space dialog seeded with the current name.", "Benennt einen Space auf dem Hub um; ohne neuen Namen öffnet es den Dialog Space umbenennen mit dem aktuellen Namen."))
        .action_describe("shareSpace", LocalizedLabel::native("Adds or updates a member of one space on the hub by email and role; without an email it opens the Share Space dialog.", "Fügt auf dem Hub einem Space ein Mitglied per E-Mail und Rolle hinzu oder aktualisiert es; ohne E-Mail öffnet es den Dialog Space teilen."))
        .action_describe("manageSpace", LocalizedLabel::native("Opens the shell's administration pane for one space, where the hub decides what the user may manage.", "Öffnet den Verwaltungsbereich der Shell für einen Space, in dem der Hub entscheidet, was der Nutzer verwalten darf."))
        .action_describe("copyInviteLink", LocalizedLabel::native("Asks the hub to mint an invite for one space and copies the redeemable link to the clipboard.", "Lässt den Hub eine Einladung für einen Space erzeugen und kopiert den einlösbaren Link in die Zwischenablage."))
        .action_describe("promoteToHubSpace", LocalizedLabel::native("Promotes a temporary local studio to a shared space on the hub under the given name, so it can be shared and edited together.", "Stuft ein temporäres lokales Studio unter dem angegebenen Namen zu einem geteilten Space auf dem Hub hoch, damit es geteilt und gemeinsam bearbeitet werden kann."))
        .action_describe("persistLocally", LocalizedLabel::native("Saves a temporary local studio into a folder on this machine so it survives restarts; it stays local-only and unshared.", "Speichert ein temporäres lokales Studio in einen Ordner auf diesem Rechner, damit es Neustarts übersteht; es bleibt lokal und ungeteilt."))
        .action_audience("applyDirectoryEventPage", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_audience("applyLocalCatalogDocument", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_audience("presenceHeartbeat", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️HomeManifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
