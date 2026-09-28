//! 📕️ Xlsx editor (ecma-376/🌉️transitional) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/transitional`. Transitional reuses `🧱️base`'s own `XlsxSnapshot` verbatim
//! (same Rust type, same `s.stdio.xlsx` schema id), so this surface is authored fresh against that
//! same composed shape (`opc`: the verbatim OPC package; `workbook`: the typed semantic view). One
//! real window, `🪟️main` (`TableWindowKit`), flattens every sheet's cells into a single row-per-cell
//! table (see `xlsx_flat_cells`'s own doc comment for why a per-sheet pick was rejected). Its one
//! editable column (`value`) funnels through this file's single typed command,
//! `XlsxTransitionalEditorCommand::SetCell`, into `XlsxMutation::SetCell` — the cleanest possible
//! fit `TableWindowKit`'s `set-cell` action has in this artifact's whole mutation surface.

use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{cell_address::xlsx_cell_address, set_cell, set_snapshot};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
use crate::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/transitional`. The sibling schema facet
/// (owned by the live peer ticket 26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-
/// MUTATIONS) already exports an identical `pub const DIALECT` at `🧬️schema/🦀️component.rs`, but
/// this ticket's own scope excludes importing across that boundary — restated here directly from
/// this ticket's own contract, matching `🧱️base`/`🔒️strict`'s sibling surfaces' identical restating.
pub const XLSX_TRANSITIONAL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
/// 🧮 Flattens every sheet's cells into one row-per-cell projection — `(sheet, row, col, value)`.
/// Picked over a fixed "render one sheet" first pass because a workbook this artifact composes may
/// hold any number of sheets, and hiding every sheet but one would silently drop data from view;
/// this flat projection stays lossless and uniform regardless of sheet count. Row order is sheets in
/// `workbook.sheets` storage order, then each sheet's own `cells` storage order (sparse, never
/// re-sorted). Editing uses each row's worksheet name and native row/column identity, independent
/// of this display order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xlsx_flat_cells(document: &XlsxSnapshot) -> Vec<(String, u32, u32, XlsxCellValue)> {
    document.project_workbook().map_or_else(|_| Vec::new(), |workbook| workbook.sheets.into_iter().flat_map(|sheet| sheet.cells.into_iter().map(move |cell| (sheet.name.clone(), cell.row, cell.col, cell.value))).collect())
}

pub(crate) use crate::editor::xlsx::standards::v_ecma_376::subsets::base::{parse_xlsx_cell_value, render_xlsx_cell_value, xlsx_cell_revision};
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. The worksheet/row/column tuple is the durable
/// identity and `revision` guards the user's draft against a concurrent cell change.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum XlsxTransitionalEditorCommand {
    SetCell { sheet_name: String, row: u32, column: u32, revision: String, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxTransitionalEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value } = self;
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
        Ok(XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(XlsxTransitionalEditorCommand, ["set-cell"]);
//#endregion 🔖️OpBinaryCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XlsxTransitionalEditor;

impl ArtifactEditor for XlsxTransitionalEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxTransitionalEditorCommand>;

    const DIALECT: Dialect = XLSX_TRANSITIONAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs",
        controller: "s.stdio.xlsx@ecma-376/transitional#editor",
        artifact_schema: "stdio.xlsx",
        preparation: "stdio-xlsx-transitional-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-cell")
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-cell" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_stable_cell_edit(args)?;
                Ok(XlsxTransitionalEditorCommand::SetCell { sheet_name: edit.sheet_name, row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.unhandled-action"), format!("unknown xlsx editor action '{other}'"))),
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
            main::BODY_KEY => main::render(doc.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.xlsx@ecma-376/transitional#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxTransitionalEditorCommand) -> Result<Emit<XlsxMutation>, Fault> {
    let XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value } = command;
    let workbook = snapshot.project_workbook().map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.projection-invalid"), error.to_string()))?;
    let sheet = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.name == *sheet_name)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.sheet-stale"), format!("worksheet '{sheet_name}' no longer exists")))?;
    let cell = sheet
        .cells
        .iter()
        .find(|cell| cell.row == *row && cell.col == *column)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.cell-stale"), format!("cell {sheet_name}!{row},{column} no longer exists")))?;
    let address = xlsx_cell_address(snapshot, sheet_name, *row, *column).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.cell-stale"), message))?;
    if address.revision != *revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.xlsx.transitional.cell-conflict"), format!("cell {sheet_name}!{row},{column} changed before this draft was applied")));
    }
    if render_xlsx_cell_value(&cell.value, &workbook.shared_strings) == *value {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![XlsxMutation::SetCell(set_cell::SetCell { address, value: parse_xlsx_cell_value(value) })], description: Some(format!("Set {sheet_name}!{row},{column}")), ..Default::default() })
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for XlsxTransitionalEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: XlsxTransitionalEditor,
    tools: ["set-cell"],
    payload_schema: "semio.stdio.xlsx-transitional-cell-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            return Err(Fault::from("stdio-xlsx-transitional-native-edit-command-mismatch"));
        };
        xlsx_set_cell_emit(snapshot, command)
    },
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_transitional_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(XLSX_TRANSITIONAL_DIALECT)
        .document(["stdio", "xlsx", "transitional"])
        .icon_id("table")
        .mode_def(edit::definition())
        .default_mode_id(edit::XLSX_TRANSITIONAL_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Transitional workbook"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
