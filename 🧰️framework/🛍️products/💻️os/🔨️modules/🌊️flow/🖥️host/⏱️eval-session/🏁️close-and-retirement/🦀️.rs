//! 🏁️ Close and retirement of the evaluation session: closing, the session source, the close and retirement demands and steps and the terminal state. See [`super`].

use super::*;

impl FlowEvalSession {
    /// 🧹️ Marks the original session closed without releasing or replacing any source.
    pub fn begin_close(&mut self){self.closing=true;self.tick_scheduled=false;}

    fn owns_session_source(&self)->bool{
        let state=&*self.state;
        state.pending_tick_snapshot.is_some()||state.pending_tick_snapshot_retirement.is_some()||state.pending_tick_snapshot_origin.is_some()||state.invocation_origin_growth.is_some()||state.invocation_origins.capacity()!=0||state.pending_tick_display.is_some()||state.operator_registry.is_some()||state.neural_cache.is_some()||state.previous_snapshot.is_some()||state.previous_channels.is_some()||state.eval_json.is_some()
            ||state.painted_eval_json.capacity()!=0||state.converged_eval_json.capacity()!=0||state.status_json.capacity()!=0||state.extension_evaluate_fault.is_some()
            ||state.preview_retention.is_some()||!state.window_tick_latches.terminal_is_empty()||!state.live_geometry_handles.terminal_is_empty()||!state.preview_mesh_by_handle.terminal_is_empty()
            ||!state.pending_tessellate_by_hash.terminal_is_empty()||!state.tessellate_handle_by_hash.terminal_is_empty()||!state.tessellate_progress_by_hash.terminal_is_empty()
            ||!state.eval_progress_by_hash.terminal_is_empty()||!state.preview_diagnostics_by_handle.terminal_is_empty()
    }

