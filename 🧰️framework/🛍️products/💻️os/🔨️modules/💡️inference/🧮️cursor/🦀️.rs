//! 🧮️ Resumable inference cursor of the inference cache: the original scope, the cursor with its phases and the step report. See [`super`].

use super::*;

//#region 🔖️Cursor
/// 🏡️ Retains the exact source parent, recipient, clock and deadline of an original field run.
pub(super) struct OriginalInferenceScope {
    pub(super) parent: usize,
    recipient: usize,
    pub(super) clock: fn() -> Option<u64>,
    clock_binding:Option<u64>,
    pub(super) deadline: u64,
}

impl OriginalInferenceScope {
    pub(super) fn capture(cx: &semio_framework_job::StepContext<'_>) -> Result<Self, semio_framework_value::ValueError> {
        Ok(Self { parent: cx.original_source_parent()?.source_identity()?, recipient: cx.original_recipient_identity(), clock: cx.clock_source(),clock_binding:cx.original_clock_binding(), deadline: cx.original_absolute_deadline_us() })
    }
    pub(super) fn matches(&self, cx: &semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        Ok(self.parent == cx.original_source_parent()?.source_identity()? && self.recipient == cx.original_recipient_identity() && std::ptr::fn_addr_eq(self.clock, cx.clock_source()) && self.clock_binding==cx.original_clock_binding() && self.deadline == cx.original_absolute_deadline_us())
    }
}

/// 🧷 The explicit, resumable position of one field run: the plan, the next entity, the dependency hash of every finished
/// entity and the compute in flight. The caller owns it between [`infer_field_step`] calls and may [`cancel`](Self::cancel) it.
pub struct InferenceCursor<K> {
    pub(super) original_field_source: Option<usize>,
    pub(super) plan: Option<Vec<InferenceStep<K>>>,
    pub(super) next: usize,
    pub(super) hashes: HashMap<K, DepHash>,
    pub(super) pending: Option<Box<dyn InferencePending>>,
    pub(super) progress: f32,
    pub(super) cancelled: bool,
    pub(super) original_caller:Option<(u64,u64,usize)>,
    pub(super) original_scope:Option<OriginalInferenceScope>,
    pub(super) original_plan:Option<OriginalInferencePlan>,
    pub(super) original_phase:OriginalInferencePhase,
    pub(super) original_edge:usize,
    pub(super) original_parent_count:usize,
}

#[derive(Default)]
pub(super) enum OriginalInferencePhase{#[default] CountParents,ReserveParents,CopyParents,Compute,ClosePending,CloseParents,Publish,Advance}

impl<K> Default for InferenceCursor<K> {
    fn default() -> Self {
        Self { plan: None, next: 0, hashes: HashMap::new(), pending: None, progress: 0.0, cancelled: false, original_caller:None,original_field_source:None,original_scope:None,original_plan:None,original_phase:Default::default(),original_edge:0,original_parent_count:0 }
    }
}

impl<K: Eq + std::hash::Hash> InferenceCursor<K> {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🛑 Cancels the in-flight compute and makes every later step refuse with [`InferenceError::Cancelled`]; finished values stay valid.
    pub fn cancel(&mut self) {
        if let Some(pending) = self.pending.as_mut() {
            pending.cancel();
        }
        self.cancelled = true;
    }

    /// 🧭️ Borrows the funded order and original source ordinals retained by this run.
    pub fn original_plan(&self)->Option<&OriginalInferencePlan>{self.original_plan.as_ref()}
        /// 📏️ Observes the exact native pending owner selected by the cursor without moving or closing it.
    pub fn original_pending(&self)->Option<&dyn InferencePending>{self.pending.as_deref()}
    /// 📏️ Quotes the exact retained plan close or its metadata/source return.
    pub fn original_plan_retirement_demands(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.pending.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending must close before plan demand"))}if let Some(plan)=self.original_plan.as_ref(){if !plan.terminal_is_empty(){return plan.close_demands()}}Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
/// 🧹️ Keeps original dynamic pending payload and box custody through every denied closure turn.
    pub fn close_original_pending_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        if let Some(source)=self.original_scope.as_ref(){if !source.matches(cx)?{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference pending retirement source changed").into())}}
        if self.original_caller.is_some_and(|source|source.0!=cx.operation().0||source.1!=cx.generation().0){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference pending retirement caller changed").into())}
        let Some(pending)=self.pending.as_mut()else{return Ok(true)};
        if !pending.close_original(cx)?{return Ok(false)}
        let bytes=pending.original_box_release_bytes().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original pending box has no complete physical release demand"))?;let grant=cx.retained_grant();
        if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<bytes{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.pending.take();Ok(true)
    }
    /// ♻️ Closes the original planned graph without replacing its caller or dropping paid source pages.
    pub fn close_original_plan_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        if !self.close_original_pending_step(cx)?{return Ok(false)}
        if self.original_caller.is_some_and(|source|source.0!=cx.operation().0||source.1!=cx.generation().0){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference plan retirement caller changed").into())}
        if let Some(plan)=self.original_plan.as_mut(){if !plan.close_step(cx)?{return Ok(false)}if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_plan=None;return Ok(false)}
        if self.original_caller.is_some(){if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.original_caller=None;self.original_field_source=None;self.original_scope=None;return Ok(false)}
        Ok(true)
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// ✅ Entities finished so far.
    pub fn completed(&self) -> usize {
        self.next
    }

    /// 🧮 Entities in the plan; zero until the first step has planned.
    pub fn total(&self) -> usize {
        self.original_plan.as_ref().map_or_else(||self.plan.as_ref().map_or(0,Vec::len),OriginalInferencePlan::len)
    }

    /// 📈 Finished entities plus the in-flight compute's own progress, as a fraction of the plan.
    pub fn fraction(&self) -> f32 {
        match self.total() {
            0 => 0.0,
            total => ((self.next as f32 + self.progress.clamp(0.0, 1.0)) / total as f32).min(1.0),
        }
    }

    pub fn is_done(&self) -> bool {
        self.original_plan.as_ref().map_or_else(||self.plan.as_ref().is_some_and(|plan|self.next>=plan.len()),|plan|plan.complete()&&self.next>=plan.len())
    }

    /// 🔗 The dependency hash of a finished entity.
    pub fn hash(&self, key: &K) -> Option<DepHash> {
        self.hashes.get(key).copied()
    }
}

/// 📋 What one driver call did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InferenceStepReport {
    pub done: bool,
    pub fuel_used: usize,
    pub computed: usize,
    pub hits: usize,
    pub progress: f32,
}
//#endregion 🔖️Cursor
