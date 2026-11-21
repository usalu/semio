//! 🕳️ Exact boundary of an axis-aligned box with a through, blind or enclosed circular bore.

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
    bore_span:[f64;2],
    volume: f64,
}

impl BoreBoundary {
    pub fn new(stock: SolidPrimitive, tool: SolidPrimitive) -> Result<Self, ValueError> {
        let SolidPrimitive::Box { origin, height, .. } = stock else { return Err(unsupported()); };
        let SolidPrimitive::Cylinder { frame, radius, height: tool_length } = tool else { return Err(unsupported()); };
        let travel = if frame.z == Vec3::Z { tool_length }
            else if frame.z == -Vec3::Z { -tool_length }
            else { return Err(unsupported()); };
        let center = frame.origin;
        let end = center.z + travel;
        let tool_min = center.z.min(end);
        let tool_max = center.z.max(end);
        let top = origin.z + height;
        let [lo,hi]=[tool_min.max(origin.z),tool_max.min(top)];
        let tol=Tol::DEFAULT.value();
        let clipped_height=hi-lo;
        if !end.is_finite()||clipped_height<=tol||(lo>origin.z&&lo-origin.z<=tol)||(hi<top&&top-hi<=tol)
            || (lo+clipped_height-hi).abs()>tol{return Err(unsupported());}
        let mut bore=Self::from_profile(stock,[center.x,center.y],radius)?;
        if bore.bore_span==[lo,hi]{return Ok(bore);}
        bore.bore_span=[lo,hi];
        bore.cylinder=SolidPrimitive::Cylinder{frame:Frame3{origin:Pnt3::new(center.x,center.y,lo),..Frame3::WORLD},radius,height:clipped_height};
        bore.volume=stock.volume()-bore.cylinder.volume();
        if !bore.volume.is_finite()||bore.volume<=0.0{return Err(unsupported());}
        Ok(bore)
    }

    /// 🕳️ Closes a rectangular stock around one strictly interior circular profile.
    pub fn from_profile(stock: SolidPrimitive, center: [f64;2], radius: f64) -> Result<Self, ValueError> {
        let SolidPrimitive::Box { origin, width, depth, height } = stock else { return Err(unsupported()); };
        stock.validate().map_err(|_|unsupported())?;
        let tol=Tol::DEFAULT.value();
        let [x,y]=center;
        if !radius.is_finite() || radius<=tol || center.into_iter().any(|v| v-radius>=v || v+radius<=v) {return Err(unsupported());}
        let radial_clearances = [
            x - radius - origin.x, origin.x + width - (x + radius),
            y - radius - origin.y, origin.y + depth - (y + radius),
        ];
        if radial_clearances.into_iter().any(|distance| !distance.is_finite() || distance <= tol) {
            return Err(unsupported());
        }
        let cylinder = SolidPrimitive::Cylinder {
            frame: Frame3 { origin: Pnt3::new(x, y, origin.z), ..Frame3::WORLD }, radius, height,
        };
        let volume = stock.volume() - cylinder.volume();
        if !volume.is_finite() || volume <= 0.0 { return Err(unsupported()); }
        Ok(Self { box_shape:stock,cylinder,bore_span:[origin.z,origin.z+height],volume })
    }

    pub fn preview_source(&self, first_label: u64) -> BorePreviewSource {
        let SolidPrimitive::Box { origin, width, depth, height } = self.box_shape else { unreachable!("bore stock is a box"); };
        let SolidPrimitive::Cylinder { frame, radius, .. } = self.cylinder else { unreachable!("bore tool is a cylinder"); };
        BorePreviewSource {
            origin: [origin.x, origin.y, origin.z], dimensions: [width, depth, height],
            center:[frame.origin.x,frame.origin.y],bore_span:self.bore_span,radius, first_label, volume: self.volume,
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
        if face<6{return self.box_shape.surface(face);}
        if face==6{return self.cylinder.surface(0);}
        self.cylinder.surface(if face==self.preview_source(0).cap_face(false){1}else{2})
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
        let source=self.preview_source(0);
        let Surface::Plane{frame}=self.surface(source.cap_face(top))else{unreachable!("bore cap is planar");};
        PrimitiveTrim{edge,forward:top!=source.is_open(top),curve:planar_pcurve(&spec.curve,frame),range:spec.range}
    }
}
