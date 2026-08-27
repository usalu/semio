//! 🛒️ Sourcing curate app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `🧬️schema`. This file is a routing table: `handle` → `SourcingCurateCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::artifacts::curate::op::SourcingMutation;
use crate::artifacts::curate::{CurateSnapshot, SOURCING_CURATE_SCHEMA};
use crate::editor::sourcing::config::{SourcingCurateConfig, SourcingCurateConfigMutation};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::presence::{SourcingCuratePresence, SourcingCuratePresenceMutation};
use crate::editor::sourcing::terminology::sourcing_curate_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest,
    ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider,
    HoverSpec, InteractionDefinition, InteractionRef, Label,
    LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec,
};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `SOURCING_CURATE_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment`).
pub fn sourcing_curate_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: SOURCING_CURATE_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Constants
/// 🎯️ Still used as a plain namespace tag for `ActionFactory`/`TableScene`/`WorldScene` addressing
/// below — NOT a trait const any more (contract §7.4: the real surface id is now derived from
/// `SOURCING_DIALECT` + `AppRole` via `surface_app_id`, never hand-written).
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curate";
pub const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
pub const DEMO_STOCK_EXAMPLE_ID: &str = "demo-stock";
pub const EMPTY_EXAMPLE_ID: &str = "empty-curation";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// builds its `on_change`/drop actions with.
pub fn sourcing_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SOURCING_CONTROLLER_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SourcingCurateApp::Command` — the SOLE dispatch surface for curate's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) —
    /// they are genuinely different vocabularies, and `setLocale`/`locale` is the row that proves it.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SourcingCurateCommand for CurateSnapshot, SourcingMutation, SourcingCurateConfig, SourcingCurateConfigMutation {
        "setDocument" as "document-json" => set_artifact_json::SetArtifactJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "stockFromCatalogue" as "stock-from-catalogue" => stock_from_catalogue::StockFromCatalogue,
        "curateAdd" as "curate-add" => curate_add::CurateAdd,
        "curateSetCount" as "curate-set-count" => curate_set_count::CurateSetCount,
        "curateRemove" as "curate-remove" => curate_remove::CurateRemove,
        "dropOnPool" as "drop-on-pool" => drop_on_pool::DropOnPool,
        "dropOnCurated" as "drop-on-curated" => drop_on_curated::DropOnCurated,
        "setFilterQuery" as "filter-query" => set_filter_query::SetFilterQuery,
        "setFilterModule" as "filter-module" => set_filter_module::SetFilterModule,
        "setFilterTypology" as "filter-typology" => set_filter_typology::SetFilterTypology,
        "setFilterMinAvailability" as "filter-min-availability" => set_filter_min_availability::SetFilterMinAvailability,
        "sortTable" as "sort-table" => sort_table::SortTable,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::sourcing::commands::set_contributions;
use crate::editor::sourcing::commands::set_locale;
use crate::editor::sourcing::commands::{curate_add, curate_remove, curate_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::commands::{set_active_example, set_artifact_json, stock_from_catalogue};
use crate::editor::sourcing::commands::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table};

/// 🎯️ Host action id + JSON args → the closed `SourcingCurateCommand` vocabulary — the production
/// bridge between the manifest's *declared* action surface (`🔖️Manifest`, camelCase arg names) and
/// the typed command channel that actually dispatches.
///
/// Chrome built by the `🎭️modes` nodes addresses this app by action id (`sourcing_action`), and
/// several of those rows carry `None` args because the host fills them at dispatch time from the
/// interaction itself (a text input's `value`, a slider's `delta`, a drop's `objectId`) — so every
/// field is read defensively and coerced, never assumed present.
///
/// Without this, `ArtifactApp::command_from_action`'s default rejects every app-owned action and the
/// pane cannot even load its own example. See `📐️cad`'s `cad_command_from_action` twin.
fn sourcing_curate_command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<SourcingCurateCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(serde_json::Value::as_bool);
    let text_of = |key: &str| -> Option<String> {
        args.and_then(|value| value.get(key)).and_then(|value| match value {
            serde_json::Value::String(text) => Some(text.clone()),
            serde_json::Value::Bool(flag) => Some(flag.to_string()),
            serde_json::Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
    };
    let json_field = |key: &str| -> String {
        match args.and_then(|value| value.get(key)) {
            Some(serde_json::Value::String(text)) => text.clone(),
            Some(other) => other.to_string(),
            None => args.map(serde_json::Value::to_string).unwrap_or_default(),
        }
    };
    let object_id = || str_field("objectId").unwrap_or_default();
    Ok(match action {
        "setDocument" => SourcingCurateCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: json_field("json") }),
        "setActiveExample" => SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "stockFromCatalogue" => SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        "curateAdd" => SourcingCurateCommand::CurateAdd(curate_add::CurateAdd { object_id: object_id() }),
        "curateSetCount" => SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: object_id(), delta: f64_field("delta"), value: f64_field("value") }),
        "curateRemove" => SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id: object_id() }),
        "dropOnPool" => SourcingCurateCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id() }),
        "dropOnCurated" => SourcingCurateCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id() }),
        "setFilterQuery" => SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: text_of("value").unwrap_or_default() }),
        "setFilterModule" => SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: str_field("moduleId").unwrap_or_default(), enabled: bool_field("enabled").unwrap_or(false) }),
        "setFilterTypology" => SourcingCurateCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: str_field("path").or_else(|| text_of("value")).unwrap_or_default() }),
        "setFilterMinAvailability" => SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: f64_field("delta"), value: f64_field("value") }),
        "sortTable" => SourcingCurateCommand::SortTable(sort_table::SortTable { column_id: str_field("columnId").unwrap_or_default(), direction: str_field("direction").unwrap_or_default() }),
        "setLocale" => SourcingCurateCommand::SetLocale(set_locale::SetLocale { value: text_of("value").unwrap_or_default() }),
        "setContributions" => SourcingCurateCommand::SetContributions(set_contributions::SetContributions { json: json_field("json") }),
        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{other}' is not a sourcing curate command"))),
    })
}
//#endregion 🔖️Commands

//#region 🔖️SourcingCurateApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::editor::sourcing::config::
/// SourcingCurateConfig`, written through `SourcingCurateConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurateApp;

//#region 🧵️RetainedCommands
const SOURCING_CURATE_BOUNDED_TOOL_IDS: &[&str] = &["setFilterQuery", "setFilterTypology", "setFilterMinAvailability", "sortTable", "setContributions"];
const SOURCING_CURATE_BATCH_ONLY_TOOL_IDS: &[&str] = &[
    "setDocument",
    "setActiveExample",
    "stockFromCatalogue",
    "curateAdd",
    "curateSetCount",
    "curateRemove",
    "dropOnPool",
    "dropOnCurated",
    "setFilterModule",
];
const SOURCING_CURATE_RETAINED_SCHEMA: &str = "sourcing.curate/v1.tool-command.v1";
const SOURCING_CURATE_RETAINED_RAW_BYTES: usize = 8_192;
const SOURCING_CURATE_RETAINED_WORK_ITEMS: usize = 1;

fn sourcing_curate_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SOURCING_CURATE_RETAINED_RAW_BYTES, 64, 1, 16_384, 7_500)
}

fn sourcing_curate_bounded_extent(command: &SourcingCurateCommand, _snapshot: &CurateSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SOURCING_CURATE_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

fn sourcing_curate_retained_reduce(
    command: &SourcingCurateCommand,
    snapshot: &CurateSnapshot,
    config: &SourcingCurateConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, NoDraftMutation>, Fault> {
    if !SOURCING_CURATE_BOUNDED_TOOL_IDS.contains(&command.command_id()) { return Err(Fault::from("sourcing-curate-retained-route-mismatch")); }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    command.dispatch(&doc, &cfg)
}

struct SourcingCurateBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SourcingCurateBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SOURCING_CURATE_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SourcingCurateBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SourcingCurateApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SourcingCurateApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SOURCING_CURATE_RETAINED_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        sourcing_curate_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SOURCING_CURATE_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Sourcing bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SourcingCurateBoundedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SourcingCurateApp>;
    const TOOL_IDS: &'static [&'static str] = SOURCING_CURATE_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setFilterQuery", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterTypology", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterMinAvailability", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "sortTable", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}

//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
const SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES: usize = 768;
const SOURCING_CURATE_CONFIG_STORE_MAXIMUM_ITEMS: usize = 256;
const SOURCING_CURATE_CONFIG_TEXT_BYTES: usize = 96;
const SOURCING_CURATE_CONFIG_METADATA_BYTES: usize = 64;

struct SourcingCurateConfigPreparationFactory;

struct SourcingCurateConfigPreparation {
    base: Option<store::SnapshotRead<SourcingCurateConfig>>,
    mutation: Option<SourcingCurateConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(SourcingCurateConfig, Vec<SourcingCurateConfigMutation>, SourcingCurateConfigMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<SourcingCurateConfig, SourcingCurateConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

fn sourcing_curate_config_bytes(config: &SourcingCurateConfig) -> Result<usize, String> {
    let items = config.filters.module_ids.len().saturating_add(config.filters.typology_path.len());
    if items > SOURCING_CURATE_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config base exceeds its retained item envelope".into()); }
    let bytes = config.filters.query.len()
        .saturating_add(config.filters.module_ids.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.typology_path.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.sort.as_ref().map_or(0, |sort| sort.column_id.len()))
        .saturating_add(config.locale.len())
        .saturating_add(config.contributions_json.len());
    if bytes > SOURCING_CURATE_CONFIG_TEXT_BYTES { return Err("Sourcing Config base exceeds its encoded text envelope".into()); }
    let bytes = bytes.saturating_add(std::mem::size_of::<SourcingCurateConfig>())
        .saturating_add(items.saturating_mul(std::mem::size_of::<String>()));
    if bytes > SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES { return Err("Sourcing Config base exceeds its retained byte envelope".into()); }
    Ok(bytes)
}

fn sourcing_curate_config_mutation_footprint(mutation: &SourcingCurateConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let (work_items, retained_bytes) = match mutation {
        SourcingCurateConfigMutation::Snapshot { .. } | SourcingCurateConfigMutation::SetLocale { .. } | SourcingCurateConfigMutation::SetFilterModules { .. } => return Err("Sourcing Config preparation rejects a non-retained mutation".into()),
        SourcingCurateConfigMutation::SetFilterQuery { value } | SourcingCurateConfigMutation::SetContributions { json: value } => (1, value.len()),
        SourcingCurateConfigMutation::SetFilterTypology { path } => {
            if path.len() > SOURCING_CURATE_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config typology exceeds its retained item envelope".into()); }
            (path.len().max(1), path.iter().map(String::len).sum())
        }
        SourcingCurateConfigMutation::SetSort { sort } => (1, sort.as_ref().map_or(0, |sort| sort.column_id.len())),
        SourcingCurateConfigMutation::SetFilterMinAvailability { .. } => (1, 0),
    };
    if retained_bytes > SOURCING_CURATE_CONFIG_TEXT_BYTES { return Err("Sourcing Config mutation exceeds its encoded text envelope".into()); }
    let retained_bytes = retained_bytes.saturating_add(std::mem::size_of::<SourcingCurateConfigMutation>()).saturating_add(work_items.saturating_mul(std::mem::size_of::<String>()));
    if work_items > SOURCING_CURATE_CONFIG_STORE_MAXIMUM_ITEMS || retained_bytes > SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sourcing Config mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}

fn prepare_sourcing_curate_config(base: &SourcingCurateConfig, mutation: SourcingCurateConfigMutation) -> Result<(SourcingCurateConfig, Vec<SourcingCurateConfigMutation>, SourcingCurateConfigMutation), String> {
    sourcing_curate_config_mutation_footprint(&mutation)?;
    sourcing_curate_config_bytes(base)?;
    let mut post = base.clone();
    let inverse = match &mutation {
        SourcingCurateConfigMutation::SetFilterQuery { value } => { post.filters.query = value.clone(); SourcingCurateConfigMutation::SetFilterQuery { value: base.filters.query.clone() } }
        SourcingCurateConfigMutation::SetFilterTypology { path } => { post.filters.typology_path = path.clone(); SourcingCurateConfigMutation::SetFilterTypology { path: base.filters.typology_path.clone() } }
        SourcingCurateConfigMutation::SetFilterMinAvailability { value } => { post.filters.min_availability = *value; SourcingCurateConfigMutation::SetFilterMinAvailability { value: base.filters.min_availability } }
        SourcingCurateConfigMutation::SetSort { sort } => { post.filters.sort = sort.clone(); SourcingCurateConfigMutation::SetSort { sort: base.filters.sort.clone() } }
        SourcingCurateConfigMutation::SetContributions { json } => { post.contributions_json = json.clone(); SourcingCurateConfigMutation::SetContributions { json: base.contributions_json.clone() } }
        _ => return Err("Sourcing Config preparation rejects a non-retained mutation".into()),
    };
    sourcing_curate_config_bytes(&post)?;
    Ok((post, vec![inverse], mutation))
}

fn sourcing_curate_store_edit(
    forward: SourcingCurateConfigMutation,
    inverse: Vec<SourcingCurateConfigMutation>,
    description: Option<String>,
    authority: &store::ArtifactStoreOneItemLiveAuthority,
) -> protocol::Edit<SourcingCurateConfigMutation> {
    let id = format!("sourcing-curate-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<SourcingCurateConfig, SourcingCurateConfigMutation> for SourcingCurateConfigPreparationFactory {
    fn preflight(&self, mutation: &SourcingCurateConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > SOURCING_CURATE_CONFIG_METADATA_BYTES) {
            return Err("Sourcing Config preparation rejected its lane or description envelope".into());
        }
        sourcing_curate_config_mutation_footprint(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<SourcingCurateConfig, SourcingCurateConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SourcingCurateConfig, SourcingCurateConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<SourcingCurateConfig, SourcingCurateConfigMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATE_CONFIG_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SourcingCurateConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<SourcingCurateConfig, SourcingCurateConfigMutation> for SourcingCurateConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Sourcing Config preparation lost its exact base root".to_string())?.get();
            sourcing_curate_config_bytes(base)?;
            let bytes = SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024;
            if grant.maximum_bytes < bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.mutation.take().ok_or_else(|| "Sourcing Config preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_sourcing_curate_config(base, mutation)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Sourcing Config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Sourcing Config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(sourcing_curate_store_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<SourcingCurateConfig, SourcingCurateConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<SourcingCurateConfig, SourcingCurateConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.is_some() || self.candidate.is_some() {
            if grant.maximum_bytes < self.retained_bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            if self.prepared.take().is_none() { self.candidate = None; }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(mutation) = self.mutation.as_ref() {
            let bytes = sourcing_curate_config_mutation_footprint(mutation)?.retained_bytes;
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(description) = self.description.as_ref() {
            let bytes = description.len();
            if grant.maximum_bytes < bytes { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Sourcing Config preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

impl ArtifactEditor for SourcingCurateApp {
    type Snapshot = CurateSnapshot;
    type Mutation = SourcingMutation;
    type Config = SourcingCurateConfig;
    type ConfigMutation = SourcingCurateConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SourcingCuratePresence;
    type PresenceMutation = SourcingCuratePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SourcingCurateCommand;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(SourcingCurateConfigPreparationFactory))
    }

    const DIALECT: Dialect = crate::artifacts::curate::SOURCING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATE_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<SourcingCurateApp>,
        owner_file: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.sourcing.curate@1/*#editor",
        document_schema: "sourcing.curate/v1",
        factory: "SourcingCurateBoundedCommandJobFactory",
        factory_type: SourcingCurateBoundedCommandJobFactory,
        tools: {
            "setFilterQuery" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterTypology" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterMinAvailability" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "sortTable" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setContributions" => semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SourcingCurateBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let bounded = SOURCING_CURATE_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str());
        if !bounded {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("sourcing-curate-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, sourcing_curate_retained_reduce, sourcing_curate_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            SourcingCurateCommand::command_id,
            SOURCING_CURATE_RETAINED_RAW_BYTES,
            SOURCING_CURATE_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::sourcing::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CurateSnapshot {
        crate::artifacts::curate::schema::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(sourcing_curate_io())
    }

    /// 🎞️ `catalog:out` (see `crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment`)
    /// plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated inline —
    /// overriding `export_media` shadows the trait's provided body for every port on this app, not just
    /// the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: crate::artifacts::curate::schema::inferences::sourcing_catalog_fragment(doc.snapshot).to_string() },
            }),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (the former whole-
    /// snapshot-replace variant — see `📓️taxonomy.md`'s forbidden vocabulary), so this app does NOT
    /// override `whole_document_operation`
    /// (stays at the trait's own `None` default) and instead overrides `import_media` below to build a
    /// `Effect::LoadDocument` via `reset_document_effect`, outside undo history.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CurateSnapshot>) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "document:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <CurateSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    fn command_id(command: &SourcingCurateCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Production action bridge — see `sourcing_curate_command_from_action`. Overriding this is
    /// mandatory for any app that declares its own actions: the trait default only admits the
    /// framework-reserved ids and rejects everything else.
    fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<SourcingCurateCommand, Fault> {
        sourcing_curate_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&serde_json::Value>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| SourcingCurateConfigMutation::SetContributions {
            json: args.and_then(|value| value.get("json")).and_then(serde_json::Value::as_str).unwrap_or("[]").to_string(),
        }))
    }

    fn handle(
        command: &SourcingCurateCommand,
        doc: &ArtifactView<'_, CurateSnapshot>,
        cfg: &ConfigView<'_, SourcingCurateConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SourcingMutation, SourcingCurateConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CurateSnapshot>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let labels = sourcing_curate_labels(config);
        match body_key {
            pool::SOURCING_CURATE_BODY_POOL => pool::render(snapshot, config, labels).map(semio_framework_plugin::built_to_component_tree),
            curated::SOURCING_CURATE_BODY_CURATED => curated::render(snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // preview window degrades to its "no selection" default until a future wave threads
            // interaction into render. Flagged as a discovered framework gap, not worked around here.
            preview::SOURCING_CURATE_BODY_PREVIEW => preview::render(snapshot, &[], labels).map(semio_framework_plugin::built_to_component_tree),
            grid::SOURCING_CURATE_BODY_GRID => grid::render(snapshot, config).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data("")),
        }
    }
}
//#endregion 🔖️SourcingCurateApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"document:in"` above, `commands::document::{set_active_example, set_artifact_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub fn reset_document_effect(document: &CurateSnapshot) -> semio_framework::kernel::Effect {
    let pack = <CurateSnapshot as ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<CurateSnapshot, SourcingMutation>(SOURCING_CURATE_SCHEMA, "curate", document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("curate document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — the curate/DnD arms that mutate
/// the persisted `CurateSnapshot` but are only ever dispatched from window chrome.
fn hidden_operation(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation) }
}

/// 🙈️👁️ The filter/sort/selection/world-pick arms emit ONLY `config_mutations`, so (unlike
/// `hidden_operation` above) they're declared `ActionKind::View`, letting `VcsArtifactApp`'s
/// kind-discipline check actually enforce "must not emit document operations".
fn hidden_view_action(id: &str, label: impl Into<LocalizedLabel>) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::View) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` — the
/// pre-migration `App { definition, examples }` split means `.editor::<E>(def: AppDefinition)` only
/// ever registers an empty `examples` list. The two demo examples this app used to register here
/// (`DEMO_STOCK_EXAMPLE_ID`/`EMPTY_EXAMPLE_ID`) are dropped from `AppDefinition`, not ported — they
/// stay reachable through the `setActiveExample` action (`action_args` below) and this subset's own
/// `📚️examples` facet, just no longer wired into the manifest's `examples` list. See
/// `📓️w2-cad-report.md`'s "SDK gaps found" #4 for the same gap hit by the pilot packet.
pub fn create_sourcing_curate_app() -> AppDefinition {
    Editor::builder(crate::artifacts::curate::SOURCING_DIALECT)
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
                definition
            })
            .action_interactive_job("setContributions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .document(["semio", "sourcing", "curate"])
            .artifact_kind(crate::artifacts::curate::artifact_kind())
            .artifact_kind(ArtifactKindSpec {
                id: "catalogue.kinds".into(),
                name: "Kind Catalogue".into(),
                source_format: "catalogue.kinds".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "catalogue.kinds".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            // 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: the `catalog:out` port's declared kind —
            // harmless duplicate `ArtifactKindSpec` across producers (see `s/plugin/block`'s `3d` app,
            // which declares the SAME `kit.catalog` shape independently).
            .artifact_kind(ArtifactKindSpec {
                id: "kit.catalog".into(),
                name: "Kit Catalogue".into(),
                source_format: "kit.catalog".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "kit.catalog".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("library")
            .mode_def(edit::definition())
            .default_mode_id(edit::SOURCING_CURATE_MODE_CURATE)
            .window_kind_def(pool::definition())
            .window_kind_def(curated::definition())
            .window_kind_def(preview::definition())
            .window_kind_def(grid::definition())
            .default_layout(edit::layout())
            // 🕹️ The "rows" interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
            // MECHANISM) replaces the deleted `selected_object_id` config field and the `select-row`/
            // `world-select` commands — a curation table picks exactly one row. The six framework verbs
            // (`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity`) auto-inject; the pool/curated tables and the grid's world3d
            // pick surface all carry it.
            .interaction(InteractionDefinition {
                id: "rows".into(),
                label: LocalizedLabel::native("Rows", "Zeilen"),
                granularities: vec![GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(pool::SOURCING_CURATE_WINDOW_POOL, vec![InteractionRef::new("rows")])
            .window_kind_interactions(curated::SOURCING_CURATE_WINDOW_CURATED, vec![InteractionRef::new("rows")])
            .window_kind_interactions(grid::SOURCING_CURATE_WINDOW_GRID, vec![InteractionRef::new("rows")])
            // 🔧️ Curation counts/stock edits are persisted in `CurateSnapshot`, so each arm emits a
            // whole-document `SetArtifact` operation and is declared as a Mutation, never a View.
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("stockFromCatalogue", LocalizedLabel::native("Stock From Catalogue", "Bestand aus Katalog"))
            .action_with(hidden_operation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen")))
            .action_with(hidden_operation("curateAdd", LocalizedLabel::native("Curate Add", "Kuratierung hinzufügen")))
            .action_with(hidden_operation("curateSetCount", LocalizedLabel::native("Curate Set Count", "Kuratierte Anzahl festlegen")))
            .action_with(hidden_operation("curateRemove", LocalizedLabel::native("Curate Remove", "Kuratierung entfernen")))
            .action_with(hidden_operation("dropOnPool", LocalizedLabel::native("Drop On Pool", "Auf Pool ablegen")))
            .action_with(hidden_operation("dropOnCurated", LocalizedLabel::native("Drop On Curated", "Auf Kuratiert ablegen")))
            // 👁️ Filters/sort/selection — session-only `SourcingCurateConfig` view state, never the document.
            .action_with(hidden_view_action("setFilterQuery", LocalizedLabel::native("Set Filter Query", "Filterabfrage festlegen")))
            .action_with(hidden_view_action("setFilterModule", LocalizedLabel::native("Set Filter Module", "Filtermodul festlegen")))
            .action_with(hidden_view_action("setFilterTypology", LocalizedLabel::native("Set Filter Typology", "Filtertypologie festlegen")))
            .action_with(hidden_view_action("setFilterMinAvailability", LocalizedLabel::native("Set Filter Min Availability", "Mindestverfügbarkeit festlegen")))
            .action_with(hidden_view_action("sortTable", LocalizedLabel::native("Sort Table", "Tabelle sortieren")))
            // 📝️ Staged argument form for the panel-visible example switch.
            .action_args(
                "setActiveExample",
                vec![ActionArgDef::select(
                    "exampleId",
                    LocalizedLabel::native("Example", "Beispiel"),
                    vec![ActionArgOption::new(DEMO_STOCK_EXAMPLE_ID, LocalizedLabel::native("Demo Stock", "Beispielbestand")), ActionArgOption::new(EMPTY_EXAMPLE_ID, LocalizedLabel::native("Empty Curation", "Leere Kuratierung"))],
                )
                .default_value(DEMO_STOCK_EXAMPLE_ID)],
            )
            // 🎯️ Typed channel surface — this app's typed commands are dispatched via
            // `SourcingCurateCommand`'s `OpBinary` codec directly (`setLocale` deliberately left
            // undeclared above, mirroring `flow_ui`: `VcsArtifactApp`'s kind-discipline check only runs
            // when the registry actually declares a command's id).
            .io(sourcing_curate_io())
            .action_interactive_job("setDocument", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("stockFromCatalogue", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("curateAdd", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("curateSetCount", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("curateRemove", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("dropOnPool", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("dropOnCurated", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFilterQuery", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterModule", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setFilterTypology", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterMinAvailability", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("sortTable", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", semio_framework_plugin::InteractiveJobClassification::ForbiddenFromUi)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_app_impl, new_app_with_registry as new_app_with_registry_impl};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurateApp>>;

    /// 🧪️ Framework testkit gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
    /// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
    /// not the `AppDefinition`-returning `create_sourcing_curate_app`. Local wrapper until that lands.
    pub(crate) fn sourcing_manifest_for_testkit() -> App {
        App { definition: create_sourcing_curate_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> SourcingApp {
        new_app_impl::<EditorApp<SourcingCurateApp>>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn new_app_with_registry() -> SourcingApp {
        new_app_with_registry_impl::<EditorApp<SourcingCurateApp>>(sourcing_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut SourcingApp, command: SourcingCurateCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut SourcingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧪️RetainedConfigOracle
    #[test]
    fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
        let base = SourcingCurateConfig::default();
        let mut expected = serde_json::to_value(&base).expect("JSON oracle base");
        expected["filters"]["query"] = serde_json::json!("timber");
        let (post, inverse, _) = prepare_sourcing_curate_config(&base, SourcingCurateConfigMutation::SetFilterQuery { value: "timber".into() }).expect("bounded config candidate");
        assert_eq!(serde_json::to_value(post).expect("JSON oracle post"), expected);
        assert!(matches!(&inverse[0], SourcingCurateConfigMutation::SetFilterQuery { value } if value == &base.filters.query));
        assert!(sourcing_curate_config_mutation_footprint(&SourcingCurateConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATE_CONFIG_TEXT_BYTES) }).is_ok());
        assert!(sourcing_curate_config_mutation_footprint(&SourcingCurateConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATE_CONFIG_TEXT_BYTES + 1) }).is_err());
        assert!(sourcing_curate_config_mutation_footprint(&SourcingCurateConfigMutation::SetFilterModules { module_ids: Vec::new() }).is_err());
        assert_eq!(SOURCING_CURATE_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024, 4_096);
    }
    //#endregion 🧪️RetainedConfigOracle
    use crate::editor::sourcing::testkit::{new_app_with_registry, sourcing_manifest_for_testkit};
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{EditorApp, PluginApp};

    #[semio_framework_async_macros::async_test]
    async fn view_kind_config_only_commands_pass_kind_discipline() {
        // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }), &testkit::meta("local")).expect("filter query");
        assert!(result.mutations.is_empty(), "setFilterQuery is config-only, no document operations");
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 15, "every SourcingCurateCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: the PRODUCTION action bridge covers every declared row. `ArtifactEditor`'s default
    /// `command_from_action` rejects app-owned actions outright, so an app that declares actions but
    /// never overrides it ships a UI whose every control faults — which is exactly how this pane's
    /// `setActiveExample` (and with it the whole `demo-stock` example) was dead in the demonstrator.
    /// Asserted through `<SourcingCurateApp as ArtifactEditor>` — NOT the free function — because the
    /// trait method is the seam the host actually calls.
    #[semio_framework_async_macros::async_test]
    async fn the_production_action_bridge_admits_every_declared_command() {
        for command in every_command() {
            let action = command.command_id();
            let built = <SourcingCurateApp as ArtifactEditor>::command_from_action(action, None).unwrap_or_else(|fault| panic!("action '{action}' is declared but the production bridge rejects it: {fault:?}"));
            assert_eq!(built.command_id(), action, "the bridge routed '{action}' to the wrong command");
        }
    }

    /// ⚖️ LAW: the framework's own conformance harness — it walks the actions this app's window kinds
    /// actually RENDER, stages each one's declared args exactly as the host does, and knows which ids
    /// are framework-injected (`undo`/`copy`/`recordTutorial`/…) and must be skipped. Strictly stronger
    /// than enumerating the command rows: it catches an action that chrome declares but no command row
    /// backs.
    #[semio_framework_async_macros::async_test]
    async fn every_rendered_action_bridges_through_the_framework_harness() {
        testkit::assert_declared_actions_bridge_to_commands::<EditorApp<SourcingCurateApp>>(sourcing_manifest_for_testkit);
    }

    /// ⚖️ LAW: the bridge reads the manifest's OWN arg names — `setActiveExample` declares a select
    /// arg keyed `exampleId` (`🔖️Manifest`), and the payload field is `example_id`; the two
    /// vocabularies are joined here and nowhere else.
    #[semio_framework_async_macros::async_test]
    async fn the_action_bridge_reads_the_declared_arg_names() {
        let built = <SourcingCurateApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": DEMO_STOCK_EXAMPLE_ID }))).expect("setActiveExample must convert");
        assert_eq!(built, SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }));
        let filter = <SourcingCurateApp as ArtifactEditor>::command_from_action("setFilterModule", Some(&serde_json::json!({ "moduleId": "walls", "enabled": true }))).expect("setFilterModule must convert");
        assert_eq!(filter, SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "walls".into(), enabled: true }));
        assert!(<SourcingCurateApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err(), "an undeclared action must fault, not silently no-op");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row — the
    /// permanent successor of the old `📡️protocol` crate's
    /// `sourcing_curate_command_op_text_round_trips_every_variant`.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
    /// verbatim from each `app_commands!` row's `as "…"` literal (NOT a mechanical kebab-case of the
    /// manifest action id: `setDocument`/`document-json`, `setActiveExample`/`active-example`, the whole
    /// `setFilter*` family, and `setLocale`/`locale` all drop or rewrite the `set` prefix). This is what a
    /// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
    /// keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        async fn expected_wire_key(id: &str) -> &'static str {
            match id {
                "setDocument" => "document-json",
                "setActiveExample" => "active-example",
                "stockFromCatalogue" => "stock-from-catalogue",
                "curateAdd" => "curate-add",
                "curateSetCount" => "curate-set-count",
                "curateRemove" => "curate-remove",
                "dropOnPool" => "drop-on-pool",
                "dropOnCurated" => "drop-on-curated",
                "setFilterQuery" => "filter-query",
                "setFilterModule" => "filter-module",
                "setFilterTypology" => "filter-typology",
                "setFilterMinAvailability" => "filter-min-availability",
                "sortTable" => "sort-table",
                "setLocale" => "locale",
                "setContributions" => "contributions",
                other => panic!("expected_wire_key: unhandled command id {other}"),
            }
        }
        for command in every_command() {
            let id = command.command_id();
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_wire_key(id), "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `semio-s-app-sourcing-curate-protocol` crate
    /// (`wire-baseline-before.txt` in this ticket's folder). A regression here is a real format break, not
    /// a test-fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(SourcingCurateCommand, &str, &str); 2] = [
            (
                SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
                "curate-set-count curate-set-count object-id=beam-glulam-gl24h delta=1",
                "010401116265616d2d676c756c616d2d676c323468020006000105000000000000f03f",
            ),
            (
                SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) }),
                "curate-set-count curate-set-count object-id=beam-glulam-gl24h value=4",
                "010401116265616d2d676c756c616d2d676c3234680200060002050000000000001040",
            ),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
    /// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
    async fn every_command() -> Vec<SourcingCurateCommand> {
        vec![
            SourcingCurateCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: "{}".into() }),
            SourcingCurateCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo-stock".into() }),
            SourcingCurateCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
            SourcingCurateCommand::CurateAdd(curate_add::CurateAdd { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::CurateSetCount(curate_set_count::CurateSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
            SourcingCurateCommand::CurateRemove(curate_remove::CurateRemove { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: "beam-glulam-gl24h".into() }),
            SourcingCurateCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }),
            SourcingCurateCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "beams".into(), enabled: true }),
            SourcingCurateCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: "beams/steel".into() }),
            SourcingCurateCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }),
            SourcingCurateCommand::SortTable(sort_table::SortTable { column_id: "availability".into(), direction: "desc".into() }),
            SourcingCurateCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            SourcingCurateCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    #[semio_framework_async_macros::async_test]
    async fn app_definition_labels_resolve_german() {
        let def = &create_sourcing_curate_app();
        let (terminology, locale) = (semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De);
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATE_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
        assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATE_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
        assert_eq!(def.modes.iter().find(|entry| entry.id == edit::SOURCING_CURATE_MODE_CURATE).expect("curate mode").label.resolve(terminology, locale), "Kuratieren");
    }

    #[test]
    fn host_contributions_resolve_to_the_event_sourced_config_lane() {
        let mutation = <SourcingCurateApp as ArtifactEditor>::host_configuration_mutation(
            "setContributions",
            Some(&serde_json::json!({ "json": "[{\"id\":\"sourcing\"}]" })),
        )
        .expect("host configuration")
        .expect("sourcing contribution mutation");
        assert_eq!(mutation, SourcingCurateConfigMutation::SetContributions { json: "[{\"id\":\"sourcing\"}]".into() });
        assert_eq!(<SourcingCurateApp as ArtifactEditor>::host_configuration_mutation("setFilterQuery", None).expect("non-host action"), None);
    }

    #[test]
    fn retained_factories_declare_every_publication_lane() {
        let bounded = <SourcingCurateBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
        assert_eq!(bounded.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), SOURCING_CURATE_BOUNDED_TOOL_IDS);
        assert!(bounded.iter().all(|contract| contract.lanes == [ArtifactToolPublicationLane::Config]));
        assert_eq!(SOURCING_CURATE_BATCH_ONLY_TOOL_IDS.len(), 9);
        assert!(SOURCING_CURATE_BATCH_ONLY_TOOL_IDS.contains(&"setFilterModule"));
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curate_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
        let io = sourcing_curate_io();
        assert_eq!(io.document_schema, SOURCING_CURATE_SCHEMA);
        let ports = io.all_ports();
        assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
        let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog_out.media_type.class, MediaClass::Kit);
        assert_eq!(catalog_out.media_type.form, MediaForm::Type);
    }

    #[semio_framework_async_macros::async_test]
    async fn sourcing_curate_io_and_catalog_export_round_trip() {
        let mut app = crate::editor::sourcing::testkit::new_app();
        let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("catalog export");
        assert_eq!(media.media_type.class, MediaClass::Kit);
        assert_eq!(media.media_type.form, MediaForm::Type);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let fragment: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.snapshot().expect("snapshot").stock_extra.len());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[test]
    fn retained_route_catalog_is_exact_and_batch_only_is_explicit() {
        let mut routes = SOURCING_CURATE_BOUNDED_TOOL_IDS.to_vec();
        routes.extend_from_slice(SOURCING_CURATE_BATCH_ONLY_TOOL_IDS);
        routes.sort_unstable();
        routes.dedup();
        assert_eq!(routes.len(), 14);
        assert!(SOURCING_CURATE_BOUNDED_TOOL_IDS.iter().all(|route| !SOURCING_CURATE_BATCH_ONLY_TOOL_IDS.contains(route)));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_sourcing_curate_app()).expect("app definition json");
        for id in [pool::SOURCING_CURATE_WINDOW_POOL, curated::SOURCING_CURATE_WINDOW_CURATED, preview::SOURCING_CURATE_WINDOW_PREVIEW, grid::SOURCING_CURATE_WINDOW_GRID] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::SOURCING_CURATE_MODE_CURATE), "mode missing from the manifest");
        assert!(json.contains("catalogue.sourcing"), "artifact kind missing from the manifest");
    }
}
//#endregion 🧪️Tests
