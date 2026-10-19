//! ⏱️ Blend job: the member and minted records, the pcurve branch alignment, the phase indices and the resumable blend job with its staged operation. See [`super`].

use super::*;

// #region 🔖️Surgery
/// 🎨️ One coedge of a face being built: the edge, its traversal sense, and the exact p-curve that
/// edge carries ON THIS face.
pub(super) struct Member {
    pub(super) edge: EdgeId,
    pub(super) forward: bool,
    pub(super) pcurve: Curve2,
    pub(super) prange: (f64, f64),
}

/// 🎨️ Everything the assembly mints, keyed so both faces of a shared edge reach the same entity.
pub(super) struct Minted {
    pub(super) vertices: BTreeMap<VertexId, VertexId>,
    pub(super) edges: BTreeMap<EdgeId, EdgeId>,
    pub(super) ranges: BTreeMap<EdgeId, (f64, f64)>,
    pub(super) tangency: BTreeMap<(EdgeId, usize), EdgeId>,
    pub(super) ends: BTreeMap<(EdgeId, usize), EdgeId>,
}

/// 🎨️ Shifts each member's p-curve by whole periods so the ring is CONTINUOUS in the face's own
/// `(u, v)`, chaining every member's start onto its predecessor's end.
///
/// Each p-curve is derived independently, and an inversion on a periodic surface answers in
/// whichever branch its own `atan2` landed in — so a corner patch's three arcs could come back at
/// `u = 0`, `u = −π/2` and `u = 3π/2`, all three geometrically right and the polygon between them
/// three times too wide, winding backwards. A pole is exempt on purpose: two meridians meeting
/// there legitimately carry different `u`, and shifting one onto the other would collapse the lune
/// — the chain simply carries the arriving branch forward, which is what the departing arc's own
/// `u` is then measured against.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn align_pcurve_branches(surface: &Surface, members: &mut [Member]) {
    let (u_periodic, v_periodic) = (surface.is_u_periodic(), surface.is_v_periodic());
    if !u_periodic && !v_periodic {
        return;
    }
    let mut previous: Option<Pnt2> = None;
    for member in members.iter_mut() {
        let (start_at, end_at) = if member.forward { (member.prange.0, member.prange.1) } else { (member.prange.1, member.prange.0) };
        if let Some(previous) = previous {
            let start = member.pcurve.eval(start_at);
            let du = if u_periodic { ((previous.x - start.x) / TAU).round() * TAU } else { 0.0 };
            let dv = if v_periodic { ((previous.y - start.y) / TAU).round() * TAU } else { 0.0 };
            if du != 0.0 || dv != 0.0 {
                member.pcurve = member.pcurve.translated(Vec2::new(du, dv));
            }
        }
        previous = Some(member.pcurve.eval(end_at));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn attach_with_pcurves(body: &mut Body, surface: SurfaceId, members: &mut [Member], flipped: bool, rec: &mut OpRecorder) -> FaceId {
    let support = body.surfaces.get(surface).expect("face surface").clone();
    align_pcurve_branches(&support, members);
    let pairs: Vec<(EdgeId, bool)> = members.iter().map(|m| (m.edge, m.forward)).collect();
    let outer = make_loop(body, FaceId::from_raw(0, 0), &pairs);
    let face = add_face(body, surface, Some(outer), vec![], flipped, Tol::DEFAULT, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    for (cid, member) in body.loop_coedges(outer).into_iter().zip(members) {
        let pcurve = body.curves2.insert(member.pcurve.clone());
        let coedge = body.coedges.get_mut(cid).unwrap();
        coedge.pcurve = Some(pcurve);
        coedge.prange = member.prange;
    }
    face
}

/// 🎨️ Phase indices of a [`BlendJob`] plan, in execution order.
const PATCHES: usize = 0;

const CORNERS: usize = 1;

const TRIM: usize = 2;

const CORNER_VERTICES: usize = 3;

const CARRIED: usize = 4;

const SEAMS: usize = 5;

const FACES: usize = 6;

const PATCH_FACES: usize = 7;

const CORNER_FACES: usize = 8;

/// ⏱️ The blend engine as a resumable job: one unit builds one patch (per selected edge), classifies
/// one corner (per reached vertex), trims one patch, mints one corner's vertices, carries one
/// unblended edge, mints one patch's seam edges, rebuilds one original face, assembles one patch
/// face or one corner face, and finally closes the shell. The input solid is never touched — every
/// entity of the result is freshly minted — so a job that is abandoned leaves only unreachable
/// garbage in the body it ran in.
pub struct BlendJob {
    pub(super) spec: BlendSpec,
    pub(super) solid_faces: BTreeSet<FaceId>,
    selected: Vec<EdgeId>,
    pub(super) incident: Vec<(VertexId, Vec<EdgeId>)>,
    carried: Vec<EdgeId>,
    face_order: Vec<FaceId>,
    pub(super) patches: BTreeMap<EdgeId, Patch>,
    pub(super) corners: BTreeMap<VertexId, Corner>,
    pub(super) minted: Minted,
    pub(super) faces: Vec<FaceId>,
    plan: Plan,
}

impl BlendJob {
    /// 🎨️ Plans a constant-radius fillet of `edges` of `solid` after validating the request.
    pub fn fillet(body: &Body, solid: SolidId, edges: &[EdgeId], radius: f64) -> Result<Self, KernelError> {
        validate_blend_request(body, solid, edges, radius)?;
        Self::plan(body, solid, edges, BlendSpec::Fillet(radius))
    }

    /// 🎨️ Plans a linearly varying fillet of one `edge` after validating the request.
    pub fn variable_fillet(body: &Body, solid: SolidId, edge: EdgeId, r0: f64, r1: f64) -> Result<Self, KernelError> {
        if r0 <= 0.0 || r1 <= 0.0 {
            return Err(KernelError::InvalidInput("variable fillet radii must be positive".into()));
        }
        validate_blend_request(body, solid, &[edge], r0.max(r1))?;
        Self::plan(body, solid, &[edge], BlendSpec::VariableFillet(r0, r1))
    }

    /// 🎨️ Plans an asymmetric chamfer of `edges` of `solid` after validating the request.
    pub fn chamfer(body: &Body, solid: SolidId, edges: &[EdgeId], d1: f64, d2: f64) -> Result<Self, KernelError> {
        validate_blend_request(body, solid, edges, d1.min(d2))?;
        if !(d2.is_finite() && d2 > 0.0) {
            return Err(KernelError::InvalidInput("chamfer distance must be positive".into()));
        }
        Self::plan(body, solid, edges, BlendSpec::Chamfer(d1, d2))
    }

    fn plan(body: &Body, solid: SolidId, edges: &[EdgeId], spec: BlendSpec) -> Result<Self, KernelError> {
        let solid_faces: BTreeSet<FaceId> = body.solid_faces(solid).into_iter().collect();
        let selected_set: BTreeSet<EdgeId> = edges.iter().copied().collect();
        let mut incident: BTreeMap<VertexId, Vec<EdgeId>> = BTreeMap::new();
        for &edge in &selected_set {
            let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
            incident.entry(ent.v0).or_default().push(edge);
            if ent.v1 != ent.v0 {
                incident.entry(ent.v1).or_default().push(edge);
            }
        }
        let carried: Vec<EdgeId> = solid_edge_set(body, solid).into_iter().filter(|edge| !selected_set.contains(edge)).collect();
        let selected: Vec<EdgeId> = selected_set.into_iter().collect();
        let face_order: Vec<FaceId> = solid_faces.iter().copied().collect();
        let plan = Plan::new(&[
            ("patches", selected.len()),
            ("corners", incident.len()),
            ("trim", selected.len()),
            ("corner-vertices", incident.len()),
            ("carried-edges", carried.len()),
            ("seams", selected.len()),
            ("faces", face_order.len()),
            ("patch-faces", selected.len()),
            ("corner-faces", incident.len()),
            ("assemble", 1),
        ]);
        let minted = Minted { vertices: BTreeMap::new(), edges: BTreeMap::new(), ranges: BTreeMap::new(), tangency: BTreeMap::new(), ends: BTreeMap::new() };
        Ok(Self { spec, solid_faces, selected, incident: incident.into_iter().collect(), carried, face_order, patches: BTreeMap::new(), corners: BTreeMap::new(), minted, faces: Vec::new(), plan })
    }
}

impl StagedOperation for BlendJob {
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("blend job already finished".into()))?;
        match unit.phase {
            PATCHES => {
                let edge = self.selected[unit.index];
                self.patches.insert(edge, build_patch(body, edge, &self.solid_faces, self.spec)?);
            }
            CORNERS => {
                let (vertex, here) = &self.incident[unit.index];
                self.corners.insert(*vertex, build_corner(body, *vertex, here, &self.patches, &self.solid_faces, self.spec)?);
            }
            TRIM => {
                let edge = self.selected[unit.index];
                let patch = self.patches.get_mut(&edge).expect("patch built before trimming");
                trim_patch(body, edge, patch, &self.corners)?;
            }
            CORNER_VERTICES => {
                let corner = self.corners.get_mut(&self.incident[unit.index].0).expect("corner built before minting");
                let points: Vec<(FaceId, Pnt3)> = corner.points.iter().map(|(f, p)| (*f, *p)).collect();
                for (face, point) in points {
                    let id = make_vertex(body, point, Tol::DEFAULT, rec);
                    corner.verts.insert(face, id);
                }
            }
            CARRIED => mint_carried_edge(body, self.carried[unit.index], &self.corners, &self.solid_faces, &mut self.minted, rec)?,
            SEAMS => {
                let edge = self.selected[unit.index];
                let patch = &self.patches[&edge];
                for i in 0..2 {
                    let curve_id = body.curves3.insert(patch.tangency(i));
                    let v0 = self.corners[&patch.end_vertex[0]].verts[&patch.faces[i]];
                    let v1 = self.corners[&patch.end_vertex[1]].verts[&patch.faces[i]];
                    let fresh = make_edge(body, curve_id, patch.run[i], v0, v1, Tol::DEFAULT, rec);
                    self.minted.tangency.insert((edge, i), fresh);
                }
                for slot in 0..2 {
                    if patch.closed && slot == 1 {
                        let seam = self.minted.ends[&(edge, 0)];
                        self.minted.ends.insert((edge, 1), seam);
                        continue;
                    }
                    let (curve, range) = patch.end_curve(slot);
                    let curve_id = body.curves3.insert(curve);
                    let v0 = self.corners[&patch.end_vertex[slot]].verts[&patch.faces[0]];
                    let v1 = self.corners[&patch.end_vertex[slot]].verts[&patch.faces[1]];
                    let fresh = make_edge(body, curve_id, range, v0, v1, Tol::DEFAULT, rec);
                    self.minted.ends.insert((edge, slot), fresh);
                }
            }
            FACES => self.faces.push(rebuild_face(body, self.face_order[unit.index], &self.patches, &self.corners, &self.minted, rec)?),
            PATCH_FACES => {
                let edge = self.selected[unit.index];
                self.faces.push(build_patch_face(body, edge, &self.patches[&edge], &self.minted, rec)?);
            }
            CORNER_FACES => {
                let vertex = self.incident[unit.index].0;
                if self.corners[&vertex].corner_surface.is_some() {
                    self.faces.push(build_corner_face(body, vertex, &self.corners[&vertex], &self.patches, &self.minted, rec)?);
                }
            }
            _ => {
                let shell = add_shell(body, std::mem::take(&mut self.faces), rec);
                return Ok(StageStep::Done(StageOutput::Solid(add_solid(body, shell, vec![], rec))));
            }
        }
        Ok(StageStep::Working)
    }
}
// #endregion 🔖️Surgery
