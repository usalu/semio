//! 🪜️ Energy engine retirement: the close-turn vocabulary and the bounded close ladders of every engine owner a
//! numerical job retains (results, time series, meters, summaries, sizing, model, config).

extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_output as output;
extern crate semio_s_plugin_energy_engine_results as results;

use crate::kernel::SimulationConfig;
use crate::meters::MeterTable;
use crate::model::Model;
use crate::output::TimeSeriesTable;
use crate::results::{Results, SizingResult, SizingTables, SummaryRow, SummaryTables};
use semio_framework_job::{
    InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🪜️ Upper bound of one ladder rung: a single popped owner or one UTF-8 character of a string.
pub const ENERGY_RUNG_DEMAND: RetirementDemand = RetirementDemand { copy_bytes: 0, capacity_bytes: 0, release_bytes: 4, depth: 1 };

/// 🎟️ Whether one grant covers one quoted demand with at least one item.
pub fn energy_grant_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🛡️ Closes one owner frontier only when the grant covers its own quote, otherwise reports an idle turn.
pub fn energy_guarded(grant: RetainedCloneGrant, demand: Result<RetirementDemand, ValueError>, close: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    if !energy_grant_covers(grant, demand?) {
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    }
    close()
}

/// 🧯️ Maps one owner close turn onto the job close vocabulary and reports whether that owner finished; an owner
/// refusal propagates unchanged.
pub fn energy_turn(step: Result<RetainedCloneStep, ValueError>) -> Result<(InteractiveJobCloseStep, bool), ValueError> {
    Ok(match step? {
        RetainedCloneStep::Progress(progress) => (InteractiveJobCloseStep::Pending { progress }, false),
        RetainedCloneStep::Complete(progress) => (InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }, true),
    })
}

/// 🪜️ One popped ladder owner releasing `released_bytes` of character backing.
pub fn energy_rung(released: (usize, usize)) -> InteractiveJobCloseStep {
    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: released.1, ..Default::default() } }
}

/// 🪜️ One popped ladder owner without backing bytes.
pub fn energy_one() -> InteractiveJobCloseStep {
    energy_rung((1, 0))
}

/// 💤️ A turn the grant could not fund.
pub fn energy_idle() -> InteractiveJobCloseStep {
    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }
}

/// 🏁️ A close ladder with nothing left.
pub fn energy_done() -> InteractiveJobCloseStep {
    InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
}

pub fn close_string_vector_step(owners: &mut Vec<String>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let owner = owners.last_mut()?;
    if !owner.is_empty() {
        if maximum_bytes == 0 {
            return Some((0, 0));
        }
        return owner.pop().map(|character| (0, character.len_utf8()));
    }
    owners.pop();
    Some((1, 0))
}

