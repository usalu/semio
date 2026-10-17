//! 🔺️ Fenestration placement, polygon helpers and the gray-body enclosure radiation factors.

use super::*;

// #region 🔖️Geometry
/// 🧭️ Rotates a model-coordinate point about the vertical axis into world coordinates for a
/// building whose north axis is turned `north_axis_deg` clockwise from true north.
pub fn rotate(point: [f64; 3], north_axis_deg: f64) -> [f64; 3] {
    let angle = north_axis_deg.to_radians();
    let (sine, cosine) = (angle.sin(), angle.cos());
    [point[0] * cosine + point[1] * sine, -point[0] * sine + point[1] * cosine, point[2]]
}

/// 🪟️ Rectangle of a window on its host polygon: `count` windows share the host's horizontal
/// extent in equal bays, this one (`position`) centred in its bay, `sill_m` above the host's lowest
/// point, `area_m2 / height_m` wide. Vertices wind counter-clockwise seen from outside.
pub fn place_window(host: &[[f64; 3]], normal: [f64; 3], count: usize, position: usize, area_m2: f64, height_m: f64, sill_m: f64) -> Vec<[f64; 3]> {
    let height = height_m.max(1e-3);
    let width = area_m2 / height;
    let horizontal = if normal[2].abs() > 0.99 {
        [1.0, 0.0, 0.0]
    } else {
        let h = [-normal[1], normal[0], 0.0];
        let length = (h[0] * h[0] + h[1] * h[1]).sqrt();
        [h[0] / length, h[1] / length, 0.0]
    };
    let upward = [normal[1] * horizontal[2] - normal[2] * horizontal[1], normal[2] * horizontal[0] - normal[0] * horizontal[2], normal[0] * horizontal[1] - normal[1] * horizontal[0]];
    let origin = host[0];
    let coordinate = |point: [f64; 3], axis: [f64; 3]| dot([point[0] - origin[0], point[1] - origin[1], point[2] - origin[2]], axis);
    let (mut u_min, mut u_max, mut v_min) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY);
    for vertex in host {
        u_min = u_min.min(coordinate(*vertex, horizontal));
        u_max = u_max.max(coordinate(*vertex, horizontal));
        v_min = v_min.min(coordinate(*vertex, upward));
    }
    let bay = (u_max - u_min) / count.max(1) as f64;
    let centre = u_min + (position as f64 + 0.5) * bay;
    let point = |u: f64, v: f64| [origin[0] + horizontal[0] * u + upward[0] * v, origin[1] + horizontal[1] * u + upward[1] * v, origin[2] + horizontal[2] * u + upward[2] * v];
    let (u0, u1) = (centre - 0.5 * width, centre + 0.5 * width);
    let (v0, v1) = (v_min + sill_m, v_min + sill_m + height);
    vec![point(u0, v0), point(u1, v0), point(u1, v1), point(u0, v1)]
}

/// 🔶️ THE aperture polygon — the single source of truth every placement of a window goes through
/// (the engine's [`Precompute::step_window`] and the epJSON exporter's `aperture_rectangle` both
/// call it, so the two can never drift again).
///
/// `own_vertices` is [`crate::model::Fenestration::vertices_m`] expressed in the SAME frame as
/// `host`: non-empty and the polygon IS the aperture, returned verbatim; empty and the rectangle is
/// derived from `area_m2`/`height_m`/`sill_m` in bay `index` of `count` exactly as before.
/// Callers that own scalar fallbacks (the exporter's `APERTURE_ASPECT`/`APERTURE_SILL_M`) resolve
/// them before calling, so this function never invents a dimension.
pub fn place_aperture(host: &[[f64; 3]], own_vertices: &[[f64; 3]], area_m2: f64, height_m: f64, sill_m: f64, index: usize, count: usize) -> Vec<[f64; 3]> {
    if !own_vertices.is_empty() {
        return own_vertices.to_vec();
    }
    place_window(host, polygon_normal(host), count, index, area_m2, height_m, sill_m)
}

