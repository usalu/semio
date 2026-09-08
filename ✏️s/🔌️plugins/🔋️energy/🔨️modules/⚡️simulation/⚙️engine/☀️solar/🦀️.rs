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
pub fn beam_incidence_cosine(surface_normal: [f64; 3], sun_altitude_deg: f64, sun_azimuth_deg: f64) -> f64 {
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
pub fn sun_direction(sun_altitude_deg: f64, sun_azimuth_deg: f64) -> [f64; 3] {
    let alt = deg_to_rad(sun_altitude_deg);
    let az = deg_to_rad(sun_azimuth_deg);
    [alt.cos() * az.sin(), alt.cos() * az.cos(), alt.sin()]
}
// #endregion 🔖️Incidence

// #region 🔖️Shading
/// 🌳️ Shading factor (0 = fully shaded, 1 = unshaded).
pub fn shading_factor(unshaded_fraction: f64, overhang_depth_m: f64, window_height_m: f64, sun_altitude_deg: f64) -> f64 {
    let base = unshaded_fraction.clamp(0.0, 1.0);
    if overhang_depth_m <= 0.0 || window_height_m <= 0.0 || sun_altitude_deg <= 1.0 {
        return base;
    }
    let alt = deg_to_rad(sun_altitude_deg);
    let shadow_fraction = (overhang_depth_m / window_height_m * alt.tan()).clamp(0.0, 1.0);
    base * (1.0 - shadow_fraction)
}

/// 🪟️ Geometry of the projections attached to one window, in the glazing plane's own frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowProjections {
    pub width_m: f64,
    pub height_m: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
}

impl WindowProjections {
    /// 🪟️ True when nothing is attached, so the caller can skip the whole computation.
    pub fn is_unshaded(&self) -> bool {
        self.overhang_depth_m <= 0.0 && self.fin_depth_m <= 0.0
    }
}

/// 🌳️ Sunlit fraction of the beam component and the surviving fraction of the sky-diffuse
/// component for a window carrying an overhang and/or a symmetric pair of fins.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowShading {
    pub beam_sunlit_fraction: f64,
    pub diffuse_sky_fraction: f64,
}

/// 🌳️ Overhang/fin shading for one window — the profile-angle construction EnergyPlus's
/// `Shading:Overhang:Projection`/`Shading:Fin:Projection` describe geometrically.
///
/// Beam: the wall-solar azimuth `γ = sun_azimuth − surface_azimuth` gives the vertical profile
/// angle through `tan Ω = tan(altitude) / cos γ`; the overhang casts `depth · tan Ω` down the wall
/// and each fin casts `depth · tan |γ|` across it, both reduced by their offset from the head and
/// the jamb. The two shadows are treated as independent rectangular strips, so the sunlit fraction
/// is the product of the sunlit height and width fractions — exact for a rectangular window with a
/// full-width overhang and full-height fins, which is the ANSI/ASHRAE 140 §5.2 610/630/910/930
/// geometry.
///
/// Diffuse: each projection is treated as an INFINITE strip seen from the window centre. For a
/// vertical surface the sky view factor between two elevation angles is `(sin θ₂ − sin θ₁)/2`, so an
/// overhang whose outer edge sits at height `y` and depth `d` blocks everything above
/// `atan(y/d)` and the surviving share of the window's own sky view is `y/√(d² + y²)`; the fins give
/// the same expression in azimuth with `x = fin_offset + width/2`. Exact for an infinite strip, an
/// approximation for a real finite projection, and the honest limit of this model:
/// ground-reflected diffuse is not attenuated at all here because the caller's sky model has no
/// separate ground term.
pub fn window_shading(projections: &WindowProjections, surface_azimuth_deg: f64, sun_altitude_deg: f64, sun_azimuth_deg: f64) -> WindowShading {
    let height = projections.height_m.max(1e-6);
    let width = projections.width_m.max(1e-6);
    let overhang = projections.overhang_depth_m.max(0.0);
    let fin = projections.fin_depth_m.max(0.0);
    let overhang_edge_height = projections.overhang_offset_m.max(0.0) + height * 0.5;
    let fin_edge_offset = projections.fin_offset_m.max(0.0) + width * 0.5;
    let diffuse_sky_fraction = (overhang_edge_height / (overhang * overhang + overhang_edge_height * overhang_edge_height).sqrt()) * (fin_edge_offset / (fin * fin + fin_edge_offset * fin_edge_offset).sqrt());
    if sun_altitude_deg <= 0.0 {
        return WindowShading { beam_sunlit_fraction: 0.0, diffuse_sky_fraction };
    }
    let mut relative_azimuth = (sun_azimuth_deg - surface_azimuth_deg).rem_euclid(360.0);
    if relative_azimuth > 180.0 {
        relative_azimuth -= 360.0;
    }
    let gamma = deg_to_rad(relative_azimuth);
    let cos_gamma = gamma.cos();
    if cos_gamma <= 1e-6 {
        return WindowShading { beam_sunlit_fraction: 0.0, diffuse_sky_fraction };
    }
    let profile_tangent = deg_to_rad(sun_altitude_deg).tan() / cos_gamma;
    let overhang_shadow = (overhang * profile_tangent - projections.overhang_offset_m.max(0.0)).clamp(0.0, height);
    let fin_shadow = (fin * gamma.abs().tan() - projections.fin_offset_m.max(0.0)).clamp(0.0, width);
    WindowShading { beam_sunlit_fraction: ((height - overhang_shadow) / height) * ((width - fin_shadow) / width), diffuse_sky_fraction }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window-shading/🦀️.rs"]
mod window_shading_tests;
// #endregion 🔖️Shading

// #region 🔖️Absorption
/// ☀️ Absorbed solar on opaque surface [W/m²].
pub fn surface_solar_absorption(direct_normal_irradiance_w_m2: f64, diffuse_horizontal_irradiance_w_m2: f64, incidence_cosine: f64, shading: f64, solar_absorptance: f64, tilt_deg: f64) -> SurfaceSolarAbsorption {
    let tilt_rad = deg_to_rad(tilt_deg);
    let view_factor_sky = (1.0 + tilt_rad.cos()) * 0.5;
    let beam = direct_normal_irradiance_w_m2 * incidence_cosine * shading * solar_absorptance;
    let diffuse = diffuse_horizontal_irradiance_w_m2 * view_factor_sky * solar_absorptance;
    SurfaceSolarAbsorption { beam_w_m2: beam, diffuse_w_m2: diffuse, total_w_m2: beam + diffuse }
}

/// ☀️ Absorbed solar from polygon vertices and sun position.
pub fn surface_solar_from_vertices(
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
pub fn distribute_interior_solar(transmitted_solar_w: f64, mode: InteriorSolarDistribution, floor_area_m2: f64, surface_areas_m2: &[f64]) -> Vec<f64> {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
