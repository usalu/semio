//! 🛠️ Retained modeling work, measured in vertices, corners, and candidate transitions.

use super::*;
use crate::brep::representation::vector::{Pnt3,Vec3 as GeometryVector};
use crate::brep::representation::vector::matrix::{Affine3,Mat3};
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

#[cfg(test)]
#[path = "../🧪️tests/🔬️jobs/🦀️.rs"]
mod tests;

/// 📊 Completed work and a conservative remaining-work estimate, finalized on completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeshModelingProgress {
    pub units_done: usize,
    pub units_total: usize,
    pub phase: &'static str,
}

/// 🧭 A budgeted modeling result; completed geometry is transferred exactly once.
#[derive(Debug)]
pub enum MeshModelingStep {
    Working(MeshModelingProgress),
    Done(HalfedgeMesh),
    Cancelled(MeshModelingProgress),
}

/// 🧵 Owns a source snapshot and retained geometry work without changing the source.
pub struct MeshModelingJob {
    work: Work,
    progress: MeshModelingProgress,
    cancelled: bool,
    retired: bool,
}

enum Work {
    Bevel(Box<Bevel>),
    Decimate(Box<Decimate>),
    Mirror(Box<Mirror>),
    Merge(Box<Merge>),
    Subdivide(Box<Subdivide>),
    LoopCut(Box<LoopCut>),
    Orient(Box<Orient>),
    FillHoles(Box<FillHoles>),
    Inflate(Box<Inflate>),
    Import(Box<Import>),
    Knife(Box<Knife>),
    Transform(Box<Transform>),
}

impl MeshModelingJob {
    /// 🔃️ Counts completed orientation changes without scanning the face roster.
    pub fn orientation_flip_count(&self)->Option<usize> {match &self.work {Work::Orient(work)=>Some(work.flip_count),_=>None}}
    /// 🕳️ Counts caps produced by the existing retained fill operation.
    pub fn filled_hole_count(&self)->Option<usize> {match &self.work {Work::FillHoles(work)=>Some(work.filled),_=>None}}
    /// 🧩 Counts accepted face unions in the existing merge owner.
    pub fn coplanar_merge_count(&self)->Option<usize> {match &self.work {Work::Merge(work)=>work.coplanar.as_ref().map(|work|work.merges),_=>None}}
    /// 🧲️ Counts precision-grid vertex identifications retained by the merge owner.
    pub fn welded_vertex_count(&self)->Option<usize> {match &self.work {Work::Merge(work) if work.quantized=>Some(work.removed),_=>None}}
    pub fn progress(&self) -> MeshModelingProgress { self.progress }
    /// 🧵 Transfers imported raw normals and corner identities once after reconstruction.
    pub fn take_imported_corner_normals(&mut self)->Option<(Vec<f32>,Vec<u32>)> { if !self.retired {return None;}match &mut self.work {Work::Import(work) if !work.normals.is_empty()=>Some((std::mem::take(&mut work.normals),std::mem::take(&mut work.indices))),_=>None} }


    /// 🧹️ Moves the same modeling Work payload into bounded typed retirement.
    pub fn into_retirement(self)->Box<dyn protocol::value::ErasedSnapshotRetirement> {protocol::value::retirement::owned_retirement(self.work)}

    pub fn cancel(&mut self) {
        if !self.retired { self.cancelled = true; }
    }

    pub(super) fn finish_with_progress(mut self, mut progress: impl FnMut(f32) -> bool) -> MeshResult<HalfedgeMesh> {
        loop {
            let value = self.progress();
            if !progress(value.units_done as f32 / value.units_total as f32) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
            match self.step(256)? {
                MeshModelingStep::Working(_) => {},
                MeshModelingStep::Done(mesh) => {
                    if !progress(1.0) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
                    return Ok(mesh);
                }
                MeshModelingStep::Cancelled(_) => return Err(MeshKernelError::InvalidInput("operation cancelled".into())),
            }
        }
    }

    pub fn step(&mut self, budget: usize) -> MeshResult<MeshModelingStep> {
        if self.retired { return Err(MeshKernelError::InvalidInput("modeling job already retired".into())); }
        if self.cancelled { return Ok(MeshModelingStep::Cancelled(self.progress)); }
        for _ in 0..budget {
            let next=self.progress.units_done.checked_add(1).ok_or_else(||MeshKernelError::InvalidInput("modeling progress overflow".into()))?;
            let result = match &mut self.work {
                Work::Bevel(work) => work.advance(),
                Work::Decimate(work) => work.advance(),
                Work::Mirror(work) => work.advance(),
                Work::Merge(work) => work.advance(),
                Work::Subdivide(work) => work.advance(),
                Work::LoopCut(work) => work.advance(),
                Work::Orient(work) => work.advance(),
                Work::FillHoles(work) => work.advance(),
                Work::Inflate(work) => work.advance(),
                Work::Import(work) => work.advance(),
                Work::Knife(work) => work.advance(),
                Work::Transform(work) => work.advance(),
            };
            let output = match result {
                Ok(output) => output,
                Err(error) => { self.retired = true; return Err(error); }
            };
            self.progress.units_done = next;
            self.progress.phase = match &self.work {
                Work::Bevel(work) => work.phase(),
                Work::Decimate(work) => work.phase(),
                Work::Mirror(work) => work.phase(),
                Work::Merge(work) => work.phase(),
                Work::Subdivide(work) => work.phase(),
                Work::LoopCut(work) => work.phase(),
                Work::Orient(work) => work.phase(),
                Work::FillHoles(work) => work.phase(),
                Work::Inflate(work) => work.phase(),
                Work::Import(work) => work.phase(),
                Work::Knife(work) => work.phase(),
                Work::Transform(work) => work.phase(),
            };
            if let Some(mut mesh) = output {
                let source=match &mut self.work {Work::Merge(work)=>Some(&mut work.snapshot.source),_=>None};
                if let Some(source)=source {if !source.materials.is_empty() {mesh.materials=std::mem::take(&mut source.materials);}if !source.textures.is_empty() {mesh.textures=std::mem::take(&mut source.textures);}}
                self.progress.units_total = self.progress.units_done;
                self.progress.phase = "done";
                self.retired = true;
                return Ok(MeshModelingStep::Done(mesh));
            }
            self.progress.units_total = self.progress.units_total.max(self.progress.units_done.saturating_add(1));
        }
        Ok(MeshModelingStep::Working(self.progress))
    }
}

