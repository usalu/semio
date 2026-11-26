//! 🏗️ Funded solid construction and rollback over native body stores.

use semio_framework_3d_brep_boolean::certified_boundaries::{BoreBoundary, MAX_ORTHOGONAL_ENTITIES, OrthogonalBoundary, OrthogonalCells};
use semio_framework_3d_brep_construction::{circular_prism::CircularPrism,conical_frustum::ConicalFrustum,point_hull::HullBoundary, prism_boundary::PrismBoundary, primitives::{PrimitiveEdge, PrimitiveTrim, SolidPrimitive}};
use semio_framework_3d_brep_representation::arena::{ArenaId, FaceId, ShellId, SolidId, Store};
use semio_framework_3d_brep_representation::contract::{AnnularRevolution,BoxSectionDraft,ClippedBox,ClippedBoxBoundary,PerforatedPrism,PERFORATED_RING_SIZES,BorePreviewSource,RoundedBox,SphericalCaps,SphericalShell,TorusSector};
use semio_framework_3d_brep_representation::topology::{Body, Coedge, Edge, EntityRef, Face, Loop, Shell, Solid, Vertex, dummy_coedge, history::{LabelSource, PersistentLabel}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,matrix::Frame3};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_3d_brep_solid_topology::{clip_boundary, rounded_box_boundary, sector_boundary, spherical_cap_boundary};
use std::mem::{size_of, take};

use semio_framework_3d_brep_spline_construction::nurbs_cuboid::{self, Payload as SplinePayload, Recipe as SplineRecipe};

#[path = "🪅️owned-solid/🦀️.rs"]
mod owned_solid;
pub use owned_solid::OwnedSolidPublication;

#[path = "🪷️conic-face-draft/🦀️.rs"]
mod conic_face_draft;
pub use conic_face_draft::{Conic,ConicBoundary,ConicFacePublication,SectionArc};


#[path = "🧿️annular-boundary/🦀️.rs"]
mod annular_boundary;
pub use annular_boundary::plan_annular_revolution;

#[path="📐️conic-section/🦀️.rs"]
mod conic_section;
pub use conic_section::{apex_section,axial_cylinder_section,conic_section,frustum_section};

#[path = "🕳️circular-opening-offset/🦀️.rs"]
mod circular_opening_offset;
pub use circular_opening_offset::CircularOpeningOffset;

#[path = "📐️rectangular-extrusion/🦀️.rs"]
mod rectangular_extrusion;
pub use rectangular_extrusion::{RectangleProfile,rectangular_extrusion,rectangular_face_depth};

#[path = "🪟️perforated-extrusion/🦀️.rs"]
mod perforated_extrusion;
pub use perforated_extrusion::perforated_extrusion;

#[path = "📐️box-face-draft/🦀️.rs"]
mod box_face_draft;
#[path = "🧰️box-finish/🦀️.rs"]
mod box_finish;
pub use box_finish::BoxFinish;

#[path = "🧰️box-edge-chamfer/🦀️.rs"]
mod box_edge_chamfer;
pub use box_edge_chamfer::box_edge_chamfer;

#[path = "🔶️chamfered-box/🦀️.rs"]
mod chamfered_box;
pub use chamfered_box::ChamferedBox;

#[path = "📦️oriented-box/🦀️.rs"]
mod oriented_box;
pub use oriented_box::{BoxPrismBatch, BoxPrismSource, OrientedBox};

#[path = "🧮️boolean-recipe/🦀️.rs"]
pub mod boolean_recipe;

#[path = "🧵️cuboid-sew/🦀️.rs"]
mod cuboid_sew;
pub use cuboid_sew::CuboidSew;

#[path = "📐️solid-pattern/🦀️.rs"]
mod solid_pattern;
pub use solid_pattern::{SolidGroupOwnership, SolidPatternRecipe, UnpublishedSolidPattern};

#[derive(Clone, Copy)]
pub struct PrimitiveProvenance {
    pub primitive: SolidPrimitive,
    pub preview_first_label: Option<u64>,
    pub nurbs_cuboid: bool,
}
semio_framework_value::artifact_retire_leaf!(PrimitiveProvenance);

#[path="🐚️shell-recipe/🦀️.rs"]
mod shell_recipe;
pub use shell_recipe::{ShellStock, spherical_shell_distance};

