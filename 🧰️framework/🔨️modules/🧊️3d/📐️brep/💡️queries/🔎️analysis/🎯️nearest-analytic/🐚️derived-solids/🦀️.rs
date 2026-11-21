//! 🐚️ Finds the nearest original boundary of a derived analytic solid.

use super::{Frame3, Pnt3, SolidPrimitive, ValueError, Vec3, check, invalid, primitive_nearest};
use semio_framework_3d_brep_representation::contract::{SphericalCaps, SphericalShell, SphericalShellBoundary};

fn nearest(target:Pnt3, candidates:impl IntoIterator<Item=Pnt3>)->Result<Pnt3,ValueError>{
    let mut best=None;
    for point in candidates{
        check(point)?;
        let d=point-target;
        let length=d.x.hypot(d.y).hypot(d.z);
        if length.is_finite()&&best.is_none_or(|(_,distance)|length<distance){best=Some((point,length));}
    }
    best.map(|(point,_)|point).ok_or_else(invalid)
}

/// 🌐️ Compares the complete authored outer and inner supports.
pub fn spherical_shell_nearest(shell:SphericalShell,target:Pnt3)->Result<Pnt3,ValueError>{
    let project=|inside|match shell.boundary(inside){
        SphericalShellBoundary::Sphere{frame,radius}=>primitive_nearest(SolidPrimitive::Sphere{frame,radius},target),
        SphericalShellBoundary::Box{origin,size:[width,depth,height]}=>primitive_nearest(SolidPrimitive::Box{origin,width,depth,height},target),
    };
    nearest(target,[project(false)?,project(true)?])
}

fn on_meridian(frame:Frame3,azimuth:f64,radius:f64,z:f64)->Pnt3{
    let (sin,cos)=azimuth.sin_cos();
    frame.origin+frame.x*(radius*cos)+frame.y*(radius*sin)+frame.z*z
}

/// 🏔️ Compares each spherical patch and its exact planar closures.
pub fn spherical_caps_nearest(caps:SphericalCaps,target:Pnt3)->Result<Pnt3,ValueError>{
    check(target)?;
    let candidates=(0..caps.ring_sizes().len()).map(|face|{
        let frame=caps.frame(face);
        let local=frame.to_local(target);
        let azimuth=local.y.atan2(local.x);
        let radial=local.x.hypot(local.y);
        if caps.is_disk(face){
            return on_meridian(frame,azimuth,radial.min(caps.rim_radius(face==2)),0.);
        }
        let (low,high,_,_)=caps.band(face);
        let latitude=local.z.atan2(radial).clamp(low,high);
        let (sin,cos)=latitude.sin_cos();
        let radius=caps.radius(face);
        on_meridian(frame,azimuth,radius*cos,radius*sin)
    });
    nearest(target,candidates)
}

/// 📏️ Projects onto the conical generator and both finite cap disks.
pub fn frustum_nearest(frame:Frame3,radii:[f64;2],height:f64,target:Pnt3)->Result<Pnt3,ValueError>{
    check(target)?;
    let local=frame.to_local(target);
    check(local)?;
    let [small,large]=radii;
    let delta=large-small;
    let span=height.hypot(delta);
    if !span.is_finite()||span<=0.||height<=0.||small<=0.||large<=0.{return Err(invalid());}
    let radial=local.x.hypot(local.y);
    let azimuth=local.y.atan2(local.x);
    let along=((radial-small)*(delta/span)+local.z*(height/span)).clamp(0.,span)/span;
    nearest(target,[
        on_meridian(frame,azimuth,delta.mul_add(along,small),height*along),
        on_meridian(frame,azimuth,radial.min(small),0.),
        on_meridian(frame,azimuth,radial.min(large),height),
    ])
}

/// 📦️ Returns exact world bounds of the authored exterior around the cavity.
pub fn spherical_shell_bounds(shell:SphericalShell)->[Pnt3;2]{
    match shell.boundary(false){
        SphericalShellBoundary::Sphere{frame,radius}=>{
            let offset=Vec3::from_array([radius;3]);
            [frame.origin+ -offset,frame.origin+offset]
        }
        SphericalShellBoundary::Box{origin,size}=>[origin,origin+Vec3::from_array(size)],
    }
}

fn radial_component(frame:Frame3,axis:usize)->f64{
    frame.x.to_array()[axis].hypot(frame.y.to_array()[axis])
}

/// 🌅️ Finds each support coordinate of the spherical faces and their planar closures.
pub fn spherical_caps_bounds(caps:SphericalCaps)->[Pnt3;2]{
    [-1.,1.].map(|sign|Pnt3::from_array(std::array::from_fn(|axis|{
        let support=(0..caps.ring_sizes().len()).map(|face|{
            let frame=caps.frame(face);
            let radial=radial_component(frame,axis);
            let extent=if caps.is_disk(face){
                radial*caps.rim_radius(face==2)
            }else{
                let axial=sign*frame.z.to_array()[axis];
                let (low,high,_,_)=caps.band(face);
                let (sin,cos)=axial.atan2(radial).clamp(low,high).sin_cos();
                caps.radius(face)*(radial*cos+axial*sin)
            };
            sign*frame.origin.to_array()[axis]+extent
        }).fold(f64::NEG_INFINITY,f64::max);
        sign*support
    })))
}

/// 🔺️ Uses the support of both end disks of the original conical frustum.
pub fn frustum_bounds(frame:Frame3,radii:[f64;2],height:f64)->[Pnt3;2]{
    let end=frame.origin+frame.z*height;
    let centers=[frame.origin.to_array(),end.to_array()];
    [-1.,1.].map(|sign|Pnt3::from_array(std::array::from_fn(|axis|{
        let radial=radial_component(frame,axis);
        let [a,b]=[0,1].map(|end|sign*centers[end][axis]+radii[end]*radial);
        sign*a.max(b)
    })))
}
