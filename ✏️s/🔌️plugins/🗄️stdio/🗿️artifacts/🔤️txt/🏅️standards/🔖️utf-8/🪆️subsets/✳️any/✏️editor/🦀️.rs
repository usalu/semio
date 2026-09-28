//! ✏️ Txt editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.txt@utf-8/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TextWindowKit`), replacing the document through the direct line, line-ending, and trailing-newline mutations
//! — a `replace-text` command is inherently whole-buffer, so per-line `InsertLine`/`SetLine` are not
//! reachable through this window (documented, not silently dropped: a future line-addressable editor
//! could target those directly).

use crate::editor::txt::modes::edit;
use crate::editor::txt::modes::edit::windows::main;
use crate::schema::mutation_support::txt_usize_to_u32;
use crate::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetLineEndingMutation, SetTrailingNewlineMutation};
use crate::{TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView,
    Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
    ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const TXT_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum TxtEditorCommand {
    ReplaceText {
        revision: String,
        text: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

/// 🔤️ Hand-rolled hex codec — `OpText::print_op` must be one line, and `ReplaceText` carries
/// arbitrary multi-line/UTF-8 text, so every byte is hex-escaped rather than attempting a
/// space/newline escaping scheme.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    let bytes = text.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err("odd-length hex string".into());
    }
    fn nibble(byte: u8) -> Result<u8, String> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err(format!("non-hex byte 0x{byte:02x}")),
        }
    }
    bytes.chunks_exact(2).map(|pair| Ok((nibble(pair[0])? << 4) | nibble(pair[1])?)).collect()
}

