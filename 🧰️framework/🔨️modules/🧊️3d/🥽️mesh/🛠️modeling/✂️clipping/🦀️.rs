//! 🧱️ Retained solid clipping owns its complete mutable work and provenance.
use semio_framework_3d_mesh_contract::{Vec3,MeshKernelError,MeshResult,MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex,MeshRankedEdge as Ranked};
use semio_framework_mesh_engine::{HistoryFoldIndex,HistoryFoldSet};
use std::{cmp::Reverse,collections::BinaryHeap};
#[path="⚗️convex-intersection/🦀️.rs"]
mod convex_intersection;
pub use convex_intersection::{MeshConvexDifference,MeshConvexIntersection,MeshConvexUnion};
pub use semio_framework_3d_mesh_contract::{MeshConvexIntersectionStep,MeshIntersectionSolid,MeshIntersectionFace};
/// ✂️ Signed float32 classification used by every actual retained clipping edge.
pub fn mesh_clip_edge(a:Vec3,b:Vec3,normal:Vec3,distance:f32,epsilon:f32)->(f32,f32,Option<(Vec3,f32)>) {
    let da=a.dot(normal)-distance;let db=b.dot(normal)-distance;
    let intersection=if (da>epsilon&&db< -epsilon)||(da< -epsilon&&db>epsilon){let t=da/(da-db);Some((a.lerp(b,t),t))}else{None};
    (da,db,intersection)
}
fn anchored_distance(point:Vec3,normal:Vec3,origin:Vec3)->f32{
    anchored_height(point,normal,origin) as f32
}
fn anchored_height(point:Vec3,normal:Vec3,origin:Vec3)->f64{
    (0..3).map(|axis|(f64::from(point.0[axis])-f64::from(origin.0[axis]))*f64::from(normal.0[axis])).sum()
}

pub struct MeshClipJob {
    normal: Vec3,
    distance: f32,
    origin:Option<Vec3>,
    retired_faces:Vec<Vec<u32>>,
    ring_area:[f64;3],
    epsilon: f32,
    intersections: HistoryFoldIndex<(u32, u32), u32>,
    cap: HistoryFoldSet<u32>,
    cap_ids: Vec<u32>,
    clipped: Vec<Vec<u32>>,
    corners: Vec<u32>,
    sorted: BinaryHeap<Reverse<Ranked>>,
    center: Vec3,
    face: usize,
    corner: usize,
    vertex_sources:Vec<SourceVertex>,
    source_faces:Vec<u32>,
    corner_faces:Vec<u32>,
    face_weights:Vec<Vec<u32>>,
    cap_owners:HistoryFoldIndex<u32,u32>,
    cap_weights:Vec<u32>,
    source_vertices:usize,
    cursor:usize,
    phase: u8,
    completed:bool,
    transferred:u8,
}

impl MeshClipJob {
    /// 🧬️ Transfers the retained face provenance once to the actual parent remapper.
    pub fn take_source_faces(&mut self)->MeshResult<Vec<u32>>{self.transfer(1)?;Ok(std::mem::take(&mut self.source_faces))}
    /// 🧵️ Transfers ordered corner provenance without copying its retained allocation.
    pub fn take_corner_faces(&mut self)->MeshResult<Vec<u32>>{self.transfer(2)?;Ok(std::mem::take(&mut self.corner_faces))}
    /// ⚖️ Transfers contributor weights after the clipping machine has completed.
    pub fn take_face_weights(&mut self)->MeshResult<Vec<Vec<u32>>>{self.transfer(4)?;Ok(std::mem::take(&mut self.face_weights))}
    /// 📍️ Reads one actual generated vertex origin for the parent's current output vertex.
    pub fn vertex_source(&self,index:usize)->MeshResult<SourceVertex>{if !self.completed{return Err(MeshKernelError::InvalidInput("clipping provenance is not complete".into()));}self.vertex_sources.get(index).copied().ok_or(MeshKernelError::InvalidHandle)}
    fn transfer(&mut self,bit:u8)->MeshResult<()>{if !self.completed||self.transferred&bit!=0{return Err(MeshKernelError::InvalidInput("clipping provenance transfer refused".into()));}self.transferred|=bit;Ok(())}

