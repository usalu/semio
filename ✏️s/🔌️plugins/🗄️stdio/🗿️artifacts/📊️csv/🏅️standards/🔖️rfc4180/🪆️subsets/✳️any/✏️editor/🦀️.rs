//! ✏️ Csv editor — the FIRST authored `ArtifactEditor` surface for `s.stdio.csv@rfc4180/*`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET). One real window, `🪟️main`
//! (`TableWindowKit`), directly editing `CsvSnapshot.records` through the artifact's own
//! `CsvMutation::SetField`.

use crate::editor::csv::modes::edit;
use crate::editor::csv::modes::edit::windows::main;
use crate::{CsvMutation, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactStoreInitializationJob, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, InteractiveJobClassification, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation,
    StandardId, SubsetId, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
};

//#region 🔖️Dialect
/// 🪪️ Artifact coordinate — verified against `crate::schema::derived_analysis::
/// CsvAnalyzerAnalysis::DIALECT` (the artifact's own real analysis-capability row), not guessed.
/// Duplicated (not imported) in the sibling `👁️viewer` surface root — never shared through an
/// `editor`-rooted import, so a viewer file can never depend on this module.
pub const CSV_EDITOR_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Command
/// ✏️ The editor's typed command channel — exactly the one edit `🪟️main`'s `editable_window_kind()`
/// action (`set-cell`, contract §2.6) can trigger. `row`/`column` index the rendered grid (post
/// header-split, see the window's own `render` doc comment) — `handle` below does the row-offset
/// math back to `CsvMutation::SetField`'s `record_index`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum CsvEditorCommand {
    SetCell { row: u32, column: u32, value: String },
    /// 🎬️ The navbar example picker's payload — see the `🧵️RetainedRoutes` region below.
    SetActiveExample { example_id: String },
}

impl protocol::OpText for CsvEditorCommand {
    fn print_op(&self) -> String {
        match self {
            CsvEditorCommand::SetCell { row, column, value } => format!("set-cell row={row} column={column} value={}", value.replace('\\', "\\\\").replace(' ', "\\s")),
            CsvEditorCommand::SetActiveExample { example_id } => format!("active-example id={}", example_id.replace('\\', "\\\\").replace(' ', "\\s")),
        }
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        if let Some(rest) = line.strip_prefix("active-example id=") {
            return Ok(CsvEditorCommand::SetActiveExample { example_id: rest.replace("\\s", " ").replace("\\\\", "\\") });
        }
        let rest = line.strip_prefix("set-cell ").ok_or_else(|| store::TextError::new(format!("csv editor command: unknown line {line:?}"), dsl::TextSpan::at(1, 1)))?;
        let mut row = None;
        let mut column = None;
        let mut value = String::new();
        for token in rest.split(' ') {
            let (key, raw) = token.split_once('=').ok_or_else(|| store::TextError::new(format!("csv editor command: bad token {token:?}"), dsl::TextSpan::at(1, 1)))?;
            let decoded = raw.replace("\\s", " ").replace("\\\\", "\\");
            match key {
                "row" => row = decoded.parse::<u32>().ok(),
                "column" => column = decoded.parse::<u32>().ok(),
                "value" => value = decoded,
                _ => {}
            }
        }
        let (row, column) = row.zip(column).ok_or_else(|| store::TextError::new("csv editor command: missing row/column", dsl::TextSpan::at(1, 1)))?;
        Ok(CsvEditorCommand::SetCell { row, column, value })
    }
}

