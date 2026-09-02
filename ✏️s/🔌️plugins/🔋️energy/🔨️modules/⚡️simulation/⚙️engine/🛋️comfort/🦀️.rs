//! 😌️ Thermal comfort: PMV, PPD, operative temperature, MRT, adaptive models.

use crate::props::saturation_pressure_pa;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️ComfortInput
/// 🧍️ Inputs for comfort evaluation per ISO 7730 / ASHRAE 55.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ComfortInput {
    pub air_temp_c: f64,
    pub mean_radiant_temp_c: f64,
    pub air_speed_m_s: f64,
    pub relative_humidity: f64,
    pub metabolic_rate_met: f64,
    pub clothing_insulation_clo: f64,
    pub external_work_met: f64,
}
// #endregion 🔖️ComfortInput

// #region 🔖️AdaptiveComfort
/// 🌿️ Adaptive comfort standard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum AdaptiveStandard {
    Ashrae55,
    Cen15251,
}
// #endregion 🔖️AdaptiveComfort

// #region 🔖️OperativeTemp
/// 🌡️ Operative temperature [°C] from convective/radiant weighting.
pub fn operative_temp_c(air_temp_c: f64, mean_radiant_temp_c: f64, air_speed_m_s: f64) -> f64 {
    let v = air_speed_m_s.max(0.05);
    let hc = 2.38 * (air_temp_c - mean_radiant_temp_c).abs().powf(0.25).max(3.0);
    let hv = 12.1 * v.sqrt();
    let h = if v > 0.2 { hv } else { hc };
    let hr = 4.0;
    (h * air_temp_c + hr * mean_radiant_temp_c) / (h + hr)
}

/// ☀️ Mean radiant temperature [°C] from surface temperatures and view factors.
pub fn mean_radiant_temp_c(surface_temps_k: &[f64], view_factors: &[f64]) -> f64 {
    if surface_temps_k.is_empty() || surface_temps_k.len() != view_factors.len() {
        return 293.15 - 273.15;
    }
    let t4: f64 = surface_temps_k.iter().zip(view_factors.iter()).map(|(&t, &vf)| vf * (t / 100.0).powi(4)).sum();
    100.0 * t4.powf(0.25) - 273.15
}

/// ☀️ MRT from dry-bulb and enclosure delta-T approximation [°C].
pub fn mean_radiant_temp_from_enclosure(air_temp_c: f64, enclosure_delta_t_k: f64) -> f64 {
    air_temp_c + enclosure_delta_t_k
}
// #endregion 🔖️OperativeTemp

// #region 🔖️Pmv
/// 😌️ Predicted Mean Vote per ISO 7730 Fanger model.
pub fn pmv(input: &ComfortInput) -> f64 {
    let m = input.metabolic_rate_met * 58.15;
    let w = input.external_work_met * 58.15;
    let i_cl = 0.155 * input.clothing_insulation_clo;
    let f_cl = if i_cl <= 0.078 { 1.0 + 1.29 * i_cl } else { 1.05 + 0.645 * i_cl };
    let t_a = input.air_temp_c;
    let t_r = input.mean_radiant_temp_c;
    let v = input.air_speed_m_s.max(0.0);
    let p_a = input.relative_humidity.clamp(0.0, 1.0) * saturation_pressure_pa(t_a);
    let t_cl = solve_clothing_temp_c(t_a, t_r, m, w, i_cl, f_cl, v);
    let h_c = if v < 0.1 { 2.38 * (t_cl - t_a).abs().powf(0.25) } else { 12.1 * v.sqrt() };
    let t_cl_k = t_cl + 273.15;
    let t_r_k = t_r + 273.15;
    let e_r = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
    let e_c = f_cl * h_c * (t_cl - t_a);
    let e_sw = 3.05e-3 * (5733.0 - 6.99 * (m - w) - p_a).max(0.0);
    let e_diff = if m > 58.15 { 0.42 * (m - w - 58.15) } else { 0.0 };
    let e = e_sw + e_diff;
    let c_res = 1.7e-5 * m * (34.0 - t_a);
    let l = m - w - e - e_r - e_c - c_res;
    ((0.303 * (-0.035 * m).exp() + 0.028) * l).clamp(-3.0, 3.0)
}

