use super::*;
use crate::model::*;

#[test]
fn precompute_builds_surface_chains() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    assert!(pre.surfaces.get(&EntityId(30)).is_some_and(|surface| surface.chain.nodes() >= 2));
    assert!(pre.zone_geometry.contains_key(&EntityId(1)));
}

#[test]
fn thermostat_overrides_default_setpoints() {
    let mut model = crate::sim::test_model_single_zone();
    model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 3.0, cooling_throttle_range_k: 4.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let sp = pre.default_setpoints.get(&EntityId(1)).unwrap();
    assert!((sp.heating_throttle_k - 3.0).abs() < 1e-9);
    assert!((sp.cooling_throttle_k - 4.0).abs() < 1e-9);
}

/// 🐛️ Floor-area defect: the zone floor area is the area of its floors only. Summing every surface
/// made case 600's 48 m² into 171.6 m², multiplying its 200 W of equipment to 715 W.
#[test]
fn bestest_floor_area_counts_only_floors() {
    let model = crate::bestest::model("600").expect("case 600");
    let pre = PrecomputedModel::build(&model, 10, 10);
    let zone = model.zones[0].id;
    assert!((pre.zone_geometry.get(&zone).expect("zone geometry").floor_area_m2 - 48.0).abs() < 1e-9);
}

/// 🧪️ Windows are cut out of their host: two 6 m² windows on the 21.6 m² south wall leave 9.6 m²
/// of opaque wall, and the enclosure carries both window faces.
#[test]
fn windows_are_placed_on_and_subtracted_from_their_host() {
    let model = crate::bestest::model("600").expect("case 600");
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
        let model = crate::bestest::model(case).expect("case");
        let pre = PrecomputedModel::build(&model, 10, 10);
        pre.surfaces.iter().map(|(_, surface)| surface.area_m2 * surface.chain.capacitance_j_m2k.iter().sum::<f64>()).sum::<f64>()
    };
    assert!(capacity("900") > 5.0 * capacity("600"));
}

/// 🧪️ Radiant exchange: approximate view factors are reciprocal and complete, and for black
/// faces the gray-body exchange factors reduce to the view factors.
#[test]
fn enclosure_factors_are_reciprocal_complete_and_black_body_consistent() {
    let model = crate::bestest::model("600").expect("case 600");
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
