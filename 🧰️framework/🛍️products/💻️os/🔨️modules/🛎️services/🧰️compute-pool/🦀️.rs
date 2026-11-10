//! 🧰️ The bounded compute pool: job admission, deadline and cancellation and the retained completion custody.

use semio_framework_async::{Either, HostAsyncRuntime, Lane, OperationContext, ScopeHandle, WorkerPool, select2};
use semio_framework_async_sync::{OwnedPermit, Semaphore, oneshot};
use semio_framework_job::{Generation as JobGeneration, InteractiveJob, OperationId};
use std::sync::Arc;
use semio_framework_job_worker::{MountedWorkerJobSession,WorkerJobSessionAdmission};
use super::{ComputeJobParamsPreparation,ComputeOriginalCaller};
use super::host_runtime::global_worker_pool;

/// 🔁️ Consecutive progressing admission actions the driver runs before it sleeps one poll interval.
const COMPUTE_IMMEDIATE_STEPS_PER_YIELD: u32 = 256;

/// 🌀️ Hands the executor one turn between two progressing job steps, so concurrent jobs interleave instead of one monopolising the thread.
struct YieldNow(bool);

impl std::future::Future for YieldNow {
    type Output = ();
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        if self.0 {
            return std::task::Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

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

pub struct ComputeJobCompletion<J:InteractiveJob+'static>{pub(super) operation:OperationId,pub(super) generation:JobGeneration,admission:WorkerJobSessionAdmission<J,MountedWorkerJobSession<J>>,session:Option<MountedWorkerJobSession<J>>,pub(super) result:Result<(),ComputeError>,permit:Option<OwnedPermit>,fault:std::mem::ManuallyDrop<Option<semio_framework_value::ValueError>>}
impl<J:InteractiveJob+'static> ComputeJobCompletion<J>{
    pub fn identity(&self)->(OperationId,JobGeneration){(self.operation,self.generation)}
    pub fn result(&self)->&Result<(),ComputeError>{&self.result}
    pub fn session(&self)->Option<&MountedWorkerJobSession<J>>{self.session.as_ref()}
    pub fn fault(&self)->Option<&semio_framework_value::ValueError>{self.fault.as_ref()}
    pub fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.session.as_ref().is_none_or(MountedWorkerJobSession::terminal_is_empty)&&self.permit.is_none()&&self.fault.is_none()}
    fn fail(&mut self,failure:semio_framework_job::OriginalTurnFailure){drop(failure);*self.fault=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"compute job callback refused its original turn"));self.result=Err(ComputeError::WorkerLost)}
    fn exhausted(&mut self){*self.fault=Some(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"compute job exhausted its retained authority"));self.result=Err(ComputeError::WorkerLost)}
    /// 🪢️ Retires the same mounted custody through the caller's original clock, root and receipt.
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job_worker_authority::WorkerJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep,close_original_retained_step,receive_original_retained_work,OriginalTurnFailure};
        use semio_framework_job_worker::WorkerJobDemandError;use semio_framework_job_worker_authority::WorkerJobCloseStep;
        use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
        if self.terminal_is_empty(){return Ok(WorkerJobCloseStep::Complete{progress:Default::default()})}
        let map=|step:InteractiveJobCloseStep|match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked};
        if let Some(fault)=self.fault.as_ref(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::ValueError>>(),release_bytes:match &fault.message{std::borrow::Cow::Owned(message)=>message.capacity(),std::borrow::Cow::Borrowed(_)=>0},depth:1,..Default::default()};
            return close_original_retained_step(cx,demand,|_|{drop(self.fault.take());RetainedWorkOutcome::<_,semio_framework_value::ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})),work:1}}).map(map)
        }
        if self.session.as_ref().is_some_and(MountedWorkerJobSession::terminal_is_empty){drop(self.session.take());}
        if let Some(session)=self.session.as_mut(){
            if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(WorkerJobCloseStep::Pending{progress:Default::default()})}
            if session.checked_out_outcome_descriptor().is_none(){session.begin_close();}
            match session.has_original_cancel_alias_witness(cx.original_cancel_token()){Err(WorkerJobDemandError::Refused(error))=>return Err(OriginalTurnFailure::debit_original(error,cx)),Err(WorkerJobDemandError::Contention(_))=>return Ok(WorkerJobCloseStep::Blocked),Ok(true)=>{
                let turn=cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))?.map_or(semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},|(turn,_)|turn);let blocked=!matches!(&turn.result,Ok(Some(_)));
                let result=match turn.result{Ok(Some(step))=>Ok(step),Ok(None)|Err(WorkerJobDemandError::Contention(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(WorkerJobDemandError::Refused(error))=>Err(error)};
                let step=receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;
                return Ok(if blocked{WorkerJobCloseStep::Blocked}else{WorkerJobCloseStep::Pending{progress:step.progress()}})
            },Ok(false)=>{}}
            return session.close_step(cx)
        }
        if self.permit.is_some(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<OwnedPermit>>(),depth:1,..Default::default()};
            let step=close_original_retained_step(cx,demand,|_|{drop(self.permit.take());RetainedWorkOutcome::<_,semio_framework_value::ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;
            return Ok(if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:step.progress()}}else{map(step)})
        }
        let step=self.admission.close_step(cx)?;
        Ok(if self.terminal_is_empty(){WorkerJobCloseStep::Complete{progress:match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,InteractiveJobCloseStep::Blocked=>Default::default()}}}else{map(step)})
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

    /// 🧮️ Drives `job` to a terminal outcome through one retained mounted session whose every callback
    /// runs on `caller`, the original source that funds, clocks and cancels it; each turn debits
    /// `retained_recipient` with its actual receipt. Birth is staged: a call that cannot fund the next
    /// birth step returns `Ok(false)` and leaves `job` and `params` where they were. The admission permit
    /// spans the whole job. Cancellation is checked before admission and between steps, while an
    /// absolute deadline cancels the job and returns [`ComputeError::DeadlineExceeded`].
    pub async fn run_job<J: InteractiveJob + 'static, R: HostAsyncRuntime>(&self, runtime: &R, ctx: &OperationContext, caller: &mut ComputeOriginalCaller, job: &mut Option<J>, params: &mut Option<semio_framework_job_worker_authority::BatchJobParams>, retained_recipient: &mut impl ComputeRetainedRecipient, completion: &mut Option<ComputeJobCompletion<J>>, recipient: impl for<'a> FnOnce(semio_framework_job::JobOutcomeView<'a>)) -> Result<bool, semio_framework_value::ValueError> {
        use semio_framework_job_worker::WorkerJobPoll;
        let identity=(OperationId(ctx.trace.0),JobGeneration(u64::from(ctx.generation)));
        if let Some(owner)=completion.as_ref(){if owner.identity()!=identity{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"compute driver received another original completion owner"))}}
        if completion.is_none(){
            let mut admission=WorkerJobSessionAdmission::<J,MountedWorkerJobSession<J>>::new();let mut session=None;
            let turn=caller.turn_funded(retained_recipient,|cx|admission.advance(job,params,&mut session,cx));
            if !admission.terminal_is_empty()||session.is_some(){*completion=Some(ComputeJobCompletion{operation:identity.0,generation:identity.1,admission,session,result:Ok(()),permit:None,fault:std::mem::ManuallyDrop::new(None)});}
            match (turn,completion.as_mut()){
                (Ok(_),None)=>return Ok(false),
                (Ok(_),Some(_))=>{},
                (Err(failure),Some(owner))=>{owner.fail(failure);return Ok(true)},
                (Err(_),None)=>return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"compute admission was refused its original turn")),
            }
        }
        let Some(completion)=completion.as_mut() else{return Ok(false)};
        if completion.result.is_err()||completion.fault.is_some(){return Ok(true)}
        let mut immediate=0u32;
        while completion.session.is_none(){
            if ctx.cancel.is_cancelled_now(){completion.result=Err(ComputeError::Cancelled);return Ok(true)}
            let ComputeJobCompletion{admission,session,..}=&mut *completion;
            if let Err(failure)=caller.turn_funded(retained_recipient,|cx|admission.advance(job,params,session,cx)){completion.fail(failure);return Ok(true)}
            if caller.receipt()==Default::default(){if caller.is_exhausted(retained_recipient){completion.exhausted();return Ok(true)}return Ok(false)}
            immediate+=1;if immediate>=COMPUTE_IMMEDIATE_STEPS_PER_YIELD{immediate=0;runtime.sleep_until(runtime.now_ms().await.saturating_add(1)).await;}
        }
        if completion.session.as_ref().and_then(|session|session.checked_out_outcome().ok().flatten()).is_some_and(|view|view.is_terminal()){return Ok(true)}
        if completion.permit.is_none(){match self.acquire_job_permit(runtime,ctx).await{Ok(Some(permit))=>completion.permit=Some(permit),Ok(None)=>{completion.result=Err(ComputeError::Cancelled);return Ok(true)},Err(error)=>{completion.result=Err(error);return Ok(true)}}}
        let mut recipient=Some(recipient);
        loop{
            let now=runtime.now_ms().await;
            if ctx.deadline_ms.is_some_and(|deadline|now>=deadline){ctx.cancel.cancel().await;completion.result=Err(ComputeError::DeadlineExceeded);return Ok(true)}
            if ctx.cancel.is_cancelled_now(){completion.result=Err(ComputeError::Cancelled);return Ok(true)}
            if caller.is_exhausted(retained_recipient){completion.exhausted();return Ok(true)}
            let Some(session)=completion.session.as_mut() else{completion.result=Err(ComputeError::WorkerLost);return Ok(true)};
            match caller.turn_funded(retained_recipient,|cx|session.pump_one_for_interactive_turn(cx)){
                Ok(WorkerJobPoll::Outcome|WorkerJobPoll::Terminal)=>{
                    if session.take_checked_out_retained_step_receipt().is_none(){completion.result=Err(ComputeError::WorkerLost);return Ok(true)}
                    match session.checked_out_outcome(){
                        Ok(Some(view))if view.is_terminal()=>{if let Some(recipient)=recipient.take(){recipient(view)}return Ok(true)},
                        Ok(Some(_))|Ok(None)=>{},
                        _=>{completion.result=Err(ComputeError::WorkerLost);return Ok(true)},
                    }
                    while session.checked_out_outcome_descriptor().is_some(){
                        if let Err(failure)=caller.turn_funded(retained_recipient,|cx|session.acknowledge_checked_out_outcome(cx)){completion.fail(failure);return Ok(true)}
                        if caller.receipt()==Default::default(){completion.result=Err(ComputeError::WorkerLost);return Ok(true)}
                    }
                    if session.resume().is_err(){completion.result=Err(ComputeError::WorkerLost);return Ok(true)}
                    YieldNow(false).await;continue
                },
                Ok(_)=>{},
                Err(failure)=>{completion.fail(failure);return Ok(true)},
            }
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

//#endregion 🧮️ComputePool
