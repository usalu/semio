//! 🥫️ Exact material Boolean boundaries between parallel finite cylinders.

use super::*;
use semio_framework_3d_brep_construction::circular_prism::{CircleSupports,DiskPairOperation};
use semio_framework_3d_brep_vector::matrix::Frame3;
use std::f64::consts::PI;

fn unit_segment(angle:f64)->f64{
    if angle.abs()>=0.01{return angle-angle.sin()*angle.cos();}
    let square=angle*angle;
    angle*square*(2./3.-square*(2./15.-square*(4./315.-square*2./2835.)))
}

fn cylinder(frame:Frame3,radius:f64,height:f64)->Result<SolidBoundary,ValueError>{
    let shape=SolidPrimitive::Cylinder{frame,radius,height};
    shape.validate().map_err(|_|unsupported())?;
    Ok(SolidBoundary::Analytic(shape))
}

fn trim_stock(frame:Frame3,radius:f64,height:f64,lo:f64,hi:f64)->Result<SolidBoundary,ValueError>{
    if lo==0.&&hi==height{return Err(empty_result());}
    if hi==height{return cylinder(frame,radius,lo);}
    if lo>0.{return Err(unsupported());}
    cylinder(Frame3{origin:frame.origin+frame.z*hi,..frame},radius,height-hi)
}

pub(super) fn combine(a:SolidPrimitive,b:SolidPrimitive,operation:BooleanOp)->Result<SolidBoundary,ValueError>{
    let (SolidPrimitive::Cylinder{frame:f,radius:r,height:h},SolidPrimitive::Cylinder{frame:g,radius:s,height:k})=(a,b)
        else{return Err(unsupported());};
    let aligned=if f.z==g.z{true}else if f.z==-g.z{false}else{return Err(unsupported());};
    let between=g.origin-f.origin;
    let shift=between.dot(f.z);
    let radial=between-f.z*shift;
    let distance=radial.norm();
    if !distance.is_finite()||!shift.is_finite()||!(r+s).is_finite(){return Err(unsupported());}
    let [b0,b1]=if aligned{[shift,shift+k]}else{[shift-k,shift]};
    let [lo,hi]=[0_f64.max(b0),h.min(b1)];
    let contained=distance<=(r-s).abs();
    match operation{
        BooleanOp::Unite=>{
            if contained&&r>=s&&b0>=0.&&b1<=h{return Ok(SolidBoundary::Analytic(a));}
            if contained&&s>=r&&b0<=0.&&b1>=h{return Ok(SolidBoundary::Analytic(b));}
            if contained{
                let (frame,outer,inner,height,span,offset)=if r>s{(f,r,s,h,[b0,b1],radial)}else{
                    let span=if aligned{[-shift,h-shift]}else{[shift-h,shift]};
                    (g,s,r,k,span,-radial)
                };
                let tol=Tol::DEFAULT.value();
                return AnnularRevolution::from_axial_union(frame,inner,outer,height,span,tol)
                    .and_then(|shape|shape.with_offset(offset,tol)).map(SolidBoundary::Annular).ok_or_else(unsupported);
            }
            if b0!=0.||b1!=h||distance>=r+s{return Err(unsupported());}
        }
        BooleanOp::Intersect=>{
            if hi<=lo||distance>=r+s{return Err(empty_result());}
        }
        BooleanOp::Cut=>{
            if hi<=lo||distance>=r+s{return Ok(SolidBoundary::Analytic(a));}
            if contained&&s>=r{return trim_stock(f,r,h,lo,hi);}
            if contained{
                let tol=Tol::DEFAULT.value();
                return AnnularRevolution::from_axial_cut(f,s,r,h,[b0,b1],tol)
                    .and_then(|shape|shape.with_offset(radial,tol)).map(SolidBoundary::Annular).ok_or_else(unsupported);
            }
            if b0>0.||b1<h||distance<=r{return Err(unsupported());}
        }
    };
    let height=hi-lo;
    let tolerance=Tol::DEFAULT.value();
    if height<=tolerance{return Err(unsupported());}
    let centers=[f.origin+f.z*lo,g.origin+f.z*(lo-shift)];
    if contained{
        let (center,frame,radius)=if r<=s{(centers[0],f,r)}else{
            let offset=if aligned{lo}else{hi};
            (g.origin+f.z*(offset-shift),g,s)
        };
        return cylinder(Frame3{origin:center,..frame},radius,height);
    }
    if distance-(r-s).abs()<=tolerance||r+s-distance<=tolerance{return Err(unsupported());}
    let x=radial.normalized().ok_or_else(unsupported)?;
    let y=f.z.cross(x);
    let a_x=0.5*(distance+((r-s)/distance)*(r+s));
    let b_x=distance-a_x;
    let half_chord=((r-a_x)*(r+a_x)).sqrt();
    if !half_chord.is_finite()||half_chord<=tolerance{return Err(unsupported());}
    let [a,b]=[half_chord.atan2(a_x),half_chord.atan2(b_x)];
    let (angles,fan,disk_operation)=match operation{
        BooleanOp::Intersect=>([a,b],0.5*(distance-s+r),DiskPairOperation::Intersect),
        BooleanOp::Unite=>([PI-a,PI-b],0.5*(distance-s+r),DiskPairOperation::Unite),
        BooleanOp::Cut=>{
            let last=distance-s*(s/b_x);
            if !last.is_finite()||last+r<=tolerance{return Err(unsupported());}
            ([PI-a,-b],0.5*(last-r),DiskPairOperation::Cut)
        }
    };
    let area=r*r*unit_segment(angles[0])+s*s*unit_segment(angles[1]);
    let chord=centers[0]+x*a_x;
    let interior=centers[0]+x*fan;
    let mut corners=[Pnt3::new(0.,0.,0.);8];
    let [first,last]=[chord-y*half_chord,chord+y*half_chord];
    corners[..2].copy_from_slice(&if matches!(operation,BooleanOp::Intersect){[first,last]}else{[last,first]});
    let mut arcs=[0.;8];
    arcs[..2].copy_from_slice(&angles.map(|angle|2.*angle));
    let supports=CircleSupports::Pair{circles:[(centers[0],r),(centers[1],s)],operation:disk_operation};
    CircularPrism::from_profile(corners,arcs,2,interior,f.z,supports,height,height*area)
        .map(SolidBoundary::CircularPrism).ok_or_else(unsupported)
}
