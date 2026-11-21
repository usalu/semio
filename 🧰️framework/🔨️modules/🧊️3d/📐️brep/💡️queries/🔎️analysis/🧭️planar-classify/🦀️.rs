//! 🧭️ Classify an original closed planar shell against its authored line trims.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{
    arena::{EdgeId, SolidId},
    contract::PointClassification,
    topology::Body,
};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

const MAX_EDGES: usize = 64;
const MAX_RING: usize = 32;

fn face_contribution(ring: &[Pnt3], normal: Vec3, point: Pnt3, tolerance: f64) -> Option<(bool, f64)> {
    let [first, ..] = ring else { return None; };
    if ring.len() < 3 { return None; }
    let area = ring[1..].windows(2).map(|pair| (pair[0] - *first).cross(pair[1] - *first).dot(normal)).sum::<f64>();
    if area == 0.0 || !area.is_finite() { return None; }
    let orientation = area.signum();
    let height = (point - *first).dot(normal);
    let mut boundary = false;
    let mut over_face = true;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let c = ring[(index + 2) % ring.len()];
        let edge = b - a;
        let next = c - b;
        let length = edge.norm();
        let turn = edge.cross(next).dot(normal) * orientation;
        if !length.is_finite() || length == 0.0 || turn < -64.0 * f64::EPSILON * length * next.norm() { return None; }
        if ((*first - a).dot(normal)).abs() > tolerance { return None; }
        let direction = edge * (1.0 / length);
        let projection = (point - a).dot(direction).clamp(0.0, length);
        boundary |= (point - (a + direction * projection)).norm() <= tolerance;
        over_face &= direction.cross(point - a).dot(normal) * orientation >= 0.0;
    }
    if boundary || over_face && height.abs() <= tolerance { return Some((true, 0.0)); }
    let rays = |vertex: Pnt3| {
        let ray = vertex - point;
        let length = ray.x.hypot(ray.y).hypot(ray.z);
        (length > 0.0 && length.is_finite()).then(|| ray * (1.0 / length))
    };
    let a = rays(*first)?;
    let mut previous = rays(ring[1])?;
    let mut angle = 0.0;
    for pair in ring[1..].windows(2) {
        let c = rays(pair[1])?;
        if (pair[0] - *first).cross(pair[1] - *first).dot(normal) * orientation > 0.0 {
            let b = previous;
            let determinant = a.dot(b.cross(c));
            let denominator = 1.0 + a.dot(b) + b.dot(c) + c.dot(a);
            angle += 2.0 * determinant.atan2(denominator);
        }
        previous = c;
    }
    angle.is_finite().then_some((false, orientation * angle))
}

/// 🎟️ Quotes the original arena paths the bounded planar classifier may read.
pub fn planar_classification_depth(body: &Body, solid: SolidId) -> Result<usize, semio_framework_value::ValueError> {
    let mut depth = body.solids.next_remove_depth_demand(solid)?;
    let Some(solid) = body.solids.get(solid) else { return Ok(depth); };
    if solid.inners.len() > 4 { return Ok(depth); }
    for &id in std::iter::once(&solid.outer).chain(&solid.inners) {
        depth = depth.max(body.shells.next_remove_depth_demand(id)?);
        let Some(shell) = body.shells.get(id) else { return Ok(depth); };
        if !(4..=MAX_EDGES).contains(&shell.faces.len()) { return Ok(depth); }
        for &id in &shell.faces {
            depth = depth.max(body.faces.next_remove_depth_demand(id)?);
            let Some(face) = body.faces.get(id) else { return Ok(depth); };
            depth = depth.max(body.surfaces.next_remove_depth_demand(face.surface)?);
            let Some(loop_id) = face.outer.filter(|_| face.inners.is_empty()) else { return Ok(depth); };
            depth = depth.max(body.loops.next_remove_depth_demand(loop_id)?);
            let Some(ring) = body.loops.get(loop_id) else { return Ok(depth); };
            let mut index = ring.first;
            for _ in 0..MAX_RING {
                depth = depth.max(body.coedges.next_remove_depth_demand(index)?);
                let Some(coedge) = body.coedges.get(index) else { return Ok(depth); };
                depth = depth.max(body.edges.next_remove_depth_demand(coedge.edge)?);
                let Some(edge) = body.edges.get(coedge.edge) else { return Ok(depth); };
                depth = depth.max(body.curves3.next_remove_depth_demand(edge.curve)?);
                depth = depth.max(body.vertices.next_remove_depth_demand(if coedge.forward { edge.v0 } else { edge.v1 })?);
                index = coedge.next;
                if index == ring.first { break; }
            }
        }
    }
    Ok(depth)
}

