//! 🛠️ Retained modeling work, measured in vertices, corners, and candidate transitions.

use super::*;
use semio_framework_3d_mesh_reconstruction::{MeshSourceSnapshot as Snapshot,MeshTopologyBuild as Build,MeshPolygonNormal as Normal,MeshSourceRemap as SourceAttributeRemap,MeshMergeJob as Merge,MeshImportJob as Import,MeshMergeIntent,MeshPrimitiveKind};

use semio_framework_3d_mesh_contract::{MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex,MeshRankedEdge as Ranked};
use semio_framework_3d_mesh_clipping::MeshClipJob;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_3d_brep_vector::{Pnt3, Vec3 as GeometryVector};
use semio_framework_3d_brep_vector::matrix::{Affine3, Mat3};
use std::cmp::Reverse;
use std::collections::{BinaryHeap,HashMap};
use semio_framework_mesh_engine::HistoryFoldIndex;

type Vec3f64 = (f64, f64, f64);

#[cfg(feature="mesh-test-observation")]
#[path="🔬️inspection/🦀️.rs"]
pub(super) mod inspection;

#[path = "📥️input/🦀️.rs"]
mod input;
pub use input::{MeshTranslationSource,MeshTranslationInput,MeshPlaneProduction,MeshTranslationProduction};

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

