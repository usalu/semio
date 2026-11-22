//! 🕳️ Identifies circular bores and their rectangular stock from native topology.

use super::*;
use semio_framework_3d_brep_construction::primitives::BoxGeometry;
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_representation::arena::{EdgeId, LoopId, VertexId};
use semio_framework_3d_brep_vector::{Pnt2, Vec3, matrix::Frame3};
use std::f64::consts::TAU;

const STOCK_FACES: usize = 6;
const MAX_BORES: usize = 2;
const ANGLE: f64 = 1e-8;
const LINEAR: f64 = semio_framework_3d_brep_tolerance::Resolution::DEFAULT.linear;

#[derive(Clone, Copy)]
struct Opening {
    edge: EdgeId,
    forward: bool,
    seam_vertex: VertexId,
}

#[derive(Clone, Copy)]
struct Wall<'a> {
    id: FaceId,
    face: &'a Face,
    surface: &'a Surface,
    center: Pnt3,
    radius: f64,
}

impl Wall<'_> {
    fn matches(&self, center: Pnt3, radius: f64) -> bool {
        close(self.radius,radius,LINEAR) && close(self.center.x,center.x,LINEAR) && close(self.center.y,center.y,LINEAR)
    }
}

/// 🕳️ Restores rectangular stock after proving all selected bore walls and any interior floor.
pub fn certify_bore_stock(body: &Body, shell: &Shell, selected: &[FaceId]) -> Option<SolidPrimitive> {
    if !(1..=MAX_BORES).contains(&selected.len()) || shell.faces.len() != STOCK_FACES + selected.len() {
        return None;
    }
    let mut walls = [None; MAX_BORES];
    let mut wall_count = 0;
    let mut floor = None;
    for (index, &id) in selected.iter().enumerate() {
        if !shell.faces.contains(&id) || selected[..index].contains(&id) { return None; }
        let face = body.faces.get(id)?;
        if !face.inners.is_empty() { return None; }
        let surface = body.surfaces.get(face.surface)?;
        match surface {
            Surface::Cylinder { frame, radius } if face.flipped && rigid(*frame) && close(frame.z.z.abs(), 1.0, ANGLE) => {
                walls[wall_count] = Some(Wall { id, face, surface, center: frame.origin, radius: *radius });
                wall_count += 1;
            }
            Surface::Plane { frame } if floor.is_none() && rigid(*frame) => floor = Some((id, face, surface, *frame)),
            _ => return None,
        }
    }
    if wall_count == 0 { return None; }
    if let [Some(a), Some(b)] = walls {
        let distance = (a.center.x-b.center.x).hypot(a.center.y-b.center.y);
        if !distance.is_finite() || distance <= a.radius+b.radius+LINEAR { return None; }
    }
    let mut planes = [None; STOCK_FACES];
    for &id in &shell.faces {
        if selected.contains(&id) { continue; }
        let face = body.faces.get(id)?;
        let plane @ Surface::Plane { frame } = body.surfaces.get(face.surface)? else { return None; };
        if !rigid(*frame) { return None; }
        let normal = (if face.flipped { -frame.z } else { frame.z }).to_array();
        let axes = [Vec3::X, Vec3::Y, Vec3::Z];
        let axis = normal.iter().position(|value| close(value.abs(), 1.0, ANGLE))?;
        let positive = normal[axis] > 0.0;
        if !same_vector(Vec3::from_array(normal), if positive { axes[axis] } else { -axes[axis] }) {
            return None;
        }
        if planes[axis * 2 + usize::from(positive)].replace((id, face, plane, frame)).is_some() {
            return None;
        }
    }
    let [x0,x1,y0,y1,z0,z1] = planes.map(|plane| plane.map(|(_, _, _, f)| f.origin));
    let origin = Pnt3::new(x0?.x, y0?.y, z0?.z);
    let maximum = Pnt3::new(x1?.x, y1?.y, z1?.z);
    let stock = SolidPrimitive::Box {
        origin, width: maximum.x-origin.x, depth: maximum.y-origin.y, height: maximum.z-origin.z,
    };
    for wall in walls.iter().flatten() {
        BoreBoundary::from_profile(stock, [wall.center.x, wall.center.y], wall.radius).ok()?;
    }
    let mut edges = [None; 12];
    let mut vertices = [None; 8];
    let points: [Pnt3; 8] = std::array::from_fn(|index| stock.vertex(index));
    let mut openings = [[None; 2]; MAX_BORES];
    for (index, entry) in planes.into_iter().enumerate() {
        let (id, face, plane, frame) = entry?;
        let mut area = Vec3::new(0.0, 0.0, 0.0);
        for coedge in ring::<4>(body, id, face.outer?)? {
            let edge = body.edges.get(coedge.edge)?;
            let curve = body.curves3.get(edge.curve)?;
            if !matches!(curve, Curve3::Line { .. }) || !trim_matches(body, plane, coedge, edge, curve, false) {
                return None;
            }
            let positions = [body.vertices.get(edge.v0)?.position, body.vertices.get(edge.v1)?.position];
            let ends = positions.map(|p| points.iter().position(|corner| same_point(p, *corner)));
            let ends = [ends[0]?, ends[1]?];
            let ordinal = BoxGeometry::EDGES.iter().position(|&candidate| {
                candidate == ends || candidate == [ends[1], ends[0]]
            })?;
            let axis = index / 2;
            if positions.iter().any(|p| !close(p.to_array()[axis], frame.origin.to_array()[axis], LINEAR)) {
                return None;
            }
            bind(&mut vertices, ends[0], edge.v0)?;
            bind(&mut vertices, ends[1], edge.v1)?;
            let forward = coedge.forward != face.flipped;
            match &mut edges[ordinal] {
                entry @ None => *entry = Some((coedge.edge, Some(forward))),
                Some((id, prior)) if *id == coedge.edge && *prior == Some(!forward) => *prior = None,
                _ => return None,
            }
            let cross = (positions[0]-frame.origin).cross(positions[1]-frame.origin);
            area = area + if coedge.forward { cross } else { -cross };
        }
        let signed_area = area.dot(frame.z);
        if !signed_area.is_finite() || signed_area <= 0.0 { return None; }
        if index < 4 {
            if !face.inners.is_empty() { return None; }
            continue;
        }
        let hole_count = face.inners.len();
        if hole_count > wall_count || (floor.is_none() && hole_count != wall_count) { return None; }
        for &hole in &face.inners {
            let (opening, center, radius) = round_rim(body, id, plane, hole, true)?;
            let wall = walls.iter().position(|wall| wall.is_some_and(|wall| wall.matches(center,radius)))?;
            if openings[wall][index-4].replace(opening).is_some() { return None; }
        }
    }
    if edges.iter().any(|entry| !matches!(entry, Some((_, None)))) { return None; }
    if let Some((id, face, plane, frame)) = floor {
        let missing = match openings[0] { [None, Some(_)] => 0, [Some(_), None] => 1, _ => return None };
        let outward = if face.flipped { -frame.z } else { frame.z };
        let into_bore = if missing == 0 { Vec3::Z } else { -Vec3::Z };
        let height = frame.origin.z;
        if !same_vector(outward, into_bore) || height-origin.z <= LINEAR || maximum.z-height <= LINEAR {
            return None;
        }
        let (opening, center, radius) = round_rim(body, id, plane, face.outer?, false)?;
        if !walls[0]?.matches(center,radius) { return None; }
        openings[0][missing] = Some(opening);
    }
    for (wall, [bottom, top]) in walls.into_iter().zip(openings).take(wall_count) {
        let wall = wall?;
        certify_wall(body, wall.id, wall.face, wall.surface, [bottom?, top?])?;
    }
    Some(stock)
}