    pub fn new(normal: Vec3, distance: f32, epsilon: f32) -> Self {
        Self { normal, distance, origin:None, retired_faces:Vec::new(), ring_area:[0.0;3], epsilon, intersections: HistoryFoldIndex::new(), cap: HistoryFoldSet::new(), cap_ids: Vec::new(), clipped: Vec::new(), corners: Vec::new(), sorted: BinaryHeap::new(), center: Vec3::ZERO, face: 0, corner: 0,vertex_sources:Vec::new(),source_faces:Vec::new(),corner_faces:Vec::new(),face_weights:Vec::new(),cap_owners:HistoryFoldIndex::new(),cap_weights:Vec::new(),source_vertices:0,cursor:0,phase:6,completed:false,transferred:0 }
    }

    fn anchored(normal:Vec3,origin:Vec3,epsilon:f32)->Self {
        let mut clip=Self::new(normal,0.0,epsilon);
        clip.origin=Some(origin);clip
    }

    fn edge(&self,a:Vec3,b:Vec3)->(f64,f64,Option<(Vec3,f64)>){
        let Some(origin)=self.origin else{
            let (da,db,hit)=mesh_clip_edge(a,b,self.normal,self.distance,self.epsilon);
            return (f64::from(da),f64::from(db),hit.map(|(point,t)|(point,f64::from(t))));
        };
        let da=anchored_height(a,self.normal,origin);
        let db=anchored_height(b,self.normal,origin);
        let epsilon=f64::from(self.epsilon);
        let intersection=if (da>epsilon&&db< -epsilon)||(db>epsilon&&da< -epsilon){
            let point=Vec3(std::array::from_fn(|axis|{
                if self.normal.0[axis]!=0.0 && (0..3).all(|other|other==axis||self.normal.0[other]==0.0){
                    origin.0[axis]
                }else{
                    ((da*f64::from(b.0[axis])-db*f64::from(a.0[axis]))/(da-db)) as f32
                }
            }));
            Some((point,da/(da-db)))
        }else{None};
        (da,db,intersection)
    }

    fn local_point(&self,point:Vec3)->Vec3{self.origin.map_or(point,|origin|point.sub(origin))}

