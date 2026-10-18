//! 📨️ Invocation origin, reply and snapshot intake of the evaluation session: the retained invocation origins and replies, the neural cache and the tick snapshot intake steps. See [`super`].

use super::*;

impl FlowEvalSession {
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
}
