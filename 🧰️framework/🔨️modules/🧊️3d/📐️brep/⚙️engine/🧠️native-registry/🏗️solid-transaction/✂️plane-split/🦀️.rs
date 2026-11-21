//! ✂️ Certified axial plane splits with two independent analytic boundaries.

use super::*;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_representation::contract::AxialPartition;

/// 🪓️ Returns the two closed analytic halves in signed plane order.
pub fn plane_split(stock: SolidPrimitive, origin: [f64; 3], normal: [f64; 3]) -> Result<[SolidBoundary; 2], ValueError> {
    let invalid = |reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason);
    let (frame, radius, height, cone) = match stock {
        SolidPrimitive::Box { .. } if normal.iter().filter(|&&n|n!=0.).count()<=1 => return stock.split_by_axis_plane(origin,normal).map(|pair|pair.map(SolidBoundary::Analytic)).map_err(invalid),
        SolidPrimitive::Box { .. } => return OrientedBox::from_primitive(stock).unwrap().split(origin,normal),
        SolidPrimitive::Cylinder { frame, radius, height } => (frame, radius, height, false),
        SolidPrimitive::Cone { frame, radius, height } => (frame, radius, height, true),
        _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Split requires a certified box, cylinder, or cone")),
    };
    let AxialPartition{frame:moved,heights:[low,high],end,section_radius,positive_is_upper}
        =AxialPartition::new(frame,radius,height,cone,origin,normal)?;
    let (lower, upper) = if cone {
        let reversed = Frame3 { y: -frame.y, z: -frame.z, ..moved };
        let frustum = ConicalFrustum::new(reversed, [section_radius, radius], low)
            .filter(|shape| (shape.apex() - end).norm() <= Tol::DEFAULT.value())
            .ok_or_else(|| invalid("cone split cannot preserve the authored conical support"))?;
        let tip = SolidPrimitive::Cone { frame: moved, radius: section_radius, height: high };
        tip.validate().map_err(invalid)?;
        (SolidBoundary::Frustum(frustum), SolidBoundary::Analytic(tip))
    } else {
        let lower = SolidPrimitive::Cylinder { frame, radius, height: low };
        let upper = SolidPrimitive::Cylinder { frame: moved, radius, height: high };
        lower.validate().map_err(invalid)?;
        upper.validate().map_err(invalid)?;
        (SolidBoundary::Analytic(lower), SolidBoundary::Analytic(upper))
    };
    Ok(if positive_is_upper { [upper, lower] } else { [lower, upper] })
}
impl SolidProvenance {
    /// 🔍️ Chooses the bounded plane split of the captured original solid encoding.
    pub fn split_by_plane(self, origin:[f64;3],normal:[f64;3])->Result<[SolidBoundary;2],ValueError>{
        let unsupported=||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Split requires a certified box, cylinder, or cone");
        match self {
            Self::Primitive(source)=>plane_split(source.primitive,origin,normal),
            Self::PlanarBoundary(Some(source))=>source.rectangular().ok_or_else(unsupported)?.split(origin,normal),
            _=>Err(unsupported()),
        }
    }
}