impl HalfedgeMesh {
    /// 🧭 Retains a column-major affine map, inverse-transpose normals and reflection winding.
    pub fn affine_transform_job(&self,matrix:[f64;16])->MeshResult<MeshModelingJob> {self.clone().affine_transform_job_owned(matrix)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn affine_transform_job_owned(self,matrix:[f64;16])->MeshResult<MeshModelingJob> {
        let (affine,normal_matrix)=Self::affine_parameters(matrix)?;let determinant=affine.determinant();
        let mut job=self.transform_job_owned([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],[0.0;3],normal_matrix,determinant<0.0,true)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Matrix(affine);Ok(job)
    }
    /// 🛂 Admits constant-sized affine parameters before capturing any mesh source.
    pub fn validate_affine_matrix(matrix:[f64;16])->MeshResult<()> {Self::affine_parameters(matrix).map(|_|())}
    fn affine_parameters(matrix:[f64;16])->MeshResult<(Affine3,[[f64;3];3])> {
        if matrix.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::InvalidInput("matrix must contain finite numbers".into()));}
        if [matrix[3],matrix[7],matrix[11],matrix[15]]!=[0.0,0.0,0.0,1.0] {return Err(MeshKernelError::InvalidInput("matrix must be affine with last row [0,0,0,1]".into()));}
        let affine=Affine3 {linear:Mat3::from_rows([[matrix[0],matrix[4],matrix[8]],[matrix[1],matrix[5],matrix[9]],[matrix[2],matrix[6],matrix[10]]]),translation:GeometryVector::from_array([matrix[12],matrix[13],matrix[14]])};let determinant=affine.determinant();
        if !determinant.is_finite() || determinant==0.0 {return Err(MeshKernelError::InvalidInput("matrix must have a finite nonsingular linear part".into()));}
        let normal_matrix=affine.linear.cofactor().scaled(1.0/determinant).rows;
        if normal_matrix.iter().flatten().any(|value|!value.is_finite()) {return Err(MeshKernelError::InvalidInput("matrix inverse-transpose must be finite".into()));}
        Ok((affine,normal_matrix))
    }
    /// 🧩 Retains adjacency, coplanarity, loop splicing and collinear cleanup in the merge owner.
    pub fn merge_coplanar_faces_job(&self)->MeshResult<MeshModelingJob> {self.clone().merge_coplanar_faces_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn merge_coplanar_faces_job_owned(self)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,Some(Coplanar::default()),Vec::new(),WeldMode::Center,0.0,false))),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(24).saturating_add(source_vertices).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// ✂️ Retains explicit edge unions and source-channel provenance in the merge owner.
    pub fn dissolve_edges_job(&self,edges:&[EdgeId])->MeshResult<MeshModelingJob> {self.clone().dissolve_edges_job_owned(edges)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn dissolve_edges_job_owned(self,edges:&[EdgeId])->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if edges.is_empty() {return Err(MeshKernelError::EmptySelection);}let mut work=Coplanar::default();work.explicit=Some(edges.to_vec());work.preserve_corners=true;work.phase=23;
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,Some(work),Vec::new(),WeldMode::Center,0.0,false))),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(source_vertices).saturating_add(1),phase:"dissolve-selection"},cancelled:false,retired:false})
    }
    /// 🧵 Retains planar vertex stars and original boundary contributors in the merge owner.
    pub fn dissolve_vertices_job(&self,vertices:&[VertexId])->MeshResult<MeshModelingJob> {self.clone().dissolve_vertices_job_owned(vertices)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn dissolve_vertices_job_owned(self,vertices:&[VertexId])->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if vertices.is_empty() {return Err(MeshKernelError::EmptySelection);}let mut work=Coplanar::default();work.vertex_input=Some(vertices.to_vec());work.preserve_corners=true;work.phase=23;
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,Some(work),Vec::new(),WeldMode::Center,0.0,false))),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(vertices.len().saturating_mul(32)).saturating_add(source_vertices).saturating_add(1),phase:"dissolve-selection"},cancelled:false,retired:false})
    }
    /// ↔️ Retains whole-mesh translation by one vertex per unit.
    pub fn translate_job(&self,delta:Vec3)->MeshResult<MeshModelingJob> {self.clone().translate_job_owned(delta)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn translate_job_owned(self,delta:Vec3)->MeshResult<MeshModelingJob> {if delta.0.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.transform_job_owned([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],delta.0,[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],false,false)}
    /// 🔄 Retains rotation and each normal contribution in the same modeling owner.
    pub fn rotate_job(&self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob> {self.clone().rotate_job_owned(axis,angle)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn rotate_job_owned(self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob> {
        if !angle.is_finite() || axis.0.iter().any(|x|!x.is_finite()) || axis.0==[0.0;3] {return Err(MeshKernelError::DegenerateOperation);}let length=(axis.x() as f64).hypot(axis.y() as f64).hypot(axis.z() as f64);let ax=Vec3(axis.0.map(|x|(x as f64/length) as f32));let (x,y,z)=(ax.x(),ax.y(),ax.z());let (c,s)=(angle.cos(),angle.sin());let t=1.0-c;let matrix=[[t*x*x+c,t*x*y-s*z,t*x*z+s*y],[t*x*y+s*z,t*y*y+c,t*y*z-s*x],[t*x*z-s*y,t*y*z+s*x,t*z*z+c]];self.transform_job_owned(matrix,[0.0;3],matrix.map(|row|row.map(f64::from)),false,true)
    }
    /// 📐 Retains scale, optional winding repair and normal-channel transformation.
    pub fn scale_job(&self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob> {self.clone().scale_job_owned(factor,reverse_winding)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn scale_job_owned(self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob> {if factor.0.iter().any(|x|!x.is_finite() || *x==0.0) {return Err(MeshKernelError::DegenerateOperation);}self.transform_job_owned([[factor.x(),0.0,0.0],[0.0,factor.y(),0.0],[0.0,0.0,factor.z()]],[0.0;3],[[1.0/factor.x() as f64,0.0,0.0],[0.0,1.0/factor.y() as f64,0.0],[0.0,0.0,1.0/factor.z() as f64]],reverse_winding,true)}
    /// 🎯 Retains component expansion, centroid and translation through the existing transform work.
    pub fn move_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob> {self.clone().move_components_job_owned(vertices,edges,faces,delta)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn move_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob> {
        let job=self.translate_job_owned(delta)?;
        Self::select_transform(job,vertices,edges,faces,Some(Vec3::ZERO),[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],false,None)
    }
    /// 🌀 Retains component rotation and splits indexed normal aliases by selected domain.
    pub fn rotate_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {self.clone().rotate_components_job_owned(vertices,edges,faces,axis,angle,pivot)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn rotate_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {
        let job=self.rotate_job_owned(axis,angle)?;let length=(axis.x() as f64).hypot(axis.y() as f64).hypot(axis.z() as f64);let [x,y,z]=axis.0.map(|value|value as f64/length);let c=(angle as f64).cos();let t=1.0-c;let sin=(angle as f64).sin();let matrix=[[t*x*x+c,t*x*y-sin*z,t*x*z+sin*y],[t*x*y+sin*z,t*y*y+c,t*y*z-sin*x],[t*x*z-sin*y,t*y*z+sin*x,t*z*z+c]];
        Self::select_transform(job,vertices,edges,faces,pivot,matrix,true,Some(matrix))
    }
    /// 📏 Retains component scale and every authored normal domain contribution.
    pub fn scale_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {self.clone().scale_components_job_owned(vertices,edges,faces,factor,pivot)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn scale_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {
        let job=self.scale_job_owned(factor,false)?;
        Self::select_transform(job,vertices,edges,faces,pivot,[[factor.x() as f64,0.0,0.0],[0.0,factor.y() as f64,0.0],[0.0,0.0,factor.z() as f64]],true,None)
    }
    /// 🧲️ Retains selected grid rounding and geometry-normal recomputation.
    pub fn snap_vertices_job(&self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob> {self.clone().snap_vertices_job_owned(vertices,grid)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn snap_vertices_job_owned(self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob> {
        if !grid.is_finite() || grid<=0.0 {return Err(MeshKernelError::InvalidInput("grid must be finite and positive".into()));}let mut job=self.move_components_job_owned(vertices,Vec::new(),Vec::new(),Vec3::ZERO)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Grid(grid);Ok(job)
    }
    /// 🌊 Retains selection and one radial falloff contribution per vertex.
    pub fn move_proportional_job(&self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob> {self.clone().move_proportional_job_owned(vertices,delta,pivot,radius)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn move_proportional_job_owned(self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob> {
        if !radius.is_finite() || radius<=0.0 || pivot.0.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}let mut job=self.move_components_job_owned(vertices,Vec::new(),Vec::new(),delta)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Proportional {pivot,radius};Ok(job)
    }
    fn select_transform(mut job:MeshModelingJob,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,pivot:Option<Vec3>,matrix:[[f64;3];3],normals:bool,normal_matrix:Option<[[f64;3];3]>)->MeshResult<MeshModelingJob> {
        if vertices.is_empty() && edges.is_empty() && faces.is_empty() {return Err(MeshKernelError::EmptySelection);}
        if pivot.is_some_and(|pivot|pivot.0.iter().any(|value|!value.is_finite())) {return Err(MeshKernelError::DegenerateOperation);}
        let Work::Transform(work)=&mut job.work else {unreachable!()};work.selection=Some(TransformSelection {vertices,edges,faces,pivot,selected:BTreeSet::new(),sum:[0.0;3],mode:0,cursor:0,next:None,start:0});work.component_matrix=Some(matrix);if let Some(matrix)=normal_matrix {work.normal_matrix=matrix;}work.transform_attributes=normals;work.recompute=true;work.phase=9;job.progress.phase="transform-selection";Ok(job)
    }
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn transform_job_owned(self,matrix:[[f32;3];3],offset:[f32;3],normal_matrix:[[f64;3];3],flip:bool,recompute:bool)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        Ok(MeshModelingJob {work:Work::Transform(Box::new(Transform {mesh:self,policy:PositionPolicy::Affine,matrix,offset,normal_matrix,flip,recompute,flat:Vec::new(),smooth:Vec::new(),hes:Vec::new(),normal:Normal::default(),face_normal:Vec3::ZERO,face:0,vertex:0,cursor:0,start:0,next:0,attribute_names:Vec::new(),attribute:0,selection:None,component_matrix:None,component_pivot:[0.0;3],transform_attributes:recompute,attribute_values:Vec::new(),attribute_indices:Vec::new(),attribute_samples:BTreeMap::new(),attribute_retired:Vec::new(),attribute_affected:None,attribute_corner:None,phase:0})),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(8).saturating_add(source_vertices.saturating_mul(3)).saturating_add(1),phase:"transform-vertices"},cancelled:false,retired:false})
    }
    /// 🔪 Retains projection tests, concave tessellation, clipping and neighboring corners.
    pub fn knife_cut_job(&self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob> {self.clone().knife_cut_job_owned(face,a,b)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn knife_cut_job_owned(self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        self.face_boundary(face)?;if a.0.iter().chain(&b.0).any(|x|!x.is_finite()) {return Err(MeshKernelError::InvalidInput("knife points must be finite".into()));}
        Ok(MeshModelingJob {work:Work::Knife(Box::new(Knife {snapshot:Snapshot::new(self),face:face.0 as usize,a:a.0.map(f64::from),b:b.0.map(f64::from),normal:(0.0,0.0,0.0),plane:(0.0,0.0,0.0),origin:(0.0,0.0,0.0),scale:0.0,tolerance:0.0,distances:Vec::new(),positive:false,negative:false,planar:true,convex:true,tessellation:None,transfer:None,pieces:Vec::new(),piece:0,piece_positive:false,piece_negative:false,sign:1.0,split:Vec::new(),intersections:HashMap::new(),soup:Soup::default(),corners:Vec::new(),cursor:0,source_face:0,corner:0,emitted_corners:0,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase:0})),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(source_vertices).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// 📥 Retains position welding, corner identities and indexed reconstruction.
    pub fn indexed_triangle_job(positions:Vec<f32>,indices:Vec<u32>,normals:Vec<f32>)->MeshResult<MeshModelingJob> {
        if !positions.len().is_multiple_of(3) || !indices.len().is_multiple_of(3) || !normals.is_empty() && normals.len()!=positions.len() {return Err(MeshKernelError::InvalidInput("invalid indexed triangulation buffers".into()));}
        if indices.len()/3>100_000 || positions.len().saturating_add(indices.len()).saturating_add(normals.len()).saturating_mul(4)>64_000_000 {return Err(MeshKernelError::InvalidInput("indexed triangulation exceeds mesh capacity".into()));}
        let total=positions.len().saturating_add(indices.len().saturating_mul(16)).saturating_add(1);
        Ok(MeshModelingJob {work:Work::Import(Box::new(Import {polygon:None,primitive:None,primitive_faces:Vec::new(),midpoints:BTreeMap::new(),mids:Vec::new(),generation:0,face:0,part:0,positions,indices,normals,unique:HashMap::new(),remap:Vec::new(),soup:Soup::default(),corners:Vec::new(),corner_normals:Vec::new(),mesh:None,cursor:0,build:None,phase:0})),progress:MeshModelingProgress {units_done:0,units_total:total,phase:"mesh-import-vertices"},cancelled:false,retired:false})
    }
    /// 📦 Retains reconstruction of an admitted typed polygon source without cloning its channels.
    pub fn polygon_source_job(source:semio_framework_mesh_engine::PolygonMeshSource)->MeshResult<MeshModelingJob> {let mut job=Self::indexed_triangle_job(Vec::new(),Vec::new(),Vec::new())?;let Work::Import(work)=&mut job.work else {unreachable!()};work.polygon=Some(source);work.phase=10;job.progress.phase=work.phase();Ok(job)}
    /// 🧊 Retains each box vertex, face and reconstruction unit.
    pub fn box_primitive_job(width:f32,height:f32,depth:f32)->MeshResult<MeshModelingJob> {Self::primitive_job(0,[width,height,depth],0)}
    /// 🔳 Retains plane vertices and boundary reconstruction.
    pub fn plane_primitive_job(width:f32,depth:f32)->MeshResult<MeshModelingJob> {Self::primitive_job(1,[width,depth,0.0],0)}
    /// 🥫 Retains cylinder rings, cap corners and side faces.
    pub fn cylinder_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(2,[radius,height,0.0],segments)}
    /// 📐 Retains cone ring, apex, cap corners and side faces.
    pub fn cone_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(3,[radius,height,0.0],segments)}
    /// 🌐 Retains spherical edge midpoints, subdivision faces and radius application.
    pub fn sphere_primitive_job(radius:f32,subdivisions:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(4,[radius,0.0,0.0],subdivisions)}
    fn primitive_job(kind:u8,dimensions:[f32;3],parameter:u32)->MeshResult<MeshModelingJob> {
        let width=if kind==0 {3}else if kind==4 {1}else {2};if dimensions[..width].iter().any(|value|!value.is_finite() || *value<=0.0) || matches!(kind,2|3) && !(3..=100_000).contains(&parameter) || kind==4 && parameter>5 {return Err(MeshKernelError::InvalidInput("invalid primitive dimensions or subdivision".into()));}
        let (vertices,faces,corners)=match kind {0=>(8,6,24),1=>(4,1,4),2=>(parameter as usize*2,parameter as usize+2,parameter as usize*6),3=>(parameter as usize+1,parameter as usize+1,parameter as usize*4),_=>(10*4usize.pow(parameter)+2,20*4usize.pow(parameter),60*4usize.pow(parameter))};if vertices>100_000 || faces>100_000 || corners>600_000 {return Err(MeshKernelError::InvalidInput("primitive exceeds mesh capacity".into()));}
        let mut job=Self::indexed_triangle_job(Vec::new(),Vec::new(),Vec::new())?;let Work::Import(work)=&mut job.work else {unreachable!()};work.primitive=Some((kind,dimensions,parameter));work.phase=4;job.progress=MeshModelingProgress {units_done:0,units_total:vertices.saturating_add(corners.saturating_mul(16)).saturating_add(1),phase:work.phase()};Ok(job)
    }
    /// 🧱 Retains selected normals, displaced vertices, boundary sides and reconstruction.
    pub fn extrude_faces_job(&self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob> {self.clone().extrude_faces_job_owned(faces,distance)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn extrude_faces_job_owned(self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob> { self.inflate_job_owned(faces,distance,false) }
    /// 🔲 Retains each inset corner, inner ring and border before reconstruction.
    pub fn inset_faces_job(&self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob> {self.clone().inset_faces_job_owned(faces,amount)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn inset_faces_job_owned(self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob> { self.inflate_job_owned(faces,amount,true) }
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn inflate_job_owned(self,faces:&[FaceId],amount:f32,inset:bool)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}if !amount.is_finite() || amount==0.0 || inset && amount<0.0 {return Err(MeshKernelError::DegenerateOperation);}
        let work=Inflate { snapshot:Snapshot::new(self),input:faces.to_vec(),selected:HashSet::new(),amount,inset,normals:BTreeMap::new(),boundary:HashMap::new(),edges:BTreeSet::new(),displaced:HashMap::new(),last_vertex:None,last_edge:None,soup:Soup::default(),borders:Vec::new(),inner:Vec::new(),corners:Vec::new(),cursor:0,face:0,corner:0,emitted_corners:0,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),border_faces:Vec::new(),attributes:None,phase:0 };
        let phase=work.phase();Ok(MeshModelingJob {work:Work::Inflate(Box::new(work)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(1),phase},cancelled:false,retired:false})
    }
    /// 🧭 Retains incidence, component traversal, polygon reversal and reconstruction.
    pub fn orient_faces_job(&self)->MeshResult<MeshModelingJob> {self.clone().orient_faces_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn orient_faces_job_owned(self)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work:Work::Orient(Box::new(Orient {input:None,flip_count:0,attributes:None,snapshot:Snapshot::new(self),edges:HashMap::new(),keys:Vec::new(),adjacency:Vec::new(),oriented:Vec::new(),flipped:Vec::new(),stack:Vec::new(),active:None,face:0,corner:0,cursor:0,neighbor:0,reversing:0,build:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul(16).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// 🔄 Retains selected winding changes through existing orientation reconstruction.
    pub fn flip_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob> {self.clone().flip_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn flip_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob> {if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}let mut job=self.orient_faces_job_owned()?;let Work::Orient(work)=&mut job.work else {unreachable!()};work.input=Some(faces.to_vec());Ok(job)}
    /// 🕳️ Retains boundary rotations and cap corners before reconstruction.
    pub fn fill_holes_job(&self)->MeshResult<MeshModelingJob> {self.clone().fill_holes_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn fill_holes_job_owned(self)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work:Work::FillHoles(Box::new(FillHoles { snapshot:Snapshot::new(self),visited:HashSet::new(),vertices:Vec::new(),boundary_edges:Vec::new(),cap:Vec::new(),cap_faces:Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),face_sources:Vec::new(),attributes:None,cursor:0,start:0,probe:0,rotations:0,corner:0,corners:source_corners,filled:0,build:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul(16).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// ✂️ Retains strip propagation, exact expansion admission and grid construction.
    pub fn loop_cut_job(&self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob> {self.clone().loop_cut_job_owned(edges,cuts)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn loop_cut_job_owned(self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !(1..=256).contains(&cuts) { return Err(MeshKernelError::InvalidInput("cuts must be in 1..=256".into())); }
        Ok(MeshModelingJob { work:Work::LoopCut(Box::new(LoopCut { snapshot:Snapshot::new(self),edges:edges.to_vec(),cuts:cuts as usize,incidence:HashMap::new(),marked:BTreeSet::new(),pending:Vec::new(),cut_vertices:HashMap::new(),last_edge:None,cut_edge:None,cut_ids:Vec::new(),soup:Soup::default(),boundary:Vec::new(),grid:Vec::new(),face:0,corner:0,cursor:0,step:0,nx:1,ny:1,vertices:0,faces:0,corners:0,building_grid:true,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul((cuts as usize+1).saturating_mul(32)).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// 🕸️ Retains face convexity, concave triangulation, centroid fans and reconstruction.
    pub fn subdivide_faces_job(&self, faces: &[FaceId])->MeshResult<MeshModelingJob> {self.clone().subdivide_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn subdivide_faces_job_owned(self, faces: &[FaceId])->MeshResult<MeshModelingJob> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        self.subdivision_job_owned(faces,false,false)
    }
    /// 🔺 Retains triangulation and exact source channel mapping in existing subdivision work.
    pub fn triangulate_job(&self)->MeshResult<MeshModelingJob> {self.clone().triangulate_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn triangulate_job_owned(self)->MeshResult<MeshModelingJob> { self.subdivision_job_owned(&[],true,false) }
    /// 🧹️ Deletes selected faces through the existing retained subdivision reconstruction.
    pub fn delete_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob> {self.clone().delete_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn delete_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob> {if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}self.subdivision_job_owned(faces,false,true)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn subdivision_job_owned(self,faces:&[FaceId],triangulate_only:bool,delete_selected:bool)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work: Work::Subdivide(Box::new(Subdivide { snapshot: Snapshot::new(self), input: faces.to_vec(),triangulate_only,delete_selected, selected: HashSet::new(), concave: HashSet::new(), convex: true, tessellation: None, transfer: None, soup: Soup::default(), corners: Vec::new(), center: Vec3::ZERO, center_id: 0, emitted_corners: 0, emitting: false, cursor: 0, face: 0, corner: 0, triangle: 0, build: None, vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase: if triangulate_only {1}else {0} })), progress: MeshModelingProgress { units_done: 0, units_total: source_corners.saturating_mul(32).saturating_add(source_vertices.saturating_mul(2)).saturating_add(1), phase: if triangulate_only {"snapshot"}else {"subdivide-selection"} }, cancelled: false, retired: false })
    }
    /// 🧲️ Retains vertex-distance pairs, root walks, polygon remapping and reconstruction.
    pub fn merge_vertices_job(&self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob> {self.clone().merge_vertices_job_owned(vertices,mode,threshold)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn merge_vertices_job_owned(self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if vertices.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if mode == WeldMode::ByDistance && (!threshold.is_finite() || threshold < 0.0) { return Err(MeshKernelError::DegenerateOperation); }
        Ok(MeshModelingJob { work: Work::Merge(Box::new(Merge::new(self,None,vertices.to_vec(),mode,threshold,false))), progress: MeshModelingProgress { units_done: 0, units_total: vertices.len().saturating_mul(vertices.len()).saturating_mul(4).saturating_add(source_corners.saturating_mul(12)).saturating_add(1), phase: "merge-selection" }, cancelled: false, retired: false })
    }
    /// 🧲️ Retains precision-grid welding and owned source contributors in the merge work.
    pub fn weld_coincident_vertices_job(&self,precision:f32)->MeshResult<MeshModelingJob> {self.clone().weld_coincident_vertices_job_owned(precision)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn weld_coincident_vertices_job_owned(self,precision:f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if !precision.is_finite() || precision<=0.0 {return Err(MeshKernelError::InvalidInput("weld precision must be positive and finite".into()));}
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,None,Vec::new(),WeldMode::First,precision,true))),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(24).saturating_add(source_vertices.saturating_mul(8)).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// 🪞️ Retains one-sided reflection, seam sharing and reconstruction in the modeling owner.
    pub fn mirror_job(&self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob> {self.clone().mirror_job_owned(axis,threshold)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn mirror_job_owned(self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if !threshold.is_finite() || threshold < 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let axis = match axis { MirrorAxis::X => 0, MirrorAxis::Y => 1, MirrorAxis::Z => 2 };
        Ok(MeshModelingJob { work: Work::Mirror(Box::new(Mirror { source_vertices:Vec::new(),source_faces:Vec::new(),mirrored_faces:Vec::new(),attribute_vertices:Vec::new(),attributes:None,snapshot: Snapshot::new(self), axis, tolerance: threshold * 0.5, positive: false, negative: false, reflected: Vec::new(), soup: Soup::default(), original: Vec::new(), mirrored: Vec::new(), emitted_corners: 0, plane: true, vertex: 0, face: 0, corner: 0, build: None, phase: 0 })), progress: MeshModelingProgress { units_done: 0, units_total: source_vertices.saturating_mul(4).saturating_add(source_corners.saturating_mul(16)).saturating_add(1), phase: "snapshot" }, cancelled: false, retired: false })
    }
    /// 🪚 Captures a segmented bevel as retained, cancellable geometry work.
    pub fn bevel_job(&self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob> {self.clone().bevel_job_owned(edges,amount,segments)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn bevel_job_owned(self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertices.len();let source_corners=self.halfedges.len();let source_faces=self.faces.len();
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !amount.is_finite() || amount <= 0.0 || !(1..=64).contains(&segments) { return Err(MeshKernelError::InvalidInput("bevel requires positive finite width and 1..=64 segments".into())); }
        let work = source_vertices.saturating_mul(source_faces).saturating_add(edges.len().saturating_mul(segments as usize).saturating_mul(source_corners));
        if work > 4_000_000 { return Err(MeshKernelError::InvalidInput("bevel work budget exceeded".into())); }
        for edge in edges {
            let halfedge = self.halfedges.get(edge.0 as usize).ok_or(MeshKernelError::InvalidHandle)?;
            if halfedge.twin.is_none() { return Err(MeshKernelError::NonManifold); }
        }
        Ok(MeshModelingJob {
            work: Work::Bevel(Box::new(Bevel::new(self, edges.to_vec(), amount, segments))),
            progress: MeshModelingProgress { units_done: 0, units_total: work.saturating_mul(16).saturating_add(1), phase: "snapshot" },
            cancelled: false, retired: false,
        })
    }

    /// 🪶 Captures safe edge collapses as retained, cancellable candidate work.
    pub fn decimate_job(&self, ratio: f32)->MeshResult<MeshModelingJob> {self.clone().decimate_job_owned(ratio)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    pub fn decimate_job_owned(self, ratio: f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedges.len();
        if !ratio.is_finite() || ratio <= 0.0 || ratio > 1.0 { return Err(MeshKernelError::InvalidInput("decimation ratio must be in (0,1]".into())); }
        let initial = source_vertices;
        let target = ((initial as f32 * ratio).ceil() as usize).max(4);
        let work = initial.saturating_mul(source_corners).saturating_mul(initial.saturating_sub(target));
        if work > 32_000_000 { return Err(MeshKernelError::InvalidInput("decimation work budget exceeded".into())); }
        Ok(MeshModelingJob {
            work: Work::Decimate(Box::new(Decimate::new(self, target))),
            progress: MeshModelingProgress { units_done: 0, units_total: work.saturating_mul(16).saturating_add(1), phase: "snapshot" },
            cancelled: false, retired: false,
        })
    }
}

struct TransformSelection {
    vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,pivot:Option<Vec3>,selected:BTreeSet<usize>,sum:[f64;3],mode:u8,cursor:usize,next:Option<u32>,start:u32,
}
impl TransformSelection {
    fn add(&mut self,mesh:&HalfedgeMesh,id:usize)->MeshResult<()> {let point=mesh.vertices.get(id).ok_or(MeshKernelError::InvalidHandle)?.position;if self.selected.insert(id) {for axis in 0..3 {self.sum[axis]+=point[axis] as f64;}}Ok(())}
    fn advance(&mut self,mesh:&HalfedgeMesh)->MeshResult<bool> {
        match self.mode {
            0=>{if self.cursor==self.vertices.len() {self.cursor=0;self.mode=1;}else {self.add(mesh,self.vertices[self.cursor].0 as usize)?;self.cursor+=1;}}
            1=>{if self.cursor==self.edges.len() {self.cursor=0;self.mode=2;}else {let (a,b)=mesh.edge_endpoints(self.edges[self.cursor])?;self.add(mesh,a.0 as usize)?;self.add(mesh,b.0 as usize)?;self.cursor+=1;}}
            2=>{if self.cursor==self.faces.len() {return Ok(true)}if let Some(next)=self.next {let halfedge=mesh.halfedges.get(next as usize).ok_or(MeshKernelError::InvalidHandle)?;let(vertex,next)=(halfedge.vertex,halfedge.next);self.add(mesh,vertex as usize)?;self.next=if next==self.start {self.cursor+=1;None}else {Some(next)};}else {self.start=mesh.face_boundary(self.faces[self.cursor])?.0.0;self.next=Some(self.start);}}
            _=>unreachable!(),
        }Ok(false)
    }
}
enum PositionPolicy { Affine,Matrix(Affine3),Grid(f32),Proportional{pivot:Vec3,radius:f32} }
struct Transform {
    mesh:HalfedgeMesh,
    policy:PositionPolicy,
    matrix:[[f32;3];3],
    offset:[f32;3],
    normal_matrix:[[f64;3];3],
    flip:bool,
    recompute:bool,
    flat:Vec<Option<Vec3>>,
    smooth:Vec<Vec3>,
    hes:Vec<u32>,
    normal:Normal,
    face_normal:Vec3,
    face:usize,
    vertex:usize,
    cursor:usize,
    start:u32,
    next:u32,
    attribute_names:Vec<String>,
    attribute:usize,
    selection:Option<TransformSelection>,
    component_matrix:Option<[[f64;3];3]>,
    component_pivot:[f32;3],
    transform_attributes:bool,
    attribute_values:Vec<protocol::value::DslValue>,
    attribute_indices:Vec<u32>,
    attribute_samples:BTreeMap<(u32,bool),u32>,
    attribute_retired:Vec<protocol::value::DslValue>,
    attribute_affected:Option<bool>,
    attribute_corner:Option<u32>,
    phase:u8,
}
impl Transform {
    fn directional_value(value:&protocol::value::DslValue,matrix:[[f64;3];3],tangent:bool,reflected:bool)->MeshResult<protocol::value::DslValue> {
        let values=value.as_array().ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction".into()))?;
        if values.len()!=if tangent {4}else {3} {return Err(MeshKernelError::InvalidInput("invalid authored direction dimensions".into()));}
        let coordinate=|axis:usize|values[axis].as_f64().filter(|value|value.is_finite()).ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction coordinate".into()));let point=[coordinate(0)?,coordinate(1)?,coordinate(2)?];
        let direction=[0,1,2].map(|axis|matrix[axis][0]*point[0]+matrix[axis][1]*point[1]+matrix[axis][2]*point[2]);let length=direction[0].hypot(direction[1]).hypot(direction[2]);let direction=direction.map(|value|value/length);
        if direction.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::DegenerateOperation);}
        let mut output=direction.into_iter().map(protocol::value::DslValue::float).collect::<Vec<_>>();
        if tangent {let handedness=values[3].as_f64().filter(|value|*value==1.0 || *value== -1.0).ok_or_else(||MeshKernelError::InvalidInput("invalid authored tangent handedness".into()))?;output.push(protocol::value::DslValue::float(if reflected {-handedness}else {handedness}));}
        Ok(protocol::value::DslValue::Array(output))
    }
    fn directional_matrix(&self,tangent:bool)->([[f64;3];3],bool) {if !tangent {return (self.normal_matrix,false)}let matrix=match self.policy {PositionPolicy::Matrix(matrix)=>matrix.linear.rows,_=>self.component_matrix.unwrap_or_else(||self.matrix.map(|row|row.map(f64::from)))};(matrix,Mat3::from_rows(matrix).determinant()<0.0)}
    fn phase(&self)->&'static str {["transform-vertices","transform-winding","transform-initialize","transform-corners","transform-normal","transform-scatter","transform-vertex-normal","transform-attributes","transform-done","transform-selection","transform-attribute-release","transform-selection-release","transform-attribute-face"][self.phase as usize]}
    fn halfedge(&self,index:usize)->u32 {self.hes[if self.mesh.faces[self.face].flipped {self.hes.len()-1-index}else {index}]}
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if self.vertex==self.mesh.vertex_count() {self.vertex=0;self.phase=if self.recompute {1}else {8};}else {
                    let selected=self.selection.as_ref().is_none_or(|selection|selection.selected.contains(&self.vertex));
                    if selected || matches!(self.policy,PositionPolicy::Proportional {..}) {
                        let p=self.mesh.vertices[self.vertex].position;let result=match self.policy {
                            PositionPolicy::Matrix(matrix)=>matrix.apply_point(Pnt3::from_array(p.map(f64::from))).to_array().map(|value|value as f32),
                            PositionPolicy::Grid(grid)=>p.map(|coordinate|((coordinate as f64/grid as f64).round()*grid as f64) as f32),
                            PositionPolicy::Proportional {pivot,radius}=>{let weight=if selected {1.0}else {(1.0-Vec3(p).sub(pivot).length()/radius).max(0.0)};Vec3(p).add(Vec3(self.offset).scale(weight)).0},
                            PositionPolicy::Affine=>if let Some(matrix)=self.component_matrix {let relative=[0,1,2].map(|axis|p[axis] as f64-self.component_pivot[axis] as f64);[0,1,2].map(|axis|(self.component_pivot[axis] as f64+matrix[axis][0]*relative[0]+matrix[axis][1]*relative[1]+matrix[axis][2]*relative[2]+self.offset[axis] as f64) as f32)}else {[0,1,2].map(|axis|self.matrix[axis][0]*p[0]+self.matrix[axis][1]*p[1]+self.matrix[axis][2]*p[2]+self.offset[axis])},
                        };if result.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.mesh.vertices[self.vertex].position=result;
                    }self.vertex+=1;
                }
            }

            1=>{if !self.flip || self.face==self.mesh.face_count() {self.face=0;self.phase=2;}else {self.mesh.faces[self.face].flipped=!self.mesh.faces[self.face].flipped;self.face+=1;}}
            2=>{if self.vertex==self.mesh.vertex_count() {self.vertex=0;self.phase=3;}else {self.flat.push(None);self.smooth.push(Vec3::ZERO);self.vertex+=1;}}
            3=>{
                if self.face==self.mesh.face_count() {self.phase=6;self.vertex=0;}
                else {if self.hes.is_empty() {self.start=self.mesh.faces[self.face].halfedge;self.next=self.start;}let edge=&self.mesh.halfedges[self.next as usize];self.hes.push(self.next);self.next=edge.next;if self.next==self.start {self.cursor=0;self.phase=4;}}
            }
            4=>{let n=self.hes.len();let point=|i:usize|Vec3(self.mesh.vertices[self.mesh.halfedges[self.halfedge(i%n) as usize].vertex as usize].position);let (a,b)=(point(self.cursor),point(self.cursor+1));if self.cursor==0 {self.normal.origin=a;}self.normal.add(a,b);self.cursor+=1;if self.cursor==n {self.face_normal=self.normal.value();self.cursor=0;self.phase=5;}}
            5=>{let vertex=self.mesh.halfedges[self.halfedge(self.cursor) as usize].vertex as usize;if self.flat[vertex].is_none() {self.flat[vertex]=Some(self.face_normal);}if self.mesh.faces[self.face].smooth {self.smooth[vertex]=self.smooth[vertex].add(self.face_normal);}self.cursor+=1;if self.cursor==self.hes.len() {self.face+=1;self.hes.clear();self.normal=Normal::default();self.phase=3;}}
            6=>{if self.vertex==self.mesh.vertex_count() {if !self.transform_attributes {self.phase=if self.selection.is_some() {11}else {8};return Ok(None)}self.attribute_names=self.mesh.attributes.iter().filter_map(|(name,attribute)|(attribute.semantic==MeshAttributeSemantic::Normal || name=="tangent" && attribute.semantic==MeshAttributeSemantic::Custom).then_some(name.clone())).collect();self.attribute=0;self.cursor=0;self.phase=7;}else {self.mesh.vertices[self.vertex].normal=if self.smooth[self.vertex].length()>0.0 {Some(self.smooth[self.vertex].normalize().0)}else {self.flat[self.vertex].map(|n|n.0)};self.vertex+=1;}}
            7=>{
                if self.attribute==self.attribute_names.len() {self.phase=if self.selection.is_some() {11}else {8};}
                else {
                    let name=&self.attribute_names[self.attribute];let attribute=self.mesh.attributes.get(name).unwrap();let (matrix,reflected)=self.directional_matrix(name=="tangent");
                    if self.selection.is_some() {
                        if self.cursor==attribute.domain_len() {let attribute=self.mesh.attributes.get_mut(name).unwrap();self.attribute_retired=std::mem::replace(&mut attribute.values,std::mem::take(&mut self.attribute_values));attribute.indices=Some(std::mem::take(&mut self.attribute_indices));self.cursor=0;self.phase=10;}
                        else if attribute.domain==MeshAttributeDomain::Face && self.attribute_affected.is_none() {self.attribute_corner=Some(self.mesh.faces[self.cursor].halfedge);self.phase=12;}
                        else {
                            let affected=if attribute.domain==MeshAttributeDomain::Face {self.attribute_affected.take().unwrap()}else {let vertex=if attribute.domain==MeshAttributeDomain::Corner {self.mesh.halfedges[self.cursor].vertex as usize}else {self.cursor};self.selection.as_ref().unwrap().selected.contains(&vertex)};
                            let source=attribute.indices.as_ref().map_or(self.cursor as u32,|indices|indices[self.cursor]);let sample=if let Some(sample)=self.attribute_samples.get(&(source,affected)) {*sample}else {let original=&attribute.values[source as usize];let value=if affected {Self::directional_value(original,matrix,name=="tangent",reflected)?}else {original.clone()};let sample=self.attribute_values.len() as u32;self.attribute_values.push(value);self.attribute_samples.insert((source,affected),sample);sample};self.attribute_indices.push(sample);self.cursor+=1;
                        }
                    }else if self.cursor==attribute.values.len() {self.cursor=0;self.attribute+=1;}else {let value=Self::directional_value(&attribute.values[self.cursor],matrix,name=="tangent",reflected)?;self.mesh.attributes.get_mut(name).unwrap().values[self.cursor]=value;self.cursor+=1;}
                }
            }

            8=>return Ok(Some(std::mem::replace(&mut self.mesh,HalfedgeMesh::empty()))),
            9=>{let selection=self.selection.as_mut().unwrap();if selection.advance(&self.mesh)? {if selection.selected.is_empty() {return Err(MeshKernelError::EmptySelection);}self.component_pivot=selection.pivot.map_or_else(||selection.sum.map(|value|(value/selection.selected.len() as f64) as f32),|pivot|pivot.0);self.phase=0;}}
            10=>{if self.attribute_retired.pop().is_none() && self.attribute_samples.pop_first().is_none() {self.attribute+=1;self.phase=7;}}
            11=>{if self.selection.as_mut().unwrap().selected.pop_first().is_none() {self.phase=8;}}

            12=>{let corner=self.attribute_corner.unwrap();let edge=&self.mesh.halfedges[corner as usize];let affected=self.selection.as_ref().unwrap().selected.contains(&(edge.vertex as usize));if self.attribute_affected.is_some_and(|previous|previous!=affected) {return Err(MeshKernelError::InvalidInput("face direction cannot represent a mixed component transform".into()));}self.attribute_affected=Some(affected);let next=edge.next;if next==self.mesh.faces[self.cursor].halfedge {self.attribute_corner=None;self.phase=7;}else {self.attribute_corner=Some(next);}}
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Knife {
    snapshot:Snapshot,
    face:usize,
    a:[f64;3],
    b:[f64;3],
    normal:Vec3f64,
    plane:Vec3f64,
    origin:Vec3f64,
    scale:f64,
    tolerance:f64,
    distances:Vec<f64>,
    positive:bool,
    negative:bool,
    planar:bool,
    convex:bool,
    tessellation:Option<MeshTessellationJob>,
    transfer:Option<MeshTransfer>,
    pieces:Vec<Vec<u32>>,
    piece:usize,
    piece_positive:bool,
    piece_negative:bool,
    sign:f64,
    split:Vec<Vec<u32>>,
    intersections:HashMap<(u32,u32),u32>,
    soup:Soup,
    corners:Vec<u32>,
    cursor:usize,
    source_face:usize,
    corner:usize,
    emitted_corners:usize,
    build:Option<Build>,
    vertex_sources:Vec<SourceVertex>,
    source_faces:Vec<u32>,
    attributes:Option<SourceAttributeRemap>,
    phase:u8,
}
impl Knife {
    fn tuple(p:[f32;3])->Vec3f64 {(p[0] as f64,p[1] as f64,p[2] as f64)}
    fn phase(&self)->&'static str {["snapshot","knife-scale","knife-distances","knife-projection","knife-triangulate","knife-pieces","knife-classify","knife-clip","knife-neighbors","knife-reconstruct","knife-source-vertices","knife-source-remap","knife-attributes"][self.phase as usize]}
    fn push_corner(&mut self,id:u32) {if self.corners.last()!=Some(&id) {self.corners.push(id);}}
    fn finish_piece(&mut self)->MeshResult<()> {if self.corners.first()==self.corners.last() {self.corners.pop();}if self.corners.len()<3 {return Err(MeshKernelError::InvalidInput("knife cut produces a degenerate piece".into()));}if self.snapshot.soup.faces.len().saturating_sub(1).saturating_add(self.split.len()).saturating_add(1)>100_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.split.push(std::mem::take(&mut self.corners));Ok(())}
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.advance() {self.normal=Self::tuple(self.snapshot.soup.normals[self.face].0);let direction=sub3((self.b[0],self.b[1],self.b[2]),(self.a[0],self.a[1],self.a[2]));let plane=cross3(direction,self.normal);let length=length3(plane);if length==0.0 || length<=length3(direction)*1e-7 {return Err(MeshKernelError::InvalidInput("knife direction must project onto a nondegenerate face".into()));}self.plane=(plane.0/length,plane.1/length,plane.2/length);self.origin=Self::tuple(self.snapshot.soup.positions[self.snapshot.soup.faces[self.face][0] as usize]);self.cursor=0;self.phase=10;}}
            1=>{let face=&self.snapshot.soup.faces[self.face];let point=Self::tuple(self.snapshot.soup.positions[face[self.cursor] as usize]);self.scale=self.scale.max(length3(sub3(point,self.origin)));self.cursor+=1;if self.cursor==face.len() {self.tolerance=self.scale*1e-7;self.cursor=0;self.phase=2;}}
            2=>{
                if self.cursor==self.snapshot.soup.positions.len() {self.phase=3;self.cursor=0;}
                else {let d=dot3(sub3(Self::tuple(self.snapshot.soup.positions[self.cursor]),(self.a[0],self.a[1],self.a[2])),self.plane);self.distances.push(if d.abs()<=self.tolerance {0.0}else {d});self.cursor+=1;}
            }
            3=>{
                let face=&self.snapshot.soup.faces[self.face];let n=face.len();let point=|i:usize|Self::tuple(self.snapshot.soup.positions[face[i%n] as usize]);let distance=self.distances[face[self.cursor] as usize];self.positive|=distance>0.0;self.negative|=distance<0.0;self.planar&=dot3(sub3(point(self.cursor),self.origin),self.normal).abs()<=self.tolerance;self.convex&=dot3(cross3(sub3(point(self.cursor+1),point(self.cursor)),sub3(point(self.cursor+2),point(self.cursor+1))),self.normal)>=-self.scale*self.tolerance;self.cursor+=1;
                if self.cursor==n {if !self.positive || !self.negative {return Err(MeshKernelError::InvalidInput("knife line must cross the face interior".into()));}if !self.planar || !self.convex {self.tessellation=Some(MeshTessellationJob::selected(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty()),HashSet::from([self.face as u32])));}self.cursor=0;self.phase=4;}
            }
            4=>{if let Some(job)=&mut self.tessellation {if let MeshTessellationStep::Done(transfer)=job.step(1)? {self.transfer=Some(transfer);self.snapshot.source=self.tessellation.take().unwrap().into_source();self.phase=5;}}else {self.phase=5;}}
            5=>{
                if let Some(transfer)=&self.transfer {if self.cursor==transfer.indices.len() {self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=6;}else {self.pieces.push([0,1,2].map(|i|transfer.vertex_ids[transfer.indices[self.cursor+i] as usize]).to_vec());self.cursor+=3;}}
                else {let face=&self.snapshot.soup.faces[self.face];if self.cursor==face.len() {self.pieces.push(std::mem::take(&mut self.corners));self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=6;}else {self.corners.push(face[self.cursor]);self.cursor+=1;}}
            }
            6=>{
                if self.piece==self.pieces.len() {self.phase=8;self.cursor=0;self.source_face=0;self.corner=0;}
                else {let piece=&self.pieces[self.piece];if self.cursor==piece.len() {self.cursor=0;self.sign=1.0;self.phase=7;}else {let d=self.distances[piece[self.cursor] as usize];self.piece_positive|=d>0.0;self.piece_negative|=d<0.0;self.cursor+=1;}}
            }
            7=>{
                let piece=&self.pieces[self.piece];
                if self.cursor==piece.len() {
                    self.finish_piece()?;if self.piece_positive && self.piece_negative && self.sign>0.0 {self.sign=-1.0;self.cursor=0;}else {self.piece+=1;self.cursor=0;self.piece_positive=false;self.piece_negative=false;self.phase=6;}
                }else {
                    let (a,b)=(piece[self.cursor],piece[(self.cursor+1)%piece.len()]);let (da,db)=(self.distances[a as usize],self.distances[b as usize]);
                    if !self.piece_positive || !self.piece_negative {self.push_corner(a);}else {
                        if da*self.sign>=0.0 {self.push_corner(a);}if da>0.0 && db<0.0 || da<0.0 && db>0.0 {
                            let edge=(a.min(b),a.max(b));let id=if let Some(&id)=self.intersections.get(&edge) {id}else {if self.soup.positions.len()>=100_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}let (a,b)=(edge.0 as usize,edge.1 as usize);let t=self.distances[a]/(self.distances[a]-self.distances[b]);let point=[0,1,2].map(|axis|((1.0-t)*self.soup.positions[a][axis] as f64+t*self.soup.positions[b][axis] as f64) as f32);if point.iter().any(|x|!x.is_finite()) || point==self.soup.positions[a] || point==self.soup.positions[b] {return Err(MeshKernelError::InvalidInput("knife intersection collapses at this precision".into()));}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Edge {a:edge.0,b:edge.1,t});self.intersections.insert(edge,id);id};self.push_corner(id);
                        }
                    }self.cursor+=1;
                }
            }
            8=>{
                if self.source_face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=9;}
                else if self.source_face==self.face {if self.cursor==self.split.len() {self.source_face+=1;self.cursor=0;}else {let corners=self.split[self.cursor].len();if self.emitted_corners.saturating_add(corners)>600_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.emitted_corners+=corners;self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.split[self.cursor]));self.cursor+=1;}}
                else {let face=&self.snapshot.soup.faces[self.source_face];let (a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);let intersection=self.intersections.get(&(a.min(b),a.max(b))).copied();let n=1+usize::from(intersection.is_some());if self.emitted_corners.saturating_add(n)>600_000 {return Err(MeshKernelError::InvalidInput("knife cut exceeds mesh capacity".into()));}self.emitted_corners+=n;self.corners.push(a);if let Some(id)=intersection {self.corners.push(id);}self.corner+=1;if self.corner==face.len() {self.source_faces.push(self.source_face as u32);self.soup.faces.push(std::mem::take(&mut self.corners));self.corner=0;self.source_face+=1;}}
            }
            9=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().generated_vertices=Some(Vec::new());self.cursor=0;self.phase=11;}},
            10=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            11=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.phase=12;}else {self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            12=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Import {
    polygon:Option<semio_framework_mesh_engine::PolygonMeshSource>,
    primitive:Option<(u8,[f32;3],u32)>,
    primitive_faces:Vec<Vec<u32>>,
    midpoints:BTreeMap<(u32,u32),u32>,
    mids:Vec<u32>,
    generation:u32,
    face:usize,
    part:usize,
    positions:Vec<f32>,
    indices:Vec<u32>,
    normals:Vec<f32>,
    unique:HashMap<[u32;3],u32>,
    remap:Vec<u32>,
    soup:Soup,
    corners:Vec<u32>,
    corner_normals:Vec<protocol::value::DslValue>,
    mesh:Option<HalfedgeMesh>,
    cursor:usize,
    build:Option<Build>,
    phase:u8,
}
impl Import {
    fn phase(&self)->&'static str { ["mesh-import-vertices","mesh-import-corners","mesh-import-reconstruct","mesh-import-normals","mesh-primitive-vertices","mesh-primitive-faces","mesh-primitive-level","mesh-primitive-midpoints","mesh-primitive-release","mesh-primitive-radius","mesh-source-capture","mesh-source-assets"][self.phase as usize] }
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if self.cursor==self.positions.len() {self.phase=1;self.cursor=0;}
                else {let point=[0,1,2].map(|axis|self.positions[self.cursor+axis]);if point.iter().any(|x|!x.is_finite()) || !self.normals.is_empty() && self.normals[self.cursor..self.cursor+3].iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::InvalidInput("indexed triangulation contains non-finite values".into()));}let key=point.map(|x|if x==0.0 {0}else {x.to_bits()});let id=if let Some(&id)=self.unique.get(&key) {id}else {if self.soup.positions.len()>=100_000 {return Err(MeshKernelError::InvalidInput("indexed triangulation exceeds vertex capacity".into()));}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.unique.insert(key,id);id};self.remap.push(id);self.cursor+=3;}
            }
            1=>{
                if self.cursor==self.indices.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;}
                else {let index=self.indices[self.cursor] as usize;let id=*self.remap.get(index).ok_or(MeshKernelError::InvalidHandle)?;self.corners.push(id);self.cursor+=1;if self.corners.len()==3 {self.soup.faces.push(std::mem::take(&mut self.corners));}}
            }
            2=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {if self.normals.is_empty() && self.polygon.is_none() {return Ok(Some(mesh));}self.mesh=Some(mesh);self.cursor=0;self.phase=if self.polygon.is_some() {11}else {3};}}
            3=>{
                if self.cursor==self.indices.len() {let mut mesh=self.mesh.take().unwrap();mesh.attributes.insert("normal".into(),MeshAttribute { indices:None,domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Normal,interpolation:MeshAttributeInterpolation::Linear,values:std::mem::take(&mut self.corner_normals)});return Ok(Some(mesh));}
                let index=self.indices[self.cursor] as usize*3;let normal=[0,1,2].map(|axis|self.normals[index+axis]);if normal==[0.0;3] {return Err(MeshKernelError::InvalidInput("imported mesh normal cannot be zero".into()));}self.corner_normals.push(protocol::value::DslValue::Array(normal.into_iter().map(|x|protocol::value::DslValue::float(x as f64)).collect()));self.cursor+=1;
            }
            4=>{
                let (kind,[a,b,c],parameter)=self.primitive.unwrap();let count=match kind {0=>8,1=>4,2=>parameter as usize*2,3=>parameter as usize+1,_=>12};
                if self.cursor==count {self.cursor=0;self.face=0;self.phase=5;}else {
                    let point=match kind {
                        0=>[[-a/2.0,-b/2.0,-c/2.0],[a/2.0,-b/2.0,-c/2.0],[a/2.0,b/2.0,-c/2.0],[-a/2.0,b/2.0,-c/2.0],[-a/2.0,-b/2.0,c/2.0],[a/2.0,-b/2.0,c/2.0],[a/2.0,b/2.0,c/2.0],[-a/2.0,b/2.0,c/2.0]][self.cursor],
                        1=>[[-a/2.0,0.0,-b/2.0],[a/2.0,0.0,-b/2.0],[a/2.0,0.0,b/2.0],[-a/2.0,0.0,b/2.0]][self.cursor],
                        2|3=>{if kind==3 && self.cursor==parameter as usize {[0.0,b/2.0,0.0]}else {let angle=(self.cursor%parameter as usize)as f32/parameter as f32*std::f32::consts::TAU;[a*angle.cos(),if self.cursor<parameter as usize {-b/2.0}else {b/2.0},a*angle.sin()]}},
                        _=>{let t=(1.0+5.0f32.sqrt())/2.0;Vec3([[-1.0,t,0.0],[1.0,t,0.0],[-1.0,-t,0.0],[1.0,-t,0.0],[0.0,-1.0,t],[0.0,1.0,t],[0.0,-1.0,-t],[0.0,1.0,-t],[t,0.0,-1.0],[t,0.0,1.0],[-t,0.0,-1.0],[-t,0.0,1.0]][self.cursor]).normalize().0},
                    };self.soup.positions.push(point);self.cursor+=1;
                }
            },
            5=>{
                let (kind,_,parameter)=self.primitive.unwrap();let count=match kind {0=>6,1=>1,2=>parameter as usize+2,3=>parameter as usize+1,_=>20};
                if self.face==count {self.cursor=0;self.face=0;self.phase=if kind==4 {6}else {2};if kind!=4 {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));}}else if matches!(kind,2|3) && self.face<if kind==2 {2}else {1} {
                    self.corners.push(if self.face==0 {self.cursor as u32}else {parameter*2-1-self.cursor as u32});self.cursor+=1;if self.cursor==parameter as usize {self.soup.faces.push(std::mem::take(&mut self.corners));self.face+=1;self.cursor=0;}
                }else {let face=match kind {
                    0=>[[3,2,1,0],[5,6,7,4],[1,5,4,0],[2,6,5,1],[3,7,6,2],[0,4,7,3]][self.face].to_vec(),1=>vec![0,1,2,3],
                    2=>{let id=self.face as u32-2;let next=(id+1)%parameter;vec![id,id+parameter,next+parameter,next]},3=>{let id=self.face as u32-1;vec![id,parameter,(id+1)%parameter]},
                    _=>[[0,11,5],[0,5,1],[0,1,7],[0,7,10],[0,10,11],[1,5,9],[5,11,4],[11,10,2],[10,7,6],[7,1,8],[3,9,4],[3,4,2],[3,2,6],[3,6,8],[3,8,9],[4,9,5],[2,4,11],[6,2,10],[8,6,7],[9,8,1]][self.face].to_vec(),
                };self.soup.faces.push(face);self.face+=1;}
            },
            6=>{if self.generation==self.primitive.unwrap().2 {self.cursor=0;self.phase=9;}else {self.primitive_faces=std::mem::take(&mut self.soup.faces);self.face=0;self.cursor=0;self.part=0;self.phase=7;}},
            7=>{if self.face==self.primitive_faces.len() {self.phase=8;}else {let face=&self.primitive_faces[self.face];if self.cursor<3 {let a=face[self.cursor];let b=face[(self.cursor+1)%3];let key=(a.min(b),a.max(b));let id=if let Some(id)=self.midpoints.get(&key) {*id}else {let point=Vec3(self.soup.positions[a as usize]).lerp(Vec3(self.soup.positions[b as usize]),0.5).normalize().0;let id=self.soup.positions.len()as u32;self.soup.positions.push(point);self.midpoints.insert(key,id);id};self.mids.push(id);self.cursor+=1;}else {self.soup.faces.push(if self.part<3 {vec![face[self.part],self.mids[self.part],self.mids[(self.part+2)%3]]}else {std::mem::take(&mut self.mids)});self.part+=1;if self.part==4 {self.face+=1;self.part=0;self.cursor=0;}}}},
            8=>{if self.midpoints.pop_first().is_none() && self.primitive_faces.pop().is_none() {self.generation+=1;self.phase=6;}},
            9=>{if self.cursor==self.soup.positions.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;}else {let radius=self.primitive.unwrap().1[0];self.soup.positions[self.cursor]=self.soup.positions[self.cursor].map(|value|value*radius);self.cursor+=1;}},
            10=>{let source=self.polygon.as_mut().unwrap();self.soup.positions=std::mem::take(&mut source.vertices);self.soup.faces=std::mem::take(&mut source.faces);self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=2;},
            11=>{let mut mesh=self.mesh.take().unwrap();let source=self.polygon.as_mut().unwrap();mesh.attributes=std::mem::take(&mut source.attributes);mesh.materials=std::mem::take(&mut source.materials);mesh.textures=std::mem::take(&mut source.textures);return Ok(Some(mesh));},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Inflate {
    snapshot:Snapshot,
    input:Vec<FaceId>,
    selected:HashSet<u32>,
    amount:f32,
    inset:bool,
    normals:BTreeMap<u32,Vec3>,
    boundary:HashMap<(u32,u32),(u32,u32,usize,u32)>,
    edges:BTreeSet<(u32,u32)>,
    displaced:HashMap<u32,u32>,
    last_vertex:Option<u32>,
    last_edge:Option<(u32,u32)>,
    soup:Soup,
    borders:Vec<Vec<u32>>,
    inner:Vec<u32>,
    corners:Vec<u32>,
    cursor:usize,
    face:usize,
    corner:usize,
    emitted_corners:usize,
    build:Option<Build>,
    vertex_sources:Vec<SourceVertex>,
    source_faces:Vec<u32>,
    border_faces:Vec<u32>,
    attributes:Option<SourceAttributeRemap>,
    phase:u8,
}
impl Inflate {
    fn phase(&self)->&'static str { if self.inset {["inset-selection","snapshot","inset-vertices","inset-borders","inset-corners","inset-transfer","inset-reconstruct","inset-source-vertices","inset-source-remap","inset-attributes"][self.phase as usize]}else {["extrude-selection","snapshot","extrude-boundaries","extrude-vertices","extrude-corners","extrude-borders","extrude-reconstruct","extrude-source-vertices","extrude-source-remap","extrude-attributes"][self.phase as usize]} }
    fn admit_position(&self)->MeshResult<()> {if self.soup.positions.len()>=100_000 {Err(MeshKernelError::InvalidInput("face expansion exceeds vertex capacity".into()))}else {Ok(())}}
    fn admit_face(&mut self,corners:usize)->MeshResult<()> {if self.soup.faces.len()>=100_000 || self.emitted_corners.saturating_add(corners)>600_000 {return Err(MeshKernelError::InvalidInput("face expansion exceeds mesh capacity".into()));}self.emitted_corners+=corners;Ok(())}
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.cursor==self.input.len() {self.cursor=0;self.phase=1;}else {let face=self.input[self.cursor];self.snapshot.source.face_boundary(face)?;if !self.selected.insert(face.0) {return Err(MeshKernelError::InvalidInput("duplicate selected face".into()));}self.cursor+=1;}}
            1=>{if self.snapshot.advance() {self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=7;}}
            2=>{
                if self.cursor==self.input.len() {
                    if self.inset {self.phase=4;self.face=0;self.corner=0;}else {if self.snapshot.soup.faces.len().saturating_add(self.edges.len())>100_000 || self.snapshot.source.halfedge_count().saturating_add(self.edges.len().saturating_mul(4))>600_000 {return Err(MeshKernelError::InvalidInput("extrusion exceeds mesh capacity".into()));}self.phase=3;}
                }else {
                    let id=self.input[self.cursor].0 as usize;let face=&self.snapshot.soup.faces[id];let n=face.len();let normal=self.snapshot.soup.normals[id];
                    if self.inset {
                        self.admit_position()?;let face=&self.snapshot.soup.faces[id];let point=|i:usize|Vec3(self.soup.positions[face[i] as usize]);let previous=point((self.corner+n-1)%n);let current=point(self.corner);let next=point((self.corner+1)%n);let a=normal.cross(current.sub(previous).normalize());let b=normal.cross(next.sub(current).normalize());let denominator=1.0+a.dot(b);if denominator<=1e-6 {return Err(MeshKernelError::DegenerateOperation);}let offset=a.add(b).scale(self.amount/denominator);if offset.length()>=current.sub(previous).length().min(next.sub(current).length())*0.5 {return Err(MeshKernelError::InvalidInput("inset exceeds local edge clearance".into()));}let position=current.add(offset).0;if position.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.inner.push(self.soup.positions.len() as u32);self.soup.positions.push(position);self.vertex_sources.push(SourceVertex::Original(face[self.corner]));
                    }else {let (a,b)=(face[self.corner],face[(self.corner+1)%n]);self.normals.entry(a).and_modify(|sum|*sum=sum.add(normal)).or_insert(normal);let edge=self.boundary.entry((a.min(b),a.max(b))).or_insert((a,b,0,id as u32));if edge.2==0 {self.edges.insert((a,b));}else {self.edges.remove(&(edge.0,edge.1));}edge.2+=1;}
                    self.corner+=1;if self.corner==n {self.corner=0;if self.inset {self.phase=3;}else {self.cursor+=1;}}
                }
            }
            3=>{
                if self.inset {
                    let id=self.input[self.cursor].0 as usize;let face=&self.snapshot.soup.faces[id];let next=(self.corner+1)%face.len();if self.snapshot.soup.faces.len().saturating_add(self.borders.len()).saturating_add(1)>100_000 || self.snapshot.source.halfedge_count().saturating_add((self.borders.len()+1).saturating_mul(4))>600_000 {return Err(MeshKernelError::InvalidInput("inset exceeds mesh capacity".into()));}self.borders.push(vec![face[self.corner],face[next],self.inner[next],self.inner[self.corner]]);self.border_faces.push(id as u32);self.corner+=1;if self.corner==face.len() {self.snapshot.soup.faces[id]=std::mem::take(&mut self.inner);self.cursor+=1;self.corner=0;self.phase=2;}
                }else {
                    let next=if let Some(last)=self.last_vertex {self.normals.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().map(|(&id,&normal)|(id,normal))}else {self.normals.first_key_value().map(|(&id,&normal)|(id,normal))};
                    if let Some((id,normal))=next {self.admit_position()?;let normal=normal.normalize();if normal.length()<1e-6 {return Err(MeshKernelError::DegenerateOperation);}let point=Vec3(self.soup.positions[id as usize]).add(normal.scale(self.amount)).0;if point.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.displaced.insert(id,self.soup.positions.len() as u32);self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Original(id));self.last_vertex=Some(id);}else {self.phase=4;self.face=0;self.corner=0;}
                }
            }
            4=>{
                if self.face==self.snapshot.soup.faces.len() {self.phase=5;self.cursor=0;}
                else {self.admit_face(1)?;let face=&self.snapshot.soup.faces[self.face];let id=face[self.corner];self.corners.push(if !self.inset && self.selected.contains(&(self.face as u32)) {self.displaced[&id]}else {id});self.corner+=1;if self.corner==face.len() {self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.corners));self.corner=0;self.face+=1;}}
            }
            5=>{
                if self.inset {if self.cursor==self.borders.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=6;}else {self.admit_face(4)?;self.source_faces.push(self.border_faces[self.cursor]);self.soup.faces.push(std::mem::take(&mut self.borders[self.cursor]));self.cursor+=1;}}
                else {let next=if let Some(last)=self.last_edge {self.edges.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().copied()}else {self.edges.first().copied()};if let Some((a,b))=next {self.admit_face(4)?;self.source_faces.push(self.boundary[&(a.min(b),a.max(b))].3);self.soup.faces.push(vec![a,b,self.displaced[&b],self.displaced[&a]]);self.last_edge=Some((a,b));}else {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=6;}}
            }
            6=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().generated_vertices=Some(Vec::new());self.cursor=0;self.phase=8;}},
            7=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            8=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize]);self.cursor+=1;}},
            9=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Orient {
    input:Option<Vec<FaceId>>,
    flip_count:usize,
    snapshot:Snapshot,
    attributes:Option<SourceAttributeRemap>,
    edges:HashMap<(u32,u32),Vec<(usize,bool)>>,
    keys:Vec<(u32,u32)>,
    adjacency:Vec<Vec<(usize,bool)>>,
    oriented:Vec<bool>,
    flipped:Vec<bool>,
    stack:Vec<usize>,
    active:Option<usize>,
    face:usize,
    corner:usize,
    cursor:usize,
    neighbor:usize,
    reversing:usize,
    build:Option<Build>,
    phase:u8,
}
impl Orient {
    fn phase(&self)->&'static str { ["snapshot","orient-initialize","orient-incidence","orient-adjacency","orient-components","orient-corners","orient-reconstruct","orient-attributes","orient-selection"][self.phase as usize] }
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.advance() {self.phase=1;}}
            1=>{if self.cursor==self.snapshot.soup.faces.len() {self.phase=if self.input.is_some() {8}else {2};self.cursor=0;}else {self.adjacency.push(Vec::new());self.oriented.push(false);self.flipped.push(false);self.cursor+=1;}}
            2=>{
                if self.face==self.snapshot.soup.faces.len() {self.phase=3;self.face=0;}
                else {let face=&self.snapshot.soup.faces[self.face];let (a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);let key=(a.min(b),a.max(b));let edge=self.edges.entry(key).or_default();if edge.is_empty() {self.keys.push(key);}edge.push((self.face,a<b));self.corner+=1;if self.corner==face.len() {self.face+=1;self.corner=0;}}
            }
            3=>{
                if self.cursor==self.keys.len() {self.phase=4;self.cursor=0;}
                else {let owners=&self.edges[&self.keys[self.cursor]];if owners.len()==2 {let (a,af)=owners[0];let (b,bf)=owners[1];self.adjacency[a].push((b,af==bf));self.adjacency[b].push((a,af==bf));}self.cursor+=1;}
            }
            4=>{
                if let Some(face)=self.active {
                    if self.neighbor==self.adjacency[face].len() {self.active=None;}
                    else {let (neighbor,relative)=self.adjacency[face][self.neighbor];if !self.oriented[neighbor] {self.oriented[neighbor]=true;self.flipped[neighbor]=self.flipped[face]^relative;self.flip_count+=usize::from(self.flipped[neighbor]);self.stack.push(neighbor);}self.neighbor+=1;}
                }else if let Some(face)=self.stack.pop() {self.active=Some(face);self.neighbor=0;}
                else if self.cursor==self.oriented.len() {self.phase=5;self.face=0;}
                else {if !self.oriented[self.cursor] {self.oriented[self.cursor]=true;self.stack.push(self.cursor);}self.cursor+=1;}
            }
            5=>{
                if self.face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=6;}
                else {let face=&mut self.snapshot.soup.faces[self.face];if self.flipped[self.face] && self.reversing<face.len()/2 {let opposite=face.len()-1-self.reversing;face.swap(self.reversing,opposite);self.reversing+=1;}else {self.face+=1;self.reversing=0;}}
            }
            6=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,None,None,None));self.phase=7;}},
            7=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            8=>{let input=self.input.as_ref().unwrap();if self.cursor==input.len() {self.face=0;self.cursor=0;self.phase=5;}else {let id=input[self.cursor];self.snapshot.source.face_boundary(id)?;self.flipped[id.0 as usize]=!self.flipped[id.0 as usize];self.cursor+=1;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct FillHoles {
    snapshot:Snapshot,
    visited:HashSet<u32>,
    vertices:Vec<u32>,
    boundary_edges:Vec<u32>,
    cap:Vec<u32>,
    cap_faces:Vec<u32>,
    corner_sources:Vec<Option<u32>>,
    edge_sources:Vec<Option<u32>>,
    face_sources:Vec<Vec<u32>>,
    attributes:Option<SourceAttributeRemap>,
    cursor:usize,
    start:u32,
    probe:u32,
    rotations:usize,
    corner:usize,
    corners:usize,
    filled:usize,
    build:Option<Build>,
    phase:u8,
}
impl FillHoles {
    fn phase(&self)->&'static str { ["snapshot","fill-holes-boundaries","fill-holes-rotate","fill-holes-corners","fill-holes-reconstruct","fill-holes-source-corners","fill-holes-source-faces","fill-holes-attributes"][self.phase as usize] }
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.advance() {self.cursor=0;self.phase=5;}}
            1=>{
                if self.cursor==self.snapshot.source.halfedge_count() {if self.filled==0 {return Ok(Some(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty())));}self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=4;}
                else {let id=self.cursor as u32;let edge=&self.snapshot.source.halfedges[self.cursor];self.cursor+=1;if edge.twin.is_none() && self.visited.insert(id) {self.start=id;self.vertices.push(edge.vertex);self.boundary_edges.push(id);self.probe=edge.next;self.rotations=0;self.phase=2;}}
            }
            2=>{
                let edge=&self.snapshot.source.halfedges[self.probe as usize];
                if let Some(twin)=edge.twin {self.probe=self.snapshot.source.halfedges[twin as usize].next;self.rotations+=1;if self.rotations>self.snapshot.source.halfedge_count()+4 {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}}
                else if self.probe==self.start {if self.vertices.len()>=3 {if self.snapshot.soup.faces.len()>=100_000 || self.corners.saturating_add(self.vertices.len())>600_000 {return Err(MeshKernelError::InvalidInput("hole filling exceeds mesh capacity".into()));}self.corners+=self.vertices.len();self.corner=0;self.phase=3;}else {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}}
                else if !self.visited.insert(self.probe) {self.vertices.clear();self.boundary_edges.clear();self.phase=1;}
                else {self.vertices.push(edge.vertex);self.boundary_edges.push(self.probe);self.probe=edge.next;self.rotations=0;}
            }
            3=>{let index=self.vertices.len()-1-self.corner;let corner=self.boundary_edges[index];let edge=self.boundary_edges[(index+self.vertices.len()-1)%self.vertices.len()];self.cap.push(self.vertices[index]);self.corner_sources.push(Some(corner));self.edge_sources.push(Some(edge));self.cap_faces.push(self.snapshot.source.halfedges[edge as usize].face.unwrap());self.corner+=1;if self.corner==self.vertices.len() {self.snapshot.soup.faces.push(std::mem::take(&mut self.cap));self.face_sources.push(std::mem::take(&mut self.cap_faces));self.filled+=1;self.vertices.clear();self.boundary_edges.clear();self.phase=1;}},
            4=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let mut attributes=SourceAttributeRemap::new(mesh,None,None,None,None);attributes.corner_sources=Some(std::mem::take(&mut self.corner_sources));attributes.edge_sources=Some(std::mem::take(&mut self.edge_sources));attributes.generated_faces=Some(std::mem::take(&mut self.face_sources));self.attributes=Some(attributes);self.phase=7;}},
            5=>{if self.cursor==self.snapshot.source.halfedge_count() {self.cursor=0;self.phase=6;}else {self.corner_sources.push(None);self.edge_sources.push(None);self.cursor+=1;}},
            6=>{if self.cursor==self.snapshot.source.face_count() {self.cursor=0;self.phase=1;}else {self.face_sources.push(vec![self.cursor as u32]);self.cursor+=1;}},
            7=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct LoopCut {
    snapshot:Snapshot,
    edges:Vec<EdgeId>,
    cuts:usize,
    incidence:HashMap<(u32,u32),Vec<(usize,usize)>>,
    marked:BTreeSet<(u32,u32)>,
    pending:Vec<(u32,u32)>,
    cut_vertices:HashMap<(u32,u32),Vec<u32>>,
    last_edge:Option<(u32,u32)>,
    cut_edge:Option<(u32,u32)>,
    cut_ids:Vec<u32>,
    soup:Soup,
    boundary:Vec<u32>,
    grid:Vec<u32>,
    face:usize,
    corner:usize,
    cursor:usize,
    step:usize,
    nx:usize,
    ny:usize,
    vertices:u64,
    faces:u64,
    corners:u64,
    building_grid:bool,
    build:Option<Build>,
    vertex_sources:Vec<SourceVertex>,
    source_faces:Vec<u32>,
    attributes:Option<SourceAttributeRemap>,
    phase:u8,
}
impl LoopCut {
    fn key(a:u32,b:u32)->(u32,u32) { (a.min(b),a.max(b)) }
    fn phase(&self)->&'static str { ["snapshot","loop-cut-incidence","loop-cut-selection","loop-cut-propagate","loop-cut-admission","loop-cut-vertices","loop-cut-grid","loop-cut-reconstruct","loop-cut-source-vertices","loop-cut-source-remap","loop-cut-attributes"][self.phase as usize] }
    fn dimensions(&self,face:&[u32])->(usize,usize) { (if self.marked.contains(&Self::key(face[0],face[1])) { self.cuts+1 } else { 1 },if self.marked.contains(&Self::key(face[1],face[2])) { self.cuts+1 } else { 1 }) }
    fn on_edge(&self,a:u32,b:u32,step:usize,segments:usize)->u32 { if step==0 { a } else if step==segments { b } else { self.cut_vertices[&Self::key(a,b)][if a<b { step-1 } else { segments-step-1 }] } }
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.advance() {self.cursor=0;self.phase=8;}}
            1=>{
                if self.face==self.snapshot.soup.faces.len() { self.phase=2;self.cursor=0;self.face=0; }
                else { let face=&self.snapshot.soup.faces[self.face];self.incidence.entry(Self::key(face[self.corner],face[(self.corner+1)%face.len()])).or_default().push((self.face,self.corner));self.corner+=1;if self.corner==face.len() {self.corner=0;self.face+=1;} }
            }
            2=>{
                if self.cursor==self.edges.len() { self.phase=3;self.cursor=0; }
                else { let (a,b)=self.snapshot.source.edge_endpoints(self.edges[self.cursor])?;let edge=Self::key(a.0,b.0);let owners=&self.incidence[&edge];if owners.len()>2 {return Err(MeshKernelError::NonManifold);}if !owners.iter().any(|&(face,_)|self.snapshot.soup.faces[face].len()==4) {return Err(MeshKernelError::InvalidInput("selected edges must touch a quad".into()));}if self.marked.insert(edge) {self.pending.push(edge);}self.cursor+=1; }
            }
            3=>{
                if self.cursor==self.pending.len() { self.vertices=self.snapshot.soup.positions.len() as u64+self.marked.len() as u64*self.cuts as u64;self.phase=4;self.face=0;self.corner=0; }
                else {let edge=self.pending[self.cursor];let owners=&self.incidence[&edge];if owners.len()>2 {return Err(MeshKernelError::NonManifold);}for &(fi,i) in owners {let face=&self.snapshot.soup.faces[fi];if face.len()==4 {let opposite=Self::key(face[(i+2)%4],face[(i+3)%4]);if self.marked.insert(opposite) {self.pending.push(opposite);}}}self.cursor+=1;}
            }
            4=>{
                if self.face==self.snapshot.soup.faces.len() { self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.face=0;self.corner=0;self.phase=5; }
                else {let face=&self.snapshot.soup.faces[self.face];if face.len()==4 {let (nx,ny)=self.dimensions(face);self.vertices+=((nx-1)*(ny-1)) as u64;self.faces+=(nx*ny) as u64;self.corners+=(nx*ny*4) as u64;self.face+=1;}else {if self.corner==0 {self.faces+=1;self.corners+=face.len() as u64;}if self.marked.contains(&Self::key(face[self.corner],face[(self.corner+1)%face.len()])) {self.corners+=self.cuts as u64;}self.corner+=1;if self.corner==face.len() {self.face+=1;self.corner=0;}}}
                if self.vertices>100_000 || self.faces>100_000 || self.corners>600_000 {return Err(MeshKernelError::InvalidInput("loop cut exceeds mesh capacity".into()));}
            }
            5=>{
                if self.cut_edge.is_none() {
                    let edge=if let Some(last)=self.last_edge {self.marked.range((std::ops::Bound::Excluded(last),std::ops::Bound::Unbounded)).next().copied()}else {self.marked.first().copied()};
                    if let Some(edge)=edge {self.cut_edge=Some(edge);self.last_edge=Some(edge);self.step=1;}else {self.phase=6;self.cursor=0;self.step=0;}
                }else {
                    let (a,b)=self.cut_edge.unwrap();let t=self.step as f64/(self.cuts+1) as f64;let point=[0,1,2].map(|axis|(self.soup.positions[a as usize][axis] as f64*(1.0-t)+self.soup.positions[b as usize][axis] as f64*t) as f32);
                    let previous=self.cut_ids.last().map(|id|self.soup.positions[*id as usize]).unwrap_or(self.soup.positions[a as usize]);if point.iter().any(|x|!x.is_finite()) || point==previous || point==self.soup.positions[b as usize] {return Err(MeshKernelError::DegenerateOperation);}
                    self.cut_ids.push(self.soup.positions.len() as u32);self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Edge {a,b,t});self.step+=1;if self.step>self.cuts {self.cut_vertices.insert((a,b),std::mem::take(&mut self.cut_ids));self.cut_edge=None;}
                }
            }
            6=>{
                if self.face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.soup)));self.phase=7;}
                else {
                    let face=&self.snapshot.soup.faces[self.face];
                    if face.len()==4 {
                        if self.cursor==0 && self.building_grid {(self.nx,self.ny)=self.dimensions(face);}
                        if self.building_grid {
                            let (x,y)=(self.cursor%(self.nx+1),self.cursor/(self.nx+1));let id=if y==0 {self.on_edge(face[0],face[1],x,self.nx)}else if y==self.ny {self.on_edge(face[3],face[2],x,self.nx)}else if x==0 {self.on_edge(face[0],face[3],y,self.ny)}else if x==self.nx {self.on_edge(face[1],face[2],y,self.ny)}else {
                                let (u,v)=(x as f64/self.nx as f64,y as f64/self.ny as f64);let weights=[(1.0-u)*(1.0-v),u*(1.0-v),u*v,(1.0-u)*v];let point=[0,1,2].map(|axis|face.iter().zip(weights).map(|(&id,w)|self.soup.positions[id as usize][axis] as f64*w).sum::<f64>() as f32);if point.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}let id=self.soup.positions.len() as u32;self.soup.positions.push(point);self.vertex_sources.push(SourceVertex::Weighted {vertices:[face[0],face[1],face[2],face[3]],weights});id
                            };self.grid.push(id);self.cursor+=1;if self.cursor==(self.nx+1)*(self.ny+1) {self.cursor=0;self.building_grid=false;}
                        }else {let (x,y)=(self.cursor%self.nx,self.cursor/self.nx);let i=y*(self.nx+1)+x;self.source_faces.push(self.face as u32);self.soup.faces.push(vec![self.grid[i],self.grid[i+1],self.grid[i+self.nx+2],self.grid[i+self.nx+1]]);self.cursor+=1;if self.cursor==self.nx*self.ny {self.face+=1;self.cursor=0;self.grid.clear();self.building_grid=true;}}
                    }else {
                        let (a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);if self.step==0 {self.boundary.push(a);if self.marked.contains(&Self::key(a,b)) {self.step=1;}else {self.corner+=1;}}else {self.boundary.push(self.on_edge(a,b,self.step,self.cuts+1));self.step+=1;if self.step>self.cuts {self.step=0;self.corner+=1;}}
                        if self.corner==face.len() {self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.boundary));self.corner=0;self.face+=1;}
                    }
                }
            }
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().generated_vertices=Some(Vec::new());self.cursor=0;self.phase=9;}},
            8=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            9=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.phase=10;}else {self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            10=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Mirror {
    snapshot: Snapshot,
    source_vertices:Vec<u32>,
    source_faces:Vec<u32>,
    mirrored_faces:Vec<bool>,
    attribute_vertices:Vec<u32>,
    attributes:Option<SourceAttributeRemap>,
    axis: usize,
    tolerance: f32,
    positive: bool,
    negative: bool,
    reflected: Vec<u32>,
    soup: Soup,
    original: Vec<u32>,
    mirrored: Vec<u32>,
    emitted_corners: usize,
    plane: bool,
    vertex: usize,
    face: usize,
    corner: usize,
    build: Option<Build>,
    phase: u8,
}

struct Subdivide {
    snapshot: Snapshot,
    input: Vec<FaceId>,
    triangulate_only:bool,
    delete_selected:bool,
    selected: HashSet<u32>,
    concave: HashSet<u32>,
    convex: bool,
    tessellation: Option<MeshTessellationJob>,
    transfer: Option<MeshTransfer>,
    soup: Soup,
    corners: Vec<u32>,
    center: Vec3,
    center_id: u32,
    emitted_corners: usize,
    emitting: bool,
    cursor: usize,
    face: usize,
    corner: usize,
    triangle: usize,
    build: Option<Build>,
    vertex_sources:Vec<SourceVertex>,
    source_faces:Vec<u32>,
    attributes:Option<SourceAttributeRemap>,
    phase: u8,
}

impl Subdivide {
    fn phase(&self) -> &'static str { ["subdivide-selection","snapshot","subdivide-convexity","subdivide-triangulate","subdivide-corners","subdivide-reconstruct","subdivide-source-vertices","subdivide-source-remap","subdivide-attributes"][self.phase as usize] }
    fn admit(&mut self,vertices:usize,faces:usize,corners:usize)->MeshResult<()> {
        if self.soup.positions.len().saturating_add(vertices)>100_000 || self.soup.faces.len().saturating_add(faces)>100_000 || self.emitted_corners.saturating_add(corners)>600_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); }
        self.emitted_corners+=corners;Ok(())
    }
    fn next_face(&mut self) { self.face += 1; self.corner = 0; self.emitting = false; self.center = Vec3::ZERO; self.convex = true; }
    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.cursor==self.input.len() {if self.delete_selected && self.selected.len()==self.snapshot.source.face_count() {return Err(MeshKernelError::InvalidInput("deletion would leave an empty mesh".into()));}self.phase = 1;}
                else { let face = self.input[self.cursor]; self.snapshot.source.face_boundary(face)?; if !self.selected.insert(face.0) { return Err(MeshKernelError::InvalidInput("duplicate face selection".into())); } self.cursor += 1; }
            }
            1 => {if self.snapshot.advance() {self.cursor=0;self.phase=6;}}
            2 => {
                if self.face==self.snapshot.soup.faces.len() {
                    if !self.concave.is_empty() { self.tessellation = Some(MeshTessellationJob::selected(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty()),std::mem::take(&mut self.concave))); }
                    self.phase = 3; self.face = 0; self.corner = 0;
                }else if self.delete_selected {self.next_face();}else if self.triangulate_only {if self.snapshot.soup.faces[self.face].len()>3 {self.selected.insert(self.face as u32);self.concave.insert(self.face as u32);}self.next_face();}
                else if !self.selected.contains(&(self.face as u32)) { self.next_face(); }
                else {
                    let face = &self.snapshot.soup.faces[self.face]; let n = face.len(); let point = |index:usize|Vec3(self.snapshot.soup.positions[face[index%n] as usize]);
                    self.convex &= point(self.corner+1).sub(point(self.corner)).cross(point(self.corner+2).sub(point(self.corner+1))).dot(self.snapshot.soup.normals[self.face])>0.0;
                    self.corner += 1;
                    if self.corner==n { if !self.convex { self.concave.insert(self.face as u32); } self.next_face(); }
                }
            }
            3 => {
                if let Some(tessellation) = &mut self.tessellation {
                    if let MeshTessellationStep::Done(transfer) = tessellation.step(1)? { self.transfer = Some(transfer);self.snapshot.source=self.tessellation.take().unwrap().into_source(); self.soup.positions = std::mem::take(&mut self.snapshot.soup.positions); self.phase = 4; }
                } else { self.soup.positions = std::mem::take(&mut self.snapshot.soup.positions); self.phase = 4; }
            }
            4 => {
                if self.face==self.snapshot.soup.faces.len() { self.build = Some(Build::unreferenced(std::mem::take(&mut self.soup))); self.phase = 5; }
                else if self.delete_selected && self.selected.contains(&(self.face as u32)) {self.next_face();}
                else if self.delete_selected || !self.selected.contains(&(self.face as u32)) {
                    self.admit(0,usize::from(self.corner+1==self.snapshot.soup.faces[self.face].len()),1)?; let face = &self.snapshot.soup.faces[self.face]; self.corners.push(face[self.corner]); self.corner += 1;
                    if self.corner==face.len() { self.source_faces.push(self.face as u32);self.soup.faces.push(std::mem::take(&mut self.corners)); self.next_face(); }
                } else if self.transfer.as_ref().is_some_and(|transfer|transfer.face_ids.get(self.triangle)==Some(&(self.face as u32))) {
                    self.admit(if self.triangulate_only {0}else {1},if self.triangulate_only {1}else {3},if self.triangulate_only {3}else {9})?;let transfer=self.transfer.as_ref().unwrap();let ids=[0,1,2].map(|corner|transfer.vertex_ids[transfer.indices[self.triangle*3+corner] as usize]);
                    if self.triangulate_only {self.source_faces.push(self.face as u32);self.soup.faces.push(ids.to_vec());}else {
                        let center=ids.into_iter().fold(Vec3::ZERO,|sum,id|sum.add(Vec3(self.soup.positions[id as usize]))).scale(1.0/3.0);let center_id=self.soup.positions.len() as u32;self.soup.positions.push(center.0);self.vertex_sources.push(SourceVertex::Weighted {vertices:[ids[0],ids[1],ids[2],ids[2]],weights:[1.0/3.0,1.0/3.0,1.0/3.0,0.0]});for corner in 0..3 {self.source_faces.push(self.face as u32);self.soup.faces.push(vec![ids[corner],ids[(corner+1)%3],center_id]);}
                    }
                    self.triangle += 1;
                    if self.transfer.as_ref().unwrap().face_ids.get(self.triangle)!=Some(&(self.face as u32)) { self.next_face(); }
                } else {
                    let face = &self.snapshot.soup.faces[self.face];
                    if !self.emitting {
                        self.center = self.center.add(Vec3(self.soup.positions[face[self.corner] as usize])); self.corner += 1;
                        if self.corner==face.len() { if self.soup.positions.len()>=100_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); } self.center_id = self.soup.positions.len() as u32; self.soup.positions.push(self.center.scale(1.0/face.len() as f32).0);self.vertex_sources.push(SourceVertex::Face(self.face as u32)); self.corner = 0; self.emitting = true; }
                    } else {
                        if self.soup.faces.len()>=100_000 || self.emitted_corners.saturating_add(3)>600_000 { return Err(MeshKernelError::InvalidInput("subdivision exceeds mesh capacity".into())); } self.emitted_corners+=3;self.source_faces.push(self.face as u32); self.soup.faces.push(vec![face[self.corner],face[(self.corner+1)%face.len()],self.center_id]); self.corner += 1;
                        if self.corner==face.len() { self.next_face(); }
                    }
                }
            }
            5=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().generated_vertices=Some(Vec::new());self.cursor=0;self.phase=7;}},
            6=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            7=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.phase=8;}else {self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            8=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _ => unreachable!(),
        }
        Ok(None)
    }
}

