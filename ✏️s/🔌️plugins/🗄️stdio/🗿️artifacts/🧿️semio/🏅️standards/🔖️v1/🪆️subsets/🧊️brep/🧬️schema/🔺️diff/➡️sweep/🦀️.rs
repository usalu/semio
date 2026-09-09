//! ➡️ Exact extrude/revolve/loft/sweep/pipe/helical-sweep. No sampled section counts, no fan
//! caps, no `solid_from_triangle_soup`: every lateral face is either the recognized analytic
//! surface (`Plane`/`Cylinder`/`Cone`/`Torus`/`Sphere`) for the profile-edge kinds where that stays
//! exact, or a NURBS surface built directly from the edge's own `to_nurbs()` control net (never
//! sampled/fit through 3D points except where the profiles/path genuinely differ shape — loft's
//! harmonization, or a sweep path's rotation-minimizing-frame stations). See `📓️w2c-sweeps.md` for
//! the coedge-orientation and pcurve-exactness derivations every submodule below relies on.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/➡️sweep` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL. Rewritten to
//! exact analytic/NURBS sweeps (no sampling, no triangle soup) in ticket
//! 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave W2-C.

#[path = "🧮️core/🦀️.rs"]
mod core;
#[path = "🐍️frame/🦀️.rs"]
mod frame;
#[path = "🥞️loft/🦀️.rs"]
mod loft;
#[path = "🌀️revolve/🦀️.rs"]
mod revolve;

use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_planar_face_from_wire, Wire};
use crate::standards::v1::subsets::brep::schema::diff::transform::transform_face;
use crate::standards::v1::subsets::brep::schema::snapshot::arena::FaceId;
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

pub use loft::loft_profiles;
pub use revolve::revolve_face;

// #region 🔖️Extrude

/// ➡️ Extrudes `face` along `direction` by `distance`. Every profile edge yields an exact side
/// face (`Plane` for a line, `Cylinder` for a circle whose axis is parallel to `direction`, a
/// degree-1-in-v NURBS extrusion surface for a free-form edge — see `🧮️core::translate_lateral`);
/// holes get their own tube faces (every loop, not just the outer one). The profile face itself
/// becomes one cap (flipped in place, recorded modified); a `transform_face` translate becomes the
/// other (recorded generated), matching the ticket's "profile as modified" history convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extrude_face(body: &mut Body, face: FaceId, direction: Vec3, distance: f64, rec: &mut OpRecorder) -> Result<crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId, KernelError> {
    core::require_positive("extrude distance", distance.abs())?;
    let dir = direction.normalized().ok_or_else(|| KernelError::InvalidInput("extrude direction is zero-length".into()))?;
    let offset = dir * distance;
    let prism = core::build_prism(body, face, &core::Placement::Translate { offset }, rec)?;
    let mut faces = vec![face, prism.top];
    faces.extend(prism.laterals);
    Ok(core::finish_solid(body, faces, rec))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newell_normal(points: &[Pnt3]) -> Option<Vec3> {
    if points.len() < 3 {
        return None;
    }
    let mut n = Vec3::ZERO;
    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        n.x += (p.y - q.y) * (p.z + q.z);
        n.y += (p.z - q.z) * (p.x + q.x);
        n.z += (p.x - q.x) * (p.y + q.y);
    }
    n.normalized()
}

/// ➡️ Extrudes a closed wire directly (builds its planar face via [`make_planar_face_from_wire`],
/// then [`extrude_face`]). An open wire's shell-only (no-cap) extrusion is not yet implemented in
/// this pass (documented gap, see `📓️w2c-sweeps.md`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extrude_wire(body: &mut Body, wire: &Wire, direction: Vec3, distance: f64, rec: &mut OpRecorder) -> Result<crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId, KernelError> {
    if !wire.closed {
        return Err(KernelError::Operation("extrude_wire: open-wire (shell-only) extrusion is not yet supported in this pass".into()));
    }
    let pts: Vec<Pnt3> = wire.vertices.iter().map(|&v| body.vertices.get(v).unwrap().position).collect();
    let normal = newell_normal(&pts).ok_or_else(|| KernelError::InvalidInput("extrude_wire: wire is degenerate".into()))?;
    let face = make_planar_face_from_wire(body, wire, pts[0], normal, rec)?;
    extrude_face(body, face, direction, distance, rec)
}

// #endregion 🔖️Extrude

// #region 🔖️Sweep

