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

use crate::brep::operations::primitives::{make_planar_face_from_wire, Wire};
use crate::brep::operations::staged::{drive_solid, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use crate::brep::operations::transform::transform_face;
use crate::brep::representation::arena::FaceId;
use crate::brep::representation::curve::Curve3;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::surface::Surface;
use crate::brep::representation::topology::history::OpRecorder;
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::{Pnt3, Vec3};

pub use loft::{loft_profiles, LoftJob};
pub use revolve::{revolve_face, RevolveJob};

// #region 🔖️Extrude

/// ➡️ Resumable [`extrude_face`]: one setup unit flips the profile into one cap and transform-copies
/// the other, one unit per profile coedge builds its side face, one unit closes the solid.
pub struct ExtrudeJob {
    face: FaceId,
    offset: Vec3,
    prism: Option<core::PrismBuilder>,
    plan: Plan,
}

impl ExtrudeJob {
    /// ➡️ Plans an extrusion of `face` along `direction` by `distance`.
    pub fn new(body: &Body, face: FaceId, direction: Vec3, distance: f64) -> Result<Self, KernelError> {
        core::require_positive("extrude distance", distance.abs())?;
        let dir = direction.normalized().ok_or_else(|| KernelError::InvalidInput("extrude direction is zero-length".into()))?;
        core::planar_outward_normal(body, face)?;
        let plan = Plan::new(&[("setup", 1), ("laterals", core::PrismBuilder::lateral_count(body, face)), ("close", 1)]);
        Ok(Self { face, offset: dir * distance, prism: None, plan })
    }
}

impl StagedOperation for ExtrudeJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("extrude job already finished".into()))?;
        match unit.phase {
            0 => self.prism = Some(core::PrismBuilder::begin(body, self.face, core::Placement::Translate { offset: self.offset }, rec)?),
            1 => self.prism.as_mut().expect("prism begun before laterals").lateral(body, rec, unit.index)?,
            _ => {
                let prism = self.prism.take().expect("prism begun before closing").finish();
                let mut faces = vec![self.face, prism.top];
                faces.extend(prism.laterals);
                return Ok(StageStep::Done(StageOutput::Solid(core::finish_solid(body, faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

/// ➡️ Extrudes `face` along `direction` by `distance`. Every profile edge yields an exact side
/// face (`Plane` for a line, `Cylinder` for a circle whose axis is parallel to `direction`, a
/// degree-1-in-v NURBS extrusion surface for a free-form edge — see `🧮️core::translate_lateral`);
/// holes get their own tube faces (every loop, not just the outer one). The profile face itself
/// becomes one cap (flipped in place, recorded modified); a `transform_face` translate becomes the
/// other (recorded generated), matching the ticket's "profile as modified" history convention.
/// This is [`ExtrudeJob`] driven to completion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extrude_face(body: &mut Body, face: FaceId, direction: Vec3, distance: f64, rec: &mut OpRecorder) -> Result<crate::brep::representation::arena::SolidId, KernelError> {
    drive_solid(&mut ExtrudeJob::new(body, face, direction, distance)?, body, rec)
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

/// ➡️ Resumable [`extrude_wire`]: one unit builds the wire's planar face, then the units of the
/// [`ExtrudeJob`] over it.
pub struct ExtrudeWireJob {
    wire: Wire,
    origin: Pnt3,
    normal: Vec3,
    direction: Vec3,
    distance: f64,
    child: Option<ExtrudeJob>,
    plan: Plan,
}

impl ExtrudeWireJob {
    /// ➡️ Plans the extrusion of closed `wire` along `direction` by `distance`.
    pub fn new(body: &Body, wire: &Wire, direction: Vec3, distance: f64) -> Result<Self, KernelError> {
        if !wire.closed {
            return Err(KernelError::Operation("extrude_wire: open-wire (shell-only) extrusion is not yet supported in this pass".into()));
        }
        let pts: Vec<Pnt3> = wire.vertices.iter().map(|&v| body.vertices.get(v).unwrap().position).collect();
        let normal = newell_normal(&pts).ok_or_else(|| KernelError::InvalidInput("extrude_wire: wire is degenerate".into()))?;
        core::require_positive("extrude distance", distance.abs())?;
        direction.normalized().ok_or_else(|| KernelError::InvalidInput("extrude direction is zero-length".into()))?;
        let plan = Plan::new(&[("face", 1), ("setup", 1), ("laterals", wire.members.len()), ("close", 1)]);
        Ok(Self { wire: wire.clone(), origin: pts[0], normal, direction, distance, child: None, plan })
    }
}

impl StagedOperation for ExtrudeWireJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("extrude wire job already finished".into()))?;
        if unit.phase == 0 {
            let face = make_planar_face_from_wire(body, &self.wire, self.origin, self.normal, rec)?;
            self.child = Some(ExtrudeJob::new(body, face, self.direction, self.distance)?);
            return Ok(StageStep::Working);
        }
        self.child.as_mut().expect("face built before extruding").advance(body, rec)
    }
}

/// ➡️ Extrudes a closed wire directly (builds its planar face via [`make_planar_face_from_wire`],
/// then [`extrude_face`]). An open wire's shell-only (no-cap) extrusion is not yet implemented in
/// this pass (documented gap, see `📓️w2c-sweeps.md`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extrude_wire(body: &mut Body, wire: &Wire, direction: Vec3, distance: f64, rec: &mut OpRecorder) -> Result<crate::brep::representation::arena::SolidId, KernelError> {
    drive_solid(&mut ExtrudeWireJob::new(body, wire, direction, distance)?, body, rec)
}

// #endregion 🔖️Extrude

// #region 🔖️Sweep

/// ➡️ Where a [`StationSweepJob`]'s frames come from.
enum StationSource {
    /// ➡️ The sampled stations of a path wire, optionally re-pointed at a guide wire.
    Path { wire: Wire, guide: Option<Wire> },
    /// ➡️ The closed-form stations of a helix about `(axis_origin, axis)`.
    Helix { axis_origin: Pnt3, axis: Vec3, radius: f64, pitch: f64, turns: f64, steps: usize },
}

/// ➡️ Phase indices of a [`StationSweepJob`] plan.
const PATH: usize = 0;
const FRAMES: usize = 1;
const GUIDE: usize = 2;
const ALIGN: usize = 3;
const SEGMENTS: usize = 4;

/// ➡️ Resumable general sweep: the profile is carried along a chain of rotation-minimizing
/// stations, one prism segment per station pair (`🧮️core::PrismBuilder` under
/// `Placement::General`). Units are one path edge sampled, the frame propagation, one guide
/// re-pointing per frame, the profile alignment, then per segment one setup unit and one unit per
/// profile coedge, and a closing unit. The number of stations is only known once the path is
/// sampled, so the segment phase grows when the frames exist.
pub struct StationSweepJob {
    profile: FaceId,
    source: StationSource,
    lateral_count: usize,
    stations: Vec<frame::Station>,
    previous: Option<Pnt3>,
    rmf: Vec<frame::RmfFrame>,
    frames: Vec<crate::brep::representation::vector::matrix::Frame3>,
    aligned: Option<FaceId>,
    bottom: Option<FaceId>,
    prism: Option<core::PrismBuilder>,
    laterals: Vec<FaceId>,
    plan: Plan,
}

impl StationSweepJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn planar_profile_frame(body: &Body, profile: FaceId, label: &str) -> Result<crate::brep::representation::vector::matrix::Frame3, KernelError> {
        let f = body.faces.get(profile).ok_or_else(|| KernelError::MissingEntity("profile".into()))?;
        match body.surfaces.get(f.surface) {
            Some(Surface::Plane { frame }) => Ok(*frame),
            _ => Err(KernelError::InvalidInput(format!("{label} profile face must be planar"))),
        }
    }

    /// ➡️ Plans a sweep of `profile` along `path` (honouring `guide`) over sampled stations.
    fn along_path(body: &Body, profile: FaceId, path: &Wire, guide: Option<&Wire>) -> Result<Self, KernelError> {
        Self::planar_profile_frame(body, profile, "sweep")?;
        let lateral_count = core::PrismBuilder::lateral_count(body, profile);
        let plan = Plan::new(&[("path", path.members.len()), ("frames", 1), ("guide", 0), ("align", 1), ("segments", 0), ("close", 1)]);
        Ok(Self { profile, source: StationSource::Path { wire: path.clone(), guide: guide.cloned() }, lateral_count, stations: Vec::new(), previous: None, rmf: Vec::new(), frames: Vec::new(), aligned: None, bottom: None, prism: None, laterals: Vec::new(), plan })
    }

    /// ➡️ Plans a helical sweep of `profile` about `(axis_origin, axis_dir)`: an analytic helix
    /// parametrized directly (point/tangent closed-form, no `Curve3` needed), sampled at `24`
    /// stations per turn.
    fn helical(body: &Body, profile: FaceId, (axis_origin, axis_dir): (Pnt3, Vec3), radius: f64, pitch: f64, turns: f64) -> Result<Self, KernelError> {
        core::require_positive("helical radius", radius)?;
        if !turns.is_finite() || turns.abs() <= 1e-12 {
            return Err(KernelError::InvalidInput("helical turns must be non-zero".into()));
        }
        let axis = axis_dir.normalized().ok_or_else(|| KernelError::InvalidInput("helical axis is zero-length".into()))?;
        Self::planar_profile_frame(body, profile, "helical_sweep")?;
        let steps = ((turns.abs() * 24.0).ceil() as usize).max(2);
        let lateral_count = core::PrismBuilder::lateral_count(body, profile);
        let plan = Plan::new(&[("path", 0), ("frames", 1), ("guide", 0), ("align", 1), ("segments", steps * (1 + lateral_count)), ("close", 1)]);
        Ok(Self { profile, source: StationSource::Helix { axis_origin, axis, radius, pitch, turns, steps }, lateral_count, stations: Vec::new(), previous: None, rmf: Vec::new(), frames: Vec::new(), aligned: None, bottom: None, prism: None, laterals: Vec::new(), plan })
    }

    /// ➡️ Units per prism segment: its setup plus one per profile coedge.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn segment_units(&self) -> usize {
        1 + self.lateral_count
    }
}