struct Coplanar {
    explicit:Option<Vec<EdgeId>>,
    vertex_input:Option<Vec<VertexId>>,
    selected_vertices:BTreeSet<u32>,
    star_vertex:u32,
    star_incident:Vec<usize>,
    star_edges:BTreeMap<(u32,u32),(usize,usize)>,
    star_next:BTreeMap<u32,(u32,usize,usize)>,
    star_start:u32,
    star_probe:u32,
    star_contributors:Vec<u32>,
    star_extent:f32,
    star_origin:Vec3,

    output_vertices:Vec<u32>,
    source_corners:BTreeMap<(u32,u32),u32>,
    face_sources:Vec<Vec<u32>>,
    corner_sources:Vec<Vec<u32>>,
    edge_sources:Vec<Vec<u32>>,
    spliced_corners:Vec<u32>,
    spliced_edges:Vec<u32>,
    output_corners:Vec<Option<u32>>,
    output_edges:Vec<Option<u32>>,
    output_faces:Vec<Vec<u32>>,
    attributes:Option<SourceAttributeRemap>,
    attribute_name:Option<String>,
    equality:Option<SourceValueEquality>,
    preserve_corners:bool,
    endpoint:usize,
    keep:usize,
    remove:usize,
    incidence:BTreeMap<(u32,u32),(usize,usize)>,
    pending:BTreeSet<(u32,u32)>,
    seen:BTreeSet<u32>,
    neighbors:BTreeMap<u32,Option<(u32,u32)>>,
    removable:BTreeSet<u32>,
    face:usize,
    cursor:usize,
    a:usize,
    b:usize,
    ai:usize,
    bi:usize,
    side:usize,
    normal:Normal,
    na:Vec3,
    min:Vec3,
    max:Vec3,
    tolerance:f32,
    shared:usize,
    corners:Vec<u32>,
    soup:Soup,
    build:Option<Build>,
    merges:usize,
    phase:u8,
}
impl Default for Coplanar {
    fn default()->Self {Self {explicit:None,vertex_input:None,selected_vertices:BTreeSet::new(),star_vertex:0,star_incident:Vec::new(),star_edges:BTreeMap::new(),star_next:BTreeMap::new(),star_start:0,star_probe:0,star_contributors:Vec::new(),star_extent:0.0,star_origin:Vec3::ZERO,output_vertices:Vec::new(),source_corners:BTreeMap::new(),face_sources:Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),spliced_corners:Vec::new(),spliced_edges:Vec::new(),output_corners:Vec::new(),output_edges:Vec::new(),output_faces:Vec::new(),attributes:None,attribute_name:None,equality:None,preserve_corners:false,endpoint:0,keep:0,remove:0,incidence:BTreeMap::new(),pending:BTreeSet::new(),seen:BTreeSet::new(),neighbors:BTreeMap::new(),removable:BTreeSet::new(),face:0,cursor:0,a:0,b:0,ai:0,bi:0,side:0,normal:Normal::default(),na:Vec3::ZERO,min:Vec3::ZERO,max:Vec3::ZERO,tolerance:0.0,shared:0,corners:Vec::new(),soup:Soup::default(),build:None,merges:0,phase:0}}
}
impl Coplanar {
    fn phase(&self)->&'static str {match self.phase {0=>"snapshot",1=>"coplanar-incidence",2=>"coplanar-candidates",3|4=>"coplanar-normal",5=>"coplanar-distance",6=>"coplanar-shared-edges",7=>"coplanar-splice",8|9=>"coplanar-remove-edges",10=>"coplanar-insert-edges",11=>"coplanar-release-candidate",12=>"coplanar-cleanup-incidence",13=>"coplanar-cleanup-classify",14=>"coplanar-cleanup-corners",17|18=>"coplanar-source-corners",22=>"coplanar-channel-policy",23=>"dissolve-selection",24=>"dissolve-vertex-provenance",25=>"dissolve-face-metadata",26=>"dissolve-star-selection",27=>"dissolve-star-incidence",28=>"dissolve-star-normal",29=>"dissolve-star-plane",30=>"dissolve-star-boundary",31=>"dissolve-star-next",32=>"dissolve-star-loop",33=>"dissolve-star-release",34=>"dissolve-star-contributors",19=>"coplanar-face-contributors",20=>"coplanar-channel-boundary",21=>"coplanar-attributes",_=>"coplanar-reconstruct"}}
    fn reject(&mut self)->MeshResult<()> {if self.explicit.is_some() || self.vertex_input.is_some() {return Err(MeshKernelError::InvalidInput("selected dissolution cannot preserve the boundary contract".into()));}self.cursor=0;self.phase=11;Ok(())}
    fn advance(&mut self,snapshot:&mut Snapshot)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if snapshot.advance() {self.cursor=0;self.phase=22;}},
            1=>{
                if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=2;}
                else {let face=&snapshot.soup.faces[self.face];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.insert(key,(self.face,self.cursor)).is_some() {return Err(MeshKernelError::NonManifold);}if self.explicit.is_none() {self.pending.insert(key);}self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}
            },
            2=>{
                if let Some((u,v))=self.pending.pop_first() {
                    if let (Some(&(a,ai)),Some(&(b,bi)))=(self.incidence.get(&(u,v)),self.incidence.get(&(v,u))) {if a!=b {self.a=a;self.b=b;self.ai=ai;self.bi=bi;self.cursor=0;self.side=0;self.shared=0;self.normal=Normal::default();let point=Vec3(snapshot.soup.positions[snapshot.soup.faces[a][0] as usize]);self.min=point;self.max=point;self.phase=if self.explicit.is_some() {6}else {3};}}else if self.explicit.is_some() {return Err(MeshKernelError::DegenerateOperation);}
                }else {self.face=0;self.cursor=0;self.phase=12;}
            },
            3|4=>{
                let face=&snapshot.soup.faces[if self.phase==3 {self.a}else {self.b}];let p=Vec3(snapshot.soup.positions[face[self.cursor] as usize]);let next=Vec3(snapshot.soup.positions[face[(self.cursor+1)%face.len()] as usize]);if self.cursor==0 {self.normal.origin=p;}self.normal.add(p,next);for axis in 0..3 {self.min.0[axis]=self.min.0[axis].min(p.0[axis]);self.max.0[axis]=self.max.0[axis].max(p.0[axis]);}self.cursor+=1;
                if self.cursor==face.len() {let normal=self.normal.value();self.normal=Normal::default();self.cursor=0;if normal.length()==0.0 {self.reject()?;}else if self.phase==3 {self.na=normal;self.phase=4;}else if self.na.dot(normal)<1.0-1e-4 {self.reject()?;}else {self.tolerance=(self.max.sub(self.min).length()*1e-4).max(1e-6);self.phase=5;}}
            },
            5=>{
                let face=&snapshot.soup.faces[self.b];let origin=Vec3(snapshot.soup.positions[snapshot.soup.faces[self.a][0] as usize]);let point=Vec3(snapshot.soup.positions[face[self.cursor] as usize]);if point.sub(origin).dot(self.na).abs()>self.tolerance {self.reject()?;}else {self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.phase=6;}}
            },
            6=>{
                let face=&snapshot.soup.faces[self.a];let (u,v)=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.get(&(v,u)).is_some_and(|&(other,_)|other==self.b) {self.shared+=1;}self.cursor+=1;if self.shared>1 {self.reject()?;}else if self.cursor==face.len() {self.cursor=0;self.attribute_name=None;self.endpoint=0;self.phase=20;}
            },
            7=>{
                let a=&snapshot.soup.faces[self.a];let b=&snapshot.soup.faces[self.b];let total=a.len()+b.len()-2;
                if self.cursor==total {self.cursor=0;self.side=0;self.phase=8;}
                else {let (face,index)=if self.cursor==0 {(self.a,self.ai)}else if self.cursor<b.len()-1 {(self.b,(self.bi+self.cursor+1)%b.len())}else {(self.a,(self.ai+self.cursor-(b.len()-2))%a.len())};let vertex=snapshot.soup.faces[face][index];if !self.seen.insert(vertex) {self.reject()?;}else {self.corners.push(vertex);self.spliced_corners.push(self.corner_sources[face][index]);self.spliced_edges.push(if self.cursor==0 {self.edge_sources[self.b][(self.bi+1)%b.len()]}else {self.edge_sources[face][index]});self.cursor+=1;}}
            },
            8=>{
                let face=&snapshot.soup.faces[if self.side==0 {self.a}else {self.b}];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);self.incidence.remove(&key);self.pending.remove(&key);self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.side+=1;if self.side==2 {self.phase=9;}}
            },
            9=>{self.keep=self.a.min(self.b);self.remove=self.a.max(self.b);snapshot.soup.faces[self.keep]=std::mem::take(&mut self.corners);snapshot.soup.faces[self.remove].clear();self.corner_sources[self.keep]=std::mem::take(&mut self.spliced_corners);self.edge_sources[self.keep]=std::mem::take(&mut self.spliced_edges);self.corner_sources[self.remove].clear();self.edge_sources[self.remove].clear();self.a=self.keep;self.cursor=0;self.merges+=1;self.phase=19;},
            10=>{let face=&snapshot.soup.faces[self.a];let key=(face[self.cursor],face[(self.cursor+1)%face.len()]);if self.incidence.insert(key,(self.a,self.cursor)).is_some() {return Err(MeshKernelError::NonManifold);}if self.explicit.is_none() {self.pending.insert(key);}self.cursor+=1;if self.cursor==face.len() {self.cursor=0;self.phase=11;}},
            11=>{if self.seen.pop_first().is_none() {self.corners.clear();self.spliced_corners.clear();self.spliced_edges.clear();self.attribute_name=None;self.equality=None;self.phase=2;}},
            12=>{
                if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=13;}
                else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;}else {let id=face[self.cursor];let pair=(face[(self.cursor+face.len()-1)%face.len()],face[(self.cursor+1)%face.len()]);self.neighbors.entry(id).and_modify(|value|{if *value!=Some(pair) {*value=None;}}).or_insert(Some(pair));self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}}
            },
            13=>{
                if let Some((id,Some((prev,next))))=self.neighbors.pop_first() {if !self.preserve_corners && prev!=next && prev!=id && next!=id {let pos=|id:u32|Vec3(snapshot.soup.positions[id as usize]);let a=pos(id).sub(pos(prev));let b=pos(next).sub(pos(id));if a.length()>=1e-9 && b.length()>=1e-9 && a.normalize().dot(b.normalize())>1.0-1e-4 {self.removable.insert(id);}}}
                else if self.neighbors.is_empty() {self.soup.positions=std::mem::take(&mut snapshot.soup.positions);self.phase=14;}
            },
            14=>{
                if self.face==snapshot.soup.faces.len() {self.build=Some(if self.explicit.is_some() || self.vertex_input.is_some() {Build::new(std::mem::take(&mut self.soup),false)}else {Build::unreferenced(std::mem::take(&mut self.soup))});self.phase=15;}
                else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;}else {let id=face[self.cursor];if !self.removable.contains(&id) {self.corners.push(id);self.output_corners.push(Some(self.corner_sources[self.face][self.cursor]));self.output_edges.push(Some(self.edge_sources[self.face][self.cursor]));}self.cursor+=1;if self.cursor==face.len() {if self.corners.len()>=3 {self.soup.faces.push(std::mem::take(&mut self.corners));self.output_faces.push(std::mem::take(&mut self.face_sources[self.face]));}else {self.corners.clear();}self.face+=1;self.cursor=0;}}}
            },
            15=>{if self.incidence.pop_first().is_none() && self.removable.pop_first().is_none() && self.source_corners.pop_first().is_none() {self.phase=16;}},
            16=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let mut attributes=SourceAttributeRemap::new(mesh,None,None,None,None);attributes.corner_sources=Some(std::mem::take(&mut self.output_corners));attributes.edge_sources=Some(std::mem::take(&mut self.output_edges));attributes.generated_faces=Some(std::mem::take(&mut self.output_faces));self.attributes=Some(attributes);self.cursor=0;self.phase=if self.explicit.is_some() || self.vertex_input.is_some() {24}else {25};}},
            17=>{if self.cursor==snapshot.source.halfedge_count() {self.cursor=0;self.face=0;self.phase=18;}else {let edge=&snapshot.source.halfedges[self.cursor];self.source_corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}},
            18=>{if self.face==snapshot.soup.faces.len() {self.face=0;self.cursor=0;self.phase=if self.vertex_input.is_some() {26}else {1};}else {let face=&snapshot.soup.faces[self.face];if self.cursor==0 {self.face_sources.push(vec![self.face as u32]);self.corner_sources.push(Vec::new());self.edge_sources.push(Vec::new());}let corner=self.source_corners[&(self.face as u32,face[self.cursor])];let edge=if snapshot.source.faces[self.face].flipped {self.source_corners[&(self.face as u32,face[(self.cursor+1)%face.len()])]}else {corner};self.corner_sources[self.face].push(corner);self.edge_sources[self.face].push(edge);self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}},
            19=>{if let Some(face)=self.face_sources[self.remove].pop() {self.face_sources[self.keep].push(face);}else {self.phase=10;}},
            20=>{
                let a=self.edge_sources[self.a][self.ai];let b=self.edge_sources[self.b][self.bi];
                if snapshot.source.uv_seams.contains(&a) || snapshot.source.uv_seams.contains(&b) {self.reject()?;return Ok(None);}
                if self.attribute_name.is_none() {self.attribute_name=snapshot.source.attributes.first_key_value().map(|(name,_)|name.clone());}
                let Some(name)=self.attribute_name.as_ref() else {self.phase=if self.vertex_input.is_some() {30}else {7};return Ok(None);};let attribute=&snapshot.source.attributes[name];
                if attribute.domain==MeshAttributeDomain::Corner {
                    let a=self.corner_sources[self.a][(self.ai+self.endpoint)%snapshot.soup.faces[self.a].len()];let b=self.corner_sources[self.b][(self.bi+1-self.endpoint)%snapshot.soup.faces[self.b].len()];let a=attribute.indices.as_ref().map_or(a,|indices|indices[a as usize]);let b=attribute.indices.as_ref().map_or(b,|indices|indices[b as usize]);
                    if self.equality.is_none() {self.equality=Some(SourceValueEquality::new(a,b));}
                    if let Some(equal)=self.equality.as_mut().unwrap().step(&attribute.values)? {self.equality=None;if !equal {self.reject()?;return Ok(None);}self.endpoint+=1;}else {return Ok(None);}
                    if self.endpoint<2 {return Ok(None);}
                }
                let next=snapshot.source.attributes.range((std::ops::Bound::Excluded(name.clone()),std::ops::Bound::Unbounded)).next().map(|(name,_)|name.clone());self.attribute_name=next;self.endpoint=0;if self.attribute_name.is_none() {self.phase=if self.vertex_input.is_some() {30}else {7};}
            },
            21=>return self.attributes.as_mut().unwrap().advance(&mut snapshot.source),
            22=>{self.preserve_corners|=!snapshot.source.uv_seams.is_empty();if self.attribute_name.is_none() {self.attribute_name=snapshot.source.attributes.first_key_value().map(|(name,_)|name.clone());}if let Some(name)=self.attribute_name.as_ref() {self.preserve_corners|=matches!(snapshot.source.attributes[name].domain,MeshAttributeDomain::Corner|MeshAttributeDomain::Edge);self.attribute_name=snapshot.source.attributes.range((std::ops::Bound::Excluded(name.clone()),std::ops::Bound::Unbounded)).next().map(|(name,_)|name.clone());if self.attribute_name.is_none() {self.phase=17;}}else {self.phase=17;}},
            23=>{if let Some(vertices)=&self.vertex_input {if self.cursor==vertices.len() {self.cursor=0;self.phase=0;}else {let id=vertices[self.cursor];snapshot.source.vertex_position(id)?;self.selected_vertices.insert(id.0);self.cursor+=1;}}else {let edges=self.explicit.as_ref().unwrap();if self.cursor==edges.len() {self.cursor=0;self.phase=0;}else {let (a,b)=snapshot.source.edge_endpoints(edges[self.cursor])?;self.pending.insert((a.0.min(b.0),a.0.max(b.0)));self.cursor+=1;}}},
            24=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.attributes.as_mut().unwrap().vertices=Some(std::mem::take(&mut self.output_vertices));self.cursor=0;self.phase=25;}else {self.output_vertices.push(origins[self.cursor]);self.cursor+=1;}},
            25=>{let attributes=self.attributes.as_mut().unwrap();if self.cursor==attributes.mesh.face_count() {self.phase=21;}else {let face=attributes.generated_faces.as_ref().unwrap()[self.cursor][0];attributes.mesh.faces[self.cursor].smooth=snapshot.source.faces[face as usize].smooth;self.cursor+=1;}},
            26=>{if let Some(vertex)=self.selected_vertices.pop_first() {self.star_vertex=vertex;self.star_incident.clear();self.face=0;self.cursor=0;self.phase=27;}else {self.face=0;self.cursor=0;self.phase=12;}},
            27=>{if self.face==snapshot.soup.faces.len() {let face=*self.star_incident.first().ok_or(MeshKernelError::DegenerateOperation)?;self.star_origin=Vec3(snapshot.soup.positions[snapshot.soup.faces[face][0]as usize]);self.star_extent=0.0;self.normal=Normal::default();self.normal.origin=self.star_origin;self.cursor=0;self.phase=28;}else {let face=&snapshot.soup.faces[self.face];if face.is_empty() {self.face+=1;self.cursor=0;}else if face[self.cursor]==self.star_vertex {self.star_incident.push(self.face);self.face+=1;self.cursor=0;}else {self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}}},
            28=>{let face=&snapshot.soup.faces[self.star_incident[0]];let a=Vec3(snapshot.soup.positions[face[self.cursor]as usize]);let b=Vec3(snapshot.soup.positions[face[(self.cursor+1)%face.len()]as usize]);self.normal.add(a,b);self.star_extent=self.star_extent.max(a.sub(self.star_origin).length());self.cursor+=1;if self.cursor==face.len() {self.na=self.normal.value();if self.na.length()==0.0 {return Err(MeshKernelError::DegenerateOperation);}self.face=0;self.cursor=0;self.phase=29;}},
            29=>{if self.face==self.star_incident.len() {self.face=0;self.cursor=0;self.phase=30;}else {let face=&snapshot.soup.faces[self.star_incident[self.face]];let point=Vec3(snapshot.soup.positions[face[self.cursor]as usize]);if point.sub(self.star_origin).dot(self.na).abs()>self.star_extent*1e-5 {return Err(MeshKernelError::InvalidInput("vertex dissolution requires a planar star".into()));}self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}}},
            30=>{if self.face==self.star_incident.len() {self.phase=31;}else {let id=self.star_incident[self.face];let face=&snapshot.soup.faces[id];let index=self.cursor;let (a,b)=(face[index],face[(index+1)%face.len()]);self.cursor+=1;if self.cursor==face.len() {self.face+=1;self.cursor=0;}if let Some((other,corner))=self.star_edges.remove(&(b,a)) {self.a=other;self.ai=corner;self.b=id;self.bi=index;self.attribute_name=None;self.endpoint=0;self.phase=20;}else if self.star_edges.insert((a,b),(id,index)).is_some() {return Err(MeshKernelError::NonManifold);}}},
            31=>{if let Some(((a,b),(face,corner)))=self.star_edges.pop_first() {if self.star_next.insert(a,(b,face,corner)).is_some() {return Err(MeshKernelError::NonManifold);}}else {self.star_start=*self.star_next.first_key_value().ok_or(MeshKernelError::DegenerateOperation)?.0;self.star_probe=self.star_start;self.phase=32;}},
            32=>{let (next,face,corner)=self.star_next.remove(&self.star_probe).ok_or(MeshKernelError::NonManifold)?;if self.star_probe!=self.star_vertex {self.corners.push(self.star_probe);self.spliced_corners.push(self.corner_sources[face][corner]);self.spliced_edges.push(self.edge_sources[face][corner]);}else if snapshot.source.attributes.values().any(|attribute|attribute.domain==MeshAttributeDomain::Edge) || snapshot.source.uv_seams.contains(&self.edge_sources[face][corner]) {return Err(MeshKernelError::InvalidInput("boundary vertex dissolution cannot preserve an authored removed edge".into()));}self.star_probe=next;if next==self.star_start {if !self.star_next.is_empty() || self.corners.len()<3 {return Err(MeshKernelError::NonManifold);}self.keep=self.star_incident[0];self.face=0;self.cursor=0;self.phase=34;}},
            34=>{if self.face==self.star_incident.len() {self.face=0;self.phase=33;}else {let id=self.star_incident[self.face];if let Some(face)=self.face_sources[id].pop() {self.star_contributors.push(face);}else {self.face+=1;}}},
            33=>{if self.face==self.star_incident.len() {snapshot.soup.faces[self.keep]=std::mem::take(&mut self.corners);self.corner_sources[self.keep]=std::mem::take(&mut self.spliced_corners);self.edge_sources[self.keep]=std::mem::take(&mut self.spliced_edges);self.face_sources[self.keep]=std::mem::take(&mut self.star_contributors);self.phase=26;}else {let id=self.star_incident[self.face];if snapshot.soup.faces[id].pop().is_some() {self.corner_sources[id].pop();self.edge_sources[id].pop();}else {self.face+=1;}}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Merge {
    snapshot:Snapshot,coplanar:Option<Coplanar>,input:Vec<VertexId>,selected:Vec<u32>,seen:HashSet<u32>,mode:WeldMode,threshold:f32,
    roots:Vec<usize>,remap:Vec<u32>,soup:Soup,corners:Vec<u32>,unique:HashMap<u32,usize>,center:[f64;3],
    cursor:usize,i:usize,j:usize,a:usize,b:usize,face:usize,corner:usize,build:Option<Build>,phase:u8,
    quantized:bool,grid:BTreeMap<(i64,i64,i64),u32>,contributors:Vec<Vec<u32>>,source_corners:HashMap<(u32,u32),u32>,
    corner_origins:Vec<Option<u32>>,edge_origins:Vec<Option<u32>>,output_corners:Vec<Option<u32>>,output_edges:Vec<Option<u32>>,output_faces:Vec<u32>,attributes:Option<SourceAttributeRemap>,removed:usize,
}

impl Merge {
    fn new(source:HalfedgeMesh,coplanar:Option<Coplanar>,input:Vec<VertexId>,mode:WeldMode,threshold:f32,quantized:bool)->Self {
        Self {snapshot:Snapshot::new(source),coplanar,input,selected:Vec::new(),seen:HashSet::new(),mode,threshold,roots:Vec::new(),remap:Vec::new(),soup:Soup::default(),corners:Vec::new(),unique:HashMap::new(),center:[0.0;3],cursor:0,i:1,j:0,a:0,b:0,face:0,corner:0,build:None,phase:if quantized {1}else {0},quantized,grid:BTreeMap::new(),contributors:Vec::new(),source_corners:HashMap::new(),corner_origins:Vec::new(),edge_origins:Vec::new(),output_corners:Vec::new(),output_edges:Vec::new(),output_faces:Vec::new(),attributes:None,removed:0}
    }
    fn phase(&self)->&'static str {if let Some(work)=&self.coplanar {return work.phase();}match self.phase {0=>"merge-selection",1=>"snapshot",2..=4=>"merge-pairs",5..=7=>"merge-remap",8..=10=>"merge-corners",11=>"merge-reconstruct",12=>"merge-source-corners",13=>"merge-contributors",14=>"weld-grid",15=>"merge-vertex-provenance",16=>"merge-face-metadata",_=>"merge-attributes"}}
    fn next_pair(&mut self) {self.j+=1;if self.j==self.i {self.i+=1;self.j=0;}self.phase=2;}
    fn advance(&mut self)->MeshResult<Option<HalfedgeMesh>> {
        if let Some(work)=&mut self.coplanar {return work.advance(&mut self.snapshot);}
        match self.phase {
            0=>{if self.cursor==self.input.len() {if self.selected.len()<2 {return Err(MeshKernelError::EmptySelection);}self.phase=1;self.cursor=0;}else {let id=self.input[self.cursor];self.snapshot.source.vertex_position(id)?;if self.seen.insert(id.0) {self.roots.push(if self.mode==WeldMode::ByDistance {self.selected.len()}else {0});self.selected.push(id.0);}self.cursor+=1;}},
            1=>{if self.snapshot.advance() {self.phase=if self.quantized {14}else {2};}},
            2=>{if self.mode!=WeldMode::ByDistance || self.i==self.selected.len() {self.phase=5;}else if Vec3(self.snapshot.soup.positions[self.selected[self.i]as usize]).sub(Vec3(self.snapshot.soup.positions[self.selected[self.j]as usize])).length()<=self.threshold {self.a=self.i;self.b=self.j;self.phase=3;}else {self.next_pair();}},
            3=>{if self.roots[self.a]==self.a {self.phase=4;}else {self.a=self.roots[self.a];}},
            4=>{if self.roots[self.b]==self.b {self.roots[self.a.max(self.b)]=self.a.min(self.b);self.next_pair();}else {self.b=self.roots[self.b];}},
            5=>{if self.cursor==self.snapshot.soup.positions.len() {self.phase=6;self.cursor=0;}else {self.remap.push(self.cursor as u32);self.contributors.push(Vec::new());self.cursor+=1;}},
            6=>{if self.cursor==self.selected.len() {if self.mode==WeldMode::Center {self.snapshot.soup.positions[self.selected[0]as usize]=self.center.map(|value|value as f32);}self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=13;}else {self.a=self.cursor;self.phase=7;}},
            7=>{if self.roots[self.a]!=self.a {self.a=self.roots[self.a];}else {let id=self.selected[self.cursor];self.remap[id as usize]=self.selected[self.a];if self.mode==WeldMode::Center {for axis in 0..3 {self.center[axis]+=self.snapshot.soup.positions[id as usize][axis]as f64/self.selected.len()as f64;}}self.cursor+=1;self.phase=6;}},
            8=>{
                if self.face==self.snapshot.soup.faces.len() {let soup=std::mem::take(&mut self.soup);self.build=Some(if self.quantized {Build::unreferenced(soup)}else {Build::new(soup,false)});self.phase=11;}else {
                    let face=&self.snapshot.soup.faces[self.face];let original=face[self.corner];let id=self.remap[original as usize];let source_corner=self.source_corners[&(self.face as u32,original)];let end=face[(self.corner+1)%face.len()];let source_edge=if self.snapshot.source.faces[self.face].flipped {self.source_corners[&(self.face as u32,end)]}else {source_corner};
                    if self.corners.last()!=Some(&id) {self.corners.push(id);self.corner_origins.push(Some(source_corner));self.edge_origins.push(Some(source_edge));}else {*self.edge_origins.last_mut().unwrap()=Some(source_edge);}
                    self.corner+=1;if self.corner==face.len() {if self.corners.first()==self.corners.last() {self.corners.pop();self.corner_origins.pop();self.edge_origins.pop();}self.phase=9;self.cursor=0;}
                }
            },
            9=>{if self.cursor==self.corners.len() {self.phase=10;self.cursor=0;}else {if self.unique.insert(self.corners[self.cursor],self.face)==Some(self.face) {return Err(MeshKernelError::NonManifold);}self.cursor+=1;}},
            10=>{
                if self.corners.len()>=3 && self.cursor<self.corners.len() {self.output_corners.push(self.corner_origins[self.cursor]);self.output_edges.push(self.edge_origins[self.cursor]);self.cursor+=1;}else {if self.corners.len()>=3 {self.soup.faces.push(std::mem::take(&mut self.corners));self.output_faces.push(self.face as u32);}else {self.corners.clear();}self.corner_origins.clear();self.edge_origins.clear();self.face+=1;self.corner=0;self.phase=8;}
            },
            11=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.removed=self.snapshot.source.vertex_count()-mesh.vertex_count();let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.output_faces)),None,None);attributes.corner_sources=Some(std::mem::take(&mut self.output_corners));attributes.edge_sources=Some(std::mem::take(&mut self.output_edges));attributes.generated_vertices=Some(Vec::new());attributes.vertex_contributors=Some(std::mem::take(&mut self.contributors));self.attributes=Some(attributes);self.cursor=0;self.phase=15;}},
            12=>{if self.cursor==self.snapshot.source.halfedge_count() {self.phase=8;self.cursor=0;}else {let edge=&self.snapshot.source.halfedges[self.cursor];self.source_corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}},
            13=>{if self.cursor==self.remap.len() {self.cursor=0;self.phase=12;}else {self.contributors[self.remap[self.cursor]as usize].push(self.cursor as u32);self.cursor+=1;}},
            14=>{
                if self.cursor==self.snapshot.soup.positions.len() {self.removed=self.snapshot.source.vertex_count()-self.soup.positions.len();if self.removed==0 {return Ok(Some(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty())));}self.cursor=0;self.phase=12;}else {
                    let point=self.snapshot.soup.positions[self.cursor];let scale=1.0/self.threshold.max(1e-9);let key=point.map(|value|(value as f64*scale as f64).round());if key.iter().any(|value|!value.is_finite() || *value<i64::MIN as f64 || *value>=i64::MAX as f64) {return Err(MeshKernelError::InvalidInput("weld coordinate exceeds precision grid".into()));}let key=(key[0]as i64,key[1]as i64,key[2]as i64);
                    let id=if let Some(id)=self.grid.get(&key) {*id}else {let id=self.soup.positions.len()as u32;self.grid.insert(key,id);self.soup.positions.push(point);self.contributors.push(Vec::new());id};self.remap.push(id);self.contributors[id as usize].push(self.cursor as u32);self.cursor+=1;
                }
            },
            15=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.cursor==origins.len() {self.cursor=0;self.phase=16;}else {let origin=origins[self.cursor];let attributes=self.attributes.as_mut().unwrap();let group=&attributes.vertex_contributors.as_ref().unwrap()[origin as usize];attributes.generated_vertices.as_mut().unwrap().push(if group.len()==1 {SourceVertex::Original(group[0])}else {SourceVertex::Contributors(origin)});self.cursor+=1;}},
            16=>{let attributes=self.attributes.as_mut().unwrap();if self.cursor==attributes.mesh.face_count() {self.phase=17;}else {let original=attributes.faces.as_ref().unwrap()[self.cursor];attributes.mesh.faces[self.cursor].smooth=self.snapshot.source.faces[original as usize].smooth;self.cursor+=1;}},
            17=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

