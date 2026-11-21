//! 🔵️ Certified material shells with an enclosed spherical cavity.

use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::f64::consts::PI;

/// 🧊️ An exact primitive boundary usable by either side of a spherical cavity.
#[derive(Clone,Copy)]
pub enum SphericalShellBoundary {
    Sphere{frame:Frame3,radius:f64},
    Box{origin:Pnt3,size:[f64;3]},
}

/// 🐚️ The material exterior and authored inner sphere of one closed solid.
#[derive(Clone,Copy)]
pub struct SphericalShell {
    exterior:SphericalShellBoundary,
    inner_frame:Frame3,
    inner:f64,
}

fn representable(frame:Frame3,radius:f64)->bool{
    frame.origin.to_array().into_iter().all(|x|
        (x.abs()+radius).is_finite()&&x+radius>x&&x-radius<x)
}

impl SphericalShell {
    pub fn new(frame:Frame3,outer:f64,thickness:f64,tolerance:f64)->Option<Self>{
        Self::from_spheres([frame;2],[outer,outer-thickness],tolerance)
    }

    /// 🥣️ Keeps both authored supports when the inner sphere has strict radial clearance.
    pub fn from_spheres(frames:[Frame3;2],radii:[f64;2],tolerance:f64)->Option<Self>{
        let [frame,inner_frame]=frames;
        let [outer,inner]=radii;
        let distance=(frame.origin-inner_frame.origin).norm();
        if !(inner>tolerance&&outer.is_finite()&&outer-inner-distance>tolerance){return None;}
        let shell=Self{exterior:SphericalShellBoundary::Sphere{frame,radius:outer},inner_frame,inner};
        (frames.into_iter().zip(radii).all(|(f,r)|representable(f,r))&&shell.volume().is_finite()).then_some(shell)
    }

    /// 🧺️ Keeps a spherical tool with positive clearance to all six stock planes.
    pub fn from_box(origin:Pnt3,size:[f64;3],inner_frame:Frame3,inner:f64,tolerance:f64)->Option<Self>{
        let lo=origin.to_array();
        let center=inner_frame.origin.to_array();
        let valid=(0..3).all(|i|{
            let hi=lo[i]+size[i];
            hi.is_finite()&&(center[i]-lo[i]).min(hi-center[i])-inner>tolerance
        });
        if !(inner>tolerance&&valid&&representable(inner_frame,inner)){return None;}
        let shell=Self{exterior:SphericalShellBoundary::Box{origin,size},inner_frame,inner};
        let volume=shell.volume();
        (volume.is_finite()&&volume>0.&&shell.area().is_finite()).then_some(shell)
    }

    /// 📦️ Returns the exact independently oriented exterior or cavity boundary.
    pub fn boundary(self,inside:bool)->SphericalShellBoundary{
        if inside{SphericalShellBoundary::Sphere{frame:self.inner_frame,radius:self.inner}}else{self.exterior}
    }

    fn cavity_volume(self)->f64{(4.*PI/3.)*self.inner*self.inner*self.inner}

    pub fn volume(self)->f64{
        let inner=self.inner;
        match self.exterior{
            SphericalShellBoundary::Sphere{radius:outer,..}=>
                (4.*PI/3.)*(outer-inner)*(outer*outer+outer*inner+inner*inner),
            SphericalShellBoundary::Box{size:[w,d,h],..}=>w*d*h-self.cavity_volume(),
        }
    }

    /// 📐️ Returns the area of both material boundaries.
    pub fn area(self)->f64{
        let outer=match self.exterior{
            SphericalShellBoundary::Sphere{radius,..}=>4.*PI*radius*radius,
            SphericalShellBoundary::Box{size:[w,d,h],..}=>2.*(w*d+d*h+h*w),
        };
        outer+4.*PI*self.inner*self.inner
    }

    /// ⚖️ Subtracts the void's moment in the local outer center frame.
    pub fn center_of_mass(self)->Pnt3{
        let (center,weight)=match self.exterior{
            SphericalShellBoundary::Sphere{frame,radius}=>{
                let ratio=self.inner/radius;
                let gap=(radius-self.inner)/radius;
                (frame.origin,ratio*ratio*ratio/(gap*(1.+ratio+ratio*ratio)))
            }
            SphericalShellBoundary::Box{origin,size}=>{
                (origin+Vec3::from_array(size.map(|x|x*0.5)), self.cavity_volume()/self.volume())
            }
        };
        center+(center-self.inner_frame.origin)*weight
    }
}
