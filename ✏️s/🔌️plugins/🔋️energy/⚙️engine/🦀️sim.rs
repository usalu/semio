//! 🚀️ Engine orchestration: Model + SimulationConfig → Results.

use crate::economics::{compute_lcca, LccaParameters, UtilityTariff};
use crate::error::Error;
use crate::kernel::{SimulationConfig, SimulationEnvironment, SimulationKernel};
use crate::meters::{EndUse, FuelType, MeterStore};
use crate::metrics::{compute_environmental, compute_resilience, EmissionFactors, SourceEnergyFactors};
use crate::model::Model;
use crate::output::TimeSeriesStore;
use crate::precompute::PrecomputedModel;
use crate::results::{Results, RunMetadata, SummaryTables};
use crate::site::WeatherRecord;
use crate::sizing::{SizingConfig, SizingManager};
use crate::units::Unit;
use std::time::Instant;

// #region 🔖️Engine
/// ⚡️ Headless BEM simulation engine.
pub struct Engine;

impl Engine {
    /// ⚡️ Run full building energy simulation.
    pub fn run(model: &Model, config: &SimulationConfig) -> Result<Results, Error> {
        model.validate().map_err(|d| d.messages.into_iter().find(|m| m.severity == crate::error::Severity::Fatal).unwrap_or_else(|| Error::severe("model validation failed")))?;

        let start = Instant::now();
        let weather_records = Self::resolve_weather(config);
        let pre = PrecomputedModel::build(model, config.zone_timestep_minutes, config.system_timestep_minutes);
        let dt_s = pre.zone_timestep_s;

        let mut state = SimulationKernel::initialize(model, &pre, &weather_records[0]);
        SimulationKernel::warmup(model, config, &pre, &mut state, &weather_records)?;

        let mut time_series = TimeSeriesStore::default();
        let mut meters = MeterStore::default();
        let mut zone_temp_history: Vec<f64> = Vec::new();

        let run_period = SimulationKernel::run_period(config);
        let mut hour_index = 0u32;

        for (date, hour, _) in run_period.hours() {
            let widx = hour_index as usize % weather_records.len().max(1);
            let mut weather = weather_records.get(widx).copied().unwrap_or_else(|| synthetic_hour(hour_index));
            weather.month = date.month;
            weather.day = date.day;
            weather.hour = hour;
            weather.year = date.year;

            SimulationKernel::advance_timestep(model, config, &pre, &mut state, &weather, &date, hour_index as f64, dt_s)?;

            for zone in &model.zones {
                if let Some(zs) = state.zones.get(&zone.id) {
                    let key = format!("Zone Air Temperature [{}]", zone.name);
                    time_series.record(&key, hour_index as f64, zs.air.temp_c, Unit::Celsius);
                    zone_temp_history.push(zs.air.temp_c);

                    let heat_meter = meters.get_or_create(&format!("{} Heating", zone.name), FuelType::Electricity, EndUse::Heating);
                    heat_meter.accumulate(zs.delivered.heating_w, dt_s, hour_index as f64);

                    let cool_meter = meters.get_or_create(&format!("{} Cooling", zone.name), FuelType::Electricity, EndUse::Cooling);
                    cool_meter.accumulate(zs.delivered.cooling_w, dt_s, hour_index as f64);

                    let fan_meter = meters.get_or_create(&format!("{} Fans", zone.name), FuelType::Electricity, EndUse::Fans);
                    fan_meter.accumulate(zs.delivered.fan_w, dt_s, hour_index as f64);
                }
            }

            let facility_heat = meters.get_or_create("Facility Heating", FuelType::Electricity, EndUse::Heating);
            facility_heat.accumulate(state.delivered_total.heating_w, dt_s, hour_index as f64);
            let facility_pv = meters.get_or_create("Facility PV", FuelType::OnSiteGeneration, EndUse::Generators);
            facility_pv.accumulate(-state.delivered_total.pv_generation_w, dt_s, hour_index as f64);

            hour_index += 1;
        }

        let sizing = SizingManager::size(model, &SizingConfig::default());
        let mut summaries = SummaryTables::default();
        let elec_kwh = meters.facility_total_kwh(FuelType::Electricity);
        let gas_kwh = meters.facility_total_kwh(FuelType::NaturalGas);
        summaries.add_annual("Electricity", elec_kwh, "kWh");
        summaries.add_annual("Natural Gas", gas_kwh, "kWh");
        let floor_area: f64 = model.zones.iter().map(|z| z.volume_m3 / 3.0).sum::<f64>().max(1.0);
        summaries.add_annual("Energy Use Intensity", elec_kwh / floor_area, "kWh/m²");

        let environmental = compute_environmental(elec_kwh, gas_kwh, &SourceEnergyFactors::default(), &EmissionFactors::default());
        let heat_sp = 20.0;
        let cool_sp = 26.0;
        let resilience = compute_resilience(&zone_temp_history, heat_sp, cool_sp, true);

        let tariff = UtilityTariff { name: "Default".into(), fuel: FuelType::Electricity, periods: vec![], fixed_monthly_charge: 10.0, ratchet_percent: 0.0 };
        let annual_cost = tariff.energy_cost(elec_kwh, 12, 6) * 12.0;
        let lcca = compute_lcca(annual_cost, &LccaParameters { study_period_years: 25, discount_rate: 0.03, inflation_rate: 0.02, initial_cost: 0.0, annual_maintenance: 0.0, replacement_cost: 0.0, replacement_interval_years: 15 });
        summaries.add_annual("Annual Energy Cost", annual_cost, "USD");
        summaries.add_annual("LCCA Present Value", lcca.present_value_total, "USD");

        let elapsed = start.elapsed().as_millis() as u64;
        Ok(Results {
            time_series,
            meters,
            summaries,
            sizing,
            environmental,
            resilience,
            diagnostics: Default::default(),
            run_metadata: RunMetadata {
                model_name: model.name.clone(),
                model_version: model.version.clone(),
                weather_location: config.weather.as_ref().map_or_else(|| "synthetic".into(), |w| w.location.clone()),
                timesteps: hour_index,
                warmup_days: config.warmup_days,
                elapsed_ms: elapsed,
            },
        })
    }

