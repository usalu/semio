//! 🗜️ Deflate editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.deflate@rfc1950/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `DeflateSnapshot` is a typed RFC1950
//! zlib container: CMF/FLG header fields plus an opaque decompressed `payload`. One window,
//! `🪟️main` (`TextWindowKit`), renders the header fields as an editable `key=value` summary; its
//! `replace-text` action funnels through the one typed command this surface declares,
//! `DeflateEditorCommand::ReplaceText`, which parses the summary back into the header leaves it changed — `SetCompressionParams`
//! and/or `SetPresetDictionary`, nothing when the Apply changed neither (see the window's own doc comment for why `payload`
//! itself is never shown or parsed here — a compressed byte stream has no honest text form).

use crate::editor::deflate::modes::edit;
use crate::editor::deflate::modes::edit::windows::main;
use crate::schema::mutations::{set_compression_params, set_payload, set_preset_dictionary};
use crate::{DeflateMutation, DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
#[cfg(test)]
use semio_framework_plugin::Component;
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
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.deflate@rfc1950/*`, verified against this
/// artifact's own `🏅️standards/🔖️rfc1950/🪆️subsets/✳️any` location on disk. No reusable const
/// exists on the artifact's own root `🦀️.rs` (checked before adding this), so it is
/// inlined here and reused for both `impl ArtifactEditor` and the manifest below.
pub const DEFLATE_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum DeflateEditorCommand {
    #[dsl(key = "replace-deflate-text")]
    ReplaceText { text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for DeflateEditorCommand {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for DeflateEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DeflateEditorCommand, ["textEdit"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️TextParse
/// 📐️ Parses `render()`'s `key=value` summary back into `(method, window_bits, level_hint, dict_id)`.
/// `#`-prefixed and blank lines are ignored (the payload byte-count comment). `None` on any missing
/// or malformed required key or out-of-schema header nibble — the caller rejects the whole command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_header_summary(text: &str) -> Option<(u8, u8, crate::schema::snapshot::DeflateLevelHint, Option<u32>)> {
    let mut fields = std::collections::BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=')?;
        let key = key.trim();
        if !matches!(key, "method" | "windowBits" | "levelHint" | "presetDictionary") || fields.insert(key, value.trim()).is_some() {
            return None;
        }
    }
    let method = fields.get("method")?.parse::<u8>().ok()?;
    let window_bits = fields.get("windowBits")?.parse::<u8>().ok()?;
    if method > 15 || window_bits > 15 {
        return None;
    }
    let level_hint = main::parse_level_hint(fields.get("levelHint")?)?;
    let dict_id = match fields.get("presetDictionary").copied() {
        None | Some("none") => None,
        Some(other) => Some(other.parse::<u32>().ok()?),
    };
    Some((method, window_bits, level_hint, dict_id))
}
//#endregion 🔖️TextParse

//#region 🧵️RetainedTextRoute
const DEFLATE_TEXT_ACTION_ID: &str = "textEdit";
const DEFLATE_TEXT_TOOL_IDS: &[&str] = &[DEFLATE_TEXT_ACTION_ID];
const DEFLATE_TEXT_PAYLOAD_SCHEMA: &str = "stdio.deflate.text-edit.v1";
const DEFLATE_TEXT_MAXIMUM_RAW_BYTES: usize = 8 * 1_024;
const DEFLATE_TEXT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: DEFLATE_TEXT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deflate_text_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(DEFLATE_TEXT_MAXIMUM_RAW_BYTES, 64, 1, 4, 7_500)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deflate_command_id(command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DeflateEditorCommand>) -> &'static str {
    semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| DEFLATE_TEXT_ACTION_ID)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deflate_text_emit(command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DeflateEditorCommand>, snapshot: &DeflateSnapshot) -> Result<Emit<DeflateMutation>, Fault> {
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DeflateEditorCommand::ReplaceText { text }) = command else {
        return Err(Fault::from("stdio-deflate-snapshot-edit-routed-to-native-reducer"));
    };
    let (method, window_bits, level_hint, dict_id) = parse_header_summary(text).ok_or_else(|| {
        Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.deflate.invalid-summary"), "The compression summary must contain valid method, windowBits, levelHint, and presetDictionary fields.")
    })?;
    Ok(Emit::mutations(deflate_header_mutations(snapshot, method, window_bits, level_hint, dict_id)))
}

/// 🧮️ The header leaves that carry `base`'s header to the given one: `set-compression-params` when the method, window or level
/// hint moved, `set-preset-dictionary` when the dictionary id did, nothing when neither.
fn deflate_header_mutations(base: &DeflateSnapshot, method: u8, window_bits: u8, level_hint: crate::schema::snapshot::DeflateLevelHint, dict_id: Option<u32>) -> Vec<DeflateMutation> {
    let params =
        ((method, window_bits, level_hint) != (base.compression_method, base.window_bits, base.compression_level_hint)).then(|| DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method, window_bits, level_hint }));
    let dictionary = (dict_id != base.dict_id).then(|| DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id }));
    params.into_iter().chain(dictionary).collect()
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deflate_text_reduce(
    command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DeflateEditorCommand>,
    snapshot: &DeflateSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<DeflateEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<DeflateMutation>, Fault> {
    deflate_text_emit(command, snapshot)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deflate_text_extent(command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DeflateEditorCommand>, _snapshot: &DeflateSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(_)).then_some(4)
}

struct DeflateTextCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl DeflateTextCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: DEFLATE_TEXT_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for DeflateTextCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<DeflateEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<DeflateEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        DEFLATE_TEXT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        deflate_text_contract()
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
        if input.declared_bytes() > DEFLATE_TEXT_MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio deflate text edit rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for DeflateTextCommandJobFactory {
    type Owner = EditorApp<DeflateEditor>;
    const TOOL_IDS: &'static [&'static str] = DEFLATE_TEXT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DEFLATE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = DEFLATE_TEXT_PUBLICATION_CONTRACTS;
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_deflate_text_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<DeflateEditor>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if request.tool_id != DEFLATE_TEXT_ACTION_ID {
        return Ok(None);
    }
    if deflate_command_id(&request.command) != request.tool_id {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-deflate-text-command-tool-mismatch"));
    }
    let tool_id = deflate_command_id(&request.command);
    let operation = AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id,
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
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
        deflate_command_id,
        DEFLATE_TEXT_MAXIMUM_RAW_BYTES,
        4,
        Box::new(BoundedArtifactCommandWork::new(tool_id, deflate_text_reduce, deflate_text_extent)),
    );
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🧵️RetainedTextRoute

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DeflateEditor;

impl ArtifactEditor for DeflateEditor {
    type Snapshot = DeflateSnapshot;
    type Mutation = DeflateMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DeflateEditorCommand>;

    const DIALECT: Dialect = DEFLATE_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DEFLATE_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.deflate@rfc1950/*#editor",
        artifact_schema: "stdio.deflate",
        preparation: "stdio-deflate-snapshot-edit",
        native: {
            factory_name: "DeflateTextCommandJobFactory",
            factory_type: DeflateTextCommandJobFactory,
            contract: deflate_text_contract(),
            build: build_deflate_text_tool_job,
            tools: ["textEdit"]
        },
    }

    fn command_id(command: &Self::Command) -> &'static str {
        deflate_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "textEdit" => Ok(DeflateEditorCommand::ReplaceText { text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? }),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.deflate.unhandled-action"), format!("unknown deflate editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DeflateSnapshot {
        DeflateSnapshot::default()
    }

    /// ✏️ Parses the whole `key=value` summary and, if every required field is present and valid,
    /// emits the header leaves the Apply changed as one gesture: `SetCompressionParams` when the method, window or level hint
    /// moved, `SetPresetDictionary` when the dictionary id did, nothing when neither. A malformed
    /// summary (missing, out-of-schema, or unparsable required field) returns a fault without a
    /// partial apply.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DeflateEditorCommand::ReplaceText { .. }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        deflate_text_emit(command, doc.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.deflate@rfc1950/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for DeflateEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &crate::editor::deflate::edit_rules::EDIT_RULES
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_deflate_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(DEFLATE_EDITOR_DIALECT)
        .document(["stdio", "deflate"])
        .icon_id("package")
        .mode_def(edit::definition())
        .default_mode_id(edit::DEFLATE_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Compression"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
