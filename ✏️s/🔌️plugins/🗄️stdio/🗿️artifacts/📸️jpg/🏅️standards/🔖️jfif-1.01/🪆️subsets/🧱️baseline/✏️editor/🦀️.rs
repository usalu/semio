//! ✏️ `jpg` editor (baseline) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Keeps the native image preview while retained Details actions publish typed artifact mutations.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::jpg_baseline::modes::edit;
use crate::editor::jpg_baseline::modes::edit::windows::main;
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation;
use crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot;
use crate::{JPG_BASELINE_DIALECT, STDIO_JPG_DOCUMENT_SCHEMA};
use semio_framework_2d_compute::EngineHandles;
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
use {semio_framework_artifact_reference::Dialect};
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
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum JpgBaselineEditCommand {
    SetPixelRegion {
        pixels: Vec<u8>,
    },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample {
        example_id: String,
    },
    EditSnapshot {
        event: editing::SnapshotEditEvent,
    },
}

impl protocol::OpBinary for JpgBaselineEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_JPG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed =
            semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_baseline-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "jpg_baseline-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const STDIO_JPG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.jpg.tool-command.v1";
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn jpg_baseline_editor_example_snapshot(example_id: &str) -> JpgSnapshot {
    if example_id == crate::examples::demo::ID {
        <JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        JpgSnapshot::default()
    }
}
fn jpg_baseline_editor_command_id(command: &JpgBaselineEditCommand) -> &'static str {
    if let JpgBaselineEditCommand::EditSnapshot { event } = command {
        return event.action_id();
    }
    match command {
        JpgBaselineEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        _ => "other",
    }
}
fn jpg_baseline_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<JpgBaselineEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) {
        return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| JpgBaselineEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit"))));
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(JpgBaselineEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        _ => Err(Fault::from(format!("action '{action}' is not setActiveExample"))),
    }
}
fn jpg_baseline_editor_retained_extent(command: &JpgBaselineEditCommand, _snapshot: &JpgSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, JpgBaselineEditCommand::SetActiveExample { .. }).then_some(1)
}
fn jpg_baseline_editor_retained_reduce(
    command: &JpgBaselineEditCommand,
    _snapshot: &JpgSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JpgBaselineEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<JpgMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        JpgBaselineEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&jpg_baseline_editor_example_snapshot(example_id), STDIO_JPG_DOCUMENT_SCHEMA)], ..Default::default() }),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
struct JpgBaselineEditorExampleFactory {
    keys: Vec<ToolFactoryKey>,
}
impl JpgBaselineEditorExampleFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl ToolJobFactory for JpgBaselineEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<JpgBaselineEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<JpgBaselineEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::bounded_first_step(STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500)
    }
    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for JpgBaselineEditorExampleFactory {
    type Owner = EditorApp<JpgBaselineEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT];
}
//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct JpgBaselineEditor;

impl ArtifactEditor for JpgBaselineEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = JpgBaselineEditCommand;

    const DIALECT: Dialect = JPG_BASELINE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_JPG_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.jpg@jfif-1.01", extension: ".jpg", media_type: "image/jpeg", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        let hard = crate::standards::v_jfif_1_01::subsets::baseline::io::check_baseline_content(snapshot)
            .into_iter()
            .filter(|diagnostic| matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
            .map(|diagnostic| diagnostic.message)
            .collect::<Vec<_>>();
        if !hard.is_empty() {
            return Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), hard.join("; ")));
        }
        crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg(snapshot, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::default()).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        if !bytes.ends_with(&[0xff, 0xd9]) {
            return Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), "JPEG input must end at its EOI marker".into()));
        }
        let admitted = crate::standards::v_jfif_1_01::subsets::document::io::decode_jpg_with_observations(&bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:natural".into(), error.to_string()))?;
        let hard = crate::standards::v_jfif_1_01::subsets::baseline::schema::conformance::check_baseline_facts(&admitted.observations.baseline_facts())
            .into_iter()
            .filter(|diagnostic| matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
            .map(|diagnostic| diagnostic.message)
            .collect::<Vec<_>>();
        if hard.is_empty() {
            Ok(admitted.snapshot)
        } else {
            Err(semio_framework_plugin::MediaError::Payload("artifact:natural".into(), hard.join("; ")))
        }
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<JpgBaselineEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs",
        controller: "s.stdio.jpg@jfif-1.01/baseline#editor",
        artifact_schema: "stdio.jpg",
        factory: "JpgBaselineEditorExampleFactory",
        factory_type: JpgBaselineEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(JpgBaselineEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) {
            return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx);
        }
        if !STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if jpg_baseline_editor_command_id(&original.command) != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, jpg_baseline_editor_command_id, STDIO_JPG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |request| BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, jpg_baseline_editor_retained_reduce, jpg_baseline_editor_retained_extent))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_JPG_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str {
        jpg_baseline_editor_command_id(command)
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        jpg_baseline_editor_command_from_action(action, args)
    }

    fn reference_birth()->Option<fn()->Self::Snapshot>{Some(||{
        crate::standards::v_jfif_1_01::subsets::document::io::blank_jpg_snapshot()
    })}

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            JpgBaselineEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            JpgBaselineEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&jpg_baseline_editor_example_snapshot(example_id), STDIO_JPG_DOCUMENT_SCHEMA)], ..Default::default() }),
            JpgBaselineEditCommand::SetPixelRegion { pixels } => {
                Ok(Emit::mutations(vec![JpgMutation::ReplacePixels(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::ReplacePixelsMutation { pixels:pixels.clone() })]))
            }
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.jpg@jfif-1.01/baseline#editor", &semio_framework_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY))
                .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

