//! ⏱️ Behaviour of `FlowEvalSession`: baseline capture, ticks, window latches, preview retention and close steps.

use super::*;

impl FlowEvalSession {
    /// 📔️ Shares the explicitly supplied registry with this session's evaluation hosts.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(!self.closing, "closed session refuses an operator registry");
        assert!(self.state.operator_registry.is_none(), "operator registry is supplied once");
        self.state.operator_registry = Some(registry);
        self.state.flow_extension_generation = 0;
        self
    }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn semio_framework_os_flow_geometry::GeometryPort>) -> Self {
        assert!(!self.closing, "closed session refuses a geometry authority");
        assert!(self.state.geometry_port.is_none(), "geometry authority is supplied once");
        self.state.geometry_port = Some(port);
        self
    }
    /// 🌐️ Reads the explicitly supplied geometry authority for this evaluation session.
    pub fn geometry_port(&self) -> Option<&dyn semio_framework_os_flow_geometry::GeometryPort> { self.state.geometry_port.as_deref() }

    pub fn new() -> Self {
        Self {
            state: std::mem::ManuallyDrop::new(FlowEvalSessionState {
                retiring_geometry_port: None,
                geometry_port: None,
                operator_registry: None,
                neural_cache: Some(Arc::new(NeuralCache::new())),
                previous_snapshot: None,
                previous_channels: None,
                eval_json: Some(Arc::new(String::new())),
                painted_eval_json: String::new(),
                converged_eval_json: String::new(),
                status_json: "{}".into(),
                tick_scheduled: false,
                window_tick_latches: HistoryFoldIndex::new(),
                live_geometry_handles: HistoryFoldIndex::new(),
                preview_mesh_by_handle: HistoryFoldIndex::new(),
                pending_tessellate_by_hash: HistoryFoldIndex::new(),
                tessellate_handle_by_hash: HistoryFoldIndex::new(),
                tessellate_progress_by_hash: HistoryFoldIndex::new(),
                eval_progress_by_hash: HistoryFoldIndex::new(),
                preview_diagnostics_by_handle: HistoryFoldIndex::new(),
                retirement: SessionRetirement::default(),
                flow_extension_generation: flow_extension_registry_generation(),
                extension_evaluate_fault: None,
                preview_cancelled: false,
                published_preview_mesh_digest: 0,
                baseline_geometry_pending:false,
                invocation_origin_growth:None,
                invocation_origins:Vec::new(),
                pending_tick_display:None,
                tick_display_progress:Default::default(),
                tick_evaluation_ready:0,
                pending_host:None,
                pending_snapshot_copy:None,
                pending_tick_snapshot:None,
                pending_tick_snapshot_retirement:None,
                pending_tick_snapshot_origin:None,
                retiring_host:None,
                pending_host_cancelled:false,
                preview_cancellation_cursor:None,
                preview_cancellation_phase:0,
                preview_cancellation_progress:Default::default(),
                preview_retention:None,
                preview_retention_progress:Default::default(),
                closing: false,
            }),
        }
    }

    /// 📏️ Quotes the real new ledger allocation before moving any original request alias.
    pub fn next_invocation_origin_capacity_bytes(&self)->usize{if self.state.invocation_origin_growth.is_none()&&self.state.invocation_origins.len()==self.state.invocation_origins.capacity(){(self.state.invocation_origins.len()+1)*size_of::<FlowInvocationEntry>()}else{0}}
    pub fn next_invocation_origin_reserve_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.state.invocation_origin_growth.is_some()&&self.state.invocation_origins.is_empty(){if !self.state.retirement.terminal_is_empty(){return self.state.retirement.demands(copy)}if self.state.invocation_origins.capacity()!=0{return self.state.retirement.admission_demands()}}
        Ok(RetirementDemand{capacity_bytes:self.next_invocation_origin_capacity_bytes(),depth:1,..Default::default()})
    }
    pub fn invocation_origin_reserve_pending(&self)->bool{self.state.invocation_origin_growth.is_some()||self.next_invocation_origin_capacity_bytes()!=0}
    pub fn reserve_invocation_origin_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if !self.invocation_origin_reserve_pending()||grant.maximum_items==0{return Ok(Default::default())}let d=self.next_invocation_origin_reserve_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation roster requires source depth"))}if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes{return Ok(Default::default())}
        if self.state.invocation_origin_growth.is_none(){let mut incoming=Vec::new();incoming.try_reserve_exact(self.state.invocation_origins.len()+1).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation roster birth refused"))?;let bytes=incoming.capacity()*size_of::<FlowInvocationEntry>();self.state.invocation_origin_growth=Some(incoming);return Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})}
        if let Some(original)=self.state.invocation_origins.pop(){self.state.invocation_origin_growth.as_mut().unwrap().push(original);return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
        if !self.state.retirement.terminal_is_empty(){return self.state.retirement.step(grant).map(|step|step.progress())}
        if self.state.invocation_origins.capacity()!=0{self.state.retirement.preflight(grant)?;let original=std::mem::take(&mut self.state.invocation_origins);return match self.state.retirement.admit(SessionCollectionOwner::InvocationOrigins(original),grant){Ok(progress)=>Ok(progress),Err((error,SessionCollectionOwner::InvocationOrigins(original)))=>{self.state.invocation_origins=original;Err(error)},Err(_)=>unreachable!("original invocation allocation return")}}
        self.state.invocation_origins=self.state.invocation_origin_growth.take().unwrap();Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    /// 📥️ Moves the exact preborn request source alias before the original invocation leaves its session.
    pub fn retain_invocation_origin_step(&mut self,source:FlowInvocationOriginLease,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,FlowInvocationOriginLease)>{
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original invocation source requires an admitted item"),source))}if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation source requires depth"),source))}if self.invocation_origin_reserve_pending(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation source requires a reserved roster slot"),source))}self.state.invocation_origins.push(FlowInvocationEntry{origin:source,reply:None});Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    /// 🧊️ The diagnostic caller uses the same original ledger before producing its cold invocation wire.
    pub fn retain_invocation_origin_cold(&mut self,source:FlowInvocationOriginLease){for _ in 0..FLOW_EVAL_SESSION_COLD_CLOSE_STEPS{if !self.invocation_origin_reserve_pending(){break}let d=self.next_invocation_origin_reserve_demands(4096).expect("cold original invocation reserve demand");self.reserve_invocation_origin_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}).expect("cold original invocation reserve");}assert!(!self.invocation_origin_reserve_pending());self.retain_invocation_origin_step(source,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).unwrap_or_else(|_|unreachable!("cold original invocation was reserved"));}
    pub fn invocation_origin_at(&self,index:usize)->Option<&FlowInvocationOrigin>{self.state.invocation_origins.get(index).map(|entry|&*entry.origin)}
    /// 🧊️ Borrows the exact diagnostic request source for its genuine window and original node hash.
    pub fn invocation_origin_cold(&self,window:&str,node_hash:u64)->Option<&FlowInvocationOrigin>{self.state.invocation_origins.iter().find(|entry|entry.origin.window_id==window&&entry.origin.node_hash==node_hash).map(|entry|&*entry.origin)}
    /// 📥️ Moves the original decoded response into its already retained source slot.
    pub fn retain_invocation_reply_step(&mut self,index:usize,reply:String,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,String)>{
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original invocation reply requires an admitted item"),reply))}if grant.maximum_depth==0{return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"original invocation reply requires source depth"),reply))}let Some(entry)=self.state.invocation_origins.get_mut(index)else{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original invocation reply source is absent"),reply))};if entry.reply.is_some(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original invocation reply source is occupied"),reply))}entry.reply=Some(reply);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    pub fn invocation_reply_at(&self,index:usize)->Option<&str>{self.state.invocation_origins.get(index).and_then(|entry|entry.reply.as_deref())}
    /// 🧊️ The explicit diagnostic receiver retains its decoded response until typed result admission.
    pub fn retain_invocation_reply_cold(&mut self,window:&str,node_hash:u64,reply:String)->Result<(),(ValueError,String)>{let Some(index)=self.state.invocation_origins.iter().position(|entry|entry.origin.window_id==window&&entry.origin.node_hash==node_hash)else{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original invocation reply ledger is absent"),reply))};self.retain_invocation_reply_step(index,reply,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).map(|_|())}

    pub fn neural_cache(&self) -> Arc<NeuralCache> {
        self.neural_cache.as_ref().expect("live Flow evaluation session owns its neural cache").clone()
    }

    /// 🪙️ Quotes the selected original source and its concrete producer frame before either is born.
    pub fn tick_snapshot_intake_constructor_demands<Source,P:semio_framework_job::OriginalSnapshotIntakeProducer<FlowHostSnapshot,Source>>(producer:&P)->Result<RetirementDemand,ValueError>{
        let mut demand=producer.source_demands()?;demand.capacity_bytes=demand.capacity_bytes.checked_add(size_of::<P::Intake>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick intake metadata overflows"))?;Ok(demand)
    }
    /// 🧾️ Retains the exact original source through its selected concrete producer before normal copying begins.
    pub fn begin_tick_snapshot_intake<Source,P:semio_framework_job::OriginalSnapshotIntakeProducer<FlowHostSnapshot,Source>>(&mut self,read:Source,producer:&P,revision:[u8;32],source_generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),FlowSnapshotIntakeFailure<P::Rejected>>{
        if self.closing||!self.tick_snapshot_intake_terminal_is_empty()||self.state.pending_host.is_some()||self.state.retiring_host.is_some(){return Err(FlowSnapshotIntakeFailure::Original{failure:ValueError::literal(ValueRefusalKind::WorkLimit,"original tick snapshot intake custody is occupied").into(),source:producer.preserve_source(read)})}
        let birth=size_of::<P::Intake>();let demand=match producer.source_demands(){Ok(demand)=>demand,Err(error)=>return Err(FlowSnapshotIntakeFailure::Original{failure:error.into(),source:producer.preserve_source(read)})};let grant=cx.retained_grant();let Some(capacity)=demand.capacity_bytes.checked_add(birth)else{return Err(FlowSnapshotIntakeFailure::Original{failure:ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tick intake metadata overflows").into(),source:producer.preserve_source(read)})};
        if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_capacity_bytes<capacity{return Err(FlowSnapshotIntakeFailure::Original{failure:ValueError::literal(ValueRefusalKind::WorkLimit,"original tick snapshot intake awaits constructor admission").into(),source:producer.preserve_source(read)})}
        let copy=producer.admit(read,revision,source_generation,cx).map_err(|(failure,source)|FlowSnapshotIntakeFailure::Original{failure,source})?;
        self.state.pending_snapshot_copy=Some(Box::new(copy));self.state.pending_tick_snapshot_origin=Some((cx.operation().0,cx.generation().0));cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:birth,..Default::default()}).map_err(|error|FlowSnapshotIntakeFailure::Retained{failure:error.into()})?;Ok(())
    }
    pub fn next_tick_snapshot_intake_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.state.pending_snapshot_copy.as_ref().map_or(Ok(Default::default()),|source|source.next_normal_demands(body))}
    /// 🧵️ Advances the same read-bound copy with the initiating operation's actual remaining authority.
    pub fn tick_snapshot_intake_step(&mut self,revision:[u8;32],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let source=self.state.pending_snapshot_copy.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original tick snapshot read was not admitted"))?;
        source.advance(revision,cx)?;Ok(source.complete())
    }
    /// 📤️ Moves only the completed owned snapshot while retaining its original read for full funded closure.
    pub fn take_tick_snapshot_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if cx.is_cancelled()||cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
        if self.state.pending_tick_snapshot_origin!=Some((cx.operation().0,cx.generation().0)){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tick transfer requires its initiating operation").into())}
        if self.state.pending_tick_snapshot.is_some(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original completed snapshot custody is occupied").into())}
        let source=self.state.pending_snapshot_copy.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original tick snapshot read is absent"))?;
        if !source.complete(){return Ok(false)}
        self.state.pending_tick_snapshot=source.take();source.begin_close();cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)
    }
    pub fn admitted_tick_snapshot(&self)->Option<&FlowHostSnapshot>{self.state.pending_tick_snapshot.as_ref()}
    /// ♻️ Closes only the original intake source while its completed snapshot remains retained.
    pub fn close_tick_snapshot_intake_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.state.pending_snapshot_copy.is_some()&&self.state.pending_tick_snapshot_origin!=Some((cx.operation().0,cx.generation().0)){if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked)}cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tick source close requires its initiating operation").into())}
        let Some(source)=self.state.pending_snapshot_copy.as_mut()else{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})};
        source.begin_close();let d=source.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
        if source.terminal_is_empty(){let step=semio_framework_job::close_original_retained_step(cx,d,|_|{self.state.pending_snapshot_copy=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()}))})?;return Ok(if self.state.pending_snapshot_copy.is_none(){match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},semio_framework_job::InteractiveJobCloseStep::Blocked=>semio_framework_job::InteractiveJobCloseStep::Blocked}}else{step})}
        semio_framework_job::close_original_retained_step(cx,d,|grant|source.close_step(grant))
    }

    /// 🛑️ Quotes closure of only this tick's original read and incomplete snapshot construction.
    pub fn abort_tick_snapshot_intake_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(source)=self.state.pending_snapshot_copy.as_ref(){return source.next_close_demands(body)}
        if let Some(source)=self.state.pending_tick_snapshot_retirement.as_ref(){return source.selected_demand(body)}
        Ok(RetirementDemand{depth:usize::from(self.state.pending_tick_snapshot.is_some()||self.state.pending_tick_snapshot_origin.is_some()),..Default::default()})
    }
    /// ✅️ Snapshot intake reaches terminal only after the original read, snapshot and initiating identity close.
    pub fn tick_snapshot_intake_terminal_is_empty(&self)->bool{self.state.pending_snapshot_copy.is_none()&&self.state.pending_tick_snapshot.is_none()&&self.state.pending_tick_snapshot_retirement.is_none()&&self.state.pending_tick_snapshot_origin.is_none()}
    /// ♻️ Retains every partial original tick owner until its same initiating context finishes paid closure.
    pub fn abort_tick_snapshot_intake_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step};
        if cx.should_yield(){return Ok(InteractiveJobCloseStep::Blocked)}
        if self.state.pending_tick_snapshot_origin.is_some_and(|origin|origin!=(cx.operation().0,cx.generation().0)){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tick close requires its initiating operation").into())}
        if self.state.pending_snapshot_copy.is_some(){return self.close_tick_snapshot_intake_step(cx).map(|step|match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}
        let d=self.abort_tick_snapshot_intake_demands(cx.retained_grant().maximum_copy_bytes)?;
        if let Some(source)=self.state.pending_tick_snapshot_retirement.as_mut(){let step=close_original_retained_step(cx,d,|grant|source.step(grant))?;if source.terminal_is_empty(){self.state.pending_tick_snapshot_retirement=None}return Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}
        if self.state.pending_tick_snapshot.is_some(){return close_original_retained_step(cx,d,|_|{let source=self.state.pending_tick_snapshot.take().unwrap();self.state.pending_tick_snapshot_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!()));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))})}
        if self.state.pending_tick_snapshot_origin.is_some(){let step=close_original_retained_step(cx,d,|_|{self.state.pending_tick_snapshot_origin=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))})?;return Ok(if self.tick_snapshot_intake_terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

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

    //#region 🔒️TickLatch
    /// 🪙️ Quotes the exact original latch arena without constructing a replacement row.
    pub fn next_window_tick_latch_copy_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_copy_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_capacity_byte_demand(&self,window_id:&str,copy:usize)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,copy)}
    }
    pub fn next_window_tick_latch_release_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_release_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_depth_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_depth_demand(&key)}
    }

    /// 🌱️ Admits original backing before placing the original Copy-keyed latch.
    pub fn prepare_window_tick_latch(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,(ValueError,RetainedCloneProgress)>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original latch preparation requires one admitted item"),Default::default()))}
        if self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,grant.maximum_copy_bytes).map_err(|error|(error,RetainedCloneProgress::default()))?>0{
            return self.state.window_tick_latches.reserve_insert_step(&key,grant).map(RetainedCloneStep::Progress);
        }
        match self.state.window_tick_latches.insert_reserved(key,FlowEvalWindowTickLatch::default(),grant){
            Ok((None,progress))=>Ok(RetainedCloneStep::Complete(progress)),
            Ok((Some(_),_))=>unreachable!("original latch preparation cannot displace an occupied row"),
            Err((error,_,_))=>Err((error,Default::default())),
        }
    }

    fn window_tick_latch_mut(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<&mut FlowEvalWindowTickLatch,ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original latch mutation requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original latch mutation requires source depth"))}
        self.state.window_tick_latches.get_mut(&flow_eval_window_key(window_id)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original latch backing must be admitted before mutation"))
    }

    /// 🔒️ Arms `window_id`'s `flowEvalTick` if — and only if — nothing already owes one.
    ///
    /// This is the ONE gate every arming source goes through: the host refresh poll
    /// (`ArtifactApp::pending_effects`), the `setContributions` install, `setActiveExample`, every
    /// view command, and the chain's own continuations. Answers whether the CALLER owes the effect,
    /// so a refused arm emits nothing rather than duplicating a tick that is already in flight.
    ///
    /// ⏳️ A window waiting on an extension answer is not armed at all — it is marked `owed` instead,
    /// and [`FlowEvalSession::settle_window_extension`] hands the arm to whichever answer lands
    /// last. Ticking a window whose invocation is outstanding recomputes the identical pending
    /// request, which is how one graph turned into 298 invocations against 111 settles in 80 s of
    /// live console (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn arm_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if latch.armed {
            return Ok((false,progress));
        }
        if latch.in_flight > 0 {
            latch.owed = true;
            return Ok((false,progress));
        }
        latch.armed = true;
        latch.owed = false;
        Ok((true,progress))
    }

    /// 🔎️ Whether `window_id` owes a tick that nothing has armed — a window that has never ticked
    /// owes its first one, and a window whose own last tick reported unfinished work owes another
    /// only while no tick and no extension answer is already chasing it. The REFRESH poll's whole
    /// question, and the reason it costs no evaluation at all.
    pub fn window_tick_owed(&self, window_id: &str) -> bool {
        if self.preview_cancelled{return false}
        match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
            None => true,
            Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
        }
    }

    /// 🔒️ Arms the tick `window_id` actually owes — [`FlowEvalSession::window_tick_owed`] and
    /// [`FlowEvalSession::arm_window_tick`] as the single question a refresh poll asks.
    pub fn arm_owed_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        if self.window_tick_owed(window_id){self.arm_window_tick(window_id,grant)}else{Ok((false,Default::default()))}
    }

    /// ▶️ Marks `window_id`'s armed tick as RUNNING: the effect has been delivered, so the latch is
    /// free for whatever this tick's own outcome decides to arm next.
    /// 🛑 A tick that actually begins is work RESUMING, which is the one thing that retires the
    /// `cancelled` banner an explicit gesture raised — see [`FlowEvalSession::preview_cancelled`].
    pub fn begin_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if !self.preview_cancellation_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original preview cancellation must finish before another tick"))}
        self.window_tick_latch_mut(window_id,grant)?.armed=false;
        self.state.preview_cancelled = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// ⏳️ Records that `window_id`'s tick parked `count` extension invocations. Those answers own
    /// the continuation from here — the tick that parked them owes no re-arm.
    pub fn note_window_extensions_in_flight(&mut self, window_id: &str, count: usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 📝️ Records what `window_id`'s tick reported: `unfinished` is the tick's own "there is more to
    /// compute". Only this makes a refresh poll able to answer [`FlowEvalSession::window_tick_owed`]
    /// without evaluating anything.
    pub fn note_window_tick_outcome(&mut self, window_id: &str, unfinished: bool,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        self.window_tick_latch_mut(window_id,grant)?.unfinished=unfinished;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🚧️ Marks `window_id`'s chain as GIVEN UP: an extension answer this process cannot fold (a
    /// faulted `invokeExtension`, an answer that seeds no cache entry) is not slow work, and the
    /// next tick would park the identical request and fault again at the host's own cadence. Nothing
    /// but a gesture — a contributions install, an example switch, an edit — may resume it, and each
    /// of those arms through [`FlowEvalSession::arm_window_tick`] directly.
    pub fn abandon_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.unfinished = false;
        latch.owed = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🧹️ Drops the latch of every window that is no longer attached. A detached window's latch can
    /// never be discharged by a tick — the retained route refuses a window that has left the
    /// roster — so keeping it would make a window that comes back unarmable forever.
    pub fn retain_window_tick_latches(&mut self,window_ids:&[&str],cursor:&mut usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_depth<self.state.window_tick_latches.next_extract_slot_depth_demand(*cursor)?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"session latch pruning requires original arena depth"))}
        let slot=*cursor;
        if slot>=self.state.window_tick_latches.slot_count(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        self.state.window_tick_latches.extract_slot_if(slot,|key,_|window_ids.iter().any(|id|flow_eval_window_key(id)==*key));
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        *cursor+=1;
        Ok(if *cursor==self.state.window_tick_latches.slot_count(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    /// ✅️ Folds ONE extension answer back into `window_id`'s latch. Answers whether that settle
    /// discharged an arm a sibling answer had already asked for, so the LAST answer of a fan-out
    /// emits exactly one re-arm and the earlier ones emit none.
    pub fn settle_window_extension(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_sub(1);
        if latch.in_flight == 0 && latch.owed && !latch.armed {
            latch.armed = true;
            latch.owed = false;
            return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    /// 🔁️ Whether the fold that just settled `window_id`'s answer may run that window's next wave
    /// INLINE, inside the answer's own guest turn, instead of leaving the `previewEval` run job to
    /// dispatch a `flowEvalTick` hop for it.
    ///
    /// 🪜️ The question is deliberately the run job's OWN scheduling question
    /// ([`FlowEvalSession::window_tick_owed`]): a fold may only take over a hop the scheduler would
    /// otherwise have dispatched, never invent one. So the hop count can fall but the chain's shape
    /// cannot change — and a fold that declines simply leaves the latch alone, which IS the park:
    /// the run job's next step reads the same debt and dispatches the round trip as before. There is
    /// no parked-continuation state anywhere, because "not inline" is the pre-existing behaviour.
    ///
    /// 🛑 A CANCELLED session is refused outright. `begin_window_tick` retires the `cancelled` banner
    /// (work resuming is the one thing that may), so an inline continuation on a cancelled chain
    /// would not merely compute one wave too many — it would un-cancel the run the user stopped. A
    /// cancel also defaults every latch, so `window_tick_owed` already answers `false`; the explicit
    /// guard is kept because the two facts must not be one accident apart
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏱️ The wall half is [`flow_eval_inline_continuation_fits`]: a turn with less than
    /// [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] of its evaluation allowance left parks. BOTH
    /// instants are the caller's, so the rule is pure and a law states the park boundary in
    /// microseconds instead of racing a clock — and so a native test, which installs no clock at all,
    /// can still drive the branch a browser turn takes.
    pub fn inline_continuation_admitted(&self, window_id: &str, turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
        !self.preview_cancelled() && self.window_tick_owed(window_id) && flow_eval_inline_continuation_fits(turn_started_us, now_us)
    }

    /// 🔎️ How many extension answers `window_id` is still waiting for — readable so a law can state
    /// the in-flight rule rather than infer it.
    pub fn window_extensions_in_flight(&self, window_id: &str) -> u32 {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).map(|latch| latch.in_flight).unwrap_or(0)
    }

    /// 🔎️ Whether a `flowEvalTick` is currently pending for `window_id`.
    pub fn window_tick_is_armed(&self, window_id: &str) -> bool {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).is_some_and(|latch| latch.armed)
    }

    /// 🧹️ Abandons every window's latch — what a chain RESTART owes itself. The results the old
    /// chain was chasing have just been swept (a registry replacement, a fresh document), so the
    /// invocations it parked can no longer resume anything and their windows must be free to be
    /// armed again from scratch.
    pub fn clear_window_tick_latches(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if self.state.window_tick_latches.terminal_is_empty(){return Ok(Default::default())}
        self.state.retirement.preflight(grant)?;
        let source=SessionCollectionOwner::Latches(std::mem::take(&mut self.state.window_tick_latches));
        match self.state.retirement.admit(source,grant){
            Ok(progress)=>Ok(progress),
            Err((error,SessionCollectionOwner::Latches(original)))=>{self.state.window_tick_latches=original;Err(error)},
            Err(_)=>unreachable!("session original latch admission kind"),
        }
    }
    //#endregion 🔒️TickLatch

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

    /// 📨 Notes an in-flight tessellate; returns true when the caller should emit `InvokeExtension`.
    pub fn note_pending_tessellate(&mut self, node_hash: u64, handle: String) -> bool {
        if self.preview_mesh_by_handle.contains_key(&handle){return false;}
        if self.preview_diagnostics_by_handle.contains_key(&handle) {
            return false;
        }
        if self.pending_tessellate_by_hash.values().any(|pending| pending == &handle) {
            return false;
        }
        self.tessellate_handle_by_hash.insert(node_hash, handle.clone());
        self.pending_tessellate_by_hash.insert(node_hash, handle);
        true
    }

    /// 📈 Aggregate progress of every tessellation this session has admitted and not yet finished —
    /// what the preview window's status object reports.
    pub fn preview_tessellate_status(&self) -> PreviewTessellateStatus {
        let mut status = PreviewTessellateStatus::default();
        status.in_flight = self.pending_tessellate_by_hash.len() as u32;
        status.diagnostics = self.preview_diagnostics_by_handle.len() as u32;
        for progress in self.tessellate_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.faces_done = status.faces_done.saturating_add(progress.faces_done);
            status.faces_total = status.faces_total.saturating_add(progress.faces_total);
            if progress.phase!=PreviewTessellatePhase::Complete{status.phase=progress.phase;}
        }
        if status.units_total == 0 && status.in_flight > 0 {
            status.phase = PreviewTessellatePhase::SamplingEdges;
        }
        status
    }

    /// 🛑️ Records cancellation while retaining every original source until its admitted turn.
    pub fn cancel_preview_evaluation(&mut self,_window_id:&str,grant:RetainedCloneGrant)->Result<(usize,RetainedCloneProgress),ValueError>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview session is closing"))}
        if self.preview_cancelled{return Ok((0,Default::default()))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"preview cancellation requires an admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires admitted depth"))}
        let count=self.state.pending_tessellate_by_hash.len();
        self.state.preview_cancelled=true;self.state.tick_scheduled=false;self.state.pending_host_cancelled=self.state.pending_host.is_some();self.state.preview_cancellation_cursor=Some(0);self.state.preview_cancellation_phase=0;
        Ok((count,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    pub fn next_preview_cancellation_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.state.preview_cancellation_cursor.is_none(){return Ok(Default::default())}
        match self.state.preview_cancellation_phase{
            0=>self.retirement.admission_demands(),
            4=>self.geometry_port.as_ref().map_or(Ok(Default::default()),|port|Ok(RetirementDemand{copy_bytes:port.next_cancel_copy_byte_demand()?,capacity_bytes:port.next_cancel_capacity_byte_demand(copy)?,release_bytes:port.next_cancel_release_byte_demand()?,depth:port.next_cancel_depth_demand()?})),
            5=>self.retirement.demands(copy),
            _=>Ok(RetirementDemand{depth:1,..Default::default()}),
        }
    }

    /// ⏱️ Hands off exact sources, resets one actual row, or advances the original geometry cancellation.
    pub fn preview_cancellation_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.state.preview_cancellation_progress=Default::default();
        let Some(cursor)=self.state.preview_cancellation_cursor else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_preview_cancellation_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires its original cursor depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        match self.state.preview_cancellation_phase{
            0=>{
                self.retirement.preflight(grant)?;
                let original=SessionPreviewCancellationSources::take(&mut self.state);
                match self.retirement.admit(SessionCollectionOwner::Cancellation(original),grant){Ok(receipt)=>progress=receipt,Err((error,SessionCollectionOwner::Cancellation(original)))=>{original.restore(&mut self.state);return Err(error)},Err(_)=>unreachable!("original cancellation source changed")}
                self.state.preview_cancellation_phase=1;
            },
            1=>{
                if cursor<self.state.tessellate_progress_by_hash.slot_count(){if let Some((_,row))=self.state.tessellate_progress_by_hash.slot_entry_mut(cursor){if row.phase!=PreviewTessellatePhase::Complete{row.phase=PreviewTessellatePhase::Cancelled;}}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=2;self.state.preview_cancellation_cursor=Some(0);}
            },
            2=>{
                if cursor<self.state.window_tick_latches.slot_count(){if let Some((_,row))=self.state.window_tick_latches.slot_entry_mut(cursor){*row=FlowEvalWindowTickLatch::default();}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=3;}
            },
            3=>{if let Some(port)=self.geometry_port.as_mut(){port.begin_cancel()?;self.state.preview_cancellation_phase=4;}else{self.state.preview_cancellation_phase=5;}},
            4=>{
                let state=&mut *self.state;
                let port=state.geometry_port.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancelling geometry source disappeared"))?;
                let result=port.cancel_step(grant);progress=port.cancel_step_progress();state.preview_cancellation_progress=progress;let step=result.map_err(|error|error.with_retained_progress(progress))?;
                semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.cancel_terminal_is_empty(),"Flow session original geometry cancellation")?;
                if port.cancel_terminal_is_empty(){state.preview_cancellation_phase=5;}
            },
            5=>{let result=self.retirement.step(grant);progress=self.retirement.step_progress();self.state.preview_cancellation_progress=progress;let step=result?;progress=step.progress();if self.retirement.terminal_is_empty(){self.state.preview_cancellation_cursor=None;}},
            _=>unreachable!("original preview cancellation phase"),
        }
        self.state.preview_cancellation_progress=progress;
        Ok(if self.preview_cancellation_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn preview_cancellation_step_progress(&self)->RetainedCloneProgress{self.state.preview_cancellation_progress}

    pub fn preview_cancellation_terminal_is_empty(&self)->bool{self.state.preview_cancellation_cursor.is_none()}

    /// 🛑 The `tessellateCancel` request body the cancelling command sends to the geometry extension.
    /// Deliberately addresses NO handle: the session keys its ledger by `nodeHash`, which is a
    /// one-way digest of `(handle, tolerance bits)`, so the tolerance half of the extension's job key
    /// is not recoverable here — and a cancel that can only retire SOME of the jobs it means to
    /// retire is worse than the whole-registry branch the capability already offers
    /// (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `tessellateCancel` handler).
    pub fn preview_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        format!("{{\"windowId\":{},\"windowKindId\":{}}}", semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_id.to_string())), semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_kind_id.to_string())))
    }

    /// 🧊️ Decodes the explicit diagnostic envelope; normal response work retains its original source.
    pub fn resolve_preview_eval_cold(&mut self,node_hash:u64,envelope_json:&str)->Result<PreviewEvalOutcome,ValueError>{
        let envelope=semio_framework_pack_json::parse(envelope_json,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(semio_framework_pack_json::JsonError::into_value_error)?;
        if let Some(pending)=envelope.get("pending"){
            let text=|key|pending.get(key).and_then(semio_framework_pack_json::Value::as_str).map(str::to_owned).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original pending response metadata is absent"));
            let hash=pending.get("nodeHash").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original pending response hash is absent"))?;
            return Ok(PreviewEvalOutcome::Pending(neural::PendingExtensionEval{neuron_id:text("neuronId")?,extension_id:text("extensionId")?,operator_id:text("operatorId")?,node_hash:hash,input_json:text("inputJson")?}));
        }
        let done=envelope.get("done").and_then(semio_framework_pack_json::Value::as_bool).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response completion is absent"))?;
        let phase=PreviewEvalPhase::from_job_tag(envelope.get("phase").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response phase is absent"))?);
        let units_done=envelope.get("unitsDone").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response done units are absent"))?;
        let units_total=envelope.get("unitsTotal").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response total units are absent"))?;
        if !done{self.eval_progress_by_hash.insert(node_hash,PreviewEvalProgress{units_done,units_total,phase});return Ok(PreviewEvalOutcome::Working)}
        self.eval_progress_by_hash.remove(&node_hash);
        if matches!(phase,PreviewEvalPhase::Cancelled){return Ok(PreviewEvalOutcome::Cancelled)}
        let output_json=envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original completed evaluation payload is absent"))?.to_owned();Ok(PreviewEvalOutcome::Complete{output_json})
    }

    /// 📈 Aggregate progress of every budgeted evaluation this session has admitted and not yet
    /// finished — what the preview window's status object reports for the `evaluate` half of the
    /// work.
    pub fn preview_eval_status(&self) -> PreviewEvalStatus {
        let mut status = PreviewEvalStatus { in_flight: self.eval_progress_by_hash.len() as u32, ..PreviewEvalStatus::default() };
        for progress in self.eval_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.phase = progress.phase;
        }
        status
    }

    /// ⛓️ The CHAIN's own progress — the ledger that is live for the WHOLE of an evaluation, which
    /// neither finer ledger is.
    ///
    /// 🩸️ [`FlowEvalSession::preview_tessellate_status`] reads `pending_tessellate_by_hash` and
    /// [`FlowEvalSession::preview_eval_status`] reads `eval_progress_by_hash`. Both are empty at
    /// every hop boundary of a `flowEvalTick` chain: a tessellation is admitted and answered inside
    /// one hop, and a budgeted evaluation only lands in its ledger once an extension has already
    /// answered `done: false`. An `evaluate` round trip parked at the geometry extension — which is
    /// where a boolean preview spends its whole slow half — is recorded in NEITHER. So every status
    /// a preview window published between hops was byte-identical `phase: "idle", inFlight: 0,
    /// ratio: 1.0` while the kernel was busy, on both renderers: 54 identical publications across a
    /// 23 s evaluation, no pill text, no progress, nothing to read
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
    ///
    /// ⛓️ What IS live throughout is the per-window tick latch — `armed` (a hop is outstanding),
    /// `in_flight` (an extension answer is owed), `owed` (an answer asked for a continuation) and
    /// `unfinished` (the window's own last tick said there is more) — plus `tick_scheduled`. The
    /// monotone denominator is the session's own published per-node census: a node leaves
    /// `queued`/`computing` exactly once per chain and never returns to it, so
    /// `nodes_done / nodes_total` only ever grows while one evaluation runs.
    pub fn preview_chain_status(&self) -> PreviewChainStatus {
        let in_flight: u32 = self.window_tick_latches.values().map(|latch| latch.in_flight).sum();
        let working = self.tick_scheduled || self.window_tick_latches.values().any(|latch| latch.armed || latch.owed || latch.unfinished || latch.in_flight > 0);
        let mut status = PreviewChainStatus { in_flight, working, ..PreviewChainStatus::default() };
        if let Some(widgets) = semio_framework_pack_json::parse(&self.status_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_object().cloned()) {
            for (_, entry) in widgets.iter() {
                status.nodes_total = status.nodes_total.saturating_add(1);
                if !matches!(entry.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("queued" | "computing")) {
                    status.nodes_done = status.nodes_done.saturating_add(1);
                }
            }
        }
        status
    }

    /// 🛑 The `evaluateCancel` request body the cancelling command sends to the geometry extension.
    /// Whole-registry, for the same reason [`FlowEvalSession::preview_cancel_invocation_request_json`]
    /// is: a session cancels its preview, not one named operator hop.
    pub fn preview_eval_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        Self::preview_cancel_invocation_request_json(window_id, window_kind_id)
    }

    /// 🛑 Whether the user's explicit cancel is the last thing that happened to this evaluation —
    /// what makes a surface able to publish `phase: "cancelled"` even when the gesture landed before
    /// any tessellation had been admitted.
    pub fn preview_cancelled(&self) -> bool {
        self.preview_cancelled
    }

    /// ⏳️ How many extension answers this session is waiting for across EVERY window it publishes
    /// into — the session-wide half of "is there work in flight", which the per-window latch can only
    /// answer one window at a time. A surface's `cancellable` is this OR a tessellation in flight: an
    /// `evaluate` round trip admits no tessellation at all, so the tessellation ledger alone reports
    /// `idle` through the whole (slowest) part of a boolean preview.
    pub fn extensions_in_flight(&self) -> u32 {
        self.window_tick_latches.values().map(|latch| latch.in_flight).sum()
    }

    /// 📥️ Advances the genuine original completed receiver into the session-owned canonical cache.
    pub fn next_preview_tessellate_demands(&self,receiver:&FlowPreviewCompletedReceiver,copy:usize)->Result<RetirementDemand,ValueError>{receiver.next_demands(&self.preview_mesh_by_handle,copy)}
    pub fn resolve_preview_tessellate(&mut self,receiver:&mut FlowPreviewCompletedReceiver,address:&FlowPreviewCompletedAddress<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview session is closing").into())}
        receiver.step(address,&mut self.state.preview_mesh_by_handle,cx)
    }

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

    pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
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
    pub fn reserve_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.retirement.owners.reserve_step(grant)}
    pub fn next_retirement_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.copy_bytes)}
    pub fn next_retirement_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.retirement.demands(copy)?.capacity_bytes)}
    pub fn next_retirement_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.release_bytes)}
    pub fn next_retirement_depth_demand(&self)->Result<usize,ValueError>{Ok(self.retirement.demands(0)?.depth)}
    pub fn retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.retirement.step(grant)}

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,ValueError>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let empty=RetainedCloneProgress::default();
        if !self.closing{return Ok(Step::Blocked)}
        if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original Flow session close requires its actual cursor depth"))}
        if let Some(source)=self.state.pending_snapshot_copy.as_mut(){if source.terminal_is_empty(){if grant.maximum_release_bytes<demand.release_bytes{return Ok(Step::Pending{progress:empty})}self.state.pending_snapshot_copy=None;return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}})}source.begin_close();let step=source.close_step(grant)?;return Ok(Step::Pending{progress:step.progress()})}
        if let Some(source)=self.state.pending_tick_snapshot_retirement.as_mut(){let step=source.step(grant)?;if source.terminal_is_empty(){self.state.pending_tick_snapshot_retirement=None}return Ok(Step::Pending{progress:step.progress()})}
        if let Some(source)=self.state.pending_tick_snapshot.take(){self.state.pending_tick_snapshot_retirement=Some(ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!()));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if self.state.pending_tick_snapshot_origin.take().is_some(){return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if let Some(host)=self.state.retiring_host.as_mut(){
            let result=host.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,host.terminal_is_empty(),"Flow session original pending Host"));
            if host.terminal_is_empty(){self.state.retiring_host=None;}
            return match result{Ok(step)=>if self.terminal_is_empty(){Ok(Step::Complete{progress:step.progress()})}else{Ok(Step::Pending{progress:step.progress()})},Err(error)=>Err(error)}
        }
        if let Some(host)=self.state.pending_host.take(){self.state.retiring_host=Some(FlowHostRetirement::new(host));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
        if let Some(port)=self.retiring_geometry_port.as_mut(){
            let result=port.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.terminal_is_empty(),"Flow session original geometry port"));
            if port.terminal_is_empty(){self.retiring_geometry_port=None;}
            return match result{Ok(step)=>Ok(Step::Pending{progress:step.progress()}),Err(error)=>Err(error)}
        }
        if !self.retirement.terminal_is_empty(){
            return match self.retirement.step(grant){Ok(step)=>if self.terminal_is_empty(){Ok(Step::Complete{progress:step.progress()})}else{Ok(Step::Pending{progress:step.progress()})},Err(error)=>Err(error)}
        }
        if let Some(port)=self.geometry_port.take(){self.retiring_geometry_port=Some(semio_framework_os_flow_geometry::GeometryPortRetirement::new(port));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..empty}})}
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
            self.close_step(grant).expect("cold session original owner refused");
        }
        assert!(self.terminal_is_empty(),"cold session disposal did not reach original terminal ownership");
    }

    pub fn terminal_is_empty(&self)->bool{self.closing&&self.state.pending_snapshot_copy.is_none()&&self.state.pending_host.is_none()&&self.state.retiring_host.is_none()&&self.geometry_port.is_none()&&self.retiring_geometry_port.is_none()&&!self.owns_session_source()&&self.retirement.terminal_is_empty()}
}
