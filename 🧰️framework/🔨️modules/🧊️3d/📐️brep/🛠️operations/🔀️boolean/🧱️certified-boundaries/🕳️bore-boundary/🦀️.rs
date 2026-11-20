//! 🕳️ Exact boundary of an axis-aligned box with one fully penetrating circular bore.

use super::{unsupported, ValueError};
use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim, SolidPrimitive, planar_pcurve};
use semio_framework_3d_brep_representation::contract::BorePreviewSource;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

pub struct BoreBoundary {
    box_shape: SolidPrimitive,
    cylinder: SolidPrimitive,
    volume: f64,
}

impl BoreBoundary {
    pub fn new(stock: SolidPrimitive, tool: SolidPrimitive) -> Result<Self, ValueError> {
        let SolidPrimitive::Box { origin, width, depth, height } = stock else { return Err(unsupported()); };
        let SolidPrimitive::Cylinder { frame, radius, height: tool_length } = tool else { return Err(unsupported()); };
        let travel = if frame.z == Vec3::Z { tool_length }
            else if frame.z == -Vec3::Z { -tool_length }
            else { return Err(unsupported()); };
        let center = frame.origin;
        let end = center.z + travel;
        let tool_min = center.z.min(end);
        let tool_max = center.z.max(end);
        let top = origin.z + height;
        let tol = Tol::DEFAULT.value();
        if !end.is_finite() || tool_min > origin.z || tool_max < top {
            return Err(unsupported());
        }
        if [center.x, center.y].into_iter().any(|coordinate| {
            coordinate - radius >= coordinate || coordinate + radius <= coordinate
        }) { return Err(unsupported()); }
        let radial_clearances = [
            center.x - radius - origin.x, origin.x + width - (center.x + radius),
            center.y - radius - origin.y, origin.y + depth - (center.y + radius),
        ];
        if radial_clearances.into_iter().any(|distance| !distance.is_finite() || distance <= tol) {
            return Err(unsupported());
        }
        let cylinder = SolidPrimitive::Cylinder {
            frame: Frame3 { origin: Pnt3::new(center.x, center.y, origin.z), ..Frame3::WORLD }, radius, height,
        };
        let volume = stock.volume() - cylinder.volume();
        if !volume.is_finite() || volume <= 0.0 { return Err(unsupported()); }
        Ok(Self { box_shape: stock, cylinder, volume })
    }

    pub fn preview_source(&self, first_label: u64) -> BorePreviewSource {
        let SolidPrimitive::Box { origin, width, depth, height } = self.box_shape else { unreachable!("bore stock is a box"); };
        let SolidPrimitive::Cylinder { frame, radius, .. } = self.cylinder else { unreachable!("bore tool is a cylinder"); };
        BorePreviewSource {
            origin: [origin.x, origin.y, origin.z], dimensions: [width, depth, height],
            center: [frame.origin.x, frame.origin.y], radius, first_label, volume: self.volume,
        }
    }

    pub fn vertex(&self, index: usize) -> Pnt3 {
        if index < 8 { self.box_shape.vertex(index) } else { self.cylinder.vertex(index - 8) }
    }

    pub fn edge(&self, index: usize) -> PrimitiveEdge {
        if index < 12 { return self.box_shape.edge(index); }
        let mut edge = self.cylinder.edge(index - 12);
        edge.vertices = edge.vertices.map(|vertex| vertex + 8);
        edge
    }

    pub fn surface(&self, face: usize) -> Surface {
        if face < 6 { self.box_shape.surface(face) } else { self.cylinder.surface(0) }
    }

    pub fn trim(&self, ring: usize, member: usize) -> PrimitiveTrim {
        if ring < 6 { return self.box_shape.trim(ring, member); }
        if ring == 6 {
            let mut trim = self.cylinder.trim(0, member);
            trim.edge += 12;
            return trim;
        }
        let top = ring == 8;
        let edge = 12 + usize::from(top);
        let spec = self.edge(edge);
        let Surface::Plane { frame } = self.surface(usize::from(top)) else { unreachable!("box cap is planar"); };
        PrimitiveTrim { edge, forward: !top, curve: planar_pcurve(&spec.curve, frame), range: spec.range }
    }
}
