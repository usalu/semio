//! ☀️ Solar incidence, shading, and absorbed solar on surfaces and windows.

use crate::geometry::{polygon_normal, surface_tilt_azimuth};
use crate::units::deg_to_rad;

// #region 🔖️Types
/// ☀️ Interior solar distribution mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteriorSolarDistribution {
    DirectToFloor,
    UniformOnSurfaces,
    SplitFlux,
}

/// ☀️ Solar heat absorbed on a surface [W/m²].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceSolarAbsorption {
    pub beam_w_m2: f64,
    pub diffuse_w_m2: f64,
    pub total_w_m2: f64,
}
// #endregion 🔖️Types

// #region 🔖️Incidence
/// ☀️ Cosine of beam incidence angle (0–1).
pub async fn beam_incidence_cosine(surface_normal: [f64; 3], sun_altitude_deg: f64, sun_azimuth_deg: f64) -> f64 {
    let alt = deg_to_rad(sun_altitude_deg);
    let az = deg_to_rad(sun_azimuth_deg);
    let sun_dir = [alt.cos() * az.sin(), alt.cos() * az.cos(), alt.sin()];
    let mut n = surface_normal;
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-9 {
        n = [n[0] / len, n[1] / len, n[2] / len];
    }
    let cos_theta = n[0] * sun_dir[0] + n[1] * sun_dir[1] + n[2] * sun_dir[2];
    cos_theta.max(0.0)
}

/// ☀️ Sun direction unit vector from altitude/azimuth (Z up, Y north).
pub async fn sun_direction(sun_altitude_deg: f64, sun_azimuth_deg: f64) -> [f64; 3] {
    let alt = deg_to_rad(sun_altitude_deg);
    let az = deg_to_rad(sun_azimuth_deg);
    [alt.cos() * az.sin(), alt.cos() * az.cos(), alt.sin()]
}
// #endregion 🔖️Incidence

// #region 🔖️Shading
/// 🌳️ Shading factor (0 = fully shaded, 1 = unshaded).
pub async fn shading_factor(unshaded_fraction: f64, overhang_depth_m: f64, window_height_m: f64, sun_altitude_deg: f64) -> f64 {
    let base = unshaded_fraction.clamp(0.0, 1.0);
    if overhang_depth_m <= 0.0 || window_height_m <= 0.0 || sun_altitude_deg <= 1.0 {
        return base;
    }
    let alt = deg_to_rad(sun_altitude_deg);
    let shadow_fraction = (overhang_depth_m / window_height_m * alt.tan()).clamp(0.0, 1.0);
    base * (1.0 - shadow_fraction)
}
// #endregion 🔖️Shading

// #region 🔖️Absorption
/// ☀️ Absorbed solar on opaque surface [W/m²].
pub async fn surface_solar_absorption(direct_normal_irradiance_w_m2: f64, diffuse_horizontal_irradiance_w_m2: f64, incidence_cosine: f64, shading: f64, solar_absorptance: f64, tilt_deg: f64) -> SurfaceSolarAbsorption {
    let tilt_rad = deg_to_rad(tilt_deg);
    let view_factor_sky = (1.0 + tilt_rad.cos()) * 0.5;
    let beam = direct_normal_irradiance_w_m2 * incidence_cosine * shading * solar_absorptance;
    let diffuse = diffuse_horizontal_irradiance_w_m2 * view_factor_sky * solar_absorptance;
    SurfaceSolarAbsorption { beam_w_m2: beam, diffuse_w_m2: diffuse, total_w_m2: beam + diffuse }
}

/// ☀️ Absorbed solar from polygon vertices and sun position.
pub async fn surface_solar_from_vertices(
    vertices_m: &[[f64; 3]],
    north_axis_deg: f64,
    sun_altitude_deg: f64,
    sun_azimuth_deg: f64,
    direct_normal_irradiance_w_m2: f64,
    diffuse_horizontal_irradiance_w_m2: f64,
    shading: f64,
    solar_absorptance: f64,
) -> SurfaceSolarAbsorption {
    let normal = polygon_normal(vertices_m);
    let tilt = surface_tilt_azimuth(normal, north_axis_deg);
    let cos_inc = beam_incidence_cosine(normal, sun_altitude_deg, sun_azimuth_deg);
    surface_solar_absorption(direct_normal_irradiance_w_m2, diffuse_horizontal_irradiance_w_m2, cos_inc, shading, solar_absorptance, tilt.tilt_deg)
}
// #endregion 🔖️Absorption

// #region 🔖️Distribution
/// 💡️ Distribute transmitted solar to interior surfaces [W] per mode.
pub async fn distribute_interior_solar(transmitted_solar_w: f64, mode: InteriorSolarDistribution, floor_area_m2: f64, surface_areas_m2: &[f64]) -> Vec<f64> {
    match mode {
        InteriorSolarDistribution::DirectToFloor => {
            let mut out = vec![0.0; surface_areas_m2.len()];
            if !surface_areas_m2.is_empty() && floor_area_m2 > 0.0 {
                out[0] = transmitted_solar_w;
            }
            out
        }
        InteriorSolarDistribution::UniformOnSurfaces => {
            let total: f64 = surface_areas_m2.iter().sum();
            if total <= 0.0 {
                return vec![0.0; surface_areas_m2.len()];
            }
            surface_areas_m2.iter().map(|&a| transmitted_solar_w * a / total).collect()
        }
        InteriorSolarDistribution::SplitFlux => {
            let total: f64 = surface_areas_m2.iter().sum();
            let floor_share = 0.4;
            let mut out = vec![0.0; surface_areas_m2.len()];
            if !out.is_empty() {
                out[0] = transmitted_solar_w * floor_share;
            }
            let wall_share = transmitted_solar_w * (1.0 - floor_share);
            if total > 0.0 {
                for (i, a) in surface_areas_m2.iter().enumerate().skip(1) {
                    out[i] = wall_share * a / total;
                }
            }
            out
        }
    }
}
// #endregion 🔖️Distribution

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn vertical_south_wall_noon_incidence() {
        let cos = beam_incidence_cosine([0.0, -1.0, 0.0], 60.0, 180.0);
        assert!(cos > 0.4);
    }

    #[semio_framework_async_macros::async_test]
    async fn shading_reduces_with_overhang() {
        let unshaded = shading_factor(1.0, 0.0, 1.5, 45.0);
        let shaded = shading_factor(1.0, 0.8, 1.5, 45.0);
        assert!(shaded < unshaded);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorption_positive_at_noon() {
        let abs = surface_solar_absorption(800.0, 100.0, 0.8, 1.0, 0.6, 90.0);
        assert!(abs.total_w_m2 > 100.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn split_flux_allocates_floor_share() {
        let areas = vec![20.0, 10.0, 10.0];
        let dist = distribute_interior_solar(1000.0, InteriorSolarDistribution::SplitFlux, 20.0, &areas);
        assert!((dist[0] - 400.0).abs() < 1e-6);
    }
}
