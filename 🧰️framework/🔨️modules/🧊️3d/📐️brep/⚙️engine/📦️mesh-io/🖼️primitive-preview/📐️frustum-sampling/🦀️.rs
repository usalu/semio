//! 📐️ Uses the native circle sections for the funded frustum preview.

use super::{ConicalFrustum,TAU,Vertex};

pub(super) fn slots(face:usize,segments:usize)->usize{if face==0{2*segments}else{segments}}

pub(super) fn triangle(shape:ConicalFrustum,face:usize,slot:usize,segments:usize)->[Vertex;3]{
    let angle=|column:usize|TAU*(column%segments) as f64/segments as f64;
    let radial=|column|shape.frame.x*angle(column).cos()+shape.frame.y*angle(column).sin();
    let rim=|index,column|shape.rim(index).origin+radial(column)*shape.radii[index];
    if face!=0 {
        let index=face-1;
        let normal=if index==0{-shape.frame.z}else{shape.frame.z};
        return [shape.rim(index).origin,rim(index,slot),rim(index,slot+1)].map(|point|Vertex{point,normal});
    }
    let column=slot/2;
    let vertex=|index,column|Vertex{
        point:rim(index,column),
        normal:radial(column)*shape.half_angle.cos()-shape.frame.z*shape.half_angle.sin(),
    };
    let a=vertex(0,column);
    let c=vertex(1,column+1);
    if slot%2==0{[a,vertex(0,column+1),c]}else{[a,c,vertex(1,column)]}
}
