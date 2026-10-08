//! ⏱️ Resumable offset family: [`OffsetFaceJob`], [`OffsetSolidJob`], [`ThickenJob`], [`ShellJob`] and
//! [`DraftJob`]. Every one is a [`StagedOperation`] whose units are the algorithm's own loops — one
//! offset surface per face, one rebuilt vertex, edge or face per unit of the shared [`Rebuild`]
//! pass, one ruled side per boundary edge — and the one-shot functions of this module are
//! [`drive`] over them, so there is exactly one implementation of each operation (ticket
//! `26/09/09/PROCEDURAL-3D-END-TO-END`, lane J).

use super::*;
use crate::brep::operations::blend::BlendJob;
use crate::brep::operations::staged::{ChildLedger, Plan, StageOutput, StageProgress, StageStep, StagedOperation};
use crate::brep::operations::transform::FaceCopier;
use crate::brep::representation::topology::Face;

// #region 🔖️Rebuild

/// 🧮️ How a [`Rebuild`] decides where a touched vertex and edge land and whether rebuilt faces flip.
enum Policy {
    /// ↔️ Uniform normal offset by `distance`.
    Offset { distance: f64 },
    /// 🐚️ Inward offset by `distance` (negative) with `open` faces keeping their own plane.
    Shell { distance: f64, open: HashSet<FaceId> },
    /// 📐️ Corners solved from the drafted planes themselves.
    Draft,
}

impl Policy {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn flips_new_faces(&self) -> bool {
        matches!(self, Policy::Shell { .. })
    }

    /// 🎯️ Where `vertex` lands given every touched face's outward normal there.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn vertex_target(&self, body: &Body, vertex: VertexId, touched: &[(FaceId, Vec3)], surfaces: &HashMap<FaceId, Surface>) -> Pnt3 {
        match self {
            Policy::Offset { distance } => {
                let normals: Vec<Vec3> = touched.iter().map(|&(_, n)| n).collect();
                body.vertices.get(vertex).unwrap().position + solve_vertex_displacement(&normals, *distance)
            }
            Policy::Shell { distance, open } => {
                let base = body.vertices.get(vertex).unwrap().position;
                let planes: Vec<(Vec3, f64)> = touched.iter().map(|&(f, n)| (n, n.dot(base.to_vec()) + if open.contains(&f) { 0.0 } else { *distance })).collect();
                solve_plane_point(&planes).unwrap_or_else(|| base + solve_vertex_displacement(&touched.iter().map(|&(_, n)| n).collect::<Vec<_>>(), *distance))
            }
            Policy::Draft => {
                let mut planes: Vec<(Vec3, f64)> = Vec::new();
                for &(f, _) in touched {
                    let fd = body.faces.get(f).unwrap();
                    let final_surface = surfaces.get(&f).cloned().unwrap_or_else(|| body.surfaces.get(fd.surface).unwrap().clone());
                    if let Surface::Plane { frame } = final_surface {
                        planes.push((frame.z, frame.z.dot(frame.origin.to_vec())));
                    }
                }
                solve_plane_point(&planes).unwrap_or_else(|| body.vertices.get(vertex).unwrap().position)
            }
        }
    }

    /// 🎯️ The anchor an edge's intersection branch is chosen nearest to.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn edge_target(&self, body: &Body, edge: EdgeId, normal: Vec3) -> Pnt3 {
        let ent = body.edges.get(edge).unwrap();
        let mid = body.curves3.get(ent.curve).unwrap().eval(0.5 * (ent.range.0 + ent.range.1));
        match self {
            Policy::Offset { distance } | Policy::Shell { distance, .. } => mid + normal * *distance,
            Policy::Draft => mid,
        }
    }
}

/// ↔️ Rebuilds a solid's boundary against a per-face surface substitution one vertex, edge and face
/// per unit: a face present in `surfaces` gets that new surface, a face absent keeps its own; every
/// edge touching a changed face — or ending at a moved vertex — is recomputed as the exact
/// intersection of its two adjacent (possibly one unchanged) surfaces, a self-adjacent seam as the
/// changed face's own isocurve trimmed to the repositioned vertices, a degenerate pole edge by
/// re-evaluating its relocated vertex. Only faces in `materialize` are rebuilt into new faces.
struct Rebuild {
    tol: f64,
    policy: Policy,
    surfaces: HashMap<FaceId, Surface>,
    solid_faces: HashSet<FaceId>,
    edge_set: HashSet<EdgeId>,
    vertex_list: Vec<VertexId>,
    edge_list: Vec<EdgeId>,
    face_list: Vec<FaceId>,
    vertex_pos: HashMap<VertexId, Pnt3>,
    out: RebuiltTopology,
}

impl Rebuild {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(body: &Body, solid: SolidId, policy: Policy, materialize: HashSet<FaceId>, tol: f64) -> Self {
        let solid_faces: HashSet<FaceId> = body.solid_faces(solid).into_iter().collect();
        let edge_set = solid_edges(body, &solid_faces);
        let mut vertex_list: Vec<VertexId> = solid_vertices(body, &edge_set).into_iter().collect();
        vertex_list.sort_unstable();
        let mut edge_list: Vec<EdgeId> = edge_set.iter().copied().collect();
        edge_list.sort_unstable();
        let mut face_list: Vec<FaceId> = solid_faces.iter().copied().filter(|face| materialize.contains(face)).collect();
        face_list.sort_unstable();
        Self { tol, policy, surfaces: HashMap::new(), solid_faces, edge_set, vertex_list, edge_list, face_list, vertex_pos: HashMap::new(), out: RebuiltTopology { face_new: HashMap::new(), edge_new: HashMap::new(), vertex_new: HashMap::new() } }
    }