    pub fn advance(&mut self, soup: &mut Soup) -> MeshResult<bool> {
        if self.completed{return Err(MeshKernelError::InvalidInput("clipping already completed".into()));}
        match self.phase {
            0 => {
                if self.face == soup.faces.len() {
                    if self.cap.len() < 3 { return Err(MeshKernelError::InvalidInput("bevel width does not intersect the solid".into())); }
                    self.phase = 1;
                } else {
                    let face = &soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let (da,_,intersection)=self.edge(Vec3(soup.positions[a as usize]),Vec3(soup.positions[b as usize]));
                    let epsilon=f64::from(self.epsilon);
                    if da <= epsilon {
                        if self.corners.last() != Some(&a) { self.corners.push(a); }
                        if da.abs()<=epsilon {self.cap.insert(a);self.cap_owners.entry(a).or_insert_with(||self.face as u32);}
                    }
                    if let Some((point,t))=intersection {
                        let id = *self.intersections.entry((a.min(b), a.max(b))).or_insert_with(|| {
                            let id = soup.positions.len() as u32;
                            soup.positions.push(point.0);self.vertex_sources.push(SourceVertex::Edge {a,b,t});
                            id
                        });
                        if self.corners.last() != Some(&id) { self.corners.push(id); }
                        self.cap.insert(id);self.cap_owners.entry(id).or_insert_with(||self.face as u32);
                    }
                    self.corner += 1;
                    if self.corner == face.len() {
                        if self.corners.first() == self.corners.last() { self.corners.pop(); }
                        if self.corners.len()>=3 {self.source_faces.push(self.face as u32);self.face_weights.push(vec![self.face as u32]);self.cursor=0;self.phase=7;}else {self.corners.clear();}
                        self.corner = 0;
                        self.face += 1;
                    }
                }
            }
            1 => {
                if let Some(id) = self.cap.pop_first() { self.cap_ids.push(id); }
                else { self.phase = 2; }
            }
            2 => {
                if self.corner == self.cap_ids.len() { self.phase = 3; self.corner = 0; }
                else {
                    let id = self.cap_ids[self.corner];
                    self.center = self.center.add(self.local_point(Vec3(soup.positions[id as usize])).scale(1.0 / self.cap_ids.len() as f32));
                    self.corner += 1;
                }
            }
            3 => {
                if self.corner == self.cap_ids.len() { self.phase = 4; }
                else {
                    let axis = if self.normal.x().abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
                    let u = self.normal.cross(axis).normalize();
                    let v = self.normal.cross(u);
                    let id = self.cap_ids[self.corner];
                    let point = self.local_point(Vec3(soup.positions[id as usize])).sub(self.center);
                    self.sorted.push(Reverse(Ranked { rank: point.dot(v).atan2(point.dot(u)), a: id, b: 0 }));
                    self.corner += 1;
                }
            }
            4 => {
                if let Some(Reverse(point))=self.sorted.pop() {self.corners.push(point.a);let face=self.cap_owners[&point.a];self.corner_faces.push(face);self.cap_weights.push(face);}
                else {
                    self.source_faces.push(*self.cap_weights.first().ok_or(MeshKernelError::DegenerateOperation)?);self.face_weights.push(std::mem::take(&mut self.cap_weights));self.clipped.push(std::mem::take(&mut self.corners));
                    let faces=std::mem::take(&mut self.clipped);
                    if self.origin.is_some(){self.retired_faces=std::mem::replace(&mut soup.faces,faces);self.phase=8;}
                    else{soup.faces=faces;self.completed=true;return Ok(true);}
                }
            }
            8=>if self.retired_faces.pop().is_none(){self.completed=true;return Ok(true);},
            6=>{if self.source_vertices==soup.positions.len() {self.phase=0;}else {self.vertex_sources.push(SourceVertex::Original(self.source_vertices as u32));self.source_vertices+=1;}},
            7=>{
                if self.cursor==self.corners.len(){
                    if self.origin.is_some()&&convex_intersection::length(self.ring_area)<=f64::from(self.epsilon).powi(2){
                        self.corner_faces.truncate(self.corner_faces.len()-self.corners.len());
                        self.source_faces.pop();self.face_weights.pop();self.corners.clear();
                    }else{self.clipped.push(std::mem::take(&mut self.corners));}
                    self.ring_area=[0.0;3];self.phase=0;
                }else{
                    if self.origin.is_some(){
                        let point=|index:usize|soup.positions[self.corners[index] as usize];
                        let anchor=point(0);
                        let a=convex_intersection::difference(point(self.cursor),anchor);
                        let b=convex_intersection::difference(point((self.cursor+1)%self.corners.len()),anchor);
                        for (sum,v) in self.ring_area.iter_mut().zip(convex_intersection::cross(a,b)){*sum+=v;}
                    }
                    self.corner_faces.push(*self.source_faces.last().unwrap());self.cursor+=1;
                }
            },
            _=>unreachable!(),
        }
        Ok(false)
    }
}

impl semio_framework_value::retirement::RetireOwned for MeshClipJob {
    type RetirementCursor=semio_framework_value::retirement_fields_type!(HistoryFoldIndex<(u32, u32), u32>,HistoryFoldSet<u32>,Vec<u32>,Vec<Vec<u32>>,Vec<u32>,BinaryHeap<Reverse<Ranked>>,Vec<SourceVertex>,Vec<u32>,Vec<u32>,Vec<Vec<u32>>,HistoryFoldIndex<u32,u32>,Vec<u32>,Vec<Vec<u32>>);
    fn retirement_seed(self)->Self::RetirementCursor {semio_framework_value::retirement_fields_seed!(self.intersections,self.cap,self.cap_ids,self.clipped,self.corners,self.sorted,self.vertex_sources,self.source_faces,self.corner_faces,self.face_weights,self.cap_owners,self.cap_weights,self.retired_faces)}
    fn controlled_retirement_supported()->bool {true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
