//! 📕️ Xlsx editor (ecma-376/🧱️base) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/*`. This subset has zero pre-existing document apps, so this is authored
//! fresh against `XlsxSnapshot`'s canonical OPC/XML authority. `🪟️main` renders one windowed A1
//! grid per worksheet, including revision-bound vacancies that insert cells without rebuilding XML.

use crate::editor::xlsx::standards::v_ecma_376::subsets::base::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::base::modes::edit::windows::main;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{
    cell_address::{xlsx_cell_address, xlsx_cell_target_revision, xlsx_cell_vacancy_address},
    insert_cell, edit_rules, set_cell,
};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
use crate::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
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
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};
use semio_framework_ui_locale::Label;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/*`. Duplicated (not imported) from the
/// sibling schema facet's own coordinate: that facet is owned by a different, live peer ticket
/// (26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS) and, unlike the
/// `🔒️strict`/`🌉️transitional` subsets, `🧱️base`'s own schema module exposes no standalone `pub const
/// DIALECT` to import (only an associated const on its `XlsxAnalyzer` impl) — this ticket's own
/// contract hands the value directly, so it is restated here rather than reached for across the
/// scope boundary.
pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
/// 🔎 Renders one cell value to display text. `SharedString` resolves against this document's own
/// `workbook.shared_strings` (the typed semantic view's own index-keyed table — never `opc`'s raw
/// XML, see the snapshot module's own doc comment on why the two must stay distinct); an
/// out-of-range index degrades to `"#<index>"` rather than panicking. `Formula` shows its editable
/// `=expr`; its optional computed cache remains separately editable in Details.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn render_xlsx_cell_value(value: &XlsxCellValue, shared_strings: &[String]) -> String {
    match value {
        XlsxCellValue::Number(n) => format!("{n}"),
        XlsxCellValue::SharedString(index) => shared_strings.get(*index).cloned().unwrap_or_else(|| format!("#{index}")),
        XlsxCellValue::InlineString(text) => text.clone(),
        XlsxCellValue::Boolean(flag) => flag.to_string(),
        XlsxCellValue::Error(error) => error.clone(),
        XlsxCellValue::Formula { expr, .. } => format!("={expr}"),
        XlsxCellValue::Empty => String::new(),
    }
}

/// ✍️ Parses one spreadsheet cell draft without losing its common value kind. Empty text clears the
/// cell, `=expression` creates a formula, an apostrophe forces literal text, exact booleans retain
/// their type, and only finite complete numbers become numeric cells. Shared-string indexing and a
/// formula cache remain available through Details because neither has an unambiguous text spelling.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xlsx_cell_value(text: &str) -> XlsxCellValue {
    if text.is_empty() {
        return XlsxCellValue::Empty;
    }
    if let Some(literal) = text.strip_prefix("'") {
        return XlsxCellValue::InlineString(literal.to_string());
    }
    if let Some(expr) = text.strip_prefix('=') {
        return XlsxCellValue::Formula { expr: expr.to_string(), cached: None };
    }
    match text {
        "true" => XlsxCellValue::Boolean(true),
        "false" => XlsxCellValue::Boolean(false),
        _ => match text.parse::<f64>() {
            Ok(number) if number.is_finite() => XlsxCellValue::Number(number),
            _ => XlsxCellValue::InlineString(text.to_string()),
        },
    }
}

/// 🔐️ Binds a draft to its typed value and the text of its addressed shared-string entry.
pub(crate) fn xlsx_cell_revision(value: &XlsxCellValue, shared_strings: &[String]) -> String {
    use semio_framework_plugin::app::DocumentWindowKit;
    let typed = DocumentWindowKit::text_revision(&semio_s_artifact_stdio_contract::editing::snapshot_edit_source(value));
    match value {
        XlsxCellValue::SharedString(index) => match shared_strings.get(*index) {
            Some(text) => format!("{typed}:{}", DocumentWindowKit::text_revision(text)),
            None => format!("{typed}:missing"),
        },
        _ => typed,
    }
}
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. The worksheet/row/column tuple is the durable
/// identity and `revision` guards the user's draft against a concurrent cell change.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum XlsxEditorCommand {
    SetCell { sheet_name: String, row: u32, column: u32, revision: String, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxEditorCommand::SetCell { sheet_name, row, column, revision, value } = self;
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
        Ok(XlsxEditorCommand::SetCell { sheet_name, row, column, revision, value })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(XlsxEditorCommand, ["set-cell"]);
//#endregion 🔖️OpBinaryCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XlsxEditor;

impl ArtifactEditor for XlsxEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxEditorCommand>;

    const DIALECT: Dialect = XLSX_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_framework_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_framework_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

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

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.xlsx@ecma-376/*#editor",
        artifact_schema: "stdio.xlsx",
        preparation: "stdio-xlsx-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-cell")
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        xlsx_agent_target_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-cell" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_stable_cell_edit(args)?;
                Ok(XlsxEditorCommand::SetCell { sheet_name: edit.sheet_name, row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.unhandled-action"), format!("unknown xlsx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> XlsxSnapshot {
        XlsxSnapshot::default()
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
                "s.stdio.xlsx@ecma-376/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

/// 🔐️ The token an agent's omitted `set-cell` revision is admitted against: the addressed cell's own token, as its rendered
/// binding carries it; a stale address is refused exactly as the edit itself would refuse it.
pub(crate) fn xlsx_agent_target_revision(snapshot: &XlsxSnapshot, args: &semio_framework_value::DslValue) -> Result<Option<String>, Fault> {
    let sheet_name = semio_s_artifact_stdio_contract::window_kit_required_text_argument(Some(args), "sheetName")?;
    let row = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "row")?;
    let column = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "column")?;
    xlsx_cell_target_revision(snapshot, &sheet_name, row, column).map(Some).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.cell-stale"), message))
}

fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxEditorCommand) -> Result<Emit<XlsxMutation>, Fault> {
    let XlsxEditorCommand::SetCell { sheet_name, row, column, revision, value } = command;
    xlsx_set_cell_emit_fields(snapshot, sheet_name, *row, *column, revision, value, "stdio.xlsx")
}

pub(crate) fn xlsx_set_cell_emit_fields(snapshot: &XlsxSnapshot, sheet_name: &str, row: u32, column: u32, revision: &str, value: &str, fault_prefix: &'static str) -> Result<Emit<XlsxMutation>, Fault> {
    let fault = |suffix: &'static str, message: String| {
        Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new(match (fault_prefix, suffix) {
                ("stdio.xlsx.strict", "projection-invalid") => "stdio.xlsx.strict.projection-invalid",
                ("stdio.xlsx.strict", "sheet-stale") => "stdio.xlsx.strict.sheet-stale",
                ("stdio.xlsx.strict", "cell-stale") => "stdio.xlsx.strict.cell-stale",
                ("stdio.xlsx.strict", "cell-conflict") => "stdio.xlsx.strict.cell-conflict",
                ("stdio.xlsx.transitional", "projection-invalid") => "stdio.xlsx.transitional.projection-invalid",
                ("stdio.xlsx.transitional", "sheet-stale") => "stdio.xlsx.transitional.sheet-stale",
                ("stdio.xlsx.transitional", "cell-stale") => "stdio.xlsx.transitional.cell-stale",
                ("stdio.xlsx.transitional", "cell-conflict") => "stdio.xlsx.transitional.cell-conflict",
                (_, "projection-invalid") => "stdio.xlsx.projection-invalid",
                (_, "sheet-stale") => "stdio.xlsx.sheet-stale",
                (_, "cell-conflict") => "stdio.xlsx.cell-conflict",
                _ => "stdio.xlsx.cell-stale",
            }),
            message,
        )
    };
    let workbook = snapshot.project_workbook().map_err(|error| fault("projection-invalid", error.to_string()))?;
    let sheet = workbook.sheets.iter().find(|sheet| sheet.name == sheet_name).ok_or_else(|| fault("sheet-stale", format!("worksheet '{sheet_name}' no longer exists")))?;
    if let Some(cell) = sheet.cells.iter().find(|cell| cell.row == row && cell.col == column) {
        let address = xlsx_cell_address(snapshot, sheet_name, row, column).map_err(|message| fault("cell-stale", message))?;
        if address.revision != revision {
            return Err(fault("cell-conflict", format!("cell {sheet_name}!{row},{column} changed before this draft was applied")));
        }
        if render_xlsx_cell_value(&cell.value, &workbook.shared_strings) == value {
            return Ok(Emit::default());
        }
        return Ok(Emit { artifact_mutations: vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: parse_xlsx_cell_value(value), node: None })], ..Default::default() });
    }
    let address = xlsx_cell_vacancy_address(snapshot, sheet_name, row, column).map_err(|message| fault("cell-stale", message))?;
    if address.worksheet.revision != revision {
        return Err(fault("cell-conflict", format!("cell vacancy {sheet_name}!{row},{column} changed before this draft was applied")));
    }
    if value.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![XlsxMutation::InsertCell(insert_cell::InsertCell { address, value: parse_xlsx_cell_value(value), node: None })], ..Default::default() })
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for XlsxEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &edit_rules::EDIT_RULES
    }

    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot).map_err(|error| Fault::from(error.to_string()))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: XlsxEditor,
    tools: ["set-cell"],
    payload_schema: "semio.stdio.xlsx-cell-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            return Err(Fault::from("stdio-xlsx-native-edit-command-mismatch"));
        };
        xlsx_set_cell_emit(snapshot, command)
    },
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(XLSX_DIALECT)
        .document(["stdio", "xlsx"])
        .icon_id("table")
        .mode_def(edit::definition())
        .default_mode_id(edit::XLSX_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Workbook"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
