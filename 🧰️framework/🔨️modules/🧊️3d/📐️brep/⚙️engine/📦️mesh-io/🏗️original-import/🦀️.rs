//! 🏗️ Original forward mesh import, funded one arena or lookup-map action at a time.

use super::*;
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, Store, SurfaceId, VertexId},
    topology::{Coedge, Edge, Face, Loop, Shell, Solid, Vertex, dummy_coedge}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueRefusalKind, retained_clone::retained_clone_grant_admits};
use std::mem::size_of;

type Key = [u64; 3];
type EdgeKey = (Key, Key);
type Progress = RetainedCloneProgress;
type Grant = RetainedCloneGrant;
const NORMAL: RetirementDemand = RetirementDemand { depth: 1, copy_bytes: 0, capacity_bytes: 0, release_bytes: 0 };

#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Volume,
    ReserveJournal,
    ReserveFaces,
    Triangle,
    Vertex(usize),
    VertexIndex(usize),
    Edge(usize),
    Curve(usize),
    EdgeIndex(usize),
    Surface,
    Face,
    Loop,
    Trim(usize),
    Coedge(usize),
    Link(usize),
    LinkLoop,
    Finalize,
    Shell,
    Solid,
    Ready,
}

struct Triangle {
    points: [Pnt3; 3],
    keys: [Key; 3],
    frame: Frame3,
    vertices: [Option<VertexId>; 3],
    edges: [Option<EdgeId>; 3],
    uses: [Option<CoedgeId>; 3],
    curve: Option<Curve3Id>,
    surface: Option<SurfaceId>,
    face: Option<FaceId>,
    ring: Option<LoopId>,
    trim: Option<Curve2Id>,
}

impl Triangle {
    fn endpoints(&self, i: usize) -> (EdgeKey, bool) {
        let [a, b] = [self.keys[i], self.keys[(i + 1) % 3]];
        ((a.min(b), a.max(b)), a < b)
    }

    fn ordered(&self, i: usize) -> (usize, usize) {
        let j = (i + 1) % 3;
        if self.endpoints(i).1 { (i, j) } else { (j, i) }
    }
}

#[derive(Default)]
pub(super) struct OriginalMeshImport {
    phase: Phase,
    triangle: Option<Triangle>,
    pub(super) owned: Vec<EntityRef>,
}

fn demand<T, Id: ArenaId>(store: &Store<T, Id>) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { depth: store.next_insert_depth_demand()?.max(1),
        capacity_bytes: store.next_insert_capacity_byte_demand(0)?, ..NORMAL })
}

fn reserve<T, Id: ArenaId>(store: &mut Store<T, Id>, grant: Grant) -> Result<Progress, ValueError> {
    store.reserve_insert_step(grant).map_err(|(error, progress)| error.with_retained_progress(progress))
}

fn insert<T, Id: ArenaId>(store: &mut Store<T, Id>, value: T, grant: Grant) -> Result<Id, ValueError> {
    store.insert_reserved(value, grant).map(|(id, _)| id).map_err(|(error, _)| error)
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original mesh import requires finite nondegenerate triangles with manifold coherent winding")
}

impl MeshImportCursor {
    pub(super) fn last_import_entity(&self) -> Option<EntityRef> {
        self.original.owned.last().copied().or_else(|| self.recorder.last_owned())
    }

