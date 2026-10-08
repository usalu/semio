//! 📦️ Tight axis-aligned bounds of one shape (not of the whole body).
//!
//! Extrema are solved in closed form: line endpoints, the axis-extreme angles of circles and ellipses, and
//! the stationary points of spheres and tori (kept only when they lie on the trimmed face). Cylinder and
//! cone faces are ruled, so their extrema sit on the boundary curves. NURBS curves are sampled and refined
//! per axis, NURBS faces are tessellated; either one clears `exact`.

use super::{ScopeMembers, ShapeScope};
use crate::brep::queries::mass_properties::closest_point_on_face;
use crate::brep::queries::tessellation::tessellate_face;
use crate::brep::representation::arena::FaceId;
use crate::brep::representation::curve::Curve3;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::{Pnt3, Vec3};
use std::f64::consts::{PI, TAU};

/// 📦️ Axis-aligned bounds. `exact` is `false` when a NURBS curve or face made them approximate.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ShapeBounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub exact: bool,
}

struct Accumulator {
    min: [f64; 3],
    max: [f64; 3],
    exact: bool,
}

impl Accumulator {
    fn add(&mut self, p: Pnt3) {
        for (axis, value) in [p.x, p.y, p.z].into_iter().enumerate() {
            self.min[axis] = self.min[axis].min(value);
            self.max[axis] = self.max[axis].max(value);
        }
    }
    fn diagonal(&self) -> f64 {
        (0..3).map(|axis| (self.max[axis] - self.min[axis]).max(0.0).powi(2)).sum::<f64>().sqrt()
    }
}

const AXES: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];

fn within(angle: f64, from: f64, to: f64) -> bool {
    let (from, to) = (from.min(to), from.max(to));
    if to - from >= TAU - 1e-12 {
        return true;
    }
    (angle - from).rem_euclid(TAU) <= to - from + 1e-12
}

fn add_curve(curve: &Curve3, range: (f64, f64), acc: &mut Accumulator) {
    acc.add(curve.eval(range.0));
    acc.add(curve.eval(range.1));
    match curve {
        Curve3::Line { .. } => {}
        Curve3::Circle { frame, .. } | Curve3::Ellipse { frame, .. } => {
            let (major, minor) = match curve {
                Curve3::Ellipse { major_radius, minor_radius, .. } => (*major_radius, *minor_radius),
                _ => (1.0, 1.0),
            };
            for axis in AXES {
                let angle = (minor * axis.dot(frame.y)).atan2(major * axis.dot(frame.x));
                for candidate in [angle, angle + PI] {
                    if within(candidate, range.0, range.1) {
                        acc.add(curve.eval(candidate));
                    }
                }
            }
        }
        Curve3::Nurbs { .. } => {
            acc.exact = false;
            const SAMPLES: usize = 256;
            let (a, b) = (range.0.min(range.1), range.0.max(range.1));
            let at = |i: usize| a + (b - a) * i as f64 / SAMPLES as f64;
            for axis in AXES {
                for sign in [1.0, -1.0] {
                    let score = |t: f64| sign * (curve.eval(t) - Pnt3::new(0.0, 0.0, 0.0)).dot(axis);
                    let best = (0..=SAMPLES).max_by(|&i, &j| score(at(i)).total_cmp(&score(at(j)))).unwrap_or(0);
                    let (mut lo, mut hi) = (at(best.saturating_sub(1)), at((best + 1).min(SAMPLES)));
                    for _ in 0..60 {
                        let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
                        if score(m1) < score(m2) {
                            lo = m1;
                        } else {
                            hi = m2;
                        }
                    }
                    acc.add(curve.eval(0.5 * (lo + hi)));
                }
            }
        }
    }
}

fn on_face(body: &Body, face: FaceId, p: Pnt3, scale: f64) -> bool {
    closest_point_on_face(body, face, p).is_ok_and(|(_, distance)| distance <= 1e-7 * scale.max(1.0))
}

fn add_face_interior(body: &Body, face: FaceId, acc: &mut Accumulator) -> Result<(), KernelError> {
    let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
    let surface = body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
    let scale = acc.diagonal();
    match surface {
        Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Cone { .. } => {}
        Surface::Sphere { frame, radius } => {
            for axis in AXES {
                for sign in [1.0, -1.0] {
                    let p = frame.origin + axis * (sign * radius);
                    if on_face(body, face, p, scale) {
                        acc.add(p);
                    }
                }
            }
        }
        Surface::Torus { frame, major_radius, minor_radius } => {
            for axis in AXES {
                for sign in [1.0, -1.0] {
                    let d = frame.to_local_vector(axis * sign);
                    let rho = d.x.hypot(d.y);
                    let samples = if rho < 1e-12 { 16 } else { 1 };
                    for k in 0..samples {
                        let base = if rho < 1e-12 { TAU * k as f64 / samples as f64 } else { d.y.atan2(d.x) };
                        for (turn, v) in [(0.0, d.z.atan2(rho)), (0.0, d.z.atan2(rho) + PI), (PI, (-d.z).atan2(rho)), (PI, (-d.z).atan2(rho) + PI)] {
                            let u = base + turn;
                            let radial = major_radius + minor_radius * v.cos();
                            let p = frame.to_world(Pnt3::new(radial * u.cos(), radial * u.sin(), minor_radius * v.sin()));
                            if on_face(body, face, p, scale) {
                                acc.add(p);
                            }
                        }
                    }
                }
            }
        }
        Surface::Nurbs { .. } => {
            acc.exact = false;
            let mesh = tessellate_face(body, face, (1e-3 * scale).max(1e-6))?;
            for point in mesh.position.chunks_exact(3) {
                acc.add(Pnt3::new(point[0] as f64, point[1] as f64, point[2] as f64));
            }
        }
    }
    Ok(())
}

fn gather(body: &Body, members: &ScopeMembers) -> Result<Accumulator, KernelError> {
    let mut acc = Accumulator { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3], exact: true };
    for &vertex in &members.vertices {
        acc.add(body.vertices.get(vertex).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
    }
    for &edge in &members.edges {
        let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
        add_curve(curve, edge_ent.range, &mut acc);
    }
    for &face in &members.faces {
        add_face_interior(body, face, &mut acc)?;
    }
    Ok(acc)
}

/// 🧊 Tight bounds of `scope`: its vertices, edge curves and the interior extrema of its faces.
pub fn bounding_box(body: &Body, scope: &ShapeScope) -> Result<ShapeBounds, KernelError> {
    let members = scope.members(body)?;
    let acc = gather(body, &members)?;
    if !acc.min.iter().all(|value| value.is_finite()) {
        return Err(KernelError::InvalidInput("shape has no geometry to bound".into()));
    }
    Ok(ShapeBounds { min: acc.min, max: acc.max, exact: acc.exact })
}
