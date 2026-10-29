//! 🧰️ Energy mutation fixture laws: the committed-vector case shape and the eight laws every vector is held to, the
//! roster the language-neutral adapter mirrors, and the codec probe corpus. Its `[[test]]` targets run them.

#![allow(async_fn_in_trait, clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_mut_air_loop as air_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_air_loop_nodes as air_loop_nodes_mutations;
extern crate semio_s_artifact_energy_model_mut_battery as battery_mutations;
extern crate semio_s_artifact_energy_model_mut_boundary as boundary_mutations;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_mut_day_schedule as day_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_daylight_zone as daylight_zone_mutations;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_mut_enclosure as enclosure_mutations;
extern crate semio_s_artifact_energy_model_mut_equipment_gain as equipment_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_fault as fault_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration_geometry as fenestration_geometry_mutations;
extern crate semio_s_artifact_energy_model_mut_finish as finish_mutations;
extern crate semio_s_artifact_energy_model_mut_fixed_schedule as fixed_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_glazing as glazing_mutations;
extern crate semio_s_artifact_energy_model_mut_humidistat as humidistat_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads as ideal_loads_mutations;
extern crate semio_s_artifact_energy_model_mut_ideal_loads_limits as ideal_loads_limits_mutations;
extern crate semio_s_artifact_energy_model_mut_infiltration as infiltration_mutations;
extern crate semio_s_artifact_energy_model_mut_leakage as leakage_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain as lighting_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain_emission as lighting_gain_emission_mutations;
extern crate semio_s_artifact_energy_model_mut_load_center as load_center_mutations;
extern crate semio_s_artifact_energy_model_mut_material as material_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mut_model as model_mutations;
extern crate semio_s_artifact_energy_model_mut_model_simulation as model_simulation_mutations;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_outdoor_air as outdoor_air_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain as people_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain_fractions as people_gain_fractions_mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop as plant_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop_design as plant_loop_design_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system as pv_system_mutations;
extern crate semio_s_artifact_energy_model_mut_pv_system_performance as pv_system_performance_mutations;
extern crate semio_s_artifact_energy_model_mut_refrigeration as refrigeration_mutations;
extern crate semio_s_artifact_energy_model_mut_room_air as room_air_mutations;
extern crate semio_s_artifact_energy_model_mut_run as run_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_setpoint as setpoint_mutations;
extern crate semio_s_artifact_energy_model_mut_shading_device as shading_device_mutations;
extern crate semio_s_artifact_energy_model_mut_shading as shading_mutations;
extern crate semio_s_artifact_energy_model_mut_shw_system as shw_system_mutations;
extern crate semio_s_artifact_energy_model_mut_site as site_mutations;
extern crate semio_s_artifact_energy_model_mut_site_ground as site_ground_mutations;
extern crate semio_s_artifact_energy_model_mut_sizing as sizing_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_solar_thermal as solar_thermal_mutations;
extern crate semio_s_artifact_energy_model_mut_space_list as space_list_mutations;
extern crate semio_s_artifact_energy_model_mut_space as space_mutations;
extern crate semio_s_artifact_energy_model_mut_surface as surface_mutations;
extern crate semio_s_artifact_energy_model_mut_surface_properties as surface_properties_mutations;
extern crate semio_s_artifact_energy_model_mut_thermostat as thermostat_mutations;
extern crate semio_s_artifact_energy_model_mut_time_series as time_series_mutations;
extern crate semio_s_artifact_energy_model_mut_ventilation as ventilation_mutations;
extern crate semio_s_artifact_energy_model_mut_water_system as water_system_mutations;
extern crate semio_s_artifact_energy_model_mut_week_schedule as week_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer as window_layer_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer_optics as window_layer_optics_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer_optics_visible as window_layer_optics_visible_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule as year_schedule_mutations;
extern crate semio_s_artifact_energy_model_mut_year_schedule_holidays as year_schedule_holidays_mutations;
extern crate semio_s_artifact_energy_model_mut_zone_equipment as zone_equipment_mutations;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

#[path = "🧫️case/🦀️.rs"]
mod case;
pub use case::{fixture_root, Case};

#[path = "🛠️constructors/🦀️.rs"]
mod constructors;
pub use constructors::{construction, gas_material, glazing_material, link, material, shading, snapshot, space, surface, window, zone};

#[path = "⚖️laws/🦀️.rs"]
mod laws;
pub use laws::{assert_canonical, assert_diff, assert_diff_applies, assert_diff_canonical, assert_forward, assert_inverse, assert_laws, assert_outcome, assert_semantics, committed, write_when_requested};

#[path = "🏷️kinds/🦀️.rs"]
mod kinds;
pub use kinds::KINDS;

#[path = "🗂️directories/🦀️.rs"]
mod directories;
pub use directories::DIRECTORIES;

#[path = "🧵️probe-corpus/🦀️.rs"]
mod probe_corpus;
pub use probe_corpus::wire_probes;
