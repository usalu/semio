//! 🐚️ Certifies and separates closed material shells in an authored planar Box arrangement.

use super::*;

type FaceMask=u64;

fn indices(mut mask:FaceMask)->impl Iterator<Item=usize>{
    std::iter::from_fn(move||{
        if mask==0{return None;}
        let index=mask.trailing_zeros() as usize;
        mask&=mask-1;
        Some(index)
    })
}

fn sides(face:&ConvexBoxFace)->impl Iterator<Item=(u8,u8)>+'_{
    let ring=&face.ring[..face.count];
    ring.iter().copied().zip(ring.iter().copied().cycle().skip(1))
}

struct Links{
    edges:[[[u8;2];BOX_BOUNDARY_VERTICES];BOX_BOUNDARY_VERTICES],
    vertices:[FaceMask;BOX_BOUNDARY_VERTICES],
}

impl Links{
    fn build(boundary:&BoxSupportBoundary)->Option<Self>{
        let mut links=Self{edges:[[[0;2];BOX_BOUNDARY_VERTICES];BOX_BOUNDARY_VERTICES],vertices:[0;BOX_BOUNDARY_VERTICES]};
        for (index,face) in boundary.faces().iter().enumerate(){
            if !(3..=face.ring.len()).contains(&face.count){return None;}
            let bit=1<<index;
            for (a,b) in sides(face){
                let (low,high)=(usize::from(a.min(b)),usize::from(a.max(b)));
                if low==high||high>=boundary.point_count{return None;}
                let vertex=&mut links.vertices[usize::from(a)];
                if *vertex&bit!=0{return None;}
                *vertex|=bit;
                let use_face=&mut links.edges[low][high][usize::from(a>b)];
                if *use_face!=0{return None;}
                *use_face=(index+1) as u8;
            }
        }
        Some(links)
    }

    fn flood(&self,boundary:&BoxSupportBoundary,allowed:FaceMask,vertex:Option<u8>)->Option<FaceMask>{
        let mut frontier=allowed&allowed.wrapping_neg();
        let mut seen=0;
        while frontier!=0{
            let index=frontier.trailing_zeros() as usize;
            let bit=1<<index;
            frontier&=!bit;
            seen|=bit;
            for (a,b) in sides(&boundary.faces[index]){
                if vertex.is_some_and(|vertex|a!=vertex&&b!=vertex){continue;}
                let neighbor=self.edges[usize::from(a.min(b))][usize::from(a.max(b))][usize::from(a<b)];
                let near=1<<neighbor.checked_sub(1)?;
                if allowed&near==0{return None;}
                frontier|=near&!seen;
            }
        }
        Some(seen)
    }
}

struct Shells{
    masks:[FaceMask;BOUNDARY_FACES/4],
    count:usize,
}

impl Shells{
    fn build(boundary:&BoxSupportBoundary)->Option<Self>{
        let links=Links::build(boundary)?;
        for (vertex,&fan) in links.vertices.get(..boundary.point_count)?.iter().enumerate(){
            if fan==0||links.flood(boundary,fan,Some(vertex as u8))?!=fan{return None;}
        }
        let mut remaining=(1<<boundary.face_count)-1;
        let mut result=Self{masks:[0;BOUNDARY_FACES/4],count:0};
        while remaining!=0{
            let candidates=links.flood(boundary,remaining,None)?;
            if candidates.count_ones()<4{return None;}
            *result.masks.get_mut(result.count)?=candidates;
            result.count+=1;
            remaining&=!candidates;
        }
        Some(result)
    }
}

fn volume(boundary:&BoxSupportBoundary,faces:FaceMask)->Option<f64>{
    let anchor=boundary.points[0];
    let signed=indices(faces).map(|index|{
        let face=&boundary.faces[index];
        let ring=&face.ring[..face.count];
        let start=boundary.points[usize::from(ring[0])]-anchor;
        ring[1..].windows(2).map(|edge|{
            let left=boundary.points[usize::from(edge[0])]-anchor;
            let right=boundary.points[usize::from(edge[1])]-anchor;
            start.dot(left.cross(right))
        }).sum::<f64>()
    }).sum::<f64>()/6.;
    signed.is_finite().then_some(signed)
}

