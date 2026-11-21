//! 📦️ Exact support-plane scan for a small original point hull.

use crate::primitives::{PrimitiveEdge, PrimitiveTrim, planar_pcurve};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_intersect::convex_box::{BoxSupportBoundary,BOX_BOUNDARY_VERTICES};
use semio_framework_3d_brep_representation::contract::Vec3 as EVec3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec3 as NativeVec3};
use semio_framework_3d_brep_vector::predicates::{Orient, orient2d, orient3d};
use std::mem::take;

const MAX_POINTS: usize = 16;
const MAX_EDGES: usize = 3 * MAX_POINTS - 6;
const MAX_FACES: usize = 2 * MAX_POINTS - 4;

/// ⚠️ A point hull rejects invalid geometry or an out-of-order draft operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HullRefusal {
    InvalidGeometry(&'static str),
    UnexpectedStep(&'static str),
}

impl std::fmt::Display for HullRefusal {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGeometry(message) | Self::UnexpectedStep(message) => output.write_str(message),
        }
    }
}

impl std::error::Error for HullRefusal {}

#[derive(Clone, Copy, Default)]
struct Facet {
    mask: u16,
    support:Option<(Pnt3,NativeVec3)>,
    vertices: [u8; MAX_POINTS],
    edges: [u8; MAX_POINTS],
}

/// 🧱️ Fixed storage for the certified faces, edges, and vertices of a planar boundary.
pub struct HullBoundary {
    points: [Pnt3; BOX_BOUNDARY_VERTICES],
    vertices: [u8; BOX_BOUNDARY_VERTICES],
    edges: [[u8; 2]; MAX_EDGES],
    uses: [[u8; 2]; MAX_EDGES],
    faces: [Facet; MAX_FACES],
    rings: [usize; MAX_FACES],
    vertex_count: usize,
    edge_count: usize,
    face_count: usize,
    inner_faces:usize,
}

impl Default for HullBoundary {
    fn default() -> Self {
        Self {
            points: [Pnt3::default();BOX_BOUNDARY_VERTICES], vertices: [0;BOX_BOUNDARY_VERTICES], edges: [[0; 2]; MAX_EDGES],
            uses: [[0; 2]; MAX_EDGES], faces: [Facet::default(); MAX_FACES], rings: [0; MAX_FACES],
            vertex_count: 0, edge_count: 0, face_count: 0, inner_faces:0,
        }
    }
}

fn projection(p: Pnt3, drop_axis: usize) -> Pnt2 {
    let xyz = [p.x, p.y, p.z];
    Pnt2::new(xyz[(drop_axis + 1) % 3], xyz[(drop_axis + 2) % 3])
}

fn ordinary_normal(a: Pnt3, b: Pnt3, c: Pnt3) -> NativeVec3 {
    (b - a).cross(c - a)
}

fn convex_ring(points: &[Pnt3], mask: u16, axis: usize) -> ([u8; MAX_POINTS * 2], usize) {
    let mut order = [0u8; MAX_POINTS];
    let mut count = 0;
    for index in 0..points.len() {
        if mask & (1u16 << index) != 0 {
            order[count] = index as u8;
            count += 1;
        }
    }
    let projected = |id: u8| projection(points[usize::from(id)], axis);
    order[..count].sort_unstable_by(|&left, &right| {
        let a = projected(left);
        let b = projected(right);
        a.x.partial_cmp(&b.x).unwrap().then_with(|| a.y.partial_cmp(&b.y).unwrap())
    });
    let mut ring = [0; MAX_POINTS * 2];
    let mut end = 0;
    let mut start = 0;
    for reverse in [false, true] {
        for rank in 0..count {
            let id = order[if reverse { count - 1 - rank } else { rank }];
            while end >= start + 2 && orient2d(projected(ring[end - 2]), projected(ring[end - 1]), projected(id)) != Orient::Positive {
                end -= 1;
            }
            ring[end] = id;
            end += 1;
        }
        end -= 1;
        start = end;
    }
    (ring, end)
}

impl HullBoundary {
    /// 🏔️ Uses authored supports from a certified closed planar shell.
    pub fn from_box_faces(plan:BoxSupportBoundary)->Option<Self>{
        if !plan.certifies_shell_layout(){return None;}
        let mut boundary=Self{points:plan.points,inner_faces:plan.inner_faces,..Self::default()};
        for face in plan.faces(){
            boundary.add_facet(0,&face.ring[..face.count]).ok()?;
            boundary.faces[boundary.face_count-1].support=Some((face.origin,face.normal));
        }
        (boundary.vertex_count==plan.point_count).then_some(boundary)
    }

