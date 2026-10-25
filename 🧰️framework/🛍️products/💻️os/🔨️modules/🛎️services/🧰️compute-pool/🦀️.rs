//! 🧰️ The bounded compute pool: job admission, deadline and cancellation and the retained completion custody.

use semio_framework_async::{Either, HostAsyncRuntime, Lane, OperationContext, ScopeHandle, WorkerPool, WorkerSubmitErrorKind, select2};
use semio_framework_async_sync::{OwnedPermit, Semaphore, oneshot};
use semio_framework_job::{
    BACKGROUND_LANE_FUEL,
    BACKGROUND_LANE_WALL_US,
    Generation as JobGeneration,
    INTERACTIVE_LANE_FUEL,
    INTERACTIVE_LANE_WALL_US,
    InteractiveJob,
    InteractiveStage,
    MAINTENANCE_LANE_FUEL,
    MAINTENANCE_LANE_WALL_US,
    OperationId,
    USER_VISIBLE_LANE_FUEL,
    USER_VISIBLE_LANE_WALL_US,
};
use std::sync::Arc;
use super::ComputeJobParamsPreparation;
use super::host_runtime::global_worker_pool;

/// 🔁️ Consecutive progressing job steps the driver runs before it sleeps one poll interval.
const COMPUTE_IMMEDIATE_STEPS_PER_YIELD: u32 = 256;

//#region 🧮️ComputePool
/// 🚫️ [`ComputePool`]'s dispatch failure modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComputeError {
    /// ⏰️ `ctx.deadline_ms` elapsed before admission or terminal completion. Interactive jobs
    /// receive cancellation immediately and observe it at their next bounded step; platform I/O
    /// remains non-preemptible once its operating-system call has started.
    DeadlineExceeded,
    /// 💥️ The result channel closed before a value arrived (the worker thread panicked, or the
    /// runtime is shutting down).
    WorkerLost,
    /// 🛑️ Cancellation refused admission before an original semantic result could be borrowed.
    Cancelled,
}

impl std::fmt::Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeError::Cancelled => write!(f,"compute cancelled before admission"),
            ComputeError::DeadlineExceeded => write!(f, "compute deadline exceeded"),
            ComputeError::WorkerLost => write!(f, "compute worker lost before returning a result"),
        }
    }
}
impl std::error::Error for ComputeError {}

/// 🧳️ Keeps the original mounted owner available for borrowed receiving and separately paid retirement.
/// 🎟️ Borrows the one original issued wallet that receives every physical compute transition.
pub trait ComputeRetainedRecipient {
    fn remaining_grant(&self)->Result<semio_framework_job::RetainedCloneGrant,semio_framework_value::ValueError>;
    fn record_progress(&mut self,progress:semio_framework_job::RetainedCloneProgress)->Result<(),semio_framework_value::ValueError>;
}

