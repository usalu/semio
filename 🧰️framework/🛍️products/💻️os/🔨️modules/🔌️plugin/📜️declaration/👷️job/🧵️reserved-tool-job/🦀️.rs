//! 🧵️ Reserved tool job of the tool job contract: the owned job request, the reserved input and request, the reserved job trait and state and the reserved tool job with its interactive job and drop implementations. See [`super`].

use super::*;
use crate::ArtifactInstanceOperationOwnerHandle;

#[path="../🎟️request/🔒️command/🦀️.rs"]
mod original_command;
pub use original_command::ArtifactOriginalToolCommand;

/// 🎒️ Fully owned input handed to an app-defined factory builder after exact raw admission.
pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
    pub source_binding: semio_framework::action_bus::OriginalToolSourceBinding,
    pub retained:RetainedCloneGrant,
    pub command: ArtifactOriginalToolCommand<A::Command>,
    pub input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    pub checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    pub operation: semio_framework_job::Operation,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub decoded_items: usize,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🔐️ Exact immutable store authority retained by producers until their private owners close.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub window_config: Option<WindowConfigSnapshot>,
    pub history: std::sync::Arc<HistoryView>,
    pub interaction_state: std::sync::Arc<protocol::InteractionState>,
    pub interaction_hover: std::sync::Arc<InteractionHoverState>,
    pub context: ArtifactOwnedContextHandle<A>,
    pub instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: ArtifactToolCompletion<A>,
}
#[path="../🎟️request/♻️retirement/🦀️.rs"]
mod original_request_retirement;


/// 🧭️ Owned input for an app-defined implementation of a framework-reserved route. The runtime
/// selects the exact owner/factory/tool/schema proof before constructing this value.
pub enum ArtifactReservedToolInput {
    Action { args: Option<DslValue>, interaction: protocol::InteractionState, hover: InteractionHoverState },
    Media { port: String, media: Media },
}

/// 🎒️ Immutable app state plus an owned reserved-route input. Implementations only construct a
/// persistent job here; producer work belongs in that job's bounded `step` calls.
pub struct ArtifactReservedToolJobRequest<A: ArtifactApp> {
    pub retained:RetainedCloneGrant,
    pub controller_id: String,
    pub tool_id: String,
    pub payload_schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub app_instance_id: u32,
    /// 🌐️ Caller-selected ephemeral presentation axes, when a host context is present.
    pub ui_axes: Option<(Locale, Terminology)>,
    pub parent_document_id: String,
    pub canonical_base_revision: [u8; 32],
    pub authoring_seed: String,
    pub snapshot: std::sync::Arc<A::Snapshot>,
    /// 🧾️ Exact immutable store authority for retained reserved-route source traversal.
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: std::sync::Arc<A::Config>,
    pub history: std::sync::Arc<HistoryView>,
    /// 🧩️ The parent's composed children, the same view `ArtifactView` hands the synchronous
    /// `A::import_media`. A composing app's LIVE scene is its children, not its own snapshot
    /// (sequence's steps live on the `content` Flow child), and the mounted route builds the emit
    /// exclusively from the resumable job — so without this a composed app could not write a
    /// resumable importer at all and every delivery died `interactive-job.missing-reserved-builder`
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, media §8). Cheap to clone
    /// (`Option<Arc<ChildContentRoot>>`); a job releases it as soon as it has decoded.
    pub children: ChildContentView,
    pub raw_wire: Vec<u8>,
    pub input: ArtifactReservedToolInput,
    pub completion: ArtifactToolCompletion<A>,
}

/// 📬️ Completed document mutations paired with their ephemeral output.
pub(super) type ArtifactToolEmission<A> = (ArtifactMutationOutcome<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, EphemeralEmit<A>);

/// 🧹️ Required disposal side of an app-owned persistent job. `Complete` is an assertion
/// that dropping the concrete job immediately afterward is constant-time; no blanket/default
/// implementation exists because an erased job may own arbitrarily deep snapshots or queues.
pub trait ArtifactReservedJob: semio_framework_job::InteractiveJob + std::any::Any {}

/// 📦️ Sized ActionBus payload around one app-owned concrete resumable state machine.
pub struct ArtifactReservedToolJobState {
    pub(super) inner:ManuallyDrop<Option<Box<dyn ArtifactReservedJob+Send>>>,
    pub(super) operation:Option<semio_framework_job::Operation>,
}

#[derive(Clone)]
pub struct ArtifactReservedToolJob {
    pub(super) state:ManuallyDrop<Option<std::sync::Arc<semio_framework_value::retirement::controlled::RetainedOwnerGate<ArtifactReservedToolJobState>>>>,
}

