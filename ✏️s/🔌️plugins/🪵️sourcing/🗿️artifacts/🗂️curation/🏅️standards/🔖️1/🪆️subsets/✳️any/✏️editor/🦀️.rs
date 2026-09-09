//! 🛒️ Sourcing curation app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `🧬️schema`. This file is a routing table: `handle` → `SourcingCurationCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::op::SourcingMutation;
use crate::{CurationSnapshot, CuratedItem, ObjectKindExtra, SOURCING_CURATION_SCHEMA};
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::presence::{self, SourcingCurationPresence, SourcingCurationPresenceMutation};
use crate::editor::sourcing::terminology::sourcing_curation_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest,
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
/// `SOURCING_CURATION_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `crate::schema::inferences::sourcing_catalog_fragment`).
pub fn sourcing_curation_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: SOURCING_CURATION_SCHEMA.into(),
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
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curation";
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
    /// 🎯️ `SourcingCurationApp::Command` — the SOLE dispatch surface for curation's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) —
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SourcingCurationCommand for CurationSnapshot, SourcingMutation, SourcingCurationConfig, SourcingCurationConfigMutation {
        "setDocument" as "document-json" => set_artifact_json::SetArtifactJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "stockFromCatalogue" as "stock-from-catalogue" => stock_from_catalogue::StockFromCatalogue,
        "curationAdd" as "curation-add" => curation_add::CurationAdd,
        "curationSetCount" as "curation-set-count" => curation_set_count::CurationSetCount,
        "curationRemove" as "curation-remove" => curation_remove::CurationRemove,
        "dropOnPool" as "drop-on-pool" => drop_on_pool::DropOnPool,
        "dropOnCurated" as "drop-on-curated" => drop_on_curated::DropOnCurated,
        "setFilterQuery" as "filter-query" => set_filter_query::SetFilterQuery,
        "setFilterModule" as "filter-module" => set_filter_module::SetFilterModule,
        "setFilterTypology" as "filter-typology" => set_filter_typology::SetFilterTypology,
        "setFilterMinAvailability" as "filter-min-availability" => set_filter_min_availability::SetFilterMinAvailability,
        "sortTable" as "sort-table" => sort_table::SortTable,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::sourcing::commands::set_contributions;
use crate::editor::sourcing::commands::{curation_add, curation_remove, curation_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::commands::{set_active_example, set_artifact_json, stock_from_catalogue};
use crate::editor::sourcing::commands::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, sort_table};

/// 🎯️ Host action id + JSON args → the closed `SourcingCurationCommand` vocabulary — the production
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
fn sourcing_curation_command_from_action(action: &str, args: Option<&protocol::DslValue>) -> Result<SourcingCurationCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_bool);
    let text_of = |key: &str| -> Option<String> {
        args.and_then(|value| value.get(key)).and_then(|value| match value {
            protocol::DslValue::String(text) => Some(text.clone()),
            protocol::DslValue::Bool(flag) => Some(flag.to_string()),
            protocol::DslValue::Number(number) => Some(match number {
                protocol::Number::UInt(number) => number.to_string(),
                protocol::Number::Int(number) => number.to_string(),
                protocol::Number::Float(number) => number.to_string(),
            }),
            _ => None,
        })
    };
    let json_field = |key: &str| -> String {
        match args.and_then(|value| value.get(key)) {
            Some(protocol::DslValue::String(text)) => text.clone(),
            Some(other) => protocol::json::to_json_string(other),
            None => args.map(protocol::json::to_json_string).unwrap_or_default(),
        }
    };
    let object_id = || str_field("objectId").unwrap_or_default();
    Ok(match action {
        "setDocument" => SourcingCurationCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: json_field("json") }),
        "setActiveExample" => SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "stockFromCatalogue" => SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        "curationAdd" => SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id() }),
        "curationSetCount" => SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id(), delta: f64_field("delta"), value: f64_field("value") }),
        "curationRemove" => SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: object_id() }),
        "dropOnPool" => SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id() }),
        "dropOnCurated" => SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id() }),
        "setFilterQuery" => SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: text_of("value").unwrap_or_default() }),
        "setFilterModule" => SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: str_field("moduleId").unwrap_or_default(), enabled: bool_field("enabled").unwrap_or(false) }),
        "setFilterTypology" => SourcingCurationCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: str_field("path").or_else(|| text_of("value")).unwrap_or_default() }),
        "setFilterMinAvailability" => SourcingCurationCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: f64_field("delta"), value: f64_field("value") }),
        "sortTable" => SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: str_field("columnId").unwrap_or_default(), direction: str_field("direction").unwrap_or_default() }),
        "setContributions" => SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: json_field("json") }),
        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{other}' is not a sourcing curation command"))),
    })
}
//#endregion 🔖️Commands

//#region 🔖️SourcingCurationApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::editor::sourcing::config::
/// SourcingCurationConfig`, written through `SourcingCurationConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurationApp;

//#region 🧵️RetainedCommands
/// 🧵️ Every UI-reachable command of this app, all on the retained bounded first-step lane. A command
/// left off this list is unreachable at runtime, not merely untested: `validate_ui_dispatch_classification`
/// rejects any dispatch whose registry classification is not `Migrated`, and `qualified_tool_proof`
/// refuses a typed command that owns no tool proof — so the eight ids that used to sit in a
/// `BATCH_ONLY_PENDING_REWRITE` list (the whole curation vocabulary plus the module filter and both
/// whole-document replacements) could not be invoked from the browser at all.
const SOURCING_CURATION_BOUNDED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "setDocument",
    "stockFromCatalogue",
    "curationAdd",
    "curationSetCount",
    "curationRemove",
    "dropOnPool",
    "dropOnCurated",
    "setFilterQuery",
    "setFilterModule",
    "setFilterTypology",
    "setFilterMinAvailability",
    "sortTable",
    "setContributions",
];
const SOURCING_CURATION_RETAINED_SCHEMA: &str = "sourcing.curation/v1.tool-command.v1";
const SOURCING_CURATION_RETAINED_RAW_BYTES: usize = 8_192;
const SOURCING_CURATION_RETAINED_WORK_ITEMS: usize = 1;

fn sourcing_curation_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SOURCING_CURATION_RETAINED_RAW_BYTES, 64, 1, 16_384, 7_500)
}

