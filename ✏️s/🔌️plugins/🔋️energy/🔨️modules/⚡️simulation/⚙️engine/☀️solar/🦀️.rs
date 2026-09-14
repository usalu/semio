//! ☀️ Solar geometry on surfaces: incidence, the Perez anisotropic sky, ground-reflected diffuse,
//! polygon shadows cast by obstructions, sky-diffuse shading ratios, and the beam patches a
//! window's sunlight paints on the inside of a convex zone.
//!
//! The irradiance split and shading follow the EnergyPlus "Shading Module" (Engineering
//! Reference): Perez (1990) circumsolar and horizon brightening with the full-precision
//! coefficients, isotropic/horizon diffuse shading from a 6 × 24 sky-patch integration, and beam
//! shadows by convex polygon clipping in the receiving plane.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️Polygon
/// 📐️ Largest vertex count a clipped polygon may carry.
pub const MAXIMUM_POLYGON_VERTICES: usize = 48;
const MAXIMUM_SUNLIT_PIECES: usize = 48;

/// 📐️ Allocation-free planar polygon in a receiving plane's own coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanePolygon {
    points: [[f64; 2]; MAXIMUM_POLYGON_VERTICES],
    len: usize,
}

impl PlanePolygon {
    const EMPTY: Self = Self { points: [[0.0; 2]; MAXIMUM_POLYGON_VERTICES], len: 0 };

    fn push(&mut self, point: [f64; 2]) -> bool {
        if self.len == MAXIMUM_POLYGON_VERTICES {
            return false;
        }
        self.points[self.len] = point;
        self.len += 1;
        true
    }

    fn signed_area(&self) -> f64 {
        (0..self.len).map(|i| {
            let (p, q) = (self.points[i], self.points[(i + 1) % self.len]);
            p[0] * q[1] - q[0] * p[1]
        }).sum::<f64>() * 0.5
    }

    /// 📏️ Unsigned area.
    pub fn area(&self) -> f64 {
        if self.len < 3 {
            0.0
        } else {
            self.signed_area().abs()
        }
    }

    fn counter_clockwise(mut self) -> Self {
        if self.signed_area() < 0.0 {
            self.points[..self.len].reverse();
        }
        self
    }

    fn clip(&self, a: [f64; 2], b: [f64; 2], keep_left: bool) -> Option<Self> {
        let side = |v: [f64; 2]| {
            let s = (b[0] - a[0]) * (v[1] - a[1]) - (b[1] - a[1]) * (v[0] - a[0]);
            if keep_left { s } else { -s }
        };
        let mut out = Self::EMPTY;
        for i in 0..self.len {
            let (p, q) = (self.points[i], self.points[(i + 1) % self.len]);
            let (sp, sq) = (side(p), side(q));
            if sp >= 0.0 && !out.push(p) {
                return None;
            }
            if (sp >= 0.0) != (sq >= 0.0) {
                let t = sp / (sp - sq);
                if !out.push([p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1])]) {
                    return None;
                }
            }
        }
        Some(out)
    }

    fn intersect_convex(&self, other: &Self) -> Option<Self> {
        let mut piece = *self;
        for i in 0..other.len {
            if piece.len < 3 {
                break;
            }
            piece = piece.clip(other.points[i], other.points[(i + 1) % other.len], true)?;
        }
        Some(piece)
    }
}

/// 🧭️ Orthonormal frame of a planar polygon: origin at its first vertex, `u` along its first edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneFrame {
    origin: [f64; 3],
    u: [f64; 3],
    v: [f64; 3],
    normal: [f64; 3],
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// ⚫️ Dot product.
pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn normalized(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    if length > 0.0 { [a[0] / length, a[1] / length, a[2] / length] } else { [0.0, 0.0, 1.0] }
}

impl PlaneFrame {
    /// 🧭️ Frame of `polygon` with outward `normal`.
    pub fn of(polygon: &[[f64; 3]], normal: [f64; 3]) -> Self {
        let origin = polygon[0];
        let u = normalized(sub(polygon[1 % polygon.len()], origin));
        Self { origin, u, v: cross(normal, u), normal }
    }

    fn project(&self, point: [f64; 3]) -> [f64; 2] {
        let d = sub(point, self.origin);
        [dot(d, self.u), dot(d, self.v)]
    }

    fn polygon(&self, polygon: &[[f64; 3]]) -> Option<PlanePolygon> {
        let mut out = PlanePolygon::EMPTY;
        for &point in polygon {
            if !out.push(self.project(point)) {
                return None;
            }
        }
        Some(out.counter_clockwise())
    }

