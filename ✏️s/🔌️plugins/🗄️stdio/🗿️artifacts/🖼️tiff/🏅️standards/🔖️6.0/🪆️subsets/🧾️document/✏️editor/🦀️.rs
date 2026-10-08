//! ✏️ `tiff` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::tiff_any::modes::edit;
use crate::editor::tiff_any::modes::edit::windows::main;
use crate::standards::v6_0::subsets::document::schema::mutations::{ReplaceSamplesMutation, TiffMutation};
use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffIfd, TiffSnapshot, TiffTag};
use crate::{STDIO_TIFF_DOCUMENT_SCHEMA, TIFF_ANY_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactBoundedFirstStepProof;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_2d::compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs"]
pub(crate) mod paint_region;
#[path = "🎚️config/🦀️.rs"]
pub(crate) mod config;
use config::{TiffEditorConfig, TiffEditorConfigMutation};

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum TiffAnyEditCommand {
    PaintRegion(paint_region::PaintRegionCommand),
    SelectIfd { ifd_index: usize },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for TiffAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_any-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "tiff_any-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    main::SELECT_IFD_ACTION_ID,
];
const STDIO_TIFF_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    main::SELECT_IFD_ACTION_ID,
    paint_region::ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.tiff.tool-command.v1";
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: main::SELECT_IFD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
];
fn tiffAnyEditor_example_snapshot(example_id: &str) -> TiffSnapshot {
    if example_id == crate::examples::demo::ID { <TiffSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { TiffSnapshot::default() }
}
fn tiffAnyEditor_command_id(command: &TiffAnyEditCommand) -> &'static str {
    if let TiffAnyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        TiffAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        TiffAnyEditCommand::PaintRegion(_) => paint_region::ACTION_ID,
        TiffAnyEditCommand::SelectIfd { .. } => main::SELECT_IFD_ACTION_ID,
        TiffAnyEditCommand::EditSnapshot { .. } => unreachable!("handled above"),
    }
}
fn tiffAnyEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<TiffAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| TiffAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        paint_region::ACTION_ID => Ok(TiffAnyEditCommand::PaintRegion(paint_region::PaintRegionCommand::from_action(args)?)),
        main::SELECT_IFD_ACTION_ID => {
            let Some(semio_framework_value::DslValue::Object(fields)) = args else { return Err(Fault::from("TIFF page selection requires arguments")); };
            let value = fields.iter().find(|(key, _)| key == "ifdIndex").map(|(_, value)| value).ok_or_else(|| Fault::from("TIFF page selection requires ifdIndex"))?;
            let semio_framework_value::DslValue::Number(value) = value else { return Err(Fault::from("TIFF page selection ifdIndex must be an integer")); };
            let ifd_index = value.as_u64().and_then(|value| usize::try_from(value).ok()).ok_or_else(|| Fault::from("TIFF page selection ifdIndex is outside the address space"))?;
            Ok(TiffAnyEditCommand::SelectIfd { ifd_index })
        }
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TiffAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not a TIFF editor action"))),
    }
}
fn tiffAnyEditor_retained_extent(command: &TiffAnyEditCommand, _snapshot: &TiffSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, TiffAnyEditCommand::SetActiveExample { .. } | TiffAnyEditCommand::SelectIfd { .. }).then_some(1)
}
fn tiffAnyEditor_retained_reduce(command: &TiffAnyEditCommand, snapshot: &TiffSnapshot, _config: &TiffEditorConfig, _history: &semio_framework_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_framework_plugin::app::InteractionHoverState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TiffAnyEditor>>>, _operation: &AppOperationContext) -> Result<Emit<TiffMutation, TiffEditorConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TiffAnyEditCommand::SetActiveExample { example_id } => Ok(Emit { config_mutations: vec![TiffEditorConfigMutation::SetSelectedIfd { selected_ifd: 0 }], effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffAnyEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)], ..Default::default() }),
        TiffAnyEditCommand::SelectIfd { ifd_index } => {
            if *ifd_index >= snapshot.ifds.len() { return Err(tiffAnyEditor_edit_fault("stdio.tiff.ifd-out-of-range", format!("image page {ifd_index} is outside 0..{}", snapshot.ifds.len()))); }
            Ok(Emit { config_mutations: vec![TiffEditorConfigMutation::SetSelectedIfd { selected_ifd: *ifd_index }], ..Default::default() })
        }
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn tiffAnyEditor_edit_fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message)
}
struct TiffAnyEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl TiffAnyEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for TiffAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TiffAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TiffAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for TiffAnyEditorExampleFactory {
    type Owner = EditorApp<TiffAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_CONTRACTS;
}
const STDIO_TIFF_PAINT_REGION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: paint_region::ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];
const fn tiffAnyEditor_paint_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(paint_region::MAXIMUM_RAW_BYTES, 64, paint_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct TiffPaintRegionFactory { keys: Vec<ToolFactoryKey> }
impl TiffPaintRegionFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: paint_region::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for TiffPaintRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TiffAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TiffAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { paint_region::PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { tiffAnyEditor_paint_region_execution_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload, input: semio_framework_plugin::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > paint_region::MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("TIFF paint region rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for TiffPaintRegionFactory {
    type Owner = EditorApp<TiffAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = paint_region::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = STDIO_TIFF_PAINT_REGION_CONTRACTS;
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TiffAnyEditor;

impl ArtifactEditor for TiffAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = TiffSnapshot;
    type Mutation = TiffMutation;
    type Config = TiffEditorConfig;
    type ConfigMutation = TiffEditorConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TiffAnyEditCommand;

    const DIALECT: Dialect = TIFF_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TIFF_DOCUMENT_SCHEMA;

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.tiff@6.0/*#editor";
        let mut proofs = vec![
            ArtifactBoundedFirstStepProof::new::<EditorApp<TiffAnyEditor>>(OWNER_FILE, CONTROLLER, "TiffAnyEditorExampleFactory", semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, "stdio.tiff", ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<TiffAnyEditor>, TiffAnyEditorExampleFactory>(),
            ArtifactBoundedFirstStepProof::new::<EditorApp<TiffAnyEditor>>(OWNER_FILE, CONTROLLER, "TiffAnyEditorExampleFactory", main::SELECT_IFD_ACTION_ID, "stdio.tiff", ToolExecutionContract::bounded_first_step(STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)).with_factory_type::<EditorApp<TiffAnyEditor>, TiffAnyEditorExampleFactory>(),
            ArtifactBoundedFirstStepProof::new::<EditorApp<TiffAnyEditor>>(OWNER_FILE, CONTROLLER, "TiffPaintRegionFactory", paint_region::ACTION_ID, "stdio.tiff", tiffAnyEditor_paint_region_execution_contract()).with_factory_type::<EditorApp<TiffAnyEditor>, TiffPaintRegionFactory>(),
        ];
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.tiff"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(TiffAnyEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(TiffPaintRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request); }
        if request.tool_id == paint_region::ACTION_ID {
            if tiffAnyEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-tiff-paint-region-tool-mismatch")); }
            let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
            let payload = ArtifactRetainedCommandPayload::try_new(
                ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion },
                tiffAnyEditor_command_id,
                paint_region::MAXIMUM_RAW_BYTES,
                paint_region::CAPACITY.work_items(),
                Box::new(paint_region::PaintRegionWork::new()),
            )?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if !STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if tiffAnyEditor_command_id(&request.command) != request.tool_id { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-example-tool-mismatch")); }
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id, operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision, authoring_seed: request.authoring_seed.clone() };
        let action_id = match request.tool_id.as_str() {
            semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
            main::SELECT_IFD_ACTION_ID => main::SELECT_IFD_ACTION_ID,
            _ => unreachable!("filtered above"),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation, completion: request.completion }, tiffAnyEditor_command_id, STDIO_TIFF_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, Box::new(BoundedArtifactCommandWork::new(action_id, tiffAnyEditor_retained_reduce, tiffAnyEditor_retained_extent)))?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-tiff-config-retained", 8_192))
    }
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }
    fn build_document_store_initialization_job(envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_TIFF_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { tiffAnyEditor_command_id(command) }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> { tiffAnyEditor_command_from_action(action, args) }

    fn initial_snapshot() -> Self::Snapshot {
        crate::standards::v6_0::subsets::document::schema::blank_tiff_snapshot()
    }

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            TiffAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            TiffAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                config_mutations: vec![TiffEditorConfigMutation::SetSelectedIfd { selected_ifd: 0 }],
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tiffAnyEditor_example_snapshot(example_id), STDIO_TIFF_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            TiffAnyEditCommand::SelectIfd { .. } => Err(Fault::from("TIFF page selection requires the retained config route")),
            TiffAnyEditCommand::PaintRegion(_) => Err(Fault::from("TIFF region painting requires the cancellable retained route")),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, cfg.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.tiff@6.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for TiffAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { TiffAnyEditCommand::EditSnapshot { event } => Some(event), TiffAnyEditCommand::SetActiveExample { .. } | TiffAnyEditCommand::PaintRegion(_) | TiffAnyEditCommand::SelectIfd { .. } => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::tiff_any::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        use crate::standards::v6_0::subsets::document::schema::snapshot::TiffWord64;
        let editing::SnapshotEditEvent::SetValue { path, .. } = event else { return Ok(None) };
        let segments: Vec<&str> = path.split('/').skip(1).collect();
        let ["ifds", ifd, "blocks", block, "samples", sample, rest @ ..] = segments.as_slice() else { return Ok(None) };
        let (Ok(ifd_index), Ok(block_index), Ok(offset)) = (ifd.parse::<usize>(), block.parse::<usize>(), sample.parse::<usize>()) else { return Ok(None) };
        if rest.len() > 1 {
            return Ok(None);
        }
        let current = snapshot.ifds.get(ifd_index).and_then(|page| page.blocks.get(block_index)).and_then(|block| block.samples.get(offset)).ok_or_else(|| Fault::from(format!("sample {path} is outside the image")))?;
        let word = editing::edited_subtree(&semio_framework_value::ToValue::to_value(current), &format!("/ifds/{ifd}/blocks/{block}/samples/{sample}"), event).map_err(|error| tiffAnyEditor_edit_fault(error.code, error.to_string()))?;
        let word = <TiffWord64 as semio_framework_value::FromValue>::from_value(word).map_err(|error| tiffAnyEditor_edit_fault("stdio.tiff.invalid-sample", error.to_string()))?;
        Ok(Some(vec![TiffMutation::ReplaceSamples(ReplaceSamplesMutation { ifd_index, block: block_index, offset, samples: vec![word] })]))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_tiff_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(TIFF_ANY_DIALECT).document(["semio", "tiff"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_with(paint_region::action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_interactive_job(main::SELECT_IFD_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_interactive_job(paint_region::ACTION_ID, InteractiveJobClassification::Migrated)
        ;
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
