//! 📍️ Exact signed distances from retained original solid proofs.

use super::*;
use semio_framework_3d_brep_representation::contract::frustum_signed_distance;

impl SolidProvenance {
    /// 📌️ Measures finite signed distance with the solid's original trimmed supports.
    pub fn certified_signed_distance(self,point:Pnt3)->Result<f64,ValueError>{
        let unsupported=||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original point classification requires a supported certified boundary");
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"point classification requires finite coordinates and distance");
        Ok(match self {
            Self::Primitive(source)=>source.primitive.signed_boundary_distance(point),
            Self::Bore(source)=>source.signed_boundary_distance(point.to_array()),
            Self::Orthogonal(source)=>source.nearest_boundary(point.to_array()).1,
            Self::PlacedOrthogonal(source,frame)=>source.nearest_boundary(frame.to_local(point).to_array()).1,
            Self::SphericalCaps(source,_)=>source.signed_boundary_distance(point),
            Self::SphericalTorusCut(source,_)=>source.signed_boundary_distance(point),
            Self::SphericalCornerBox(source,_)=>source.signed_boundary_distance(point),
            Self::RoundedBox(source,_)=>source.signed_boundary_distance(point.to_array()),
            Self::ClippedBox(source)=>source.signed_boundary_distance(point).ok_or_else(invalid)?,
            Self::SphericalShell(source,_)=>spherical_shell_distance(source,point),
            Self::CircularPrism(source,_)=>circular_prism_distance(source,point).ok_or_else(unsupported)?,
            Self::PlanarBoundary(Some(source))=>source.signed_boundary_distance(point),
            Self::Frustum(source,_)=>frustum_signed_distance(source.frame,source.radii,source.height,point),
            Self::TorusSector(source,_)=>source.signed_boundary_distance(point),
            Self::Annular(source,_)=>source.signed_boundary_distance(point),
            Self::PlanarBoundary(None)=>return Err(unsupported()),
        })
    }
}
