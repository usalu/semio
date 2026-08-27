//! ✏️ Txt editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.txt@utf-8/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TextWindowKit`), replacing the document through the direct line, line-ending, and trailing-newline mutations
//! — a `replace-text` command is inherently whole-buffer, so per-line `InsertLine`/`SetLine` are not
//! reachable through this window (documented, not silently dropped: a future line-addressable editor
//! could target those directly).

use crate::artifacts::txt::schema::mutation_support::txt_usize_to_u32;
use crate::artifacts::txt::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetTrailingNewlineMutation};
use crate::artifacts::txt::{STDIO_TXT_DOCUMENT_SCHEMA, TxtMutation, TxtSnapshot};
use crate::editor::txt::modes::edit;
use crate::editor::txt::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const TXT_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`replace-text`, contract §2.6) can trigger.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TxtEditorCommand {
    ReplaceText { text: String },
}

/// 🔤️ Hand-rolled hex codec — `OpText::print_op` must be one line, and `ReplaceText` carries
/// arbitrary multi-line/UTF-8 text, so every byte is hex-escaped rather than attempting a
/// space/newline escaping scheme.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 {
        return Err("odd-length hex string".into());
    }
    (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect()
}

impl protocol::OpText for TxtEditorCommand {
    fn print_op(&self) -> String {
        let TxtEditorCommand::ReplaceText { text } = self;
        format!("replace-text text={}", hex_encode(text.as_bytes()))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let hex = line.strip_prefix("replace-text text=").ok_or_else(|| store::TextError::new(format!("txt editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let bytes = hex_decode(hex).map_err(|error| store::TextError::new(format!("txt editor command: bad hex {error}"), dsl::TextSpan::at(1, 1)))?;
        let text = String::from_utf8(bytes).map_err(|error| store::TextError::new(format!("txt editor command: bad utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
        Ok(TxtEditorCommand::ReplaceText { text })
    }
}

impl protocol::OpBinary for TxtEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "txt editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️TextSplit
/// 🧮️ Splits a plain `\n`-joined buffer into `(lines, trailing_newline)`. The document's existing
/// `line_ending` convention is preserved rather than re-detected — a plain-text window edit never
/// carries `\r\n` metadata worth trusting.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_text(text: &str) -> (Vec<String>, bool) {
    if text.is_empty() {
        return (Vec::new(), false);
    }
    let trailing = text.ends_with('\n');
    let body = if trailing { &text[..text.len() - 1] } else { text };
    let lines = body.split('\n').map(|line| line.trim_end_matches('\r').to_string()).collect();
    (lines, trailing)
}
//#endregion 🔖️TextSplit

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TxtEditor;

impl ArtifactEditor for TxtEditor {
    type Snapshot = TxtSnapshot;
    type Mutation = TxtMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TxtEditorCommand;

    const DIALECT: Dialect = TXT_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TXT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> TxtSnapshot {
        TxtSnapshot::default()
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let TxtEditorCommand::ReplaceText { text } = command;
        let (lines, trailing_newline) = split_text(text);
        let mut mutations = Vec::new();
        if doc.snapshot.trailing_newline {
            mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false }));
        }
        for index in (0..doc.snapshot.lines.len()).rev() {
            let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
            mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index }));
        }
        for (index, text) in lines.into_iter().enumerate() {
            let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
            mutations.push(TxtMutation::InsertLine(InsertLineMutation { index, text }));
        }
        if trailing_newline {
            mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: true }));
        }
        Ok(Emit { artifact_mutations: mutations, description: Some("Replace text".into()), ..Default::default() })
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
pub fn create_txt_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TXT_EDITOR_DIALECT).document(["semio", "stdio", "txt"]).icon_id("type").mode_def(edit::definition()).default_mode_id(edit::TXT_EDIT_MODE_ID).window_kind_def(main::definition()).default_layout(edit::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_txt_editor_builds_a_definition_for_the_editor_role() {
        let def = create_txt_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, TXT_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TxtEditor as ArtifactEditor>::DIALECT, TXT_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_text_window() {
        let def = create_txt_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn split_text_detects_trailing_newline() {
        assert_eq!(split_text("a\nb\n"), (vec!["a".to_string(), "b".to_string()], true));
        assert_eq!(split_text("a\nb"), (vec!["a".to_string(), "b".to_string()], false));
        assert_eq!(split_text(""), (Vec::new(), false));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = TxtEditorCommand::ReplaceText { text: "hello\nworld".into() };
        let printed = <TxtEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <TxtEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
