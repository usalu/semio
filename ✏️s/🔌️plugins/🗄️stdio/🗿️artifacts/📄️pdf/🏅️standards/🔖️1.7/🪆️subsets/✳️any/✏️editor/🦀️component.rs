//! 📄️ PDF Document (1.7) editor -- one of stdio's 10 real PDF subset editors (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). All 10 PDF dialects (standards 1.4/1.7 x
//! their real subsets) share ONE unified logical object model -- `crate::artifacts::pdf::PdfSnapshot`/
//! `PdfMutation`, canonically the 1.7-shaped page/object/trailer graph (1.7 folds 1.0-1.7 in
//! leniently, per that standard's own doc comment) -- the artifact kind root re-exports both bare,
//! and its own `document_codec_bare::<PdfSnapshot, PdfMutation>(...)` call binds them to the 1.7
//! document schema id (confirmed by direct read before writing this file). `Pdf17Editor`
//! implements `ArtifactEditor` over that shared type, tagged with this file's own `PDF17_DIALECT`
//! (`standard: "1.7"`, `subset: "*"`) -- subsets are constraint PROFILES of the same
//! object model (validated on write by each subset's own IO validator), never a separate schema. One
//! real window, `main` (`DocumentWindowKit`) -- see its own module doc comment for the render/
//! mutation-mapping strategy and its honest scope limit.

use crate::artifacts::pdf::{PdfMutation, PdfSnapshot, PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};
use crate::editor::pdf17::modes::edit;
use crate::editor::pdf17::modes::edit::windows::main;
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId, UiNode,
};
use serde::{Deserialize, Serialize};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.7/*` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Duplicated verbatim in the sibling read-only surface (no shared constant lives outside these two surfaces).
pub const PDF17_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.7"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel -- the ONE edit `main`'s `editable_window_kind()` action
/// (`set-page`, contract §2.6) can trigger. See `main`'s own module doc comment for why this appends
/// to the page's existing text rather than replacing it (`PdfMutation` has no "replace" primitive).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Pdf17EditorCommand {
    #[dsl(key = "set-page")]
    SetPage { index: usize, text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only -- `OpText`/`OpBinary` are
/// handcrafted per artifact, same shape as the energy exemplar's own `EnergyModelEditorCommand`).
impl protocol::OpText for Pdf17EditorCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Pdf17EditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: error.to_string() })
    }
}
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Pdf17Editor;

impl ArtifactEditor for Pdf17Editor {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Pdf17EditorCommand;

    const DIALECT: Dialect = PDF17_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> PdfSnapshot {
        PdfSnapshot::default()
    }

    /// ✏️ `set-page` -> `PdfMutation::AppendPageContent` -- the closest real primitive `PdfMutation`
    /// exposes (see `main`'s own doc comment for why this appends rather than replaces). An
    /// out-of-range `index` is a documented no-op (`Emit::default()`), never a panic, matching
    /// `apply_pdf_mutation`'s own out-of-range-is-noop contract.
    async fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            Pdf17EditorCommand::SetPage { index, text } => {
                if doc.snapshot.pages.get(*index).is_none() {
                    return Ok(Emit::default());
                }
                Ok(Emit { artifact_mutations: vec![PdfMutation::AppendPageContent { index: *index, text: text.clone() }], description: Some(format!("Set page {index}")), ..Default::default() })
            }
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf17_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PDF17_DIALECT)
        .document(["stdio", "pdf", "1.7", "any"])
        .icon_id("file-text")
        .mode_def(edit::definition())
        .default_mode_id(edit::PDF17_EDIT_MODE_ID)
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
    async fn create_pdf17_editor_builds_a_definition_for_the_editor_role() {
        let def = create_pdf17_editor();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
        assert_eq!(def.dialect, PDF17_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Pdf17Editor as ArtifactEditor>::DIALECT, PDF17_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_declares_the_main_window() {
        let def = create_pdf17_editor();
        assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
    }
}
//#endregion 🧪️Tests