    /// 🧮️ Units per pass: vertices, edges, materialized faces.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn counts(&self) -> [usize; 3] {
        [self.vertex_list.len(), self.edge_list.len(), self.face_list.len()]
    }

    /// 📍️ Pass 1: the new position of one vertex touching a changed face.
    fn vertex(&mut self, body: &mut Body, rec: &mut OpRecorder, index: usize) -> Result<(), KernelError> {
        let v = self.vertex_list[index];
        let mut touched = false;
        let mut touched_faces: Vec<(FaceId, Vec3)> = Vec::new();
        for e in body.vertex_edges(v) {
            if !self.edge_set.contains(&e) {
                continue;
            }
            for f in edge_unique_faces(body, &self.solid_faces, e) {
                if let Some(cid) = coedge_on_face(body, e, f) {
                    if self.surfaces.contains_key(&f) {
                        touched = true;
                    }
                    if !touched_faces.iter().any(|&(ef, _)| ef == f) {
                        touched_faces.push((f, face_normal_at(body, f, cid, Some(v))?));
                    }
                }
            }
        }
        if !touched {
            return Ok(());
        }
        let position = self.policy.vertex_target(body, v, &touched_faces, &self.surfaces);
        self.vertex_pos.insert(v, position);
        let tol_v = body.vertices.get(v).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.tol;
        self.out.vertex_new.insert(v, make_vertex(body, position, tol_v, rec));
        Ok(())
    }

    /// 📏️ Pass 2: one edge touching a changed face or ending at a moved vertex.
    fn edge(&mut self, body: &mut Body, rec: &mut OpRecorder, index: usize) -> Result<(), KernelError> {
        let e = self.edge_list[index];
        let faces_here = edge_unique_faces(body, &self.solid_faces, e);
        let edge_ent = body.edges.get(e).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
        // An edge whose own two faces are both untouched still has to be rebuilt when one of its
        // ENDS moved: the vertex it used to stop at no longer exists on this solid. Skipping those
        // (what this used to do) left the far edges of a drafted box still ending at the ORIGINAL
        // corner while the drafted face ended at the new one — ten vertices where a box has eight —
        // so the result was neither the old shape nor the new one and its volume landed between the
        // two (measured 0.9324 where the trapezoid is 0.8986).
        if !faces_here.iter().any(|f| self.surfaces.contains_key(f)) && !(self.out.vertex_new.contains_key(&edge_ent.v0) || self.out.vertex_new.contains_key(&edge_ent.v1)) {
            return Ok(());
        }
        let orig_curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?.clone();
        let is_degenerate = edge_ent.v0 == edge_ent.v1 && matches!(&orig_curve, Curve3::Line { dir, .. } if dir.norm() < 1e-12);
        let nv0 = self.out.vertex_new.get(&edge_ent.v0).copied().unwrap_or(edge_ent.v0);
        let nv1 = self.out.vertex_new.get(&edge_ent.v1).copied().unwrap_or(edge_ent.v1);

        if is_degenerate {
            let pos = self.vertex_pos.get(&edge_ent.v0).copied().unwrap_or(body.vertices.get(edge_ent.v0).unwrap().position);
            let new_curve = Curve3::Line { origin: pos, dir: Vec3::ZERO };
            let cid = body.curves3.insert(new_curve.clone());
            let ne = make_edge_entry(body, cid, edge_ent.range, nv0, nv0, edge_ent.tol, rec);
            self.out.edge_new.insert(e, (ne, new_curve, edge_ent.range));
            return Ok(());
        }

        if faces_here.len() == 1 {
            let f = faces_here[0];
            let ns = face_surface(body, f, &self.surfaces)?;
            let cid_coedge = coedge_on_face(body, e, f).ok_or_else(|| KernelError::Operation("seam edge missing coedge".into()))?;
            let pid = body.coedges.get(cid_coedge).unwrap().pcurve.ok_or_else(|| KernelError::Operation("seam edge missing pcurve".into()))?;
            let pc = body.curves2.get(pid).ok_or_else(|| KernelError::MissingEntity("pcurve".into()))?.clone();
            let (dir, konst) = match pc {
                Curve2::Line { origin, dir } if dir.x.abs() < 1e-9 => (IsoDirection::U, origin.x),
                Curve2::Line { origin, dir } if dir.y.abs() < 1e-9 => (IsoDirection::V, origin.y),
                _ => return Err(KernelError::Operation("seam edge pcurve is not axis-aligned".into())),
            };
            // The isocurve at the seam's own (unchanged) `u`/`v` constant is exact and already
            // correctly positioned on the NEW surface (e.g. the new radius) — but its own native
            // parametrization has no reason to still line up with `edge_ent.range` (growing/
            // shrinking a solid moves the CAPS a self-adjacent lateral seam spans between, even
            // though the lateral surface's own frame — hence its isocurve's own v=0 origin — does
            // not move at all). Trimming it to the two already-correctly-repositioned vertex
            // targets (same technique the real-dihedral-edge branch below uses) is exact for any
            // op (offset, shell, draft) and needs no per-surface-kind "seam shift" special case —
            // this replaced a previous version that reused the SEAM'S OLD p-curve's `offset`/
            // `scale` verbatim (silently wrong the moment a cap moves, confirmed by a direct debug
            // run: a rebuilt offset cylinder's seam spanned its OLD z-range, not the new one).
            let iso = ns.isocurve(dir, konst);
            let v0_target = self.vertex_pos.get(&edge_ent.v0).copied().unwrap_or(body.vertices.get(edge_ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
            let v1_target = self.vertex_pos.get(&edge_ent.v1).copied().unwrap_or(body.vertices.get(edge_ent.v1).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
            let search_domain = if matches!(iso, Curve3::Line { .. }) { (-1.0e6, 1.0e6) } else { iso.domain() };
            let t0 = closest_parameter(&iso, search_domain, v0_target, self.tol).t;
            let t1 = closest_parameter(&iso, search_domain, v1_target, self.tol).t;
            let new_curve = iso;
            let cid = body.curves3.insert(new_curve.clone());
            let ne = make_edge_entry(body, cid, (t0, t1), nv0, nv1, edge_ent.tol, rec);
            self.out.edge_new.insert(e, (ne, new_curve, (t0, t1)));
            return Ok(());
        }

        if faces_here.len() != 2 {
            return Err(KernelError::Operation("offset/draft edge has an unexpected number of adjacent faces".into()));
        }
        let (fa, fb) = (faces_here[0], faces_here[1]);
        let sa = face_surface(body, fa, &self.surfaces)?;
        let sb = face_surface(body, fb, &self.surfaces)?;
        let candidates: Vec<IntCurve> = intersect_surface_surface(&sa, &sb, self.tol)?;
        if candidates.is_empty() {
            return Err(KernelError::Operation("offset/draft: adjacent offset surfaces do not intersect".into()));
        }
        let mut mid_normals = Vec::new();
        for f in [fa, fb] {
            if let Some(cid) = coedge_on_face(body, e, f) {
                mid_normals.push(face_normal_at(body, f, cid, None)?);
            }
        }
        let mid_n = average_normal(&mid_normals).unwrap_or(Vec3::Z);
        let anchor = self.policy.edge_target(body, e, mid_n);
        let mut best: Option<(&IntCurve, f64)> = None;
        for cand in &candidates {
            let cp = closest_parameter(&cand.curve3, (cand.domain.min, cand.domain.max), anchor, self.tol);
            if best.as_ref().is_none_or(|(_, d)| cp.distance < *d) {
                best = Some((cand, cp.distance));
            }
        }
        let (chosen, _) = best.unwrap();
        let new_curve = chosen.curve3.clone();
        let domain = (chosen.domain.min, chosen.domain.max);
        // A CLOSED edge (`v0 == v1`, e.g. a cylinder cap's own full-circle boundary) has no
        // second vertex to independently project onto the curve, but it still needs `t0` (hence
        // `t1 = t0 + period`) to land at the SHARED VERTEX's own position, not merely anywhere
        // on the curve — this edge's neighbour (a self-adjacent lateral seam, say) starts exactly
        // where this one's `curve.eval(t0)` sits, and `loop_uv_polygon` stitches consecutive
        // coedges by CONTINUITY, not by re-deriving positions; landing `t0` anywhere else (e.g.
        // the SSI candidate's own arbitrary domain start) silently opens a gap in the (u, v)
        // boundary polygon there, corrupting the sampled area/volume (confirmed directly: with
        // `t0 = domain.0` verbatim the rebuilt cylinder's lateral area came out ~15% too high).
        let (t0, t1) = if edge_ent.v0 == edge_ent.v1 {
            let v0_target = self.vertex_pos.get(&edge_ent.v0).copied().unwrap_or(body.vertices.get(edge_ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
            let t0 = closest_parameter(&new_curve, domain, v0_target, self.tol).t;
            (t0, t0 + (domain.1 - domain.0))
        } else {
            let v0_target = self.vertex_pos.get(&edge_ent.v0).copied().unwrap_or(body.vertices.get(edge_ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
            let v1_target = self.vertex_pos.get(&edge_ent.v1).copied().unwrap_or(body.vertices.get(edge_ent.v1).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position);
            (closest_parameter(&new_curve, domain, v0_target, self.tol).t, closest_parameter(&new_curve, domain, v1_target, self.tol).t)
        };
        let cid = body.curves3.insert(new_curve.clone());
        let ne = make_edge_entry(body, cid, (t0, t1), nv0, nv1, edge_ent.tol, rec);
        self.out.edge_new.insert(e, (ne, new_curve, (t0, t1)));
        Ok(())
    }

    /// 🧱️ Pass 3: one materialized face on its new surface and rebuilt boundary.
    fn face(&mut self, body: &mut Body, rec: &mut OpRecorder, index: usize) -> Result<(), KernelError> {
        let f = self.face_list[index];
        let face_data = body.faces.get(f).ok_or_else(|| KernelError::MissingEntity("face".into()))?.clone();
        let ns = face_surface(body, f, &self.surfaces)?;
        let ns_id = body.surfaces.insert(ns.clone());
        let mut loops = Vec::new();
        if let Some(o) = face_data.outer {
            loops.push(o);
        }
        loops.extend(face_data.inners.iter().copied());
        if loops.is_empty() {
            return Err(KernelError::Operation("face has no loops".into()));
        }
        let mut member_lists: Vec<Vec<(EdgeId, bool)>> = Vec::new();
        for lp in &loops {
            let mut members = Vec::new();
            for cid in body.loop_coedges(*lp) {
                let c = body.coedges.get(cid).unwrap();
                let new_edge = self.out.edge_new.get(&c.edge).map_or(c.edge, |(ne, _, _)| *ne);
                members.push((new_edge, c.forward));
            }
            member_lists.push(members);
        }
        let flipped = self.policy.flips_new_faces();
        let new_face = attach_face(body, ns_id, &member_lists[0], flipped, face_data.tol, rec);
        for members in &member_lists[1..] {
            let lp = make_loop(body, new_face, members);
            body.faces.get_mut(new_face).unwrap().inners.push(lp);
        }
        let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
        for lp in &loops {
            for cid in body.loop_coedges(*lp) {
                let c = body.coedges.get(cid).unwrap();
                if let Some((ne, curve, range)) = self.out.edge_new.get(&c.edge) {
                    edge_geom.insert(*ne, (curve.clone(), *range));
                } else {
                    let e = body.edges.get(c.edge).unwrap();
                    let curve = body.curves3.get(e.curve).unwrap().clone();
                    edge_geom.insert(c.edge, (curve, e.range));
                }
            }
        }
        set_face_pcurves(body, new_face, &ns, &edge_geom, self.tol);
        self.out.face_new.insert(f, new_face);
        Ok(())
    }
}

/// 🧮️ Phase indices of the rebuild passes inside a wrapper job's plan, offset by `base`.
const VERTICES: usize = 1;
const EDGES: usize = 2;
const FACES: usize = 3;

// #endregion 🔖️Rebuild

// #region 🔖️OffsetFace

/// ↔️ Resumable [`offset_face`]: one unit offsets the surface, one rebuilds each boundary edge on it,
/// one attaches the new face, one fits its p-curves.
pub struct OffsetFaceJob {
    face_data: Face,
    signed: f64,
    loops: Vec<LoopId>,
    boundary: Vec<CoedgeId>,
    surface: Option<(Surface, crate::brep::representation::arena::SurfaceId, Affine3)>,
    moved_vertices: HashMap<VertexId, VertexId>,
    moved_edges: HashMap<EdgeId, EdgeId>,
    new_face: Option<FaceId>,
    plan: Plan,
}

impl OffsetFaceJob {
    /// ↔️ Plans the offset of `face` by `distance` along its outward normal.
    pub fn new(body: &Body, face: FaceId, distance: f64) -> Result<Self, KernelError> {
        if !distance.is_finite() {
            return Err(KernelError::InvalidInput("offset distance must be finite".into()));
        }
        let face_data = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity(format!("face {face}")))?.clone();
        let mut loops: Vec<LoopId> = Vec::new();
        if let Some(o) = face_data.outer {
            loops.push(o);
        }
        loops.extend(face_data.inners.iter().copied());
        if loops.is_empty() {
            return Err(KernelError::InvalidInput("face has no loops".into()));
        }
        let boundary: Vec<CoedgeId> = loops.iter().flat_map(|lp| body.loop_coedges(*lp)).collect();
        let signed = if face_data.flipped { -distance } else { distance };
        let plan = Plan::new(&[("surface", 1), ("boundary", boundary.len()), ("face", 1), ("pcurves", 1)]);
        Ok(Self { face_data, signed, loops, boundary, surface: None, moved_vertices: HashMap::new(), moved_edges: HashMap::new(), new_face: None, plan })
    }
}

impl StagedOperation for OffsetFaceJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("offset face job already finished".into()))?;
        match unit.phase {
            0 => {
                let surface = body.surfaces.get(self.face_data.surface).ok_or_else(|| KernelError::MissingEntity(format!("surface {}", self.face_data.surface)))?.clone();
                let new_surface = offset_surface(&surface, self.signed, OFFSET_TOL)?;
                let new_surface_id = body.surfaces.insert(new_surface.clone());
                // The offset face's boundary has to live ON the offset surface, so every boundary vertex and
                // edge is rebuilt through the point map that carries the original support onto the offset one
                // ([`offset_point_map`]) rather than shared with the original face. Sharing them — what this
                // used to do — produced a face whose surface had moved but whose rim had not: `thicken_face`
                // then ruled each side between an edge and ITSELF, so all four sides were degenerate,
                // zero-area, and the thickened box measured a volume of exactly 0.
                let map = offset_point_map(&surface, &new_surface).ok_or_else(|| KernelError::Operation(format!("offset_face: no exact boundary map from {surface:?} to its offset")))?;
                self.surface = Some((new_surface, new_surface_id, map));
            }
            1 => {
                let (_, _, map) = self.surface.as_ref().expect("surface offset before boundary");
                let cid = self.boundary[unit.index];
                let coedge = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid:?}")))?.clone();
                if !self.moved_edges.contains_key(&coedge.edge) {
                    let edge = body.edges.get(coedge.edge).ok_or_else(|| KernelError::MissingEntity(format!("edge {:?}", coedge.edge)))?.clone();
                    let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", edge.curve)))?.clone();
                    let moved = curve.transformed(map);
                    for vertex in [edge.v0, edge.v1] {
                        if !self.moved_vertices.contains_key(&vertex) {
                            let source = body.vertices.get(vertex).ok_or_else(|| KernelError::MissingEntity(format!("vertex {vertex}")))?;
                            let (position, tol_v) = (map.apply_point(source.position), source.tol);
                            let created = make_vertex(body, position, tol_v, rec);
                            self.moved_vertices.insert(vertex, created);
                        }
                    }
                    let curve_id = body.curves3.insert(moved);
                    let created = make_edge_entry(body, curve_id, edge.range, self.moved_vertices[&edge.v0], self.moved_vertices[&edge.v1], edge.tol, rec);
                    self.moved_edges.insert(coedge.edge, created);
                }
            }
            2 => {
                let (_, new_surface_id, _) = self.surface.as_ref().expect("surface offset before face");
                let mut member_lists: Vec<Vec<(EdgeId, bool)>> = Vec::new();
                for lp in &self.loops {
                    let mut members = Vec::new();
                    for cid in body.loop_coedges(*lp) {
                        let c = body.coedges.get(cid).ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid:?}")))?;
                        members.push((self.moved_edges[&c.edge], c.forward));
                    }
                    member_lists.push(members);
                }
                let new_face = attach_face(body, *new_surface_id, &member_lists[0], self.face_data.flipped, self.face_data.tol, rec);
                for members in &member_lists[1..] {
                    let lp = make_loop(body, new_face, members);
                    body.faces.get_mut(new_face).unwrap().inners.push(lp);
                }
                self.new_face = Some(new_face);
            }
            _ => {
                let (new_surface, _, _) = self.surface.as_ref().expect("surface offset before p-curves");
                let new_face = self.new_face.expect("face attached before p-curves");
                let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
                for &moved in self.moved_edges.values() {
                    let e = body.edges.get(moved).ok_or_else(|| KernelError::MissingEntity(format!("edge {moved:?}")))?;
                    let curve = body.curves3.get(e.curve).ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", e.curve)))?.clone();
                    edge_geom.insert(moved, (curve, e.range));
                }
                set_face_pcurves(body, new_face, new_surface, &edge_geom, OFFSET_TOL);
                return Ok(StageStep::Done(StageOutput::Face(new_face)));
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️OffsetFace

// #region 🔖️OffsetSolid

/// ↔️ Resumable [`offset_solid_with_corner`]: one unit offsets one face's surface, the shared
/// [`Rebuild`] passes run one vertex, edge and face per unit, one unit closes the sharp solid, and a
/// `Round` corner then hands over to a nested [`BlendJob`] whose units are its own.
pub struct OffsetSolidJob {
    distance: f64,
    corner: OffsetCorner,
    faces: Vec<FaceId>,
    rebuild: Rebuild,
    sharp: Option<SolidId>,
    blend: Option<BlendJob>,
    ledger: ChildLedger,
    plan: Plan,
}

impl OffsetSolidJob {
    /// ↔️ Plans a uniform offset of `solid` by `distance` with an explicit corner policy.
    pub fn new(body: &Body, solid: SolidId, distance: f64, corner: OffsetCorner) -> Result<Self, KernelError> {
        if !distance.is_finite() {
            return Err(KernelError::InvalidInput("offset distance must be finite".into()));
        }
        if body.solids.get(solid).is_none() {
            return Err(KernelError::MissingEntity(format!("solid {solid}")));
        }
        if distance.abs() <= 1e-15 {
            return Err(KernelError::Operation("offset distance must be non-zero".into()));
        }
        let faces = body.solid_faces(solid);
        let rebuild = Rebuild::new(body, solid, Policy::Offset { distance }, faces.iter().copied().collect(), OFFSET_TOL);
        let [vertices, edges, rebuilt_faces] = rebuild.counts();
        let plan = Plan::new(&[("surfaces", faces.len()), ("vertices", vertices), ("edges", edges), ("faces", rebuilt_faces), ("close", 1), ("blend", usize::from(corner == OffsetCorner::Round))]);
        Ok(Self { distance, corner, faces, rebuild, sharp: None, blend: None, ledger: ChildLedger::default(), plan })
    }

    /// ↔️ Plans an offset with the default corner policy of [`offset_solid`].
    pub fn with_default_corner(body: &Body, solid: SolidId, distance: f64) -> Result<Self, KernelError> {
        let corner = if is_planar_only(body, solid) { OffsetCorner::Sharp } else { OffsetCorner::Round };
        Self::new(body, solid, distance, corner)
    }
}

impl StagedOperation for OffsetSolidJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("offset solid job already finished".into()))?;
        match unit.phase {
            0 => {
                let f = self.faces[unit.index];
                let fd = body.faces.get(f).unwrap().clone();
                let surface = body.surfaces.get(fd.surface).unwrap().clone();
                let signed = if fd.flipped { -self.distance } else { self.distance };
                self.rebuild.surfaces.insert(f, offset_surface(&surface, signed, OFFSET_TOL)?);
            }
            VERTICES => self.rebuild.vertex(body, rec, unit.index)?,
            EDGES => self.rebuild.edge(body, rec, unit.index)?,
            FACES => self.rebuild.face(body, rec, unit.index)?,
            4 => {
                let mut faces: Vec<FaceId> = Vec::with_capacity(self.faces.len());
                for f in &self.faces {
                    faces.push(*self.rebuild.out.face_new.get(f).ok_or_else(|| KernelError::Operation("offset_solid: face was not rebuilt".into()))?);
                }
                let sharp = finish_solid(body, faces, rec);
                if self.corner == OffsetCorner::Sharp {
                    return Ok(StageStep::Done(StageOutput::Solid(sharp)));
                }
                self.sharp = Some(sharp);
            }
            _ => {
                let sharp = self.sharp.expect("sharp solid closed before blending");
                if self.blend.is_none() {
                    let round_faces: HashSet<FaceId> = body.solid_faces(sharp).into_iter().collect();
                    // Only real dihedral edges (shared by two *distinct* faces) are meaningful fillet
                    // targets — a self-adjacent seam edge (e.g. a cylinder's own lateral seam) has no
                    // second face to blend against and is skipped.
                    let edges: Vec<EdgeId> = solid_edges(body, &round_faces).into_iter().filter(|&e| edge_unique_faces(body, &round_faces, e).len() == 2).collect();
                    if edges.is_empty() {
                        return Ok(StageStep::Done(StageOutput::Solid(sharp)));
                    }
                    let job = BlendJob::fillet(body, sharp, &edges, self.distance.abs())?;
                    self.ledger.admit(&mut self.plan, 5, &job);
                    self.blend = Some(job);
                }
                let job = self.blend.as_mut().expect("blend admitted above");
                let step = job.advance(body, rec)?;
                self.ledger.follow(&mut self.plan, 5, job);
                return Ok(step);
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️OffsetSolid

// #region 🔖️Thicken

/// ↔️ Resumable [`thicken_face`]: the cap is a nested [`OffsetFaceJob`], then one unit rules each side
/// between the two caps' corresponding boundary edges, then one unit closes the solid.
pub struct ThickenJob {
    face: FaceId,
    cap: OffsetFaceJob,
    cap_face: Option<FaceId>,
    pairs: Vec<(CoedgeId, CoedgeId)>,
    sides: Vec<FaceId>,
    outer_count: usize,
    plan: Plan,
}

impl ThickenJob {
    /// ↔️ Plans a thickening of `face` into a solid of thickness `distance`.
    pub fn new(body: &Body, face: FaceId, distance: f64) -> Result<Self, KernelError> {
        if !distance.is_finite() || distance.abs() <= 1e-15 {
            return Err(KernelError::InvalidInput("thicken distance must be non-zero".into()));
        }
        let cap = OffsetFaceJob::new(body, face, distance)?;
        let outer = body.faces.get(face).and_then(|f| f.outer).ok_or_else(|| KernelError::Operation("thicken: cap has no outer loop".into()))?;
        let outer_count = body.loop_coedges(outer).len();
        let plan = Plan::new(&[("cap", cap.progress().total), ("sides", outer_count), ("close", 1)]);
        Ok(Self { face, cap, cap_face: None, pairs: Vec::new(), sides: Vec::new(), outer_count, plan })
    }

    /// ↔️ Builds one ruled side face between the cap coedges `c0` and `c1`.
    fn ruled_side(body: &mut Body, (c0, c1): (CoedgeId, CoedgeId), tol: f64, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
        let c0 = body.coedges.get(c0).unwrap().clone();
        let c1 = body.coedges.get(c1).unwrap().clone();
        let e0 = body.edges.get(c0.edge).unwrap().clone();
        let e1 = body.edges.get(c1.edge).unwrap().clone();
        let curve0 = body.curves3.get(e0.curve).unwrap().clone();
        let curve1 = body.curves3.get(e1.curve).unwrap().clone();
        let ruled = ruled_surface_from_curves(&curve0, e0.range, &curve1, e1.range)?;
        let ruled_id = body.surfaces.insert(ruled.clone());
        let p00 = body.vertices.get(e0.v0).unwrap().position;
        let p01 = body.vertices.get(e0.v1).unwrap().position;
        let p10 = body.vertices.get(e1.v0).unwrap().position;
        let p11 = body.vertices.get(e1.v1).unwrap().position;
        let vert_a = line_edge(body, p00, p10, e0.v0, e1.v0, Tol::DEFAULT, rec);
        let vert_b = line_edge(body, p01, p11, e0.v1, e1.v1, Tol::DEFAULT, rec);
        let members = [(c0.edge, true), (vert_b, true), (c1.edge, false), (vert_a, false)];
        let face = attach_face(body, ruled_id, &members, false, Tol::DEFAULT, rec);
        let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
        edge_geom.insert(c0.edge, (curve0, e0.range));
        edge_geom.insert(c1.edge, (curve1, e1.range));
        edge_geom.insert(vert_a, (Curve3::Line { origin: p00, dir: p10 - p00 }, (0.0, 1.0)));
        edge_geom.insert(vert_b, (Curve3::Line { origin: p01, dir: p11 - p01 }, (0.0, 1.0)));
        set_face_pcurves(body, face, &ruled, &edge_geom, tol);
        Ok(face)
    }
}

impl StagedOperation for ThickenJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("thicken job already finished".into()))?;
        match unit.phase {
            0 => {
                if let StageStep::Done(StageOutput::Face(cap)) = self.cap.advance(body, rec)? {
                    // The solid grows along the ORIGINAL face's outward normal, so the offset cap already faces
                    // outward and it is the original that now faces INTO the new material. Reversing the far cap
                    // instead (what this used to do) left both caps pointing the same way: the two contributions
                    // then cancelled instead of adding, and a 2 × 1 × 0.5 thickened quad measured 1/3 of its volume.
                    if let Some(fd) = body.faces.get_mut(self.face) {
                        fd.flipped = !fd.flipped;
                    }
                    let outer0 = body.faces.get(self.face).unwrap().outer.ok_or_else(|| KernelError::Operation("thicken: cap has no outer loop".into()))?;
                    let outer1 = body.faces.get(cap).unwrap().outer.ok_or_else(|| KernelError::Operation("thicken: offset cap has no outer loop".into()))?;
                    let (ce0, ce1) = (body.loop_coedges(outer0), body.loop_coedges(outer1));
                    if ce0.len() != ce1.len() || ce0.len() != self.outer_count {
                        return Err(KernelError::Operation("thicken: cap loop structures diverged".into()));
                    }
                    self.pairs = ce0.into_iter().zip(ce1).collect();
                    self.cap_face = Some(cap);
                }
            }
            1 => {
                let side = Self::ruled_side(body, self.pairs[unit.index], OFFSET_TOL, rec)?;
                self.sides.push(side);
            }
            _ => {
                let mut faces = vec![self.face, self.cap_face.expect("cap built before closing")];
                faces.extend(std::mem::take(&mut self.sides));
                return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️Thicken

// #region 🔖️Shell

/// 🐚️ Resumable closed [`shell_solid`]: the inner cavity is a nested [`OffsetSolidJob`] by
/// `-thickness`; the unit that finishes it also flips and nests it.
pub struct ClosedShellJob {
    outer_faces: Vec<FaceId>,
    inner: OffsetSolidJob,
}

impl StagedOperation for ClosedShellJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.inner.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let StageStep::Done(StageOutput::Solid(inner_solid)) = self.inner.advance(body, rec)? else {
            return Ok(StageStep::Working);
        };
        let inner_faces = body.solid_faces(inner_solid);
        for &f in &inner_faces {
            if let Some(fd) = body.faces.get_mut(f) {
                fd.flipped = !fd.flipped;
            }
        }
        retire_solid_scaffold(body, inner_solid, rec);
        let outer_shell = add_shell(body, std::mem::take(&mut self.outer_faces), rec);
        let inner_shell = add_shell(body, inner_faces, rec);
        Ok(StageStep::Done(StageOutput::Solid(add_solid(body, outer_shell, vec![inner_shell], rec))))
    }
}

/// 🐚️ Resumable [`shell_solid_with_open_faces`]: offset surfaces, the shared [`Rebuild`] passes, one
/// copy per kept face, one ruled rim per open-face boundary coedge, then the closing unit.
pub struct OpenShellJob {
    kept: Vec<FaceId>,
    faces: Vec<FaceId>,
    open: Vec<FaceId>,
    rims: Vec<(FaceId, CoedgeId)>,
    distance: f64,
    rebuild: Rebuild,
    copier: FaceCopier,
    rim_connectors: HashMap<VertexId, EdgeId>,
    shell_faces: Vec<FaceId>,
    plan: Plan,
}

impl OpenShellJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(body: &Body, solid: SolidId, thickness: f64, open_faces: &[FaceId]) -> Result<Self, KernelError> {
        let distance = -thickness;
        let faces = body.solid_faces(solid);
        let solid_faces: HashSet<FaceId> = faces.iter().copied().collect();
        let open_set: HashSet<FaceId> = open_faces.iter().copied().collect();
        for f in &open_set {
            if !solid_faces.contains(f) {
                return Err(KernelError::MissingEntity("open face is not on the solid".into()));
            }
        }
        let mut open: Vec<FaceId> = open_set.iter().copied().collect();
        open.sort_unstable();
        let kept: Vec<FaceId> = faces.iter().copied().filter(|face| !open_set.contains(face)).collect();
        let mut rims = Vec::new();
        for &open_face in &open {
            let face_data = body.faces.get(open_face).unwrap();
            let mut loops = Vec::new();
            if let Some(o) = face_data.outer {
                loops.push(o);
            }
            loops.extend(face_data.inners.iter().copied());
            for lp in loops {
                rims.extend(body.loop_coedges(lp).into_iter().map(|cid| (open_face, cid)));
            }
        }
        let materialize: HashSet<FaceId> = solid_faces.difference(&open_set).copied().collect();
        let rebuild = Rebuild::new(body, solid, Policy::Shell { distance, open: open_set }, materialize, OFFSET_TOL);
        let [vertices, edges, rebuilt_faces] = rebuild.counts();
        let plan = Plan::new(&[("surfaces", faces.len()), ("vertices", vertices), ("edges", edges), ("faces", rebuilt_faces), ("copy", kept.len()), ("rims", rims.len()), ("close", 1)]);
        Ok(Self { kept, faces, open, rims, distance, rebuild, copier: FaceCopier::default(), rim_connectors: HashMap::new(), shell_faces: Vec::new(), plan })
    }
}

impl StagedOperation for OpenShellJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("shell job already finished".into()))?;
        match unit.phase {
            0 => {
                // An OPEN face is not a wall: the cavity runs right up to it, so its own surface stays put and
                // only trims the inner faces that meet it. Offsetting it like every other face (what this used
                // to do) pulled the cavity's roof `thickness` below the opening — a 2³ box shelled at 0.2 with
                // its top open came out with a 1.6-tall cavity instead of 1.8, and a slanted ruled rim instead
                // of the flat frame the opening actually is.
                let f = self.faces[unit.index];
                let fd = body.faces.get(f).unwrap().clone();
                let s = body.surfaces.get(fd.surface).unwrap().clone();
                let signed = if fd.flipped { -self.distance } else { self.distance };
                let surface = if self.open.binary_search(&f).is_ok() { s } else { offset_surface(&s, signed, OFFSET_TOL)? };
                self.rebuild.surfaces.insert(f, surface);
            }
            VERTICES => self.rebuild.vertex(body, rec, unit.index)?,
            EDGES => self.rebuild.edge(body, rec, unit.index)?,
            FACES => self.rebuild.face(body, rec, unit.index)?,
            4 => self.copier.copy(body, self.kept[unit.index], rec)?,
            5 => {
                let (open_f, cid) = self.rims[unit.index];
                let face_data = body.faces.get(open_f).unwrap().clone();
                let c = body.coedges.get(cid).unwrap().clone();
                let e = c.edge;
                let neighbours = edge_unique_faces(body, &self.rebuild.solid_faces, e);
                let Some(&neighbour) = neighbours.iter().find(|&&f| f != open_f) else {
                    return Ok(StageStep::Working);
                };
                if self.open.binary_search(&neighbour).is_ok() {
                    return Ok(StageStep::Working);
                }
                let (new_edge, new_curve, new_range) = self.rebuild.out.edge_new.get(&e).cloned().ok_or_else(|| KernelError::Operation("shell: rim edge was not built".into()))?;
                let orig_edge = body.edges.get(e).unwrap().clone();
                let orig_curve = body.curves3.get(orig_edge.curve).unwrap().clone();
                let outer_edge = self.copier.edge(e).ok_or_else(|| KernelError::Operation("shell: outer rim edge was not copied".into()))?;
                let outer_v0 = self.copier.vertex(orig_edge.v0).ok_or_else(|| KernelError::Operation("shell: outer rim vertex was not copied".into()))?;
                let outer_v1 = self.copier.vertex(orig_edge.v1).ok_or_else(|| KernelError::Operation("shell: outer rim vertex was not copied".into()))?;
                let directed = |range: (f64, f64)| if c.forward { range } else { (range.1, range.0) };
                let rim_surf = ruled_surface_from_curves(&orig_curve, directed(orig_edge.range), &new_curve, directed(new_range))?;
                let rim_id = body.surfaces.insert(rim_surf.clone());
                let nv0 = self.rebuild.out.vertex_new.get(&orig_edge.v0).copied().unwrap_or(orig_edge.v0);
                let nv1 = self.rebuild.out.vertex_new.get(&orig_edge.v1).copied().unwrap_or(orig_edge.v1);
                let p00 = body.vertices.get(orig_edge.v0).unwrap().position;
                let p01 = body.vertices.get(orig_edge.v1).unwrap().position;
                let p10 = body.vertices.get(nv0).unwrap().position;
                let p11 = body.vertices.get(nv1).unwrap().position;
                let vert_a = *self.rim_connectors.entry(orig_edge.v0).or_insert_with(|| line_edge(body, p00, p10, outer_v0, nv0, Tol::DEFAULT, rec));
                let vert_b = *self.rim_connectors.entry(orig_edge.v1).or_insert_with(|| line_edge(body, p01, p11, outer_v1, nv1, Tol::DEFAULT, rec));
                let members = if c.forward { [(outer_edge, true), (vert_b, true), (new_edge, false), (vert_a, false)] } else { [(outer_edge, false), (vert_a, true), (new_edge, true), (vert_b, false)] };
                let rim_face = attach_face(body, rim_id, &members, face_data.flipped, Tol::DEFAULT, rec);
                let mut edge_geom: HashMap<EdgeId, (Curve3, (f64, f64))> = HashMap::new();
                edge_geom.insert(outer_edge, (orig_curve, orig_edge.range));
                edge_geom.insert(new_edge, (new_curve, new_range));
                edge_geom.insert(vert_a, (Curve3::Line { origin: p00, dir: p10 - p00 }, (0.0, 1.0)));
                edge_geom.insert(vert_b, (Curve3::Line { origin: p01, dir: p11 - p01 }, (0.0, 1.0)));
                set_face_pcurves(body, rim_face, &rim_surf, &edge_geom, OFFSET_TOL);
                self.shell_faces.push(rim_face);
            }
            _ => {
                let mut shell_faces = self.copier.faces().to_vec();
                for &f in &self.faces {
                    if self.open.binary_search(&f).is_ok() {
                        continue;
                    }
                    shell_faces.push(*self.rebuild.out.face_new.get(&f).ok_or_else(|| KernelError::Operation("shell: inner face was not built".into()))?);
                }
                shell_faces.extend(std::mem::take(&mut self.shell_faces));
                return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, shell_faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

/// 🐚️ Resumable [`shell_solid_with_open_faces`]: closed when no face is open, otherwise
/// [`OpenShellJob`].
pub enum ShellJob {
    Closed(Box<ClosedShellJob>),
    Open(Box<OpenShellJob>),
}

impl ShellJob {
    /// 🐚️ Plans a shell of `solid` with wall `thickness` leaving `open_faces` open.
    pub fn new(body: &Body, solid: SolidId, thickness: f64, open_faces: &[FaceId]) -> Result<Self, KernelError> {
        if !thickness.is_finite() || thickness <= 1e-15 {
            return Err(KernelError::InvalidInput("shell thickness must be positive".into()));
        }
        if body.solids.get(solid).is_none() {
            return Err(KernelError::MissingEntity(format!("solid {solid}")));
        }
        if !open_faces.is_empty() {
            return Ok(Self::Open(Box::new(OpenShellJob::new(body, solid, thickness, open_faces)?)));
        }
        let corner = if is_planar_only(body, solid) { OffsetCorner::Sharp } else { OffsetCorner::Round };
        let inner = OffsetSolidJob::new(body, solid, -thickness, corner)?;
        Ok(Self::Closed(Box::new(ClosedShellJob { outer_faces: body.solid_faces(solid), inner })))
    }
}

impl StagedOperation for ShellJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        match self {
            Self::Closed(job) => job.progress(),
            Self::Open(job) => job.progress(),
        }
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        match self {
            Self::Closed(job) => job.advance(body, rec),
            Self::Open(job) => job.advance(body, rec),
        }
    }
}

// #endregion 🔖️Shell

// #region 🔖️Draft

/// 📐️ Resumable [`draft_angle`]: one unit drafts one face's surface, the shared [`Rebuild`] passes
/// recompute every adjacent vertex, edge and face, then one unit closes the solid.
pub struct DraftJob {
    drafted: Vec<FaceId>,
    solid_faces: Vec<FaceId>,
    neutral_plane: Surface,
    pull: Vec3,
    angle: f64,
    rebuild: Rebuild,
    plan: Plan,
}

impl DraftJob {
    /// 📐️ Plans `angle_rad` of draft on `faces` of `solid` about the neutral plane.
    pub fn new(body: &Body, solid: SolidId, faces: &[FaceId], pull_dir: Vec3, (neutral_origin, neutral_normal): (Pnt3, Vec3), angle_rad: f64) -> Result<Self, KernelError> {
        if body.solids.get(solid).is_none() {
            return Err(KernelError::MissingEntity(format!("solid {solid}")));
        }
        if !angle_rad.is_finite() || angle_rad.abs() <= 1e-15 {
            return Err(KernelError::Operation("draft angle must be non-zero".into()));
        }
        if faces.is_empty() {
            return Err(KernelError::InvalidInput("draft requires at least one face".into()));
        }
        let pull = pull_dir.normalized().ok_or_else(|| KernelError::InvalidInput("pull direction must be non-zero".into()))?;
        let nn = neutral_normal.normalized().ok_or_else(|| KernelError::InvalidInput("neutral plane normal must be non-zero".into()))?;
        let solid_faces = body.solid_faces(solid);
        for f in faces {
            if !solid_faces.contains(f) {
                return Err(KernelError::MissingEntity("draft face is not on the solid".into()));
            }
        }
        let neutral_plane = Surface::Plane { frame: Frame3::from_normal(neutral_origin, nn).ok_or_else(|| KernelError::InvalidInput("degenerate neutral plane".into()))? };
        let rebuild = Rebuild::new(body, solid, Policy::Draft, solid_faces.iter().copied().collect(), OFFSET_TOL);
        let [vertices, edges, rebuilt_faces] = rebuild.counts();
        let plan = Plan::new(&[("surfaces", faces.len()), ("vertices", vertices), ("edges", edges), ("faces", rebuilt_faces), ("close", 1)]);
        Ok(Self { drafted: faces.to_vec(), solid_faces, neutral_plane, pull, angle: angle_rad, rebuild, plan })
    }
}

impl StagedOperation for DraftJob {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn progress(&self) -> StageProgress {
        self.plan.progress()
    }

    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError> {
        let unit = self.plan.take().ok_or_else(|| KernelError::Operation("draft job already finished".into()))?;
        match unit.phase {
            0 => {
                let f = self.drafted[unit.index];
                let fd = body.faces.get(f).unwrap();
                let surface = body.surfaces.get(fd.surface).unwrap().clone();
                self.rebuild.surfaces.insert(f, draft_one_surface(&surface, &self.neutral_plane, self.pull, self.angle, OFFSET_TOL)?);
            }
            VERTICES => self.rebuild.vertex(body, rec, unit.index)?,
            EDGES => self.rebuild.edge(body, rec, unit.index)?,
            FACES => self.rebuild.face(body, rec, unit.index)?,
            _ => {
                let mut out_faces = Vec::with_capacity(self.solid_faces.len());
                for f in &self.solid_faces {
                    out_faces.push(*self.rebuild.out.face_new.get(f).ok_or_else(|| KernelError::Operation("draft: face was not rebuilt".into()))?);
                }
                return Ok(StageStep::Done(StageOutput::Solid(finish_solid(body, out_faces, rec))));
            }
        }
        Ok(StageStep::Working)
    }
}

// #endregion 🔖️Draft
