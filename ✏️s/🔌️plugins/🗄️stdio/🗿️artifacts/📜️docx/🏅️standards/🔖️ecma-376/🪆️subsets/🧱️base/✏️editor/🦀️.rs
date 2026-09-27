//! ✏️ Docx editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.docx@ecma-376/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`DocumentWindowKit`), rendering one page per top-level `DocxDocument.body` block and editing
//! it through the artifact's own `DocxMutation::SetBlockContent`.

use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit::windows::main;
use crate::schema::diff::DocxBlockPath;
use crate::schema::mutations::{set_block_content, set_snapshot};
use crate::schema::snapshot::{DocxBlock, DocxRun};
use crate::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.docx@ecma-376/*` (the unrestricted `any` subset). Duplicated
/// (not imported) in the sibling read-only surface root — never shared through this module, so
/// that surface can never depend on this one.
pub const DOCX_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `index` addresses `DocxDocument.body` directly
/// (one page per top-level block, see the window's own `render` doc comment).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum DocxEditorCommand {
    SetPage { index: u32, text: String },
}

impl protocol::OpText for DocxEditorCommand {
    fn print_op(&self) -> String {
        let DocxEditorCommand::SetPage { index, text } = self;
        format!("set-page index={index} text={}", text.replace('\\', "\\\\").replace('\n', "\\n").replace(' ', "\\s"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-page ").ok_or_else(|| store::TextError::new(format!("docx editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut index = None;
        let mut text = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("docx editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\n", "\n").replace("\\\\", "\\");
            match key {
                "index" => index = decoded.parse::<u32>().ok(),
                "text" => text = decoded,
                _ => {}
            }
        }
        let index = index.ok_or_else(|| store::TextError::new("docx editor command: missing index", dsl::TextSpan::at(1, 1)))?;
        Ok(DocxEditorCommand::SetPage { index, text })
    }
}

impl protocol::OpBinary for DocxEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "docx editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "docx editor command", offset: 0, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DocxEditorCommand, []);
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
    Some(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block: DocxBlock::Paragraph(replacement) }))
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct DocxEditor;

impl ArtifactEditor for DocxEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<DocxEditorCommand>;

    const DIALECT: Dialect = DOCX_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_DOCX_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.docx@ecma-376/*#editor",
        artifact_schema: "stdio.docx",
        preparation: "stdio-docx-base-snapshot-edit"
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-page")
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => Ok(DocxEditorCommand::SetPage {
                index: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["index", "page", "row"], 0),
                text: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["text", "value"], ""),
            }),
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.unhandled-action"), format!("unknown docx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DocxSnapshot {
        DocxSnapshot::default()
    }

    /// ✏️ `set-page` replaces the addressed top-level block's whole text in one shot via
    /// `DocxMutation::SetBlockContent` — only when that block is a `Paragraph`: its runs collapse
    /// into a single plain run carrying `text` (any per-run formatting/`extra_run_properties` on the
    /// runs being replaced is intentionally dropped), while the paragraph's own `style`/
    /// `extra_paragraph_properties` are preserved unchanged. A `Table` block, or an out-of-range
    /// `index`, is a documented no-op (`Emit::default()`) — collapsing arbitrary text into a table's
    /// row/cell structure has no honest single-shot mapping, so this first pass does not attempt it.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { index, text }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let index = *index as usize;
        match build_set_page_mutation(doc.snapshot, index, text) {
            Some(mutation) => Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set page {index}")), ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.docx@ecma-376/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for DocxEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_is_admitted(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> bool {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_value_is_admitted(event, snapshot)
    }

    fn snapshot_edit_emit(
        event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent,
        snapshot: &Self::Snapshot,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_docx_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(DOCX_EDITOR_DIALECT)
        .document(["semio", "stdio", "docx"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::DOCX_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Document"))
        ;
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