    /// 🌑️ Shadow in this plane of `caster` lit from direction `sun`: the part of the caster in front
    /// of the plane, translated along the sun ray onto it. `None` when nothing lies in front.
    fn shadow(&self, caster: &[[f64; 3]], sun: [f64; 3]) -> Option<PlanePolygon> {
        let toward = dot(sun, self.normal);
        if toward <= 1e-9 {
            return None;
        }
        let height = |p: [f64; 3]| dot(sub(p, self.origin), self.normal);
        let mut out = PlanePolygon::EMPTY;
        let emit = |p: [f64; 3], out: &mut PlanePolygon| {
            let t = height(p) / toward;
            out.push(self.project([p[0] - t * sun[0], p[1] - t * sun[1], p[2] - t * sun[2]]))
        };
        for i in 0..caster.len() {
            let (p, q) = (caster[i], caster[(i + 1) % caster.len()]);
            let (hp, hq) = (height(p), height(q));
            if hp >= -1e-9 && !emit(p, &mut out) {
                return None;
            }
            if (hp >= -1e-9) != (hq >= -1e-9) {
                let t = hp / (hp - hq);
                if !emit([p[0] + t * (q[0] - p[0]), p[1] + t * (q[1] - p[1]), p[2] + t * (q[2] - p[2])], &mut out) {
                    return None;
                }
            }
        }
        (out.len >= 3 && out.area() > 1e-12).then(|| out.counter_clockwise())
    }
}
// #endregion 🔖️Polygon

// #region 🔖️Incidence
/// ☀️ Signed cosine of the angle between a surface's outward normal and the direction to the sun.
pub fn incidence_cosine(normal: [f64; 3], sun: [f64; 3]) -> f64 {
    dot(normal, sun)
}

/// 🌤️ Perez circumsolar (`f1`) and horizon (`f2`) brightening coefficients.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct PerezBrightening {
    pub circumsolar: f64,
    pub horizon: f64,
}

const PEREZ_EPSILON_LIMITS: [f64; 7] = [1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2];
const PEREZ_F11: [f64; 8] = [-0.0083117, 0.1299457, 0.3296958, 0.5682053, 0.8730280, 1.1326077, 1.0601591, 0.6777470];
const PEREZ_F12: [f64; 8] = [0.5877285, 0.6825954, 0.4868735, 0.1874525, -0.3920403, -1.2367284, -1.5999137, -0.3272588];
const PEREZ_F13: [f64; 8] = [-0.0620636, -0.1513752, -0.2210958, -0.2951290, -0.3616149, -0.4118494, -0.3589221, -0.2504286];
const PEREZ_F21: [f64; 8] = [-0.0596012, -0.0189325, 0.0554140, 0.1088631, 0.2255647, 0.2877813, 0.2642124, 0.1561313];
const PEREZ_F22: [f64; 8] = [0.0721249, 0.0659650, -0.0639588, -0.1519229, -0.4620442, -0.8230357, -1.1272340, -1.3765031];
const PEREZ_F23: [f64; 8] = [-0.0220216, -0.0288748, -0.0260542, -0.0139754, 0.0012448, 0.0558651, 0.1310694, 0.2506212];

/// 🌤️ Perez (1990) brightening coefficients from beam-normal and diffuse-horizontal irradiance,
/// the cosine of the solar zenith and the site elevation (for the relative air mass).
pub fn perez_brightening(beam_normal_w_m2: f64, diffuse_horizontal_w_m2: f64, cos_zenith: f64, elevation_m: f64) -> PerezBrightening {
    if diffuse_horizontal_w_m2 <= 0.0 || cos_zenith <= 0.0 {
        return PerezBrightening::default();
    }
    let zenith = cos_zenith.min(1.0).acos();
    let zenith_deg = zenith.to_degrees();
    let pressure_correction = 1.0 - 0.1 * elevation_m / 1000.0;
    let air_mass = if zenith_deg <= 75.0 { pressure_correction / cos_zenith } else { pressure_correction / (cos_zenith + 0.15 * (93.9 - zenith_deg).powf(-1.253)) };
    let kappa = 1.041 * zenith.powi(3);
    let clearness = ((beam_normal_w_m2 + diffuse_horizontal_w_m2) / diffuse_horizontal_w_m2 + kappa) / (1.0 + kappa);
    let brightness = diffuse_horizontal_w_m2 * air_mass / 1353.0;
    let bin = PEREZ_EPSILON_LIMITS.iter().position(|limit| clearness < *limit).unwrap_or(7);
    PerezBrightening {
        circumsolar: (PEREZ_F11[bin] + PEREZ_F12[bin] * brightness + PEREZ_F13[bin] * zenith).max(0.0),
        horizon: PEREZ_F21[bin] + PEREZ_F22[bin] * brightness + PEREZ_F23[bin] * zenith,
    }
}