impl Work {
    fn snapshot_frontier(&self)->Option<&Snapshot> {
        match self {
            Self::Bevel(work)if work.phase==0||work.phase==10=>Some(&work.snapshot),
            Self::Decimate(work)if work.phase==0&&work.corner>=work.snapshot.source.halfedges.len()&&work.snapshot.source.vertex_count()>work.target=>Some(&work.snapshot),
            Self::Mirror(work)if work.phase==0=>Some(&work.snapshot),
            Self::Merge(work)=>work.snapshot_frontier(),
            Self::Subdivide(work)if work.phase==1=>Some(&work.snapshot),
            Self::LoopCut(work)if work.phase==0=>Some(&work.snapshot),
            Self::Orient(work)if work.phase==0=>Some(&work.snapshot),
            Self::FillHoles(work)if work.phase==0=>Some(&work.snapshot),
            Self::Inflate(work)if work.phase==0=>Some(&work.snapshot),
            Self::Knife(work)if work.phase==0=>Some(&work.snapshot),
            _=>None,
        }.filter(|owner|!owner.complete())
    }
    fn snapshot_frontier_mut(&mut self)->Option<&mut Snapshot> {
        match self {
            Self::Bevel(work)if work.phase==0||work.phase==10=>Some(&mut work.snapshot),
            Self::Decimate(work)if work.phase==0&&work.corner>=work.snapshot.source.halfedges.len()&&work.snapshot.source.vertex_count()>work.target=>Some(&mut work.snapshot),
            Self::Mirror(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::Merge(work)=>work.snapshot_frontier_mut(),
            Self::Subdivide(work)if work.phase==1=>Some(&mut work.snapshot),
            Self::LoopCut(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::Orient(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::FillHoles(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::Inflate(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::Knife(work)if work.phase==0=>Some(&mut work.snapshot),
            _=>None,
        }.filter(|owner|!owner.complete())
    }
}

impl MeshModelingJob {
    /// 🔃️ Counts completed orientation changes without scanning the face roster.
    pub fn orientation_flip_count(&self)->Option<usize> {match &self.work {Work::Orient(work)=>Some(work.flip_count),_=>None}}
    /// 🕳️ Counts caps produced by the existing retained fill operation.
    pub fn filled_hole_count(&self)->Option<usize> {match &self.work {Work::FillHoles(work)=>Some(work.filled),_=>None}}
    /// 🧩 Counts accepted face unions in the existing merge owner.
    pub fn coplanar_merge_count(&self)->Option<usize> {match &self.work {Work::Merge(work)=>work.coplanar_merge_count(),_=>None}}
    /// 🧲️ Counts precision-grid vertex identifications retained by the merge owner.
    pub fn welded_vertex_count(&self)->Option<usize> {match &self.work {Work::Merge(work)=>work.welded_vertex_count(),_=>None}}
    pub fn progress(&self) -> MeshModelingProgress { self.progress }
    /// 🧵 Transfers imported raw normals and corner identities once after reconstruction.
    pub fn take_imported_corner_normals(&mut self)->Option<(Vec<f32>,Vec<u32>)> { if !self.retired {return None;}match &mut self.work {Work::Import(work)=>work.take_corner_normals(),_=>None} }

    /// 🧹️ Moves the same modeling Work payload into bounded typed retirement.
    pub fn retirement_birth_bytes(&self)->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<Work>()}
    pub fn into_retirement(self,grant:RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(semio_framework_value::ValueError,Self)> {
        let Self {work,progress,cancelled,retired}=self;
        semio_framework_value::retirement::admit_owned_retirement(work,grant).map_err(|(error,work)|(error,Self {work,progress,cancelled,retired}))
    }

    pub fn cancel(&mut self) {
        if !self.retired { self.cancelled = true; }
    }

    pub(super) fn finish_with_progress(mut self, mut progress: impl FnMut(f32) -> bool) -> MeshResult<HalfedgeMesh> {
        loop {
            let value = self.progress();
            if !progress(value.units_done as f32 / value.units_total as f32) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
            match self.step(256,RetainedCloneGrant {maximum_items:256,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut RetainedCloneProgress::default())? {
                MeshModelingStep::Working(_) => {},
                MeshModelingStep::Done(mesh) => {
                    if !progress(1.0) { return Err(MeshKernelError::InvalidInput("operation cancelled".into())); }
                    return Ok(mesh);
                }
                MeshModelingStep::Cancelled(_) => return Err(MeshKernelError::InvalidInput("operation cancelled".into())),
            }
        }
    }

    pub fn step(&mut self, budget: usize,grant:RetainedCloneGrant,tessellation_progress:&mut RetainedCloneProgress) -> MeshResult<MeshModelingStep> {
        *tessellation_progress=Default::default();
        if grant.maximum_items==0{return Ok(MeshModelingStep::Working(self.progress));}
        if self.retired { return Err(MeshKernelError::InvalidInput("modeling job already retired".into())); }
        if self.cancelled { return Ok(MeshModelingStep::Cancelled(self.progress)); }
        if let Some(demand)=self.tessellation_demands(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)? {if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Modeling tessellation frontier exceeds admitted depth")));}}
        for _ in 0..budget.min(grant.maximum_items) {
            if let Some(demand)=self.snapshot_demands().map_err(MeshKernelError::Retained)? {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Modeling snapshot frontier exceeds admitted depth")));}
                let child=RetainedCloneGrant {maximum_depth:grant.maximum_depth-2,..grant};
                self.work.snapshot_frontier_mut().unwrap().advance(child,tessellation_progress).map_err(MeshKernelError::Retained)?;
                if *tessellation_progress!=RetainedCloneProgress::default() {self.progress.units_done=self.progress.units_done.checked_add(1).ok_or_else(||MeshKernelError::InvalidInput("modeling progress overflow".into()))?;self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));}
                return Ok(MeshModelingStep::Working(self.progress));
            }
            let retained_tessellation=matches!(&self.work,Work::Knife(work)if work.phase==4&&(work.tessellation.is_some()||work.tessellation_retirement.is_some()))||matches!(&self.work,Work::Subdivide(work)if work.phase==3&&(work.tessellation.is_some()||work.tessellation_retirement.is_some()));
            let retained_initialization=self.orientation_initialization_demands().map_err(MeshKernelError::Retained)?;
            if let Some(demand)=retained_initialization {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation initialization exceeds admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_incidence=self.orientation_incidence_demands(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?;
            if let Some(demand)=retained_incidence {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation incidence exceeds admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_adjacency=self.orientation_adjacency_demands().map_err(MeshKernelError::Retained)?;
            if let Some(demand)=retained_adjacency {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation adjacency exceeds admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_components=self.orientation_components_demands().map_err(MeshKernelError::Retained)?;
            if let Some(demand)=retained_components {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation components exceed admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_corners=self.orientation_corners_demands();
            if let Some(demand)=retained_corners {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation corners exceed admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_translation=self.translation_demands();
            if let Some(demand)=retained_translation {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original translation exceeds admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
                *tessellation_progress=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
            }
            let next=self.progress.units_done.checked_add(1).ok_or_else(||MeshKernelError::InvalidInput("modeling progress overflow".into()))?;
            let result = match &mut self.work {
                Work::Bevel(work) => work.advance(),
                Work::Decimate(work) => work.advance(),
                Work::Mirror(work) => work.advance(),
                Work::Merge(work) => work.advance(),
                Work::Subdivide(work) => work.advance(grant,tessellation_progress),
                Work::LoopCut(work) => work.advance(),
                Work::Orient(work) => work.advance(grant,tessellation_progress),
                Work::FillHoles(work) => work.advance(),
                Work::Inflate(work) => work.advance(),
                Work::Import(work) => work.advance(),
                Work::Knife(work) => work.advance(grant,tessellation_progress),
                Work::Transform(work) => work.advance(),
            };
            let output = match result {
                Ok(output) => output,
                Err(error) => { self.retired = true; return Err(error); }
            };
            if retained_tessellation&&*tessellation_progress==RetainedCloneProgress::default(){return Ok(MeshModelingStep::Working(self.progress));}
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
                if let Work::Merge(work)=&mut self.work {work.transfer_remaining_assets(&mut mesh)?;}
                self.progress.units_total = self.progress.units_done;
                self.progress.phase = "done";
                self.retired = true;
                return Ok(MeshModelingStep::Done(mesh));
            }
            self.progress.units_total = self.progress.units_total.max(self.progress.units_done.saturating_add(1));
            if retained_tessellation||retained_translation.is_some()||retained_initialization.is_some()||retained_incidence.is_some()||retained_adjacency.is_some()||retained_components.is_some()||retained_corners.is_some(){return Ok(MeshModelingStep::Working(self.progress));}
        }
        Ok(MeshModelingStep::Working(self.progress))
    }

    /// ↔️ Quotes the original translation position atom or metadata transition through its retained parent frames.
    pub fn translation_demands(&self)->Option<semio_framework_value::RetirementDemand> {
        let Work::Transform(work)=&self.work else{return None};
        if work.recompute||work.selection.is_some()||work.component_matrix.is_some()||!matches!(work.policy,PositionPolicy::Affine)||!matches!(work.phase,0|8) {return None;}
        Some(semio_framework_value::RetirementDemand {copy_bytes:if work.phase==0&&work.vertex<work.mesh.vertices.len(){size_of::<[f32;3]>()}else{0},depth:3,..Default::default()})
    }

    /// 🪆️ Quotes the same original tessellation child or retained handback frontier.
    pub fn tessellation_demands(&self,copy:usize)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        let(child,retirement)=match &self.work {Work::Knife(work)if work.phase==4=>(&work.tessellation,&work.tessellation_retirement),Work::Subdivide(work)if work.phase==3=>(&work.tessellation,&work.tessellation_retirement),_=>return Ok(None)};
        let mut demand=if let Some(owner)=retirement {semio_framework_value::RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}}else if let Some(owner)=child {semio_framework_value::RetirementDemand {copy_bytes:owner.next_normal_copy_byte_demand()?,capacity_bytes:owner.next_normal_capacity_byte_demand(copy)?,release_bytes:owner.next_normal_release_byte_demand()?,depth:owner.next_normal_depth_demand()?}}else{return Ok(None)};
        demand.depth=demand.depth.checked_add(2).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Modeling tessellation depth overflow"))?;Ok(Some(demand))
    }

    /// 🧺️ Quotes the same original source projection through its existing parent frames.
    pub fn snapshot_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        let Some(owner)=self.work.snapshot_frontier()else{return Ok(None)};let mut demand=owner.preparation_demands()?;demand.depth=demand.depth.checked_add(2).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Modeling snapshot depth overflow"))?;Ok(Some(demand))
    }

    /// 🧭️ Quotes the same original orientation initialization backing or face row without touching its source.
    pub fn orientation_initialization_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {Work::Orient(work)if work.phase==1=>work.initialization_demands().map(Some),_=>Ok(None)}
    }
    /// 🗝️ Quotes the same original incidence page, encounter key or edge owner without collecting a directory.
    pub fn orientation_incidence_demands(&self,copy:usize)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {Work::Orient(work)if work.phase==2=>work.incidence_demands(copy).map(Some),_=>Ok(None)}
    }
    /// 🕸️ Quotes one original reciprocal neighbor backing or tuple through its actual index depth.
    pub fn orientation_adjacency_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {Work::Orient(work)if work.phase==3=>work.adjacency_demands().map(Some),_=>Ok(None)}
    }
    /// 🔁️ Quotes the original two-corner swap or free source-header/cursor move.
    pub fn orientation_corners_demands(&self)->Option<semio_framework_value::RetirementDemand> {
        match &self.work {Work::Orient(work)if work.phase==5=>Some(work.corners_demands()),_=>None}
    }
    /// 🌐️ Quotes the original stack birth, visit writes or free component cursor move.
    pub fn orientation_components_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {Work::Orient(work)if work.phase==4=>work.components_demands().map(Some),_=>Ok(None)}
    }
}

fn close_tessellation_handoff(owner:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<bool> {
    let Some(current)=owner.as_mut()else{return Ok(true)};let result=current.step(grant);*receipt=current.step_progress();result.map_err(MeshKernelError::Retained)?;
    if current.terminal_is_empty(){owner.take();Ok(true)}else{Ok(false)}
}

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
    /// Builds a halfedge mesh from a CAD solid tessellation that carries one B-Rep face id per triangle.
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
    /// Unifies every group of vertices at (nearly) the same position — as commonly produced by importers
    /// that tessellate adjacent source faces independently, leaving duplicate, non-shared vertex ids along
    /// shared boundaries — into a single vertex id per position, so the halfedge topology (twins, boundary
    /// detection) reflects the true geometric connectivity. Returns the number of vertices removed.
    fn weld_coincident_vertices(&mut self,precision:f32)->MeshResult<usize> {
        let mut job=self.weld_coincident_vertices_job(precision)?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.welded_vertex_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh weld cancelled".into()))}}
    }
    /// Flips faces so every undirected edge is traversed in opposite directions by its two incident faces.
    /// CAD imports often leave inconsistently oriented face wires; without this pass, halfedge twins are
    /// missing even though the undirected mesh is closed. Returns the number of faces flipped.
    fn orient_faces_consistently(&mut self) -> MeshResult<usize> {
        let mut job=self.orient_faces_job()?;loop {match job.step(256,semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())? {MeshModelingStep::Done(mesh)=>{let count=job.orientation_flip_count().unwrap();*self=mesh;return Ok(count);},MeshModelingStep::Working(_)=>{},MeshModelingStep::Cancelled(_)=>return Err(MeshKernelError::InvalidInput("mesh orientation cancelled".into()))}}
    }
    /// Finds every closed boundary loop (a chain of edges with no opposite face on the other side) in the
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
trait MeshModelingInternals: Sized {
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

struct TransformSelection {
    vertices:Vec<VertexId>,edges:Vec<EdgeId>,faces:Vec<FaceId>,pivot:Option<Vec3>,selected:HistoryFoldSet<usize>,sum:[f64;3],mode:u8,cursor:usize,next:Option<u32>,start:u32,
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
    attribute_values:Vec<semio_framework_value::DslValue>,
    attribute_indices:Vec<u32>,
    attribute_samples:HistoryFoldIndex<(u32,bool),u32>,
    attribute_retired:Vec<semio_framework_value::DslValue>,
    attribute_affected:Option<bool>,
    attribute_corner:Option<u32>,
    phase:u8,
}
impl Transform {
    fn directional_value(value:&semio_framework_value::DslValue,matrix:[[f64;3];3],tangent:bool,reflected:bool)->MeshResult<semio_framework_value::DslValue> {
        let values=value.as_array().ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction".into()))?;
        if values.len()!=if tangent {4}else {3} {return Err(MeshKernelError::InvalidInput("invalid authored direction dimensions".into()));}
        let coordinate=|axis:usize|values[axis].as_f64().filter(|value|value.is_finite()).ok_or_else(||MeshKernelError::InvalidInput("invalid authored direction coordinate".into()));let point=[coordinate(0)?,coordinate(1)?,coordinate(2)?];
        let direction=[0,1,2].map(|axis|matrix[axis][0]*point[0]+matrix[axis][1]*point[1]+matrix[axis][2]*point[2]);let length=direction[0].hypot(direction[1]).hypot(direction[2]);let direction=direction.map(|value|value/length);
        if direction.iter().any(|value|!value.is_finite() || !(*value as f32).is_finite()) {return Err(MeshKernelError::DegenerateOperation);}
        let mut output=direction.into_iter().map(semio_framework_value::DslValue::float).collect::<Vec<_>>();
        if tangent {let handedness=values[3].as_f64().filter(|value|*value==1.0 || *value== -1.0).ok_or_else(||MeshKernelError::InvalidInput("invalid authored tangent handedness".into()))?;output.push(semio_framework_value::DslValue::float(if reflected {-handedness}else {handedness}));}
        Ok(semio_framework_value::DslValue::Array(output))
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
            4=>{let n=self.hes.len();let point=|i:usize|Vec3(self.mesh.vertices[self.mesh.halfedges[self.halfedge(i%n) as usize].vertex as usize].position);let (a,b)=(point(self.cursor),point(self.cursor+1));if self.cursor==0 {self.normal.set_origin(a);}self.normal.add(a,b);self.cursor+=1;if self.cursor==n {self.face_normal=self.normal.value();self.cursor=0;self.phase=5;}}
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
    tessellation_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,
    transfer:Option<MeshTransfer>,
    pieces:Vec<Vec<u32>>,
    piece:usize,
    piece_positive:bool,
    piece_negative:bool,
    sign:f64,
    split:Vec<Vec<u32>>,
    intersections:HistoryFoldIndex<(u32,u32),u32>,
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
    fn advance(&mut self,grant:RetainedCloneGrant,tessellation_progress:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>> {
        if self.tessellation_retirement.is_some(){if close_tessellation_handoff(&mut self.tessellation_retirement,grant,tessellation_progress)?{self.phase=5;}return Ok(None);}
        match self.phase {
            0=>{if self.snapshot.complete() {self.normal=Self::tuple(self.snapshot.soup.normals[self.face].0);let direction=sub3((self.b[0],self.b[1],self.b[2]),(self.a[0],self.a[1],self.a[2]));let plane=cross3(direction,self.normal);let length=length3(plane);if length==0.0 || length<=length3(direction)*1e-7 {return Err(MeshKernelError::InvalidInput("knife direction must project onto a nondegenerate face".into()));}self.plane=(plane.0/length,plane.1/length,plane.2/length);self.origin=Self::tuple(self.snapshot.soup.positions[self.snapshot.soup.faces[self.face][0] as usize]);self.cursor=0;self.phase=10;}}
            1=>{let face=&self.snapshot.soup.faces[self.face];let point=Self::tuple(self.snapshot.soup.positions[face[self.cursor] as usize]);self.scale=self.scale.max(length3(sub3(point,self.origin)));self.cursor+=1;if self.cursor==face.len() {self.tolerance=self.scale*1e-7;self.cursor=0;self.phase=2;}}
            2=>{
                if self.cursor==self.snapshot.soup.positions.len() {self.phase=3;self.cursor=0;}
                else {let d=dot3(sub3(Self::tuple(self.snapshot.soup.positions[self.cursor]),(self.a[0],self.a[1],self.a[2])),self.plane);self.distances.push(if d.abs()<=self.tolerance {0.0}else {d});self.cursor+=1;}
            }
            3=>{
                let face=&self.snapshot.soup.faces[self.face];let n=face.len();let point=|i:usize|Self::tuple(self.snapshot.soup.positions[face[i%n] as usize]);let distance=self.distances[face[self.cursor] as usize];self.positive|=distance>0.0;self.negative|=distance<0.0;self.planar&=dot3(sub3(point(self.cursor),self.origin),self.normal).abs()<=self.tolerance;self.convex&=dot3(cross3(sub3(point(self.cursor+1),point(self.cursor)),sub3(point(self.cursor+2),point(self.cursor+1))),self.normal)>=-self.scale*self.tolerance;self.cursor+=1;
                if self.cursor==n {if !self.positive || !self.negative {return Err(MeshKernelError::InvalidInput("knife line must cross the face interior".into()));}if !self.planar || !self.convex {self.tessellation=Some(MeshTessellationJob::selected(std::mem::replace(&mut self.snapshot.source,HalfedgeMesh::empty()),HistoryFoldSet::from([self.face as u32])));}self.cursor=0;self.phase=4;}
            }
            4=>{if let Some(job)=&mut self.tessellation {let result=job.step(grant);*tessellation_progress=job.normal_step_progress();if let MeshTessellationStep::Done(transfer)=result.map_err(MeshKernelError::Retained)?.0 {self.transfer=Some(transfer);let mut original=self.tessellation.take().unwrap();self.snapshot.source=original.take_source();self.tessellation_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original tessellation handback requires typed retirement")));}}else {self.phase=5;}}
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
            9=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=11;}},
            10=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            11=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=12;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            12=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Inflate {
    snapshot:Snapshot,
    input:Vec<FaceId>,
    selected:HistoryFoldSet<u32>,
    amount:f32,
    inset:bool,
    normals:HistoryFoldIndex<u32,Vec3>,
    boundary:HistoryFoldIndex<(u32,u32),(u32,u32,usize,u32)>,
    edges:HistoryFoldSet<(u32,u32)>,
    displaced:HistoryFoldIndex<u32,u32>,
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
            1=>{if self.snapshot.complete() {self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.cursor=0;self.phase=7;}}
            2=>{
                if self.cursor==self.input.len() {
                    if self.inset {self.phase=4;self.face=0;self.corner=0;}else {if self.snapshot.soup.faces.len().saturating_add(self.edges.len())>100_000 || self.snapshot.source.halfedge_count().saturating_add(self.edges.len().saturating_mul(4))>600_000 {return Err(MeshKernelError::InvalidInput("extrusion exceeds mesh capacity".into()));}self.phase=3;}
                }else {
                    let id=self.input[self.cursor].0 as usize;let face=&self.snapshot.soup.faces[id];let n=face.len();let normal=self.snapshot.soup.normals[id];
                    if self.inset {
                        self.admit_position()?;let face=&self.snapshot.soup.faces[id];let point=|i:usize|Vec3(self.soup.positions[face[i] as usize]);let previous=point((self.corner+n-1)%n);let current=point(self.corner);let next=point((self.corner+1)%n);let a=normal.cross(current.sub(previous).normalize());let b=normal.cross(next.sub(current).normalize());let denominator=1.0+a.dot(b);if denominator<=1e-6 {return Err(MeshKernelError::DegenerateOperation);}let offset=a.add(b).scale(self.amount/denominator);if offset.length()>=current.sub(previous).length().min(next.sub(current).length())*0.5 {return Err(MeshKernelError::InvalidInput("inset exceeds local edge clearance".into()));}let position=current.add(offset).0;if position.iter().any(|x|!x.is_finite()) {return Err(MeshKernelError::DegenerateOperation);}self.inner.push(self.soup.positions.len() as u32);self.soup.positions.push(position);self.vertex_sources.push(SourceVertex::Original(face[self.corner]));
                    }else {let (a,b)=(face[self.corner],face[(self.corner+1)%n]);self.normals.entry(a).and_modify(|sum|*sum=sum.add(normal)).or_insert_with(||normal);let edge=self.boundary.entry((a.min(b),a.max(b))).or_insert_with(||(a,b,0,id as u32));if edge.2==0 {self.edges.insert((a,b));}else {self.edges.remove(&(edge.0,edge.1));}edge.2+=1;}
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
            6=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=8;}},
            7=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            8=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize]);self.cursor+=1;}},
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
    edges:HistoryFoldIndex<(u32,u32),Vec<(usize,bool)>>,
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
    incidence:u8,
    build:Option<Build>,
    phase:u8,
}
impl Orient {
    fn phase(&self)->&'static str { ["snapshot","orient-initialize","orient-incidence","orient-adjacency","orient-components","orient-corners","orient-reconstruct","orient-attributes","orient-selection"][self.phase as usize] }
    fn initialization_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        let count=self.snapshot.soup.faces.len();
        let (slots,bytes)=if self.adjacency.capacity()<count {(count,size_of::<Vec<(usize,bool)>>())}else if self.oriented.capacity()<count||self.flipped.capacity()<count {(count,size_of::<bool>())}else {(0,0)};
        let capacity=slots.checked_mul(bytes).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original orientation initialization backing extent overflow"))?;
        let copy=if slots!=0||self.cursor==count {0}else {size_of::<Vec<(usize,bool)>>()+2*size_of::<bool>()};
        Ok(semio_framework_value::RetirementDemand {copy_bytes:copy,capacity_bytes:capacity,depth:3,..Default::default()})
    }
    fn incidence_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if self.face==self.snapshot.soup.faces.len() {return Ok(RetirementDemand {depth:3,..Default::default()});}
        let face=&self.snapshot.soup.faces[self.face];let(a,b)=(face[self.corner],face[(self.corner+1)%face.len()]);let key=(a.min(b),a.max(b));
        let(copy_bytes,capacity_bytes,child_depth)=match self.incidence {
            0=>{let membership=self.edges.next_contains_depth_demand(&key)?;if self.edges.contains_key(&key) {(0,0,membership.max(1))}else {(0,self.edges.next_insert_capacity_byte_demand(&key,copy)?,membership.max(self.edges.next_insert_depth_demand(&key)?))}},
            1=>{if self.keys.capacity()==0 {(0,self.snapshot.source.halfedges.len().checked_mul(size_of::<(u32,u32)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original incidence key backing extent overflow"))?,1)}else {(size_of::<(u32,u32)>(),0,1)}},
            2=>{let owners=self.edges.get(&key).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original incidence key lost its admitted row"))?;if owners.len()>=2 {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original incidence edge has more than two manifold owners"));}let capacity=if owners.capacity()==0 {2*size_of::<(usize,bool)>()}else {0};(if capacity!=0 {0}else {size_of::<(usize,bool)>()},capacity,self.edges.next_contains_depth_demand(&key)?.max(1))},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original incidence substage is invalid")),
        };
        let depth=child_depth.checked_add(2).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original incidence parent depth overflow"))?;
        Ok(RetirementDemand {copy_bytes,capacity_bytes,depth,..Default::default()})
    }
    fn adjacency_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if self.cursor==self.keys.len() {return Ok(RetirementDemand {depth:3,..Default::default()});}
        let key=self.keys[self.cursor];let owners=self.edges.get(&key).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency lost its edge owner row"))?;
        let depth=self.edges.next_contains_depth_demand(&key)?.max(1).checked_add(2).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original adjacency parent depth overflow"))?;
        if owners.len()!=2 {return Ok(RetirementDemand {depth,..Default::default()});}
        let face=owners.get(self.incidence as usize).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency reciprocal substage is invalid"))?.0;
        let row=&self.adjacency[face];
        let capacity_bytes=if row.capacity()==0 {self.snapshot.soup.faces[face].len().checked_mul(size_of::<(usize,bool)>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original adjacency backing extent overflow"))?}else {0};
        if capacity_bytes==0&&row.len()==row.capacity() {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original adjacency exceeds its face corner backing"));}
        Ok(RetirementDemand {copy_bytes:if capacity_bytes==0 {size_of::<(usize,bool)>()}else {0},capacity_bytes,depth,..Default::default()})
    }
    fn components_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        let copy_bytes=if let Some(face)=self.active {self.adjacency[face].get(self.neighbor).filter(|(neighbor,_)|!self.oriented[*neighbor]).map_or(0,|_|size_of::<usize>()+2*size_of::<bool>())}else if self.stack.is_empty()&&self.cursor<self.oriented.len()&&!self.oriented[self.cursor] {size_of::<usize>()+size_of::<bool>()}else {0};
        let capacity_bytes=if copy_bytes!=0&&self.stack.capacity()==0 {self.oriented.len().checked_mul(size_of::<usize>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original component stack backing extent overflow"))?}else {0};
        if copy_bytes!=0&&capacity_bytes==0&&self.stack.len()==self.stack.capacity() {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original component stack exceeds its face backing"));}
        Ok(RetirementDemand {copy_bytes:if capacity_bytes!=0 {0}else {copy_bytes},capacity_bytes,depth:3,..Default::default()})
    }
    fn corners_demands(&self)->semio_framework_value::RetirementDemand {
        let swapping=self.snapshot.soup.faces.get(self.face).is_some_and(|face|self.flipped[self.face]&&self.reversing<face.len()/2);
        semio_framework_value::RetirementDemand {copy_bytes:if swapping {2*size_of::<u32>()}else {0},depth:3,..Default::default()}
    }
    fn advance(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0=>{if self.snapshot.complete() {self.phase=1;}}
            1=>{
                let count=self.snapshot.soup.faces.len();let demand=self.initialization_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if demand.capacity_bytes!=0 {
                    let result=if self.adjacency.capacity()<count {self.adjacency.try_reserve_exact(count).map(|()|self.adjacency.capacity()*size_of::<Vec<(usize,bool)>>())}else if self.oriented.capacity()<count {self.oriented.try_reserve_exact(count).map(|()|self.oriented.capacity()*size_of::<bool>())}else {self.flipped.try_reserve_exact(count).map(|()|self.flipped.capacity()*size_of::<bool>())};
                    let bytes=result.map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original orientation initialization backing reservation failed")))?;
                    *receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..Default::default()};
                    if bytes>grant.maximum_capacity_bytes {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original orientation allocator exceeded admitted backing").with_retained_progress(*receipt)));}
                }else {
                    if self.cursor==count {self.phase=if self.input.is_some() {8}else {2};self.cursor=0;}else {self.adjacency.push(Vec::new());self.oriented.push(false);self.flipped.push(false);self.cursor+=1;}
                    *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
                }
            }
            2=>{
                let demand=self.incidence_demands(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.face==self.snapshot.soup.faces.len() {self.phase=3;self.face=0;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                else {
                    let face=&self.snapshot.soup.faces[self.face];let(a,b,count)=(face[self.corner],face[(self.corner+1)%face.len()],face.len());let key=(a.min(b),a.max(b));
                    match self.incidence {
                        0=>{
                            let child=RetainedCloneGrant {maximum_depth:grant.maximum_depth-2,..grant};
                            if self.edges.contains_key(&key) {self.incidence=2;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                            else if demand.capacity_bytes!=0 {match self.edges.reserve_insert_step(&key,child) {Ok(progress)=>*receipt=progress,Err((error,progress))=>{*receipt=progress;return Err(MeshKernelError::Retained(error.with_retained_progress(progress)));}}}
                            else {let(previous,progress)=self.edges.insert_reserved(key,Vec::new(),child).map_err(|(error,_,_)|MeshKernelError::Retained(error))?;assert!(previous.is_none(),"original incidence vacancy changed during its admitted turn");*receipt=progress;self.incidence=1;}
                        }
                        1=>{
                            if demand.capacity_bytes!=0 {self.keys.try_reserve_exact(self.snapshot.source.halfedges.len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original incidence key backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:self.keys.capacity()*size_of::<(u32,u32)>(),..Default::default()};}
                            else {if self.keys.len()==self.keys.capacity() {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original incidence encounter key exceeds admitted backing")));}self.keys.push(key);self.incidence=2;*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        }
                        2=>{
                            let owners=self.edges.get_mut(&key).unwrap();
                            if demand.capacity_bytes!=0 {owners.try_reserve_exact(2).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original incidence edge owner backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:owners.capacity()*size_of::<(usize,bool)>(),..Default::default()};}
                            else {if owners.len()==owners.capacity() {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original incidence owner row exceeds admitted backing")));}owners.push((self.face,a<b));self.incidence=0;self.corner+=1;if self.corner==count {self.face+=1;self.corner=0;}*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        }
                        _=>unreachable!(),
                    }
                    if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original incidence allocator exceeded its admitted backing").with_retained_progress(*receipt)));}
                }
            }
            3=>{
                let demand=self.adjacency_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.cursor==self.keys.len() {self.phase=4;self.cursor=0;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                else {
                    let owners=&self.edges[&self.keys[self.cursor]];
                    if owners.len()!=2 {self.cursor+=1;*receipt=RetainedCloneProgress {copied_items:1,..Default::default()};}
                    else {
                        let(face,_)=owners[self.incidence as usize];let(neighbor,_)=owners[1-self.incidence as usize];let relative=owners[0].1==owners[1].1;
                        let row=&mut self.adjacency[face];
                        if demand.capacity_bytes!=0 {row.try_reserve_exact(self.snapshot.soup.faces[face].len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original adjacency backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:row.capacity()*size_of::<(usize,bool)>(),..Default::default()};}
                        else {row.push((neighbor,relative));if self.incidence==0 {self.incidence=1;}else {self.incidence=0;self.cursor+=1;}*receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};}
                        if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original adjacency allocator exceeded its admitted backing").with_retained_progress(*receipt)));}
                    }
                }
            }
            4=>{
                let demand=self.components_demands().map_err(MeshKernelError::Retained)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if demand.capacity_bytes!=0 {self.stack.try_reserve_exact(self.oriented.len()).map_err(|_|MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original component stack backing reservation failed")))?;*receipt=RetainedCloneProgress {copied_items:1,retained_capacity_bytes:self.stack.capacity()*size_of::<usize>(),..Default::default()};if !receipt.fits(grant) {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original component allocator exceeded its admitted backing").with_retained_progress(*receipt)));}return Ok(None);}
                if let Some(face)=self.active {
                    if self.neighbor==self.adjacency[face].len() {self.active=None;}
                    else {let (neighbor,relative)=self.adjacency[face][self.neighbor];if !self.oriented[neighbor] {self.oriented[neighbor]=true;self.flipped[neighbor]=self.flipped[face]^relative;self.flip_count+=usize::from(self.flipped[neighbor]);self.stack.push(neighbor);}self.neighbor+=1;}
                }else if let Some(face)=self.stack.pop() {self.active=Some(face);self.neighbor=0;}
                else if self.cursor==self.oriented.len() {self.phase=5;self.face=0;}
                else {if !self.oriented[self.cursor] {self.oriented[self.cursor]=true;self.stack.push(self.cursor);}self.cursor+=1;}
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
            }
            5=>{
                let demand=self.corners_demands();
                if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth {return Ok(None);}
                if self.face==self.snapshot.soup.faces.len() {self.build=Some(Build::unreferenced(std::mem::take(&mut self.snapshot.soup)));self.phase=6;}
                else {let face=&mut self.snapshot.soup.faces[self.face];if self.flipped[self.face] && self.reversing<face.len()/2 {let opposite=face.len()-1-self.reversing;face.swap(self.reversing,opposite);self.reversing+=1;}else {self.face+=1;self.reversing=0;}}
                *receipt=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
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
    visited:HistoryFoldSet<u32>,
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
            0=>{if self.snapshot.complete() {self.cursor=0;self.phase=5;}}
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
            4=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let mut attributes=SourceAttributeRemap::new(mesh,None,None,None,None);attributes.configuration_mut()?.corner_sources=Some(std::mem::take(&mut self.corner_sources));attributes.configuration_mut()?.edge_sources=Some(std::mem::take(&mut self.edge_sources));attributes.configuration_mut()?.generated_faces=Some(std::mem::take(&mut self.face_sources));self.attributes=Some(attributes);self.phase=7;}},
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
    incidence:HistoryFoldIndex<(u32,u32),Vec<(usize,usize)>>,
    marked:HistoryFoldSet<(u32,u32)>,
    pending:Vec<(u32,u32)>,
    cut_vertices:HistoryFoldIndex<(u32,u32),Vec<u32>>,
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
            0=>{if self.snapshot.complete() {self.cursor=0;self.phase=8;}}
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
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=9;}},
            8=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=1;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            9=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=10;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
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
    selected: HistoryFoldSet<u32>,
    concave: HistoryFoldSet<u32>,
    convex: bool,
    tessellation: Option<MeshTessellationJob>,
    tessellation_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,
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
    fn advance(&mut self,grant:RetainedCloneGrant,tessellation_progress:&mut RetainedCloneProgress) -> MeshResult<Option<HalfedgeMesh>> {
        if self.tessellation_retirement.is_some(){if close_tessellation_handoff(&mut self.tessellation_retirement,grant,tessellation_progress)?{self.soup.positions=std::mem::take(&mut self.snapshot.soup.positions);self.phase=4;}return Ok(None);}
        match self.phase {
            0 => {
                if self.cursor==self.input.len() {if self.delete_selected && self.selected.len()==self.snapshot.source.face_count() {return Err(MeshKernelError::InvalidInput("deletion would leave an empty mesh".into()));}self.phase = 1;}
                else { let face = self.input[self.cursor]; self.snapshot.source.face_boundary(face)?; if !self.selected.insert(face.0) { return Err(MeshKernelError::InvalidInput("duplicate face selection".into())); } self.cursor += 1; }
            }
            1 => {if self.snapshot.complete() {self.cursor=0;self.phase=6;}}
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
                    let result=tessellation.step(grant);*tessellation_progress=tessellation.normal_step_progress();if let MeshTessellationStep::Done(transfer) = result.map_err(MeshKernelError::Retained)?.0 {self.transfer=Some(transfer);let mut original=self.tessellation.take().unwrap();self.snapshot.source=original.take_source();self.tessellation_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original tessellation handback requires typed retirement")));}
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
            5=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {self.attributes=Some(SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None));self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices=Some(Vec::new());self.cursor=0;self.phase=7;}},
            6=>{if self.cursor==self.snapshot.source.vertex_count() {self.cursor=0;self.phase=2;}else {self.vertex_sources.push(SourceVertex::Original(self.cursor as u32));self.cursor+=1;}},
            7=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.cursor==origins.len() {self.phase=8;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.vertex_sources[origins[self.cursor] as usize].clone());self.cursor+=1;}},
            8=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _ => unreachable!(),
        }
        Ok(None)
    }
}

impl Mirror {
    fn phase(&self) -> &'static str { ["snapshot","mirror-side","mirror-vertices","mirror-corners","mirror-reconstruct","mirror-vertex-provenance","mirror-attributes"][self.phase as usize] }
    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.complete() { self.phase = 1; } }
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
            5=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.vertex==origins.len() {self.attributes.as_mut().unwrap().configuration_mut()?.vertices=Some(std::mem::take(&mut self.attribute_vertices));self.phase=6;}else {self.attribute_vertices.push(self.source_vertices[origins[self.vertex] as usize]);self.vertex+=1;}},
            6=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
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
    selected: HistoryFoldSet<u32>,
    planes: Vec<(Vec3, f32)>,
    profile: Option<Profile>,
    clip: Option<MeshClipJob>,
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
        Self {retain_channels:!source.attributes.is_empty() || !source.uv_seams.is_empty(),attributes:None,snapshot: Snapshot::new(source), edges, amount, segments, extent: 0.0, epsilon: 0.0, face: 0, vertex: 0, edge: 0, selected: HistoryFoldSet::new(), planes: Vec::new(), profile: None, clip: None, build: None, phase: 0 }
    }

    fn phase(&self) -> &'static str {
        match self.phase { 0 => "snapshot", 1..=3 => "validate", 4 => "profile", 5 => "clip", 6=>"reconstruct",7=>"bevel-clip-reconstruct",8=>"bevel-source-vertices",9=>"bevel-clip-attributes",10=>"snapshot",_=>"bevel-final-attributes" }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => { if self.snapshot.complete() { self.phase = 1; } }
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
                    self.clip = Some(MeshClipJob::new(normal, distance, self.epsilon));
                } else {
                    self.build = Some(Build::new(std::mem::take(&mut self.snapshot.soup), true));
                    self.phase = 6;
                }
            }
            6 => {
                if let Some(mesh) = self.build.as_mut().unwrap().advance()? {
                    if mesh.face_count() < 4 { return Err(MeshKernelError::InvalidInput("bevel removed the solid".into())); }
                    self.attributes=Some(SourceAttributeRemap::new(mesh,Some(self.build.as_mut().unwrap().take_origin_vertices()?),None,None,None));self.phase=11;
                }
            }
            7=>{if let Some(mesh)=self.build.as_mut().unwrap().advance()? {let clip=self.clip.as_mut().unwrap();let mut attributes=SourceAttributeRemap::new(mesh,None,Some(clip.take_source_faces()?),None,None);attributes.configuration_mut()?.corner_faces=Some(clip.take_corner_faces()?);attributes.configuration_mut()?.generated_faces=Some(clip.take_face_weights()?);attributes.configuration_mut()?.generated_vertices=Some(Vec::new());self.attributes=Some(attributes);self.vertex=0;self.phase=8;}},
            8=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.vertex==origins.len() {self.phase=9;}else {self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(self.clip.as_ref().unwrap().vertex_source(origins[self.vertex] as usize)?);self.vertex+=1;}},
            9=>{if let Some(mesh)=self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source)? {self.snapshot=Snapshot::new(mesh);self.clip=None;self.build=None;self.attributes=None;self.phase=10;}},
            10=>{if self.snapshot.complete() {self.edge+=1;self.phase=5;}},
            11=>return self.attributes.as_mut().unwrap().advance(&mut self.snapshot.source),
            _=>unreachable!(),
        }
        Ok(None)
    }
}

struct Decimate {
    snapshot: Snapshot,
    target: usize,
    closed: bool,
    edges: HistoryFoldSet<(u32, u32)>,
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
        Self { snapshot: Snapshot::new(source), target, closed: true, edges: HistoryFoldSet::new(), ranked: BinaryHeap::new(), candidate: None, build: None, face: 0, corner: 0, source_faces:Vec::new(),edge_sources:Vec::new(),collapse:(0,0),attributes:None,phase: 0 }
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
                } else if self.snapshot.complete() { self.phase = 1; self.corner = 0; }
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
                        let mut attributes=SourceAttributeRemap::new(mesh,None,Some(std::mem::take(&mut self.source_faces)),None,None);attributes.configuration_mut()?.generated_vertices=Some(Vec::new());attributes.configuration_mut()?.edge_source_keys=Some(std::mem::take(&mut self.edge_sources));self.attributes=Some(attributes);self.corner=0;self.phase=4;
                    }
                }
            }
            4=>{let origins=self.build.as_ref().unwrap().origin_vertices();if self.corner==origins.len() {self.phase=5;}else {let vertex=origins[self.corner];self.attributes.as_mut().unwrap().configuration_mut()?.generated_vertices.as_mut().unwrap().push(if vertex==self.collapse.0 {SourceVertex::Edge{a:self.collapse.0,b:self.collapse.1,t:0.5}}else {SourceVertex::Original(vertex)});self.corner+=1;}},
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
    seen: HistoryFoldIndex<u32, usize>,
    normal: Normal,
    incidence: HistoryFoldIndex<(u32, u32), (usize, i32)>,
    links: HistoryFoldIndex<u32, u32>,
    face: usize,
    corner: usize,
    vertex: usize,
    start: Option<u32>,
    cursor: u32,
    phase: u8,
}

