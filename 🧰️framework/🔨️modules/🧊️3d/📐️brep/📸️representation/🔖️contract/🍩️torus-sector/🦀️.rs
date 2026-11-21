//! 🍩️ Exact bounded portion of a ring torus swept over a positive angular domain.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::f64::consts::{PI,TAU};

/// 🌀️ Captures the frame, two radii and finite sweep shared by native topology and preview.
#[derive(Clone,Copy)]
pub struct TorusSector {
    pub frame:Frame3,
    pub major:f64,
    pub minor:f64,
    pub sweep:f64,
}

impl TorusSector {
    /// 🎟️ Checks that both meridian rims remain distinct at kernel tolerance.
    pub fn new(frame:Frame3,major:f64,minor:f64,sweep:f64,tolerance:f64)->Option<Self>{
        if ![major,minor,sweep].into_iter().all(f64::is_finite)||minor<=tolerance
            || major-minor<=tolerance||sweep<=0.||sweep>=TAU{return None;}
        let sector=Self{frame,major,minor,sweep};
        let clearance=2.*(major-minor)*(sweep*0.5).sin();
        let separation=sector.vertex(0).distance(sector.vertex(1));
        (clearance>tolerance&&separation>tolerance&&sector.volume().is_finite()).then_some(sector)
    }

    pub fn radial(self,u:f64)->Vec3{self.frame.x*u.cos()+self.frame.y*u.sin()}

    pub fn rim(self,index:usize)->Frame3{
        let x=self.radial(if index==0{0.}else{self.sweep});
        let y=self.frame.z;
        Frame3{origin:self.frame.origin+x*self.major,x,y,z:x.cross(y)}
    }

    pub fn vertex(self,index:usize)->Pnt3{
        let rim=self.rim(index);
        rim.origin+rim.x*self.minor
    }

    /// 〰️ Returns the original analytic rim or seam curve with its edge range and incidence.
    pub fn edge(self,index:usize)->(Curve3,(f64,f64),[usize;2]){
        if index<2{
            (Curve3::Circle{frame:self.rim(index),radius:self.minor},(0.,TAU),[index;2])
        }else{
            (Curve3::Circle{frame:self.frame,radius:self.major+self.minor},(0.,self.sweep),[0,1])
        }
    }

    pub fn point(self,u:f64,v:f64)->Pnt3{
        self.frame.origin+self.radial(u)*(self.major+self.minor*v.cos())+self.frame.z*(self.minor*v.sin())
    }

    pub fn normal(self,u:f64,v:f64)->Vec3{self.radial(u)*v.cos()+self.frame.z*v.sin()}

    /// 📍️ Measures closed distance to the torus flank and both finite meridian disks.
    pub fn signed_boundary_distance(self,point:Pnt3)->f64{
        use super::revolution_distance::{cap_coordinates,inside_sweep,signed};
        let [x,y,z]=self.frame.to_local(point).to_array();
        if ![x,y,z].into_iter().all(f64::is_finite) {return f64::INFINITY;}
        let angle_inside=inside_sweep(x,y,self.sweep);
        let flank=(x.hypot(y)-self.major).hypot(z)-self.minor;
        let cap=|angle|{
            let [radial,perp]=cap_coordinates(x,y,angle);
            perp.hypot(((radial-self.major).hypot(z)-self.minor).max(0.))
        };
        let distance=cap(0.).min(cap(self.sweep));
        signed(if angle_inside{distance.min(flank.abs())}else{distance},angle_inside&&flank<=0.)
    }

    pub fn volume(self)->f64{PI*self.minor*self.minor*self.major*self.sweep}
}
