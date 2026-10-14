use super::*;
use crate::geometry::surface_area_m2;
use crate::kernel::SimulationEnvironment;
use crate::precompute::{enclosure_factors, mean_radiant_participation, EnclosureRadiation, PrecomputedModel};
use crate::sim::Engine;

#[test]
fn every_registered_case_builds_and_validates() {
    for case in CASES {
        let built = model(case).unwrap_or_else(|| panic!("case {case} must build"));
        if let Err(diagnostics) = built.validate() {
            panic!("case {case} failed validation: {:?}", diagnostics.messages);
        }
    }
}

#[test]
fn unknown_case_is_none() {
    assert!(model("601").is_none());
}

#[test]
fn free_float_cases_carry_no_hvac() {
    for case in ["600FF", "900FF"] {
        let built = model(case).expect("free-float case builds");
        assert!(built.ideal_loads.is_empty(), "{case} must have no ideal loads");
        assert!(built.thermostats.is_empty(), "{case} must have no thermostat");
    }
}

#[test]
fn base_case_geometry_matches_the_standard() {
    let built = model("600").expect("case 600 builds");
    assert!((built.zones[0].volume_m3 - 129.6).abs() < 1e-9);
    assert_eq!(built.surfaces.len(), 6);
    let glazing: f64 = built.fenestrations.iter().map(|window| window.area_m2).sum();
    assert!((glazing - 12.0).abs() < 1e-9, "south glazing was {glazing} m²");
    assert!(built.fenestrations.iter().all(|window| window.surface_id == SOUTH_WALL));
    let floor = built.surfaces.iter().find(|surface| surface.id == FLOOR).expect("floor");
    assert_eq!(floor.outside_boundary_condition, OutsideBoundary::OutdoorAir, "ASHRAE 140 §5.2.1.10: the floor is exposed to outdoor air");
    assert!(!floor.sun_exposed && !floor.wind_exposed, "…with no sun and no wind");
    assert!(built.fenestrations.iter().all(|window| window.glazing_construction_id == Some(WINDOW_CONSTRUCTION)), "windows are the layered double pane");
}

#[test]
fn high_mass_cases_swap_only_the_wall_and_floor() {
    let light = model("600").expect("case 600 builds");
    let heavy = model("900").expect("case 900 builds");
    assert_eq!(light.surfaces, heavy.surfaces);
    assert_eq!(light.fenestrations, heavy.fenestrations);
    let heavy_wall = heavy.constructions.iter().find(|construction| construction.id == WALL_CONSTRUCTION).expect("wall construction");
    assert_eq!(heavy_wall.layer_material_ids, vec![WOOD_SIDING, FOAM_INSULATION, CONCRETE_BLOCK]);
}

#[test]
fn shaded_cases_carry_their_projections() {
    let overhang = model("610").expect("610");
    assert_eq!(overhang.shading_surfaces.len(), 1, "610 carries one full-width overhang");
    assert!(overhang.shading_surfaces[0].vertices_m.iter().all(|vertex| (vertex[2] - HEIGHT_M).abs() < 1e-9), "the overhang sits at roof level");
    assert!(overhang.fenestrations.iter().all(|window| window.overhang_depth_m == 0.0));
    assert!(model("630").expect("630").fenestrations.iter().all(|window| window.overhang_depth_m > 0.0 && window.fin_depth_m > 0.0));
    assert!(model("600").expect("600").shading_surfaces.is_empty());
}

#[test]
fn orientation_cases_move_the_glazing_east_and_west() {
    let built = model("620").expect("620");
    let hosts: Vec<EntityId> = built.fenestrations.iter().map(|window| window.surface_id).collect();
    assert_eq!(hosts, vec![EAST_WALL, WEST_WALL]);
}

#[test]
fn setback_case_drops_to_ten_degrees_overnight() {
    let built = model("640").expect("640");
    let schedule = built.schedules.daily.iter().find(|schedule| schedule.id == HEATING_SETBACK).expect("setback schedule");
    assert!((schedule.hourly_values[2] - 10.0).abs() < 1e-9);
    assert!((schedule.hourly_values[12] - 20.0).abs() < 1e-9);
    assert!((schedule.hourly_values[23] - 10.0).abs() < 1e-9);
}

#[test]
fn night_ventilation_case_runs_the_fan_only_overnight() {
    let built = model("650").expect("650");
    let schedule = built.schedules.daily.iter().find(|schedule| schedule.id == NIGHT_VENTILATION_SCHEDULE).expect("ventilation schedule");
    assert!((schedule.hourly_values[2] - 1.0).abs() < 1e-9);
    assert!(schedule.hourly_values[12].abs() < 1e-9);
    assert!((built.mechanical_ventilations[0].design_flow_m3_s - NIGHT_VENTILATION_M3_S).abs() < 1e-9);
}

/// 🧪️ A model whose thermostat points at a schedule nobody defines must be rejected — the whole
/// reason `schedules` lives on the model rather than only on the run configuration.
#[test]
fn dangling_schedule_reference_is_rejected() {
    let mut built = model("600").expect("600");
    built.schedules.constants.retain(|schedule| schedule.id != HEATING_SETPOINT);
    assert!(built.validate().is_err());
}

