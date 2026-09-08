
use super::*;

#[test]
fn meter_accumulates_energy() {
    let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Heating, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
    m.accumulate(1000.0, 3600.0, 1.0);
    assert!((m.energy_kwh() - 1.0).abs() < 1e-6);
}

#[test]
fn meter_tracks_peak_demand_hour() {
    let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Cooling, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
    m.accumulate(500.0, 3600.0, 1.0);
    m.accumulate(1500.0, 3600.0, 2.0);
    m.accumulate(200.0, 3600.0, 3.0);
    assert!((m.peak_demand_w - 1500.0).abs() < 1e-9);
    assert!((m.peak_demand_hour - 2.0).abs() < 1e-9);
}

#[test]
fn store_get_or_create_is_idempotent_and_totals_by_fuel() {
    let mut store = MeterTable::default();
    store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
    store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 1.0);
    store.get_or_create("Boiler Gas", FuelType::NaturalGas, EndUse::Heating).accumulate(2000.0, 3600.0, 0.0);
    assert_eq!(store.meters.len(), 2);
    assert!((store.facility_total_kwh(FuelType::Electricity) - 2.0).abs() < 1e-6);
    assert!((store.facility_total_kwh(FuelType::NaturalGas) - 2000.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
    assert_eq!(store.facility_total_kwh(FuelType::Propane), 0.0);
}

#[test]
fn end_use_breakdown_aggregates_by_category() {
    let mut store = MeterTable::default();
    store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
    store.get_or_create("Zone2 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
    store.get_or_create("Fans", FuelType::Electricity, EndUse::Fans).accumulate(500.0, 3600.0, 0.0);
    let breakdown = store.end_use_breakdown();
    let heating = breakdown.iter().find(|(end_use, _)| *end_use == EndUse::Heating).map(|(_, value)| *value).unwrap();
    let fans = breakdown.iter().find(|(end_use, _)| *end_use == EndUse::Fans).map(|(_, value)| *value).unwrap();
    assert!((heating - 2.0 * 1000.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
    assert!((fans - 500.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
}
