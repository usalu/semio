//! ⏱️ Resumable, cancellable B-Rep operation jobs (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane J).
//!
//! Every expensive kernel operation — the blends, shell, draft, offsets, thicken, defeature, the
//! sweeps, revolve, loft, section, split, compound cut, the three patterns and the convex hull — is a
//! [`BrepOperation`] that [`Brep::operation_job_sync`] admits and [`Brep::step_operation_job_sync`]
//! advances in budgeted units, reporting monotone [`BrepOperationProgress`] and honouring
//! [`BrepOperationJob::cancel`]. The twin of [`Brep::boolean_job_sync`] for everything that is not a
//! set operation.
//!
//! 🔒️ Isolation: a job runs inside a private working copy of exactly the entities its inputs reach
//! ([`Body::extract`]). The session's topology is not touched until the job is `Ready`, when the
//! result alone is appended ([`Body::absorb`]) and registered — so a cancelled or failed job leaves
//! the session byte-identical to before admission, with no orphaned entities and no consumed or
//! flipped inputs. Every existing one-shot `*_sync` operation is [`Brep::run_operation_sync`]: the
//! job driven to completion, so there is exactly one implementation of each operation.

use super::*;
use crate::brep::operations::blend::BlendJob;
use crate::brep::operations::boolean::{BooleanFoldJob, SectionJob, SplitJob};
use crate::brep::operations::offset::{DraftJob, OffsetFaceJob, OffsetSolidJob, ShellJob, ThickenJob};
use crate::brep::operations::primitives::ConvexHullJob;
use crate::brep::operations::sew::DefeatureJob;
use crate::brep::operations::staged::{StageOutput, StageProgress, StageStep, StagedOperation};
use crate::brep::operations::sweep::{ExtrudeJob, ExtrudeWireJob, LoftJob, RevolveJob, SweepJob};
use crate::brep::representation::topology::{EntityRef, MergeMap};

/// 🧭️ The tolerance every plane operation and fold of the engine runs at.
const KERNEL_TOLERANCE: f64 = 1e-6;

// #region 🔖️Descriptor

/// 🧭️ One expensive kernel operation, named by its input handles and parameters. The language-agnostic
/// fixture `🧫️fixtures/⏱️operation-jobs/🔣️.json` names the same operations and parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum BrepOperation {
    Fillet { shape: GeometryHandle, radius: f64 },
    FilletVariable { shape: GeometryHandle, radius_start: f64, radius_end: f64 },
    FilletEdges { shape: GeometryHandle, edges: Vec<GeometryHandle>, radius: f64 },
    Chamfer { shape: GeometryHandle, distance: f64 },
    ChamferAsymmetric { shape: GeometryHandle, first: f64, second: f64 },
    ChamferEdges { shape: GeometryHandle, edges: Vec<GeometryHandle>, distance: f64 },
    Shell { shape: GeometryHandle, thickness: f64, open_faces: Vec<GeometryHandle> },
    Draft { shape: GeometryHandle, faces: Vec<GeometryHandle>, pull_direction: Vec3, neutral_point: Vec3, angle: f64 },
    OffsetSolid { shape: GeometryHandle, distance: f64 },
    OffsetFace { face: GeometryHandle, distance: f64 },
    ThickenFace { face: GeometryHandle, thickness: f64 },
    Defeature { shape: GeometryHandle, faces: Vec<GeometryHandle> },
    Extrude { face: GeometryHandle, direction: Vec3, distance: f64 },
    ExtrudeWire { wire: GeometryHandle, vector: Vec3 },
    Revolve { face: GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, angle: f64 },
    Sweep { profile: GeometryHandle, path: GeometryHandle },
    Pipe { profile: GeometryHandle, path: GeometryHandle, guide: Option<GeometryHandle> },
    HelicalSweep { profile: GeometryHandle, axis_origin: Vec3, axis_direction: Vec3, radius: f64, pitch: f64, turns: f64 },
    Loft { profiles: Vec<GeometryHandle>, smooth: bool },
    Section { solid: GeometryHandle, plane_origin: Vec3, plane_normal: Vec3 },
    Split { solid: GeometryHandle, plane_origin: Vec3, plane_normal: Vec3 },
    CompoundCut { target: GeometryHandle, tools: Vec<GeometryHandle> },
    LinearPattern { shape: GeometryHandle, direction: Vec3, spacing: f64, count: usize },
    CircularPattern { shape: GeometryHandle, axis: Vec3, count: usize },
    GridPattern { shape: GeometryHandle, dir_x: Vec3, dir_y: Vec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize },
    ConvexHull { points: Vec<Vec3> },
}