/// 🔶️ The aperture polygon of `window` on its host `surface`, in model coordinates, placed in bay
/// `index` of `count` when the window carries no polygon of its own. See [`place_aperture`].
pub fn fenestration_polygon_in_bay(surface: &model::Surface, window: &model::Fenestration, index: usize, count: usize) -> Vec<[f64; 3]> {
    place_aperture(&surface.vertices_m, &window.vertices_m, window.area_m2, window.height_m, window.sill_height_m, index, count)
}

/// 🔶️ The aperture polygon of `window` on its host `surface`, in model coordinates, treating the
/// window as the sole occupant of its host.
///
/// This is the form a viewer reaches for when it already knows a surface hosts one window; a host
/// with several windows splits into bays, so render those through
/// [`model_fenestration_polygon`] (which finds the siblings for you) or
/// [`fenestration_polygon_in_bay`].
pub fn fenestration_polygon(surface: &model::Surface, window: &model::Fenestration) -> Vec<[f64; 3]> {
    fenestration_polygon_in_bay(surface, window, 0, 1)
}

/// 🔶️ The aperture polygon of `window` inside `model` — finds the host surface and the window's
/// bay among its siblings the same way the engine does, so a viewer draws exactly what the
/// simulation shades. Answers an empty polygon when the host surface is missing.
pub fn model_fenestration_polygon(model: &Model, window: &model::Fenestration) -> Vec<[f64; 3]> {
    let Some(host) = model.surfaces.iter().find(|surface| surface.id == window.surface_id) else {
        return Vec::new();
    };
    let siblings: Vec<&model::Fenestration> = model.fenestrations.iter().filter(|other| other.surface_id == window.surface_id).collect();
    let index = siblings.iter().position(|other| other.id == window.id).unwrap_or(0);
    fenestration_polygon_in_bay(host, window, index, siblings.len().max(1))
}

