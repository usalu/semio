//! 🪟️ Exact axial extrusion of certified planar rectangular profiles with one opening.

use super::{OrthogonalBoundary,SolidBoundary,rectangular_extrusion::bored_rectangular_extrusion};
use semio_framework_3d_brep_boolean::BooleanOp;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::{FaceId,LoopId},topology::Body};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_value::{ValueError,ValueRefusalKind};

fn unsupported()->ValueError{
    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original perforated extrusion requires one interior circular or axis-aligned rectangular opening and a normal vector")
}

/// 🪟️ Uses the exact analytic bore or closed orthogonal cells selected by the original inner trim.
pub fn perforated_extrusion(body:&Body,id:FaceId,vector:[f64;3])->Result<SolidBoundary,ValueError>{
    let face=body.faces.get(id).ok_or_else(unsupported)?;
    let [inner]=face.inners.as_slice()else{return Err(unsupported());};
    let leading=body.loops.get(*inner).and_then(|ring|body.coedges.get(ring.first))
        .and_then(|use_|body.edges.get(use_.edge)).and_then(|edge|body.curves3.get(edge.curve));
    if matches!(leading,Some(Curve3::Circle{..})) {
        return bored_rectangular_extrusion(body,id,vector).map(SolidBoundary::Bore);
    }
    if !vector.into_iter().all(f64::is_finite){return Err(unsupported());}
    let mut components=vector.into_iter().enumerate().filter(|(_,component)|*component!=0.);
    let Some((axis,delta))=components.next()else{return Err(unsupported());};
    if components.next().is_some()||delta.abs()<=Tol::DEFAULT.value(){return Err(unsupported());}
    let Some(Surface::Plane{frame})=body.surfaces.get(face.surface)else{return Err(unsupported());};
    let normal=frame.z.to_array();
    if normal[axis].abs()!=1. || normal[(axis+1)%3]!=0. || normal[(axis+2)%3]!=0. {return Err(unsupported());}
    let level=frame.origin.to_array()[axis];
    let (mut outer,winding)=rectangular_ring(body,id,face.outer.ok_or_else(unsupported)?,axis,level)?;
    let (mut opening,opening_winding)=rectangular_ring(body,id,*inner,axis,level)?;
    if winding*normal[axis]<=0. || opening_winding*normal[axis]>=0. {return Err(unsupported());}
    for coordinate in [(axis+1)%3,(axis+2)%3] {
        if opening[0][coordinate]-outer[0][coordinate]<=Tol::DEFAULT.value()
            || outer[1][coordinate]-opening[1][coordinate]<=Tol::DEFAULT.value()
        {return Err(unsupported());}
    }
    let end=level+delta;
    if !end.is_finite()||end==level{return Err(unsupported());}
    for span in [&mut outer,&mut opening] {
        span[0][axis]=level.min(end);
        span[1][axis]=level.max(end);
    }
    OrthogonalBoundary::new(outer,opening,BooleanOp::Cut).map(SolidBoundary::Orthogonal)
}

fn rectangular_ring(body:&Body,owner:FaceId,id:LoopId,axis:usize,level:f64)->Result<([[f64;3];2],f64),ValueError>{
    let ring=body.loops.get(id).filter(|ring|ring.face==owner).ok_or_else(unsupported)?;
    let mut member=ring.first;
    let mut first=None;
    let mut previous=None;
    let mut corners=[None;4];
    for (index,corner) in corners.iter_mut().enumerate() {
        if index!=0&&member==ring.first{return Err(unsupported());}
        let use_=body.coedges.get(member).filter(|use_|use_.loop_id==id).ok_or_else(unsupported)?;
        let edge=body.edges.get(use_.edge).ok_or_else(unsupported)?;
        let Some(Curve3::Line{..})=body.curves3.get(edge.curve)else{return Err(unsupported());};
        let (start,end)=if use_.forward{(edge.v0,edge.v1)}else{(edge.v1,edge.v0)};
        first.get_or_insert(start);
        if previous.is_some_and(|end|end!=start){return Err(unsupported());}
        let position=&body.vertices.get(start).ok_or_else(unsupported)?.position;
        *corner=Some(position);
        let point=position.to_array();
        if point[axis]!=level||!point.into_iter().all(f64::is_finite){return Err(unsupported());}
        previous=Some(end);member=use_.next;
    }
    if member!=ring.first||first!=previous{return Err(unsupported());}
    let u=(axis+1)%3;
    let v=(axis+2)%3;
    let corners=corners.map(|corner|corner.expect("each corner has one native vertex"));
    let mut span=[corners[0].to_array();2];
    for (i,p) in corners.into_iter().enumerate() {
        let p=p.to_array();
        let q=corners[(i+1)%4].to_array();
        if (p[u]==q[u])==(p[v]==q[v]){return Err(unsupported());}
        for j in [u,v]{span[0][j]=span[0][j].min(p[j]);span[1][j]=span[1][j].max(p[j]);}
    }
    let mut mask=0;
    for point in corners{
        let point=point.to_array();
        if [u,v].into_iter().any(|i|point[i]!=span[0][i]&&point[i]!=span[1][i]){return Err(unsupported());}
        mask|=1<<(u8::from(point[u]==span[1][u])+2*u8::from(point[v]==span[1][v]));
    }
    let winding=(*corners[1]-*corners[0]).cross(*corners[2]-*corners[1]).to_array()[axis];
    if mask!=0b1111||!winding.is_finite()||span[1][u]-span[0][u]<=Tol::DEFAULT.value()
        || span[1][v]-span[0][v]<=Tol::DEFAULT.value(){return Err(unsupported());}
    Ok((span,winding))
}
