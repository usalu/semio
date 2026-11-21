//! 🧿️ Shared exact coordinates of a revolved meridional rectangle.

use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
use std::f64::consts::TAU;

/// 🛞️ A full hollow cylinder or bounded annular sector about one native frame.
#[derive(Clone,Copy)]
pub struct AnnularRevolution {
    pub frame:Frame3,
    pub inner:f64,
    pub outer:f64,
    pub height:f64,
    pub sweep:f64,
    floor:f64,
    ceiling:f64,
    shoulder:f64,
    lower_shoulder:f64,
    eccentricity:f64,
}

impl AnnularRevolution {
    pub fn new(frame:Frame3,inner:f64,outer:f64,height:f64,sweep:f64,tolerance:f64)->Option<Self>{
        if ![inner,outer,height,sweep].into_iter().all(f64::is_finite)||inner<=tolerance
            || outer-inner<=tolerance||height<=tolerance||sweep<=0.||sweep>TAU{return None;}
        let shape=Self{frame,inner,outer,height,sweep,floor:0.,ceiling:height,shoulder:height,lower_shoulder:0.,eccentricity:0.};
        if !shape.full()&&2.*inner*(sweep*0.5).sin()<=tolerance{return None;}
        let volume=shape.volume();
        (volume.is_finite()&&volume>0.&&(0..shape.vertex_count()).all(|index|
            shape.vertex(index).to_array().into_iter().all(f64::is_finite))).then_some(shape)
    }

    /// 🕳️ Admits a blind coaxial cut opening at the positive end of this frame.
    pub fn new_blind(frame:Frame3,inner:f64,outer:f64,height:f64,floor:f64,tolerance:f64)->Option<Self>{
        if !floor.is_finite()||floor<=tolerance||height-floor<=tolerance{return None;}
        let shape=Self{floor,..Self::new(frame,inner,outer,height,TAU,tolerance)?};
        (shape.volume().is_finite()&&shape.vertex(0).to_array().into_iter().all(f64::is_finite)).then_some(shape)
    }

    /// 🫧️ Admits a complete cylindrical void bounded by two interior caps.
    pub fn new_cavity(frame:Frame3,inner:f64,outer:f64,height:f64,levels:[f64;2],tolerance:f64)->Option<Self>{
        let [floor,ceiling]=levels;
        if !ceiling.is_finite()||height-ceiling<=tolerance||ceiling-floor<=tolerance{return None;}
        let shape=Self{ceiling,..Self::new_blind(frame,inner,outer,height,floor,tolerance)?};
        (shape.volume().is_finite()&&shape.vertex(3).to_array().into_iter().all(f64::is_finite)).then_some(shape)
    }

    /// 🔩️ Clips an axially parallel tool to the stock and certifies its cap topology.
    pub fn from_axial_cut(frame:Frame3,inner:f64,outer:f64,height:f64,[lo,hi]:[f64;2],tol:f64)->Option<Self>{
        if lo<=0.&&hi>=height{return Self::new(frame,inner,outer,height,TAU,tol);}
        if lo>0.&&hi>=height{return Self::new_blind(frame,inner,outer,height,lo,tol);}
        if lo<=0.&&hi<height{
            let reversed=Frame3{origin:frame.origin+frame.z*height,y:-frame.y,z:-frame.z,..frame};
            return Self::new_blind(reversed,inner,outer,height,height-hi,tol);
        }
        Self::new_cavity(frame,inner,outer,height,[lo,hi],tol)
    }

