//! 🧿️ Exact topology of a meridional rectangle sweeping an annular region.

use super::{AnnularRevolution,Body,FaceId,PrimitiveEdge,PrimitiveTrim,Surface,Tol};
use semio_framework_3d_brep_construction::primitives::planar_pcurve;
use semio_framework_3d_brep_curve::{Curve2,Curve3};
use semio_framework_3d_brep_vector::{Pnt2,Pnt3,Vec2,Vec3,matrix::Frame3};
use semio_framework_value::{ValueError,ValueRefusalKind};

#[path="🪜️double-shoulder/🦀️.rs"]
mod two_steps;

fn unsupported()->ValueError{
    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Revolve requires a circular profile or a meridional line rectangle clear of the axis")
}

/// 📐️ Certifies four original connected and axis-aligned sides in the meridional plane.
pub fn plan_annular_revolution(body:&Body,id:FaceId,origin:Pnt3,axis:Vec3,sweep:f64)->Result<AnnularRevolution,ValueError>{
    let tol=Tol::DEFAULT.value();
    let face=body.faces.get(id).ok_or_else(unsupported)?;
    let Some(Surface::Plane{frame})=body.surfaces.get(face.surface)else{return Err(unsupported());};
    if !face.inners.is_empty()||frame.z.dot(axis).abs()>1e-10||(origin-frame.origin).dot(frame.z).abs()>tol{return Err(unsupported());}
    let loop_id=face.outer.ok_or_else(unsupported)?;
    let ring=body.loops.get(loop_id).filter(|ring|ring.face==id).ok_or_else(unsupported)?;
    let mut current=ring.first;
    let mut corners=[origin;4];
    let mut first=None;
    let mut previous=None;
    for (index,corner) in corners.iter_mut().enumerate(){
        if index!=0&&current==ring.first{return Err(unsupported());}
        let use_=body.coedges.get(current).filter(|u|u.loop_id==loop_id).ok_or_else(unsupported)?;
        let edge=body.edges.get(use_.edge).ok_or_else(unsupported)?;
        let Some(curve@Curve3::Line{..})=body.curves3.get(edge.curve)else{return Err(unsupported());};
        let (a,b)=if use_.forward{(edge.v0,edge.v1)}else{(edge.v1,edge.v0)};
        let a_point=body.vertices.get(a).ok_or_else(unsupported)?.position;
        let b_point=body.vertices.get(b).ok_or_else(unsupported)?.position;
        let ends=if use_.forward{edge.range}else{(edge.range.1,edge.range.0)};
        first.get_or_insert(a);
        if previous.is_some_and(|id|id!=a)||!(curve.eval(ends.0).distance(a_point)<=tol&&curve.eval(ends.1).distance(b_point)<=tol){
            return Err(unsupported());
        }
        *corner=a_point;
        previous=Some(b);
        current=use_.next;
    }
    if current!=ring.first||previous!=first{return Err(unsupported());}
    let along=corners[0]-origin;
    let radial=(along-axis*along.dot(axis)).normalized().ok_or_else(unsupported)?;
    let tangent=axis.cross(radial).normalized().ok_or_else(unsupported)?;
    let radial=tangent.cross(axis);
    let local=corners.map(|corner|{
        let offset=corner-origin;
        [offset.dot(radial),offset.dot(axis),offset.dot(tangent)]
    });
    if local.iter().any(|p|!p.into_iter().all(|value|value.is_finite())||p[2].abs()>tol){return Err(unsupported());}
    let mut lo=[local[0][0],local[0][1]];
    let mut hi=lo;
    for i in 0..4{
        let [p,q]=[local[i],local[(i+1)%4]];
        if ((p[0]-q[0]).abs()<=tol)==((p[1]-q[1]).abs()<=tol){return Err(unsupported());}
        for j in 0..2{lo[j]=lo[j].min(p[j]);hi[j]=hi[j].max(p[j]);}
    }
    let mut mask=0;
    for point in local{
        if (0..2).any(|j|(point[j]-lo[j]).abs()>tol&&(point[j]-hi[j]).abs()>tol){return Err(unsupported());}
        mask|=1<<(u8::from((point[0]-hi[0]).abs()<=tol)+2*u8::from((point[1]-hi[1]).abs()<=tol));
    }
    if mask!=0b1111{return Err(unsupported());}
    let frame=Frame3{origin:origin+axis*lo[1],x:radial,y:tangent,z:axis};
    AnnularRevolution::new(frame,lo[0],hi[0],hi[1]-lo[1],sweep,tol).ok_or_else(unsupported)
}

