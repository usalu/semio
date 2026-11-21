//! 🟠️ Evidence that all STEP faces and shared trims comprise one exact rounded box.

use super::*;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::contract::{RoundedBox, box_finish_layout::{TOPOLOGY, face}};
use semio_framework_3d_brep_tolerance::Tol;

const FACES: usize = 26;
const TOLERANCE: f64 = 2.5e-8;
const COMPLETE: u32 = (1 << FACES) - 1;

pub(super) struct RoundedProof {
    planes: [[f64;2];3],
    plane_counts: [usize;6],
    radius: Option<f64>,
    curved: bool,
    face_mask: u32,
    edges: [u64;48],
    vertices: [u64;24],
}

enum Support {
    Plane { origin: Pnt3, normal: Vec3 },
    Cylinder { center: Pnt3, axis: Vec3, radius: f64 },
    Sphere { center: Pnt3, radius: f64 },
}

impl RoundedProof {
    pub(super) fn new() -> Self {
        Self { planes: [[f64::INFINITY,f64::NEG_INFINITY];3], plane_counts: [0;6], radius: None,
            curved: false, face_mask: 0, edges: [0;48], vertices: [0;24] }
    }

    pub(super) fn observe(&mut self, decoder: &StepSolidDecode, source: &str, id: u64) -> Result<(), ValueError> {
        let bound = decoder.bound(source,id)?;
        match support(decoder,source,&bound)? {
            Support::Plane { origin, normal } => {
                let normal = normal.to_array();
                let axis = only_axis(normal).ok_or_else(required)?;
                let high = usize::from(normal[axis] > 0.);
                let coordinate = origin.to_array()[axis];
                let limit = &mut self.planes[axis][high];
                *limit = if high==0 { limit.min(coordinate) } else { limit.max(coordinate) };
                self.plane_counts[2*axis+high] += 1;
            }
            Support::Cylinder { .. } => self.curved = true,
            Support::Sphere { radius,.. } => {
                self.curved = true;
                if self.radius.is_none() { self.radius = Some(radius); }
            }
        }
        Ok(())
    }

    pub(super) fn recipe(&self) -> Result<Option<StepSolid>,ValueError> {
        if !self.curved { return Ok(None); }
        if self.plane_counts != [1;6] { return Err(required()); }
        let origin = self.planes.map(|[low,_]|low);
        let size = self.planes.map(|[low,high]|high-low);
        let radius = self.radius.ok_or_else(required)?;
        let result = StepSolid::RoundedBox { origin,size,radius };
        result.rounded_box().ok_or_else(required)?;
        Ok(Some(result))
    }

    pub(super) fn verify(&mut self, decoder: &StepSolidDecode, source: &str, id: u64, shape: RoundedBox) -> Result<(),ValueError> {
        let bound = decoder.bound(source,id)?;
        let support = support(decoder,source,&bound)?;
        let mut authored = [None;4];
        for (slot,&member) in authored.iter_mut().zip(&bound.members[..bound.count]) {
            let use_ = decoder.coedge(source,member)?;
            let forward = use_.forward == bound.forward;
            let [a,b] = if forward { [use_.from,use_.to] } else { [use_.to,use_.from] };
            *slot = Some((use_,[a,b],decoder.vertex(source,a)?));
        }
        let ring = &mut authored[..bound.count];
        if !bound.forward { ring.reverse(); }
        let fit = (0..FACES).filter(|&index| self.face_mask & (1<<index)==0 && support.matches(shape,index))
            .find_map(|index| {
                let expected = face(index);
                if expected.count != ring.len() { return None; }
                (0..expected.count).find(|&start| ring.iter().enumerate().all(|(i,use_)|
                    near(shape.vertex(expected.vertices[(i+start)%expected.count]), Pnt3::from_array(use_.unwrap().2))))
                    .map(|start| (index,start))
            }).ok_or_else(required)?;
        let (index,start) = fit;
        let expected = face(index);
        for (i,use_) in ring.iter().enumerate() {
            let (authored,ids,_) = use_.unwrap();
            let member = (i+start)%expected.count;
            let next = ring[(i+1)%ring.len()].unwrap().1[0];
            if ids[1]!=next { return Err(required()); }
            bind(&mut self.vertices,expected.vertices[member],ids[0])?;
            let edge = TOPOLOGY.members[index][member];
            bind(&mut self.edges,edge,authored.edge)?;
            verify_curve(decoder,source,shape,edge,authored)?;
        }
        self.face_mask |= 1<<index;
        Ok(())
    }

