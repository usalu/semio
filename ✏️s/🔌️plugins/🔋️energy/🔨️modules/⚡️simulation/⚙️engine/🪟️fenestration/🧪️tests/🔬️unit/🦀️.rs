use super::*;

fn bestest_clear_pane() -> Pane {
    Pane { thickness_m: 0.003048, conductivity_w_m_k: 1.0, solar_transmittance: 0.834, solar_reflectance_front: 0.075, solar_reflectance_back: 0.075, infrared_transmittance: 0.0, infrared_emissivity_front: 0.84, infrared_emissivity_back: 0.84 }
}

fn bestest_double_glazing() -> GlazingSystem {
    GlazingSystem::layered(&[bestest_clear_pane(), bestest_clear_pane()], &[Gap { width_m: 0.012, gas: GasKind::Air }]).expect("two panes, one gap")
}

/// 🔮️ Window optics defect: the BESTEST double clear glazing must reproduce EnergyPlus 25.2's
/// layer-by-layer angular transmittance (`eplusout.eio` "WindowConstruction" and the angular
/// report of the committed reference run), not a constant SHGC.
#[test]
fn layered_double_glazing_matches_energyplus_angular_transmittance() {
    let expected = [0.6995, 0.6986, 0.6953, 0.6882, 0.673, 0.6407, 0.5717, 0.4321, 0.2011, 0.0];
    let system = bestest_double_glazing();
    for (index, value) in expected.iter().enumerate() {
        let cs = (index as f64 * 10.0).to_radians().cos();
        let ours = system.beam_transmittance(cs).max(0.0);
        assert!((ours - value).abs() < 2e-3, "{}°: semio {ours:.4} vs E+ {value}", index * 10);
    }
    assert!((system.diffuse_transmittance - 0.5964).abs() < 2e-3, "diffuse {}", system.diffuse_transmittance);
    let absorbed: f64 = (0..system.panes).map(|pane| system.beam_front_absorptance(pane, 1.0)).sum();
    assert!(absorbed > 0.1 && system.beam_transmittance(1.0) + absorbed < 1.0);
    assert_eq!(system.faces(), 4);
    assert!(GlazingSystem::layered(&[bestest_clear_pane()], &[Gap { width_m: 0.012, gas: GasKind::Air }]).is_none());
}

/// 🔮️ Simple glazing: U 3.0 / SHGC 0.787 must become EnergyPlus's equivalent single layer
/// (`WindowMaterial:SimpleGlazingSystem`: T 0.72680, R 0.11440, layer conductance 6.1359 W/(m²·K))
/// and need no film-coefficient adjustment.
#[test]
fn simple_glazing_matches_the_energyplus_equivalent_layer() {
    let layer = simple_glazing_layer(3.0, 0.787);
    assert!((layer.pane.solar_transmittance - 0.72680).abs() < 1e-3, "T {}", layer.pane.solar_transmittance);
    assert!((layer.pane.solar_reflectance_front - 0.11440).abs() < 1e-3, "R {}", layer.pane.solar_reflectance_front);
    assert!((layer.pane.conductivity_w_m_k / layer.pane.thickness_m - 6.1359).abs() < 1e-2);
    let system = GlazingSystem::simple(3.0, 0.787);
    assert!((system.beam_transmittance(1.0) - 0.72680).abs() < 3e-3, "the angular fit through the curve family ends near the layer value, was {}", system.beam_transmittance(1.0));
    assert!((rated_coefficient_adjustment(&system) - 1.0).abs() < 1e-12);
}

/// 🧪️ Gap and room-side glazing films are positive and grow with the temperature difference.
#[test]
fn glazing_films_grow_with_temperature_difference() {
    let small = gap_conductance_w_m2k(280.0, 282.0, 0.012, 1.5, 90.0, GasKind::Air);
    let large = gap_conductance_w_m2k(270.0, 292.0, 0.012, 1.5, 90.0, GasKind::Air);
    assert!(small > 0.0 && large >= small);
    assert!(gap_conductance_w_m2k(270.0, 292.0, 0.012, 1.5, 90.0, GasKind::Argon) < large);
    let calm = glazing_interior_convection_w_m2k(20.5, 21.0, 0.008, 101_325.0, 2.0, 90.0);
    let cold = glazing_interior_convection_w_m2k(10.0, 21.0, 0.008, 101_325.0, 2.0, 90.0);
    assert!(cold > calm && calm > 0.0);
}

#[test]
fn condensation_when_surface_cold() {
    assert!(matches!(condensation_risk(5.0, 0.012, 101_325.0), CondensationRisk::Condensing));
    assert!(matches!(condensation_risk(25.0, 0.004, 101_325.0), CondensationRisk::None));
}
