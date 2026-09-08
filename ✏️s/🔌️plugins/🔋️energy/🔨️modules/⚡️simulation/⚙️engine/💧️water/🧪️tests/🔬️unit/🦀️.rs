
use super::*;

#[test]
fn fixture_flow_scales_with_schedule() {
    let fixture = WaterFixture { name: "Lav".into(), fixture_type: FixtureType::Lavatory, peak_flow_l_s: 0.1, schedule_factor: 0.25, hot_water_fraction: 0.5 };
    assert!((fixture.flow_m3_s() - 0.000_025).abs() < 1e-9);
}

#[test]
fn tank_level_rises_with_inflow() {
    let tank = WaterTank { volume_m3: 5.0, level_m: 1.0, max_level_m: 3.0, min_level_m: 0.2, inlet_temperature_c: 15.0 };
    let (new_level, _) = tank.simulate(0.01, 0.0, 3600.0, 5.0);
    assert!(new_level > tank.level_m);
}

#[test]
fn rainwater_harvest_reduces_with_first_flush() {
    let system = RainwaterSystem { catchment_area_m2: 200.0, runoff_coefficient: 0.85, tank: WaterTank { volume_m3: 10.0, level_m: 1.5, max_level_m: 2.5, min_level_m: 0.1, inlet_temperature_c: 15.0 }, first_flush_l: 50.0, filter_efficiency: 0.95 };
    let harvest = system.harvest_m3(10.0, 3600.0);
    assert!(harvest > 0.0);
    assert!(harvest < 200.0 * 10.0 / 1000.0);
}

#[test]
fn cooling_makeup_increases_with_load() {
    let makeup = CoolingTowerMakeup { cycles_of_concentration: 4.0, drift_fraction: 0.001, basin_volume_m3: 2.0 };
    let low = makeup.makeup_kg_s(100_000.0);
    let high = makeup.makeup_kg_s(500_000.0);
    assert!(high > low);
}

#[test]
fn water_balance_mains_import() {
    let fixtures = vec![WaterFixture { name: "Shower".into(), fixture_type: FixtureType::Shower, peak_flow_l_s: 0.12, schedule_factor: 1.0, hot_water_fraction: 0.8 }];
    let balance = water_balance(&fixtures, 0.0, 0.05, 0.0, 0.0);
    assert!(balance.mains_import_m3_s > 0.0);
}

#[test]
fn irrigation_demand_scales_with_et0() {
    let irrigation = IrrigationSystem { landscaped_area_m2: 500.0, crop_coefficient: 0.8, irrigation_efficiency: 0.75, precipitation_mm_per_day: 2.0 };
    let low = irrigation.demand_m3_s(3.0, 1.0);
    let high = irrigation.demand_m3_s(8.0, 1.0);
    assert!(high > low);
}
