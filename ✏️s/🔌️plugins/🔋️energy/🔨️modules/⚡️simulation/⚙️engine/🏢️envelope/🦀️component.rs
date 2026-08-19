//! 🧱️ Opaque envelope heat transfer: convection, conduction CTF, and surface balance.

use crate::material::{R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
use crate::num::newton_raphson;
use crate::units::STEFAN_BOLTZMANN;

// #region 🔖️ConvectionModels
/// 🌬️ Exterior convection correlation (wind-adaptive McAdams-type).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExteriorConvectionModel {
    pub base_h_w_m2k: f64,
    pub wind_coefficient: f64,
}

impl Default for ExteriorConvectionModel {
    fn default() -> Self {
        Self { base_h_w_m2k: 5.7, wind_coefficient: 3.8 }
    }
}

impl ExteriorConvectionModel {
    /// 🌬️ Exterior convection coefficient [W/(m²·K)].
    pub async fn h_w_m2k(&self, wind_speed_m_s: f64) -> f64 {
        self.base_h_w_m2k + self.wind_coefficient * wind_speed_m_s.max(0.0)
    }
}

/// 🌬️ Interior convection correlation (adaptive natural convection).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InteriorConvectionModel {
    pub h_min_w_m2k: f64,
    pub delta_t_exponent: f64,
    pub delta_t_coefficient: f64,
}

impl Default for InteriorConvectionModel {
    fn default() -> Self {
        Self { h_min_w_m2k: 3.0, delta_t_coefficient: 5.1, delta_t_exponent: 0.25 }
    }
}

impl InteriorConvectionModel {
    /// 🌬️ Interior convection coefficient [W/(m²·K)] from |T_s − T_a|.
    pub async fn h_w_m2k(&self, surface_temp_c: f64, air_temp_c: f64) -> f64 {
        let dt = (surface_temp_c - air_temp_c).abs();
        self.h_min_w_m2k + self.delta_t_coefficient * dt.powf(self.delta_t_exponent)
    }
}
// #endregion 🔖️ConvectionModels

// #region 🔖️ConductionState
/// 🌡️ Simplified first-order CTF conduction state (one history state per surface).
#[derive(Clone, Debug, PartialEq)]
pub struct ConductionState {
    pub ctf_c0_w_m2k: f64,
    pub ctf_c1_w_m2k: f64,
    pub previous_outside_temp_c: f64,
}

impl ConductionState {
    /// 🌡️ Initialize CTF from construction U-value and thermal mass [J/(m²·K)].
    pub async fn from_u_and_capacitance(u_value_w_m2k: f64, capacitance_j_m2k: f64, time_step_s: f64) -> Self {
        let tau = capacitance_j_m2k / u_value_w_m2k.max(0.01);
        let alpha = (-time_step_s / tau.max(1.0)).exp();
        Self { ctf_c0_w_m2k: u_value_w_m2k * (1.0 - alpha), ctf_c1_w_m2k: u_value_w_m2k * alpha, previous_outside_temp_c: 20.0 }
    }

    /// 🔥️ Conduction heat flux to zone [W/m²] (positive = heat into zone).
    pub async fn heat_flux_w_m2(&self, outside_temp_c: f64, inside_temp_c: f64) -> f64 {
        self.ctf_c0_w_m2k * (outside_temp_c - inside_temp_c) + self.ctf_c1_w_m2k * (self.previous_outside_temp_c - inside_temp_c)
    }

    /// 🔄️ Advance history after a timestep.
    pub async fn advance(&mut self, outside_temp_c: f64) {
        self.previous_outside_temp_c = outside_temp_c;
    }
}
// #endregion 🔖️ConductionState

// #region 🔖️SurfaceHeatBalance
/// ⚖️ Surface heat balance terms [W/m²].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceHeatBalance {
    pub convection_w_m2: f64,
    pub conduction_w_m2: f64,
    pub solar_absorbed_w_m2: f64,
    pub longwave_net_w_m2: f64,
    pub surface_temp_c: f64,
}

impl SurfaceHeatBalance {
    /// ⚖️ Net flux into surface (should approach zero at convergence).
    pub async fn residual_w_m2(&self) -> f64 {
        self.solar_absorbed_w_m2 + self.longwave_net_w_m2 + self.conduction_w_m2 - self.convection_w_m2
    }
}
// #endregion 🔖️SurfaceHeatBalance

