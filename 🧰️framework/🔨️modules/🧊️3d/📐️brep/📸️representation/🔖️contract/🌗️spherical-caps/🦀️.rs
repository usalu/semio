//! 🌗️ Exact finite caps on spherical and planar supports.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::f64::consts::{FRAC_PI_2,PI,TAU};

#[path = "📐️measure/🦀️.rs"]
mod measure;

#[path = "📍️classification/🦀️.rs"]
mod classification;

/// 🔗️ A signed spherical boundary with a second sphere or plane joined along one common rim.
#[derive(Clone,Copy)]
pub struct SphericalCaps {
    frame:Frame3,
    other:Pnt3,
    offsets:[f64;2],
    radii:[f64;2],
    rim:f64,
    positive:[bool;2],
    inward:bool,
    closure:Closure,
}

#[derive(Clone,Copy)]
enum Closure {
    Sphere,
    Disk,
    Band{offset:f64,radius:f64},
}

impl SphericalCaps {
    pub fn new(a:Pnt3,b:Pnt3,radii:[f64;2],positive:[bool;2],inward:bool,tol:f64)->Option<Self>{
        let [r,s]=radii;
        let axis=b-a;
        let distance=axis.norm();
        if !(r>tol&&s>tol&&distance.is_finite()&&distance-(r-s).abs()>tol&&r+s-distance>tol)
            || (positive[0]==positive[1])!=inward{return None;}
        let split=(distance+(r-s)*(r+s)/distance)*0.5;
        let offsets=[split,split-distance];
        let radial=|radius:f64,at:f64|((radius-at.abs())*(radius+at.abs())).sqrt();
        let rim=radial(r,offsets[0]);
        if !(rim>tol&&(radial(s,offsets[1])-rim).abs()<=tol){return None;}
        let frame=Frame3::from_normal(a,axis)?;
        let caps=Self{frame,other:b,offsets,radii,rim,positive,inward,closure:Closure::Sphere};
        let center_a=caps.rim_frame().origin;
        let center_b=b+frame.z*offsets[1];
        if !((center_a-center_b).norm()<=tol){return None;}
        for angle in [0.,FRAC_PI_2,PI,3.*FRAC_PI_2]{
            let point=caps.rim_point(angle);
            if !(((point-a).norm()-r).abs()<=tol&&((point-b).norm()-s).abs()<=tol){return None;}
        }
        let volume=caps.volume();
        (volume.is_finite()&&volume>0.&&(0..3).all(|i|caps.vertex(i).to_array().into_iter().all(f64::is_finite))).then_some(caps)
    }

    /// ✂️ Retains the positive side of a sphere clipped by one transverse plane.
    pub fn from_plane(frame:Frame3,radius:f64,offset:f64,tol:f64)->Option<Self>{
        let clearance=radius-offset.abs();
        let rim=(clearance*(radius+offset.abs())).sqrt();
        if !(clearance>tol&&rim.is_finite()&&rim>tol){return None;}
        let cap=Self{
            frame,other:frame.origin,offsets:[offset,0.],radii:[radius,0.],
            rim,positive:[true,false],inward:false,closure:Closure::Disk,
        };
        let valid=[0.,FRAC_PI_2,PI,3.*FRAC_PI_2].into_iter()
            .all(|angle|((cap.rim_point(angle)-frame.origin).norm()-radius).abs()<=tol);
        let volume=cap.volume();
        (valid&&volume.is_finite()&&volume>0.&&cap.area().is_finite()
            &&(cap.vertex(1)-cap.rim_frame().origin).dot(frame.z)>tol).then_some(cap)
    }

    /// 🪟️ Retains the spherical band between two parallel transverse planes.
    pub fn from_band(frame:Frame3,radius:f64,span:[f64;2],tol:f64)->Option<Self>{
        let [low,high]=span;
        let clearance=radius-high.abs();
        let rim=(clearance*(radius+high.abs())).sqrt();
        if !(high-low>tol&&clearance>tol&&rim.is_finite()&&rim>tol){return None;}
        let mut band=Self::from_plane(frame,radius,low,tol)?;
        band.closure=Closure::Band{offset:high,radius:rim};
        let valid=[0.,FRAC_PI_2,PI,3.*FRAC_PI_2].into_iter()
            .all(|angle|((band.rim_point_at(true,angle)-frame.origin).norm()-radius).abs()<=tol);
        let height=(band.rim_frame_at(true).origin-band.rim_frame().origin).dot(frame.z);
        let volume=band.volume();
        (valid&&height>tol&&volume.is_finite()&&volume>0.&&band.area().is_finite()).then_some(band)
    }

