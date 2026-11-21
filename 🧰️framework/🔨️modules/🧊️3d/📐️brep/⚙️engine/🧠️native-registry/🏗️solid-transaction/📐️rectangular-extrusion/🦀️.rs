//! 📐️ Recognizes an exact rectangular extrusion as a certified primitive.

use super::{BoreBoundary, circular_opening_offset::{CircularOpeningOffset, opening_member}};
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::FaceId,topology::{Body,Wire}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3};
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 📋️ Borrows one possible original rectangular boundary.
pub enum RectangleProfile<'a> { Wire(&'a Wire), Face(FaceId) }

/// 📦️ Recognizes one horizontal rectangular line ring without copying topology.
pub fn rectangular_extrusion(body: &Body, source: RectangleProfile<'_>, vector: [f64;3]) -> Result<Option<SolidPrimitive>, ValueError> {
    rectangular_stock(body,source,vector,false)
}

fn rectangular_stock(body:&Body,source:RectangleProfile<'_>,vector:[f64;3],opening:bool)->Result<Option<SolidPrimitive>,ValueError>{
    let invalid = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original polygon extrusion requires a rectangular profile along the world Z axis");
    let mut corners = [Pnt3::new(0., 0., 0.); 4];
    let get_point = |edge_id, forward| -> Result<Pnt3, ValueError> {
        let edge = body.edges.get(edge_id).ok_or_else(invalid)?;
        if !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Err(invalid()); }
        body.vertices.get(if forward { edge.v0 } else { edge.v1 }).map(|vertex| vertex.position).ok_or_else(invalid)
    };
    match source {
        RectangleProfile::Wire(wire) if wire.closed && wire.members.len() == 4 => {
            for (point, &(edge, forward)) in corners.iter_mut().zip(&wire.members) {
                *point = get_point(edge, forward)?;
            }
        }
        RectangleProfile::Face(face_id) => {
            let face = body.faces.get(face_id).ok_or_else(invalid)?;
            if face.inners.len()!=usize::from(opening) { return Ok(None); }
            let Some(Surface::Plane { frame })=body.surfaces.get(face.surface)else{return Ok(None);};
            let ring_id=face.outer.ok_or_else(invalid)?;
            let ring=body.loops.get(ring_id).filter(|ring|ring.face==face_id).ok_or_else(invalid)?;
            let mut use_id=ring.first;
            let mut first_vertex=None;
            let mut previous_end=None;
            for point in &mut corners {
                let use_=body.coedges.get(use_id).filter(|use_|use_.loop_id==ring_id).ok_or_else(invalid)?;
                let edge=body.edges.get(use_.edge).ok_or_else(invalid)?;
                let [start,end]=if use_.forward {[edge.v0,edge.v1]}else{[edge.v1,edge.v0]};
                first_vertex.get_or_insert(start);
                if let Some(previous)=previous_end {
                    if previous!=start{return Ok(None);}
                }
                previous_end=Some(end);
                *point=get_point(use_.edge,use_.forward)?;
                if opening && point.z!=frame.origin.z{return Ok(None);}
                use_id=use_.next;
            }
            if use_id!=ring.first || previous_end!=first_vertex{return Ok(None);}
        }
        _ => return Ok(None),
    }
    let [x0, y0, z] = corners[0].to_array();
    let mut bounds = [x0, y0, x0, y0];
    for i in 0..4 {
        let p = corners[i];
        let q = corners[(i + 1) % 4];
        if p.z != z || (p.x == q.x) == (p.y == q.y) { return Ok(None); }
        bounds = [bounds[0].min(p.x), bounds[1].min(p.y), bounds[2].max(p.x), bounds[3].max(p.y)];
    }
    let [min_x, min_y, max_x, max_y] = bounds;
    let mut visited = 0u8;
    for p in corners {
        if (p.x != min_x && p.x != max_x) || (p.y != min_y && p.y != max_y) { return Ok(None); }
        visited |= 1 << (u8::from(p.x == max_x) + 2 * u8::from(p.y == max_y));
    }
    if visited != 0b1111 { return Ok(None); }
    let [dx, dy, dz] = vector;
    if dx != 0. || dy != 0. || !dz.is_finite() || dz.abs() <= Tol::DEFAULT.value() { return Ok(None); }
    let origin = Pnt3::new(min_x, min_y, z + dz.min(0.));
    let primitive = SolidPrimitive::Box { origin, width: max_x - min_x, depth: max_y - min_y, height: dz.abs() };
    Ok(Some(primitive))
}