    fn take_session_source(&mut self)->Option<SessionCollectionOwner>{
        let state=&mut*self.state;
        if let Some(source)=state.pending_tick_snapshot.take(){return Some(SessionCollectionOwner::TickSnapshot(source))}
        if let Some(source)=state.invocation_origin_growth.take(){return Some(SessionCollectionOwner::InvocationOrigins(source))}
        if state.invocation_origins.capacity()!=0{return Some(SessionCollectionOwner::InvocationOrigins(std::mem::take(&mut state.invocation_origins)))}
        if let Some(value)=state.pending_tick_display.take(){return Some(SessionCollectionOwner::LivePublication(value))}
        if let Some(value)=state.preview_retention.take(){return Some(SessionCollectionOwner::Retention(value))}
        if let Some(value)=state.operator_registry.take(){return Some(SessionCollectionOwner::Registry(value))}
        if !state.window_tick_latches.terminal_is_empty(){return Some(SessionCollectionOwner::Latches(std::mem::take(&mut state.window_tick_latches)))}
        if !state.preview_mesh_by_handle.terminal_is_empty(){return Some(SessionCollectionOwner::Meshes(std::mem::take(&mut state.preview_mesh_by_handle)))}
        if !state.preview_diagnostics_by_handle.terminal_is_empty(){return Some(SessionCollectionOwner::Diagnostics(std::mem::take(&mut state.preview_diagnostics_by_handle)))}
        if !state.tessellate_handle_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Pending(std::mem::take(&mut state.tessellate_handle_by_hash)))}
        if !state.tessellate_progress_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Tessellation(std::mem::take(&mut state.tessellate_progress_by_hash)))}
        if !state.eval_progress_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Evaluation(std::mem::take(&mut state.eval_progress_by_hash)))}
        if !state.pending_tessellate_by_hash.terminal_is_empty(){return Some(SessionCollectionOwner::Pending(std::mem::take(&mut state.pending_tessellate_by_hash)))}
        if !state.live_geometry_handles.terminal_is_empty(){return Some(SessionCollectionOwner::Handles(std::mem::take(&mut state.live_geometry_handles)))}
        if let Some(value)=state.previous_snapshot.take(){return Some(SessionCollectionOwner::Snapshot(value))}
        if let Some(value)=state.previous_channels.take(){return Some(SessionCollectionOwner::Channels(value))}
        if let Some(value)=state.eval_json.take(){return Some(SessionCollectionOwner::SourceLease(value))}
        for text in [&mut state.painted_eval_json,&mut state.converged_eval_json,&mut state.status_json]{if text.capacity()!=0{return Some(SessionCollectionOwner::Text(std::mem::take(text)))}}
        if let Some(value)=state.extension_evaluate_fault.take(){return Some(SessionCollectionOwner::Fault(value))}
        state.neural_cache.take().map(SessionCollectionOwner::Cache)
    }

    pub(in super::super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(source)=self.state.pending_snapshot_copy.as_ref(){return source.next_close_demands(copy)}
        if let Some(source)=self.state.pending_tick_snapshot_retirement.as_ref(){return source.selected_demand(copy)}
        if self.state.pending_tick_snapshot.is_some()||self.state.pending_tick_snapshot_origin.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(host)=self.state.retiring_host.as_ref(){return Ok(RetirementDemand{copy_bytes:host.next_close_copy_byte_demand()?,capacity_bytes:host.next_close_capacity_byte_demand(copy)?,release_bytes:host.next_close_release_byte_demand()?,depth:host.next_close_depth_demand()?})}
        if self.state.pending_host.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(port)=self.retiring_geometry_port.as_ref(){return Ok(RetirementDemand{copy_bytes:port.next_copy_byte_demand()?,capacity_bytes:port.next_capacity_byte_demand(copy)?,release_bytes:port.next_release_byte_demand()?,depth:port.next_depth_demand()?})}
        if !self.retirement.terminal_is_empty(){return self.retirement.demands(copy)}
        Ok(RetirementDemand{depth:usize::from(self.geometry_port.is_some()||self.owns_session_source()),..Default::default()})
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}

    /// 🎟️ Quotes a new original retirement source handoff without allocating its queue or frame.
    pub fn next_retirement_admission_demands(&self)->Result<RetirementDemand,ValueError>{self.retirement.admission_demands()}
    pub fn next_retirement_reserve_capacity_byte_demand(&self)->Result<usize,ValueError>{self.retirement.owners.next_reserve_capacity_byte_demand()}
    pub fn reserve_retirement_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{self.retirement.owners.reserve_step(effect).result}
    pub fn next_retirement_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.copy_bytes)}
    pub fn next_retirement_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.retirement.demands(copy)?.capacity_bytes)}
    pub fn next_retirement_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.release_bytes)}
    pub fn next_retirement_depth_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.depth)}
    pub fn retirement_is_empty(&self)->bool{self.retirement.terminal_is_empty()}
    /// ♻️ Forwards the existing live retirement engine's poll with its real work.
    pub fn retirement_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.retirement.step(body,effect)}

    /// 🧊️ [`Self::close_step`] outside any Job turn inside a host-pump retirement turn funded by `grant`.
    pub fn close_step_cold(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,ValueError>{
        match crate::cold_effect(grant,|effect|self.close_step(effect))?{Some(done)=>done,None=>Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
    }

    pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,ValueError>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let grant=effect.grant();
        let empty=RetainedCloneProgress::default();
        if !self.closing{return Ok(Step::Blocked)}
        if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Flow session close requires its actual cursor depth"))}
        if let Some(source)=self.state.pending_snapshot_copy.as_mut(){if source.terminal_is_empty(){if grant.maximum_release_bytes<demand.release_bytes{return Ok(Step::Pending{progress:empty})}self.state.pending_snapshot_copy=None;return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}})}source.begin_close();let semio_framework_value::RetainedWorkPoll::Complete(turn)=source.close_step(grant.maximum_copy_bytes,effect)else{return Ok(Step::Pending{progress:empty})};let step=turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?;return Ok(Step::Pending{progress:step.progress()})}
        if let Some(source)=self.state.pending_tick_snapshot_retirement.as_mut(){let poll=source.step(grant.maximum_copy_bytes,effect);let semio_framework_value::RetainedWorkPoll::Complete(turn)=poll else{return Ok(Step::Pending{progress:empty})};let step=turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?;if source.terminal_is_empty(){self.state.pending_tick_snapshot_retirement=None}return Ok(Step::Pending{progress:step.progress()})}
        if let Some(source)=self.state.pending_tick_snapshot.take(){self.state.pending_tick_snapshot_retirement=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!()));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if self.state.pending_tick_snapshot_origin.take().is_some(){return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if let Some(host)=self.state.retiring_host.as_mut(){
            let result=host.close_step(effect).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,host.terminal_is_empty(),"Flow session original pending Host"));
            if host.terminal_is_empty(){self.state.retiring_host=None;}
            return match result{Ok(step)=>if self.terminal_is_empty(){Ok(Step::Complete{progress:step.progress()})}else{Ok(Step::Pending{progress:step.progress()})},Err(error)=>Err(error)}
        }
        if let Some(host)=self.state.pending_host.take(){self.state.retiring_host=Some(FlowHostRetirement::new(host));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if let Some(port)=self.retiring_geometry_port.as_mut(){
            let result=port.close_step(effect).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.terminal_is_empty(),"Flow session original geometry port"));
            if port.terminal_is_empty(){self.retiring_geometry_port=None;}
            return match result{Ok(step)=>Ok(Step::Pending{progress:step.progress()}),Err(error)=>Err(error)}
        }
        if !self.retirement.terminal_is_empty(){
            let semio_framework_value::RetainedWorkPoll::Complete(turn)=self.retirement.step(grant.maximum_copy_bytes,effect)else{return Ok(Step::Pending{progress:empty})};
            return match turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error){Ok(step)=>if self.terminal_is_empty(){Ok(Step::Complete{progress:step.progress()})}else{Ok(Step::Pending{progress:step.progress()})},Err(error)=>Err(error)}
        }
        if let Some(port)=self.geometry_port.take(){self.retiring_geometry_port=Some(semio_os_flow_geometry::GeometryPortRetirement::new(port));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if let Some(source)=self.take_session_source(){
            self.retirement.root=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("declared original session source root supports controlled retirement")));
            return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})
        }
        Ok(Step::Complete{progress:empty})
    }

    /// 🧊️ Pays each original close currency explicitly at the existing cold disposal boundary.
    pub fn retire_cold(mut self){
        self.begin_close();
        for _ in 0..FLOW_EVAL_SESSION_COLD_CLOSE_STEPS{
            if self.terminal_is_empty(){return}
            let copy=self.next_close_copy_byte_demand().expect("cold session original copy demand").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold session original capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold session original release demand"),maximum_depth:self.next_close_depth_demand().expect("cold session original depth demand")};
            crate::cold_effect(grant,|effect|self.close_step(effect)).expect("cold session original turn").expect("cold session original quote").expect("cold session original owner refused");
        }
        assert!(self.terminal_is_empty(),"cold session disposal did not reach original terminal ownership");
    }

    pub fn terminal_is_empty(&self)->bool{self.closing&&self.state.pending_snapshot_copy.is_none()&&self.state.pending_host.is_none()&&self.state.retiring_host.is_none()&&self.geometry_port.is_none()&&self.retiring_geometry_port.is_none()&&!self.owns_session_source()&&self.retirement.terminal_is_empty()}
}

impl FlowEvalSession{
 /// 🧹️ Closes the genuine session source after its separate intake, host and geometry frontiers end.
 pub fn retire_source_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  use semio_framework_value::RetainedWorkOutcome as Turn;let grant=effect.grant();
  if !self.closing||self.state.pending_snapshot_copy.is_some()||self.state.pending_tick_snapshot.is_some()||self.state.pending_tick_snapshot_retirement.is_some()||self.state.pending_tick_snapshot_origin.is_some()||self.state.pending_host.is_some()||self.state.retiring_host.is_some()||self.state.geometry_port.is_some()||self.state.retiring_geometry_port.is_some(){return Turn{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original session source awaits its separately owned frontiers")),work:0}}
  let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Turn{result:Err(error),work:0}};if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Turn{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.retirement.terminal_is_empty(){return match self.retirement.step(grant.maximum_copy_bytes,effect){semio_framework_value::RetainedWorkPoll::Complete(turn)=>Turn{result:turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),work:turn.work},semio_framework_value::RetainedWorkPoll::Pending{work}=>Turn{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
  if let Some(source)=self.take_session_source(){self.retirement.root=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!("original session source declares controlled retirement")));return Turn{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  Turn{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
}
