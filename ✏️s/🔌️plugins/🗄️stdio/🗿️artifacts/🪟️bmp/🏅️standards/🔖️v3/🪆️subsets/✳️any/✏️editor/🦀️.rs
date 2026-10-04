//! ✏️ `bmp` editor (any) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::bmp::modes::edit;
use crate::editor::bmp::modes::edit::windows::main;
use crate::standards::v_v3::subsets::any::schema::mutations::{patch_snapshot, set_snapshot as snapshot_edit_set_snapshot, BmpMutation};
use crate::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot;
use crate::{BMP_DIALECT, STDIO_BMP_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactBoundedFirstStepProof;
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
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_s_artifact_stdio_contract::editing;

#[path = "🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs"]
pub(crate) mod paint_region;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum BmpNativeEditCommand {
    PaintIndexedRegion(paint_region::PaintIndexedRegionCommand),
    PaintDirectRegion(paint_region::PaintDirectRegionCommand),
    /// 🎬️ Navbar example picker payload.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpBinary for BmpNativeEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_BMP_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "bmp-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "bmp-edit-command", offset: 0, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(BmpNativeEditCommand, [semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, paint_region::INDEXED_ACTION_ID, paint_region::DIRECT_ACTION_ID]);
pub type BmpEditCommand = editing::SnapshotEditingCommand<BmpNativeEditCommand>;
//#endregion 🔖️Command

const STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_BMP_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, paint_region::INDEXED_ACTION_ID, paint_region::DIRECT_ACTION_ID];
const STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.bmp.tool-command.v1";
const STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn bmpEditor_example_snapshot(example_id: &str) -> BmpSnapshot {
    if example_id == crate::examples::demo::ID {
        <BmpSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        BmpSnapshot::default()
    }
}
fn bmpEditor_native_command_id(command: &BmpNativeEditCommand) -> &'static str {
    match command {
        BmpNativeEditCommand::PaintIndexedRegion(_) => paint_region::INDEXED_ACTION_ID,
        BmpNativeEditCommand::PaintDirectRegion(_) => paint_region::DIRECT_ACTION_ID,
        BmpNativeEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}
fn bmpEditor_command_id(command: &BmpEditCommand) -> &'static str {
    editing::snapshot_editing_command_id(command, bmpEditor_native_command_id)
}
fn bmpEditor_native_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<BmpNativeEditCommand, Fault> {
    match action {
        paint_region::INDEXED_ACTION_ID => Ok(BmpNativeEditCommand::PaintIndexedRegion(paint_region::PaintIndexedRegionCommand::from_action(args)?)),
        paint_region::DIRECT_ACTION_ID => Ok(BmpNativeEditCommand::PaintDirectRegion(paint_region::PaintDirectRegionCommand::from_action(args)?)),
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(BmpNativeEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not a BMP editor action"))),
    }
}
fn bmpEditor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<BmpEditCommand, Fault> {
    editing::snapshot_editing_command_from_action(action, args, bmpEditor_native_command_from_action)
}
fn bmpEditor_retained_extent(command: &BmpEditCommand, _snapshot: &BmpSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, BmpEditCommand::Native(BmpNativeEditCommand::SetActiveExample { .. })).then_some(1)
}
fn bmpEditor_retained_reduce(
    command: &BmpEditCommand,
    _snapshot: &BmpSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BmpEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<BmpMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        BmpEditCommand::Native(BmpNativeEditCommand::SetActiveExample { example_id }) => {
            Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&bmpEditor_example_snapshot(example_id), STDIO_BMP_DOCUMENT_SCHEMA)], ..Default::default() })
        }
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct BmpEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}
impl BmpEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for BmpEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BmpEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BmpEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)
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
        if input.declared_bytes() > STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio example command rejects oversized wire"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for BmpEditorExampleFactory {
    type Owner = EditorApp<BmpEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BMP_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
