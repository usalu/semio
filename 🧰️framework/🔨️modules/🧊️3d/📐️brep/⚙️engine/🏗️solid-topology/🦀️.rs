//! 🏗️ Funded construction and reverse retirement of unpublished solid topology.

use semio_framework_3d_brep_boolean::certified_boundaries::{BoreBoundary, MAX_ORTHOGONAL_ENTITIES, OrthogonalBoundary};
use semio_framework_3d_brep_construction::point_hull::HullBoundary;
use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim, SolidPrimitive};
use semio_framework_3d_brep_construction::prism_boundary::PrismBoundary;
use semio_framework_3d_brep_representation::arena::{ArenaId, FaceId, ShellId, SolidId, Store};
use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
use semio_framework_3d_brep_representation::topology::{Body, Coedge, Edge, EntityRef, Face, Loop, Shell, Solid, Vertex, dummy_coedge};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::{size_of, take};

/// 🧩️ The certified recipe that drives one unpublished solid topology.
pub enum SolidBoundary {
    Analytic(SolidPrimitive),
    Orthogonal(OrthogonalBoundary),
    Prism(PrismBoundary),
    Bore(BoreBoundary),
    Hull(HullBoundary),
}

impl SolidBoundary {
    /// 🔢️ Counts the persistent labels the live registry must reserve.
    pub fn label_count(&self) -> usize {
        self.vertex_count() + self.edge_count() + self.face_count() + self.shell_count() + 1
    }

    fn vertex_count(&self) -> usize {
        match self { Self::Analytic(p) => p.vertex_count(), Self::Orthogonal(p) => p.vertex_count(), Self::Prism(p) => p.vertex_count(), Self::Bore(_) => 10, Self::Hull(p) => p.vertex_count() }
    }
    fn edge_count(&self) -> usize {
        match self { Self::Analytic(p) => p.edge_count(), Self::Orthogonal(p) => p.edge_count(), Self::Prism(p) => p.edge_count(), Self::Bore(_) => 15, Self::Hull(p) => p.edge_count() }
    }
    fn ring_sizes(&self) -> &[usize] {
        match self { Self::Analytic(p) => p.ring_sizes(), Self::Orthogonal(p) => p.ring_sizes(), Self::Prism(p) => p.ring_sizes(), Self::Bore(_) => &[4,4,4,4,4,4,4,1,1], Self::Hull(p) => p.ring_sizes() }
    }
    fn face_count(&self) -> usize {
        if matches!(self, Self::Bore(_)) { 7 } else { self.ring_sizes().len() }
    }
    fn ring_owner(&self, ring: usize) -> usize {
        if matches!(self, Self::Bore(_)) && ring >= 7 { ring - 7 } else { ring }
    }
    fn inner_ring(&self, face: usize) -> Option<usize> {
        (matches!(self, Self::Bore(_)) && face < 2).then_some(7 + face)
    }
    fn face_flipped(&self, face: usize) -> bool {
        matches!(self, Self::Bore(_)) && face == 6
    }
    fn shell_count(&self) -> usize {
        match self {
            Self::Orthogonal(boundary) => boundary.shell_sizes().len(),
            _ => 1,
        }
    }
    fn shell_face_count(&self, shell: usize) -> usize {
        match self {
            Self::Orthogonal(boundary) => boundary.shell_sizes()[shell],
            _ => self.face_count(),
        }
    }
    fn vertex(&self, i: usize) -> Pnt3 {
        match self { Self::Analytic(p) => p.vertex(i), Self::Orthogonal(p) => p.vertex(i), Self::Prism(p) => p.vertex(i), Self::Bore(p) => p.vertex(i), Self::Hull(p) => p.vertex(i) }
    }
    fn edge(&self, i: usize) -> PrimitiveEdge {
        match self { Self::Analytic(p) => p.edge(i), Self::Orthogonal(p) => p.edge(i), Self::Prism(p) => p.edge(i), Self::Bore(p) => p.edge(i), Self::Hull(p) => p.edge(i) }
    }
    fn surface(&self, i: usize) -> Surface {
        match self { Self::Analytic(p) => p.surface(i), Self::Orthogonal(p) => p.surface(i), Self::Prism(p) => p.surface(i), Self::Bore(p) => p.surface(i), Self::Hull(p) => p.surface(i) }
    }
    fn trim(&self, i: usize, member: usize) -> PrimitiveTrim {
        match self { Self::Analytic(p) => p.trim(i, member), Self::Orthogonal(p) => p.trim(i, member), Self::Prism(p) => p.trim(i, member), Self::Bore(p) => p.trim(i, member), Self::Hull(p) => p.trim(i, member) }
    }
}