    /// 🧭️ Preserves the narrow footprint axis and aligns the paired ring seam to its offset.
    pub fn with_offset(self,offset:Vec3,tol:f64)->Option<Self>{
        if !self.full(){return None;}
        let distance=offset.norm();
        if !distance.is_finite()||self.outer-self.inner-distance<=tol{return None;}
        if distance==0.{return Some(self);}
        let y=self.frame.z.cross(offset).normalized()?;
        let x=y.cross(self.frame.z);
        if (x*distance-offset).norm()>tol{return None;}
        let shape=Self{frame:Frame3{x,y,..self.frame},eccentricity:distance,..self};
        if (shape.inner_frame().origin-self.frame.origin-offset).norm()>tol{return None;}
        let valid=(0..shape.vertex_count()).all(|index|[0.,TAU*0.25,TAU*0.5,TAU*0.75].into_iter().all(|u|{
            let radial=shape.point(index,u)-shape.center(index);
            (radial.norm()-shape.radius(index)).abs()<=tol
        }));
        valid.then_some(shape)
    }

    pub fn inner_frame(self)->Frame3{
        Frame3{origin:self.frame.origin+self.frame.x*self.eccentricity,..self.frame}
    }

    /// 🪜️ Admits two coaxial cylinders connected across one exact radial shoulder.
    pub fn new_step(frame:Frame3,inner:f64,outer:f64,height:f64,shoulder:f64,tolerance:f64)->Option<Self>{
        if !shoulder.is_finite()||shoulder<=tolerance||height-shoulder<=tolerance{return None;}
        let shape=Self{shoulder,floor:shoulder,..Self::new(frame,inner,outer,height,TAU,tolerance)?};
        (shape.volume().is_finite()&&(0..shape.vertex_count()).all(|i|shape.vertex(i).to_array().into_iter().all(f64::is_finite))).then_some(shape)
    }

    /// 🪜️ Admits a wide middle cylinder between two narrower coaxial ends.
    pub fn new_two_steps(frame:Frame3,inner:f64,outer:f64,height:f64,shoulders:[f64;2],tolerance:f64)->Option<Self>{
        let [lower_shoulder,upper]=shoulders;
        if !lower_shoulder.is_finite()||lower_shoulder<=tolerance||upper-lower_shoulder<=tolerance{return None;}
        let shape=Self{lower_shoulder,..Self::new_step(frame,inner,outer,height,upper,tolerance)?};
        (shape.volume().is_finite()&&(0..shape.vertex_count()).all(|i|shape.vertex(i).to_array().into_iter().all(f64::is_finite))).then_some(shape)
    }

    /// 🔗️ Keeps a broad axial cylinder and the projecting ends of a nested narrow footprint.
    pub fn from_axial_union(frame:Frame3,inner:f64,outer:f64,height:f64,[lo,hi]:[f64;2],tol:f64)->Option<Self>{
        if lo<0.&&hi>height{
            let start=Frame3{origin:frame.origin+frame.z*lo,..frame};
            return Self::new_two_steps(start,inner,outer,hi-lo,[-lo,height-lo],tol);
        }
        if lo>=0.&&lo<=height&&hi>height{return Self::new_step(frame,inner,outer,hi,height,tol);}
        if lo<0.&&hi>=0.&&hi<=height{
            let reversed=Frame3{origin:frame.origin+frame.z*height,y:-frame.y,z:-frame.z,..frame};
            return Self::new_step(reversed,inner,outer,height-lo,height,tol);
        }
        None
    }

    pub fn has_two_steps(self)->bool{self.lower_shoulder>0.}

    pub fn has_step(self)->bool{self.shoulder<self.height}

    pub fn ceiling(self)->f64{self.ceiling}
    pub fn has_void_shell(self)->bool{self.ceiling<self.height}