impl Mirror {
    fn phase(&self) -> &'static str { ["snapshot","mirror-side","mirror-vertices","mirror-corners","mirror-reconstruct","mirror-vertex-provenance","mirror-attributes"][self.phase as usize] }
    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.advance() { self.phase = 1; } }
            1 => {
                if self.vertex == self.snapshot.soup.positions.len() { self.phase = 2; self.vertex = 0; }
                else {
                    self.source_vertices.push(self.vertex as u32);
                    let point = self.snapshot.soup.positions[self.vertex][self.axis];
                    self.positive |= point > self.tolerance; self.negative |= point < -self.tolerance;
                    if self.positive && self.negative { return Err(MeshKernelError::InvalidInput("mirror geometry must lie on one side of its plane".into())); }
                    self.vertex += 1;
                }
            }
            2 => {
                if self.vertex == self.snapshot.source.vertex_count() { self.soup.positions = std::mem::take(&mut self.snapshot.soup.positions); self.phase = 3; }
                else {
                    let mut point = self.snapshot.soup.positions[self.vertex];
                    if point[self.axis].abs() <= self.tolerance { self.snapshot.soup.positions[self.vertex][self.axis] = 0.0; self.reflected.push(self.vertex as u32); }
                    else { if self.snapshot.soup.positions.len()>=100_000 { return Err(MeshKernelError::InvalidInput("mirror exceeds mesh capacity".into())); } point[self.axis] = -point[self.axis]; self.reflected.push(self.snapshot.soup.positions.len() as u32); self.snapshot.soup.positions.push(point);self.source_vertices.push(self.vertex as u32); }
                    self.vertex += 1;
                }
            }
            3 => {
                if self.face == self.snapshot.soup.faces.len() { self.build = Some(Build::new(std::mem::take(&mut self.soup),false)); self.phase = 4; }
                else {
                    let face = &self.snapshot.soup.faces[self.face];
                    let vertex = face[self.corner]; self.plane &= self.soup.positions[vertex as usize][self.axis] == 0.0;
                    self.original.push(vertex); self.mirrored.push(self.reflected[face[face.len()-1-self.corner] as usize]); self.corner += 1;
                    if self.corner == face.len() {
                        if !self.plane { if self.soup.faces.len().saturating_add(2)>100_000 || self.emitted_corners.saturating_add(self.original.len().saturating_mul(2))>600_000 { return Err(MeshKernelError::InvalidInput("mirror exceeds mesh capacity".into())); } self.emitted_corners+=self.original.len()*2; self.source_faces.extend_from_slice(&[self.face as u32,self.face as u32]);self.mirrored_faces.extend_from_slice(&[false,true]);self.soup.faces.push(std::mem::take(&mut self.original)); self.soup.faces.push(std::mem::take(&mut self.mirrored)); }
                        else { self.original.clear(); self.mirrored.clear(); }
                        self.face += 1; self.corner = 0; self.plane = true;
                    }
                }
            }
            4=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),Some(std::mem::take(&mut self.mirrored_faces)),Some(self.axis)));self.vertex=0;self.phase=5;}},
            5=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.vertex==origins.len() {self.attributes.as_mut().unwrap().vertices=Some(std::mem::take(&mut self.attribute_vertices));self.phase=6;}else {self.attribute_vertices.push(self.source_vertices[origins[self.vertex] as usize]);self.vertex+=1;}},
            6=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _ => unreachable!(),
        }
        Ok(None)
    }
}

