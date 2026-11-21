//! 📐️ Samples exact circular edges and their adjoining caps using shared native coordinates.

use super::{CircularPrism,Surface,Vertex};

pub(super) fn slots(shape:CircularPrism,face:usize,segments:usize)->usize{
    match face{
        0|1=>(0..shape.count).map(|side|if shape.has_arc(side){segments}else{1}).sum(),
        _ if shape.has_arc(face-2)=>segments*2,
        _=>2,
    }
}

pub(super) fn triangle(shape:CircularPrism,face:usize,slot:usize,segments:usize)->[Vertex;3]{
    let fraction=|i:usize|i as f64/segments as f64;
    let n=shape.count;
    if face<2 {
        let top=face==1;
        let Surface::Plane{frame}=shape.surface(face)else{unreachable!()};
        let mut slot=slot;
        let (side,column)=(0..n).find_map(|side|{
            let count=if shape.has_arc(side){segments}else{1};
            if slot<count{return Some((side,slot));}
            slot-=count;None
        }).expect("bounded cap sample");
        let base=n*usize::from(top);
        let [a,b]=if shape.has_arc(side) {
            [shape.arc_point(side,top,fraction(column)),shape.arc_point(side,top,fraction(column+1))]
        }else{
            [shape.vertex(base+side),shape.vertex(base+(side+1)%n)]
        };
        let center=shape.center+shape.axis*(if top{shape.length}else{0.});
        return [center,a,b].map(|point|Vertex{point,normal:frame.z});
    }
    let i=face-2;
    if shape.has_arc(i) {
        let Surface::Cylinder{frame,..}=shape.surface(face)else{unreachable!()};
        let column=slot/2;
        let [a,b]=[column,column+1];
        let sample=|column,top|{
            let u=shape.arc_parameter(i,fraction(column));
            let radial=frame.x*u.cos()+frame.y*u.sin();
            let normal=if shape.face_flipped(face){-radial}else{radial};
            Vertex{point:shape.arc_point(i,top,fraction(column)),normal}
        };
        return if slot%2==0{[sample(a,false),sample(b,false),sample(b,true)]}
            else{[sample(a,false),sample(b,true),sample(a,true)]};
    }
    let next=(i+1)%n;
    let indices=if slot==0{[i,next,next+n]}else{[i,next+n,i+n]};
    let Surface::Plane{frame}=shape.surface(face)else{unreachable!()};
    indices.map(|vertex|Vertex{point:shape.vertex(vertex),normal:frame.z})
}