impl BrepOperation {
    /// 🏷️ The stable camelCase name every host and fixture calls this operation by.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Fillet { .. } => "fillet",
            Self::FilletVariable { .. } => "filletVariable",
            Self::FilletEdges { .. } => "filletEdges",
            Self::Chamfer { .. } => "chamfer",
            Self::ChamferAsymmetric { .. } => "chamferAsymmetric",
            Self::ChamferEdges { .. } => "chamferEdges",
            Self::Shell { .. } => "shell",
            Self::Draft { .. } => "draft",
            Self::OffsetSolid { .. } => "offsetSolid",
            Self::OffsetFace { .. } => "offsetFace",
            Self::ThickenFace { .. } => "thickenFace",
            Self::Defeature { .. } => "defeature",
            Self::Extrude { .. } => "extrude",
            Self::ExtrudeWire { .. } => "extrudeWire",
            Self::Revolve { .. } => "revolve",
            Self::Sweep { .. } => "sweep",
            Self::Pipe { .. } => "pipe",
            Self::HelicalSweep { .. } => "helicalSweep",
            Self::Loft { .. } => "loft",
            Self::Section { .. } => "section",
            Self::Split { .. } => "split",
            Self::CompoundCut { .. } => "compoundCut",
            Self::LinearPattern { .. } => "linearPattern",
            Self::CircularPattern { .. } => "circularPattern",
            Self::GridPattern { .. } => "gridPattern",
            Self::ConvexHull { .. } => "convexHull",
        }
    }
}

// #endregion 🔖️Descriptor

// #region 🔖️Job

/// 📈️ Progress of one [`BrepOperationJob`]: `done` never decreases and a finished job has
/// `done == total`. `total` is the plan known so far — it grows when a nested job (a boolean inside a
/// pattern, a fillet inside a round offset) reveals its own size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrepOperationProgress {
    pub done: usize,
    pub total: usize,
    pub phase: &'static str,
}

impl From<StageProgress> for BrepOperationProgress {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from(progress: StageProgress) -> Self {
        Self { done: progress.done, total: progress.total, phase: progress.phase }
    }
}

/// 🔀️ What [`Brep::operation_job_sync`] admitted.
pub enum BrepOperationAdmission {
    /// ⚡️ The operation is the identity (a one-instance pattern): these handles answer it, nothing to resume.
    Answered(Vec<GeometryHandle>),
    /// ⏱️ The operation as a budgetable job.
    Job(BrepOperationJob),
}

/// ⏱️ Outcome of one [`Brep::step_operation_job_sync`].
pub enum BrepOperationStep {
    /// 🔁 Budget spent, work remains.
    Working(BrepOperationProgress),
    /// ✅️ Terminal: the result handles, in the operation's own order (`split`: positive then negative side).
    Ready(Vec<GeometryHandle>),
    /// 🛑️ Terminal: the job was cancelled; the session was never touched.
    Cancelled(BrepOperationProgress),
}

/// 🔒️ What a working job owns: its private copy of the inputs and the staged operation over it.
struct Running {
    scratch: Body,
    rec: OpRecorder,
    stage: Box<dyn StagedOperation>,
    base: u64,
}

/// 🏁 How a job ended, if it has.
enum Outcome {
    Pending,
    Ready(Vec<GeometryHandle>),
    Cancelled,
    Failed(BrepError),
}

