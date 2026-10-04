//! 📄️ PDF/H Document (1.7) editor -- one of stdio's 10 real PDF subset editors (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). All 10 PDF dialects (standards 1.4/1.7 x
//! their real subsets) share ONE unified logical object model -- `crate::PdfSnapshot`/
//! `PdfMutation`, canonically the 1.7-shaped page/object/trailer graph (1.7 folds 1.0-1.7 in
//! leniently, per that standard's own doc comment) -- the artifact kind root re-exports both bare,
//! and its own `document_codec_bare::<PdfSnapshot, PdfMutation>(...)` call binds them to the 1.7
//! document schema id (confirmed by direct read before writing this file). `Pdf17HEditor`
//! implements `ArtifactEditor` over that shared type, tagged with this file's own `PDF17H_DIALECT`
//! (`standard: "1.7"`, `subset: "h"`) -- subsets are constraint PROFILES of the same
//! object model (validated on write by each subset's own IO validator), never a separate schema. One
//! real window, `main` (`DocumentWindowKit`) -- see its own module doc comment for the render/
//! mutation-mapping strategy and its honest scope limit.

use crate::editor::pdf17h::modes::edit;
use crate::editor::pdf17h::modes::edit::windows::main;
use crate::standards::v1_7::subsets::base::schema::mutations::set_snapshot;
use crate::standards::v1_7::subsets::base::schema::mutations::patch_snapshot;
use crate::{page_text_edit_mutation, PdfMutation, PdfSnapshot, PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF17_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    built_to_component_tree, ArtifactEditor, ArtifactView, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, Fault, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId,
};
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.7/h` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Duplicated verbatim in the sibling read-only surface (no shared constant lives outside these two surfaces).
pub const PDF17H_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.7"), subset: SubsetId("h") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel replaces one explicitly addressed page's faithful
/// Unicode text projection after its optimistic revision matches.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Pdf17HEditorCommand {
    #[dsl(key = "set-page")]
    SetPage { page: u32, item: u32, revision: String, text: String },
    #[dsl(key = "page-edit")]
    PageEdit { action: String, payload: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only -- `OpText`/`OpBinary` are
/// handcrafted per artifact, same shape as the energy exemplar's own `EnergyModelEditorCommand`).
impl protocol::OpText for Pdf17HEditorCommand {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Pdf17HEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(Pdf17HEditorCommand, ["set-page", "set-text", "move", "resize", "delete", "set-fill", "set-stroke", "insert-text", "insert-rectangle", "insert-line", "insert-image", "insert-page", "remove-page", "move-page", "set-page-size", "set-info", "set-annotation", "set-image", "set-font", "set-outline", "set-page-rotation", "set-page-box", "set-page-user-unit", "set-language", "set-page-layout", "set-page-mode", "set-optional-content", "set-embedded-file", "remove-embedded-file", "set-named-destination", "remove-named-destination", "set-page-label", "set-mark-info", "set-metadata", "set-viewer-preferences", "set-encryption", "set-output-intent", "set-form-field", "set-open-action", "set-document-id", "set-font-program", "set-graphics-state", "set-pattern", "set-color-space", "set-properties", "set-font-metrics", "set-image-mask", "set-form-content", "set-page-transition", "set-catalog-entry", "set-trailer-entry", "set-annotation-appearance", "set-glyph", "set-indirect-object", "set-mesh-data", "set-info-field", "set-page-extra", "set-annotation-style", "set-annotation-border", "set-annotation-markup", "set-form-settings", "set-extra-entry", "set-annotation-kind", "set-field-data", "set-resource-detail", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Pdf17HEditor;

impl ArtifactEditor for Pdf17HEditor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<Pdf17HEditorCommand>;

    const DIALECT: Dialect = PDF17H_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs",
        controller: "s.stdio.pdf@1.7/h#editor",
        artifact_schema: "stdio.pdf.1.7",
        preparation: "stdio-pdf-1-7-h-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            Pdf17HEditorCommand::SetPage { .. } => "set-page",
            Pdf17HEditorCommand::PageEdit { action, .. } => crate::editor::page::static_action(action),
        })
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        crate::page_text_agent_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(Pdf17HEditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            other if crate::editor::page::is_page_action(other) => {
                let edit = crate::editor::page::edit_from_action(other, args)?;
                Ok(Pdf17HEditorCommand::PageEdit { action: edit.action, payload: edit.payload })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pdf.unhandled-action"), format!("unknown pdf editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> PdfSnapshot {
        crate::standards::v1_7::subsets::base::schema::snapshot::blank_pdf_snapshot()
    }

    /// ✏️ Replaces the addressed page's Unicode text projection while retaining encoded text,
    /// graphics, geometry, metadata, and every non-text content operator.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17HEditorCommand::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page_text_edit_mutation(doc.snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
                        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17HEditorCommand::PageEdit { action, payload }) => {
                crate::editor::page::emit_page_edit(doc.snapshot, action, payload)
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(built_to_component_tree),
            crate::editor::page::INSPECTOR_BODY_KEY => crate::editor::page::render_inspector(doc.snapshot, None, "s.stdio.pdf@1.7/h#editor", semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?, view_state.locale).map(built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.pdf@1.7/h#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        if body_key == main::BODY_KEY {
            return crate::editor::page::render_selected(doc.snapshot, interaction).map(built_to_component_tree);
        }
        if body_key == crate::editor::page::INSPECTOR_BODY_KEY {
            let selected = interaction.selection(crate::editor::page::OBJECT_DOMAIN).ids.first().map(String::as_str);
            return crate::editor::page::render_inspector(doc.snapshot, selected, "s.stdio.pdf@1.7/h#editor", semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?, view_state.locale).map(built_to_component_tree);
        }
        Self::render(body_key, doc, cfg, view_state)
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for Pdf17HEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(event, snapshot, |patch| PdfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }), Some(|snapshot| PdfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: Pdf17HEditor,
    tools: ["set-page", "set-text", "move", "resize", "delete", "set-fill", "set-stroke", "insert-text", "insert-rectangle", "insert-line", "insert-image", "insert-page", "remove-page", "move-page", "set-page-size", "set-info", "set-annotation", "set-image", "set-font", "set-outline", "set-page-rotation", "set-page-box", "set-page-user-unit", "set-language", "set-page-layout", "set-page-mode", "set-optional-content", "set-embedded-file", "remove-embedded-file", "set-named-destination", "remove-named-destination", "set-page-label", "set-mark-info", "set-metadata", "set-viewer-preferences", "set-encryption", "set-output-intent", "set-form-field", "set-open-action", "set-document-id", "set-font-program", "set-graphics-state", "set-pattern", "set-color-space", "set-properties", "set-font-metrics", "set-image-mask", "set-form-content", "set-page-transition", "set-catalog-entry", "set-trailer-entry", "set-annotation-appearance", "set-glyph", "set-indirect-object", "set-mesh-data", "set-info-field", "set-page-extra", "set-annotation-style", "set-annotation-border", "set-annotation-markup", "set-form-settings", "set-extra-entry", "set-annotation-kind", "set-field-data", "set-resource-detail", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17HEditorCommand::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page_text_edit_mutation(snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17HEditorCommand::PageEdit { action, payload }) => {
                crate::editor::page::emit_page_edit(snapshot, action, payload)
            }
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),
        }
    },
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf17_h_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(PDF17H_DIALECT);
    let builder = builder.document(["stdio", "pdf", "1.7", "h"]);
    let builder = builder.icon_id("file-text");
    let builder = builder.mode_def(edit::definition());
    let builder = builder.default_mode_id(edit::PDF17H_EDIT_MODE_ID);
    let builder = builder.window_kind_def(main::definition());
    let builder = builder.window_kind_def(crate::editor::page::inspector_window_definition());
    let builder = builder.window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition());
    let builder = builder.default_layout(crate::editor::page::document_layout(main::WINDOW_KIND_ID));
    let mut definition = semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition();
    crate::editor::page::install_object_interaction(&mut definition);
    definition
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