impl Candidate {
    fn new(a: u32, b: u32) -> Self {
        Self { a, b, soup: Soup::default(), corners: Vec::new(),corner_sources:Vec::new(),edge_sources:Vec::new(),face_sources:Vec::new(), seen: HistoryFoldIndex::new(), normal: Normal::default(), incidence: HistoryFoldIndex::new(), links: HistoryFoldIndex::new(), face: 0, corner: 0, vertex: 0, start: None, cursor: 0, phase: 0 }
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
                if self.corner == 0 { self.normal.set_origin(a); }
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

impl semio_framework_value::retirement::RetireOwned for TransformSelection {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.vertices),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.faces),semio_framework_value::retirement::deferred(self.selected)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Transform {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.mesh),semio_framework_value::retirement::deferred(self.flat),semio_framework_value::retirement::deferred(self.smooth),semio_framework_value::retirement::deferred(self.hes),semio_framework_value::retirement::deferred(self.attribute_names),semio_framework_value::retirement::deferred(self.selection),semio_framework_value::retirement::deferred(self.attribute_values),semio_framework_value::retirement::deferred(self.attribute_indices),semio_framework_value::retirement::deferred(self.attribute_samples),semio_framework_value::retirement::deferred(self.attribute_retired)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.mesh),semio_framework_value::retirement::deferred_birth_bytes_for(&self.flat),semio_framework_value::retirement::deferred_birth_bytes_for(&self.smooth),semio_framework_value::retirement::deferred_birth_bytes_for(&self.hes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_names),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selection),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_values),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_indices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_samples),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_retired)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Knife {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.distances),semio_framework_value::retirement::deferred(self.tessellation),semio_framework_value::retirement::deferred(self.tessellation_retirement),semio_framework_value::retirement::deferred(self.transfer),semio_framework_value::retirement::deferred(self.pieces),semio_framework_value::retirement::deferred(self.split),semio_framework_value::retirement::deferred(self.intersections),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.vertex_sources),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.distances),semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation),semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation_retirement),semio_framework_value::retirement::deferred_birth_bytes_for(&self.transfer),semio_framework_value::retirement::deferred_birth_bytes_for(&self.pieces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.split),semio_framework_value::retirement::deferred_birth_bytes_for(&self.intersections),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for Inflate {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.input),semio_framework_value::retirement::deferred(self.selected),semio_framework_value::retirement::deferred(self.normals),semio_framework_value::retirement::deferred(self.boundary),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.displaced),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.borders),semio_framework_value::retirement::deferred(self.inner),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.vertex_sources),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.border_faces),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.input),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.normals),semio_framework_value::retirement::deferred_birth_bytes_for(&self.boundary),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.displaced),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.borders),semio_framework_value::retirement::deferred_birth_bytes_for(&self.inner),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.border_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Orient {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.input),semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.keys),semio_framework_value::retirement::deferred(self.adjacency),semio_framework_value::retirement::deferred(self.oriented),semio_framework_value::retirement::deferred(self.flipped),semio_framework_value::retirement::deferred(self.stack),semio_framework_value::retirement::deferred(self.build)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.input),semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.keys),semio_framework_value::retirement::deferred_birth_bytes_for(&self.adjacency),semio_framework_value::retirement::deferred_birth_bytes_for(&self.oriented),semio_framework_value::retirement::deferred_birth_bytes_for(&self.flipped),semio_framework_value::retirement::deferred_birth_bytes_for(&self.stack),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for FillHoles {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.visited),semio_framework_value::retirement::deferred(self.vertices),semio_framework_value::retirement::deferred(self.boundary_edges),semio_framework_value::retirement::deferred(self.cap),semio_framework_value::retirement::deferred(self.cap_faces),semio_framework_value::retirement::deferred(self.corner_sources),semio_framework_value::retirement::deferred(self.edge_sources),semio_framework_value::retirement::deferred(self.face_sources),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.build)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.visited),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.boundary_edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.cap),semio_framework_value::retirement::deferred_birth_bytes_for(&self.cap_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.face_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for LoopCut {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.incidence),semio_framework_value::retirement::deferred(self.marked),semio_framework_value::retirement::deferred(self.pending),semio_framework_value::retirement::deferred(self.cut_vertices),semio_framework_value::retirement::deferred(self.cut_ids),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.boundary),semio_framework_value::retirement::deferred(self.grid),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.vertex_sources),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.incidence),semio_framework_value::retirement::deferred_birth_bytes_for(&self.marked),semio_framework_value::retirement::deferred_birth_bytes_for(&self.pending),semio_framework_value::retirement::deferred_birth_bytes_for(&self.cut_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.cut_ids),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.boundary),semio_framework_value::retirement::deferred_birth_bytes_for(&self.grid),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Mirror {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.source_vertices),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.mirrored_faces),semio_framework_value::retirement::deferred(self.attribute_vertices),semio_framework_value::retirement::deferred(self.attributes),semio_framework_value::retirement::deferred(self.reflected),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.original),semio_framework_value::retirement::deferred(self.mirrored),semio_framework_value::retirement::deferred(self.build)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.mirrored_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attribute_vertices),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.reflected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.original),semio_framework_value::retirement::deferred_birth_bytes_for(&self.mirrored),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Subdivide {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.input),semio_framework_value::retirement::deferred(self.selected),semio_framework_value::retirement::deferred(self.concave),semio_framework_value::retirement::deferred(self.tessellation),semio_framework_value::retirement::deferred(self.tessellation_retirement),semio_framework_value::retirement::deferred(self.transfer),semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.vertex_sources),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.input),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.concave),semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation),semio_framework_value::retirement::deferred_birth_bytes_for(&self.tessellation_retirement),semio_framework_value::retirement::deferred_birth_bytes_for(&self.transfer),semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.vertex_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}