//#region 🧠️PrecomputeOnCases
/// 🐛️ Floor-area defect: the zone floor area is the area of its floors only. Summing every surface
/// made case 600's 48 m² into 171.6 m², multiplying its 200 W of equipment to 715 W.
#[test]
fn bestest_floor_area_counts_only_floors() {
    let model = model("600").expect("case 600");
    let pre = PrecomputedModel::build(&model, 10, 10);
    let zone = model.zones[0].id;
    assert!((pre.zone_geometry.get(&zone).expect("zone geometry").floor_area_m2 - 48.0).abs() < 1e-9);
}

/// 🧪️ Windows are cut out of their host: two 6 m² windows on the 21.6 m² south wall leave 9.6 m²
/// of opaque wall, and the enclosure carries both window faces.
#[test]
fn windows_are_placed_on_and_subtracted_from_their_host() {
    let model = model("600").expect("case 600");
    let pre = PrecomputedModel::build(&model, 10, 10);
    assert_eq!(pre.windows.len(), 2);
    let south = pre.windows.iter().next().expect("window").1.surface_id;
    let wall = pre.surfaces.get(&south).expect("host");
    assert!((wall.gross_area_m2 - 21.6).abs() < 1e-5 && (wall.area_m2 - 9.6).abs() < 1e-5, "gross {} net {}", wall.gross_area_m2, wall.area_m2);
    for (_, window) in pre.windows.iter() {
        assert!((surface_area_m2(&window.polygon) - 6.0).abs() < 1e-5);
        assert!((window.glazing.beam_transmittance(1.0) - 0.6995).abs() < 2e-3, "BESTEST windows are the layered double pane");
    }
    let enclosure = pre.enclosures.get(&model.zones[0].id).expect("enclosure");
    assert_eq!(enclosure.faces.len(), 8);
}

/// 🧪️ Mass defect: the heavy case's wall and floor chains store an order of magnitude more heat.
#[test]
fn heavy_constructions_carry_more_capacitance() {
    let capacity = |case: &str| {
        let model = model(case).expect("case");
        let pre = PrecomputedModel::build(&model, 10, 10);
        pre.surfaces.iter().map(|(_, surface)| surface.area_m2 * surface.chain.capacitance_j_m2k.iter().sum::<f64>()).sum::<f64>()
    };
    assert!(capacity("900") > 5.0 * capacity("600"));
}

/// 🧪️ Radiant exchange: approximate view factors are reciprocal and complete, and for black
/// faces the gray-body exchange factors reduce to the view factors.
#[test]
fn enclosure_factors_are_reciprocal_complete_and_black_body_consistent() {
    let model = model("600").expect("case 600");
    let pre = PrecomputedModel::build(&model, 10, 10);
    let enclosure = pre.enclosures.get(&model.zones[0].id).expect("enclosure");
    let EnclosureRadiation::Exchange { view_factors, .. } = &enclosure.radiation else { panic!("an 8-face enclosure uses exact factors") };
    let n = enclosure.faces.len();
    for i in 0..n {
        let row: f64 = (0..n).map(|j| view_factors[i * n + j]).sum();
        assert!((row - 1.0).abs() < 1e-2, "row {i} summed to {row}");
        for j in 0..n {
            let (a, b) = (enclosure.areas_m2[i] * view_factors[i * n + j], enclosure.areas_m2[j] * view_factors[j * n + i]);
            assert!((a - b).abs() < 1e-6 * enclosure.areas_m2[i].max(1.0), "A F not reciprocal between {i} and {j}");
        }
    }
    let black = vec![0.99999; n];
    let descriptors: Vec<(f64, f64, bool)> = (0..n).map(|i| (i as f64 * 45.0, 90.0, false)).collect();
    let (view, exchange) = enclosure_factors(&enclosure.areas_m2, &black, &descriptors).expect("factors");
    for index in 0..n * n {
        if index % (n + 1) != 0 {
            assert!((exchange[index] - view[index]).abs() < 1e-3, "black exchange {} vs view {}", exchange[index], view[index]);
        }
    }
    let participation = mean_radiant_participation(&enclosure.areas_m2, &enclosure.emissivities).expect("participation");
    assert!(participation.iter().all(|p| *p > 0.0));
}
//#endregion 🧠️PrecomputeOnCases

//#region ☀️DesignDaySolar
#[test]
fn a_window_row_carries_transmitted_solar_on_a_sunny_design_day() {
    let model = model("600").expect("BESTEST case 600 is a bundled fixture");
    let windows: Vec<_> = model.fenestrations.iter().map(|fenestration| fenestration.id).collect();
    assert!(!windows.is_empty(), "case 600 is the glazed case");
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::CoolingDesignDay, ..Default::default() };
    let config = SimulationConfig { schedules: simulation_config(&model, None, 1).schedules, ..config };
    let results = Engine::run(model, config).expect("cooling design day runs");

    let window_rows: Vec<_> = results.per_surface.iter().filter(|row| windows.contains(&row.id)).collect();
    assert_eq!(window_rows.len(), windows.len(), "every fenestration gets its own row");
    let transmitted: f64 = window_rows.iter().map(|row| row.solar_transmitted_kwh).sum();
    let absorbed: f64 = window_rows.iter().map(|row| row.solar_absorbed_kwh).sum();
    assert!(transmitted > 0.0, "a south window under an 800 W/m² beam must transmit solar, got {transmitted} kWh");
    assert!(absorbed > 0.0, "and its panes must absorb some of it, got {absorbed} kWh");
    assert!(results.per_surface.iter().any(|row| !windows.contains(&row.id) && row.solar_absorbed_kwh > 0.0), "the sun-exposed opaque surfaces absorb solar on their outside face too");
}
//#endregion ☀️DesignDaySolar
