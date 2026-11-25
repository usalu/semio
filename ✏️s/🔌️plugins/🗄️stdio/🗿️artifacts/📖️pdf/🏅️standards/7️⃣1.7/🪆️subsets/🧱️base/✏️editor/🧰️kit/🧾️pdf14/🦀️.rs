//! 🧰️ The shared PDF 1.4 subset editor root, one declarative definition expanded by every subset root.

/// 📖️ Expands one PDF 1.4 subset editor root: command channel, editor wiring, view and presence lanes, native work, action roster and manifest; the call site brings `edit`, `main` and `page` into scope.
#[macro_export]
macro_rules! pdf14_editor {
    ($command:ident, $editor:ident, $dialect:ident, $create:ident, $mode_id:ident, $subset:literal, $document:literal, $format:literal, $controller:literal, $owner:literal, $preparation:literal $(,)?) => {

use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
use crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;
pub use semio_s_artifact_stdio_pdf14_canvas as canvas;
use {semio_plugin::built_to_component_tree,semio_plugin::ArtifactEditor,semio_plugin::ArtifactView,semio_plugin::ComponentTree,semio_plugin::ConfigView,semio_framework_artifact_reference::Dialect,semio_plugin::DraftView,semio_plugin::Editor,semio_plugin::Emit,semio_plugin::Fault,semio_plugin::NoConfig,semio_plugin::NoConfigMutation,semio_plugin::NoDraft,semio_plugin::NoDraftMutation,semio_plugin::NoTransient,semio_plugin::NoTransientMutation,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_2d_compute::EngineHandles;

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- this editor's surface-id -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Duplicated verbatim in the sibling read-only surface (no shared constant lives outside these two surfaces).
pub const $dialect: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.4"), subset: SubsetId($subset) };

/// ✏️ The editor's typed command channel replaces one explicitly addressed page's faithful
/// Unicode text projection after its optimistic revision matches.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum $command {
    #[dsl(key = "set-page")]
    SetPage { page: u32, item: u32, revision: String, text: String },
    #[dsl(key = "page-edit")]
    PageEdit { action: String, payload: String },
    #[dsl(key = "replace-page-text")]
    ReplacePageText { page: u32, revision: String, text: String },
}

/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only -- `OpText`/`OpBinary` are
/// handcrafted per artifact, same shape as the energy exemplar's own `EnergyModelEditorCommand`).
impl protocol::OpText for $command {
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
        match variants.iter().find(|(k, _)| k == &keyword) {
            Some((_, spec_fn)) => semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline),
            None => keyword.to_string(),
        }
    }
}

impl protocol::OpBinary for $command {
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
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!($command, ["set-page", "replace-page-text", "insert-page", "remove-page", "move-page", "set-page-size", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "set-text", "zoom-in", "zoom-out", "fit-page", "fit-width", "actual-size", "go-to-page", "set-query", "toggle-match-case", "next-hit", "previous-hit", "presence-pointer", "presence-pointer-leave", "presence-caret", "presence-caret-clear"]);
semio_s_artifact_stdio_pdf_view_lane::view_native!($command);

#[derive(Default, Clone, Copy)]
pub struct $editor;

impl ArtifactEditor for $editor {
    type Snapshot = PdfSnapshot;
    type Mutation = PdfMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_s_artifact_stdio_pdf_presence_lane::PdfPresence;
    type PresenceMutation = semio_s_artifact_stdio_pdf_presence_lane::PdfPresenceMutation;
    type Transient = semio_s_artifact_stdio_pdf_view_lane::PdfViewTransient;
    type TransientMutation = semio_s_artifact_stdio_pdf_view_lane::PdfViewTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<$command>;

    const DIALECT: Dialect = $dialect;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF_DOCUMENT_SCHEMA;

