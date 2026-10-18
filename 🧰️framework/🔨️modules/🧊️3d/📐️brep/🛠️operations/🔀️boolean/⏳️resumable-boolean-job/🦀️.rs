//! ⏳️ Resumable boolean job: the boolean phases, progress, steps and admission, the boolean job with its stepping and the job constructor. See [`super`].

use super::*;

// #region ⏱️ResumableBoolean
/// ⏱️ What a [`BooleanJob`] is currently doing. Phases run in declaration order; `Complete` and
/// `Cancelled` are terminal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BooleanPhase {
    /// ✂️ Imprinting one (face-of-A, face-of-B) pair per unit.
    #[default]
    Imprint,
    /// 🧩 Applying A's queued imprints, one face per unit.
    ApplyA,
    /// 🧩 Applying B's queued imprints, one face per unit.
    ApplyB,
    /// 🎯 Classifying one piece of A against B per unit.
    ClassifyA,
    /// 🎯 Classifying one piece of B against A per unit.
    ClassifyB,
    /// 🧵 Stitching the selected faces into shells/solids and retiring the operands.
    Stitch,
    /// 🩺 Validating the result, one [`BodyValidationJob`] unit per unit.
    Validate,
    Complete,
    Cancelled,
}

impl BooleanPhase {
    /// 🏷️ The stable wire tag every host/extension/UI layer names this phase by.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn tag(self) -> &'static str {
        match self {
            Self::Imprint => "imprint",
            Self::ApplyA => "applyA",
            Self::ApplyB => "applyB",
            Self::ClassifyA => "classifyA",
            Self::ClassifyB => "classifyB",
            Self::Stitch => "stitch",
            Self::Validate => "validate",
            Self::Complete => "complete",
            Self::Cancelled => "cancelled",
        }
    }
}

/// 📈 Progress of one resumable boolean. `units_done` never decreases. `units_total` is the plan
/// known SO FAR and is revised upward exactly once — when the stitch lands and the result's
/// validation plan (how many entities there are to check) can finally be read; nothing about the
/// post-stitch body is knowable before the stitch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BooleanProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub faces_done: usize,
    pub faces_total: usize,
    pub phase: BooleanPhase,
}

/// ⏱️ Outcome of one budgeted [`BooleanJob::step`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanStep {
    /// 🔁 Budget spent, work remains — call `step` again.
    Working(BooleanProgress),
    /// ✅ Terminal: the result solid is live in the body.
    Done(SolidId),
    /// 🛑 Terminal: [`BooleanJob::cancel`] retired the job; no solid is produced.
    Cancelled(BooleanProgress),
}

/// 🔀 What [`boolean_job`] admitted: either a fast path already answered (microseconds, nothing to
/// resume), or the general engine owes a resumable job.
pub enum BooleanAdmission {
    /// ⚡️ A trivial/box fast path answered exactly, in place.
    Answered(SolidId),
    /// ⏱️ The general exact engine, as a budgetable job.
    Job(BooleanJob),
}