/// 🕳️ Builds an exact bored solid from a certified horizontal rectangular face.
pub fn bored_rectangular_extrusion(body:&Body,face:FaceId,vector:[f64;3])->Result<BoreBoundary,ValueError>{
    let unsupported=||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Thicken requires a horizontal rectangular cap with one complete interior circular opening");
    let stock=rectangular_stock(body,RectangleProfile::Face(face),vector,true)?.ok_or_else(unsupported)?;
    let record=body.faces.get(face).ok_or_else(unsupported)?;
    let [ring]=record.inners.as_slice() else{return Err(unsupported());};
    let (_,use_,edge)=opening_member(body,face,*ring).ok_or_else(unsupported)?;
    use_.pcurve.and_then(|id|body.curves2.get(id)).ok_or_else(unsupported)?;
    let Some(Surface::Plane{frame:plane})=body.surfaces.get(record.surface)else{return Err(unsupported());};
    let Some(Curve3::Circle{frame,radius})=body.curves3.get(edge.curve)else{return Err(unsupported());};
    if !CircularOpeningOffset::orientation(plane) || !CircularOpeningOffset::orientation(frame)
        || (plane.z!=Vec3::Z && plane.z!=-Vec3::Z) || (frame.z!=Vec3::Z && frame.z!=-Vec3::Z)
        || frame.origin.z!=plane.origin.z || (plane.z==frame.z)==use_.forward
    {return Err(unsupported());}
    let endpoint=body.vertices.get(edge.v0).ok_or_else(unsupported)?.position;
    if !((endpoint-(frame.origin+frame.x*(*radius))).norm()<=Tol::DEFAULT.value()){return Err(unsupported());}
    BoreBoundary::from_profile(stock,[frame.origin.x,frame.origin.y],*radius)
}

/// 🎟️ Quotes the arena paths used by the fixed-size rectangular cap certificate.
pub fn rectangular_face_depth(body:&Body,face:FaceId)->Result<usize,ValueError>{
    let mut depth=1;
    let Some(face_record)=body.faces.get(face)else{return Ok(depth);};
    let mut quote=|ring_id,count|{
        depth=depth.max(body.loops.next_remove_depth_demand(ring_id)?);
        let Some(ring)=body.loops.get(ring_id)else{return Ok(());};
        let mut id=ring.first;
        for _ in 0..count {
            depth=depth.max(body.coedges.next_remove_depth_demand(id)?);
            let Some(use_)=body.coedges.get(id)else{break;};
            depth=depth.max(body.edges.next_remove_depth_demand(use_.edge)?);
            if let Some(trim)=use_.pcurve{depth=depth.max(body.curves2.next_remove_depth_demand(trim)?);}
            let Some(edge)=body.edges.get(use_.edge)else{break;};
            depth=depth.max(body.curves3.next_remove_depth_demand(edge.curve)?)
                .max(body.vertices.next_remove_depth_demand(edge.v0)?).max(body.vertices.next_remove_depth_demand(edge.v1)?);
            id=use_.next;
            if id==ring.first{break;}
        }
        Ok::<(),ValueError>(())
    };
    if let Some(outer)=face_record.outer{quote(outer,4)?;}
    if let [inner]=face_record.inners.as_slice(){quote(*inner,4)?;}
    Ok(depth)
}