/// ☀️ Irradiance incident on a surface [W/m²], split by origin.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IncidentSolar {
    pub beam_w_m2: f64,
    pub sky_diffuse_w_m2: f64,
    pub ground_diffuse_w_m2: f64,
}

impl IncidentSolar {
    /// ☀️ Sky plus ground diffuse.
    pub fn diffuse_w_m2(&self) -> f64 {
        self.sky_diffuse_w_m2 + self.ground_diffuse_w_m2
    }

    /// ☀️ Beam plus diffuse.
    pub fn total_w_m2(&self) -> f64 {
        self.beam_w_m2 + self.diffuse_w_m2()
    }
}

/// 🧮️ Sun-and-sky inputs shared by every surface in one timestep.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SkyState {
    pub sun: [f64; 3],
    pub beam_normal_w_m2: f64,
    pub diffuse_horizontal_w_m2: f64,
    pub brightening: PerezBrightening,
    pub ground_reflectance: f64,
}

impl SkyState {
    /// 🌤️ Sky for a timestep; beam and diffuse are dropped while the sun is below the horizon.
    pub fn new(sun: [f64; 3], beam_normal_w_m2: f64, diffuse_horizontal_w_m2: f64, elevation_m: f64, ground_reflectance: f64) -> Self {
        if sun[2] <= 0.0 {
            return Self { sun, ground_reflectance, ..Self::default() };
        }
        Self { sun, beam_normal_w_m2, diffuse_horizontal_w_m2, brightening: perez_brightening(beam_normal_w_m2, diffuse_horizontal_w_m2, sun[2], elevation_m), ground_reflectance }
    }

    /// ☀️ Irradiance on a surface with outward `normal`, beam `sunlit_fraction`, and the
    /// isotropic/horizon sky-diffuse shading ratios of its obstructions.
    pub fn incident(&self, normal: [f64; 3], sunlit_fraction: f64, isotropic_ratio: f64, horizon_ratio: f64) -> IncidentSolar {
        if self.sun[2] <= 0.0 {
            return IncidentSolar::default();
        }
        let cos_incidence = incidence_cosine(normal, self.sun).max(0.0);
        let cos_tilt = normal[2];
        let sin_tilt = (1.0 - cos_tilt * cos_tilt).max(0.0).sqrt();
        let isotropic = 0.5 * (1.0 + cos_tilt) * (1.0 - self.brightening.circumsolar) * isotropic_ratio;
        let circumsolar = self.brightening.circumsolar * cos_incidence / self.sun[2].max(0.0871557) * sunlit_fraction;
        let horizon = self.brightening.horizon * sin_tilt * horizon_ratio;
        IncidentSolar {
            beam_w_m2: self.beam_normal_w_m2 * cos_incidence * sunlit_fraction,
            sky_diffuse_w_m2: self.diffuse_horizontal_w_m2 * (isotropic + circumsolar + horizon),
            ground_diffuse_w_m2: (self.beam_normal_w_m2 * self.sun[2] + self.diffuse_horizontal_w_m2) * self.ground_reflectance * 0.5 * (1.0 - cos_tilt),
        }
    }
}
// #endregion 🔖️Incidence

// #region 🔖️Shading
/// 🌑️ Area [m²] of `receiver` (outward `normal`) left in shadow by the union of `casters` when
/// lit from `sun`; `None` when the sun is behind the receiver.
pub fn shadowed_area_m2<'a>(receiver: &[[f64; 3]], normal: [f64; 3], casters: impl Iterator<Item = &'a [[f64; 3]]>, sun: [f64; 3]) -> Option<f64> {
    if incidence_cosine(normal, sun) <= 1e-9 {
        return None;
    }
    let frame = PlaneFrame::of(receiver, normal);
    let Some(receiver_polygon) = frame.polygon(receiver) else { return Some(0.0) };
    let full = receiver_polygon.area();
    let mut pieces = [PlanePolygon::EMPTY; MAXIMUM_SUNLIT_PIECES];
    pieces[0] = receiver_polygon;
    let mut count = 1;
    for caster in casters {
        let Some(shadow) = frame.shadow(caster, sun) else { continue };
        let mut next = [PlanePolygon::EMPTY; MAXIMUM_SUNLIT_PIECES];
        let mut next_count = 0;
        for piece in &pieces[..count] {
            let mut remainder = *piece;
            for edge in 0..shadow.len {
                if remainder.len < 3 {
                    break;
                }
                let (a, b) = (shadow.points[edge], shadow.points[(edge + 1) % shadow.len]);
                let Some(outside) = remainder.clip(a, b, false) else { return Some(0.0) };
                if outside.area() > 1e-12 {
                    if next_count == MAXIMUM_SUNLIT_PIECES {
                        return Some(0.0);
                    }
                    next[next_count] = outside;
                    next_count += 1;
                }
                let Some(inside) = remainder.clip(a, b, true) else { return Some(0.0) };
                remainder = inside;
            }
        }
        pieces = next;
        count = next_count;
    }
    let sunlit: f64 = pieces[..count].iter().map(PlanePolygon::area).sum();
    Some((full - sunlit).max(0.0))
}

