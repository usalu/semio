//! 🧰️ Dispatches exact fixed-sized finishing boundaries for certified boxes.

use super::{ChamferedBox, OrientedBox, RoundedBox, SolidBoundary, Tol, box_edge_chamfer::chamfer_placed};

#[path="🧰️fillet-profile/🦀️.rs"]
mod fillet_profile;
use semio_framework_3d_brep_construction::primitives::{BoxGeometry,SolidPrimitive};

/// 🗺️ The original planar finish applied to one certified box member.
#[derive(Clone, Copy)]
pub enum BoxFinish {
    Chamfer(f64),
    ChamferAll(f64),
    Fillet(f64),
    FilletAll(f64),
    Draft { pull: [f64;3], neutral: [f64;3], angle: f64 },
}

impl BoxFinish {
    /// 🧱️ Certifies and constructs the exact source member's replacement boundary.
    pub fn boundary(self, primitive: SolidPrimitive, ordinals: &[usize]) -> Option<SolidBoundary> {
        match self {
            Self::ChamferAll(distance) => ChamferedBox::new(primitive, distance).map(SolidBoundary::ChamferedBox),
            Self::Chamfer(distance) => super::box_edge_chamfer(primitive, *ordinals.first()?, distance).map(SolidBoundary::Prism),
            Self::FilletAll(radius) => {
                let SolidPrimitive::Box { origin,width,depth,height }=primitive else{return None;};
                RoundedBox::new(origin,[width,depth,height],radius,Tol::DEFAULT.value()).map(SolidBoundary::RoundedBox)
            }
            Self::Fillet(radius) => fillet_profile::fillet_box(primitive,ordinals,radius).map(SolidBoundary::CircularPrism),
            Self::Draft { pull,neutral,angle } => super::box_face_draft::box_face_draft(primitive, *ordinals.first()?, pull, neutral, angle).map(SolidBoundary::Prism),
        }
    }

    pub fn source_requirement(self) -> &'static str {
        match self {
            Self::ChamferAll(_) => "all-edge chamfer requires a certified box",
            Self::Chamfer(_) => "retained chamfer requires an original edge of a certified box",
            Self::Fillet(_) => "retained fillet requires an original edge of a certified box",
            Self::FilletAll(_) => "all-edge fillet requires a certified box",
            Self::Draft { .. } => "retained draft requires an original side of a certified box",
        }
    }

    pub fn geometry_requirement(self) -> &'static str {
        match self {
            Self::ChamferAll(_) => "all-edge chamfer distance must leave all six stock faces nondegenerate",
            Self::Chamfer(_) => "chamfer distance must keep both adjoining box sides nondegenerate",
            Self::Fillet(_) => "retained fillet requires distinct parallel edges with positive straight-side clearance",
            Self::FilletAll(_) => "all-edge fillet radius must leave all six stock faces nondegenerate",
            Self::Draft { .. } => "draft requires a perpendicular principal-axis pull and a finite angle that keeps the box nondegenerate",
        }
    }
}

impl OrientedBox {
    /// 🔸️ Bevels all twelve edges and closes the eight corners in the stock frame.
    pub fn chamfer_all(self,distance:f64)->Option<SolidBoundary>{
        let (origin,axes,size)=self.placement();
        ChamferedBox::placed(origin,axes,size,distance).map(SolidBoundary::ChamferedBox)
    }

    fn canonical_edge(&self,edge:usize)->Option<usize>{
        if edge>=BoxGeometry::EDGES.len(){return None;}
        let (_,[x,y,z],_)=self.placement();
        if x.cross(y).dot(z)>=0.{return Some(edge);}
        Some(if edge<8{edge/4*4+3-edge%4}else{8+(4-edge%4)%4})
    }

    /// 🔶️ Chamfers one native prism edge in its rectangular placement.
    pub fn chamfer(self,edge:usize,distance:f64)->Option<SolidBoundary>{
        let (origin,axes,size)=self.placement();
        chamfer_placed(origin,axes,size,self.canonical_edge(edge)?,distance).map(SolidBoundary::Prism)
    }

    /// 🧰️ Fillets parallel original prism edges in this rectangular placement.
    pub fn fillet(self,edges:&[usize],radius:f64)->Option<SolidBoundary>{
        if !(1..=4).contains(&edges.len()){return None;}
        let mut canonical=[0;4];
        let (origin,axes,size)=self.placement();
        for (slot,&edge) in canonical.iter_mut().zip(edges){*slot=self.canonical_edge(edge)?;}
        fillet_profile::fillet_placed(origin,axes,size,&canonical[..edges.len()],radius).map(SolidBoundary::CircularPrism)
    }
}
