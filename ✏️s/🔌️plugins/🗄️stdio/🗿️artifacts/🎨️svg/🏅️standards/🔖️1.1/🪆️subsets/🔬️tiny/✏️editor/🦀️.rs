//! ✏️ `svg` editor (tiny) — `ArtifactEditor` surface built on the frozen
//! `ImageWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! SVG has no pixel buffer: `set-pixel-region` carries the current drawing to the one parsed from its DSL text as the concrete element and attribute kinds that replace the root element's content (`parse_dsl`, then `edit_rules::region`) — not a pixel edit and never a differenced edit.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::svg_tiny::modes::edit;
use crate::editor::svg_tiny::modes::edit::windows::main;
use crate::standards::v1_1::subsets::tiny::schema::mutations::edit_rules;
use crate::standards::v1_1::subsets::tiny::schema::mutations::SvgTinyMutation;
use crate::standards::v1_1::subsets::tiny::schema::snapshot::SvgSnapshot;
use crate::{STDIO_SVG_DOCUMENT_SCHEMA, SVG_TINY_DIALECT};
use semio_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactOwnedToolJobFactory;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactStoreInitializationJob;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::EditorApp;
use semio_plugin::InteractiveJobClassification;
use semio_plugin::ToolExecutionContract;
use semio_plugin::ToolFactoryKey;
use semio_plugin::ToolJobFactory;
use semio_plugin::ToolJobFactoryError;
use semio_plugin::ToolOperationSpec;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_ui_locale::Label;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_stdio_contract::editing;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum SvgTinyEditCommand {
    SetPixelRegion { source: String },
    /// ✂️ Drops every element and presentation attribute SVG Tiny 1.1 excludes.
    StripNonTiny,
    /// 🏷️ Sets (empty text clears) the root's `baseProfile` and `version` declaration.
    StampBaseProfile { base_profile: String, version: String },
    /// 🎬️ Navbar example picker payload.
    SetActiveExample { example_id: String },
    EditSnapshot { event: editing::SnapshotEditEvent },
}

impl protocol::OpBinary for SvgTinyEditCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = STDIO_SVG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_tiny-edit-command", offset: 0, detail: error.to_string() })?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "svg_tiny-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command


