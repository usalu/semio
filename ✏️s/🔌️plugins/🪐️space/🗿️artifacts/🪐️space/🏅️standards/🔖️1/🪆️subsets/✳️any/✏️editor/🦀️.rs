//! ✏️ S Space index editor — the `ArtifactEditor` impl (dispatch-only) for the space's artifact index.
//! Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4. Lane 2-B: the real table
//! (name · kind · subset · updated · updated-by · presence), create/open/delete/rename commands, the
//! members panel, and the folded-directory/presence `Config` state that feeds them both.

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::SPACE_INDEX_DIALECT;
use crate::editor::space_index::commands::{
    copy_invite_link, create_artifact, delete_artifact, fold_directory_events, invite_member, open_artifact, open_artifact_with, presence_heartbeat, remove_member, rename_artifact, request_delete_artifact, request_invite_member, set_visibility,
    touch_artifact,
};
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use crate::editor::space_index::modes::edit;
use crate::editor::space_index::modes::edit::windows::main;
use crate::editor::space_index::panels::members as members_panel;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    built_to_component_tree, ActionArgDef, ActionArgOption, ActionFactory, ActionRef, ArtifactEditor, ArtifactView, ComponentTree, ConfigView, DialogDefinition, DraftView, Editor, Emit, Fault, FaultCode, FaultOrigin, LocalizedLabel, NoDraft,
    NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiAssemblyResult,
};
use store::EngineHandles;

//#region 🔖️Actions
/// 🎯️ Every panel/dialog-adjacent action this app declares addresses itself through this factory —
/// mirrors `draw_play_action`'s precedent (`🖍️draw`'s editor root).
pub const SPACE_INDEX_CONTROLLER_ID: &str = "s.space.space@1/*#editor";