impl protocol::OpText for TxtEditorCommand {
    fn print_op(&self) -> String {
        match self {
            TxtEditorCommand::ReplaceText { revision, text } => format!("replace-text revision={} text={}", hex_encode(revision.as_bytes()), hex_encode(text.as_bytes())),
            TxtEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            TxtEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(hex) = line.strip_prefix("active-example id=") {
            let bytes = hex_decode(hex).map_err(|error| store::TextError::new(format!("txt editor command: bad hex {error}"), dsl::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("txt editor command: bad utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(TxtEditorCommand::SetActiveExample { example_id });
        }
        if let Some(hex) = line.strip_prefix("snapshot-edit event=") {
            let bytes = hex_decode(hex).map_err(|error| store::TextError::new(format!("txt editor command: bad snapshot edit hex {error}"), dsl::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| store::TextError::new(format!("txt editor command: bad snapshot edit {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(TxtEditorCommand::EditSnapshot { event });
        }
        let rest = line.strip_prefix("replace-text revision=").ok_or_else(|| store::TextError::new(format!("txt editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let (revision, text) = rest.split_once(" text=").ok_or_else(|| store::TextError::new("txt editor command: missing text", dsl::TextSpan::at(1, 1)))?;
        let revision = String::from_utf8(hex_decode(revision).map_err(|error| store::TextError::new(format!("txt editor command: bad revision hex {error}"), dsl::TextSpan::at(1, 1)))?)
            .map_err(|error| store::TextError::new(format!("txt editor command: bad revision utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
        let text = String::from_utf8(hex_decode(text).map_err(|error| store::TextError::new(format!("txt editor command: bad text hex {error}"), dsl::TextSpan::at(1, 1)))?)
            .map_err(|error| store::TextError::new(format!("txt editor command: bad text utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
        Ok(TxtEditorCommand::ReplaceText { revision, text })
    }
}

impl protocol::OpBinary for TxtEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TextWindowKit`
    /// mints `replace-text`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = TXT_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TextWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const TXT_KIT_ACTION_ID: &str = "textEdit";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `replace-text`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `replace-text` with
/// `interactive-job.missing-factory`.
const TXT_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, TXT_KIT_ACTION_ID];
const TXT_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    TXT_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const TXT_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.txt.tool-command.v1";
/// 📏️ `replace-text` carries the whole buffer, so the wire bound is the largest document this route
/// admits — kept under the guest's 64 KiB contiguous-request ceiling.
const TXT_RETAINED_RAW_BYTES: usize = 16 * 1_024 * 1_024;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `replace-text` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const TXT_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: TXT_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(TXT_RETAINED_RAW_BYTES, 4_096, 1, TXT_RETAINED_RAW_BYTES, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_example_snapshot(example_id: &str) -> TxtSnapshot {
    if example_id == crate::examples::demo::ID {
        <TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        TxtSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<TxtEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(TxtEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TxtEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        TXT_KIT_ACTION_ID => {
            Ok(TxtEditorCommand::ReplaceText { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?, text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? })
        }
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.txt.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, replace-text)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_command_id(command: &TxtEditorCommand) -> &'static str {
    match command {
        TxtEditorCommand::ReplaceText { .. } => TXT_KIT_ACTION_ID,
        TxtEditorCommand::EditSnapshot { event } => event.action_id(),
        TxtEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_extent(_command: &TxtEditorCommand, _snapshot: &TxtSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `replace-text` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_emit(command: &TxtEditorCommand, snapshot: &TxtSnapshot, canonical_revision: Option<[u8; 32]>) -> Result<Emit<TxtMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let (revision, text) = match command {
        TxtEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&txt_example_snapshot(example_id), STDIO_TXT_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() })
        }
        TxtEditorCommand::ReplaceText { revision, text } => (revision, text),
        TxtEditorCommand::EditSnapshot { .. } => return Err(Fault::from("stdio-txt-snapshot-edit-routed-to-native-reducer")),
    };
    let current_revision = canonical_revision.map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), semio_s_artifact_stdio_contract::window_kit_canonical_revision);
    if revision != &current_revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.txt.stale-text-edit"), "the text document changed while this draft was open"));
    }
    let mut next = TxtSnapshot::from_body(text);
    next.schema.clone_from(&snapshot.schema);
    if &next == snapshot {
        return Ok(Emit::default());
    }
    let mut mutations = Vec::new();
    for index in (0..snapshot.lines.len()).rev() {
        let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
        mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index }));
    }
    for (index, text) in next.lines.iter().cloned().enumerate() {
        let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
        mutations.push(TxtMutation::InsertLine(InsertLineMutation { index, text }));
    }
    if next.trailing_newline != snapshot.trailing_newline {
        mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: next.trailing_newline }));
    }
    if next.line_ending != snapshot.line_ending {
        mutations.push(TxtMutation::SetLineEnding(SetLineEndingMutation { value: next.line_ending }));
    }
    Ok(Emit { artifact_mutations: mutations, description: Some("Replace text".into()), ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_reduce(
    command: &TxtEditorCommand,
    snapshot: &TxtSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TxtEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<TxtMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    txt_emit(command, snapshot, Some(operation.canonical_base_revision))
}

struct TxtRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl TxtRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: TXT_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for TxtRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TxtEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TxtEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        TXT_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        txt_retained_contract()
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
        if input.declared_bytes() > TXT_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio txt retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for TxtRetainedCommandJobFactory {
    type Owner = EditorApp<TxtEditor>;
    const TOOL_IDS: &'static [&'static str] = TXT_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TXT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TXT_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TxtEditor;

impl ArtifactEditor for TxtEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TxtEditorCommand;

    const DIALECT: Dialect = TXT_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TXT_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<TxtEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.txt@utf-8/*#editor",
        artifact_schema: "stdio.txt",
        factory: "TxtRetainedCommandJobFactory",
        factory_type: TxtRetainedCommandJobFactory,
        contract: txt_retained_contract(),
        tools: ["setActiveExample", "textEdit"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(TxtRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !TXT_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if txt_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-txt-retained-command-tool-mismatch"));
        }
        let tool_id = txt_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
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
            txt_command_id,
            TXT_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, txt_retained_reduce, txt_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-txt-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
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

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_TXT_DOCUMENT_SCHEMA, operation, generation))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        txt_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        txt_command_from_action(action, args)
    }

    fn initial_snapshot() -> TxtSnapshot {
        TxtSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            TxtEditorCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            _ => txt_emit(command, doc.snapshot, doc.operation_optional().map(|operation| operation.canonical_base_revision)),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision =
                    doc.render_operation().map_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot), |operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                main::render(doc.snapshot, view_state.locale, &revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.txt@utf-8/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for TxtEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            TxtEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let next = semio_s_artifact_stdio_contract::editing::apply_snapshot_edit(snapshot, event).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(error.code), error.to_string()))?;
        if next.schema != snapshot.schema {
            return Err(Fault::from("stdio-txt-schema-is-immutable"));
        }
        let mut emit = txt_emit(&TxtEditorCommand::ReplaceText { revision: semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot), text: next.to_body() }, snapshot, None)?;
        emit.description = Some("Edit text details".into());
        Ok(emit)
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_txt_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(TXT_EDITOR_DIALECT)
        .document(["semio", "stdio", "txt"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("type")
        .mode_def(edit::definition())
        .default_mode_id(edit::TXT_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Text"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
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