#[path = "✂️plane-split/🦀️.rs"]
mod plane_split;
pub use plane_split::plane_split;

#[path = "🥫️circular-prism-query/🦀️.rs"]
mod circular_prism_query;
pub use circular_prism_query::circular_prism_distance;


#[path="📍️solid-distance/🦀️.rs"]
mod solid_distance;
#[path="↔️solid-offset/🦀️.rs"]
mod solid_offset;

#[derive(Clone, Copy)]
pub enum SolidProvenance {
    Primitive(PrimitiveProvenance),
    Bore(BorePreviewSource),
    CircularPrism(CircularPrism,u64),
    Frustum(ConicalFrustum,u64),
    TorusSector(TorusSector,u64),
    Annular(AnnularRevolution,u64),
    SphericalShell(SphericalShell,u64),
    SphericalCaps(SphericalCaps,u64),
    RoundedBox(RoundedBox,u64),
    ClippedBox(ClippedBox),
    Orthogonal(OrthogonalCells),
    PlacedOrthogonal(OrthogonalCells,Frame3),
    PlanarBoundary(Option<BoxPrismSource>),
}

impl SolidProvenance {
    /// ✂️ Opens a bounded edge cursor for certified convex box-derived stock.
    pub fn box_section(self,origin:[f64;3],normal:[f64;3])->Result<Option<BoxSectionDraft>,ValueError>{
        let draft=match self{
            Self::Primitive(source)=>OrientedBox::from_primitive(source.primitive).map(|stock|stock.section(origin,normal)),
            Self::PlanarBoundary(Some(source))=>source.section(origin,normal),
            Self::ClippedBox(stock)=>Some(BoxSectionDraft::clipped(stock,origin,normal)),
            _=>None,
        };
        draft.transpose().map_err(|message|ValueError::literal(ValueRefusalKind::InvalidValue,message))
    }

    /// ⚖️ Reads the analytic measure attached to a retained solid.
    pub fn volume(self) -> Option<f64> {
        match self {
            Self::Primitive(primitive) => Some(primitive.primitive.volume()),
            Self::Bore(source) => Some(source.volume),
            Self::CircularPrism(source,_) => Some(source.volume),
            Self::Frustum(source,_) => Some(source.volume),
            Self::TorusSector(source,_) => Some(source.volume()),
            Self::Annular(source,_) => Some(source.volume()),
            Self::SphericalShell(source,_)=>Some(source.volume()),
            Self::SphericalCaps(source,_)=>Some(source.volume()),
            Self::RoundedBox(source,_)=>Some(source.volume()),
            Self::PlanarBoundary(..) | Self::Orthogonal(_) | Self::PlacedOrthogonal(..) | Self::ClippedBox(_) => None,
        }
    }
}

semio_framework_value::artifact_retire_leaf!(SolidProvenance);

pub enum SolidBoundary {
    Analytic(SolidPrimitive),
    Orthogonal(OrthogonalBoundary),
    Prism(PrismBoundary),
    BoxPrism(PrismBoundary, BoxPrismSource),
    PerforatedBox(PerforatedPrism, BoxPrismSource),
    Bore(BoreBoundary),
    CircularPrism(CircularPrism),
    Frustum(ConicalFrustum),
    TorusSector(TorusSector),
    Annular(AnnularRevolution),
    SphericalShell(SphericalShell),
    SphericalCaps(SphericalCaps),
    RoundedBox(RoundedBox),
    ClippedBox(ClippedBoxBoundary,ClippedBox),
    Hull(HullBoundary),
    ChamferedBox(ChamferedBox),
}

/// 📥️ Exact bounded construction certified by an exchange cursor.
pub enum SolidExchangeBoundary {
    Prism(PrismBoundary),
    RoundedBox(RoundedBox),
}

impl From<SolidExchangeBoundary> for SolidBoundary {
    fn from(source: SolidExchangeBoundary) -> Self {
        match source {
            SolidExchangeBoundary::Prism(shape) => Self::Prism(shape),
            SolidExchangeBoundary::RoundedBox(shape) => Self::RoundedBox(shape),
        }
    }
}

