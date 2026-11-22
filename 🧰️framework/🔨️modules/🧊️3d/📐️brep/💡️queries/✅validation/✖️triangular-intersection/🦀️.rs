//! ✖️ Bounded exact intersection tests for planar triangular faces in one solid.

use super::*;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::{Pnt2,predicates::{Orient,orient2d,orient3d}};
use semio_framework_value::{RetirementDemand,ValueError};

#[derive(Default)]
pub(super) struct TrianglePairs { shell_a:usize,shell_b:usize,face_a:usize,face_b:usize,left:Option<[Pnt3;3]> }
semio_framework_value::artifact_retire_struct!(TrianglePairs {shell_a,shell_b,face_a,face_b,left});

fn finite(point:Pnt3)->bool{[point.x,point.y,point.z].into_iter().all(f64::is_finite)}

pub(super) fn triangle(body:&Body,id:FaceId)->Option<[Pnt3;3]>{
    read_triangle(body,id,None,&|_,position|position).ok().flatten()
}

fn read_triangle(body:&Body,id:FaceId,mut demand:Option<&mut usize>,positions:&impl Fn(VertexId,Pnt3)->Pnt3)->Result<Option<[Pnt3;3]>,ValueError>{
    macro_rules! row {
        ($arena:ident,$key:expr)=>{
            {
                let key=$key;
                if let Some(depth)=demand.as_deref_mut(){*depth=(*depth).max(body.$arena.next_remove_depth_demand(key)?);}
                let Some(row)=body.$arena.get(key)else{return Ok(None);};
                row
            }
        };
    }
    let face=row!(faces,id);
    if !face.inners.is_empty()||!matches!(row!(surfaces,face.surface),Surface::Plane{..}){return Ok(None);}
    let Some(outer)=face.outer else{return Ok(None);};
    let ring=row!(loops,outer);
    let mut current=ring.first;
    let mut points=[Pnt3::new(0.0,0.0,0.0);3];
    for point in &mut points{
        let use_=row!(coedges,current);
        let edge=row!(edges,use_.edge);
        if !matches!(row!(curves3,edge.curve),Curve3::Line{..}){return Ok(None);}
        let vertex=if use_.forward{edge.v0}else{edge.v1};
        *point=positions(vertex,row!(vertices,vertex).position);
        if !finite(*point){return Ok(None);}
        current=use_.next;
    }
    Ok((current==ring.first).then_some(points))
}

fn project(p:Pnt3,axis:usize)->Pnt2{
    match axis{0=>Pnt2::new(p.y,p.z),1=>Pnt2::new(p.x,p.z),_=>Pnt2::new(p.x,p.y)}
}

fn projection(points:[Pnt3;3])->Option<usize>{
    (0..3).find(|&axis|{
        let [a,b,c]=points.map(|point|project(point,axis));
        orient2d(a,b,c)!=Orient::Zero
    })
}

