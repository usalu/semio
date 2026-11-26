//! 🍩️ Native capped ring-torus sector boundary with explicit periodic seam uses.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim};
use semio_framework_3d_brep_representation::contract::TorusSector;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_vector::{Pnt2,Vec2,matrix::Frame3};
use std::f64::consts::TAU;

pub fn edge(shape:TorusSector,index:usize)->PrimitiveEdge{
    let (curve,range,vertices)=shape.edge(index);
    PrimitiveEdge{curve,range,vertices}
}

pub fn surface(shape:TorusSector,face:usize)->Surface{
    match face{
        0=>Surface::Torus{frame:shape.frame,major_radius:shape.major,minor_radius:shape.minor},
        1=>Surface::Plane{frame:shape.rim(0)},
        _=>{let rim=shape.rim(1);Surface::Plane{frame:Frame3{y:-rim.y,z:-rim.z,..rim}}}
    }
}

pub fn trim(shape:TorusSector,face:usize,member:usize)->PrimitiveTrim{
    let (index,forward)=match (face,member){
        (0,0)=>(2,true),(0,1)=>(1,true),(0,2)=>(2,false),(0,_)=>(0,false),
        (1,_)=>(0,true),_=>(1,false),
    };
    let spec=edge(shape,index);
    let curve=if let Surface::Plane{frame}=surface(shape,face){planar_pcurve(&spec.curve,frame)}
        else{
            let [u,v,du,dv]=[[0.,0.,1.,0.],[shape.sweep,0.,0.,1.],[0.,TAU,1.,0.],[0.,0.,0.,1.]][member];
            Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)}
        };
    PrimitiveTrim{edge:index,forward,curve,range:spec.range}
}