pub struct ComputeJobCompletion<J:InteractiveJob+'static>{pub(super) operation:OperationId,pub(super) generation:JobGeneration,session:semio_framework_job_worker::MountedWorkerJobSession<J>,pub(super) result:Result<(),ComputeError>,permit:Option<OwnedPermit>,fault:std::mem::ManuallyDrop<Option<semio_framework_value::ValueError>>}
impl<J:InteractiveJob+'static> ComputeJobCompletion<J>{
    pub fn identity(&self)->(OperationId,JobGeneration){(self.operation,self.generation)}
    pub fn result(&self)->&Result<(),ComputeError>{&self.result}
    pub fn session(&self)->&semio_framework_job_worker::MountedWorkerJobSession<J>{&self.session}
    pub fn fault(&self)->Option<&semio_framework_value::ValueError>{self.fault.as_ref()}
    pub fn terminal_is_empty(&self)->bool{self.session.terminal_is_empty()&&self.permit.is_none()&&self.fault.is_none()}
    /// 🪢️ Retires the same mounted custody through the caller's original clock, root and receipt.
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job_worker_authority::WorkerJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step,OriginalTurnFailure};
use semio_framework_job_worker::{WorkerJobDemandError};use semio_framework_job_worker_authority::{WorkerJobCloseStep};
        use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep};
        if self.terminal_is_empty(){return Ok(WorkerJobCloseStep::Complete{progress:Default::default()})}
        let map=|step:InteractiveJobCloseStep|match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked};
        if let Some(fault)=self.fault.as_ref(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::ValueError>>(),release_bytes:match &fault.message{std::borrow::Cow::Owned(message)=>message.capacity(),std::borrow::Cow::Borrowed(_)=>0},depth:1,..Default::default()};
            return close_original_retained_step(cx,demand,|_|{drop(self.fault.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}).map(map)
        }
        if self.session.terminal_is_empty(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<OwnedPermit>>(),depth:1,..Default::default()};
            let step=close_original_retained_step(cx,demand,|_|{drop(self.permit.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})?;
            return Ok(if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:step.progress()}}else{map(step)})
        }
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
        if self.session.checked_out_outcome_descriptor().is_none(){self.session.begin_close();}
        let alias=self.session.has_original_cancel_alias_witness(cx.original_cancel_token());
        match alias{Err(WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Ok(true)=>{
            let grant=cx.retained_grant();let step=match self.session.return_original_cancel_alias_step(cx.original_cancel_token(),grant){Ok(Some(step))=>step,Err(WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Ok(None)=>return Ok(WorkerJobCloseStep::Blocked)};
            cx.consume_fuel(1);cx.consume_retained(step.progress())?;return Ok(WorkerJobCloseStep::Pending{progress:step.progress()})
        },Ok(false)=>{}}
        self.session.close_step(cx)
    }
}
impl<J:InteractiveJob+'static> Drop for ComputeJobCompletion<J>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"compute completion abandoned original mounted custody");}}

/// 🧮️ Bounds every interactive compute job and blocking platform-I/O call admitted to
/// `capacity`, independent of
/// [`WorkerPool`]'s own (larger, process-wide) `worker_count` — the real dispatch substrate is now
/// [`global_worker_pool`] (this type's constructor is frozen by external call sites this packet's
/// boundary does not own — see that fn's doc — so it cannot accept an injected pool), but a
/// [`Semaphore`] sized to `capacity` still gates LOGICAL admission on top of it, and is also what
/// lets `ctx.deadline_ms` be enforced honestly (a bounded semaphore has somewhere to race a timeout
/// against; submitting straight to the pool with no local admission gate would not).
pub struct ComputePool {
    pub(super) admission: Arc<Semaphore>,
    pub(super) pool: WorkerPool,
}

impl ComputePool {
    pub async fn new(capacity: u32) -> ComputePool {
        Self::with_pool(capacity, global_worker_pool())
    }

    pub fn with_pool(capacity: u32, pool: WorkerPool) -> ComputePool {
        ComputePool { admission: Arc::new(Semaphore::new(capacity.max(1) as usize)), pool }
    }