/// ⏱️ A retained resumable operation. It borrows nothing, so a host may keep it across turns and
/// re-present the kernel on every [`Brep::step_operation_job_sync`].
pub struct BrepOperationJob {
    running: Option<Running>,
    outcome: Outcome,
    last: BrepOperationProgress,
}

impl BrepOperationJob {
    /// 📈️ Progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn progress(&self) -> BrepOperationProgress {
        self.running.as_ref().map_or(self.last, |running| running.stage.progress().into())
    }

    /// 🛑️ Retires the job at the next observable boundary and frees its working copy. A finished job
    /// is never cancelled — supersession may only retire work still in flight.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cancel(&mut self) {
        if matches!(self.outcome, Outcome::Pending) {
            self.last = self.progress();
            self.running = None;
            self.outcome = Outcome::Cancelled;
        }
    }

    /// ✅️ True once the job reached a terminal state (ready, cancelled or failed).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_terminal(&self) -> bool {
        !matches!(self.outcome, Outcome::Pending)
    }
}

// #endregion 🔖️Job

// #region 🔖️Extraction

/// 🔒️ The private working copy of a job's inputs and the translation from session ids into it.
struct Extraction {
    body: Body,
    map: MergeMap,
    base: u64,
}

impl Extraction {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn translate<K: Copy + Eq + std::hash::Hash + std::fmt::Display>(map: &HashMap<K, K>, id: K) -> Result<K, BrepError> {
        map.get(&id).copied().ok_or_else(|| BrepError::MissingHandle(id.to_string()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn solid(&self, id: SolidId) -> Result<SolidId, BrepError> {
        Self::translate(&self.map.solids, id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn face(&self, id: FaceId) -> Result<FaceId, BrepError> {
        Self::translate(&self.map.faces, id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn faces(&self, ids: &[FaceId]) -> Result<Vec<FaceId>, BrepError> {
        ids.iter().map(|&id| self.face(id)).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edges(&self, ids: &[EdgeId]) -> Result<Vec<EdgeId>, BrepError> {
        ids.iter().map(|&id| Self::translate(&self.map.edges, id)).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wire(&self, wire: &Wire) -> Result<Wire, BrepError> {
        let members = wire.members.iter().map(|&(edge, forward)| Ok((Self::translate(&self.map.edges, edge)?, forward))).collect::<Result<Vec<_>, BrepError>>()?;
        let vertices = wire.vertices.iter().map(|&vertex| Self::translate(&self.map.vertices, vertex)).collect::<Result<Vec<_>, BrepError>>()?;
        Ok(Wire { members, vertices, closed: wire.closed })
    }
}

impl Brep {
    /// 🔒️ Copies exactly what `handles` reach into a private working body.
    fn extraction(&self, handles: &[&GeometryHandle]) -> Result<Extraction, BrepError> {
        let mut roots = Vec::new();
        for handle in handles {
            roots.extend(entity_roots(self.entity(handle)?));
        }
        let base = self.body.labels.next();
        let (body, map) = self.body.extract(&roots);
        Ok(Extraction { body, map, base })
    }

    /// 🔒️ An empty working body for an operation without input shapes.
    fn empty_extraction(&self) -> Extraction {
        let base = self.body.labels.next();
        let (body, map) = self.body.extract(&[]);
        Extraction { body, map, base }
    }

    /// 🎯️ Face ids of `handles`, each resolved through the session.
    fn face_ids(&self, handles: &[GeometryHandle]) -> Result<Vec<FaceId>, BrepError> {
        handles.iter().map(|handle| self.face_id(handle)).collect()
    }
}

// #endregion 🔖️Extraction

// #region 🔖️Admission

/// 🧭️ A planned operation: its working copy and the staged job over it.
struct Planned {
    extraction: Extraction,
    stage: Box<dyn StagedOperation>,
}

/// 🧭️ Wraps a staged job's constructor error the way the one-shot operations always reported it.
fn kernel<T>(result: Result<T, KernelError>) -> Result<T, BrepError> {
    result.map_err(|error| map_err(&error))
}

impl Brep {
    /// 🧭️ Resolves handles, validates the request and plans the staged job; nothing in the session changes.
    fn plan_operation(&self, operation: &BrepOperation) -> Result<Planned, BrepError> {
        match operation {
            BrepOperation::Fillet { shape, radius } => {
                let solid = self.solid_id(shape)?;
                let x = self.extraction(&[shape])?;
                let solid = x.solid(solid)?;
                let edges = all_edges(&x.body, solid);
                let stage = kernel(BlendJob::fillet(&x.body, solid, &edges, *radius))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::FilletVariable { shape, radius_start, radius_end } => {
                let solid = self.solid_id(shape)?;
                let x = self.extraction(&[shape])?;
                let solid = x.solid(solid)?;
                let edge = *all_edges(&x.body, solid).first().ok_or_else(|| BrepError::InvalidInput("no edges".into()))?;
                let stage = kernel(BlendJob::variable_fillet(&x.body, solid, edge, *radius_start, *radius_end))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::FilletEdges { shape, edges, radius } => {
                require_positive_size("fillet radius", *radius)?;
                let solid = self.solid_id(shape)?;
                let selected = self.scoped_solid_edges(solid, edges)?;
                let x = self.extraction(&[shape])?;
                let stage = kernel(BlendJob::fillet(&x.body, x.solid(solid)?, &x.edges(&selected)?, *radius))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Chamfer { shape, distance } => self.plan_chamfer_all(shape, *distance, *distance),
            BrepOperation::ChamferAsymmetric { shape, first, second } => self.plan_chamfer_all(shape, *first, *second),
            BrepOperation::ChamferEdges { shape, edges, distance } => {
                require_positive_size("chamfer distance", *distance)?;
                let solid = self.solid_id(shape)?;
                let selected = self.scoped_solid_edges(solid, edges)?;
                let x = self.extraction(&[shape])?;
                let stage = kernel(BlendJob::chamfer(&x.body, x.solid(solid)?, &x.edges(&selected)?, *distance, *distance))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Shell { shape, thickness, open_faces } => {
                require_positive_size("shell thickness", *thickness)?;
                let solid = self.solid_id(shape)?;
                let scope: std::collections::BTreeSet<_> = self.body.solid_faces(solid).into_iter().collect();
                let mut open_ids = std::collections::BTreeSet::new();
                for handle in open_faces {
                    let face = self.face_id(handle)?;
                    if !scope.contains(&face) {
                        return Err(BrepError::InvalidInput("Open face is not part of the selected solid".into()));
                    }
                    open_ids.insert(face);
                }
                if !open_ids.is_empty() && open_ids.len() == scope.len() {
                    return Err(BrepError::InvalidInput("Keep at least one face when shelling a solid".into()));
                }
                let x = self.extraction(&[shape])?;
                let open: Vec<FaceId> = x.faces(&open_ids.into_iter().collect::<Vec<_>>())?;
                let stage = kernel(ShellJob::new(&x.body, x.solid(solid)?, *thickness, &open))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Draft { shape, faces, pull_direction, neutral_point, angle } => {
                let solid = self.solid_id(shape)?;
                let selected = self.face_ids(faces)?;
                let mut handles = vec![shape];
                handles.extend(faces.iter());
                let x = self.extraction(&handles)?;
                let solid = x.solid(solid)?;
                let faces = if selected.is_empty() { x.body.solid_faces(solid) } else { x.faces(&selected)? };
                let stage = kernel(DraftJob::new(&x.body, solid, &faces, vec3(*pull_direction), (pnt(*neutral_point), vec3(*pull_direction)), *angle))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::OffsetSolid { shape, distance } => {
                let solid = self.solid_id(shape)?;
                let x = self.extraction(&[shape])?;
                let stage = kernel(OffsetSolidJob::with_default_corner(&x.body, x.solid(solid)?, *distance))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::OffsetFace { face, distance } => {
                let id = self.face_id(face)?;
                let x = self.extraction(&[face])?;
                let stage = kernel(OffsetFaceJob::new(&x.body, x.face(id)?, *distance))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::ThickenFace { face, thickness } => {
                let id = self.face_id(face)?;
                let x = self.extraction(&[face])?;
                let stage = kernel(ThickenJob::new(&x.body, x.face(id)?, *thickness))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Defeature { shape, faces } => {
                let solid = self.solid_id(shape)?;
                let selected = self.face_ids(faces)?;
                let mut handles = vec![shape];
                handles.extend(faces.iter());
                let x = self.extraction(&handles)?;
                let stage = kernel(DefeatureJob::new(&x.body, x.solid(solid)?, &x.faces(&selected)?))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Extrude { face, direction, distance } => {
                let id = self.face_id(face)?;
                let x = self.extraction(&[face])?;
                let stage = kernel(ExtrudeJob::new(&x.body, x.face(id)?, vec3(*direction), *distance))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::ExtrudeWire { wire, vector } => {
                let source = self.wire_ref(wire)?.clone();
                let distance = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
                let direction = if distance > 1e-15 { [vector[0] / distance, vector[1] / distance, vector[2] / distance] } else { [0.0, 0.0, 1.0] };
                let x = self.extraction(&[wire])?;
                let stage = kernel(ExtrudeWireJob::new(&x.body, &x.wire(&source)?, vec3(direction), distance))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Revolve { face, axis_origin, axis_direction, angle } => {
                let id = self.face_id(face)?;
                let x = self.extraction(&[face])?;
                let stage = kernel(RevolveJob::new(&x.body, x.face(id)?, pnt(*axis_origin), vec3(*axis_direction), *angle))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Sweep { profile, path } => self.plan_pipe(profile, path, None),
            BrepOperation::Pipe { profile, path, guide } => self.plan_pipe(profile, path, guide.as_ref()),
            BrepOperation::HelicalSweep { profile, axis_origin, axis_direction, radius, pitch, turns } => {
                let id = self.face_id(profile)?;
                let x = self.extraction(&[profile])?;
                let stage = kernel(SweepJob::helical(&x.body, x.face(id)?, (pnt(*axis_origin), vec3(*axis_direction)), *radius, *pitch, *turns))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Loft { profiles, smooth } => {
                let ids = self.face_ids(profiles)?;
                let x = self.extraction(&profiles.iter().collect::<Vec<_>>())?;
                let stage = kernel(LoftJob::new(&x.body, &x.faces(&ids)?, *smooth))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Section { solid, plane_origin, plane_normal } => {
                let id = self.solid_id(solid)?;
                let x = self.extraction(&[solid])?;
                let stage = kernel(SectionJob::new(&x.body, x.solid(id)?, pnt(*plane_origin), vec3(*plane_normal), KERNEL_TOLERANCE))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::Split { solid, plane_origin, plane_normal } => {
                let id = self.solid_id(solid)?;
                let x = self.extraction(&[solid])?;
                let stage = kernel(SplitJob::new(&x.body, x.solid(id)?, pnt(*plane_origin), vec3(*plane_normal), KERNEL_TOLERANCE))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::CompoundCut { target, tools } => {
                let target_id = self.solid_id(target)?;
                let tool_ids = tools.iter().map(|tool| self.solid_id(tool)).collect::<Result<Vec<_>, _>>()?;
                let mut handles = vec![target];
                handles.extend(tools.iter());
                let x = self.extraction(&handles)?;
                let tool_ids = tool_ids.iter().map(|&tool| x.solid(tool)).collect::<Result<Vec<_>, _>>()?;
                let stage = kernel(BooleanFoldJob::compound_cut(&x.body, x.solid(target_id)?, &tool_ids, KERNEL_TOLERANCE))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
            BrepOperation::LinearPattern { shape, direction, spacing, count } => {
                let mut transforms = Vec::new();
                for i in 1..(*count).max(1) {
                    let offset = [direction[0] * spacing * i as f64, direction[1] * spacing * i as f64, direction[2] * spacing * i as f64];
                    require_finite_vector("translate offset", offset)?;
                    transforms.push(Affine3::translation(vec3(offset)));
                }
                self.plan_pattern(shape, transforms)
            }
            BrepOperation::CircularPattern { shape, axis, count } => {
                let n = (*count).max(1);
                let mut transforms = Vec::new();
                for i in 1..n {
                    require_direction("rotate axis", *axis)?;
                    let angle = std::f64::consts::TAU * i as f64 / n as f64;
                    require_finite_scalar("rotate angle", angle)?;
                    transforms.push(Affine3::rotation_axis_angle(vec3(*axis), angle));
                }
                self.plan_pattern(shape, transforms)
            }
            BrepOperation::GridPattern { shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y } => {
                let mut transforms = Vec::new();
                for i in 0..(*count_x).max(1) {
                    for j in 0..(*count_y).max(1) {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let (fi, fj) = (i as f64, j as f64);
                        let offset = [dir_x[0] * spacing_x * fi + dir_y[0] * spacing_y * fj, dir_x[1] * spacing_x * fi + dir_y[1] * spacing_y * fj, dir_x[2] * spacing_x * fi + dir_y[2] * spacing_y * fj];
                        require_finite_vector("translate offset", offset)?;
                        transforms.push(Affine3::translation(vec3(offset)));
                    }
                }
                self.plan_pattern(shape, transforms)
            }
            BrepOperation::ConvexHull { points } => {
                let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
                let x = self.empty_extraction();
                let stage = kernel(ConvexHullJob::new(&pts))?;
                Ok(Planned { extraction: x, stage: Box::new(stage) })
            }
        }
    }

    /// 🎨️ Plans a chamfer of every edge of `shape` with the two face distances.
    fn plan_chamfer_all(&self, shape: &GeometryHandle, first: f64, second: f64) -> Result<Planned, BrepError> {
        let solid = self.solid_id(shape)?;
        let x = self.extraction(&[shape])?;
        let solid = x.solid(solid)?;
        let edges = all_edges(&x.body, solid);
        let stage = kernel(BlendJob::chamfer(&x.body, solid, &edges, first, second))?;
        Ok(Planned { extraction: x, stage: Box::new(stage) })
    }

    /// ➡️ Plans a sweep of `profile` along `path` with an optional `guide`.
    fn plan_pipe(&self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<Planned, BrepError> {
        let face = self.face_id(profile)?;
        let path_wire = self.wire_ref(path)?.clone();
        let guide_wire = match guide {
            Some(handle) => Some(self.wire_ref(handle)?.clone()),
            None => None,
        };
        let mut handles = vec![profile, path];
        handles.extend(guide);
        let x = self.extraction(&handles)?;
        let guide_wire = guide_wire.as_ref().map(|wire| x.wire(wire)).transpose()?;
        let stage = kernel(SweepJob::pipe(&x.body, x.face(face)?, &x.wire(&path_wire)?, guide_wire.as_ref()))?;
        Ok(Planned { extraction: x, stage: Box::new(stage) })
    }

    /// 🔂️ Plans a pattern of `shape` under `transforms`; no transform means the pattern is `shape` itself.
    fn plan_pattern(&self, shape: &GeometryHandle, transforms: Vec<Affine3>) -> Result<Planned, BrepError> {
        let solid = self.solid_id(shape)?;
        let x = self.extraction(&[shape])?;
        let stage = kernel(BooleanFoldJob::pattern(&x.body, x.solid(solid)?, transforms, KERNEL_TOLERANCE))?;
        Ok(Planned { extraction: x, stage: Box::new(stage) })
    }

    /// ⏱️ Admits `operation`: resolves its handles, validates it and returns it as a budgetable job —
    /// or answers at once when the operation is the identity (a pattern of fewer than two instances).
    /// Nothing in the session changes, whatever happens to the job afterwards.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn operation_job_sync(&self, operation: BrepOperation) -> Result<BrepOperationAdmission, BrepError> {
        match &operation {
            BrepOperation::LinearPattern { shape, count, .. } if *count <= 1 => return Ok(BrepOperationAdmission::Answered(vec![shape.clone()])),
            BrepOperation::CircularPattern { shape, count, .. } if *count <= 1 => return Ok(BrepOperationAdmission::Answered(vec![shape.clone()])),
            BrepOperation::GridPattern { shape, count_x, count_y, .. } if (*count_x).max(1) * (*count_y).max(1) <= 1 => return Ok(BrepOperationAdmission::Answered(vec![shape.clone()])),
            _ => {}
        }
        let Planned { extraction, stage } = self.plan_operation(&operation)?;
        let last = stage.progress().into();
        Ok(BrepOperationAdmission::Job(BrepOperationJob { running: Some(Running { scratch: extraction.body, rec: OpRecorder::new(), stage, base: extraction.base }), outcome: Outcome::Pending, last }))
    }
}

// #endregion 🔖️Admission

// #region 🔖️Stepping

impl Brep {
    /// 📥️ Appends the finished result of `running` to the session and registers its handles.
    fn commit_operation(&mut self, running: Running, output: StageOutput) -> Vec<GeometryHandle> {
        let roots: Vec<EntityRef> = match &output {
            StageOutput::Solid(solid) => vec![EntityRef::Solid(*solid)],
            StageOutput::Face(face) => vec![EntityRef::Face(*face)],
            StageOutput::Faces(faces) => faces.iter().map(|face| EntityRef::Face(*face)).collect(),
            StageOutput::Solids(solids) => solids.iter().map(|solid| EntityRef::Solid(*solid)).collect(),
        };
        let map = self.body.absorb(&running.scratch, &roots, running.base);
        match output {
            StageOutput::Solid(solid) => vec![self.register_solid(map.solids[&solid])],
            StageOutput::Face(face) => vec![self.register_face(map.faces[&face])],
            StageOutput::Faces(faces) => faces.into_iter().map(|face| self.register_face(map.faces[&face])).collect(),
            StageOutput::Solids(solids) => solids.into_iter().map(|solid| self.register_solid(map.solids[&solid])).collect(),
        }
    }

    /// ⏱️ Advances `job` by at most `budget` units against its own working copy. The job borrows
    /// nothing, so a host may retain it across turns and re-present the kernel on every step. The
    /// session changes exactly once, in the step that returns [`BrepOperationStep::Ready`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn step_operation_job_sync(&mut self, job: &mut BrepOperationJob, budget: usize) -> Result<BrepOperationStep, BrepError> {
        match &job.outcome {
            Outcome::Ready(handles) => return Ok(BrepOperationStep::Ready(handles.clone())),
            Outcome::Cancelled => return Ok(BrepOperationStep::Cancelled(job.last)),
            Outcome::Failed(error) => return Err(error.clone()),
            Outcome::Pending => {}
        }
        let running = job.running.as_mut().expect("a pending job owns its working copy");
        let mut finished = None;
        for _ in 0..budget {
            match running.stage.advance(&mut running.scratch, &mut running.rec) {
                Ok(StageStep::Working) => {}
                Ok(StageStep::Done(output)) => {
                    finished = Some(Ok(output));
                    break;
                }
                Err(error) => {
                    finished = Some(Err(map_err(&error)));
                    break;
                }
            }
        }
        job.last = running.stage.progress().into();
        match finished {
            None => Ok(BrepOperationStep::Working(job.last)),
            Some(Err(error)) => {
                job.running = None;
                job.outcome = Outcome::Failed(error.clone());
                Err(error)
            }
            Some(Ok(output)) => {
                let running = job.running.take().expect("a pending job owns its working copy");
                let handles = self.commit_operation(running, output);
                job.outcome = Outcome::Ready(handles.clone());
                Ok(BrepOperationStep::Ready(handles))
            }
        }
    }

    /// ♾️ Admits `operation` and drives it to completion — the one-shot form of every operation here.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn run_operation_sync(&mut self, operation: BrepOperation) -> Result<Vec<GeometryHandle>, BrepError> {
        match self.operation_job_sync(operation)? {
            BrepOperationAdmission::Answered(handles) => Ok(handles),
            BrepOperationAdmission::Job(mut job) => loop {
                match self.step_operation_job_sync(&mut job, usize::MAX)? {
                    BrepOperationStep::Ready(handles) => return Ok(handles),
                    BrepOperationStep::Cancelled(_) => return Err(BrepError::Operation("operation cancelled".to_string())),
                    BrepOperationStep::Working(_) => continue,
                }
            },
        }
    }

    /// ♾️ [`Brep::run_operation_sync`] for an operation with exactly one result.
    fn run_single(&mut self, operation: BrepOperation) -> Result<GeometryHandle, BrepError> {
        self.run_operation_sync(operation)?.into_iter().next().ok_or_else(|| BrepError::Operation("operation produced no result".into()))
    }
}

// #endregion 🔖️Stepping

// #region 🔖️OneShot

impl Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn convex_hull_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::ConvexHull { points: points.to_vec() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn offset_face_sync(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::OffsetFace { face: face.clone(), distance })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::ThickenFace { face: face.clone(), thickness })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn extrude_wire_sync(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::ExtrudeWire { wire: wire.clone(), vector })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn extrude_sync(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Extrude { face: face.clone(), direction, distance })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Revolve { face: face.clone(), axis_origin, axis_direction, angle })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Loft { profiles: profiles.to_vec(), smooth })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Sweep { profile: profile.clone(), path: path.clone() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pipe_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Pipe { profile: profile.clone(), path: path.clone(), guide: guide.cloned() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::HelicalSweep { profile: profile.clone(), axis_origin, axis_direction, radius, pitch, turns })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compound_cut_sync(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::CompoundCut { target: target.clone(), tools: tools.to_vec() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn linear_pattern_sync(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::LinearPattern { shape: shape.clone(), direction, spacing, count })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn circular_pattern_sync(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::CircularPattern { shape: shape.clone(), axis, count })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, (spacing_x, spacing_y): (f64, f64), count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::GridPattern { shape: shape.clone(), dir_x, dir_y, spacing_x, spacing_y, count_x, count_y })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Fillet { shape: shape.clone(), radius })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::FilletVariable { shape: shape.clone(), radius_start, radius_end })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fillet_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::FilletEdges { shape: shape.clone(), edges: edges.to_vec(), radius })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Chamfer { shape: shape.clone(), distance })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::ChamferAsymmetric { shape: shape.clone(), first: d1, second: d2 })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn chamfer_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::ChamferEdges { shape: shape.clone(), edges: edges.to_vec(), distance })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Shell { shape: shape.clone(), thickness, open_faces: open_faces.to_vec() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Draft { shape: shape.clone(), faces: faces.to_vec(), pull_direction, neutral_point, angle })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::OffsetSolid { shape: shape.clone(), distance })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn defeature_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.run_single(BrepOperation::Defeature { shape: shape.clone(), faces: faces.to_vec() })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.run_operation_sync(BrepOperation::Section { solid: solid.clone(), plane_origin, plane_normal })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let mut sides = self.run_operation_sync(BrepOperation::Split { solid: solid.clone(), plane_origin, plane_normal })?.into_iter();
        match (sides.next(), sides.next()) {
            (Some(positive), Some(negative)) => Ok((positive, negative)),
            _ => Err(BrepError::Operation("split produced fewer than two solids".into())),
        }
    }
}

// #endregion 🔖️OneShot

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
