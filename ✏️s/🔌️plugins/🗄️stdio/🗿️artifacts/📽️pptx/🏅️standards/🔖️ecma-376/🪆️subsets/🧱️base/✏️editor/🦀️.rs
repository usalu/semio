//! ✏️ Pptx editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.pptx@ecma-376/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`DocumentWindowKit`), rendering one page per slide and editing every addressed text-bearing shape on
//! that slide through the artifact's own `PptxMutation::SetShapeText`.

use crate::editor::pptx::standards::v_ecma_376::subsets::base::modes::edit;
use crate::editor::pptx::standards::v_ecma_376::subsets::base::modes::edit::windows::main;
use crate::schema::mutations::{patch_snapshot, set_shape_text, set_snapshot};
use crate::{PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::StandardId;
use semio_framework_plugin::SubsetId;
use semio_framework_ui_locale::Label;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.pptx@ecma-376/*` (the unrestricted `any` subset). Duplicated
/// (not imported) in the sibling read-only surface root — never shared through this module, so
/// that surface can never depend on this one.
pub const PPTX_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `index` addresses `presentation.slides` directly
/// (one page per slide, see the window's own `render` doc comment).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum PptxEditorCommand {
    SetPage { page: u32, item: u32, revision: String, text: String },
}

semio_s_artifact_stdio_contract::impl_serde_op_codec!(PptxEditorCommand, "pptx editor command");
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(PptxEditorCommand, ["set-page"]);
//#endregion 🔖️Command

//#region 🔖️Helpers
fn build_set_page_mutation(snapshot: &PptxSnapshot, page: usize, item: usize, revision: &str, text: &str) -> Result<Option<PptxMutation>, Fault> {
    let slides = crate::schema::mutations::xml_address::pptx_slides(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.projection"), error))?;
    let slide = slides.get(page).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.stale-slide"), format!("PPTX slide {page} no longer exists")))?;
    let shape = slide.shapes.get(item).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.stale-shape"), format!("PPTX shape {page}/{item} no longer exists")))?;
    let current =
        shape.text.as_ref().ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pptx.set-page.unsupported-target"), format!("PPTX shape {page}/{item} has no editable text frame")))?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(current, revision, "stdio.pptx.set-page.conflict")?;
    if current == text {
        return Ok(None);
    }
    Ok(Some(PptxMutation::SetShapeText(set_shape_text::SetShapeText { address: shape.address.clone(), text: text.into() })))
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct PptxEditor;

impl ArtifactEditor for PptxEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<PptxEditorCommand>;

    const DIALECT: Dialect = PPTX_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PPTX_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec {
            format_kind: "s.stdio.pptx@ecma-376",
            extension: ".pptx",
            media_type: "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            binary: true,
        })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_pptx(snapshot)
            .map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(bytes)
            .map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.pptx@ecma-376/*#editor",
        artifact_schema: "stdio.pptx",
        preparation: "stdio-pptx-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-page")
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(PptxEditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
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
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(PptxEditorCommand::SetPage { page, item, revision, text }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        let page = *page as usize;
        let item = *item as usize;
        let Some(mutation) = build_set_page_mutation(doc.snapshot, page, item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale, publication_revision).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.pptx@ecma-376/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for PptxEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| PptxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }), Some(|snapshot| PptxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: PptxEditor,
    tools: ["set-page"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(PptxEditorCommand::SetPage { page, item, revision, text }) = command else {
            return Err(Fault::from("stdio-pptx-native-edit-command-mismatch"));
        };
        let page = *page as usize;
        let item = *item as usize;
        let Some(mutation) = build_set_page_mutation(snapshot, page, item, revision, text)? else { return Ok(Emit::default()) };
        Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
    },
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pptx_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(PPTX_EDITOR_DIALECT)
        .document(["semio", "stdio", "pptx"])
        .icon_id("presentation")
        .mode_def(edit::definition())
        .default_mode_id(edit::PPTX_EDIT_MODE_ID)
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
