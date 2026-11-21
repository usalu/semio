//! ✂️ Intersects a round vertical extrusion with a rectangular vertical extrusion.

use super::CircularPrism;
use semio_framework_3d_brep_construction::circular_prism::CircleSupports;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Vec3};
use std::f64::consts::TAU;

#[derive(Clone,Copy,Default)]
struct Member {start:[f64;2],end:[f64;2],sweep:f64}

fn distance(a:[f64;2],b:[f64;2])->f64{(a[0]-b[0]).hypot(a[1]-b[1])}

fn circle(angle:f64,radius:f64)->[f64;2]{
    let (sin,cos)=angle.sin_cos();
    [radius*cos,radius*sin]
}

/// 🧮️ Certifies one mixed disk/rectangle footprint over an overlapping axial interval.
pub(super) fn clip_disk(center:[f64;2],radius:f64,rectangle:[[f64;2];2],span:[f64;2])->Option<CircularPrism>{
    let tol=Tol::DEFAULT.value();
    let [lo,hi]=rectangle.map(|p|[p[0]-center[0],p[1]-center[1]]);
    let [bottom,top]=span;
    let length=top-bottom;
    if !lo.into_iter().chain(hi).chain(span).chain([radius]).all(f64::is_finite)
        || radius<=tol || length<=tol || (0..2).any(|axis|hi[axis]-lo[axis]<=tol){return None;}
    let contains=|p:[f64;2]|(0..2).all(|axis|lo[axis]<=p[axis]&&p[axis]<=hi[axis]);
    let mut members=[Member::default();8];
    let mut n=0;
    let mut angles=[0.;8];
    let mut angle_count=0;
    for (fixed,offset,reverse) in [(1,lo[1],false),(0,hi[0],false),(1,hi[1],true),(0,lo[0],true)]{
        if offset.abs()>=radius{continue;}
        let other=1-fixed;
        let root=((radius-offset.abs())*(radius+offset.abs())).sqrt();
        let low=lo[other].max(-root);
        let high=hi[other].min(root);
        if high<=low{continue;}
        if high-low<=tol{return None;}
        let at=|value|if fixed==0{[offset,value]}else{[value,offset]};
        let [mut start,mut end]=[at(low),at(high)];
        if reverse{std::mem::swap(&mut start,&mut end);}
        members[n]=Member{start,end,sweep:0.};
        n+=1;
        for p in [at(-root),at(root)]{
            if !contains(p){continue;}
            let angle=p[1].atan2(p[0]).rem_euclid(TAU);
            if angles[..angle_count].iter().any(|&existing|distance(circle(existing,radius),p)<=tol){continue;}
            angles[angle_count]=angle;
            angle_count+=1;
        }
    }
    angles[..angle_count].sort_unstable_by(f64::total_cmp);
    let straight=n;
    for i in 0..angle_count{
        let start=angles[i];
        let sweep=(angles[(i+1)%angle_count]-start).rem_euclid(TAU);
        if sweep==0.||!contains(circle(start+sweep*0.5,radius)){continue;}
        if n==members.len(){return None;}
        members[n]=Member{start:circle(start,radius),end:circle(start+sweep,radius),sweep};
        n+=1;
    }
    if straight==0||straight==n{return None;}
    let mut area=0.;
    let mut interior=[0.;2];
    let mut interior_count=0_u32;
    let mut corners=[Pnt3::new(0.,0.,0.);8];
    let mut arcs=[0.;8];
    let world=|p:[f64;2]|Pnt3::new(p[0]+center[0],p[1]+center[1],bottom);
    for i in 0..n{
        let Member{start,end,sweep}=members[i];
        if i+1<n{
            let next=(i+1..n).find(|&next|distance(members[next].start,end)<=tol)?;
            members.swap(i+1,next);
        }else if distance(members[0].start,end)>tol{return None;}
        corners[i]=world(start);
        arcs[i]=sweep;
        for axis in 0..2{interior[axis]+=start[axis];}
        interior_count+=1;
        if sweep==0.{
            area+=start[0]*end[1]-start[1]*end[0];
        }else{
            area+=radius*radius*sweep;
            let midpoint=circle(start[1].atan2(start[0])+sweep*0.5,radius);
            for axis in 0..2{interior[axis]+=midpoint[axis];}
            interior_count+=1;
        }
    }
    let interior=interior.map(|value|value/f64::from(interior_count));
    let volume=0.5*area*length;
    let supports=CircleSupports::Shared{radius,center:Some(world([0.,0.]))};
    CircularPrism::from_profile(corners,arcs,n,world(interior),Vec3::Z,supports,length,volume)
}
