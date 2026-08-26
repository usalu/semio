//! ✏️ Csv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.csv@rfc4180/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `CsvSnapshot.records` through the artifact's own
//! `CsvMutation::SetField`.

use crate::artifacts::csv::{CsvMutation, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use crate::editor::csv::modes::edit;
use crate::editor::csv::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against `crate::artifacts::csv::schema::derived_analysis::
/// CsvAnalyzerAnalysis::DIALECT` (the artifact's own real analysis-capability row), not guessed.
/// Duplicated (not imported) in the sibling `👁️viewer` surface root — never shared through an
/// `editor`-rooted import, so a viewer file can never depend on this module.
pub const CSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index the rendered grid (post
/// header-split, see the window's own `render` doc comment) — `handle` below does the row-offset
/// math back to `CsvMutation::SetField`'s `record_index`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CsvEditorCommand {
    SetCell { row: u32, column: u32, value: String },
}

impl protocol::OpText for CsvEditorCommand {
    fn print_op(&self) -> String {
        let CsvEditorCommand::SetCell { row, column, value } = self;
        format!("set-cell row={row} column={column} value={}", value.replace('\\', "\\\\").replace(' ', "\\s"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-cell ").ok_or_else(|| store::TextError::new(format!("csv editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut value = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("csv editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\\\", "\\");
            match key {
                "row" => row = decoded.parse::<u32>().ok(),
                "column" => column = decoded.parse::<u32>().ok(),
                "value" => value = decoded,
                _ => {}
            }
        }
        let (row, column) = row.zip(column).ok_or_else(|| store::TextError::new("csv editor command: missing row/column", dsl::TextSpan::at(1, 1)))?;
        Ok(CsvEditorCommand::SetCell { row, column, value })
    }
}

impl protocol::OpBinary for CsvEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️GridMapping
/// 🧮️ Pure row-offset math, kept standalone so it is directly unit-testable without constructing
/// a full `ArtifactView`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn grid_row_to_record_index(has_header: bool, row: u32) -> usize {
    if has_header {
        row as usize + 1
    } else {
        row as usize
    }
}
//#endregion 🔖️GridMapping

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct CsvEditor;

impl ArtifactEditor for CsvEditor {
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = CsvEditorCommand;

    const DIALECT: Dialect = CSV_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_CSV_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> CsvSnapshot {
        CsvSnapshot::default()
    }

    /// ✏️ Maps the rendered grid's `row` back to `CsvSnapshot.records`' real index — `+1` when
    /// `has_header` (row 0 in the grid is `records[1]`), unchanged otherwise. Out-of-range is a
    /// documented no-op (`Emit::default()`), never a panic.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let CsvEditorCommand::SetCell { row, column, value } = command;
        let record_index = grid_row_to_record_index(doc.snapshot.has_header, *row);
        let Some(record) = doc.snapshot.records.get(record_index) else { return Ok(Emit::default()) };
        let quoted = record.fields.get(*column as usize).map(|field| field.quoted).unwrap_or(false);
        Ok(Emit { artifact_mutations: vec![CsvMutation::SetField { record_index, field_index: *column as usize, value: value.clone(), quoted }], description: Some(format!("Set cell {row},{column}")), ..Default::default() })
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
pub fn create_csv_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(CSV_EDITOR_DIALECT).document(["semio", "stdio", "csv"]).icon_id("table-2").mode_def(edit::definition()).default_mode_id(edit::CSV_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_csv_editor_builds_a_definition_for_the_editor_role() {
        let def = create_csv_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, CSV_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<CsvEditor as ArtifactEditor>::DIALECT, CSV_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_table_window() {
        let def = create_csv_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn grid_row_offsets_by_one_when_has_header() {
        assert_eq!(grid_row_to_record_index(true, 0), 1);
        assert_eq!(grid_row_to_record_index(false, 0), 0);
        assert_eq!(grid_row_to_record_index(true, 3), 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = CsvEditorCommand::SetCell { row: 2, column: 5, value: "a value".into() };
        let printed = <CsvEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <CsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
