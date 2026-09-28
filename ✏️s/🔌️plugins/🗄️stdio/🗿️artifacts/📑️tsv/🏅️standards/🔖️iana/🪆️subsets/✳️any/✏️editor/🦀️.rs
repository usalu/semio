//! ✏️ Tsv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.tsv@iana/*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `TsvSnapshot.records` through the artifact's own
//! `TsvMutation::SetCell`.

use crate::editor::tsv::modes::edit;
use crate::editor::tsv::modes::edit::windows::main;
use crate::standards::iana::subsets::any::schema::mutations::{insert_row, remove_row, set_cell, set_snapshot};
use crate::{TsvMutation, TsvSnapshot, STDIO_TSV_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView,
    Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, StandardId, SubsetId,
    ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against the artifact's own `🚪️io`/`🧬️schema` `DIALECT`
/// consts. Duplicated (not imported) in the sibling `👁️viewer` surface root.
pub const TSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tsv", standard: StandardId("iana"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index `TsvSnapshot.records`
/// directly and `revision` prevents a concurrent row shift from redirecting the edit.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum TsvEditorCommand {
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
    EditSnapshot {
        event: SnapshotEditEvent,
    },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample {
        example_id: String,
    },
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
    let mut pairs = text.as_bytes().chunks_exact(2);
    let decoded = pairs.by_ref().map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect::<Result<Vec<_>, String>>()?;
    if pairs.remainder().is_empty() {
        Ok(decoded)
    } else {
        Err("odd-length hex string".into())
    }
}

