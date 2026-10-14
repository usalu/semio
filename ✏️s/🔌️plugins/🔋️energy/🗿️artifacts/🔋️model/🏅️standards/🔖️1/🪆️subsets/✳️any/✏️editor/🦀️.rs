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
use semio_framework_2d::compute::EngineHandles;



/// 📅️ The run-period field leaves that carry `was` to `now`, ordered so every intermediate period is a calendar interval
/// whenever such an order exists — a field leaf refuses a period whose bounds disagree (`mutation.target-mismatch`), so
/// moving a whole period later sets its end before its start.
fn run_period_steps(was: crate::calendar::RunPeriod, now: crate::calendar::RunPeriod) -> Vec<EnergyModelMutation> {
    let take = |period: &mut crate::calendar::RunPeriod, field: usize| match field {
        0 => period.start_month = now.start_month,
        1 => period.start_day = now.start_day,
        2 => period.end_month = now.end_month,
        3 => period.end_day = now.end_day,
        _ => period.year = now.year,
    };
    let changed: Vec<usize> = (0..5)
        .filter(|field| {
            let mut probe = was;
            take(&mut probe, *field);
            probe != was
        })
        .collect();
    let valid = |order: &Vec<usize>| {
        let mut period = was;
        order.iter().all(|field| {
            take(&mut period, *field);
            period.is_interval()
        })
    };
    let order = orderings(&changed).into_iter().find(valid).unwrap_or(changed);
    order
        .into_iter()
        .map(|field| match field {
            0 => crate::run_mutations::change_run_start_month(now.start_month),
            1 => crate::run_mutations::change_run_start_day(now.start_day),
            2 => crate::run_mutations::change_run_end_month(now.end_month),
            3 => crate::run_mutations::change_run_end_day(now.end_day),
            _ => crate::run_mutations::change_run_year(now.year),
        })
        .collect()
}

/// 🔀️ Every ordering of `items`, the given order first (at most 5! = 120 for the run period's fields).
fn orderings(items: &[usize]) -> Vec<Vec<usize>> {
    if items.len() < 2 {
        return vec![items.to_vec()];
    }
    (0..items.len())
        .flat_map(|index| {
            let rest: Vec<usize> = items.iter().enumerate().filter(|(other, _)| *other != index).map(|(_, item)| *item).collect();
            orderings(&rest).into_iter().map(move |tail| std::iter::once(items[index]).chain(tail).collect::<Vec<usize>>())
        })
        .collect()
}

/// 🚧️ The interzone partner an `OutsideBoundary` carries — the boundary mutation names the tag
/// through `OutsideBoundaryKind` and the partner through this separate optional id, because
/// `dsl::DslScalar` binds unit variants only.
fn interzone_partner(boundary: OutsideBoundary) -> Option<EntityId> {
    match boundary {
        OutsideBoundary::Interzone(partner) => Some(partner),
        _ => None,
    }
}

/// 📂️ The sanctioned whole-document load: a `kernel::Effect::LoadDocument` carrying a genesis
/// pack+spr the host swaps into the live store through `ArtifactStore::reset`, OUTSIDE undo history.
/// This is why `📚️examples` need no `replace-model` kind — whole-document replace has no mutation
/// representative in this artifact's vocabulary at all (`📓️derivation-rules.md` rule 6), exactly as
/// in `📐️cad`'s `reset_document_effect` and `🔱️trinity`'s. A freshly minted envelope has no edits,
/// so its spr encode is infallible.
fn load_document_effect(model: &crate::model::Model) -> semio_framework_plugin::kernel::Effect {
    let snapshot = crate::snapshot::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, model, None);
    let pack = <EnergyModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let envelope = store::os_store::document_materialize::create_document_envelope::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, "model", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners();
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::print_document_spr(&envelope)).expect("energy model document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::kernel::Effect::LoadDocument { pack, spr }
}

/// 🪜️ The one mutation `step` when the edit `differs` from the stored value, none when it restates it — an inspector edit that
/// repeats the current value is a quiet no-op, never a history row.
fn step_if(differs: bool, step: impl FnOnce() -> EnergyModelMutation) -> Vec<EnergyModelMutation> {
    if differs {
        vec![step()]
    } else {
        Vec::new()
    }
}

/// 🆔️ Mints the next free `EntityId` for a collection addressed by its own ids — `max + 1`, so an
/// id is never reused after a delete and every reference in the document stays unambiguous.
fn next_entity_id(existing: impl Iterator<Item = EntityId>) -> EntityId {
    EntityId(existing.map(|id| id.0).max().unwrap_or(0).saturating_add(1))
}

fn surface_class_from_id(id: &str) -> Option<SurfaceClass> {
    Some(match id {
        "exteriorWall" => SurfaceClass::ExteriorWall,
        "interiorWall" => SurfaceClass::InteriorWall,
        "roof" => SurfaceClass::Roof,
        "ceiling" => SurfaceClass::Ceiling,
        "floor" => SurfaceClass::Floor,
        "interzone" => SurfaceClass::Interzone,
        "adiabatic" => SurfaceClass::Adiabatic,
        "ground" => SurfaceClass::Ground,
        _ => return None,
    })
}

fn target_missing(entity: &str, id: u32) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("the energy model has no {entity} with id {id}"))
}

fn target_in_use(entity: &str, id: u32, blocker: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.target-in-use"), format!("{entity} {id} is still referenced by at least one {blocker}"))
}
//#endregion 🧬️MutationSeam