pub fn close_results_step(results: &mut Results, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_time_series_step(&mut results.time_series, close_string_owner, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_meter_table_step(&mut results.meters, close_string_owner, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_summaries_step(&mut results.summaries, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_sizing_step(&mut results.sizing, maximum_bytes) {
        return Some(released);
    }
    if results.per_surface.pop().is_some() {
        return Some((1, 0));
    }
    if let Some(error) = results.diagnostics.messages.last_mut() {
        if let Some(context) = error.context.as_mut() {
            if let Some(released) = close_string_step(context, maximum_bytes) {
                return Some(released);
            }
            error.context = None;
            return Some((1, 0));
        }
        if let Some(released) = close_string_step(&mut error.message, maximum_bytes) {
            return Some(released);
        }
        results.diagnostics.messages.pop();
        return Some((1, 0));
    }
    if let Some(released) = close_string_step(&mut results.run_metadata.weather_location, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_string_step(&mut results.run_metadata.model_version, maximum_bytes) {
        return Some(released);
    }
    close_string_step(&mut results.run_metadata.model_name, maximum_bytes)
}

pub fn close_time_series_step(table: &mut TimeSeriesTable, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some((_, series)) = table.series.last_mut() {
        if series.values.pop().is_some() || series.timestamps_hours.pop().is_some() {
            return Some((1, 0));
        }
        if let Some(released) = close_string_step(&mut series.key, maximum_bytes) {
            return Some(released);
        }
    }
    if table.series.is_empty() {
        return None;
    }
    if let Some((key, _)) = table.series.pop() {
        *close_string_owner = key;
    }
    Some((1, 0))
}

pub fn close_meter_table_step(table: &mut MeterTable, close_string_owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some((_, meter)) = table.meters.last_mut() {
        if let Some(released) = close_string_step(&mut meter.name, maximum_bytes) {
            return Some(released);
        }
    }
    if table.meters.is_empty() {
        return None;
    }
    if let Some((key, _)) = table.meters.pop() {
        *close_string_owner = key;
    }
    Some((1, 0))
}

pub fn close_summaries_step(summaries: &mut SummaryTables, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_summary_rows_step(&mut summaries.annual_energy, maximum_bytes) {
        return Some(released);
    }
    if let Some((_, rows)) = summaries.monthly_energy.last_mut() {
        if let Some(released) = close_summary_rows_step(rows, maximum_bytes) {
            return Some(released);
        }
    }
    if !summaries.monthly_energy.is_empty() {
        summaries.monthly_energy.pop();
        return Some((1, 0));
    }
    if let Some(released) = close_summary_rows_step(&mut summaries.peak_loads, maximum_bytes) {
        return Some(released);
    }
    close_summary_rows_step(&mut summaries.comfort, maximum_bytes)
}

pub fn close_summary_rows_step(rows: &mut Vec<SummaryRow>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let row = rows.last_mut()?;
    if let Some(released) = close_string_step(&mut row.unit, maximum_bytes) {
        return Some(released);
    }
    if let Some(released) = close_string_step(&mut row.key, maximum_bytes) {
        return Some(released);
    }
    rows.pop();
    Some((1, 0))
}

pub fn close_sizing_step(sizing: &mut SizingTables, maximum_bytes: usize) -> Option<(usize, usize)> {
    if let Some(released) = close_sizing_rows_step(&mut sizing.zone_loads, maximum_bytes) {
        return Some(released);
    }
    close_sizing_rows_step(&mut sizing.equipment, maximum_bytes)
}

pub fn close_sizing_rows_step(rows: &mut Vec<SizingResult>, maximum_bytes: usize) -> Option<(usize, usize)> {
    let row = rows.last_mut()?;
    if let Some(released) = close_string_step(&mut row.component, maximum_bytes) {
        return Some(released);
    }
    rows.pop();
    Some((1, 0))
}

pub fn close_string_step(owner: &mut String, maximum_bytes: usize) -> Option<(usize, usize)> {
    if owner.is_empty() {
        return None;
    }
    if maximum_bytes == 0 {
        return Some((0, 0));
    }
    owner.pop().map(|character| (0, character.len_utf8()))
}

pub fn summaries_are_terminal_empty(summaries: &SummaryTables) -> bool {
    summaries.annual_energy.is_empty() && summaries.monthly_energy.is_empty() && summaries.peak_loads.is_empty() && summaries.comfort.is_empty()
}

pub fn close_model_step(model: &mut Model, maximum_bytes: usize) -> Option<(usize, usize)> {
    macro_rules! close_string {
        ($owner:expr) => {
            if !$owner.is_empty() {
                if maximum_bytes == 0 {
                    return Some((0, 0));
                }
                let bytes = $owner.pop().map_or(0, |character| character.len_utf8());
                return Some((0, bytes));
            }
        };
    }
    macro_rules! close_named {
        ($owners:expr) => {
            if let Some(owner) = $owners.last_mut() {
                close_string!(owner.name);
                $owners.pop();
                return Some((1, 0));
            }
        };
    }
    macro_rules! close_plain {
        ($owners:expr) => {
            if $owners.pop().is_some() {
                return Some((1, 0));
            }
        };
    }
    close_string!(model.name);
    close_string!(model.version);
    close_named!(model.zones);
    close_named!(model.spaces);
    if let Some(surface) = model.surfaces.last_mut() {
        close_plain!(surface.vertices_m);
        close_string!(surface.name);
        model.surfaces.pop();
        return Some((1, 0));
    }
    close_named!(model.fenestrations);
    close_named!(model.materials);
    close_named!(model.glazing_materials);
    close_named!(model.gas_materials);
    if let Some(construction) = model.constructions.last_mut() {
        close_plain!(construction.layer_material_ids);
        close_string!(construction.name);
        model.constructions.pop();
        return Some((1, 0));
    }
    close_plain!(model.people);
    close_plain!(model.lighting);
    close_plain!(model.equipment);
    close_plain!(model.thermostats);
    close_plain!(model.humidistats);
    close_named!(model.setpoint_managers);
    close_plain!(model.ideal_loads);
    close_plain!(model.zone_equipment);
    if let Some(loop_owner) = model.air_loops.last_mut() {
        close_plain!(loop_owner.terminal_zone_ids);
        close_string!(loop_owner.name);
        model.air_loops.pop();
        return Some((1, 0));
    }
    if let Some(loop_owner) = model.plant_loops.last_mut() {
        close_plain!(loop_owner.equipment_ids);
        close_string!(loop_owner.name);
        model.plant_loops.pop();
        return Some((1, 0));
    }
    close_plain!(model.outdoor_air_systems);
    close_plain!(model.infiltrations);
    close_plain!(model.mechanical_ventilations);
    if let Some(shading) = model.shading_surfaces.last_mut() {
        close_plain!(shading.vertices_m);
        close_string!(shading.name);
        model.shading_surfaces.pop();
        return Some((1, 0));
    }
    if let Some(list) = model.space_lists.last_mut() {
        close_plain!(list.space_ids);
        close_string!(list.name);
        model.space_lists.pop();
        return Some((1, 0));
    }
    if let Some(enclosure) = model.thermal_enclosures.last_mut() {
        close_plain!(enclosure.zone_ids);
        close_string!(enclosure.name);
        model.thermal_enclosures.pop();
        return Some((1, 0));
    }
    close_plain!(model.adjacency_pairs);
    if let Some(network) = model.airflow_network.as_mut() {
        close_plain!(network.zone_node_ids);
        close_plain!(network.link_ids);
        model.airflow_network = None;
        return Some((1, 0));
    }
    if let Some(center) = model.electrical_load_centers.last_mut() {
        close_plain!(center.generator_ids);
        close_plain!(center.pv_ids);
        close_plain!(center.battery_ids);
        close_string!(center.name);
        model.electrical_load_centers.pop();
        return Some((1, 0));
    }
    close_plain!(model.pv_systems);
    close_plain!(model.battery_storage);
    close_plain!(model.shw_systems);
    close_plain!(model.solar_thermal_systems);
    close_plain!(model.refrigeration_systems);
    close_plain!(model.water_systems);
    close_plain!(model.faults);
    if let Some(output) = model.output_variables.last_mut() {
        close_string!(output.key);
        close_string!(output.name);
        model.output_variables.pop();
        return Some((1, 0));
    }
    close_plain!(model.sizing_objects);
    close_plain!(model.daylight_zones);
    close_plain!(model.room_air_models);
    None
}

pub fn close_config_step(config: &mut SimulationConfig, maximum_bytes: usize) -> Option<(usize, usize)> {
    macro_rules! close_plain {
        ($owners:expr) => {
            if $owners.pop().is_some() {
                return Some((1, 0));
            }
        };
    }
    if let Some(weather) = config.weather.as_mut() {
        close_plain!(weather.records);
        if !weather.location.is_empty() {
            if maximum_bytes == 0 {
                return Some((0, 0));
            }
            return weather.location.pop().map(|character| (0, character.len_utf8()));
        }
        config.weather = None;
        return Some((1, 0));
    }
    if let Some(schedule) = config.schedules.time_series.last_mut() {
        close_plain!(schedule.values);
        config.schedules.time_series.pop();
        return Some((1, 0));
    }
    if let Some(schedule) = config.schedules.annual.last_mut() {
        close_plain!(schedule.rules);
        close_plain!(schedule.holiday_dates);
        config.schedules.annual.pop();
        return Some((1, 0));
    }
    close_plain!(config.schedules.weekly);
    close_plain!(config.schedules.daily);
    close_plain!(config.schedules.constants);
    None
}

pub fn model_is_terminal_empty(model: &Model) -> bool {
    model.name.is_empty()
        && model.version.is_empty()
        && model.zones.is_empty()
        && model.spaces.is_empty()
        && model.surfaces.is_empty()
        && model.fenestrations.is_empty()
        && model.materials.is_empty()
        && model.glazing_materials.is_empty()
        && model.gas_materials.is_empty()
        && model.constructions.is_empty()
        && model.people.is_empty()
        && model.lighting.is_empty()
        && model.equipment.is_empty()
        && model.thermostats.is_empty()
        && model.humidistats.is_empty()
        && model.setpoint_managers.is_empty()
        && model.ideal_loads.is_empty()
        && model.zone_equipment.is_empty()
        && model.air_loops.is_empty()
        && model.plant_loops.is_empty()
        && model.outdoor_air_systems.is_empty()
        && model.infiltrations.is_empty()
        && model.mechanical_ventilations.is_empty()
        && model.shading_surfaces.is_empty()
        && model.space_lists.is_empty()
        && model.thermal_enclosures.is_empty()
        && model.adjacency_pairs.is_empty()
        && model.airflow_network.is_none()
        && model.electrical_load_centers.is_empty()
        && model.pv_systems.is_empty()
        && model.battery_storage.is_empty()
        && model.shw_systems.is_empty()
        && model.solar_thermal_systems.is_empty()
        && model.refrigeration_systems.is_empty()
        && model.water_systems.is_empty()
        && model.faults.is_empty()
        && model.output_variables.is_empty()
        && model.sizing_objects.is_empty()
        && model.daylight_zones.is_empty()
        && model.room_air_models.is_empty()
}

pub fn config_is_terminal_empty(config: &SimulationConfig) -> bool {
    config.weather.is_none() && config.schedules.constants.is_empty() && config.schedules.daily.is_empty() && config.schedules.weekly.is_empty() && config.schedules.annual.is_empty() && config.schedules.time_series.is_empty()
}

/// 🎟️ A grant that pays exactly one quoted close turn.
pub fn energy_self_funded(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}
