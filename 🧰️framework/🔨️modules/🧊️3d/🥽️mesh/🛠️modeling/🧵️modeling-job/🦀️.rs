//! 🧵️ Modeling job: the retained mesh modeling job with its work enum, its stepping and the tessellation handoff close. See [`super`].

use super::*;

/// 🧵 Owns a source snapshot and retained geometry work without changing the source.
pub struct MeshModelingJob {
    pub(super) work: Work,
    pub(super) progress: MeshModelingProgress,
    pub(super) cancelled: bool,
    pub(super) retired: bool,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum Work {
    Bevel(Box<Bevel>),
    Decimate(Box<Decimate>),
    Mirror(Box<Mirror>),
    Merge(Box<Merge>),
    Subdivide(Box<Subdivide>),
    LoopCut(Box<LoopCut>),
    Orient(Box<Orient>),
    FillHoles(Box<FillHoles>),
    Bridge(Box<Bridge>),
    BoundaryExtrude(Box<BoundaryExtrude>),
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
            Self::Bridge(work)if work.phase==0=>Some(&work.snapshot),
            Self::BoundaryExtrude(work)if work.phase==0=>Some(&work.snapshot),
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
            Self::Bridge(work)if work.phase==0=>Some(&mut work.snapshot),
            Self::BoundaryExtrude(work)if work.phase==0=>Some(&mut work.snapshot),
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
    /// 🧾️ Carries the actual owned-payload admission work through either original result branch.
    pub fn into_original_retirement(self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(semio_framework_value::ValueError,Option<Self>)>{
        let Self{work,progress,cancelled,retired}=self;let mut slot=Some(work);let turn=semio_framework_value::retirement::admit_owned_retirement(&mut slot,effect);
        let restore=|work:Option<Work>|work.map(|work|Self{work,progress,cancelled,retired});
        let result=match turn.result{Ok(Some(admitted))=>Ok(admitted),Ok(None)=>Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"mesh modeling retirement quote was not admitted"),restore(slot))),Err(error)=>Err((error,restore(slot)))};
        semio_framework_value::RetainedWorkOutcome{work:turn.work,result}
    }
    pub fn into_retirement(self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,RetainedCloneProgress),(semio_framework_value::ValueError,Option<Self>)>{self.into_original_retirement(effect).result}

    pub fn cancel(&mut self) {
        if !self.retired { self.cancelled = true; }
    }

    pub(in super::super) fn finish_with_progress(mut self, mut progress: impl FnMut(f32) -> bool) -> MeshResult<HalfedgeMesh> {
        let mut wallet=semio_framework_value::AuthoringWallet::new(RetainedCloneGrant {maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10},1_000_000_000_000);
        loop {
            let value=self.progress();
            if !self.cancelled && !progress(value.units_done as f32/value.units_total as f32) {self.cancel();}
            let step=self.finish_authored_step(&mut wallet);
            match step {
                Err(error)=>{self.abandon_decimation_discard();return Err(error);}
                Ok(MeshModelingStep::Working(_))=>{}
                Ok(MeshModelingStep::Done(mesh))=>{
                    if !progress(1.0) {return Err(MeshKernelError::InvalidInput("operation cancelled".into()));}
                    return Ok(mesh);
                }
                Ok(MeshModelingStep::Cancelled(_))=>return Err(MeshKernelError::InvalidInput("operation cancelled".into())),
            }
        }
    }

    fn finish_authored_step(&mut self,wallet:&mut semio_framework_value::AuthoringWallet)->MeshResult<MeshModelingStep> {
        let discard=self.decimation_retirement_demand(1<<16).map_err(MeshKernelError::Retained)?;
        let grant=if let Some(demand)=discard {
            RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
        }else {
            RetainedCloneGrant {maximum_items:256,maximum_copy_bytes:128,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10}.restricted_by(wallet.remaining())
        };
        if grant.maximum_items==0 {
            return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"modeling authoring stock exhausted")));
        }
        let mut receipt=RetainedCloneProgress::default();
        let turn=wallet.run_invoked(grant,1<<20,|effect|self.step(256,effect,&mut receipt)).map_err(MeshKernelError::Retained)?
            .ok_or_else(||MeshKernelError::InvalidInput("modeling quote refused".into()))?;
        wallet.post(receipt).map_err(MeshKernelError::Retained)?;
        turn.output
    }

    pub fn step(&mut self, budget: usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,tessellation_progress:&mut RetainedCloneProgress) -> MeshResult<MeshModelingStep> {
        let grant=effect.grant();*tessellation_progress=Default::default();
        if grant.maximum_items==0{return Ok(MeshModelingStep::Working(self.progress));}
        if self.retired { return Err(MeshKernelError::InvalidInput("modeling job already retired".into())); }
        if self.cancelled && !matches!(&self.work,Work::Decimate(work) if work.discard.is_some()) { return Ok(MeshModelingStep::Cancelled(self.progress)); }
        if let Some(demand)=self.tessellation_demands(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)? {if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Modeling tessellation frontier exceeds admitted depth")));}}
        for _ in 0..budget.min(grant.maximum_items) {
            if let Work::Decimate(work)=&mut self.work {
                if work.discard.is_some() {
                    let result=work.advance(effect,tessellation_progress);
                    self.progress.phase=work.phase();
                    if *tessellation_progress!=RetainedCloneProgress::default() {
                        self.progress.units_done=self.progress.units_done.saturating_add(1);
                        self.progress.units_total=self.progress.units_total.max(self.progress.units_done.saturating_add(1));
                    }
                    if let Err(error)=result {self.retired=true;return Err(error);}
                    if self.cancelled && work.discard.is_none() {return Ok(MeshModelingStep::Cancelled(self.progress));}
                    return Ok(MeshModelingStep::Working(self.progress));
                }
                if effect.remaining_work_units().map_err(MeshKernelError::Retained)?==0 {
                    return Ok(MeshModelingStep::Working(self.progress));
                }
            }
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
                if effect.remaining_work_units().map_err(MeshKernelError::Retained)?==0 {return Ok(MeshModelingStep::Working(self.progress));}
                if !effect.performed(1).map_err(MeshKernelError::Retained)? {return Ok(MeshModelingStep::Working(self.progress));}

            }
            let retained_corners=self.orientation_corners_demands();
            if let Some(demand)=retained_corners {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original orientation corners exceed admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
            }
            let retained_surface=self.refined_surface_demand(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained)?;
            if let Some(demand)=retained_surface {
                if grant.maximum_depth<demand.depth {
                    return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"refined surface transfer exceeds admitted depth")));
                }
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)
                    || effect.remaining_work_units().map_err(MeshKernelError::Retained)?==0
                {return Ok(MeshModelingStep::Working(self.progress));}
                if !matches!(self.work,Work::Decimate(_)) && !effect.performed(1).map_err(MeshKernelError::Retained)? {
                    return Ok(MeshModelingStep::Working(self.progress));
                }
            }
            let retained_translation=self.translation_demands();
            if let Some(demand)=retained_translation {
                if grant.maximum_depth<demand.depth {return Err(MeshKernelError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original translation exceeds admitted depth")));}
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(MeshModelingStep::Working(self.progress));}
                *tessellation_progress=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
            }
            let next=self.progress.units_done.checked_add(1).ok_or_else(||MeshKernelError::InvalidInput("modeling progress overflow".into()))?;
            let result = match &mut self.work {
                Work::Bevel(work) => work.advance(grant,tessellation_progress),
                Work::Decimate(work) => work.advance(effect,tessellation_progress),
                Work::Mirror(work) => work.advance(effect,tessellation_progress),
                Work::Merge(work) => work.advance(grant,tessellation_progress),
                Work::Subdivide(work) => work.advance(effect,tessellation_progress),
                Work::LoopCut(work) => work.advance(effect.grant(),tessellation_progress),
                Work::Orient(work) => work.advance(grant,tessellation_progress),
                Work::FillHoles(work) => work.advance(grant,tessellation_progress),
                Work::Bridge(work) => work.advance(grant,tessellation_progress),
                Work::BoundaryExtrude(work) => work.advance(grant,tessellation_progress),
                Work::Inflate(work) => work.advance(effect.grant(),tessellation_progress),
                Work::Import(work) => work.advance(),
                Work::Knife(work) => work.advance(effect,tessellation_progress),
                Work::Transform(work) => work.advance(tessellation_progress),
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
                Work::Bridge(work) => work.phase(),
                Work::BoundaryExtrude(work) => work.phase(),
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
            if retained_tessellation||retained_surface.is_some()||retained_translation.is_some()||retained_initialization.is_some()||retained_incidence.is_some()||retained_adjacency.is_some()||retained_components.is_some()||retained_corners.is_some(){return Ok(MeshModelingStep::Working(self.progress));}
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

    fn abandon_decimation_discard(&mut self) {
        if let Work::Decimate(work)=&mut self.work {
            if let Some(owner)=work.discard.take() {owner.abandon();}
        }
    }

    /// 🧮️ Quotes the same retired decimation candidate or replaced topology held by this job.
    pub fn decimation_retirement_demand(&self, copy:usize)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        let Work::Decimate(work)=&self.work else {return Ok(None)};
        work.discard.as_ref().map(|owner|owner.selected_demand(copy)).transpose()
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
    /// 🎨️ Quotes the currently retained raw surface row for a mesh refinement.
    pub fn refined_surface_demand(&self,copy:usize)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {
            Work::Orient(work)=>work.surface_demand(copy),
            Work::FillHoles(work) if work.phase==8=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Bridge(work) if work.phase==6=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::BoundaryExtrude(work) if work.phase==12=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Transform(work)=>work.surface_demand(),
            Work::Bevel(work) if work.phase==12=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::LoopCut(work) if work.phase==10=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Knife(work) if work.phase==12=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Inflate(work) if work.phase==9=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Mirror(work) if work.phase==6=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Subdivide(work) if work.phase==8=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            Work::Merge(work)=>work.retained_surface_demand(copy),
            Work::Decimate(work) if work.phase==5 && work.discard.is_none()=>work.attributes.as_ref().unwrap().refined_surface_demand(&work.snapshot.source,copy),
            _=>Ok(None),
        }
    }

    /// 🗺️ Reads the current retained winding stage quote for host dispatch.
    pub fn orientation_normal_demand(&self,copy:usize)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        let Work::Orient(work)=&self.work else {return Ok(None);};
        match work.phase {
            1=>work.initialization_demands().map(Some),
            2=>work.incidence_demands(copy).map(Some),
            3=>work.adjacency_demands().map(Some),
            4=>work.components_demands().map(Some),
            5=>Ok(Some(work.corners_demands())),
            _=>Ok(None),
        }
    }

    /// 🌐️ Quotes the original stack birth, visit writes or free component cursor move.
    pub fn orientation_components_demands(&self)->Result<Option<semio_framework_value::RetirementDemand>,semio_framework_value::ValueError> {
        match &self.work {Work::Orient(work)if work.phase==4=>work.components_demands().map(Some),_=>Ok(None)}
    }
}

