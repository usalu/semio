/** 🔺️ Native sparse prepared branches preserve complete rows and original link slots. */
import type {EnergyModel,EnergyModelSnapshot} from "../📸️snapshot/🟦️.ts";
export interface EnergyModelBranches {
  identity: Pick<EnergyModel,"name"|"version"|"site"|"ground_temperature"|"run_period">;
  geometry: Pick<EnergyModel,"zones"|"spaces"|"surfaces"|"fenestrations"|"shading_surfaces"|"space_lists"|"thermal_enclosures"|"adjacency_pairs">;
  materials: Pick<EnergyModel,"materials"|"glazing_materials"|"gas_materials"|"constructions">;
  loads: Pick<EnergyModel,"people"|"lighting"|"equipment"|"infiltrations"|"mechanical_ventilations"|"daylight_zones"|"room_air_models">;
  controls: Pick<EnergyModel,"thermostats"|"humidistats"|"setpoint_managers"|"ideal_loads"|"zone_equipment">;
  systems: Pick<EnergyModel,"air_loops"|"plant_loops"|"outdoor_air_systems"|"airflow_network"|"electrical_load_centers">;
  generation: Pick<EnergyModel,"pv_systems"|"battery_storage"|"shw_systems"|"solar_thermal_systems"|"refrigeration_systems"|"water_systems"|"faults">;
  outputs: Pick<EnergyModel,"output_variables"|"sizing_objects"|"schedules">;
}
export interface EnergyModelDiff {
  model: Partial<EnergyModelBranches>;
  referencedModel?: EnergyModelSnapshot["referencedModel"];
  weatherLink?: EnergyModelSnapshot["weatherLink"];
}
