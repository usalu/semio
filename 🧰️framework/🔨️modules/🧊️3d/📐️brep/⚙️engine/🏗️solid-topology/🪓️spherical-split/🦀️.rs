//! 🪓️ Certifies both original solid boundaries of a spherical plane partition.

use semio_framework_3d_brep_representation::contract::SphericalCaps;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_3d_brep_tolerance::Tol;

/// 📏️ Returns the original signed axial interval of a planar-closed spherical solid.
pub fn spherical_axial_span(shape:SphericalCaps)->Option<[f64;2]>{
    if !shape.is_disk(1){return None;}
    let (low,high,_,_)=shape.band(0);
    Some([low,high].map(|angle|shape.radius(0)*angle.sin()))
}

/// 🪓️ Partitions an original sphere or an axially aligned retained spherical strip in signed plane order.
pub fn split_spherical_support(
    sphere:Frame3,radius:f64,span:Option<[f64;2]>,origin:[f64;3],normal:[f64;3],
)->Result<[SphericalCaps;2],ValueError>{
    let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"spherical Split requires a finite plane crossing its interior");
    if !origin.into_iter().chain(normal).all(f64::is_finite){return Err(invalid());}
    let scale=normal.into_iter().map(f64::abs).fold(0.,f64::max);
    let normal=Vec3::from_array(normal.map(|value|value/scale));
    let support=Frame3::from_normal(sphere.origin,normal).ok_or_else(invalid)?;
    let delta=Pnt3::from_array(origin)-sphere.origin;
    let tolerance=Tol::DEFAULT.value();
    if let Some([low,high])=span {
        let deviation=support.z.cross(sphere.z).norm();
        if deviation>f64::EPSILON*16. || deviation*radius>tolerance{
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained spherical strip Split requires a plane parallel to its closures"));
        }
        let limit=delta.dot(sphere.z);
        if !(limit>low+tolerance&&limit<high-tolerance){return Err(invalid());}
        let below=SphericalCaps::from_band(sphere,radius,[low,limit],tolerance).ok_or_else(invalid)?;
        let above=if high>=radius-tolerance{
            SphericalCaps::from_plane(sphere,radius,limit,tolerance)
        }else{
            SphericalCaps::from_band(sphere,radius,[limit,high],tolerance)
        }.ok_or_else(invalid)?;
        return Ok(if support.z.dot(sphere.z)>0.{[above,below]}else{[below,above]});
    }
    let offset=delta.dot(support.z);
    let opposite=Frame3{y:-support.y,z:-support.z,..support};
    let [Some(positive),Some(negative)]=[(support,offset),(opposite,-offset)]
        .map(|(side,limit)|SphericalCaps::from_plane(side,radius,limit,tolerance))
        else{return Err(invalid());};
    Ok([positive,negative])
}