/// 🧩️ The one pure reducer both `ArtifactEditor::handle` and the retained bounded work step run —
/// identical semantics on the interactive path and on the retained path by construction. Each command is read against the live
/// snapshot and answers the concrete semantic mutations that carry it, one per field it changes; nothing is copied or differenced.
fn reduce(command: &EnergyModelEditorCommand, doc: &ArtifactView<'_, EnergyModelSnapshot>) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation>, Fault> {
    let model = &doc.snapshot.model;
    let steps = match command {
        EnergyModelEditorCommand::SetStructureField { field, value } => match field.as_str() {
            "name" => step_if(model.name != *value, || crate::model_mutations::rename_model(value.clone())),
            "version" => step_if(model.version != *value, || crate::model_mutations::change_model_version(value.clone())),
            _ => return Ok(Emit::default()),
        },
        EnergyModelEditorCommand::SetZoneCell { row, column, value } => {
            let Some(zone) = model.zones.get(*row as usize) else { return Ok(Emit::default()) };
            let id = zone.id;
            match column.as_str() {
                "name" => step_if(zone.name != *value, || crate::zone_mutations::rename_zone(id, value.clone())),
                "volumeM3" => match value.parse::<f64>() {
                    Ok(parsed) => step_if(zone.volume_m3 != parsed, || crate::zone_mutations::change_zone_volume(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "multiplier" => match value.parse::<u32>() {
                    Ok(parsed) => step_if(zone.multiplier != parsed, || crate::zone_mutations::change_zone_multiplier(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "conditioned" => match value.parse::<bool>() {
                    Ok(parsed) => step_if(zone.conditioned != parsed, || crate::zone_mutations::change_zone_conditioned(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                "partOfTotalFloorArea" => match value.parse::<bool>() {
                    Ok(parsed) => step_if(zone.part_of_total_floor_area != parsed, || crate::zone_mutations::change_zone_floor_area_participation(id, parsed)),
                    Err(_) => return Ok(Emit::default()),
                },
                _ => return Ok(Emit::default()),
            }
        }
        EnergyModelEditorCommand::CreateZone { name, volume_m3, multiplier, conditioned } => {
            if *volume_m3 <= 0.0 {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "a zone volume must be strictly positive"));
            }
            let id = next_entity_id(model.zones.iter().map(|zone| zone.id));
            vec![crate::zone_mutations::create_zone(id, name.clone(), *volume_m3, (*multiplier).max(1), *conditioned, true, None)]
        }
        EnergyModelEditorCommand::RenameZone { zone, new_name } => {
            let target = model.zones.iter().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            step_if(target.name != *new_name, || crate::zone_mutations::rename_zone(target.id, new_name.clone()))
        }
        EnergyModelEditorCommand::DeleteZone { zone } => {
            let id = EntityId(*zone);
            if !model.zones.iter().any(|entry| entry.id == id) {
                return Err(target_missing("zone", *zone));
            }
            if model.spaces.iter().any(|space| space.zone_id == id) {
                return Err(target_in_use("zone", *zone, "space"));
            }
            if model.surfaces.iter().any(|surface| surface.zone_id == id) {
                return Err(target_in_use("zone", *zone, "surface"));
            }
            if model.thermostats.iter().any(|thermostat| thermostat.zone_id == id) {
                return Err(target_in_use("zone", *zone, "thermostat"));
            }
            vec![crate::zone_mutations::delete_zone(id)]
        }
        EnergyModelEditorCommand::CreateSurface { name, zone, construction, class } => {
            let zone_id = EntityId(*zone);
            let construction_id = EntityId(*construction);
            if !model.zones.iter().any(|entry| entry.id == zone_id) {
                return Err(target_missing("zone", *zone));
            }
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let class = surface_class_from_id(class).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{class}' is not a surface class")))?;
            let id = next_entity_id(model.surfaces.iter().map(|surface| surface.id));
            let vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]];
            vec![crate::surface_mutations::create_surface(id, name.clone(), zone_id, class, vertices, construction_id, OutsideBoundary::OutdoorAir.kind(), None, true, true, 1, None)]
        }
        EnergyModelEditorCommand::DeleteSurface { surface } => {
            let id = EntityId(*surface);
            if !model.surfaces.iter().any(|entry| entry.id == id) {
                return Err(target_missing("surface", *surface));
            }
            vec![crate::surface_mutations::delete_surface(id)]
        }
        EnergyModelEditorCommand::AssignSurfaceConstruction { surface, construction } => {
            let construction_id = EntityId(*construction);
            if !model.constructions.iter().any(|entry| entry.id == construction_id) {
                return Err(target_missing("construction", *construction));
            }
            let target = model.surfaces.iter().find(|entry| entry.id.0 == *surface).ok_or_else(|| target_missing("surface", *surface))?;
            step_if(target.construction_id != construction_id, || crate::surface_mutations::change_surface_construction(target.id, construction_id))
        }
        EnergyModelEditorCommand::SetMaterialProperty { material, property, value } => {
            let target = model.materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("material", *material))?;
            set_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetConstructionProperty { construction, property, value } => set_construction_property(model, *construction, property, value)?,
        EnergyModelEditorCommand::SetSurfaceProperty { surface, property, value, partner_surface } => set_surface_property(model, *surface, property, value, *partner_surface)?,
        EnergyModelEditorCommand::SetFenestrationProperty { fenestration, property, value } => set_fenestration_property(model, *fenestration, property, value)?,
        EnergyModelEditorCommand::SetGlazingMaterialProperty { material, property, value } => {
            let target = model.glazing_materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("glazing material", *material))?;
            set_glazing_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetGasMaterialProperty { material, property, value } => {
            let target = model.gas_materials.iter().find(|entry| entry.id.0 == *material).ok_or_else(|| target_missing("gas material", *material))?;
            set_gas_material_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetZoneProperty { zone, property, value } => {
            let target = model.zones.iter().find(|entry| entry.id.0 == *zone).ok_or_else(|| target_missing("zone", *zone))?;
            set_zone_property(target, property, value)?
        }
        EnergyModelEditorCommand::SetThermostatSetpoints { thermostat, heating_schedule, cooling_schedule, heating_throttle_range_k, cooling_throttle_range_k } => {
            let schedules = &model.schedules;
            if !schedule_exists(schedules, *heating_schedule) {
                return Err(target_missing("schedule", *heating_schedule));
            }
            if !schedule_exists(schedules, *cooling_schedule) {
                return Err(target_missing("schedule", *cooling_schedule));
            }
            let target = model.thermostats.iter().find(|entry| entry.id.0 == *thermostat).ok_or_else(|| target_missing("thermostat", *thermostat))?;
            let id = target.id;
            [
                step_if(target.heating_setpoint_schedule_id != ScheduleId(*heating_schedule), || crate::thermostat_mutations::change_thermostat_heating_setpoint_schedule(id, ScheduleId(*heating_schedule))),
                step_if(target.cooling_setpoint_schedule_id != ScheduleId(*cooling_schedule), || crate::thermostat_mutations::change_thermostat_cooling_setpoint_schedule(id, ScheduleId(*cooling_schedule))),
                step_if(target.heating_throttle_range_k != *heating_throttle_range_k, || crate::thermostat_mutations::change_thermostat_heating_throttle_range(id, *heating_throttle_range_k)),
                step_if(target.cooling_throttle_range_k != *cooling_throttle_range_k, || crate::thermostat_mutations::change_thermostat_cooling_throttle_range(id, *cooling_throttle_range_k)),
            ]
            .concat()
        }
        EnergyModelEditorCommand::SetSite { latitude_deg, longitude_deg, elevation_m, time_zone_hours, north_axis_deg } => {
            if latitude_deg.is_some_and(|value| !(-90.0..=90.0).contains(&value)) || longitude_deg.is_some_and(|value| !(-180.0..=180.0).contains(&value)) {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the site latitude/longitude are outside their SI ranges"));
            }
            let site = &model.site;
            [
                latitude_deg.filter(|value| *value != site.latitude_deg).map(crate::site_mutations::change_site_latitude),
                longitude_deg.filter(|value| *value != site.longitude_deg).map(crate::site_mutations::change_site_longitude),
                elevation_m.filter(|value| *value != site.elevation_m).map(crate::site_mutations::change_site_elevation),
                time_zone_hours.filter(|value| *value != site.time_zone_hours).map(crate::site_mutations::change_site_time_zone),
                north_axis_deg.filter(|value| *value != site.north_axis_deg).map(crate::site_mutations::change_site_north_axis),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        EnergyModelEditorCommand::SetRunPeriod { start_month, start_day, end_month, end_day } => {
            let valid = (1..=12).contains(start_month) && (1..=12).contains(end_month) && (1..=31).contains(start_day) && (1..=31).contains(end_day);
            if !valid {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the run period must address real calendar months and days"));
            }
            let now = crate::calendar::RunPeriod { start_month: *start_month as u8, start_day: *start_day as u8, end_month: *end_month as u8, end_day: *end_day as u8, year: model.run_period.year };
            run_period_steps(model.run_period, now)
        }
        EnergyModelEditorCommand::SetActiveExample { example_id } => {
            let loaded = example_model(example_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("mutation.target-missing"), format!("this artifact bundles no example {example_id:?}")))?;
            return Ok(Emit { effects: vec![load_document_effect(&loaded)], ..Default::default() });
        }
        EnergyModelEditorCommand::SetSimulationSettings { zone_timestep_minutes, system_timestep_minutes, warmup_days } => {
            let settings = ChangeSimulationSettings { zone_timestep_minutes: *zone_timestep_minutes, system_timestep_minutes: *system_timestep_minutes, warmup_days: *warmup_days };
            if !settings.config().is_valid() {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the simulation settings are outside the engine's admissible timestep and warmup ranges"));
            }
            return Ok(Emit { config_mutations: vec![EnergyModelConfigMutation::ChangeSimulationSettings(settings)], ..Default::default() });
        }
        EnergyModelEditorCommand::SetResultField { field } => {
            let Some(selected) = crate::editor_config::ResultField::from_id(field) else {
                return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{field}' is not a published per-surface result field")));
            };
            // 🎨️ Only the 3d model window's body re-renders: the map, the ramp and the legend are all
            // derived inside its own `render`, and nothing else in the editor reads `resultField`.
            return Ok(Emit {
                config_mutations: vec![EnergyModelConfigMutation::ChangeResultField(ChangeResultField { field: selected.id().to_string() })],
                ui_scope: UiDirtyScope::Partial {
                    window_bodies: vec![crate::session::ENERGY_MODEL_3D_WINDOW_KIND_ID.to_string(), simulation::BODY_KEY.to_string()],
                    panel_bodies: Vec::new(),
                    utilities: false,
                    tools: false,
                    engagements: false,
                    measures: false,
                    labels: false,
                },
                ..Default::default()
            });
        }
        // 🎥️ A camera is addressed at ONE window instance, and `reduce` sees no `ViewModel` — both
        // dispatch routes intercept `SetCamera` through `camera_emit` before they ever get here, so
        // reaching this arm means the gesture arrived without its window context.
        EnergyModelEditorCommand::SetCamera { .. } => {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"));
        }
    };
    Ok(Emit { artifact_mutations: steps, ..Default::default() })
}

/// 🎥️ The window-addressed half of the command set: `setCamera` writes the addressed
/// `energy.model.3d` window's own retained pose, never the document and never the app config. Both
/// dispatch routes (`ArtifactEditor::handle` and the retained `energy_model_reduce`) call this FIRST
/// and only fall through to [`reduce`] when it answers `None`, so the two can never diverge.
///
/// 🐢️ `UiDirtyScope::Partial` with nothing listed: the pane that sent the pose already holds it, and
/// republishing the 3d body on every debounced orbit tick would be pure churn. A window that opens
/// later reads the stored pose on its own first render.
fn camera_emit(command: &EnergyModelEditorCommand, view_state: Option<&semio_framework_plugin::ViewModel>) -> Option<Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault>> {
    let EnergyModelEditorCommand::SetCamera { camera } = command else { return None };
    Some((|| {
        let view = view_state.ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("energy.model.3d.window-required"), "a camera change requires a concrete 3d model window"))?;
        if camera.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "setCamera carries no {position,target,zoom} pose"));
        }
        let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(camera, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is not a value: {error}")))?;
        let pose = <model_window::config::EnergyModelCameraPose as semio_framework_value::FromValue>::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("the camera pose is malformed: {error}")))?;
        if !pose.is_valid() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), "the camera pose is not finite, or its zoom is not positive"));
        }
        let mutation = model_window::config::EnergyModelWindowConfigMutation::SetCamera(model_window::config::SetCamera { camera: pose });
        Ok(Emit {
            window_config_mutations: vec![model_window::config::addressed(view, mutation)?],
            ui_scope: UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
            ..Default::default()
        })
    })())
}

