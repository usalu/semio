//! 📦️ Bounded convex intersection of two rectangular arrangements of authored planes.

use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};

pub const BOX_BOUNDARY_VERTICES:usize=32;
const BOX_INTERSECTION_VERTICES:usize=20;
const SUPPORTS:usize=12;
const FACE_VERTICES:usize=32;
const BOUNDARY_FACES:usize=6*6+6;

#[path="🪚️recess/🦀️.rs"]
mod recess;
pub use recess::{box_recess_faces,union_box_faces};

#[path="🐚️shells/🦀️.rs"]
mod shells;
pub use shells::{BoxSupportPieces,box_recess_pieces};

/// 🧭️ The corner, orthonormal axes and finite lengths of a rectangular stock.
pub type BoxPlacement=(Pnt3,[Vec3;3],[f64;3]);

#[derive(Clone,Copy)]
struct Support{origin:Pnt3,normal:Vec3,offset:f64}

/// 🔷️ One authored support with a counterclockwise ring of vertex indices.
#[derive(Clone,Copy)]
pub struct ConvexBoxFace{
    pub origin:Pnt3,
    pub normal:Vec3,
    pub ring:[u8;FACE_VERTICES],
    pub count:usize,
}

/// ✂️ A bounded planar boundary with authored faces and an optional inward void shell.
pub struct BoxSupportBoundary{
    pub points:[Pnt3;BOX_BOUNDARY_VERTICES],
    pub point_count:usize,
    pub inner_faces:usize,
    faces:[ConvexBoxFace;BOUNDARY_FACES],
    face_count:usize,
}

impl BoxSupportBoundary{
    pub fn faces(&self)->&[ConvexBoxFace]{&self.faces[..self.face_count]}
}

pub fn support_interval((origin,axes,size):BoxPlacement,anchor:Pnt3,axis:Vec3)->[f64;2]{
    let center=(origin-anchor).dot(axis);
    axes.into_iter().zip(size).fold([center;2],|[lo,hi],(direction,length)|{
        let span=direction.dot(axis)*length;
        [lo+span.min(0.),hi+span.max(0.)]
    })
}

/// 📐️ Verifies containment against every closed stock support at the requested clearance.
pub fn contains_box(outer:BoxPlacement,inner:BoxPlacement,clearance:f64)->bool{
    outer.1.into_iter().zip(outer.2).all(|(axis,length)|{
        let [lo,hi]=support_interval(inner,outer.0,axis);
        lo>=clearance&&hi<=length-clearance
    })
}

/// 🧪️ Tests both sets of stock normals and every nondegenerate edge-cross support.
pub fn boxes_separated(first:BoxPlacement,second:BoxPlacement,contact:f64)->Option<bool>{
    let axes=first.1.into_iter().chain(second.1).chain(first.1.into_iter().flat_map(|a|second.1.map(|b|a.cross(b))));
    for direction in axes{
        let length=direction.norm();
        if length<=64.*f64::EPSILON{continue;}
        let axis=direction*(1./length);
        let [a,b]=support_interval(first,second.0,axis);
        let [c,d]=support_interval(second,second.0,axis);
        if ![a,b,c,d].into_iter().all(f64::is_finite){return None;}
        if b.min(d)-a.max(c)<=contact{return Some(true);}
    }
    Some(false)
}

fn supports(boxes:[BoxPlacement;2])->[Support;SUPPORTS]{
    let reference=boxes[0].0;
    std::array::from_fn(|i|{
        let (corner,axes,size)=boxes[i/6];
        let axis=(i%6)/2;
        let (origin,normal)=if i%2==0{(corner,-axes[axis])}else{(corner+axes[axis]*size[axis],axes[axis])};
        Support{origin,normal,offset:(origin-reference).dot(normal)}
    })
}

fn crossing([a,b,c]:[Support;3])->Option<Vec3>{
    let [bc,ca,ab]=[b.normal.cross(c.normal),c.normal.cross(a.normal),a.normal.cross(b.normal)];
    let determinant=a.normal.dot(bc);
    if determinant.abs()<=64.*f64::EPSILON{return None;}
    Some((bc*a.offset+ca*b.offset+ab*c.offset)*(1./determinant))
}

fn order_face(plane:Support,points:&[Vec3],masks:&[u16],plane_index:usize,epsilon:f64)->Option<Option<ConvexBoxFace>>{
    let mut ring=[0;FACE_VERTICES];
    let mut count=0;
    let mut center=Vec3::ZERO;
    for (i,(&point,&mask)) in points.iter().zip(masks).enumerate(){
        if mask&(1<<plane_index)==0{continue;}
        *ring.get_mut(count)?=i as u8;
        count+=1;
        center=center+point;
    }
    if count<3{return Some(None);}
    center=center*(1./count as f64);
    let frame=Frame3::from_normal(plane.origin,plane.normal)?;
    let angle=|i:u8|{
        let delta=points[usize::from(i)]-center;
        delta.dot(frame.y).atan2(delta.dot(frame.x))
    };
    ring[..count].sort_unstable_by(|&a,&b|angle(a).total_cmp(&angle(b)));
    for i in 0..count{
        let [a,b,c]=[i,(i+1)%count,(i+2)%count].map(|j|points[usize::from(ring[j])]);
        if !((b-a).cross(c-b).dot(plane.normal)>epsilon*epsilon){return None;}
    }
    Some(Some(ConvexBoxFace{origin:plane.origin,normal:plane.normal,ring,count}))
}

