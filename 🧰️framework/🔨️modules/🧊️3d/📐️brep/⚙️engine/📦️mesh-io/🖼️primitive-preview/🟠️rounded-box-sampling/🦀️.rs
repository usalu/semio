//! 🟠️ Matched edge and face tessellation of certified rounded stock.

use super::{Pnt3, RoundedBox, Vertex, rounded_box_quadrant as quadrant};
use semio_framework_3d_brep_representation::contract::box_finish_layout::face;
use semio_framework_3d_brep_curve::Curve3;

pub(super) fn slots(face: usize, segments: usize) -> usize {
    match face {
        0..6 => 2,
        6..18 => 2*segments,
        _ => segments*(2*segments-1),
    }
}

pub(super) fn edge_point(shape: RoundedBox, index: usize, step: usize, segments: usize) -> Pnt3 {
    let (ids,curve,_) = shape.edge(index);
    if step == 0 { return shape.vertex(ids[0]); }
    if step == segments { return shape.vertex(ids[1]); }
    let Curve3::Circle { frame, radius } = curve else { unreachable!("subdivided rounded-box edge"); };
    let [c,s] = quadrant(step,segments);
    frame.origin + (frame.x*c+frame.y*s)*radius
}

pub(super) fn triangle(shape: RoundedBox, index: usize, slot: usize, segments: usize) -> [Vertex;3] {
    let frame = shape.face_frame(index);
    if index < 6 {
        let ring = face(index);
        let members = if slot==0 { [0,1,2] } else { [0,2,3] };
        return members.map(|i| Vertex { point: shape.vertex(ring.vertices[i]), normal: frame.z });
    }
    let cylinder = index < 18;
    let corner_row = (segments-1)*2*segments;
    let (u,v,first) = if !cylinder && slot>=corner_row {
        (slot-corner_row,segments-1,true)
    } else {
        ((slot/2)%segments,slot/(2*segments),slot%2==0)
    };
    let uv = if first { [(u,v),(u+1,v),(u+1,v+1)] } else { [(u,v),(u+1,v+1),(u,v+1)] };
    uv.map(|(u,v)| {
        let [c,s] = quadrant(u,segments);
        let (origin,normal) = if cylinder {
            (if v==0 { frame.origin } else { shape.cylinder_end(index) },frame.x*c+frame.y*s)
        } else {
            let [d,h] = quadrant(v,segments);
            (frame.origin,frame.x*(c*d)+frame.y*(s*d)+frame.z*h)
        };
        Vertex { point: origin+normal*shape.radius(), normal }
    })
}