/// ⏱️ The general exact imprint→classify→select→stitch→validate pipeline split into budgetable
/// units, so a host can run it inside an interactive step ceiling across many turns, report
/// progress, and cancel it — the boolean twin of
/// [`semio_framework_3d_brep_tessellation::TessellationJob`].
///
/// One unit is one face PAIR (imprint), one face (apply/classify) or one
/// [`BodyValidationJob`] unit (validate). Measured on `🍩️sphere-cut-with-torus` in a native debug
/// build the whole operation costs 4.5 s, of which the stitch is 1.5 s and the validation 3.0 s —
/// so the validation's per-face units are the ones that actually make this preemptible, and the
/// stitch (one unit, group + signed-volume probe + containment) is the coarsest step this design
/// admits (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
///
/// 🚧️ A cancelled or failed job leaves the imprints it already applied in the body. Those imprints
/// live on the job's OWN working copies of the operands ([`BooleanJob::new`]), never on the
/// operands themselves, so a cancelled boolean leaves its inputs byte-identical and leaves behind
/// only unreachable garbage the next compaction reclaims; nothing is rolled back, because the
/// kernel has no transaction.
pub struct BooleanJob {
    pub(super) a: SolidId,
    pub(super) b: SolidId,
    pub(super) op: BooleanOp,
    pub(super) tol: f64,
    pub(super) pre_existing_solids: HashSet<SolidId>,
    /// ♻️ Every edge/vertex that already existed when this job was planned. [`gc_orphan_edges_and_vertices`]
    /// may free only what the job itself minted: a bare wire's edges carry no coedge at all, so a
    /// whole-arena orphan sweep would eat every live `Wire` handle's geometry as collateral.
    pub(super) protected_edges: HashSet<EdgeId>,
    pub(super) protected_vertices: HashSet<VertexId>,
    coincident_a: HashMap<FaceId, bool>,
    /// 🧱 Pieces of B found coincident with a piece of A AFTER imprinting (a tool face lying within a
    /// stock face — the flush notch), dropped in `ClassifyB` the way a whole coincident B face is
    /// dropped up front; their A twins join `coincident_a`.
    coincident_b: HashSet<FaceId>,
    pub(super) faces_a: Vec<FaceId>,
    pub(super) faces_b: Vec<FaceId>,
    /// 🔗 Per-face-of-A support cache: the surface and AABB the whole `fb` row is tested against,
    /// read once per row instead of once per pair.
    row: Option<(Surface, semio_framework_3d_brep_representation::contract::Aabb)>,
    pub(super) weld: Vec<(Pnt3, VertexId)>,
    pub(super) pending_a: HashMap<FaceId, Vec<Pending>>,
    pub(super) pending_b: HashMap<FaceId, Vec<Pending>>,
    pieces_a: Vec<FaceId>,
    pieces_b: Vec<FaceId>,
    pub(super) selected: Vec<FaceId>,
    pub(super) result: Option<SolidId>,
    validation: Option<BodyValidationJob>,
    cursor_a: usize,
    cursor_b: usize,
    apply_a: usize,
    apply_b: usize,
    classify_a: usize,
    classify_b: usize,
    pub(super) units_done: usize,
    pub(super) units_total: usize,
    pub(super) phase: BooleanPhase,
}