pub fn space_index_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    ActionFactory::new(SPACE_INDEX_CONTROLLER_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
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
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Actions

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SpaceIndexEditor::Command` — the SOLE dispatch surface for the index editor's own behavior:
    /// the four frozen document mutations, plus the opening/deletion-confirm/directory-relay/presence
    /// commands lane 2-B adds on top (worker-brief tasks 2–3). Row order is the binary variant
    /// ordinal — appending is safe, reordering is a wire-format break.
    pub enum SpaceIndexCommand for SSpaceSnapshot, SSpaceMutation, SpaceIndexConfig, SpaceIndexConfigMutation {
        "createArtifact" as "create-artifact" => create_artifact::CreateArtifact,
        "deleteArtifact" as "delete-artifact" => delete_artifact::DeleteArtifact,
        "renameArtifact" as "rename-artifact" => rename_artifact::RenameArtifact,
        "touchArtifact" as "touch-artifact" => touch_artifact::TouchArtifact,
        "requestDeleteArtifact" as "request-delete-artifact" => request_delete_artifact::RequestDeleteArtifact,
        "openArtifact" as "open-artifact" => open_artifact::OpenArtifact,
        "openArtifactWith" as "open-artifact-with" => open_artifact_with::OpenArtifactWith,
        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "inviteMember" as "invite-member" => invite_member::InviteMember,
        "removeMember" as "remove-member" => remove_member::RemoveMember,
        "setVisibility" as "set-visibility" => set_visibility::SetVisibility,
        "copyInviteLink" as "copy-invite-link" => copy_invite_link::CopyInviteLink,
        "requestInviteMember" as "request-invite-member" => request_invite_member::RequestInviteMember,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every id this app declares is `Migrated`, so every id needs exactly one proof row joined to a
/// live owned factory — `validate_tool_job_rows` rejects the whole app otherwise
/// (`interactive-job.catalog-incomplete`), which is why an app with migrated ids and no factory is
/// not "slow", it is unconstructable.
const SPACE_INDEX_RETAINED_TOOL_IDS: &[&str] = &[
    "createArtifact",
    "deleteArtifact",
    "renameArtifact",
    "touchArtifact",
    "requestDeleteArtifact",
    "openArtifact",
    "openArtifactWith",
    "foldDirectoryEvents",
    "presenceHeartbeat",
    "inviteMember",
    "removeMember",
    "setVisibility",
    "copyInviteLink",
    "requestInviteMember",
];
const SPACE_INDEX_RETAINED_PAYLOAD_SCHEMA: &str = "s.space.index.tool-command.v1";
const SPACE_INDEX_RETAINED_RAW_BYTES: usize = 128 * 1024;
const SPACE_INDEX_RETAINED_WORK_ITEMS: usize = 1;
const SPACE_INDEX_RETAINED_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
/// 🛣️ Read off each handler's own `Emit`: the four document commands publish `artifact_mutations`,
/// the two directory/presence folds publish `config_mutations`, and every `Effect`-only relay
/// publishes nothing at all.
const SPACE_INDEX_RETAINED_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "createArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "touchArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "requestDeleteArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openArtifact", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openArtifactWith", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "foldDirectoryEvents", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "presenceHeartbeat", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "inviteMember", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeMember", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setVisibility", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "copyInviteLink", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "requestInviteMember", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
];

fn space_index_retained_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(SPACE_INDEX_RETAINED_RAW_BYTES, 64, SPACE_INDEX_RETAINED_WORK_ITEMS as u64, SPACE_INDEX_RETAINED_OUTPUT_BYTES, 7_500)
}

fn space_index_retained_extent(command: &SpaceIndexCommand, _snapshot: &SSpaceSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SPACE_INDEX_RETAINED_TOOL_IDS.contains(&command.command_id()).then_some(SPACE_INDEX_RETAINED_WORK_ITEMS)
}

fn space_index_retained_reduce(
    command: &SpaceIndexCommand,
    snapshot: &SSpaceSnapshot,
    config: &SpaceIndexConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SpaceIndexEditor>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation, NoDraftMutation>, Fault> {
    if space_index_retained_extent(command, snapshot, interaction).is_none() {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.index.retained.route"), "the bounded space index reducer rejects an unregistered tool"));
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

pub struct SpaceIndexRetainedCommandJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SpaceIndexRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SPACE_INDEX_RETAINED_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SpaceIndexRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SpaceIndexEditor>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SpaceIndexEditor>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SPACE_INDEX_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        space_index_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SPACE_INDEX_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("bounded space index command rejects an oversized wire or a checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SpaceIndexRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SpaceIndexEditor>;
    const TOOL_IDS: &'static [&'static str] = SPACE_INDEX_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::S_SPACE_INDEX_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SPACE_INDEX_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔖️SpaceIndexEditor
#[derive(Default)]
pub struct SpaceIndexEditor;

impl ArtifactEditor for SpaceIndexEditor {
    type Snapshot = SSpaceSnapshot;
    type Mutation = SSpaceMutation;
    type Config = SpaceIndexConfig;
    type ConfigMutation = SpaceIndexConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = SpaceIndexCommand;

    const DIALECT: Dialect = SPACE_INDEX_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::S_SPACE_INDEX_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SpaceIndexEditor>,
        owner_file: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.space.space@1/*#editor",
        document_schema: "s.space",
        factory: "SpaceIndexRetainedCommandJobFactory",
        factory_type: SpaceIndexRetainedCommandJobFactory,
        contract: space_index_retained_contract(),
        tools: ["createArtifact", "deleteArtifact", "renameArtifact", "touchArtifact", "requestDeleteArtifact", "openArtifact", "openArtifactWith",
            "foldDirectoryEvents", "presenceHeartbeat", "inviteMember", "removeMember", "setVisibility", "copyInviteLink", "requestInviteMember"]
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::space_core::space_retained_store_preparation::<Self::Snapshot, Self::Mutation>("space-index-artifact-retained", SPACE_INDEX_RETAINED_OUTPUT_BYTES)
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        crate::space_core::space_retained_store_preparation::<Self::Config, Self::ConfigMutation>("space-index-config-retained", SPACE_INDEX_RETAINED_OUTPUT_BYTES)
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SpaceIndexRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !SPACE_INDEX_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.index.retained.tool-mismatch"), "space index command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, space_index_retained_reduce, space_index_retained_extent));
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            SpaceIndexCommand::command_id,
            SPACE_INDEX_RETAINED_RAW_BYTES,
            SPACE_INDEX_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn initial_snapshot() -> SSpaceSnapshot {
        SSpaceSnapshot::default()
    }

    fn command_id(command: &SpaceIndexCommand) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &SpaceIndexCommand,
        doc: &ArtifactView<'_, SSpaceSnapshot>,
        cfg: &ConfigView<'_, SpaceIndexConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: this bridge was
    /// missing entirely — `ArtifactEditor::command_from_action`'s default impl unconditionally errors
    /// (`app.command.unsupported`), and `dispatch_action`'s final `else` arm (the ONLY path a plain
    /// `onAction`/`handleAction` click — every row button, every toolbar button — reaches an app's own
    /// command through) calls exactly this. Every one of this app's own actions was therefore a dead
    /// click until now: `openArtifact`/`requestDeleteArtifact` (table row buttons), `createArtifact`
    /// (the new `#s-space-create-artifact` toolbar button), the members panel's invite/remove/visibility/
    /// copy-link buttons — all of it. Mirrors `HomeCommand::command_from_action`'s `str_field` idiom
    /// (`🏠️home/…/✏️editor/🦀️.rs`) field-for-field against each command payload struct.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<SpaceIndexCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string);
        let u64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64).map(|value| value as u64);
        match action {
            "createArtifact" => Ok(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact { name: str_field("name").unwrap_or_default(), kind_choice: str_field("kindChoice").unwrap_or_default() })),
            "deleteArtifact" => Ok(SpaceIndexCommand::DeleteArtifact(delete_artifact::DeleteArtifact { id: str_field("id").unwrap_or_default() })),
            "renameArtifact" => Ok(SpaceIndexCommand::RenameArtifact(rename_artifact::RenameArtifact { id: str_field("id").unwrap_or_default(), new_name: str_field("newName").or_else(|| str_field("new_name")).unwrap_or_default() })),
            "touchArtifact" => Ok(SpaceIndexCommand::TouchArtifact(touch_artifact::TouchArtifact {
                id: str_field("id").unwrap_or_default(),
                now_ms: u64_field("nowMs").or_else(|| u64_field("now_ms")).unwrap_or_default(),
                actor: str_field("actor").unwrap_or_default(),
            })),
            "requestDeleteArtifact" => Ok(SpaceIndexCommand::RequestDeleteArtifact(request_delete_artifact::RequestDeleteArtifact { id: str_field("id").unwrap_or_default() })),
            "openArtifact" => Ok(SpaceIndexCommand::OpenArtifact(open_artifact::OpenArtifact { id: str_field("id").unwrap_or_default() })),
            "openArtifactWith" => Ok(SpaceIndexCommand::OpenArtifactWith(open_artifact_with::OpenArtifactWith {
                id: str_field("id").unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
                plugin_id: str_field("pluginId").or_else(|| str_field("plugin_id")).unwrap_or_default(),
                app_id: str_field("appId").or_else(|| str_field("app_id")).unwrap_or_default(),
            })),
            "foldDirectoryEvents" => Ok(SpaceIndexCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents { events_json: str_field("eventsJson").or_else(|| str_field("events_json")).unwrap_or_else(|| "[]".into()) })),
            "presenceHeartbeat" => Ok(SpaceIndexCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {
                artifact_id: str_field("artifactId").or_else(|| str_field("artifact_id")).unwrap_or_default(),
                actors_csv: str_field("actorsCsv").or_else(|| str_field("actors_csv")).unwrap_or_default(),
            })),
            "inviteMember" => Ok(SpaceIndexCommand::InviteMember(invite_member::InviteMember { email: str_field("email").unwrap_or_default(), role: str_field("role").unwrap_or_default() })),
            "removeMember" => Ok(SpaceIndexCommand::RemoveMember(remove_member::RemoveMember { user_id: str_field("userId").or_else(|| str_field("user_id")).unwrap_or_default() })),
            "setVisibility" => Ok(SpaceIndexCommand::SetVisibility(set_visibility::SetVisibility { visibility: str_field("visibility").unwrap_or_default() })),
            "copyInviteLink" => Ok(SpaceIndexCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { role: str_field("role").unwrap_or_default(), ttl_secs: u64_field("ttlSecs").or_else(|| u64_field("ttl_secs")).unwrap_or(0) })),
            "requestInviteMember" => Ok(SpaceIndexCommand::RequestInviteMember(request_invite_member::RequestInviteMember {})),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.unhandled-action"), format!("space index: unhandled action id {other}"))),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => Ok(built_to_component_tree(main::render(doc.snapshot, cfg.snapshot)?)),
            members_panel::SPACE_INDEX_BODY_MEMBERS => Ok(built_to_component_tree(members_panel::render(cfg.snapshot)?)),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️SpaceIndexEditor