/// 🗓️ A thermostat setpoint reference must resolve inside the model's own `ScheduleSet` — the five
/// schedule families are addressed by one shared `ScheduleId` space, so any of them may own it.
fn schedule_exists(schedules: &crate::schedule::ScheduleSet, id: u32) -> bool {
    let id = ScheduleId(id);
    schedules.constants.iter().any(|schedule| schedule.id == id)
        || schedules.daily.iter().any(|schedule| schedule.id == id)
        || schedules.weekly.iter().any(|schedule| schedule.id == id)
        || schedules.annual.iter().any(|schedule| schedule.id == id)
        || schedules.time_series.iter().any(|schedule| schedule.id == id)
}

//#region 🔍️InspectorProperties
/// ⛔️ `'{property}'` is not a field of `{entity}`.
fn unknown_property(entity: &str, property: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{property}' is not a {entity} property"))
}

/// ⛔️ `'{value}'` cannot be read as `{property}`, or lies outside its SI range.
fn invalid_value(property: &str, value: &str) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-payload"), format!("'{value}' is outside the admissible range of property '{property}'"))
}

fn as_f64(property: &str, value: &str, admits: impl Fn(f64) -> bool) -> Result<f64, Fault> {
    let parsed: f64 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !parsed.is_finite() || !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

fn as_u32(property: &str, value: &str, admits: impl Fn(u32) -> bool) -> Result<u32, Fault> {
    let parsed: u32 = value.trim().parse().map_err(|_| invalid_value(property, value))?;
    if !admits(parsed) {
        return Err(invalid_value(property, value));
    }
    Ok(parsed)
}

fn as_bool(property: &str, value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" | "yes" => Ok(true),
        "false" | "0" | "off" | "no" | "" => Ok(false),
        _ => Err(invalid_value(property, value)),
    }
}

/// 🚧️ The wire spelling of an [`OutsideBoundaryKind`] — camelCase, the same vocabulary
/// [`surface_class_from_id`] uses for [`SurfaceClass`].
fn outside_boundary_kind_from_id(id: &str) -> Option<crate::model::OutsideBoundaryKind> {
    use crate::model::OutsideBoundaryKind as Kind;
    Some(match id {
        "outdoorAir" => Kind::OutdoorAir,
        "ground" => Kind::Ground,
        "otherSideTemperature" => Kind::OtherSideTemperature,
        "adiabatic" => Kind::Adiabatic,
        "interzone" => Kind::Interzone,
        _ => return None,
    })
}

/// 🟫️ Every `Surface` field the inspector addresses. `construction`/`boundary` validate their
/// references against the live model FIRST, so a dangling id is a refusal and never a document that
/// points at nothing.
fn set_surface_property(model: &crate::model::Model, surface: u32, property: &str, value: &str, partner_surface: u32) -> Result<Vec<EnergyModelMutation>, Fault> {
    let boundary = match property {
        "boundary" => {
            let kind = outside_boundary_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            // 🚧️ An interzone boundary needs its other half. A control that carries only the kind can
            // name the partner nowhere, so a partner already on the surface is CARRIED FORWARD rather
            // than dropped — re-picking `interzone` on a surface that is already interzone keeps its
            // neighbour instead of refusing.
            let held = model.surfaces.iter().find(|entry| entry.id.0 == surface).and_then(|entry| interzone_partner(entry.outside_boundary_condition));
            let partner = (partner_surface != 0).then_some(EntityId(partner_surface)).or(held);
            if let Some(partner) = partner {
                if !model.surfaces.iter().any(|entry| entry.id == partner) {
                    return Err(target_missing("surface", partner.0));
                }
            }
            Some(OutsideBoundary::from_parts(kind, partner).ok_or_else(|| invalid_value(property, value))?)
        }
        // 🚧️ The one control that can MAKE a surface interzone: `value` is the partner surface, and the
        // boundary becomes `Interzone(partner)` in the same step. `partner_surface` stays the palette's
        // spelling; the inspector's flat `{field, id, value}` shape has only one slot to carry it in.
        "interzonePartner" => {
            let partner = EntityId(as_u32(property, value, |parsed| parsed != 0)?);
            if partner.0 == surface {
                return Err(invalid_value(property, value));
            }
            if !model.surfaces.iter().any(|entry| entry.id == partner) {
                return Err(target_missing("surface", partner.0));
            }
            Some(OutsideBoundary::Interzone(partner))
        }
        _ => None,
    };
    let construction = match property {
        "construction" => {
            let id = EntityId(as_u32(property, value, |_| true)?);
            if !model.constructions.iter().any(|entry| entry.id == id) {
                return Err(target_missing("construction", id.0));
            }
            Some(id)
        }
        _ => None,
    };
    let target = model.surfaces.iter().find(|entry| entry.id.0 == surface).ok_or_else(|| target_missing("surface", surface))?;
    let id = target.id;
    Ok(match property {
        "name" => step_if(target.name != value, || crate::surface_mutations::rename_surface(id, value.to_string())),
        "class" => {
            let class = surface_class_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(target.class != class, || crate::surface_mutations::change_surface_class(id, class))
        }
        "boundary" | "interzonePartner" => {
            let boundary = boundary.expect("the boundary arm parsed its payload above");
            step_if(target.outside_boundary_condition != boundary, || crate::boundary_mutations::change_surface_boundary_condition(id, boundary.kind(), interzone_partner(boundary)))
        }
        "construction" => {
            let construction = construction.expect("the construction arm parsed its payload above");
            step_if(target.construction_id != construction, || crate::surface_mutations::change_surface_construction(id, construction))
        }
        "sunExposed" => {
            let exposed = as_bool(property, value)?;
            step_if(target.sun_exposed != exposed, || crate::boundary_mutations::change_surface_sun_exposed(id, exposed))
        }
        "windExposed" => {
            let exposed = as_bool(property, value)?;
            step_if(target.wind_exposed != exposed, || crate::boundary_mutations::change_surface_wind_exposed(id, exposed))
        }
        "multiplier" => {
            let multiplier = as_u32(property, value, |parsed| parsed >= 1)?;
            step_if(target.multiplier != multiplier, || crate::surface_mutations::change_surface_multiplier(id, multiplier))
        }
        _ => return Err(unknown_property("surface", property)),
    })
}