    fn is_closed(&self)->bool{
        self.face_count>=4&&self.vertex_count+self.face_count==self.edge_count+2*self.shell_count()
            &&self.uses[..self.edge_count].iter().all(|count|*count==[1,1])
    }

    pub fn shell_count(&self)->usize{1+usize::from(self.inner_faces!=0)}
    pub fn shell_face_count(&self,index:usize)->usize{[self.face_count-self.inner_faces,self.inner_faces][index]}
    pub fn vertex_count(&self) -> usize { self.vertex_count }
    pub fn edge_count(&self) -> usize { self.edge_count }
    pub fn ring_sizes(&self) -> &[usize] { &self.rings[..self.face_count] }
    pub fn vertex(&self, index: usize) -> Pnt3 { self.points[usize::from(self.vertices[index])] }

    pub fn edge(&self, index: usize) -> PrimitiveEdge {
        let vertices = self.edges[index].map(usize::from);
        let [a, b] = vertices.map(|id| self.vertex(id));
        PrimitiveEdge { vertices, curve: Curve3::Line { origin: a, dir: b - a }, range: (0.0, 1.0) }
    }

    pub fn surface(&self, index: usize) -> Surface {
        let face=&self.faces[index];
        let (origin,normal)=face.support.unwrap_or_else(||{
            let [a,b,c]=[face.vertices[0],face.vertices[1],face.vertices[2]].map(|id|self.vertex(usize::from(id)));
            (a,ordinary_normal(a,b,c))
        });
        Surface::Plane { frame:Frame3::from_normal(origin,normal).expect("certified hull face frame") }
    }

    pub fn trim(&self, index: usize, member: usize) -> PrimitiveTrim {
        let face = &self.faces[index];
        let edge = usize::from(face.edges[member]);
        let spec = self.edge(edge);
        let Surface::Plane { frame } = self.surface(index) else { unreachable!("planar hull facet") };
        PrimitiveTrim {
            edge, forward: spec.vertices[0] == usize::from(face.vertices[member]),
            curve: planar_pcurve(&spec.curve, frame), range: spec.range,
        }
    }

    fn vertex_index(&mut self, point: u8) -> u8 {
        if let Some(index) = self.vertices[..self.vertex_count].iter().position(|&id| id == point) {
            return index as u8;
        }
        let id = self.vertex_count;
        self.vertices[id] = point;
        self.vertex_count += 1;
        id as u8
    }

    fn add_facet(&mut self, mask: u16, ring: &[u8]) -> Result<(), HullRefusal> {
        let failed = || HullRefusal::InvalidGeometry("convex hull boundary exceeded its planar topology limits");
        let mut face = Facet { mask, ..Facet::default() };
        if !(3..=face.vertices.len()).contains(&ring.len()) { return Err(failed()); }
        for (index, &point) in ring.iter().enumerate() {
            face.vertices[index] = self.vertex_index(point);
        }
        for member in 0..ring.len() {
            let [a, b] = [face.vertices[member], face.vertices[(member + 1) % ring.len()]];
            if (self.vertex(usize::from(a)) - self.vertex(usize::from(b))).norm() <= Tol::DEFAULT.value() {
                return Err(HullRefusal::InvalidGeometry("convex hull edge collapsed at kernel tolerance"));
            }
            let key = [a.min(b), a.max(b)];
            let index = match self.edges[..self.edge_count].iter().position(|&edge| edge == key) {
                Some(index) => index,
                None => {
                    let index = self.edge_count;
                    *self.edges.get_mut(index).ok_or_else(failed)? = key;
                    self.edge_count += 1;
                    index
                }
            };
            self.uses[index][usize::from(a < b)] += 1;
            face.edges[member] = index as u8;
        }
        *self.faces.get_mut(self.face_count).ok_or_else(failed)? = face;
        self.rings[self.face_count] = ring.len();
        self.face_count += 1;
        Ok(())
    }

