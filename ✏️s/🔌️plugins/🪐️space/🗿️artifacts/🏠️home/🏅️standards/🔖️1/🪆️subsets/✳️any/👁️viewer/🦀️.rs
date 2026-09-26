//! 👁️ S Home viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `HomeViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<HomeViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::{SHomeSnapshot, HOME_DIALECT, S_HOME_DOCUMENT_SCHEMA};
use crate::editor::home::config::{home_retained_contract, HomeConfig, HomeConfigMutation, HomeConfigPreparationFactory, HOME_DIRECTORY_PAGE_BYTES};
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::viewer::home::modes::view;
use crate::viewer::home::modes::view::windows::main;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, DslValue, Emit, Fault, HistoryView,
    InteractiveJobClassification, LocalizedLabel, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, PluginAssemblyError, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, UiAssemblyResult, ViewEmit, Viewer,
    ViewerApp,
};
use store::EngineHandles;

//#region 🔖️Command
/// 🏷️ Every verb the read-only Home is dispatched with: the host's sealed directory page, and nothing a human authors.
pub const HOME_VIEW_TOOL_IDS: &[&str] = &["applyDirectoryEventPage"];
const HOME_VIEW_RETAINED_PAYLOAD_SCHEMA: &str = "space.home.view-tool-command.v1";

/// 👁️ The read-only Home's typed command channel. Its one row is the host's sealed-page feed: the viewer folds the same
/// authenticated directory pages into its OWN config store that the editor folds into its own, so a Home opened as a
/// viewer lists the signed-in human's hub spaces instead of only the device's local rows (ticket 26/09/23 S16, audit
/// s13 §6 #16). A config write is not a document mutation — the artifact lane stays unreachable by construction.
///
/// 🔒️ Row order is the binary variant ordinal: appending is safe, reordering is a wire break.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum HomeViewCommand {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON the authenticated hub returned, sealed by its receipt.
    #[dsl(key = "apply-directory-event-page")]
    ApplyDirectoryEventPage { page_json: String },
}

impl Default for HomeViewCommand {
    /// 🌱️ An empty page, which the retained route refuses as invalid rather than applying: there is no "do nothing"
    /// verb to default to.
    fn default() -> Self {
        Self::ApplyDirectoryEventPage { page_json: String::new() }
    }
}

impl HomeViewCommand {
    /// 🪪️ The manifest action id this command stands for.
    pub fn action_id(&self) -> &'static str {
        match self {
            Self::ApplyDirectoryEventPage { .. } => "applyDirectoryEventPage",
        }
    }
}

impl protocol::OpBinary for HomeViewCommand {
    /// 🎯️ The exact manifest ids this typed command schema owns — equal to the `bounded_first_step_tool_proofs!` rows.
    const TOOL_JOB_IDS: &'static [&'static str] = HOME_VIEW_TOOL_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedCommands
/// 📏️ The page rides the retained wire budget the editor's page route admits — one bound for both surfaces.
fn home_view_retained_extent(command: &HomeViewCommand, _snapshot: &SHomeSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let HomeViewCommand::ApplyDirectoryEventPage { page_json } = command;
    (page_json.len() <= HOME_DIRECTORY_PAGE_BYTES).then_some(1)
}

/// 📬️ The viewer's one reducer: the shared page emission ([`HomeConfig::directory_event_page_emit`]) — a config
/// replacement plus the typed receipt the host acknowledges the page by, never a document mutation.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn home_view_retained_reduce(
    command: &HomeViewCommand,
    _snapshot: &SHomeSnapshot,
    config: &HomeConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<HomeViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
    let HomeViewCommand::ApplyDirectoryEventPage { page_json } = command;
    config.directory_event_page_emit(page_json)
}

