//! 🧵️ Native UV trims on two spherical faces sharing one circle.

use super::*;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_vector::{Pnt2,Vec2};

pub(super) fn surface(shape:SphericalCaps,face:usize)->Surface{
    let frame=shape.frame(face);
    if shape.is_disk(face){Surface::Plane{frame}}else{Surface::Sphere{frame,radius:shape.radius(face)}}
}

pub(super) fn trim(shape:SphericalCaps,face:usize,member:usize)->PrimitiveTrim{
    if shape.is_disk(face){
        let edge=if face==2{2}else{0};
        let (curve,range,_)=shape.edge(edge);
        return PrimitiveTrim{
            edge,forward:face==2,range,
            curve:semio_framework_3d_brep_construction::primitives::planar_pcurve(&curve,shape.frame(face)),
        };
    }
    let (a,b,low_pole,_)=shape.band(face);
    let seam=1+2*face;
    let pole=seam+1;
    let [lower,upper]=if shape.is_band(){[0,2]}else if low_pole{[pole,0]}else{[0,pole]};
    let turn=std::f64::consts::TAU;
    let (edge,forward,u,v,du,dv,range)=[
        (lower,true,0.,a,1.,0.,(0.,turn)),(seam,true,turn,0.,0.,1.,(a,b)),
        (upper,false,0.,b,1.,0.,(0.,turn)),(seam,false,0.,0.,0.,1.,(a,b)),
    ][member];
    PrimitiveTrim{edge,forward,curve:Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)},range}
}