impl ArtifactReservedToolJob {
    pub fn new<J:ArtifactReservedJob+Send+'static>(job:J)->Self {
        Self{state:ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ArtifactReservedToolJobState{inner:ManuallyDrop::new(Some(Box::new(job))),operation:None}))))}
    }

    #[cfg(any(test, feature = "reserved-producer-testing"))]
    #[doc(hidden)]
    pub fn test_source_identity(&self)->Option<usize>{self.state.as_ref().map(|state|std::sync::Arc::as_ptr(state)as usize)}
    #[cfg(any(test, feature = "reserved-producer-testing"))]
    #[doc(hidden)]
    pub fn test_bound_operation(&self)->Option<semio_framework_job::Operation>{self.state.as_ref()?.try_lock().ok()?.operation}
    pub fn bind_operation(&mut self,operation:semio_framework_job::Operation)->Result<(),ValueError>{
        let source=self.state.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job state already returned"))?;
        let mut state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority is busy"))?;
        if state.operation.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved job operation authority was already bound"))}
        state.operation=Some(operation);Ok(())
    }

    fn close_demand(&self,kind:u8,copy:usize)->Result<usize,ValueError>{
        let Some(source)=self.state.as_ref()else{return Ok(0)};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer demand authority is borrowed"))?;
        let Some(inner)=state.inner.as_ref()else{return Ok(if state.operation.is_some(){match kind{0=>size_of::<semio_framework_job::Operation>(),3=>1,_=>0}}else{match kind{0=>size_of::<Self>(),2=>Self::original_shared_frame_bytes(),3=>1,_=>0}})};
        if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){return Ok(match kind{2=>size_of_val(&**inner),3=>1,_=>0})}
        match kind{0=>inner.next_close_copy_byte_demand(),1=>inner.next_close_capacity_byte_demand(copy),2=>inner.next_close_release_byte_demand(),_=>inner.next_close_depth_demand()}
    }

    pub fn terminal_is_empty(&self)->bool{self.state.is_none()}

    pub fn begin_close(&mut self){if let Some(source)=self.state.as_ref(){if let Ok(mut state)=source.try_lock(){if let Some(inner)=state.inner.as_mut(){inner.begin_close()}}}}
}

impl semio_framework_job::InteractiveJob for ArtifactReservedToolJob {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
        let Some(source)=self.state.as_ref()else{return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)};
        match source.try_lock(){
            Ok(mut state)=>match state.inner.as_mut(){Some(inner)=>{let inner:*mut(dyn ArtifactReservedJob+Send)=&mut**inner;drop(state);unsafe{(&mut*inner).step(cx)}},None=>JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)},
            Err(_)=>JobOutcomeBorrow::admit_yield(cx).map_err(Into::into),
        }
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
        let Some(source)=self.state.as_ref()else{return match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer state returned before outcome handback"))}};
        let state=source.try_lock().map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"reserved producer outcome authority is borrowed"))?;
        match state.inner.as_ref(){Some(inner)=>{let inner:*const(dyn ArtifactReservedJob+Send)=&**inner;drop(state);unsafe{(&*inner).borrow_outcome(descriptor)}},None=>match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"reserved producer released before its outcome borrow"))}}
    }

    fn begin_close(&mut self){ArtifactReservedToolJob::begin_close(self)}

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        let Some(source)=self.state.as_ref()else{return Ok(Step::Complete{progress:Default::default()})};
        if cx.retirement_should_yield(){return Ok(Step::Pending{progress:Default::default()})}let grant=cx.retained_grant();
        let mut state=match source.try_lock(){Ok(state)=>state,Err(_)=>return Ok(Step::Blocked)};
        if let Some(inner)=state.inner.as_mut(){
            if semio_framework_job::InteractiveJob::terminal_is_empty(&**inner){let demand=RetirementDemand{release_bytes:size_of_val(&**inner),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{drop(state.inner.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})}
            let step=semio_framework_job::InteractiveJob::close_step(&mut**inner,cx)?.admit(grant,semio_framework_job::InteractiveJob::terminal_is_empty(&**inner))?;return Ok(match step{Step::Complete{progress}=>Step::Pending{progress},step=>step})
        }
        if state.operation.is_some(){let demand=RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,demand,|_|{state.operation.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})}
        drop(state);
        let demand=RetirementDemand{copy_bytes:size_of::<Self>(),release_bytes:Self::original_shared_frame_bytes(),depth:1,..Default::default()};
        semio_framework_job::close_original_retained_step(cx,demand,|_|{let original=std::sync::Arc::into_inner(self.state.take().unwrap());let released_bytes=if let Some(original)=original{let original=original.into_inner();assert!(original.inner.is_none()&&original.operation.is_none(),"original shared producer frame still retains a body");drop(original);demand.release_bytes}else{0};Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}))})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(0,0)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.close_demand(1,copy)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(2,0)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(3,0)}
    fn terminal_is_empty(&self)->bool{ArtifactReservedToolJob::terminal_is_empty(self)}
}

impl Drop for ArtifactReservedToolJob {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original reserved producer requires its paid shared frame return")}
}
