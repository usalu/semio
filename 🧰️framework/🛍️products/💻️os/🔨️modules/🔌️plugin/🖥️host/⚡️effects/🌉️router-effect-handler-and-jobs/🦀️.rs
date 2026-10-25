//! 🌉️ Router effect handler and jobs: the router effect, its handler, the dynamic, fault and complete effect jobs, the effect wallet, the router effect job runner and the unwired handler. See [`super`].

use super::*;

//#region 🌉️RouterEffectHandler
/// 🌉️ Owned mirror of the six `Effect` variants design-runtime.md §2 routes through the EXISTING
/// routers (`IoRouter`/`ArtifactInferenceRouter`/`ArtifactMutationRouter`/
/// `HostTransactionCoordinator`/`AppRouter`), dispatched as an explicit [`InteractiveJob`] after
/// the turn that produced the effect. The compute service drives exactly one bounded step per
/// worker closure and resubmits only resumable outcomes.
#[derive(Clone, Debug)]
pub enum RouterEffect {
    BlobWrite { media_type: MediaType, bytes: Vec<u8> },
    BlobLoad { hash: String },
    DocumentRead { doc: u128, lane: String },
    DocumentWrite { doc: u128, lane: String, ops: Vec<u8> },
    IoCompose { key: String, sources: Vec<String> },
    CacheDerive { engine_id: String, input: Vec<u8> },
    CacheRead { engine_id: String, key: String },
    InvokeExtension { extension_id: String, capability: String, request_json: String },
    DispatchAction { action: String, args: Option<DslValue>, delay_ms: u64 },
}

/// 🌉️ Produces the resumable job that performs one [`RouterEffect`] — injected by the
/// caller. **Which concrete router (`IoRouter` vs `ArtifactInferenceRouter` vs
/// `ArtifactMutationRouter` vs `HostTransactionCoordinator` vs `AppRouter`) answers which
/// `RouterEffect` variant has no established mapping anywhere in this codebase today** — every
/// router in `🖥️host/🦀️.rs` is invoked from application-level orchestration
/// (`WasmtimeNodeHost::run_transaction`, `resolve_open_artifact`, ...) today, never from a
/// per-effect dispatch loop (see the packet report's `## honest gaps`). `AsyncEffectExecutor` only
/// guarantees bounded scheduling, cancellation, deadline, and terminal propagation after the turn;
/// it never re-implements the routers' own logic.
// 🔀️ dedyn-fw-os-misc: DELIBERATELY left `dyn` — a reasoned exception, not an oversight.
// `dyn_enum_close!` needs every implementor nameable from ONE always-compiled declaration site, but
// two of the three (`RecordingRouterHandler`, `AlwaysOkRouterHandler`, this file's own `mod tests`)
// are `#[cfg(test)]`-only while `AsyncEffectExecutor.router_handler: Arc<dyn RouterEffectHandler>`
// is an always-compiled field — same `dyn_enum_close!`-DSL-has-no-per-variant-`#[cfg]` blocker as
// `HttpBody`'s own exception note in `🛎️services/🦀️.rs`. `AsyncEffectExecutor::new` is,
// as of this packet, called ONLY from `mod tests` repo-wide (verified: zero production call sites)
// — genuinely not wired to a real router yet (see `UnwiredRouterEffectHandler`'s own doc), so there
// is no live production seam to generic-ize either. Revisit once a real caller exists.
pub trait RouterEffectHandler: Send + Sync {
    /// 🏭️ Constructs persistent job state only. Effect work belongs in
    /// [`InteractiveJob::step`], where the runtime can bound, cancel, and resume it.
    fn create_job(&self, effect: RouterEffect) -> Box<dyn InteractiveJob + Send>;
}

pub(super) struct DynRouterEffectJob(pub(super) Option<Box<dyn InteractiveJob + Send>>);