#[derive(Clone, Copy)]
struct TopologyOffsets {
    curves: usize,
    edges: usize,
    surfaces: usize,
    faces: usize,
    loops: usize,
    trims: usize,
    coedges: usize,
    shell: usize,
    solid: usize,
}

impl TopologyOffsets {
    fn new(primitive: &SolidBoundary) -> Self {
        let curves = primitive.vertex_count();
        let edges = curves + primitive.edge_count();
        let surfaces = edges + primitive.edge_count();
        let faces = surfaces + primitive.face_count();
        let loops = faces + primitive.face_count();
        let trims = loops + primitive.ring_sizes().len();
        let uses: usize = primitive.ring_sizes().iter().sum();
        let coedges = trims + uses;
        let shell = coedges + uses;
        let solid = shell + primitive.shell_count();
        debug_assert!(solid < MAX_ORTHOGONAL_ENTITIES);
        Self { curves, edges, surfaces, faces, loops, trims, coedges, shell, solid }
    }

    fn face_uses(primitive: &SolidBoundary, index: usize) -> (usize, usize, usize) {
        let mut first = 0;
        for (face, &size) in primitive.ring_sizes().iter().enumerate() {
            if index < first + size { return (face, first, size); }
            first += size;
        }
        unreachable!("primitive use index")
    }
}

#[derive(Clone, Copy)]
enum Build {
    Journal,
    Insert(usize),
    LinkCoedge(usize),
    LinkLoop(usize),
    LinkFace(usize),
    ShellFaces(usize),
    InnerShells,
    Ready,
}

/// 🏗️ An unpublished transaction over a body’s native topology stores.
pub struct SolidTopology {
    boundary: SolidBoundary,
    offsets: TopologyOffsets,
    first_label: u64,
    inserted: Vec<EntityRef>,
    shell_faces: Vec<FaceId>,
    inner_shells: Vec<ShellId>,
    build: Build,
}

macro_rules! entity {
    ($self:expr, $index:expr, $variant:ident) => {
        match $self.inserted[$index] {
            EntityRef::$variant(id) => id,
            _ => unreachable!("solid primitive recipe references a previously inserted entity"),
        }
    };
}

fn item() -> Progress { Progress { copied_items: 1, ..Default::default() } }

pub fn store_demand<T, Id: ArenaId>(store: &Store<T, Id>) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand {
        capacity_bytes: store.next_insert_capacity_byte_demand(0)?,
        depth: store.next_insert_depth_demand()?,
        ..Default::default()
    })
}

pub fn store_insert<T, Id: ArenaId>(store: &mut Store<T, Id>, grant: Grant, value: impl FnOnce() -> T) -> Result<(Option<Id>, Progress), ValueError> {
    if store.next_insert_capacity_byte_demand(0)? != 0 {
        return store.reserve_insert_step(grant).map(|receipt| (None, receipt))
            .map_err(|(error, receipt)| error.with_retained_progress(receipt));
    }
    store.insert_reserved(value(), grant).map(|(id, receipt)| (Some(id), receipt)).map_err(|(error, _)| error)
}

impl SolidTopology {
    /// 🎟️ Binds one certified recipe to its reserved range of topology labels.
    pub fn new(boundary: SolidBoundary, first_label: u64) -> Self {
        let offsets = TopologyOffsets::new(&boundary);
        Self {
            boundary, offsets, first_label, inserted: Vec::new(),
            shell_faces: Vec::new(), inner_shells: Vec::new(), build: Build::Journal,
        }
    }

    pub fn ready(&self) -> bool { matches!(self.build, Build::Ready) }
    pub fn journal_is_empty(&self) -> bool { self.inserted.capacity() == 0 }
    pub fn boundary(&self) -> &SolidBoundary { &self.boundary }
    pub fn first_label(&self) -> u64 { self.first_label }
    pub fn solid_id(&self) -> SolidId { entity!(self, self.offsets.solid, Solid) }
    fn label(&self, offset: usize) -> PersistentLabel { PersistentLabel(self.first_label + offset as u64) }