    /// 🎟️ Prepares the original invocation parameters only after its supplied control funds the alias and metadata.
    pub fn prepare_job_params(preparation:&mut ComputeJobParamsPreparation,params:&mut Option<semio_framework_job_worker_authority::BatchJobParams>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{preparation.advance(params,cx)}

    /// 🧮️ Drives `job` to a terminal outcome on [`global_worker_pool`] through one retained mounted
    /// session. Every worker closure pumps at most one bounded session transition; resumable outcomes
    /// enqueue a fresh closure on the context lane. The admission permit spans the whole job. Cancellation is checked
    /// before admission and inside every step, while an absolute deadline cancels the job and returns
    /// [`ComputeError::DeadlineExceeded`].
    pub async fn run_job<J: InteractiveJob + 'static, R: HostAsyncRuntime>(&self, runtime: &R, _scope: &ScopeHandle, ctx: &OperationContext, _retained: semio_framework_job::RetainedCloneGrant, job: &mut Option<J>, params: &mut Option<semio_framework_job_worker_authority::BatchJobParams>, control: &mut impl semio_framework_job_worker_authority::WorkerJobAdmissionControl, retained_recipient: &mut impl ComputeRetainedRecipient, completion: &mut Option<ComputeJobCompletion<J>>, recipient: impl for<'a> FnOnce(semio_framework_job::JobOutcomeView<'a>)) -> Result<bool, semio_framework_value::ValueError> {
        let lane = Lane::from_context_lane(ctx.lane);
        if let Some(owner)=completion.as_ref(){if owner.identity()!=control.admission_identity(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"compute driver received another original completion owner"))}}
        if completion.is_none(){
            let grant=control.admission_grant()?;let remaining=retained_recipient.remaining_grant()?;let header=size_of::<Option<ComputeJobCompletion<J>>>();let child=semio_framework_job_worker::WorkerJobSession::<J>::owned_admission_demand()?;let copied_bytes=child.copy_bytes.checked_add(header).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original compute admission layout overflow"))?;
            if grant.maximum_items<2||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<child.capacity_bytes||grant.maximum_depth<child.depth||remaining.maximum_items<2||remaining.maximum_copy_bytes<copied_bytes||remaining.maximum_capacity_bytes<child.capacity_bytes||remaining.maximum_depth<child.depth||!control.admission_is_open(){return Ok(false)}
            let Some((session,admission))=semio_framework_job_worker::MountedWorkerJobSession::try_admit_owned(job, params, control)?else{return Ok(false)};
            *completion=Some(ComputeJobCompletion{operation:control.admission_identity().0,generation:control.admission_identity().1,session,result:Ok(()),permit:None,fault:std::mem::ManuallyDrop::new(None)});
            let header_progress=semio_framework_job::RetainedCloneProgress{copied_items:1,copied_bytes:header,..Default::default()};if let Err(error)=retained_recipient.record_progress(admission.checked_add(header_progress)?){let owner=completion.as_mut().unwrap();*owner.fault=Some(error);owner.result=Err(ComputeError::WorkerLost);return Ok(true)}if let Err(error)=control.receive_admission(header_progress){let owner=completion.as_mut().unwrap();*owner.fault=Some(error);owner.result=Err(ComputeError::WorkerLost);return Ok(true)}
        }
        let completion=completion.as_mut().expect("original caller retains admitted compute owner");
        if completion.result.is_err()||completion.fault.is_some(){return Ok(true)}
        if completion.session.checked_out_outcome().ok().flatten().is_some_and(|view|view.is_terminal()){return Ok(true)}
        if completion.permit.is_none(){match self.acquire_job_permit(runtime,ctx).await{Ok(Some(permit))=>completion.permit=Some(permit),Ok(None)=>{completion.result=Err(ComputeError::Cancelled);return Ok(true)},Err(error)=>{completion.result=Err(error);return Ok(true)}}}
        let mut recipient=Some(recipient);
        let mut immediate=0u32;
        loop{
            let now=runtime.now_ms().await;
            if ctx.deadline_ms.is_some_and(|deadline|now>=deadline){ctx.cancel.cancel().await;completion.result=Err(ComputeError::DeadlineExceeded);return Ok(true)}
            if ctx.cancel.is_cancelled_now(){completion.result=Err(ComputeError::Cancelled);return Ok(true)}
            let retained=retained_recipient.remaining_grant()?;
            match completion.session.pump_one(&self.pool, lane, retained){
                Ok(semio_framework_job_worker::WorkerJobPoll::Outcome|semio_framework_job_worker::WorkerJobPoll::Terminal)=>{
                    let Some((_issued,progress))=completion.session.take_checked_out_retained_step_receipt()else{completion.result=Err(ComputeError::WorkerLost);return Ok(true)};
                    if let Err(error)=retained_recipient.record_progress(progress){*completion.fault=Some(error);completion.result=Err(ComputeError::WorkerLost);return Ok(true)}
                    match completion.session.checked_out_outcome(){
                        Ok(Some(view))if view.is_terminal()=>{recipient.take().expect("original compute terminal recipient")(view);return Ok(true)},
                        Ok(Some(_))|Ok(None)=>{},
                        _=>{completion.result=Err(ComputeError::WorkerLost);return Ok(true)},
                    }
                    while completion.session.checked_out_outcome_descriptor().is_some(){let retained=retained_recipient.remaining_grant()?;let step=completion.session.acknowledge_checked_out_outcome(retained);if let Err(error)=retained_recipient.record_progress(step.progress()){*completion.fault=Some(error);completion.result=Err(ComputeError::WorkerLost);return Ok(true)}if step.progress()==Default::default(){completion.result=Err(ComputeError::WorkerLost);return Ok(true)}}
                    if completion.session.resume().is_err(){completion.result=Err(ComputeError::WorkerLost);return Ok(true)}
                    immediate+=1;if immediate<COMPUTE_IMMEDIATE_STEPS_PER_YIELD{continue}
                },
                Ok(_)=>{},
                Err(semio_framework_job_worker::MountedWorkerJobPumpFault::Submit(semio_framework_job_worker::WorkerJobSubmitFault::Pool(WorkerSubmitErrorKind::Contended|WorkerSubmitErrorKind::Saturated)))=>{},
                Err(_)=>{completion.result=Err(ComputeError::WorkerLost);return Ok(true)},
            }
            immediate=0;
            runtime.sleep_until(ctx.deadline_ms.map_or(now.saturating_add(1),|deadline|deadline.min(now.saturating_add(1)))).await;
        }
    }

    /// 🌐️ Runs a blocking platform-I/O boundary on the pool's dedicated fair I/O lane.
    pub async fn run_io<T: Send + 'static, R: HostAsyncRuntime>(&self, runtime: &R, _scope: &ScopeHandle, ctx: OperationContext, work: impl FnOnce() -> T + Send + 'static) -> Result<T, ComputeError> {
        self.run_in_lane(runtime, ctx, Lane::Io, work).await
    }

    async fn acquire_job_permit<R: HostAsyncRuntime>(&self, runtime: &R, ctx: &OperationContext) -> Result<Option<OwnedPermit>, ComputeError> {
        loop {
            if ctx.cancel.is_cancelled().await {
                return Ok(None);
            }
            let now_ms = runtime.now_ms().await;
            if ctx.deadline_ms.is_some_and(|deadline| now_ms >= deadline) {
                return Err(ComputeError::DeadlineExceeded);
            }
            let poll_deadline = ctx.deadline_ms.map_or(now_ms.saturating_add(1), |deadline| deadline.min(now_ms.saturating_add(1)));
            match select2(self.admission.acquire_owned(), runtime.sleep_until(poll_deadline)).await {
                Either::Left(permit) => return Ok(Some(permit)),
                Either::Right(()) => {}
            }
        }
    }

    async fn run_in_lane<T: Send + 'static, R: HostAsyncRuntime>(&self, runtime: &R, ctx: OperationContext, lane: Lane, work: impl FnOnce() -> T + Send + 'static) -> Result<T, ComputeError> {
        let admission = self.admission.clone();
        let permit = match ctx.deadline_ms {
            Some(deadline_ms) => match select2(admission.acquire_owned(), runtime.sleep_until(deadline_ms)).await {
                Either::Left(permit) => permit,
                Either::Right(()) => return Err(ComputeError::DeadlineExceeded),
            },
            None => admission.acquire_owned().await,
        };
        let (result_tx, result_rx) = oneshot::channel::<T>();
        self.pool.submit(
            lane,
            Box::new(move || {
                let _permit = permit;
                let result = work();
                let _ = result_tx.send(result);
            }),
        );
        match ctx.deadline_ms {
            Some(deadline_ms) => match select2(result_rx, runtime.sleep_until(deadline_ms)).await {
                Either::Left(result) => result.map_err(|_| ComputeError::WorkerLost),
                Either::Right(()) => Err(ComputeError::DeadlineExceeded),
            },
            None => result_rx.await.map_err(|_| ComputeError::WorkerLost),
        }
    }
}

fn compute_job_budget(lane: Lane) -> (InteractiveStage, u64, u64) {
    match lane {
        Lane::Interactive => (InteractiveStage::InteractiveStep, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US),
        Lane::UserVisible => (InteractiveStage::UserVisibleSimStep, USER_VISIBLE_LANE_FUEL, USER_VISIBLE_LANE_WALL_US),
        Lane::Background | Lane::Io | Lane::Timer => (InteractiveStage::BackgroundStep, BACKGROUND_LANE_FUEL, BACKGROUND_LANE_WALL_US),
        Lane::Maintenance => (InteractiveStage::BackgroundStep, MAINTENANCE_LANE_FUEL, MAINTENANCE_LANE_WALL_US),
    }
}

//#endregion 🧮️ComputePool