impl DynRouterEffectJob {
    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let Some(original) = self.0.as_ref() else { return Ok(Default::default()); };
        if original.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(&**original), depth: 1, ..Default::default() }); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: original.next_close_copy_byte_demand()?, capacity_bytes: original.next_close_capacity_byte_demand(body)?, release_bytes: original.next_close_release_byte_demand()?, depth: original.next_close_depth_demand()?.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "router job wrapper depth overflow"))? })
    }
}

impl InteractiveJob for DynRouterEffectJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure> { self.0.as_mut().expect("original router job remains live").step(cx) }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,semio_framework_value::ValueError> { self.0.as_ref().expect("original router job remains checked out").borrow_outcome(descriptor) }
    fn begin_close(&mut self) { if let Some(original) = self.0.as_mut() { original.begin_close(); } }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::OriginalTurnFailure;
        let grant=cx.retained_grant();
        if self.0.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let demand=self.close_demands(grant.maximum_copy_bytes).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let original=self.0.as_mut().expect("original router close owns job");
        if original.terminal_is_empty(){
            let progress=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()};
            cx.consume_retained(progress)?;drop(self.0.take());cx.consume_fuel(1);return Ok(InteractiveJobCloseStep::Complete{progress})
        }
        let child=semio_framework_value::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        let step=cx.with_retained_loan(child,|cx|original.close_step(cx))?.admit(grant,original.terminal_is_empty()).map_err(OriginalTurnFailure::from)?;
        Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }
    fn register_close_wake(&self, wake: &std::task::Waker) -> bool { self.0.as_ref().is_some_and(|original| original.register_close_wake(wake)) }
    fn terminal_is_empty(&self) -> bool { self.0.is_none() }
}

fn router_effect_close_demands(writer: &Option<RetainedPayloadBuilder>, source: &Option<Vec<u8>>) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    use semio_framework_value::RetirementDemand;
    if let Some(writer) = writer.as_ref() {
        if writer.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }); }
        let mut demand = writer.retirement_demands()?;
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "router writer parent depth overflow"))?;
        return Ok(demand);
    }
    Ok(source.as_ref().map_or(Default::default(), |source| RetirementDemand { copy_bytes: size_of::<Option<Vec<u8>>>(), release_bytes: source.capacity(), depth: 1, ..Default::default() }))
}

fn close_router_effect_sources(writer: &mut Option<RetainedPayloadBuilder>, source: &mut Option<Vec<u8>>, grant: semio_framework_value::RetainedCloneGrant) -> Result<InteractiveJobCloseStep,semio_framework_value::ValueError> {
    use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetirementTurnError};
    let result = router_effect_close_demands(writer, source).and_then(|demand| semio_framework_value::advance_retirement_turn(demand, grant, |child| {
        if let Some(original) = writer.as_mut() {
            if original.terminal_is_empty() {
                *writer = None;
                return Ok((RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }), false));
            }
            let step = original.close_step_granted(semio_framework_value::RetainedCloneGrant { maximum_depth: child.maximum_depth - 1, ..child })?;
            return Ok((RetainedCloneStep::Progress(step.progress()), false));
        }
        if source.is_some() {
            drop(source.take());
            return Ok((RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }), source.is_none()));
        }
        Ok((RetainedCloneStep::Complete(Default::default()), true))
    }).map_err(|error| match error { RetirementTurnError::Owner(error) | RetirementTurnError::Receipt(error) => error }));
    let step=result?;
    let step=if writer.is_none()&&source.is_none(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}};
    step.admit(grant,writer.is_none()&&source.is_none())
}

pub(super) struct FaultRouterEffectJob {
    pub(super) detail: Option<Vec<u8>>,
    pub(super) writer: Option<RetainedPayloadBuilder>,
    pub(super) cursor: usize,
    pub(super) closing: bool,
}