fn plane_vertices<const N:usize,const MAX:usize>(
    supports:[Support;N],tolerance:f64,keeps:impl Fn([f64;N],f64)->bool,
)->Option<([Vec3;MAX],[u16;MAX],usize)>{
    let epsilon=tolerance/16.;
    if !supports.iter().all(|plane|plane.offset.is_finite()){return None;}
    let mut points=[Vec3::ZERO;MAX];
    let mut masks=[0;MAX];
    let mut count=0;
    for a in 0..N-2{
        for b in a+1..N-1{
            for c in b+1..N{
                let Some(point)=crossing([supports[a],supports[b],supports[c]])else{continue;};
                let signed=supports.map(|plane|plane.normal.dot(point)-plane.offset);
                if signed.into_iter().any(|distance|!distance.is_finite()){return None;}
                if !keeps(signed,epsilon){continue;}
                let mask=signed.into_iter().enumerate().fold(0u16,|bits,(i,distance)|bits|(u16::from(distance.abs()<=epsilon)<<i));
                if ![a,b,c].into_iter().all(|i|mask&(1<<i)!=0){return None;}
                if let Some(i)=points[..count].iter().position(|&prior|(prior-point).norm()<=tolerance){
                    if masks[i]!=mask||(points[i]-point).norm()>epsilon{return None;}
                    continue;
                }
                *points.get_mut(count)?=point;
                masks[count]=mask;
                count+=1;
            }
        }
    }
    Some((points,masks,count))
}

/// ⌖️ Intersects the twelve authored supports and certifies each active plane and vertex.
pub fn intersect_box_faces(first:BoxPlacement,second:BoxPlacement,tolerance:f64)->Option<BoxSupportBoundary>{
    let supports=supports([first,second]);
    let epsilon=tolerance/16.;
    let (points,masks,count)=plane_vertices::<SUPPORTS,BOX_INTERSECTION_VERTICES>(supports,tolerance,|signed,epsilon|{
        signed.into_iter().all(|distance|distance<=epsilon)
    })?;
    if count<4{return None;}
    let middle=points[..count].iter().fold(Vec3::ZERO,|sum,&point|sum+point*(1./count as f64));
    if supports.iter().any(|plane|!(plane.offset-plane.normal.dot(middle)>epsilon)){return None;}
    let blank=ConvexBoxFace{origin:first.0,normal:Vec3::ZERO,ring:[0;FACE_VERTICES],count:0};
    let mut result=BoxSupportBoundary{points:[first.0;BOX_BOUNDARY_VERTICES],point_count:count,inner_faces:0,faces:[blank;BOUNDARY_FACES],face_count:0};
    for (i,&point) in points[..count].iter().enumerate(){
        let world=first.0+point;
        if !(((world-first.0)-point).norm()<=epsilon){return None;}
        result.points[i]=world;
    }
    for (i,plane) in supports.into_iter().enumerate(){
        if let Some(face)=order_face(plane,&points[..count],&masks[..count],i,epsilon)?{
            result.faces[result.face_count]=face;
            result.face_count+=1;
        }
    }
    (result.face_count>=4).then_some(result)
}

/// 🕳️ Builds an exact outer and inward box boundary with strictly separated supports.
pub fn box_cavity_faces(outer:BoxPlacement,inner:BoxPlacement,tolerance:f64)->Option<BoxSupportBoundary>{
    if !contains_box(outer,inner,tolerance){return None;}
    let planes=supports([outer,inner]);
    let blank=ConvexBoxFace{origin:outer.0,normal:Vec3::ZERO,ring:[0;FACE_VERTICES],count:0};
    let mut result=BoxSupportBoundary{
        points:[outer.0;BOX_BOUNDARY_VERTICES],point_count:16,inner_faces:6,faces:[blank;BOUNDARY_FACES],face_count:SUPPORTS,
    };
    for (shell,(origin,axes,size)) in [outer,inner].into_iter().enumerate(){
        let inner=shell==1;
        let reflected=axes[0].cross(axes[1]).dot(axes[2])<0.;
        for id in 0..8{
            let offset=(0..3).filter(|axis|id&(1<<axis)!=0).fold(Vec3::ZERO,|delta,axis|delta+axes[axis]*size[axis]);
            let point=origin+offset;
            if !(((point-origin)-offset).norm()<=tolerance/16.){return None;}
            result.points[8*shell+id]=point;
        }
        for face in 0..6{
            let [axis,side]=[face/2,face%2];
            let [u,v]=[1<<((axis+1)%3),1<<((axis+2)%3)];
            let corner=side<<axis;
            let mut ring=[0;FACE_VERTICES];
            ring[..4].copy_from_slice(&[corner,corner|u,corner|u|v,corner|v].map(|id|(8*shell+id) as u8));
            if (side==0)^reflected^inner{ring[..4].reverse();}
            let support=planes[6*shell+face];
            let normal=if inner{-support.normal}else{support.normal};
            result.faces[6*shell+face]=ConvexBoxFace{origin:support.origin,normal,ring,count:4};
        }
    }
    Some(result)
}