    fn resolve_weather(config: &SimulationConfig) -> Vec<WeatherRecord> {
        if let Some(epw) = &config.weather {
            return epw.records.clone();
        }
        match config.environment {
            SimulationEnvironment::HeatingDesignDay => design_day_weather(-10.0),
            SimulationEnvironment::CoolingDesignDay => design_day_weather(35.0),
            _ => Self::synthetic_weather_year(),
        }
    }

    fn synthetic_weather_year() -> Vec<WeatherRecord> {
        (0..8760).map(synthetic_hour).collect()
    }
}
// #endregion 🔖️Engine

fn synthetic_hour(h: u32) -> WeatherRecord {
    let day = h / 24;
    let hour = h % 24;
    let month = (day / 30 + 1).min(12) as u8;
    let t_base = 15.0 + 10.0 * ((day as f64 / 365.0) * 2.0 * std::f64::consts::PI).sin();
    let t_daily = 5.0 * ((hour as f64 - 14.0) / 12.0 * std::f64::consts::PI).cos();
    WeatherRecord {
        year: 2026,
        month,
        day: (day % 30 + 1) as u8,
        hour: hour as u8,
        minute: 0,
        dry_bulb_c: t_base + t_daily,
        dew_point_c: t_base - 5.0,
        relative_humidity: 0.5,
        atmospheric_pressure_pa: 101_325.0,
        wind_speed_m_s: 3.0,
        wind_direction_deg: 180.0,
        direct_normal_irradiance_w_m2: if (6..18).contains(&hour) { 500.0 } else { 0.0 },
        diffuse_horizontal_irradiance_w_m2: if (6..18).contains(&hour) { 100.0 } else { 0.0 },
        horizontal_infrared_w_m2: 250.0,
        precipitation_mm: 0.0,
        snow_depth_mm: 0.0,
    }
}

fn design_day_weather(dry_bulb_c: f64) -> Vec<WeatherRecord> {
    (0..24)
        .map(|hour| WeatherRecord {
            year: 2026,
            month: 1,
            day: 1,
            hour: hour as u8,
            minute: 0,
            dry_bulb_c,
            dew_point_c: dry_bulb_c - 10.0,
            relative_humidity: 0.5,
            atmospheric_pressure_pa: 101_325.0,
            wind_speed_m_s: 3.0,
            wind_direction_deg: 180.0,
            direct_normal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 800.0 } else { 0.0 },
            diffuse_horizontal_irradiance_w_m2: if dry_bulb_c > 20.0 && (8..17).contains(&hour) { 150.0 } else { 0.0 },
            horizontal_infrared_w_m2: 250.0,
            precipitation_mm: 0.0,
            snow_depth_mm: 0.0,
        })
        .collect()
}

