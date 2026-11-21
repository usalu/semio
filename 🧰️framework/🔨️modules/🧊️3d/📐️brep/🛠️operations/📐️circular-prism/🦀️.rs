//! 📐️ Native axial solid topology for a star-shaped profile of lines and circular arcs.

use crate::primitives::{PrimitiveEdge,PrimitiveTrim,planar_pcurve};
use semio_framework_3d_brep_curve::{Curve2,Curve3};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt2,Pnt3,Vec2,Vec3,matrix::Frame3};
use std::f64::consts::TAU;

/// 🛞️ Supplies the exact circle centers and radii for a bounded cap profile.
#[derive(Clone,Copy)]
pub enum CircleSupports {
    Shared{radius:f64,center:Option<Pnt3>},
    Pair{circles:[(Pnt3,f64);2],operation:DiskPairOperation},
}

/// 🔀️ Identifies which material of the two disk supports survives.
#[derive(Clone,Copy)]
pub enum DiskPairOperation { Intersect, Unite, Cut }

impl CircleSupports {
    fn arc(self,side:usize)->(Option<Pnt3>,f64){
        match self {
            Self::Shared{radius,center}=>(center,radius),
            Self::Pair{circles,..}=>(Some(circles[side].0),circles[side].1),
        }
    }
}

/// 🧷️ Retains the profile that owns the cap and side incidences of one axial solid.
#[derive(Clone,Copy)]
pub struct CircularPrism {
    corners:[Pnt3;8],
    arcs:[f64;8],
    supports:CircleSupports,
    pub count:usize,
    pub center:Pnt3,
    pub axis:Vec3,
    pub length:f64,
    pub volume:f64,
}

impl CircularPrism {
    /// 🌙️ Builds and checks native trims from a certified counterclockwise profile visible from its interior fan point.
    pub fn from_profile(
        corners:[Pnt3;8],arcs:[f64;8],count:usize,center:Pnt3,axis:Vec3,supports:CircleSupports,
        length:f64,volume:f64,
    )->Option<Self>{
        let tol=Tol::DEFAULT.value();
        if !(2..=8).contains(&count)||!volume.is_finite()||volume<=0.||!length.is_finite()||length<=tol{return None;}
        if matches!(supports,CircleSupports::Pair{operation,..} if count!=2||arcs[..2].contains(&0.)||arcs[0]<0.||(arcs[1]<0.)!=matches!(operation,DiskPairOperation::Cut)){return None;}
        let result=Self{corners,arcs,count,center,axis,length,volume,supports};
        for i in 0..count{
            let next=(i+1)%count;
            let chord=(corners[next]-corners[i]).norm();
            if !chord.is_finite()||chord<=tol || (result.vertex(i+count)-corners[i]-axis*length).norm()>tol{return None;}
            let sweep=result.arc_span(i);
            if sweep==0.{continue;}
            let radius=result.supports.arc(i).1;
            let [first,last]=result.profile_vertices(i);
            if !(0. ..TAU).contains(&sweep)||!radius.is_finite()||radius<=tol{return None;}
            for top in [false,true]{
                let frame=result.arc_frame(i,top)?;
                let (sin,cos)=sweep.sin_cos();
                let cap=count*usize::from(top);
                let start=(result.vertex(cap+first)-(frame.origin+frame.x*radius)).norm();
                let end=(result.vertex(cap+last)-(frame.origin+(frame.x*cos+frame.y*sin)*radius)).norm();
                if !(start<=tol&&end<=tol){return None;}
            }
        }
        Some(result)
    }

    /// 🧮️ Measures the two planar caps and the exact linear and cylindrical sides.
    pub fn area(&self)->f64{
        let perimeter=self.arcs[..self.count].iter().enumerate().map(|(i,&span)|{
            if span!=0.{span.abs()*self.supports.arc(i).1}
            else{(self.corners[(i+1)%self.count]-self.corners[i]).norm()}
        }).sum::<f64>();
        2.*(self.volume/self.length)+self.length*perimeter
    }

