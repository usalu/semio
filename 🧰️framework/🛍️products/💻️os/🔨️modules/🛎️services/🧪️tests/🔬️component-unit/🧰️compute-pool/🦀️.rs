//! 🧰️ Tests of the compute pool: bounded bursts, deadlines, stopped pools, original admission refusal and the original compute admission law with its fixture wallet recipient.

use semio_framework_async::{HostAsyncRuntime, OperationContext, ScopeHandle, ScopeOwner};
use semio_framework_job::{InteractiveJob, InteractiveStage, OperationId};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::task::{Context, Poll, Waker};
use super::super::compute_pool::{ComputeError, ComputeJobCompletion, ComputePool, ComputeRetainedRecipient};
use super::super::host_runtime::TokioHostRuntime;
use super::support::{sleep_ms, test_ctx, test_pool};

fn fixture_compute_grant()->semio_framework_job::RetainedCloneGrant{let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️compute-retained/🔣️.json")).unwrap();serde_json::from_value(law["callerGrant"].clone()).unwrap()}

struct OriginalComputeRecipient { original: semio_framework_job::RetainedCloneGrant, pub(super) progress: semio_framework_job::RetainedCloneProgress }
impl ComputeRetainedRecipient for OriginalComputeRecipient {
 fn remaining_grant(&self)->Result<semio_framework_job::RetainedCloneGrant,semio_framework_value::ValueError>{
  if !self.progress.fits(self.original){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original compute fixture wallet is exhausted"))}
  Ok(semio_framework_job::RetainedCloneGrant{maximum_items:self.original.maximum_items-self.progress.copied_items,maximum_copy_bytes:self.original.maximum_copy_bytes-self.progress.copied_bytes,maximum_capacity_bytes:self.original.maximum_capacity_bytes-self.progress.retained_capacity_bytes,maximum_release_bytes:self.original.maximum_release_bytes-self.progress.released_bytes,maximum_depth:self.original.maximum_depth})
 }
 fn record_progress(&mut self,progress:semio_framework_job::RetainedCloneProgress)->Result<(),semio_framework_value::ValueError>{
  self.progress=self.progress.checked_add(progress)?;self.remaining_grant().map(|_|())
 }
}

fn close_original_compute<J:InteractiveJob+'static>(completion:&mut ComputeJobCompletion<J>,ctx:&OperationContext,deadline:u64,sequence:&mut u64,authority:&semio_framework_job::JobPayloadAuthority,receipt:&mut OriginalComputeRecipient)->semio_framework_job_worker_authority::WorkerJobCloseStep{
    let grant=receipt.remaining_grant().unwrap();let mut progress=Default::default();
    let mut cx=semio_framework_job::StepContext::with_payload_authority(semio_framework_job::OperationId(ctx.trace.0),semio_framework_job::Generation(u64::from(ctx.generation)),semio_framework_job::StepBudget::new(1,deadline,grant),&ctx.cancel,semio_framework_job::default_now_us,sequence,&mut progress,authority).unwrap();
    let step=completion.close_step(&mut cx).unwrap();assert_eq!(step.progress(),cx.retained_progress());drop(cx);receipt.record_progress(progress).unwrap();step
}

async fn receive_original_compute_job<J:InteractiveJob+'static>(pool:&ComputePool,runtime:&TokioHostRuntime,scope:&ScopeHandle,ctx:&OperationContext,original:J)->Result<(),ComputeError>{
    use semio_framework_job::{Generation,OperationId,RetainedCloneProgress,StepBudget};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobAdmissionContext};
    let grant=fixture_compute_grant();let mut receipt=OriginalComputeRecipient{original:grant,progress:RetainedCloneProgress::default()};let mut admission=RetainedCloneProgress::default();let mut job=Some(original);let mut params:Option<BatchJobParams>=None;
    let mut close_sequence=0;let mut close_authority=semio_framework_job::JobPayloadAuthority::admit(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),grant).unwrap().unwrap().0;
    let deadline=ctx.deadline_ms.map_or(u64::MAX,|deadline|deadline.saturating_sub(runtime.pool.now_ms()).saturating_mul(1000).saturating_add(semio_framework_job::default_now_us().unwrap()));
    let mut control=WorkerJobAdmissionContext::new(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),StepBudget::new(1,deadline,grant),semio_framework_job::default_now_us,&mut admission).unwrap();
    params=Some(BatchJobParams{operation:OperationId(ctx.trace.0),generation:Generation(u64::from(ctx.generation)),cancel:ctx.cancel.clone(),config:BatchDriveConfig{retained:grant,site:"compute-original-fixture",stage:InteractiveStage::InteractiveStep,fuel_per_step:64,step_budget_us:1000},now_us:semio_framework_job::default_now_us});
    let mut completion=None;assert!(pool.run_job(runtime,scope,ctx,grant,&mut job,&mut params,&mut control,&mut receipt,&mut completion,|outcome|assert!(outcome.is_terminal())).await.unwrap(),"original supplied fixture authority admits unchanged source");let completion=completion.as_mut().unwrap();
    drop(control);assert!(job.is_none()&&params.is_none());assert!(admission.retained_capacity_bytes>0);let result=completion.result().clone();
    while !completion.terminal_is_empty(){let step=close_original_compute(completion,ctx,deadline,&mut close_sequence,&close_authority,&mut receipt);assert!(step.progress().fits(grant));if step.progress()==Default::default(){sleep_ms(runtime,1).await;}}
    assert!(receipt.progress.released_bytes>=admission.retained_capacity_bytes);drop(completion);assert!(close_authority.close_step(grant).unwrap().progress().fits(grant));assert!(close_authority.terminal_is_empty());result
}
//#region 🧮️ComputePoolTests
struct CountingComputeJob {
    pub(super) current: Option<Arc<AtomicU32>>,
    observed_max: Option<Arc<AtomicU32>>,
    remaining_steps: u8,
    entered: bool,
    closing: bool,
}

