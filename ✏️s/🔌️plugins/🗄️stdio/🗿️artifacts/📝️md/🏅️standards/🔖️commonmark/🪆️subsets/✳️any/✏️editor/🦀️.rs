//! ✏️ `md` editor (any) — `ArtifactEditor` surface built on the frozen
//! `TextWindowKit` window kit (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6).
//! Emits the frozen `textEdit` action by `splices`: the CommonMark source the main window edits is an explicit draft, and one Apply carries
//! the change set the editor made to it (ranges of Unicode scalars of the source) as ONE `splice-source` mutation (design §13.2 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); the sparse diff reparses only the blocks the ranges touch. Replacing the whole document is the details pane's load.
//! MUST NOT be reached by the sibling `viewer` module (`policyViewerPurityBreaches`).

use crate::editor::md::modes::edit;
use crate::editor::md::modes::edit::windows::main;
use crate::standards::v_commonmark::subsets::any::schema::mutations::{splice_source, MdMutation};

use crate::standards::v_commonmark::subsets::any::schema::snapshot::MdSnapshot;
use crate::{MD_DIALECT, STDIO_MD_DOCUMENT_SCHEMA};
use semio_framework_2d_compute::EngineHandles;
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
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Command
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum MdEditCommand {
    SpliceText {
        revision: String,
        splices: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpBinary for MdEditCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TextWindowKit`
    /// mints `textEdit`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = MD_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = semio_framework_pack_json::parse_bytes(bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| protocol::ProtocolError::Malformed { what: "md-edit-command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TextWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
pub const MD_KIT_ACTION_ID: &str = "textEdit";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `textEdit`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `textEdit` with
/// `interactive-job.missing-factory`.
const MD_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, MD_KIT_ACTION_ID];
const MD_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    MD_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const MD_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.md.tool-command.v1";
/// 📏️ `textEdit` carries the draft's change set, whose inserted text is bounded by the largest document this route
/// admits — kept under the guest's 64 KiB contiguous-request ceiling.
const MD_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `textEdit` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const MD_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: MD_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(MD_RETAINED_RAW_BYTES, 4_096, 1, MD_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md_example_snapshot(example_id: &str) -> MdSnapshot {
    if example_id == crate::examples::demo::ID {
        <MdSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        MdSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<MdEditCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(MdEditCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(MdEditCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        MD_KIT_ACTION_ID => Ok(MdEditCommand::SpliceText { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?, splices: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "splices")? }),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.md.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, replace-text)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_command_id(command: &MdEditCommand) -> &'static str {
    match command {
        MdEditCommand::SpliceText { .. } => MD_KIT_ACTION_ID,
        MdEditCommand::EditSnapshot { event } => event.action_id(),
        MdEditCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_extent(_command: &MdEditCommand, _snapshot: &MdSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `textEdit` by `splices` becomes ONE `splice-source` mutation carrying the editor's ranges verbatim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md_emit(command: &MdEditCommand, snapshot: &MdSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<MdMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let fault = |code: &'static str, message: String| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), message);
    match command {
        MdEditCommand::SpliceText { revision, splices } => {
            let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
            if revision != &current_revision {
                return Err(fault("stdio.md.stale-text-edit", "the markdown document changed while this draft was open".to_string()));
            }
            let ranges = semio_s_artifact_stdio_contract::draft_splices_from_json(splices)?;
            if ranges.is_empty() {
                return Ok(Emit::default());
            }
            let range_fault = |_| fault("stdio.md.splice-out-of-range", "a range of the change set exceeds the supported source size".to_string());
            let mutation = MdMutation::SpliceSource(splice_source::SpliceSource {
                splices: ranges.iter().map(|range| Ok(splice_source::SourceSplice { offset: u32::try_from(range.offset)?, delete: u32::try_from(range.delete)?, insert: range.insert.clone() })).collect::<Result<Vec<_>, std::num::TryFromIntError>>().map_err(range_fault)?,
            });
            let outcome = <MdMutation as protocol::Mutation<MdSnapshot>>::diff(&mutation, snapshot);
            if let Some(message) = outcome.messages().first() {
                return Err(fault("stdio.md.splice-unrepresentable", message.message.to_string()));
            }
            Ok(if outcome.diff() == &crate::schema::diff::MdDiff::default() { Emit::default() } else { Emit::mutations(vec![mutation]) })
        }
        MdEditCommand::EditSnapshot { .. } => Err(Fault::from("stdio-md-snapshot-edit-routed-to-native-reducer")),
        MdEditCommand::SetActiveExample { example_id } => Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&md_example_snapshot(example_id), STDIO_MD_DOCUMENT_SCHEMA)], ..Default::default() }),
    }
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_retained_reduce(
    command: &MdEditCommand,
    snapshot: &MdSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<MdEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<MdMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    md_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct MdRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl MdRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: MD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for MdRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<MdEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<MdEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        MD_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        md_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > MD_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio md retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for MdRetainedCommandJobFactory {
    type Owner = EditorApp<MdEditor>;
    const TOOL_IDS: &'static [&'static str] = MD_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MD_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = MD_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes


//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct MdEditor;

impl ArtifactEditor for MdEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = MdSnapshot;
    type Mutation = MdMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = MdEditCommand;

    const DIALECT: Dialect = MD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_MD_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.md@commonmark", extension: ".md", media_type: "text/markdown", binary: false })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        Ok(snapshot.to_text().into_bytes())
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))?;
        Ok(MdSnapshot::from_text(text))
    }

    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<MdEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.md@commonmark/*#editor",
        artifact_schema: "stdio.md",
        factory: "MdRetainedCommandJobFactory",
        factory_type: MdRetainedCommandJobFactory,
        contract: md_retained_contract(),
        tools: ["setActiveExample", "textEdit"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(MdRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !MD_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if md_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-md-retained-command-tool-mismatch"));
        }
        let tool_id = md_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
            retained: request.retained,
        };
        let payload = ArtifactRetainedCommandPayload::new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            md_command_id,
            MD_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, md_retained_reduce, md_retained_extent)),
        );
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        md_command_id(command)
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        md_command_from_action(action, args)
    }

    fn initial_snapshot() -> Self::Snapshot {
        MdSnapshot::default()
    }

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
            MdEditCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            _ => md_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision)),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, &revision, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.md@commonmark/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for MdEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            MdEditCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &crate::editor::md::edit_rules::EDIT_RULES
    }
    fn snapshot_edit_special(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        crate::editor::md::edit_rules::resolve(snapshot, event)
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_md_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(MD_DIALECT)
        .document(["semio", "md"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Markdown"))
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

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_MD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.md".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Md", "Md"),
        source_format: STDIO_MD_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.md.v1", artifact: "md", kind: artifact_kind, codec: native_codec }]
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

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_MD_DOCUMENT_SCHEMA};
use crate::{artifact_kind};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
pub use crate::schema::mutations::MdMutation;
pub use crate::schema::snapshot::MdSnapshot;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling `crate::engine::register()` imperatively before
/// `Plugin::builder` was even constructed, mirroring the `🔋️energy`/`🗒️note` exemplars. `crate::
/// artifacts::md::standards::v_commonmark::engine::register()` (stdio's own `⚙️engine` — UNTOUCHED,
/// per this ticket's rule that stdio's engines stay a public surface other plugins reach into) called,
/// in call order: `io_registry::register()` → `.composers(...)` below, the same
/// `standards::v_commonmark::engine::io_registry::entries()` this artifact's own root `io_registry`
/// module already wraps (module path collapses the `subsets::any` folder level away — same shape as
/// `💾️binary`/`🔤️txt`/`🔣️json`, not `📰️xml`/`📊️csv`'s deeper `subsets::any::engine`);
/// `register_artifact_schema()`/`register_artifact_inferences()` → `.schema(...)`/`.inferences(...)`;
/// `register_pilot_languages()` → `.languages(...)`, replicated verbatim below (same `OnceLock`-leak
/// shape `🔋️energy`'s own `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const
/// fn`-constructible); `register_document_codec` → `.document_codec_bare::<MdSnapshot,
/// MdMutation>(...)`. This artifact's `register_pilot_languages()` doc already states
/// `register_schema_spec` is "deliberately NOT called here" — so unlike `🔤️txt`/`💾️binary` there is no
/// uncovered call left behind. `standards::v_commonmark::engine::register()` itself is left in place,
/// now orphaned/uncalled — deleting it means editing `⚙️engine/`, off-limits here.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("md", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::schema::md_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::v_commonmark::subsets::any::schema::inferences::md_artifact_inference_descriptor()])
        .composers(crate::standards::v_commonmark::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<MdSnapshot, MdMutation>(STDIO_MD_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of `standards::v_commonmark::
/// subsets::any::engine::register_pilot_languages()`'s five `LanguageSpec`s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.md",
                    extension: Some("md"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::standards::v_commonmark::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_commonmark::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_commonmark::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_commonmark::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.md"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.md.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::standards::v_commonmark::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_commonmark::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::standards::v_commonmark::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_commonmark::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.md.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.md.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::standards::v_commonmark::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::standards::v_commonmark::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.md.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.md.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_commonmark::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_commonmark::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.md.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.md.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::standards::v_commonmark::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::standards::v_commonmark::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.md.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems9

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "md", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems10

//#region 🧩️ArtifactItems11
pub mod artifact_items_11 {
use crate::{artifact_kind};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems11
