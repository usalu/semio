//! ↔️ Certified sharp offsets by exact support displacement.

use super::*;
use semio_framework_3d_brep_vector::matrix::Frame3;

fn unsupported()->ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Offset Solid requires a certified box, sphere, cylinder, cone, frustum or ring torus")
}

fn offset_distance(distance:f64)->Result<(),ValueError>{
    if !distance.is_finite()||distance.abs()<=1e-15 {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"solid offset requires a finite nonzero distance"));
    }
    Ok(())
}

fn shifted_cap(frame:Frame3,distance:f64)->Frame3{
    Frame3{origin:frame.origin-frame.z*distance,..frame}
}

fn offset_axial_size(radius:f64,height:f64)->Result<(),ValueError>{
    if [radius,height].into_iter().all(|n|n.is_finite()&&n>Tol::DEFAULT.value()){
        return Ok(());
    }
    Err(ValueError::literal(ValueRefusalKind::InvalidValue,"solid offset collapsed the axial height or radius"))
}

fn offset_certified_primitive(source:SolidPrimitive,distance:f64)->Result<SolidPrimitive,ValueError>{
    let invalid=|message|ValueError::literal(ValueRefusalKind::InvalidValue,message);
    offset_distance(distance)?;
    let tol=Tol::DEFAULT.value();
    match source {
        SolidPrimitive::Box{origin,width,depth,height}=>{
            let axes=[(origin.x,width),(origin.y,depth),(origin.z,height)].map(|(min,size)|{
                let max=(min+size)+distance;
                let min=min-distance;
                (min,max-min)
            });
            if axes.iter().any(|&(min,span)|!min.is_finite()||!span.is_finite()||span<=tol) {
                return Err(invalid("solid offset collapsed a box dimension or exceeded the coordinate range"));
            }
            let [(x,width),(y,depth),(z,height)]=axes;
            Ok(SolidPrimitive::Box{origin:Pnt3::new(x,y,z),width,depth,height})
        }
        SolidPrimitive::Sphere{frame,radius}=>{
            let radius=radius+distance;
            if !radius.is_finite()||radius<=tol{return Err(invalid("solid offset collapsed the sphere radius or exceeded the coordinate range"));}
            Ok(SolidPrimitive::Sphere{frame,radius})
        }
        SolidPrimitive::Cylinder{frame,radius,height}=>{
            let radius=radius+distance;
            let height=distance.mul_add(2.,height);
            offset_axial_size(radius,height)?;
            Ok(SolidPrimitive::Cylinder{frame:shifted_cap(frame,distance),radius,height})
        }
        SolidPrimitive::Cone{frame,radius,height}=>{
            let displaced_height=distance.mul_add(1.+height.hypot(radius)/radius,height);
            let displaced_radius=radius*(displaced_height/height);
            offset_axial_size(displaced_radius,displaced_height)?;
            Ok(SolidPrimitive::Cone{frame:shifted_cap(frame,distance),radius:displaced_radius,height:displaced_height})
        }
        SolidPrimitive::Torus{frame,major,minor}=>{
            let minor=minor+distance;
            if !minor.is_finite()||minor<=tol||major-minor<=tol {
                return Err(invalid("solid offset collapsed the torus cross section or closed its central opening"));
            }
            Ok(SolidPrimitive::Torus{frame,major,minor})
        }
        _=>Err(unsupported()),
    }
}

fn offset_frustum(frame:Frame3,[small,large]:[f64;2],height:f64,distance:f64)->Result<ConicalFrustum,ValueError>{
    offset_distance(distance)?;
    let slope=(large-small)/height;
    let hypotenuse=slope.hypot(1.)+slope;
    let radii=[distance.mul_add(hypotenuse.recip(),small),distance.mul_add(hypotenuse,large)];
    ConicalFrustum::new(shifted_cap(frame,distance),radii,distance.mul_add(2.,height))
        .ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"solid offset collapsed a frustum rim or axial height"))
}

impl SolidProvenance {
    /// ↔️ Rebuilds the original exact solid from its displaced support geometry.
    pub fn offset_boundary(&self,distance:f64)->Result<SolidBoundary,ValueError>{
        match self{
            Self::Primitive(source)=>offset_certified_primitive(source.primitive,distance).map(SolidBoundary::Analytic),
            Self::Frustum(source,_)=>offset_frustum(source.frame,source.radii,source.height,distance).map(SolidBoundary::Frustum),
            _=>Err(unsupported()),
        }
    }
}