const STDIO_BMP_PAINT_REGION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: paint_region::INDEXED_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: paint_region::DIRECT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];
const fn bmpEditor_paint_region_execution_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(paint_region::MAXIMUM_RAW_BYTES, 64, paint_region::CAPACITY.work_items() as u64, 65_536, 7_500)
}
struct BmpPaintRegionFactory {
    keys: Vec<ToolFactoryKey>,
}
impl BmpPaintRegionFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: paint_region::TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for BmpPaintRegionFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BmpEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BmpEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        paint_region::PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        bmpEditor_paint_region_execution_contract()
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
        if input.declared_bytes() > paint_region::MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("BMP paint region rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}
impl ArtifactOwnedToolJobFactory for BmpPaintRegionFactory {
    type Owner = EditorApp<BmpEditor>;
    const TOOL_IDS: &'static [&'static str] = paint_region::TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BMP_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = STDIO_BMP_PAINT_REGION_CONTRACTS;
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BmpEditor;

impl ArtifactEditor for BmpEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = BmpSnapshot;
    type Mutation = BmpMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BmpEditCommand;

    const DIALECT: Dialect = BMP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BMP_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.bmp@v3", extension: ".bmp", media_type: "image/bmp", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::io::encode_bmp(snapshot).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        crate::io::decode_bmp(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(BmpMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot }))
    }

    fn bounded_first_step_tool_proofs() -> Vec<ArtifactBoundedFirstStepProof> {
        const OWNER_FILE: &str = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs";
        const CONTROLLER: &str = "s.stdio.bmp@v3/*#editor";
        let mut proofs = vec![ArtifactBoundedFirstStepProof::new::<EditorApp<BmpEditor>>(
            OWNER_FILE,
            CONTROLLER,
            "BmpEditorExampleFactory",
            semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
            "stdio.bmp",
            ToolExecutionContract::bounded_first_step(STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        )
        .with_factory_type::<EditorApp<BmpEditor>, BmpEditorExampleFactory>()];
        for tool_id in paint_region::TOOL_IDS {
            proofs.push(
                ArtifactBoundedFirstStepProof::new::<EditorApp<BmpEditor>>(OWNER_FILE, CONTROLLER, "BmpPaintRegionFactory", tool_id, "stdio.bmp", bmpEditor_paint_region_execution_contract())
                    .with_factory_type::<EditorApp<BmpEditor>, BmpPaintRegionFactory>(),
            );
        }
        proofs.extend(editing::snapshot_edit_bounded_first_step_proofs::<Self>(OWNER_FILE, CONTROLLER, "stdio.bmp"));
        proofs
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(BmpEditorExampleFactory::new(registry.controller_id()))?;
        registry.register(BmpPaintRegionFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if editing::is_snapshot_edit_action(&request.tool_id) {
            return editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if paint_region::TOOL_IDS.contains(&request.tool_id.as_str()) {
            if bmpEditor_command_id(&request.command) != request.tool_id {
                return Err(Fault::from("stdio-bmp-paint-region-tool-mismatch"));
            }
            let operation = AppOperationContext {
                app_instance_id: request.app_instance_id,
                parent_document_id: request.parent_document_id,
                operation_id: request.operation.operation.0,
                generation: request.operation.generation.0,
                canonical_base_revision: request.canonical_base_revision,
                authoring_seed: request.authoring_seed.clone(),
            };
            let tool_id = paint_region::TOOL_IDS.iter().copied().find(|tool_id| *tool_id == request.tool_id).expect("checked BMP paint tool");
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
                bmpEditor_command_id,
                paint_region::MAXIMUM_RAW_BYTES,
                paint_region::CAPACITY.work_items(),
                Box::new(paint_region::PaintRegionWork::new(tool_id)),
            )?;
            return Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        if !STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if bmpEditor_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-example-tool-mismatch"));
        }
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
            bmpEditor_command_id,
            STDIO_BMP_DOCUMENT_SCHEMA_EXAMPLE_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, bmpEditor_retained_reduce, bmpEditor_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory("stdio-snapshot-edit-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_BMP_DOCUMENT_SCHEMA, operation, generation))
    }
    fn command_id(command: &Self::Command) -> &'static str {
        bmpEditor_command_id(command)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        bmpEditor_command_from_action(action, args)
    }

    fn initial_snapshot() -> Self::Snapshot {
        BmpSnapshot::default()
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
            BmpEditCommand::Edit(event) => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            BmpEditCommand::Native(BmpNativeEditCommand::SetActiveExample { example_id }) => {
                Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&bmpEditor_example_snapshot(example_id), STDIO_BMP_DOCUMENT_SCHEMA)], ..Default::default() })
            }
            BmpEditCommand::Native(BmpNativeEditCommand::PaintIndexedRegion(_) | BmpNativeEditCommand::PaintDirectRegion(_)) => Err(Fault::from("BMP region painting requires the cancellable retained route")),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.bmp@v3/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for BmpEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            BmpEditCommand::Edit(event) => Some(event),
            BmpEditCommand::Native(_) => None,
        }
    }
    fn snapshot_edit_mutations(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        editing::snapshot_edit_patch(event, snapshot, |patch| BmpMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }), Some(|snapshot| BmpMutation::SetSnapshot(snapshot_edit_set_snapshot::SetSnapshot { snapshot })))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_bmp_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(BMP_DIALECT)
        .document(["semio", "bmp"])
        .icon_id("image")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(editing::snapshot_details_window_definition())
        .default_layout(edit::layout())
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_with(paint_region::indexed_action())
        .action_describe(
            paint_region::INDEXED_ACTION_ID,
            LocalizedLabel::native("Paints palette indices directly without color matching and keeps duplicate palette identities distinct.", "Malt Palettenindizes direkt ohne Farbabgleich und erhält doppelte Palettenidentitäten."),
        )
        .action_interactive_job(paint_region::INDEXED_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_with(paint_region::direct_action())
        .action_describe(
            paint_region::DIRECT_ACTION_ID,
            LocalizedLabel::native("Paints declared direct-color channels while preserving unmasked sample bits and unrelated BMP bytes.", "Malt deklarierte Direktfarbkanäle und erhält unmaskierte Sample-Bits sowie unabhängige BMP-Bytes."),
        )
        .action_interactive_job(paint_region::DIRECT_ACTION_ID, InteractiveJobClassification::Migrated);
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
