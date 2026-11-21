//! 🪚️ Sewn convex partitions for the removal of a skew recess on authored supports.

use super::*;

const STOCK_SUPPORTS:u16=(1<<6)-1;
const RECESS_CANDIDATES:usize=128;

#[derive(Clone,Copy)]
struct PatchSide{inside:u16,outside:u16}

impl PatchSide{
    fn contains(self,planes:&[Support],point:Vec3,tolerance:f64)->bool{
        planes.iter().enumerate().all(|(index,plane)|{
            let bit=1<<index;
            let distance=plane.normal.dot(point)-plane.offset;
            (self.inside&bit==0||distance<=tolerance)&&(self.outside&bit==0||distance>=-tolerance)
        })
    }
}

fn patch(
    support:Support,slot:usize,side:PatchSide,planes:&[Support],points:&[Vec3],masks:&[u16],epsilon:f64,
)->Option<Option<ConvexBoxFace>>{
    let mut ring=[0;FACE_VERTICES];
    let mut count=0;
    let mut center=Vec3::ZERO;
    let surface=1<<slot;
    let trim=(side.inside|side.outside)&!surface;
    for (i,(&point,&mask)) in points.iter().zip(masks).enumerate(){
        if mask&surface==0||mask&trim==0||!side.contains(planes,point,epsilon){continue;}
        *ring.get_mut(count)?=i as u8;
        count+=1;
        center=center+point;
    }
    if count<3{return Some(None);}
    let frame=Frame3::from_normal(support.origin,support.normal)?;
    center=center*(1./count as f64);
    let angle=|i:u8|{
        let delta=points[usize::from(i)]-center;
        delta.dot(frame.y).atan2(delta.dot(frame.x))
    };
    ring[..count].sort_unstable_by(|&a,&b|angle(a).total_cmp(&angle(b)));
    let point=|i:usize|points[usize::from(ring[i%count])];
    let mut area=0.;
    let mut perimeter=0.;
    for i in 0..count{
        let edge=point(i+1)-point(i);
        let next=point(i+2)-point(i+1);
        let length=edge.norm();
        if !(edge.cross(next).dot(support.normal)>=-epsilon*(length+next.norm())){return None;}
        perimeter+=length;
        area+=(point(i)-center).cross(point(i+1)-center).dot(support.normal);
    }
    if !(area>epsilon*perimeter){return Some(None);}
    Some(Some(ConvexBoxFace{origin:support.origin,normal:support.normal,ring,count}))
}

fn sewn_recess<const N:usize>(origin:Pnt3,active:[Support;N],tolerance:f64)->Option<BoxSupportBoundary>{
    let normalize=N>9;
    let (points,masks,count)=plane_vertices::<N,RECESS_CANDIDATES>(active,tolerance,|signed,epsilon|{
        signed[..6].iter().all(|&distance|distance<=epsilon)
            &&(normalize||signed[6..].iter().any(|&distance|distance>=-epsilon))
    })?;
    if count<4{return None;}
    let epsilon=tolerance/16.;
    let blank=ConvexBoxFace{origin,normal:Vec3::ZERO,ring:[0;FACE_VERTICES],count:0};
    let mut result=BoxSupportBoundary{points:[origin;BOX_BOUNDARY_VERTICES],point_count:0,inner_faces:0,
        faces:[blank;BOUNDARY_FACES],face_count:0};
    let face=|support,slot,side|patch(support,slot,side,&active,&points[..count],&masks[..count],epsilon);
    let complete=PatchSide{inside:(1<<N)-1,outside:0};
    for (i,&support) in active[..6].iter().enumerate(){
        if normalize&&face(support,i,complete)?.is_none(){
            let stock=PatchSide{inside:STOCK_SUPPORTS,outside:0};
            *result.faces.get_mut(result.face_count)?=face(support,i,stock)??;
            result.face_count+=1;
            continue;
        }
        let mut inside=STOCK_SUPPORTS;
        for cutter in 6..N{
            let outside=1<<cutter;
            if let Some(patch)=face(support,i,PatchSide{inside,outside})?{
                *result.faces.get_mut(result.face_count)?=patch;
                result.face_count+=1;
            }
            inside|=outside;
        }
    }
    for (i,&wall) in active.iter().enumerate().skip(6){
        let support=Support{normal:-wall.normal,..wall};
        *result.faces.get_mut(result.face_count)?=face(support,i,complete)??;
        result.face_count+=1;
    }
    if normalize{omit_straight_edge_vertices(&points,&mut result.faces[..result.face_count],epsilon);}
    compact_box_faces::<RECESS_CANDIDATES>(origin,&points[..count],result,epsilon)
}

fn compact_box_faces<const M:usize>(
    origin:Pnt3,points:&[Vec3],mut result:BoxSupportBoundary,epsilon:f64,
)->Option<BoxSupportBoundary>{
    let mut used=[false;M];
    for face in result.faces(){
        for &vertex in &face.ring[..face.count]{used[usize::from(vertex)]=true;}
    }
    let mut remap=[0;M];
    for (i,&local) in points.iter().enumerate(){
        if !used[i]{continue;}
        let world=origin+local;
        if !(((world-origin)-local).norm()<=epsilon){return None;}
        *result.points.get_mut(result.point_count)?=world;
        remap[i]=result.point_count as u8;
        result.point_count+=1;
    }
    for face in &mut result.faces[..result.face_count]{
        for id in &mut face.ring[..face.count]{*id=remap[usize::from(*id)];}
    }
    Some(result)
}

