//! 🧭️ Exact finite support distances for revolved solid boundaries.

use semio_framework_3d_brep_vector::{Pnt3,matrix::Frame3};
use std::f64::consts::TAU;

/// 🪣️ Measures the two terminal disks and conical flank of a closed finite frustum.
pub fn frustum_signed_distance(frame:Frame3,radii:[f64;2],height:f64,point:Pnt3)->f64{
    let [x,y,z]=frame.to_local(point).to_array();
    if ![x,y,z].into_iter().all(f64::is_finite) {return f64::INFINITY;}
    let radial=x.hypot(y);
    let [a,b]=radii;
    let slope=b-a;
    let length=height.hypot(slope);
    let [uz,ur]=[height/length,slope/length];
    let offset=radial-a;
    let along=(z*uz+offset*ur).clamp(0.,length);
    let side=(z-uz*along).hypot(offset-ur*along);
    let disk=|r:f64,level:f64|plane_distance(radial,r,0.,0.,z,level);
    let distance=side.min(disk(a,0.)).min(disk(b,height));
    signed(distance,z>=0.&&z<=height&&offset<=(z/height)*slope)
}

pub(super) fn inside_sweep(x:f64,y:f64,sweep:f64)->bool { y.atan2(x).rem_euclid(TAU)<=sweep }

pub(super) fn signed(distance:f64,inside:bool)->f64 { if inside{-distance}else{distance} }

pub(super) fn cap_coordinates(x:f64,y:f64,angle:f64)->[f64;2]{
    let (s,c)=angle.sin_cos();
    [x*c+y*s,y*c-x*s]
}

pub(super) fn plane_distance(radial:f64,radius:f64,hole_rho:f64,hole:f64,z:f64,level:f64)->f64{
    (z-level).hypot((radial-radius).max(hole-hole_rho).max(0.))
}

pub(super) fn wall_distance(radial:f64,radius:f64,z:f64,low:f64,high:f64)->f64{
    (radial-radius).hypot(z-z.clamp(low,high))
}