impl InteractiveJob for CountingComputeJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            if self.entered {
                self.current.as_ref().expect("compute current counter").fetch_sub(1, Ordering::SeqCst);
                self.entered = false;
            }
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        if !self.entered {
            let now = self.current.as_ref().expect("compute current counter").fetch_add(1, Ordering::SeqCst) + 1;
            self.observed_max.as_ref().expect("compute observed counter").fetch_max(now, Ordering::SeqCst);
            self.entered = true;
        }
        cx.consume_fuel(1);
        if self.remaining_steps > 0 {
            self.remaining_steps -= 1;
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        self.current.as_ref().expect("compute current counter").fetch_sub(1, Ordering::SeqCst);
        self.entered = false;
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "counting compute fixture publishes yield, cancelled and empty complete only")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.entered { return Ok(0) }
        Ok(self.current.as_ref().or(self.observed_max.as_ref()).filter(|owner| Arc::strong_count(owner) == 1 && Arc::weak_count(owner) == 0).map_or(0, |_| std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<AtomicU32>()).expect("original Arc layout").0.pad_to_align().size()))
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(if self.terminal_is_empty() { 0 } else { 1 }) }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let demand=semio_framework_value::RetirementDemand{release_bytes:self.next_close_release_byte_demand()?,depth:usize::from(!self.terminal_is_empty()),..Default::default()};
        let step=semio_framework_job::close_original_retained_step(cx,demand,|_|{
            self.begin_close();
            if self.entered{self.current.as_ref().unwrap().fetch_sub(1,Ordering::SeqCst);self.entered=false;}
            else if self.current.is_some(){drop(self.current.take());}
            else if self.observed_max.is_some(){drop(self.observed_max.take());}
            else{return Ok(semio_framework_job::RetainedCloneStep::Complete(Default::default()))}
            Ok(semio_framework_job::RetainedCloneStep::Progress(semio_framework_job::RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))
        })?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && !self.entered && self.current.is_none() && self.observed_max.is_none()
    }
}