fn round_rim(body: &Body, owner: FaceId, plane: &Surface, loop_id: LoopId, inner: bool) -> Option<(Opening,Pnt3,f64)> {
    let Surface::Plane { frame } = plane else { return None; };
    let [coedge] = ring::<1>(body, owner, loop_id)?;
    let edge = body.edges.get(coedge.edge)?;
    let curve @ Curve3::Circle { frame: circle, radius } = body.curves3.get(edge.curve)? else { return None; };
    let aligned = (circle.z.dot(frame.z) > 0.0) == coedge.forward;
    if edge.v0 != edge.v1 || !rigid(*circle) || !close(circle.z.z.abs(), 1.0, ANGLE)
        || !close(circle.origin.z,frame.origin.z,LINEAR) || !close(edge.range.1-edge.range.0,TAU,ANGLE)
        || aligned == inner || !trim_matches(body,plane,coedge,edge,curve,true) {
        return None;
    }
    let flipped = body.faces.get(owner)?.flipped;
    Some((Opening { edge: coedge.edge, forward: coedge.forward != flipped, seam_vertex: edge.v0 }, circle.origin, *radius))
}

fn certify_wall(body: &Body, wall_id: FaceId, wall: &Face, cylinder: &Surface, openings: [Opening; 2]) -> Option<()> {
    if openings[0].edge == openings[1].edge { return None; }
    let mut seen = [false; 2];
    let mut seam = None;
    let mut uv_extent = [f64::INFINITY, f64::NEG_INFINITY];
    let mut first = None;
    let mut previous = None;
    for coedge in ring::<4>(body, wall_id, wall.outer?)? {
        let edge = body.edges.get(coedge.edge)?;
        let curve = body.curves3.get(edge.curve)?;
        if !trim_matches(body, cylinder, coedge, edge, curve, false) { return None; }
        let pcurve = body.curves2.get(coedge.pcurve?)?;
        let from = pcurve.eval(coedge.prange.0);
        let to = pcurve.eval(coedge.prange.1);
        let (start,end) = if coedge.forward { (from,to) } else { (to,from) };
        if previous.is_some_and(|previous| !same_uv(previous,start)) { return None; }
        first.get_or_insert(start);
        previous = Some(end);
        if let Some(end) = openings.iter().position(|opening| opening.edge == coedge.edge) {
            if seen[end] || openings[end].forward == (coedge.forward != wall.flipped)
                || !close((to.x-from.x).abs(),TAU,ANGLE) || !close(from.y,to.y,LINEAR) {
                return None;
            }
            seen[end] = true;
        } else {
            let vertices = openings.map(|opening| opening.seam_vertex);
            if !matches!(curve, Curve3::Line { .. }) || vertices[0] == vertices[1]
                || !((edge.v0 == vertices[0] && edge.v1 == vertices[1]) || (edge.v0 == vertices[1] && edge.v1 == vertices[0])) {
                return None;
            }
            let (a,b) = (body.vertices.get(vertices[0])?.position,body.vertices.get(vertices[1])?.position);
            if !close(from.x, to.x, ANGLE) || !close(a.x,b.x,LINEAR) || !close(a.y,b.y,LINEAR) { return None; }
            uv_extent = [uv_extent[0].min(from.x), uv_extent[1].max(from.x)];
            match seam {
                None => seam = Some((coedge.edge, coedge.forward)),
                Some((prior, forward)) if prior == coedge.edge && forward != coedge.forward => (),
                _ => return None,
            }
        }
    }
    (seen.into_iter().all(|seen| seen) && same_uv(previous?,first?)
        && close(uv_extent[1]-uv_extent[0], TAU, ANGLE)).then_some(())
}