impl InteractiveJob for FaultRouterEffectJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() || self.closing { return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
        if cx.should_yield() { return Ok(None); }
        if self.writer.is_none() {
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            self.writer = Some(RetainedPayloadBuilder::new(JobPayloadStream::Fault));
            return Ok(None);
        }
        let writer = self.writer.as_mut().unwrap();
        if !writer.is_initialized() { writer.advance_initialization(cx)?; return Ok(None); }
        let source = self.detail.as_deref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "router effect original source custody is absent"))?;
        if self.cursor < source.len() { writer.append_original(cx, source, &mut self.cursor)?; return Ok(None); }
        if writer.published().is_none() { writer.seal(cx)?; return Ok(None); }
        let original = writer.published().unwrap();
        JobOutcomeBorrow::admit_fault(cx, original).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, original: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        if original.kind() == JobOutcomeKind::Cancelled { return original.cancelled(); }
        let payload = self.writer.as_ref().and_then(RetainedPayloadBuilder::published).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "router effect descriptor requires its same published original"))?;
        original.fault(payload)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.begin_close();
        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=close_router_effect_sources(&mut self.writer,&mut self.detail,grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        cx.consume_retained(step.progress())?;if step.progress().copied_items!=0{cx.consume_fuel(1)}Ok(step)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.detail)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.detail)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.detail)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.detail)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.detail.is_none() && self.writer.is_none()
    }
}

#[cfg(test)]
pub(super) struct CompleteRouterEffectJob {
    pub(super) output: Option<Vec<u8>>,
    pub(super) writer: Option<RetainedPayloadBuilder>,
    pub(super) cursor: usize,
    pub(super) closing: bool,
}

#[cfg(test)]
impl InteractiveJob for CompleteRouterEffectJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() || self.closing { return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from); }
        if cx.should_yield() { return Ok(None); }
        if self.writer.is_none() {
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            self.writer = Some(RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput));
            return Ok(None);
        }
        let writer = self.writer.as_mut().unwrap();
        if !writer.is_initialized() { writer.advance_initialization(cx)?; return Ok(None); }
        let source = self.output.as_deref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "router effect original source custody is absent"))?;
        if self.cursor < source.len() { writer.append_original(cx, source, &mut self.cursor)?; return Ok(None); }
        if writer.published().is_none() { writer.seal(cx)?; return Ok(None); }
        let original = writer.published().unwrap();
        JobOutcomeBorrow::admit_complete(cx, None, Some(original)).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, original: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        if original.kind() == JobOutcomeKind::Cancelled { return original.cancelled(); }
        let payload = self.writer.as_ref().and_then(RetainedPayloadBuilder::published).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "router effect descriptor requires its same published original"))?;
        original.complete(None, Some(payload))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        self.begin_close();
        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=close_router_effect_sources(&mut self.writer,&mut self.output,grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        cx.consume_retained(step.progress())?;if step.progress().copied_items!=0{cx.consume_fuel(1)}Ok(step)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.output)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.output)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.output)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(router_effect_close_demands(&self.writer, &self.output)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.writer.is_none()
    }
}

pub enum RouterEffectJobOutcome {
    Complete(Vec<u8>),
    Cancelled,
    DeadlineExceeded,
    WorkerLost,
    Fault(String),
}

fn copy_router_effect_payload(payload: &RetainedJobPayload) -> Result<Vec<u8>, String> {
    if payload.len() > semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES {
        return Err(format!("router effect payload exceeds {} bytes", semio_framework_job_payload::JOB_PAYLOAD_OPERATION_BYTES));
    }
    let mut bytes = Vec::with_capacity(payload.len());
    let mut reader = payload.reader();
    while let Some(page) = reader.read_page(1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES) {
        bytes.extend_from_slice(page);
    }
    if !reader.terminal_is_empty() {
        return Err("router effect payload page exceeds the fixed codec grant".to_string());
    }
    Ok(bytes)
}

/// 💰️ One router effect's original wallet: every admission, step and close receipt of its compute job debits it.
struct RouterEffectWallet {
    pub(super) original: semio_framework_job::RetainedCloneGrant,
    pub(super) progress: semio_framework_job::RetainedCloneProgress,
}