    pub fn floor(self)->f64{self.floor}
    pub fn has_floor(self)->bool{self.floor>0.&&!self.has_step()}
    pub fn full(self)->bool{self.sweep==TAU}
    pub fn face_count(self)->usize{if self.has_two_steps(){7}else if self.full(){4+usize::from(self.has_floor()||self.has_step())+usize::from(self.has_void_shell())}else{6}}
    pub fn vertex_count(self)->usize{if self.has_two_steps(){6}else if self.full(){4}else{8}}
    pub fn edge_count(self)->usize{if self.has_two_steps(){9}else if self.full(){6}else{12}}
    pub fn edge_is_arc(self,index:usize)->bool{if self.full(){index<self.vertex_count()}else{index>=8}}
    pub fn volume(self)->f64{
        if self.has_step(){
            let broad=self.shoulder-self.lower_shoulder;
            return std::f64::consts::PI*(self.outer*self.outer*broad+self.inner*self.inner*(self.height-broad));
        }
        self.sweep*0.5*((self.outer-self.inner)*(self.outer+self.inner)*self.height+self.inner*self.inner*(self.floor+(self.height-self.ceiling)))}
    /// 🗜️ Measures cylindrical walls, planar caps and any meridional end faces.
    pub fn area(self)->f64{
        let gap=self.outer-self.inner;
        if self.has_step(){
            let broad=self.shoulder-self.lower_shoulder;
            return TAU*(self.outer*broad+self.inner*(self.height-broad)+self.outer*self.outer);
        }
        let rings=gap*(self.outer+self.inner)
            +f64::from(u8::from(self.has_floor())+u8::from(self.has_void_shell()))*self.inner*self.inner;
        let walls=self.outer*self.height+self.inner*(self.ceiling-self.floor);
        let ends=if self.full(){0.}else{2.*self.height*gap};
        self.sweep*(walls+rings)+ends
    }

    /// 🕹️ Integrates the axial and radial first moments of the material profile.
    pub fn center_of_mass(self)->Pnt3{
        let ratio=self.inner/self.outer;
        let inner=ratio*ratio;
        let gap=((self.outer-self.inner)/self.outer)*(1.+ratio);
        let mid=self.height*0.5;
        let center=self.frame.origin+self.frame.z*mid;
        if self.has_step(){
            let broad=self.shoulder-self.lower_shoulder;
            let extra=gap*broad;
            let total=inner*self.height+extra;
            let offset=self.lower_shoulder+broad*0.5-mid;
            let lateral=self.eccentricity*(inner*(self.height-broad)/total);
            return center+self.frame.z*(offset*(extra/total))+self.frame.x*lateral;
        }
        if !self.full(){
            let half=self.sweep*0.5;
            let radial=(2./3.)*self.outer*((1.+ratio+inner)/(1.+ratio))*(half.sin()/half);
            return center+self.radial(half)*radial;
        }
        let bore=self.ceiling-self.floor;
        let weight=inner*bore/(gap*self.height+inner*(self.height-bore));
        let offset=mid-(self.floor+bore*0.5);
        center+(self.frame.z*offset-self.frame.x*self.eccentricity)*weight
    }