impl semio_framework_value::retirement::RetireOwned for Bevel {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.selected),semio_framework_value::retirement::deferred(self.planes),semio_framework_value::retirement::deferred(self.clip),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.selected),semio_framework_value::retirement::deferred_birth_bytes_for(&self.planes),semio_framework_value::retirement::deferred_birth_bytes_for(&self.clip),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Decimate {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.snapshot),semio_framework_value::retirement::deferred(self.edges),semio_framework_value::retirement::deferred(self.ranked),semio_framework_value::retirement::deferred(self.candidate),semio_framework_value::retirement::deferred(self.build),semio_framework_value::retirement::deferred(self.source_faces),semio_framework_value::retirement::deferred(self.edge_sources),semio_framework_value::retirement::deferred(self.attributes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.snapshot),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edges),semio_framework_value::retirement::deferred_birth_bytes_for(&self.ranked),semio_framework_value::retirement::deferred_birth_bytes_for(&self.candidate),semio_framework_value::retirement::deferred_birth_bytes_for(&self.build),semio_framework_value::retirement::deferred_birth_bytes_for(&self.source_faces),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.attributes)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Candidate {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(self.soup),semio_framework_value::retirement::deferred(self.corners),semio_framework_value::retirement::deferred(self.corner_sources),semio_framework_value::retirement::deferred(self.edge_sources),semio_framework_value::retirement::deferred(self.face_sources),semio_framework_value::retirement::deferred(self.seen),semio_framework_value::retirement::deferred(self.incidence),semio_framework_value::retirement::deferred(self.links)])}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.soup),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corners),semio_framework_value::retirement::deferred_birth_bytes_for(&self.corner_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.edge_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.face_sources),semio_framework_value::retirement::deferred_birth_bytes_for(&self.seen),semio_framework_value::retirement::deferred_birth_bytes_for(&self.incidence),semio_framework_value::retirement::deferred_birth_bytes_for(&self.links)])}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Work {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {match self {Self::Bevel(value)=>semio_framework_value::retirement::deferred(value),Self::Decimate(value)=>semio_framework_value::retirement::deferred(value),Self::Mirror(value)=>semio_framework_value::retirement::deferred(value),Self::Merge(value)=>semio_framework_value::retirement::deferred(value),Self::Subdivide(value)=>semio_framework_value::retirement::deferred(value),Self::LoopCut(value)=>semio_framework_value::retirement::deferred(value),Self::Orient(value)=>semio_framework_value::retirement::deferred(value),Self::FillHoles(value)=>semio_framework_value::retirement::deferred(value),Self::Inflate(value)=>semio_framework_value::retirement::deferred(value),Self::Import(value)=>semio_framework_value::retirement::deferred(value),Self::Knife(value)=>semio_framework_value::retirement::deferred(value),Self::Transform(value)=>semio_framework_value::retirement::deferred(value)}}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(match self {Self::Bevel(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Decimate(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Mirror(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Merge(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Subdivide(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::LoopCut(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Orient(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::FillHoles(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Inflate(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Import(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Knife(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value),Self::Transform(value)=>semio_framework_value::retirement::deferred_birth_bytes_for(value)})}
    fn controlled_retirement_supported()->bool {true}
}

