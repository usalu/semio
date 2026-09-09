use super::*;

fn double_glazing() -> WindowModel {
    WindowModel {
        glazing_layers: vec![
            GlazingLayer { thickness_m: 0.004, conductivity_w_m_k: 0.9, solar_transmittance: 0.82, solar_reflectance: 0.08, visible_transmittance: 0.88, ir_emissivity: 0.84 },
            GlazingLayer { thickness_m: 0.004, conductivity_w_m_k: 0.9, solar_transmittance: 0.74, solar_reflectance: 0.12, visible_transmittance: 0.80, ir_emissivity: 0.84 },
        ],
        gap_resistance_m2k_w: vec![0.15],
        frame_fraction: 0.15,
        frame_u_value_w_m2k: 2.5,
        divider_fraction: 0.05,
        divider_conductance_w_k: 0.5,
        interior_shade: ShadeState::OPEN,
        exterior_shade: ShadeState::OPEN,
    }
}

#[test]
fn double_glazing_u_below_single() {
    let win = double_glazing();
    let u_double = win.center_u_value_w_m2k();
    let single = WindowModel { glazing_layers: vec![win.glazing_layers[0]], gap_resistance_m2k_w: vec![], ..win.clone() };
    assert!(u_double < single.center_u_value_w_m2k());
}

#[test]
fn shade_reduces_shgc() {
    let mut win = double_glazing();
    win.interior_shade = ShadeState { deployed: true, solar_transmittance: 0.1, solar_reflectance: 0.5, visible_transmittance: 0.1, ir_transmittance: 0.2 };
    assert!(win.center_shgc() < 0.2);
}

#[test]
fn conduction_cold_outside_negative_into_zone() {
    let q = window_conduction_w(-10.0, 20.0, 1.2, 2.0);
    assert!(q < 0.0);
}

#[test]
fn condensation_when_surface_cold() {
    let risk = condensation_risk(5.0, 22.0, 0.012, 101_325.0);
    assert!(matches!(risk, CondensationRisk::Condensing | CondensationRisk::Risk { .. }));
}
