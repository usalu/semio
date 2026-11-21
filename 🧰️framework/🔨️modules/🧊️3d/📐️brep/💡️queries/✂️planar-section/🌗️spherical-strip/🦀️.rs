//! 🌗️ Sections the exact circular profile of a sphere between axial closure planes.

use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::f64::consts::TAU;

type CircularStrip = (Frame3,f64,[Option<(f64,f64)>;2]);

/// ✂️ Returns a full circle, one arc and chord, or two arcs and chords in cyclic order.
pub fn spherical_strip_section(
    sphere:Frame3,sphere_radius:f64,[low,high]:[f64;2],origin:[f64;3],normal:[f64;3],tolerance:f64,
)->Result<CircularStrip,&'static str>{
    let invalid="section plane and spherical strip must be finite and nondegenerate";
    let miss="section has no nondegenerate material contour";
    if !origin.into_iter().chain(normal).all(f64::is_finite) { return Err(invalid); }
    let scale=normal.into_iter().map(f64::abs).fold(0.,f64::max);
    let normal=Vec3::from_array(normal.map(|x|x/scale)).normalized().ok_or(invalid)?;
    let offset=(sphere.origin-Pnt3::from_array(origin)).dot(normal);
    let clearance=sphere_radius-offset.abs();
    if clearance<=tolerance { return Err(miss); }
    let radius=(clearance*(sphere_radius+offset.abs())).sqrt();
    let center=sphere.origin-normal*offset;
    if !radius.is_finite()||radius<=tolerance||!center.to_array().into_iter().all(|x|(x.abs()+radius).is_finite()){
        return Err(invalid);
    }
    let mid=(center-sphere.origin).dot(sphere.z);
    let transverse=sphere.z-normal*sphere.z.dot(normal);
    let slope=transverse.norm();
    let extent=slope*radius;
    if mid+extent<=low+tolerance||mid-extent>=high-tolerance { return Err(miss); }
    let (below,above)=(mid-extent<low-tolerance,mid+extent>high+tolerance);
    if !below&&!above {
        let basis=Frame3::from_normal(center,normal).ok_or(invalid)?;
        return Ok((basis,radius,[None;2]));
    }
    let mut x=transverse.normalized().ok_or(invalid)?;
    let angle=|bound:f64|((bound-mid)/extent).clamp(-1.,1.).acos();
    let arcs=match (below,above) {
        (true,true)=>{
            let [start,end]=[angle(high),angle(low)];
            [Some((start,end)),Some((TAU-end,TAU-start))]
        }
        (true,false)=>{
            x=-x;
            let start=((mid-low)/extent).clamp(-1.,1.).acos();
            [Some((start,TAU-start)),None]
        }
        (false,true)=>{
            let start=angle(high);
            [Some((start,TAU-start)),None]
        }
        (false,false)=>unreachable!(),
    };
    let basis=Frame3{origin:center,x,y:normal.cross(x),z:normal};
    Ok((basis,radius,arcs))
}