#[derive(Clone,Copy)]
enum SourceVertex { Original(u32),Contributors(u32),Edge{a:u32,b:u32,t:f64},Face(u32),FaceSamples(u32),Weighted{vertices:[u32;4],weights:[f64;4]} }

enum SourceEqualityTask {Node(Vec<usize>),Sequence(Vec<usize>,usize,bool),Text(Vec<usize>,usize,Option<usize>)}
struct SourceValueEquality {a:u32,b:u32,tasks:Vec<SourceEqualityTask>}
impl SourceValueEquality {
    fn new(a:u32,b:u32)->Self {Self {a,b,tasks:vec![SourceEqualityTask::Node(Vec::new())]}}
    fn value<'a>(mut value:&'a protocol::value::DslValue,path:&[usize])->Option<&'a protocol::value::DslValue> {for index in path {value=match value {protocol::value::DslValue::Array(values)=>values.get(*index)?,protocol::value::DslValue::Object(values)=>&values.get(*index)?.1,_=>return None};}Some(value)}
    fn step(&mut self,values:&[protocol::value::DslValue])->MeshResult<Option<bool>> {
        use protocol::value::DslValue;
        let Some(task)=self.tasks.pop() else {return Ok(Some(true));};
        let path=match &task {SourceEqualityTask::Node(path)|SourceEqualityTask::Sequence(path,_,_)|SourceEqualityTask::Text(path,_,_)=>path};
        if path.len()>64 || self.tasks.len()>256 {return Err(MeshKernelError::InvalidInput("mesh constant comparison exceeds nesting limit".into()));}
        let (a,b)=(Self::value(&values[self.a as usize],path).unwrap(),Self::value(&values[self.b as usize],path).unwrap());
        match task {
            SourceEqualityTask::Node(path)=>match (a,b) {
                (DslValue::String(a),DslValue::String(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Text(path,0,None));},
                (DslValue::Array(a),DslValue::Array(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,false));},
                (DslValue::Object(a),DslValue::Object(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,true));},
                (DslValue::Bytes(a),DslValue::Bytes(b))=>{if a.len()!=b.len() {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Sequence(path,0,false));},
                (DslValue::Null,DslValue::Null)=>{},
                (DslValue::Bool(a),DslValue::Bool(b))=>if a!=b {return Ok(Some(false));},
                (DslValue::Number(a),DslValue::Number(b))=>if a!=b {return Ok(Some(false));},
                _=>return Ok(Some(false)),
            },
            SourceEqualityTask::Sequence(path,index,object)=>{
                let length=match a {DslValue::Array(values)=>values.len(),DslValue::Object(values)=>values.len(),DslValue::Bytes(values)=>values.len(),_=>unreachable!()};
                if index<length {
                    self.tasks.push(SourceEqualityTask::Sequence(path.clone(),index+1,object));
                    if let (DslValue::Bytes(a),DslValue::Bytes(b))=(a,b) {if a[index]!=b[index] {return Ok(Some(false));}}
                    else {let mut child=path.clone();child.push(index);self.tasks.push(SourceEqualityTask::Node(child));if object {self.tasks.push(SourceEqualityTask::Text(path,0,Some(index)));}}
                }
            },
            SourceEqualityTask::Text(path,offset,key)=>{
                let (a,b)=match key {Some(index)=>(a.as_object().unwrap()[index].0.as_str(),b.as_object().unwrap()[index].0.as_str()),None=>(a.as_str().unwrap(),b.as_str().unwrap())};if a.len()!=b.len() {return Ok(Some(false));}
                if offset<a.len() {let (a,b)=(a[offset..].chars().next().unwrap(),b[offset..].chars().next().unwrap());if a!=b {return Ok(Some(false));}self.tasks.push(SourceEqualityTask::Text(path,offset+a.len_utf8(),key));}
            },
        }
        Ok(if self.tasks.is_empty() {Some(true)}else {None})
    }
}