    /// 📏️ Quotes one original allocation, source read or topology action.
    pub fn original_import_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let t = self.original.triangle.as_ref();
        let mut quote = NORMAL;
        match self.original.phase {
            Phase::Volume | Phase::Triangle => quote.copy_bytes = 3 * size_of::<Pnt3>() + 3 * size_of::<u32>(),
            Phase::ReserveJournal => quote.capacity_bytes = (self.triangles * 18 + 2) * size_of::<EntityRef>(),
            Phase::ReserveFaces => quote.capacity_bytes = self.triangles * size_of::<FaceId>(),
            Phase::Vertex(i) => {
                let key = &t.unwrap().keys[i];
                quote.depth = self.vertex_map.next_contains_depth_demand(key)?;
                if !self.vertex_map.contains_key(key) {
                    let mut next = demand(&body.vertices)?;
                    next.depth = next.depth.max(quote.depth);
                    return Ok(next);
                }
            }
            Phase::VertexIndex(i) => {
                let key = &t.unwrap().keys[i];
                quote.depth = self.vertex_map.next_insert_depth_demand(key)?;
                quote.capacity_bytes = self.vertex_map.next_insert_capacity_byte_demand(key, 0)?;
            }
            Phase::Edge(i) => {
                let (key, _) = t.unwrap().endpoints(i);
                quote.depth = self.edge_map.next_contains_depth_demand(&key)?;
                if !self.edge_map.contains_key(&key) {
                    let mut next = demand(&body.curves3)?;
                    next.depth = next.depth.max(quote.depth);
                    return Ok(next);
                }
            }
            Phase::Curve(_) => return demand(&body.edges),
            Phase::EdgeIndex(i) => {
                let (key, _) = t.unwrap().endpoints(i);
                quote.depth = self.edge_map.next_insert_depth_demand(&key)?;
                quote.capacity_bytes = self.edge_map.next_insert_capacity_byte_demand(&key, 0)?;
            }
            Phase::Surface => return demand(&body.surfaces),
            Phase::Face => return demand(&body.faces),
            Phase::Loop => return demand(&body.loops),
            Phase::Trim(i) => {
                let edge = t.unwrap().edges[i].unwrap();
                let curve = body.edges.get(edge).ok_or_else(invalid)?.curve;
                quote = demand(&body.curves2)?;
                quote.depth = quote.depth.max(body.edges.next_remove_depth_demand(edge)?)
                    .max(body.curves3.next_remove_depth_demand(curve)?);
            }
            Phase::Coedge(_) => return demand(&body.coedges),
            Phase::Link(i) => quote.depth = body.coedges.next_remove_depth_demand(t.unwrap().uses[i].unwrap())?,
            Phase::LinkLoop => quote.depth = body.loops.next_remove_depth_demand(t.unwrap().ring.unwrap())?,
            Phase::Finalize => {
                quote.depth = body.faces.next_remove_depth_demand(t.unwrap().face.unwrap())?;
                quote.copy_bytes = size_of::<FaceId>();
            }
            Phase::Shell => return demand(&body.shells),
            Phase::Solid => return demand(&body.solids),
            Phase::Ready => {}
        }
        Ok(quote)
    }

    /// 🪪️ Borrows the completed root while the rollback journal still owns it.
    pub fn original_import_root(&self) -> Option<EntityRef> {
        if !matches!(self.original.phase, Phase::Ready) || self.cancelled { return None; }
        self.solid.map(EntityRef::Solid).or_else(|| self.shell.map(EntityRef::Shell))
    }

    /// 🫴️ Hands completed original topology to a separately published live root.
    pub fn commit_original_import(&mut self) { self.phase = 4; }

    /// ⏱️ Performs one admitted forward action and records rollback custody at insertion.
    pub fn step_original_import(&mut self, body: &mut Body, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let quote = self.original_import_demand(body)?;
        if self.cancelled || self.original_import_root().is_some() || grant.maximum_items == 0
            || !retained_clone_grant_admits(grant, quote) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let receipt = Progress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() };
        let result = self.advance_original(body, grant, quote, receipt);
        if result.is_err() { self.cancelled = true; }
        result.map_err(|error| if error.retained_progress() == Progress::default() { error.with_retained_progress(receipt) } else { error })
    }

    fn advance_original(&mut self, body: &mut Body, grant: Grant, quote: RetirementDemand, mut receipt: Progress) -> Result<Progress, ValueError> {
        macro_rules! arena {
            ($store:ident, $kind:ident, $value:expr) => {{
                if quote.capacity_bytes != 0 { return reserve(&mut body.$store, grant); }
                let value = $value;
                let id = insert(&mut body.$store, value, grant)?;
                self.original.owned.push(EntityRef::$kind(id));
                id
            }};
        }
        let mut phase = self.original.phase;
        let allocation = || ValueError::literal(ValueRefusalKind::AllocationFailed, "original mesh import backing allocation failed");
        match phase {
            Phase::Volume => {
                if self.cursor == self.indices.len() {
                    self.cursor = 0;
                    phase = Phase::ReserveJournal;
                } else {
                    let [a, b, c] = self.points().map_err(|_| invalid())?;
                    self.volume += a.to_vec().dot(b.to_vec().cross(c.to_vec()));
                    if !self.volume.is_finite() { return Err(invalid()); }
                    self.cursor += 3;
                }
            }
            Phase::ReserveJournal => {
                self.original.owned.try_reserve_exact(self.triangles * 18 + 2).map_err(|_| allocation())?;
                receipt.retained_capacity_bytes = self.original.owned.capacity() * size_of::<EntityRef>();
                phase = Phase::ReserveFaces;
            }
            Phase::ReserveFaces => {
                self.faces.try_reserve_exact(self.triangles).map_err(|_| allocation())?;
                receipt.retained_capacity_bytes = self.faces.capacity() * size_of::<FaceId>();
                phase = Phase::Triangle;
            }
            Phase::Triangle => {
                if self.cursor == self.indices.len() {
                    if self.faces.is_empty() { return Err(invalid()); }
                    phase = Phase::Shell;
                } else {
                    let [a, mut b, mut c] = self.points().map_err(|_| invalid())?;
                    let normal = (b - a).cross(c - a);
                    let supplied = self.normal_at_first();
                    if supplied.map_or(self.volume < 0.0, |n| n.dot(normal) < 0.0) { std::mem::swap(&mut b, &mut c); }
                    if a == b || b == c || a == c {
                        self.cursor += 3;
                    } else {
                        let frame = Frame3::from_normal(a, (b-a).cross(c-a)).ok_or_else(invalid)?;
                        let points = [a, b, c];
                        let keys = points.map(|point| point.to_array().map(|v| if v == 0.0 { 0 } else { v.to_bits() }));
                        self.original.triangle = Some(Triangle { points, keys, frame, vertices: [None; 3], edges: [None; 3],
                            uses: [None; 3], curve: None, surface: None, face: None, ring: None, trim: None });
                        phase = Phase::Vertex(0);
                    }
                }
            }
            Phase::Vertex(i) => {
                let t = self.original.triangle.as_mut().unwrap();
                if let Some(&id) = self.vertex_map.get(&t.keys[i]) {
                    t.vertices[i] = Some(id);
                    phase = next_vertex(i);
                } else {
                    t.vertices[i] = Some(arena!(vertices, Vertex, Vertex { position: t.points[i], tol: Tol::DEFAULT, label: body.new_label() }));
                    phase = Phase::VertexIndex(i);
                }
            }
            Phase::VertexIndex(i) => {
                let t = self.original.triangle.as_ref().unwrap();
                let key = t.keys[i];
                if quote.capacity_bytes != 0 { return self.vertex_map.reserve_insert_step(&key, grant).map_err(|(e,p)|e.with_retained_progress(p)); }
                self.vertex_map.insert_reserved(key, t.vertices[i].unwrap(), grant).map_err(|(e,_,_)|e)?;
                phase = next_vertex(i);
            }
            Phase::Edge(i) => {
                let t = self.original.triangle.as_mut().unwrap();
                let (key, forward) = t.endpoints(i);
                if let Some((edge, count, direction)) = self.edge_map.get_mut(&key) {
                    if *count != 1 || *direction == forward { return Err(invalid()); }
                    *count = 2;
                    self.boundary_edges -= 1;
                    t.edges[i] = Some(*edge);
                    phase = next_edge(i);
                } else {
                    let (a, b) = t.ordered(i);
                    t.curve = Some(arena!(curves3, Curve3, Curve3::Line { origin: t.points[a], dir: t.points[b] - t.points[a] }));
                    phase = Phase::Curve(i);
                }
            }
            Phase::Curve(i) => {
                let t = self.original.triangle.as_mut().unwrap();
                let (a, b) = t.ordered(i);
                t.edges[i] = Some(arena!(edges, Edge, Edge { curve: t.curve.unwrap(), range: (0.0, 1.0),
                    v0: t.vertices[a].unwrap(), v1: t.vertices[b].unwrap(), tol: Tol::DEFAULT, label: body.new_label() }));
                phase = Phase::EdgeIndex(i);
            }
            Phase::EdgeIndex(i) => {
                let t = self.original.triangle.as_ref().unwrap();
                let (key, forward) = t.endpoints(i);
                if quote.capacity_bytes != 0 { return self.edge_map.reserve_insert_step(&key, grant).map_err(|(e,p)|e.with_retained_progress(p)); }
                self.edge_map.insert_reserved(key, (t.edges[i].unwrap(), 1, forward), grant).map_err(|(e,_,_)|e)?;
                self.boundary_edges += 1;
                phase = next_edge(i);
            }
            Phase::Surface => {
                let t = self.original.triangle.as_mut().unwrap();
                t.surface = Some(arena!(surfaces, Surface, Surface::Plane { frame: t.frame }));
                phase = Phase::Face;
            }
            Phase::Face => {
                let t = self.original.triangle.as_mut().unwrap();
                t.face = Some(arena!(faces, Face, Face { surface: t.surface.unwrap(), outer: None, inners: Vec::new(),
                    flipped: false, tol: Tol::DEFAULT, label: body.new_label() }));
                phase = Phase::Loop;
            }
            Phase::Loop => {
                let t = self.original.triangle.as_mut().unwrap();
                t.ring = Some(arena!(loops, Loop, Loop { first: dummy_coedge(), face: t.face.unwrap() }));
                phase = Phase::Trim(0);
            }
            Phase::Trim(i) => {
                let t = self.original.triangle.as_mut().unwrap();
                t.trim = Some(arena!(curves2, Curve2, {
                    let edge = body.edges.get(t.edges[i].unwrap()).ok_or_else(invalid)?;
                    planar_pcurve(body.curves3.get(edge.curve).ok_or_else(invalid)?, t.frame)
                }));
                phase = Phase::Coedge(i);
            }
            Phase::Coedge(i) => {
                let t = self.original.triangle.as_mut().unwrap();
                let forward = t.endpoints(i).1;
                t.uses[i] = Some(arena!(coedges, Coedge, Coedge { edge: t.edges[i].unwrap(), forward, pcurve: t.trim,
                    prange: if forward { (0.0, 1.0) } else { (1.0, 0.0) }, loop_id: t.ring.unwrap(),
                    next: dummy_coedge(), prev: dummy_coedge() }));
                phase = if i == 2 { Phase::Link(0) } else { Phase::Trim(i + 1) };
            }
            Phase::Link(i) => {
                let t = self.original.triangle.as_ref().unwrap();
                let coedge = body.coedges.get_mut(t.uses[i].unwrap()).ok_or_else(invalid)?;
                coedge.next = t.uses[(i + 1) % 3].unwrap();
                coedge.prev = t.uses[(i + 2) % 3].unwrap();
                phase = if i == 2 { Phase::LinkLoop } else { Phase::Link(i + 1) };
            }
            Phase::LinkLoop => {
                let t = self.original.triangle.as_ref().unwrap();
                body.loops.get_mut(t.ring.unwrap()).ok_or_else(invalid)?.first = t.uses[0].unwrap();
                phase = Phase::Finalize;
            }
            Phase::Finalize => {
                let t = self.original.triangle.take().unwrap();
                let (face, ring) = (t.face.unwrap(), t.ring.unwrap());
                body.faces.get_mut(face).ok_or_else(invalid)?.outer = Some(ring);
                self.faces.push(face);
                self.cursor += 3;
                phase = Phase::Triangle;
            }
            Phase::Shell => {
                if quote.capacity_bytes != 0 { return reserve(&mut body.shells, grant); }
                let shell = Shell { faces: std::mem::take(&mut self.faces), label: body.new_label() };
                let (id, _) = body.shells.insert_reserved(shell, grant).map_err(|(error, shell)| { self.faces = shell.faces; error })?;
                self.original.owned.push(EntityRef::Shell(id));
                self.shell = Some(id);
                phase = if self.boundary_edges == 0 { Phase::Solid } else { Phase::Ready };
            }
            Phase::Solid => {
                self.solid = Some(arena!(solids, Solid, Solid { outer: self.shell.unwrap(), inners: Vec::new(), label: body.new_label() }));
                phase = Phase::Ready;
            }
            Phase::Ready => unreachable!("completed topology is published separately"),
        }
        if !receipt.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original mesh backing exceeded grant").with_retained_progress(receipt));
        }
        self.original.phase = phase;
        Ok(receipt)
    }

    fn normal_at_first(&self) -> Option<Vec3> {
        let index = self.indices[self.cursor] as usize;
        if self.positions64.is_empty() && self.normals.len() >= self.positions.len() {
            let n = self.normals.get(index * 3..index * 3 + 3)?;
            Some(Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64))
        } else if !self.positions64.is_empty() && self.normals64.len() >= self.positions64.len() {
            self.normals64.get(index).copied()
        } else { None }
    }
}

fn next_vertex(index: usize) -> Phase { if index == 2 { Phase::Edge(0) } else { Phase::Vertex(index + 1) } }
fn next_edge(index: usize) -> Phase { if index == 2 { Phase::Surface } else { Phase::Edge(index + 1) } }