impl editing::SnapshotEditingEditor for JpgBaselineEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command {
            JpgBaselineEditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &crate::editor::jpg_any::edit_rules::EDIT_RULES
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_jpg_baseline_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(JPG_BASELINE_DIALECT)
        .document(["semio", "jpg"])
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
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_JPG_DOCUMENT_SCHEMA};
use crate::{JPG_ANY_DIALECT, JPG_BASELINE_DIALECT};
pub use crate::schema::mutations::JpgMutation;
pub use crate::schema::snapshot::JpgSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::standards::v_jfif_1_01::subsets::document::io::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// rows — 🧾️document + 🧱️baseline already folded into one list there) by its FULLY QUALIFIED path,
/// never the bare `io_registry::entries()` shortcut that would silently rebind to this file's own
/// shadowing `io_registry` module below (repo-wide "silent rebind" hazard this ticket tracks —
/// that module returns `&[&ComposerEntry]`, a different type, and is left in place as orphaned
/// dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). The baseline subset's
/// `SubsetValidator` (`🧱️baseline/🚪️io`'s own `JpgBaselineValidator`, previously registered via
/// `⚙️engine::register()`'s trailing `subsets::baseline::io::register()` call) is re-derived here
/// via `subset_validator_entry_of::<JpgBaselineValidator>()` rather than reused from that file's
/// own private `validator_entry()` cache (not `pub`) — same erasure helper, fresh instance, same
/// registry effect. `register_schema_specs()` (this engine's own, real, `#[cfg]`-unconditional
/// `pub fn`) is deliberately dropped, not re-wired: its body is a genuine no-op (`{}`, see that
/// fn's own doc comment — `JpgSnapshot`/`JpgDiff`/`JpgMutation` all fail `dsl`'s derive machinery
/// on hand-rolled fields, so it never calls `dsl::registry::register_schema_spec` at all), so
/// dropping the call changes zero runtime behaviour. `⚙️engine` itself is untouched — this only
/// REFERENCES what it already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("jpg", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::v_jfif_1_01::subsets::document::schema::inferences::jpg_artifact_inference_descriptor()])
        .composers(crate::standards::v_jfif_1_01::subsets::baseline::io::io_registry::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<JpgSnapshot, JpgMutation>(STDIO_JPG_DOCUMENT_SCHEMA, JPG_ANY_DIALECT)
        .document_codec_bare::<JpgSnapshot, JpgMutation>(STDIO_JPG_DOCUMENT_SCHEMA, JPG_BASELINE_DIALECT)
        .try_build()
}

/// 🛡️ Re-derives the 🧱️baseline subset's `SubsetValidatorEntry` — see `declaration()`'s own doc for
/// why this calls `subset_validator_entry_of` directly instead of reusing the private cache in
/// `🧱️baseline/🚪️io`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declared_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<crate::standards::v_jfif_1_01::subsets::baseline::io::JpgBaselineValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.jpg",
                    extension: Some("jpg"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.jpg"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.jpg.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.jpg.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.jpg.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.jpg.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.jpg.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.jpg.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.jpg.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.jpg.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems10

//#region 🧩️ArtifactItems11
pub mod artifact_items_11 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "jpg", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems11