/// 🌑️ Beam sunlit fraction of a receiver whose own sunlit area excludes `openings` (the windows it
/// hosts): `(A_gross − A_openings − shadow_gross + shadow_openings) / A_net`.
pub fn sunlit_fraction<'a>(receiver: &[[f64; 3]], normal: [f64; 3], openings: impl Iterator<Item = &'a [[f64; 3]]> + Clone, casters: impl Iterator<Item = &'a [[f64; 3]]> + Clone, sun: [f64; 3]) -> f64 {
    let Some(gross_shadow) = shadowed_area_m2(receiver, normal, casters.clone(), sun) else { return 0.0 };
    let gross = crate::geometry::surface_area_m2(receiver);
    let mut net = gross;
    let mut net_shadow = gross_shadow;
    for opening in openings {
        net -= crate::geometry::surface_area_m2(opening);
        net_shadow -= shadowed_area_m2(opening, normal, casters.clone(), sun).unwrap_or(0.0);
    }
    if net <= 1e-9 {
        return 1.0;
    }
    (1.0 - net_shadow.max(0.0) / net).clamp(0.0, 1.0)
}

/// 🌥️ Isotropic and horizon sky-diffuse shading ratios of a receiver: its sunlit fraction
/// integrated over a 6 × 24 sky-patch grid weighted by patch solid angle and incidence, relative
/// to the unobstructed integral (the horizon ratio uses the lowest altitude band only).
pub fn sky_diffuse_shading_ratios<'a>(receiver: &[[f64; 3]], normal: [f64; 3], openings: impl Iterator<Item = &'a [[f64; 3]]> + Clone, casters: impl Iterator<Item = &'a [[f64; 3]]> + Clone) -> (f64, f64) {
    const ALTITUDES: usize = 6;
    const AZIMUTHS: usize = 24;
    let altitude_step = std::f64::consts::FRAC_PI_2 / ALTITUDES as f64;
    let azimuth_step = 2.0 * std::f64::consts::PI / AZIMUTHS as f64;
    let (mut shaded, mut open, mut shaded_horizon, mut open_horizon) = (0.0, 0.0, 0.0, 0.0);
    for altitude_index in 0..ALTITUDES {
        let altitude = (altitude_index as f64 + 0.5) * altitude_step;
        for azimuth_index in 0..AZIMUTHS {
            let azimuth = azimuth_index as f64 * azimuth_step;
            let direction = [altitude.cos() * azimuth.cos(), altitude.cos() * azimuth.sin(), altitude.sin()];
            let cosine = incidence_cosine(normal, direction);
            if cosine < 0.0 {
                continue;
            }
            let weight = altitude.cos() * azimuth_step * altitude_step * cosine;
            let lit = weight * sunlit_fraction(receiver, normal, openings.clone(), casters.clone(), direction);
            shaded += lit;
            open += weight;
            if altitude_index == 0 {
                shaded_horizon += lit;
                open_horizon += weight;
            }
        }
    }
    let ratio = |with: f64, without: f64| if without > 1e-12 { with / without } else { 1.0 };
    (ratio(shaded, open), ratio(shaded_horizon, open_horizon))
}

/// 🟨️ Area [m², measured in the window plane] of the sunlight entering through `window` that lands
/// on the inside of `back`, a surface of the same convex zone facing the incoming beam.
pub fn beam_overlap_m2(window: &[[f64; 3]], window_normal: [f64; 3], back: &[[f64; 3]], back_normal: [f64; 3], sun: [f64; 3]) -> f64 {
    if incidence_cosine(window_normal, sun) <= 1e-9 || incidence_cosine(back_normal, sun) >= -1e-9 {
        return 0.0;
    }
    let frame = PlaneFrame::of(window, window_normal);
    let toward = dot(sun, window_normal);
    let Some(window_polygon) = frame.polygon(window) else { return 0.0 };
    let mut projected = PlanePolygon::EMPTY;
    for &point in back {
        let t = dot(sub(point, frame.origin), window_normal) / toward;
        if !projected.push(frame.project([point[0] - t * sun[0], point[1] - t * sun[1], point[2] - t * sun[2]])) {
            return 0.0;
        }
    }
    let projected = projected.counter_clockwise();
    window_polygon.intersect_convex(&projected).map_or(0.0, |overlap| overlap.area())
}
// #endregion 🔖️Shading

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
