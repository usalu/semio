use super::*;

/// 🧪️ TARP natural convection: vertical faces use the Walton 1.31·|ΔT|^⅓ branch, a warm floor face
/// facing up the enhanced branch and a warm ceiling face facing down the reduced branch.
#[test]
fn natural_convection_follows_the_tarp_branches() {
    assert!((natural_convection_w_m2k(8.0, 0.0) - 2.62).abs() < 1e-9);
    assert!((natural_convection_w_m2k(8.0, 1.0) - 2.0 * 9.482 / 6.238).abs() < 1e-9);
    assert!((natural_convection_w_m2k(8.0, -1.0) - 2.0 * 1.810 / 2.382).abs() < 1e-9);
    assert!((interior_convection_w_m2k(20.0, 20.0, 0.0) - MINIMUM_CONVECTION_W_M2K).abs() < 1e-12);
}

/// 🧪️ Without wind the DOE-2 exterior coefficient is exactly its natural part; wind raises it,
/// windward more than leeward, and a rough surface more than a smooth one.
#[test]
fn exterior_convection_adds_roughness_scaled_forced_part() {
    let still = exterior_convection_w_m2k(10.0, 0.0, 0.0, 0.0, true, 2.17);
    assert!((still - natural_convection_w_m2k(10.0, 0.0)).abs() < 1e-12);
    let windward = exterior_convection_w_m2k(10.0, 0.0, 0.0, 4.0, true, 1.0);
    let leeward = exterior_convection_w_m2k(10.0, 0.0, 0.0, 4.0, false, 1.0);
    let rough = exterior_convection_w_m2k(10.0, 0.0, 0.0, 4.0, true, 2.17);
    assert!(rough > windward && windward > leeward && leeward > still);
    assert!(is_windward(0.0, 180.0, 170.0) && !is_windward(0.0, 180.0, 0.0) && is_windward(1.0, 0.0, 180.0));
    assert!((wind_speed_at_height(3.0, WEATHER_STATION_HEIGHT_M) - 3.0).abs() < 1e-9);
}

/// 🧪️ A vertical wall splits its long-wave view half to the ground and half to the sky hemisphere,
/// of which `√½` is sky and the rest air; a roof sees only sky.
#[test]
fn exterior_radiation_splits_sky_air_and_ground() {
    let (sky, air, ground) = exterior_radiation_w_m2k(10.0, 10.0, 10.0, 0.9, 0.0);
    let total = sky + air + ground;
    assert!((total - 0.9 * STEFAN_BOLTZMANN * 4.0 * 283.15_f64.powi(3)).abs() < 1e-9);
    assert!((ground / total - 0.5).abs() < 1e-12);
    assert!((sky / total - 0.5 * 0.5_f64.sqrt()).abs() < 1e-12);
    let (_, roof_air, roof_ground) = exterior_radiation_w_m2k(10.0, 0.0, -20.0, 0.9, 1.0);
    assert!(roof_air.abs() < 1e-12 && roof_ground.abs() < 1e-12);
}

fn concrete_chain(time_step_s: f64) -> NodeChain {
    let layer = ConductionLayer { thickness_m: 0.1, conductivity_w_m_k: 1.13, volumetric_heat_capacity_j_m3k: 1400.0 * 1000.0 };
    let mut chain = NodeChain::default();
    chain.push_layer(&layer, time_step_s);
    chain
}

/// 🧪️ The finite-difference chain lumps exactly the layer's heat capacity and conductance, and is
/// divided finely enough that each sub-layer is no thicker than one step's penetration depth.
#[test]
fn node_chain_conserves_layer_capacity_and_resistance() {
    let chain = concrete_chain(600.0);
    assert_eq!(chain.nodes(), NodeChain::node_count(&[ConductionLayer { thickness_m: 0.1, conductivity_w_m_k: 1.13, volumetric_heat_capacity_j_m3k: 1.4e6 }], 600.0));
    assert!(chain.nodes() > 3, "a 10 cm slab at 10 min must be sub-divided, had {} nodes", chain.nodes());
    assert!((chain.capacitance_j_m2k.iter().sum::<f64>() - 0.1 * 1.4e6).abs() < 1e-6);
    assert!((chain.conductance_w_m2k() - 11.3).abs() < 1e-9);
}

/// 🧪️ Thermal mass defect: the implicit chain must store heat. With an adiabatic outside and the
/// inside face held 10 K warmer, the energy that entered equals `Σ C·ΔT`, the slab warms gradually
/// rather than instantly, and it settles at the inside temperature without oscillating.
#[test]
fn implicit_chain_stores_heat_and_settles_monotonically() {
    let step = 600.0;
    let chain = concrete_chain(step);
    let nodes = chain.nodes();
    let mut temperatures = vec![20.0; nodes];
    let (mut a, mut b) = (vec![0.0; nodes], vec![0.0; nodes]);
    let mut previous_outside = 20.0;
    let mut entered_j_m2 = 0.0;
    for index in 0..400 {
        let before = temperatures.clone();
        eliminate_chain(&chain.conductance_w_m2k, &chain.capacitance_j_m2k, &before, |_| 0.0, step, 0.0, 0.0, &mut a, &mut b);
        let inside_neighbour = |t: &[f64]| t[nodes - 2];
        substitute_chain(&a, &b, 30.0, &mut temperatures);
        let last = nodes - 1;
        entered_j_m2 += step * (chain.conductance_w_m2k[last - 1] * (30.0 - inside_neighbour(&temperatures)) + chain.capacitance_j_m2k[last] * (30.0 - before[last]) / step);
        if index == 0 {
            assert!(temperatures[0] < 21.0, "the adiabatic face must lag, was {} °C", temperatures[0]);
        }
        assert!(temperatures[0] >= previous_outside - 1e-9 && temperatures[0] <= 30.0 + 1e-9);
        previous_outside = temperatures[0];
    }
    let stored: f64 = chain.capacitance_j_m2k.iter().zip(&temperatures).map(|(c, t)| c * (t - 20.0)).sum();
    assert!((entered_j_m2 - stored).abs() / stored < 1e-6, "entered {entered_j_m2} J/m², stored {stored} J/m²");
    assert!((temperatures[0] - 30.0).abs() < 1e-3);
}