    fn insert(&mut self, body: &mut Body, index: usize, grant: Grant) -> Result<Progress, ValueError> {
        macro_rules! put {
            ($store:ident, $variant:ident, $value:expr) => {{
                let (id, receipt) = store_insert(&mut body.$store, grant, || $value)?;
                (id.map(EntityRef::$variant), receipt)
            }};
        }
        let o = self.offsets;
        let primitive = &self.boundary;
        let (id, receipt) = match index {
            i if i < o.curves => put!(vertices, Vertex, Vertex { position: primitive.vertex(i), tol: Tol::DEFAULT, label: self.label(i) }),
            i if i < o.edges => put!(curves3, Curve3, primitive.edge(i - o.curves).curve),
            i if i < o.surfaces => {
                let edge = i - o.edges;
                let spec = primitive.edge(edge);
                let [a, b] = spec.vertices;
                put!(edges, Edge, Edge {
                    curve: entity!(self, o.curves + edge, Curve3), range: spec.range, tol: Tol::DEFAULT,
                    v0: entity!(self, a, Vertex), v1: entity!(self, b, Vertex), label: self.label(o.curves + edge),
                })
            }
            i if i < o.faces => put!(surfaces, Surface, primitive.surface(i - o.surfaces)),
            i if i < o.loops => {
                let face = i - o.faces;
                put!(faces, Face, Face {
                    surface: entity!(self, o.surfaces + face, Surface), outer: None, inners: Vec::new(),
                    flipped: primitive.face_flipped(face), tol: Tol::DEFAULT, label: self.label(o.surfaces - o.curves + face),
                })
            }
            i if i < o.trims => put!(loops, Loop, Loop { first: dummy_coedge(), face: entity!(self, o.faces + primitive.ring_owner(i - o.loops), Face) }),
            i if i < o.coedges => {
                let use_index = i - o.trims;
                let (face, first, _) = TopologyOffsets::face_uses(primitive, use_index);
                put!(curves2, Curve2, primitive.trim(face, use_index - first).curve)
            }
            i if i < o.shell => {
                let use_index = i - o.coedges;
                let (face, first, _) = TopologyOffsets::face_uses(primitive, use_index);
                let trim = primitive.trim(face, use_index - first);
                put!(coedges, Coedge, Coedge {
                    edge: entity!(self, o.edges + trim.edge, Edge), forward: trim.forward,
                    pcurve: Some(entity!(self, o.trims + use_index, Curve2)), prange: trim.range,
                    loop_id: entity!(self, o.loops + face, Loop), next: dummy_coedge(), prev: dummy_coedge(),
                })
            }
            i if i < o.solid => {
                let shell = i - o.shell;
                let label = self.label(primitive.vertex_count() + primitive.edge_count() + primitive.face_count() + shell);
                if body.shells.next_insert_capacity_byte_demand(0)? > 0 {
                    return body.shells.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
                }
                match body.shells.insert_reserved(Shell { faces: take(&mut self.shell_faces), label }, grant) {
                    Ok((id, receipt)) => (Some(EntityRef::Shell(id)), receipt),
                    Err((error, shell)) => { self.shell_faces = shell.faces; return Err(error); }
                }
            }
            i if i == o.solid => {
                if body.solids.next_insert_capacity_byte_demand(0)? > 0 {
                    return body.solids.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
                }
                let label = self.label(primitive.vertex_count() + primitive.edge_count() + primitive.face_count() + primitive.shell_count());
                let solid = Solid { outer: entity!(self, o.shell, Shell), inners: take(&mut self.inner_shells), label };
                match body.solids.insert_reserved(solid, grant) {
                    Ok((id, receipt)) => (Some(EntityRef::Solid(id)), receipt),
                    Err((error, solid)) => { self.inner_shells = solid.inners; return Err(error); }
                }
            }
            _ => unreachable!("solid primitive topology index"),
        };
        if let Some(id) = id {
            debug_assert_eq!(self.inserted.len(), index);
            self.inserted.push(id);
            self.build = match index {
                i if i == o.shell - 1 => Build::LinkCoedge(0),
                i if i == o.solid => Build::Ready,
                i if i >= o.shell => {
                    if i + 1 < o.solid { Build::ShellFaces(i - o.shell + 1) }
                    else { Build::InnerShells }
                }
                _ => Build::Insert(index + 1),
            };
        }
        Ok(receipt)
    }

