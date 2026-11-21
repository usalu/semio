//! 📐️ Exact coaxial unequal circular sections and their conical lateral surface.

use crate::primitives::{PrimitiveEdge,PrimitiveTrim,planar_pcurve};
use semio_framework_3d_brep_curve::{Curve2,Curve3};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{Pnt3,Pnt2,Vec2,matrix::Frame3};
use std::f64::consts::{FRAC_PI_2,PI,TAU};

/// 🪣️ Owns both planar circular rims and the nondegenerate axial cone interval.
#[derive(Clone,Copy)]
pub struct ConicalFrustum {
    pub frame:Frame3,
    pub radii:[f64;2],
    pub height:f64,
    pub half_angle:f64,
    apex_distance:f64,
    pub volume:f64,
}

impl ConicalFrustum {
    pub const RINGS:&'static [usize]=&[4,1,1];

    /// 🎟️ Certifies a finite, representable truncation with a positive small radius.
    pub fn new(frame:Frame3,radii:[f64;2],height:f64)->Option<Self>{
        let [small,large]=radii;
        let tol=Tol::DEFAULT.value();
        if ![small,large,height].into_iter().all(f64::is_finite)||small<=tol||large-small<=tol||height<=tol{return None;}
        let basis_error=(frame.x.cross(frame.y)-frame.z).norm();
        if !basis_error.is_finite()||basis_error>1e-9||[frame.x,frame.y,frame.z].into_iter().any(|axis|(axis.norm()-1.).abs()>1e-9){return None;}
        let half_angle=(large-small).atan2(height);
        if half_angle<=0.||half_angle>=FRAC_PI_2{return None;}
        let slope=half_angle.tan();
        let apex_distance=small/slope;
        let end=apex_distance+height;
        let apex=frame.origin-frame.z*apex_distance;
        let volume=PI/3.*height*(small*small+small*large+large*large);
        let result=Self{frame,radii,height,half_angle,apex_distance,volume};
        if !volume.is_finite()||volume<=0.||!end.is_finite()||end<=apex_distance
            || !apex.to_array().into_iter().all(f64::is_finite) {return None;}
        let side=result.surface(0);
        for i in 0..2 {
            let ring=result.rim(i);
            let v=if i==0{apex_distance}else{end};
            for angle in [0.,FRAC_PI_2,PI,3.*FRAC_PI_2] {
                let point=ring.to_world(Pnt3::new(radii[i]*angle.cos(),radii[i]*angle.sin(),0.));
                if !(side.eval(angle,v).distance(point)<=tol) {return None;}
            }
        }
        Some(result)
    }

    /// 📐️ Returns both cap areas and the exact conical lateral area.
    pub fn area(self)->f64{
        let [a,b]=self.radii;
        PI*(a*a+b*b+(a+b)*self.height.hypot(b-a))
    }

    /// ⚖️ Returns the axial material centroid in the authored world frame.
    pub fn center_of_mass(self)->Pnt3{
        let [a,b]=self.radii.map(|radius|radius/self.radii[1]);
        let ratio=(a*a+2.*a*b+3.*b*b)/(4.*(a*a+a*b+b*b));
        self.frame.origin+self.frame.z*(self.height*ratio)
    }

    /// 🔺️ Returns the apex of the stored conical support.
    pub fn apex(self)->Pnt3{self.frame.origin-self.frame.z*self.apex_distance}

    pub fn rim(self,index:usize)->Frame3{
        if index==0 {self.frame}
        else {Frame3{origin:self.frame.origin+self.frame.z*self.height,..self.frame}}
    }

    pub fn vertex(self,index:usize)->Pnt3{
        let frame=self.rim(index);
        frame.origin+frame.x*self.radii[index]
    }

    pub fn edge(self,index:usize)->PrimitiveEdge{
        if index<2{
            return PrimitiveEdge{vertices:[index;2],range:(0.,TAU),curve:Curve3::Circle{frame:self.rim(index),radius:self.radii[index]}};
        }
        let origin=self.vertex(0);
        PrimitiveEdge{vertices:[0,1],range:(0.,1.),curve:Curve3::Line{origin,dir:self.vertex(1)-origin}}
    }

    pub fn surface(self,face:usize)->Surface{
        match face{
            0=>Surface::Cone{frame:Frame3{origin:self.apex(),..self.frame},half_angle:self.half_angle},
            1=>Surface::Plane{frame:Frame3{y:-self.frame.y,z:-self.frame.z,..self.frame}},
            _=>Surface::Plane{frame:self.rim(1)},
        }
    }

    pub fn trim(self,face:usize,member:usize)->PrimitiveTrim{
        let (edge,forward)=match (face,member){
            (0,0)=>(0,true),(0,1)=>(2,true),(0,2)=>(1,false),(0,_)=>(2,false),
            (1,_)=>(0,false),_=>(1,true),
        };
        let spec=self.edge(edge);
        let curve=if let Surface::Plane{frame}=self.surface(face){planar_pcurve(&spec.curve,frame)}
        else{
            let v=self.apex_distance;
            let [u,v,du,dv]=[[0.,v,1.,0.],[TAU,v,0.,self.height],[0.,v+self.height,1.,0.],[0.,v,0.,self.height]][member];
            Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)}
        };
        PrimitiveTrim{edge,forward,curve,range:spec.range}
    }
}
