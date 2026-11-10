//! 🧵️ Shared typed Flow ownership frontiers for resumable copying and retirement.

use crate::{neural, FlowArtifact, FlowHostComponents, FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, NodeChrome, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement, retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value_retirement_queue::RetirementQueue;

//#region 📑️SelectedCopy
#[path = "🧬️fields/🦀️.rs"]
mod fields;
//#endregion 📑️SelectedCopy

//#region 🧹️TypedRetirement
#[derive(semio_framework_value::RetireOwned)]
pub enum FlowOwner {
    Spec(SynapseSpec),Neuron(neural::Neuron),Synapse(neural::Synapse),Preview(FlowPreviewGui),Channel(crate::FlowChannelRef),
    Bytes(Vec<u8>),Strings(Vec<String>),Set(OrderedSet),Dictionary(neural::Dictionary),Value(neural::Value),
    HostSnapshot(FlowHostComponents),Widget(Widget),Widgets(Vec<Widget>),Specs(Vec<SynapseSpec>),Layouts(OrderedMap<WidgetLayout>),
    Tree(neural::Tree),Neurons(Vec<neural::Neuron>),Synapses(Vec<neural::Synapse>),Gui(FlowGui),Nodes(OrderedMap<FlowNodeGui>),
    Previews(Vec<FlowPreviewGui>),Layout(Vec<FlowLayoutEntry>),Chrome(NodeChrome),
}

const FLOW_COLD_CLOSE_STEPS:usize=1_000_000;
/// 🧊️ Runs one owner-less cold close step inside a pump retirement turn; `None` means its work quote was not admitted.
pub fn cold_effect<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Option<R> {
    semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant).expect("cold Flow retirement turn").with_effect(run).expect("cold Flow retirement effect").map(|(output,_)|output)
}
fn grant_of(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

#[must_use="Flow ownership must close every original owner and admitted scaffold"]
pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue,progress:RetainedCloneProgress}
impl Default for FlowRetirement {fn default()->Self {Self {root:None,queue:RetirementQueue::default(),progress:Default::default()}}}
impl FlowRetirement {
    pub fn from_owner(owner:FlowOwner)->Self {let mut retirement=Self::default();retirement.push(owner).unwrap_or_else(|_|unreachable!("empty Flow root"));retirement}
    pub fn push(&mut self,owner:FlowOwner)->Result<(),FlowOwner> {if self.root.is_some(){return Err(owner);}self.root=Some(ControlledRetirement::new(owner).unwrap_or_else(|(error,_)|panic!("Flow owner retirement refused: {error}")));Ok(())}
    pub fn text(&mut self,text:String)->Result<(),FlowOwner> {self.push(FlowOwner::Bytes(text.into_bytes()))}
    /// 🪑️ Quotes the next original queue backing page a reserving push must fund.
    pub fn reserve_demand(&self)->Result<RetirementDemand,ValueError> {self.queue.reserve_demand()}
    /// 🪜️ Quotes the exact admission of one displaced owner into the root or the reserved queue slot.
    pub fn admission_demand(&self,owner:&FlowOwner)->Result<RetirementDemand,ValueError> {
        if self.root.is_none(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !self.queue.has_reserved_slot(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flow retirement requires an admitted original queue slot"))}
        self.queue.admit_owned_demand(owner)
    }
    /// 🪑️ Reserves one original queue slot under the caller's effect; a refused or unfunded quote keeps all state.
    pub fn reserve_push(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError> {self.queue.reserve_step(effect)}
    /// 🎟️ Admits one displaced owner under the caller's effect; refusal or an unfunded quote leaves the owner with the caller.
    pub fn admit_owner(&mut self,original:&mut Option<FlowOwner>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {
        if self.root.is_some(){return self.queue.admit_owned(original,effect)}
        let grant=effect.grant();
        if grant.maximum_items==0{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Flow handoff requires an admitted item")),work:0}}
        if grant.maximum_depth==0{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Flow handoff requires admitted depth")),work:0}}
        let Some(owner)=original.take()else{return RetainedWorkOutcome{result:Err(ValueError::invariant("Flow handoff owner is absent")),work:0}};
        self.root=Some(ControlledRetirement::retain(owner));
        let work=match effect.performed(1){Ok(true)=>1,_=>0};
        RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()})),work}
    }
    pub fn push_cold(&mut self,owner:FlowOwner)->Result<(),ValueError>{
        if self.root.is_none(){return self.push(owner).map_err(|_|ValueError::invariant("empty Flow root admission refused"));}
        while !self.queue.has_reserved_slot(){let grant=grant_of(self.queue.reserve_demand()?);cold_effect(grant,|effect|self.queue.reserve_step(effect)).ok_or_else(||ValueError::invariant("cold Flow reservation quote refused"))?.result?;}
        let grant=grant_of(self.queue.admit_owned_demand(&owner)?);let mut original=Some(owner);
        cold_effect(grant,|effect|self.queue.admit_owned(&mut original,effect)).ok_or_else(||ValueError::invariant("cold Flow admission quote refused"))?.result?;
        if original.is_some(){return Err(ValueError::invariant("cold Flow admission denied after reservation"))}Ok(())
    }
    pub fn terminal_is_empty(&self)->bool {self.root.is_none()&&self.queue.terminal_is_empty()}
    /// 🎯️ Quotes the next original close action for the admitted body.
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(root)=self.root.as_ref(){return root.next_demand(body)}
        self.queue.next_demand(body)
    }
    pub fn step_progress(&self)->RetainedCloneProgress {self.progress}
    pub fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        self.progress=Default::default();
        let Some(root)=self.root.as_mut()else{let poll=self.queue.step(body,effect);self.progress=self.queue.step_progress();return poll};
        let grant=effect.grant();
        let poll=root.step(body,effect);self.progress=root.step_progress();
        let RetainedWorkPoll::Complete(turn)=poll else{return poll};
        let empty=root.terminal_is_empty();let work=turn.work;
        let result=turn.result.and_then(|step|admit_retained_clone_close(grant,step,empty,"Flow original root").map_err(Into::into));
        if empty{self.root=None;}
        let done=self.terminal_is_empty();
        RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map(|step|if done{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())}),work})
    }
    pub fn retire_cold(mut self) {
        for _ in 0..FLOW_COLD_CLOSE_STEPS{
            if self.terminal_is_empty(){return}
            let copy=4096.max(self.demands(0).expect("cold Flow demand").copy_bytes);let demand=self.demands(copy).expect("cold Flow demand");
            let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..grant_of(demand)};
            let poll=cold_effect(grant,|effect|self.step(copy,effect)).expect("cold Flow retirement quote refused");
            let RetainedWorkPoll::Complete(turn)=poll else{panic!("cold Flow retirement stayed pending")};
            turn.result.expect("cold Flow retirement");
        }
        panic!("cold Flow retirement did not terminate");
    }
}
impl Drop for FlowRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Flow retirement abandoned original ownership");}}

impl Widget {
    /// 🧊️ Explicit cold-only disposal of a detached widget.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Widget(self)).retire_cold();
    }
}

impl FlowGui {
    /// 🧊️ Explicit cold-only disposal of a detached gui projection — its `nodes` are an
    /// `OrderedMap<FlowNodeGui>` that refuses a bare drop.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Gui(self)).retire_cold();
    }
}

impl FlowArtifact {
    /// 🧊️ Explicit cold-only disposal of a detached artifact projection. `FlowHost::document()`
    /// hands back an OWNED projection built by `FlowHostSnapshot::to_artifact`, and both of its halves
    /// refuse a bare drop — the `Tree`'s neurons own `Dictionary` params, the `FlowUi`'s `nodes` are
    /// an `OrderedMap` — so a caller that only reads it still has to close it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Tree(self.tree)).retire_cold();
        self.ui.retire_cold();
    }
}