impl protocol::OpText for TsvEditorCommand {
    fn print_op(&self) -> String {
        match self {
            TsvEditorCommand::SetCell { row, column, revision, value } => {
                format!("set-cell row={row} column={column} revision={} value={}", hex_encode(revision.as_bytes()), hex_encode(value.as_bytes()))
            }
            TsvEditorCommand::AddRow { revision } => format!("add-row revision={}", hex_encode(revision.as_bytes())),
            TsvEditorCommand::RemoveRow { row, revision } => format!("remove-row row={row} revision={}", hex_encode(revision.as_bytes())),
            TsvEditorCommand::AddColumn { revision } => format!("add-column revision={}", hex_encode(revision.as_bytes())),
            TsvEditorCommand::RemoveColumn { column, revision } => format!("remove-column column={column} revision={}", hex_encode(revision.as_bytes())),
            TsvEditorCommand::EditSnapshot { event } => format!("snapshot-edit event={}", hex_encode(&<SnapshotEditEvent as protocol::OpBinary>::encode_op(event).expect("snapshot edit event encodes"))),
            TsvEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", hex_encode(example_id.as_bytes())),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            let value = String::from_utf8(hex_decode(rest).map_err(|error| store::TextError::new(format!("tsv editor command: invalid example hex {error}"), dsl::TextSpan::at(1, 1)))?)
                .map_err(|error| store::TextError::new(format!("tsv editor command: invalid example utf8 {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(TsvEditorCommand::SetActiveExample { example_id: value });
        }
        if let Some(rest) = line.strip_prefix("snapshot-edit event=") {
            let bytes = hex_decode(rest).map_err(|error| store::TextError::new(format!("tsv editor command: invalid snapshot edit hex {error}"), dsl::TextSpan::at(1, 1)))?;
            let event = <SnapshotEditEvent as protocol::OpBinary>::decode_op(&bytes).map_err(|error| store::TextError::new(format!("tsv editor command: invalid snapshot edit {error}"), dsl::TextSpan::at(1, 1)))?;
            return Ok(TsvEditorCommand::EditSnapshot { event });
        }
        let (action, rest) = line.split_once(' ').ok_or_else(|| store::TextError::new(format!("tsv editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut revision = None;
        let mut value = None;
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("tsv editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            match key {
                "row" => row = raw.parse::<u32>().ok(),
                "column" => column = raw.parse::<u32>().ok(),
                "revision" => {
                    revision = Some(
                        String::from_utf8(hex_decode(raw).map_err(|error| store::TextError::new(format!("tsv editor command: invalid revision hex {error}"), dsl::TextSpan::at(1, 1)))?)
                            .map_err(|error| store::TextError::new(format!("tsv editor command: invalid revision utf8 {error}"), dsl::TextSpan::at(1, 1)))?,
                    )
                }
                "value" => {
                    value = Some(
                        String::from_utf8(hex_decode(raw).map_err(|error| store::TextError::new(format!("tsv editor command: invalid value hex {error}"), dsl::TextSpan::at(1, 1)))?)
                            .map_err(|error| store::TextError::new(format!("tsv editor command: invalid value utf8 {error}"), dsl::TextSpan::at(1, 1)))?,
                    )
                }
                _ => return Err(store::TextError::new(format!("tsv editor command: unknown argument {key:?}"), dsl::TextSpan::at(1, 1))),
            }
        }
        let missing = |fields: &str| store::TextError::new(format!("tsv editor command: missing {fields}"), dsl::TextSpan::at(1, 1));
        match action {
            "set-cell" => {
                Ok(TsvEditorCommand::SetCell { row: row.ok_or_else(|| missing("row"))?, column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))?, value: value.ok_or_else(|| missing("value"))? })
            }
            "add-row" => Ok(TsvEditorCommand::AddRow { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-row" => Ok(TsvEditorCommand::RemoveRow { row: row.ok_or_else(|| missing("row"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            "add-column" => Ok(TsvEditorCommand::AddColumn { revision: revision.ok_or_else(|| missing("revision"))? }),
            "remove-column" => Ok(TsvEditorCommand::RemoveColumn { column: column.ok_or_else(|| missing("column"))?, revision: revision.ok_or_else(|| missing("revision"))? }),
            _ => Err(store::TextError::new(format!("tsv editor command: unknown action {action:?}"), dsl::TextSpan::at(1, 1))),
        }
    }
}

impl protocol::OpBinary for TsvEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TableWindowKit`
    /// mints `set-cell`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = TSV_COMMAND_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(<Self as protocol::OpText>::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = String::from_utf8(bytes.to_vec()).map_err(|error| protocol::ProtocolError::Malformed { what: "tsv editor command utf8", offset: 0, detail: error.to_string() })?;
        <Self as protocol::OpText>::parse_op(&line).map_err(|error| protocol::ProtocolError::Malformed { what: "tsv editor command", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️Command

//#region 🧵️RetainedRoutes
/// 🪟️ The verb the `TableWindowKit` mints for `🪟️main` — declared by the framework, reduced only here.
const TSV_KIT_ACTION_ID: &str = "set-cell";
/// 🧵️ The app-owned retained routes this editor declares: the example switch and `set-cell`.
/// `validate_ui_dispatch_classification` refuses any verb that is not `Migrated`, and `Migrated`
/// only survives the guest's `interactive-job.catalog-incomplete` boot check when this roster, the
/// publication contracts and the `bounded_first_step_tool_proofs!` block below all name the same
/// ids. Without the kit verb's row the reactor refused every `set-cell` with
/// `interactive-job.missing-factory`.
const TSV_RETAINED_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    TSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
];
const TSV_COMMAND_TOOL_IDS: &[&str] = &[
    semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    TSV_KIT_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
    semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::SET_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::INSERT_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REMOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::MOVE_SNAPSHOT_VALUE_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::RENAME_SNAPSHOT_KEY_ACTION_ID,
    semio_s_artifact_stdio_contract::editing::REPLACE_SNAPSHOT_SOURCE_ACTION_ID,
];
const TSV_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.tsv.tool-command.v1";
const TSV_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-cell` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const TSV_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: TSV_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(TSV_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500)
}

/// 📚️ The document a named example loads. The subset publishes exactly one (crate::examples::demo, `ID = "demo"`),
/// whose asset IS this app's curated document; every other id — including the empty id the shell
/// sends for "the app's own default document" — opens the genesis document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_example_snapshot(example_id: &str) -> TsvSnapshot {
    if example_id == crate::examples::demo::ID {
        <TsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap_or_default()
    } else {
        TsvSnapshot::default()
    }
}

/// 🌉️ Resolves the react/wgpu shells' `{action, args}` pair into this editor's typed command.
/// `ArtifactEditor::command_from_action`'s default refuses EVERY id, which is why the boot example,
/// every navbar pick and every Actions-pane row died before reaching a command.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<TsvEditorCommand, Fault> {
    if let Some(event) = semio_s_artifact_stdio_contract::editing::snapshot_edit_event_from_action(action, args)? {
        return Ok(TsvEditorCommand::EditSnapshot { event });
    }
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(TsvEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        TSV_KIT_ACTION_ID => {
            let edit = semio_s_artifact_stdio_contract::window_kit_revisioned_cell_edit(args)?;
            Ok(TsvEditorCommand::SetCell { row: edit.row, column: edit.column, revision: edit.revision, value: edit.value })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID => Ok(TsvEditorCommand::AddRow { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID => {
            Ok(TsvEditorCommand::RemoveRow { row: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "row")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID => Ok(TsvEditorCommand::AddColumn { revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? }),
        semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID => {
            Ok(TsvEditorCommand::RemoveColumn { column: semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, "column")?, revision: semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, "revision")? })
        }
        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.tsv.unhandled-action"), format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-cell)"))),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_command_id(command: &TsvEditorCommand) -> &'static str {
    match command {
        TsvEditorCommand::SetCell { .. } => TSV_KIT_ACTION_ID,
        TsvEditorCommand::AddRow { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_ROW_ACTION_ID,
        TsvEditorCommand::RemoveRow { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_ROW_ACTION_ID,
        TsvEditorCommand::AddColumn { .. } => semio_s_artifact_stdio_contract::ADD_TABLE_COLUMN_ACTION_ID,
        TsvEditorCommand::RemoveColumn { .. } => semio_s_artifact_stdio_contract::REMOVE_TABLE_COLUMN_ACTION_ID,
        TsvEditorCommand::EditSnapshot { event } => event.action_id(),
        TsvEditorCommand::SetActiveExample { .. } => semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_retained_extent(_command: &TsvEditorCommand, _snapshot: &TsvSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// ✏️ The one reduction `handle` and the retained route share: the example switch hands the host
/// its document, `set-cell` becomes this artifact's own mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_emit(command: &TsvEditorCommand, snapshot: &TsvSnapshot) -> Result<Emit<TsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    tsv_emit_at_revision(command, snapshot, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_emit_at_revision(command: &TsvEditorCommand, snapshot: &TsvSnapshot, canonical_revision: Option<&str>) -> Result<Emit<TsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let TsvEditorCommand::EditSnapshot { event } = command {
        return <TsvEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, snapshot);
    }
    if let TsvEditorCommand::SetActiveExample { example_id } = command {
        return Ok(Emit { effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&tsv_example_snapshot(example_id), STDIO_TSV_DOCUMENT_SCHEMA)], description: Some(format!("Load example {example_id}")), ..Default::default() });
    }
    let revision = match command {
        TsvEditorCommand::SetCell { revision, .. } | TsvEditorCommand::AddRow { revision } | TsvEditorCommand::RemoveRow { revision, .. } | TsvEditorCommand::AddColumn { revision } | TsvEditorCommand::RemoveColumn { revision, .. } => revision,
        TsvEditorCommand::EditSnapshot { .. } | TsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    let current_revision = canonical_revision.map(str::to_owned).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(snapshot));
    if current_revision != *revision {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.tsv.table-conflict"), "The TSV document changed before this table draft was applied."));
    }
    let mutation = match command {
        TsvEditorCommand::SetCell { row, column, value, .. } => {
            let row_index = *row as usize;
            let current = snapshot
                .records
                .get(row_index)
                .and_then(|record| record.get(*column as usize))
                .ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.tsv.cell-stale"), format!("TSV cell {row},{column} no longer exists")))?;
            if current == value {
                return Ok(Emit::default());
            }
            TsvMutation::SetCell(set_cell::SetCell { row_index, field_index: *column as usize, value: value.clone() })
        }
        TsvEditorCommand::AddRow { .. } => {
            let width = snapshot.records.iter().map(Vec::len).max().unwrap_or(0).max(1);
            let row = vec![String::new(); width];
            TsvMutation::InsertRow(insert_row::InsertRow { index: snapshot.records.len(), row })
        }
        TsvEditorCommand::RemoveRow { row, .. } => {
            let index = *row as usize;
            if snapshot.records.get(index).is_none() {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.tsv.row-stale"), format!("TSV row {row} no longer exists")));
            }
            TsvMutation::RemoveRow(remove_row::RemoveRow { index })
        }
        TsvEditorCommand::AddColumn { .. } => {
            let mut next = snapshot.clone();
            if next.records.is_empty() {
                next.records.push(Vec::new());
            }
            for record in &mut next.records {
                record.push(String::new());
            }
            TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next })
        }
        TsvEditorCommand::RemoveColumn { column, .. } => {
            let column = *column as usize;
            if !snapshot.records.iter().any(|record| column < record.len()) {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.tsv.column-stale"), format!("TSV column {column} no longer exists")));
            }
            let mut next = snapshot.clone();
            for record in &mut next.records {
                if column < record.len() {
                    record.remove(column);
                }
            }
            TsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: next })
        }
        TsvEditorCommand::EditSnapshot { .. } | TsvEditorCommand::SetActiveExample { .. } => unreachable!(),
    };
    Ok(Emit { artifact_mutations: vec![mutation], description: Some(tsv_command_id(command).to_string()), ..Default::default() })
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tsv_retained_reduce(
    command: &TsvEditorCommand,
    snapshot: &TsvSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TsvEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<TsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision);
    tsv_emit_at_revision(command, snapshot, Some(&revision))
}