//#region 🔖️Manifest
pub fn create_space_index_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SPACE_INDEX_DIALECT)
        .document(["semio", "s", "space", "index"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("layout-grid")
        .mode_def(edit::definition())
        .default_mode_id(edit::SPACE_INDEX_MODE_EDIT)
        .window_kind_def(main::definition())
        .panel_tab_def(members_panel::definition())
        .default_layout(edit::layout())
        // 🌱 Document mutations — palette-visible.
        .mutation("createArtifact", LocalizedLabel::native("Create Artifact", "Artefakt erstellen"))
        .mutation("deleteArtifact", LocalizedLabel::native("Delete Artifact", "Artefakt löschen"))
        .mutation("renameArtifact", LocalizedLabel::native("Rename Artifact", "Artefakt umbenennen"))
        .mutation("touchArtifact", LocalizedLabel::native("Touch Artifact", "Artefakt aktualisieren"))
        // 🐚 Shell-effect relays — no document mutation of their own (contract §C6).
        .shell_action("requestDeleteArtifact", LocalizedLabel::native("Delete Artifact…", "Artefakt löschen…"))
        .shell_action("openArtifact", LocalizedLabel::native("Open Artifact", "Artefakt öffnen"))
        .shell_action("openArtifactWith", LocalizedLabel::native("Open Artifact With…", "Artefakt öffnen mit…"))
        .shell_action("inviteMember", LocalizedLabel::native("Invite Member (Submit)", "Mitglied einladen (Absenden)"))
        .shell_action("requestInviteMember", LocalizedLabel::native("Invite Member…", "Mitglied einladen…"))
        .shell_action("removeMember", LocalizedLabel::native("Remove Member", "Mitglied entfernen"))
        .shell_action("setVisibility", LocalizedLabel::native("Set Visibility", "Sichtbarkeit festlegen"))
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))
        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse übernehmen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        // 🧵️ Blanket pass FIRST: `action_interactive_job` only reaches `self.actions`, while the
        // window kinds re-expose the same commands under `framework.window.table:<id>` — those stay
        // `Unclassified` and abort `build_definition` at runtime unless something covers them. This
        // is the same idiom the `🌍️gis` editors use. The per-action calls below then refine it.
        .interactive_jobs(InteractiveJobClassification::Migrated)
        // 🧵️ Phase-8 interactive-job disposition. `validate_interactive_job_classification` rejects a
        // catalog containing any `Unclassified` action, and the default IS `Unclassified`, so every
        // action above needs an entry here. Dispositions mirror the sibling `🏠️home` editor for the
        // three names it also declares (`copyInviteLink`/`presenceHeartbeat` migrated,
        // `foldDirectoryEvents` batch-only) and follow its closest analogue otherwise: one-turn
        // interactive commands are `Migrated`, and `renameArtifact` is `BatchOnlyPendingRewrite`
        // because `home` classifies its own `renameSpace` that way.
        .action_interactive_job("createArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("renameArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("touchArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("requestDeleteArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("openArtifact", InteractiveJobClassification::Migrated)
        .action_interactive_job("openArtifactWith", InteractiveJobClassification::Migrated)
        .action_interactive_job("inviteMember", InteractiveJobClassification::Migrated)
        .action_interactive_job("requestInviteMember", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeMember", InteractiveJobClassification::Migrated)
        .action_interactive_job("setVisibility", InteractiveJobClassification::Migrated)
        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("foldDirectoryEvents", InteractiveJobClassification::Migrated)
        .action_interactive_job("presenceHeartbeat", InteractiveJobClassification::Migrated)
        // 👁️ View actions — fold host-pushed state into `Config`, never in the palette.
        // 🗨️ Dialogs (worker-brief tasks 2–3). `createArtifact`'s submit re-dispatches the real
        // mutation directly (its own payload has no field the staged form can't supply); the delete
        // confirm and the invite form each go through a `request*` opener (see those commands' own
        // doc comments for why).
        .dialog(
            DialogDefinition::new("createArtifact", LocalizedLabel::native("Create Artifact", "Artefakt erstellen"), ActionRef::new("createArtifact"))
                .args(vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(), ActionArgDef::artifact_kind("kindChoice", LocalizedLabel::native("Kind", "Art"), vec![semio_framework_plugin::AppRole::Editor]).required()])
                .submit_label(LocalizedLabel::native("Create", "Erstellen")),
        )
        .dialog(
            DialogDefinition::new("deleteArtifact", LocalizedLabel::native("Delete Artifact?", "Artefakt löschen?"), ActionRef::new("deleteArtifact"))
                .body(LocalizedLabel::native("This removes the artifact from the space. This cannot be undone.", "Dies entfernt das Artefakt aus dem Space. Dies kann nicht rückgängig gemacht werden."))
                .submit_label(LocalizedLabel::native("Delete", "Löschen")),
        )
        .dialog(
            DialogDefinition::new("inviteMember", LocalizedLabel::native("Invite Member", "Mitglied einladen"), ActionRef::new("inviteMember"))
                .args(vec![
                    ActionArgDef::text("email", LocalizedLabel::native("Email", "E-Mail")).required(),
                    ActionArgDef::select("role", LocalizedLabel::native("Role", "Rolle"), vec![ActionArgOption::new("author", LocalizedLabel::native("Author", "Autor")), ActionArgOption::new("spectator", LocalizedLabel::native("Spectator", "Betrachter"))]).default_value(&"spectator").required(),
                ])
                .submit_label(LocalizedLabel::native("Invite", "Einladen")),
        )
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