/// 🧪️ Drives one modeling step for cold synchronous callers under an explicit authoring stock.
pub(super) fn modeling_step(job:&mut MeshModelingJob)->MeshResult<MeshModelingStep> {
    let grant=RetainedCloneGrant {maximum_items:256,maximum_copy_bytes:128,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10};
    let mut wallet=semio_framework_value::AuthoringWallet::new(grant,1_000_000_000_000);
    match wallet.run(grant,1<<20,|effect|job.step(256,effect,&mut RetainedCloneProgress::default())).map_err(MeshKernelError::Retained)? {Some(done)=>done.output,None=>Err(MeshKernelError::InvalidInput("modeling quote refused".into()))}
}

pub(super) fn close_modeling_handoff<T:semio_framework_value::retirement::RetireOwned>(owner:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->MeshResult<bool> {
    let Some(current)=owner.as_mut()else{return Ok(true)};
    let body=effect.grant().maximum_copy_bytes;
    let poll=current.step(body,effect);
    *receipt=current.step_progress();
    if let semio_framework_value::RetainedWorkPoll::Complete(turn)=poll {
        turn.result.map_err(|failure|MeshKernelError::Retained(failure.into_value_error()))?;
    }
    if current.terminal_is_empty(){owner.take();Ok(true)}else{Ok(false)}
}