struct TsvRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl TsvRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: TSV_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for TsvRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TsvEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TsvEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        TSV_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        tsv_retained_contract()
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
        if input.declared_bytes() > TSV_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("stdio tsv retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for TsvRetainedCommandJobFactory {
    type Owner = EditorApp<TsvEditor>;
    const TOOL_IDS: &'static [&'static str] = TSV_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TSV_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = TSV_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedRoutes

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct TsvEditor;

impl ArtifactEditor for TsvEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = TsvSnapshot;
    type Mutation = TsvMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TsvEditorCommand;

    const DIALECT: Dialect = TSV_EDITOR_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = STDIO_TSV_DOCUMENT_SCHEMA;

    semio_s_artifact_stdio_contract::snapshot_editing_bounded_first_step_tool_proofs! {
        owner: EditorApp<TsvEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.tsv@iana/*#editor",
        artifact_schema: "stdio.tsv",
        factory: "TsvRetainedCommandJobFactory",
        factory_type: TsvRetainedCommandJobFactory,
        contract: tsv_retained_contract(),
        tools: ["setActiveExample", "set-cell", "add-row", "remove-row", "add-column", "remove-column"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(TsvRetainedCommandJobFactory::new(&controller))?;
        semio_s_artifact_stdio_contract::editing::register_snapshot_edit_tool_factory::<Self>(registry)
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if semio_s_artifact_stdio_contract::editing::is_snapshot_edit_action(&request.tool_id) {
            return semio_s_artifact_stdio_contract::editing::build_snapshot_edit_tool_job::<Self>(request);
        }
        if !TSV_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if tsv_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-tsv-retained-command-tool-mismatch"));
        }
        let tool_id = tsv_command_id(&request.command);
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
            tsv_command_id,
            TSV_RETAINED_RAW_BYTES,
            1,
            Box::new(BoundedArtifactCommandWork::new(tool_id, tsv_retained_reduce, tsv_retained_extent)),
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
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-tsv-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
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
        tsv_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        tsv_command_from_action(action, args)
    }

    fn initial_snapshot() -> TsvSnapshot {
        TsvSnapshot::default()
    }

    /// ✏️ Stale revisions, rows, and columns fault without publishing a mutation.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        match command {
            TsvEditorCommand::EditSnapshot { event } => <Self as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(event, doc.snapshot),
            _ => {
                let revision = doc.operation_optional().map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision));
                tsv_emit_at_revision(command, doc.snapshot, revision.as_deref())
            }
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => {
                let revision = doc
                    .render_operation()
                    .map(|operation| semio_s_artifact_stdio_contract::window_kit_canonical_revision(operation.canonical_base_revision))
                    .unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_snapshot_revision(doc.snapshot));
                main::render_revisioned(doc.snapshot, &revision, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, main::BODY_KEY)).map(semio_framework_plugin::built_to_component_tree)
            }
            semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY => semio_s_artifact_stdio_contract::editing::render_snapshot_details(
                doc.snapshot,
                view_state.locale,
                "s.stdio.tsv@iana/*#editor",
                &semio_framework_plugin::TreeWindows::for_body(view_state, semio_s_artifact_stdio_contract::editing::SNAPSHOT_DETAILS_BODY_KEY),
            )
            .map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

impl semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor for TsvEditor {
    fn snapshot_edit_event(command: &Self::Command) -> Option<&SnapshotEditEvent> {
        match command {
            TsvEditorCommand::EditSnapshot { event } => Some(event),
            _ => None,
        }
    }

    fn snapshot_edit_mutations(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_artifact_stdio_contract::editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| TsvMutation::SetSnapshot(crate::standards::iana::subsets::any::schema::mutations::set_snapshot::SetSnapshot { snapshot }))
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_tsv_editor() -> semio_framework_plugin::AppDefinition {
    let builder = Editor::builder(TSV_EDITOR_DIALECT)
        .document(["semio", "stdio", "tsv"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("table-2")
        .mode_def(edit::definition())
        .default_mode_id(edit::TSV_EDIT_MODE_ID)
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