impl BoxSupportBoundary{
    /// 🛡️ Confirms a connected positive outer manifold and its optional connected inward shell.
    pub fn certifies_shell_layout(&self)->bool{
        let Some(shells)=Shells::build(self)else{return false;};
        let Some(outer_count)=self.face_count.checked_sub(self.inner_faces)else{return false;};
        let outer=(1<<outer_count)-1;
        if shells.masks[0]!=outer||!volume(self,outer).is_some_and(|volume|volume>0.){return false;}
        if self.inner_faces==0{return shells.count==1;}
        let inside=((1<<self.face_count)-1)^outer;
        shells.count==2&&shells.masks[1]==inside&&volume(self,inside).is_some_and(|volume|volume<0.)
    }
}

/// 🧺️ Retains the exact partition once and exposes separate positive solid boundaries.
pub struct BoxSupportPieces{
    plan:BoxSupportBoundary,
    shells:Shells,
}

impl BoxSupportPieces{
    pub fn count(&self)->usize{self.shells.count}

    pub fn member(&self,index:usize)->Option<BoxSupportBoundary>{
        let mask=*self.shells.masks[..self.shells.count].get(index)?;
        let first=self.plan.points[0];
        let mut result=BoxSupportBoundary{
            points:[first;BOX_BOUNDARY_VERTICES],point_count:0,inner_faces:0,
            faces:[self.plan.faces[0];BOUNDARY_FACES],face_count:0,
        };
        let mut remap=[u8::MAX;BOX_BOUNDARY_VERTICES];
        for index in indices(mask){
            let mut face=self.plan.faces[index];
            for id in &mut face.ring[..face.count]{
                let original=usize::from(*id);
                let local=&mut remap[original];
                if *local==u8::MAX{
                    *local=result.point_count as u8;
                    result.points[result.point_count]=self.plan.points[original];
                    result.point_count+=1;
                }
                *id=*local;
            }
            result.faces[result.face_count]=face;
            result.face_count+=1;
        }
        Some(result)
    }
}

/// 🪓️ Returns disconnected positive shells of one certified original Box Cut.
pub fn box_recess_pieces(stock:BoxPlacement,tool:BoxPlacement,tolerance:f64)->Option<BoxSupportPieces>{
    let plan=box_recess_faces(stock,tool,tolerance)?;
    let shells=Shells::build(&plan)?;
    (shells.count>1&&shells.masks[..shells.count].iter().all(|&mask|volume(&plan,mask).is_some_and(|volume|volume>0.)))
        .then_some(BoxSupportPieces{plan,shells})
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn rejects_separate_vertex_fans_with_reciprocal_edges(){
        let origin=Pnt3::from_array([0.;3]);
        let face=ConvexBoxFace{origin,normal:Vec3::ZERO,ring:[0;FACE_VERTICES],count:3};
        let mut boundary=BoxSupportBoundary{
            points:[origin;BOX_BOUNDARY_VERTICES],point_count:7,inner_faces:0,
            faces:[face;BOUNDARY_FACES],face_count:8,
        };
        let tetrahedron=[[0,2,1],[0,1,3],[0,3,2],[1,2,3]];
        for (index, vertices) in [[0,1,2,3],[0,4,5,6]].into_iter().enumerate(){
            for (face,ring) in tetrahedron.into_iter().enumerate(){
                boundary.faces[4*index+face].ring[..3].copy_from_slice(&ring.map(|id|vertices[id]));
            }
        }
        let links=Links::build(&boundary).unwrap();
        assert_eq!(links.flood(&boundary,255,None),Some(15));
        assert!(Shells::build(&boundary).is_none());
    }
}