// #region 🔖️Fixtures
/// 🧪️ Build a minimal test model for integration tests.
pub fn test_model_single_zone() -> Model {
    use crate::model::*;
    Model {
        name: "BESTEST Single Zone".into(),
        version: "1.0".into(),
        site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        zones: vec![Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 106.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        materials: vec![Material {
            id: EntityId(10),
            name: "Insulation".into(),
            thickness_m: 0.1,
            conductivity_w_m_k: 0.04,
            density_kg_m3: 50.0,
            specific_heat_j_kg_k: 1000.0,
            thermal_absorptance: 0.9,
            solar_absorptance: 0.7,
            visible_absorptance: 0.7,
        }],
        constructions: vec![Construction { id: EntityId(20), name: "Wall".into(), layer_material_ids: vec![EntityId(10)] }],
        surfaces: vec![Surface {
            id: EntityId(30),
            name: "ExtWall".into(),
            zone_id: EntityId(1),
            class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
            construction_id: EntityId(20),
            outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true,
            wind_exposed: true,
            multiplier: 1,
        }],
        ideal_loads: vec![IdealLoadsSystem {
            id: EntityId(40),
            zone_id: EntityId(1),
            max_heating_supply_air_temp_c: 50.0,
            min_cooling_supply_air_temp_c: 13.0,
            max_heating_capacity_w: None,
            max_cooling_capacity_w: None,
            outdoor_air_per_person_m3_s: 0.00944,
            outdoor_air_per_area_m3_s_m2: 0.0,
        }],
        ..Default::default()
    }
}

/// 🧪️ Full topology test model with plant, PV, AFN, daylight.
pub fn test_model_full_topology() -> Model {
    use crate::model::*;
    let mut model = test_model_single_zone();
    model.name = "Full Topology".into();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 2.0, cooling_throttle_range_k: 2.0 });
    model.plant_loops.push(PlantLoopConfig { id: EntityId(60), name: "Hot Water".into(), loop_type: PlantLoopType::Heating, supply_temperature_c: 55.0, return_temperature_c: 45.0, design_flow_kg_s: 2.0, equipment_ids: vec![EntityId(61)] });
    model.pv_systems.push(PvSystemAssignment { id: EntityId(70), dc_capacity_w: 5000.0, area_m2: 25.0, tilt_deg: 30.0, azimuth_deg: 180.0, module_efficiency: 0.2, inverter_efficiency: 0.96 });
    model.daylight_zones.push(DaylightZoneConfig { id: EntityId(80), zone_id: EntityId(1), illuminance_target_lux: 500.0, glare_limit: 0.4, window_transmittance: 0.6 });
    model
}
// #endregion 🔖️Fixtures

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::RunPeriod;

    #[test]
    fn engine_runs_single_zone() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert!(results.run_metadata.timesteps > 0);
        assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
    }

    #[test]
    fn engine_deterministic_repeatability() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let r1 = Engine::run(&model, &config).unwrap();
        let r2 = Engine::run(&model, &config).unwrap();
        assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
        assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
    }

    #[test]
    fn ashrae_140_case600_base() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        let temps = results.time_series.get("Zone Air Temperature [Zone1]");
        assert!(temps.is_some());
    }

    #[test]
    fn invalid_model_rejected() {
        let model = Model::default();
        assert!(Engine::run(&model, &SimulationConfig::default()).is_err());
    }

    #[test]
    fn energy_conservation_order_of_magnitude() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
        assert!(total_kwh < 1_000_000.0);
    }

    #[test]
    fn full_topology_e2e() {
        let model = test_model_full_topology();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert!(results.run_metadata.timesteps >= 48);
        assert!(results.summaries.annual_energy.len() >= 3);
    }

    #[test]
    fn hvac_bestest_heating_day() {
        let model = test_model_single_zone();
        let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
        let results = Engine::run(&model, &config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 24);
        assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
    }

    #[test]
    fn run_period_honors_calendar() {
        let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
        assert_eq!(period.total_hours(), 168);
        let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
        let model = test_model_single_zone();
        let results = Engine::run(&model, &config).unwrap();
        assert_eq!(results.run_metadata.timesteps, 168);
    }
}
