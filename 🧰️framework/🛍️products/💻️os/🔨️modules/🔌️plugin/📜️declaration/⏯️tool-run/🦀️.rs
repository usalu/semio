//! ⏯️ The tool run contract an app builds jobs for: run job requests, ports, trace keys and the run view.
use semio_framework::Effect;
use std::collections::BTreeSet;
use std::mem::ManuallyDrop;
use crate::child_view::ChildContentView;
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin_window_config::WindowConfigSnapshot;
use semio_framework_tool_run::ToolRunDefinition;
use semio_framework_tool_run::ToolRunIdentity;
use semio_framework_tool_run::ToolRunState;
use semio_framework_tool_run::ToolRunStepRing;
use std::sync::Arc;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;

//#region 🔖️Limits
/// 🪧️ Framework-owned panel body served before any app body, next to `FRAMEWORK_HISTORY_BODY_KEY`.
pub const FRAMEWORK_TOOL_RUN_BODY_KEY: &str = "framework.body.toolRun";

//#endregion 🔖️Selection

//#region 🔖️Request
/// 🎯️ Which job of a run the app builds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolRunJobPurpose {
    Run,
    Revalidate,
}

/// 🧵️ A boxed run or revalidate job; it reports `ToolRunTick` pages via `StepOutcome::PreviewReady`. `Send`
/// explicitly: the browser target's `InteractiveJob` drops its thread-transfer bound, but the ledger lives
/// in a `PluginApp`, which stays `Send` on every target.
pub type ToolRunJob = Box<dyn semio_framework_job::InteractiveJob + Send>;

/// 🔑️ The trace key allocator every job of one run shares: keys handed out lie above every key the run's
/// ticks have upserted and every key allocated before.
#[derive(Clone, Debug, Default, semio_framework_value::RetireOwned)]
pub struct ToolRunTraceKeys {
    next: Arc<std::sync::atomic::AtomicU64>,
}

impl ToolRunTraceKeys {
    /// 🔢️ The first key neither upserted by this run nor allocated.
    pub fn next(&self) -> u64 {
        self.next.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// 🎟️ Reserves `count` consecutive unused keys.
    pub fn allocate(&self, count: u64) -> std::ops::Range<u64> {
        let start = self.next.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |next| Some(next.saturating_add(count))).unwrap_or_else(|next| next);
        start..start.saturating_add(count)
    }

    pub fn observe(&self, key: u64) {
        self.next.fetch_max(key.saturating_add(1), std::sync::atomic::Ordering::SeqCst);
    }
}

/// 🎯️ The in-place retarget hooks of the tool run job protocol (§3.3). A job built through
/// `ArtifactApp::build_retargetable_tool_run_job` stays resident across a base change and a settings change —
/// and while its run is `complete` — instead of being closed and rebuilt from its checkpoint.
pub trait ToolRunRetargetableJob<C>: semio_framework_job::InteractiveJob + Send {
    /// 🪢️ Every tick after this call carries `identity` (base changed: new generation and base revision).
    fn rebind(&mut self, identity: ToolRunIdentity);
    /// 🎚️ Retargets the job to `config` under `identity`; `false` when it cannot continue in place, and the
    /// driver then closes it and rebuilds it from its checkpoint.
    fn reconfigure(&mut self, identity: ToolRunIdentity, config: Arc<C>) -> bool;
}

/// 📮️ What a run job shares with its app instance across turns: the host effects it hands the driver (an
/// extension round trip whose answer returns as an app command cannot happen inside `step`) and the external
/// wake of a job waiting on such work. A waiting job is never stepped and keeps its instance idle, so waiting
/// costs no turn; whoever folds the awaited answer calls [`ToolRunJobPort::wake`].
#[derive(Clone, Default)]
pub struct ToolRunJobPort {
    pub inner: Arc<ToolRunJobPortState>,
}