pub const STRIP_NON_TINY_ACTION_ID: &str = "stripNonTiny";
pub const STAMP_BASE_PROFILE_ACTION_ID: &str = "stampBaseProfile";
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, STRIP_NON_TINY_ACTION_ID, STAMP_BASE_PROFILE_ACTION_ID];
const STDIO_SVG_DOCUMENT_SCHEMA_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    STRIP_NON_TINY_ACTION_ID,
    STAMP_BASE_PROFILE_ACTION_ID,
    editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA: &str = "stdio.svg.tool-command.v1";
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES: usize = 8_192;
const STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
const STDIO_SVG_DOCUMENT_SCHEMA_STRIP_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: STRIP_NON_TINY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
const STDIO_SVG_DOCUMENT_SCHEMA_STAMP_CONTRACT: ArtifactToolPublicationContract = ArtifactToolPublicationContract { tool_id: STAMP_BASE_PROFILE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] };
fn svg_tiny_editor_example_snapshot(example_id: &str) -> SvgSnapshot {
    if example_id == crate::examples::demo::ID { <SvgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default() } else { SvgSnapshot::default() }
}
fn svg_tiny_editor_command_id(command: &SvgTinyEditCommand) -> &'static str {
    if let SvgTinyEditCommand::EditSnapshot { event } = command { return event.action_id(); }
    match command {
        SvgTinyEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
        SvgTinyEditCommand::StripNonTiny => STRIP_NON_TINY_ACTION_ID,
        SvgTinyEditCommand::StampBaseProfile { .. } => STAMP_BASE_PROFILE_ACTION_ID,
        _ => "other",
    }
}
fn svg_tiny_editor_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<SvgTinyEditCommand, Fault> {
    if editing::is_snapshot_edit_action(action) { return editing::snapshot_edit_event_from_action(action, args).and_then(|event| event.map(|event| SvgTinyEditCommand::EditSnapshot { event }).ok_or_else(|| Fault::from(format!("action '{action}' is not a snapshot edit")))); }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(SvgTinyEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        STRIP_NON_TINY_ACTION_ID => Ok(SvgTinyEditCommand::StripNonTiny),
        STAMP_BASE_PROFILE_ACTION_ID => {
            let text = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| value.as_str()).unwrap_or_default().to_string();
            Ok(SvgTinyEditCommand::StampBaseProfile { base_profile: text("baseProfile"), version: text("version") })
        }
        _ => Err(Fault::from(format!("action '{action}' is not an SVG Tiny editor action"))),
    }
}
fn svg_tiny_editor_retained_extent(command: &SvgTinyEditCommand, _snapshot: &SvgSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, SvgTinyEditCommand::SetActiveExample { .. } | SvgTinyEditCommand::StripNonTiny | SvgTinyEditCommand::StampBaseProfile { .. }).then_some(1)
}
fn svg_tiny_editor_retained_reduce(command: &SvgTinyEditCommand, _snapshot: &SvgSnapshot, _config: &NoConfig, _history: &semio_plugin::HistoryView, _interaction: &protocol::InteractionState, _hover: &semio_plugin::app::InteractionHoverState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SvgTinyEditor>>>, _operation: &AppOperationContext) -> Result<Emit<SvgTinyMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        SvgTinyEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&svg_tiny_editor_example_snapshot(example_id), STDIO_SVG_DOCUMENT_SCHEMA)], ..Default::default() }),
        SvgTinyEditCommand::StripNonTiny => Ok(Emit::mutations(vec![svg_tiny_strip_mutation()])),
        SvgTinyEditCommand::StampBaseProfile { base_profile, version } => Ok(Emit::mutations(vec![svg_tiny_stamp_mutation(base_profile, version)])),
        _ => Err(Fault::from("stdio-example-retained-route-mismatch")),
    }
}
fn svg_tiny_strip_mutation() -> SvgTinyMutation {
    SvgTinyMutation::StripNonTiny(crate::standards::v1_1::subsets::tiny::schema::mutations::strip_non_tiny::StripNonTiny {})
}
fn svg_tiny_stamp_mutation(base_profile: &str, version: &str) -> SvgTinyMutation {
    let text = |value: &str| (!value.is_empty()).then(|| value.to_string());
    SvgTinyMutation::StampBaseProfile(crate::standards::v1_1::subsets::tiny::schema::mutations::stamp_base_profile::StampBaseProfile { base_profile: text(base_profile), version: text(version), base_profile_index: None, version_index: None })
}
struct SvgTinyEditorExampleFactory { keys: Vec<ToolFactoryKey> }
impl SvgTinyEditorExampleFactory { fn new(controller_id: &str) -> Self { Self { keys: STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() } } }
impl ToolJobFactory for SvgTinyEditorExampleFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SvgTinyEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SvgTinyEditor>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500) }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, checkpoint_resume: false);
}
impl ArtifactOwnedToolJobFactory for SvgTinyEditorExampleFactory {
    type Owner = EditorApp<SvgTinyEditor>;
    const TOOL_IDS: &'static [&'static str] = STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_CONTRACT, STDIO_SVG_DOCUMENT_SCHEMA_STRIP_CONTRACT, STDIO_SVG_DOCUMENT_SCHEMA_STAMP_CONTRACT];
}
//#region 🔖️Editor
fn svg_tiny_editor_validate_natural_snapshot(snapshot: &SvgSnapshot) -> Result<(), semio_plugin::MediaError> {
    snapshot.validate_natural().map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error))?;
    if let Some(diagnostic) = crate::standards::v1_1::subsets::tiny::schema::conformance::check_svg_tiny_conformance(snapshot)
        .into_iter()
        .find(|diagnostic| matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal))
    {
        return Err(semio_plugin::MediaError::Payload("artifact:native".into(), diagnostic.message));
    }
    Ok(())
}

#[derive(Default, Clone, Copy)]
pub struct SvgTinyEditor;