/// 🪟️ Every `Fenestration` field the inspector addresses — the fifteen scalars plus the optional
/// glazing construction, whose EMPTY value clears the binding rather than naming a construction.
fn set_fenestration_property(model: &crate::model::Model, fenestration: u32, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let glazing = match property {
        "glazingConstruction" => {
            let trimmed = value.trim();
            if trimmed.is_empty() || trimmed == "none" {
                Some(None)
            } else {
                let id = EntityId(as_u32(property, value, |_| true)?);
                if !model.constructions.iter().any(|entry| entry.id == id) {
                    return Err(target_missing("construction", id.0));
                }
                Some(Some(id))
            }
        }
        _ => None,
    };
    let target = model.fenestrations.iter().find(|entry| entry.id.0 == fenestration).ok_or_else(|| target_missing("fenestration", fenestration))?;
    let id = target.id;
    let positive = |parsed: f64| parsed > 0.0;
    let non_negative = |parsed: f64| parsed >= 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(target.name != value, || crate::fenestration_mutations::rename_fenestration(id, value.to_string())),
        "uValueWM2K" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.u_value_w_m2k != parsed, || crate::glazing_mutations::change_fenestration_u_value(id, parsed))
        }
        "shgc" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(target.shgc != parsed, || crate::glazing_mutations::change_fenestration_shgc(id, parsed))
        }
        "vlt" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(target.vlt != parsed, || crate::glazing_mutations::change_fenestration_vlt(id, parsed))
        }
        "areaM2" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.area_m2 != parsed, || crate::fenestration_mutations::change_fenestration_area(id, parsed))
        }
        "heightM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(target.height_m != parsed, || crate::fenestration_mutations::change_fenestration_height(id, parsed))
        }
        "sillHeightM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.sill_height_m != parsed, || crate::fenestration_mutations::change_fenestration_sill_height(id, parsed))
        }
        "frameConductanceWK" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.frame_conductance_w_k != parsed, || crate::glazing_mutations::change_fenestration_frame_conductance(id, parsed))
        }
        "dividerConductanceWK" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.divider_conductance_w_k != parsed, || crate::glazing_mutations::change_fenestration_divider_conductance(id, parsed))
        }
        "overhangDepthM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.overhang_depth_m != parsed, || crate::shading_device_mutations::change_fenestration_overhang_depth(id, parsed))
        }
        "overhangOffsetM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.overhang_offset_m != parsed, || crate::shading_device_mutations::change_fenestration_overhang_offset(id, parsed))
        }
        "finDepthM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.fin_depth_m != parsed, || crate::shading_device_mutations::change_fenestration_fin_depth(id, parsed))
        }
        "finOffsetM" => {
            let parsed = as_f64(property, value, non_negative)?;
            step_if(target.fin_offset_m != parsed, || crate::shading_device_mutations::change_fenestration_fin_offset(id, parsed))
        }
        "glazingConstruction" => {
            let glazing = glazing.expect("the glazing arm parsed its payload above");
            step_if(target.glazing_construction_id != glazing, || match glazing {
                Some(construction) => crate::glazing_mutations::bind_fenestration_glazing_construction(id, construction),
                None => crate::glazing_mutations::clear_fenestration_glazing_construction(id),
            })
        }
        _ => return Err(unknown_property("fenestration", property)),
    })
}

/// 🧊️ The glazing-material fields this artifact's vocabulary can actually name. The seven other
/// optical scalars (`solar_reflectance_*`, `visible_reflectance_*`, `infrared_transmittance`) have no
/// mutation kind, so they are refused here rather than written and then faulted by the probe.
fn set_glazing_material_property(material: &crate::model::GlazingMaterial, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(material.name != value, || crate::window_layer_mutations::rename_glazing_material(id, value.to_string())),
        "thicknessM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.thickness_m != parsed, || crate::window_layer_mutations::change_glazing_material_thickness(id, parsed))
        }
        "conductivityWMK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.conductivity_w_m_k != parsed, || crate::window_layer_mutations::change_glazing_material_conductivity(id, parsed))
        }
        "solarTransmittance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.solar_transmittance != parsed, || crate::window_layer_mutations::change_glazing_material_solar_transmittance(id, parsed))
        }
        "visibleTransmittance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.visible_transmittance != parsed, || crate::window_layer_mutations::change_glazing_material_visible_transmittance(id, parsed))
        }
        "infraredEmissivityFront" => {
            let front = as_f64(property, value, fraction)?;
            step_if(material.infrared_emissivity_front != front, || crate::window_layer_mutations::change_glazing_material_infrared_emissivity(id, front, material.infrared_emissivity_back))
        }
        "infraredEmissivityBack" => {
            let back = as_f64(property, value, fraction)?;
            step_if(material.infrared_emissivity_back != back, || crate::window_layer_mutations::change_glazing_material_infrared_emissivity(id, material.infrared_emissivity_front, back))
        }
        _ => return Err(unknown_property("glazing material", property)),
    })
}

/// 💨️ The gas-gap fields: thickness, fill gas and name.
fn set_gas_material_property(material: &crate::model::GasMaterial, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    Ok(match property {
        "name" => step_if(material.name != value, || crate::window_layer_mutations::rename_gas_material(id, value.to_string())),
        "thicknessM" => {
            let parsed = as_f64(property, value, |parsed| parsed > 0.0)?;
            step_if(material.thickness_m != parsed, || crate::window_layer_mutations::change_gas_material_thickness(id, parsed))
        }
        "gas" => {
            let gas = gas_kind_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(material.gas != gas, || crate::window_layer_mutations::change_gas_material_gas(id, gas))
        }
        _ => return Err(unknown_property("gas material", property)),
    })
}


fn gas_kind_from_id(id: &str) -> Option<crate::model::GasKind> {
    use crate::model::GasKind as Kind;
    Some(match id {
        "air" => Kind::Air,
        "argon" => Kind::Argon,
        "krypton" => Kind::Krypton,
        "xenon" => Kind::Xenon,
        _ => return None,
    })
}


/// 🏘️ Every `Zone` field, addressed by the zone's own `EntityId` rather than by its table row.
fn set_zone_property(zone: &Zone, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = zone.id;
    Ok(match property {
        "name" => step_if(zone.name != value, || crate::zone_mutations::rename_zone(id, value.to_string())),
        "volumeM3" => {
            let parsed = as_f64(property, value, |parsed| parsed > 0.0)?;
            step_if(zone.volume_m3 != parsed, || crate::zone_mutations::change_zone_volume(id, parsed))
        }
        "multiplier" => {
            let parsed = as_u32(property, value, |parsed| parsed >= 1)?;
            step_if(zone.multiplier != parsed, || crate::zone_mutations::change_zone_multiplier(id, parsed))
        }
        "conditioned" => {
            let parsed = as_bool(property, value)?;
            step_if(zone.conditioned != parsed, || crate::zone_mutations::change_zone_conditioned(id, parsed))
        }
        "partOfTotalFloorArea" => {
            let parsed = as_bool(property, value)?;
            step_if(zone.part_of_total_floor_area != parsed, || crate::zone_mutations::change_zone_floor_area_participation(id, parsed))
        }
        _ => return Err(unknown_property("zone", property)),
    })
}
//#endregion 🔍️InspectorProperties

