//! 🧩️ Extension generation and preview retention of the evaluation session: the flow extension generation and invalidation, evaluate faults, the node cache seed and the retained preview meshes. See [`super`].

use super::*;

impl FlowEvalSession {
    /// 🔢 Which flow extension registry replacement this session's held results were computed
    /// against — the invalidation key, readable so a surface can state it rather than infer it.
    pub fn flow_extension_generation(&self) -> u64 {
        self.flow_extension_generation
    }

    /// 🔄️ Re-arms this session against the flow extension registry as it is NOW.
    ///
    /// A node whose operator no plugin had contributed yet does not merely stall: its miss is
    /// retained everywhere a result is retained — the neural cache entry, the incremental
    /// baseline (`previous_snapshot`/`previous_channels`), the published `eval_json`/`status_json`,
    /// and the tessellation ledger whose `phase` a surface projects as
    /// `PreviewTessellatePhase::Faulted`. Installing the contribution AFTERWARDS therefore changes
    /// nothing a surface can see until somebody says those results predate the registry, and
    /// `tick_scheduled` is still `true` from the chain that gave up, so `sync` refuses to arm a new
    /// one. This is that statement, and the REGISTRY GENERATION is its key: any later contribution
    /// change bumps it too, so a re-push of an unchanged closure (whose generation is unmoved)
    /// invalidates nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// Answers whether anything was actually invalidated, so the caller only re-arms a chain it
    /// owes.
    pub fn invalidate_for_flow_extension_registry(&mut self,generation:u64,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
        if self.operator_registry.is_some()||self.state.flow_extension_generation==generation{return Ok((false,Default::default()))}
        self.state.retirement.preflight(grant)?;
        let original=SessionResetSources::take(&mut self.state,true);
        match self.state.retirement.admit(SessionCollectionOwner::Reset(original),grant){
            Ok(progress)=>{self.state.flow_extension_generation=generation;self.state.preview_cancelled=false;self.state.tick_scheduled=false;self.state.published_preview_mesh_digest=0;self.state.pending_host_cancelled=self.state.pending_host.is_some();Ok((true,progress))},
            Err((error,SessionCollectionOwner::Reset(original)))=>{original.restore(&mut self.state);Err(error)},
            Err(_)=>unreachable!("original registry invalidation returned a different source family"),
        }
    }

