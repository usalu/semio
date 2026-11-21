//! 🌗️ Certified sphere and box Boolean results with one plane or an opposing pair.

use super::*;
use semio_framework_3d_brep_vector::{matrix::Frame3, Vec3};

pub(super) fn combine(a:SolidPrimitive,b:SolidPrimitive,op:BooleanOp)->Result<SolidBoundary,ValueError>{
    let sphere_is_a=matches!(a,SolidPrimitive::Sphere{..});
    let (sphere,box_shape)=if sphere_is_a{(a,b)}else{(b,a)};
    let SolidPrimitive::Sphere{frame,radius}=sphere else{return Err(unsupported());};
    let SolidPrimitive::Box{origin,width,depth,height}=box_shape else{return Err(unsupported());};
    let Bounds{lo,hi}=Bounds::box_primitive(origin,width,depth,height);
    let center=frame.origin.to_array();
    let near:[f64;3]=std::array::from_fn(|i|center[i]-center[i].clamp(lo[i],hi[i]));
    let far:[f64;3]=std::array::from_fn(|i|(center[i]-lo[i]).abs().max((center[i]-hi[i]).abs()));
    let length=|[x,y,z]:[f64;3]|x.hypot(y).hypot(z);
    if length(near)>=radius{
        return match op{
            BooleanOp::Intersect=>Err(empty_result()),BooleanOp::Cut=>Ok(SolidBoundary::Analytic(a)),
            BooleanOp::Unite=>Err(unsupported()),
        };
    }
    let sphere_inside=(0..3).all(|i|lo[i]<=center[i]-radius&&hi[i]>=center[i]+radius);
    if sphere_inside{
        if matches!(op,BooleanOp::Cut)&&!sphere_is_a{
            return SphericalShell::from_box(origin,[width,depth,height],frame,radius,Tol::DEFAULT.value())
                .map(SolidBoundary::SphericalShell).ok_or_else(unsupported);
        }
        return nested_primitives(box_shape,sphere,!sphere_is_a,op).map(SolidBoundary::Analytic);
    }
    if length(far)<=radius{return nested_primitives(sphere,box_shape,sphere_is_a,op).map(SolidBoundary::Analytic);}
    let sign=match op{
        BooleanOp::Intersect=>1.,BooleanOp::Cut if sphere_is_a=>-1.,_=>return Err(unsupported()),
    };
    let mut clips=[None;2];
    for i in 0..3{
        for (normal,limit,crosses) in [
            (1.,lo[i],lo[i]>center[i]-radius),(-1.,hi[i],hi[i]<center[i]+radius),
        ]{
            if crosses{
                *clips.iter_mut().find(|clip|clip.is_none()).ok_or_else(unsupported)?=Some((i,normal*sign,limit));
            }
        }
    }
    let (axis,direction,limit)=clips[0].ok_or_else(unsupported)?;
    let orient=Vec3::from_array(std::array::from_fn(|i|if i==axis{direction}else{0.}));
    let support=Frame3::from_normal(frame.origin,orient).ok_or_else(unsupported)?;
    let tol=Tol::DEFAULT.value();
    let offset=direction*(limit-center[axis]);
    let result=match clips[1]{
        None=>SphericalCaps::from_plane(support,radius,offset,tol),
        Some((other,-1.,high)) if matches!(op,BooleanOp::Intersect)&&axis==other&&direction==1.=>{
            SphericalCaps::from_band(support,radius,[offset,high-center[axis]],tol)
                .filter(|shape|(shape.frame(2).origin.to_array()[axis]-high).abs()<=tol)
        }
        _=>None,
    }.ok_or_else(unsupported)?;
    if (result.rim_frame().origin.to_array()[axis]-limit).abs()>tol{return Err(unsupported());}
    Ok(SolidBoundary::SphericalCaps(result))
}
