//! 🍩️ Samples the preserved torus domain and the two meridian caps.

use super::{TAU,TorusSector,Vertex};

pub(super) fn slots(face:usize,columns:usize,rows:usize)->usize{
    if face==0{2*columns*rows}else{rows}
}

pub(super) fn triangle(shape:TorusSector,face:usize,slot:usize,columns:usize,rows:usize)->[Vertex;3]{
    let u=|i:usize|shape.sweep*(i as f64/columns as f64);
    let v=|j:usize|TAU*(j%rows) as f64/rows as f64;
    if face!=0{
        let index=face-1;
        let rim=shape.rim(index);
        let normal=if index==0{rim.z}else{-rim.z};
        let angle=if index==0{0.}else{shape.sweep};
        return [rim.origin,shape.point(angle,v(slot)),shape.point(angle,v(slot+1))].map(|point|Vertex{point,normal});
    }
    let i=slot/2/rows;
    let j=slot/2%rows;
    let point=|i,j|{
        let (u,v)=(u(i),v(j));
        Vertex{point:shape.point(u,v),normal:shape.normal(u,v)}
    };
    let a=point(i,j);
    let c=point(i+1,j+1);
    if slot%2==0{[a,point(i+1,j),c]}else{[a,c,point(i,j+1)]}
}