impl ArtifactEditor for SvgTinyEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = SvgSnapshot;
    type Mutation = SvgTinyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SvgTinyEditCommand;

    const DIALECT: Dialect = SVG_TINY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_SVG_DOCUMENT_SCHEMA;

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec { format_kind: "s.stdio.svg@1.1", extension: ".svg", media_type: "image/svg+xml", binary: false })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        svg_tiny_editor_validate_natural_snapshot(snapshot)?;
        snapshot.export_utf8().map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        let snapshot = SvgSnapshot::import_utf8(bytes).map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error))?;
        svg_tiny_editor_validate_natural_snapshot(&snapshot)?;
        Ok(snapshot)
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<SvgTinyEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/✏️editor/🦀️.rs",
        controller: "s.stdio.svg@1.1/tiny#editor",
        artifact_schema: "stdio.svg",
        factory: "SvgTinyEditorExampleFactory",
        factory_type: SvgTinyEditorExampleFactory,
        contract: ToolExecutionContract::bounded_first_step(STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 64, 1, 65_536, 7_500),
        tools: ["setActiveExample", "stripNonTiny", "stampBaseProfile"]
    }
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        registry.register(SvgTinyEditorExampleFactory::new(registry.controller_id()))?;
        editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }
    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if editing::is_snapshot_edit_action(&original.tool_id) { return editing::build_snapshot_edit_tool_job::<Self>(request, output, cx); }
        if !STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_TOOL_IDS.contains(&original.tool_id.as_str()) { return Ok(false); }
        if svg_tiny_editor_command_id(&original.command) != original.tool_id { return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "stdio-example-tool-mismatch").into()); }
        let tool = svg_tiny_editor_command_id(&original.command);
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, svg_tiny_editor_command_id, STDIO_SVG_DOCUMENT_SCHEMA_EXAMPLE_BYTES, 1, false, |_request| BoundedArtifactCommandWork::new(tool, svg_tiny_editor_retained_reduce, svg_tiny_editor_retained_extent))
    }
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }
    fn build_document_store_initialization_job(envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_plugin::bounded_document_store_initialization_job::<_,_,::semio_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, STDIO_SVG_DOCUMENT_SCHEMA, operation, generation, actor))
    }
    fn command_id(command: &Self::Command) -> &'static str { svg_tiny_editor_command_id(command) }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref(); svg_tiny_editor_command_from_action(action, args) }

    fn reference_birth()->Option<fn()->Result<Self::Snapshot,semio_plugin::Fault>>{Some(||{
        Ok(SvgSnapshot::default())
    })}

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            SvgTinyEditCommand::EditSnapshot { event } => <Self as editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            SvgTinyEditCommand::SetActiveExample { example_id } => Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&svg_tiny_editor_example_snapshot(example_id), STDIO_SVG_DOCUMENT_SCHEMA)],
                ..Default::default()
            }),
            SvgTinyEditCommand::StripNonTiny => Ok(Emit::mutations(vec![svg_tiny_strip_mutation()])),
            SvgTinyEditCommand::StampBaseProfile { base_profile, version } => Ok(Emit::mutations(vec![svg_tiny_stamp_mutation(base_profile, version)])),
            SvgTinyEditCommand::SetPixelRegion { source } => match <SvgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(source) {
                Ok(snapshot) => edit_rules::region(doc.snapshot, &snapshot).map(Emit::mutations).map_err(Fault::from),
                Err(_) => Ok(Emit::default()),
            },
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_plugin::built_to_component_tree),
            editing::SNAPSHOT_DETAILS_BODY_KEY => editing::render_snapshot_details(doc, view_state.locale, "s.stdio.svg@1.1/tiny#editor", &semio_plugin::TreeWindows::for_body(view_state, editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor


impl editing::SnapshotEditingEditor for SvgTinyEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { SvgTinyEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
    fn snapshot_edit_rules() -> &'static editing::EditRules {
        &edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot).map_err(|error| Fault::from(error.to_string()))
    }
}

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_svg_tiny_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(SVG_TINY_DIALECT).document(["semio", "svg"]).icon_id("image").mode_def(edit::definition()).default_mode_id(edit::MODE_ID).window_kind_def(main::definition()).window_kind_def(editing::snapshot_details_window_definition()).default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main")).action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_with(semio_plugin::ActionDefinition::new(STRIP_NON_TINY_ACTION_ID, semio_ui_locale::LocalizedLabel::native("Strip to SVG Tiny", "Auf SVG Tiny reduzieren"), semio_plugin::ActionKind::Mutation, "scissors"))
        .action_describe(STRIP_NON_TINY_ACTION_ID, semio_ui_locale::LocalizedLabel::native("Removes every element and presentation attribute that SVG Tiny 1.1 excludes; undo restores them at their exact positions.", "Entfernt jedes Element und Darstellungsattribut, das SVG Tiny 1.1 ausschließt; Rückgängig stellt sie an ihren genauen Positionen wieder her."))
        .action_interactive_job(STRIP_NON_TINY_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_with(semio_plugin::ActionDefinition::new(STAMP_BASE_PROFILE_ACTION_ID, semio_ui_locale::LocalizedLabel::native("Stamp Base Profile", "Basisprofil stempeln"), semio_plugin::ActionKind::Mutation, "tag"))
        .action_args(STAMP_BASE_PROFILE_ACTION_ID, vec![
            semio_plugin::ActionArgDef::text("baseProfile", semio_ui_locale::LocalizedLabel::native("Base profile", "Basisprofil")).min_length(0),
            semio_plugin::ActionArgDef::text("version", semio_ui_locale::LocalizedLabel::native("Version", "Version")).min_length(0),
        ])
        .action_describe(STAMP_BASE_PROFILE_ACTION_ID, semio_ui_locale::LocalizedLabel::native("Sets the root element's baseProfile and version declaration; an empty value clears it.", "Setzt die baseProfile- und version-Angabe des Wurzelelements; ein leerer Wert löscht sie."))
        .action_interactive_job(STAMP_BASE_PROFILE_ACTION_ID, InteractiveJobClassification::Migrated)
        ;
    editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
