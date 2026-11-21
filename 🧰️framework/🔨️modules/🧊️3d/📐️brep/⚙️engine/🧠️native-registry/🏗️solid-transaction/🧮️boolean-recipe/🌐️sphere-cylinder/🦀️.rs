//! 🌐️ Sphere and finite-cylinder Booleans certified in the cylinder frame.

use semio_framework_3d_brep_vector::matrix::Frame3;
use super::*;

fn spherical_clip(frame:Frame3,radius:f64,span:[f64;2])->Result<SolidBoundary,ValueError>{
    let [low,high]=span;
    let tolerance=Tol::DEFAULT.value();
    let result=if low<= -radius{
        let reverse=Frame3{y:-frame.y,z:-frame.z,..frame};
        SphericalCaps::from_plane(reverse,radius,-high,tolerance)
    }else if high>=radius{
        SphericalCaps::from_plane(frame,radius,low,tolerance)
    }else{
        SphericalCaps::from_band(frame,radius,span,tolerance)
    };
    result.map(SolidBoundary::SphericalCaps).ok_or_else(unsupported)
}

pub(super) fn combine(a:SolidPrimitive,b:SolidPrimitive,operation:BooleanOp)->Result<SolidBoundary,ValueError>{
    let sphere_is_a=matches!(a,SolidPrimitive::Sphere{..});
    let (sphere,cylinder)=if sphere_is_a{(a,b)}else{(b,a)};
    let SolidPrimitive::Sphere{frame:ball,radius}=sphere else{return Err(unsupported());};
    let SolidPrimitive::Cylinder{frame:tube,radius:tool_radius,height}=cylinder else{return Err(unsupported());};
    let offset=ball.origin-tube.origin;
    let radial=offset.dot(tube.x).hypot(offset.dot(tube.y));
    let low=-offset.dot(tube.z);
    let high=low+height;
    if !(radial.is_finite()&&low.is_finite()&&high.is_finite()&&low<high){return Err(unsupported());}
    let nearest_axis=low.max(-high).max(0.);
    if (radial-tool_radius).max(0.).hypot(nearest_axis)>=radius{
        return match operation{
            BooleanOp::Intersect=>Err(empty_result()),
            BooleanOp::Cut=>Ok(SolidBoundary::Analytic(a)),
            BooleanOp::Unite=>Err(unsupported()),
        };
    }
    let farthest_axis=low.abs().max(high.abs());
    if (radial+tool_radius).hypot(farthest_axis)<=radius{
        return nested_primitives(sphere,cylinder,sphere_is_a,operation).map(SolidBoundary::Analytic);
    }
    let overlap=[low.max(-radius),high.min(radius)];
    let cross_section=((radius-nearest_axis)*(radius+nearest_axis)).sqrt();
    if !(tool_radius-radial>=cross_section){return Err(unsupported());}
    if overlap==[-radius,radius]{
        return nested_primitives(cylinder,sphere,!sphere_is_a,operation).map(SolidBoundary::Analytic);
    }
    let span=match operation{
        BooleanOp::Intersect=>overlap,
        BooleanOp::Cut if sphere_is_a&&overlap[0]== -radius=>[overlap[1],radius],
        BooleanOp::Cut if sphere_is_a&&overlap[1]==radius=>[-radius,overlap[0]],
        _=>return Err(unsupported()),
    };
    spherical_clip(Frame3{origin:ball.origin,..tube},radius,span)
}