    pub(super) fn complete(&self) -> Result<(),ValueError> {
        if self.face_mask!=COMPLETE || self.edges.contains(&0) || self.vertices.contains(&0) { return Err(required()); }
        Ok(())
    }
}

impl Support {
    fn matches(&self, shape: RoundedBox, index: usize) -> bool {
        let frame = shape.face_frame(index);
        match *self {
            Self::Plane { origin,normal } => index < 6 && (normal-frame.z).norm() <= TOLERANCE
                && (origin-frame.origin).dot(frame.z).abs() <= TOLERANCE,
            Self::Cylinder { center,axis,radius } => (6..18).contains(&index)
                && (radius-shape.radius()).abs() <= TOLERANCE && axis.cross(frame.z).norm() <= TOLERANCE
                && (center-frame.origin).cross(frame.z).norm() <= TOLERANCE,
            Self::Sphere { center,radius } => index >= 18 && (radius-shape.radius()).abs() <= TOLERANCE
                && near(center,frame.origin),
        }
    }
}

fn support(decoder: &StepSolidDecode, source: &str, bound: &Bound) -> Result<Support,ValueError> {
    let placement = |id| decoder.placement(source,reference(id)?);
    let id = bound.surface;
    if let Ok(attrs) = decoder.row(source,id,"PLANE") {
        let [_,axis] = fields(attrs)?;
        let chart = placement(axis)?;
        let normal = normalized(chart.axis)?;
        return Ok(Support::Plane { origin: Pnt3::from_array(chart.origin), normal: if bound.same {normal} else {-normal} });
    }
    if !bound.same { return Err(required()); }
    for (kind,is_sphere) in [("SPHERICAL_SURFACE",true),("CYLINDRICAL_SURFACE",false)] {
        if let Ok(attrs) = decoder.row(source,id,kind) {
            let [_,axis,radius] = fields(attrs)?;
            let chart = placement(axis)?;
            let center = Pnt3::from_array(chart.origin);
            let radius = number(radius)?;
            return Ok(if is_sphere { Support::Sphere { center,radius } }
                else { Support::Cylinder { center, axis: normalized(chart.axis)?, radius } });
        }
    }
    Err(required())
}

fn verify_curve(decoder: &StepSolidDecode, source: &str, shape: RoundedBox, edge: usize, usage: AuthoredCoedge) -> Result<(),ValueError> {
    let [a,b] = TOPOLOGY.edges[edge];
    let from = Pnt3::from_array(decoder.vertex(source,usage.from)?);
    let to = Pnt3::from_array(decoder.vertex(source,usage.to)?);
    let increasing = near(from,shape.vertex(a)) && near(to,shape.vertex(b));
    if !increasing && !(near(to,shape.vertex(a)) && near(from,shape.vertex(b))) { return Err(required()); }
    if !shape.edge_is_arc(edge) {
        decoder.line_edge(source,usage,usage.curve)?;
        return Ok(());
    }
    let [_,axis,radius] = fields(decoder.row(source,usage.curve,"CIRCLE")?)?;
    let chart = decoder.placement(source,reference(axis)?)?;
    let (_,Curve3::Circle { frame,radius: expected },_) = shape.edge(edge) else { return Err(required()); };
    let axis = normalized(chart.axis)?;
    let expected_normal = if usage.same == increasing { frame.z } else { -frame.z };
    if (expected-number(radius)?).abs() > TOLERANCE || !near(Pnt3::from_array(chart.origin),frame.origin)
        || (axis-expected_normal).norm() > TOLERANCE { return Err(required()); }
    Ok(())
}

fn bind<const N:usize>(mapping: &mut [u64;N], slot: usize, id: u64) -> Result<(),ValueError> {
    if mapping[slot] == id { return Ok(()); }
    if id==0 || mapping[slot]!=0 || mapping.contains(&id) { return Err(required()); }
    mapping[slot] = id;
    Ok(())
}

fn near(a: Pnt3, b: Pnt3) -> bool { (a-b).norm() <= TOLERANCE }
fn normalized(direction: [f64;3]) -> Result<Vec3,ValueError> { Vec3::from_array(direction).normalized().ok_or_else(required) }
fn required() -> ValueError { unsupported("STEP rounded box requires complete planes, quarter cylinders, spherical octants and matching trims") }

pub(super) fn recipe(origin: [f64;3], size: [f64;3], radius: f64) -> Option<RoundedBox> {
    RoundedBox::new(Pnt3::from_array(origin),size,radius,Tol::DEFAULT.value())
}