impl SolidBoundary {
    fn vertex_count(&self) -> usize {
        match self { Self::Analytic(p) => p.vertex_count(), Self::Orthogonal(p) => p.vertex_count(), Self::Prism(p) | Self::BoxPrism(p, _) => p.vertex_count(), Self::PerforatedBox(..) => 16, Self::Bore(_) => 10, Self::CircularPrism(p) => p.vertex_count(), Self::Frustum(_) | Self::TorusSector(_) => 2, Self::SphericalShell(p) => shell_recipe::exterior(*p).vertex_count()+2, Self::SphericalCaps(p)=>p.vertex_count(), Self::Annular(p) => p.vertex_count(), Self::Hull(p) => p.vertex_count(), Self::ChamferedBox(p) => p.vertex_count(), Self::ClippedBox(p,_)=>p.vertex_count(), Self::RoundedBox(p) => p.vertex_count() }
    }
    fn edge_count(&self) -> usize {
        match self { Self::Analytic(p) => p.edge_count(), Self::Orthogonal(p) => p.edge_count(), Self::Prism(p) | Self::BoxPrism(p, _) => p.edge_count(), Self::PerforatedBox(..) => 24, Self::Bore(_) => 15, Self::CircularPrism(p) => p.edge_count(), Self::Frustum(_) | Self::TorusSector(_) => 3, Self::SphericalShell(p) => shell_recipe::exterior(*p).edge_count()+3, Self::SphericalCaps(p)=>p.edge_count(), Self::Annular(p) => p.edge_count(), Self::Hull(p) => p.edge_count(), Self::ChamferedBox(p) => p.edge_count(), Self::ClippedBox(p,_)=>p.edge_count(), Self::RoundedBox(p) => p.edge_count() }
    }
    fn ring_sizes(&self) -> &[usize] {
        match self { Self::Analytic(p) => p.ring_sizes(), Self::Orthogonal(p) => p.ring_sizes(), Self::Prism(p) | Self::BoxPrism(p, _) => p.ring_sizes(), Self::PerforatedBox(..) => &PERFORATED_RING_SIZES, Self::Bore(_) => &[4,4,4,4,4,4,4,1,1], Self::CircularPrism(p) => p.ring_sizes(), Self::Frustum(_) | Self::TorusSector(_) => &[4,1,1], Self::SphericalShell(p) => shell_recipe::rings(*p), Self::SphericalCaps(p)=>p.ring_sizes(), Self::Annular(p) => annular_boundary::ring_sizes(*p), Self::Hull(p) => p.ring_sizes(), Self::ChamferedBox(p) => p.ring_sizes(), Self::ClippedBox(p,_)=>p.ring_sizes(), Self::RoundedBox(p) => p.ring_sizes() }
    }
    fn face_count(&self) -> usize {
        if let Self::Annular(shape)=self {shape.face_count()} else if let Self::Bore(shape)=self {shape.preview_source(0).face_count()} else if matches!(self,Self::PerforatedBox(..)) {10} else { self.ring_sizes().len() }
    }
    fn ring_owner(&self, ring: usize) -> usize {
        if let Self::Annular(shape)=self {annular_boundary::face_index(*shape,ring)}
        else if matches!(self,Self::PerforatedBox(..)) && ring>=10 {ring-10}
        else if let Self::Bore(shape)=self {if ring>=7{shape.preview_source(0).cap_face(ring==8)}else{ring}} else { ring }
    }
    fn inner_ring(&self, face: usize) -> Option<usize> {
        if matches!(self,Self::PerforatedBox(..)) { return (face<2).then_some(10+face); }
        if matches!(self,Self::Annular(shape) if shape.has_two_steps()) { return match face{2=>Some(7),4=>Some(8),_=>None}; }
        if let Self::Annular(shape)=self { return match (shape.full(),face){(true,0) if !shape.has_floor()&&!shape.has_step()=>Some(4),(true,2) if shape.has_step()=>Some(4),(true,2) if !shape.has_void_shell()=>Some(5),_=>None};}
        matches!(self,Self::Bore(shape) if face<2&&shape.preview_source(0).is_open(face==1)).then_some(7+face)
    }
    fn face_flipped(&self, face: usize) -> bool {
        matches!(self, Self::Bore(_)) && face >= 6
            || matches!(self,Self::SphericalShell(shape) if face+1==shell_recipe::rings(*shape).len())
            || matches!(self,Self::SphericalCaps(shape) if shape.is_inward(face))
            || matches!(self,Self::Annular(shape) if annular_boundary::face_flipped(*shape,face))
            || matches!(self,Self::CircularPrism(shape) if shape.face_flipped(face))
    }
    fn shell_count(&self) -> usize {
        match self {
            Self::Orthogonal(boundary) => boundary.shell_sizes().len(),
            Self::Hull(boundary)=>boundary.shell_count(),
            Self::Annular(shape) if shape.has_void_shell()=>2,
            Self::SphericalShell(_)=>2,
            Self::Bore(shape) if shape.preview_source(0).closed_bore()=>2,
            _ => 1,
        }
    }
    fn shell_face_count(&self, shell: usize) -> usize {
        match self {
            Self::Orthogonal(boundary) => boundary.shell_sizes()[shell],
            Self::Hull(boundary)=>boundary.shell_face_count(shell),
            Self::Annular(shape) if shape.has_void_shell()=>3,
            Self::SphericalShell(shape)=>if shell==0{shell_recipe::rings(*shape).len()-1}else{1},
            Self::Bore(shape) if shape.preview_source(0).closed_bore()=>if shell==0{6}else{3},
            _ => self.face_count(),
        }
    }
    fn vertex(&self, i: usize) -> Pnt3 {
        match self { Self::Analytic(p) => p.vertex(i), Self::Orthogonal(p) => p.vertex(i), Self::Prism(p) | Self::BoxPrism(p, _) => p.vertex(i), Self::PerforatedBox(p, _) => p.vertex(i), Self::Bore(p) => p.vertex(i), Self::CircularPrism(p) => p.vertex(i), Self::Frustum(p) => p.vertex(i), Self::TorusSector(p) => p.vertex(i), Self::Annular(p) => p.vertex(i), Self::SphericalShell(p)=>shell_recipe::vertex(*p,i), Self::SphericalCaps(p)=>p.vertex(i), Self::Hull(p) => p.vertex(i), Self::ChamferedBox(p) => p.vertex(i), Self::ClippedBox(p,_)=>p.vertex(i), Self::RoundedBox(p) => p.vertex(i) }
    }
    fn edge(&self, i: usize) -> PrimitiveEdge {
        match self { Self::Analytic(p) => p.edge(i), Self::Orthogonal(p) => p.edge(i), Self::Prism(p) | Self::BoxPrism(p, _) => p.edge(i), Self::PerforatedBox(p, _) => oriented_box::perforated_edge(p,i), Self::Bore(p) => p.edge(i), Self::CircularPrism(p) => p.edge(i), Self::Frustum(p) => p.edge(i), Self::TorusSector(p) => sector_boundary::edge(*p,i), Self::Annular(p) => annular_boundary::edge(*p,i), Self::SphericalShell(p)=>shell_recipe::edge(*p,i), Self::SphericalCaps(p)=>{let (curve,range,vertices)=p.edge(i);PrimitiveEdge{curve,range,vertices}}, Self::Hull(p) => p.edge(i), Self::ChamferedBox(p) => p.edge(i), Self::ClippedBox(p,_)=>clip_boundary::edge(p,i), Self::RoundedBox(p) => rounded_box_boundary::edge(*p,i) }
    }
    fn surface(&self, i: usize) -> Surface {
        match self { Self::Analytic(p) => p.surface(i), Self::Orthogonal(p) => p.surface(i), Self::Prism(p) | Self::BoxPrism(p, _) => p.surface(i), Self::PerforatedBox(p, _) => Surface::Plane{frame:p.frame(i)}, Self::Bore(p) => p.surface(i), Self::CircularPrism(p) => p.surface(i), Self::Frustum(p) => p.surface(i), Self::TorusSector(p) => sector_boundary::surface(*p,i), Self::Annular(p) => annular_boundary::surface(*p,i), Self::SphericalShell(p)=>shell_recipe::surface(*p,i), Self::SphericalCaps(p)=>spherical_cap_boundary::surface(*p,i), Self::Hull(p) => p.surface(i), Self::ChamferedBox(p) => p.surface(i), Self::ClippedBox(p,_)=>Surface::Plane{frame:p.frame(i)}, Self::RoundedBox(p) => rounded_box_boundary::surface(*p,i) }
    }
    fn trim(&self, i: usize, member: usize) -> PrimitiveTrim {
        match self { Self::Analytic(p) => p.trim(i, member), Self::Orthogonal(p) => p.trim(i, member), Self::Prism(p) | Self::BoxPrism(p, _) => p.trim(i, member), Self::PerforatedBox(p, _) => oriented_box::perforated_trim(p,i,member), Self::Bore(p) => p.trim(i, member), Self::CircularPrism(p) => p.trim(i, member), Self::Frustum(p) => p.trim(i, member), Self::TorusSector(p) => sector_boundary::trim(*p,i, member), Self::Annular(p) => annular_boundary::trim(*p,i, member), Self::SphericalShell(p)=>shell_recipe::trim(*p,i,member), Self::SphericalCaps(p)=>spherical_cap_boundary::trim(*p,i,member), Self::Hull(p) => p.trim(i, member), Self::ChamferedBox(p) => p.trim(i, member), Self::ClippedBox(p,_)=>clip_boundary::trim(p,i,member), Self::RoundedBox(p) => rounded_box_boundary::trim(*p,i, member) }
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

/// 🏗️ A private transaction over the native entity stores. Only its final solid receives a live handle.
pub struct SolidPublication {
    owner: usize,
    boundary: SolidBoundary,
    offsets: TopologyOffsets,
    first_label: u64,
    handle: [u8; 64],
    inserted: Vec<EntityRef>,
    shell_faces: Vec<FaceId>,
    inner_shells: Vec<ShellId>,
    build: Build,
    spline: Option<SplinePayload>,
    open: bool,
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

impl SolidPublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("the digest uses lowercase hexadecimal") }
    pub fn topology_ready(&self) -> bool { matches!(self.build, Build::Ready) }
    pub fn terminal_is_empty(&self) -> bool { !self.open && self.inserted.capacity() == 0 }
    fn label(&self, offset: usize) -> PersistentLabel { PersistentLabel(self.first_label + offset as u64) }

    pub fn check_owner(&self, owner: usize) -> Result<(), ValueError> {
        if self.open && self.owner == owner { Ok(()) }
        else { Err(ValueError::invariant("solid primitive publication requires its original unpublished session")) }
    }

    fn spline_recipe(&self, index: usize) -> Option<SplineRecipe> {
        let SolidBoundary::Analytic(primitive) = &self.boundary else { return None; };
        let o = self.offsets;
        if (o.curves..o.edges).contains(&index) { return Some(SplineRecipe::edge(*primitive, index - o.curves)); }
        (o.surfaces..o.faces).contains(&index).then(|| SplineRecipe::face(*primitive, index - o.surfaces))
    }

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
            i if i < o.edges => if let Some(spline) = &mut self.spline {
                let store = &mut body.curves3;
                if store.next_insert_capacity_byte_demand(0)? != 0 {
                    return store.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
                }
                let SplinePayload::Edge(curve) = take(spline) else { return Err(ValueError::invariant("cuboid edge spline is missing")); };
                match store.insert_reserved(curve, grant) {
                    Ok((id, receipt)) => (Some(EntityRef::Curve3(id)), receipt),
                    Err((error, curve)) => { *spline = SplinePayload::Edge(curve); return Err(error); }
                }
            } else { put!(curves3, Curve3, primitive.edge(i - o.curves).curve) },
            i if i < o.surfaces => {
                let edge = i - o.edges;
                let spec = primitive.edge(edge);
                let [a, b] = spec.vertices;
                put!(edges, Edge, Edge {
                    curve: entity!(self, o.curves + edge, Curve3), range: spec.range, tol: Tol::DEFAULT,
                    v0: entity!(self, a, Vertex), v1: entity!(self, b, Vertex), label: self.label(o.curves + edge),
                })
            }
            i if i < o.faces => if let Some(spline) = &mut self.spline {
                let store = &mut body.surfaces;
                if store.next_insert_capacity_byte_demand(0)? != 0 {
                    return store.reserve_insert_step(grant).map_err(|(error, receipt)| error.with_retained_progress(receipt));
                }
                let SplinePayload::Face(surface) = take(spline) else { return Err(ValueError::invariant("cuboid face spline is missing")); };
                match store.insert_reserved(surface, grant) {
                    Ok((id, receipt)) => (Some(EntityRef::Surface(id)), receipt),
                    Err((error, surface)) => { *spline = SplinePayload::Face(surface); return Err(error); }
                }
            } else { put!(surfaces, Surface, primitive.surface(i - o.surfaces)) },
            i if i < o.loops => {
                let face = i - o.faces;
                put!(faces, Face, Face {
                    surface: entity!(self, o.surfaces + face, Surface), outer: None, inners: Vec::new(),
                    flipped: primitive.face_flipped(face), tol: Tol::DEFAULT, label: self.label(primitive.vertex_count() + primitive.edge_count() + face),
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
        if let (Build::Insert(index), Some(spline)) = (self.build, &self.spline) {
            if (o.curves..o.edges).contains(&index) || (o.surfaces..o.faces).contains(&index) {
                if let Some(demand) = spline.demand() { return Ok(demand); }
            }
        }
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
            Build::Insert(index) => {
                if self.spline.as_ref().is_some_and(|spline| spline.demand().is_some()) {
                    if let Some(recipe) = self.spline_recipe(index) {
                        return self.spline.as_mut().expect("cuboid spline state").prepare(recipe);
                    }
                }
                self.insert(body, index, grant)
            },
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

impl SolidPublication {
    /// 🎟️ Reserves stable labels for one original unpublished body transaction.
    pub fn begin(owner: usize, boundary: SolidBoundary, labels: &mut LabelSource, handle: impl FnOnce(u64) -> [u8; 64]) -> Result<Self, ValueError> {
        let primitive = &boundary;
        let first_label = labels.next();
        let count = primitive.vertex_count() + primitive.edge_count() + primitive.face_count() + primitive.shell_count() + 1;
        let next = first_label.checked_add(count as u64).ok_or_else(|| ValueError::invariant("solid primitive label range exhausted"))?;
        *labels = LabelSource::from_next(next);
        let offsets = TopologyOffsets::new(primitive);
        Ok(Self {
            owner, boundary, offsets,
            first_label, handle: handle(next - 1), inserted: Vec::new(),
            shell_faces: Vec::new(), inner_shells: Vec::new(), build: Build::Journal, spline: None, open: true,
        })
    }

    /// 🧵️ Opens a cuboid publication whose original edges and face supports use degree-one NURBS.
    pub fn begin_nurbs_cuboid(owner: usize, primitive: SolidPrimitive, labels: &mut LabelSource, handle: impl FnOnce(u64) -> [u8; 64]) -> Result<Self, ValueError> {
        if !matches!(primitive, SolidPrimitive::Box { .. }) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "NURBS conversion requires a certified cuboid"));
        }
        primitive.validate().map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        if !(0..primitive.edge_count()).all(|i| SplineRecipe::edge(primitive, i).is_finite())
            || !(0..primitive.ring_sizes().len()).all(|i| SplineRecipe::face(primitive, i).is_finite())
        {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "cuboid NURBS conversion requires finite controls and noncollapsed knot spans"));
        }
        let mut plan = Self::begin(owner, SolidBoundary::Analytic(primitive), labels, handle)?;
        plan.spline = Some(SplinePayload::Empty);
        Ok(plan)
    }