pub(super) fn ring_sizes(shape:AnnularRevolution)->&'static [usize]{
    if shape.has_two_steps(){two_steps::RINGS}else if shape.full(){&[1,4,1,4,1,1]}else{&[4;6]}
}

pub(super) fn face_index(shape:AnnularRevolution,ring:usize)->usize{
    if shape.has_two_steps(){return two_steps::face(ring);}
    if shape.has_step(){return match ring{4=>2,5=>4,_=>ring};}
    if shape.has_void_shell(){ring}else if shape.has_floor()&&ring==4{4}else if shape.full()&&ring>=4{2*(ring-4)}else{ring}
}

pub(super) fn face_flipped(shape:AnnularRevolution,face:usize)->bool{!shape.has_step()&&face==if shape.full(){3}else{5}}

pub(super) fn edge(shape:AnnularRevolution,index:usize)->PrimitiveEdge{
    let (curve,range,vertices)=shape.edge(index);
    PrimitiveEdge{curve,range,vertices}
}

pub(super) fn surface(shape:AnnularRevolution,face:usize)->Surface{
    if shape.has_two_steps(){return two_steps::surface(shape,face);}
    if !shape.full()&&face<2{return Surface::Plane{frame:shape.cut_plane(face)};}
    let side=if shape.full(){face}else{face-2};
    let frame=shape.frame;
    match side{
        0=>Surface::Plane{frame:Frame3{y:-frame.y,z:-frame.z,..frame}},
        1=>Surface::Cylinder{frame,radius:shape.outer},
        2=>Surface::Plane{frame:Frame3{origin:frame.origin+frame.z*shape.level(2),..frame}},
        3=>Surface::Cylinder{frame:shape.inner_frame(),radius:shape.inner},
        4=>Surface::Plane{frame:Frame3{origin:shape.center(if shape.has_step(){3}else{0}),..frame}},
        _=>Surface::Plane{frame:Frame3{origin:shape.center(3),y:-frame.y,z:-frame.z,..frame}},
    }
}

pub(super) fn trim(shape:AnnularRevolution,ring:usize,member:usize)->PrimitiveTrim{
    if shape.has_two_steps(){return two_steps::trim(shape,ring,member);}
    let face=face_index(shape,ring);
    let side=face as isize-if shape.full(){0}else{2};
    let (index,forward)=if shape.full(){
        match ring{
            0=>(1,false),
            1=>[(4,false),(1,true),(4,true),(2,false)][member],
            2=>(2,true),
            3=>[(5,true),(0,true),(5,false),(3,false)][member],
            4=>(0,!shape.has_step()),
            _=>(3,shape.has_step()),
        }
    }else if face<2{
        if face==0{(member,true)}else{(7-member,false)}
    }else if side==3{
        [(3,true),(8,true),(7,false),(11,false)][member]
    }else{
        let i=side as usize;
        [(i,false),(8+i,true),(4+i,true),(8+(i+1)%4,false)][member]
    };
    let spec=edge(shape,index);
    let trim=match surface(shape,face){
        Surface::Plane{frame}=>planar_pcurve(&spec.curve,frame),
        _=>{
            let [a,b]=if side==1{[0.,shape.level(2)]}else{[shape.ceiling(),shape.floor()]};
            let [u,v,du,dv]=[[0.,a,0.,b-a],[0.,a.min(b),1.,0.],[shape.sweep,a,0.,b-a],[0.,a.max(b),1.,0.]][member];
            Curve2::Line{origin:Pnt2::new(u,v),dir:Vec2::new(du,dv)}
        }
    };
    PrimitiveTrim{edge:index,forward,curve:trim,range:spec.range}
}