// #region 🔖️Longwave
/// 🌡️ Net longwave exchange [W/m²] (surface ↔ sky/ground).
pub async fn longwave_net_w_m2(surface_temp_c: f64, exterior_temp_k: f64, emissivity: f64) -> f64 {
    let t_s_k = surface_temp_c + 273.15;
    emissivity * STEFAN_BOLTZMANN * (exterior_temp_k.powi(4) - t_s_k.powi(4))
}
// #endregion 🔖️Longwave

// #region 🔖️Solve
/// 🌡️ Solve exterior surface temperature [°C] for heat balance.
pub async fn solve_exterior_surface_temp(outside_air_c: f64, sky_temp_k: f64, wind_speed_m_s: f64, solar_absorbed_w_m2: f64, conduction_from_inside_w_m2: f64, emissivity: f64, ext_conv: &ExteriorConvectionModel) -> f64 {
    let h = ext_conv.h_w_m2k(wind_speed_m_s);
    let f = |t_s: f64| {
        let conv = h * (outside_air_c - t_s);
        let lw = longwave_net_w_m2(t_s, sky_temp_k, emissivity);
        solar_absorbed_w_m2 + lw - conv - conduction_from_inside_w_m2
    };
    let df = |t_s: f64| {
        let eps = 0.1;
        (f(t_s + eps) - f(t_s - eps)) / (2.0 * eps)
    };
    newton_raphson(outside_air_c, f, df, 30, 1e-4).unwrap_or(outside_air_c)
}

/// 🌡️ Solve interior surface temperature [°C] for heat balance.
pub async fn solve_interior_surface_temp(zone_air_c: f64, conduction_from_outside_w_m2: f64, solar_absorbed_w_m2: f64, int_conv: &InteriorConvectionModel) -> SurfaceHeatBalance {
    let mut t_s = zone_air_c;
    for _ in 0..20 {
        let h = int_conv.h_w_m2k(t_s, zone_air_c);
        t_s = zone_air_c - (solar_absorbed_w_m2 + conduction_from_outside_w_m2) / h.max(0.1);
    }
    let h = int_conv.h_w_m2k(t_s, zone_air_c);
    SurfaceHeatBalance { convection_w_m2: h * (zone_air_c - t_s), conduction_w_m2: conduction_from_outside_w_m2, solar_absorbed_w_m2, longwave_net_w_m2: 0.0, surface_temp_c: t_s }
}

/// 🔥️ Steady-state opaque conduction flux [W/m²] through construction.
pub async fn steady_opaque_flux_w_m2(outside_temp_c: f64, inside_temp_c: f64, u_value_w_m2k: f64) -> f64 {
    u_value_w_m2k * (outside_temp_c - inside_temp_c)
}

/// 🔥️ Film-inclusive U-value from construction U and film resistances.
pub async fn overall_u_value_w_m2k(construction_u: f64) -> f64 {
    let r_total = 1.0 / construction_u.max(1e-6);
    let r_construction = r_total - R_FILM_INTERIOR_M2K_W - R_FILM_EXTERIOR_M2K_W;
    if r_construction <= 0.0 {
        return construction_u;
    }
    1.0 / (R_FILM_INTERIOR_M2K_W + r_construction + R_FILM_EXTERIOR_M2K_W)
}
// #endregion 🔖️Solve

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn exterior_h_increases_with_wind() {
        let model = ExteriorConvectionModel::default();
        assert!(model.h_w_m2k(5.0) > model.h_w_m2k(0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn interior_h_increases_with_delta_t() {
        let model = InteriorConvectionModel::default();
        assert!(model.h_w_m2k(30.0, 20.0) > model.h_w_m2k(21.0, 20.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn ctf_flux_sign_correct() {
        let state = ConductionState::from_u_and_capacitance(0.3, 50_000.0, 3600.0);
        let flux = state.heat_flux_w_m2(0.0, 20.0);
        assert!(flux < 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn steady_flux_cold_outside() {
        let q = steady_opaque_flux_w_m2(-5.0, 20.0, 0.25);
        assert!(q < 0.0);
        assert!((q - (-6.25)).abs() < 0.01);
    }

    #[semio_framework_async_macros::async_test]
    async fn interior_surface_balance_near_air() {
        let balance = solve_interior_surface_temp(22.0, -2.0, 0.0, &InteriorConvectionModel::default());
        assert!(balance.surface_temp_c > 22.0);
        assert!(balance.residual_w_m2().abs() < 0.1);
    }
}
