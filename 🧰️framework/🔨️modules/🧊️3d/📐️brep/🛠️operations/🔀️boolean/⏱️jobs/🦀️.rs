//! ⏱️ Resumable plane operations and boolean folds: [`SectionJob`], [`SplitJob`] and
//! [`BooleanFoldJob`] (compound cut and the three patterns). Their one-shot functions
//! ([`section_solid_by_plane`], [`split_solid_by_plane`], [`compound_cut`]) are [`drive`] over these
//! jobs, so each operation has exactly one implementation (ticket
//! `26/09/09/PROCEDURAL-3D-END-TO-END`, lane J). A fold nests one [`BooleanStage`] per operand and
//! reports their units as its own, so the boolean engine's phases stay visible through it.

use super::*;
use crate::brep::operations::primitives::ConvexHullJob;
use crate::brep::operations::staged::{BooleanStage, ChildLedger, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use crate::brep::queries::tessellation::{TessellationJob, TessellationStep};

// #region 🔖️Section

/// ✂️ Resumable [`section_solid_by_plane`]: one unit per solid face collects the in-plane vertices,
/// one unit per solid edge samples its plane crossing, one unit builds the planar face.
pub struct SectionJob {
    origin: Pnt3,
    normal: Vec3,
    tol: f64,
    faces: Vec<FaceId>,
    edges: Vec<EdgeId>,
    seen: HashSet<VertexId>,
    points: Vec<Pnt3>,
    plan: Plan,
}

impl SectionJob {
    /// ✂️ Plans the planar section of `solid` by the plane `(origin, normal)`.
    pub fn new(body: &Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64) -> Result<Self, KernelError> {
        require_tol(tol)?;
        require_solid(body, solid)?;
        let normal = plane_normal(normal)?;
        let faces = body.solid_faces(solid);
        let mut edges: Vec<EdgeId> = faces.iter().flat_map(|&face| body.face_coedges(face)).filter_map(|cid| body.coedges.get(cid).map(|co| co.edge)).collect::<HashSet<_>>().into_iter().collect();
        edges.sort_unstable();
        let plan = Plan::new(&[("vertices", faces.len()), ("edges", edges.len()), ("face", 1)]);
        Ok(Self { origin, normal, tol, faces, edges, seen: HashSet::new(), points: Vec::new(), plan })
    }
}

impl StagedOperation for SectionJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("section job already finished".into()))?;
        match unit.phase {
            0 => {
                for coedge in body.face_coedges(self.faces[unit.index]) {
                    let Some((v0, _)) = body.coedge_endpoints(coedge) else { continue };
                    if self.seen.insert(v0) {
                        if let Some(vertex) = body.vertices.get(v0) {
                            if ((vertex.position - self.origin).dot(self.normal)).abs() <= self.tol * 10.0 {
                                self.points.push(vertex.position);
                            }
                        }
                    }
                }
            }
            1 => {
                let Some(edge) = body.edges.get(self.edges[unit.index]) else { return Ok(StageStep::Working) };
                let Some(v0) = body.vertices.get(edge.v0).map(|v| v.position) else { return Ok(StageStep::Working) };
                let Some(v1) = body.vertices.get(edge.v1).map(|v| v.position) else { return Ok(StageStep::Working) };
                let d0 = (v0 - self.origin).dot(self.normal);
                let d1 = (v1 - self.origin).dot(self.normal);
                if d0 * d1 > 0.0 {
                    return Ok(StageStep::Working);
                }
                let denom = d0 - d1;
                if denom.abs() <= 1e-15 {
                    return Ok(StageStep::Working);
                }
                self.points.push(v0 + (v1 - v0) * (d0 / denom));
            }
            _ => {
                if self.points.len() < 3 {
                    return Ok(StageStep::Done(StageOutput::Faces(Vec::new())));
                }
                let face = crate::brep::operations::primitives::make_planar_face_from_points(body, &self.points, rec)?;
                return Ok(StageStep::Done(StageOutput::Faces(vec![face])));
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️Section

// #region 🔖️Split

/// ✂️ Triangles classified per classification unit.
const SPLIT_CHUNK: usize = 256;

/// ✂️ Phase indices of a [`SplitJob`] plan.
const TESSELLATE: usize = 0;
const CLASSIFY: usize = 1;
const POSITIVE: usize = 2;
const NEGATIVE: usize = 3;

/// ✂️ One side of a split under construction.
#[derive(Default)]
struct Side {
    triangles: Vec<[Pnt3; 3]>,
    points: Vec<Pnt3>,
    hull: Option<ConvexHullJob>,
    ledger: ChildLedger,
}

/// ✂️ Resumable [`split_solid_by_plane`]: a nested [`TessellationJob`]'s units, one unit per chunk of
/// classified triangles, then one unit per side that builds its solid from the triangle soup — or,
/// when the soup does not close or the tessellation did not straddle the plane, runs a nested
/// [`ConvexHullJob`] over the side's points.
pub struct SplitJob {
    solid: SolidId,
    origin: Pnt3,
    normal: Vec3,
    tol: f64,
    tessellation: Option<TessellationJob>,
    mesh: Option<MeshTransfer>,
    positive: Side,
    negative: Side,
    hull_only: bool,
    first: Option<SolidId>,
    plan: Plan,
}

impl SplitJob {
    /// ✂️ Plans the split of `solid` by the plane `(origin, normal)`.
    pub fn new(body: &Body, solid: SolidId, origin: Pnt3, normal: Vec3, tol: f64) -> Result<Self, KernelError> {
        require_tol(tol)?;
        require_solid(body, solid)?;
        let normal = plane_normal(normal)?;
        let tessellation = TessellationJob::for_solid(body, solid, tol.max(1e-3))?;
        let plan = Plan::new(&[("tessellate", tessellation.progress().units_total.max(1)), ("classify", 0), ("positive", 1), ("negative", 1)]);
        Ok(Self { solid, origin, normal, tol, tessellation: Some(tessellation), mesh: None, positive: Side::default(), negative: Side::default(), hull_only: false, first: None, plan })
    }

    /// ✂️ Classifies one chunk of triangles by their centroid's side of the plane.
    fn classify(&mut self, index: usize) -> Result<(), KernelError> {
        let mesh = self.mesh.as_ref().ok_or_else(|| KernelError::Operation("split: tessellation did not finish".into()))?;
        let npos = mesh.position.len() / 3;
        let triangles = mesh.index.as_chunks::<3>().0;
        for tri in &triangles[index * SPLIT_CHUNK..((index + 1) * SPLIT_CHUNK).min(triangles.len())] {
            let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            if i0 >= npos || i1 >= npos || i2 >= npos {
                return Err(KernelError::InvalidInput("mesh index out of range".into()));
            }
            let p0 = mesh_position(mesh, i0);
            let p1 = mesh_position(mesh, i1);
            let p2 = mesh_position(mesh, i2);
            let c = Pnt3::new((p0.x + p1.x + p2.x) / 3.0, (p0.y + p1.y + p2.y) / 3.0, (p0.z + p1.z + p2.z) / 3.0);
            let d = (c - self.origin).dot(self.normal);
            if d >= -self.tol {
                self.positive.triangles.push([p0, p1, p2]);
                self.positive.points.extend([p0, p1, p2]);
            }
            if d <= self.tol {
                self.negative.triangles.push([p0, p1, p2]);
                self.negative.points.extend([p0, p1, p2]);
            }
        }
        Ok(())
    }

    /// ✂️ Advances one side's solid by one unit; `Some` once its solid exists.
    fn build_side(body: &mut Body, hull_only: bool, side: &mut Side, plan: &mut Plan, phase: usize, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
        if side.hull.is_none() {
            if !hull_only {
                if let Ok(id) = solid_from_triangle_soup(body, &side.triangles, rec) {
                    return Ok(Some(id));
                }
            }
            let hull = ConvexHullJob::new(&side.points)?;
            side.ledger.admit(plan, phase, &hull);
            side.hull = Some(hull);
        }
        let hull = side.hull.as_mut().expect("hull admitted above");
        let step = hull.advance(body, rec)?;
        side.ledger.follow(plan, phase, hull);
        match step {
            StageStep::Done(StageOutput::Solid(id)) => Ok(Some(id)),
            _ => Ok(None),
        }
    }
}

impl StagedOperation for SplitJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("split job already finished".into()))?;
        match unit.phase {
            TESSELLATE => {
                let job = self.tessellation.as_mut().ok_or_else(|| KernelError::Operation("split: tessellation already finished".into()))?;
                if let TessellationStep::Done(_) = job.step(body, 1)? {
                    let (mesh, _) = self.tessellation.take().and_then(TessellationJob::into_mesh).ok_or_else(|| KernelError::Operation("split: tessellation produced no mesh".into()))?;
                    if mesh.index.len() % 3 != 0 {
                        return Err(KernelError::InvalidInput("mesh index length must be a multiple of 3".into()));
                    }
                    self.plan.grow(CLASSIFY, (mesh.index.len() / 3).div_ceil(SPLIT_CHUNK));
                    self.mesh = Some(mesh);
                }
            }
            CLASSIFY => self.classify(unit.index)?,
            POSITIVE => {
                if unit.index == 0 {
                    self.mesh = None;
                    self.hull_only = self.positive.triangles.is_empty() || self.negative.triangles.is_empty();
                    if self.hull_only {
                        let (origin, normal, tol) = (self.origin, self.normal, self.tol);
                        let vertices = solid_vertex_positions(body, self.solid);
                        self.positive.points = vertices.iter().copied().filter(|p| (*p - origin).dot(normal) >= -tol).collect();
                        self.negative.points = vertices.iter().copied().filter(|p| (*p - origin).dot(normal) <= tol).collect();
                        if self.positive.points.len() < 4 || self.negative.points.len() < 4 {
                            return Err(KernelError::Boolean(BooleanError::InvalidResult("split_solid_by_plane: one side has too few points".into())));
                        }
                    }
                }
                if let Some(solid) = Self::build_side(body, self.hull_only, &mut self.positive, &mut self.plan, POSITIVE, rec)? {
                    self.first = Some(solid);
                }
            }
            _ => {
                let Some(positive) = self.first else { return Err(KernelError::Operation("split: the positive side was not built".into())) };
                if let Some(negative) = Self::build_side(body, self.hull_only, &mut self.negative, &mut self.plan, NEGATIVE, rec)? {
                    return Ok(StageStep::Done(StageOutput::Solids(vec![positive, negative])));
                }
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️Split

// #region 🔖️Fold

/// 🔀 Where each fold operand comes from.
enum Operand {
    /// 🔀 An existing solid the fold must never free.
    Existing(SolidId),
    /// 🔀 A transformed copy of the fold's base solid, freed once folded in.
    Transformed(Affine3),
}

/// 🔀 Resumable fold of one boolean over a list of operands: compound cut (successive
/// [`BooleanOp::Cut`] of tools from a target) and the linear, circular and grid patterns (successive
/// [`BooleanOp::Unite`] of transformed copies of a shape). Every intermediate is a solid the fold
/// minted and nobody else can name, so it is freed as soon as the next round supersedes it; the
/// base solid and the existing operands are never touched. One unit per nested boolean unit.
pub struct BooleanFoldJob {
    base: SolidId,
    op: BooleanOp,
    tol: f64,
    operands: Vec<Operand>,
    current: SolidId,
    next_operand: usize,
    child: Option<(BooleanStage, Option<SolidId>)>,
    ledger: ChildLedger,
    plan: Plan,
}

impl BooleanFoldJob {
    /// 🔀 Plans a fold that successively cuts `tools` from `target`.
    pub fn compound_cut(body: &Body, target: SolidId, tools: &[SolidId], tol: f64) -> Result<Self, KernelError> {
        require_tol(tol)?;
        require_solid(body, target)?;
        if tools.is_empty() {
            return Err(KernelError::InvalidInput("compound_cut requires at least one tool solid".into()));
        }
        Ok(Self::fold(target, BooleanOp::Cut, tol, tools.iter().map(|&tool| Operand::Existing(tool)).collect()))
    }

    /// 🔀 Plans a fold that successively unites `shape` with its copies under `transforms`.
    pub fn pattern(body: &Body, shape: SolidId, transforms: Vec<Affine3>, tol: f64) -> Result<Self, KernelError> {
        require_tol(tol)?;
        require_solid(body, shape)?;
        Ok(Self::fold(shape, BooleanOp::Unite, tol, transforms.into_iter().map(Operand::Transformed).collect()))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fold(base: SolidId, op: BooleanOp, tol: f64, operands: Vec<Operand>) -> Self {
        let plan = Plan::new(&[("fold", operands.len())]);
        Self { base, op, tol, operands, current: base, next_operand: 0, child: None, ledger: ChildLedger::default(), plan }
    }
}

impl StagedOperation for BooleanFoldJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        self.plan.take().ok_or_else(|| KernelError::Operation("boolean fold already finished".into()))?;
        if self.child.is_none() {
            let (operand, minted) = match &self.operands[self.next_operand] {
                Operand::Existing(solid) => (*solid, None),
                Operand::Transformed(map) => {
                    let copy = transform_solid(body, self.base, map, rec)?;
                    (copy, Some(copy))
                }
            };
            let stage = BooleanStage::new(body, self.current, operand, self.op, self.tol, rec)?;
            self.ledger = ChildLedger::default();
            self.ledger.admit(&mut self.plan, 0, &stage);
            self.child = Some((stage, minted));
        }
        let (stage, _) = self.child.as_mut().expect("operand admitted above");
        let step = stage.advance(body, rec)?;
        self.ledger.follow(&mut self.plan, 0, stage);
        let StageStep::Done(StageOutput::Solid(folded)) = step else {
            return Ok(StageStep::Working);
        };
        let (_, minted) = self.child.take().expect("operand admitted above");
        if let Some(copy) = minted {
            remove_solid_and_orphans(body, copy, &HashSet::new(), rec);
        }
        if self.current != self.base {
            remove_solid_and_orphans(body, self.current, &HashSet::new(), rec);
        }
        self.current = folded;
        self.next_operand += 1;
        if self.next_operand == self.operands.len() {
            self.plan.settle();
            return Ok(StageStep::Done(StageOutput::Solid(self.current)));
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️Fold