struct SourceAttributeRemap {
    vertex_contributors:Option<Vec<Vec<u32>>>,
    corner_faces:Option<Vec<u32>>,
    generated_vertices:Option<Vec<SourceVertex>>,generated_faces:Option<Vec<Vec<u32>>>,corner_sources:Option<Vec<Option<u32>>>,edge_sources:Option<Vec<Option<u32>>>,edge_source_keys:Option<Vec<(u32,u32)>>,
    face_values_base:usize,
    influence:Option<SourceVertex>,influence_face:u32,influence_vertex:u32,weight_cursor:usize,weight_next:u32,weight_start:u32,sum:[f64;16],width:usize,total:f64,best:Option<(u32,f64,u32)>,constant:Option<u32>,equality:Option<SourceValueEquality>,numeric_samples:BTreeMap<(u32,u32),u32>,
    mesh:HalfedgeMesh,vertices:Option<Vec<u32>>,faces:Option<Vec<u32>>,mirrored_faces:Option<Vec<bool>>,normal_axis:Option<usize>,corners:HashMap<(u32,u32),u32>,name:Option<String>,indices:Vec<u32>,normal_values:Vec<protocol::value::DslValue>,normal_samples:BTreeMap<(u32,bool),u32>,cursor:usize,phase:u8,
}
impl SourceAttributeRemap {
    fn new(mesh:HalfedgeMesh,vertices:Option<Vec<u32>>,faces:Option<Vec<u32>>,mirrored_faces:Option<Vec<bool>>,normal_axis:Option<usize>)->Self {Self {vertex_contributors:None,corner_faces:None,generated_vertices:None,generated_faces:None,corner_sources:None,edge_sources:None,edge_source_keys:None,face_values_base:0,influence:None,influence_face:0,influence_vertex:0,weight_cursor:0,weight_next:0,weight_start:0,sum:[0.0;16],width:0,total:0.0,best:None,constant:None,equality:None,numeric_samples:BTreeMap::new(),mesh,vertices,faces,mirrored_faces,normal_axis,corners:HashMap::new(),name:None,indices:Vec::new(),normal_values:Vec::new(),normal_samples:BTreeMap::new(),cursor:0,phase:0}}
    fn advance(&mut self,source:&mut HalfedgeMesh)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{
                if source.attributes.is_empty() && source.uv_seams.is_empty() {self.phase=2;}else if self.cursor==source.halfedge_count() {self.cursor=0;self.phase=1;}else {let edge=&source.halfedges[self.cursor];self.corners.insert((edge.face.unwrap(),edge.vertex),self.cursor as u32);self.cursor+=1;}
            }
            1=>{
                if source.attributes.is_empty() {self.cursor=0;self.phase=7;return Ok(None);}
                if self.name.is_none() {self.name=source.attributes.first_key_value().map(|(name,_)|name.clone());}
                let attribute=&source.attributes[self.name.as_ref().unwrap()];
                let reflect=attribute.semantic==MeshAttributeSemantic::Normal && self.normal_axis.is_some();
                let domain=if reflect && attribute.domain==MeshAttributeDomain::Vertex {MeshAttributeDomain::Corner}else {attribute.domain};
                let count=match domain {MeshAttributeDomain::Vertex=>self.mesh.vertex_count(),MeshAttributeDomain::Face=>self.mesh.face_count(),_=>self.mesh.halfedge_count()};
                if self.cursor==count {
                    if (reflect || (attribute.interpolation==MeshAttributeInterpolation::Linear && self.generated_vertices.is_some() && domain!=MeshAttributeDomain::Face)) && !attribute.values.is_empty() {self.phase=5;return Ok(None);}
                    if self.generated_faces.is_some() && domain==MeshAttributeDomain::Face && attribute.interpolation==MeshAttributeInterpolation::Linear && !self.normal_values.is_empty() {self.face_values_base=attribute.values.len();self.weight_cursor=0;self.phase=8;return Ok(None);}
                    let name=self.name.take().unwrap();let mut attribute=source.attributes.remove(&name).unwrap();attribute.domain=domain;attribute.indices=Some(std::mem::take(&mut self.indices));
                    if reflect || (attribute.interpolation==MeshAttributeInterpolation::Linear && self.generated_vertices.is_some() && domain!=MeshAttributeDomain::Face) {attribute.values=std::mem::take(&mut self.normal_values);}
                    self.mesh.attributes.insert(name,attribute);self.cursor=0;
                }else {
                    if domain==MeshAttributeDomain::Edge {if let Some(keys)=&self.edge_source_keys {let original=*self.corners.get(&keys[self.cursor]).ok_or_else(||MeshKernelError::InvalidInput("mesh source edge key is absent".into()))?;let original=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear && self.generated_vertices.is_some() {if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len()as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample}}else {original};self.indices.push(sample);self.cursor+=1;return Ok(None);}}
                    if domain==MeshAttributeDomain::Face && self.generated_faces.as_ref().is_some_and(|faces|faces[self.cursor].len()>1) {
                        self.influence=Some(SourceVertex::FaceSamples(self.cursor as u32));self.influence_face=self.cursor as u32;self.influence_vertex=self.cursor as u32;self.weight_cursor=0;self.sum=[0.0;16];self.width=0;self.total=0.0;self.best=None;self.constant=None;self.phase=3;return Ok(None);
                    }
                    if !reflect && matches!(domain,MeshAttributeDomain::Corner|MeshAttributeDomain::Edge) {
                        let overrides=if domain==MeshAttributeDomain::Edge {&self.edge_sources}else {&self.corner_sources};
                        if let Some(original)=overrides.as_ref().and_then(|sources|sources[self.cursor]) {let original=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear && self.generated_vertices.is_some() {if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len()as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample}}else {original};self.indices.push(sample);self.cursor+=1;return Ok(None);}
                    }
                    if let Some(vertices)=&self.generated_vertices {
                        if domain!=MeshAttributeDomain::Face {
                            let (vertex,face)=if domain==MeshAttributeDomain::Vertex {(self.cursor as u32,u32::MAX)}else {let edge=&self.mesh.halfedges[self.cursor];(edge.vertex,self.corner_faces.as_ref().map_or_else(||self.faces.as_ref().map_or(edge.face.unwrap(),|faces|faces[edge.face.unwrap() as usize]),|faces|faces[self.cursor]))};
                            if attribute.interpolation==MeshAttributeInterpolation::Linear {if let Some(index)=self.numeric_samples.get(&(vertex,face)) {self.indices.push(*index);self.cursor+=1;return Ok(None);}}
                            let mut influence=vertices[vertex as usize];
                            if domain==MeshAttributeDomain::Edge {
                                if let SourceVertex::Original(start)=influence {
                                    let end=vertices[self.mesh.halfedges[self.mesh.halfedges[self.cursor].next as usize].vertex as usize];
                                    let pair=match end {SourceVertex::Original(end)=>Some((start,end)),SourceVertex::Edge{a,b,..} if start==a || start==b=>Some((a,b)),_=>None};
                                    if let Some((a,b))=pair {if let Some(edge)=self.corners.get(&(face,a)).copied() {if source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b {influence=SourceVertex::Original(a);}else if let Some(edge)=self.corners.get(&(face,b)).copied() {if source.halfedges[source.halfedges[edge as usize].next as usize].vertex==a {influence=SourceVertex::Original(b);}}}}
                                }
                                if let SourceVertex::Edge{a,b,..}=influence {let edge=self.corners.get(&(face,a)).copied();let opposite=self.corners.get(&(face,b)).copied();if edge.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b) || opposite.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==a) {influence=SourceVertex::Original(if edge.is_some_and(|edge|source.halfedges[source.halfedges[edge as usize].next as usize].vertex==b) {a}else {b});}}
                            }
                            if attribute.interpolation==MeshAttributeInterpolation::Linear {
                                if let SourceVertex::Original(vertex)=influence {
                                    let id=if attribute.domain==MeshAttributeDomain::Vertex {vertex}else {*self.corners.get(&(face,vertex)).ok_or_else(||MeshKernelError::InvalidInput("mesh source corner is absent".into()))?};
                                    let original=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                                    let sample=if let Some(sample)=self.normal_samples.get(&(original,false)) {*sample}else {let sample=self.normal_values.len() as u32;self.normal_values.push(attribute.values[original as usize].clone());self.normal_samples.insert((original,false),sample);sample};
                                    self.indices.push(sample);self.cursor+=1;return Ok(None);
                                }
                            }
                            self.influence=Some(influence);self.influence_face=face;self.influence_vertex=vertex;self.weight_cursor=0;self.sum=[0.0;16];self.width=0;self.total=0.0;self.best=None;self.constant=None;
                            if let SourceVertex::Face(face)=influence {self.weight_start=source.faces[face as usize].halfedge;self.weight_next=self.weight_start;}
                            self.phase=3;return Ok(None);
                        }
                    }
                    let source_vertex=|vertex|self.vertices.as_ref().map_or(vertex,|vertices|vertices[vertex as usize]);
                    let source_face=|face|self.faces.as_ref().map_or(face,|faces|faces[face as usize]);
                    let (id,mirrored)=match domain {
                        MeshAttributeDomain::Vertex=>(source_vertex(self.cursor as u32),false),
                        MeshAttributeDomain::Face=>(self.generated_faces.as_ref().map_or_else(||source_face(self.cursor as u32),|faces|faces[self.cursor][0]),false),
                        _=>{let overrides=if domain==MeshAttributeDomain::Edge {&self.edge_sources}else {&self.corner_sources};
                            if let Some(original)=overrides.as_ref().and_then(|sources|sources[self.cursor]) {let index=attribute.indices.as_ref().map_or(original,|indices|indices[original as usize]);self.indices.push(index);self.cursor+=1;return Ok(None);}
                            let edge=&self.mesh.halfedges[self.cursor];let face=edge.face.unwrap();let vertex=source_vertex(edge.vertex);let original_face=source_face(face);let mut corner=self.corners[&(original_face,vertex)];
                            if domain==MeshAttributeDomain::Edge {let end=source_vertex(self.mesh.halfedges[edge.next as usize].vertex);if source.halfedges[source.halfedges[corner as usize].next as usize].vertex!=end {corner=self.corners[&(original_face,end)];}}
                            (if attribute.domain==MeshAttributeDomain::Vertex {vertex}else {corner},self.mirrored_faces.as_ref().is_some_and(|faces|faces[face as usize]))
                        },
                    };
                    let original=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                    let index=if reflect {
                        if let Some(index)=self.normal_samples.get(&(original,mirrored)) {*index}else {let tuple=attribute.values[original as usize].as_array().unwrap();let axis=self.normal_axis.unwrap();let normal=protocol::value::DslValue::Array(tuple.iter().enumerate().map(|(coordinate,value)|protocol::value::DslValue::float(value.as_f64().unwrap()*if mirrored && coordinate==axis {-1.0}else {1.0})).collect());let index=self.normal_values.len() as u32;self.normal_values.push(normal);self.normal_samples.insert((original,mirrored),index);index}
                    }else {original};
                    self.indices.push(index);self.cursor+=1;
                }
            }
            2=>{self.mesh.materials=std::mem::take(&mut source.materials);self.mesh.textures=std::mem::take(&mut source.textures);return Ok(Some(std::mem::replace(&mut self.mesh,HalfedgeMesh::empty())));},
            3=>{
                let influence=self.influence.unwrap();
                let contribution=match influence {
                    SourceVertex::Original(vertex)=>(self.weight_cursor==0).then_some((vertex,1.0)),
                    SourceVertex::Contributors(group)=>self.vertex_contributors.as_ref().unwrap()[group as usize].get(self.weight_cursor).map(|vertex|(*vertex,1.0)),
                    SourceVertex::Edge{a,b,t}=>match self.weight_cursor {0=>Some((a,1.0-t)),1=>Some((b,t)),_=>None},
                    SourceVertex::Weighted{vertices,weights}=>vertices.get(self.weight_cursor).map(|vertex|(*vertex,weights[self.weight_cursor])),
                    SourceVertex::FaceSamples(face)=>self.generated_faces.as_ref().unwrap()[face as usize].get(self.weight_cursor).map(|face|(*face,1.0)),
                    SourceVertex::Face(_)=>if self.weight_cursor>0 && self.weight_next==self.weight_start {None}else {let edge=&source.halfedges[self.weight_next as usize];self.weight_next=edge.next;Some((edge.vertex,1.0))},
                };
                let attribute=&source.attributes[self.name.as_ref().unwrap()];
                if let Some((vertex,weight))=contribution {
                    self.weight_cursor+=1;if !weight.is_finite() || weight<0.0 {return Err(MeshKernelError::InvalidInput("mesh interpolation requires finite nonnegative weights".into()));}if weight==0.0 {return Ok(None);}
                    let id=if matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Face) {vertex}else {match self.corners.get(&(self.influence_face,vertex)) {Some(id)=>*id,None if matches!(influence,SourceVertex::Edge{..}|SourceVertex::Contributors(_))=>return Ok(None),None=>return Err(MeshKernelError::InvalidInput("mesh corner interpolation source is absent".into()))}};
                    let sample=attribute.indices.as_ref().map_or(id,|indices|indices[id as usize]);
                    if self.best.is_none_or(|(best,best_weight,_)|weight>best_weight || weight==best_weight && id<best) {self.best=Some((id,weight,sample));}
                    self.total+=weight;
                    match attribute.interpolation {
                        MeshAttributeInterpolation::Linear=>{
                            let value=&attribute.values[sample as usize];let width=value.as_array().map_or(0,|values|values.len());if self.total==weight {self.width=width;}else if self.total>weight && self.width!=width {return Err(MeshKernelError::InvalidInput("mesh linear interpolation dimensions disagree".into()));}
                            if width==0 {self.sum[0]+=value.as_f64().unwrap()*weight;}else {for axis in 0..width {self.sum[axis]+=value.as_array().unwrap()[axis].as_f64().unwrap()*weight;}}
                        },
                        MeshAttributeInterpolation::Constant=>{if let Some(first)=self.constant {if first!=sample {self.equality=Some(SourceValueEquality::new(first,sample));self.phase=4;}}else {self.constant=Some(sample);}},
                        MeshAttributeInterpolation::Nearest=>{},
                    }
                }else {
                    if self.total<=0.0 || !self.total.is_finite() {return Err(MeshKernelError::InvalidInput("mesh interpolation weights are empty".into()));}
                    let sample=if attribute.interpolation==MeshAttributeInterpolation::Linear {
                        let width=if self.width==0 {1}else {self.width};let mut values=self.sum[..width].iter().map(|value|*value/self.total).collect::<Vec<_>>();
                        if attribute.semantic==MeshAttributeSemantic::Normal {let length=values[0].hypot(values[1]).hypot(values[2]);if length==0.0 || !length.is_finite() {return Err(MeshKernelError::InvalidInput("mesh interpolated normal is zero".into()));}for value in &mut values {*value/=length;}}
                        if values.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::InvalidInput("mesh interpolated channel exceeds numeric range".into()));}
                        let sample=if attribute.domain==MeshAttributeDomain::Face {attribute.values.len().saturating_add(self.normal_values.len()) as u32}else {self.normal_values.len() as u32};self.normal_values.push(if self.width==0 {protocol::value::DslValue::float(values[0])}else {protocol::value::DslValue::Array(values.into_iter().map(protocol::value::DslValue::float).collect())});self.numeric_samples.insert((self.influence_vertex,self.influence_face),sample);sample
                    }else if attribute.interpolation==MeshAttributeInterpolation::Constant {self.constant.unwrap()}else {self.best.unwrap().2};
                    self.indices.push(sample);self.cursor+=1;self.phase=1;
                }
            }
            4=>{let attribute=&source.attributes[self.name.as_ref().unwrap()];if let Some(equal)=self.equality.as_mut().unwrap().step(&attribute.values)? {self.equality=None;if !equal {return Err(MeshKernelError::InvalidInput(format!("constant mesh attribute '{}' has conflicting source values",self.name.as_ref().unwrap())));}self.phase=3;}},
            7=>{
                if source.uv_seams.is_empty() || self.cursor==self.mesh.halfedge_count() {self.phase=2;return Ok(None);}
                if let Some(keys)=&self.edge_source_keys {let original=*self.corners.get(&keys[self.cursor]).ok_or(MeshKernelError::InvalidHandle)?;if source.uv_seams.contains(&original) || source.halfedges[original as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin)) {self.mesh.uv_seams.insert(self.cursor as u32);}self.cursor+=1;return Ok(None);}
                if let Some(original)=self.edge_sources.as_ref().and_then(|sources|sources[self.cursor]) {if source.uv_seams.contains(&original) || source.halfedges[original as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin)) {self.mesh.uv_seams.insert(self.cursor as u32);}self.cursor+=1;return Ok(None);}
                let edge=&self.mesh.halfedges[self.cursor];let face=self.corner_faces.as_ref().map_or_else(||self.faces.as_ref().map_or(edge.face.unwrap(),|faces|faces[edge.face.unwrap() as usize]),|faces|faces[self.cursor]);
                let origin=|vertex:u32|self.generated_vertices.as_ref().map_or_else(||SourceVertex::Original(self.vertices.as_ref().map_or(vertex,|vertices|vertices[vertex as usize])),|vertices|vertices[vertex as usize]);
                let (a,b)=(origin(edge.vertex),origin(self.mesh.halfedges[edge.next as usize].vertex));
                let pair=match (a,b) {(SourceVertex::Original(a),SourceVertex::Original(b))=>Some((a,b)),(SourceVertex::Original(vertex),SourceVertex::Edge{a,b,..})|(SourceVertex::Edge{a,b,..},SourceVertex::Original(vertex)) if vertex==a || vertex==b=>Some((a,b)),(SourceVertex::Edge{a,b,..},SourceVertex::Edge{a:c,b:d,..}) if a==c && b==d || a==d && b==c=>Some((a,b)),_=>None};
                if let Some((a,b))=pair {let original=self.corners.get(&(face,a)).copied().filter(|id|source.halfedges[source.halfedges[*id as usize].next as usize].vertex==b).or_else(||self.corners.get(&(face,b)).copied().filter(|id|source.halfedges[source.halfedges[*id as usize].next as usize].vertex==a));if original.is_some_and(|id|source.uv_seams.contains(&id) || source.halfedges[id as usize].twin.is_some_and(|twin|source.uv_seams.contains(&twin))) {self.mesh.uv_seams.insert(self.cursor as u32);}}
                self.cursor+=1;
            }
            5=>{let attribute=source.attributes.get_mut(self.name.as_ref().unwrap()).unwrap();if attribute.values.pop().is_none() {self.phase=6;}},
            6=>{if self.normal_samples.pop_first().is_none() && self.numeric_samples.pop_first().is_none() {self.phase=1;}},
            8=>{if let Some(value)=self.normal_values.pop() {source.attributes.get_mut(self.name.as_ref().unwrap()).unwrap().values.push(value);}else {self.phase=9;}},
            9=>{if self.weight_cursor==self.indices.len() {self.phase=6;}else {let index=&mut self.indices[self.weight_cursor];if *index as usize>=self.face_values_base {let count=source.attributes[self.name.as_ref().unwrap()].values.len()-self.face_values_base;*index=(self.face_values_base+count-1-(*index as usize-self.face_values_base)) as u32;}self.weight_cursor+=1;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Normal {
    origin: Vec3,
    sum: [f64; 3],
}

impl Default for Normal {
    fn default() -> Self { Self { origin: Vec3::ZERO, sum: [0.0; 3] } }
}

impl Normal {
    fn add(&mut self, a: Vec3, b: Vec3) {
        let a = [0, 1, 2].map(|i| a.0[i] as f64 - self.origin.0[i] as f64);
        let b = [0, 1, 2].map(|i| b.0[i] as f64 - self.origin.0[i] as f64);
        self.sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        self.sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        self.sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    fn value(&self) -> Vec3 {
        let length = self.sum[0].hypot(self.sum[1]).hypot(self.sum[2]);
        if length == 0.0 { Vec3::ZERO } else { Vec3(self.sum.map(|v| (v / length) as f32)).normalize() }
    }
}

#[derive(Default)]
struct Soup {
    positions: Vec<[f32; 3]>,
    faces: Vec<Vec<u32>>,
    normals: Vec<Vec3>,
}

struct Snapshot {
    source: HalfedgeMesh,
    soup: Soup,
    vertex: usize,
    face: usize,
    halfedge: Option<u32>,
    corners: Vec<u32>,
    normal: Normal,
    normal_corner: usize,
    normalizing: bool,
    reversing: Option<usize>,
}

impl Snapshot {
    fn new(source: HalfedgeMesh) -> Self {
        Self { source, soup: Soup::default(), vertex: 0, face: 0, halfedge: None, corners: Vec::new(), normal: Normal::default(), normal_corner: 0, normalizing: false, reversing: None }
    }

    fn advance(&mut self) -> bool {
        if self.vertex < self.source.vertices.len() {
            self.soup.positions.push(self.source.vertices[self.vertex].position);
            self.vertex += 1;
        } else if self.face < self.source.faces.len() {
            let face = &self.source.faces[self.face];
            if let Some(index) = self.reversing {
                if index < self.corners.len() / 2 {
                    let opposite = self.corners.len() - 1 - index;
                    self.corners.swap(index, opposite);
                    self.reversing = Some(index + 1);
                } else { self.reversing = None; self.normalizing = true; }
            } else if self.normalizing {
                let i = self.normal_corner;
                let a = self.corners[i];
                let b = self.corners[(i + 1) % self.corners.len()];
                if i == 0 { self.normal.origin = Vec3(self.soup.positions[a as usize]); }
                self.normal.add(Vec3(self.soup.positions[a as usize]), Vec3(self.soup.positions[b as usize]));
                self.normal_corner += 1;
                if self.normal_corner == self.corners.len() {
                    self.soup.normals.push(self.normal.value());
                    self.soup.faces.push(std::mem::take(&mut self.corners));
                    self.face += 1;
                    self.halfedge = None;
                    self.normalizing = false;
                    self.normal_corner = 0;
                    self.normal = Normal::default();
                }
            } else {
                let id = self.halfedge.unwrap_or(face.halfedge);
                let edge = &self.source.halfedges[id as usize];
                self.corners.push(edge.vertex);
                self.halfedge = Some(edge.next);
                if edge.next == face.halfedge {
                    if face.flipped { self.reversing = Some(0); } else { self.normalizing = true; }
                }
            }
        } else { return true; }
        false
    }
}

struct Build {
    origin_vertices:Vec<u32>,
    soup: Soup,
    mesh: HalfedgeMesh,
    used: HashSet<u32>,
    remap: HashMap<u32, u32>,
    edges: HashMap<(u32, u32), u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: u32,
    normal: Normal,
    phase: u8,
    closed: bool,
    retain_unreferenced: bool,
    admitted_corners: usize,
}

impl Build {
    fn new(soup: Soup, closed: bool) -> Self {
        Self { origin_vertices:Vec::new(),soup, mesh: HalfedgeMesh::empty(), used: HashSet::new(), remap: HashMap::new(), edges: HashMap::new(), face: 0, corner: 0, vertex: 0, start: 0, normal: Normal::default(), phase: 0, closed, retain_unreferenced: false, admitted_corners: 0 }
    }
    fn unreferenced(soup: Soup) -> Self { let mut build = Self::new(soup,false); build.retain_unreferenced = true; build }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.face == self.soup.faces.len() { self.phase = 1; self.face = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    if self.soup.faces.len()>100_000 || self.soup.positions.len()>100_000 || self.admitted_corners>=600_000 { return Err(MeshKernelError::InvalidInput("generated mesh exceeds capacity".into())); } self.admitted_corners+=1;
                    if face.len() < 3 { return Err(MeshKernelError::DegenerateOperation); }
                    self.used.insert(face[self.corner]);
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            1 => {
                if self.vertex == self.soup.positions.len() { self.phase = 2; }
                else {
                    if self.retain_unreferenced || self.used.contains(&(self.vertex as u32)) {
                        self.remap.insert(self.vertex as u32, self.mesh.vertices.len() as u32);
                        self.origin_vertices.push(self.vertex as u32);
                        self.mesh.add_vertex(self.soup.positions[self.vertex]);
                    }
                    self.vertex += 1;
                }
            }
            2 => {
                if self.face == self.soup.faces.len() { self.phase = 3; self.face = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    let a = *self.remap.get(&face[self.corner]).ok_or(MeshKernelError::InvalidHandle)?;
                    let b = *self.remap.get(&face[(self.corner + 1) % face.len()]).ok_or(MeshKernelError::InvalidHandle)?;
                    let id = self.mesh.halfedges.len() as u32;
                    if self.corner == 0 { self.start = id; }
                    let next = if self.corner + 1 == face.len() { self.start } else { id + 1 };
                    let twin = self.edges.get(&(b, a)).copied();
                    self.mesh.halfedges.push(HalfEdge { vertex: a, twin, next, face: Some(self.face as u32), uv: [0.0, 0.0] });
                    if let Some(twin) = twin { self.mesh.halfedges[twin as usize].twin = Some(id); }
                    self.edges.insert((a, b), id);
                    self.corner += 1;
                    if self.corner == face.len() {
                        self.mesh.faces.push(MeshFace { halfedge: self.start, smooth: false, flipped: false });
                        let a = self.mesh.halfedges[self.start as usize].vertex;
                        self.mesh.vertices[a as usize].halfedge = Some(self.start);
                        self.corner = 0;
                        self.face += 1;
                    }
                }
            }
            3 => {
                if self.face == self.soup.faces.len() { self.phase = 5; self.vertex = 0; }
                else {
                    let face = &self.soup.faces[self.face];
                    let a = Vec3(self.soup.positions[face[self.corner] as usize]);
                    let b = Vec3(self.soup.positions[face[(self.corner + 1) % face.len()] as usize]);
                    if self.corner == 0 { self.normal.origin = a; }
                    self.normal.add(a, b);
                    self.corner += 1;
                    if self.corner == face.len() { self.phase = 4; self.corner = 0; }
                }
            }
            4 => {
                let face = &self.soup.faces[self.face];
                let vertex = &mut self.mesh.vertices[self.remap[&face[self.corner]] as usize];
                if vertex.normal.is_none() { vertex.normal = Some(self.normal.value().0); }
                self.corner += 1;
                if self.corner == face.len() { self.phase = 3; self.corner = 0; self.face += 1; self.normal = Normal::default(); }
            }
            5 => {
                if self.vertex == self.mesh.halfedges.len() { return Ok(Some(std::mem::replace(&mut self.mesh, HalfedgeMesh::empty()))); }
                if self.closed {
                    let edge = &self.mesh.halfedges[self.vertex];
                    if edge.twin.is_none_or(|twin| self.mesh.halfedges[twin as usize].twin != Some(self.vertex as u32)) { return Err(MeshKernelError::NonManifold); }
                }
                self.vertex += 1;
            }
            _ => unreachable!(),
        }
        Ok(None)
    }
}

struct Bevel {
    snapshot: Snapshot,
    edges: Vec<EdgeId>,
    amount: f32,
    segments: u32,
    extent: f32,
    epsilon: f32,
    face: usize,
    vertex: usize,
    edge: usize,
    selected: HashSet<u32>,
    planes: Vec<(Vec3, f32)>,
    profile: Option<Profile>,
    clip: Option<Clip>,
    build: Option<Build>,
    attributes:Option<SourceAttributeRemap>,
    retain_channels:bool,
    phase: u8,
}

struct Profile {
    faces: [usize; 2],
    directions: [Vec3; 2],
    origin: Vec3,
    normal: Vec3,
    tangent: Vec3,
    center: Vec3,
    radius: f32,
    theta: f32,
    clearance: f32,
    side: usize,
    corner: usize,
    segment: u32,
}

impl Bevel {
    fn new(source: HalfedgeMesh, edges: Vec<EdgeId>, amount: f32, segments: u32) -> Self {
        Self {retain_channels:!source.attributes.is_empty() || !source.uv_seams.is_empty(),attributes:None,snapshot: Snapshot::new(source), edges, amount, segments, extent: 0.0, epsilon: 0.0, face: 0, vertex: 0, edge: 0, selected: HashSet::new(), planes: Vec::new(), profile: None, clip: None, build: None, phase: 0 }
    }

    fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1..=3 => "validate", 4 => "profile", 5 => "clip", 6=>"reconstruct",7=>"bevel-clip-reconstruct",8=>"bevel-source-vertices",9=>"bevel-clip-attributes",10=>"snapshot",_=>"bevel-final-attributes" }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.advance() { self.phase = 1; } }
            1 => {
                let positions = &self.snapshot.soup.positions;
                if self.vertex == positions.len() {
                    self.epsilon = self.extent * 1e-6;
                    if self.epsilon == 0.0 { return Err(MeshKernelError::DegenerateOperation); }
                    self.vertex = 0;
                    self.phase = 2;
                } else { self.extent = self.extent.max(Vec3(positions[self.vertex]).sub(Vec3(positions[0])).length()); self.vertex += 1; }
            }
            2 => {
                let soup = &self.snapshot.soup;
                if self.face == soup.faces.len() { self.phase = 3; self.vertex = 0; }
                else {
                    let normal = soup.normals[self.face];
                    let origin = Vec3(soup.positions[soup.faces[self.face][0] as usize]);
                    if normal.length() == 0.0 || Vec3(soup.positions[self.vertex]).sub(origin).dot(normal) > self.epsilon { return Err(MeshKernelError::InvalidInput("bevel requires an outward convex mesh".into())); }
                    self.vertex += 1;
                    if self.vertex == soup.positions.len() { self.vertex = 0; self.face += 1; }
                }
            }
            3 => {
                if self.vertex == self.snapshot.source.halfedges.len() { self.phase = 4; }
                else {
                    if self.snapshot.source.halfedges[self.vertex].twin.is_none() { return Err(MeshKernelError::InvalidInput("bevel requires a closed manifold mesh".into())); }
                    self.vertex += 1;
                }
            }
            4 => {
                if let Some(profile) = &mut self.profile {
                    if profile.side < 2 {
                        let face = &self.snapshot.soup.faces[profile.faces[profile.side]];
                        let point = Vec3(self.snapshot.soup.positions[face[profile.corner] as usize]);
                        let distance = profile.origin.sub(point).dot(profile.directions[profile.side]);
                        if distance > self.epsilon { profile.clearance = profile.clearance.min(distance); }
                        profile.corner += 1;
                        if profile.corner == face.len() { profile.corner = 0; profile.side += 1; }
                    } else {
                        if self.amount >= profile.clearance * 0.5 || self.amount <= self.epsilon { return Err(MeshKernelError::InvalidInput("bevel width must stay below half the neighboring face clearance".into())); }
                        let angle = profile.theta * (profile.segment as f32 + 0.5) / self.segments as f32;
                        let normal = profile.normal.scale(angle.cos()).add(profile.tangent.scale(angle.sin()));
                        let distance = normal.dot(profile.center) + profile.radius * (profile.theta / (2.0 * self.segments as f32)).cos();
                        self.planes.push((normal, distance));
                        profile.segment += 1;
                        if profile.segment == self.segments { self.profile = None; }
                    }
                } else if self.edge == self.edges.len() { self.phase = 5; self.edge = 0; }
                else {
                    let id = self.edges[self.edge].0;
                    self.edge += 1;
                    let he = &self.snapshot.source.halfedges[id as usize];
                    let twin = he.twin.ok_or(MeshKernelError::NonManifold)?;
                    if self.selected.insert(id.min(twin)) {
                        let faces = [he.face.ok_or(MeshKernelError::InvalidHandle)? as usize, self.snapshot.source.halfedges[twin as usize].face.ok_or(MeshKernelError::InvalidHandle)? as usize];
                        let n1 = self.snapshot.soup.normals[faces[0]];
                        let n2 = self.snapshot.soup.normals[faces[1]];
                        let cosine = n1.dot(n2).clamp(-1.0, 1.0);
                        let theta = cosine.acos();
                        let sine = theta.sin();
                        if sine.abs() < 1e-5 { return Err(MeshKernelError::DegenerateOperation); }
                        let origin = Vec3(self.snapshot.soup.positions[he.vertex as usize]);
                        let tangent = n2.sub(n1.scale(cosine)).scale(1.0 / sine);
                        self.profile = Some(Profile { faces, directions: [tangent, n1.sub(n2.scale(cosine)).scale(1.0 / sine)], origin, normal: n1, tangent, center: origin.sub(n1.add(n2).scale(self.amount / sine)), radius: self.amount * (1.0 + cosine) / sine, theta, clearance: f32::INFINITY, side: 0, corner: 0, segment: 0 });
                    }
                }
            }
            5 => {
                if let Some(clip) = &mut self.clip {
                    if clip.advance(&mut self.snapshot.soup)? {if self.retain_channels {self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=7;}else {self.clip=None;self.edge+=1;}}
                } else if self.edge < self.planes.len() {
                    let (normal, distance) = self.planes[self.edge];
                    self.clip = Some(Clip::new(normal, distance, self.epsilon));
                } else {
                    self.build = Some(Build::new(std::mem::take(&mut self.snapshot.soup), true));
                    self.phase = 6;
                }
            }
            6 => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.face_count() < 4 { return Err(MeshKernelError::InvalidInput("bevel removed the solid".into())); }
                    self.attributes=Some(SourceAttributeRemap::new(mesh,Some(std::mem::take(&mut self.build.as_mut().unwrap().origin_vertices)),None,None,None));self.phase=11;
                }
            }
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let clip=self.clip.as_mut().unwrap();let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut clip.source_faces)),None,None);attributes.corner_faces=Some(std::mem::take(&mut clip.corner_faces));attributes.generated_faces=Some(std::mem::take(&mut clip.face_weights));attributes.generated_vertices=Some(Vec::new());self.attributes=Some(attributes);self.vertex=0;self.phase=8;}},
            8=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.vertex==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(self.clip.as_ref().unwrap().vertex_sources[origins[self.vertex] as usize]);self.vertex+=1;}},
            9=>{if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)? {self.snapshot=Snapshot::new(mesh);self.clip=None;self.build=None;self.attributes=None;self.phase=10;}},
            10=>{if self.snapshot.advance() {self.edge+=1;self.phase=5;}},
            11=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Ranked {
    rank: f32,
    a: u32,
    b: u32,
}