/// 🧱️ The material record's WHOLE addressable surface: its name, its roughness class and its seven
/// SI scalars. `value` is text like every other inspector verb, so one control shape carries a name,
/// an enum spelling and a number alike; the numeric properties parse it and refuse a non-finite or
/// out-of-range reading rather than writing it.
fn set_material_property(material: &Material, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let id = material.id;
    let positive = |parsed: f64| parsed > 0.0;
    let fraction = |parsed: f64| (0.0..=1.0).contains(&parsed);
    Ok(match property {
        "name" => step_if(material.name != value, || crate::material_mutations::rename_material(id, value.to_string())),
        "roughness" => {
            let roughness = surface_roughness_from_id(value.trim()).ok_or_else(|| invalid_value(property, value))?;
            step_if(material.roughness != roughness, || crate::finish_mutations::change_material_roughness(id, roughness))
        }
        "thicknessM" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.thickness_m != parsed, || crate::material_mutations::change_material_thickness(id, parsed))
        }
        "conductivityWMK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.conductivity_w_m_k != parsed, || crate::material_mutations::change_material_conductivity(id, parsed))
        }
        "densityKgM3" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.density_kg_m3 != parsed, || crate::material_mutations::change_material_density(id, parsed))
        }
        "specificHeatJKgK" => {
            let parsed = as_f64(property, value, positive)?;
            step_if(material.specific_heat_j_kg_k != parsed, || crate::material_mutations::change_material_specific_heat(id, parsed))
        }
        "thermalAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.thermal_absorptance != parsed, || crate::finish_mutations::change_material_thermal_absorptance(id, parsed))
        }
        "solarAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.solar_absorptance != parsed, || crate::finish_mutations::change_material_solar_absorptance(id, parsed))
        }
        "visibleAbsorptance" => {
            let parsed = as_f64(property, value, fraction)?;
            step_if(material.visible_absorptance != parsed, || crate::finish_mutations::change_material_visible_absorptance(id, parsed))
        }
        _ => return Err(unknown_property("material", property)),
    })
}

/// 🧱️ One construction field by name, or ONE list edit of its layer stack. The layer verbs carry
/// their operand in `value` because a rendered control merges exactly one scalar there:
/// `addLayer` = the material id to append, `removeLayer`/`moveLayerUp`/`moveLayerDown` = the layer
/// index, `replaceLayer:<index>` = the material id that takes that slot (the un-suffixed spelling
/// takes `<index>:<materialId>`, which is what a palette invocation types).
///
/// ⚠️ `add-construction-layer` admits an OPAQUE `Material` only — its own diff refuses a glazing or
/// gas id with `mutation.target-missing`. So an add/replace naming one is refused HERE, loudly and
/// early, instead of reducing cleanly and dying at the store.
fn set_construction_property(model: &crate::model::Model, construction: u32, property: &str, value: &str) -> Result<Vec<EnergyModelMutation>, Fault> {
    let opaque: Vec<EntityId> = model.materials.iter().map(|material| material.id).collect();
    let target = model.constructions.iter().find(|entry| entry.id.0 == construction).ok_or_else(|| target_missing("construction", construction))?;
    let (id, layers) = (target.id, target.layer_material_ids.as_slice());
    let layer_material = |raw: &str| -> Result<EntityId, Fault> {
        let id = EntityId(as_u32(property, raw, |_| true)?);
        if !opaque.contains(&id) {
            return Err(Fault::new(
                FaultOrigin::App,
                FaultCode::new("app.command.invalid-payload"),
                format!("layer material {} is not an opaque material — 'add-construction-layer' declares no glazing or gas layer", id.0),
            ));
        }
        Ok(id)
    };
    let slot = |raw: &str, layers: &[EntityId]| -> Result<usize, Fault> {
        let at = as_u32(property, raw, |_| true)? as usize;
        if at >= layers.len() {
            return Err(invalid_value(property, raw));
        }
        Ok(at)
    };
    Ok(match property {
        "name" => step_if(target.name != value, || crate::construction_mutations::rename_construction(id, value.to_string())),
        "addLayer" => {
            let material = layer_material(value.trim())?;
            vec![crate::construction_mutations::add_construction_layer(id, layers.len() as u32, material)]
        }
        "removeLayer" => {
            let at = slot(value.trim(), layers)?;
            vec![crate::construction_mutations::remove_construction_layer(id, at as u32)]
        }
        "moveLayerUp" | "moveLayerDown" => {
            let at = slot(value.trim(), layers)?;
            let other = if property == "moveLayerUp" { at.checked_sub(1) } else { at.checked_add(1) };
            let Some(other) = other.filter(|other| *other < layers.len()) else { return Err(invalid_value(property, value)) };
            let mut swapped = layers.to_vec();
            swapped.swap(at, other);
            step_if(swapped != layers, || crate::construction_mutations::reorder_construction_layers(id, swapped.clone()))
        }
        _ if property == "replaceLayer" || property.starts_with("replaceLayer:") => {
            let (raw_index, raw_material) = match property.strip_prefix("replaceLayer:") {
                Some(at) => (at, value.trim()),
                None => value.trim().split_once(':').ok_or_else(|| invalid_value(property, value))?,
            };
            let at = slot(raw_index.trim(), layers)?;
            let material = layer_material(raw_material.trim())?;
            if layers[at] == material {
                Vec::new()
            } else {
                vec![crate::construction_mutations::remove_construction_layer(id, at as u32), crate::construction_mutations::add_construction_layer(id, at as u32, material)]
            }
        }
        _ => return Err(unknown_property("construction", property)),
    })
}


fn surface_roughness_from_id(id: &str) -> Option<crate::model::SurfaceRoughness> {
    use crate::model::SurfaceRoughness as Kind;
    Some(match id {
        "veryRough" => Kind::VeryRough,
        "rough" => Kind::Rough,
        "mediumRough" => Kind::MediumRough,
        "mediumSmooth" => Kind::MediumSmooth,
        "smooth" => Kind::Smooth,
        "verySmooth" => Kind::VerySmooth,
        _ => return None,
    })
}


//#endregion 🔖️Reduce

//#region 🧵️RetainedCommands
const ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA: &str = "energy.model.tool-command.v1";
const ENERGY_MODEL_RETAINED_RAW_BYTES: usize = 65_536;
/// 🧮️ One retained work item covers one whole-model edit; the extent guard below refuses a document
/// whose addressable entity count would exceed the bounded first step.
const ENERGY_MODEL_RETAINED_WORK_ITEMS: usize = 65_536;

fn energy_model_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_RETAINED_RAW_BYTES, ENERGY_MODEL_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Refuses a first step whose document is larger than the declared retained extent, so an
/// oversized model is rejected at admission rather than truncated mid-edit.
fn energy_model_extent(command: &EnergyModelEditorCommand, snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&command.action_id()) {
        return None;
    }
    let model = &snapshot.model;
    let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
    (items <= ENERGY_MODEL_RETAINED_WORK_ITEMS).then_some(1)
}

