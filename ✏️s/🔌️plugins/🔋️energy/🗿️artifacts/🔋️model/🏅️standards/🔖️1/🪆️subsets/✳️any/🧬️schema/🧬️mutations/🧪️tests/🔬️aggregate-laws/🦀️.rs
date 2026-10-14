//! 🎯️ Laws over the whole aggregate: middle-row inverses, structural correspondence of the roster with the leaves
//! and the oracle catalog, and the binary op codec round trip over the probe corpus.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_mut_air_loop as air_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_battery as battery_mutations;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_mut_day_schedule as day_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_daylight_zone as daylight_zone_mutations;
extern crate semio_s_artifact_energy_model_mut_enclosure as enclosure_mutations;
extern crate semio_s_artifact_energy_model_mut_equipment_gain as equipment_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_fault as fault_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_fixed_schedule as fixed_schedule_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_humidistat as humidistat_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads as ideal_loads_mutations;
extern crate semio_s_artifact_energy_model_mut_infiltration as infiltration_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain as lighting_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_load_center as load_center_mutations;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_outdoor_air as outdoor_air_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain as people_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop as plant_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system as pv_system_mutations;
extern crate semio_s_artifact_energy_model_mut_refrigeration as refrigeration_mutations;
extern crate semio_s_artifact_energy_model_mut_room_air as room_air_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_setpoint as setpoint_mutations;
extern crate semio_s_artifact_energy_model_mut_shading as shading_mutations;
extern crate semio_s_artifact_energy_model_mut_shw_system as shw_system_mutations;
extern crate semio_s_artifact_energy_model_mut_sizing as sizing_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_solar_thermal as solar_thermal_mutations;
extern crate semio_s_artifact_energy_model_mut_space_list as space_list_mutations;
extern crate semio_s_artifact_energy_model_mut_space as space_mutations;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;
extern crate semio_s_artifact_energy_model_mut_time_series as time_series_mutations;
extern crate semio_s_artifact_energy_model_mut_ventilation as ventilation_mutations;
extern crate semio_s_artifact_energy_model_mut_water_system as water_system_mutations;
extern crate semio_s_artifact_energy_model_mut_week_schedule as week_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule as year_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_zone_equipment as zone_equipment_mutations;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

use crate::fixtures::{wire_probes, DIRECTORIES, KINDS};
use crate::mutations::binary::{decode_op, encode_op};
use crate::mutations::EnergyModelMutation;

#[path = "../🔬️middle-row-laws/🦀️.rs"]
mod middle_row_laws;
#[path = "../🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence;
#[path = "../../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod binary_codec;