impl StagedOperation for StationSweepJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("sweep job already finished".into()))?;
        match unit.phase {
            PATH => {
                let StationSource::Path { wire, .. } = &self.source else { return Err(KernelError::Operation("sweep: a helix plans no path units".into())) };
                frame::sample_path_edge(body, wire, unit.index, (4, 32), &mut self.previous, &mut self.stations)?;
            }
            FRAMES => {
                match &self.source {
                    StationSource::Path { guide, .. } => {
                        if self.stations.len() < 2 {
                            return Err(KernelError::InvalidInput("sweep path needs at least two distinct stations".into()));
                        }
                        let r0 = self.stations[0].tangent.any_orthogonal();
                        self.rmf = frame::propagate_rmf(&self.stations, r0);
                        let segments = self.rmf.len() - 1;
                        self.plan.grow(SEGMENTS, segments * self.segment_units());
                        if guide.is_some() {
                            self.plan.grow(GUIDE, self.rmf.len());
                        } else {
                            self.frames = self.rmf.iter().map(frame::RmfFrame::frame3).collect();
                        }
                    }
                    StationSource::Helix { axis_origin, axis, radius, pitch, turns, steps } => {
                        let x0 = axis.any_orthogonal();
                        let y0 = axis.cross(x0);
                        let mut stations = Vec::with_capacity(steps + 1);
                        for i in 0..=*steps {
                            let s = i as f64 / *steps as f64;
                            let total_angle = turns * std::f64::consts::TAU;
                            let angle = total_angle * s;
                            let along = pitch * turns * s;
                            let point = *axis_origin + x0 * (radius * angle.cos()) + y0 * (radius * angle.sin()) + *axis * along;
                            let raw_tangent = x0 * (-radius * angle.sin() * total_angle) + y0 * (radius * angle.cos() * total_angle) + *axis * (pitch * turns);
                            let tangent = raw_tangent.normalized().unwrap_or(*axis);
                            stations.push(frame::Station { point, tangent });
                        }
                        self.frames = frame::stations_to_frames(&stations);
                    }
                }
            }
            GUIDE => {
                let StationSource::Path { guide: Some(guide), .. } = &self.source else { return Err(KernelError::Operation("sweep: an unguided sweep plans no guide units".into())) };
                let guide_edges: Vec<_> = guide.members.iter().map(|&(e, _)| e).collect();
                self.frames.push(frame::guide_frame(body, &guide_edges, &self.rmf[unit.index])?);
            }
            ALIGN => {
                let profile_frame = Self::planar_profile_frame(body, self.profile, "sweep")?;
                let label = match self.source {
                    StationSource::Path { .. } => "sweep",
                    StationSource::Helix { .. } => "helical_sweep",
                };
                let align = core::frame_to_affine(&self.frames[0]).compose(&core::frame_to_affine(&profile_frame).inverse().ok_or_else(|| KernelError::Operation(format!("{label}: singular profile placement")))?);
                let profile_label = body.faces.get(self.profile).unwrap().label;
                let aligned = transform_face(body, self.profile, &align, rec)?;
                rec.record_deleted(profile_label);
                self.aligned = Some(aligned);
                self.bottom = Some(aligned);
            }
            SEGMENTS => {
                let (segment, within) = (unit.index / self.segment_units(), unit.index % self.segment_units());
                if within == 0 {
                    let label = match self.source {
                        StationSource::Path { .. } => "sweep",
                        StationSource::Helix { .. } => "helical_sweep",
                    };
                    let map = core::frame_to_affine(&self.frames[segment + 1]).compose(&core::frame_to_affine(&self.frames[segment]).inverse().ok_or_else(|| KernelError::Operation(format!("{label}: singular station placement")))?);
                    self.prism = Some(core::PrismBuilder::begin(body, self.bottom.expect("profile aligned before segments"), core::Placement::General { map }, rec)?);
                } else {
                    self.prism.as_mut().expect("segment begun before laterals").lateral(body, rec, within - 1)?;
                }
                if within == self.lateral_count {
                    let prism = self.prism.take().expect("segment begun before finishing").finish();
                    self.laterals.extend(prism.laterals);
                    if segment + 1 != self.frames.len() - 1 {
                        let label = body.faces.get(prism.top).unwrap().label;
                        rec.record_deleted(label);
                    }
                    self.bottom = Some(prism.top);
                }
            }
            _ => {
                let mut faces = vec![self.aligned.expect("profile aligned"), self.bottom.expect("segments built")];
                faces.extend(std::mem::take(&mut self.laterals));
                return Ok(StageStep::Done(StageOutput::Solid(core::finish_solid(body, faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

/// ➡️ Resumable pipe/sweep/helical sweep. A single straight-line path delegates to [`ExtrudeJob`]
/// (exact `Cylinder`/`Plane` fast paths); a single circular-arc path (no guide) delegates to
/// [`revolve::RevolveJob`] (exact `Torus`/`Cylinder`/`Cone` fast paths, a circle profile along a
/// circular path is an exact torus segment); any other path is a [`StationSweepJob`].
pub enum SweepJob {
    Extrude(ExtrudeJob),
    Revolve(RevolveJob),
    Stations(Box<StationSweepJob>),
}

impl SweepJob {
    /// ➡️ Plans a sweep of `profile` along `path`, honouring `guide` if present (per-station
    /// `x`-axis points at the guide's closest point). Circle/ellipse profile edges are refused on
    /// the general path, not mis-parametrized — see `📓️w2c-sweeps.md` §pcurve.
    pub fn pipe(body: &Body, profile: FaceId, path: &Wire, guide: Option<&Wire>) -> Result<Self, KernelError> {
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
                        return Ok(Self::Extrude(ExtrudeJob::new(body, profile, d, len)?));
                    }
                }
                Curve3::Circle { frame, .. } => {
                    let angle = if forward { range.1 - range.0 } else { range.0 - range.1 };
                    return Ok(Self::Revolve(RevolveJob::new(body, profile, frame.origin, frame.z, angle)?));
                }
                _ => {}
            }
        }
        Ok(Self::Stations(Box::new(StationSweepJob::along_path(body, profile, path, guide)?)))
    }

    /// ➡️ Plans a helical sweep of `profile` about `(axis_origin, axis_dir)`.
    pub fn helical(body: &Body, profile: FaceId, axis: (Pnt3, Vec3), radius: f64, pitch: f64, turns: f64) -> Result<Self, KernelError> {
        Ok(Self::Stations(Box::new(StationSweepJob::helical(body, profile, axis, radius, pitch, turns)?)))
    }
}

impl StagedOperation for SweepJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        match self {
            Self::Extrude(job) => job.progress(),
            Self::Revolve(job) => job.progress(),
            Self::Stations(job) => job.progress(),
        }
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        match self {
            Self::Extrude(job) => job.advance(body, rec),
            Self::Revolve(job) => job.advance(body, rec),
            Self::Stations(job) => job.advance(body, rec),
        }
    }
}