impl BooleanJob {
    /// 🎟️ Runs the existing result-validation producer on its actual supplied caller turn.
    pub fn step(&mut self, body:&mut Body, _rec:&mut OpRecorder, cx:&mut semio_framework_job::StepContext<'_>)->Result<BooleanStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        if self.phase!=BooleanPhase::Validate{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original boolean phase has no funded producer yet").into())}
        let validation=self.validation.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original boolean result lost its validation producer"))?;
        let grant=cx.retained_grant();let demand=validation.funded_demands(body)?;
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(BooleanStep::Working(self.progress()))}
        cx.consume_fuel(1);
        let(receipt,units)=validation.step_funded(body,1,grant).map_err(|primary|semio_framework_job::OriginalTurnFailure::debit_original(primary,cx))?;
        cx.consume_retained(receipt).map_err(|primary|semio_framework_job::OriginalTurnFailure::from(primary.with_retained_progress(receipt)))?;
        self.units_done+=units;self.units_total=self.units_total.max(self.units_done.saturating_add(validation.progress().units_total.saturating_sub(validation.progress().units_done)));
        Ok(BooleanStep::Working(self.progress()))
    }

    /// 🔀 Plans the general exact boolean of `a` and `b`. Every plan input (which faces exist,
    /// which pairs are coincident, the weld table) is read ONCE here, so the walk below is a pure
    /// cursor advance.
    ///
    /// 🐛 The engine imprints DESTRUCTIVELY (it splits the faces it classifies) and the stitch then
    /// drops whatever the selection did not keep, so the plan runs on deep copies of both operands
    /// ([`copy_solid`]) and the operands themselves are never touched. Without that, one
    /// `brep.bool.fuse`/`brep.bool.cut` freed the arena entities of BOTH its inputs, and since the
    /// evaluator serves an unchanged input node from cache, the next evaluation of the same graph
    /// answered `missing handle: <digest>` for a node whose own parameters never moved
    /// (`📓️brep-boolean-input-lifetime-2026-09-15.md`).
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn new(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Self, KernelError> {
        let pre_existing_solids: HashSet<SolidId> = body.solids.iter().map(|(id, _)| id).collect();
        let protected_edges: HashSet<EdgeId> = body.edges.iter().map(|(id, _)| id).collect();
        let protected_vertices: HashSet<VertexId> = body.vertices.iter().map(|(id, _)| id).collect();
        let a = copy_solid(body, a, rec)?;
        let b = copy_solid(body, b, rec)?;
        let faces_a_all = body.solid_faces(a);
        let faces_b_all = body.solid_faces(b);
        let coincident = find_coincident_face_pairs(body, &faces_a_all, &faces_b_all, tol);
        let coincident_b: HashSet<FaceId> = coincident.iter().map(|&(_, fb, _)| fb).collect();
        let coincident_a: HashMap<FaceId, bool> = coincident.iter().map(|&(fa, _, aligned)| (fa, aligned)).collect();
        let faces_b: Vec<FaceId> = faces_b_all.iter().copied().filter(|f| !coincident_b.contains(f)).collect();
        let faces_a: Vec<FaceId> = faces_a_all;
        // 🔗 Imprint endpoints are welded against each other AND against both operands' existing
        // boundary vertices: an intersection segment that ends exactly on a pole (a plane through a
        // sphere's centre ends its arc there) must reuse that pole's vertex, otherwise the chain's
        // end is a fresh id that `splice_boundary_vertex` cannot find on the ring.
        let mut weld: Vec<(Pnt3, VertexId)> = Vec::new();
        for &face in faces_a.iter().chain(faces_b_all.iter()) {
            for coedge_id in body.face_coedges(face) {
                let Some((vertex_id, _)) = body.coedge_endpoints(coedge_id) else { continue };
                let Some(vertex) = body.vertices.get(vertex_id) else { continue };
                if !weld.iter().any(|&(_, existing)| existing == vertex_id) {
                    weld.push((vertex.position, vertex_id));
                }
            }
        }
        let units_total = faces_a.len() * faces_b.len() + 2 * (faces_a.len() + faces_b.len()) + 1;
        Ok(Self {
            a,
            b,
            op,
            tol,
            pre_existing_solids,
            protected_edges,
            protected_vertices,
            coincident_a,
            coincident_b: HashSet::new(),
            faces_a,
            faces_b,
            row: None,
            weld,
            pending_a: HashMap::new(),
            pending_b: HashMap::new(),
            pieces_a: Vec::new(),
            pieces_b: Vec::new(),
            selected: Vec::new(),
            result: None,
            validation: None,
            cursor_a: 0,
            cursor_b: 0,
            apply_a: 0,
            apply_b: 0,
            classify_a: 0,
            classify_b: 0,
            units_done: 0,
            units_total,
            phase: BooleanPhase::Imprint,
        })
    }

    /// 📈 This job's progress right now — safe to read between steps and after termination.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn progress(&self) -> BooleanProgress {
        BooleanProgress {
            units_done: self.units_done,
            units_total: self.units_total.max(self.units_done),
            faces_done: self.apply_a + self.apply_b + self.classify_a + self.classify_b,
            faces_total: 2 * (self.faces_a.len() + self.faces_b.len()),
            phase: self.phase,
        }
    }

    /// 🛑 Retires this job at the next observable boundary. A completed job is never cancelled —
    /// supersession may only retire work still in flight, never destroy a result already paid for
    /// (the same law `TessellationJob::cancel` states).
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn cancel(&mut self) {
        if matches!(self.phase, BooleanPhase::Complete) {
            return;
        }
        self.phase = BooleanPhase::Cancelled;
        self.pending_a.clear();
        self.pending_b.clear();
        self.weld.clear();
        self.validation = None;
    }

    /// ✅ True once the job reached a terminal phase.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn is_terminal(&self) -> bool {
        matches!(self.phase, BooleanPhase::Complete | BooleanPhase::Cancelled)
    }

    /// ♾️ Runs every remaining unit in one call — the unbudgeted façade [`boolean_solid`] is.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn run_to_completion(mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
        loop {
            match self.step_unbudgeted(body, rec, usize::MAX)? {
                BooleanStep::Done(id) => return Ok(id),
                BooleanStep::Cancelled(_) => return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean cancelled".into()))),
                BooleanStep::Working(_) => continue,
            }
        }
    }

    /// ⏱️ Advances by at most `budget` units. A `budget` of zero is a legal progress probe that
    /// performs no work.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub(crate) fn step_unbudgeted(&mut self, body: &mut Body, rec: &mut OpRecorder, budget: usize) -> Result<BooleanStep, KernelError> {
        match self.phase {
            BooleanPhase::Complete => return Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
            BooleanPhase::Cancelled => return Ok(BooleanStep::Cancelled(self.progress())),
            _ => {}
        }
        let mut spent = 0usize;
        while spent < budget {
            match self.phase {
                BooleanPhase::Imprint => {
                    if self.cursor_a >= self.faces_a.len() {
                        self.phase = BooleanPhase::ApplyA;
                        continue;
                    }
                    let fa = self.faces_a[self.cursor_a];
                    if self.coincident_a.contains_key(&fa) || self.faces_b.is_empty() {
                        self.cursor_a += 1;
                        self.cursor_b = 0;
                        self.row = None;
                        continue;
                    }
                    if self.row.is_none() {
                        let Some(face_a) = body.faces.get(fa).cloned() else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        let Some(sa) = body.surfaces.get(face_a.surface).cloned() else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        let Ok(bb_a) = face_aabb(body, fa) else {
                            self.cursor_a += 1;
                            self.cursor_b = 0;
                            continue;
                        };
                        self.row = Some((sa, bb_a));
                    }
                    if self.cursor_b >= self.faces_b.len() {
                        self.cursor_a += 1;
                        self.cursor_b = 0;
                        self.row = None;
                        continue;
                    }
                    let fb = self.faces_b[self.cursor_b];
                    let (sa, bb_a) = self.row.take().expect("row cached above");
                    let outcome = imprint_face_pair(body, (fa, fb), (&sa, &bb_a), self.tol, &mut self.weld, (&mut self.pending_a, &mut self.pending_b), rec);
                    self.row = Some((sa, bb_a));
                    outcome?;
                    self.cursor_b += 1;
                }
                BooleanPhase::ApplyA => {
                    if self.apply_a >= self.faces_a.len() {
                        self.phase = BooleanPhase::ApplyB;
                        continue;
                    }
                    let fa = self.faces_a[self.apply_a];
                    if self.coincident_a.contains_key(&fa) {
                        self.pieces_a.push(fa);
                    } else {
                        match self.pending_a.remove(&fa) {
                            Some(list) => {
                                let pieces = arrangement::split_face(body, fa, &list, self.tol, rec)?;
                                self.pieces_a.extend(pieces);
                            }
                            None => self.pieces_a.push(fa),
                        }
                    }
                    self.apply_a += 1;
                }
                BooleanPhase::ApplyB => {
                    if self.apply_b >= self.faces_b.len() {
                        self.phase = BooleanPhase::ClassifyA;
                        continue;
                    }
                    let fb = self.faces_b[self.apply_b];
                    match self.pending_b.remove(&fb) {
                        Some(list) => {
                            let pieces = arrangement::split_face(body, fb, &list, self.tol, rec)?;
                            self.pieces_b.extend(pieces);
                        }
                        None => self.pieces_b.push(fb),
                    }
                    self.apply_b += 1;
                }
                BooleanPhase::ClassifyA => {
                    if self.classify_a == 0 {
                        // 🧱 Imprinting can carve a piece of A that coincides exactly with a piece of B
                        // (the stock's top face over a flush notch, its side faces beside it). Such a
                        // pair is one boundary, not two: A's piece follows the whole-face coincident
                        // rule below and B's piece is dropped, or both survived as duplicate topology
                        // (edges used by three coedges) and the stitch failed validation.
                        for (fa, fb, aligned) in find_coincident_face_pairs(body, &self.pieces_a, &self.pieces_b, self.tol) {
                            self.coincident_a.insert(fa, aligned);
                            self.coincident_b.insert(fb);
                        }
                    }
                    if self.classify_a >= self.pieces_a.len() {
                        self.phase = BooleanPhase::ClassifyB;
                        continue;
                    }
                    let f = self.pieces_a[self.classify_a];
                    if let Some(&aligned) = self.coincident_a.get(&f) {
                        if keep_coincident_boundary(self.op, aligned) {
                            self.selected.push(f);
                        }
                    } else {
                        let class = classify_face_against_solid(body, f, self.b, self.tol)?;
                        if keep_face(self.op, true, class) {
                            self.selected.push(f);
                        }
                    }
                    self.classify_a += 1;
                }
                BooleanPhase::ClassifyB => {
                    if self.classify_b >= self.pieces_b.len() {
                        self.phase = BooleanPhase::Stitch;
                        continue;
                    }
                    let f = self.pieces_b[self.classify_b];
                    if self.coincident_b.contains(&f) {
                        self.classify_b += 1;
                        continue;
                    }
                    let class = classify_face_against_solid(body, f, self.a, self.tol)?;
                    if keep_face(self.op, false, class) {
                        if matches!(self.op, BooleanOp::Cut) {
                            flip_face(body, f);
                        }
                        self.selected.push(f);
                    }
                    self.classify_b += 1;
                }
                BooleanPhase::Stitch => {
                    if self.selected.is_empty() {
                        return Err(KernelError::Boolean(BooleanError::InvalidResult("exact boolean selection kept no faces".into())));
                    }
                    let selected_set: HashSet<FaceId> = self.selected.iter().copied().collect();
                    let result = stitch_selected_faces(body, &self.selected, self.tol, rec)?;
                    remove_solid_and_orphans(body, self.a, &selected_set, rec);
                    remove_solid_and_orphans(body, self.b, &selected_set, rec);
                    gc_orphan_edges_and_vertices(body, (&self.protected_edges, &self.protected_vertices), rec);
                    self.result = Some(result);
                    let validation = BodyValidationJob::new(body);
                    self.units_total = self.units_done + 1 + validation.progress().units_total;
                    self.validation = Some(validation);
                    self.phase = BooleanPhase::Validate;
                    self.units_done += 1;
                    spent += 1;
                    continue;
                }
                BooleanPhase::Validate => {
                    let Some(validation) = self.validation.as_mut() else {
                        self.phase = BooleanPhase::Complete;
                        continue;
                    };
                    if validation.is_complete() {
                        let issues = issues_scoped_to_new_solids(body, &self.pre_existing_solids, self.validation.take().expect("checked above").into_issues());
                        if !issues.is_empty() {
                            let listed: Vec<String> = issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect();
                            return Err(KernelError::Boolean(BooleanError::InvalidResult(format!("exact boolean result failed validation: {} issue(s): {}", issues.len(), listed.join(" | ")))));
                        }
                        self.phase = BooleanPhase::Complete;
                        continue;
                    }
                    let before = validation.progress().units_done;
                    let after = validation.step(body, budget - spent).units_done;
                    let advanced = after.saturating_sub(before).max(1);
                    self.units_done += advanced;
                    self.units_total = self.units_total.max(self.units_done.saturating_add(validation.progress().units_total.saturating_sub(after)));
                    spent += advanced;
                    continue;
                }
                BooleanPhase::Complete => return Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
                BooleanPhase::Cancelled => return Ok(BooleanStep::Cancelled(self.progress())),
            }
            self.units_done += 1;
            spent += 1;
        }
        match self.phase {
            BooleanPhase::Complete => Ok(BooleanStep::Done(self.result.ok_or_else(|| KernelError::Boolean(BooleanError::InvalidResult("completed boolean carries no solid".into())))?)),
            BooleanPhase::Cancelled => Ok(BooleanStep::Cancelled(self.progress())),
            _ => Ok(BooleanStep::Working(self.progress())),
        }
    }
}

