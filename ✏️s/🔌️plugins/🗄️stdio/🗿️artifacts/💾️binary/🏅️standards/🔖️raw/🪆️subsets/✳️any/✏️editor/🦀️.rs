//! 💾️ Binary editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.binary@raw/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `BinarySnapshot` is the simplest
//! possible document: one opaque `bytes: Vec<u8>` buffer. One window, `🪟️main` (`TextWindowKit`),
//! renders it as a complete paged hex dump; its `textEdit` action funnels through the one typed command
//! this surface declares, `BinaryEditorCommand::ReplaceText`, which parses the hex text back into
//! bytes and emits a whole-buffer `BinaryMutation::ReplaceByteRange` through the artifact-owned
//! retained command factory.

use crate::editor::binary::modes::edit;
use crate::editor::binary::modes::edit::windows::main;
use crate::schema::mutations::{replace_byte_range, set_snapshot};
use crate::{BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
#[cfg(test)]
use semio_framework_plugin::Component;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault,
    InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, ToolExecutionContract,
    ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.binary@raw/*`, verified against this artifact's
/// own `🏅️standards/🔖️raw/🪆️subsets/✳️any` location on disk. No reusable const exists on the
/// artifact's own root `🦀️.rs` (checked before adding this), so it is inlined here and
/// reused for both `impl ArtifactEditor` and the manifest below.
pub const BINARY_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum BinaryEditorCommand {
    #[dsl(key = "replace-binary-text")]
    ReplaceText { text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for BinaryEditorCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for BinaryEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
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
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(BinaryEditorCommand, ["textEdit"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️TextParse
/// 📐️ Parses `render()`'s hex dump back into bytes: strips `#`-prefixed comment lines and
/// whitespace, then decodes the remaining contiguous hex — same convention
/// `BinarySnapshot::parse_dsl` already uses. `None` on odd length or an invalid hex digit — the
/// caller rejects the complete command, never a partial apply.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_hex_dump(text: &str) -> Option<Vec<u8>> {
    let hex: String = text.lines().filter(|line| !line.trim_start().starts_with('#')).collect::<Vec<_>>().join("").chars().filter(|c| !c.is_whitespace()).collect();
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    while let (Some(high), Some(low)) = (chars.next(), chars.next()) {
        let high = high.to_digit(16)?;
        let low = low.to_digit(16)?;
        bytes.push(((high << 4) | low) as u8);
    }
    Some(bytes)
}
//#endregion 🔖️TextParse

//#region 🧵️RetainedTextRoute
const BINARY_TEXT_ACTION_ID: &str = "textEdit";
const BINARY_TEXT_TOOL_IDS: &[&str] = &[BINARY_TEXT_ACTION_ID];
const BINARY_TEXT_PAYLOAD_SCHEMA: &str = "stdio.binary.text-edit.v1";
const BINARY_TEXT_MAXIMUM_RAW_BYTES: usize = 4 * semio_framework_plugin::plugin_app_close_prelude::store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
const BINARY_TEXT_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] =
    &[ArtifactToolPublicationContract { tool_id: BINARY_TEXT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binary_text_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BINARY_TEXT_MAXIMUM_RAW_BYTES, 4_096, 1, main::HEX_EDITOR_MAX_BYTES, 7_500)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binary_command_id(command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<BinaryEditorCommand>) -> &'static str {
    semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| BINARY_TEXT_ACTION_ID)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binary_text_emit(
    command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<BinaryEditorCommand>,
    snapshot: &BinarySnapshot,
) -> Result<Emit<BinaryMutation>, Fault> {
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(BinaryEditorCommand::ReplaceText { text }) = command else {
        return Err(Fault::from("stdio-binary-snapshot-edit-routed-to-native-reducer"));
    };
    let parsed = parse_hex_dump(text).ok_or_else(|| {
        Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.binary.invalid-hex"),
            "The byte editor contains an odd digit count or a non-hexadecimal character.",
        )
    })?;
    Ok(Emit {
        artifact_mutations: vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 0, remove_len: snapshot.bytes.len(), insert: parsed })],
        description: Some("Replace bytes".into()),
        ..Default::default()
    })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binary_text_reduce(
    command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<BinaryEditorCommand>,
    snapshot: &BinarySnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BinaryEditor>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<BinaryMutation>, Fault> {
    binary_text_emit(command, snapshot)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn binary_text_extent(
    command: &semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<BinaryEditorCommand>,
    _snapshot: &BinarySnapshot,
    _interaction: &protocol::InteractionState,
) -> Option<usize> {
    match command {
        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(BinaryEditorCommand::ReplaceText { text }) => parse_hex_dump(text).map(|bytes| bytes.len().max(1)),
        _ => None,
    }
}

struct BinaryTextCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BinaryTextCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BINARY_TEXT_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for BinaryTextCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BinaryEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BinaryEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BINARY_TEXT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        binary_text_contract()
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
        if input.declared_bytes() > BINARY_TEXT_MAXIMUM_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio binary text edit rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for BinaryTextCommandJobFactory {
    type Owner = EditorApp<BinaryEditor>;
    const TOOL_IDS: &'static [&'static str] = BINARY_TEXT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BINARY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BINARY_TEXT_PUBLICATION_CONTRACTS;
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_binary_text_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<BinaryEditor>>) -> Result<Option<ToolOperationSpec>, Fault> {
    if request.tool_id != BINARY_TEXT_ACTION_ID {
        return Ok(None);
    }
    if binary_command_id(&request.command) != request.tool_id {
        return Err(Fault::from("stdio-binary-text-command-tool-mismatch"));
    }
    let tool_id = binary_command_id(&request.command);
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
        binary_command_id,
        BINARY_TEXT_MAXIMUM_RAW_BYTES,
        main::HEX_EDITOR_MAX_BYTES,
        Box::new(BoundedArtifactCommandWork::new(tool_id, binary_text_reduce, binary_text_extent)),
    )?;
    Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🧵️RetainedTextRoute

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BinaryEditor;

impl ArtifactEditor for BinaryEditor {
    type Snapshot = BinarySnapshot;
    type Mutation = BinaryMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<BinaryEditorCommand>;

    const DIALECT: Dialect = BINARY_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_BINARY_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.binary@raw/*#editor",
        artifact_schema: "stdio.binary",
        preparation: "stdio-binary-snapshot-edit",
        native: {
            factory_name: "BinaryTextCommandJobFactory",
            factory_type: BinaryTextCommandJobFactory,
            contract: binary_text_contract(),
            build: build_binary_text_tool_job,
            tools: ["textEdit"]
        },
    }

    fn command_id(command: &Self::Command) -> &'static str {
        binary_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "textEdit" => Ok(BinaryEditorCommand::ReplaceText {
                text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")?,
            }),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.binary.unhandled-action"), format!("unknown binary editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> BinarySnapshot {
        BinarySnapshot::default()
    }

    /// ✏️ Parses the hex text and, if well-formed, replaces the WHOLE buffer via
    /// `BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 0, remove_len: <old len>, insert: <parsed> })`. Malformed
    /// hex (odd length or an invalid digit) returns a fault without changing the document.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(BinaryEditorCommand::ReplaceText { .. }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        binary_text_emit(command, doc.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.binary@raw/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for BinaryEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }


    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_binary_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(BINARY_EDITOR_DIALECT)
        .document(["stdio", "binary"])
        .icon_id("binary")
        .mode_def(edit::definition())
        .default_mode_id(edit::BINARY_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Binary"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
