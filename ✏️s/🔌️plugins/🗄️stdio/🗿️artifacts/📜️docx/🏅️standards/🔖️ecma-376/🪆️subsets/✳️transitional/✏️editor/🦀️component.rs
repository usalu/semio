//! ✏️ Docx transitional editor — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.docx@ecma-376/transitional` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
//! One real window, `🪟️main` (`DocumentWindowKit`), rendering one page per top-level
//! `DocxDocument.body` block and editing it through the artifact's own
//! `DocxMutation::SetBlockContent`.

use crate::artifacts::docx::schema::diff::DocxBlockPath;
use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxRun};
use crate::artifacts::docx::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::editor::docx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/transitional`. Duplicated (not imported) in the
/// sibling read-only surface root — never shared through this module, so that surface can never
/// depend on this one.
pub const DOCX_TRANSITIONAL_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `index` addresses `DocxDocument.body` directly
/// (one page per top-level block, see the window's own `render` doc comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocxTransitionalEditorCommand {
    SetPage { index: u32, text: String },
}

impl protocol::OpText for DocxTransitionalEditorCommand {
    fn print_op(&self) -> String {
        let DocxTransitionalEditorCommand::SetPage { index, text } = self;
        format!("set-page index={index} text={}", text.replace('\\', "\\\\").replace('\n', "\\n").replace(' ', "\\s"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-page ").ok_or_else(|| store::TextError::new(format!("docx transitional editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut index = None;
        let mut text = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("docx transitional editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\n", "\n").replace("\\\\", "\\");
            match key {
                "index" => index = decoded.parse::<u32>().ok(),
                "text" => text = decoded,
                _ => {}
            }
        }
        let index = index.ok_or_else(|| store::TextError::new("docx transitional editor command: missing index", dsl::TextSpan::at(1, 1)))?;
        Ok(DocxTransitionalEditorCommand::SetPage { index, text })
    }
}

impl protocol::OpBinary for DocxTransitionalEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "docx transitional editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "docx transitional editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ Pure `set-page` -> `DocxMutation` mapping, standalone so it is directly unit-testable
/// without constructing a full `ArtifactView`. `None` covers both "index out of range" and "block
/// at index is not a Paragraph" — both documented no-ops.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &DocxSnapshot, index: usize, text: &str) -> Option<DocxMutation> {
    let DocxBlock::Paragraph(paragraph) = snapshot.document.body.get(index)? else { return None };
    let mut replacement = paragraph.clone();
    replacement.runs = vec![DocxRun { text: text.to_string(), ..Default::default() }];
    let path = DocxBlockPath { segments: Vec::new(), index };
    Some(DocxMutation::SetBlockContent { path, block: DocxBlock::Paragraph(replacement) })
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DocxTransitionalEditor;

impl ArtifactEditor for DocxTransitionalEditor {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = DocxTransitionalEditorCommand;

    const DIALECT: Dialect = DOCX_TRANSITIONAL_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DOCX_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> DocxSnapshot {
        DocxSnapshot::default()
    }

    /// ✏️ `set-page` replaces the addressed top-level block's whole text in one shot via
    /// `DocxMutation::SetBlockContent` — only when that block is a `Paragraph`: its runs collapse
    /// into a single plain run carrying `text` (any per-run formatting/`extra_run_properties` on the
    /// runs being replaced is intentionally dropped), while the paragraph's own `style`/
    /// `extra_paragraph_properties` are preserved unchanged. A `Table` block, or an out-of-range
    /// `index`, is a documented no-op (`Emit::default()`) — collapsing arbitrary text into a table's
    /// row/cell structure has no honest single-shot mapping, so this first pass does not attempt it.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let DocxTransitionalEditorCommand::SetPage { index, text } = command;
        let index = *index as usize;
        match build_set_page_mutation(doc.snapshot, index, text) {
            Some(mutation) => Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set page {index}")), ..Default::default() }),
            None => Ok(Emit::default()),
        }
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
pub fn create_docx_transitional_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(DOCX_TRANSITIONAL_EDITOR_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::DOCX_TRANSITIONAL_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_docx_transitional_editor_builds_a_definition_for_the_editor_role() {
        let def = create_docx_transitional_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, DOCX_TRANSITIONAL_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<DocxTransitionalEditor as ArtifactEditor>::DIALECT, DOCX_TRANSITIONAL_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_document_window() {
        let def = create_docx_transitional_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_page_replaces_a_paragraph_blocks_runs_with_a_single_plain_run() {
        let mut snapshot = DocxSnapshot::default();
        snapshot.document.body.push(DocxBlock::paragraph("hello"));
        let mutation = build_set_page_mutation(&snapshot, 0, "goodbye").expect("mutation");
        let DocxMutation::SetBlockContent { path, block } = &mutation else { panic!("expected SetBlockContent") };
        assert_eq!(path.index, 0);
        let DocxBlock::Paragraph(paragraph) = block else { panic!("expected Paragraph") };
        assert_eq!(paragraph.runs.len(), 1);
        assert_eq!(paragraph.runs[0].text, "goodbye");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_page_on_a_table_block_is_a_documented_no_op() {
        let mut snapshot = DocxSnapshot::default();
        snapshot.document.body.push(DocxBlock::Table(crate::artifacts::docx::schema::snapshot::DocxTable::default()));
        assert!(build_set_page_mutation(&snapshot, 0, "text").is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = DocxTransitionalEditorCommand::SetPage { index: 2, text: "a\nmulti line value".into() };
        let printed = <DocxTransitionalEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <DocxTransitionalEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
