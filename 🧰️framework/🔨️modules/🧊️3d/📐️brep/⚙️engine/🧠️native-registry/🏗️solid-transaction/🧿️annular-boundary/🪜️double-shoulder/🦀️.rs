//! 🪜️ Native boundary of three parallel axial sections with two radial shoulders.

use super::*;

pub(super) const RINGS: &[usize] = &[1,4,1,4,1,4,1,1,1];

pub(super) fn face(ring: usize) -> usize {
    match ring { 7=>2, 8=>4, _=>ring }
}

pub(super) fn surface(shape: AnnularRevolution, face: usize) -> Surface {
    let frame=shape.frame;
    let corner=match face {
        1|5=>return Surface::Cylinder{frame:shape.inner_frame(),radius:shape.inner},
        3=>return Surface::Cylinder{frame,radius:shape.outer},
        0=>0,
        2=>2,
        4=>3,
        _=>5,
    };
    let origin=shape.center(corner);
    let frame=if matches!(face,0|2){Frame3{origin,y:-frame.y,z:-frame.z,..frame}}
        else{Frame3{origin,..frame}};
    Surface::Plane{frame}
}

pub(super) fn trim(shape: AnnularRevolution,ring: usize,member: usize) -> PrimitiveTrim {
    let surface=surface(shape,face(ring));
    let (index,forward,trim)=match surface {
        Surface::Cylinder{..}=>{
            let axial=ring/2;
            let bottom=2*axial;
            let top=bottom+1;
            let seam=6+axial;
            let lo=shape.level(bottom);
            let hi=shape.level(top);
            let (index,forward,u,v,du,dv)=[
                (seam,false,0.,lo,0.,hi-lo),(bottom,true,0.,lo,1.,0.),
                (seam,true,shape.sweep,lo,0.,hi-lo),(top,false,0.,hi,1.,0.),
            ][member];
            (index,forward,Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)})
        }
        Surface::Plane{frame}=>{
            let (index,forward)=match ring {
                0=>(0,false),2=>(2,false),4=>(3,true),6=>(5,true),7=>(1,true),8=>(4,false),
                _=>unreachable!("two-shoulder planar ring"),
            };
            (index,forward,planar_pcurve(&shape.edge(index).0,frame))
        }
        _=>unreachable!("two-shoulder support"),
    };
    PrimitiveTrim{edge:index,forward,curve:trim,range:shape.edge(index).1}
}
