//! ✏️ Docx editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.docx@ecma-376/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`DocumentWindowKit`), rendering one page per top-level `DocxDocument.body` block and editing
//! it through the artifact's own `DocxMutation::SetBlockContent`.

use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit;
use crate::editor::docx::standards::v_ecma_376::subsets::base::modes::edit::windows::main;
use crate::schema::diff::DocxBlockPath;
use crate::schema::mutations::{set_run_text, set_snapshot};
use crate::schema::snapshot::DocxBlock;
use crate::{DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep},
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
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
    SetPage { page: u32, item: u32, revision: String, text: String },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(DocxEditorCommand, "docx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(DocxEditorCommand, ["set-page"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ Maps one strictly addressed, revision-checked paragraph draft to a reversible mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &DocxSnapshot, page: usize, item: u32, revision: &str, text: &str) -> Result<Option<DocxMutation>, Fault> {
    let (run_index, current) = addressed_run_text(snapshot, page, item)?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(current, revision, "stdio.docx.set-page.conflict")?;
    if current == text {
        return Ok(None);
    }
    let path = DocxBlockPath { segments: Vec::new(), index: page };
    Ok(Some(DocxMutation::SetRunText(set_run_text::SetRunText { path, run_index, text: text.to_string() })))
}

fn addressed_run_text(snapshot: &DocxSnapshot, page: usize, item: u32) -> Result<(usize, &str), Fault> {
    let block = snapshot.document.body.get(page).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.stale-target"), format!("DOCX body block {page} no longer exists")))?;
    let DocxBlock::Paragraph(paragraph) = block else {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.unsupported-target"), format!("DOCX body block {page} is not a paragraph")));
    };
    let run_index = item as usize;
    let run =
        paragraph.runs.get(run_index).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.stale-item"), format!("DOCX text run {page}/{run_index} no longer exists")))?;
    Ok((run_index, &run.text))
}

const DOCX_TEXT_WORK_PAGE_BYTES: usize = 4_096;

struct DocxSetPageWork {
    scan_cursor: usize,
    revision_hash: u64,
    equal: bool,
    copying: bool,
    copied_text: semio_s_artifact_stdio_contract::editing::RetainedTextCopy,
    complete: bool,
    closing: bool,
}

impl Default for DocxSetPageWork {
    fn default() -> Self {
        Self { scan_cursor: 0, revision_hash: 0xcbf29ce484222325, equal: true, copying: false, copied_text: Default::default(), complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<DocxEditor>> for DocxSetPageWork {
    fn tool_id(&self) -> &'static str {
        "set-page"
    }

    fn extent(
        &self,
        command: &<DocxEditor as ArtifactEditor>::Command,
        snapshot: &DocxSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<DocxEditor>>>,
    ) -> Option<usize> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { page, item, text, .. }) = command else {
            return None;
        };
        addressed_run_text(snapshot, *page as usize, *item).ok()?;
        (text.len() <= semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES).then_some(2)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<DocxEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<DocxEditor>>, Fault> {
        if self.closing || self.complete {
            return Err(Fault::from("stdio.docx.set-page.work-closed"));
        }
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { page, item, revision, text }) = input.command else {
            return Err(Fault::from("stdio.docx.set-page.command-mismatch"));
        };
        let page = *page as usize;
        let (run_index, current) = addressed_run_text(input.snapshot, page, *item)?;
        if !self.copying {
            if self.scan_cursor < current.len() {
                let end = self.scan_cursor.saturating_add(DOCX_TEXT_WORK_PAGE_BYTES).min(current.len());
                let source = &current.as_bytes()[self.scan_cursor..end];
                for byte in source {
                    self.revision_hash = (self.revision_hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
                }
                self.equal &= text.as_bytes().get(self.scan_cursor..end) == Some(source);
                self.scan_cursor = end;
                return Ok(ArtifactCommandWorkStep::Replay { stage: "docx-check-run-text", preview: br#"{"en":"Checking paragraph revision","de":"Absatzversion wird gepr\u00fcft"}"# });
            }
            self.equal &= current.len() == text.len();
            if format!("{:016x}", self.revision_hash) != *revision {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.conflict"), "The paragraph changed while this draft was open"));
            }
            if self.equal {
                self.complete = true;
                return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
            }
            self.copying = true;
        }
        self.copied_text.advance(text, DOCX_TEXT_WORK_PAGE_BYTES).map_err(|message| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.set-page.copy-refused"), message))?;
        if !self.copied_text.is_complete() {
            return Ok(ArtifactCommandWorkStep::Replay { stage: "docx-copy-run-text", preview: br#"{"en":"Preparing paragraph text","de":"Absatztext wird vorbereitet"}"# });
        }
        let copied = self.copied_text.take().ok_or_else(|| Fault::from("stdio.docx.set-page.copy-incomplete"))?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit {
            artifact_mutations: vec![DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: Vec::new(), index: page }, run_index, text: copied })],
            description: Some(format!("Set page {page}")),
            ..Default::default()
        }))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        self.copied_text.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.copied_text.terminal_is_empty()
    }
}

fn docx_set_page_work(_tool_id: &'static str) -> Box<dyn ArtifactCommandWork<EditorApp<DocxEditor>>> {
    Box::new(DocxSetPageWork::default())
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
        preparation: "stdio-docx-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-page")
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(DocxEditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.docx.unhandled-action"), format!("unknown docx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> DocxSnapshot {
        DocxSnapshot::default()
    }

    /// ✏️ Replaces the addressed paragraph's text after its optimistic revision matches.
    /// The exact block/run structure is retained and invalid or stale targets return a fault.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { page, item, revision, text }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let page = *page as usize;
        let Some(mutation) = build_set_page_mutation(doc.snapshot, page, *item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set page {page}")), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
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

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: DocxEditor,
    tools: ["set-page"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(DocxEditorCommand::SetPage { page, item, revision, text }) = command else {
            return Err(Fault::from("stdio-docx-native-edit-command-mismatch"));
        };
        let page = *page as usize;
        let Some(mutation) = build_set_page_mutation(snapshot, page, *item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set page {page}")), ..Default::default() })
    },
    work: docx_set_page_work,
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
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Document"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
