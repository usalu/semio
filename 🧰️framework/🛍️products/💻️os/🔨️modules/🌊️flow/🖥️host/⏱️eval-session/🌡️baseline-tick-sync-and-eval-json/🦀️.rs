//! 🌡️ Baseline, tick, sync and eval JSON of the evaluation session: the tick host, baseline capture, tick evaluation and display steps, repaint, the eval JSON lease and publication and the pending flag. See [`super`].

use super::*;

impl FlowEvalSession {
    /// 📥️ Retains the same admitted original Host across evaluation and publication turns.
    pub fn retain_tick_host(&mut self,host:FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,FlowHost)>{
        if self.closing||self.state.pending_host.is_some()||self.state.retiring_host.is_some(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original tick Host custody is occupied"),host))}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original tick Host intake requires an admitted item"),host))}
        if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"original tick Host intake requires source depth"),host))}
        self.state.pending_host=Some(host);self.state.pending_host_cancelled=false;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔎️ Borrows the exact original Host retained by the current tick.
    pub fn tick_host(&self)->Option<&FlowHost>{self.state.pending_host.as_ref()}

    pub fn install_baseline_into(&self,host:&mut FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        match host.install_eval_baseline(self.previous_snapshot.clone(),self.previous_channels.clone(),self.state.flow_extension_generation,grant){
            Ok(progress)=>Ok(progress),Err((error,snapshot,channels))=>{drop(snapshot);drop(channels);Err(error)},
        }
    }

    /// 🧵️ Takes over a host's eval baseline, INCLUDING which registry replacement it was computed
    /// against.
    ///
    /// The generation carry-back is monotonic and gated on the host actually holding a snapshot: a
    /// host that has evaluated against a newer registry owns results that are current, and leaving
    /// the session pinned to the older generation would make every ephemeral host it seeds refuse
    /// the incremental fast path forever. It can never walk BACKWARDS, so it cannot cancel a
    /// pending [`FlowEvalSession::invalidate_for_flow_extension_registry`]
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn capture_baseline_from(&mut self,host:&FlowHost,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if !host.baseline_publication_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original host baseline publication must complete before capture"))}
        if self.state.baseline_geometry_pending{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original geometry membership must finish before another baseline capture"))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline capture requires an admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original baseline capture requires source depth"))}
        let displaced=self.state.previous_snapshot.is_some()||self.state.previous_channels.is_some();
        if displaced{self.state.retirement.preflight(grant)?;}
        let (snapshot,channels)=host.eval_baseline();
        let evaluated_generation = host.eval_baseline_registry_generation();
        let progress=if displaced{
            let original=FlowBaselineLeases{snapshot:self.state.previous_snapshot.take(),channels:self.state.previous_channels.take(),current:None};
            match self.state.retirement.admit(SessionCollectionOwner::Baseline(original),grant){
                Ok(progress)=>progress,
                Err((error,SessionCollectionOwner::Baseline(original)))=>{self.state.previous_snapshot=original.snapshot;self.state.previous_channels=original.channels;return Err(error)},
                Err(_)=>unreachable!("original session baseline admission kind"),
            }
        }else{RetainedCloneProgress{copied_items:1,..Default::default()}};
        if snapshot.is_some()&&evaluated_generation>self.state.flow_extension_generation{self.state.flow_extension_generation=evaluated_generation;}
        self.state.previous_snapshot=snapshot;self.state.previous_channels=channels;
        self.state.baseline_geometry_pending=self.state.previous_channels.is_some();
        Ok(progress)
    }

    /// 🌐️ Geometry claims remain pending until their original admitted roster operation finishes.
    pub fn baseline_capture_terminal_is_empty(&self)->bool{!self.state.baseline_geometry_pending}

    /// 🧵️ Drives the genuine Work owned by the same retained tick Host with the full incoming wallet.
    pub fn tick_evaluation_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<FlowEvaluationStep,neural::OperatorWorkFailure>{
        self.state.tick_display_progress=Default::default();if cx.is_cancelled()||cx.should_yield(){return Ok(FlowEvaluationStep::Working)}let grant=cx.retained_grant();let demand=self.next_tick_evaluation_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(FlowEvaluationStep::Working)}
        if self.state.tick_evaluation_ready!=0{let result=self.tick_display_step(grant);let progress=self.state.tick_display_progress;if progress!=Default::default()||result.is_err(){cx.consume_fuel(1)}return match result{Ok(step)=>{cx.consume_retained(progress).map_err(semio_framework_job::OriginalTurnFailure::from)?;Ok(step)},Err(error)=>Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(progress),cx).into())}}
        let host=self.state.pending_host.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation tick Host was not admitted"))?;
        let step=host.evaluate_step(cx);self.state.tick_display_progress=host.evaluation_step_progress();if matches!(step,Ok(FlowEvaluationStep::Complete)){self.state.tick_evaluation_ready=1;return Ok(FlowEvaluationStep::Working)}step
    }
    pub fn tick_evaluation_step_progress(&self)->RetainedCloneProgress{self.state.tick_display_progress}
    pub fn next_tick_evaluation_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let host=self.state.pending_host.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation tick Host was not admitted"))?;
        if self.state.tick_evaluation_ready==0{return host.next_evaluation_demands(copy)}
        if !self.state.retirement.terminal_is_empty(){return self.state.retirement.demands(copy)}
        self.state.pending_tick_display.as_ref().map_or(Ok(RetirementDemand{depth:1,..Default::default()}),|owner|owner.demands(&host.last_eval_json))
    }
    fn tick_display_step(&mut self,grant:RetainedCloneGrant)->Result<FlowEvaluationStep,ValueError>{
        if !self.state.retirement.terminal_is_empty(){let result=self.state.retirement.step(grant);self.state.tick_display_progress=self.state.retirement.step_progress();result?;return Ok(FlowEvaluationStep::Working)}
        if self.state.tick_evaluation_ready==2{self.state.tick_evaluation_ready=0;self.state.tick_display_progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(FlowEvaluationStep::Complete)}
        if self.state.pending_tick_display.as_ref().is_some_and(SessionLivePublication::published){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original session cursor handoff requires depth"))}let original=self.state.pending_tick_display.take().unwrap();self.state.retirement.root=Some(ControlledRetirement::new(SessionCollectionOwner::LivePublication(original)).unwrap_or_else(|_|unreachable!("original session publication declares full retirement")));self.state.tick_display_progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.state.tick_evaluation_ready=2;return Ok(FlowEvaluationStep::Working)}
        if self.state.pending_tick_display.is_none(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original session display intake requires depth"))}let text=&self.state.pending_host.as_ref().unwrap().last_eval_json;self.state.pending_tick_display=Some(SessionLivePublication::new(text));self.state.tick_display_progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(FlowEvaluationStep::Working)}
        if !self.state.pending_tick_display.as_ref().unwrap().ready(){let state=&mut *self.state;let host=state.pending_host.as_ref().unwrap();let result=state.pending_tick_display.as_mut().unwrap().step(&host.last_eval_json,grant);match result{Ok(progress)=>state.tick_display_progress=progress,Err(error)=>{state.tick_display_progress=error.retained_progress();return Err(error)}}return Ok(FlowEvaluationStep::Working)}
        self.state.retirement.preflight(grant)?;
        let shared=self.state.pending_tick_display.as_mut().unwrap().take_shared().unwrap();
        match self.publish_eval_json(shared,grant){Ok(progress)=>self.state.tick_display_progress=progress,Err((error,shared))=>{self.state.pending_tick_display.as_mut().unwrap().restore_shared(shared);return Err(error)}}
        self.state.pending_tick_display.as_mut().unwrap().mark_published();Ok(FlowEvaluationStep::Working)
    }

    pub fn sync(&mut self, host: &FlowHost) -> bool {
        let remaining = host.pending_eval_widget_ids();
        self.status_json = build_flow_status_json(host, &remaining);
        if remaining.is_empty() {
            return false;
        }
        if self.tick_scheduled {
            return false;
        }
        self.tick_scheduled = true;
        true
    }

    /// 🔁️ Checks the existing partial-response ledger before sending a compact extension resume.
    pub fn has_evaluation_progress(&self,node_hash:u64)->bool {self.eval_progress_by_hash.contains_key(&node_hash)}

    /// ⏱️ One budgeted dag walk. `turn_started_us` is when the guest turn this walk belongs to began
    /// — `None` for a walk that opens its own turn, `Some` for one running inline inside a fold's
    /// turn, which then shares that turn's single deadline (see [`flow_eval_tick_budget`]).
    pub fn tick_cold(&mut self, host: &mut FlowHost, turn_started_us: Option<u64>) -> bool {
        let remaining = host.evaluate_cold_step(flow_eval_tick_budget(turn_started_us),&|hash|!self.has_evaluation_progress(hash)).expect("cold evaluation diagnostics require completed publication");
        while !host.baseline_publication_terminal_is_empty(){let demand=host.next_baseline_publication_demands(4096).expect("cold baseline demand");host.baseline_publication_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}).expect("cold baseline publication");}
        let state = &mut *self.state;
        if let Some(previous) = state.eval_json.replace(Arc::new(host.last_eval_json.clone())) { retire_session_collection_cold(SessionCollectionOwner::SourceLease(previous)); }
        self.status_json = build_flow_status_json(host, &remaining);
        if remaining.is_empty() {
            while !self.state.retirement.terminal_is_empty(){let d=self.state.retirement.demands(4096).expect("cold session retirement demand");self.state.retirement.step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}).expect("cold session retirement");}
            self.capture_baseline_from(host,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).expect("cold session baseline admission");
        }
        self.repaint(host, remaining.is_empty());
        self.tick_scheduled = !remaining.is_empty();
        self.tick_scheduled
    }

    /// 🖼️ Advances what a preview PAINTS, and — on a converged walk only — what a later unconverged
    /// one may fall back on.
    ///
    /// 🪟️ The fallback is filtered through the host's CURRENT widget roster, so switching example
    /// replaces the graph instead of painting the previous example's leftovers over the new one: a
    /// node id the document no longer carries is not a node whose answer is merely late.
    fn repaint(&mut self, host: &FlowHost, converged: bool) {
        let painted = if converged { self.eval_json().to_owned() } else { merge_unanswered_eval_entries(self.eval_json(), &self.converged_eval_json, &host.host_snapshot) };
        let state = &mut *self.state;
        std::mem::replace(&mut state.painted_eval_json, painted).retire_cold();
        if converged {
            let converged_text = state.eval_json.as_ref().map_or_else(String::new, |text| text.as_str().to_owned());
            std::mem::replace(&mut state.converged_eval_json, converged_text).retire_cold();
        }
    }

    pub fn eval_json(&self) -> &str {
        self.eval_json.as_ref().map_or("", |text| text.as_str())
    }

    /// 🔗️ Leases this primary immutable evaluation allocation without copying its text.
    pub fn lease_eval_json(&self)->Option<Arc<String>> {self.eval_json.as_ref().map(Arc::clone)}

    /// 🔍️ Admits publication only while the current primary owner is the held source lease.
    pub fn owns_eval_json(&self,source:&Arc<String>)->bool {self.eval_json.as_ref().is_some_and(|current|Arc::ptr_eq(current,source))}

    /// 🖼️ The evaluation a preview paints — see [`FlowEvalSessionState::painted_eval_json`]. This is
    /// what every publication carries; `eval_json` stays the live walk's own answer, because that is
    /// what the next walk, the pending-handle probe and every law about convergence read.
    pub fn painted_eval_json(&self) -> &str {
        if self.painted_eval_json.is_empty() {
            return self.eval_json();
        }
        &self.painted_eval_json
    }

    /// 🏁️ The last evaluation that fully converged — what a preview may fall back to while a node's
    /// new brep handle is still crossing and its tessellation has not landed yet.
    pub fn converged_eval_json(&self) -> &str {
        &self.converged_eval_json
    }

    /// 📤️ What this session owes ONE surface whose retained preview publication currently holds
    /// `retained`. See [`flow_eval_publication_for`] — the decision is per PUBLICATION TARGET, never
    /// per session: two preview windows on one instance each own their own retained bytes.
    pub fn eval_publication_for(&self, retained: Option<&str>) -> FlowEvalPublication {
        flow_eval_publication_for(self, retained)
    }

    pub fn status_json(&self) -> &str {
        if self.status_json.is_empty(){"{}"}else{&self.status_json}
    }

    pub fn status_json_for_host(&self, host: &FlowHost) -> String {
        let remaining = host.pending_eval_widget_ids();
        build_flow_status_json(host, &remaining)
    }

    /// 📤️ Advances the original live evaluation lease while preserving progress and preview owners.
    pub fn publish_eval_json(&mut self,eval_json:Arc<String>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Arc<String>)>{
        if self.state.eval_json.as_ref().is_some_and(|current|Arc::ptr_eq(current,&eval_json)){return Ok(Default::default())}
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,eval_json))}
        if let Some(original)=self.state.eval_json.take(){
            match self.state.retirement.admit(SessionCollectionOwner::SourceLease(original),grant){
                Ok(progress)=>{self.state.eval_json=Some(eval_json);Ok(progress)},
                Err((error,SessionCollectionOwner::SourceLease(original)))=>{self.state.eval_json=Some(original);Err((error,eval_json))},
                Err(_)=>unreachable!("original live evaluation lease returned a different source family"),
            }
        }else{self.state.eval_json=Some(eval_json);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
    }

    /// 🪙️ Borrows the pending Host publication demand without creating an evaluation owner.
    pub fn next_tick_publication_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        self.state.pending_host.as_ref().map_or(Ok(Default::default()),|host|host.next_baseline_publication_demands(copy))
    }

    pub fn next_tick_publication_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.copy_bytes)}
    pub fn next_tick_publication_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(copy)?.capacity_bytes)}
    pub fn next_tick_publication_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.release_bytes)}
    pub fn next_tick_publication_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_tick_publication_demands(0)?.depth)}

    /// 🎟️ Advances only the exact pending original Host publication under the incoming grant.
    pub fn tick_publication_step_progress(&self)->RetainedCloneProgress{self.state.pending_host.as_ref().map_or(Default::default(),FlowHost::baseline_publication_step_progress)}

    pub fn tick_publication_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let Some(host)=self.state.pending_host.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        let step=host.baseline_publication_step(grant)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,host.baseline_publication_terminal_is_empty(),"Flow original retained tick publication")
    }

    /// 🔗️ Captures the original retained tick's completed shared baseline, preserving refusal custody.
    pub fn capture_tick_baseline_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let Some(host)=self.state.pending_host.take()else{return Ok(Default::default())};
        let result=self.capture_baseline_from(&host,grant);self.state.pending_host=Some(host);result
    }

    /// 🎟️ Publishes one preborn immutable evaluation after admitting the original reset owners.
    pub fn set_eval_json(&mut self,eval_json:Arc<String>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Arc<String>)>{
        if let Err(error)=self.state.retirement.preflight(grant){return Err((error,eval_json))}
        let original=SessionResetSources::take(&mut self.state,false);
        match self.state.retirement.admit(SessionCollectionOwner::Reset(original),grant){
            Ok(progress)=>{self.state.eval_json=Some(eval_json);self.state.tick_scheduled=false;self.state.published_preview_mesh_digest=0;self.state.pending_host_cancelled=self.state.pending_host.is_some();Ok(progress)},
            Err((error,SessionCollectionOwner::Reset(original)))=>{original.restore(&mut self.state);Err((error,eval_json))},
            Err(_)=>unreachable!("original session reset returned a different source family"),
        }
    }

    pub fn pending(&self) -> bool {
        self.tick_scheduled
    }
}
