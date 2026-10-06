//! ✏️ Csv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.csv@rfc4180/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `CsvSnapshot.records` through the artifact's own
//! `CsvMutation::SetField`.

use crate::editor::csv::modes::edit;
use crate::editor::csv::modes::edit::windows::main;
use crate::{CsvField, CsvMutation, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactStoreInitializationJob;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Dialect;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
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
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolOperationSpec;
use semio_framework_ui_locale::Label;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against `crate::schema::derived_analysis::
/// CsvAnalyzerAnalysis::DIALECT` (the artifact's own real analysis-capability row), not guessed.
/// Duplicated (not imported) in the sibling `👁️viewer` surface root — never shared through an
/// `editor`-rooted import, so a viewer file can never depend on this module.
pub const CSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index the rendered grid after the
/// header split and `revision` prevents a concurrent row shift from redirecting the edit.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum CsvEditorCommand {
    SetCell {
        row: u32,
        column: u32,
        revision: String,
        value: String,
    },
    AddRow {
        revision: String,
    },
    RemoveRow {
        row: u32,
        revision: String,
    },
    AddColumn {
        revision: String,
    },
    RemoveColumn {
        column: u32,
        revision: String,
    },
    SetHeader {
        column: u32,
        revision: String,
        value: String,
    },
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

impl protocol::OpText for CsvEditorCommand {
    fn print_op(&self) -> String {
        match self {
            CsvEditorCommand::SetCell { row, column, revision, value } => format!("set-cell row={row} column={column} revision={} value={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes()), crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(value.as_bytes())),
            CsvEditorCommand::AddRow { revision } => format!("add-row revision={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::RemoveRow { row, revision } => format!("remove-row row={row} revision={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::AddColumn { revision } => format!("add-column revision={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::RemoveColumn { column, revision } => format!("remove-column column={column} revision={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes())),
            CsvEditorCommand::SetHeader { column, revision, value } => format!("set-header column={column} revision={} value={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(revision.as_bytes()), crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(value.as_bytes())),
            CsvEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            CsvEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let bytes = crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_decode(rest)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid id hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let example_id = String::from_utf8(bytes)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid id utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(CsvEditorCommand::SetActiveExample { example_id });
        }
        if let Some(raw) = line.strip_prefix("snapshot-edit event=") {
            let bytes = crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_decode(raw)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid snapshot edit hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes)
                .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid snapshot edit: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            return Ok(CsvEditorCommand::EditSnapshot { event });
        }
        let (action, rest) = line
            .split_once(' ')
            .ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown line {line:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token
                .split_once('=')
                .ok_or_else(|| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: bad token {token:?}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            match key {
                "row" => row = raw.parse::<u32>().ok(),
                "column" => column = raw.parse::<u32>().ok(),
                "revision" => {
                    let bytes = crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_decode(raw).map_err(|error| {
                        semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid revision hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1))
                    })?;
                    revision = Some(String::from_utf8(bytes).map_err(|error| {
                        semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid revision utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1))
                    })?);
                }
                "value" => {
                    let bytes = crate::standards::v_rfc4180::subsets::any::io::text::diff::hex_decode(raw)
                        .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid value hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
                    value = Some(String::from_utf8(bytes).map_err(|error| {
                        semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: invalid value utf8: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1))
                    })?);
                }
                _ => return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown argument {key:?}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            }
        }
        let missing = |fields: &str| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: missing {fields}"), semio_framework_diagnostic::TextSpan::at(1, 1));
        match action {
            "set-cell" => {
                Ok(CsvEditorCommand::SetCell { row: row.ok_or_else(|| missing("row"))?, column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))?, value: value.ok_or_else(|| missing("value"))? })
            }
            "add-row" => Ok(CsvEditorCommand::AddRow { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-row" => Ok(CsvEditorCommand::RemoveRow { row: row.ok_or_else(|| missing("row"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            "add-column" => Ok(CsvEditorCommand::AddColumn { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-column" => Ok(CsvEditorCommand::RemoveColumn { column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            "set-header" => Ok(CsvEditorCommand::SetHeader { column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))?, value: value.ok_or_else(|| missing("value"))? }),
            _ => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("csv editor command: unknown action {action:?}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
}

impl protocol::OpBinary for CsvEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TableWindowKit`
    /// mints `set-cell`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = CSV_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "csv editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🔖️GridMapping
/// 🧮️ Pure row-offset math, kept standalone so it is directly unit-testable without constructing
/// a full `ArtifactView`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn grid_row_to_record_index(has_header: bool, row: u32) -> usize {
    if has_header {
        row as usize + 1
    } else {
        row as usize
    }
}
//#endregion 🔖️GridMapping

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TableWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const CSV_KIT_ACTION_ID: &str = "set-cell";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-cell`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-cell` with
/// `interactive-job.missing-factory`.
const CSV_RETAINED_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    CSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
];
const CSV_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    CSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const CSV_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.csv.tool-command.v1";
const CSV_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-cell` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const CSV_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: CSV_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(CSV_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (`crate::examples::demo`,
/// `ID = "demo"`), whose asset IS this app's curated document; every other id — including the empty
/// id the shell sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_example_snapshot(example_id: &str) -> CsvSnapshot {
    if example_id == crate::examples::demo::ID {
        <CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        CsvSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<CsvEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(CsvEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(CsvEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        CSV_KIT_ACTION_ID => {
            let edit = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_edit(args)?;
            Ok(CsvEditorCommand::SetCell { row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID => Ok(CsvEditorCommand::AddRow { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID => {
            Ok(CsvEditorCommand::RemoveRow { row: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID => Ok(CsvEditorCommand::AddColumn { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID => {
            Ok(CsvEditorCommand::RemoveColumn { column: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID => Ok(CsvEditorCommand::SetHeader {
            column: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?,
            revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")?,
            value: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "value")?,
        }),
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-cell)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_command_id(command: &CsvEditorCommand) -> &'static str {
    match command {
        CsvEditorCommand::SetCell { .. } => CSV_KIT_ACTION_ID,
        CsvEditorCommand::AddRow { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
        CsvEditorCommand::RemoveRow { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
        CsvEditorCommand::AddColumn { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
        CsvEditorCommand::RemoveColumn { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
        CsvEditorCommand::SetHeader { .. } => semio_s_artifact_stdio_contract::SET_TABLE_HEADER_ACTION_ID,
        CsvEditorCommand::EditSnapshot { event } => event.action_id(),
        CsvEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_extent(_command: &CsvEditorCommand, _snapshot: &CsvSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-cell` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_emit(command: &CsvEditorCommand, snapshot: &CsvSnapshot) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    csv_emit_at_revision(command, snapshot, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_emit_at_revision(command: &CsvEditorCommand, snapshot: &CsvSnapshot, canonical_revision: Option<&str>) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let CsvEditorCommand::EditSnapshot { event } = command {
        return <CsvEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    if let CsvEditorCommand::SetActiveExample { example_id } = command {
        return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&csv_example_snapshot(example_id), STDIO_CSV_DOCUMENT_SCHEMA)], ..Default::default() });
    }
    let revision = match command {
        CsvEditorCommand::SetCell { revision, .. }
        | CsvEditorCommand::AddRow { revision }
        | CsvEditorCommand::RemoveRow { revision, .. }
        | CsvEditorCommand::AddColumn { revision }
        | CsvEditorCommand::RemoveColumn { revision, .. }
        | CsvEditorCommand::SetHeader { revision, .. } => revision,
        CsvEditorCommand::EditSnapshot { .. } | CsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map(str::to_owned).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot));
    if current_revision != *revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.table-conflict"), "The CSV document changed before this table draft was applied."));
    }
    let mutation = match command {
        CsvEditorCommand::SetCell { row, column, value, .. } => {
            let record_index = grid_row_to_record_index(snapshot.has_header, *row);
            let field = snapshot
                .records
                .get(record_index)
                .and_then(|record| record.fields.get(*column as usize))
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.cell-stale"), format!("CSV cell {row},{column} no longer exists")))?;
            if field.value == *value {
                return Ok(Emit::default());
            }
            CsvMutation::SetField(crate::schema::mutations::set_field::SetField { record_index, field_index: *column as usize, value: value.clone(), quoted: field.quoted })
        }
        CsvEditorCommand::AddRow { .. } => {
            let width = snapshot.records.first().filter(|_| snapshot.has_header).map(|record| record.fields.len()).unwrap_or_else(|| snapshot.records.iter().map(|record| record.fields.len()).max().unwrap_or(0)).max(1);
            let row = CsvRecord { fields: vec![CsvField::default(); width] };
            if snapshot.has_header && snapshot.records.is_empty() {
                let mut next = snapshot.clone();
                next.records.push(CsvRecord { fields: vec![CsvField::default(); width] });
                next.records.push(row);
                CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
            } else {
                CsvMutation::InsertRecord(crate::schema::mutations::insert_record::InsertRecord { index: snapshot.records.len(), record: row })
            }
        }
        CsvEditorCommand::RemoveRow { row, .. } => {
            let index = grid_row_to_record_index(snapshot.has_header, *row);
            if snapshot.records.get(index).is_none() {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.row-stale"), format!("CSV row {row} no longer exists")));
            }
            CsvMutation::RemoveRecord(crate::schema::mutations::remove_record::RemoveRecord { index })
        }
        CsvEditorCommand::AddColumn { .. } => {
            let mut next = snapshot.clone();
            if next.records.is_empty() {
                next.records.push(CsvRecord::default());
            }
            for record in &mut next.records {
                record.fields.push(CsvField::default());
            }
            CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
        }
        CsvEditorCommand::RemoveColumn { column, .. } => {
            let column = *column as usize;
            if !snapshot.records.iter().any(|record| column < record.fields.len()) {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.column-stale"), format!("CSV column {column} no longer exists")));
            }
            let mut next = snapshot.clone();
            for record in &mut next.records {
                if column < record.fields.len() {
                    record.fields.remove(column);
                }
            }
            CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
        }
        CsvEditorCommand::SetHeader { column, value, .. } => {
            let column = *column as usize;
            if snapshot.has_header {
                let field = snapshot
                    .records
                    .first()
                    .and_then(|record| record.fields.get(column))
                    .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.csv.header-stale"), format!("CSV header {column} no longer exists")))?;
                if field.value == *value {
                    return Ok(Emit::default());
                }
                CsvMutation::SetField(crate::schema::mutations::set_field::SetField { record_index: 0, field_index: column, value: value.clone(), quoted: field.quoted })
            } else {
                let mut next = snapshot.clone();
                let width = next.records.iter().map(|record| record.fields.len()).max().unwrap_or(0).max(column + 1);
                let mut header = CsvRecord { fields: vec![CsvField::default(); width] };
                header.fields[column].value = value.clone();
                next.records.insert(0, header);
                next.has_header = true;
                CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot: next })
            }
        }
        CsvEditorCommand::EditSnapshot { .. } | CsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    Ok(Emit { artifact_mutations: vec![mutation], ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_retained_reduce(
    command: &CsvEditorCommand,
    snapshot: &CsvSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CsvEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
    csv_emit_at_revision(command, snapshot, Some(&revision))
}

struct CsvRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl CsvRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: CSV_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for CsvRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<CsvEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<CsvEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        CSV_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        csv_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_plugin::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_plugin::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_plugin::action_bus::RetainedToolWireInput, Option<semio_framework_plugin::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > CSV_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio csv retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for CsvRetainedCommandJobFactory {
    type Owner = EditorApp<CsvEditor>;
    const TOOL_IDS: &'static [&'static str] = CSV_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_CSV_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = CSV_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct CsvEditor;

impl ArtifactEditor for CsvEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = CsvSnapshot;
    type Mutation = CsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = CsvEditorCommand;

    const DIALECT: Dialect = CSV_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_CSV_DOCUMENT_SCHEMA;

    fn natural_file_codec() -> Option<semio_framework_plugin::NaturalFileCodec> {
        Some(semio_framework_plugin::NaturalFileCodec { format_kind: "s.stdio.csv@rfc4180", extension: ".csv", media_type: "text/csv", binary: false })
    }

    fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_framework_plugin::MediaError> {
        Ok(crate::standards::v_rfc4180::subsets::any::io::text::snapshot::encode_csv(snapshot).into_bytes())
    }

    fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_framework_plugin::MediaError> {
        let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))?;
        crate::standards::v_rfc4180::subsets::any::io::text::snapshot::decode_csv(text).map_err(|error| semio_framework_plugin::MediaError::Payload("artifact:native".into(), error))
    }

    fn whole_document_operation(snapshot: Self::Snapshot) -> Option<Self::Mutation> {
        Some(CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
    }

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<CsvEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.csv@rfc4180/*#editor",
        artifact_schema: "stdio.csv",
        factory: "CsvRetainedCommandJobFactory",
        factory_type: CsvRetainedCommandJobFactory,
        contract: csv_retained_contract(),
        tools: ["setActiveExample", "set-cell", "add-row", "remove-row", "add-column", "remove-column", "set-header"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(CsvRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !CSV_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if csv_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "stdio-csv-retained-command-tool-mismatch"));
        }
        let tool_id = csv_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            csv_command_id,
            CSV_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, csv_retained_reduce, csv_retained_extent)),
        )?;
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-csv-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    /// 🧹️ The rest of the close protocol installing a document owner implies: an app that owns its
    /// document store must own EVERY store it opens, or `close_step` refuses with
    /// `interactive-job.close-owned-disposer-missing`. Every one of these lanes is a `No…` unit type
    /// here, so each takes the framework's own empty-terminal owner.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🏗️ Admits the whole-document replacement the example switch emits. The trait default refuses
    /// the envelope, which answers every `setActiveExample` with
    /// `artifact-store.persisted-initializer-refused` at the archive-load boundary.
    #[allow(clippy::result_large_err, reason = "Mirrors the framework trait signature, which returns the original envelope on refusal.")]

    fn command_id(command: &Self::Command) -> &'static str {
        csv_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        csv_command_from_action(action, args)
    }

    fn initial_snapshot() -> CsvSnapshot {
        CsvSnapshot::default()
    }

    /// ✏️ Maps the rendered grid's `row` back to `CsvSnapshot.records`' real index — `+1` when
    /// `has_header` (row 0 in the grid is `records[1]`), unchanged otherwise. Out-of-range is a
    /// documented no-op (`Emit::default()`), never a panic.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &semio_framework_2d::compute::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        if let Some(event) = <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_event(command) {
            return <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot);
        }
        let revision = doc.operation_optional().map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
        csv_emit_at_revision(command, doc.snapshot, revision.as_deref())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision = doc
                    .render_operation()
                    .map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision))
                    .unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot));
                let publication_revision = semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
                main::render_revisioned(doc.snapshot, &revision, publication_revision, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc,
                view_state.locale,
                "s.stdio.csv@rfc4180/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for CsvEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            CsvEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_patch(
            event,
            snapshot,
            |patch| CsvMutation::PatchSnapshot(crate::schema::mutations::patch_snapshot::PatchSnapshot { patch }),
            Some(|snapshot| CsvMutation::SetSnapshot(crate::schema::mutations::set_snapshot::SetSnapshot { snapshot })),
        )
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_csv_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(CSV_EDITOR_DIALECT)
        .document(["semio", "stdio", "csv"])
        .icon_id("table-2")
        .mode_def(edit::definition())
        .default_mode_id(edit::CSV_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .window_kind_def(semio_s_artifact_stdio_contract::editing::snapshot_details_window_definition())
        .default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Table"))
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_describe(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_description())
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated);
    semio_s_artifact_stdio_contract::editing::snapshot_edit_actions_with(builder).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
