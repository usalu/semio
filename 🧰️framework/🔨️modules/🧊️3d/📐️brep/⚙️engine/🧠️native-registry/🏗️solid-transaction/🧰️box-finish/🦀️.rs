//! 🧰️ Dispatches exact fixed-sized finishing boundaries for certified boxes.

use semio_framework_3d_brep_construction::{primitives::SolidPrimitive,prism_boundary::PrismBoundary};

/// 🗺️ The original planar finish applied to one certified box member.
#[derive(Clone, Copy)]
pub enum BoxFinish {
    Chamfer(f64),
    Draft { pull: [f64;3], neutral: [f64;3], angle: f64 },
}

impl BoxFinish {
    /// 🧱️ Certifies and constructs the exact source member's replacement boundary.
    pub fn boundary(self, primitive: SolidPrimitive, ordinal: usize) -> Option<PrismBoundary> {
        match self {
            Self::Chamfer(distance) => super::box_edge_chamfer(primitive, ordinal, distance),
            Self::Draft { pull,neutral,angle } => super::box_face_draft::box_face_draft(primitive, ordinal, pull, neutral, angle),
        }
    }

    pub fn source_requirement(self) -> &'static str {
        match self {
            Self::Chamfer(_) => "retained chamfer requires an original edge of a certified box",
            Self::Draft { .. } => "retained draft requires an original side of a certified box",
        }
    }

    pub fn geometry_requirement(self) -> &'static str {
        match self {
            Self::Chamfer(_) => "chamfer distance must keep both adjoining box sides nondegenerate",
            Self::Draft { .. } => "draft requires a perpendicular principal-axis pull and a finite angle that keeps the box nondegenerate",
        }
    }
}
