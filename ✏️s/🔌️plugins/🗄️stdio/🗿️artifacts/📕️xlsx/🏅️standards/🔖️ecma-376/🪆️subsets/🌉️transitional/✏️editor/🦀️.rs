//! 📕️ Xlsx editor (ecma-376/🌉️transitional) — the first authored `ArtifactEditor` surface for
//! `s.stdio.xlsx@ecma-376/transitional`. Transitional reuses `🧱️base`'s own `XlsxSnapshot` verbatim
//! (same Rust type, same `s.stdio.xlsx` schema id). Its shared canonical editor renders windowed
//! worksheet grids and inserts cells into explicitly addressed vacancies.

use crate::standards::v_ecma_376::subsets::transitional::io;
use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit;
use crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::modes::edit::windows::main;
use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::edit_rules;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::{cell_address::xlsx_cell_address, set_cell};
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxCellValue;
use semio_s_artifact_stdio_xlsx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::XlsxMutation;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_XLSX_DOCUMENT_SCHEMA;
use semio_framework_2d_compute::EngineHandles;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::NoPresence;
use semio_plugin::NoPresenceMutation;
use semio_plugin::NoTransient;
use semio_plugin::NoTransientMutation;
use {semio_framework_artifact_reference::StandardId};
use {semio_framework_artifact_reference::SubsetId};

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this subset — `s.stdio.xlsx@ecma-376/transitional`. The sibling schema facet
/// (owned by the live peer ticket 26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-
/// MUTATIONS) already exports an identical `pub const DIALECT` at `🧬️schema/🦀️component.rs`, but
/// this ticket's own scope excludes importing across that boundary — restated here directly from
/// this ticket's own contract, matching `🧱️base`/`🔒️strict`'s sibling surfaces' identical restating.
pub const XLSX_TRANSITIONAL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("transitional") };
//#endregion 🔖️Dialect

//#region 🔖️TableProjection
//#endregion 🔖️TableProjection

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. The worksheet/row/column tuple is the durable
/// identity and `revision` guards the user's draft against a concurrent cell change.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum XlsxTransitionalEditorCommand {
    SetCell { sheet_name: String, row: u32, column: u32, revision: String, value: String },
}

