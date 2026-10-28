//! 🧩️ Pack load contract and value frames: the mounted token, the controlled demands, the pack load diagnostic, phase, progress and step, the erased pack load, the expected and built values, the value frame, the retained string and the retained typed state. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadDiagnostic {
    EnvelopeIdentity,
    Pack,
    TypedState,
    History,
    InnerIdentity,
    Replay,
    Capacity,
    Stale,
    Cancelled,
    Retirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadPhase {
    EnvelopeIdentity,
    PackIngress,
    PackReplay,
    PackRetirement,
    HistoryReplay,
    InputRetirement,
    StoreHydration,
    StoreAddress,
    Ready,
    RetiringRejectedCandidate,
    RetiringDisplacedStore,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowConfigPackLoadProgress {
    pub phase: WindowConfigPackLoadPhase,
    pub completed_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadStep {
    Pending(WindowConfigPackLoadProgress),
    OwnershipProgress(WindowConfigPackLoadProgress,RetainedCloneProgress),
    Ready,
    Rejected(WindowConfigPackLoadDiagnostic),
    Complete,
}

pub trait ErasedWindowConfigPackLoad: Send {
    fn as_any(&self)->&dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn registry_lifetime(&self) -> u64;
    fn phase(&self) -> WindowConfigPackLoadPhase;
    fn progress(&self) -> WindowConfigPackLoadProgress;
    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic>;
    fn insertion_pending(&self)->bool;
    fn advance_context(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<WindowConfigPackLoadStep,semio_framework_job::OriginalTurnFailure>;
    fn close_step_context(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>;
    fn request_cancel(&mut self);
    fn reject_stale(&mut self) -> WindowConfigPackLoadStep;
    fn demand_bytes(&self) -> usize;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct WindowConfigPackLoad {
    pub(in super::super) inner:ManuallyDrop<Option<Box<dyn ErasedWindowConfigPackLoad>>>,
    pub(super) closed_progress:WindowConfigPackLoadProgress,
    pub(super) closed_diagnostic:Option<WindowConfigPackLoadDiagnostic>,
}

impl WindowConfigPackLoad {
    #[cfg(test)]
    #[doc(hidden)]
    pub fn candidate_actor_for_test<O:WindowConfigOwner>(&mut self)->Option<protocol::ActorId>{let typed=self.inner.as_mut()?.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>()?;typed.candidate.as_ref().map(|candidate|candidate.store.local_actor_id().clone())}
    pub fn phase(&self)->WindowConfigPackLoadPhase{self.inner.as_ref().map_or(WindowConfigPackLoadPhase::Complete,|inner|inner.phase())}
    pub fn progress(&self)->WindowConfigPackLoadProgress{self.inner.as_ref().map_or(self.closed_progress,|inner|inner.progress())}
    pub fn diagnostic(&self)->Option<WindowConfigPackLoadDiagnostic>{self.inner.as_ref().map_or(self.closed_diagnostic,|inner|inner.diagnostic())}
    pub fn request_cancel(&mut self){if let Some(inner)=self.inner.as_mut(){inner.request_cancel();}}
    pub fn terminal_is_empty(&self)->bool{self.inner.is_none()}
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let Some(inner)=self.inner.as_ref()else{return Ok(Default::default());};
        if inner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(inner.as_ref()),depth:1,..Default::default()});}
        let mut demand=inner.retirement_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"window load frame depth overflow"))?;Ok(demand)
    }

    /// ♻️ Retires the exact decoder, initializer and final erased frame under the mounted original caller.
    pub fn close_step_context(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        let Some(inner)=self.inner.as_ref()else{return Ok(PluginLifecycleStep::Complete(Default::default()))};
        if !inner.terminal_is_empty(){return self.inner.as_mut().unwrap().close_step_context(cx).map(|step|match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),other=>other})}
        let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|_|{let original=self.inner.take().unwrap();self.closed_progress=original.progress();self.closed_diagnostic=original.diagnostic();drop(original);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})?;
        Ok(if self.inner.is_none(){PluginLifecycleStep::Complete(step.progress())}else{PluginLifecycleStep::Progress(step.progress())})
    }


}

impl Drop for WindowConfigPackLoad {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.inner.is_none(), "window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}