    /// 🧭️ Reveals the arena identity and provenance only once its complete boundary is ready.
    pub fn product(&self, owner: usize) -> Result<(SolidId, Option<SolidProvenance>), ValueError> {
        self.check_owner(owner)?;
        if !self.topology_ready() { return Err(ValueError::invariant("solid topology is still unpublished")); }
        let provenance = match &self.boundary {
            SolidBoundary::Analytic(primitive) => Some(SolidProvenance::Primitive(PrimitiveProvenance { primitive: *primitive, preview_first_label: Some(self.first_label), nurbs_cuboid: self.spline.is_some() })),
            SolidBoundary::Bore(shape) => Some(SolidProvenance::Bore(shape.preview_source(self.first_label))),
            SolidBoundary::CircularPrism(shape) => Some(SolidProvenance::CircularPrism(*shape,self.first_label)),
            SolidBoundary::Frustum(shape) => Some(SolidProvenance::Frustum(*shape,self.first_label)),
            SolidBoundary::TorusSector(shape) => Some(SolidProvenance::TorusSector(*shape,self.first_label)),
            SolidBoundary::Annular(shape) => Some(SolidProvenance::Annular(*shape,self.first_label)),
            SolidBoundary::SphericalShell(shape) => Some(SolidProvenance::SphericalShell(*shape,self.first_label)),
            SolidBoundary::SphericalCaps(shape)=>Some(SolidProvenance::SphericalCaps(*shape,self.first_label)),
            SolidBoundary::RoundedBox(shape)=>Some(SolidProvenance::RoundedBox(*shape,self.first_label)),
            SolidBoundary::ClippedBox(_,proof)=>Some(SolidProvenance::ClippedBox(*proof)),
            SolidBoundary::Orthogonal(shape) => Some(match shape.placement(){
                Some(frame)=>SolidProvenance::PlacedOrthogonal(shape.material(),frame),None=>SolidProvenance::Orthogonal(shape.material()),
            }),
            SolidBoundary::BoxPrism(_, shape) | SolidBoundary::PerforatedBox(_, shape) => Some(SolidProvenance::PlanarBoundary(Some(*shape))),
            SolidBoundary::Prism(_) | SolidBoundary::Hull(_) | SolidBoundary::ChamferedBox(_) => Some(SolidProvenance::PlanarBoundary(None)),
        };
        Ok((entity!(self, self.offsets.solid, Solid), provenance))
    }

