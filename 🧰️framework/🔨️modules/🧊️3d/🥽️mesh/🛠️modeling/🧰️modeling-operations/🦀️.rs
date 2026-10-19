//! 🧰️ Modeling operations: the public mesh modeling operations trait with its halfedge mesh implementation and the internal operations trait. See [`super`].

use super::*;

pub trait MeshModelingOps: Sized {
    fn affine_transform_job(&self,matrix:[f64;16])->MeshResult<MeshModelingJob>;
    fn affine_transform_job_owned(self,matrix:[f64;16])->MeshResult<MeshModelingJob>;
    fn validate_affine_matrix(matrix:[f64;16])->MeshResult<()>;
    fn merge_coplanar_faces_job(&self)->MeshResult<MeshModelingJob>;
    fn merge_coplanar_faces_job_owned(self)->MeshResult<MeshModelingJob>;
    fn dissolve_edges_job(&self,edges:&[EdgeId])->MeshResult<MeshModelingJob>;
    fn dissolve_edges_job_owned(self,edges:&[EdgeId])->MeshResult<MeshModelingJob>;
    fn dissolve_vertices_job(&self,vertices:&[VertexId])->MeshResult<MeshModelingJob>;
    fn dissolve_vertices_job_owned(self,vertices:&[VertexId])->MeshResult<MeshModelingJob>;
    fn translate_job(&self,delta:Vec3)->MeshResult<MeshModelingJob>;
    fn translate_job_owned(self,delta:Vec3)->MeshResult<MeshModelingJob>;
    fn rotate_job(&self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob>;
    fn rotate_job_owned(self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob>;
    fn scale_job(&self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob>;
    fn scale_job_owned(self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob>;
    fn move_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob>;
    fn move_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob>;
    fn rotate_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob>;
    fn rotate_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob>;
    fn scale_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob>;
    fn scale_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob>;
    fn snap_vertices_job(&self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob>;
    fn snap_vertices_job_owned(self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob>;
    fn move_proportional_job(&self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob>;
    fn move_proportional_job_owned(self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob>;
    fn knife_cut_job(&self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob>;
    fn knife_cut_job_owned(self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob>;
    fn indexed_triangle_job(positions:Vec<f32>,indices:Vec<u32>,normals:Vec<f32>)->MeshResult<MeshModelingJob>;
    fn polygon_source_job(source:semio_framework_mesh_engine::PolygonMeshSource)->MeshResult<MeshModelingJob>;
    fn box_primitive_job(width:f32,height:f32,depth:f32)->MeshResult<MeshModelingJob>;
    fn plane_primitive_job(width:f32,depth:f32)->MeshResult<MeshModelingJob>;
    fn cylinder_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob>;
    fn cone_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob>;
    fn sphere_primitive_job(radius:f32,subdivisions:u32)->MeshResult<MeshModelingJob>;
    fn extrude_faces_job(&self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob>;
    fn extrude_faces_job_owned(self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob>;
    fn inset_faces_job(&self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob>;
    fn inset_faces_job_owned(self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob>;
    fn orient_faces_job(&self)->MeshResult<MeshModelingJob>;
    fn orient_faces_job_owned(self)->MeshResult<MeshModelingJob>;
    fn flip_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob>;
    fn flip_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob>;
    fn fill_holes_job(&self)->MeshResult<MeshModelingJob>;
    fn fill_holes_job_owned(self)->MeshResult<MeshModelingJob>;
    fn loop_cut_job(&self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob>;
    fn loop_cut_job_owned(self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob>;
    fn subdivide_faces_job(&self, faces: &[FaceId])->MeshResult<MeshModelingJob>;
    fn subdivide_faces_job_owned(self, faces: &[FaceId])->MeshResult<MeshModelingJob>;
    fn triangulate_job(&self)->MeshResult<MeshModelingJob>;
    fn triangulate_job_owned(self)->MeshResult<MeshModelingJob>;
    fn delete_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob>;
    fn delete_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob>;
    fn merge_vertices_job(&self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob>;
    fn merge_vertices_job_owned(self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob>;
    fn weld_coincident_vertices_job(&self,precision:f32)->MeshResult<MeshModelingJob>;
    fn weld_coincident_vertices_job_owned(self,precision:f32)->MeshResult<MeshModelingJob>;
    fn mirror_job(&self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob>;
    fn mirror_job_owned(self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob>;
    fn bevel_job(&self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob>;
    fn bevel_job_owned(self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob>;
    fn decimate_job(&self, ratio: f32)->MeshResult<MeshModelingJob>;
    fn decimate_job_owned(self, ratio: f32)->MeshResult<MeshModelingJob>;
    fn flip_faces(&mut self,faces:&[FaceId])->MeshResult<()>;
    fn from_indexed_triangles_by_face_id(positions: &[f32], indices: &[u32], face_ids: &[u32]) -> MeshResult<Self>;
    fn box_prim(width:f32,height:f32,depth:f32)->MeshResult<Self>;
    fn plane_prim(width:f32,depth:f32)->MeshResult<Self>;
    fn cylinder_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self>;
    fn cone_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self>;
    fn ico_sphere_prim(radius:f32,subdivisions:u32)->MeshResult<Self>;
    fn translate(&mut self, delta: Vec3) -> MeshResult<()>;
    fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()>;
    fn scale(&mut self, factor: Vec3) -> MeshResult<()>;
    fn move_vertices_proportional(&mut self,verts:&[VertexId],delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<()>;
    fn snap_vertices_to_grid(&mut self,verts:&[VertexId],grid:f32)->MeshResult<()>;
    fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()>;
    fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()>;
    fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, segments: u32) -> MeshResult<()>;
    fn bevel_edges_with_progress(&mut self, edges: &[EdgeId], amount: f32, segments: u32, progress: impl FnMut(f32) -> bool) -> MeshResult<()>;
    fn loop_cut(&mut self,edges:&[EdgeId],cuts:u32)->MeshResult<()>;
    fn knife_cut(&mut self,face:FaceId,cut_a:Vec3,cut_b:Vec3)->MeshResult<()>;
    fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()>;
    fn dissolve_edges(&mut self,edges:&[EdgeId])->MeshResult<()>;
    fn dissolve_vertices(&mut self,vertices:&[VertexId])->MeshResult<()>;
    fn delete_faces(&mut self,faces:&[FaceId])->MeshResult<()>;
    fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()>;
    fn triangulate(&mut self)->MeshResult<()>;
    fn merge_coplanar_faces(&mut self) -> MeshResult<usize>;
    fn weld_coincident_vertices(&mut self,precision:f32)->MeshResult<usize>;
    fn orient_faces_consistently(&mut self) -> MeshResult<usize>;
    fn fill_holes(&mut self) -> MeshResult<usize>;
    fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()>;
    fn decimate(&mut self, target_ratio: f32) -> MeshResult<()>;
    fn decimate_with_progress(&mut self, target_ratio: f32, progress: impl FnMut(f32) -> bool) -> MeshResult<()>;
}

impl MeshModelingOps for HalfedgeMesh {
    /// 🧭 Retains a column-major affine map, inverse-transpose normals and reflection winding.
    fn affine_transform_job(&self,matrix:[f64;16])->MeshResult<MeshModelingJob> {self.clone().affine_transform_job_owned(matrix)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn affine_transform_job_owned(self,matrix:[f64;16])->MeshResult<MeshModelingJob> {
        let (affine,normal_matrix)=Self::affine_parameters(matrix)?;let determinant=affine.determinant();
        let mut job=self.transform_job_owned([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],[0.0;3],normal_matrix,determinant<0.0,true)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Matrix(affine);Ok(job)
    }
    /// 🛂 Admits constant-sized affine parameters before capturing any mesh source.
    fn validate_affine_matrix(matrix:[f64;16])->MeshResult<()> {Self::affine_parameters(matrix).map(|_|())}
    /// 🧩 Retains adjacency, coplanarity, loop splicing and collinear cleanup in the merge owner.
    fn merge_coplanar_faces_job(&self)->MeshResult<MeshModelingJob> {self.clone().merge_coplanar_faces_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn merge_coplanar_faces_job_owned(self)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,MeshMergeIntent::Coplanar)?)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(24).saturating_add(source_vertices).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// ✂️ Retains explicit edge unions and source-channel provenance in the merge owner.
    fn dissolve_edges_job(&self,edges:&[EdgeId])->MeshResult<MeshModelingJob> {self.clone().dissolve_edges_job_owned(edges)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn dissolve_edges_job_owned(self,edges:&[EdgeId])->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if edges.is_empty() {return Err(MeshKernelError::EmptySelection);}
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,MeshMergeIntent::DissolveEdges(edges.to_vec()))?)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(source_vertices).saturating_add(1),phase:"dissolve-selection"},cancelled:false,retired:false})
    }
    /// 🧵 Retains planar vertex stars and original boundary contributors in the merge owner.
    fn dissolve_vertices_job(&self,vertices:&[VertexId])->MeshResult<MeshModelingJob> {self.clone().dissolve_vertices_job_owned(vertices)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn dissolve_vertices_job_owned(self,vertices:&[VertexId])->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if vertices.is_empty() {return Err(MeshKernelError::EmptySelection);}
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,MeshMergeIntent::DissolveVertices(vertices.to_vec()))?)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(vertices.len().saturating_mul(32)).saturating_add(source_vertices).saturating_add(1),phase:"dissolve-selection"},cancelled:false,retired:false})
    }
    /// ↔️ Retains whole-mesh translation by one vertex per unit.
    fn translate_job(&self,delta:Vec3)->MeshResult<MeshModelingJob> {self.clone().translate_job_owned(delta)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn translate_job_owned(self,delta:Vec3)->MeshResult<MeshModelingJob> {if delta.0.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.transform_job_owned([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],delta.0,[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],false,false)}
    /// 🔄 Retains rotation and each normal contribution in the same modeling owner.
    fn rotate_job(&self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob> {self.clone().rotate_job_owned(axis,angle)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn rotate_job_owned(self,axis:Vec3,angle:f32)->MeshResult<MeshModelingJob> {
        if !angle.is_finite() || axis.0.iter().any(|x|!x.is_finite()) || axis.0==[0.0;3] {return Err(MeshKernelError::DegenerateOperation);}let length=(axis.x() as f64).hypot(axis.y() as f64).hypot(axis.z() as f64);let ax=Vec3(axis.0.map(|x|(x as f64/length) as f32));let (x,y,z)=(ax.x(),ax.y(),ax.z());let (c,s)=(angle.cos(),angle.sin());let t=1.0-c;let matrix=[[t*x*x+c,t*x*y-s*z,t*x*z+s*y],[t*x*y+s*z,t*y*y+c,t*y*z-s*x],[t*x*z-s*y,t*y*z+s*x,t*z*z+c]];self.transform_job_owned(matrix,[0.0;3],matrix.map(|row|row.map(f64::from)),false,true)
    }
    /// 📐 Retains scale, optional winding repair and normal-channel transformation.
    fn scale_job(&self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob> {self.clone().scale_job_owned(factor,reverse_winding)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn scale_job_owned(self,factor:Vec3,reverse_winding:bool)->MeshResult<MeshModelingJob> {if factor.0.iter().any(|x|!x.is_finite() || *x==0.0) {return Err(MeshKernelError::DegenerateOperation);}self.transform_job_owned([[factor.x(),0.0,0.0],[0.0,factor.y(),0.0],[0.0,0.0,factor.z()]],[0.0;3],[[1.0/factor.x() as f64,0.0,0.0],[0.0,1.0/factor.y() as f64,0.0],[0.0,0.0,1.0/factor.z() as f64]],reverse_winding,true)}
    /// 🎯 Retains component expansion, centroid and translation through the existing transform work.
    fn move_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob> {self.clone().move_components_job_owned(vertices,edges,faces,delta)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn move_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,delta:Vec3)->MeshResult<MeshModelingJob> {
        let job=self.translate_job_owned(delta)?;
        Self::select_transform(job,vertices,edges,faces,Some(Vec3::ZERO),[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],false,None)
    }
    /// 🌀 Retains component rotation and splits indexed normal aliases by selected domain.
    fn rotate_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {self.clone().rotate_components_job_owned(vertices,edges,faces,axis,angle,pivot)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn rotate_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,axis:Vec3,angle:f32,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {
        let job=self.rotate_job_owned(axis,angle)?;let length=(axis.x() as f64).hypot(axis.y() as f64).hypot(axis.z() as f64);let [x,y,z]=axis.0.map(|value|value as f64/length);let c=(angle as f64).cos();let t=1.0-c;let sin=(angle as f64).sin();let matrix=[[t*x*x+c,t*x*y-sin*z,t*x*z+sin*y],[t*x*y+sin*z,t*y*y+c,t*y*z-sin*x],[t*x*z-sin*y,t*y*z+sin*x,t*z*z+c]];
        Self::select_transform(job,vertices,edges,faces,pivot,matrix,true,Some(matrix))
    }
    /// 📏 Retains component scale and every authored normal domain contribution.
    fn scale_components_job(&self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {self.clone().scale_components_job_owned(vertices,edges,faces,factor,pivot)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn scale_components_job_owned(self,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,factor:Vec3,pivot:Option<Vec3>)->MeshResult<MeshModelingJob> {
        let job=self.scale_job_owned(factor,false)?;
        Self::select_transform(job,vertices,edges,faces,pivot,[[factor.x() as f64,0.0,0.0],[0.0,factor.y() as f64,0.0],[0.0,0.0,factor.z() as f64]],true,None)
    }
    /// 🧲️ Retains selected grid rounding and geometry-normal recomputation.
    fn snap_vertices_job(&self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob> {self.clone().snap_vertices_job_owned(vertices,grid)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn snap_vertices_job_owned(self,vertices:Vec<VertexId>,grid:f32)->MeshResult<MeshModelingJob> {
        if !grid.is_finite() || grid<=0.0 {return Err(MeshKernelError::InvalidInput("grid must be finite and positive".into()));}let mut job=self.move_components_job_owned(vertices,Vec::new(),Vec::new(),Vec3::ZERO)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Grid(grid);Ok(job)
    }
    /// 🌊 Retains selection and one radial falloff contribution per vertex.
    fn move_proportional_job(&self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob> {self.clone().move_proportional_job_owned(vertices,delta,pivot,radius)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn move_proportional_job_owned(self,vertices:Vec<VertexId>,delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<MeshModelingJob> {
        if !radius.is_finite() || radius<=0.0 || pivot.0.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}let mut job=self.move_components_job_owned(vertices,Vec::new(),Vec::new(),delta)?;let Work::Transform(work)=&mut job.work else {unreachable!()};work.policy=PositionPolicy::Proportional {pivot,radius};Ok(job)
    }
    /// 🔪 Retains projection tests, concave tessellation, clipping and neighboring corners.
    fn knife_cut_job(&self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob> {self.clone().knife_cut_job_owned(face,a,b)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn knife_cut_job_owned(self,face:FaceId,a:Vec3,b:Vec3)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        self.face_boundary(face)?;if a.0.iter().chain(&b.0).any(|x|!x.is_finite()) {return Err(MeshKernelError::InvalidInput("knife points must be finite".into()));}
        Ok(MeshModelingJob {work:Work::Knife(Box::new(Knife {snapshot:Snapshot::new(self),face:face.0 as usize,a:a.0.map(f64::from),b:b.0.map(f64::from),normal:(0.0,0.0,0.0),plane:(0.0,0.0,0.0),origin:(0.0,0.0,0.0),scale:0.0,tolerance:0.0,distances:Vec::new(),positive:false,negative:false,planar:true,convex:true,tessellation:None,tessellation_retirement:None,transfer:None,pieces:Vec::new(),piece:0,piece_positive:false,piece_negative:false,sign:1.0,split:Vec::new(),intersections:HistoryFoldIndex::new(),soup:Soup::default(),corners:Vec::new(),cursor:0,source_face:0,corner:0,emitted_corners:0,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase:0})),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(source_vertices).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// 📥 Retains position welding, corner identities and indexed reconstruction.
    fn indexed_triangle_job(positions:Vec<f32>,indices:Vec<u32>,normals:Vec<f32>)->MeshResult<MeshModelingJob> {let(work,progress)=Import::indexed(positions,indices,normals)?;Ok(MeshModelingJob {work:Work::Import(Box::new(work)),progress,cancelled:false,retired:false})}
    /// 📦 Retains reconstruction of an admitted typed polygon source without cloning its channels.
    fn polygon_source_job(source:semio_framework_mesh_engine::PolygonMeshSource)->MeshResult<MeshModelingJob> {let(work,progress)=Import::polygon(source)?;Ok(MeshModelingJob {work:Work::Import(Box::new(work)),progress,cancelled:false,retired:false})}
    /// 🧊 Retains each box vertex, face and reconstruction unit.
    fn box_primitive_job(width:f32,height:f32,depth:f32)->MeshResult<MeshModelingJob> {Self::primitive_job(MeshPrimitiveKind::Box,[width,height,depth],0)}
    /// 🔳 Retains plane vertices and boundary reconstruction.
    fn plane_primitive_job(width:f32,depth:f32)->MeshResult<MeshModelingJob> {Self::primitive_job(MeshPrimitiveKind::Plane,[width,depth,0.0],0)}
    /// 🥫 Retains cylinder rings, cap corners and side faces.
    fn cylinder_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(MeshPrimitiveKind::Cylinder,[radius,height,0.0],segments)}
    /// 📐 Retains cone ring, apex, cap corners and side faces.
    fn cone_primitive_job(radius:f32,height:f32,segments:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(MeshPrimitiveKind::Cone,[radius,height,0.0],segments)}
    /// 🌐 Retains spherical edge midpoints, subdivision faces and radius application.
    fn sphere_primitive_job(radius:f32,subdivisions:u32)->MeshResult<MeshModelingJob> {Self::primitive_job(MeshPrimitiveKind::IcoSphere,[radius,0.0,0.0],subdivisions)}
    /// 🧱 Retains selected normals, displaced vertices, boundary sides and reconstruction.
    fn extrude_faces_job(&self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob> {self.clone().extrude_faces_job_owned(faces,distance)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn extrude_faces_job_owned(self,faces:&[FaceId],distance:f32)->MeshResult<MeshModelingJob> { self.inflate_job_owned(faces,distance,false) }
    /// 🔲 Retains each inset corner, inner ring and border before reconstruction.
    fn inset_faces_job(&self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob> {self.clone().inset_faces_job_owned(faces,amount)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn inset_faces_job_owned(self,faces:&[FaceId],amount:f32)->MeshResult<MeshModelingJob> { self.inflate_job_owned(faces,amount,true) }
    /// 🧭 Retains incidence, component traversal, polygon reversal and reconstruction.
    fn orient_faces_job(&self)->MeshResult<MeshModelingJob> {self.clone().orient_faces_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn orient_faces_job_owned(self)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work:Work::Orient(Box::new(Orient {input:None,flip_count:0,attributes:None,snapshot:Snapshot::new(self),edges:HistoryFoldIndex::new(),keys:Vec::new(),adjacency:Vec::new(),oriented:Vec::new(),flipped:Vec::new(),stack:Vec::new(),active:None,face:0,corner:0,cursor:0,neighbor:0,reversing:0,incidence:0,build:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul(16).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// 🔄 Retains selected winding changes through existing orientation reconstruction.
    fn flip_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob> {self.clone().flip_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn flip_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob> {if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}let mut job=self.orient_faces_job_owned()?;let Work::Orient(work)=&mut job.work else {unreachable!()};work.input=Some(faces.to_vec());Ok(job)}
    /// 🕳️ Retains boundary rotations and cap corners before reconstruction.
    fn fill_holes_job(&self)->MeshResult<MeshModelingJob> {self.clone().fill_holes_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn fill_holes_job_owned(self)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work:Work::FillHoles(Box::new(FillHoles { snapshot:Snapshot::new(self),visited:HistoryFoldSet::new(),vertices:Vec::new(),boundary_edges:Vec::new(),cap:Vec::new(),cap_faces:Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),face_sources:Vec::new(),attributes:None,cursor:0,start:0,probe:0,rotations:0,corner:0,corners:source_corners,filled:0,build:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul(16).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// ✂️ Retains strip propagation, exact expansion admission and grid construction.
    fn loop_cut_job(&self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob> {self.clone().loop_cut_job_owned(edges,cuts)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn loop_cut_job_owned(self,edges:&[EdgeId],cuts:u32)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if edges.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if !(1..=256).contains(&cuts) { return Err(MeshKernelError::InvalidInput("cuts must be in 1..=256".into())); }
        Ok(MeshModelingJob { work:Work::LoopCut(Box::new(LoopCut { snapshot:Snapshot::new(self),edges:edges.to_vec(),cuts:cuts as usize,incidence:HistoryFoldIndex::new(),marked:HistoryFoldSet::new(),pending:Vec::new(),cut_vertices:HistoryFoldIndex::new(),last_edge:None,cut_edge:None,cut_ids:Vec::new(),soup:Soup::default(),boundary:Vec::new(),grid:Vec::new(),face:0,corner:0,cursor:0,step:0,nx:1,ny:1,vertices:0,faces:0,corners:0,building_grid:true,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase:0 })),progress:MeshModelingProgress { units_done:0,units_total:source_corners.saturating_mul((cuts as usize+1).saturating_mul(32)).saturating_add(1),phase:"snapshot" },cancelled:false,retired:false })
    }
    /// 🕸️ Retains face convexity, concave triangulation, centroid fans and reconstruction.
    fn subdivide_faces_job(&self, faces: &[FaceId])->MeshResult<MeshModelingJob> {self.clone().subdivide_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn subdivide_faces_job_owned(self, faces: &[FaceId])->MeshResult<MeshModelingJob> {
        if faces.is_empty() { return Err(MeshKernelError::EmptySelection); }
        self.subdivision_job_owned(faces,false,false)
    }
    /// 🔺 Retains triangulation and exact source channel mapping in existing subdivision work.
    fn triangulate_job(&self)->MeshResult<MeshModelingJob> {self.clone().triangulate_job_owned()}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn triangulate_job_owned(self)->MeshResult<MeshModelingJob> { self.subdivision_job_owned(&[],true,false) }
    /// 🧹️ Deletes selected faces through the existing retained subdivision reconstruction.
    fn delete_faces_job(&self,faces:&[FaceId])->MeshResult<MeshModelingJob> {self.clone().delete_faces_job_owned(faces)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn delete_faces_job_owned(self,faces:&[FaceId])->MeshResult<MeshModelingJob> {if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}self.subdivision_job_owned(faces,false,true)}
    /// 🧲️ Retains vertex-distance pairs, root walks, polygon remapping and reconstruction.
    fn merge_vertices_job(&self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob> {self.clone().merge_vertices_job_owned(vertices,mode,threshold)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn merge_vertices_job_owned(self, vertices: &[VertexId], mode: WeldMode, threshold: f32)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if vertices.is_empty() { return Err(MeshKernelError::EmptySelection); }
        if mode == WeldMode::ByDistance && (!threshold.is_finite() || threshold < 0.0) { return Err(MeshKernelError::DegenerateOperation); }
        Ok(MeshModelingJob { work: Work::Merge(Box::new(Merge::new(self,MeshMergeIntent::Vertices {vertices:vertices.to_vec(),mode,threshold})?)), progress: MeshModelingProgress { units_done: 0, units_total: vertices.len().saturating_mul(vertices.len()).saturating_mul(4).saturating_add(source_corners.saturating_mul(12)).saturating_add(1), phase: "merge-selection" }, cancelled: false, retired: false })
    }
    /// 🧲️ Retains precision-grid welding and owned source contributors in the merge work.
    fn weld_coincident_vertices_job(&self,precision:f32)->MeshResult<MeshModelingJob> {self.clone().weld_coincident_vertices_job_owned(precision)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn weld_coincident_vertices_job_owned(self,precision:f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if !precision.is_finite() || precision<=0.0 {return Err(MeshKernelError::InvalidInput("weld precision must be positive and finite".into()));}
        Ok(MeshModelingJob {work:Work::Merge(Box::new(Merge::new(self,MeshMergeIntent::Grid {precision})?)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(24).saturating_add(source_vertices.saturating_mul(8)).saturating_add(1),phase:"snapshot"},cancelled:false,retired:false})
    }
    /// 🪞️ Retains one-sided reflection, seam sharing and reconstruction in the modeling owner.
    fn mirror_job(&self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob> {self.clone().mirror_job_owned(axis,threshold)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn mirror_job_owned(self, axis: MirrorAxis, threshold: f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        if !threshold.is_finite() || threshold < 0.0 { return Err(MeshKernelError::DegenerateOperation); }
        let axis = match axis { MirrorAxis::X => 0, MirrorAxis::Y => 1, MirrorAxis::Z => 2 };
        Ok(MeshModelingJob { work: Work::Mirror(Box::new(Mirror { source_vertices:Vec::new(),source_faces:Vec::new(),mirrored_faces:Vec::new(),attribute_vertices:Vec::new(),attributes:None,snapshot: Snapshot::new(self), axis, tolerance: threshold * 0.5, positive: false, negative: false, reflected: Vec::new(), soup: Soup::default(), original: Vec::new(), mirrored: Vec::new(), emitted_corners: 0, plane: true, vertex: 0, face: 0, corner: 0, build: None, phase: 0 })), progress: MeshModelingProgress { units_done: 0, units_total: source_vertices.saturating_mul(4).saturating_add(source_corners.saturating_mul(16)).saturating_add(1), phase: "snapshot" }, cancelled: false, retired: false })
    }
    /// 🪚 Captures a segmented bevel as retained, cancellable geometry work.
    fn bevel_job(&self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob> {self.clone().bevel_job_owned(edges,amount,segments)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn bevel_job_owned(self, edges: &[EdgeId], amount: f32, segments: u32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertices.len();let source_corners=self.halfedges.len();let source_faces=self.faces.len();
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
    fn decimate_job(&self, ratio: f32)->MeshResult<MeshModelingJob> {self.clone().decimate_job_owned(ratio)}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn decimate_job_owned(self, ratio: f32)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedges.len();
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
    /// 🔄 Drains selected winding work through the shared retained orientation owner.
    fn flip_faces(&mut self,faces:&[FaceId])->MeshResult<()> {*self=self.flip_faces_job(faces)?.finish_with_progress(|_|true)?;Ok(())}
    /// 🗃️ Builds a halfedge mesh from a CAD solid tessellation that carries one B-Rep face id per triangle.
    ///
    /// Each B-Rep face is reconstructed independently: coplanar triangles of a simply-connected face merge
    /// into one n-gon; faces with holes keep their triangulation so openings are not filled. Call
    /// [`Self::weld_coincident_vertices`] afterwards so independently-tessellated seam vertices become shared.
    fn from_indexed_triangles_by_face_id(positions: &[f32], indices: &[u32], face_ids: &[u32]) -> MeshResult<Self> {
        if face_ids.is_empty() {
            return Self::from_indexed_triangles(positions, indices);
        }
        if !positions.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("positions length must be a multiple of 3".into()));
        }
        if !indices.len().is_multiple_of(3) {
            return Err(MeshKernelError::InvalidInput("indices length must be a multiple of 3".into()));
        }
        let tri_count = indices.len() / 3;
        if face_ids.len() != tri_count {
            return Err(MeshKernelError::InvalidInput(format!("face_ids length {} must equal triangle count {}", face_ids.len(), tri_count)));
        }
        let mut groups: HashMap<u32, Vec<u32>> = HashMap::new();
        for (triangle_index, &face_id) in face_ids.iter().enumerate() {
            let base = triangle_index * 3;
            let group = groups.entry(face_id).or_default();
            group.extend_from_slice(&indices[base..base + 3]);
        }
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for group_indices in groups.values() {
            let mut face_mesh = Self::from_indexed_triangles(positions, group_indices)?;
            let _ = face_mesh.merge_coplanar_faces()?;
            let (_, group_faces) = face_mesh.polygon_soup();
            faces.extend(group_faces);
        }
        let verts: Vec<[f32; 3]> = positions.as_chunks::<3>().0.to_vec();
        Self::from_faces(&verts, &faces)
    }
    /// 🧊 Drains the canonical retained box constructor.
    fn box_prim(width:f32,height:f32,depth:f32)->MeshResult<Self> {Self::box_primitive_job(width,height,depth)?.finish_with_progress(|_|true)}
    /// 🔳 Drains the canonical retained plane constructor.
    fn plane_prim(width:f32,depth:f32)->MeshResult<Self> {Self::plane_primitive_job(width,depth)?.finish_with_progress(|_|true)}
    /// 🥫 Drains the canonical retained cylinder constructor.
    fn cylinder_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self> {Self::cylinder_primitive_job(radius,height,segments)?.finish_with_progress(|_|true)}
    /// 📐 Drains the canonical retained cone constructor.
    fn cone_prim(radius:f32,height:f32,segments:u32)->MeshResult<Self> {Self::cone_primitive_job(radius,height,segments)?.finish_with_progress(|_|true)}
    /// 🌐 Drains the canonical retained spherical subdivision constructor.
    fn ico_sphere_prim(radius:f32,subdivisions:u32)->MeshResult<Self> {Self::sphere_primitive_job(radius,subdivisions)?.finish_with_progress(|_|true)}
    fn translate(&mut self, delta: Vec3) -> MeshResult<()> {
        let mesh=self.translate_job(delta)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }
    fn rotate(&mut self, axis: Vec3, angle_rad: f32) -> MeshResult<()> {
        let mesh=self.rotate_job(axis,angle_rad)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }
    fn scale(&mut self, factor: Vec3) -> MeshResult<()> {
        let mesh=self.scale_job(factor,false)?.finish_with_progress(|_|true)?;*self=mesh;Ok(())
    }
    /// 🌊 Moves the selection fully and nearby vertices with linear radial falloff.
    fn move_vertices_proportional(&mut self,verts:&[VertexId],delta:Vec3,pivot:Vec3,radius:f32)->MeshResult<()> {
        *self=self.move_proportional_job(verts.to_vec(),delta,pivot,radius)?.finish_with_progress(|_|true)?;Ok(())
    }
    /// 🧲️ Applies selected grid rounding through the retained transform owner.
    fn snap_vertices_to_grid(&mut self,verts:&[VertexId],grid:f32)->MeshResult<()> {
        *self=self.snap_vertices_job(verts.to_vec(),grid)?.finish_with_progress(|_|true)?;Ok(())
    }
    fn extrude_faces(&mut self, faces: &[FaceId], distance: f32) -> MeshResult<()> {
        *self=self.extrude_faces_job(faces,distance)?.finish_with_progress(|_|true)?;Ok(())
    }
    fn inset_faces(&mut self, faces: &[FaceId], amount: f32) -> MeshResult<()> {
        *self=self.inset_faces_job(faces,amount)?.finish_with_progress(|_|true)?;Ok(())
    }
    /// 🪚 Bevels convex closed meshes with a segmented circular profile and mitered corners.
    fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, segments: u32) -> MeshResult<()> {
        self.bevel_edges_with_progress(edges, amount, segments, |_| true)
    }
    /// ⏳ Reports progress and cancels atomically when the callback returns false.
    fn bevel_edges_with_progress(&mut self, edges: &[EdgeId], amount: f32, segments: u32, progress: impl FnMut(f32) -> bool) -> MeshResult<()> {
        *self = self.bevel_job(edges, amount, segments)?.finish_with_progress(progress)?;
        Ok(())
    }
    /// ✂️ Splits connected quad strips, sharing cut vertices across adjacent faces.
    fn loop_cut(&mut self,edges:&[EdgeId],cuts:u32)->MeshResult<()> {
        *self=self.loop_cut_job(edges,cuts)?.finish_with_progress(|_|true)?;Ok(())
    }
    /// ✂️ Splits a face by a projected line and reuses boundary intersections in neighboring polygons.
    fn knife_cut(&mut self,face:FaceId,cut_a:Vec3,cut_b:Vec3)->MeshResult<()> {
        *self=self.knife_cut_job(face,cut_a,cut_b)?.finish_with_progress(|_|true)?;Ok(())
    }
    fn merge_vertices(&mut self, verts: &[VertexId], mode: WeldMode, threshold: f32) -> MeshResult<()> {
        *self = self.merge_vertices_job(verts,mode,threshold)?.finish_with_progress(|_| true)?;
        Ok(())
    }
    /// ✂️ Drains the retained explicit-edge merge without changing the source on refusal.
    fn dissolve_edges(&mut self,edges:&[EdgeId])->MeshResult<()> {*self=self.dissolve_edges_job(edges)?.finish_with_progress(|_|true)?;Ok(())}
    /// 🧵 Drains retained planar-star dissolution, preserving source ownership on refusal.
    fn dissolve_vertices(&mut self,vertices:&[VertexId])->MeshResult<()> {*self=self.dissolve_vertices_job(vertices)?.finish_with_progress(|_|true)?;Ok(())}
    /// 🧹️ Preserves source channels while removing selected face domains.
    fn delete_faces(&mut self,faces:&[FaceId])->MeshResult<()> {*self=self.delete_faces_job(faces)?.finish_with_progress(|_|true)?;Ok(())}
    fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        *self = self.subdivide_faces_job(faces)?.finish_with_progress(|_| true)?;
        Ok(())
    }
    fn triangulate(&mut self)->MeshResult<()> {
        *self=self.triangulate_job()?.finish_with_progress(|_|true)?;Ok(())
    }
    fn merge_coplanar_faces(&mut self) -> MeshResult<usize> {
        let mut job=self.merge_coplanar_faces_job()?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.coplanar_merge_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh coplanar merge cancelled".into()))}}
    }
    /// 🔩️ Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    fn weld_coincident_vertices(&mut self,precision:f32)->MeshResult<usize> {
        let mut job=self.weld_coincident_vertices_job(precision)?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.welded_vertex_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh weld cancelled".into()))}}
    }
    /// 🟦️ Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let mut job=self.orient_faces_job()?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.orientation_flip_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh orientation cancelled".into()))}}
    }
    /// 🪣️ Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
    /// current halfedge topology and caps each with a new n-gon face, so the mesh becomes watertight. Call
    /// `weld_coincident_vertices` first if the mesh may contain importer-duplicated boundary vertices, or
    /// this will also "cap" seams that are actually already shared with a differently-indexed neighbor.
    /// Returns the number of holes filled.
    fn fill_holes(&mut self) -> MeshResult<usize> {
        let mut job=self.fill_holes_job()?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.filled_hole_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh hole filling cancelled".into()))}}
    }
    /// 🪞 Reflects geometry on one side of the axis plane and welds its seam.
    fn mirror(&mut self, axis: MirrorAxis, weld_threshold: f32) -> MeshResult<()> {
        *self = self.mirror_job(axis,weld_threshold)?.finish_with_progress(|_| true)?;
        Ok(())
    }
    fn decimate(&mut self, target_ratio: f32) -> MeshResult<()> {
        self.decimate_with_progress(target_ratio, |_| true)
    }
    /// 🪶 Collapses shortest safe edges, preserving winding and manifold edge incidence.
    fn decimate_with_progress(&mut self, target_ratio: f32, progress: impl FnMut(f32) -> bool) -> MeshResult<()> {
        *self = self.decimate_job(target_ratio)?.finish_with_progress(progress)?;
        Ok(())
    }
}

pub(super) trait MeshModelingInternals: Sized {
    fn affine_parameters(matrix:[f64;16])->MeshResult<(Affine3,[[f64;3];3])>;
    fn select_transform(job:MeshModelingJob,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,pivot:Option<Vec3>,matrix:[[f64;3];3],normals:bool,normal_matrix:Option<[[f64;3];3]>)->MeshResult<MeshModelingJob>;
    fn transform_job_owned(self,matrix:[[f32;3];3],offset:[f32;3],normal_matrix:[[f64;3];3],flip:bool,recompute:bool)->MeshResult<MeshModelingJob>;
    fn transform_owner(self,matrix:[[f32;3];3],offset:[f32;3],normal_matrix:[[f64;3];3],flip:bool,recompute:bool)->MeshModelingJob;
    fn primitive_job(kind:MeshPrimitiveKind,dimensions:[f32;3],parameter:u32)->MeshResult<MeshModelingJob>;
    fn inflate_job_owned(self,faces:&[FaceId],amount:f32,inset:bool)->MeshResult<MeshModelingJob>;
    fn subdivision_job_owned(self,faces:&[FaceId],triangulate_only:bool,delete_selected:bool)->MeshResult<MeshModelingJob>;
}

impl MeshModelingInternals for HalfedgeMesh {
    fn affine_parameters(matrix:[f64;16])->MeshResult<(Affine3,[[f64;3];3])> {
        if matrix.iter().any(|value|!value.is_finite()) {return Err(MeshKernelError::InvalidInput("matrix must contain finite numbers".into()));}
        if [matrix[3],matrix[7],matrix[11],matrix[15]]!=[0.0,0.0,0.0,1.0] {return Err(MeshKernelError::InvalidInput("matrix must be affine with last row [0,0,0,1]".into()));}
        let affine=Affine3 {linear:Mat3::from_rows([[matrix[0],matrix[4],matrix[8]],[matrix[1],matrix[5],matrix[9]],[matrix[2],matrix[6],matrix[10]]]),translation:GeometryVector::from_array([matrix[12],matrix[13],matrix[14]])};let determinant=affine.determinant();
        if !determinant.is_finite() || determinant==0.0 {return Err(MeshKernelError::InvalidInput("matrix must have a finite nonsingular linear part".into()));}
        let normal_matrix=affine.linear.cofactor().scaled(1.0/determinant).rows;
        if normal_matrix.iter().flatten().any(|value|!value.is_finite()) {return Err(MeshKernelError::InvalidInput("matrix inverse-transpose must be finite".into()));}
        Ok((affine,normal_matrix))
    }
    fn select_transform(mut job:MeshModelingJob,vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,pivot:Option<Vec3>,matrix:[[f64;3];3],normals:bool,normal_matrix:Option<[[f64;3];3]>)->MeshResult<MeshModelingJob> {
        if vertices.is_empty() && edges.is_empty() && faces.is_empty() {return Err(MeshKernelError::EmptySelection);}
        if pivot.is_some_and(|pivot|pivot.0.iter().any(|value|!value.is_finite())) {return Err(MeshKernelError::DegenerateOperation);}
        let Work::Transform(work)=&mut job.work else {unreachable!()};work.selection=Some(TransformSelection {vertices,edges,faces,pivot,selected:HistoryFoldSet::new(),sum:[0.0;3],mode:0,cursor:0,next:None,start:0});work.component_matrix=Some(matrix);if let Some(matrix)=normal_matrix {work.normal_matrix=matrix;}work.transform_attributes=normals;work.recompute=true;work.phase=9;job.progress.phase="transform-selection";Ok(job)
    }
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn transform_job_owned(self,matrix:[[f32;3];3],offset:[f32;3],normal_matrix:[[f64;3];3],flip:bool,recompute:bool)->MeshResult<MeshModelingJob> {Ok(self.transform_owner(matrix,offset,normal_matrix,flip,recompute))}
    fn transform_owner(self,matrix:[[f32;3];3],offset:[f32;3],normal_matrix:[[f64;3];3],flip:bool,recompute:bool)->MeshModelingJob {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        MeshModelingJob {work:Work::Transform(Box::new(Transform {mesh:self,policy:PositionPolicy::Affine,matrix,offset,normal_matrix,flip,recompute,flat:Vec::new(),smooth:Vec::new(),hes:Vec::new(),normal:Normal::default(),face_normal:Vec3::ZERO,face:0,vertex:0,cursor:0,start:0,next:0,attribute_names:Vec::new(),attribute:0,selection:None,component_matrix:None,component_pivot:[0.0;3],transform_attributes:recompute,attribute_values:Vec::new(),attribute_indices:Vec::new(),attribute_samples:HistoryFoldIndex::new(),attribute_retired:Vec::new(),attribute_affected:None,attribute_corner:None,phase:0})),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(8).saturating_add(source_vertices.saturating_mul(3)).saturating_add(1),phase:"transform-vertices"},cancelled:false,retired:false}
    }
    fn primitive_job(kind:MeshPrimitiveKind,dimensions:[f32;3],parameter:u32)->MeshResult<MeshModelingJob> {let(work,progress)=Import::primitive(kind,dimensions,parameter)?;Ok(MeshModelingJob {work:Work::Import(Box::new(work)),progress,cancelled:false,retired:false})}
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn inflate_job_owned(self,faces:&[FaceId],amount:f32,inset:bool)->MeshResult<MeshModelingJob> {let source_corners=self.halfedge_count();
        if faces.is_empty() {return Err(MeshKernelError::EmptySelection);}if !amount.is_finite() || amount==0.0 || inset && amount<0.0 {return Err(MeshKernelError::DegenerateOperation);}
        let work=Inflate { snapshot:Snapshot::new(self),input:faces.to_vec(),selected:HistoryFoldSet::new(),amount,inset,normals:HistoryFoldIndex::new(),boundary:HistoryFoldIndex::new(),edges:HistoryFoldSet::new(),displaced:HistoryFoldIndex::new(),last_vertex:None,last_edge:None,soup:Soup::default(),borders:Vec::new(),inner:Vec::new(),corners:Vec::new(),cursor:0,face:0,corner:0,emitted_corners:0,build:None,vertex_sources:Vec::new(),source_faces:Vec::new(),border_faces:Vec::new(),attributes:None,phase:0 };
        let phase=work.phase();Ok(MeshModelingJob {work:Work::Inflate(Box::new(work)),progress:MeshModelingProgress {units_done:0,units_total:source_corners.saturating_mul(32).saturating_add(1),phase},cancelled:false,retired:false})
    }
    /// 📦 Moves the caller-owned source into the same retained modeling work.
    fn subdivision_job_owned(self,faces:&[FaceId],triangulate_only:bool,delete_selected:bool)->MeshResult<MeshModelingJob> {let source_vertices=self.vertex_count();let source_corners=self.halfedge_count();
        Ok(MeshModelingJob { work: Work::Subdivide(Box::new(Subdivide { snapshot: Snapshot::new(self), input: faces.to_vec(),triangulate_only,delete_selected, selected: HistoryFoldSet::new(), concave: HistoryFoldSet::new(), convex: true, tessellation: None,tessellation_retirement:None, transfer: None, soup: Soup::default(), corners: Vec::new(), center: Vec3::ZERO, center_id: 0, emitted_corners: 0, emitting: false, cursor: 0, face: 0, corner: 0, triangle: 0, build: None, vertex_sources:Vec::new(),source_faces:Vec::new(),attributes:None,phase: if triangulate_only {1}else {0} })), progress: MeshModelingProgress { units_done: 0, units_total: source_corners.saturating_mul(32).saturating_add(source_vertices.saturating_mul(2)).saturating_add(1), phase: if triangulate_only {"snapshot"}else {"subdivide-selection"} }, cancelled: false, retired: false })
    }
}