pub struct HomeViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl HomeViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: HOME_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for HomeViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<HomeViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<HomeViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        HOME_VIEW_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        home_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > HOME_DIRECTORY_PAGE_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Space Home view command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for HomeViewCommandJobFactory {
    type Owner = ViewerApp<HomeViewer>;
    const TOOL_IDS: &'static [&'static str] = HOME_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = S_HOME_DOCUMENT_SCHEMA;
    /// 🔒️ CONFIG ONLY — naming `ArtifactToolPublicationLane::Artifact` here would be refused against a viewer's emit.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "applyDirectoryEventPage", lanes: &[ArtifactToolPublicationLane::Config] }];
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct HomeViewer;

impl ArtifactViewer for HomeViewer {
    type Snapshot = SHomeSnapshot;
    type Mutation = SHomeMutation;
    // 📇️ Shared with the editor (`crate::editor::home::config::HomeConfig`), not `NoConfig` — the
    // viewer renders the SAME hub-directory-fed table (`crate::home_space_rows`) and must therefore
    // read the SAME folded `directory_json`. `assert_viewer_never_mutates` only asserts the ARTIFACT/
    // draft store never advances (contract §2.5) — the config lane is fair game for both surfaces, and
    // a viewer emitting a `ConfigMutation` is not a document mutation.
    type Config = HomeConfig;
    type ConfigMutation = HomeConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = HomeViewCommand;

    const DIALECT: Dialect = HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = S_HOME_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SHomeSnapshot {
        SHomeSnapshot::default()
    }

    /// 🪪️ Same app-schema descriptor as the editor (contract requires both surfaces sharing a dialect
    /// to also share a config schema, since it is registered per-document-schema, not per-role).
    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(HomeConfigPreparationFactory))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(HomeViewCommandJobFactory::new(&controller))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<HomeViewer>,
        owner_file: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.space.home@1/*#viewer",
        artifact_schema: "s.home",
        factory: "HomeViewCommandJobFactory",
        factory_type: HomeViewCommandJobFactory,
        contract: home_retained_contract(),
        tools: ["applyDirectoryEventPage"]
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<ViewerApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !HOME_VIEW_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::from("space-home-view-command-tool-mismatch"));
        }
        if home_view_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("space-home-view-command-payload-too-large"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, home_view_retained_reduce, home_view_retained_extent));
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion },
            HomeViewCommand::action_id,
            HOME_DIRECTORY_PAGE_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    /// 🎯️ The host's sealed-page feed is the one action this surface is dispatched with; anything else is refused.
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        match action {
            "applyDirectoryEventPage" => Ok(HomeViewCommand::ApplyDirectoryEventPage {
                page_json: args.and_then(|value| value.get("pageJson")).and_then(DslValue::as_str).map(str::to_string).ok_or_else(|| Fault::from("s.home.directory-event-page-input-missing"))?,
            }),
            other => Err(Fault::from(format!("s.home.viewer.unknown-action:{other}"))),
        }
    }

    /// 👁️ Structurally read-only and inert: a sealed page is admitted ONLY on the retained route
    /// ([`home_view_retained_reduce`]), whose terminal output carries the receipt the host acknowledges it by. Applied
    /// here, the page would land without that receipt and the host's feed would re-offer it for ever.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    /// 👁️ Renders the SAME overview table the editor's main window does, read-only: no create/delete/
    /// rename/share affordances, fed by `cfg.snapshot.directory()` — never the artifact document itself.
    fn render(body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        let root = match body_key {
            main::S_HOME_VIEW_BODY => {
                let directory = cfg.snapshot.directory().map_err(|_| PluginAssemblyError::new("s.home.directory-projection-malformed", "Home directory projection is invalid"))?;
                main::render(&directory, view_state)?
            }
            _ => {
                semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("s.home.viewer.render.unknown-body", "unknown body key text admission failed"))?
            }
        };
        Ok(ComponentTree { root })
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_home_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(HOME_DIALECT)
        .document(["semio", "s", "home"])
        .icon_id("home")
        .mode_def(view::definition())
        .default_mode_id(view::S_HOME_VIEW_MODE)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .view_action("applyDirectoryEventPage", LocalizedLabel::native("Apply Directory Event Page", "Verzeichnis-Ereignisseite anwenden"))
        .action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::Migrated)
        .action_audience("applyDirectoryEventPage", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