/// 🧵️ Joins convex stock partitions and inward tool walls at their exact authored trims.
pub fn box_recess_faces(stock:BoxPlacement,tool:BoxPlacement,tolerance:f64)->Option<BoxSupportBoundary>{
    let source=supports([stock,tool]);
    let mut active=[source[0];SUPPORTS];
    active[..6].copy_from_slice(&source[..6]);
    let mut next=6;
    for &plane in &source[6..]{
        let [_,high]=support_interval(stock,stock.0,plane.normal);
        if high>plane.offset{
            *active.get_mut(next)?=plane;
            next+=1;
        }
    }
    match next{
        8 if active[6].normal.cross(active[7].normal).norm()>64.*f64::EPSILON=>
            sewn_recess::<8>(stock.0,active[..8].try_into().ok()?,tolerance),
        9=>sewn_recess::<9>(stock.0,active[..9].try_into().ok()?,tolerance),
        10=>sewn_recess::<10>(stock.0,active[..10].try_into().ok()?,tolerance),
        11=>sewn_recess::<11>(stock.0,active[..11].try_into().ok()?,tolerance),
        SUPPORTS=>sewn_recess(stock.0,active,tolerance),
        _=>None,
    }
}

const UNION_CANDIDATES:usize=128;

fn omit_straight_edge_vertices<const M:usize>(points:&[Vec3;M],faces:&mut[ConvexBoxFace],epsilon:f64){
    let mut neighbors=[[u8::MAX;3];M];
    for face in faces.iter(){
        let ring=&face.ring[..face.count];
        for (index,&id) in ring.iter().enumerate(){
            let choices=&mut neighbors[usize::from(id)];
            for adjacent in [ring[(index+ring.len()-1)%ring.len()],ring[(index+1)%ring.len()]]{
                if choices.contains(&adjacent){continue;}
                if let Some(slot)=choices.iter_mut().find(|entry|**entry==u8::MAX){*slot=adjacent;}
            }
        }
    }
    let mut remove=[false;M];
    for (id,[a,b,c]) in neighbors.into_iter().enumerate(){
        if b==u8::MAX||c!=u8::MAX{continue;}
        let current=points[id];
        let left=points[usize::from(a)]-current;
        let right=points[usize::from(b)]-current;
        remove[id]=left.dot(right)<0.&&left.cross(right).norm()<=epsilon*(left.norm()+right.norm());
    }
    for face in faces{
        let original=face.ring;
        let mut count=0;
        for id in original[..face.count].iter().copied(){
            if remove[usize::from(id)]{continue;}
            face.ring[count]=id;
            count+=1;
        }
        face.count=count;
    }
}

/// 🤝️ Sews the outward portions of both sets of authored planes for a positive-volume placed Box union.
pub fn union_box_faces(first:BoxPlacement,second:BoxPlacement,tolerance:f64)->Option<BoxSupportBoundary>{
    let active=supports([first,second]);
    let (points,masks,count)=plane_vertices::<SUPPORTS,UNION_CANDIDATES>(active,tolerance,|signed,epsilon|{
        signed[..6].iter().all(|&distance|distance<=epsilon)||signed[6..].iter().all(|&distance|distance<=epsilon)
    })?;
    let origin=first.0;
    let epsilon=tolerance/16.;
    let blank=ConvexBoxFace{origin,normal:Vec3::ZERO,ring:[0;FACE_VERTICES],count:0};
    let mut result=BoxSupportBoundary{points:[origin;BOX_BOUNDARY_VERTICES],point_count:0,inner_faces:0,
        faces:[blank;BOUNDARY_FACES],face_count:0};
    let face=|slot,side|patch(active[slot],slot,side,&active,&points[..count],&masks[..count],epsilon);
    for start in [0,6]{
        let own=STOCK_SUPPORTS<<start;
        let other=6-start;
        for slot in start..start+6{
            let overlap=face(slot,PatchSide{inside:(1<<SUPPORTS)-1,outside:0})?;
            if overlap.is_none(){
                if let Some(patch)=face(slot,PatchSide{inside:own,outside:0})?{
                    *result.faces.get_mut(result.face_count)?=patch;
                    result.face_count+=1;
                }
                continue;
            }
            let mut inside=own;
            for cutter in other..other+6{
                let outside=1<<cutter;
                if let Some(patch)=face(slot,PatchSide{inside,outside})?{
                    *result.faces.get_mut(result.face_count)?=patch;
                    result.face_count+=1;
                }
                inside|=outside;
            }
        }
    }
    omit_straight_edge_vertices(&points,&mut result.faces[..result.face_count],epsilon);
    compact_box_faces::<UNION_CANDIDATES>(origin,&points[..count],result,epsilon)
}
