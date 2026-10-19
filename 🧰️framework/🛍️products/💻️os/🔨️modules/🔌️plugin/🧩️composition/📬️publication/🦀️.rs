//! 📬️ Pending artifact store and child group publications.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
use semio_framework::kernel::{KernelMutation, UndoGroup};
use semio_framework::Fault;
use semio_framework_value_derive::ToValue;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::ManuallyDrop;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_decl_app::tool_contract::ArtifactBoundedToolFault;
use semio_framework_plugin_decl_app::tool_contract::TYPED_OPERATION_FAULT_BYTES;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;



#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedTypedCommandFullOperationStage {
    Worker,
    Publishing,
    AwaitingAck,
    Retiring,
}

#[doc(hidden)]
pub enum PendingArtifactStorePublication<A: ArtifactApp> {
    Artifact(store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>),
    Config(store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Config, A::ConfigMutation>),
    Draft(store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Draft, A::DraftMutation>),
    Presence(store::ArtifactEphemeralOneItemPublication<A::Presence, A::PresenceMutation>),
    Transient(store::ArtifactEphemeralOneItemPublication<A::Transient, A::TransientMutation>),
    WindowConfig(Box<dyn semio_framework_plugin_window_config::ErasedWindowConfigPublication>),
    WindowTransient(Box<dyn semio_framework_plugin_window_transient::window_transient::ErasedWindowTransientPublication>),
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, ToValue,semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ChildPublicationResultV1 {
    pub invocation_id: String,
    pub committed_members: usize,
    pub child_content_generation: u64,
}

/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): what ONE composed gesture's linearized
/// publication actually produced — the parent's `KernelMutation`s under its own
/// `ArtifactHandle` PLUS one per touched child under the child's content-addressed handle, and
/// the single `UndoGroup` whose `member_edits` name every document the gesture committed to.
///
/// 🎯️ Why it is retained instead of returned: on the migrated route `dispatch_typed` only
/// ADMITS, so the composed dispatch happens turns later inside the publication ladder, long
/// after the caller's `InvocationResult` was handed back. `dispatch_emit_group` builds this
/// whole thing and the ladder used to keep three scalars of it
/// (`ChildPublicationResultV1`) and DROP the rest, so the child lane of a composite gesture
/// existed in the documents and in history but was invisible to every host-visible result
/// (`📓️fp10-plugin-lib-and-lanes.md` §2.3). It now rides the same bounded host outbox the
/// effect, event, UI-scope and terminal-completion lanes ride, and is drained by the app's own
/// close ladder.
///
/// 🚫️ Deliberately NOT counted by `has_pending_typed_operations`: the composed result is
/// published in the SAME unit that queues the `Child` result page, so the operation's own ACK
/// already keeps the actor runnable until a host that wants it has had its turn to take it — and
/// a driver that loops until nothing is pending while never reading this lane (the live-cleanup
/// pump does exactly that) would otherwise spin forever on a lane it does not own.
pub struct ComposedGestureResult {
    pub operation: u64,
    pub mutations: Vec<KernelMutation>,
    pub inverse_group: UndoGroup,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PendingChildGroupPublicationPhase {
    Ready,
    Dispatching,
    Committed,
    Acknowledged,
    Faulted,
    Closing,
}

#[doc(hidden)]
pub struct PendingChildGroupPublication<A: ArtifactApp> {
    pub artifact_mutations: ManuallyDrop<Vec<A::Mutation>>,
    pub child_emits: ManuallyDrop<Vec<ChildEmit>>,
    pub transaction: ManuallyDrop<Option<protocol::TransactionRef>>,
    pub receipt: ManuallyDrop<Option<ChildPublicationResultV1>>,
    pub mutation_retirement: Option<ControlledRetirement<Vec<A::Mutation>>>,
    transaction_retirement: Option<ControlledRetirement<protocol::TransactionRef>>,
    receipt_retirement: Option<ControlledRetirement<ChildPublicationResultV1>>,
    pub fault: Option<ArtifactBoundedToolFault>,
    pub phase: PendingChildGroupPublicationPhase,
}

impl<A: ArtifactApp> PendingChildGroupPublication<A> {
    pub fn new(artifact_mutations: Vec<A::Mutation>, child_emits: Vec<ChildEmit>, transaction: Option<protocol::TransactionRef>) -> Result<Self, Fault> {
        if child_emits.is_empty() {
            return Err(plugin_sdk_fault("retained child-group publication requires at least one exact child output"));
        }
        Ok(Self { artifact_mutations:ManuallyDrop::new(artifact_mutations), child_emits:ManuallyDrop::new(child_emits), transaction:ManuallyDrop::new(transaction), receipt:ManuallyDrop::new(None), mutation_retirement:None,transaction_retirement:None,receipt_retirement:None,fault:None,phase:PendingChildGroupPublicationPhase::Ready })
    }