/// 🔲️ Approximate view factors and gray-body exchange factors of an enclosure.
///
/// A face sees every other face except itself, faces in its own plane (azimuth and tilt within
/// 10°) and — for floors — other floors; the raw area-weighted factors are then made reciprocal
/// and complete by alternating row normalization and symmetrization. The exchange factors come from
/// the partial-radiosity system `C_ij = A_i F_ji − δ_ij A_i/(1−ε_i)` (Hottel's script-F), so the net
/// long-wave flux into face `i` is `Σ_j σ 𝓕[i][j] (T_j⁴ − T_i⁴)`.
pub fn enclosure_factors(areas: &[f64], emissivities: &[f64], descriptors: &[(f64, f64, bool)]) -> Option<(Vec<f64>, Vec<f64>)> {
    let n = areas.len();
    let mut view = Vec::new();
    let mut exchange = Vec::new();
    let mut work = Vec::new();
    let mut inverse = Vec::new();
    if view.try_reserve_exact(n * n).is_err() || exchange.try_reserve_exact(n * n).is_err() || work.try_reserve_exact(n * n).is_err() || inverse.try_reserve_exact(n * n).is_err() {
        return None;
    }
    view.resize(n * n, 0.0);
    exchange.resize(n * n, 0.0);
    work.resize(n * n, 0.0);
    inverse.resize(n * n, 0.0);
    if n == 0 {
        return Some((view, exchange));
    }
    let sees = |i: usize, j: usize| {
        let (azimuth_i, tilt_i, floor_i) = descriptors[i];
        let (azimuth_j, tilt_j, floor_j) = descriptors[j];
        if i == j || (floor_i && floor_j) {
            return false;
        }
        let azimuth = (azimuth_i - azimuth_j).abs();
        floor_i || floor_j || (azimuth > 10.0 && azimuth < 350.0) || (tilt_i - tilt_j).abs() > 10.0
    };
    for i in 0..n {
        let seen: f64 = (0..n).filter(|&j| sees(i, j)).map(|j| areas[j]).sum();
        if seen > 0.0 {
            for j in 0..n {
                if sees(i, j) {
                    view[i * n + j] = areas[j] / seen;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            work[i * n + j] = view[i * n + j] * areas[i];
        }
    }
    for i in 0..n {
        for j in i + 1..n {
            let mean = 0.5 * (work[i * n + j] + work[j * n + i]);
            work[i * n + j] = mean;
            work[j * n + i] = mean;
        }
    }
    if n > 3 {
        let mut previous = 10.0;
        for _ in 0..400 {
            for i in 0..n {
                let row: f64 = (0..n).map(|j| work[i * n + j]).sum();
                let coefficient = if row.abs() > 1e-10 { areas[i] / row } else { 1.0 };
                for j in 0..n {
                    work[i * n + j] *= coefficient;
                }
            }
            for i in 0..n {
                for j in i + 1..n {
                    let mean = 0.5 * (work[i * n + j] + work[j * n + i]);
                    work[i * n + j] = mean;
                    work[j * n + i] = mean;
                }
            }
            let mut sum = 0.0;
            for i in 0..n {
                for j in 0..n {
                    let value = if areas[i] > 0.0 { work[i * n + j] / areas[i] } else { 0.0 };
                    view[i * n + j] = if value.abs() < 1e-10 { 0.0 } else { value };
                    sum += view[i * n + j];
                }
            }
            let convergence = (sum - n as f64).abs();
            if (previous - convergence).abs() < 1e-5 || convergence <= 0.001 {
                break;
            }
            previous = convergence;
        }
    } else {
        for i in 0..n {
            for j in 0..n {
                view[i * n + j] = if areas[i] > 0.0 { work[i * n + j] / areas[i] } else { 0.0 };
            }
        }
    }
    let emissivity = |i: usize| emissivities[i].min(0.99999);
    let mut matrix = Vec::new();
    if matrix.try_reserve_exact(n * n).is_err() {
        return None;
    }
    matrix.resize(n * n, 0.0);
    for i in 0..n {
        for j in 0..n {
            matrix[i * n + j] = areas[i] * view[j * n + i];
        }
        matrix[i * n + i] -= areas[i] / (1.0 - emissivity(i));
    }
    if !num::invert_dense(&matrix, &mut inverse, &mut work, n) {
        return None;
    }
    for receiver in 0..n {
        let excitation = -emissivity(receiver) * areas[receiver] / (1.0 - emissivity(receiver));
        for sender in 0..n {
            let factor = emissivity(sender) / (1.0 - emissivity(sender));
            exchange[receiver * n + sender] = factor * (inverse[sender * n + receiver] * excitation - if receiver == sender { emissivity(sender) } else { 0.0 });
        }
    }
    Some((view, exchange))
}
/// 🔲️ Carroll participation factors `p_i = σ ε_i / (ε_i / F_i + 1 − ε_i)` with the mean-radiant
/// "view factors" `F_i = 1 / (1 − A_i F_i / Σ A_j F_j)` found by fixed-point iteration.
pub fn mean_radiant_participation(areas: &[f64], emissivities: &[f64]) -> Option<Vec<f64>> {
    let n = areas.len();
    let mut factors = Vec::new();
    if factors.try_reserve_exact(n).is_err() {
        return None;
    }
    factors.resize(n, 1.0);
    let mut weighted: f64 = areas.iter().sum();
    for _ in 0..100 {
        let total = weighted;
        weighted = 0.0;
        let mut change = 0.0;
        for i in 0..n {
            let previous = factors[i];
            factors[i] = 1.0 / (1.0 - areas[i] * factors[i] / total.max(1e-12));
            if !(0.0..=100.0).contains(&factors[i]) {
                factors[i] = 100.0;
            }
            change += (factors[i] - previous).abs();
            weighted += areas[i] * factors[i];
        }
        if change / (n.max(1) as f64) < 1e-4 {
            break;
        }
    }
    for i in 0..n {
        let emissivity = emissivities[i];
        factors[i] = units::STEFAN_BOLTZMANN * emissivity / (emissivity / factors[i] + 1.0 - emissivity);
    }
    Some(factors)
}
// #endregion 🔖️Geometry
