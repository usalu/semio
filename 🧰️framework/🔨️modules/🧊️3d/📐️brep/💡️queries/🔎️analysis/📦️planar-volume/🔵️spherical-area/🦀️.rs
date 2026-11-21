//! 🔵️ Exact area of a retained spherical pole-to-rim chart.

use super::{invalid, unsupported, Body, Curve3, Surface, ValueError};
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::{arena::EdgeId, topology::{Coedge, Edge}};
use semio_framework_3d_brep_vector::Pnt3;
use std::f64::consts::{FRAC_PI_2, TAU};

#[derive(Clone, Copy, Default)]
pub(crate) struct SphericalBand {
    lower: Option<f64>,
    upper: Option<f64>,
    seam: Option<EdgeId>,
}

impl SphericalBand {
    /// 🎯️ Projects to the authored latitude interval on the original spherical support.
    pub(crate) fn project(&self, surface: &Surface, target: Pnt3) -> Result<Pnt3, ValueError> {
        let Surface::Sphere { frame, .. } = surface else { return Err(unsupported()); };
        let (Some(lo), Some(hi)) = (self.lower, self.upper) else { return Err(invalid()); };
        let unit = |value: f64| (value - 1.0).abs() <= 128.0 * f64::EPSILON;
        let orthogonal = |value: f64| value.abs() <= 128.0 * f64::EPSILON;
        let [x, y, z] = [frame.x, frame.y, frame.z];
        if ![x.dot(x), y.dot(y), z.dot(z)].into_iter().all(unit)
            || ![x.dot(y), y.dot(z), z.dot(x)].into_iter().all(orthogonal) { return Err(unsupported()); }
        let delta = target - frame.origin;
        let scale = [delta.x.abs(), delta.y.abs(), delta.z.abs()].into_iter().fold(0.0, f64::max);
        if !delta.to_array().into_iter().all(f64::is_finite) { return Err(invalid()); }
        let delta = if scale == 0.0 { delta } else {
            semio_framework_3d_brep_vector::Vec3::new(delta.x / scale, delta.y / scale, delta.z / scale)
        };
        let [x, y, z] = [delta.dot(x), delta.dot(y), delta.dot(z)];
        let radial = x.hypot(y);
        let u = if radial == 0.0 { 0.0 } else { y.atan2(x) };
        let v = if scale == 0.0 { 0.0 } else { z.atan2(radial) };
        let point = surface.eval(u, v.clamp(lo, hi));
        if point.to_array().into_iter().all(f64::is_finite) { Ok(point) } else { Err(invalid()) }
    }

    /// 📐️ Validates each authored row and returns doubled area when the rectangular chart closes.
    pub(crate) fn row(&mut self, index: usize, body: &Body, surface: &Surface, coedge: &Coedge, edge: &Edge, curve: &Curve3)
        -> Result<Option<f64>, ValueError>
    {
        let Surface::Sphere { radius, .. } = surface else { return Err(unsupported()); };
        let Curve2::Line { origin, dir } = body.curves2.get(coedge.pcurve.ok_or_else(unsupported)?).ok_or_else(invalid)?
            else { return Err(unsupported()); };
        if *radius <= 0.0 || !radius.is_finite() || edge.range != coedge.prange
            || !matches!(curve, Curve3::Circle { .. } | Curve3::Line { .. })
        { return Err(unsupported()); }
        let (lo, hi) = coedge.prange;
        let latitude = |angle| (-FRAC_PI_2..=FRAC_PI_2).contains(&angle);
        match index {
            0 | 2 if coedge.forward == (index == 0) && origin.x == 0.0 && dir.x == 1.0 && dir.y == 0.0
                && (lo, hi) == (0.0, TAU) && latitude(origin.y) => {
                if index == 0 { self.lower = Some(origin.y); }
                else if self.upper != Some(origin.y) { return Err(unsupported()); }
            }
            1 | 3 if coedge.forward == (index == 1) && origin.x == (if index == 1 { TAU } else { 0.0 })
                && origin.y == 0.0 && dir.x == 0.0 && dir.y == 1.0 && self.lower == Some(lo)
                && latitude(hi) && lo < hi && (lo == -FRAC_PI_2 || hi == FRAC_PI_2) => {
                if index == 1 {
                    self.upper = Some(hi);
                    self.seam = Some(coedge.edge);
                } else if self.upper != Some(hi) || self.seam != Some(coedge.edge) { return Err(unsupported()); }
            }
            _ => return Err(unsupported()),
        }
        for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let parameter = lo + (hi - lo) * fraction;
            let u = origin.x + dir.x * parameter;
            let v = origin.y + dir.y * parameter;
            let expected = surface.eval(u, v);
            let actual = curve.eval(parameter);
            let scale = expected.to_array().into_iter().map(f64::abs).fold(*radius, f64::max);
            let agrees = |point: Pnt3| {
                let d = point - expected;
                d.x.hypot(d.y).hypot(d.z) <= 128.0 * f64::EPSILON * scale
            };
            if !agrees(actual) { return Err(unsupported()); }
            let vertex = match fraction { 0.0 => Some(edge.v0), 1.0 => Some(edge.v1), _ => None };
            if let Some(id) = vertex {
                if !agrees(body.vertices.get(id).ok_or_else(invalid)?.position) { return Err(unsupported()); }
            }
        }
        if index != 3 { return Ok(None); }
        Ok(Some(doubled_cap_area(*radius, lo, hi)))
    }
}

/// 🌗️ Integrates a complete longitude chart from the authored pole to its rim.
pub(super) fn doubled_cap_area(radius: f64, lo: f64, hi: f64) -> f64 {
    let angular_height = if lo == -FRAC_PI_2 { hi + FRAC_PI_2 } else { FRAC_PI_2 - lo };
    let chord = radius * (angular_height * 0.5).sin();
    (4.0 * TAU * chord) * chord
}