/// 🧮️ Classifies a bounded closed planar solid, including one inward cavity shell.
pub fn classify_planar_solid(body: &Body, solid: SolidId, point: Pnt3, tolerance: f64) -> Option<PointClassification> {
    let solid = body.solids.get(solid)?;
    if solid.inners.len() > 1 { return None; }
    let mut angle = 0.0;
    let mut boundary = false;
    for id in std::iter::once(&solid.outer).chain(&solid.inners) {
        let shell = body.shells.get(*id)?;
        if !(4..=MAX_EDGES).contains(&shell.faces.len()) { return None; }
        let mut edges = [None::<(EdgeId, bool, bool)>; MAX_EDGES];
        let mut edge_count = 0;
        for &id in &shell.faces {
            let face = body.faces.get(id)?;
            let Surface::Plane { frame } = body.surfaces.get(face.surface)? else { return None; };
            if !face.inners.is_empty() { return None; }
            let normal = frame.x.cross(frame.y).normalized()?;
            let normal = if face.flipped { -normal } else { normal };
            let loop_id = face.outer?;
            let ring = body.loops.get(loop_id)?;
            if ring.face != id { return None; }
            let mut index = ring.first;
            let mut points = [Pnt3::default(); MAX_RING];
            let mut start = None;
            let mut previous = None;
            let mut count = 0;
            loop {
                if count == MAX_RING { return None; }
                let coedge = body.coedges.get(index)?;
                let edge = body.edges.get(coedge.edge)?;
                if coedge.loop_id != loop_id || !matches!(body.curves3.get(edge.curve)?, Curve3::Line { .. }) { return None; }
                let (from, to) = if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) };
                if previous.is_some_and(|end| end != from) { return None; }
                start.get_or_insert(from);
                previous = Some(to);
                let position = edges[..edge_count].iter().position(|entry| entry.is_some_and(|e| e.0 == coedge.edge));
                if let Some(position) = position {
                    let (_, forward, paired) = edges[position].as_mut()?;
                    if *paired || *forward == coedge.forward { return None; }
                    *paired = true;
                } else {
                    *edges.get_mut(edge_count)? = Some((coedge.edge, coedge.forward, false));
                    edge_count += 1;
                }
                *points.get_mut(count)? = body.vertices.get(from)?.position;
                count += 1;
                index = coedge.next;
                if index == ring.first { break; }
            }
            if start != previous { return None; }
            let (touches, winding) = face_contribution(&points[..count], normal, point, tolerance)?;
            boundary |= touches;
            angle += winding;
        }
        if edges[..edge_count].iter().any(|entry| !entry.is_some_and(|edge| edge.2)) { return None; }
    }
    Some(if boundary { PointClassification::OnBoundary } else if angle.abs() > std::f64::consts::TAU {
        PointClassification::Inside
    } else {
        PointClassification::Outside
    })
}


/// 🎟️ Quotes every original member of a bounded planar compound.
pub fn planar_compound_classification_depth(body:&Body,solids:&[SolidId])->Option<Result<usize,semio_framework_value::ValueError>>{
    (1..=4).contains(&solids.len()).then(||solids.iter().try_fold(1,|depth,&solid|{
        Ok(depth.max(planar_classification_depth(body,solid)?))
    }))
}

/// 🧺️ Resolves material membership across the independent closed shells of a planar compound.
pub fn classify_planar_compound(body:&Body,solids:&[SolidId],point:Pnt3,tolerance:f64)->Option<PointClassification>{
    if !(1..=4).contains(&solids.len()){return None;}
    solids.iter().try_fold(PointClassification::Outside,|result,&solid|{
        let member=classify_planar_solid(body,solid,point,tolerance)?;
        Some(match (result,member){
            (PointClassification::Inside,_)|(_,PointClassification::Inside)=>PointClassification::Inside,
            (PointClassification::OnBoundary,_)|(_,PointClassification::OnBoundary)=>PointClassification::OnBoundary,
            _=>PointClassification::Outside,
        })
    })
}
