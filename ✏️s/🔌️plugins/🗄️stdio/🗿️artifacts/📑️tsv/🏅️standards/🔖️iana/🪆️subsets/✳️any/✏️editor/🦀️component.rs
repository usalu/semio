//! ✏️ Tsv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.tsv@iana/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `TsvSnapshot.records` through the artifact's own
//! `TsvMutation::SetCell`.

use crate::artifacts::tsv::{TsvMutation, TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
use crate::editor::tsv::modes::edit;
use crate::editor::tsv::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const TSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index `TsvSnapshot.records`
/// directly (no header-offset math, unlike csv).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TsvEditorCommand {
    SetCell { row: u32, column: u32, value: String },
}

impl protocol::OpText for TsvEditorCommand {
    fn print_op(&self) -> String {
        let TsvEditorCommand::SetCell { row, column, value } = self;
        format!("set-cell row={row} column={column} value={}", value.replace('\\', "\\\\").replace(' ', "\\s"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-cell ").ok_or_else(|| store::TextError::new(format!("tsv editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut value = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("tsv editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\\\", "\\");
            match key {
                "row" => row = decoded.parse::<u32>().ok(),
                "column" => column = decoded.parse::<u32>().ok(),
                "value" => value = decoded,
                _ => {}
            }
        }
        let (row, column) = row.zip(column).ok_or_else(|| store::TextError::new("tsv editor command: missing row/column", dsl::TextSpan::at(1, 1)))?;
        Ok(TsvEditorCommand::SetCell { row, column, value })
    }
}

impl protocol::OpBinary for TsvEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "tsv editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "tsv editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TsvEditor;

impl ArtifactEditor for TsvEditor {
    type Snapshot = TsvSnapshot;
    type Mutation = TsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TsvEditorCommand;

    const DIALECT: Dialect = TSV_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TSV_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> TsvSnapshot {
        TsvSnapshot::default()
    }

    /// ✏️ Out-of-range row is a documented no-op (`Emit::default()`), never a panic.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let TsvEditorCommand::SetCell { row, column, value } = command;
        if doc.snapshot.records.get(*row as usize).is_none() {
            return Ok(Emit::default());
        }
        Ok(Emit { artifact_mutations: vec![TsvMutation::SetCell { row_index: *row as usize, field_index: *column as usize, value: value.clone() }], description: Some(format!("Set cell {row},{column}")), ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_tsv_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TSV_EDITOR_DIALECT).document(["semio", "stdio", "tsv"]).icon_id("table-2").mode_def(edit::definition()).default_mode_id(edit::TSV_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_tsv_editor_builds_a_definition_for_the_editor_role() {
        let def = create_tsv_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, TSV_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TsvEditor as ArtifactEditor>::DIALECT, TSV_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_table_window() {
        let def = create_tsv_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = TsvEditorCommand::SetCell { row: 2, column: 5, value: "a value".into() };
        let printed = <TsvEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <TsvEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
