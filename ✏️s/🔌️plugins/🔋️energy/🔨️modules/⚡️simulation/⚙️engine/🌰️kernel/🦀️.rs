//! 🔄️ Simulation kernel: the hourly timestep cursor machine and its coupled surface/zone heat
//! balance.
//!
//! Each hour first resolves every zone's schedules, gains, infiltration and setpoints, then advances
//! the building through `3600 s / Δt` implicit heat-balance steps. One step of one zone solves the
//! EnergyPlus heat-balance method simultaneously rather than by lagged iteration: every construction
//! is an implicit finite-difference chain reduced to its inside face, every window a massless
//! glazing chain, the inside faces exchange long wave through the enclosure's gray-body factors, and
//! the zone air stores heat by the third-order backward difference. The thermostat pins the air at
//! its active setpoint and the ideal loads deliver exactly the heat that keeps it there.

extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_coils as coils;
extern crate semio_s_plugin_energy_engine_controls as controls;
extern crate semio_s_plugin_energy_engine_curves as curves;
extern crate semio_s_plugin_energy_engine_daylight as daylight;
extern crate semio_s_plugin_energy_engine_electrical as electrical;
extern crate semio_s_plugin_energy_engine_envelope as envelope;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_fans as fans;
extern crate semio_s_plugin_energy_engine_fenestration as fenestration;
extern crate semio_s_plugin_energy_engine_gains as gains;
extern crate semio_s_plugin_energy_engine_ideal_hvac as ideal_hvac;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_num as num;
extern crate semio_s_plugin_energy_engine_plant as plant;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_props as props;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_solar as solar;
extern crate semio_s_plugin_energy_engine_units as units;
extern crate semio_s_plugin_energy_engine_zone_air as zone_air;
extern crate semio_s_plugin_energy_engine_zone_hvac as zone_hvac;

use crate::air_exchange::{infiltration_flow_m3_s, InfiltrationSpec};
use crate::calendar::{RunPeriod, SimDate};
use crate::controls::{HumidistatSpec, ThermostatSpec};
use crate::curves::PerformanceCurve;
use crate::daylight::{dimmed_lighting_power_w, lighting_dimming_fraction, reference_point_illuminance_lux, simplified_daylight_factor};
use crate::electrical::{grid_balance, PvSystem, Transformer};
use crate::envelope::{eliminate_chain, exterior_convection_w_m2k, exterior_radiation_w_m2k, interior_convection_w_m2k, is_windward, substitute_chain, wind_speed_at_height};
use crate::error::Error;
use crate::fenestration::{gap_conductance_w_m2k, glazing_interior_convection_w_m2k};
use crate::gains::{compute_equipment_gain_w, compute_lighting_gain_w, compute_people_gain_w, ActivityLevel, GainDecomposition};
use crate::ideal_hvac::{ideal_loads_deliver, IdealLoadsConfig, IdealLoadsInput};
use crate::model::{EntityId, FixedTable, Model, OutsideBoundary, MAX_PANES};
use semio_framework_math_dense::{DenseSolveBudget,DenseSolveCursor,DenseSolveDemand,DenseSolveFault,DenseSolveStatus,DenseSolveTurn};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use crate::plant::{PlantLoopSimulation, PlantStream, Pump};
use crate::precompute::{EnclosureFace, EnclosureRadiation, PrecomputedModel};
use crate::props::{moist_air_cp_j_per_kg_k, moist_air_density};
use crate::schedule::{ScheduleContext, ScheduleSet};
use crate::site::{sun_direction, GroundTemperatureModel, TimestepWeather, WeatherRecord};
use crate::solar::{beam_overlap_m2, incidence_cosine, sunlit_fraction, SkyState};
use crate::units::STEFAN_BOLTZMANN;
use crate::zone_air::ZoneAirState;
use crate::zone_hvac::{ZoneEquipment, ZoneEquipmentRequest};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

const KELVIN: f64 = 273.15;
const FIXED_TEMPERATURE_CONDUCTANCE_W_M2K: f64 = 1.0e9;
const GROUND_REFLECTANCE: f64 = 0.2;

#[path = "⚙️config-and-state/🦀️.rs"]
mod config_and_state;
pub use config_and_state::*;

#[path = "📋️timestep-job-types/🦀️.rs"]
mod timestep_job_types;
pub use timestep_job_types::*;

#[path = "🧭️zone-preparation/🦀️.rs"]
mod zone_preparation;
use zone_preparation::*;

#[path = "🧮️zone-balance/🦀️.rs"]
mod zone_balance;

#[path = "🏭️commit-and-secondary-systems/🦀️.rs"]
mod commit_and_secondary_systems;

// #region 🔖️Kernel
/// 🔄️ BEM simulation kernel with full subsystem coupling.
pub struct SimulationKernel;

impl SimulationKernel {
    /// 🔄️ Initialize state from model and precomputed data.
    pub fn initialize(model: &Model, pre: &PrecomputedModel, weather: &WeatherRecord) -> SimulationModel {
        let mut state = SimulationModel::default();
        state.zones.admit(model.zones.len()).expect("test zone state backing");
        state.surfaces.admit(pre.surfaces.len()).expect("test surface state backing");
        state.windows.admit(pre.windows.len()).expect("test window state backing");
        state.solver.reserve(pre.maximum_enclosure_faces, pre.maximum_nodes).expect("test solver backing");
        for zone in &model.zones {
            let _ = state.zones.insert(zone.id, ZoneState::new(weather.dry_bulb_c, weather.humidity_ratio()));
        }
        for (id, surface) in pre.surfaces.iter() {
            let _ = state.surfaces.insert(*id, SurfaceState { temperatures_c: vec![weather.dry_bulb_c; surface.chain.nodes()], inside_convection_w_m2k: 0.0 });
        }
        for (id, _) in pre.windows.iter() {
            let _ = state.windows.insert(*id, WindowState { face_temperatures_c: [weather.dry_bulb_c; 2 * MAX_PANES], inside_convection_w_m2k: 0.0 });
        }
        state
    }

    /// 🔄️ Advance one hour through the same bounded cursor machine used by EnergyJob.
    #[cfg(test)]
    pub(crate) fn advance_timestep(model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel, weather: &WeatherRecord, date: &SimDate, hour: f64) -> Result<(), Error> {
        let mut work = TimestepWork::new(model, pre, HourWeather { previous: *weather, current: *weather, next: *weather }, *date, hour);
        while !work.is_complete() {
            work.step(model, config, pre, state)?;
        }
        Ok(())
    }

    /// 🔄️ Check energy balance for diagnostics.
    pub fn energy_balance_check(input_w: f64, stored_w: f64, output_w: f64) -> f64 {
        (input_w - stored_w - output_w).abs()
    }

    /// 📅️ Build run period from config.
    pub fn run_period(config: &SimulationConfig) -> RunPeriod {
        RunPeriod { start_month: config.run_period_start_month, start_day: config.run_period_start_day, end_month: config.run_period_end_month, end_day: config.run_period_end_day, year: config.run_period_year }
    }
}
// #endregion 🔖️Kernel

#[path = "🧰️kernel-helpers/🦀️.rs"]
mod kernel_helpers;
use kernel_helpers::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