#[allow(clippy::needless_pass_by_value)]
fn energy_model_reduce(
    command: &EnergyModelEditorCommand,
    snapshot: &EnergyModelSnapshot,
    _config: &EnergyModelConfig,
    history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EnergyModelEditor>>>,
    operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, EnergyModelConfigMutation, NoDraftMutation>, Fault> {
    if let Some(camera) = camera_emit(command, _context.and_then(|context| context.view_state.as_ref())) {
        return camera;
    }
    reduce(command, &ArtifactView::with_operation(snapshot, history, operation.clone()))
}

struct EnergyModelCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl EnergyModelCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ENERGY_MODEL_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for EnergyModelCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<EnergyModelEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<EnergyModelEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ENERGY_MODEL_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        energy_model_contract()
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
        if input.declared_bytes() > ENERGY_MODEL_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("the bounded energy model command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for EnergyModelCommandJobFactory {
    type Owner = EditorApp<EnergyModelEditor>;
    const TOOL_IDS: &'static [&'static str] = ENERGY_MODEL_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: SET_NODE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CELL_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: RENAME_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_ZONE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: CREATE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: DELETE_SURFACE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: ASSIGN_SURFACE_CONSTRUCTION_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_THERMOSTAT_SETPOINTS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_SITE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_RUN_PERIOD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ACTIVE_EXAMPLE_ACTION_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_SETTINGS_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: simulation::SET_RESULT_FIELD_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Config] },
        // 🎥️ The camera writes ONE window instance's own retained state — never the document, never
        // the app config.
        ArtifactToolPublicationContract { tool_id: model_window::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: SET_SURFACE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_FENESTRATION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_ZONE_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_GAS_MATERIAL_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: SET_CONSTRUCTION_PROPERTY_ACTION_ID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
/// 📬️ The document lane's one-item retained preparation. Without it every verb declaring
/// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract and
/// stays dispatch-dead, whatever its classification.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct EnergyModelStorePreparationFactory;

struct EnergyModelStorePreparation {
    owners: store::OneItemOwners<EnergyModelSnapshot, EnergyModelMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<EnergyModelMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<EnergyModelMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &EnergyModelMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("the energy model store preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &EnergyModelMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "energy-model-artifact-lane"));
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<EnergyModelStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation, EnergyModelMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<EnergyModelSnapshot, EnergyModelMutation, EnergyModelMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        let model = &request.base.get().model;
        let items = model.zones.len().saturating_add(model.spaces.len()).saturating_add(model.surfaces.len()).saturating_add(model.fenestrations.len()).saturating_add(model.materials.len()).saturating_add(model.constructions.len());
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || items > ENERGY_MODEL_RETAINED_WORK_ITEMS
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "energy-model-artifact-request-refused"), request));
        }
        Ok((Box::new(EnergyModelStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<EnergyModelSnapshot, EnergyModelMutation> for EnergyModelStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        let fault = |message: &'static str| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() || self.owners.failure.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress::default()));
        }
        let authority = self.owners.authority.as_ref().ok_or_else(|| fault("energy-model-artifact-authority-missing"))?;
        let base = self.owners.base.as_ref().ok_or_else(|| fault("energy-model-artifact-base-owner-missing"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| fault("energy-model-artifact-mutation-owner-missing"))?;
        let inverse = match mutation.inverse(base.get()) {
            Ok(inverse) => inverse,
            Err(error) => {
                *self.owners.mutation = Some(mutation);
                return Err(error);
            }
        };
        let post = match protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()) {
            Ok(post) => post,
            Err(_) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.inverse = Some(inverse);
                *self.owners.failure = Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retained the original mutation application refusal"));
            }
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<EnergyModelSnapshot, EnergyModelMutation>> {
        self.owners.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.owners.begin_close();
    }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation


//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct EnergyModelEditor;

impl ArtifactEditor for EnergyModelEditor {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<ExampleSource> {
        examples()
    }
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = EnergyModelConfig;
    type ConfigMutation = EnergyModelConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelEditorCommand;
    /// 🧩️ `structure`/`zones` are `s.stdio.semio@v1/{value,table}` members, so the roster that
    /// resolves their genesis dialects is stdio's closed `SemioMembers` (same as gis/sourcing).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        crate::snapshot::energy_child_restore_projection(snapshot)
    }

    /// 🌱️ Both composed children derive from the model — see `crate::energy_genesis_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::snapshot::energy_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<EnergyModelEditor>,
        owner_file: "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.energy.model@1/*#editor",
        artifact_schema: "energy.model",
        factory: "EnergyModelCommandJobFactory",
        factory_type: EnergyModelCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 65_536, 1, 262_144, 7_500),
        tools: [
            "set-node",
            "set-cell",
            "create-zone",
            "rename-zone",
            "delete-zone",
            "create-surface",
            "delete-surface",
            "assign-surface-construction",
            "set-material-property",
            "set-thermostat-setpoints",
            "set-site",
            "set-run-period",
            "setActiveExample",
            "set-simulation-settings",
            "set-result-field",
            "set-surface-property",
            "set-fenestration-property",
            "set-zone-property",
            "set-glazing-material-property",
            "set-gas-material-property",
            "setCamera",
            "set-construction-property"
        ]
    }

    /// 🎥️ The 3d window's own retained orbit pose — one bounded window-config store, keyed by window
    /// INSTANCE, so two open 3d panes never share a camera.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::EnergyModelWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EnergyModelCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolOperationSpec>, Fault> {
        if !ENERGY_MODEL_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.action_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "the energy model command does not match its exact registered tool"));
        }
        if energy_model_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("energy.model.retained.extent"), "the energy model bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.action_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, energy_model_reduce, energy_model_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            EnergyModelEditorCommand::action_id,
            ENERGY_MODEL_RETAINED_RAW_BYTES,
            1,
            work,
        );
        Ok(Some(semio_framework_plugin::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }


    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }


    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(EnergyModelStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    /// ⏯️ The `energySimulation` run job over the run's base snapshot and the config store's settings.
    /// The run is read-only and declares no `revalidateJob`, so only the `Run` purpose builds a job.
    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        if request.tool_id != tools::simulation::TOOL_ID || request.purpose != ToolRunJobPurpose::Run {
            return Ok(None);
        }
        Ok(Some(Box::new(EnergySimulationRunJob::new(request.identity, request.snapshot, request.config.simulation_template()))))
    }

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    fn command_id(command: &EnergyModelEditorCommand) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        crate::editor_commands::args_bridge::command_from_action(action, args)
    }

    /// ✏️ Delegates to the same pure [`reduce`] the retained bounded work step runs, so the
    /// interactive and retained routes can never diverge.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(camera) = camera_emit(command, _view_state) {
            return camera;
        }
        reduce(command, doc)
    }

    /// 🎨️ The plain render path: no request context, so no live interaction domain to read. The 3d
    /// model window still renders — it simply paints nothing as selected or hovered.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::default())
    }

    /// 🕹️ The request-context render: resolves the framework-owned `energyModel` domain ONCE and
    /// threads its selection/hover into the whole body, so the 3d window's paint and (later) the
    /// inspector read one authority instead of either surface keeping selection of its own.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        render_body(body_key, doc, cfg, view_state, &EnergyModelInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }
}