    pub fn is_band(self)->bool{matches!(self.closure,Closure::Band{..})}
    pub fn is_disk(self,face:usize)->bool{!matches!(self.closure,Closure::Sphere)&&(face==1||face==2&&self.is_band())}
    pub fn vertex_count(self)->usize{if self.is_disk(1){2}else{3}}
    pub fn edge_count(self)->usize{if self.is_disk(1){3}else{5}}
    pub fn ring_sizes(self)->&'static [usize]{if self.is_band(){&[4,1,1]}else if self.is_disk(1){&[4,1]}else{&[4,4]}}
    pub fn rim_radius(self,upper:bool)->f64{
        match self.closure{Closure::Band{radius,..} if upper=>radius,_=>self.rim}
    }

    pub fn radius(self,face:usize)->f64{self.radii[face]}
    pub fn is_inward(self,face:usize)->bool{face==1&&self.inward}

    fn heights(self) -> [f64; 2] {
        std::array::from_fn(|index| {
            let signed_offset = if self.positive[index] { self.offsets[index] } else { -self.offsets[index] };
            let radius = self.radii[index];
            if signed_offset > 0.0 {
                (self.rim * 0.5) * (self.rim / (radius * 0.5 + signed_offset * 0.5))
            } else { radius - signed_offset }
        })
    }

    /// 🪣️ Returns the material volume with the inward cap subtracted.
    pub fn volume(self) -> f64 {
        if let Closure::Band{offset,radius}=self.closure{
            return measure::band(self.radii[0],[self.offsets[0],offset],[self.rim,radius]).0;
        }
        measure::volume(self.rim,self.heights(),self.inward)
    }

    /// 📏️ Includes both boundary faces, including the inward cap of a cut.
    pub fn area(self) -> f64 {
        if let Closure::Band{offset,radius}=self.closure{
            return measure::band(self.radii[0],[self.offsets[0],offset],[self.rim,radius]).1;
        }
        let [a, b] = self.heights();
        TAU * self.radii[0].mul_add(a, self.radii[1] * b)+if self.is_disk(1){PI*self.rim*self.rim}else{0.}
    }

    /// 🧲️ Returns the world centroid with moments measured from the shared rim.
    pub fn center_of_mass(self) -> Pnt3 {
        if let Closure::Band{offset,radius}=self.closure{
            return self.frame.origin+self.frame.z*measure::band(self.radii[0],[self.offsets[0],offset],[self.rim,radius]).2;
        }
        self.rim_frame().origin + self.frame.z * measure::centroid(self.rim, self.heights(), self.positive, self.inward)
    }

    pub fn frame(self,face:usize)->Frame3{
        if face==0{self.frame}else if face==2&&self.is_band(){self.rim_frame_at(true)}else if self.is_disk(face){
            let rim=self.rim_frame();
            Frame3{y:-rim.y,z:-rim.z,..rim}
        }else{Frame3{origin:self.other,..self.frame}}
    }

    pub fn rim_frame(self)->Frame3{
        Frame3{origin:self.frame.origin+self.frame.z*self.offsets[0],..self.frame}
    }

    fn rim_frame_at(self,upper:bool)->Frame3{
        match self.closure{
            Closure::Band{offset,..} if upper=>Frame3{origin:self.frame.origin+self.frame.z*offset,..self.frame},
            _=>self.rim_frame(),
        }
    }

    pub fn rim_point(self,u:f64)->Pnt3{self.rim_point_at(false,u)}

    pub fn rim_point_at(self,upper:bool,u:f64)->Pnt3{
        self.rim_frame_at(upper).origin+(self.frame.x*u.cos()+self.frame.y*u.sin())*self.rim_radius(upper)
    }

    pub fn band(self,face:usize)->(f64,f64,bool,bool){
        let latitude=self.offsets[face].atan2(self.rim);
        if let Closure::Band{offset,radius}=self.closure{return (latitude,offset.atan2(radius),false,false);}
        if self.positive[face]{(latitude,FRAC_PI_2,false,true)}else{(-FRAC_PI_2,latitude,true,false)}
    }

    pub fn vertex(self,index:usize)->Pnt3{
        if index==0||self.is_band(){return self.rim_point_at(index==1,0.);}
        let face=index-1;
        self.frame(face).origin+self.frame.z*(if self.positive[face]{1.}else{-1.})*self.radii[face]
    }

    pub fn edge(self,index:usize)->(Curve3,(f64,f64),[usize;2]){
        if index==0||index==2&&self.is_band(){
            let upper=index==2;
            let vertex=usize::from(upper);
            return (Curve3::Circle{frame:self.rim_frame_at(upper),radius:self.rim_radius(upper)},(0.,TAU),[vertex;2]);
        }
        let face=(index-1)/2;
        let pole=face+1;
        if index%2==0{return (Curve3::Line{origin:self.vertex(pole),dir:Vec3::ZERO},(0.,TAU),[pole,pole]);}
        let frame=self.frame(face);
        let (a,b,low_pole,_)=self.band(face);
        let meridian=Frame3{y:frame.z,z:-frame.y,..frame};
        (Curve3::Circle{frame:meridian,radius:self.radii[face]},(a,b),if low_pole{[pole,0]}else{[0,pole]})
    }
}
