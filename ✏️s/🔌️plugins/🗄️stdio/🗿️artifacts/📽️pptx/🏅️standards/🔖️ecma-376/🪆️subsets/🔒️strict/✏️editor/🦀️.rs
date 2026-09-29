//! ✏️ Pptx strict editor — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.pptx@ecma-376/strict` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One
//! real window, `🪟️main` (`DocumentWindowKit`), rendering one page per slide and editing every addressed
//! text-bearing shape on that slide through the artifact's own `PptxMutation::SetShapeText`.

use crate::editor::pptx::standards::v_ecma_376::subsets::strict::modes::edit;
use crate::editor::pptx::standards::v_ecma_376::subsets::strict::modes::edit::windows::main;
use crate::schema::mutations::{set_shape_text, set_snapshot};
use crate::schema::snapshot::{PptxParagraph, PptxShape};
use crate::{PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.pptx@ecma-376/strict`. Duplicated (not imported) in the
/// sibling read-only surface root — never shared through this module, so that surface can never
/// depend on this one.
pub const PPTX_STRICT_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("strict") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `index` addresses `presentation.slides` directly
/// (one page per slide, see the window's own `render` doc comment).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum PptxStrictEditorCommand {
    SetPage { page: u32, item: u32, revision: String, text: String },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(PptxStrictEditorCommand, "pptx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(PptxStrictEditorCommand, ["set-page"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
fn shape_text(shape: &PptxShape) -> Option<String> {
    match shape {
        PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.iter().map(|paragraph| paragraph.runs.iter().map(|run| run.text.as_str()).collect::<String>()).collect::<Vec<_>>().join("\n")),
        PptxShape::Picture { .. } | PptxShape::Other { .. } => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replacement_text_frame(shape: &PptxShape, text: &str) -> Option<Vec<PptxParagraph>> {
    let current = match shape {
        PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => text_frame,
        PptxShape::Picture { .. } | PptxShape::Other { .. } => return None,
    };
    let mut current = current.iter().cloned();
    Some(
        text.split('\n')
            .map(|line| {
                let mut paragraph = current.next().unwrap_or_default();
                if paragraph.runs.is_empty() {
                    paragraph = PptxParagraph::text(line);
                } else {
                    paragraph.runs[0].text = line.to_string();
                    for run in &mut paragraph.runs[1..] {
                        run.text.clear();
                    }
                }
                paragraph
            })
            .collect(),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &PptxSnapshot, page: usize, item: usize, revision: &str, text: &str) -> Result<Option<PptxMutation>, Fault> {
    let slide = snapshot.presentation.slides.get(page).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.stale-slide"), format!("PPTX slide {page} no longer exists")))?;
    let shape = slide.shapes.get(item).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.stale-shape"), format!("PPTX shape {page}/{item} no longer exists")))?;
    let current =
        shape_text(shape).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.unsupported-target"), format!("PPTX shape {page}/{item} has no editable text frame")))?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(&current, revision, "stdio.pptx.set-page.conflict")?;
    if current == text {
        return Ok(None);
    }
    let text_frame = replacement_text_frame(shape, text).expect("a text-bearing shape has a text frame");
    Ok(Some(PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index: page, shape_index: item, text_frame })))
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct PptxStrictEditor;

impl ArtifactEditor for PptxStrictEditor {
    type Snapshot = PptxSnapshot;
    type Mutation = PptxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<PptxStrictEditorCommand>;

    const DIALECT: Dialect = PPTX_STRICT_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PPTX_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs",
        controller: "s.stdio.pptx@ecma-376/strict#editor",
        artifact_schema: "stdio.pptx",
        preparation: "stdio-pptx-strict-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-page")
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(PptxStrictEditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.base.unhandled-action"), format!("unknown pptx editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> PptxSnapshot {
        crate::standards::v_ecma_376::subsets::base::schema::blank_pptx_snapshot()
    }

    /// ✏️ Replaces the addressed text-bearing shape after its optimistic revision matches.
    /// Existing paragraph and run formatting is retained wherever a submitted line corresponds.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(PptxStrictEditorCommand::SetPage { page, item, revision, text }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let page = *page as usize;
        let item = *item as usize;
        let Some(mutation) = build_set_page_mutation(doc.snapshot, page, item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set slide {page} shape {item}")), ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.pptx@ecma-376/strict#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for PptxStrictEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: PptxStrictEditor,
    tools: ["set-page"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(PptxStrictEditorCommand::SetPage { page, item, revision, text }) = command else {
            return Err(Fault::from("stdio-pptx-native-edit-command-mismatch"));
        };
        let page = *page as usize;
        let item = *item as usize;
        let Some(mutation) = build_set_page_mutation(snapshot, page, item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set slide {page} shape {item}")), ..Default::default() })
    },
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pptx_strict_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(PPTX_STRICT_EDITOR_DIALECT)
        .document(["semio", "stdio", "pptx"])
        .icon_id("presentation")
        .mode_def(edit::definition())
        .default_mode_id(edit::PPTX_STRICT_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Presentation"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