/// 🎨️ The one body dispatch both render entry points share, so the plain and the request-context
/// routes can never diverge.
fn render_body(
    body_key: &str,
    doc: &ArtifactView<'_, EnergyModelSnapshot>,
    cfg: &ConfigView<'_, EnergyModelConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    interaction: &EnergyModelInteractionSnapshot,
) -> UiAssemblyResult<ComponentTree> {
    let node = match body_key {
        structure::BODY_KEY => structure::render(doc.snapshot)?,
        zones::BODY_KEY => zones::render(doc.snapshot)?,
        simulation::BODY_KEY => simulation::render(doc.tool_run(), cfg.snapshot, &doc.snapshot.model, view_state.locale),
        model_window::BODY_KEY => {
            // 🎨️ Results mode: while a finished (or ticking) energy simulation run carries a per-surface
            // payload, its chosen field paints the surfaces and the legend rides as the caption.
            let energy = crate::editor_edit::tools::simulation::surface_energy_from_run(doc.tool_run());
            let field = crate::editor_config::result_field(cfg.snapshot);
            let painted = energy.as_ref().map(|map| crate::editor_results::surface_colors(map, field));
            let caption = painted.as_ref().map(|(_, min, max)| crate::editor_results::legend_caption(field, *min, *max));
            // 🎥️ `config::current` is the addressed window's retained orbit pose, `None` until it has
            // been moved — an unmoved window keeps the model-derived camera and `fit_json`'s framing.
            // 🎨️ The same `(min, max)` the ramp was normalized over also labels the legend strip's two
            // ends, so the caption line and the swatch strip can never round differently.
            model_window::render_with_legend(
                &crate::snapshot::energy_model(doc.snapshot),
                interaction,
                painted.as_ref().map(|(colors, _, _)| colors),
                caption.as_deref(),
                painted.as_ref().map(|(_, min, max)| (*min, *max)),
                model_window::config::current(cfg),
            )?
        }
        // 🪟️ The host's open/scroll state for THIS body, read once per render — every container of
        // the outliner materialises exactly the slice it names and stamps its own full `total`.
        artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, interaction, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact_panel::BODY_KEY))?,
        // 🎨️ `cfg` reaches the inspector because its Results section renders the CURRENT colour field
        // and binds `set-result-field` — the one control lane D's selector had nowhere to live.
        inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, interaction, cfg.snapshot, view_state.locale, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection_panel::BODY_KEY))?,
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.render", "the unknown-body label could not be assembled"))?,
    };
    Ok(semio_framework_plugin::built_to_component_tree(node))
}
//#endregion 🔖️Editor

//#region 📚️Examples
/// 📚️ The bundled `📚️examples/🎬️<slug>` fixtures, table-driven — a new example is ONE row here plus
/// its `#[path]` mount in `📦️packages/🦀️rust/🦀️.rs`. Every row names a module that exports the same
/// three items (`ID`, `LABEL_EN`, `model()`), so the table cannot go out of step with a leaf.
/// The ANSI/ASHRAE 140 case ids are proper nouns and read identically in both authored languages;
/// `Demo` likewise. That is a real translation decision, not a missing one.
fn example_rows() -> Vec<(&'static str, &'static str, crate::model::Model)> {
    use crate::examples::{bestest_600, bestest_600ff, bestest_610, bestest_620, bestest_630, bestest_640, bestest_650, bestest_900, bestest_900ff, bestest_910, bestest_920, bestest_930, bestest_940, bestest_950, demo};
    vec![
        (demo::ID, demo::LABEL_EN, demo::model()),
        (bestest_600::ID, bestest_600::LABEL_EN, bestest_600::model()),
        (bestest_600ff::ID, bestest_600ff::LABEL_EN, bestest_600ff::model()),
        (bestest_610::ID, bestest_610::LABEL_EN, bestest_610::model()),
        (bestest_620::ID, bestest_620::LABEL_EN, bestest_620::model()),
        (bestest_630::ID, bestest_630::LABEL_EN, bestest_630::model()),
        (bestest_640::ID, bestest_640::LABEL_EN, bestest_640::model()),
        (bestest_650::ID, bestest_650::LABEL_EN, bestest_650::model()),
        (bestest_900::ID, bestest_900::LABEL_EN, bestest_900::model()),
        (bestest_900ff::ID, bestest_900ff::LABEL_EN, bestest_900ff::model()),
        (bestest_910::ID, bestest_910::LABEL_EN, bestest_910::model()),
        (bestest_920::ID, bestest_920::LABEL_EN, bestest_920::model()),
        (bestest_930::ID, bestest_930::LABEL_EN, bestest_930::model()),
        (bestest_940::ID, bestest_940::LABEL_EN, bestest_940::model()),
        (bestest_950::ID, bestest_950::LABEL_EN, bestest_950::model()),
    ]
}

/// 📚️ Catalogue `EnergyModelEditor::examples` returns. `.editor` stamps it onto the manifest,
/// and the navbar dropdown reads that list.
pub fn examples() -> Vec<ExampleSource> {
    example_rows()
        .into_iter()
        .map(|(id, label, model)| {
            let snapshot = crate::snapshot::energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
            ExampleSource::new(id, LocalizedLabel::native(label, label), semio_framework_pack_json::to_json_string(&snapshot), "file")
        })
        .collect()
}

/// 📚️ The model behind one bundled example id — the read `setActiveExample` reduces against.
fn example_model(example_id: &str) -> Option<crate::model::Model> {
    example_rows().into_iter().find(|(id, _, _)| *id == example_id).map(|(_, _, model)| model)
}

/// 📚️ The picker's option list, in `example_rows` order.
fn example_options() -> Vec<semio_framework_plugin::ActionArgOption> {
    example_rows().into_iter().map(|(id, label, _)| semio_framework_plugin::ActionArgOption { value: id.to_string(), label: LocalizedLabel::native(label, label) }).collect()
}
//#endregion 📚️Examples


//#region 🔍️InspectorActions


/// 🎬️ Every action this editor declares APP-level, in one roster: `build_definition` copies each of
/// them onto every window kind, which is what makes a panel control and a keybinding dispatchable
/// whatever window is active. `setActiveExample` is app-level too, through `.mutation(…)`.
pub fn app_level_action_definitions() -> Vec<semio_framework_plugin::ActionDefinition> {
    let mut actions = inspector_action_definitions();
    actions.extend(crate::editor_edit::window_shared_action_definitions());
    actions
}
//#endregion 🔍️InspectorActions