    /// 📍️ Measures the closed material boundary, including any shoulder, opening or cavity.
    pub fn signed_boundary_distance(self,point:Pnt3)->f64{
        use super::revolution_distance::{cap_coordinates,inside_sweep,plane_distance,signed,wall_distance};
        let [x,y,z]=self.frame.to_local(point).to_array();
        if ![x,y,z].into_iter().all(f64::is_finite) {return f64::INFINITY;}
        let radial=x.hypot(y);
        let in_z=z>=0.&&z<=self.height;
        if !self.full(){
            let inside_angle=inside_sweep(x,y,self.sweep);
            let cap=|angle|{
                let [r,perp]=cap_coordinates(x,y,angle);
                (r-r.clamp(self.inner,self.outer)).hypot(z-z.clamp(0.,self.height)).hypot(perp)
            };
            let mut distance=cap(0.).min(cap(self.sweep));
            if inside_angle {
                let end=|level:f64|(z-level).hypot(radial-radial.clamp(self.inner,self.outer));
                distance=distance.min(end(0.)).min(end(self.height))
                    .min(wall_distance(radial,self.inner,z,0.,self.height)).min(wall_distance(radial,self.outer,z,0.,self.height));
            }
            return signed(distance,inside_angle&&in_z&&radial>=self.inner&&radial<=self.outer);
        }
        let narrow=(x-self.eccentricity).hypot(y);
        let large_end=|level,hole|plane_distance(radial,self.outer,narrow,hole,z,level);
        let small_end=|level|plane_distance(narrow,self.inner,0.,0.,z,level);
        if self.has_step(){
            let [low,high]=[self.lower_shoulder,self.shoulder];
            let mut distance=wall_distance(radial,self.outer,z,low,high)
                .min(wall_distance(narrow,self.inner,z,high,self.height))
                .min(large_end(high,self.inner)).min(small_end(self.height));
            distance=if low==0. {distance.min(large_end(0.,0.))}
                else {distance.min(large_end(low,self.inner)).min(small_end(0.)).min(wall_distance(narrow,self.inner,z,0.,low))};
            let inside=in_z&&if z>=low&&z<=high{radial<=self.outer}else{narrow<=self.inner};
            return signed(distance,inside);
        }
        let mut distance=wall_distance(radial,self.outer,z,0.,self.height)
            .min(wall_distance(narrow,self.inner,z,self.floor,self.ceiling));
        distance=distance.min(large_end(0.,if self.floor==0.{self.inner}else{0.}))
            .min(large_end(self.height,if self.ceiling==self.height{self.inner}else{0.}));
        if self.floor>0.{distance=distance.min(small_end(self.floor));}
        if self.ceiling<self.height{distance=distance.min(small_end(self.ceiling));}
        let void=narrow<self.inner&&z>=self.floor&&z<=self.ceiling;
        signed(distance,in_z&&radial<=self.outer&&!void)
    }

    pub fn radial(self,u:f64)->Vec3{self.frame.x*u.cos()+self.frame.y*u.sin()}

    pub fn radius(self,corner:usize)->f64{
        let narrow=if self.has_two_steps(){!matches!(corner,2|3)}else{matches!(corner,0|3)};
        if narrow{self.inner}else{self.outer}
    }
    pub fn level(self,corner:usize)->f64{
        if self.has_two_steps(){return [0.,self.lower_shoulder,self.lower_shoulder,self.shoulder,self.shoulder,self.height][corner];}
        match corner{0=>self.floor,1=>0.,2=>self.shoulder,_=>self.ceiling}
    }

    pub fn center(self,corner:usize)->Pnt3{
        let base=if self.eccentricity>0.&&self.radius(corner)==self.inner{self.inner_frame().origin}else{self.frame.origin};
        base+self.frame.z*self.level(corner)
    }

    pub fn point(self,corner:usize,u:f64)->Pnt3{
        self.center(corner)+self.radial(u)*self.radius(corner)
    }
    pub fn vertex(self,index:usize)->Pnt3{
        self.point(if self.full(){index}else{index%4},if self.full()||index<4{0.}else{self.sweep})
    }

    /// 📐️ End-face support points, with the radial and axial profile directions retained.
    pub fn cut_plane(self,index:usize)->Frame3{
        let x=self.radial(if index==0{0.}else{self.sweep});
        let y=if index==0{self.frame.z}else{-self.frame.z};
        Frame3{origin:self.frame.origin,x,y,z:x.cross(y)}
    }

    pub fn edge(self,index:usize)->(Curve3,(f64,f64),[usize;2]){
        if self.edge_is_arc(index){
            let corner=if self.full(){index}else{index%4};
            let frame=Frame3{origin:self.center(corner),..self.frame};
            let vertices=if self.full(){[corner;2]}else{[corner,corner+4]};
            return (Curve3::Circle{frame,radius:self.radius(corner)},(0.,self.sweep),vertices);
        }
        let vertices=if self.has_two_steps(){let start=2*(index-6);[start,start+1]}
            else if self.full(){if index==4{[1,2]}else{[3,0]}}
            else{[index,index/4*4+(index+1)%4]};
        let origin=self.vertex(vertices[0]);
        (Curve3::Line{origin,dir:self.vertex(vertices[1])-origin},(0.,1.),vertices)
    }
}
