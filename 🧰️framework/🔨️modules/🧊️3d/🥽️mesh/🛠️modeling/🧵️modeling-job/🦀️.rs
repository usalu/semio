//! 🧵️ Modeling job: the retained mesh modeling job with its work enum, its stepping and the tessellation handoff close. See [`super`].

use super::*;

/// 🧵 Owns a source snapshot and retained geometry work without changing the source.
pub struct MeshModelingJob {
    pub(super) work: Work,
    pub(super) progress: MeshModelingProgress,
    pub(super) cancelled: bool,
    pub(super) retired: bool,
}

pub(super) enum Work {
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

    pub(in super::super) fn finish_with_progress(mut self, mut progress: impl FnMut(f32) -> bool) -> MeshResult<HalfedgeMesh> {
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

pub(super) fn close_tessellation_handoff(owner:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<MeshTessellationJob>>,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->MeshResult<bool> {
    let Some(current)=owner.as_mut()else{return Ok(true)};let result=current.step(grant);*receipt=current.step_progress();result.map_err(MeshKernelError::Retained)?;
    if current.terminal_is_empty(){owner.take();Ok(true)}else{Ok(false)}
}
