//! 💧️ Physical property functions: moist air, water, steam, refrigerants, glycol.

use crate::num::newton_raphson;
use crate::units::{c_to_k, CP_DRY_AIR, H_FG_0C, P_STD, R_DRY_AIR, R_WATER_VAPOR};

// #region 🔖️Psychrometrics
/// 💧️ Saturation pressure of water [Pa] (Magnus-type, valid ~0–50°C).
pub fn saturation_pressure_pa(t_c: f64) -> f64 {
    let t = t_c.clamp(-50.0, 100.0);
    611.657 * ((17.2799 * t) / (t + 237.3)).exp()
}

/// 💧️ Humidity ratio W [kg_water/kg_dry_air] from dry-bulb and relative humidity.
pub fn humidity_ratio_from_rh(t_c: f64, rh: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    let p_w = rh.clamp(0.0, 1.0) * p_ws;
    0.621_945 * p_w / (p_atm - p_w).max(1.0)
}

/// 💧️ Relative humidity from humidity ratio.
pub fn rh_from_humidity_ratio(t_c: f64, w: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    let p_w = w * p_atm / (0.621_945 + w);
    (p_w / p_ws).clamp(0.0, 1.0)
}

/// 🌡️ Wet-bulb temperature [°C] via iterative psychrometric balance.
pub fn wet_bulb_c(t_db_c: f64, w: f64, p_atm: f64) -> f64 {
    let target = w;
    let f = |t_wb: f64| humidity_ratio_from_rh(t_wb, 1.0, p_atm) - target;
    let df = |t_wb: f64| {
        let eps = 0.01;
        (f(t_wb + eps) - f(t_wb - eps)) / (2.0 * eps)
    };
    newton_raphson(t_db_c, f, df, 30, 1e-6).unwrap_or(t_db_c)
}

/// 🔥️ Moist air enthalpy [J/kg dry air].
pub fn moist_air_enthalpy_j_per_kg(t_c: f64, w: f64) -> f64 {
    CP_DRY_AIR * t_c + w * (H_FG_0C + 1860.0 * t_c)
}

/// 🌡️ Dew point [°C] from humidity ratio.
pub fn dew_point_c(w: f64, p_atm: f64) -> f64 {
    let p_w = w * p_atm / (0.621_945 + w);
    let ln_pw = (p_w / 611.657).ln();
    237.3 * ln_pw / (17.2799 - ln_pw)
}

/// 💨️ Moist air density [kg/m³].
pub fn moist_air_density(t_c: f64, w: f64, p_atm: f64) -> f64 {
    let t_k = c_to_k(t_c);
    let p_w = w * p_atm / (0.621_945 + w);
    let p_d = p_atm - p_w;
    p_d / (R_DRY_AIR * t_k) + p_w / (R_WATER_VAPOR * t_k)
}
// #endregion 🔖️Psychrometrics

// #region 🔖️Water
/// 💧️ Liquid water specific heat [J/(kg·K)] (temperature-dependent polynomial).
pub fn water_cp_j_per_kg_k(t_c: f64) -> f64 {
    4217.0 - 1.2 * t_c + 0.003 * t_c * t_c
}

/// 💧️ Liquid water density [kg/m³].
pub fn water_density(t_c: f64) -> f64 {
    999.839_5 + 0.067_37 * t_c - 0.010_52 * t_c * t_c
}

/// 💧️ Liquid water thermal conductivity [W/(m·K)].
pub fn water_conductivity(t_c: f64) -> f64 {
    0.561_0 + 0.002_0 * t_c - 6.0e-6 * t_c * t_c
}
// #endregion 🔖️Water

// #region 🔖️Steam
/// 💨️ Steam saturation temperature [°C] from pressure [Pa].
pub fn steam_saturation_temp_c(p_pa: f64) -> f64 {
    let ln_p = (p_pa / 611.657).ln();
    237.3 * ln_p / (17.2799 - ln_p)
}

/// 💨️ Latent heat of vaporization [J/kg] at temperature [°C].
pub fn latent_heat_vaporization(t_c: f64) -> f64 {
    H_FG_0C - 2370.0 * t_c
}
// #endregion 🔖️Steam

// #region 🔖️Refrigerant
/// ❄️ R410A saturation pressure [Pa] simplified correlation (valid ~-40 to 40°C).
pub fn r410a_saturation_pressure_pa(t_c: f64) -> f64 {
    let t_k = c_to_k(t_c);
    let a = -1031.0;
    let b = 7.0;
    (a / t_k + b).exp() * P_STD
}

/// ❄️ R410A saturation temperature [°C] from pressure [Pa].
pub fn r410a_saturation_temp_c(p_pa: f64) -> f64 {
    let ratio = (p_pa / P_STD).ln();
    k_to_c(-1031.0 / (ratio - 7.0))
}

fn k_to_c(t_k: f64) -> f64 {
    t_k - 273.15
}
// #endregion 🔖️Refrigerant

// #region 🔖️Glycol
/// 🧪️ Glycol mixture specific heat [J/(kg·K)] (ethylene glycol fraction 0–0.6).
pub fn glycol_cp_j_per_kg_k(t_c: f64, glycol_fraction: f64) -> f64 {
    let f = glycol_fraction.clamp(0.0, 0.6);
    water_cp_j_per_kg_k(t_c) * (1.0 - f) + 2400.0 * f
}

/// 🧪️ Glycol mixture density [kg/m³].
pub fn glycol_density(t_c: f64, glycol_fraction: f64) -> f64 {
    let f = glycol_fraction.clamp(0.0, 0.6);
    water_density(t_c) * (1.0 - f) + 1110.0 * f
}

/// 🧪️ Glycol mixture dynamic viscosity [Pa·s] (simplified).
pub fn glycol_viscosity(t_c: f64, glycol_fraction: f64) -> f64 {
    let f = glycol_fraction.clamp(0.0, 0.6);
    let mu_water = 0.001_792 / (1.0 + 0.033_7 * t_c + 0.000_221 * t_c * t_c);
    mu_water * (1.0 + 5.0 * f)
}
// #endregion 🔖️Glycol

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturation_at_zero_c() {
        let p = saturation_pressure_pa(0.0);
        assert!((p - 611.657).abs() < 1.0);
    }

    #[test]
    fn rh_roundtrip() {
        let w = humidity_ratio_from_rh(25.0, 0.5, P_STD);
        let rh = rh_from_humidity_ratio(25.0, w, P_STD);
        assert!((rh - 0.5).abs() < 0.02);
    }

    #[test]
    fn water_density_near_room_temp() {
        assert!((water_density(20.0) - 998.0).abs() < 5.0);
    }
}
