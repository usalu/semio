//! 📄️ PDF Document (1.7) editor -- one of stdio's 10 real PDF subset editors (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). All 10 PDF dialects (standards 1.4/1.7 x
//! their real subsets) share ONE unified logical object model -- `crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot`/
//! `PdfMutation`, canonically the 1.7-shaped page/object/trailer graph (1.7 folds 1.0-1.7 in
//! leniently, per that standard's own doc comment) -- the artifact kind root re-exports both bare,
//! and its own `document_codec_bare::<PdfSnapshot, PdfMutation>(...)` call binds them to the 1.7
//! document schema id (confirmed by direct read before writing this file). `Pdf17Editor`
//! implements `ArtifactEditor` over that shared type, tagged with this file's own `PDF17_DIALECT`
//! (`standard: "1.7"`, `subset: "*"`) -- subsets are constraint PROFILES of the same
//! object model (validated on write by each subset's own IO validator), never a separate schema. One
//! real window, `main` (`DocumentWindowKit`) -- see its own module doc comment for the render/
//! mutation-mapping strategy and its honest scope limit.

use crate::editor::pdf17::modes::edit;
use crate::editor::pdf17::modes::edit::windows::main;
use crate::editor::page::page_text_edit_mutations;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
use {semio_plugin::built_to_component_tree,semio_plugin::ArtifactEditor,semio_plugin::ArtifactView,semio_plugin::ComponentTree,semio_plugin::ConfigView,semio_framework_artifact_reference::Dialect,semio_plugin::DraftView,semio_plugin::Editor,semio_plugin::Emit,semio_plugin::Fault,semio_plugin::NoConfig,semio_plugin::NoConfigMutation,semio_plugin::NoDraft,semio_plugin::NoDraftMutation,semio_plugin::NoTransient,semio_plugin::NoTransientMutation,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_stdio_pdf_editor_document::PickerPngWork;

#[path = "📥️local-image/🦀️.rs"]
mod image_import;

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract: this file's own surface-id
/// coordinate -- `s.stdio.pdf@1.7/*` -- measured directly against `PDF_ARTIFACT_SCHEMA_ID`
/// and this file's own on-disk standard/subset location. Duplicated verbatim in the sibling read-only surface (no shared constant lives outside these two surfaces).
pub const PDF17_DIALECT: Dialect = Dialect { artifact_kind: PDF_ARTIFACT_SCHEMA_ID, standard: StandardId("1.7"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel replaces one explicitly addressed page's faithful
/// Unicode text projection after its optimistic revision matches.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Pdf17EditorCommand {
    #[dsl(key = "set-page")]
    SetPage { page: u32, item: u32, revision: String, text: String },
    #[dsl(key = "page-edit")]
    PageEdit { action: String, payload: String },
    #[dsl(key = "pick-image")]
    PickImage { page: u32, operator: u32, x: f64, y: f64, width: f64, height: f64, guard: String },
    #[dsl(key = "import-local-image")]
    ImportImage { page: u32, operator: u32, revision: String, x: f64, y: f64, width: f64, height: f64, payload: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only -- `OpText`/`OpBinary` are
/// handcrafted per artifact, same shape as the energy exemplar's own `EnergyModelEditorCommand`).
impl protocol::OpText for Pdf17EditorCommand {
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

impl protocol::OpBinary for Pdf17EditorCommand {
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
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(Pdf17EditorCommand, ["zoom-in", "zoom-out", "fit-page", "fit-width", "actual-size", "go-to-page", "set-query", "toggle-match-case", "next-hit", "previous-hit", "set-page", "set-text", "move", "resize", "delete", "set-fill", "set-stroke", "insert-text", "insert-rectangle", "insert-line", "insert-image", "replace-image", "import-local-image", "insert-page", "remove-page", "move-page", "set-page-size", "set-info", "set-annotation", "set-image", "set-font", "set-font-size", "set-outline", "set-page-rotation", "set-page-box", "set-page-user-unit", "set-language", "set-page-layout", "set-page-mode", "set-optional-content", "set-embedded-file", "remove-embedded-file", "set-named-destination", "remove-named-destination", "set-page-label", "set-mark-info", "set-metadata", "set-viewer-preferences", "set-encryption", "set-output-intent", "set-form-field", "set-open-action", "set-document-id", "set-font-reference", "set-graphics-state", "set-pattern", "set-color-space", "set-properties", "set-font-metrics", "set-image-mask", "set-form-content", "set-page-transition", "set-catalog-entry", "set-trailer-entry", "set-annotation-appearance", "set-glyph", "set-indirect-object", "set-mesh-reference", "set-info-field", "set-page-extra", "set-annotation-style", "set-annotation-border", "set-annotation-markup", "set-form-settings", "set-extra-entry", "set-annotation-kind", "set-field-data", "set-resource-detail", "set-entry", "create-annotation", "fill-form-field", "replace-page", "replace-page-pick", "merge-document", "merge-pick", "extract-pages", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "presence-pointer", "presence-pointer-leave", "presence-caret", "presence-caret-clear"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

semio_s_artifact_stdio_pdf_view_lane::view_native!(Pdf17EditorCommand);

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Pdf17Editor;

fn emit_page_command(command: &Pdf17EditorCommand, snapshot: &PdfSnapshot) -> Option<Result<Emit<PdfMutation>, Fault>> {
    match command {
        Pdf17EditorCommand::SetPage { page, item, revision, text } => Some(page_text_edit_mutations(snapshot, *page, *item, revision, text)
            .map(|artifact_mutations| Emit { artifact_mutations, ..Default::default() })),
        Pdf17EditorCommand::PageEdit { action, payload } => Some(crate::editor::page::emit_page_edit(snapshot, action, payload)),
        _ => None,
    }
}

impl ArtifactEditor for Pdf17Editor {
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
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<Pdf17EditorCommand>;

    const DIALECT: Dialect = PDF17_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;
    semio_s_artifact_stdio_pdf_view_lane::view_lane_stores!();
    semio_s_artifact_stdio_pdf_presence_lane::presence_lane_stores!();

    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
    }
    const SINGLE_TURN_NATURAL_DECODE: bool = true;

    fn build_original_picker_job(
        request: &mut Option<semio_plugin::app::ArtifactReservedToolJobRequest<semio_plugin::EditorApp<Self>>>,
        body: &mut Option<Box<dyn semio_plugin::app::ArtifactReservedJob+Send>>,
        construction: &mut semio_plugin::__job::OriginalConstructionAdmission,
        cx: &mut semio_plugin::__job::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        semio_plugin::app::admit_reserved_work::<semio_plugin::EditorApp<Self>, PickerPngWork<semio_plugin::EditorApp<Self>>>(
            request, body, construction, cx,
        )
    }

    const PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = crate::standards::v1_7::subsets::base::io::import::PDF_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES;

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec { format_kind: "s.stdio.pdf@1.7", extension: ".pdf", media_type: "application/pdf", binary: true })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        crate::standards::v1_7::subsets::base::io::export::encode_pdf(snapshot).map_err(|error| semio_plugin::MediaError::Payload(semio_plugin::NATURAL_FILE_PORT.into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        crate::standards::v1_7::subsets::base::io::import::decode_pdf(bytes).map_err(|error| semio_plugin::MediaError::Payload(semio_plugin::NATURAL_FILE_PORT.into(), error.to_string()))
    }

    fn build_natural_file_decode_owner(bytes: Vec<u8>, contract: semio_plugin::ToolExecutionContract) -> Result<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>, semio_plugin::MediaError> {
        crate::standards::v1_7::subsets::base::io::import::build_pdf_natural_decode(bytes, contract)
    }

    fn admit_natural_file_decode_owner(
        bytes: &mut Option<Vec<u8>>, contract: semio_plugin::ToolExecutionContract,
        owner: &mut Option<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>>, context: &mut semio_plugin::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        crate::standards::v1_7::subsets::base::io::import::admit_pdf_natural_decode(bytes, contract, owner, context)
    }

    fn admit_paged_natural_file_decode_owner(
        source: &mut Option<semio_plugin::NaturalFilePagedSource>, contract: semio_plugin::ToolExecutionContract,
        owner: &mut Option<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>>, context: &mut semio_plugin::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        crate::standards::v1_7::subsets::base::io::import::admit_pdf_paged_natural_decode(source, contract, owner, context)
    }

    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs",
        controller: "s.stdio.pdf@1.7/*#editor",
        artifact_schema: "stdio.pdf.1.7",
        preparation: "stdio-pdf-1-7-base-snapshot-edit",
        document_store_owners: PdfMutation::document_store_owners,
        bounded_native: true,
        native_media_export: { factory: semio_s_artifact_stdio_pdf_native_save::factory::PdfNativeExportFactory<semio_plugin::EditorApp<Self>>, io: semio_s_artifact_stdio_pdf_native_save::factory::app_io, build: semio_s_artifact_stdio_pdf_native_save::job::PdfGraphSaveJob::<semio_plugin::EditorApp<Self>>::admit_original },
        native: {
            factory_name: "PdfLocalImageImport",
            factory_type: image_import::PdfLocalImageImport,
            contract: image_import::CONTRACT,
            build: image_import::build,
            tools: ["import-local-image"]
        }
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |native| match native {
            Pdf17EditorCommand::SetPage { .. } => "set-page",
            Pdf17EditorCommand::PageEdit { action, .. } => crate::editor::page::static_action(action),
            _ => image_import::command_id(native).expect("PDF image editor command id"),
        })
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        crate::editor::page::page_text_agent_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-page" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_document_text_edit(args)?;
                Ok(Pdf17EditorCommand::SetPage { page: edit.page, item: edit.item, revision: edit.revision, text: edit.text })
            }
            other if image_import::admits(other) => image_import::from_action(other, args),
            other if crate::editor::page::is_page_action(other) => {
                let edit = crate::editor::page::edit_from_action(other, args)?;
                Ok(Pdf17EditorCommand::PageEdit { action: edit.action, payload: edit.payload })
            }
            other => Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("stdio.pdf.unhandled-action"), format!("unknown pdf editor action '{other}'"))),
        })
    }

    fn reference_birth()->Option<fn()->Result<PdfSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(crate::standards::v1_7::subsets::base::io::export::blank_pdf_snapshot())
    })}

    /// ✏️ Replaces the addressed page's Unicode text projection while retaining encoded text,
    /// graphics, geometry, metadata, and every non-text content operator.
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
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) => emit_page_command(command, doc.snapshot)
                .unwrap_or_else(|| Err(Fault::from("PNG import requires its retained interactive job"))),
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<ComponentTree> {
        if let Some(tree) = semio_s_artifact_stdio_pdf_view_lane::render_page_body(body_key, doc, "s.stdio.pdf@1.7/*#editor", None, None, view_state.locale, main::render_windowed) {
            return tree;
        }
        match body_key {
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.pdf@1.7/*#editor",
                &semio_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(semio_ui_locale::Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn operation_progress_scope() -> semio_plugin::kernel::UiDirtyScope {
        semio_plugin::kernel::UiDirtyScope::Partial {
            window_bodies: vec![crate::editor::page::INSPECTOR_BODY_KEY.into()], panel_bodies: Vec::new(),
            utilities: false, tools: false, engagements: false, measures: false, labels: false,
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
        let rendered: semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> = (|| {
            if let Some(tree) = semio_s_artifact_stdio_pdf_view_lane::render_page_body_peers(body_key, doc.snapshot, &|| semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc), "s.stdio.pdf@1.7/*#editor", Some(interaction), Some(transient.snapshot), view_state.locale, main::render_windowed, &|interaction| semio_s_artifact_stdio_pdf_presence_lane::typed_samples(interaction, crate::editor::page::OBJECT_DOMAIN, &crate::editor::page::object_page_of(doc.snapshot), peers.iter())) {
                return tree;
            }
            Self::render(body_key, doc, cfg, view_state)
        })();
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
        cx:&mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        let rendered:semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree>=(||{
        if let Some(tree) = semio_s_artifact_stdio_pdf_view_lane::render_page_body(body_key, doc, "s.stdio.pdf@1.7/*#editor", Some(interaction), Some(transient.snapshot), view_state.locale, main::render_windowed) {
            return tree;
        }
        Self::render(body_key, doc, cfg, view_state)

        })();
        rendered.map(Some).map_err(Into::into)
        }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for Pdf17Editor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &crate::standards::v1_7::subsets::base::schema::mutations::EDIT_RULES
    }

    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        crate::standards::v1_7::subsets::base::schema::mutations::special_edit(event, snapshot)
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: Pdf17Editor,
    tools: ["set-page", "set-text", "move", "resize", "delete", "set-fill", "set-stroke", "insert-text", "insert-rectangle", "insert-line", "insert-page", "remove-page", "move-page", "set-page-size", "set-info", "set-annotation", "set-image", "set-font", "set-font-size", "set-outline", "set-page-rotation", "set-page-box", "set-page-user-unit", "set-language", "set-page-layout", "set-page-mode", "set-optional-content", "set-embedded-file", "remove-embedded-file", "set-named-destination", "remove-named-destination", "set-page-label", "set-mark-info", "set-metadata", "set-viewer-preferences", "set-encryption", "set-output-intent", "set-form-field", "set-open-action", "set-document-id", "set-font-reference", "set-graphics-state", "set-pattern", "set-color-space", "set-properties", "set-font-metrics", "set-image-mask", "set-form-content", "set-page-transition", "set-catalog-entry", "set-trailer-entry", "set-annotation-appearance", "set-glyph", "set-indirect-object", "set-mesh-reference", "set-info-field", "set-page-extra", "set-annotation-style", "set-annotation-border", "set-annotation-markup", "set-form-settings", "set-extra-entry", "set-annotation-kind", "set-field-data", "set-resource-detail", "set-entry", "create-annotation", "fill-form-field", "replace-page", "replace-page-pick", "merge-document", "merge-pick", "extract-pages", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp"],
    payload_schema: "semio.stdio.document-text-edit-command.v1",
    reduce: |command, snapshot| {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) => emit_page_command(command, snapshot)
                .unwrap_or_else(|| Err(Fault::from("stdio-pdf-native-edit-command-mismatch"))),
            _ => Err(Fault::from("stdio-pdf-native-edit-command-mismatch")),
        }
    },
    host_tools: ["insert-image", "replace-image"],
    transient_tools: ["zoom-in", "zoom-out", "fit-page", "fit-width", "actual-size", "go-to-page", "set-query", "toggle-match-case", "next-hit", "previous-hit", "presence-pointer", "presence-pointer-leave", "presence-caret", "presence-caret-clear"],
    work: image_import::native_work,
}

//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_pdf17_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(PDF17_DIALECT);
    let builder = builder.document(["stdio", "pdf", "1.7", "any"]);
    let builder = builder.icon_id("file-text");
    let builder = builder.mode_def(edit::definition());
    let builder = builder.default_mode_id(edit::PDF17_EDIT_MODE_ID);
    let builder = builder.window_kind_def(main::definition());
    let builder = builder.window_kind_def(crate::editor::page::inspector_window_definition());
    let builder = builder.window_kind_def(crate::editor::page::text_layer_window_definition());
    let builder = builder.window_kind_def(crate::editor::page::thumbnails_window_definition());
    let builder = builder.window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition());
    let builder = builder.default_layout(crate::editor::page::document_layout(main::WINDOW_KIND_ID));
    let mut definition = semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition();
    for window in definition.window_kinds.iter_mut() {
        if window.id == main::WINDOW_KIND_ID { window.actions.retain(|action| action.id != "insert-image"); }
    }
    definition.actions.extend(image_import::actions().into_iter().chain(std::iter::once(semio_s_artifact_stdio_pdf_native_save::factory::export_action())));
    crate::editor::page::install_object_interaction(&mut definition);
    definition
}
//#endregion 🔖️Manifest

//#region 🧪️Tests

//#endregion 🧪️Tests
