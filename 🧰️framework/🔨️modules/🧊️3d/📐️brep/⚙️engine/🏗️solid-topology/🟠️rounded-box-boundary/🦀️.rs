//! 🧵️ Native trims for the planar, cylindrical and spherical rounded box supports.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim};
use semio_framework_3d_brep_representation::contract::RoundedBox;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::Curve2;
use semio_framework_3d_brep_representation::contract::box_finish_layout::{TOPOLOGY, face};
use semio_framework_3d_brep_vector::{Pnt2,Vec2};

pub fn edge(shape:RoundedBox,index:usize)->PrimitiveEdge{
    let (vertices,curve,range)=shape.edge(index);
    PrimitiveEdge{vertices,curve,range}
}

pub fn surface(shape:RoundedBox,index:usize)->Surface{
    let frame=shape.face_frame(index);
    let radius=shape.radius();
    match index {
        0..6=>Surface::Plane{frame},
        6..18=>Surface::Cylinder{frame,radius},
        _=>Surface::Sphere{frame,radius},
    }
}

pub fn trim(shape:RoundedBox,index:usize,member:usize)->PrimitiveTrim{
    let edge=TOPOLOGY.members[index][member];
    let spec=self::edge(shape,edge);
    let curve=if index<6 {
        planar_pcurve(&spec.curve,shape.face_frame(index))
    } else {
        let [[u,v],[w,h]]=shape.trim_uv(index,spec.vertices);
        let span=spec.range.1-spec.range.0;
        Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new((w-u)/span,(h-v)/span)}
    };
    PrimitiveTrim{edge,forward:face(index).vertices[member]==spec.vertices[0],curve,range:spec.range}
}