impl protocol::OpBinary for CsvEditorCommand {
    /// 🎯️ The app-owned retained routes this command channel carries — the join key
    /// `AppActionRegistry::validate_tool_job_rows` demands an exact owner-local proof for. The `TableWindowKit`
    /// mints `set-cell`, but only this editor can reduce it into its own mutation, so it is an
    /// app-owned route exactly like the example switch.
    const TOOL_JOB_IDS: &'static [&'static str] = CSV_RETAINED_TOOL_IDS;

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
const CSV_RETAINED_TOOL_IDS: &[&str] = &[semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, CSV_KIT_ACTION_ID];
const CSV_RETAINED_PAYLOAD_SCHEMA: &str = "stdio.csv.tool-command.v1";
const CSV_RETAINED_RAW_BYTES: usize = 8_192;
/// 🚦️ The example switch publishes into NO document lane: it hands the host one
/// `Effect::LoadDocument`, so its only lane is `HostOnly`. `set-cell` publishes the artifact
/// mutation it reduces into, so its only lane is `Artifact`.
const CSV_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: CSV_KIT_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
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
fn csv_command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<CsvEditorCommand, Fault> {
    match action {
        semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID => Ok(CsvEditorCommand::SetActiveExample { example_id: semio_s_artifact_stdio_contract::example_id_argument(args, "") }),
        CSV_KIT_ACTION_ID => Ok(CsvEditorCommand::SetCell { row: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["row"], 0), column: semio_s_artifact_stdio_contract::window_kit_index_argument(args, &["column"], 0), value: semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["value"], "") }),
        other => Err(Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.csv.unhandled-action"),
            format!("action '{other}' is not one of this editor's declared verbs (setActiveExample, set-cell)"),
        )),
    }
}

/// 🏷️ The manifest id each command was declared under.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn csv_command_id(command: &CsvEditorCommand) -> &'static str {
    match command {
        CsvEditorCommand::SetCell { .. } => CSV_KIT_ACTION_ID,
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
    let (row, column, value) = match command {
        CsvEditorCommand::SetActiveExample { example_id } => {
            return Ok(Emit {
                effects: vec![semio_s_artifact_stdio_contract::load_example_effect(&csv_example_snapshot(example_id), STDIO_CSV_DOCUMENT_SCHEMA)],
                description: Some(format!("Load example {example_id}")),
                ..Default::default()
            })
        }
        CsvEditorCommand::SetCell { row, column, value } => (row, column, value),
    };
    let record_index = grid_row_to_record_index(snapshot.has_header, *row);
    let Some(record) = snapshot.records.get(record_index) else { return Ok(Emit::default()) };
    let quoted = record.fields.get(*column as usize).is_some_and(|field| field.quoted);
    Ok(Emit {
        artifact_mutations: vec![CsvMutation::SetField(crate::schema::mutations::set_field::SetField { record_index, field_index: *column as usize, value: value.clone(), quoted })],
        description: Some(format!("Set cell {row},{column}")),
        ..Default::default()
    })
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
    _operation: &AppOperationContext,
) -> Result<Emit<CsvMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    csv_emit(command, snapshot)
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

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<CsvEditor>,
        owner_file: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.stdio.csv@rfc4180/*#editor",
        artifact_schema: "stdio.csv",
        factory: "CsvRetainedCommandJobFactory",
        factory_type: CsvRetainedCommandJobFactory,
        contract: csv_retained_contract(),
        tools: ["setActiveExample", "set-cell"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(CsvRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<ToolOperationSpec>, Fault> {
        if !CSV_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if csv_command_id(&request.command) != request.tool_id {
            return Err(Fault::from("stdio-csv-retained-command-tool-mismatch"));
        }
        let tool_id = csv_command_id(&request.command);
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
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
    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_framework_plugin::bounded_document_store_initialization_job(envelope, STDIO_CSV_DOCUMENT_SCHEMA, operation, generation))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        csv_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
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
        _engines: &store::EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        csv_emit(command, doc.snapshot)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn create_csv_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(CSV_EDITOR_DIALECT)
        .document(["semio", "stdio", "csv"])
        .icon_id("table-2")
        .mode_def(edit::definition())
        .default_mode_id(edit::CSV_EDIT_MODE_ID)
        .window_kind_def(main::definition())
        .default_layout(edit::layout())
        // 🎬️ Example picker — one option per example `register_apps` publishes for this dialect.
        .action_with(semio_s_artifact_stdio_contract::set_active_example_action())
        .action_args(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, semio_s_artifact_stdio_contract::set_active_example_args(&[(crate::examples::demo::ID, crate::examples::demo::label())], crate::examples::demo::ID))
        .action_destructive(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID)
        .action_interactive_job(semio_s_artifact_stdio_contract::SET_ACTIVE_EXAMPLE_ACTION_ID, InteractiveJobClassification::Migrated)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