fn same_uv(a: Pnt2, b: Pnt2) -> bool {
    close(a.x,b.x,ANGLE) && close(a.y,b.y,LINEAR)
}

fn rigid(frame: Frame3) -> bool {
    frame.origin.to_array().into_iter().all(f64::is_finite)
        && [frame.x,frame.y,frame.z].into_iter().all(|axis| close(axis.dot(axis),1.0,ANGLE))
        && same_vector(frame.x.cross(frame.y),frame.z)
}

fn same_vector(a: Vec3, b: Vec3) -> bool {
    a.to_array().into_iter().zip(b.to_array()).all(|(a,b)| close(a,b,ANGLE))
}

fn same_point(a: Pnt3, b: Pnt3) -> bool {
    a.distance(b) <= LINEAR
}

fn close(a: f64, b: f64, tolerance: f64) -> bool {
    (a-b).abs() <= tolerance
}

fn bind(slots: &mut [Option<VertexId>], index: usize, vertex: VertexId) -> Option<()> {
    if let Some(existing) = slots[index] { return (existing == vertex).then_some(()); }
    if slots.contains(&Some(vertex)) { return None; }
    slots[index] = Some(vertex);
    Some(())
}

fn ring<const N: usize>(body: &Body, face: FaceId, id: LoopId) -> Option<[&Coedge; N]> {
    let ring = body.loops.get(id)?;
    if ring.face != face { return None; }
    let first = body.coedges.get(ring.first)?;
    let mut output = [first; N];
    let mut cursor = ring.first;
    for (index, slot) in output.iter_mut().enumerate() {
        if index > 0 && cursor == ring.first { return None; }
        let coedge = body.coedges.get(cursor)?;
        let next = body.coedges.get(coedge.next)?;
        if coedge.loop_id != id || next.prev != cursor { return None; }
        let a = body.edges.get(coedge.edge)?;
        let b = body.edges.get(next.edge)?;
        if (if coedge.forward { a.v1 } else { a.v0 }) != (if next.forward { b.v0 } else { b.v1 }) {
            return None;
        }
        *slot = coedge;
        cursor = coedge.next;
    }
    (cursor == ring.first).then_some(output)
}