struct NeverCompleteComputeJob {
    closing: bool,
}

impl InteractiveJob for NeverCompleteComputeJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        cx.consume_fuel(1);
        semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "never-complete compute fixture publishes yield and cancelled only")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let step=semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|_|{self.begin_close();Ok(semio_framework_job::RetainedCloneStep::Complete(semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})?;
        Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

#[semio_framework_async_macros::async_test]
async fn interactive_jobs_never_exceed_the_compute_bound_under_a_burst() {
    const COMPUTE_CAPACITY: u32 = 3;
    let runtime = TokioHostRuntime::with_pool(test_pool(2));
    let pool = ComputePool::new(COMPUTE_CAPACITY).await;
    let scope = runtime.open_scope(ScopeOwner::Service("compute-burst"), None).await;
    let current = Arc::new(AtomicU32::new(0));
    let observed_max = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();
    for i in 0..12u32 {
        let pool = &pool;
        let runtime = &runtime;
        let scope = &scope;
        let current = current.clone();
        let observed_max = observed_max.clone();
        let ctx = test_ctx(i as u64, scope.cancel.clone()).await;
        handles.push(async move {
            let job = CountingComputeJob { current: Some(current), observed_max: Some(observed_max), remaining_steps: 8, entered: false, closing: false };
            receive_original_compute_job(pool,runtime,scope,&ctx,job).await.expect("interactive job without a deadline must not fail");
        });
    }
    futures_join_all(handles).await;
    assert!(observed_max.load(Ordering::SeqCst) <= COMPUTE_CAPACITY, "observed concurrency {} exceeded the compute bound {}", observed_max.load(Ordering::SeqCst), COMPUTE_CAPACITY);
    assert!(observed_max.load(Ordering::SeqCst) >= 2, "burst should have produced measurable overlap; observed {}", observed_max.load(Ordering::SeqCst));
}

#[semio_framework_async_macros::async_test]
async fn interactive_job_deadline_cancels_the_resumable_job() {
    let runtime = TokioHostRuntime::with_pool(test_pool(4));
    let pool = ComputePool::new(4).await;
    let scope = runtime.open_scope(ScopeOwner::Service("compute-deadline"), None).await;
    let now = runtime.now_ms().await;
    let mut ctx = test_ctx(0, scope.cancel.clone()).await;
    ctx.deadline_ms = Some(now + 40);
    let cancel = ctx.cancel.clone();
    let outcome = receive_original_compute_job(&pool,&runtime,&scope,&ctx,NeverCompleteComputeJob{closing:false}).await;
    assert_eq!(outcome, Err(ComputeError::DeadlineExceeded), "a non-terminal job must stop at its absolute deadline");
    assert!(cancel.is_cancelled().await, "deadline propagation must cancel the running job");
}

#[semio_framework_async_macros::async_test]
async fn stopped_compute_pool_returns_worker_lost_and_releases_the_job_owner() {
    let runtime = TokioHostRuntime::with_pool(test_pool(1));
    let workers = test_pool(1);
    workers.shutdown();
    let pool = ComputePool::with_pool(1, workers);
    let scope = runtime.open_scope(ScopeOwner::Service("compute-stopped"), None).await;
    let mut ctx = test_ctx(0, scope.cancel.clone()).await;
    ctx.lane = 1;
    let outcome = receive_original_compute_job(&pool,&runtime,&scope,&ctx,NeverCompleteComputeJob{closing:false}).await;
    assert_eq!(outcome, Err(ComputeError::WorkerLost));
}

#[semio_framework_async_macros::async_test]
async fn compute_original_admission_refusal_retains_source_addresses_without_native_births(){
    use semio_framework_job::{Generation,OperationId,RetainedCloneProgress,StepBudget};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobAdmissionContext};
    let runtime=TokioHostRuntime::with_pool(test_pool(1));let pool=ComputePool::with_pool(1,test_pool(1));let scope=runtime.open_scope(ScopeOwner::Service("compute-original-admission"),None).await;let ctx=test_ctx(81299,scope.cancel.clone()).await;let original=fixture_compute_grant();
    let mut job=Some(NeverCompleteComputeJob{closing:false});let mut params=Some(BatchJobParams{operation:OperationId(ctx.trace.0),generation:Generation(u64::from(ctx.generation)),cancel:ctx.cancel.clone(),config:BatchDriveConfig{retained:original,site:"compute-original-admission",stage:InteractiveStage::InteractiveStep,fuel_per_step:1,step_budget_us:1},now_us:semio_framework_job::default_now_us});
    let job_pointer=job.as_ref().unwrap()as*const _;let params_pointer=params.as_ref().unwrap()as*const _;
    for grant in[semio_framework_job::RetainedCloneGrant{maximum_items:0,..original},semio_framework_job::RetainedCloneGrant{maximum_copy_bytes:0,..original},semio_framework_job::RetainedCloneGrant{maximum_capacity_bytes:0,..original},semio_framework_job::RetainedCloneGrant{maximum_depth:0,..original}]{
        let mut admission=RetainedCloneProgress::default();let mut receipt=OriginalComputeRecipient{original,progress:RetainedCloneProgress::default()};let mut completion=None;let mut control=WorkerJobAdmissionContext::new(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),StepBudget::new(1,u64::MAX,grant),semio_framework_job::default_now_us,&mut admission).unwrap();
        let(result,heap)={let mut future=std::pin::pin!(pool.run_job(&runtime,&scope,&ctx,original,&mut job,&mut params,&mut control,&mut receipt,&mut completion,|_|panic!("refused source cannot publish a semantic loan")));semio_framework_trace::observe_heap_allocations_on_this_thread(||future.as_mut().poll(&mut Context::from_waker(std::task::Waker::noop())))};assert!(matches!(result,Poll::Ready(Ok(false))));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(completion.is_none());drop(control);assert_eq!(admission,Default::default());assert_eq!(receipt.progress,Default::default());assert_eq!(job.as_ref().unwrap()as*const _,job_pointer);assert_eq!(params.as_ref().unwrap()as*const _,params_pointer);
    }
    let BatchJobParams{cancel,..}=params.take().unwrap();let mut close=semio_framework_async::CancelTokenRetirement::from_token(cancel);let step=close.return_alias_step(&ctx.cancel,original).unwrap();assert!(step.progress().fits(original));assert!(close.terminal_is_empty());drop(close);
}