impl Eq for Ranked {}
impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> Ordering { self.rank.total_cmp(&other.rank).then((self.a, self.b).cmp(&(other.a, other.b))) }
}

struct Clip {
    normal: Vec3,
    distance: f32,
    epsilon: f32,
    intersections: HashMap<(u32, u32), u32>,
    cap: BTreeSet<u32>,
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
    cap_owners:BTreeMap<u32,u32>,
    cap_weights:Vec<u32>,
    source_vertices:usize,
    cursor:usize,
    phase: u8,
}

impl Clip {
    fn new(normal: Vec3, distance: f32, epsilon: f32) -> Self {
        Self { normal, distance, epsilon, intersections: HashMap::new(), cap: BTreeSet::new(), cap_ids: Vec::new(), clipped: Vec::new(), corners: Vec::new(), sorted: BinaryHeap::new(), center: Vec3::ZERO, face: 0, corner: 0,vertex_sources:Vec::new(),source_faces:Vec::new(),corner_faces:Vec::new(),face_weights:Vec::new(),cap_owners:BTreeMap::new(),cap_weights:Vec::new(),source_vertices:0,cursor:0,phase:6 }
    }

    fn advance(&mut self, soup: &mut Soup) -> MeshResult<bool> {
        match self.phase {
            0 => {
                if self.face == soup.faces.len() {
                    if self.cap.len() < 3 { return Err(MeshKernelError::InvalidInput("bevel width does not intersect the solid".into())); }
                    self.phase = 1;
                } else {
                    let face = &soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let da = Vec3(soup.positions[a as usize]).dot(self.normal) - self.distance;
                    let db = Vec3(soup.positions[b as usize]).dot(self.normal) - self.distance;
                    if da <= self.epsilon {
                        if self.corners.last() != Some(&a) { self.corners.push(a); }
                        if da.abs()<=self.epsilon {self.cap.insert(a);self.cap_owners.entry(a).or_insert(self.face as u32);}
                    }
                    if (da > self.epsilon && db < -self.epsilon) || (da < -self.epsilon && db > self.epsilon) {
                        let id = *self.intersections.entry((a.min(b), a.max(b))).or_insert_with(|| {
                            let point = Vec3(soup.positions[a as usize]).lerp(Vec3(soup.positions[b as usize]), da / (da - db));
                            let id = soup.positions.len() as u32;
                            soup.positions.push(point.0);self.vertex_sources.push(SourceVertex::Edge {a,b,t:(da/(da-db)) as f64});
                            id
                        });
                        if self.corners.last() != Some(&id) { self.corners.push(id); }
                        self.cap.insert(id);self.cap_owners.entry(id).or_insert(self.face as u32);
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
                    self.center = self.center.add(Vec3(soup.positions[id as usize]).scale(1.0 / self.cap_ids.len() as f32));
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
                    let point = Vec3(soup.positions[id as usize]).sub(self.center);
                    self.sorted.push(Reverse(Ranked { rank: point.dot(v).atan2(point.dot(u)), a: id, b: 0 }));
                    self.corner += 1;
                }
            }
            4 => {
                if let Some(Reverse(point))=self.sorted.pop() {self.corners.push(point.a);let face=self.cap_owners[&point.a];self.corner_faces.push(face);self.cap_weights.push(face);}
                else {
                    self.source_faces.push(*self.cap_weights.first().ok_or(MeshKernelError::DegenerateOperation)?);self.face_weights.push(std::mem::take(&mut self.cap_weights));self.clipped.push(std::mem::take(&mut self.corners));
                    soup.faces = std::mem::take(&mut self.clipped);
                    return Ok(true);
                }
            }
            6=>{if self.source_vertices==soup.positions.len() {self.phase=0;}else {self.vertex_sources.push(SourceVertex::Original(self.source_vertices as u32));self.source_vertices+=1;}},
            7=>{if self.cursor==self.corners.len() {self.clipped.push(std::mem::take(&mut self.corners));self.phase=0;}else {self.corner_faces.push(*self.source_faces.last().unwrap());self.cursor+=1;}},
            _=>unreachable!(),
        }
        Ok(false)
    }
}

struct Decimate {
    snapshot: Snapshot,
    target: usize,
    closed: bool,
    edges: HashSet<(u32, u32)>,
    ranked: BinaryHeap<Reverse<Ranked>>,
    candidate: Option<Candidate>,
    build: Option<Build>,
    face: usize,
    corner: usize,
    source_faces:Vec<u32>,
    edge_sources:Vec<(u32,u32)>,
    collapse:(u32,u32),
    attributes:Option<SourceAttributeRemap>,
    phase: u8,
}

impl Decimate {
    fn new(source: HalfedgeMesh, target: usize) -> Self {
        Self { snapshot: Snapshot::new(source), target, closed: true, edges: HashSet::new(), ranked: BinaryHeap::new(), candidate: None, build: None, face: 0, corner: 0, source_faces:Vec::new(),edge_sources:Vec::new(),collapse:(0,0),attributes:None,phase: 0 }
    }

    fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1 => "edges", 2 => "candidate", 3 => "reconstruct",4=>"decimate-source-vertices",_=>"decimate-attributes" }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => {
                if self.snapshot.source.vertex_count() <= self.target { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
                if self.corner < self.snapshot.source.halfedges.len() {
                    self.closed &= self.snapshot.source.halfedges[self.corner].twin.is_some();
                    self.corner += 1;
                } else if self.snapshot.advance() { self.phase = 1; self.corner = 0; }
            }
            1 => {
                let soup = &self.snapshot.soup;
                if self.face == soup.faces.len() { self.phase = 2; }
                else {
                    let face = &soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let edge = (a.min(b), a.max(b));
                    if self.edges.insert(edge) {
                        let rank = Vec3(soup.positions[a as usize]).sub(Vec3(soup.positions[b as usize])).length();
                        self.ranked.push(Reverse(Ranked { rank, a: edge.0, b: edge.1 }));
                    }
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            2 => {
                if let Some(candidate) = &mut self.candidate {
                    match candidate.advance(&self.snapshot.soup, self.closed)? {
                        CandidateStep::Working => {},
                        CandidateStep::Rejected => { self.candidate = None; },
                        CandidateStep::Accepted(soup,faces,edges) => {self.collapse=(candidate.a,candidate.b);self.source_faces=faces;self.edge_sources=edges;self.build = Some(Build::new(soup, self.closed)); self.phase = 3; self.candidate = None;},
                    }
                } else if let Some(Reverse(edge)) = self.ranked.pop() { self.candidate = Some(Candidate::new(edge.a, edge.b)); }
                else { return Ok(Some(std::mem::replace(&mut self.snapshot.source, HalfedgeMesh::empty()))); }
            }
            3 => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.vertex_count() >= self.snapshot.source.vertex_count() { self.build = None; self.phase = 2; }
                    else {
                        let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None);attributes.generated_vertices=Some(Vec::new());attributes.edge_source_keys=Some(std::mem::take(&mut self.edge_sources));self.attributes=Some(attributes);self.corner=0;self.phase=4;
                    }
                }
            }
            4=>{let origins=&self.build.as_ref().unwrap().origin_vertices;if self.corner==origins.len() {self.phase=5;}else {let vertex=origins[self.corner];self.attributes.as_mut().unwrap().generated_vertices.as_mut().unwrap().push(if vertex==self.collapse.0 {SourceVertex::Edge{a:self.collapse.0,b:self.collapse.1,t:0.5}}else {SourceVertex::Original(vertex)});self.corner+=1;}},
            5=>{if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)? {self.snapshot=Snapshot::new(mesh);self.edges.clear();self.ranked.clear();self.build=None;self.attributes=None;self.face=0;self.corner=0;self.phase=0;}},
            _=>unreachable!(),
        }
        Ok(None)
    }
}

enum CandidateStep { Working, Rejected, Accepted(Soup,Vec<u32>,Vec<(u32,u32)>) }

struct Candidate {
    a: u32,
    b: u32,
    soup: Soup,
    corners: Vec<u32>,
    corner_sources:Vec<(u32,u32)>,
    edge_sources:Vec<(u32,u32)>,
    face_sources:Vec<u32>,
    seen: HashMap<u32, usize>,
    normal: Normal,
    incidence: BTreeMap<(u32, u32), (usize, i32)>,
    links: HashMap<u32, u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: Option<u32>,
    cursor: u32,
    phase: u8,
}

impl Candidate {
    fn new(a: u32, b: u32) -> Self {
        Self { a, b, soup: Soup::default(), corners: Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),face_sources:Vec::new(), seen: HashMap::new(), normal: Normal::default(), incidence: BTreeMap::new(), links: HashMap::new(), face: 0, corner: 0, vertex: 0, start: None, cursor: 0, phase: 0 }
    }

    fn advance(&mut self, source: &Soup, closed: bool) -> MeshResult<CandidateStep> {
        match self.phase {
            0 => {
                if self.vertex == source.positions.len() { self.phase = 1; }
                else {
                    let point = if self.vertex == self.a as usize { Vec3(source.positions[self.a as usize]).lerp(Vec3(source.positions[self.b as usize]), 0.5).0 } else { source.positions[self.vertex] };
                    self.soup.positions.push(point);
                    self.vertex += 1;
                }
            }
            1 => {
                if self.face == source.faces.len() {
                    if self.soup.faces.len() < 4 { return Ok(CandidateStep::Rejected); }
                    self.phase = 4;
                    self.face = 0;
                } else {
                    let face = &source.faces[self.face];
                    let id = if face[self.corner] == self.b { self.a } else { face[self.corner] };
                    if self.corners.last()!=Some(&id) {self.corners.push(id);self.corner_sources.push((self.face as u32,face[self.corner]));}else {*self.corner_sources.last_mut().unwrap()=(self.face as u32,face[self.corner]);}
                    self.corner += 1;
                    if self.corner == face.len() {
                        if self.corners.first()==self.corners.last() {self.corners.pop();self.corner_sources.pop();}
                        self.corner = 0;
                        if self.corners.len()<3 {self.corners.clear();self.corner_sources.clear();self.face+=1;} else { self.phase = 2; }
                    }
                }
            }
            2 => {
                let id = self.corners[self.corner];
                if self.seen.insert(id, self.face) == Some(self.face) { return Ok(CandidateStep::Rejected); }
                let a = Vec3(self.soup.positions[id as usize]);
                let b = Vec3(self.soup.positions[self.corners[(self.corner + 1) % self.corners.len()] as usize]);
                if self.corner == 0 { self.normal.origin = a; }
                self.normal.add(a, b);
                self.corner += 1;
                if self.corner == self.corners.len() {
                    let after = self.normal.value();
                    if after.length() == 0.0 || source.normals[self.face].dot(after) <= 0.0 { return Ok(CandidateStep::Rejected); }
                    self.soup.faces.push(std::mem::take(&mut self.corners));self.face_sources.push(self.face as u32);self.cursor=0;self.phase=3;return Ok(CandidateStep::Working);
                }
            }
            3=>{if self.cursor as usize==self.corner_sources.len() {self.corner_sources.clear();self.normal=Normal::default();self.corner=0;self.face+=1;self.phase=1;}else {self.edge_sources.push(self.corner_sources[self.cursor as usize]);self.cursor+=1;}},
            4 => {
                if self.face == self.soup.faces.len() { self.phase = 5; }
                else {
                    let face = &self.soup.faces[self.face];
                    let (a, b) = (face[self.corner], face[(self.corner + 1) % face.len()]);
                    let entry = self.incidence.entry((a.min(b), a.max(b))).or_default();
                    entry.0 += 1;
                    entry.1 += if a < b { 1 } else { -1 };
                    if a == self.a {
                        let previous = face[(self.corner + face.len() - 1) % face.len()];
                        if closed && self.links.insert(previous, b).is_some() { return Ok(CandidateStep::Rejected); }
                        self.start = Some(self.start.map_or(previous, |start| start.min(previous)));
                    }
                    self.corner += 1;
                    if self.corner == face.len() { self.corner = 0; self.face += 1; }
                }
            }
            5 => {
                if let Some((_, edge)) = self.incidence.pop_first() {
                    if edge != (2, 0) && (closed || edge.0 != 1) { return Ok(CandidateStep::Rejected); }
                } else if closed {
                    let Some(start) = self.start else { return Ok(CandidateStep::Rejected); };
                    self.cursor = start;
                    self.phase = 6;
                } else { return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup),std::mem::take(&mut self.face_sources),std::mem::take(&mut self.edge_sources))); }
            }
            _ => {
                if let Some(next) = self.links.remove(&self.cursor) {
                    self.cursor = next;
                    if Some(next) == self.start {
                        if !self.links.is_empty() { return Ok(CandidateStep::Rejected); }
                        return Ok(CandidateStep::Accepted(std::mem::take(&mut self.soup),std::mem::take(&mut self.face_sources),std::mem::take(&mut self.edge_sources)));
                    }
                } else { return Ok(CandidateStep::Rejected); }
            }
        }
        Ok(CandidateStep::Working)
    }
}

impl protocol::value::retirement::RetireOwned for TransformSelection {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.vertices,self.edges,self.faces,self.selected]}
}
impl protocol::value::retirement::RetireOwned for Transform {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.mesh,self.flat,self.smooth,self.hes,self.attribute_names,self.selection,self.attribute_values,self.attribute_indices,self.attribute_samples,self.attribute_retired]}
}
impl protocol::value::retirement::RetireOwned for Knife {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.distances,self.tessellation,self.transfer,self.pieces,self.split,self.intersections,self.soup,self.corners,self.build,self.vertex_sources,self.source_faces,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Import {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.polygon,self.primitive_faces,self.midpoints,self.mids,self.positions,self.indices,self.normals,self.unique,self.remap,self.soup,self.corners,self.corner_normals,self.mesh,self.build]}
}
impl protocol::value::retirement::RetireOwned for Inflate {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.input,self.selected,self.normals,self.boundary,self.edges,self.displaced,self.soup,self.borders,self.inner,self.corners,self.build,self.vertex_sources,self.source_faces,self.border_faces,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Orient {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.input,self.snapshot,self.attributes,self.edges,self.keys,self.adjacency,self.oriented,self.flipped,self.stack,self.build]}
}
impl protocol::value::retirement::RetireOwned for FillHoles {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.visited,self.vertices,self.boundary_edges,self.cap,self.cap_faces,self.corner_sources,self.edge_sources,self.face_sources,self.attributes,self.build]}
}
impl protocol::value::retirement::RetireOwned for LoopCut {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.edges,self.incidence,self.marked,self.pending,self.cut_vertices,self.cut_ids,self.soup,self.boundary,self.grid,self.build,self.vertex_sources,self.source_faces,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Mirror {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.source_vertices,self.source_faces,self.mirrored_faces,self.attribute_vertices,self.attributes,self.reflected,self.soup,self.original,self.mirrored,self.build]}
}
impl protocol::value::retirement::RetireOwned for Subdivide {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.input,self.selected,self.concave,self.tessellation,self.transfer,self.soup,self.corners,self.build,self.vertex_sources,self.source_faces,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Coplanar {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.explicit,self.vertex_input,self.selected_vertices,self.star_incident,self.star_edges,self.star_next,self.star_contributors,self.output_vertices,self.source_corners,self.face_sources,self.corner_sources,self.edge_sources,self.spliced_corners,self.spliced_edges,self.output_corners,self.output_edges,self.output_faces,self.attributes,self.attribute_name,self.equality,self.incidence,self.pending,self.seen,self.neighbors,self.removable,self.corners,self.soup,self.build]}
}
impl protocol::value::retirement::RetireOwned for Merge {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.coplanar,self.input,self.selected,self.seen,self.roots,self.remap,self.soup,self.corners,self.unique,self.build,self.grid,self.contributors,self.source_corners,self.corner_origins,self.edge_origins,self.output_corners,self.output_edges,self.output_faces,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for SourceValueEquality {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.tasks]}
}
impl protocol::value::retirement::RetireOwned for SourceAttributeRemap {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.vertex_contributors,self.corner_faces,self.generated_vertices,self.generated_faces,self.corner_sources,self.edge_sources,self.edge_source_keys,self.equality,self.numeric_samples,self.mesh,self.vertices,self.faces,self.mirrored_faces,self.corners,self.name,self.indices,self.normal_values,self.normal_samples]}
}
impl protocol::value::retirement::RetireOwned for Soup {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.positions,self.faces,self.normals]}
}
impl protocol::value::retirement::RetireOwned for Snapshot {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.source,self.soup,self.corners]}
}
impl protocol::value::retirement::RetireOwned for Build {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.origin_vertices,self.soup,self.mesh,self.used,self.remap,self.edges]}
}
impl protocol::value::retirement::RetireOwned for Bevel {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.edges,self.selected,self.planes,self.clip,self.build,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Clip {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.intersections,self.cap,self.cap_ids,self.clipped,self.corners,self.sorted,self.vertex_sources,self.source_faces,self.corner_faces,self.face_weights,self.cap_owners,self.cap_weights]}
}
impl protocol::value::retirement::RetireOwned for Decimate {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.snapshot,self.edges,self.ranked,self.candidate,self.build,self.source_faces,self.edge_sources,self.attributes]}
}
impl protocol::value::retirement::RetireOwned for Candidate {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {protocol::value::artifact_retirement_sequence![self.soup,self.corners,self.corner_sources,self.edge_sources,self.face_sources,self.seen,self.incidence,self.links]}
}
impl protocol::value::retirement::RetireOwned for Work {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {match self {Self::Bevel(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Decimate(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Mirror(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Merge(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Subdivide(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::LoopCut(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Orient(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::FillHoles(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Inflate(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Import(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Knife(value)=>protocol::value::retirement::RetireOwned::retirement(value),Self::Transform(value)=>protocol::value::retirement::RetireOwned::retirement(value)}}
}
protocol::value::artifact_retire_leaf!(SourceVertex,Ranked);
impl protocol::value::retirement::RetireOwned for SourceEqualityTask {
    fn retirement(self)->Box<dyn protocol::value::retirement::RetirementCursor> {let (Self::Node(path)|Self::Sequence(path,..)|Self::Text(path,..))=self;protocol::value::retirement::RetireOwned::retirement(path)}
}
