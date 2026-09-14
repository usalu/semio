//! 💧️ Physical property functions: moist air, water, steam, refrigerants, glycol.

use crate::units::{c_to_k, CP_DRY_AIR, H_FG_0C, P_STD, R_DRY_AIR, R_WATER_VAPOR};

// #region 🔖️Psychrometrics
/// 💧️ Saturation pressure of water [Pa] over ice below 0 °C and over liquid water above, from the
/// Hyland–Wexler correlations (ASHRAE Handbook — Fundamentals, Psychrometrics, eqs. 5–6).
pub fn saturation_pressure_pa(t_c: f64) -> f64 {
    let t = c_to_k(t_c.clamp(-100.0, 200.0));
    if t_c < 0.0 {
        (-5.674_535_9e3 / t + 6.392_524_7 - 9.677_843e-3 * t + 6.221_570_1e-7 * t * t + 2.074_782_5e-9 * t.powi(3) - 9.484_024e-13 * t.powi(4) + 4.163_501_9 * t.ln()).exp()
    } else {
        (-5.800_220_6e3 / t + 1.391_499_3 - 4.864_023_9e-2 * t + 4.176_476_8e-5 * t * t - 1.445_209_3e-8 * t.powi(3) + 6.545_967_3 * t.ln()).exp()
    }
}

/// 💧️ Humidity ratio W [kg_water/kg_dry_air] of air whose dew point is `dew_point_c`.
pub fn humidity_ratio_from_dew_point(dew_point_c: f64, p_atm: f64) -> f64 {
    let p_w = saturation_pressure_pa(dew_point_c);
    0.621_945 * p_w / (p_atm - p_w).max(1.0)
}

/// 🔥️ Moist air specific heat [J/(kg_dry_air·K)] at humidity ratio `w`.
pub fn moist_air_cp_j_per_kg_k(w: f64) -> f64 {
    1.004_84e3 + 1.858_95e3 * w
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

/// 🌡️ Thermodynamic wet-bulb temperature [°C] by bisection on the psychrometric energy balance
/// (ASHRAE Handbook — Fundamentals, Psychrometrics, eqs. 33 and 35).
pub fn wet_bulb_c(t_db_c: f64, w: f64, p_atm: f64) -> f64 {
    let implied_humidity_ratio = |t_wb: f64| {
        let w_s = humidity_ratio_from_rh(t_wb, 1.0, p_atm);
        if t_wb >= 0.0 {
            ((2501.0 - 2.326 * t_wb) * w_s - 1.006 * (t_db_c - t_wb)) / (2501.0 + 1.86 * t_db_c - 4.186 * t_wb)
        } else {
            ((2830.0 - 0.24 * t_wb) * w_s - 1.006 * (t_db_c - t_wb)) / (2830.0 + 1.86 * t_db_c - 2.1 * t_wb)
        }
    };
    let (mut low, mut high) = (-100.0_f64, t_db_c);
    for _ in 0..60 {
        let middle = 0.5 * (low + high);
        if implied_humidity_ratio(middle) > w {
            high = middle;
        } else {
            low = middle;
        }
    }
    0.5 * (low + high)
}

/// 🔥️ Moist air enthalpy [J/kg dry air].
pub fn moist_air_enthalpy_j_per_kg(t_c: f64, w: f64) -> f64 {
    CP_DRY_AIR * t_c + w * (H_FG_0C + 1860.0 * t_c)
}

/// 🌡️ Dew point [°C] from humidity ratio.
pub fn dew_point_c(w: f64, p_atm: f64) -> f64 {
    let p_w = w.max(0.0) * p_atm / (0.621_945 + w.max(0.0));
    let (mut low, mut high) = (-100.0_f64, 200.0_f64);
    for _ in 0..60 {
        let middle = 0.5 * (low + high);
        if saturation_pressure_pa(middle) > p_w {
            high = middle;
        } else {
            low = middle;
        }
    }
    0.5 * (low + high)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