#[semio_framework_async_macros::async_test]
async fn cancelled_compute_driver_preserves_the_original_caller_completion_slot(){
    use semio_framework_job::{Generation,OperationId,RetainedCloneProgress,StepBudget};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobAdmissionContext};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧮️compute-retained/🔣️.json")).unwrap();let denied_progress:RetainedCloneProgress=serde_json::from_value(law["zeroWorkClose"]["progress"].clone()).unwrap();
    let runtime=TokioHostRuntime::with_pool(test_pool(1));let pool=ComputePool::with_pool(1,test_pool(1));let scope=runtime.open_scope(ScopeOwner::Service("compute-original-driver"),None).await;let ctx=test_ctx(81300,scope.cancel.clone()).await;let grant=fixture_compute_grant();let mut job=Some(NeverCompleteComputeJob{closing:false});let mut params=None;let mut admission=RetainedCloneProgress::default();let mut receipt=OriginalComputeRecipient{original:grant,progress:RetainedCloneProgress::default()};let mut completion=None;
    let mut close_sequence=0;let mut close_authority=semio_framework_job::JobPayloadAuthority::admit(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),grant).unwrap().unwrap().0;
    let mut control=WorkerJobAdmissionContext::new(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),StepBudget::new(1,u64::MAX,grant),semio_framework_job::default_now_us,&mut admission).unwrap();params=Some(BatchJobParams{operation:OperationId(ctx.trace.0),generation:Generation(u64::from(ctx.generation)),cancel:ctx.cancel.clone(),config:BatchDriveConfig{retained:grant,site:"compute-original-fixture",stage:InteractiveStage::InteractiveStep,fuel_per_step:64,step_budget_us:1000},now_us:semio_framework_job::default_now_us});
    {let mut driver=std::pin::pin!(pool.run_job(&runtime,&scope,&ctx,grant,&mut job,&mut params,&mut control,&mut receipt,&mut completion,|_|panic!("nonterminal original job cannot publish completion")));assert!(matches!(driver.as_mut().poll(&mut Context::from_waker(std::task::Waker::noop())),Poll::Pending));}
    drop(control);assert!(job.is_none()&&params.is_none());assert!(completion.is_some());let original=completion.as_ref().unwrap()as*const _;
    let mut denied_receipt=Default::default();
    let mut denied=semio_framework_job::StepContext::with_payload_authority(OperationId(ctx.trace.0),Generation(u64::from(ctx.generation)),StepBudget::new(law["zeroWorkClose"]["work"].as_u64().unwrap(),u64::MAX,grant),&ctx.cancel,semio_framework_job::default_now_us,&mut close_sequence,&mut denied_receipt,&close_authority).unwrap();
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||completion.as_mut().unwrap().close_step(&mut denied).unwrap());
    assert_eq!(step.progress(),denied_progress);assert_eq!((heap.requested_bytes,heap.released_bytes),(denied_progress.retained_capacity_bytes,denied_progress.released_bytes));drop(denied);assert_eq!(denied_receipt,denied_progress);assert_eq!(completion.as_ref().unwrap()as*const _==original,law["zeroWorkClose"]["originalRetained"].as_bool().unwrap());
    ctx.cancel.cancel().await;
    while !completion.as_ref().unwrap().terminal_is_empty(){assert_eq!(completion.as_ref().unwrap()as*const _,original);let step=close_original_compute(completion.as_mut().unwrap(),&ctx,u64::MAX,&mut close_sequence,&close_authority,&mut receipt);assert!(step.progress().fits(grant));if step.progress()==Default::default(){sleep_ms(&runtime,1).await;}}
    assert!(receipt.progress.released_bytes>=admission.retained_capacity_bytes);drop(completion);assert!(close_authority.close_step(grant).unwrap().progress().fits(grant));assert!(close_authority.terminal_is_empty());
}