/// ➡️ Sweeps `profile` along `path`, honouring `guide` if present (per-station `x`-axis points at
/// the guide's closest point). A single straight-line path delegates to [`extrude_face`] (exact
/// `Cylinder`/`Plane` fast paths); a single circular-arc path (no guide) delegates to
/// [`revolve_face`] (exact `Torus`/`Cylinder`/`Cone` fast paths, a circle profile along a circular
/// path is an exact torus segment); any other path builds a rotation-minimizing-frame station
/// chain (`🧮️core::build_prism` under `Placement::General`, adaptively refined — see
/// `🐍️frame::sample_path`), certified only for line/free-form profile edges (circle/ellipse
/// profile edges are refused there, not mis-parametrized — see `📓️w2c-sweeps.md` §pcurve).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pipe(body: &mut Body, profile: FaceId, path: &Wire, guide: Option<&Wire>, rec: &mut OpRecorder) -> Result<crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId, KernelError> {
    if path.members.is_empty() {
        return Err(KernelError::InvalidInput("sweep path is empty".into()));
    }
    if guide.is_none() && path.members.len() == 1 {
        let (edge_id, forward) = path.members[0];
        let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity("path edge".into()))?;
        let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("path curve".into()))?.clone();
        let range = edge.range;
        match curve {
            Curve3::Line { origin, dir } => {
                let p0 = origin + dir * range.0;
                let p1 = origin + dir * range.1;
                let (from, to) = if forward { (p0, p1) } else { (p1, p0) };
                let v = to - from;
                let len = v.norm();
                if len > 1e-12 {
                    let d = v.normalized().unwrap();
                    return extrude_face(body, profile, d, len, rec);
                }
            }
            Curve3::Circle { frame, .. } => {
                let angle = if forward { range.1 - range.0 } else { range.0 - range.1 };
                return revolve_face(body, profile, frame.origin, frame.z, angle, rec);
            }
            _ => {}
        }
    }
    let frames = frame::frame_stations(body, path, guide, 4, 32)?;
    let profile_frame = {
        let f = body.faces.get(profile).ok_or_else(|| KernelError::MissingEntity("profile".into()))?;
        match body.surfaces.get(f.surface) {
            Some(Surface::Plane { frame }) => *frame,
            _ => return Err(KernelError::InvalidInput("sweep profile face must be planar".into())),
        }
    };
    let align = core::frame_to_affine(&frames[0]).compose(&core::frame_to_affine(&profile_frame).inverse().ok_or_else(|| KernelError::Operation("sweep: singular profile placement".into()))?);
    let profile_label = body.faces.get(profile).unwrap().label;
    let aligned = transform_face(body, profile, &align, rec)?;
    rec.record_deleted(profile_label);
    let mut bottom = aligned;
    let mut laterals = Vec::new();
    for i in 1..frames.len() {
        let map = core::frame_to_affine(&frames[i]).compose(&core::frame_to_affine(&frames[i - 1]).inverse().ok_or_else(|| KernelError::Operation("sweep: singular station placement".into()))?);
        let prism = core::build_prism(body, bottom, &core::Placement::General { map }, rec)?;
        laterals.extend(prism.laterals);
        if i != frames.len() - 1 {
            let label = body.faces.get(prism.top).unwrap().label;
            rec.record_deleted(label);
        }
        bottom = prism.top;
    }
    let mut faces = vec![aligned, bottom];
    faces.extend(laterals);
    Ok(core::finish_solid(body, faces, rec))
}

/// ➡️ Sweeps `profile` along `path` with no guide — [`pipe`] with `guide = None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sweep_along_path(body: &mut Body, profile: FaceId, path: &Wire, rec: &mut OpRecorder) -> Result<crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId, KernelError> {
    pipe(body, profile, path, None, rec)
}

/// ➡️ Helical sweep of `profile` about `(axis_origin, axis_dir)`: an analytic helix parametrized
/// directly (point/tangent closed-form, no `Curve3` needed), sampled at `≥16` stations per turn,
/// fed through the same rotation-minimizing-frame station chain as [`pipe`]'s general path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn helical_sweep(body: &mut Body, profile: FaceId, (axis_origin, axis_dir): (Pnt3, Vec3), radius: f64, pitch: f64, turns: f64, rec: &mut OpRecorder) -> Result<crate::standards::v1::subsets::brep::schema::snapshot::arena::SolidId, KernelError> {
    core::require_positive("helical radius", radius)?;
    if !turns.is_finite() || turns.abs() <= 1e-12 {
        return Err(KernelError::InvalidInput("helical turns must be non-zero".into()));
    }
    let axis = axis_dir.normalized().ok_or_else(|| KernelError::InvalidInput("helical axis is zero-length".into()))?;
    let x0 = axis.any_orthogonal();
    let y0 = axis.cross(x0);
    let steps_per_turn = 24usize;
    let steps = ((turns.abs() * steps_per_turn as f64).ceil() as usize).max(2);
    let mut stations = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let s = i as f64 / steps as f64;
        let total_angle = turns * std::f64::consts::TAU;
        let angle = total_angle * s;
        let along = pitch * turns * s;
        let point = axis_origin + x0 * (radius * angle.cos()) + y0 * (radius * angle.sin()) + axis * along;
        let raw_tangent = x0 * (-radius * angle.sin() * total_angle) + y0 * (radius * angle.cos() * total_angle) + axis * (pitch * turns);
        let tangent = raw_tangent.normalized().unwrap_or(axis);
        stations.push(frame::Station { point, tangent });
    }
    let frames = frame::stations_to_frames(&stations);
    let profile_frame = {
        let f = body.faces.get(profile).ok_or_else(|| KernelError::MissingEntity("profile".into()))?;
        match body.surfaces.get(f.surface) {
            Some(Surface::Plane { frame }) => *frame,
            _ => return Err(KernelError::InvalidInput("helical_sweep profile face must be planar".into())),
        }
    };
    let align = core::frame_to_affine(&frames[0]).compose(&core::frame_to_affine(&profile_frame).inverse().ok_or_else(|| KernelError::Operation("helical_sweep: singular profile placement".into()))?);
    let profile_label = body.faces.get(profile).unwrap().label;
    let aligned = transform_face(body, profile, &align, rec)?;
    rec.record_deleted(profile_label);
    let mut bottom = aligned;
    let mut laterals = Vec::new();
    for i in 1..frames.len() {
        let map = core::frame_to_affine(&frames[i]).compose(&core::frame_to_affine(&frames[i - 1]).inverse().ok_or_else(|| KernelError::Operation("helical_sweep: singular station placement".into()))?);
        let prism = core::build_prism(body, bottom, &core::Placement::General { map }, rec)?;
        laterals.extend(prism.laterals);
        if i != frames.len() - 1 {
            let label = body.faces.get(prism.top).unwrap().label;
            rec.record_deleted(label);
        }
        bottom = prism.top;
    }
    let mut faces = vec![aligned, bottom];
    faces.extend(laterals);
    Ok(core::finish_solid(body, faces, rec))
}

// #endregion 🔖️Sweep

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