fn solve_clothing_temp_c(t_a: f64, t_r: f64, m: f64, w: f64, i_cl: f64, f_cl: f64, v: f64) -> f64 {
    let mut t_cl = t_a + (35.5 - t_a) / (3.5 * i_cl + 1.0);
    for _ in 0..50 {
        let h_c = if v < 0.1 { 2.38 * (t_cl - t_a).abs().powf(0.25) } else { 12.1 * v.sqrt() };
        let t_cl_k = t_cl + 273.15;
        let t_r_k = t_r + 273.15;
        let rad = 3.96e-8 * f_cl * (t_cl_k.powi(4) - t_r_k.powi(4));
        let conv = f_cl * h_c * (t_cl - t_a);
        let t_new = 35.7 - 0.028 * (m - w) - i_cl * (rad + conv);
        if (t_new - t_cl).abs() < 0.001 {
            return t_new;
        }
        t_cl = t_new;
    }
    t_cl
}
// #endregion 🔖️Pmv

// #region 🔖️Ppd
/// 📊️ Predicted Percentage Dissatisfied from PMV.
pub fn ppd(pmv_value: f64) -> f64 {
    let pmv_c = pmv_value.clamp(-3.0, 3.0);
    100.0 - 95.0 * (-0.03353 * pmv_c.powi(4) - 0.2179 * pmv_c.powi(2)).exp()
}
// #endregion 🔖️Ppd

// #region 🔖️Adaptive
/// 🌿️ Adaptive comfort acceptable temperature range [°C] (lower, upper).
pub fn adaptive_comfort_range_c(standard: AdaptiveStandard, running_mean_outdoor_temp_c: f64, acceptability_class: u8) -> (f64, f64) {
    let t_rm = running_mean_outdoor_temp_c;
    let (center, band) = match standard {
        AdaptiveStandard::Ashrae55 => (0.31 * t_rm + 17.8, if acceptability_class <= 1 { 2.5 } else { 3.5 }),
        AdaptiveStandard::Cen15251 => (0.33 * t_rm + 18.8, if acceptability_class <= 1 { 2.0 } else { 3.0 }),
    };
    (center - band, center + band)
}

/// ✅️ Whether operative temperature is within adaptive comfort band.
pub fn adaptive_comfort_ok(standard: AdaptiveStandard, operative_temp_c: f64, running_mean_outdoor_temp_c: f64, acceptability_class: u8) -> bool {
    let (lo, hi) = adaptive_comfort_range_c(standard, running_mean_outdoor_temp_c, acceptability_class);
    operative_temp_c >= lo && operative_temp_c <= hi
}
// #endregion 🔖️Adaptive

// #region 🔖️RadiantAsymmetry
/// ☀️ Radiant asymmetry comfort limit check [°C] (ceiling vs floor).
pub fn radiant_asymmetry_ok(temp_high_c: f64, temp_low_c: f64, max_asymmetry_k: f64) -> bool {
    (temp_high_c - temp_low_c).abs() <= max_asymmetry_k
}
// #endregion 🔖️RadiantAsymmetry

#[cfg(test)]
mod tests {
    use super::*;

    fn neutral_input() -> ComfortInput {
        ComfortInput { air_temp_c: 22.0, mean_radiant_temp_c: 22.0, air_speed_m_s: 0.1, relative_humidity: 0.5, metabolic_rate_met: 1.0, clothing_insulation_clo: 0.5, external_work_met: 0.0 }
    }

    #[test]
    fn operative_equals_air_when_equal_mrt() {
        let top = operative_temp_c(22.0, 22.0, 0.1);
        assert!((top - 22.0).abs() < 0.5);
    }

    #[test]
    fn pmv_near_zero_at_neutral() {
        let p = pmv(&neutral_input());
        assert!(p.abs() < 1.5, "pmv={p}");
    }

    #[test]
    fn ppd_minimum_at_zero_pmv() {
        let p = ppd(0.0);
        assert!((p - 5.0).abs() < 1.0);
    }

    #[test]
    fn pmv_increases_when_too_warm() {
        let mut hot = neutral_input();
        hot.air_temp_c = 30.0;
        hot.mean_radiant_temp_c = 30.0;
        assert!(pmv(&hot) > pmv(&neutral_input()));
    }

    #[test]
    fn adaptive_range_centers_on_outdoor() {
        let (lo, hi) = adaptive_comfort_range_c(AdaptiveStandard::Ashrae55, 20.0, 1);
        let center = 0.5 * (lo + hi);
        assert!((center - (0.31 * 20.0 + 17.8)).abs() < 0.5);
    }

    #[test]
    fn mrt_from_surfaces() {
        let temps_k = [293.15, 303.15];
        let vfs = [0.5, 0.5];
        let mrt = mean_radiant_temp_c(&temps_k, &vfs);
        assert!(mrt > 20.0 && mrt < 30.0);
    }
}