impl semio_framework_os_services::ComputeRetainedRecipient for RouterEffectWallet {
    fn remaining_grant(&self) -> Result<semio_framework_job::RetainedCloneGrant, semio_framework_value::ValueError> {
        if !self.progress.fits(self.original) {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "router effect wallet is exhausted"));
        }
        Ok(semio_framework_job::RetainedCloneGrant {
            maximum_items: self.original.maximum_items - self.progress.copied_items,
            maximum_copy_bytes: self.original.maximum_copy_bytes - self.progress.copied_bytes,
            maximum_capacity_bytes: self.original.maximum_capacity_bytes - self.progress.retained_capacity_bytes,
            maximum_release_bytes: self.original.maximum_release_bytes - self.progress.released_bytes,
            maximum_depth: self.original.maximum_depth,
        })
    }

    fn record_progress(&mut self, progress: semio_framework_job::RetainedCloneProgress) -> Result<(), semio_framework_value::ValueError> {
        self.progress = self.progress.checked_add(progress)?;
        self.remaining_grant().map(|_| ())
    }
}

const ROUTER_EFFECT_ADMISSION_TURNS: usize = 4_096;

const ROUTER_EFFECT_CLOSE_TURNS: usize = 65_536;

/// 🏃️ Drives one router effect through the compute pool under its own paid payload authority; the authority is retired before the outcome is returned.
pub async fn run_router_effect_job<R: HostAsyncRuntime>(compute: &ComputePool, runtime: &R, scope: &ScopeHandle, ctx: OperationContext, retained: semio_framework_job::RetainedCloneGrant, handler: &Arc<dyn RouterEffectHandler>, effect: RouterEffect) -> RouterEffectJobOutcome {
    use semio_framework_job::{Generation,OperationId};
    if ctx.cancel.is_cancelled().await {
        return RouterEffectJobOutcome::Cancelled;
    }
    let (operation, generation) = (OperationId(ctx.trace.0), Generation(u64::from(ctx.generation)));
    let mut authority = match semio_framework_job_payload::JobPayloadAuthority::admit(operation, generation, retained) {
        Ok(Some((authority, _))) => authority,
        Ok(None) => return RouterEffectJobOutcome::Fault("router effect payload authority was refused its grant".to_string()),
        Err(error) => return RouterEffectJobOutcome::Fault(error.to_string()),
    };
    let mut wallet = RouterEffectWallet { original: retained, progress: semio_framework_job::RetainedCloneProgress::default() };
    let outcome = drive_router_effect_job(compute, runtime, scope, &ctx, retained, handler, effect, &authority, &mut wallet).await;
    for _ in 0..ROUTER_EFFECT_CLOSE_TURNS {
        if authority.terminal_is_empty() {
            break;
        }
        let Ok(grant) = semio_framework_os_services::ComputeRetainedRecipient::remaining_grant(&wallet) else { break };
        let Ok(step) = authority.close_step(grant) else { break };
        let progress = step.progress();
        if semio_framework_os_services::ComputeRetainedRecipient::record_progress(&mut wallet, progress).is_err() {
            break;
        }
        if progress == semio_framework_job::RetainedCloneProgress::default() {
            runtime.sleep_until(runtime.now_ms().await.saturating_add(1)).await;
        }
    }
    outcome
}

