//! 🏁️ Physical close ladder of a retained `FlowHost` and its cold disposal helpers.

use super::*;

/// 🏠️ Retains the whole original host before any independently admitted close work.
#[doc(hidden)]
pub struct FlowHostRetirementState {
    pub(super) source:Option<FlowHost>,
    pub geometry:Option<semio_framework_os_flow_geometry::GeometryPortRetirement>,
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
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"host close exceeds admitted depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(port)=self.state.geometry.as_mut(){
            if port.terminal_is_empty(){self.state.geometry=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            let step=port.close_step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,port.terminal_is_empty(),"original host geometry retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        if let Some(payload)=self.state.payload.as_mut(){
            if payload.terminal_is_empty(){self.state.payload=None;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=payload.step(child)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,payload.terminal_is_empty(),"original host payload retirement")?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let source=self.state.source.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"host admitted close lost original source"))?;
        if let Some(port)=source.geometry_port.take(){self.state.geometry=Some(semio_framework_os_flow_geometry::GeometryPortRetirement::new(port));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if let Some(store)=source.history_store.as_mut(){
            if store.close_owned_store_terminal_is_empty(){source.history_store=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=store.close_owned_store_step(child)?;
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
    pub fn retire_cold(self) {
        let mut retirement=FlowHostRetirement::new(self);
        while !retirement.terminal_is_empty(){let copy=retirement.next_close_copy_byte_demand().expect("cold host copy authority");let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_close_capacity_byte_demand(copy).expect("cold host capacity authority"),maximum_release_bytes:retirement.next_close_release_byte_demand().expect("cold host release authority"),maximum_depth:retirement.next_close_depth_demand().expect("cold host depth authority")};let step=retirement.close_step(grant).expect("cold original host close");assert!(step.progress().copied_items!=0,"cold original host close stalled at exact demand");}
    }

    /// 🏠️ Runs `body` against a host built from `fixture`, then retires that host — the ONE shape a
    /// caller that only needs a host for the length of an expression should use.
    pub fn with_host_snapshot<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
        let mut host = Self::from_host_snapshot(host_snapshot.clone());
        let result = body(&mut host);
        host.retire_cold();
        result
    }
}
