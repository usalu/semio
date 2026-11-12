//! 🏁️ Physical close ladder of a retained `FlowHost` and its cold disposal helpers.

use super::*;

/// 🏠️ Retains the whole original host before any independently admitted close work.
#[doc(hidden)]
pub struct FlowHostRetirementState {
    pub(super) source:Option<FlowHost>,
    pub geometry:Option<semio_os_flow_geometry::GeometryPortRetirement>,
    payload:Option<ControlledRetirement<FlowHostPayload>>,
}

/// 🔒️ Every original host field remains owned until its declared physical close authority completes.
#[must_use="the original host must reach physical terminal emptiness"]
pub struct FlowHostRetirement {state:std::mem::ManuallyDrop<FlowHostRetirementState>}

/// 🪜️ Identifies the next original host ownership boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FlowHostClosePhase {Geometry,Dag,Domain,Neural,Widgets,Synapses,Layout,Schema,CurrentChannels,Exports,EvalJson,Catalogue,KindInfos,PreviousSnapshot,PreviousChannels,HistoryBaseline,PendingEvals,Bridges,Cache,HistoryStore,Faulted,Complete}
impl FlowHostClosePhase {pub fn is_backing(self)->bool{matches!(self,Self::Domain|Self::Neural)}}

impl FlowHostRetirement {
    /// 🪹️ Transfers the exact original source without decomposition, allocation, release or cancellation work.
    pub fn new(host:FlowHost)->Self{Self{state:std::mem::ManuallyDrop::new(FlowHostRetirementState{source:Some(host),geometry:None,payload:None})}}
    pub fn terminal_is_empty(&self)->bool{self.state.source.is_none()&&self.state.geometry.is_none()&&self.state.payload.is_none()}
    pub fn close_phase(&self)->FlowHostClosePhase{if self.state.geometry.is_some()||self.state.source.as_ref().is_some_and(|host|host.geometry_port.is_some()){FlowHostClosePhase::Geometry}else if self.state.source.as_ref().is_some_and(|host|host.history_store.is_some()){FlowHostClosePhase::HistoryStore}else if self.state.source.is_some(){FlowHostClosePhase::Dag}else if self.state.payload.is_some(){FlowHostClosePhase::Domain}else{FlowHostClosePhase::Complete}}
    pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(RetirementDemand::default())}
        if let Some(payload)=self.state.payload.as_ref(){return Ok(if payload.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{{let mut demand=payload.selected_demand(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host payload close depth overflow"))?;demand}})}
        if let Some(port)=self.state.geometry.as_ref(){return Ok(RetirementDemand{copy_bytes:port.next_copy_byte_demand()?,capacity_bytes:port.next_capacity_byte_demand(copy)?,release_bytes:port.next_release_byte_demand()?,depth:port.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host geometry close depth overflow"))?})}
        let source=self.state.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host close lost original custody"))?;
        if source.geometry_port.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(store)=source.history_store.as_ref(){if !store.close_owned_store_terminal_is_empty(){let mut demand=store.close_owned_demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"host store close depth overflow"))?;return Ok(demand)}}
        Ok(RetirementDemand{depth:1,..Default::default()})
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(payload)=self.state.payload.as_ref(){if !payload.terminal_is_empty(){return payload.next_copy_byte_demand()}}Ok(self.close_demands(0)?.copy_bytes)}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    /// 🎟️ Uses the original supplied currencies and preserves the full actual child receipt.
    pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let grant=effect.grant();
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"host close exceeds admitted depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(port)=self.state.geometry.as_mut(){
            if port.terminal_is_empty(){self.state.geometry=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let step=port.close_step(effect)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,port.terminal_is_empty(),"original host geometry retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        if let Some(payload)=self.state.payload.as_mut(){
            if payload.terminal_is_empty(){self.state.payload=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=crate::poll_close_step(payload.step(child.maximum_copy_bytes,&mut effect.nested(child)))?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,payload.terminal_is_empty(),"original host payload retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let source=self.state.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host admitted close lost original source"))?;
        if let Some(port)=source.geometry_port.take(){self.state.geometry=Some(semio_os_flow_geometry::GeometryPortRetirement::new(port));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if let Some(store)=source.history_store.as_mut(){
            if store.close_owned_store_terminal_is_empty(){source.history_store=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=match store.close_owned_store_step(&mut effect.nested(child)){semio_framework_value::RetainedWorkPoll::Pending{..}=>RetainedCloneStep::Progress(empty),semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error)?};
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,store.close_owned_store_terminal_is_empty(),"original host history store retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let payload=FlowHostPayload::from_host(self.state.source.take().unwrap());self.state.payload=Some(ControlledRetirement::new(payload).unwrap_or_else(|_|unreachable!("all original host payload fields declare typed retirement")));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
}
impl Drop for FlowHostRetirement {
    fn drop(&mut self){if !self.terminal_is_empty(){assert!(std::thread::panicking(),"Flow host retirement abandoned its original source");return}unsafe{std::mem::ManuallyDrop::drop(&mut self.state)}}
}

impl FlowHost {
    /// 🧊️ Explicit cold-only disposal of a detached host — the twin of [`FlowHostSnapshot::retire_cold`].
    /// A `FlowHost` owns a `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` panics on a bare
    /// drop (`ordered-map root must be explicitly retired before drop`), so a host is CLOSED, never
    /// dropped. Retained callers drive [`FlowHostRetirement::retire_step`] under their own grant
    /// instead; this drains the same ladder in one uninterrupted cold pass.
    pub fn retire_cold(self)->Result<(),FlowHostCloseRefusal>{
        let mut retirement=FlowHostRetirement::new(self);
        match Self::drain_cold(&mut retirement){Ok(())=>Ok(()),Err(error)=>Err(FlowHostCloseRefusal{error,retirement})}
    }

    fn drain_cold(retirement:&mut FlowHostRetirement)->Result<(),ValueError>{
        let mut idle=0usize;while !retirement.terminal_is_empty(){let copy=retirement.next_close_copy_byte_demand()?.max(4096);let demand=retirement.close_demands(copy)?;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let step=retirement.close_step_cold(grant)?;if step.progress()==RetainedCloneProgress::default(){idle+=1;if idle>64{return Err(ValueError::invariant("cold original host close stalled at exact demand"));}}else{idle=0;}}Ok(())
    }

    /// 🧊️ Closes cold and, when the close stays refused after one retry, parks the live custody for the next close on this thread.
    pub fn retire_cold_or_park(self)->Result<(),ValueError>{
        FlowHostCloseRefusal::drain_parked();
        match self.retire_cold(){Ok(())=>Ok(()),Err(refusal)=>refusal.resolve()}
    }

    /// 📤️ Returns the exact body output alongside custody of any refused original close.
    pub fn with_cold<R>(mut self,body:impl FnOnce(&mut FlowHost)->R)->FlowHostScopeOutcome<R>{let output=body(&mut self);FlowHostScopeOutcome{output,close:self.retire_cold()}}

    /// 🏠️ Runs `body` against a host built from `fixture`, then retires that host — the ONE shape a
    /// caller that only needs a host for the length of an expression should use.
    pub fn with_host_snapshot<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> FlowHostScopeOutcome<R> {
        Self::from_host_snapshot(host_snapshot.clone()).with_cold(body)
    }
}

/// 🧳️ A refused cold close returns its exact original error and still-live physical owner.
#[must_use="the refused original retirement must remain owned until terminal closure"]
pub struct FlowHostCloseRefusal{pub error:ValueError,pub retirement:FlowHostRetirement}
impl std::fmt::Debug for FlowHostCloseRefusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("FlowHostCloseRefusal").field("error",&self.error).field("phase",&self.retirement.close_phase()).finish()}}
impl std::fmt::Display for FlowHostCloseRefusal{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"original host close refused ({:?}) at {:?}",self.error.kind,self.retirement.close_phase())}}
thread_local!{static PARKED:std::cell::RefCell<Vec<FlowHostRetirement>>=const{std::cell::RefCell::new(Vec::new())};}
impl FlowHostCloseRefusal{
    pub fn into_parts(self)->(ValueError,FlowHostRetirement){(self.error,self.retirement)}
    /// 🔁️ Drives the same refused original custody through the cold ladder once more.
    pub fn retry_cold(self)->Result<(),FlowHostCloseRefusal>{
        let Self{error:_,mut retirement}=self;
        match FlowHost::drain_cold(&mut retirement){Ok(())=>Ok(()),Err(error)=>Err(Self{error,retirement})}
    }
    /// 🅿️ Retries once; a still-refused custody is parked on this thread and its refusal returned.
    pub fn resolve(self)->Result<(),ValueError>{
        match self.retry_cold(){Ok(())=>Ok(()),Err(refusal)=>{let(error,retirement)=refusal.into_parts();PARKED.with(|parked|parked.borrow_mut().push(retirement));Err(error)}}
    }
    fn drain_parked(){
        let pending=PARKED.with(|parked|std::mem::take(&mut*parked.borrow_mut()));
        for mut retirement in pending{if FlowHost::drain_cold(&mut retirement).is_err(){PARKED.with(|parked|parked.borrow_mut().push(retirement))}}
    }
    /// 🔎️ Counts custodies still parked on this thread.
    pub fn parked()->usize{PARKED.with(|parked|parked.borrow().len())}
}

/// 📦️ The body output never disappears when its original host close refuses.
#[must_use="handle both the exact body output and the original close custody"]
pub struct FlowHostScopeOutcome<R>{pub output:R,pub close:Result<(),FlowHostCloseRefusal>}

/// 🎟️ One boxed original host close source, funded and retained by its actual command job: the boxed host is released and the boxed retirement born under receipted capacity, so a command work frame never carries either inline.
#[derive(Default)]
pub struct FlowHostCloseSlot{owner:Option<Box<FlowHostRetirement>>}
impl FlowHostCloseSlot{
 pub fn intake_demand()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<FlowHostRetirement>>>(),capacity_bytes:std::mem::size_of::<FlowHostRetirement>(),release_bytes:std::mem::size_of::<FlowHost>(),depth:1}}
 pub fn retain(&mut self,original:&mut Option<Box<FlowHost>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.owner.is_some(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original host close slot is occupied"));}
  if original.is_none(){return Ok(Default::default());}let demand=Self::intake_demand();
  if !Self::admitted(demand,grant){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original host close intake requires its quoted currencies"));}
  let host=*original.take().unwrap();self.owner=Some(Box::new(FlowHostRetirement::new(host)));Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes})
 }
 fn admitted(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
 pub fn next_demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  let Some(owner)=self.owner.as_ref()else{return Ok(Default::default())};
  if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<FlowHostRetirement>>>(),release_bytes:std::mem::size_of::<FlowHostRetirement>(),depth:1,..Default::default()});}
  let mut demand=owner.close_demands(copy)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original host close slot depth overflow"))?;Ok(demand)
 }
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let grant=effect.grant();
  if self.owner.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.next_demand(grant.maximum_copy_bytes)?;if !Self::admitted(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if self.owner.as_ref().unwrap().terminal_is_empty(){self.owner=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let owner=self.owner.as_mut().unwrap();let step=owner.close_step(&mut effect.nested(child))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original command host close")?;Ok(RetainedCloneStep::Progress(step.progress()))
 }
 pub fn terminal_is_empty(&self)->bool{self.owner.is_none()}
 /// 🧊️ [`Self::close_step`] outside any Job turn inside a host-pump retirement turn funded by `grant`.
 pub fn close_step_cold(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  match crate::cold_effect(grant,|effect|self.close_step(effect))?{Some(done)=>done,None=>Ok(RetainedCloneStep::Progress(Default::default()))}
 }
}

impl FlowHostRetirement{
 /// 🧊️ [`Self::close_step`] outside any Job turn inside a host-pump retirement turn funded by `grant`.
 pub fn close_step_cold(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  match crate::cold_effect(grant,|effect|self.close_step(effect))?{Some(done)=>done,None=>Ok(RetainedCloneStep::Progress(Default::default()))}
 }
}

#[cfg(any(test,feature="testkit"))]
impl FlowHostRetirement{
 /// 🔬️ Identifies the original retained text allocations before its first physical close event.
 pub fn test_source_text_identity(&self)->Option<(*const u8,*const u8)>{let source=self.state.source.as_ref()?;Some((source.host_catalogue_json.as_ptr(),source.edited_note.as_ref()?.as_ptr()))}
}
