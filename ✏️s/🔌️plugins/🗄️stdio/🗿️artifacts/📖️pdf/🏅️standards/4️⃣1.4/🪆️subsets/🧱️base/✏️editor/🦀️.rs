//! ✏️ PDF 1.4 resolved-page editor with addressed text drafts and own page mutations.

use crate::editor::pdf14::modes::edit;
use crate::editor::pdf14::modes::edit::windows::main;
use crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot;
use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::{PDF_ARTIFACT_SCHEMA_ID, STDIO_PDF_DOCUMENT_SCHEMA};
use crate::editor::pdf14::page;
use {semio_framework_plugin::built_to_component_tree,semio_framework_plugin::ArtifactEditor,semio_framework_plugin::ArtifactView,semio_framework_plugin::ComponentTree,semio_framework_plugin::ConfigView,semio_framework_artifact_reference::Dialect,semio_framework_plugin::DraftView,semio_framework_plugin::Editor,semio_framework_plugin::Emit,semio_framework_plugin::Fault,semio_framework_plugin::NoConfig,semio_framework_plugin::NoConfigMutation,semio_framework_plugin::NoDraft,semio_framework_plugin::NoDraftMutation,semio_framework_plugin::NoPresence,semio_framework_plugin::NoPresenceMutation,semio_framework_plugin::NoTransient,semio_framework_plugin::NoTransientMutation,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.4/*` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Duplicated verbatim in the sibling read-only surface (no shared constant lives outside these two surfaces).
pub const PDF14_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.4"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel replaces one explicitly addressed page's faithful
/// Unicode text projection after its optimistic revision matches.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Pdf14EditorCommand {
    #[dsl(key = "set-page")]
    SetPage { page: u32, item: u32, revision: String, text: String },
    #[dsl(key = "page-edit")]
    PageEdit { action: String, payload: String },
    #[dsl(key = "replace-page-text")]
    ReplacePageText { page: u32, revision: String, text: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only -- `OpText`/`OpBinary` are
/// handcrafted per artifact, same shape as the energy exemplar's own `EnergyModelEditorCommand`).
impl protocol::OpText for Pdf14EditorCommand {
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

impl protocol::OpBinary for Pdf14EditorCommand {
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
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(Pdf14EditorCommand, ["set-page", "insert-page", "remove-page", "move-page", "set-page-size", "replace-page-text"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Pdf14Editor;

impl ArtifactEditor for Pdf14Editor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<Pdf14EditorCommand>;

    const DIALECT: Dialect = PDF14_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.pdf@1.4/*#editor",
        artifact_schema: "stdio.pdf",
        preparation: "stdio-pdf-1-4-base-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            Pdf14EditorCommand::SetPage { .. } => "set-page",
            Pdf14EditorCommand::ReplacePageText { .. } => "replace-page-text",
            Pdf14EditorCommand::PageEdit { action, .. } => page::static_action(action),
        })
    }

    fn agent_target_revision(action: &str, args: &semio_framework_value::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        page::agent_target_revision(action, doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(Pdf14EditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            "replace-page-text" => Ok(Pdf14EditorCommand::ReplacePageText { page: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "page")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?, text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? }),
            other if page::is_page_action(other) => {
                let payload = page::edit_from_action(other, args)?;
                Ok(Pdf14EditorCommand::PageEdit { action: other.into(), payload })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pdf.unhandled-action"), format!("unknown pdf editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> PdfSnapshot {
        PdfSnapshot::default()
    }

    /// ✏️ Applies own page mutations after addressed text revision admission.
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
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(doc.snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
                        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::ReplacePageText { page, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(doc.snapshot, *page, 0, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::PageEdit { action, payload }) => {
                page::emit_page_edit(doc.snapshot, action, payload)
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.pdf@1.4/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}"))),
        }
    }

}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for Pdf14Editor {
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
    editor: Pdf14Editor,
    tools: ["set-page", "insert-page", "remove-page", "move-page", "set-page-size", "replace-page-text"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::ReplacePageText { page, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(snapshot, *page, 0, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::PageEdit { action, payload }) => {
                page::emit_page_edit(snapshot, action, payload)
            }
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),
        }
    },
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf14_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(PDF14_DIALECT);
    let builder = builder.document(["stdio", "pdf", "1.4", "any"]);
    let builder = builder.icon_id("file-text");
    let builder = builder.mode_def(edit::definition());
    let builder = builder.default_mode_id(edit::PDF14_EDIT_MODE_ID);
    let builder = builder.window_kind_def(main::definition());
    let builder = builder.window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition());
    let builder = builder.default_layout(page::document_layout(main::WINDOW_KIND_ID));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