//#region 🔖️OpBinaryCodec
/// 🎯️ Hand-rolled — only `protocol::OpBinary` is a trait bound on `ArtifactEditor::Command` (see
/// the framework trait's own `type Command: ::protocol::OpBinary + Send`); `OpText` is not required
/// and, with a single variant of two plain fields, would be pure ceremony here.
impl protocol::OpBinary for XlsxTransitionalEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value } = self;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut out, sheet_name.len() as u64);
        out.extend_from_slice(sheet_name.as_bytes());
        store::pack_rt::write_varint_u64(&mut out, *row as u64);
        store::pack_rt::write_varint_u64(&mut out, *column as u64);
        store::pack_rt::write_varint_u64(&mut out, revision.len() as u64);
        out.extend_from_slice(revision.as_bytes());
        store::pack_rt::write_varint_u64(&mut out, value.len() as u64);
        out.extend_from_slice(value.as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported format {format}")));
        }
        let sheet_len = reader.read_varint_u64().map_err(|e| malformed("op worksheet len", reader.position(), e.to_string()))? as usize;
        let sheet_name = String::from_utf8(reader.read_bytes(sheet_len).map_err(|e| malformed("op worksheet", reader.position(), e.to_string()))?.to_vec()).map_err(|e| malformed("op worksheet", reader.position(), e.to_string()))?;
        let row = reader.read_varint_u64().map_err(|e| malformed("op row", reader.position(), e.to_string()))? as u32;
        let column = reader.read_varint_u64().map_err(|e| malformed("op column", reader.position(), e.to_string()))? as u32;
        let revision_len = reader.read_varint_u64().map_err(|e| malformed("op revision len", reader.position(), e.to_string()))? as usize;
        let revision = String::from_utf8(reader.read_bytes(revision_len).map_err(|e| malformed("op revision", reader.position(), e.to_string()))?.to_vec()).map_err(|e| malformed("op revision", reader.position(), e.to_string()))?;
        let value_len = reader.read_varint_u64().map_err(|e| malformed("op value len", reader.position(), e.to_string()))? as usize;
        let value_bytes = reader.read_bytes(value_len).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        let value = String::from_utf8(value_bytes.to_vec()).map_err(|e| malformed("op value", reader.position(), e.to_string()))?;
        Ok(XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(XlsxTransitionalEditorCommand, ["set-cell"]);
//#endregion 🔖️OpBinaryCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct XlsxTransitionalEditor;

impl ArtifactEditor for XlsxTransitionalEditor {
    type Snapshot = XlsxSnapshot;
    type Mutation = XlsxMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<XlsxTransitionalEditorCommand>;

    const DIALECT: Dialect = XLSX_TRANSITIONAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_XLSX_DOCUMENT_SCHEMA;

    fn document_store_owners_source_demands() -> Result<semio_plugin::__value::RetirementDemand, semio_plugin::__value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
    }

    fn build_document_store_owners(
        grant: semio_plugin::__value::retained_clone::RetainedCloneGrant,
    ) -> Option<Result<
        (store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, semio_plugin::__value::retained_clone::RetainedCloneProgress),
        store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>,
    >> {
        Some(semio_plugin::cold_replay_document_store_owners::<Self::Snapshot, Self::Mutation>(grant).map(|(mut owners, progress)| {
            owners.operation_wire = XlsxMutation::install_original_history_wire(owners.operation_wire);
            (owners, progress)
        }))
    }

    /// 📂️ Opening a natural file or a document pack is the whole-document LOAD (genesis path), never a history mutation.
    fn import_media(port: &str, media: &semio_plugin::app::Media, _doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_contract::import_media_as_load::<Self>(port, media)
    }

    const SINGLE_TURN_NATURAL_DECODE: bool = true;

    const PAGED_NATURAL_SOURCE_MAXIMUM_BYTES: u32 = semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::import::controlled::XLSX_PAGED_NATURAL_SOURCE_MAXIMUM_BYTES;

    fn build_natural_file_decode_owner(bytes: Vec<u8>, contract: semio_plugin::ToolExecutionContract) -> Result<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>, semio_plugin::MediaError> {
        semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::import::controlled::build_xlsx_natural_decode(bytes, contract, io::natural_conformance)
    }

    fn admit_natural_file_decode_owner(
        bytes: &mut Option<Vec<u8>>, contract: semio_plugin::ToolExecutionContract,
        owner: &mut Option<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>>, context: &mut semio_plugin::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::import::controlled::admit_xlsx_natural_decode(bytes, contract, io::natural_conformance, owner, context)
    }

    fn admit_paged_natural_file_decode_owner(
        source: &mut Option<semio_plugin::NaturalFilePagedSource>, contract: semio_plugin::ToolExecutionContract,
        owner: &mut Option<semio_plugin::NaturalFileDecodeOwner<Self::Snapshot>>, context: &mut semio_plugin::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        semio_s_artifact_stdio_xlsx_base::standards::v_ecma_376::subsets::base::io::import::controlled::admit_xlsx_paged_natural_decode(source, contract, io::natural_conformance, owner, context)
    }

    fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
        Some(semio_plugin::NaturalFileCodec {
            format_kind: "s.stdio.xlsx@ecma-376",
            extension: ".xlsx",
            media_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            binary: true,
        })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
        io::encode_xlsx(snapshot)
            .map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
        io::decode_xlsx(bytes)
            .map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
    }

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs",
        controller: "s.stdio.xlsx@ecma-376/transitional#editor",
        artifact_schema: "stdio.xlsx",
        preparation: "stdio-xlsx-transitional-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-cell")
    }

    fn agent_target_revision(_action: &str, args: &semio_framework_value::DslValue, doc: &ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_agent_target_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-cell" => {
                let edit = semio_s_artifact_stdio_contract::window_kit_stable_cell_edit(args)?;
                Ok(XlsxTransitionalEditorCommand::SetCell { sheet_name: edit.sheet_name, row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
            }
            other => Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("stdio.xlsx.transitional.unhandled-action"), format!("unknown xlsx editor action '{other}'"))),
        })
    }

    fn reference_birth() -> Option<fn() -> Result<XlsxSnapshot, Fault>> {
        Some(|| {
            use semio_plugin::ArtifactBuilder;
            use semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::new_xlsx_editor_workbook;
            io::XlsxTransitionalBuilderConstruction::new(new_xlsx_editor_workbook())
                .build()
                .map_err(|error| semio_plugin::reference_birth_fault("valid authored transitional XLSX initial owner", format_args!("{error:?}")))
        })
    }

    /// ✏️ Resolves the rendered worksheet/row/column identity and optimistic revision, then emits
    /// one `XlsxMutation::SetCell`. Stale addresses and revisions return a fault without mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        xlsx_set_cell_emit(doc.snapshot, command, view_state.map(|view| view.locale))
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render(doc.snapshot, view_state.locale, &semio_plugin::TreeWindows::for_body(view_state, main::BODY_KEY), publication_revision).map(semio_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.xlsx@ecma-376/transitional#editor",
                &semio_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_editor_labels(view_state.locale).unknown_body.fill(&[("body", body_key)])),
        }
    }
}

fn xlsx_set_cell_emit(snapshot: &XlsxSnapshot, command: &XlsxTransitionalEditorCommand, locale: Option<semio_ui_locale::Locale>) -> Result<Emit<XlsxMutation>, Fault> {
    let XlsxTransitionalEditorCommand::SetCell { sheet_name, row, column, revision, value } = command;
    semio_s_artifact_stdio_xlsx_base::editor::xlsx::standards::v_ecma_376::subsets::base::xlsx_set_cell_emit_fields(snapshot, sheet_name, *row, *column, revision, value, locale, "stdio.xlsx.transitional")
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for XlsxTransitionalEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_rules() -> &'static semio_s_artifact_stdio_contract::editing::EditRules {
        &edit_rules::EDIT_RULES
    }

    fn snapshot_edit_special(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Option<Vec<Self::Mutation>>, Fault> {
        edit_rules::special(event, snapshot).map_err(|error| Fault::from(error.to_string()))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: XlsxTransitionalEditor,
    tools: ["set-cell"],
    payload_schema: "semio.stdio.xlsx-transitional-cell-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(command) = command else {
            return Err(Fault::from("stdio-xlsx-transitional-native-edit-command-mismatch"));
        };
        xlsx_set_cell_emit(snapshot, command, None)
    },
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_xlsx_transitional_editor() -> semio_plugin::AppDefinition {
    let builder = Editor::builder(XLSX_TRANSITIONAL_DIALECT)
        .document(["stdio", "xlsx", "transitional"])
        .icon_id("table")
        .mode_def(edit::definition())
        .default_mode_id(edit::XLSX_TRANSITIONAL_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