    pub fn begin_dispatch(&mut self) -> Result<(), Fault> {
        if self.phase != PendingChildGroupPublicationPhase::Ready {
            return Err(plugin_sdk_fault("retained child-group publication lost its single dispatch authority"));
        }
        self.phase = PendingChildGroupPublicationPhase::Dispatching;
        Ok(())
    }

    pub fn commit(&mut self, receipt: ChildPublicationResultV1) -> Result<(), Fault> {
        if self.phase != PendingChildGroupPublicationPhase::Dispatching || self.receipt.is_some() || self.fault.is_some() {
            return Err(plugin_sdk_fault("retained child-group publication attempted a duplicate commit"));
        }
        *self.receipt = Some(receipt);
        self.phase = PendingChildGroupPublicationPhase::Committed;
        Ok(())
    }

    pub fn reject(&mut self, fault: &Fault) {
        self.fault = Some(ArtifactBoundedToolFault::from_fault(fault));
        self.phase = PendingChildGroupPublicationPhase::Faulted;
    }

    pub fn reject_and_fault(&mut self, fault: &Fault) -> Fault {
        self.reject(fault);
        self.fault().expect("rejected child publication retains one bounded fault")
    }

    pub fn fault(&self) -> Option<Fault> {
        self.fault.as_ref().map(|fault| ArtifactBoundedToolFault { bytes: fault.bytes, len: fault.len }.into_fault())
    }

    pub fn acknowledge(&mut self) -> bool {
        if self.phase != PendingChildGroupPublicationPhase::Committed {
            return false;
        }
        self.phase = PendingChildGroupPublicationPhase::Acknowledged;
        true
    }

    pub fn begin_close(&mut self) {
        self.phase = PendingChildGroupPublicationPhase::Closing;
    }

