//! 🪷️ Borrows the exact curve and coedge of a complete planar conic face.

use super::{Body,FaceId,Surface};
use semio_framework_3d_brep_curve::{Curve2,Curve3};

/// 🔘️ Certifies one full circular or elliptical loop against its original plane.
pub fn complete_conic_boundary(body:&Body,id:FaceId)->Option<(&Curve3,[f64;3],bool)> {
    let face=body.faces.get(id)?;
    if !face.inners.is_empty() { return None; }
    let Surface::Plane{frame}=body.surfaces.get(face.surface)? else { return None; };
    let loop_id=face.outer?;
    let ring=body.loops.get(loop_id)?;
    let coedge=body.coedges.get(ring.first)?;
    if ring.face!=id || coedge.loop_id!=loop_id || coedge.next!=ring.first || coedge.prev!=ring.first { return None; }
    let range=(0.0,std::f64::consts::TAU);
    let edge=body.edges.get(coedge.edge)?;
    if !coedge.forward || edge.v0!=edge.v1 || edge.range!=range || coedge.prange!=range { return None; }
    let curve=body.curves3.get(edge.curve)?;
    let trim=body.curves2.get(coedge.pcurve?)?;
    match (curve,trim) {
        (Curve3::Circle{frame:source,radius},Curve2::Circle{center,radius:trim_radius})
            if source==frame && trim_radius==radius && center.x==0.0 && center.y==0.0 => {}
        (Curve3::Ellipse{frame:source,major_radius,minor_radius},Curve2::Ellipse{center,x_axis,major_radius:major,minor_radius:minor})
            if source==frame && major_radius==major && minor_radius==minor && center.x==0.0 && center.y==0.0 && x_axis.x==1.0 && x_axis.y==0.0 => {}
        _=>return None,
    }
    let normal=frame.x.cross(frame.y).normalized()?;
    let normal=if face.flipped {-normal} else {normal};
    Some((curve,[normal.x,normal.y,normal.z],face.flipped))
}