//#region 🔖️Manifest
/// 🧱️ The editor's `AppDefinition`. Every id in [`ENERGY_MODEL_RETAINED_TOOL_IDS`] is classified
/// `Migrated` here — `set-node`/`set-cell` are already stamped by their kits, the other twelve are
/// classified explicitly, and `EditorBuilder::try_build_definition` panics on any `Unclassified` id.
/// The `energySimulation` tool declares its run, so the framework injects the reserved `toolRun*`
/// actions and binds their chords; the editor binds none of its own for the simulation.
/// Each window owns its own action list (`structure::actions`/`zones::actions`/the simulation
/// window's `definition`), so this function never restates one.
pub fn create_energy_model_editor() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .artifact_kind(crate::snapshot::artifact_kind())
        .terminology("reuse")
        .terminology_document("reuse", ["Entwerfen mit Bestand", "Energie"])
        .icon_id("battery")
        .mode_def(edit::definition())
        .default_mode_id(edit::ENERGY_MODEL_EDIT_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .window_kind_def(model_window::definition())
        // 🕹️ The one framework-owned selection/hover domain the tree panel, the inspector and the 3d
        // model window share (`✏️editor/🕹️interaction/🦀️.rs`).
        .interaction(crate::editor_interaction::energy_model_interaction_definition())
        // 🧊️ Binding the 3d window to that domain is what lets the react `World3dHost` dispatch the
        // framework-reserved `interactionSelect`/`interactionHover` on its own pick — no plugin
        // pointer command exists, and none is needed.
        .window_kind_interactions(model_window::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(ENERGY_MODEL_INTERACTION_DOMAIN)])
        // 📌️ The two dock panels: the artifact tree marks and picks into that same domain, the
        // inspector edits whatever it resolves to.
        .panel_tab_def(artifact_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        .tool(tools::simulation::definition())
        .mutation(SET_ACTIVE_EXAMPLE_ACTION_ID, LocalizedLabel::native("Load example", "Beispiel laden"))
        .action_args(SET_ACTIVE_EXAMPLE_ACTION_ID, vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example_options()).required()])
        .keybinding("mod+shift+n", CREATE_ZONE_ACTION_ID)
        .keybinding("mod+shift+s", CREATE_SURFACE_ACTION_ID)
        .keybinding("mod+shift+g", SET_SITE_ACTION_ID)
        .default_layout(edit::layout());
    // 🔍️ App-level, NOT per-window: `build_definition` copies these onto every window kind, which is
    // what makes an inspector control dispatchable while the 3d window (or any other) is active.
    for action in app_level_action_definitions() {
        builder = builder.action_with(action);
    }
    for tool_id in ENERGY_MODEL_RETAINED_TOOL_IDS {
        builder = builder.action_interactive_job(*tool_id, InteractiveJobClassification::Migrated);
    }
    // ⚠️ Declared after every action is on the builder: `action_destructive` rewrites an
    // already-declared id, and the two inspector deletes only arrive with the loop above.
    for destructive_id in ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS {
        builder = builder.action_destructive(*destructive_id);
    }
    builder = builder.action_describe("assign-surface-construction", LocalizedLabel::native("Assigns an existing construction (its layer build-up) to one surface of the energy model by id.", "Weist einer Fläche des Energiemodells anhand ihrer Id eine vorhandene Konstruktion (ihren Schichtaufbau) zu."));
    builder = builder.action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole energy model with a bundled example, the demo or one of the ASHRAE 140 BESTEST cases (600 to 950), by example id.", "Ersetzt das gesamte Energiemodell durch ein mitgeliefertes Beispiel, die Demo oder einen der ASHRAE-140-BESTEST-Fälle (600 bis 950), anhand der Beispiel-Id."));
    builder = builder.action_describe("set-surface-property", LocalizedLabel::native("Sets one property of one surface: name, class, construction, outside boundary (outdoor air, ground, other-side temperature, adiabatic, or interzone with its partner surface), sun or wind exposure, or multiplier.", "Setzt eine Eigenschaft einer Fläche: Name, Klasse, Konstruktion, äußere Randbedingung (Außenluft, Erdreich, Temperatur der Gegenseite, adiabat oder zonenübergreifend mit Partnerfläche), Sonnen- oder Windexposition oder Multiplikator."));
    builder = builder.action_describe("set-fenestration-property", LocalizedLabel::native("Sets one property of one window: name, glazing construction, solar heat gain coefficient, visible transmittance, height, sill height, frame or divider conductance, or overhang and fin dimensions.", "Setzt eine Eigenschaft eines Fensters: Name, Verglasungskonstruktion, Gesamtenergiedurchlassgrad, Lichttransmission, Höhe, Brüstungshöhe, Rahmen- oder Sprossenleitwert oder Maße von Überstand und Seitenblende."));
    builder = builder.action_describe("set-zone-property", LocalizedLabel::native("Sets one property of one thermal zone: its name, multiplier, whether it is conditioned, or whether it counts towards the total floor area.", "Setzt eine Eigenschaft einer thermischen Zone: Name, Multiplikator, ob sie konditioniert ist oder ob sie zur Gesamtnutzfläche zählt."));
    builder = builder.action_describe("set-glazing-material-property", LocalizedLabel::native("Sets one property of one glazing material: name, thickness, conductivity, solar or visible transmittance, or front or back infrared emissivity.", "Setzt eine Eigenschaft eines Verglasungsmaterials: Name, Dicke, Wärmeleitfähigkeit, solaren oder sichtbaren Transmissionsgrad oder vorderen oder hinteren Infrarot-Emissionsgrad."));
    builder = builder.action_describe("set-gas-material-property", LocalizedLabel::native("Sets one property of one gas gap between glazing layers: its name, thickness, or gas (air, argon, krypton or xenon).", "Setzt eine Eigenschaft eines Gaszwischenraums zwischen Verglasungsschichten: Name, Dicke oder Gas (Luft, Argon, Krypton oder Xenon)."));
    builder = builder.action_describe("set-material-property", LocalizedLabel::native("Sets one property of one opaque material: name, roughness, thickness, conductivity, specific heat, or thermal, solar or visible absorptance.", "Setzt eine Eigenschaft eines opaken Materials: Name, Rauigkeit, Dicke, Wärmeleitfähigkeit, spezifische Wärmekapazität oder thermischen, solaren oder sichtbaren Absorptionsgrad."));
    builder = builder.action_describe("set-thermostat-setpoints", LocalizedLabel::native("Sets the heating and cooling setpoint schedules of one thermostat, with optional throttling ranges in kelvin.", "Legt die Heiz- und Kühlsollwert-Zeitpläne eines Thermostats fest, mit optionalen Regelbereichen in Kelvin."));
    builder = builder.action_describe("set-site", LocalizedLabel::native("Sets the building site: latitude, longitude, elevation, time zone and the angle of the building's north axis.", "Legt den Standort des Gebäudes fest: Breite, Länge, Höhe, Zeitzone und den Winkel der Nordachse des Gebäudes."));
    builder = builder.action_describe("delete-zone", LocalizedLabel::native("Deletes one thermal zone by id; refused while any surface, space, gain, HVAC object or airflow node still refers to it.", "Löscht eine thermische Zone anhand ihrer Id; wird abgelehnt, solange noch eine Fläche, ein Raum, eine Last, ein HLK-Objekt oder ein Luftknoten auf sie verweist."));
    builder = builder.action_describe("delete-surface", LocalizedLabel::native("Deletes one surface by id together with every window hosted on it and every adjacency naming it; refused while another surface uses it as its interzone partner.", "Löscht eine Fläche anhand ihrer Id samt aller darin sitzenden Fenster und aller sie nennenden Nachbarschaften; wird abgelehnt, solange eine andere Fläche sie als Partnerfläche nutzt."));
    builder = builder.action_describe("set-construction-property", LocalizedLabel::native("Renames one construction or edits its layer build-up: adds or removes a material layer, or moves a layer up or down.", "Benennt eine Konstruktion um oder bearbeitet ihren Schichtaufbau: fügt eine Materialschicht hinzu, entfernt sie oder verschiebt eine Schicht nach oben oder unten."));
    builder = builder.action_describe("set-simulation-settings", LocalizedLabel::native("Sets the simulation's zone and system timesteps in minutes and the number of warm-up days; a running simulation restarts with them.", "Legt die Zonen- und Systemzeitschritte der Simulation in Minuten und die Zahl der Vorlauftage fest; eine laufende Simulation startet damit neu."));
    builder = builder.action_describe("set-result-field", LocalizedLabel::native("Chooses the simulated per-surface result (conduction loss or gain, transmitted or absorbed solar) the 3D model window is coloured by; a running simulation is not restarted.", "Wählt das simulierte Flächenergebnis (Transmissionsverlust oder -gewinn, transmittierte oder absorbierte Solarstrahlung), nach dem das 3D-Modellfenster eingefärbt wird; eine laufende Simulation startet nicht neu."));
    builder = builder.action_describe("set-run-period", LocalizedLabel::native("Sets the simulated calendar period from a start month and day to an end month and day.", "Legt den simulierten Kalenderzeitraum von einem Startmonat und -tag bis zu einem Endmonat und -tag fest."));
    builder = builder.action_describe("create-surface", LocalizedLabel::native("Adds a new surface with the given name and class to an existing zone, built from an existing construction.", "Fügt einer vorhandenen Zone eine neue Fläche mit dem angegebenen Namen und der Klasse hinzu, aufgebaut aus einer vorhandenen Konstruktion."));
    builder = builder.action_describe("create-zone", LocalizedLabel::native("Adds a new thermal zone with the given name, volume in cubic metres, multiplier and conditioning.", "Fügt eine neue thermische Zone mit dem angegebenen Namen, Volumen in Kubikmetern, Multiplikator und Konditionierung hinzu."));
    builder = builder.action_describe("rename-zone", LocalizedLabel::native("Renames one thermal zone; zone names key every simulation report, so a name already in use is refused.", "Benennt eine thermische Zone um; Zonennamen sind der Schlüssel aller Simulationsberichte, ein bereits vergebener Name wird abgelehnt."));
    builder = builder.action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome);
    builder.build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 👥️Presence
#[path = "👥️presence/🧬️schema/🦀️.rs"]
pub mod presence;
//#endregion 👥️Presence