    fn scan_plane(&mut self, triple: [usize; 3], count: usize) -> Result<(), HullRefusal> {
        let [a, b, c] = triple.map(|index| self.points[index]);
        let normal = ordinary_normal(a, b, c);
        let axis = [normal.x.abs(), normal.y.abs(), normal.z.abs()].into_iter().enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(axis, _)| axis).unwrap();
        if orient2d(projection(a, axis), projection(b, axis), projection(c, axis)) == Orient::Zero {
            return Ok(());
        }
        let (mut positive, mut negative, mut interior, mut mask) = (false, false, 0, 0u16);
        for index in 0..count {
            match orient3d(a, b, c, self.points[index]) {
                Orient::Positive => { positive = true; interior = index; }
                Orient::Negative => { negative = true; interior = index; }
                Orient::Zero => mask |= 1u16 << index,
            }
            if positive && negative { return Ok(()); }
        }
        if !positive && !negative || self.faces[..self.face_count].iter().any(|face| face.mask == mask) {
            return Ok(());
        }
        let (mut ring, length) = convex_ring(&self.points[..count], mask, axis);
        let ring = &mut ring[..length];
        if ring.len() < 3 { return Err(HullRefusal::InvalidGeometry("convex hull facet collapsed")); }
        let point = |id: u8| self.points[usize::from(id)];
        let [p, q, r] = [point(ring[0]), point(ring[1]), point(ring[2])];
        if orient3d(p, q, r, self.points[interior]) == Orient::Positive { ring.reverse(); }
        let [p, q, r] = [point(ring[0]), point(ring[1]), point(ring[2])];
        if Frame3::from_normal(p, ordinary_normal(p, q, r)).is_none() {
            return Err(HullRefusal::InvalidGeometry("convex hull face plane is numerically degenerate"));
        }
        self.add_facet(mask, ring)
    }
}

/// 🧮️ Acquires one point or checks one supporting-plane candidate per original turn.
pub struct HullDraft {
    expected: usize,
    received: usize,
    unique: usize,
    triple: Option<[usize; 3]>,
    boundary: HullBoundary,
}

impl HullDraft {
    /// 🎟️ Admits a bounded original hull without allocating its point or topology storage.
    pub fn new(count: usize) -> Result<Self, HullRefusal> {
        if !(4..=MAX_POINTS).contains(&count) {
            return Err(HullRefusal::InvalidGeometry("original convex hull requires four to sixteen points"));
        }
        Ok(Self { expected: count, received: 0, unique: 0, triple: Some([0, 1, 2]), boundary: HullBoundary::default() })
    }

    pub fn next_point(&self) -> Option<usize> { (self.received < self.expected).then_some(self.received) }
    pub fn has_candidate(&self) -> bool { self.triple.is_some() }

    /// 📍️ Saves one representable input coordinate, merging exact duplicates.
    pub fn add_point(&mut self, axes: EVec3) -> Result<(), HullRefusal> {
        if self.next_point().is_none() { return Err(HullRefusal::UnexpectedStep("convex hull source already acquired")); }
        if axes.into_iter().any(|axis| !axis.is_finite() || axis.abs() > 1e90) {
            return Err(HullRefusal::InvalidGeometry("convex hull coordinates exceed the finite predicate range"));
        }
        let [x, y, z] = axes;
        let point = Pnt3::new(x, y, z);
        if !self.boundary.points[..self.unique].contains(&point) {
            self.boundary.points[self.unique] = point;
            self.unique += 1;
        }
        self.received += 1;
        if self.received == self.expected && self.unique < 4 {
            return Err(HullRefusal::InvalidGeometry("convex hull requires at least four distinct points"));
        }
        Ok(())
    }

    /// 🔍️ Tests one candidate plane and advances the fixed-size support scan.
    pub fn scan_step(&mut self) -> Result<(), HullRefusal> {
        let Some(triple @ [a, b, c]) = self.triple else { return Err(HullRefusal::UnexpectedStep("convex hull plane scan is complete")); };
        if self.next_point().is_some() { return Err(HullRefusal::UnexpectedStep("convex hull source is incomplete")); }
        self.boundary.scan_plane(triple, self.unique)?;
        self.triple = if c + 1 < self.unique { Some([a, b, c + 1]) }
            else if b + 2 < self.unique { Some([a, b + 1, b + 2]) }
            else if a + 3 < self.unique { Some([a + 1, a + 2, a + 3]) }
            else { None };
        Ok(())
    }

    pub fn finish(&mut self) -> Result<HullBoundary, HullRefusal> {
        if self.has_candidate() || self.next_point().is_some() {
            return Err(HullRefusal::UnexpectedStep("convex hull must finish its support scan before publication"));
        }
        if !self.boundary.is_closed() {
            return Err(HullRefusal::InvalidGeometry("points do not enclose a nondegenerate convex solid"));
        }
        Ok(take(&mut self.boundary))
    }
}