async fn drive_router_effect_job<R: HostAsyncRuntime>(compute: &ComputePool, runtime: &R, scope: &ScopeHandle, ctx: &OperationContext, retained: semio_framework_job::RetainedCloneGrant, handler: &Arc<dyn RouterEffectHandler>, effect: RouterEffect, authority: &semio_framework_job_payload::JobPayloadAuthority, wallet: &mut RouterEffectWallet) -> RouterEffectJobOutcome {
    use semio_framework_job::{Generation,InteractiveStage,OperationId,RetainedCloneProgress,StepBudget};
    use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobAdmissionContext};
    let (operation, generation) = (OperationId(ctx.trace.0), Generation(u64::from(ctx.generation)));
    let mut result = None;
    let mut admission = RetainedCloneProgress::default();
    let mut sequence = 0u64;
    let mut job = Some(DynRouterEffectJob(Some(handler.create_job(effect))));
    let mut params = Some(BatchJobParams { operation, generation, cancel: ctx.cancel.clone(), config: BatchDriveConfig { retained, site: "router-effect", stage: InteractiveStage::InteractiveStep, fuel_per_step: 64, step_budget_us: 1000 }, now_us: semio_framework_job::default_now_us });
    let mut completion = None;
    let now_ms = runtime.now_ms().await;
    let deadline_us = ctx.deadline_ms.map_or(u64::MAX, |deadline| deadline.saturating_sub(now_ms).saturating_mul(1000).saturating_add(semio_framework_job::default_now_us().unwrap_or(0)));
    let mut control = match WorkerJobAdmissionContext::new(operation, generation, StepBudget::new(1, deadline_us, retained), semio_framework_job::default_now_us, &mut admission) {
        Ok(control) => control,
        Err(error) => {
            close_unadmitted_router_effect(runtime, ctx, &mut job, &mut params, &mut sequence, authority, wallet).await;
            return RouterEffectJobOutcome::Fault(error.to_string());
        }
    };
    let mut admitted = false;
    for _ in 0..ROUTER_EFFECT_ADMISSION_TURNS {
        match compute
            .run_job(runtime, scope, ctx, retained, &mut job, &mut params, &mut control, wallet, &mut completion, |outcome| {
                result = Some(match outcome {
                    JobOutcomeView::Complete { output: Some(output), .. } => copy_router_effect_payload(output).map_or_else(RouterEffectJobOutcome::Fault, RouterEffectJobOutcome::Complete),
                    JobOutcomeView::Complete { output: None, .. } => RouterEffectJobOutcome::Complete(Vec::new()),
                    JobOutcomeView::Cancelled { .. } => RouterEffectJobOutcome::Cancelled,
                    JobOutcomeView::Fault { detail, .. } => match copy_router_effect_payload(detail) {
                        Ok(detail) => RouterEffectJobOutcome::Fault(String::from_utf8_lossy(&detail).into_owned()),
                        Err(error) => RouterEffectJobOutcome::Fault(error),
                    },
                    _ => RouterEffectJobOutcome::Fault("router effect compute session returned a nonterminal outcome".to_string()),
                });
            })
            .await
        {
            Ok(true) => {
                admitted = true;
                break;
            }
            Ok(false) => {}
            Err(error) => {
                drop(control);
                close_unadmitted_router_effect(runtime, ctx, &mut job, &mut params, &mut sequence, authority, wallet).await;
                return RouterEffectJobOutcome::Fault(error.to_string());
            }
        }
        runtime.sleep_until(runtime.now_ms().await.saturating_add(1)).await;
    }
    drop(control);
    let Some(mut completion) = completion else {
        close_unadmitted_router_effect(runtime, ctx, &mut job, &mut params, &mut sequence, authority, wallet).await;
        return if admitted { RouterEffectJobOutcome::WorkerLost } else { RouterEffectJobOutcome::Fault("router effect compute admission was never granted".to_string()) };
    };
    let compute_result = completion.result().clone();
    let faulted = completion.fault().is_some();
    for _ in 0..ROUTER_EFFECT_CLOSE_TURNS {
        if completion.terminal_is_empty() {
            break;
        }
        let Ok(grant) = semio_framework_os_services::ComputeRetainedRecipient::remaining_grant(&*wallet) else { break };
        let mut progress = RetainedCloneProgress::default();
        let step = {
            let Ok(mut cx) = StepContext::with_payload_authority(operation, generation, StepBudget::new(1, u64::MAX, grant), &ctx.cancel, semio_framework_job::default_now_us, &mut sequence, &mut progress, authority) else { break };
            completion.close_step(&mut cx)
        };
        let blocked = matches!(step, Ok(semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked));
        if step.is_err() || semio_framework_os_services::ComputeRetainedRecipient::record_progress(wallet, progress).is_err() {
            break;
        }
        if blocked {
            runtime.sleep_until(runtime.now_ms().await.saturating_add(1)).await;
        }
    }
    if !completion.terminal_is_empty() {
        return RouterEffectJobOutcome::WorkerLost;
    }
    drop(completion);
    match compute_result {
        Ok(()) if !faulted => result.unwrap_or(RouterEffectJobOutcome::WorkerLost),
        Ok(()) => RouterEffectJobOutcome::WorkerLost,
        Err(ComputeError::Cancelled) => RouterEffectJobOutcome::Cancelled,
        Err(ComputeError::DeadlineExceeded) => RouterEffectJobOutcome::DeadlineExceeded,
        Err(ComputeError::WorkerLost) => RouterEffectJobOutcome::WorkerLost,
    }
}

