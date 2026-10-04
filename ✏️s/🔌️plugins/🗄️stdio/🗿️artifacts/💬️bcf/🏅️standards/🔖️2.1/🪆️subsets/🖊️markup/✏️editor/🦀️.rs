//! ✏️ BCF editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `BcfAnyEditor`
//! implements `ArtifactEditor`, wiring the shared `TableWindowKit` to a single Main window.

use crate::editor::bcf::modes::edit;
use crate::editor::bcf::modes::edit::windows::main;
use crate::standards::v2_1::subsets::any::schema::mutations::{patch_snapshot::PatchSnapshot, set_snapshot::SetSnapshot, set_topic_markup::SetTopicMarkup, BcfMutation};
use crate::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
use semio_s_artifact_stdio_contract::pack;

//#region 🔖️Dialect
pub const BCF_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId::ANY };
pub const BCF_ANY_DOCUMENT_SCHEMA: &str = "stdio.bcf";
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The standard table-cell edit, guarded by a whole-snapshot revision because BCF topics are
/// stored positionally. The reducer publishes the topic-markup leaf, or a path-scoped snapshot patch for the GUID, after
/// changing exactly one modeled topic field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum BcfAnyEditCommand {
    SetCell {
        row: u32,
        column: u32,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpBinary for BcfAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = BCF_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "bcf-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "bcf-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const BCF_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "set-cell"];
const BCF_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    "set-cell",
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const BCF_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.bcf.tool-command.v1";
const BCF_RETAINED_RAW_BYTES: usize = 8_192;
const BCF_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "set-cell", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn bcf_example_snapshot(example_id: &str) -> BcfSnapshot {
    if example_id == crate::examples::demo::ID {
        <BcfSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        BcfSnapshot::default()
    }
}

fn bcf_command_id(command: &BcfAnyEditCommand) -> &'static str {
    match command {
        BcfAnyEditCommand::SetCell { .. } => "set-cell",
        BcfAnyEditCommand::EditSnapshot { event } => event.action_id(),
        BcfAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

fn bcf_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<BcfAnyEditCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(BcfAnyEditCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(BcfAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        "set-cell" => {
            let edit = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_edit(args)?;
            Ok(BcfAnyEditCommand::SetCell { row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
        }
        other => Err(Fault::from(format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-cell)"))),
    }
}

fn bcf_retained_extent(command: &BcfAnyEditCommand, _snapshot: &BcfSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    (!matches!(command, BcfAnyEditCommand::EditSnapshot { .. })).then_some(1)
}

fn bcf_emit(command: &BcfAnyEditCommand, snapshot: &BcfSnapshot) -> Result<Emit<BcfMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    bcf_emit_at_revision(command, snapshot, None)
}

fn bcf_emit_at_revision(command: &BcfAnyEditCommand, snapshot: &BcfSnapshot, canonical_revision: Option<&str>) -> Result<Emit<BcfMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        BcfAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&bcf_example_snapshot(example_id), BCF_ANY_DOCUMENT_SCHEMA)], ..Default::default() }),
        BcfAnyEditCommand::SetCell { row, column, revision, value } => {
            let current_revision = canonical_revision.map(str::to_owned).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot));
            if current_revision != *revision {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.bcf.table-conflict"), "The BCF document changed before this cell draft was applied."));
            }
            let topic = snapshot.topics.get(*row as usize).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.bcf.row-stale"), format!("BCF topic row {row} no longer exists")))?;
            let mutation = match column {
                0 => {
                    if topic.guid == *value {
                        return Ok(Emit::default());
                    }
                    BcfMutation::PatchSnapshot(PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: format!("/topics/{row}/guid"), value: semio_framework_value::DslValue::String(value.clone()) } })
                }
                1 => BcfMutation::SetTopicMarkup(SetTopicMarkup { guid: topic.guid.clone(), title: Some(value.clone()), description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None }),
                2 => BcfMutation::SetTopicMarkup(SetTopicMarkup { guid: topic.guid.clone(), title: None, description: None, status: Some(value.clone()), priority: None, labels: None, creation_date: None, creation_author: None }),
                3 => BcfMutation::SetTopicMarkup(SetTopicMarkup { guid: topic.guid.clone(), title: None, description: None, status: None, priority: Some(value.clone()), labels: None, creation_date: None, creation_author: None }),
                4 => BcfMutation::SetTopicMarkup(SetTopicMarkup { guid: topic.guid.clone(), title: None, description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: Some(value.clone()) }),
                _ => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.bcf.column-stale"), format!("BCF topic column {column} does not exist"))),
            };
            Ok(Emit::mutations(vec![mutation]))
        }
        BcfAnyEditCommand::EditSnapshot { .. } => Err(Fault::from("stdio-bcf-snapshot-edit-routed-to-native-reducer")),
    }
}

fn bcf_retained_reduce(
    command: &BcfAnyEditCommand,
    snapshot: &BcfSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BcfAnyEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<BcfMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
    bcf_emit_at_revision(command, snapshot, Some(&revision))
}

struct BcfRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BcfRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BCF_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for BcfRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BcfAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BcfAnyEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BCF_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(BCF_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
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
        if input.declared_bytes() > BCF_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio bcf retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for BcfRetainedCommandJobFactory {
    type Owner = EditorApp<BcfAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = BCF_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BCF_ANY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BCF_RETAINED_PUBLICATION_CONTRACTS;
}

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BcfAnyEditor;

impl ArtifactEditor for BcfAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = BcfSnapshot;
    type Mutation = BcfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BcfAnyEditCommand;

    const DIALECT: Dialect = BCF_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BCF_ANY_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<BcfAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs",
        controller: "s.stdio.bcf@2.1/*#editor",
        artifact_schema: "stdio.bcf",
        factory: "BcfRetainedCommandJobFactory",
        factory_type: BcfRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(BCF_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample", "set-cell"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(BcfRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !BCF_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if bcf_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-bcf-retained-command-tool-mismatch"));
        }
        let command_id = bcf_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            bcf_command_id,
            BCF_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(command_id, bcf_retained_reduce, bcf_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ `set-cell` publishes on the `Artifact` lane; without this authority every cell edit fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-bcf-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn command_id(command: &Self::Command) -> &'static str {
        bcf_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        bcf_command_from_action(action, args)
    }

    fn initial_snapshot() -> BcfSnapshot {
        BcfSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            BcfAnyEditCommand::SetCell { .. } => {
                let revision = doc.operation_optional().map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                bcf_emit_at_revision(command, doc.snapshot, revision.as_deref())
            }
            BcfAnyEditCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            BcfAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&bcf_example_snapshot(example_id), BCF_ANY_DOCUMENT_SCHEMA)], ..Default::default() }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision = doc
                    .render_operation()
                    .map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision))
                    .unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot));
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_revisioned(doc.snapshot, &revision, publication_revision, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.bcf@2.1/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for BcfAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            BcfAnyEditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| BcfMutation::PatchSnapshot(PatchSnapshot { patch }), Some(|snapshot| BcfMutation::SetSnapshot(SetSnapshot { snapshot })))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_bcf_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(BCF_ANY_DIALECT)
        .document(["stdio", "bcf"])
        .icon_id("box")
        .mode_def(edit::definition())
        .default_mode_id(edit::BCF_ANY_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Topics"))
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
