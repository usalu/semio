//! 📕️ Xlsx editor (ecma-376/🔒️strict) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/strict`. ISO/IEC 29500-1 Strict reuses `🧱️base`'s own `XlsxSnapshot`
//! verbatim (same Rust type, same `s.stdio.xlsx` schema id — conformance is a validation-gated
//! dialect stamp, not a new schema). Its shared canonical editor renders windowed worksheet grids
//! and inserts cells into explicitly addressed vacancies.

use crate::editor::xlsx::standards::v_ecma_376::subsets::strict::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::strict::modes::edit::windows::main;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{cell_address::xlsx_cell_address, patch_snapshot, set_cell, set_snapshot};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
use crate::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_ui_locale::Label;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/strict`. The sibling schema facet (owned by
/// the live peer ticket 26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS)
/// already exports an identical `pub const DIALECT` at `🧬️schema/🦀️component.rs`, but this ticket's
/// own scope excludes importing across that boundary — restated here directly from this ticket's
/// own contract, matching `🧱️base`'s sibling surface's identical restating for consistency across
/// all three subsets.
pub const XLSX_STRICT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
pub(crate) use crate::editor::xlsx::standards::v_ecma_376::subsets::base::{parse_xlsx_cell_value, render_xlsx_cell_value};
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. The worksheet/row/column tuple is the durable
/// identity and `revision` guards the user's draft against a concurrent cell change.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum XlsxStrictEditorCommand {
    SetCell { sheet_name: String, row: u32, column: u32, revision: String, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxStrictEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxStrictEditorCommand::SetCell { sheet_name, row, column, revision, value } = self;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut out, sheet_name.len() as u64);
        out.extend_from_slice(sheet_name.as_bytes());
        store::pack_rt::write_varint_u64(&mut out, *row as u64);
        store::pack_rt::write_varint_u64(&mut out, *column as u64);
        store::pack_rt::write_varint_u64(&mut out, revision.len() as u64);
        out.extend_from_slice(revision.as_bytes());
        store::pack_rt::write_varint_u64(&mut out, value.len() as u64);
        out.extend_from_slice(value.as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported format {format}")));
        }
        let sheet_len = reader.read_varint_u64().map_err(|e| malformed("op worksheet len", reader.position(), e.to_string()))? as usize;
        let sheet_name = String::from_utf8(reader.read_bytes(sheet_len).map_err(|e| malformed("op worksheet", reader.position(), e.to_string()))?.to_vec()).map_err(|e| malformed("op worksheet", reader.position(), e.to_string()))?;
        let row = reader.read_varint_u64().map_err(|e| malformed("op row", reader.position(), e.to_string()))? as u32;
        let column = reader.read_varint_u64().map_err(|e| malformed("op column", reader.position(), e.to_string()))? as u32;
        let revision_len = reader.read_varint_u64().map_err(|e| malformed("op revision len", reader.position(), e.to_string()))? as usize;
        let revision = String::from_utf8(reader.read_bytes(revision_len).map_err(|e| malformed("op revision", reader.position(), e.to_string()))?.to_vec()).map_err(|e| malformed("op revision", reader.position(), e.to_string()))?;
        let value_len = reader.read_varint_u64().map_err(|e| malformed("op value len", reader.position(), e.to_string()))? as usize;
        let value_bytes = reader.read_bytes(value_len).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        let value = String::from_utf8(value_bytes.to_vec()).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        Ok(XlsxStrictEditorCommand::SetCell { sheet_name, row, column, revision, value })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(XlsxStrictEditorCommand, ["set-cell"]);
//#endregion 🔖️OpBinaryCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XlsxStrictEditor;

impl ArtifactEditor for XlsxStrictEditor {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxStrictEditorCommand>;

    const DIALECT: Dialect = XLSX_STRICT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec {
            format_kind: "s.stdio.xlsx@ecma-376",
            extension: ".xlsx",
            media_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            binary: true,
        })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx(snapshot)
            .map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx(bytes)
            .map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs",
        controller: "s.stdio.xlsx@ecma-376/strict#editor",
        artifact_schema: "stdio.xlsx",
        preparation: "stdio-xlsx-strict-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-cell")
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        crate::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_agent_target_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-cell" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_stable_cell_edit(args)?;
                Ok(XlsxStrictEditorCommand::SetCell { sheet_name: edit.sheet_name, row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.strict.unhandled-action"), format!("unknown xlsx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> XlsxSnapshot {
        <crate::standards::v_ecma_376::subsets::strict::io::XlsxStrictBuilderConstruction as semio_framework_plugin::ArtifactBuilder>::build(crate::standards::v_ecma_376::subsets::strict::io::XlsxStrictBuilderConstruction::new(crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook::default())).expect("valid authored strict XLSX initial owner")
    }

    /// ✏️ Resolves the rendered worksheet/row/column identity and optimistic revision, then emits
    /// one `XlsxMutation::SetCell`. Stale addresses and revisions return a fault without mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        xlsx_set_cell_emit(doc.snapshot, command)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.xlsx@ecma-376/strict#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxStrictEditorCommand) -> Result<Emit<XlsxMutation>, Fault> {
    let XlsxStrictEditorCommand::SetCell { sheet_name, row, column, revision, value } = command;
    crate::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_set_cell_emit_fields(snapshot, sheet_name, *row, *column, revision, value, "stdio.xlsx.strict")
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for XlsxStrictEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }), Some(|snapshot| XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: XlsxStrictEditor,
    tools: ["set-cell"],
    payload_schema: "semio.stdio.xlsx-strict-cell-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            return Err(Fault::from("stdio-xlsx-strict-native-edit-command-mismatch"));
        };
        xlsx_set_cell_emit(snapshot, command)
    },
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_strict_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(XLSX_STRICT_DIALECT)
        .document(["stdio", "xlsx", "strict"])
        .icon_id("table")
        .mode_def(edit::definition())
        .default_mode_id(edit::XLSX_STRICT_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Strict workbook"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