    pub fn vertex_count(self)->usize{2*self.count}
    pub fn edge_count(self)->usize{3*self.count}
    pub fn ring_sizes(self)->&'static [usize]{
        match self.count{
            2=>&[2,2,4,4],
            3=>&[3,3,4,4,4],
            4=>&[4,4,4,4,4,4],
            5=>&[5,5,4,4,4,4,4],
            6=>&[6,6,4,4,4,4,4,4],
            7=>&[7,7,4,4,4,4,4,4,4],
            _=>&[8,8,4,4,4,4,4,4,4,4],
        }
    }
    pub fn has_arc(self,side:usize)->bool{self.arc_span(side)>0.}

    pub fn arc_span(self,side:usize)->f64{self.arcs[side].abs()}

    pub fn face_flipped(self,face:usize)->bool{face>=2&&self.inward(face-2)}

    pub fn arc_parameter(self,side:usize,fraction:f64)->f64{
        self.arc_span(side)*if self.inward(side){1.-fraction}else{fraction}
    }

    fn inward(self,side:usize)->bool{self.arcs[side]<0.}

    fn profile_vertices(self,side:usize)->[usize;2]{
        let next=(side+1)%self.count;
        if self.inward(side){[next,side]}else{[side,next]}
    }

    pub fn maximum_arc_span(self)->f64{self.arcs[..self.count].iter().map(|span|span.abs()).fold(0.,f64::max)}

    pub fn maximum_radius(self)->f64{self.supports.arc(0).1.max(self.supports.arc(1).1)}

    /// 🎯️ Reads the authored circle centers, radii, and disk-pair operation.
    pub fn circle_supports(self)->CircleSupports{self.supports}

    fn arc_frame(self,index:usize,top:bool)->Option<Frame3>{
        let start=self.vertex(self.profile_vertices(index)[0]+self.count*usize::from(top));
        let (center,radius)=self.supports.arc(index);
        if let Some(center)=center{
            let origin=center+self.axis*(if top{self.length}else{0.});
            let x=(start-origin).normalized()?;
            return Some(Frame3{origin,x,y:self.axis.cross(x),z:self.axis});
        }
        let previous=self.vertex((index+self.count-1)%self.count+self.count*usize::from(top));
        let y=(start-previous).normalized()?;
        let x=y.cross(self.axis);
        Some(Frame3{origin:start-x*radius,x,y,z:self.axis})
    }

    /// 📍️ Returns one exact cap vertex within the global publication order.
    pub fn vertex(self,index:usize)->Pnt3{
        self.corners[index%self.count]+self.axis*(if index>=self.count{self.length}else{0.})
    }

    pub fn edge(self,index:usize)->PrimitiveEdge{
        let n=self.count;
        let j=index%n;
        let vertices=if index<2*n{self.profile_vertices(j).map(|v|v+n*(index/n))}else{[j,j+n]};
        if index<2*n&&self.has_arc(j) {
            let frame=self.arc_frame(j,index>=n).expect("certified circular prism arc");
            return PrimitiveEdge{vertices,curve:Curve3::Circle{frame,radius:self.supports.arc(j).1},range:(0.,self.arc_span(j))};
        }
        let [a,b]=vertices.map(|vertex|self.vertex(vertex));
        PrimitiveEdge{vertices,curve:Curve3::Line{origin:a,dir:b-a},range:(0.,1.)}
    }

    pub fn surface(self,face:usize)->Surface{
        if face<2{
            let normal=if face==0{-self.axis}else{self.axis};
            let frame=Frame3::from_normal(self.vertex(face*self.count),normal).expect("circular prism axis");
            return Surface::Plane{frame};
        }
        let i=face-2;
        if self.has_arc(i){
            return Surface::Cylinder{frame:self.arc_frame(i,false).expect("certified circular prism side"),radius:self.supports.arc(i).1};
        }
        let start=self.corners[i];
        let tangent=self.corners[(i+1)%self.count]-start;
        Surface::Plane{frame:Frame3::from_normal(start,tangent.cross(self.axis)).expect("remaining straight side")}
    }

    pub fn trim(self,face:usize,member:usize)->PrimitiveTrim{
        let n=self.count;
        let (edge,forward)=match face {
            0=>{let i=n-1-member;(i,self.inward(i))}
            1=>(n+member,!self.inward(member)),
            _=>{
                let i=face-2;
                let [start,end]=self.profile_vertices(i);
                [(i,true),(2*n+end,true),(n+i,false),(2*n+start,false)][member]
            }
        };
        let spec=self.edge(edge);
        let curve=match self.surface(face){
            Surface::Plane{frame}=>planar_pcurve(&spec.curve,frame),
            _=>{
                let [u,v,du,dv]=[
                    [0.,0.,1.,0.],[self.arc_span(face-2),0.,0.,self.length],
                    [0.,self.length,1.,0.],[0.,0.,0.,self.length]
                ][member];
                Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)}
            }
        };
        PrimitiveTrim{edge,forward,curve,range:spec.range}
    }

    /// 🔁️ Samples a cap arc with exact native endpoint positions.
    pub fn arc_point(self,side:usize,top:bool,fraction:f64)->Pnt3{
        let base=self.count*usize::from(top);
        if fraction==0.{return self.vertex(base+side);}
        if fraction==1.{return self.vertex(base+(side+1)%self.count);}
        self.edge(base+side).curve.eval(self.arc_parameter(side,fraction))
    }
}