#[derive(Default)]
pub struct ToolRunJobPortState {
    pub effects: std::sync::Mutex<std::collections::VecDeque<Effect>>,
    waiting: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for ToolRunJobPortState {
    fn wake(self: Arc<Self>) { self.waiting.store(false, std::sync::atomic::Ordering::SeqCst); }
    fn wake_by_ref(self: &Arc<Self>) { self.waiting.store(false, std::sync::atomic::Ordering::SeqCst); }
}

impl ToolRunJobPort {
    /// ⏰️ Borrows the existing atomic port owner as a host-completion wake without another allocation.
    pub fn request_waker(&self) -> std::task::Waker { std::task::Waker::from(self.inner.clone()) }

    /// 🎟️ Quotes the original lease capture before any caller slot is consumed.
    pub fn original_close_demands(&self)->Result<RetirementDemand,ValueError>{
        if Arc::weak_count(&self.inner)!=0{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port weak backing remains live"));}
        let effects=self.inner.effects.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port queue is unavailable"))?;
        if Arc::strong_count(&self.inner)==1&&!effects.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port effects require native retirement"));}
        Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})
    }

    /// 📮️ Captures the same original port under unchanged caller authority.
    pub fn admit_original_close(source:&mut Option<Self>,grant:RetainedCloneGrant)->Result<Option<(OriginalToolRunJobPortRetirement,RetainedCloneProgress)>,ValueError>{
        let Some(port)=source.as_ref()else{return Ok(None)};let demand=port.original_close_demands()?;
        if !tool_original_grant_funds(grant,demand){return Ok(None);}
        Ok(Some((OriginalToolRunJobPortRetirement{source:ManuallyDrop::new(source.take()),effects:ManuallyDrop::new(None)},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }

    /// 🎟️ Births the original native queue before moving the unchanged caller effect.
    pub fn admit_original_dispatch(&self,source:&mut Option<Effect>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=Default::default();if source.is_none(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty))}let mut effects=self.inner.effects.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port queue is unavailable"))?;
        if effects.len()==effects.capacity(){let old=effects.capacity();let extent=old.checked_add(1).and_then(|capacity|capacity.checked_mul(size_of::<Effect>())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool port queue extent overflow"))?;let released=old.checked_mul(size_of::<Effect>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool port prior queue extent overflow"))?;if grant.maximum_capacity_bytes<extent||grant.maximum_release_bytes<released{return Ok(RetainedCloneStep::Progress(empty))}effects.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool port queue backing unavailable"))?;let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:effects.capacity()*size_of::<Effect>(),released_bytes:released,..empty};if effects.capacity()!=old+1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tool port native queue changed its quoted extent").with_retained_progress(progress))}return Ok(RetainedCloneStep::Progress(progress))}
        effects.push_back(source.take().unwrap());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))
    }

    /// 📨️ Queues one effect the driver hands the host on its next turn, in order.
    pub fn dispatch(&self, effect: Effect) {
        if let Ok(mut effects) = self.inner.effects.lock() {
            effects.push_back(effect);
        }
    }

    /// ⏸️ The job's next step makes no progress until [`ToolRunJobPort::wake`]; set it while holding whatever
    /// lock the waker folds its answer under, so no wake can slip between the decision and the flag.
    pub fn wait(&self) {
        self.inner.waiting.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// ⏰️ Makes a waiting job runnable again.
    pub fn wake(&self) {
        self.inner.waiting.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_waiting(&self) -> bool {
        self.inner.waiting.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn has_effects(&self) -> bool {
        self.inner.effects.lock().is_ok_and(|effects| !effects.is_empty())
    }

    pub fn pop_effect(&self) -> Option<Effect> {
        self.inner.effects.lock().ok()?.pop_front()
    }

    pub fn restore_effect(&self, effect: Effect) {
        if let Ok(mut effects) = self.inner.effects.lock() {
            effects.push_front(effect);
        }
    }
}

fn tool_original_grant_funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}

/// ♻️ Keeps atomic final-port queue custody until its original backing closes.
pub struct OriginalToolRunJobPortRetirement{
    pub source:ManuallyDrop<Option<ToolRunJobPort>>,
    pub effects:ManuallyDrop<Option<std::collections::VecDeque<Effect>>>,
}

impl OriginalToolRunJobPortRetirement{
    /// 📏️ Borrows the exact original lease or captured native queue frontier.
    pub fn demands(&self)->Result<RetirementDemand,ValueError>{
        if let Some(port)=self.source.as_ref(){if Arc::weak_count(&port.inner)!=0{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port weak backing remains live"));}return Ok(RetirementDemand{copy_bytes:0,release_bytes:if Arc::strong_count(&port.inner)==1{semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<ToolRunJobPortState>()}else{0},depth:1,..Default::default()});}
        if let Some(effects)=self.effects.as_ref(){if !effects.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool port final effects require native retirement"));}return Ok(RetirementDemand{copy_bytes:0,release_bytes:effects.capacity().checked_mul(size_of::<Effect>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool port queue extent overflow"))?,depth:1,..Default::default()});}
        Ok(Default::default())
    }

    /// 🪙️ Retires one real original frontier without draining queued effects.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=Default::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}let demand=self.demands()?;if !tool_original_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}
        if let Some(port)=self.source.take(){let state=Arc::into_inner(port.inner);let released_bytes=if let Some(state)=state{let effects=state.effects.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);*self.effects=Some(effects);demand.release_bytes}else{0};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..empty}));}
        drop(self.effects.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}))
    }

    /// 🏁️ Both original lease and final native queue must be gone.
    pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.effects.is_none()}
}

impl Drop for OriginalToolRunJobPortRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original tool port retirement abandoned live lease or queue");}}

/// 🎟️ Erases the original port lease and its final native queue behind the typed retirement ladder of owning containers.
impl semio_framework_value::retirement::RetireOwned for ToolRunJobPort {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(OriginalToolRunJobPortRetirement{source:ManuallyDrop::new(Some(self)),effects:ManuallyDrop::new(None)})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalToolRunJobPortRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

impl semio_framework_value::retirement::RetirementCursor for OriginalToolRunJobPortRetirement {
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;self.demands()}

    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{
        use semio_framework_value::retirement::RetirementStep;
        match OriginalToolRunJobPortRetirement::close_step(self,grant){
            Ok(step)=>{let progress=step.progress();if progress!=RetainedCloneProgress::default(){RetirementStep::Progress(progress)}else if self.terminal_is_empty(){RetirementStep::Complete}else{RetirementStep::BudgetExhausted}}
            Err(error)=>RetirementStep::Failure(error),
        }
    }
    fn terminal_is_empty(&self)->bool{OriginalToolRunJobPortRetirement::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.depth)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands()?.copy_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.demands().ok().map(|demand|demand.release_bytes)}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🪟️ What a renderer learns about the run on this document instance (§4.1 layer 1): its identity and state, the
/// provisional entities, the §2.3 progress (stage, counters, step ring) and the latest plugin payload a tick carried.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct ToolRunView {
    pub tool_id: String,
    pub identity: ToolRunIdentity,
    pub state: ToolRunState,
    pub provisional_entities: Arc<BTreeSet<u64>>,
    pub progress: semio_framework_tool_run::ToolRunProgress,
    pub payload: Option<Arc<Vec<u8>>>,
}

/// ⏯️ Fixed live run identity and state borrowed directly from its original ledger entry.
 #[derive(Clone,Copy,Debug,PartialEq,Eq)]
 pub struct ToolRunPollView<'a>{pub tool_id:&'a str,pub identity:ToolRunIdentity,pub state:ToolRunState}
 impl ToolRunView {
 pub fn poll_view(&self)->ToolRunPollView<'_>{ToolRunPollView{tool_id:&self.tool_id,identity:self.identity,state:self.state}}
    /// 🏗️ A view of a run with no provisional entities, no progress yet and no payload.
    pub fn new(tool_id: impl Into<String>, identity: ToolRunIdentity, state: ToolRunState) -> Self {
        let progress = semio_framework_tool_run::ToolRunProgress { identity, sequence: 0, state, stage: 0, completed: 0, total: None, counters: Vec::new(), units_per_second: 0.0, conflicts: 0, steps: ToolRunStepRing::new() };
        Self { tool_id: tool_id.into(), identity, state, provisional_entities: Arc::new(BTreeSet::new()), progress, payload: None }
    }
}