/// 🌀️ A self-contained cooperative yield with no tokio `rt`-feature dependency (this crate no
/// longer enables `rt`/`rt-multi-thread`, so `tokio::task::yield_now` is unavailable) — wakes
/// itself immediately, giving whatever executor is driving this future one chance to poll a
/// sibling before resuming.
pub(super) struct Yield(pub(super) bool);
impl Future for Yield {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 🧵️ Minimal `join_all` so this crate's tests do not pull in the `futures` crate for one call
/// site — polls every future in a simple round-robin until all are ready.
async fn futures_join_all<F: Future<Output = ()>>(futures: Vec<F>) {
    let mut pending: Vec<Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
    while !pending.is_empty() {
        let mut still_pending = Vec::new();
        for mut fut in pending {
            if Future::poll(fut.as_mut(), &mut Context::from_waker(Waker::noop())) == Poll::Pending {
                still_pending.push(fut);
            }
        }
        pending = still_pending;
        Yield(false).await;
    }
}
//#endregion 🧮️ComputePoolTests
#[test]
fn original_compute_owned_admission_quotes_the_complete_atomic_child_before_receiving(){
 let demand=semio_framework_job_worker::WorkerJobSession::<NeverCompleteComputeJob>::owned_admission_demand().unwrap();
 let original=fixture_compute_grant();
 assert!(demand.copy_bytes>=std::mem::size_of::<NeverCompleteComputeJob>());
 assert!(demand.capacity_bytes>0);
 assert!(demand.copy_bytes<=original.maximum_copy_bytes&&demand.capacity_bytes<=original.maximum_capacity_bytes&&demand.depth<=original.maximum_depth);
}
