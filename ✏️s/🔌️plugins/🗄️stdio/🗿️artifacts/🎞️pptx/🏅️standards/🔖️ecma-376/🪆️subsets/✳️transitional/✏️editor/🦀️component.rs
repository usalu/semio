//! ✏️ Pptx transitional editor — the FIRST authored `ArtifactEditor` surface for
//! `s.stdio.pptx@ecma-376/transitional` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET).
//! One real window, `🪟️main` (`DocumentWindowKit`), rendering one page per slide and editing the
//! FIRST text-bearing shape on that slide through the artifact's own `PptxMutation::SetShapeText`.

use crate::artifacts::pptx::schema::mutations::set_shape_text;
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide};
use crate::artifacts::pptx::{PptxMutation, PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};
use crate::editor::pptx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::pptx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — `s.stdio.pptx@ecma-376/transitional`. Duplicated (not imported) in the
/// sibling read-only surface root — never shared through this module, so that surface can never
/// depend on this one.
pub const PPTX_TRANSITIONAL_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-page`, contract §2.6) can trigger. `index` addresses `presentation.slides` directly
/// (one page per slide, see the window's own `render` doc comment).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PptxTransitionalEditorCommand {
    SetPage { index: u32, text: String },
}

impl protocol::OpText for PptxTransitionalEditorCommand {
    fn print_op(&self) -> String {
        let PptxTransitionalEditorCommand::SetPage { index, text } = self;
        format!("set-page index={index} text={}", text.replace('\\', "\\\\").replace('\n', "\\n").replace(' ', "\\s"))
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let rest = line.strip_prefix("set-page ").ok_or_else(|| store::TextError::new(format!("pptx transitional editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut index = None;
        let mut text = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("pptx transitional editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\n", "\n").replace("\\\\", "\\");
            match key {
                "index" => index = decoded.parse::<u32>().ok(),
                "text" => text = decoded,
                _ => {}
            }
        }
        let index = index.ok_or_else(|| store::TextError::new("pptx transitional editor command: missing index", dsl::TextSpan::at(1, 1)))?;
        Ok(PptxTransitionalEditorCommand::SetPage { index, text })
    }
}

impl protocol::OpBinary for PptxTransitionalEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx transitional editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx transitional editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️Helpers
/// 🧮️ The FIRST `TextBox`/`Placeholder` shape on a slide — the only shape `set-page` can ever
/// address (see `handle`'s own doc comment for the honest multi-shape scope note).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn first_text_shape_index(slide: &PptxSlide) -> Option<usize> {
    slide.shapes.iter().position(|shape| matches!(shape, PptxShape::TextBox { .. } | PptxShape::Placeholder { .. }))
}

/// 🧮️ Pure `set-page` -> `PptxMutation` mapping, standalone so it is directly unit-testable
/// without constructing a full `ArtifactView`. `None` covers "slide index out of range" and "no
/// text-bearing shape on that slide" — both documented no-ops.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_set_page_mutation(snapshot: &PptxSnapshot, slide_index: usize, text: &str) -> Option<PptxMutation> {
    let slide = snapshot.presentation.slides.get(slide_index)?;
    let shape_index = first_text_shape_index(slide)?;
    let text_frame: Vec<PptxParagraph> = text.split('\n').map(PptxParagraph::text).collect();
    Some(PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, text_frame }))
}
//#endregion 🔖️Helpers

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct PptxTransitionalEditor;

impl ArtifactEditor for PptxTransitionalEditor {
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
    type Command = PptxTransitionalEditorCommand;

    const DIALECT: Dialect = PPTX_TRANSITIONAL_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PPTX_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PptxSnapshot {
        PptxSnapshot::default()
    }

    /// ✏️ `set-page` writes `text` (split on `\n` into one `PptxParagraph` per line, each a single
    /// plain run) into the FIRST text-bearing shape on the addressed slide, via
    /// `PptxMutation::SetShapeText`. A slide's rendered page text is the CONCATENATION of every
    /// text-bearing shape (see the window's own `render`), but only shape 0 is writable through this
    /// simple page view — a multi-shape slide's other shapes are read-only here; a real per-shape
    /// editor is future work, not faked. A slide with no text-bearing shape, or an out-of-range
    /// `index`, is a documented no-op (`Emit::default()`).
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let PptxTransitionalEditorCommand::SetPage { index, text } = command;
        let slide_index = *index as usize;
        match build_set_page_mutation(doc.snapshot, slide_index, text) {
            Some(mutation) => Ok(Emit { artifact_mutations: vec![mutation], description: Some(format!("Set page {slide_index}")), ..Default::default() }),
            None => Ok(Emit::default()),
        }
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
pub fn create_pptx_transitional_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PPTX_TRANSITIONAL_EDITOR_DIALECT)
        .document(["semio", "stdio", "pptx"])
        .icon_id("presentation")
        .mode_def(edit::definition())
        .default_mode_id(edit::PPTX_TRANSITIONAL_EDIT_MODE_ID)
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
    async fn create_pptx_transitional_editor_builds_a_definition_for_the_editor_role() {
        let def = create_pptx_transitional_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, PPTX_TRANSITIONAL_EDITOR_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PptxTransitionalEditor as ArtifactEditor>::DIALECT, PPTX_TRANSITIONAL_EDITOR_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_document_window() {
        let def = create_pptx_transitional_editor();
        assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_page_writes_the_first_text_bearing_shape_only() {
        let mut snapshot = PptxSnapshot::default();
        snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }, PptxShape::TextBox { text_frame: vec![PptxParagraph::text("old")], position: Default::default() }] });
        let mutation = build_set_page_mutation(&snapshot, 0, "new line one\nnew line two").expect("mutation");
        let PptxMutation::SetShapeText(set_shape_text::SetShapeText { slide_index, shape_index, text_frame }) = &mutation else { panic!("expected SetShapeText") };
        assert_eq!(*slide_index, 0);
        assert_eq!(*shape_index, 1);
        assert_eq!(text_frame.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_page_on_a_slide_with_no_text_shape_is_a_documented_no_op() {
        let mut snapshot = PptxSnapshot::default();
        snapshot.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::Picture { blip_rel_id: "rId1".into(), position: Default::default() }] });
        assert!(build_set_page_mutation(&snapshot, 0, "text").is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_roundtrip() {
        let command = PptxTransitionalEditorCommand::SetPage { index: 3, text: "a\nmulti line value".into() };
        let printed = <PptxTransitionalEditorCommand as protocol::OpText>::print_op(&command);
        let parsed = <PptxTransitionalEditorCommand as protocol::OpText>::parse_op(&printed).expect("parse ok");
        assert_eq!(parsed, command);
    }
}
//#endregion 🧪️Tests