    pub fn controlled_demands<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{
        if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original pending child depth overflow"))?;demand})
    }
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(child)=self.child_emits.last(){let mut demand=child.retirement_demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original pending child metadata depth overflow"))?;return Ok(demand);}
        if self.child_emits.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.child_emits.capacity()*size_of::<ChildEmit>(),depth:1,..Default::default()});}
        if let Some(owner)=self.mutation_retirement.as_ref(){return Self::controlled_demands(owner,body);}
        if self.artifact_mutations.capacity()!=0{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.transaction_retirement.as_ref(){return Self::controlled_demands(owner,body);}
        if self.transaction.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.receipt_retirement.as_ref(){return Self::controlled_demands(owner,body);}
        Ok(RetirementDemand{depth:usize::from(self.receipt.is_some()||self.fault.is_some()),..Default::default()})
    }
    fn controlled_step<T:semio_framework_value::retirement::RetireOwned>(slot:&mut Option<ControlledRetirement<T>>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{
        let owner=slot.as_mut().expect("observed original child owner");
        if owner.terminal_is_empty(){slot.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original pending child owner")).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;
        Ok(PluginLifecycleStep::Progress(step.progress()))
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{
        let idle=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(PluginLifecycleStep::Complete(idle));}
        if self.phase!=PendingChildGroupPublicationPhase::Closing{return Ok(PluginLifecycleStep::Blocked{reason:"original child publication is not admitted for closure"});}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(idle));}
        if let Some(child)=self.child_emits.last_mut(){
            if child.terminal_is_empty(){self.child_emits.pop();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
            let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=child.close_one(child_grant).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant,step,child.terminal_is_empty(),"original pending child emit").map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.child_emits.capacity()!=0{*self.child_emits=Vec::new();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..idle}));}
        if self.mutation_retirement.is_some(){return Self::controlled_step(&mut self.mutation_retirement,grant);}
        if self.artifact_mutations.capacity()!=0{
            let original=std::mem::take(&mut*self.artifact_mutations);match ControlledRetirement::new(original){Ok(owner)=>self.mutation_retirement=Some(owner),Err((error,original))=>{*self.artifact_mutations=original;return Err(semio_framework_plugin_decl_emission::retirement_refusal_fault(&error));}}
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));
        }
        if self.transaction_retirement.is_some(){return Self::controlled_step(&mut self.transaction_retirement,grant);}
        if let Some(original)=self.transaction.take(){match ControlledRetirement::new(original){Ok(owner)=>self.transaction_retirement=Some(owner),Err((error,original))=>{*self.transaction=Some(original);return Err(semio_framework_plugin_decl_emission::retirement_refusal_fault(&error));}}return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
        if self.receipt_retirement.is_some(){return Self::controlled_step(&mut self.receipt_retirement,grant);}
        if let Some(original)=self.receipt.take(){match ControlledRetirement::new(original){Ok(owner)=>self.receipt_retirement=Some(owner),Err((error,original))=>{*self.receipt=Some(original);return Err(semio_framework_plugin_decl_emission::retirement_refusal_fault(&error));}}return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
        if self.fault.is_some(){self.fault=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle}));}
        Ok(PluginLifecycleStep::Complete(idle))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == PendingChildGroupPublicationPhase::Closing && self.artifact_mutations.is_empty() && self.artifact_mutations.capacity()==0&&self.child_emits.capacity()==0&&self.child_emits.is_empty()&&self.transaction.is_none()&&self.receipt.is_none()&&self.mutation_retirement.is_none()&&self.transaction_retirement.is_none()&&self.receipt_retirement.is_none()&&self.fault.is_none()
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "artifact-app-testing"))]
pub fn test_original_pending_child_group_closure<A:ArtifactApp>(original:impl FnOnce()->Vec<A::Mutation>,grant:RetainedCloneGrant){
    let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||PendingChildGroupPublication::<A>::new(original(),vec![ChildEmit::open("original","child",0)],Some(protocol::TransactionRef{id:"t".repeat(8192),tool:"original-tool".into()})).unwrap());
    let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let pointer=owner.artifact_mutations.as_ptr();owner.begin_close();let mut turns=0;
    while !owner.terminal_is_empty(){
        let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
        for denied in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied).unwrap());assert_eq!(step.progress().unwrap_or_default(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if owner.mutation_retirement.is_none()&&owner.artifact_mutations.capacity()!=0{assert_eq!(owner.artifact_mutations.as_ptr(),pointer);}}
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());let progress=step.progress().unwrap_or_default();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,PluginLifecycleStep::Complete(_)){assert!(owner.terminal_is_empty());}born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);
    }
    assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

impl<A:ArtifactApp> Drop for PendingChildGroupPublication<A>{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original child publication requires granted physical closure");}
}

impl<A: ArtifactApp> PendingArtifactStorePublication<A> {
    pub fn acknowledge(&mut self) -> bool {
        match self {
            Self::Artifact(publication) => publication.acknowledge(),
            Self::Config(publication) => publication.acknowledge(),
            Self::Draft(publication) => publication.acknowledge(),
            Self::Presence(publication) => publication.acknowledge(),
            Self::Transient(publication) => publication.acknowledge(),
            Self::WindowConfig(publication) => publication.acknowledge(),
            Self::WindowTransient(publication) => publication.acknowledge(),
        }
    }

