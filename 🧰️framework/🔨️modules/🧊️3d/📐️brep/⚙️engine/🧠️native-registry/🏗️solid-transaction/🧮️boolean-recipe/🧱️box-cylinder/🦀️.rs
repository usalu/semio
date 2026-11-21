//! 🧱️ Certified mixed box and vertical cylinder Boolean profiles.

use super::*;
use semio_framework_3d_brep_vector::{Vec3, matrix::Frame3};

pub(super) fn combine(a:SolidPrimitive,b:SolidPrimitive,operation:BooleanOp)->Result<SolidPrimitive,ValueError>{
    let a_is_box=matches!(a,SolidPrimitive::Box{..});
    let (box_shape,cylinder)=if a_is_box{(a,b)}else{(b,a)};
    let SolidPrimitive::Box{origin,width,depth,height:box_height}=box_shape else{return Err(unsupported());};
    let SolidPrimitive::Cylinder{frame,radius,height}=cylinder else{return Err(unsupported());};
    let axis=if frame.z==Vec3::Z{1.0}else if frame.z==-Vec3::Z{-1.0}else{return Err(unsupported());};
    let bounds=Bounds::box_primitive(origin,width,depth,box_height);
    let top=frame.origin.z+axis*height;
    let box_span=[bounds.lo[2],bounds.hi[2]];
    let tube_span=[frame.origin.z.min(top),frame.origin.z.max(top)];
    let (first,last)=if a_is_box{(box_span,tube_span)}else{(tube_span,box_span)};
    let overlap=[first[0].max(last[0]),first[1].min(last[1])];
    let center=frame.origin.to_array();
    let tolerance=Tol::DEFAULT.value();
    let x=center[0]-center[0].clamp(bounds.lo[0],bounds.hi[0]);
    let y=center[1]-center[1].clamp(bounds.lo[1],bounds.hi[1]);
    if overlap[1]<=overlap[0]||x.hypot(y)-radius>tolerance{
        return match operation{
            BooleanOp::Cut=>Ok(a),BooleanOp::Intersect=>Err(empty_result()),BooleanOp::Unite=>Err(unsupported()),
        };
    }
    let cylinder_inside=(0..2).all(|i|(center[i]-bounds.lo[i]).min(bounds.hi[i]-center[i])-radius>tolerance);
    let far=|i:usize|(center[i]-bounds.lo[i]).abs().max((center[i]-bounds.hi[i]).abs());
    let box_inside=radius-far(0).hypot(far(1))>tolerance;
    if !(cylinder_inside||box_inside){return Err(unsupported());}
    let (smaller,larger,inner_span,outer_span)=if box_inside{(box_shape,cylinder,box_span,tube_span)}
        else{(cylinder,box_shape,tube_span,box_span)};
    match operation{
        BooleanOp::Intersect=>clip(smaller,overlap),
        BooleanOp::Unite if outer_span[0]<=inner_span[0]&&outer_span[1]>=inner_span[1]=>Ok(larger),
        BooleanOp::Cut if a_is_box==box_inside=>{
            if overlap==first{return Err(empty_result());}
            if overlap[0]==first[0]{return clip(a,[overlap[1],first[1]]);}
            if overlap[1]==first[1]{return clip(a,[first[0],overlap[0]]);}
            Err(unsupported())
        }
        _=>Err(unsupported()),
    }
}

fn clip(shape:SolidPrimitive,[lo,hi]:[f64;2])->Result<SolidPrimitive,ValueError>{
    let height=hi-lo;
    let result=match shape{
        SolidPrimitive::Box{origin,width,depth,..}=>SolidPrimitive::Box{origin:Pnt3::new(origin.x,origin.y,lo),width,depth,height},
        SolidPrimitive::Cylinder{frame,radius,..}=>{
            let z=if frame.z.z>0.0{lo}else{hi};
            SolidPrimitive::Cylinder{frame:Frame3{origin:Pnt3::new(frame.origin.x,frame.origin.y,z),..frame},radius,height}
        }
        _=>return Err(unsupported()),
    };
    result.validate().map_err(|_|unsupported())?;
    Ok(result)
}

/// 📐️ Reconstructs exact side and cap trims when neither mixed radial footprint contains the other.
pub(super) fn side_intersection(a:SolidPrimitive,b:SolidPrimitive)->Result<CircularPrism,ValueError>{
    let (stock,tube)=if matches!(a,SolidPrimitive::Box{..}){(a,b)}else{(b,a)};
    let SolidPrimitive::Box{origin,width,depth,height:box_height}=stock else{return Err(unsupported());};
    let SolidPrimitive::Cylinder{frame,radius,height}=tube else{return Err(unsupported());};
    if frame.z!=Vec3::Z&&frame.z!=-Vec3::Z{return Err(unsupported());}
    let bounds=Bounds::box_primitive(origin,width,depth,box_height);
    let top=frame.origin.z+frame.z.z*height;
    let span=[bounds.lo[2].max(top.min(frame.origin.z)),bounds.hi[2].min(top.max(frame.origin.z))];
    let center=[frame.origin.x,frame.origin.y];
    let [lo,hi]=[bounds.lo,bounds.hi].map(|[x,y,_]|[x,y]);
    let gap:[f64;2]=std::array::from_fn(|i|center[i]-center[i].clamp(lo[i],hi[i]));
    if gap[0].hypot(gap[1])>=radius{return Err(empty_result());}
    super::disk_rectangle::clip_disk(center,radius,[lo,hi],span).ok_or_else(unsupported)
}