    const SINGLE_TURN_NATURAL_DECODE: bool = true;
    semio_s_artifact_stdio_pdf_view_lane::view_lane_stores!();
    semio_s_artifact_stdio_pdf_presence_lane::presence_lane_stores!();

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_plugin::__kernel::semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_plugin::Fault> {
        semio_plugin::__kernel::semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_plugin::Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("stdio.pdf.child-projection"), error.to_string()))
    }

    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<PdfSnapshot, PdfMutation>()
    }

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec { format_kind: $format, extension: ".pdf", media_type: "application/pdf", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::encode_pdf(snapshot).map_err(|error| semio_plugin::MediaError::Payload(semio_plugin::NATURAL_FILE_PORT.into(), error))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::decode_pdf_lossless(bytes).map_err(|error| semio_plugin::MediaError::Payload(semio_plugin::NATURAL_FILE_PORT.into(), error))
    }

    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: $owner,
        controller: $controller,
        artifact_schema: "stdio.pdf",
        preparation: $preparation,
        document_store_owners: PdfMutation::document_store_owners,
        bounded_native: true
    }

    fn fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {
        canvas::pdf_fault_notices()
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            $command::SetPage { .. } => "set-page",
            $command::ReplacePageText { .. } => "replace-page-text",
            $command::PageEdit { action, .. } => canvas::static_action(action),
        })
    }

    fn agent_target_revision(action: &str, args: &semio_framework_value::DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        page::agent_target_revision(action, doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok($command::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            "replace-page-text" => Ok($command::ReplacePageText { page: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "page")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?, text: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "text")? }),
            other if canvas::is_page_action(other) => {
                let payload = canvas::edit_from_action(other, args)?;
                Ok($command::PageEdit { action: other.into(), payload })
            }
            other => Err(canvas::refuse(canvas::PdfFault::UnhandledAction, format!("unknown pdf editor action '{other}'")).with_param("action", other)),
        })
    }

    fn reference_birth()->Option<fn()->Result<PdfSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(PdfSnapshot::default())
    })}

    /// ✏️ Applies own page mutations after addressed text revision admission.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(doc.snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
                        semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::ReplacePageText { page, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(doc.snapshot, *page, 0, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::PageEdit { action, payload }) => {
                canvas::emit_page_edit(doc.snapshot, action, payload)
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<ComponentTree> {
        if let Some(tree) = canvas::render_body(body_key, doc, $controller, None, None, view_state.locale) { return tree; }
        match body_key {
            main::BODY_KEY => main::render_windowed(doc.snapshot, semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?, &semio_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), view_state.locale).map(built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                $controller,
                &semio_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(canvas::pdf_editor_labels(view_state.locale).unknown_body.fill(&[("body", body_key)])),
        }
    }


    fn render_with_peers(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_plugin::app::InteractionView<'_>,
        peers: &semio_plugin::PresencePeersView<'_, Self::Presence>,
        _cx: &mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        let rendered = match canvas::render_body_peers(body_key, doc, $controller, interaction, transient.snapshot, view_state.locale, peers.iter()) {
            Some(tree) => tree,
            None => Self::render(body_key, doc, cfg, view_state),
        };
        rendered.map(Some).map_err(Into::into)
    }

    fn render_with_request_context(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_plugin::app::InteractionView<'_>,
        _cx: &mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        let rendered = match canvas::render_body(body_key, doc, $controller, Some(interaction), Some(transient.snapshot), view_state.locale) {
            Some(tree) => tree,
            None => Self::render(body_key, doc, cfg, view_state),
        };
        rendered.map(Some).map_err(Into::into)
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for $editor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &crate::standards::v1_4::subsets::base::schema::mutations::EDIT_RULES
    }

    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        crate::standards::v1_4::subsets::base::schema::mutations::special_edit(event, snapshot)
    }
}

fn native_work(tool: &'static str) -> Box<dyn semio_plugin::retained_command::ArtifactCommandWork<semio_plugin::EditorApp<$editor>>> {
    canvas::native_work::<$editor>(tool)
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: $editor,
    tools: ["set-page", "replace-page-text", "insert-page", "remove-page", "move-page", "set-page-size", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "set-text"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::SetPage { page, item, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(snapshot, *page, *item, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::ReplacePageText { page, revision, text }) => {
                let Some(mutation) = page::text_edit_mutation(snapshot, *page, 0, revision, text)? else { return Ok(Emit::default()) };
                Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
            }
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native($command::PageEdit { action, payload }) => {
                canvas::emit_page_edit(snapshot, action, payload)
            }
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),
        }
    },
    transient_tools: ["zoom-in", "zoom-out", "fit-page", "fit-width", "actual-size", "go-to-page", "set-query", "toggle-match-case", "next-hit", "previous-hit", "presence-pointer", "presence-pointer-leave", "presence-caret", "presence-caret-clear"],
    work: native_work,
}


// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn $create() -> semio_plugin::AppDefinition {
    let builder = Editor::builder($dialect);
    let builder = builder.document(["stdio", "pdf", "1.4", $document]);
    let builder = builder.icon_id("file-text");
    let builder = builder.mode_def(edit::definition());
    let builder = builder.default_mode_id(edit::$mode_id);
    let builder = builder.window_kind_def(main::definition());
    let builder = canvas::window_kinds().into_iter().fold(builder, |builder, kind| builder.window_kind_def(kind));
    let builder = builder.window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition());
    let builder = builder.default_layout(canvas::document_layout(main::WINDOW_KIND_ID));
    let mut definition = semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(canvas::edit_actions_with(builder)).build_definition();
    canvas::install_object_interaction(&mut definition);
    definition
}



    };
}
