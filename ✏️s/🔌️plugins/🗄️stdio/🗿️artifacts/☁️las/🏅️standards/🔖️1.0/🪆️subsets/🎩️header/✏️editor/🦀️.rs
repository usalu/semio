//! ✏️ LAS editor — thin, kit-based editor surface (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `LasAnyEditor`
//! implements `ArtifactEditor`, wiring the shared `MeshWindowKit` to a single Main window.

use crate::editor::las::modes::edit;
use crate::editor::las::modes::edit::windows::main;
use crate::standards::v1_0::subsets::any::schema::mutations::{LasMutation};

use crate::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Dialect
pub const LAS_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId::ANY };
pub const LAS_ANY_DOCUMENT_SCHEMA: &str = "stdio.las";
//#endregion 🔖️Dialect

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum LasAnyEditCommand {
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for LasAnyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = LAS_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "LasAnyEditCommand", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "LasAnyEditCommand", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };

const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const LAS_ANY_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.las.tool-command.v1";
const LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;

fn las_any_editor_example_snapshot(example_id: &str) -> LasSnapshot {
    if example_id == crate::examples::demo::ID {
        <LasSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        LasSnapshot::default()
    }
}

fn las_any_editor_command_id(command: &LasAnyEditCommand) -> &'static str {
    if let LasAnyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        LasAnyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}

fn las_any_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<LasAnyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| LasAnyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(LasAnyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}

fn las_any_editor_retained_extent(command: &LasAnyEditCommand, _snapshot: &LasSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, LasAnyEditCommand::SetActiveExample { .. }).then_some(1)
}

fn las_any_editor_retained_reduce(
    command: &LasAnyEditCommand,
    _snapshot: &LasSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<LasAnyEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<LasMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        LasAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&las_any_editor_example_snapshot(example_id), LAS_ANY_DOCUMENT_SCHEMA)],
            ..Default::default()
        }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}

struct LasAnyEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LasAnyEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for LasAnyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<LasAnyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<LasAnyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}

impl ArtifactOwnedToolJobFactory for LasAnyEditorExampleFactory {
    type Owner = EditorApp<LasAnyEditor>;
    const TOOL_IDS: &'static [&'static str] = LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LAS_ANY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct LasAnyEditor;

impl ArtifactEditor for LasAnyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = LasSnapshot;
    type Mutation = LasMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = LasAnyEditCommand;

    const DIALECT: Dialect = LAS_ANY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LAS_ANY_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<LasAnyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs",
        controller: "s.stdio.las@1.0/*#editor",
        artifact_schema: "stdio.las",
        factory: "LasAnyEditorExampleFactory",
        factory_type: LasAnyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(LasAnyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if !LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if las_any_editor_command_id(&original.command) != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, las_any_editor_command_id, LAS_ANY_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, las_any_editor_retained_reduce, las_any_editor_retained_extent))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn command_id(command: &Self::Command) -> &'static str { las_any_editor_command_id(command) }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); las_any_editor_command_from_action(action, args) }

    fn reference_birth()->Option<fn()->Result<LasSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(LasSnapshot::default())
    })}

    fn handle(
        command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            LasAnyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, _doc.snapshot),
            LasAnyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&las_any_editor_example_snapshot(example_id), LAS_ANY_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.las@1.0/*#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for LasAnyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { LasAnyEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::las::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        use crate::standards::v1_0::subsets::any::schema::mutations::{insert_vlr, remove_vlr, set_bounds, set_creation_date, set_points_by_return, set_scale_and_offset, set_software_info, set_system_identifier, set_version};
        use crate::standards::v1_0::subsets::any::schema::snapshot::{LasHeader, LasVlr};
        let editing::SnapshotEditEvent::SetValue { path, .. } = event else { return Ok(None) };
        let fail = |message: String| Fault::from(message);
        let segments: Vec<&str> = path.split('/').skip(1).collect();
        match segments.as_slice() {
            ["header", field, ..] => {
                let edited = editing::edited_subtree(&semio_framework_value::ToValue::to_value(&snapshot.header), "/header", event).map_err(|error| fail(error.to_string()))?;
                let new = <LasHeader as semio_framework_value::FromValue>::from_value(edited).map_err(|error| fail(error.to_string()))?;
                if new == snapshot.header {
                    return Ok(Some(Vec::new()));
                }
                let leaf = match *field {
                    "versionMajor" | "versionMinor" => LasMutation::SetVersion(set_version::SetVersion { major: new.version_major, minor: new.version_minor }),
                    "systemIdentifier" => LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: new.system_identifier }),
                    "generatingSoftware" => LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: new.generating_software }),
                    "creationDayOfYear" | "creationYear" => LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: new.creation_day_of_year, year: new.creation_year }),
                    "xScale" | "yScale" | "zScale" | "xOffset" | "yOffset" | "zOffset" => {
                        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (new.x_scale, new.y_scale, new.z_scale), offset: (new.x_offset, new.y_offset, new.z_offset) })
                    }
                    "maxX" | "minX" | "maxY" | "minY" | "maxZ" | "minZ" => LasMutation::SetBounds(set_bounds::SetBounds { max: (new.max_x, new.max_y, new.max_z), min: (new.min_x, new.min_y, new.min_z) }),
                    "pointsByReturn" => LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: new.points_by_return }),
                    other => return Err(fail(format!("no kind edits the structural header field '{other}' ({path})"))),
                };
                Ok(Some(vec![leaf]))
            }
            ["vlrs", index, field] if *field != "data" => {
                let index = index.parse::<usize>().map_err(|error| fail(error.to_string()))?;
                let current = snapshot.vlrs.get(index).ok_or_else(|| fail(format!("VLR {index} does not exist")))?;
                let edited = editing::edited_subtree(&semio_framework_value::ToValue::to_value(current), &format!("/vlrs/{index}"), event).map_err(|error| fail(error.to_string()))?;
                let vlr = <LasVlr as semio_framework_value::FromValue>::from_value(edited).map_err(|error| fail(error.to_string()))?;
                if vlr == *current {
                    return Ok(Some(Vec::new()));
                }
                Ok(Some(vec![LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }), LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr })]))
            }
            _ => Ok(None),
        }
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_las_any_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(LAS_ANY_DIALECT).document(["stdio", "las"]).icon_id("box").mode_def(edit::definition()).default_mode_id(edit::LAS_ANY_EDIT_MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(edit::layout()).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        ;
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_LAS_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::LasMutation;
pub use crate::schema::snapshot::LasSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    store::bare_artifact_codec::<LasSnapshot, LasMutation>(STDIO_LAS_DOCUMENT_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.las.v1", artifact: "las", kind: artifact_kind, codec: native_codec }]
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{native_codecs};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}
}
//#endregion 🧩️ArtifactItems7