fn overlap_2d(a:[Pnt2;3],b:[Pnt2;3])->bool{
    let separated=|left:[Pnt2;3],right:[Pnt2;3]|{
        let inside=orient2d(left[0],left[1],left[2]);
        (0..3).any(|i|right.into_iter().all(|p|orient2d(left[i],left[(i+1)%3],p)!=inside))
    };
    !separated(a,b)&&!separated(b,a)
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Membership{Outside,Boundary,Inside}

fn membership(signs:[Orient;3])->Membership{
    if signs.contains(&Orient::Positive)&&signs.contains(&Orient::Negative){Membership::Outside}
    else if signs.contains(&Orient::Zero){Membership::Boundary}
    else{Membership::Inside}
}

fn clipped_endpoints(source:[Pnt3;3],sides:[Orient;3],target:[Pnt3;3],target_axis:usize)->[Membership;2]{
    let mut ends=[Membership::Outside;2];
    let mut count=0;
    for i in 0..3{
        let next=(i+1)%3;
        let kind=if sides[i]==Orient::Zero{
            let [a,b,c]=target.map(|p|project(p,target_axis));
            let p=project(source[i],target_axis);
            membership([orient2d(a,b,p),orient2d(b,c,p),orient2d(c,a,p)])
        }else if sides[next]!=Orient::Zero&&sides[i]!=sides[next]{
            let [a,b,c]=target;
            let [p,q]=[source[i],source[next]];
            membership([orient3d(p,q,a,b),orient3d(p,q,b,c),orient3d(p,q,c,a)])
        }else{continue;};
        ends[count]=kind;
        count+=1;
    }
    ends
}

fn straddles(sides:[Orient;3])->bool{
    sides.contains(&Orient::Positive)&&sides.contains(&Orient::Negative)
}

fn overlaps(ends:[Membership;2])->bool{
    ends.contains(&Membership::Inside)||ends==[Membership::Boundary;2]
}

pub(super) fn triangles_intersect(a:[Pnt3;3],b:[Pnt3;3])->bool{
    if (0..3).any(|axis|{
        let coords=|p:Pnt3|[p.x,p.y,p.z][axis];
        let [l0,l1,l2]=a.map(coords);
        let [r0,r1,r2]=b.map(coords);
        l0.min(l1).min(l2)>r0.max(r1).max(r2)||r0.min(r1).min(r2)>l0.max(l1).max(l2)
    }){return false;}
    let (Some(ax),Some(bx))=(projection(a),projection(b))else{return false;};
    let sides_b=b.map(|p|orient3d(a[0],a[1],a[2],p));
    if sides_b==[Orient::Zero;3]{
        return overlap_2d(a.map(|p|project(p,ax)),b.map(|p|project(p,ax)));
    }
    if !straddles(sides_b){return false;}
    let sides_a=a.map(|p|orient3d(b[0],b[1],b[2],p));
    straddles(sides_a)&&(overlaps(clipped_endpoints(a,sides_a,b,bx))||overlaps(clipped_endpoints(b,sides_b,a,ax)))
}

impl TrianglePairs {
    fn advance_left(&mut self){
        self.face_a+=1;
        self.face_b=if self.shell_a==self.shell_b{self.face_a+1}else{0};
        self.left=None;
    }
}

impl BodyValidationJob {
    pub(super) fn triangle_pair_demand(&self,body:&Body)->Result<RetirementDemand,ValueError>{
        let cursor=&self.triangle_pairs;
        let mut depth=1;
        let mut copy_bytes=0;
        if let (Some(left),Some(right))=(self.shells.get(cursor.shell_a),self.shells.get(cursor.shell_b)){
            if left.solid==right.solid {
                let id=if cursor.left.is_none(){left.faces.get(cursor.face_a)}
                    else{right.faces.get(cursor.face_b)};
                if let Some(&id)=id{
                    let points=read_triangle(body,id,Some(&mut depth),&|_,position|position)?;
                    if cursor.left.is_none()&&points.is_some(){copy_bytes=size_of::<[Pnt3;3]>();}
                }
            }
        }
        Ok(RetirementDemand{copy_bytes,depth,..Default::default()})
    }

    pub(super) fn probe_triangle_pair(&mut self,body:&Body,positions:&impl Fn(VertexId,Pnt3)->Pnt3){
        let cursor=&mut self.triangle_pairs;
        let Some(left)=self.shells.get(cursor.shell_a)else{self.phase=BodyValidationPhase::Complete;return;};
        let Some(right)=self.shells.get(cursor.shell_b).filter(|right|right.solid==left.solid)else{
            cursor.shell_a+=1;
            cursor.shell_b=cursor.shell_a;
            cursor.face_a=0;
            cursor.face_b=1;
            cursor.left=None;
            return;
        };
        let Some(&a)=left.faces.get(cursor.face_a)else{
            cursor.shell_b+=1;
            cursor.face_a=0;
            cursor.face_b=0;
            cursor.left=None;
            return;
        };
        let Some(first)=cursor.left else {
            cursor.left=read_triangle(body,a,None,positions).ok().flatten().filter(|points|projection(*points).is_some());
            if cursor.left.is_none(){cursor.advance_left();}
            return;
        };
        let Some(&b)=right.faces.get(cursor.face_b)else{cursor.advance_left();return;};
        cursor.face_b+=1;
        if a!=b&&read_triangle(body,b,None,positions).ok().flatten().is_some_and(|second|triangles_intersect(first,second)){
            self.hold_topology_finding(retained_loop::LoopFinding::Intersection{a,b});
        }
    }
}
