use super::*;

#[test]
fn exterior_h_increases_with_wind() {
    let model = ExteriorConvectionModel::default();
    assert!(model.h_w_m2k(5.0) > model.h_w_m2k(0.0));
}

#[test]
fn interior_h_increases_with_delta_t() {
    let model = InteriorConvectionModel::default();
    assert!(model.h_w_m2k(30.0, 20.0) > model.h_w_m2k(21.0, 20.0));
}

#[test]
fn ctf_flux_sign_correct() {
    let state = ConductionState::from_u_and_capacitance(0.3, 50_000.0, 3600.0);
    let flux = state.heat_flux_w_m2(0.0, 20.0);
    assert!(flux < 0.0);
}

#[test]
fn steady_flux_cold_outside() {
    let q = steady_opaque_flux_w_m2(-5.0, 20.0, 0.25);
    assert!(q < 0.0);
    assert!((q - (-6.25)).abs() < 0.01);
}

/// 🧪️ Heat LEAVING the zone through the wall must leave the inside face COLDER than the room
/// air — the direction the previous formula had backwards — while the energy leaving through
/// convection still equals the energy leaving through conduction.
#[test]
fn interior_surface_balance_near_air() {
    let losing = solve_interior_surface_temp(22.0, -2.0, 0.0, &InteriorConvectionModel::default());
    assert!(losing.surface_temp_c < 22.0, "surface was {} °C", losing.surface_temp_c);
    assert!((losing.convection_w_m2 - (-2.0)).abs() < 1e-6, "convection was {} W/m²", losing.convection_w_m2);
    assert!(losing.residual_w_m2().abs() < 0.1);

    let gaining = solve_interior_surface_temp(22.0, 2.0, 0.0, &InteriorConvectionModel::default());
    assert!(gaining.surface_temp_c > 22.0, "surface was {} °C", gaining.surface_temp_c);
    assert!((gaining.convection_w_m2 - 2.0).abs() < 1e-6, "convection was {} W/m²", gaining.convection_w_m2);
}
