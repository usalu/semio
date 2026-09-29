//! 🌦️ EPW editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.epw@energyplus/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). `EpwSnapshot` is a flat lossless
//! record: 8 verbatim header lines + a `Vec<EpwRecord>` of hourly weather rows. One window,
//! `🪟️main` (`TableWindowKit`), renders the record list directly — no composed-child indirection is
//! needed since `EpwSnapshot` already IS the persisted document. The window's `set-cell` action
//! funnels through the one typed command this surface declares, `EpwEditorCommand::SetCell`, which
//! maps a column name to its canonical wire index (`EpwRecord::field_at`) and emits
//! `EpwMutation::SetRecordField` directly — no whole-document decode/re-encode round trip is needed
//! (unlike `energy`'s composed-child `Model`, `EpwSnapshot`'s fields ARE the wire fields).

use crate::editor::epw::modes::edit;
use crate::editor::epw::modes::edit::windows::main;
use crate::standards::energyplus::subsets::any::schema::mutations::{set_record_field, set_snapshot};
use crate::{EpwMutation, EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
};
use store::EngineHandles;

//#region 🔖️Dialect
/// 🎯️ This surface's dialect coordinate — `s.stdio.epw@energyplus/*`, verified against this
/// artifact's own `🏅️standards/🔖️energyplus/🪆️subsets/✳️any` location on disk. No reusable
/// `pub const DIALECT` exists on the artifact's own root `🦀️.rs` (checked before adding
/// this), so it is inlined here and reused for both `impl ArtifactEditor` and the manifest below.
pub const EPW_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit the `🪟️main` window's
/// `editable_window_kind()` action (`set-cell`, contract §2.6) can trigger. Scope note: only the 35
/// per-record columns are addressable — the 8 verbatim header lines (LOCATION, DESIGN CONDITIONS, …)
/// have no cell in a flat record table, so they are not yet editable through this surface
/// (documented honestly, matching energy's own `SetStructureField` scope note).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum EpwEditorCommand {
    #[dsl(key = "set-record-cell")]
    SetCell { row: u32, column: String, revision: String, value: String },
}

//#region 🔖️OpCodec
/// 🎯️ Handcrafted (P6: `#[derive(dsl::DslOps)]` emits `DslVariants` only — `OpText`/`OpBinary` are
/// handcrafted per artifact). Same shape as `energy`'s `EnergyModelEditorCommand`.
impl protocol::OpText for EpwEditorCommand {
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

impl protocol::OpBinary for EpwEditorCommand {
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
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}
semio_s_artifact_stdio_contract::snapshot_editing_command_roster!(EpwEditorCommand, ["set-cell"]);
//#endregion 🔖️OpCodec
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct EpwEditor;

impl ArtifactEditor for EpwEditor {
    type Snapshot = EpwSnapshot;
    type Mutation = EpwMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand<EpwEditorCommand>;

    const DIALECT: Dialect = EPW_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_EPW_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_details_editor_support! {
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.epw@energyplus/*#editor",
        artifact_schema: "stdio.epw",
        preparation: "stdio-epw-snapshot-edit",
        bounded_native: true
    }

    fn command_id(command: &Self::Command) -> &'static str {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_id(command, |_| "set-cell")
    }

    fn agent_target_revision(_action: &str, args: &dsl::DslValue, doc: &semio_framework_plugin::ArtifactView<'_, Self::Snapshot>) -> Result<Option<String>, Fault> {
        epw_agent_target_revision(doc.snapshot, args)
    }
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_editing_command_from_action(action, args, |action, args| match action {
            "set-cell" => {
                let column_index = semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")? as usize;
                let column = main::EPW_TABLE_COLUMNS.get(column_index).ok_or_else(|| {
                    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.column-range"), format!("EPW column index {column_index} is outside the {} editable fields", main::EPW_TABLE_COLUMNS.len()))
                })?;
                Ok(EpwEditorCommand::SetCell {
                    row: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?,
                    column: (*column).to_string(),
                    revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
                    value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
                })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.unhandled-action"), format!("unknown epw editor action '{other}'"))),
        })
    }

    fn initial_snapshot() -> EpwSnapshot {
        crate::standards::energyplus::subsets::any::schema::blank_epw_snapshot()
    }

    /// ✏️ Resolves the addressed column to its canonical wire index and emits one
    /// `EpwMutation::SetRecordField` through the same guarded reducer as retained execution.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(EpwEditorCommand::SetCell { row, column, revision, value }) = command else {
            let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) = command else { unreachable!() };
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        };
        epw_set_cell_emit(doc.snapshot, *row, column, revision, value)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.epw@energyplus/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

pub(crate) fn epw_row_revision(record: &crate::standards::energyplus::subsets::any::schema::snapshot::EpwRecord) -> String {
    semio_framework_plugin::app::DocumentWindowKit::text_revision(&semio_s_artifact_stdio_contract::editing::snapshot_edit_source(record))
}

/// 🔐️ The token an agent's omitted `set-cell` revision is admitted against: the addressed record's row token, as its rendered
/// binding carries it; a row out of range is refused exactly as the edit itself would refuse it.
fn epw_agent_target_revision(snapshot: &EpwSnapshot, args: &dsl::DslValue) -> Result<Option<String>, Fault> {
    let row = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "row")?;
    let record = snapshot
        .records
        .get(row as usize)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.row-range"), format!("EPW row {row} is outside the {} available records", snapshot.records.len())))?;
    Ok(Some(epw_row_revision(record)))
}

fn epw_set_cell_emit(snapshot: &EpwSnapshot, row: u32, column: &str, revision: &str, value: &str) -> Result<Emit<EpwMutation>, Fault> {
    let field_index = main::EPW_TABLE_COLUMNS
        .iter()
        .position(|candidate| candidate == &column)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.column-range"), format!("EPW column '{column}' is not an editable record field")))?;
    let record = snapshot
        .records
        .get(row as usize)
        .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.row-range"), format!("EPW row {row} is outside the {} available records", snapshot.records.len())))?;
    if epw_row_revision(record) != revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.epw.row-conflict"), format!("EPW row {row} changed before this cell draft was applied")));
    }
    Ok(Emit { artifact_mutations: vec![EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: row as usize, field_index, value: value.to_string() })], description: Some(format!("Set {column}")), ..Default::default() })
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for EpwEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&semio_s_artifact_stdio_contract::editing::SnapshotEditEvent> {
        match command {
            semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Edit(event) => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| EpwMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
    }
}

semio_s_artifact_stdio_contract::bounded_native_editing_editor! {
    editor: EpwEditor,
    tools: ["set-cell"],
    payload_schema: "semio.stdio.epw-cell-edit-command.v1",
    reduce: |command, snapshot| {
        let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(EpwEditorCommand::SetCell { row, column, revision, value }) = command else {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("stdio.epw.command-mismatch"),
                "EPW cell editing received another command",
            ));
        };
        epw_set_cell_emit(snapshot, *row, column, revision, value)
    },
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_epw_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(EPW_EDITOR_DIALECT)
        .document(["stdio", "epw"])
        .icon_id("cloud-sun")
        .mode_def(edit::definition())
        .default_mode_id(edit::EPW_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Weather"));
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