/// 🧹️ Retires the original job and parameters of an effect whose compute admission was refused.
async fn close_unadmitted_router_effect<R: HostAsyncRuntime>(runtime: &R, ctx: &OperationContext, job: &mut Option<DynRouterEffectJob>, params: &mut Option<semio_framework_job_worker_authority::BatchJobParams>, sequence: &mut u64, authority: &semio_framework_job_payload::JobPayloadAuthority, wallet: &mut RouterEffectWallet) {
    use semio_framework_job::{Generation,OperationId,RetainedCloneProgress,StepBudget};
    use semio_framework_os_services::ComputeRetainedRecipient;
    let (operation, generation) = (OperationId(ctx.trace.0), Generation(u64::from(ctx.generation)));
    for _ in 0..ROUTER_EFFECT_CLOSE_TURNS {
        let Some(original) = job.as_mut() else { break };
        original.begin_close();
        if original.terminal_is_empty() {
            drop(job.take());
            break;
        }
        let Ok(grant) = wallet.remaining_grant() else { break };
        let mut progress = RetainedCloneProgress::default();
        let step = {
            let Ok(mut cx) = StepContext::with_payload_authority(operation, generation, StepBudget::new(1, u64::MAX, grant), &ctx.cancel, semio_framework_job::default_now_us, sequence, &mut progress, authority) else { break };
            original.close_step(&mut cx)
        };
        let blocked = matches!(step, Ok(semio_framework_job::InteractiveJobCloseStep::Blocked));
        if step.is_err() || wallet.record_progress(progress).is_err() {
            break;
        }
        if blocked {
            runtime.sleep_until(runtime.now_ms().await.saturating_add(1)).await;
        }
    }
    if let Some(semio_framework_job_worker_authority::BatchJobParams { cancel, .. }) = params.take() {
        let mut retirement = semio_framework_async::CancelTokenRetirement::from_token(cancel);
        if let Ok(grant) = wallet.remaining_grant() {
            let _ = retirement.return_alias_step(&ctx.cancel, grant);
        }
    }
}

/// 🚧️ Default until a real handler is wired (mirrors `UnwiredHttpTransport`'s own honest-gap
/// pattern in `semio-framework-os-services`) — every call fails loudly.
pub struct UnwiredRouterEffectHandler;

impl RouterEffectHandler for UnwiredRouterEffectHandler {
    fn create_job(&self, effect: RouterEffect) -> Box<dyn InteractiveJob + Send> {
        Box::new(FaultRouterEffectJob { detail: Some(format!("AsyncEffectExecutor: no RouterEffectHandler wired yet for {effect:?} (see the packet report's honest gaps)").into_bytes()), writer: None, cursor: 0, closing: false })
    }
}
//#endregion 🌉️RouterEffectHandler