    pub fn begin_close(&mut self) {
        match self {
            Self::Artifact(publication) => publication.begin_close(),
            Self::Config(publication) => publication.begin_close(),
            Self::Draft(publication) => publication.begin_close(),
            Self::Presence(publication) => publication.begin_close(),
            Self::Transient(publication) => publication.begin_close(),
            Self::WindowConfig(publication) => publication.begin_close(),
            Self::WindowTransient(publication) => publication.begin_close(),
        }
    }

    pub fn frame_release_bytes(&self)->usize{match self{Self::WindowConfig(owner)=>size_of_val(&**owner),Self::WindowTransient(owner)=>size_of_val(&**owner),_=>0}}

    pub fn retirement_demands(&self, body:usize)->Result<RetirementDemand,ValueError>{
        match self {
            Self::Artifact(publication)=>publication.retirement_demands(body),
            Self::Config(publication)=>publication.retirement_demands(body),
            Self::Draft(publication)=>publication.retirement_demands(body),
            Self::Presence(publication)=>publication.retirement_demands(body),
            Self::Transient(publication)=>publication.retirement_demands(body),
            Self::WindowConfig(publication)=>publication.retirement_demands(body),
            Self::WindowTransient(publication)=>publication.retirement_demands(body),
        }
    }

    pub fn close_step(&mut self, grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let one_item=store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth};
        match self {
            Self::Artifact(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::Config(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::Draft(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::Presence(publication)=>publication.close_step(one_item).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::Transient(publication)=>publication.close_step(one_item).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::WindowConfig(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
            Self::WindowTransient(publication)=>publication.close_step(grant).map_err(|error|plugin_sdk_fault(error.into_message())),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Artifact(publication) => publication.terminal_is_empty(),
            Self::Config(publication) => publication.terminal_is_empty(),
            Self::Draft(publication) => publication.terminal_is_empty(),
            Self::Presence(publication) => publication.terminal_is_empty(),
            Self::Transient(publication) => publication.terminal_is_empty(),
            Self::WindowConfig(publication) => publication.terminal_is_empty(),
            Self::WindowTransient(publication) => publication.terminal_is_empty(),
        }
    }

    /// 🏷️ The one result lane this pending publication answers under, paired with the store label
    /// its own retirement faults are worded with.
    pub fn lane(&self) -> (TypedOperationResultLane, &'static str) {
        match self {
            Self::Artifact(_) => (TypedOperationResultLane::Artifact, "artifact-store"),
            Self::Config(_) => (TypedOperationResultLane::Config, "config-store"),
            Self::Draft(_) => (TypedOperationResultLane::Draft, "draft-store"),
            Self::Presence(_) => (TypedOperationResultLane::Presence, "presence-store"),
            Self::Transient(_) => (TypedOperationResultLane::Transient, "transient-store"),
            Self::WindowConfig(_) => (TypedOperationResultLane::WindowConfig, "window-config"),
            Self::WindowTransient(_) => (TypedOperationResultLane::WindowTransient, "window-transient"),
        }
    }

    /// ♻️ Whether the owning store already moved this publication into its retirement phase.
    pub fn is_closing(&self) -> bool {
        let phase = match self {
            Self::Artifact(publication) => publication.phase(),
            Self::Config(publication) => publication.phase(),
            Self::Draft(publication) => publication.phase(),
            Self::Presence(publication) => publication.phase(),
            Self::Transient(publication) => publication.phase(),
            Self::WindowConfig(publication) => publication.phase(),
            Self::WindowTransient(publication) => publication.phase(),
        };
        matches!(phase, store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Closing | store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Complete)
    }

    /// 💥️ The owning store's own rejection reason, for as long as the retirement still retains it.
    pub fn fault(&self) -> Option<&str> {
        match self {
            Self::Artifact(publication) => publication.fault(),
            Self::Config(publication) => publication.fault(),
            Self::Draft(publication) => publication.fault(),
            Self::Presence(publication) => publication.fault(),
            Self::Transient(publication) => publication.fault(),
            Self::WindowConfig(publication) => publication.fault(),
            Self::WindowTransient(publication) => publication.fault(),
        }
    }

    /// 🚫️ Borrows the actual retained failure across the exact publication lane.
    pub fn preparation_refusal(&self) -> Option<&ValueError> {
        match self {
            Self::Artifact(publication) => publication.preparation_refusal(),
            Self::Config(publication) => publication.preparation_refusal(),
            Self::Draft(publication) => publication.preparation_refusal(),
            Self::Presence(publication) => publication.preparation_refusal(),
            Self::Transient(publication) => publication.preparation_refusal(),
            Self::WindowConfig(publication) => publication.preparation_refusal(),
            Self::WindowTransient(publication) => publication.preparation_refusal(),
        }
    }

    /// ♻️ ONE bounded retirement turn of a publication already in `Closing`, shared verbatim by all
    /// seven store lanes.
    ///
    /// An incomplete step is [`PendingArtifactStorePublicationRetirement::Retiring`] — **never** a
    /// fault — even while [`Self::fault`] is still `Some`: a rejected authority drains its reason
    /// one scalar per turn, so answering the turn with `Err` re-raised the SAME rejection on every
    /// turn of the drain and the host saw `typed-operation failed: … is retiring a rejected
    /// authority` instead of a retiring publication
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️invoke-extension-rejected-authority-2026-09-11.md`).
    /// The rejection surfaces exactly once, when the retirement is terminal.
    ///
    /// Window-transient is deliberately the lenient lane: its authority is per-window ephemeral
    /// state that the very next turn re-captures against the live generation, so a superseded
    /// publication that retired cleanly owes the host no fault at all.
    fn retirement_turn(&mut self, grant:RetainedCloneGrant)->Result<PendingArtifactStorePublicationRetirement,Fault>{
        let(lane,label)=self.lane();let rejected=self.fault().is_some();
        match self.close_step(grant)?{
            RetainedCloneStep::Complete(_) if !self.terminal_is_empty()=>Err(plugin_sdk_fault(format!("{label} publication closed without terminal emptiness"))),
            RetainedCloneStep::Complete(_) if rejected&&lane!=TypedOperationResultLane::WindowTransient=>Ok(PendingArtifactStorePublicationRetirement::Rejected(plugin_sdk_fault(format!("{label} publication rejected stale or cancelled authority")))),
            RetainedCloneStep::Complete(_)=>Ok(PendingArtifactStorePublicationRetirement::Retired),
            _=>Ok(PendingArtifactStorePublicationRetirement::Retiring),
        }
    }

}

/// ♻️ Outcome of ONE [`PendingArtifactStorePublication::retirement_turn`].
#[derive(Debug)]
enum PendingArtifactStorePublicationRetirement {
    /// 🔁️ The publication owes more retirement turns; the operation keeps its slot and yields.
    Retiring,
    /// ✅️ The publication retired terminal-empty and owes the host nothing.
    Retired,
    /// 💥️ The publication retired terminal-empty after its store rejected it; the operation fails.
    Rejected(Fault),
}

/// 🧮️ Prices the original native Arc frame without creating a replacement ownership wrapper.
#[doc(hidden)]
pub fn original_reserved_arc_frame_bytes<T>()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("original reserved Arc layout").0.pad_to_align().size()}

/// 🧾️ Copies a bounded original fault while the checked-out descriptor keeps the native payload immovable.
#[doc(hidden)]
pub struct MountedWorkerFaultCapture {
    pub original: usize,
    pub expected: usize,
    pub page: usize,
    pub offset: usize,
    pub bounded: ArtifactBoundedToolFault,
}

/// 🧮️ Reports the next original copy or metadata unit without creating a replacement fault.
#[doc(hidden)]
pub fn original_fault_capture_demands(capture:&Option<MountedWorkerFaultCapture>,detail:&semio_framework_job::RetainedJobPayload)->Result<RetirementDemand,ValueError> {
    if detail.len()>TYPED_OPERATION_FAULT_BYTES||capture.as_ref().is_some_and(|owner|owner.original!=detail as*const _ as usize||owner.expected!=detail.len()){
        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"bounded fault capture requires its same authored native extent"));
    }
    let copied=usize::from(capture.as_ref().is_some_and(|owner|owner.bounded.len<owner.expected));
    Ok(RetirementDemand{copy_bytes:copied,depth:1,..Default::default()})
}

/// 🧾️ Yields one paid original payload copy before the separate inline custody transfer.
#[doc(hidden)]
pub fn advance_original_fault_capture(capture:&mut Option<MountedWorkerFaultCapture>,detail:&semio_framework_job::RetainedJobPayload,recipient:&mut Option<ArtifactBoundedToolFault>,grant:RetainedCloneGrant)->PluginLifecycleStep {
    let idle=RetainedCloneProgress::default();
    let demand=match original_fault_capture_demands(capture,detail){Ok(demand)=>demand,Err(_)=>return PluginLifecycleStep::Blocked{reason:"original bounded fault quote refused its native payload"}};
    if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return PluginLifecycleStep::Progress(idle);}
    if recipient.is_some(){return PluginLifecycleStep::Blocked{reason:"original bounded worker fault recipient is occupied"};}
    if detail.len()>TYPED_OPERATION_FAULT_BYTES{return PluginLifecycleStep::Blocked{reason:"original worker fault exceeds its authored bounded publication extent"};}
    let actual=(0..detail.page_count()).try_fold(0usize,|length,index|detail.page(index).and_then(|page|length.checked_add(page.len())));
    if actual!=Some(detail.len()){return PluginLifecycleStep::Blocked{reason:"original worker fault native pages disagree with their extent"};}
    let original=detail as*const _ as usize;
    let Some(owner)=capture.as_mut()else{
        *capture=Some(MountedWorkerFaultCapture{original,expected:detail.len(),page:0,offset:0,bounded:ArtifactBoundedToolFault{bytes:[0;TYPED_OPERATION_FAULT_BYTES],len:0}});
        return PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle});
    };
    if owner.original!=original||owner.expected!=detail.len(){return PluginLifecycleStep::Blocked{reason:"bounded capture requires the same original native worker fault"};}
    if owner.bounded.len==owner.expected{
        *recipient=Some(capture.take().unwrap().bounded);
        return PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..idle});
    }
    let copied=grant.maximum_copy_bytes.min(owner.expected-owner.bounded.len);
    if copied==0{return PluginLifecycleStep::Progress(idle);}
    let mut remaining=copied;
    while remaining!=0{
        let page=detail.page(owner.page).expect("validated original bounded fault page");
        if owner.offset==page.len(){owner.page+=1;owner.offset=0;continue;}
        let count=remaining.min(page.len()-owner.offset);
        owner.bounded.bytes[owner.bounded.len..owner.bounded.len+count].copy_from_slice(&page[owner.offset..owner.offset+count]);
        owner.bounded.len+=count;owner.offset+=count;remaining-=count;
    }
    PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..idle})
}

#[path = "🤝️group/🦀️.rs"]
pub mod group;

#[cfg(test)]
use semio_framework_plugin_decl_emission as app_emission;
#[cfg(test)]
use semio_framework_plugin_decl_emission::child_emit_preparation;
#[cfg(test)]
use crate::group as child_content;
#[cfg(test)]
#[path="../../🧪️tests/🧩️private-child-metadata/🦀️.rs"]
mod private_child_metadata_laws;
#[cfg(test)]
#[global_allocator]
static CHILD_METADATA_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
