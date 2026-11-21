//! ✂️ A finite transverse plane partition preserving the source axial frame.

use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🪓️ Retains the signed partition and its common axial support.
pub struct AxialPartition {
    pub frame:Frame3,
    pub heights:[f64;2],
    pub end:Pnt3,
    pub section_radius:f64,
    pub positive_is_upper:bool,
}

impl AxialPartition {
    pub fn new(frame:Frame3,radius:f64,height:f64,cone:bool,origin:[f64;3],normal:[f64;3])->Result<Self,ValueError>{
        let invalid=|reason|ValueError::literal(ValueRefusalKind::InvalidValue,reason);
        if !origin.into_iter().chain(normal).all(f64::is_finite) {
            return Err(invalid("axial split plane must be finite"));
        }
        let scale = normal.into_iter().map(f64::abs).fold(0., f64::max);
        if scale == 0. {
            return Err(invalid("axial split plane requires a nonzero normal"));
        }
        let normal = Vec3::from_array(normal.map(|value| value / scale));
        let normal = normal * (1. / normal.norm());
        let tilt = frame.z.cross(normal).norm();
        if !(tilt <= 16. * f64::EPSILON) {
            return Err(invalid("split plane must be perpendicular to the solid axis"));
        }
        let origin = Pnt3::from_array(origin);
        let alignment = frame.z.dot(normal);
        let low = (origin - frame.origin).dot(normal) / alignment;
        let high = height - low;
        let tolerance = Tol::DEFAULT.value();
        if !(low.min(high) > tolerance) {
            return Err(invalid("original Split requires a plane strictly within the solid axial span"));
        }
        let center = frame.origin + frame.z * low;
        let end = frame.origin + frame.z * height;
        let section_radius = if cone { radius * (high / height) } else { radius };
        let disk_error = frame.x.dot(normal).hypot(frame.y.dot(normal)) * section_radius;
        let cap_error = (origin - center).dot(normal).abs() + disk_error;
        if !(cap_error <= tolerance && (center - frame.origin).norm() > tolerance && (end - center).norm() > tolerance
            && (center + frame.z * high - end).norm() <= tolerance)
        {
            return Err(invalid("axial split cap exceeds the representable plane tolerance"));
        }
        let frame = Frame3 { origin: center, ..frame };
        Ok(Self{frame,heights:[low,high],end,section_radius,positive_is_upper:alignment>0.})
    }
}