    /// 🏷️ Transfers the body entities out of rollback custody after live indexing succeeds.
    pub fn mark_published(&mut self) { self.open = false; }
    pub fn is_open(&self) -> bool { self.open }
    pub fn owns(&self, owner: usize) -> bool { self.owner == owner }

    /// 🧹️ Quotes the immediate release of one unpublished arena value under original stock.
    pub fn close_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if !self.open {
            return Ok(RetirementDemand { release_bytes: self.inserted.capacity() * size_of::<EntityRef>(), depth: 1, ..Default::default() });
        }
        let spline_bytes = self.spline.as_ref().map_or(0, SplinePayload::retained_bytes);
        if spline_bytes != 0 { return Ok(RetirementDemand { release_bytes: spline_bytes, depth: 1, ..Default::default() }); }
        let mut release_bytes = self.shell_faces.capacity() * size_of::<FaceId>();
        if release_bytes == 0 { release_bytes = self.inner_shells.capacity() * size_of::<ShellId>(); }
        if release_bytes != 0 { return Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() }); }
        let depth = if let Some(entity) = self.inserted.last().copied() {
            match entity {
                EntityRef::Face(id) => release_bytes = body.faces.get(id).expect("open solid face").inners.capacity() * size_of::<semio_framework_3d_brep_representation::arena::LoopId>(),
                EntityRef::Shell(id) => release_bytes = body.shells.get(id).expect("open solid shell").faces.capacity() * size_of::<FaceId>(),
                EntityRef::Solid(id) => release_bytes = body.solids.get(id).expect("open solid root").inners.capacity() * size_of::<ShellId>(),
                EntityRef::Curve3(id) if self.spline.is_some() => release_bytes = nurbs_cuboid::curve_bytes(body.curves3.get(id).expect("open cuboid edge spline")),
                EntityRef::Surface(id) if self.spline.is_some() => release_bytes = nurbs_cuboid::surface_bytes(body.surfaces.get(id).expect("open cuboid face spline")),
                _ => {}
            }
            topology_slot_depth(body, entity)?
        } else { 1 };
        Ok(RetirementDemand { release_bytes, depth, ..Default::default() })
    }

    /// ♻️ Returns an unpublished solid's arena slots in reverse dependency order.
    pub fn close(&mut self, body: &mut Body, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        let demand = self.close_demand(body)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok(Step::Progress(Progress::default())); }
        let receipt = Progress { released_bytes: demand.release_bytes, ..item() };
        if !self.open {
            drop(take(&mut self.inserted));
            return Ok(Step::Complete(receipt));
        }
        if let Some(spline) = self.spline.as_mut().filter(|spline| spline.retained_bytes() != 0) {
            drop(take(spline));
        } else if self.shell_faces.capacity() != 0 {
            drop(take(&mut self.shell_faces));
        } else if self.inner_shells.capacity() != 0 {
            drop(take(&mut self.inner_shells));
        } else if let Some(entity) = self.inserted.pop() {
            remove_topology_slot(body, entity);
        } else {
            self.open = false;
            if self.terminal_is_empty() { return Ok(Step::Complete(receipt)); }
        }
        Ok(Step::Progress(receipt))
    }
}