/// 🔀 Admits a boolean: runs the microsecond-cheap trivial/box fast paths in place and, when
/// neither applies, hands back the general engine as a resumable [`BooleanJob`]. This is the
/// entry point an interactive host drives; [`boolean_solid`] is the unbudgeted façade over it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn boolean_job(body: &mut Body, a: SolidId, b: SolidId, op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<BooleanAdmission, KernelError> {
    require_tol(tol)?;
    require_solid(body, a)?;
    require_solid(body, b)?;
    if a == b {
        return Err(KernelError::InvalidInput("boolean operands must be distinct solids".into()));
    }
    let bb_a = solid_bounding_box(body, a)?;
    let bb_b = solid_bounding_box(body, b)?;
    if aabb_finite(&bb_a) && aabb_finite(&bb_b) {
        if let Some(id) = trivial_topology_fast_path(body, a, b, (&bb_a, &bb_b), op, tol, rec)? {
            return Ok(BooleanAdmission::Answered(id));
        }
        if let Some(id) = box_fast_path(body, a, b, (&bb_a, &bb_b), op, tol, rec)? {
            return Ok(BooleanAdmission::Answered(id));
        }
    }
    Ok(BooleanAdmission::Job(BooleanJob::new(body, a, b, op, tol, rec)?))
}
// #endregion ⏱️ResumableBoolean
