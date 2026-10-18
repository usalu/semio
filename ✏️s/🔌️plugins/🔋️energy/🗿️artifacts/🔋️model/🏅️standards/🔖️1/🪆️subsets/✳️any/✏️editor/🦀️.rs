//! 🔋️ Energy model editor — the authored `ArtifactEditor` surface for `s.energy.model@1/*`
//! (tickets 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, 26/09/06/ENERGY-PLUGIN-END-TO-END).
//! Three windows: `structure` (framework `TreeWindowKit` over the whole `crate::model::Model`),
//! `zones` (framework `TableWindowKit` over `Model::zones`) and `simulation` (run settings plus the
//! framework-reported state of the `energySimulation` tool run, whose job is `🧵️simulation-session`).
//!
//! 🧵️ Dispatch: every action this editor declares is `InteractiveJobClassification::Migrated` AND
//! carries an exact app-owned bounded-first-step proof. Both halves are load-bearing —
//! `VcsArtifactApp::dispatch_action` runs `require_complete_tool_operation_pipeline`, which refuses
//! anything but a `QualifiedToolProof::AppOwned` with `interactive-job.missing-owned-reducer`, and
//! `AppActionRegistry::validate_tool_job_rows` demands set EQUALITY between
//! `<EnergyModelEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS ∩ migrated-action-ids` and the
//! `bounded_first_step_tool_proofs!` rows (else `interactive-job.catalog-incomplete`). The two
//! framework-kit actions `set-node`/`set-cell` are stamped `Migrated` by `window_kind_definition`
//! itself, so they belong to that set too and appear in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] with
//! everything else.
//!
//! 📬️ Publication lanes: the twelve document verbs declare `ArtifactToolPublicationLane::Artifact`
//! (`setActiveExample` does NOT — it publishes a `kernel::Effect::LoadDocument`, never a mutation)
//! and are dispatchable only because [`EnergyModelEditor::build_artifact_store_one_item_preparation_factory`]
//! supplies the document lane's one-item retained preparation; `set-simulation-settings` publishes to the
//! config lane through [`EnergyModelEditor::build_config_store_one_item_preparation_factory`].
//!
//! ⏯️ The energy simulation is the `energySimulation` tool's framework `ToolRun`
//! (`📋️tool-run-contract.md`): the framework-reserved `toolRun*` actions and chords start, pause, step,
//! abort and finalize it, and [`EnergyModelEditor::build_tool_run_job`] supplies its run job.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_editor_commands as editor_commands;
extern crate semio_s_artifact_energy_model_editor_config as editor_config;
extern crate semio_s_artifact_energy_model_editor_edit as editor_edit;
extern crate semio_s_artifact_energy_model_editor_interaction as editor_interaction;
extern crate semio_s_artifact_energy_model_editor_panels as editor_panels;
extern crate semio_s_artifact_energy_model_editor_results as editor_results;
extern crate semio_s_artifact_energy_model_examples as examples;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_finish as finish_mutations;
extern crate semio_s_artifact_energy_model_mut_glazing as glazing_mutations;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_run as run_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_session as session;
extern crate semio_s_artifact_energy_model_mut_shading_device as shading_device_mutations;
extern crate semio_s_artifact_energy_model_mut_site as site_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer as window_layer_mutations;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

use crate::editor_commands::{ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID, CREATE_SURFACE_ACTION_ID, CREATE_ZONE_ACTION_ID, DELETE_SURFACE_ACTION_ID, DELETE_ZONE_ACTION_ID, ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS, ENERGY_MODEL_RETAINED_TOOL_IDS, EnergyModelEditorCommand, RENAME_ZONE_ACTION_ID, SET_ACTIVE_EXAMPLE_ACTION_ID, SET_CELL_ACTION_ID, SET_CONSTRUCTION_PROPERTY_ACTION_ID, SET_FENESTRATION_PROPERTY_ACTION_ID, SET_GAS_MATERIAL_PROPERTY_ACTION_ID, SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID, SET_MATERIAL_PROPERTY_ACTION_ID, SET_NODE_ACTION_ID, SET_RUN_PERIOD_ACTION_ID, SET_SITE_ACTION_ID, SET_SURFACE_PROPERTY_ACTION_ID, SET_THERMOSTAT_SETPOINTS_ACTION_ID, SET_ZONE_PROPERTY_ACTION_ID, inspector_action_definitions};
use crate::editor_config::{ChangeResultField, ChangeSimulationSettings, EnergyModelConfig, EnergyModelConfigMutation};
use crate::editor_edit::tools;
use crate::editor_interaction::{EnergyModelInteractionSnapshot, ENERGY_MODEL_INTERACTION_DOMAIN};
use crate::editor_panels::artifact as artifact_panel;
use crate::editor_panels::inspection as inspection_panel;
use crate::editor_edit::windows::{model as model_window, simulation, structure, zones};
use crate::session::EnergySimulationRunJob;
use crate::model::{EntityId, Material, OutsideBoundary, SurfaceClass, Zone};
use crate::schedule::ScheduleId;
use crate::mutations::EnergyModelMutation;
use crate::snapshot::{EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ComponentTree;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::ExampleSource;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::HistoryView;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use semio_framework_2d_compute::EngineHandles;

#[path = "🧮️reduce-and-helpers/🦀️.rs"]
mod reduce_and_helpers;
use reduce_and_helpers::*;

#[path = "🔍️inspector-properties/🦀️.rs"]
mod inspector_properties;
use inspector_properties::*;

#[path = "🎛️retained-commands/🦀️.rs"]
mod retained_commands;
use retained_commands::*;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

#[path = "🖥️editor-impl/🦀️.rs"]
mod editor_impl;
pub use editor_impl::*;

#[path = "📚️examples-and-manifest/🦀️.rs"]
mod examples_and_manifest;
pub use examples_and_manifest::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 👥️Presence
#[path = "👥️presence/🧬️schema/🦀️.rs"]
pub mod presence;
//#endregion 👥️Presence
