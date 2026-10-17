//! 🛒️ Sourcing curation app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `🧬️schema`. This file is a routing table: `handle` → `SourcingCurationCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::{CurationSnapshot, CuratedItem, ObjectKindExtra, SOURCING_CURATION_SCHEMA};
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation, SetFilterQueryEdit, SetFilterModulesEdit, SetFilterTypologyEdit, SetFilterMinAvailabilityEdit, SetSortEdit, SetContributionsEdit};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::modes::edit::windows::grid::config::{self as grid_config, GridWindowConfigMutation};
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::presence::{self, SourcingCurationPresence, SourcingCurationPresenceMutation};
use crate::editor::sourcing::terminology::sourcing_curation_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;


//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SourcingCurationApp::Command` — the SOLE dispatch surface for curation's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) —
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SourcingCurationCommand for CurationSnapshot, SourcingMutation, SourcingCurationConfig, SourcingCurationConfigMutation {
        "loadDocumentJson" as "load-document-json" => load_document_json::LoadDocumentJson,
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
        "setGridInstanceDisplay" as "grid-instance-display" => set_grid_instance_display::SetGridInstanceDisplay,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::sourcing::commands::set_contributions;
use crate::editor::sourcing::commands::{curation_add, curation_remove, curation_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::commands::{set_active_example, load_document_json, stock_from_catalogue};
use crate::editor::sourcing::commands::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, set_grid_instance_display, sort_table};

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
fn sourcing_curation_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<SourcingCurationCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
    let text_of = |key: &str| -> Option<String> {
        args.and_then(|value| value.get(key)).and_then(|value| match value {
            semio_framework_value::DslValue::String(text) => Some(text.clone()),
            semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
            semio_framework_value::DslValue::Number(number) => Some(match number {
                semio_framework_value::Number::UInt(number) => number.to_string(),
                semio_framework_value::Number::Int(number) => number.to_string(),
                semio_framework_value::Number::Float(number) => number.to_string(),
            }),
            _ => None,
        })
    };
    let json_field = |key: &str| -> String {
        match args.and_then(|value| value.get(key)) {
            Some(semio_framework_value::DslValue::String(text)) => text.clone(),
            Some(other) => semio_framework_pack_json::to_json_string(other),
            None => args.map(semio_framework_pack_json::to_json_string).unwrap_or_default(),
        }
    };
    let object_id = || str_field("objectId").unwrap_or_default();
    Ok(match action {
        "loadDocumentJson" => SourcingCurationCommand::LoadDocumentJson(load_document_json::LoadDocumentJson { json: json_field("json") }),
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
        "setGridInstanceDisplay" => SourcingCurationCommand::SetGridInstanceDisplay(set_grid_instance_display::SetGridInstanceDisplay { value: str_field("value").or_else(|| text_of("value")).unwrap_or_default() }),
        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{other}' is not a sourcing curation command"))),
    })
}
//#endregion 🔖️Commands

//#region 🔖️SourcingCurationApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::editor::sourcing::config::
/// SourcingCurationConfig`, written through `SourcingCurationConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurationApp;

#[path = "🧰️retained-commands/🦀️.rs"]
mod retained_commands;
pub use self::retained_commands::*;

#[path = "📬️config-store-preparation/🦀️.rs"]
mod config_store_preparation;
pub use self::config_store_preparation::*;

#[path = "🗄️artifact-store-preparation/🦀️.rs"]
mod artifact_store_preparation;
pub use self::artifact_store_preparation::*;

#[path = "🧩️sourcing-app/🦀️.rs"]
mod sourcing_app;
pub use self::sourcing_app::*;
//#endregion 🔖️SourcingCurationApp


#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use self::manifest::*;

//#region 🧪️UnitTests
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts
