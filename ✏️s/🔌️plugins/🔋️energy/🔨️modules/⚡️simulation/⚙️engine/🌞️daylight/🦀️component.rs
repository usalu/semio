//! 💡️ Daylight: reference points, illuminance, glare, and lighting control.

// #region 🔖️Types
/// 💡️ Daylight zone with reference points and glazing coupling.
#[derive(Clone, Debug, PartialEq)]
pub struct DaylightZone {
    pub zone_id: u32,
    pub floor_area_m2: f64,
    pub window_transmittance: f64,
    pub reference_points: Vec<ReferencePoint>,
    pub illuminance_target_lux: f64,
    pub glare_limit: f64,
}

/// 📍️ Interior daylight reference point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferencePoint {
    pub x_m: f64,
    pub y_m: f64,
    pub z_m: f64,
    pub fraction: f64,
}

/// 🗺️ Illuminance map at reference points [lux].
#[derive(Clone, Debug, PartialEq)]
pub struct IlluminanceMap {
    pub values_lux: Vec<f64>,
    pub min_lux: f64,
    pub max_lux: f64,
    pub average_lux: f64,
}
// #endregion 🔖️Types

// #region 🔖️Illuminance
/// 💡️ Simplified interior illuminance at a point [lux] (split-flux daylight factor).
pub fn reference_point_illuminance_lux(diffuse_horizontal_lux: f64, direct_normal_lux: f64, incidence_cosine: f64, window_transmittance: f64, daylight_factor: f64, shading_factor: f64) -> f64 {
    let diffuse_contrib = diffuse_horizontal_lux * window_transmittance * daylight_factor * shading_factor;
    let direct_contrib = direct_normal_lux * incidence_cosine * window_transmittance * shading_factor * 0.5;
    diffuse_contrib + direct_contrib
}

/// 🗺️ Build illuminance map from reference points.
pub fn illuminance_map(points: &[ReferencePoint], lux_per_point: &[f64]) -> IlluminanceMap {
    let values_lux: Vec<f64> = points.iter().zip(lux_per_point.iter()).map(|(p, &lux)| lux * p.fraction).collect();
    let empty = values_lux.is_empty();
    let min_lux = values_lux.iter().copied().fold(f64::INFINITY, f64::min);
    let max_lux = values_lux.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let total_frac: f64 = points.iter().map(|p| p.fraction).sum();
    let average_lux = if total_frac > 0.0 { values_lux.iter().sum::<f64>() / total_frac } else { 0.0 };
    IlluminanceMap { values_lux, min_lux: if empty { 0.0 } else { min_lux }, max_lux: if empty { 0.0 } else { max_lux }, average_lux }
}

/// 💡️ Zone-averaged daylight illuminance [lux].
pub fn zone_daylight_illuminance(zone: &DaylightZone, lux_per_point: &[f64]) -> f64 {
    illuminance_map(&zone.reference_points, lux_per_point).average_lux
}
// #endregion 🔖️Illuminance

// #region 🔖️Glare
/// 😎️ Simplified daylight glare index (0–1, higher = more glare).
pub fn simplified_glare_index(window_luminance_cd_m2: f64, solid_angle_sr: f64, eye_illuminance_lux: f64) -> f64 {
    let omega = solid_angle_sr.max(1e-6);
    let l_b = window_luminance_cd_m2.max(1.0);
    let e_i = eye_illuminance_lux.max(1.0);
    let ratio = l_b * omega.sqrt() / e_i;
    (ratio / (ratio + 10.0)).clamp(0.0, 1.0)
}

/// 😎️ Glare acceptable when index below limit.
pub fn glare_acceptable(glare_index: f64, limit: f64) -> bool {
    glare_index <= limit
}
// #endregion 🔖️Glare

// #region 🔖️Control
/// 💡️ Continuous lighting dimming fraction (0 = off, 1 = full) for daylight harvesting.
pub fn lighting_dimming_fraction(current_illuminance_lux: f64, target_lux: f64, min_fraction: f64) -> f64 {
    if target_lux <= 0.0 {
        return 1.0;
    }
    if current_illuminance_lux >= target_lux {
        return min_fraction;
    }
    let needed = (target_lux - current_illuminance_lux) / target_lux;
    needed.clamp(min_fraction, 1.0)
}

/// 💡️ Electric lighting power after dimming [W].
pub fn dimmed_lighting_power_w(full_power_w: f64, dimming_fraction: f64) -> f64 {
    full_power_w * dimming_fraction.clamp(0.0, 1.0)
}

/// 💡️ Daylight factor from geometry (simplified: window-to-floor ratio).
pub fn simplified_daylight_factor(window_area_m2: f64, floor_area_m2: f64, transmittance: f64) -> f64 {
    if floor_area_m2 <= 0.0 {
        return 0.0;
    }
    0.5 * (window_area_m2 / floor_area_m2) * transmittance
}
// #endregion 🔖️Control

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_zone() -> DaylightZone {
        DaylightZone {
            zone_id: 1,
            floor_area_m2: 25.0,
            window_transmittance: 0.6,
            reference_points: vec![ReferencePoint { x_m: 2.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }, ReferencePoint { x_m: 4.0, y_m: 2.0, z_m: 0.8, fraction: 0.5 }],
            illuminance_target_lux: 500.0,
            glare_limit: 0.4,
        }
    }

    #[semio_framework_async_macros::async_test]
    fn illuminance_increases_with_sun() {
        let e = reference_point_illuminance_lux(10_000.0, 50_000.0, 0.5, 0.6, 0.05, 1.0);
        assert!(e > 500.0);
    }

    #[semio_framework_async_macros::async_test]
    fn dimming_reduces_at_high_daylight() {
        let frac = lighting_dimming_fraction(600.0, 500.0, 0.1);
        assert!((frac - 0.1).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn dimming_full_when_dark() {
        let frac = lighting_dimming_fraction(50.0, 500.0, 0.1);
        assert!(frac > 0.8);
    }

    #[semio_framework_async_macros::async_test]
    fn glare_high_for_bright_window() {
        let gi = simplified_glare_index(5000.0, 0.2, 300.0);
        assert!(gi > 0.1);
    }

    #[semio_framework_async_macros::async_test]
    fn zone_average_illuminance() {
        let zone = sample_zone();
        let lux = vec![400.0, 600.0];
        let avg = zone_daylight_illuminance(&zone, &lux);
        assert!((avg - 500.0).abs() < 1e-6);
    }
}