    pub fn topology_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        let o = self.offsets;
        let depth = match self.build {
            Build::Journal => return Ok(RetirementDemand {
                capacity_bytes: (o.solid + 1) * size_of::<EntityRef>(), depth: 1, ..Default::default()
            }),
            Build::Insert(index) => return match index {
                i if i < o.curves => store_demand(&body.vertices),
                i if i < o.edges => store_demand(&body.curves3),
                i if i < o.surfaces => store_demand(&body.edges),
                i if i < o.faces => store_demand(&body.surfaces),
                i if i < o.loops => store_demand(&body.faces),
                i if i < o.trims => store_demand(&body.loops),
                i if i < o.coedges => store_demand(&body.curves2),
                i if i < o.shell => store_demand(&body.coedges),
                i if i < o.solid => store_demand(&body.shells),
                i if i == o.solid => store_demand(&body.solids),
                _ => unreachable!("solid primitive topology index"),
            },
            Build::LinkCoedge(index) => body.coedges.next_remove_depth_demand(entity!(self, o.coedges + index, Coedge))?,
            Build::LinkLoop(face) => body.loops.next_remove_depth_demand(entity!(self, o.loops + face, Loop))?,
            Build::LinkFace(face) => {
                let id = entity!(self, o.faces + face, Face);
                let depth = body.faces.next_remove_depth_demand(id)?;
                let has_inner = self.boundary.inner_ring(face).is_some();
                let copy_bytes = usize::from(has_inner) * size_of::<semio_framework_3d_brep_representation::arena::LoopId>();
                if has_inner && body.faces.get(id).is_some_and(|face| face.inners.capacity() == 0) {
                    return Ok(RetirementDemand { capacity_bytes: copy_bytes, depth, ..Default::default() });
                }
                return Ok(RetirementDemand { copy_bytes, depth, ..Default::default() });
            }
            Build::ShellFaces(shell) => {
                let count = self.boundary.shell_face_count(shell);
                let (copy_bytes, capacity_bytes) = if self.shell_faces.capacity() == 0 {
                    (0, count * size_of::<FaceId>())
                } else { (size_of::<FaceId>(), 0) };
                return Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() });
            }
            Build::InnerShells => {
                let count = self.boundary.shell_count() - 1;
                let (copy_bytes, capacity_bytes) = if count == 0 { (0,0) }
                    else if self.inner_shells.capacity() == 0 { (0,count * size_of::<ShellId>()) }
                    else { (size_of::<ShellId>(),0) };
                return Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() });
            }
            Build::Ready => return Ok(RetirementDemand::default()),
        };
        Ok(RetirementDemand { depth, ..Default::default() })
    }

    pub fn advance(&mut self, body: &mut Body, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        if !retained_clone_grant_admits(grant, self.topology_demand(body)?) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let o = self.offsets;
        let rings = self.boundary.ring_sizes();
        match self.build {
            Build::Journal => {
                self.inserted.try_reserve_exact(o.solid + 1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "solid topology journal allocation failed"))?;
                self.build = Build::Insert(0);
                Ok(Progress { retained_capacity_bytes: self.inserted.capacity() * size_of::<EntityRef>(), ..item() })
            }
            Build::Insert(index) => self.insert(body, index, grant),
            Build::LinkCoedge(index) => {
                let (_, first, size) = TopologyOffsets::face_uses(&self.boundary, index);
                let member = index - first;
                let current = body.coedges.get_mut(entity!(self, o.coedges + index, Coedge)).expect("original coedge");
                current.next = entity!(self, o.coedges + first + (member + 1) % size, Coedge);
                current.prev = entity!(self, o.coedges + first + (member + size - 1) % size, Coedge);
                self.build = if o.coedges + index + 1 == o.shell { Build::LinkLoop(0) } else { Build::LinkCoedge(index + 1) };
                Ok(item())
            }
            Build::LinkLoop(face) => {
                let original = body.loops.get_mut(entity!(self, o.loops + face, Loop)).expect("original loop");
                let first: usize = rings[..face].iter().sum();
                original.first = entity!(self, o.coedges + first, Coedge);
                self.build = if face + 1 == rings.len() { Build::LinkFace(0) } else { Build::LinkLoop(face + 1) };
                Ok(item())
            }
            Build::LinkFace(face) => {
                let original = body.faces.get_mut(entity!(self, o.faces + face, Face)).expect("original face");
                let mut receipt = item();
                if let Some(ring) = self.boundary.inner_ring(face) {
                    let bytes = size_of::<semio_framework_3d_brep_representation::arena::LoopId>();
                    if original.inners.capacity() == 0 {
                        original.inners.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "original face inner ring allocation failed"))?;
                        return Ok(Progress { retained_capacity_bytes: original.inners.capacity() * bytes, ..receipt });
                    }
                    original.inners.push(entity!(self, o.loops + ring, Loop));
                    receipt.copied_bytes = bytes;
                }
                original.outer = Some(entity!(self, o.loops + face, Loop));
                self.build = if face + 1 == self.boundary.face_count() { Build::ShellFaces(0) } else { Build::LinkFace(face + 1) };
                Ok(receipt)
            }
            Build::ShellFaces(shell) => {
                let count = self.boundary.shell_face_count(shell);
                if self.shell_faces.capacity() == 0 {
                    self.shell_faces.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "solid shell faces allocation failed"))?;
                    return Ok(Progress { retained_capacity_bytes: self.shell_faces.capacity() * size_of::<FaceId>(), ..item() });
                }
                let start: usize = (0..shell).map(|index| self.boundary.shell_face_count(index)).sum();
                let face = start + self.shell_faces.len();
                self.shell_faces.push(entity!(self, o.faces + face, Face));
                if self.shell_faces.len() == count { self.build = Build::Insert(o.shell + shell); }
                Ok(Progress { copied_bytes: size_of::<FaceId>(), ..item() })
            }
            Build::InnerShells => {
                let count = self.boundary.shell_count() - 1;
                if count == 0 {
                    self.build = Build::Insert(o.solid);
                    return Ok(item());
                }
                if self.inner_shells.capacity() == 0 {
                    self.inner_shells.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "solid inner shells allocation failed"))?;
                    return Ok(Progress { retained_capacity_bytes: self.inner_shells.capacity() * size_of::<ShellId>(), ..item() });
                }
                self.inner_shells.push(entity!(self, o.shell + self.inner_shells.len() + 1, Shell));
                if self.inner_shells.len() == count { self.build = Build::Insert(o.solid); }
                Ok(Progress { copied_bytes: size_of::<ShellId>(), ..item() })
            }
            Build::Ready => Err(ValueError::invariant("solid primitive topology has already completed")),
        }
    }
}