/// 🧭️ Compares the parameters of affine or full-turn conic boundary representations.
fn trim_matches(body: &Body, surface: &Surface, coedge: &Coedge, edge: &Edge, curve: &Curve3, circular: bool) -> bool {
    let Some(pcurve) = coedge.pcurve.and_then(|id| body.curves2.get(id)) else { return false; };
    let (lo,hi) = edge.range;
    let (u0,u1) = coedge.prange;
    if ![lo,hi,u0,u1].into_iter().all(f64::is_finite) || hi <= lo || u1 <= u0 { return false; }
    match (pcurve, circular) {
        (Curve2::Line { .. }, false) => (),
        (Curve2::Circle { .. } | Curve2::Ellipse { .. }, true) if close(u1-u0,TAU,ANGLE) => (),
        _ => return false,
    }
    [0.0,0.25,0.5,0.75,1.0].into_iter().all(|fraction| {
        let point = curve.eval(lo+(hi-lo)*fraction);
        let uv = pcurve.eval(u0+(u1-u0)*fraction);
        let vertex = |id: VertexId| body.vertices.get(id).is_some_and(|vertex| same_point(point,vertex.position));
        same_point(point,surface.eval(uv.x,uv.y))
            && (fraction != 0.0 || vertex(edge.v0)) && (fraction != 1.0 || vertex(edge.v1))
    })
}


#[cfg(test)]
#[test]
fn certifies_a_cut_solid_and_refuses_foreign_faces_and_changed_stock() {
    use semio_framework_3d_brep_boolean::{BooleanOp, boolean_solid};
    use semio_framework_3d_brep_construction::{primitives::{make_box, make_cylinder}, transform::transform_solid};
    use semio_framework_3d_brep_representation::topology::history::OpRecorder;
    use semio_framework_3d_brep_vector::matrix::Affine3;

    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let stock = make_box(&mut body,4.0,4.0,1.0,&mut recorder).unwrap();
    let tool = make_cylinder(&mut body,0.5,3.0,&mut recorder).unwrap();
    let tool = transform_solid(
        &mut body,tool,&Affine3::translation(Vec3::new(2.0,2.0,-1.0)),&mut recorder,
    ).unwrap();
    let bored = boolean_solid(&mut body,stock,tool,BooleanOp::Cut,1e-6,&mut recorder).unwrap();
    let shell = body.shells.get(body.solids.get(bored).unwrap().outer).unwrap().clone();
    let wall = shell.faces.iter().copied().find(|&id| {
        matches!(body.surfaces.get(body.faces.get(id).unwrap().surface), Some(Surface::Cylinder { .. }))
    }).unwrap();
    let Some(SolidPrimitive::Box { origin, width, depth, height }) = certify_bore_stock(&body,&shell,&[wall]) else {
        panic!("a native through bore must certify its original stock")
    };
    assert_eq!(origin,Pnt3::new(0.0,0.0,0.0));
    assert_eq!([width,depth,height],[4.0,4.0,1.0]);
    let planar = shell.faces.iter().copied().find(|&id| id!=wall).unwrap();
    assert!(certify_bore_stock(&body,&shell,&[planar]).is_none());
    let stock_id = body.solids.get(stock).unwrap().outer;
    let foreign = body.shells.get(stock_id).unwrap().faces[0];
    assert!(certify_bore_stock(&body,&shell,&[foreign]).is_none());
    let face = body.faces.get(planar).unwrap();
    let outer = body.loops.get(face.outer.unwrap()).unwrap();
    let coedge = body.coedges.get(outer.first).unwrap();
    let corner = body.edges.get(coedge.edge).unwrap().v0;
    body.vertices.get_mut(corner).unwrap().position.z += 0.03125;
    assert!(certify_bore_stock(&body,&shell,&[wall]).is_none());
}


