//! 🌀 Differential geometry of every surface kind at `(u, v)`: fundamental forms, Gaussian and mean
//! curvature, principal curvatures and directions, all with respect to the outward normal.
//!
//! Curvature is positive where the surface bends away from its outward normal, so a convex solid has
//! non-negative principal curvatures: sphere `1/r`, cylinder `(1/r, 0)`, a hole's wall `-1/r`.

use crate::brep::representation::arena::FaceId;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::Vec3;

/// 🌀 Local shape of a surface at one parameter pair.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct SurfaceDifferential {
    pub u: f64,
    pub v: f64,
    pub point: [f64; 3],
    pub normal: [f64; 3],
    pub first_fundamental_form: [f64; 3],
    pub second_fundamental_form: [f64; 3],
    pub gaussian: f64,
    pub mean: f64,
    pub principal_curvatures: [f64; 2],
    pub principal_directions: [[f64; 3]; 2],
    pub umbilic: bool,
}

fn array(v: Vec3) -> [f64; 3] {
    [v.x, v.y, v.z]
}

/// 🧶 The differential geometry of `surface` at `(u, v)`; `flipped` reverses its natural normal.
///
/// Fails at a parametric singularity (a sphere's pole, a cone's apex) where no normal exists.
/// At an umbilic point (equal principal curvatures) the directions are an arbitrary orthonormal tangent pair.
pub fn surface_differential(surface: &Surface, flipped: bool, u: f64, v: f64) -> Result<SurfaceDifferential, KernelError> {
    if !(u.is_finite() && v.is_finite()) {
        return Err(KernelError::InvalidInput("surface parameters must be finite".into()));
    }
    if surface.is_degenerate_uv(u, v) {
        return Err(KernelError::InvalidInput(format!("surface is singular at ({u}, {v}): no normal")));
    }
    let d = surface.derivatives(u, v);
    let natural = d.du.cross(d.dv).normalized().ok_or_else(|| KernelError::InvalidInput(format!("surface is singular at ({u}, {v}): no normal")))?;
    let normal = if flipped { -natural } else { natural };
    let (e, f, g) = (d.du.dot(d.du), d.du.dot(d.dv), d.dv.dot(d.dv));
    let (l, m, n) = (-d.duu.dot(normal), -d.duv.dot(normal), -d.dvv.dot(normal));
    let det = e * g - f * f;
    if det.abs() <= 1e-300 {
        return Err(KernelError::InvalidInput(format!("surface first fundamental form is singular at ({u}, {v})")));
    }
    let shape = [[(g * l - f * m) / det, (g * m - f * n) / det], [(e * m - f * l) / det, (e * n - f * m) / det]];
    let mean = (shape[0][0] + shape[1][1]) / 2.0;
    let gaussian = shape[0][0] * shape[1][1] - shape[0][1] * shape[1][0];
    let spread = (mean * mean - gaussian).max(0.0).sqrt();
    let (k1, k2) = (mean + spread, mean - spread);
    let umbilic = (k1 - k2).abs() <= 1e-9 * k1.abs().max(k2.abs()).max(1e-12);
    let first = if umbilic {
        d.du.normalized().unwrap_or(Vec3::X)
    } else {
        let candidates = [(-shape[0][1], shape[0][0] - k1), (k1 - shape[1][1], shape[1][0])];
        let (a, b) = if candidates[0].0.hypot(candidates[0].1) >= candidates[1].0.hypot(candidates[1].1) { candidates[0] } else { candidates[1] };
        (d.du * a + d.dv * b).normalized().unwrap_or(Vec3::X)
    };
    let second = normal.cross(first).normalized().unwrap_or(Vec3::Y);
    Ok(SurfaceDifferential {
        u,
        v,
        point: [d.point.x, d.point.y, d.point.z],
        normal: array(normal),
        first_fundamental_form: [e, f, g],
        second_fundamental_form: [l, m, n],
        gaussian,
        mean,
        principal_curvatures: [k1, k2],
        principal_directions: [array(first), array(second)],
        umbilic,
    })
}

/// 🪢 [`surface_differential`] on the surface of `face`, honouring the face's orientation.
pub fn face_differential(body: &Body, face: FaceId, u: f64, v: f64) -> Result<SurfaceDifferential, KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    surface_differential(surface, face_ent.flipped, u, v)
}