macro_rules! each_entity {
    ($entity:expr, $action:ident) => {
        match $entity {
            EntityRef::Vertex(id) => $action!(vertices, id),
            EntityRef::Edge(id) => $action!(edges, id),
            EntityRef::Coedge(id) => $action!(coedges, id),
            EntityRef::Loop(id) => $action!(loops, id),
            EntityRef::Face(id) => $action!(faces, id),
            EntityRef::Shell(id) => $action!(shells, id),
            EntityRef::Solid(id) => $action!(solids, id),
            EntityRef::Curve3(id) => $action!(curves3, id),
            EntityRef::Curve2(id) => $action!(curves2, id),
            EntityRef::Surface(id) => $action!(surfaces, id),
        }
    };
}

pub fn topology_slot_depth(body: &Body, entity: EntityRef) -> Result<usize, ValueError> {
    macro_rules! depth { ($store:ident, $id:ident) => { body.$store.next_remove_depth_demand($id) }; }
    each_entity!(entity, depth)
}

pub fn remove_topology_slot(body: &mut Body, entity: EntityRef) {
    macro_rules! remove { ($store:ident, $id:ident) => { drop(body.$store.remove($id).expect("unpublished topology arena entry")) }; }
    each_entity!(entity, remove)
}

impl SolidTopology {
    /// 🧾️ Quotes the funded release of one draft allocation or of the published journal.
    pub fn retirement_demand(&self, body: &Body, published: bool) -> Result<RetirementDemand, ValueError> {
        if published {
            return Ok(RetirementDemand { release_bytes: self.inserted.capacity() * size_of::<EntityRef>(), depth: 1, ..Default::default() });
        }
        let mut release_bytes = self.shell_faces.capacity() * size_of::<FaceId>();
        if release_bytes == 0 { release_bytes = self.inner_shells.capacity() * size_of::<ShellId>(); }
        if release_bytes != 0 { return Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() }); }
        let depth = if let Some(entity) = self.inserted.last().copied() {
            match entity {
                EntityRef::Face(id) => release_bytes = body.faces.get(id).expect("open solid face").inners.capacity() * size_of::<semio_framework_3d_brep_representation::arena::LoopId>(),
                EntityRef::Shell(id) => release_bytes = body.shells.get(id).expect("open solid shell").faces.capacity() * size_of::<FaceId>(),
                EntityRef::Solid(id) => release_bytes = body.solids.get(id).expect("open solid root").inners.capacity() * size_of::<ShellId>(),
                _ => {}
            }
            topology_slot_depth(body, entity)?
        } else { 1 };
        Ok(RetirementDemand { release_bytes, depth, ..Default::default() })
    }

    /// 🧹️ Frees the published journal or reverses one unpublished arena insertion.
    pub fn retire_one(&mut self, body: &mut Body, published: bool) -> bool {
        if published {
            drop(take(&mut self.inserted));
            return true;
        }
        if self.shell_faces.capacity() != 0 {
            drop(take(&mut self.shell_faces));
        } else if self.inner_shells.capacity() != 0 {
            drop(take(&mut self.inner_shells));
        } else if let Some(entity) = self.inserted.pop() {
            remove_topology_slot(body, entity);
        } else {
            return true;
        }
        false
    }
}