#[cfg(test)]
#[test]
fn certifies_two_disjoint_through_bores_with_reordered_openings() {
    use semio_framework_3d_brep_boolean::{BooleanOp, boolean_solid};
    use semio_framework_3d_brep_construction::{primitives::{make_box, make_cylinder}, transform::transform_solid};
    use semio_framework_3d_brep_representation::topology::history::OpRecorder;
    use semio_framework_3d_brep_vector::matrix::Affine3;

    let mut body = Body::new();
    let mut recorder = OpRecorder::new();
    let mut stock = make_box(&mut body,8.0,4.0,1.0,&mut recorder).unwrap();
    for (x,radius) in [(2.0,0.5),(6.0,0.75)] {
        let tool = make_cylinder(&mut body,radius,3.0,&mut recorder).unwrap();
        let tool = transform_solid(
            &mut body,tool,&Affine3::translation(Vec3::new(x,2.0,-1.0)),&mut recorder,
        ).unwrap();
        stock = boolean_solid(&mut body,stock,tool,BooleanOp::Cut,1e-6,&mut recorder).unwrap();
    }
    let shell = body.shells.get(body.solids.get(stock).unwrap().outer).unwrap().clone();
    let [a,b]: [_;2] = shell.faces.iter().copied().filter(|&id| {
        matches!(body.surfaces.get(body.faces.get(id).unwrap().surface), Some(Surface::Cylinder { .. }))
    }).collect::<Vec<_>>().try_into().unwrap();
    let cap = shell.faces.iter().copied().find(|&id| body.faces.get(id).unwrap().inners.len() == 2).unwrap();
    body.faces.get_mut(cap).unwrap().inners.reverse();
    for walls in [[a,b],[b,a]] {
        let Some(SolidPrimitive::Box { origin, width, depth, height }) = certify_bore_stock(&body,&shell,&walls) else {
            panic!("both native through bores must certify their shared stock")
        };
        assert_eq!(origin,Pnt3::new(0.0,0.0,0.0));
        assert_eq!([width,depth,height],[8.0,4.0,1.0]);
    }
    assert!(certify_bore_stock(&body,&shell,&[a]).is_none());
    assert!(certify_bore_stock(&body,&shell,&[a,a]).is_none());
    assert!(certify_bore_stock(&body,&shell,&[a,cap]).is_none());
    let face = body.faces.get_mut(cap).unwrap();
    face.inners[1] = face.inners[0];
    assert!(certify_bore_stock(&body,&shell,&[a,b]).is_none());
}


#[cfg(test)]
#[test]
fn certifies_imported_blind_stock_with_either_open_end() {
    use semio_framework_3d_brep_boolean::{BooleanOp, boolean_solid};
    use semio_framework_3d_brep_construction::{primitives::{make_box, make_cylinder}, transform::transform_solid};
    use semio_framework_3d_brep_representation::topology::history::OpRecorder;
    use semio_framework_3d_brep_vector::matrix::Affine3;

    for (z, length) in [(-1.0,1.5),(0.5,2.0)] {
        let mut body = Body::new();
        let mut recorder = OpRecorder::new();
        let stock = make_box(&mut body,4.0,4.0,1.0,&mut recorder).unwrap();
        let tool = make_cylinder(&mut body,0.5,length,&mut recorder).unwrap();
        let tool = transform_solid(
            &mut body,tool,&Affine3::translation(Vec3::new(2.0,2.0,z)),&mut recorder,
        ).unwrap();
        let bored = boolean_solid(&mut body,stock,tool,BooleanOp::Cut,1e-6,&mut recorder).unwrap();
        let shell = body.shells.get(body.solids.get(bored).unwrap().outer).unwrap();
        let wall = shell.faces.iter().copied().find(|&id| {
            matches!(body.surfaces.get(body.faces.get(id).unwrap().surface), Some(Surface::Cylinder { .. }))
        }).unwrap();
        let floor = shell.faces.iter().copied().find(|&id| body.face_coedges(id).len() == 1).unwrap();
        let Some(SolidPrimitive::Box { width, depth, height, .. }) = certify_bore_stock(&body,shell,&[floor,wall]) else {
            panic!("native blind bore must restore its stock")
        };
        assert_eq!([width,depth,height],[4.0,4.0,1.0]);
        assert!(certify_bore_stock(&body,shell,&[wall]).is_none());
        assert!(certify_bore_stock(&body,shell,&[floor]).is_none());
    }
}