/// ➡️ Sweeps `profile` along `path`, honouring `guide` if present — [`SweepJob::pipe`] driven to
/// completion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pipe(body: &mut Body, profile: FaceId, path: &Wire, guide: Option<&Wire>, rec: &mut OpRecorder) -> Result<crate::brep::representation::arena::SolidId, KernelError> {
    drive_solid(&mut SweepJob::pipe(body, profile, path, guide)?, body, rec)
}

/// ➡️ Sweeps `profile` along `path` with no guide — [`pipe`] with `guide = None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sweep_along_path(body: &mut Body, profile: FaceId, path: &Wire, rec: &mut OpRecorder) -> Result<crate::brep::representation::arena::SolidId, KernelError> {
    pipe(body, profile, path, None, rec)
}

/// ➡️ Helical sweep of `profile` about `(axis_origin, axis_dir)` — [`SweepJob::helical`] driven to
/// completion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn helical_sweep(body: &mut Body, profile: FaceId, axis: (Pnt3, Vec3), radius: f64, pitch: f64, turns: f64, rec: &mut OpRecorder) -> Result<crate::brep::representation::arena::SolidId, KernelError> {
    drive_solid(&mut SweepJob::helical(body, profile, axis, radius, pitch, turns)?, body, rec)
}

// #endregion 🔖️Sweep

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