    /// 💥 Records the fault an `evaluate` answer came back with. The extension id and the decoded
    /// message are the SDK's own (`reactor::extension_response_args` echoes `faultCode`/
    /// `faultMessage` onto the response action) — nothing here re-words them.
    pub fn note_extension_evaluate_fault(&mut self,fault:ExtensionEvaluateFault,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,ExtensionEvaluateFault)> {
        if self.state.extension_evaluate_fault.is_none(){
            if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"session fault handoff requires an admitted item"),fault))}
            if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"session fault handoff requires original source depth"),fault))}
            self.state.extension_evaluate_fault=Some(fault);return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
        }
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,fault))}
        let original=self.state.extension_evaluate_fault.take().unwrap();
        match self.state.retirement.admit(SessionCollectionOwner::Fault(original),grant){
            Ok(progress)=>{self.state.extension_evaluate_fault=Some(fault);Ok(progress)},
            Err((error,SessionCollectionOwner::Fault(original)))=>{self.state.extension_evaluate_fault=Some(original);Err((error,fault))},
            Err(_)=>unreachable!("session original fault admission kind"),
        }
    }

    /// ✅️ Forgets the last evaluate fault — an answer that folded supersedes it.
    pub fn clear_extension_evaluate_fault(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if self.state.extension_evaluate_fault.is_none(){return Ok(Default::default())}
        self.state.retirement.preflight(grant)?;
        let original=self.state.extension_evaluate_fault.take().unwrap();
        match self.state.retirement.admit(SessionCollectionOwner::Fault(original),grant){
            Ok(progress)=>Ok(progress),
            Err((error,SessionCollectionOwner::Fault(original)))=>{self.state.extension_evaluate_fault=Some(original);Err(error)},
            Err(_)=>unreachable!("session original fault admission kind"),
        }
    }

    /// 💥 The evaluate fault this session is currently living with, if any.
    pub fn extension_evaluate_fault(&self) -> Option<&ExtensionEvaluateFault> {
        self.extension_evaluate_fault.as_ref()
    }

    pub fn seed_node_cache(&self, node_hash: u64, output: Dictionary) {
        let cache = self.neural_cache.as_deref().expect("live Flow evaluation session owns its neural cache");
        seed_flow_eval_node_cache(cache, node_hash, output)
    }

    /// 🧊️ Borrows the completed canonical mesh owned by this original session.
    pub fn preview_mesh(&self,handle:&str)->Option<&MeshData>{self.preview_mesh_by_handle.get(handle)}

    /// 📏️ Mesh-residency fingerprint the preview last published — see [`FlowEvalSessionState::published_preview_mesh_digest`].
    pub fn published_preview_mesh_digest(&self) -> u64 {
        self.published_preview_mesh_digest
    }

    /// 📏️ Records the mesh-residency fingerprint a preview publication just carried.
    pub fn note_published_preview_mesh_digest(&mut self, digest: u64) {
        self.published_preview_mesh_digest = digest;
    }

    /// 🪟️ Borrows resident canonical handles without allocating a copied roster.
    pub fn preview_mesh_handles(&self)->impl Iterator<Item=&str>{self.preview_mesh_by_handle.keys().map(String::as_str)}
    /// 🪪️ Native scene construction borrows the exact indexed row only under its original cache epoch.
    pub fn preview_mesh_epoch(&self)->u64{self.preview_mesh_by_handle.mutation_epoch()}
    pub fn preview_mesh_slot_count(&self)->usize{self.preview_mesh_by_handle.slot_count()}
    pub fn preview_mesh_at_slot(&self,slot:usize,epoch:u64)->Result<Option<(&str,&MeshData)>,ValueError>{if self.preview_mesh_by_handle.mutation_epoch()!=epoch{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview mesh source epoch changed"))}Ok(self.preview_mesh_by_handle.slot_entry(slot).map(|(handle,mesh)|(handle.as_str(),mesh)))}


    /// 🩺 Blocking validate-gate findings for `handle`, as a JSON array string.
    pub fn preview_diagnostics(&self, handle: &str) -> Option<&str> {
        self.preview_diagnostics_by_handle.get(handle).map(String::as_str)
    }

    /// 🩺 Every handle the validate gate rejected, paired with its JSON issue array.
    pub fn preview_diagnostic_entries(&self) -> Vec<(&str, &str)> {
        self.preview_diagnostics_by_handle.iter().map(|(handle, issues)| (handle.as_str(), issues.as_str())).collect()
    }

    /// 📥️ Retains the exact incoming roster before pruning any original preview owner.
    pub fn begin_retain_preview_meshes(&mut self,handles:Vec<String>)->Result<(),(ValueError,Vec<String>)>{
        if self.closing||self.state.preview_retention.is_some()||!self.preview_cancellation_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original preview retention is already active or closing"),handles))}
        self.state.preview_retention=Some(SessionPreviewRetention{roster:Some(handles),phase:0,slot:0,comparison:0,matched:false,removed:None});
        self.state.preview_retention_progress=Default::default();Ok(())
    }
    pub fn preview_retention_terminal_is_empty(&self)->bool{self.state.preview_retention.is_none()}
    pub fn preview_retention_step_progress(&self)->RetainedCloneProgress{self.state.preview_retention_progress}
    fn preview_retention_handle(&self,owner:&SessionPreviewRetention)->Option<&str>{
        match owner.phase {
            0=>self.state.preview_mesh_by_handle.slot_entry(owner.slot).map(|(handle,_)|handle.as_str()),
            1=>self.state.preview_diagnostics_by_handle.slot_entry(owner.slot).map(|(handle,_)|handle.as_str()),
            2=>self.state.pending_tessellate_by_hash.slot_entry(owner.slot).map(|(_,handle)|handle.as_str()),
            3=>self.state.tessellate_progress_by_hash.slot_entry(owner.slot).and_then(|(hash,_)|self.state.tessellate_handle_by_hash.get(hash)).map(String::as_str),
            4=>self.state.tessellate_handle_by_hash.slot_entry(owner.slot).map(|(_,handle)|handle.as_str()),
            _=>None,
        }
    }
    /// 🪙️ Borrows all five currencies from the actual removed row or actual geometry cursor.
    pub fn next_preview_retention_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let Some(owner)=self.state.preview_retention.as_ref()else{return Ok(Default::default())};
        if let Some(removed)=owner.removed.as_ref(){return Ok(RetirementDemand{copy_bytes:removed.next_copy_byte_demand()?,capacity_bytes:removed.next_capacity_byte_demand(copy)?,release_bytes:removed.next_release_byte_demand()?,depth:removed.next_depth_demand()?})}
        if owner.phase==6 {let port=self.geometry_port.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original geometry retention source disappeared"))?;return Ok(RetirementDemand{copy_bytes:port.next_retain_copy_byte_demand()?,capacity_bytes:port.next_retain_capacity_byte_demand(copy)?,release_bytes:port.next_retain_release_byte_demand()?,depth:port.next_retain_depth_demand()?})}
        let depth=match owner.phase {0=>self.preview_mesh_by_handle.next_extract_slot_depth_demand(owner.slot)?,1=>self.preview_diagnostics_by_handle.next_extract_slot_depth_demand(owner.slot)?,2=>self.pending_tessellate_by_hash.next_extract_slot_depth_demand(owner.slot)?,3=>self.tessellate_progress_by_hash.next_extract_slot_depth_demand(owner.slot)?,4=>self.tessellate_handle_by_hash.next_extract_slot_depth_demand(owner.slot)?,_=>1};
        Ok(RetirementDemand{depth:depth.max(1),..Default::default()})
    }
    /// 🍂️ Compares one roster entry or closes one exact original removed row per turn.
    pub fn retain_preview_meshes_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.state.preview_retention_progress=Default::default();
        if self.preview_retention_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_preview_retention_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original preview retention exceeds admitted depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut owner=self.state.preview_retention.take().unwrap();
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if let Some(removed)=owner.removed.as_mut(){
            let result=removed.step(grant);self.state.preview_retention_progress=removed.step_progress();self.state.preview_retention=Some(owner);
            let step=result?;progress=step.progress();
            if self.state.preview_retention.as_ref().unwrap().removed.as_ref().unwrap().terminal_is_empty(){self.state.preview_retention.as_mut().unwrap().removed=None;}
            self.state.preview_retention_progress=progress;return Ok(RetainedCloneStep::Progress(progress))
        }
        if owner.phase==6 {
            let port=self.geometry_port.as_mut().unwrap();let result=port.retain_step(grant);progress=port.retain_step_progress();
            if port.retain_terminal_is_empty(){owner.phase=7;}
            self.state.preview_retention=Some(owner);self.state.preview_retention_progress=progress;
            let step=result.map_err(|error|error.with_retained_progress(progress))?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.geometry_port.as_ref().unwrap().retain_terminal_is_empty(),"Flow original preview geometry retention")?;
            return Ok(RetainedCloneStep::Progress(progress))
        }
        if owner.phase==7 {self.state.baseline_geometry_pending=false;self.state.preview_retention_progress=progress;return Ok(RetainedCloneStep::Complete(progress))}
        if owner.phase==5 {
            let handles=owner.roster.take().unwrap();
            if let Some(port)=self.geometry_port.as_mut(){if let Err((error,handles))=port.begin_retain(handles){owner.roster=Some(handles);self.state.preview_retention=Some(owner);return Err(error)}owner.phase=6;}
            else{owner.removed=Some(ControlledRetirement::new(SessionRetentionRow::Roster(handles)).unwrap_or_else(|_|unreachable!("original roster declares retirement")));owner.phase=7;}
        }else {
            let slots=match owner.phase {0=>self.preview_mesh_by_handle.slot_count(),1=>self.preview_diagnostics_by_handle.slot_count(),2=>self.pending_tessellate_by_hash.slot_count(),3=>self.tessellate_progress_by_hash.slot_count(),4=>self.tessellate_handle_by_hash.slot_count(),_=>unreachable!()};
            if owner.slot>=slots{owner.phase+=1;owner.slot=0;owner.comparison=0;owner.matched=false;}
            else if !owner.matched&&owner.comparison<owner.roster.as_ref().unwrap().len(){owner.matched=self.preview_retention_handle(&owner).is_some_and(|handle|handle==owner.roster.as_ref().unwrap()[owner.comparison]);owner.comparison+=1;}
            else{
                let keep=owner.matched;let row=match owner.phase {0=>self.preview_mesh_by_handle.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Mesh),1=>self.preview_diagnostics_by_handle.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Diagnostic),2=>self.pending_tessellate_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Pending),3=>self.tessellate_progress_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Progress),4=>self.tessellate_handle_by_hash.extract_slot_if(owner.slot,|_,_|keep).map(SessionRetentionRow::Pending),_=>unreachable!()};
                if let Some(row)=row{owner.removed=Some(ControlledRetirement::new(row).unwrap_or_else(|_|unreachable!("original preview row declares retirement")));}
                owner.slot+=1;owner.comparison=0;owner.matched=false;
            }
        }
        self.state.preview_retention=Some(owner);self.state.preview_retention_progress=progress;Ok(RetainedCloneStep::Progress(progress))
    }
}
