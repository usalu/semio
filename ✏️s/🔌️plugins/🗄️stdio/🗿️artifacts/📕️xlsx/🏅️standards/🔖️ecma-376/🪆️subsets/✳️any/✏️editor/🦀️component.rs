//! 📕️ Xlsx editor (ecma-376/✳️any) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/*`. This subset has zero pre-existing document apps, so this is authored
//! fresh straight against `XlsxSnapshot`'s own composed shape (`opc`: the verbatim OPC package;
//! `workbook`: the typed semantic view — name-keyed sheets, each a sparse `(row, col)`-addressed
//! cell list, plus an index-keyed shared-strings table). One real window, `🪟️main`
//! (`TableWindowKit`), flattens every sheet's cells into a single row-per-cell table (see
//! `xlsx_flat_cells`'s own doc comment for why a per-sheet pick was rejected). Its one editable
//! column (`value`) funnels through this file's single typed command, `XlsxEditorCommand::SetCell`,
//! into `XlsxMutation::SetCell` — the cleanest possible fit `TableWindowKit`'s `set-cell` action has
//! in this artifact's whole mutation surface.

use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::mutations::set_cell;
use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxCellValue;
use crate::artifacts::xlsx::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use crate::editor::xlsx::standards::v_ecma_376::subsets::any::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::any::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/*`. Duplicated (not imported) from the
/// sibling schema facet's own coordinate: that facet is owned by a different, live peer ticket
/// (26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS) and, unlike the
/// `✳️strict`/`✳️transitional` subsets, `✳️any`'s own schema module exposes no standalone `pub const
/// DIALECT` to import (only an associated const on its `XlsxAnalyzer` impl) — this ticket's own
/// contract hands the value directly, so it is restated here rather than reached for across the
/// scope boundary.
pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
/// 🧮 Flattens every sheet's cells into one row-per-cell projection — `(sheet, row, col, value)`.
/// Picked over a fixed "render one sheet" first pass because a workbook this artifact composes may
/// hold any number of sheets, and hiding every sheet but one would silently drop data from view;
/// this flat projection stays lossless and uniform regardless of sheet count. Row order is sheets in
/// `workbook.sheets` storage order, then each sheet's own `cells` storage order (sparse, never
/// re-sorted) — the SAME order `XlsxEditorCommand::SetCell`'s `row` indexes into, so a `set-cell`
/// edit always addresses the row this fn emitted at that position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xlsx_flat_cells(document: &XlsxSnapshot) -> Vec<(String, u32, u32, XlsxCellValue)> {
    document.workbook.sheets.iter().flat_map(|sheet| sheet.cells.iter().map(move |cell| (sheet.name.clone(), cell.row, cell.col, cell.value.clone()))).collect()
}

/// 🔎 Renders one cell value to display text. `SharedString` resolves against this document's own
/// `workbook.shared_strings` (the typed semantic view's own index-keyed table — never `opc`'s raw
/// XML, see the snapshot module's own doc comment on why the two must stay distinct); an
/// out-of-range index degrades to `"#<index>"` rather than panicking. `Formula` shows `=expr`, plus
/// its cached value in parens when present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn render_xlsx_cell_value(value: &XlsxCellValue, shared_strings: &[String]) -> String {
    match value {
        XlsxCellValue::Number(n) => format!("{n}"),
        XlsxCellValue::SharedString(index) => shared_strings.get(*index).cloned().unwrap_or_else(|| format!("#{index}")),
        XlsxCellValue::InlineString(text) => text.clone(),
        XlsxCellValue::Boolean(flag) => flag.to_string(),
        XlsxCellValue::Formula { expr, cached } => match cached {
            Some(cached) => format!("={expr} ({})", render_xlsx_cell_value(cached, shared_strings)),
            None => format!("={expr}"),
        },
        XlsxCellValue::Empty => String::new(),
    }
}

/// ✍️ Parses a `set-cell` edit's raw text back into a typed `XlsxCellValue` — cheap detection only:
/// exact `"true"`/`"false"` becomes `Boolean`, text that parses whole as `f64` becomes `Number`,
/// everything else becomes `InlineString`. Never reconstructs `SharedString`/`Formula` from bare
/// display text (that would be a guess, not a decode) — editing a formula or shared-string cell
/// through this window turns it into a literal string cell, the same "type a value over a formula"
/// behavior every spreadsheet editor has; documented narrowing, not a silent loss.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_xlsx_cell_value(text: &str) -> XlsxCellValue {
    match text {
        "true" => XlsxCellValue::Boolean(true),
        "false" => XlsxCellValue::Boolean(false),
        _ => match text.parse::<f64>() {
            Ok(n) => XlsxCellValue::Number(n),
            Err(_) => XlsxCellValue::InlineString(text.to_string()),
        },
    }
}
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row` indexes `xlsx_flat_cells`'s own output;
/// `value` is the edited cell's raw display text, parsed back by `parse_xlsx_cell_value`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum XlsxEditorCommand {
    SetCell { row: u32, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxEditorCommand::SetCell { row, value } = self;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut out, *row as u64);
        let bytes = value.as_bytes();
        store::pack_rt::write_varint_u64(&mut out, bytes.len() as u64);
        out.extend_from_slice(bytes);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let row = reader.read_varint_u64().map_err(|e| malformed("op row", reader.position(), e.to_string()))? as u32;
        let len = reader.read_varint_u64().map_err(|e| malformed("op value len", reader.position(), e.to_string()))? as usize;
        let value_bytes = reader.read_bytes(len).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        let value = String::from_utf8(value_bytes.to_vec()).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        Ok(XlsxEditorCommand::SetCell { row, value })
    }
}
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
    type Command = XlsxEditorCommand;

    const DIALECT: Dialect = XLSX_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> XlsxSnapshot {
        XlsxSnapshot::default()
    }

    /// ✏️ Resolves `command.row` against `xlsx_flat_cells`'s own flattening (so it always targets
    /// the exact cell the table rendered at that position), parses the edited text through
    /// `parse_xlsx_cell_value`, then dispatches a single `XlsxMutation::SetCell`. An out-of-range
    /// row is a documented no-op (`Emit::default()`), never a panic.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let XlsxEditorCommand::SetCell { row, value } = command;
        let Some((sheet_name, cell_row, cell_col, _)) = xlsx_flat_cells(doc.snapshot).into_iter().nth(*row as usize) else { return Ok(Emit::default()) };
        let parsed = parse_xlsx_cell_value(value);
        let description = format!("Set {sheet_name}!{cell_row},{cell_col}");
        Ok(Emit { artifact_mutations: vec![XlsxMutation::SetCell(set_cell::SetCell { sheet_name, row: cell_row, col: cell_col, value: parsed })], description: Some(description), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(XLSX_DIALECT).document(["stdio", "xlsx"]).icon_id("table").mode_def(edit::definition()).default_mode_id(edit::XLSX_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_xlsx_editor_builds_a_definition_for_the_editor_role() {
        let def = create_xlsx_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, XLSX_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<XlsxEditor as ArtifactEditor>::DIALECT, XLSX_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_xlsx_editor();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_cells_orders_by_sheet_then_cell_storage_order() {
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
        let document = XlsxSnapshot {
            workbook: XlsxWorkbook {
                sheets: vec![
                    XlsxSheet { name: "S1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] },
                    XlsxSheet { name: "S2".into(), cells: vec![XlsxCell { row: 2, col: 1, value: XlsxCellValue::Boolean(true) }] },
                ],
                ..Default::default()
            },
            ..XlsxSnapshot::default()
        };
        let rows = xlsx_flat_cells(&document);
        assert_eq!(rows, vec![("S1".to_string(), 1, 0, XlsxCellValue::Number(1.0)), ("S2".to_string(), 2, 1, XlsxCellValue::Boolean(true))]);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_value_resolves_shared_strings_and_shows_formula_cache() {
        let strings = vec!["hello".to_string()];
        assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(0), &strings), "hello");
        assert_eq!(render_xlsx_cell_value(&XlsxCellValue::SharedString(9), &strings), "#9");
        assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) }, &strings), "=SUM(A1:A2) (3)");
        assert_eq!(render_xlsx_cell_value(&XlsxCellValue::Empty, &strings), "");
    }

    #[semio_framework_async_macros::async_test]
    async fn parse_cell_value_detects_bool_and_number_before_falling_back_to_inline_string() {
        assert_eq!(parse_xlsx_cell_value("true"), XlsxCellValue::Boolean(true));
        assert_eq!(parse_xlsx_cell_value("false"), XlsxCellValue::Boolean(false));
        assert_eq!(parse_xlsx_cell_value("3.5"), XlsxCellValue::Number(3.5));
        assert_eq!(parse_xlsx_cell_value("hello"), XlsxCellValue::InlineString("hello".into()));
    }
}
//#endregion 🧪️Tests
