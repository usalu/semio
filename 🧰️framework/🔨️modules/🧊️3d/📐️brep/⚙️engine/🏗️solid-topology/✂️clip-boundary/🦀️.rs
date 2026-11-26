//! 📐️ Exact native line trims on one convex clipped box.

use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim};
use semio_framework_3d_brep_representation::contract::ClippedBoxBoundary;
use semio_framework_3d_brep_construction::primitives::planar_pcurve;

pub fn edge(shape:&ClippedBoxBoundary,index:usize)->PrimitiveEdge{
    let (vertices,curve)=shape.edge(index);
    PrimitiveEdge{vertices,curve,range:(0.,1.)}
}

pub fn trim(shape:&ClippedBoxBoundary,face:usize,member:usize)->PrimitiveTrim{
    let (id,forward)=shape.member(face,member);
    let edge=self::edge(shape,id);
    PrimitiveTrim{edge:id,forward,curve:planar_pcurve(&edge.curve,shape.frame(face)),range:edge.range}
}
