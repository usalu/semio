//! 📕️ Xlsx editor (ecma-376/✳️transitional) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/transitional`. Transitional reuses `✳️any`'s own `XlsxSnapshot` verbatim
//! (same Rust type, same `s.stdio.xlsx` schema id), so this surface is authored fresh against that
//! same composed shape (`opc`: the verbatim OPC package; `workbook`: the typed semantic view). One
//! real window, `🪟️main` (`TableWindowKit`), flattens every sheet's cells into a single row-per-cell
//! table (see `xlsx_flat_cells`'s own doc comment for why a per-sheet pick was rejected). Its one
//! editable column (`value`) funnels through this file's single typed command,
//! `XlsxTransitionalEditorCommand::SetCell`, into `XlsxMutation::SetCell` — the cleanest possible
//! fit `TableWindowKit`'s `set-cell` action has in this artifact's whole mutation surface.

use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::XlsxCellValue;
use crate::artifacts::xlsx::{XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/transitional`. The sibling schema facet
/// (owned by the live peer ticket 26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-
/// MUTATIONS) already exports an identical `pub const DIALECT` at `🧬️schema/🦀️component.rs`, but
/// this ticket's own scope excludes importing across that boundary — restated here directly from
/// this ticket's own contract, matching `✳️any`/`✳️strict`'s sibling surfaces' identical restating.
pub const XLSX_TRANSITIONAL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
/// 🧮 Flattens every sheet's cells into one row-per-cell projection — `(sheet, row, col, value)`.
/// Picked over a fixed "render one sheet" first pass because a workbook this artifact composes may
/// hold any number of sheets, and hiding every sheet but one would silently drop data from view;
/// this flat projection stays lossless and uniform regardless of sheet count. Row order is sheets in
/// `workbook.sheets` storage order, then each sheet's own `cells` storage order (sparse, never
/// re-sorted) — the SAME order `XlsxTransitionalEditorCommand::SetCell`'s `row` indexes into, so a
/// `set-cell` edit always addresses the row this fn emitted at that position.
pub(crate) async fn xlsx_flat_cells(document: &XlsxSnapshot) -> Vec<(String, u32, u32, XlsxCellValue)> {
    document.workbook.sheets.iter().flat_map(|sheet| sheet.cells.iter().map(move |cell| (sheet.name.clone(), cell.row, cell.col, cell.value.clone()))).collect()
}

/// 🔎 Renders one cell value to display text. `SharedString` resolves against this document's own
/// `workbook.shared_strings` (the typed semantic view's own index-keyed table — never `opc`'s raw
/// XML, see the snapshot module's own doc comment on why the two must stay distinct); an
/// out-of-range index degrades to `"#<index>"` rather than panicking. `Formula` shows `=expr`, plus
/// its cached value in parens when present.
pub(crate) async fn render_xlsx_cell_value(value: &XlsxCellValue, shared_strings: &[String]) -> String {
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
pub(crate) async fn parse_xlsx_cell_value(text: &str) -> XlsxCellValue {
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
pub enum XlsxTransitionalEditorCommand {
    SetCell { row: u32, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxTransitionalEditorCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxTransitionalEditorCommand::SetCell { row, value } = self;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut out, *row as u64);
        let bytes = value.as_bytes();
        store::pack_rt::write_varint_u64(&mut out, bytes.len() as u64);
        out.extend_from_slice(bytes);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().await.map_err(|e| malformed("op format", 0, e.to_string()))?;
        let row = reader.read_varint_u64().await.map_err(|e| malformed("op row", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as u32;
        let len = reader.read_varint_u64().await.map_err(|e| malformed("op value len", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as usize;
        let value_bytes = reader.read_bytes(len).await.map_err(|e| malformed("op value", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
        let value = String::from_utf8(value_bytes.to_vec()).map_err(|e| malformed("op value", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
        Ok(XlsxTransitionalEditorCommand::SetCell { row, value })
    }
}
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
    type Command = XlsxTransitionalEditorCommand;

    const DIALECT: Dialect = XLSX_TRANSITIONAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> XlsxSnapshot {
        XlsxSnapshot::default()
    }

    /// ✏️ Resolves `command.row` against `xlsx_flat_cells`'s own flattening (so it always targets
    /// the exact cell the table rendered at that position), parses the edited text through
    /// `parse_xlsx_cell_value`, then dispatches a single `XlsxMutation::SetCell`. An out-of-range
    /// row is a documented no-op (`Emit::default()`), never a panic.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let XlsxTransitionalEditorCommand::SetCell { row, value } = command;
        let Some((sheet_name, cell_row, cell_col, _)) = xlsx_flat_cells(doc.snapshot).into_iter().nth(*row as usize) else { return Ok(Emit::default()) };
        let parsed = parse_xlsx_cell_value(value);
        let description = format!("Set {sheet_name}!{cell_row},{cell_col}");
        Ok(Emit { artifact_mutations: vec![XlsxMutation::SetCell { sheet_name, row: cell_row, col: cell_col, value: parsed.await }], description: Some(description), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).await,
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))).await,
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub async fn create_xlsx_transitional_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(XLSX_TRANSITIONAL_DIALECT)
        .await.document(["stdio", "xlsx", "transitional"])
        .await.icon_id("table")
        .await.mode_def(edit::definition().await)
        .await.default_mode_id(edit::XLSX_TRANSITIONAL_EDIT_MODE_ID)
        .await.window_kind_def(main::definition().await)
        .await.default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_xlsx_transitional_editor_builds_a_definition_for_the_editor_role() {
        let def = create_xlsx_transitional_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, XLSX_TRANSITIONAL_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<XlsxTransitionalEditor as ArtifactEditor>::DIALECT, XLSX_TRANSITIONAL_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_xlsx_transitional_editor();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn flat_cells_orders_by_sheet_then_cell_storage_order() {
        use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::snapshot::{XlsxCell, XlsxSheet, XlsxWorkbook};
        let document = XlsxSnapshot {
            workbook: XlsxWorkbook {
                sheets: vec![XlsxSheet { name: "S1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "S2".into(), cells: vec![XlsxCell { row: 2, col: 1, value: XlsxCellValue::Boolean(true) }] }],
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