fn sourcing_curation_bounded_extent(command: &SourcingCurationCommand, _snapshot: &CurationSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn sourcing_curation_retained_reduce(
    command: &SourcingCurationCommand,
    snapshot: &CurationSnapshot,
    config: &SourcingCurationConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<SourcingCurationApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, NoDraftMutation>, Fault> {
    if !SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&command.command_id()) { return Err(Fault::from("sourcing-curation-retained-route-mismatch")); }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    command.dispatch(&doc, &cfg)
}

struct SourcingCurationBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl SourcingCurationBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SOURCING_CURATION_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SourcingCurationBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<SourcingCurationApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<SourcingCurationApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SOURCING_CURATION_RETAINED_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        sourcing_curation_bounded_contract()
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
        if input.declared_bytes() > SOURCING_CURATION_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Sourcing bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for SourcingCurationBoundedCommandJobFactory {
    type Owner = EditorApp<SourcingCurationApp>;
    const TOOL_IDS: &'static [&'static str] = SOURCING_CURATION_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATION_SCHEMA;
    /// 🛤️ The lane each tool publishes on. `HostOnly` is a whole-document replacement carried as an
    /// `Effect::LoadDocument` rather than a store edit (`setDocument`/`stockFromCatalogue` share
    /// `setActiveExample`'s `reset_document_effect` path); `Artifact` is the curated-selection
    /// vocabulary, admitted by `SourcingCurationArtifactPreparationFactory`; `Config` is view state.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setDocument", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "stockFromCatalogue", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "curationAdd", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "curationSetCount", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "curationRemove", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "dropOnPool", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "dropOnCurated", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setFilterQuery", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterModule", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterTypology", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setFilterMinAvailability", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "sortTable", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}

//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
const SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES: usize = 768;
const SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS: usize = 256;
const SOURCING_CURATION_CONFIG_TEXT_BYTES: usize = 96;
const SOURCING_CURATION_CONFIG_METADATA_BYTES: usize = 64;

struct SourcingCurationConfigPreparationFactory;

struct SourcingCurationConfigPreparation {
    base: Option<store::SnapshotRead<SourcingCurationConfig>>,
    mutation: Option<SourcingCurationConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(SourcingCurationConfig, Vec<SourcingCurationConfigMutation>, SourcingCurationConfigMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<SourcingCurationConfig, SourcingCurationConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

fn sourcing_curation_config_bytes(config: &SourcingCurationConfig) -> Result<usize, String> {
    let items = config.filters.module_ids.len().saturating_add(config.filters.typology_path.len());
    if items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config base exceeds its retained item envelope".into()); }
    let bytes = config.filters.query.len()
        .saturating_add(config.filters.module_ids.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.typology_path.iter().map(String::len).sum::<usize>())
        .saturating_add(config.filters.sort.as_ref().map_or(0, |sort| sort.column_id.len()))

        .saturating_add(config.contributions_json.len());
    if bytes > SOURCING_CURATION_CONFIG_TEXT_BYTES { return Err("Sourcing Config base exceeds its encoded text envelope".into()); }
    let bytes = bytes.saturating_add(size_of::<SourcingCurationConfig>())
        .saturating_add(items.saturating_mul(size_of::<String>()));
    if bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES { return Err("Sourcing Config base exceeds its retained byte envelope".into()); }
    Ok(bytes)
}

fn sourcing_curation_config_mutation_footprint(mutation: &SourcingCurationConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let (work_items, retained_bytes) = match mutation {
        SourcingCurationConfigMutation::Snapshot { .. } => return Err("Sourcing Config preparation rejects a non-retained mutation".into()),
        SourcingCurationConfigMutation::SetFilterQuery { value } | SourcingCurationConfigMutation::SetContributions { json: value } => (1, value.len()),
        SourcingCurationConfigMutation::SetFilterModules { module_ids } => {
            if module_ids.len() > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config module filter exceeds its retained item envelope".into()); }
            (module_ids.len().max(1), module_ids.iter().map(String::len).sum())
        }
        SourcingCurationConfigMutation::SetFilterTypology { path } => {
            if path.len() > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS { return Err("Sourcing Config typology exceeds its retained item envelope".into()); }
            (path.len().max(1), path.iter().map(String::len).sum())
        }
        SourcingCurationConfigMutation::SetSort { sort } => (1, sort.as_ref().map_or(0, |sort| sort.column_id.len())),
        SourcingCurationConfigMutation::SetFilterMinAvailability { .. } => (1, 0),
    };
    if retained_bytes > SOURCING_CURATION_CONFIG_TEXT_BYTES { return Err("Sourcing Config mutation exceeds its encoded text envelope".into()); }
    let retained_bytes = retained_bytes.saturating_add(size_of::<SourcingCurationConfigMutation>()).saturating_add(work_items.saturating_mul(size_of::<String>()));
    if work_items > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS || retained_bytes > SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Sourcing Config mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}

fn prepare_sourcing_curation_config(base: &SourcingCurationConfig, mutation: SourcingCurationConfigMutation) -> Result<(SourcingCurationConfig, Vec<SourcingCurationConfigMutation>, SourcingCurationConfigMutation), String> {
    sourcing_curation_config_mutation_footprint(&mutation)?;
    sourcing_curation_config_bytes(base)?;
    let mut post = base.clone();
    let inverse = match &mutation {
        SourcingCurationConfigMutation::SetFilterQuery { value } => { post.filters.query = value.clone(); SourcingCurationConfigMutation::SetFilterQuery { value: base.filters.query.clone() } }
        SourcingCurationConfigMutation::SetFilterTypology { path } => { post.filters.typology_path = path.clone(); SourcingCurationConfigMutation::SetFilterTypology { path: base.filters.typology_path.clone() } }
        SourcingCurationConfigMutation::SetFilterModules { module_ids } => { post.filters.module_ids = module_ids.clone(); SourcingCurationConfigMutation::SetFilterModules { module_ids: base.filters.module_ids.clone() } }
        SourcingCurationConfigMutation::SetFilterMinAvailability { value } => { post.filters.min_availability = *value; SourcingCurationConfigMutation::SetFilterMinAvailability { value: base.filters.min_availability } }
        SourcingCurationConfigMutation::SetSort { sort } => { post.filters.sort = sort.clone(); SourcingCurationConfigMutation::SetSort { sort: base.filters.sort.clone() } }
        SourcingCurationConfigMutation::SetContributions { json } => { post.contributions_json = json.clone(); SourcingCurationConfigMutation::SetContributions { json: base.contributions_json.clone() } }
        _ => return Err("Sourcing Config preparation rejects a non-retained mutation".into()),
    };
    sourcing_curation_config_bytes(&post)?;
    Ok((post, vec![inverse], mutation))
}

fn sourcing_curation_store_edit(
    forward: SourcingCurationConfigMutation,
    inverse: Vec<SourcingCurationConfigMutation>,
    description: Option<String>,
    authority: &store::ArtifactStoreOneItemLiveAuthority,
) -> protocol::Edit<SourcingCurationConfigMutation> {
    let id = format!("sourcing-curation-config-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<SourcingCurationConfig, SourcingCurationConfigMutation> for SourcingCurationConfigPreparationFactory {
    fn preflight(&self, mutation: &SourcingCurationConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > SOURCING_CURATION_CONFIG_METADATA_BYTES) {
            return Err("Sourcing Config preparation rejected its lane or description envelope".into());
        }
        sourcing_curation_config_mutation_footprint(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<SourcingCurationConfig, SourcingCurationConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<SourcingCurationConfig, SourcingCurationConfigMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_CONFIG_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SourcingCurationConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<SourcingCurationConfig, SourcingCurationConfigMutation> for SourcingCurationConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Sourcing Config preparation lost its exact base root".to_string())?.get();
            sourcing_curation_config_bytes(base)?;
            let bytes = SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES * 4 + 1_024;
            if grant.maximum_bytes < bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.mutation.take().ok_or_else(|| "Sourcing Config preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_sourcing_curation_config(base, mutation)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Sourcing Config preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Sourcing Config preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(sourcing_curation_store_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<SourcingCurationConfig, SourcingCurationConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<SourcingCurationConfig, SourcingCurationConfigMutation>> { self.prepared.take() }
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
            let bytes = sourcing_curation_config_mutation_footprint(mutation)?.retained_bytes;
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

//#region 📬️ArtifactStorePreparation
/// 🧺️ Fixed envelope for the curation document's retained one-item publication lane. The document is a
/// curated-selection list over a content-addressed catalog child, so the only unbounded axes are the
/// curated rows and the sourcing-owned `stock_extra` overflow — both counted here rather than encoded.
const SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest object id (and longest single mutation payload string) the retained lane admits.
const SOURCING_CURATION_DOCUMENT_TEXT_BYTES: usize = 256;
const SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;
const SOURCING_CURATION_DOCUMENT_METADATA_BYTES: usize = 64;
/// 🎟️ What one `advance`/`close_step` turn costs, and the ONLY figure the grant is ever compared
/// against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant { maximum_items: 1,
/// maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// curation outgrew one page, stalling the operation instead of failing it. The base's own size is a
/// VALIDATION (`sourcing_curation_document_bytes`, rejected past
/// `SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES`), never the gate. Same "demand exactly one full grant"
/// shape as the config lane above.
const SOURCING_CURATION_DOCUMENT_GRANT_BYTES: usize = 4_096;

struct SourcingCurationArtifactPreparationFactory;

struct SourcingCurationArtifactPreparation {
    base: Option<store::SnapshotRead<CurationSnapshot>>,
    mutation: Option<SourcingMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(CurationSnapshot, Vec<SourcingMutation>, SourcingMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<CurationSnapshot, SourcingMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

/// 📏️ The retained footprint of one document base — rejected rather than truncated when the curated
/// list, the overflow catalog half, or any single id outgrows the fixed envelope.
fn sourcing_curation_document_bytes(document: &CurationSnapshot) -> Result<usize, String> {
    let items = document.curated.len().saturating_add(document.stock_extra.len());
    if items > SOURCING_CURATION_DOCUMENT_MAXIMUM_ITEMS { return Err("Sourcing Curation base exceeds its retained item envelope".into()); }
    if document.curated.iter().any(|item| item.object_id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) || document.stock_extra.iter().any(|extra| extra.id.len() > SOURCING_CURATION_DOCUMENT_TEXT_BYTES) {
        return Err("Sourcing Curation base carries an object id beyond its encoded text envelope".into());
    }
    let bytes = document.curated.iter().map(|item| item.object_id.len()).sum::<usize>()
        .saturating_add(document.stock_extra.iter().map(|extra| extra.id.len().saturating_add(extra.name.len()).saturating_add(extra.module_id.len()).saturating_add(extra.typology_path.iter().map(String::len).sum::<usize>())).sum::<usize>())
        .saturating_add(size_of::<CurationSnapshot>())
        .saturating_add(items.saturating_mul(size_of::<CuratedItem>().max(size_of::<ObjectKindExtra>())));
    if bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation base exceeds its retained byte envelope".into()); }
    Ok(bytes)
}

/// 📏️ One semantic mutation's own retained footprint. Every variant addresses exactly one curated row,
/// so the item count is one and the byte count is that row's id.
fn sourcing_curation_mutation_footprint(mutation: &SourcingMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let text = match mutation {
        SourcingMutation::CreateCuratedItem(payload) => payload.item.object_id.len(),
        SourcingMutation::DeleteCuratedItem(payload) => payload.object_id.len(),
        SourcingMutation::ChangeCuratedItemCount(payload) => payload.object_id.len(),
    };
    if text > SOURCING_CURATION_DOCUMENT_TEXT_BYTES { return Err("Sourcing Curation mutation exceeds its encoded text envelope".into()); }
    let retained_bytes = text.saturating_add(size_of::<SourcingMutation>()).saturating_add(size_of::<String>());
    if retained_bytes > SOURCING_CURATION_DOCUMENT_MAXIMUM_BYTES { return Err("Sourcing Curation mutation exceeds its fixed one-item preparation envelope".into()); }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff.
/// A `Fatal`/`Error` outcome (duplicate id, missing target) is a REJECTION here, not a silent no-op —
/// the retained lane must never publish an edit whose forward diff the vocabulary refused.
fn prepare_sourcing_curation_document(base: &CurationSnapshot, mutation: SourcingMutation) -> Result<(CurationSnapshot, Vec<SourcingMutation>, SourcingMutation), String> {
    sourcing_curation_mutation_footprint(&mutation)?;
    sourcing_curation_document_bytes(base)?;
    let outcome = protocol::Mutation::diff(&mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal)) {
        return Err(format!("Sourcing Curation mutation was refused by its own vocabulary: {}", message.message));
    }
    let inverse = protocol::Mutation::inverse(&mutation, base);
    let post = protocol::MutationDiff::apply(outcome.diff(), base).map_err(|error| format!("Sourcing Curation mutation could not apply onto its exact base: {}", error.message))?;
    sourcing_curation_document_bytes(&post)?;
    Ok((post, inverse, mutation))
}

fn sourcing_curation_document_edit(
    forward: SourcingMutation,
    inverse: Vec<SourcingMutation>,
    description: Option<String>,
    authority: &store::ArtifactStoreOneItemLiveAuthority,
) -> protocol::Edit<SourcingMutation> {
    let id = format!("sourcing-curation-document-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<CurationSnapshot, SourcingMutation> for SourcingCurationArtifactPreparationFactory {
    fn preflight(&self, mutation: &SourcingMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > SOURCING_CURATION_DOCUMENT_METADATA_BYTES) {
            return Err("Sourcing Curation preparation rejected its lane or description envelope".into());
        }
        sourcing_curation_mutation_footprint(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: SOURCING_CURATION_DOCUMENT_GRANT_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CurationSnapshot, SourcingMutation>>, store::ArtifactStoreOneItemPreparationRequest<CurationSnapshot, SourcingMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > SOURCING_CURATION_DOCUMENT_METADATA_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(SourcingCurationArtifactPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<CurationSnapshot, SourcingMutation> for SourcingCurationArtifactPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Sourcing Curation preparation lost its exact base root".to_string())?.get();
            sourcing_curation_document_bytes(base)?;
            let bytes = SOURCING_CURATION_DOCUMENT_GRANT_BYTES;
            if grant.maximum_bytes < bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
            let mutation = self.mutation.take().ok_or_else(|| "Sourcing Curation preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_sourcing_curation_document(base, mutation)?);
            self.retained_bytes = bytes;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: bytes as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Sourcing Curation preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Sourcing Curation preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(sourcing_curation_document_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<CurationSnapshot, SourcingMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<CurationSnapshot, SourcingMutation>> { self.prepared.take() }
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
            let bytes = sourcing_curation_mutation_footprint(mutation)?.retained_bytes;
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
            if !base.return_to_registry() { return Err("Sourcing Curation preparation could not return its exact base root".into()); }
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
//#endregion 📬️ArtifactStorePreparation


//#region 🧹️EmptyLaneRetirement
struct SourcingNoTransientStoreDisposer;

impl semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>> for SourcingNoTransientStoreDisposer {
    fn close_step(
        &mut self,
        _owner: &mut store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>,
        maximum_items: usize,
        _maximum_bytes: usize,
    ) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        if maximum_items == 0 { return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        assert_eq!(size_of::<semio_framework_plugin::NoTransient>(), 0);
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self, _owner: &store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>) -> bool {
        size_of::<semio_framework_plugin::NoTransient>() == 0
    }
}
//#endregion 🧹️EmptyLaneRetirement

impl ArtifactEditor for SourcingCurationApp {
    type Snapshot = CurationSnapshot;
    type Mutation = SourcingMutation;
    type Config = SourcingCurationConfig;
    type ConfigMutation = SourcingCurationConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = SourcingCurationPresence;
    type PresenceMutation = SourcingCurationPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = SourcingCurationCommand;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        assert_eq!(size_of::<NoDraft>(), 0);
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::SourcingPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::SourcingPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(presence::SourcingPresenceStoreDisposer::new()))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(Box::new(SourcingNoTransientStoreDisposer))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(SourcingCurationArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(SourcingCurationConfigPreparationFactory))
    }

    const DIALECT: Dialect = crate::SOURCING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATION_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<SourcingCurationApp>,
        owner_file: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.sourcing.curation@1/*#editor",
        document_schema: "sourcing.curation/v1",
        factory: "SourcingCurationBoundedCommandJobFactory",
        factory_type: SourcingCurationBoundedCommandJobFactory,
        tools: {
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setDocument" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "stockFromCatalogue" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "curationAdd" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "curationSetCount" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "curationRemove" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "dropOnPool" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "dropOnCurated" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterQuery" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterModule" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterTypology" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setFilterMinAvailability" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "sortTable" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setContributions" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(SourcingCurationBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let bounded = SOURCING_CURATION_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str());
        if !bounded {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("sourcing-curation-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, sourcing_curation_retained_reduce, sourcing_curation_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            SourcingCurationCommand::command_id,
            SOURCING_CURATION_RETAINED_RAW_BYTES,
            SOURCING_CURATION_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::sourcing::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CurationSnapshot {
        crate::schema::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(sourcing_curation_io())
    }

    /// 🎞️ `catalog:out` (see `crate::schema::inferences::sourcing_catalog_fragment`)
    /// plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated inline —
    /// overriding `export_media` shadows the trait's provided body for every port on this app, not just
    /// the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, CurationSnapshot>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: dsl::json::to_json_string(&crate::schema::inferences::sourcing_catalog_fragment(doc.snapshot)) },
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
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CurationSnapshot>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "document:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <CurationSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// not a user-facing action).
    fn command_id(command: &SourcingCurationCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Production action bridge — see `sourcing_curation_command_from_action`. Overriding this is
    /// mandatory for any app that declares its own actions: the trait default only admits the
    /// framework-reserved ids and rejects everything else.
    fn command_from_action(action: &str, args: Option<&protocol::DslValue>) -> Result<SourcingCurationCommand, Fault> {
        sourcing_curation_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&protocol::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| SourcingCurationConfigMutation::SetContributions {
            json: args.and_then(|value| value.get("json")).and_then(protocol::DslValue::as_str).unwrap_or("[]").to_string(),
        }))
    }

    fn handle(
        command: &SourcingCurationCommand,
        doc: &ArtifactView<'_, CurationSnapshot>,
        cfg: &ConfigView<'_, SourcingCurationConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CurationSnapshot>, cfg: &ConfigView<'_, SourcingCurationConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let config = cfg.snapshot;
        let labels = sourcing_curation_labels(view_state);
        match body_key {
            pool::SOURCING_CURATION_BODY_POOL => pool::render(snapshot, config, labels).map(semio_framework_plugin::built_to_component_tree),
            curated::SOURCING_CURATION_BODY_CURATED => curated::render(snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
            // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) — the
            // preview window degrades to its "no selection" default until a future wave threads
            // interaction into render. Flagged as a discovered framework gap, not worked around here.
            preview::SOURCING_CURATION_BODY_PREVIEW => preview::render(snapshot, &[], labels).map(semio_framework_plugin::built_to_component_tree),
            grid::SOURCING_CURATION_BODY_GRID => grid::render(snapshot, config).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data("")),
        }
    }
}
//#endregion 🔖️SourcingCurationApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"document:in"` above, `commands::document::{set_active_example, set_artifact_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub fn reset_document_effect(document: &CurationSnapshot) -> semio_framework::kernel::Effect {
    let pack = <CurationSnapshot as ArtifactPack>::encode_pack(document);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("curation", SOURCING_CURATION_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🙈️ An internal document operation kept out of the command palette — the curation/DnD arms that mutate
/// the persisted `CurationSnapshot` but are only ever dispatched from window chrome.
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
pub fn create_sourcing_curation_app() -> AppDefinition {
    Editor::builder(crate::SOURCING_DIALECT)
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
                definition
            })
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .document(["semio", "sourcing", "curation"])
            .artifact_kind(crate::artifact_kind())
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
            .default_mode_id(edit::SOURCING_CURATION_MODE_CURATION)
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
            .window_kind_interactions(pool::SOURCING_CURATION_WINDOW_POOL, vec![InteractionRef::new("rows")])
            .window_kind_interactions(curated::SOURCING_CURATION_WINDOW_CURATED, vec![InteractionRef::new("rows")])
            .window_kind_interactions(grid::SOURCING_CURATION_WINDOW_GRID, vec![InteractionRef::new("rows")])
            // 🔧️ Curation counts/stock edits are persisted in `CurationSnapshot`, so each arm emits a
            // whole-document `SetArtifact` operation and is declared as a Mutation, never a View.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .mutation("stockFromCatalogue", LocalizedLabel::native("Stock From Catalogue", "Bestand aus Katalog"))
            .action_with(hidden_operation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen")))
            .action_with(hidden_operation("curationAdd", LocalizedLabel::native("Curation Add", "Kuratierung hinzufügen")))
            .action_with(hidden_operation("curationSetCount", LocalizedLabel::native("Curation Set Count", "Kuratierte Anzahl festlegen")))
            .action_with(hidden_operation("curationRemove", LocalizedLabel::native("Curation Remove", "Kuratierung entfernen")))
            .action_with(hidden_operation("dropOnPool", LocalizedLabel::native("Drop On Pool", "Auf Pool ablegen")))
            .action_with(hidden_operation("dropOnCurated", LocalizedLabel::native("Drop On Curated", "Auf Kuratiert ablegen")))
            // 👁️ Filters/sort/selection — session-only `SourcingCurationConfig` view state, never the document.
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
                .default_value(&DEMO_STOCK_EXAMPLE_ID)],
            )
            // 🎯️ Typed channel surface — this app's typed commands are dispatched via
            // undeclared above, mirroring `flow_ui`: `VcsArtifactApp`'s kind-discipline check only runs
            // when the registry actually declares a command's id).
            .io(sourcing_curation_io())
            .action_interactive_job("setDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("stockFromCatalogue", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationAdd", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationSetCount", InteractiveJobClassification::Migrated)
            .action_interactive_job("curationRemove", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropOnPool", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropOnCurated", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterQuery", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterModule", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterTypology", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFilterMinAvailability", InteractiveJobClassification::Migrated)
            .action_interactive_job("sortTable", InteractiveJobClassification::Migrated)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
