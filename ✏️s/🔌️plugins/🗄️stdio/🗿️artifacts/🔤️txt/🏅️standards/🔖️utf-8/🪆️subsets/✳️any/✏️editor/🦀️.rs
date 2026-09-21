//! ✏️ Txt editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.txt@utf-8/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TextWindowKit`), replacing the document through the direct line, line-ending, and trailing-newline mutations
//! — a `replace-text` command is inherently whole-buffer, so per-line `InsertLine`/`SetLine` are not
//! reachable through this window (documented, not silently dropped: a future line-addressable editor
//! could target those directly).

use crate::editor::txt::modes::edit;
use crate::editor::txt::modes::edit::windows::main;
use crate::schema::mutation_support::txt_usize_to_u32;
use crate::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetTrailingNewlineMutation};
use crate::{TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId,
    SubsetId, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};

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
    ReplaceText { text: String },
    /// 🎬️ The navbar example picker's payload — see the `🎬️ExampleSwitch` region below.
    SetActiveExample { example_id: String },
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
    if !text.len().is_multiple_of(2) {
        return Err("odd-length hex string".into());
    }
    (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}

impl protocol::OpText for TxtEditorCommand {
    fn print_op(&self) -> String {
        match self {
            TxtEditorCommand::ReplaceText { text } => format!("replace-text text={}", hex_encode(text.as_bytes())),
            TxtEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(hex) = line.strip_prefix("active-example id=") {
            let bytes = hex_decode(hex).map_err(|error| store::TextError::new(format!("txt editor command: bad hex {error}"), dsl::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("txt editor command: bad utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(TxtEditorCommand::SetActiveExample { example_id });
        }
        let hex = line.strip_prefix("replace-text text=").ok_or_else(|| store::TextError::new(format!("txt editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let bytes = hex_decode(hex).map_err(|error| store::TextError::new(format!("txt editor command: bad hex {error}"), dsl::TextSpan::at(1, 1)))?;
        let text = String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("txt editor command: bad utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
        Ok(TxtEditorCommand::ReplaceText { text })
    }
}

impl protocol::OpBinary for TxtEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The
    /// window-kind verb stays out: it is declared by the framework window kit, not by this app.
    const TOOL_JOB_IDS: &'static [&'static str] = TXT_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️TextSplit
/// 🧮️ Splits a plain `\n`-joined buffer into `(lines, trailing_newline)`. The document's existing
/// `line_ending` convention is preserved rather than re-detected — a plain-text window edit never
/// carries `\r\n` metadata worth trusting.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_text(text: &str) -> (Vec<String>, bool) {
    if text.is_empty() {
        return (Vec::new(), false);
    }
    let trailing = text.ends_with('\n');
    let body = if trailing { &text[..text.len() - 1] } else { text };
    let lines = body.split('\n').map(|line| line.trim_end_matches('\r').to_string()).collect();
    (lines, trailing)
}
//#endregion 🔖️TextSplit

//#region 🎬️ExampleSwitch
/// 🧵️ The ONE app-owned retained route this editor declares. `validate_ui_dispatch_classification`
/// refuses any verb that is not `Migrated`, and `Migrated` only survives the guest's
/// `interactive-job.catalog-incomplete` boot check when this roster, the publication contracts and
/// the `bounded_first_step_tool_proofs!` block below all name the same id.
const TXT_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID];
const TXT_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.txt.tool-command.v1";
const TXT_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`.
const TXT_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] =
    &[ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(TXT_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
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
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TxtEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        other => Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.txt.unhandled-action"),
            format!("action '{other}' is not one of this editor's declared verbs (setActiveExample)"),
        )),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_command_id(command: &TxtEditorCommand) -> &'static str {
    match command {
        TxtEditorCommand::ReplaceText { .. } => "replace-text",
        TxtEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_extent(command: &TxtEditorCommand, _snapshot: &TxtSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, TxtEditorCommand::SetActiveExample { .. }).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_retained_reduce(
    command: &TxtEditorCommand,
    _snapshot: &TxtSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TxtEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<TxtMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TxtEditorCommand::SetActiveExample { example_id } => Ok(Emit {
            effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&txt_example_snapshot(example_id), STDIO_TXT_DOCUMENT_SCHEMA)],
            description: Some(format!("Load example {example_id}")),
            ..Default::default()
        }),
        TxtEditorCommand::ReplaceText { .. } => Err(Fault::from("stdio-txt-retained-route-mismatch")),
    }
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
//#endregion 🎬️ExampleSwitch

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TxtEditor;

impl ArtifactEditor for TxtEditor {
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

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<TxtEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.txt@utf-8/*#editor",
        artifact_schema: "stdio.txt",
        factory: "TxtRetainedCommandJobFactory",
        factory_type: TxtRetainedCommandJobFactory,
        contract: txt_retained_contract(),
        tools: ["setActiveExample"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(TxtRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if !TXT_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if txt_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-txt-retained-command-tool-mismatch"));
        }
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
            Box::new(BoundedArtifactCommandWork::new(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, txt_retained_reduce, txt_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
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
        let text = match command {
            TxtEditorCommand::SetActiveExample { example_id } => {
                return Ok(Emit {
                    effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&txt_example_snapshot(example_id), STDIO_TXT_DOCUMENT_SCHEMA)],
                    description: Some(format!("Load example {example_id}")),
                    ..Default::default()
                })
            }
            TxtEditorCommand::ReplaceText { text } => text,
        };
        let (lines, trailing_newline) = split_text(text);
        let mut mutations = Vec::new();
        if doc.snapshot.trailing_newline {
            mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false }));
        }
        for index in (0..doc.snapshot.lines.len()).rev() {
            let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
            mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index }));
        }
        for (index, text) in lines.into_iter().enumerate() {
            let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
            mutations.push(TxtMutation::InsertLine(InsertLineMutation { index, text }));
        }
        if trailing_newline {
            mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: true }));
        }
        Ok(Emit { artifact_mutations: mutations, description: Some("Replace text".into()), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_txt_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TXT_EDITOR_DIALECT)
        .document(["semio", "stdio", "txt"])
        .icon_id("type")
        .mode_def(edit::definition())
        .default_mode_id(edit::TXT_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
