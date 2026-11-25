//! ⚙️ Retained Drawing gesture job and factory.

use super::*;

//#region 🧵️GestureOperationJobs
type GestureRequest = semio_plugin::ArtifactOwnedToolJobRequest<semio_plugin::EditorApp<DrawingPlayApp>>;

pub struct DrawingGestureOperationPayload {
    pub(super) source_identity: SceneIdentity,
    pub(super) original: Option<GestureRequest>,
}

/// 📦️ Decodes bounded pages of the original binary drawing command and checks its registered verb.
/// The host may encode equivalent values with a different Pack layout.
pub struct DrawingRetainedCommandDecoder {
    expected_verb: &'static str,
    pub(super) raw: Vec<u8>,
    overflow: bool,
}

impl DrawingRetainedCommandDecoder {
    pub fn new(expected_verb: &'static str) -> Self {
        Self { expected_verb, raw: Vec::new(), overflow: false }
    }

    /// 📄️ Appends one wire page within `DRAWING_GESTURE_RAW_BYTES`; an oversized owner fails closed.
    pub fn feed_page(&mut self, page: &[u8]) -> bool {
        if self.overflow || self.raw.len().checked_add(page.len()).is_none_or(|end| end > DRAWING_GESTURE_RAW_BYTES) || self.raw.try_reserve_exact(page.len()).is_err() {
            self.overflow = true;
            return false;
        }
        self.raw.extend_from_slice(page);
        true
    }

    pub fn finish(&self) -> bool {
        if self.overflow {
            return false;
        }
        let Ok(decoded) = <DrawingCommand as ::protocol::OpBinary>::decode_op(&self.raw) else { return false };
        decoded.command_id() == self.expected_verb
    }
}

pub struct DrawingGestureOperationJob {
    pub(super) payload: Option<DrawingGestureOperationPayload>,
    retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<GestureRequest>>,
    pub pending_completion_rejection: Option<semio_plugin::app::ArtifactToolCompletionRejection<semio_plugin::EditorApp<DrawingPlayApp>>>,
    pub(super) raw_input: Option<semio_framework_action_bus::RetainedToolWireInput>,
    pub(super) raw_page_cursor: usize,
    pub(super) raw_byte_cursor: usize,
    pub decoder: Option<DrawingRetainedCommandDecoder>,
    pub(super) raw_validated: bool,
    pub(super) completed: bool,
    pub(super) closing: bool,
    pub(super) outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes,
}

impl DrawingGestureOperationJob {
    pub(super) fn new(payload: DrawingGestureOperationPayload) -> Self {
        DrawingGestureOperationJob { payload: Some(payload), retirement: Default::default(), pending_completion_rejection: None, raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false, outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes::new() }
    }

    pub(super) fn from_wire(payload: DrawingGestureOperationPayload, input: semio_framework_action_bus::RetainedToolWireInput) -> Self {
        let expected_verb = payload.original.as_ref().expect("original gesture request").command.command_id();
        let mut job = Self::new(payload);
        job.raw_input = Some(input);
        job.decoder = Some(DrawingRetainedCommandDecoder::new(expected_verb));
        job.raw_validated = false;
        job
    }

    pub(super) fn turn(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_s_artifact_draw_drawing_host::outcome::DrawingJobTurn {
        use semio_s_artifact_draw_drawing_host::outcome::DrawingJobTurn;
        if self.outcomes.faulted() {
            return DrawingJobTurn::Fault;
        }
        if context.is_cancelled() {
            return DrawingJobTurn::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return DrawingJobTurn::Yield;
        }
        if !self.raw_validated {
            let Some(input) = self.raw_input.as_ref() else { return self.outcomes.fault("drawing.gesture-wire-input-missing") };
            if let Some(page) = input.page(self.raw_page_cursor) {
                let Some(decoder) = self.decoder.as_mut() else {
                    return self.outcomes.fault("drawing.gesture-wire-decoder-missing");
                };
                if !decoder.feed_page(page) {
                    return self.outcomes.fault("drawing.gesture-wire-oversized");
                }
                self.raw_page_cursor += 1;
                self.raw_byte_cursor = 0;
                context.consume_fuel(1);
                return DrawingJobTurn::Yield;
            }
            let exact = self.decoder.as_ref().is_some_and(DrawingRetainedCommandDecoder::finish);
            if !exact {
                return self.outcomes.fault("drawing.gesture-wire-command-mismatch");
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            return DrawingJobTurn::Yield;
        }
        if self.pending_completion_rejection.is_some() {
            return self.outcomes.fault("drawing.gesture-completion-rejected");
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return self.outcomes.fault("drawing.gesture-payload-missing") };
            let Some(original) = payload.original.as_ref() else { return self.outcomes.fault("drawing.gesture-payload-missing") };
            let view = original.context.view_state.as_deref().expect("admitted original Drawing Canvas view");
            let emit = original.instance_operation_owner.with_mut::<DrawingInstanceOperationOwner, _>(|owner| owner.dispatch(payload,context));
            let (emit, transient) = match emit {
                Ok(Some(output)) => output,
                Ok(None) => {
                    context.consume_fuel(context.fuel_remaining().min(1));
                    return DrawingJobTurn::Yield;
                }
                Err(error) => {
                    if let Err(rejected) = original.completion.complete(Err(error), semio_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                        return self.outcomes.fault("drawing.gesture-completion-rejected");
                    }
                    self.completed = true;
                    context.consume_fuel(context.fuel_remaining().min(1));
                    return DrawingJobTurn::Complete;
                }
            };
            let default_transient = DrawingCanvasWindowTransient::default();
            let previous_transient = original.context.window_transient.as_ref()
                .and_then(|snapshot| snapshot.get::<canvas_window::transient::DrawingCanvasWindowTransientOwner>())
                .unwrap_or(&default_transient);
            let window_transient = (&transient != previous_transient)
                .then(|| canvas_window::transient::addressed(view, transient))
                .transpose();
            let ephemeral = match window_transient {
                Ok(Some(mutation)) => semio_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
                Ok(None) => semio_plugin::EphemeralEmit::default(),
                Err(error) => {
                    if let Err(rejected) = original.completion.complete(Err(error), semio_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                        return self.outcomes.fault("drawing.gesture-completion-rejected");
                    }
                    self.completed = true;
                    context.consume_fuel(context.fuel_remaining().min(1));
                    return DrawingJobTurn::Complete;
                }
            };
            if let Err(rejected) = original.completion.complete(Ok(emit), ephemeral) {
                self.pending_completion_rejection = Some(rejected);
                return self.outcomes.fault("drawing.gesture-completion-rejected");
            }
            self.completed = true;
            context.consume_fuel(context.fuel_remaining().min(1));
        }
        DrawingJobTurn::Complete
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let unit = semio_framework_value::RetirementDemand { depth: 1, ..Default::default() };
        if !self.outcomes.terminal_is_empty() {
            return self.outcomes.retirement_demands();
        }
        if let Some(rejected) = self.pending_completion_rejection.as_ref() {
            if let Ok(emit) = rejected.emit.as_ref() {
                if let Some(demand) = emit.child_close_demands(body)? {
                    return Ok(demand);
                }
            }
            return Ok(unit);
        }
        if let Some(input) = self.raw_input.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: input.next_close_copy_byte_demand()?, capacity_bytes: input.next_close_capacity_byte_demand(body)?, release_bytes: input.next_close_release_byte_demand()?, depth: input.next_close_depth_demand()? });
        }
        if let Some(payload) = self.payload.as_ref() {
            if payload.original.is_some() || self.retirement.is_some() {
                return semio_plugin::app::plugin_typed_owner_demand(&payload.original, &self.retirement, body);
            }
            return Ok(unit);
        }
        if let Some(decoder) = self.decoder.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: decoder.raw.capacity(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }
}

impl DrawingGestureOperationJob {
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::InteractiveJobStep<'a> {
        let turn = self.turn(context);
        self.outcomes.lend(turn, context)
    }
}

impl semio_framework_job::InteractiveJob for DrawingGestureOperationJob {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>,_claim:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{self.step_value(context)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.closing && !self.outcomes.terminal_is_empty() { return self.outcomes.close_step(cx); }
        let grant = cx.retained_grant();
        if self.closing {
            if let Some(emit) = self.pending_completion_rejection.as_mut().and_then(|rejected| rejected.emit.as_mut().ok()) {
                if let Some(demand) = emit.child_close_demands(grant.maximum_copy_bytes)? {
                    use semio_framework_job::InteractiveJobCloseStep;
                    use semio_plugin::PluginLifecycleStep;
                    let step = semio_plugin::app::original_lifecycle_effect_step(cx, grant, demand, |_, effect| {
                        let step = emit.close_child_original(effect)?.unwrap_or(PluginLifecycleStep::Progress(Default::default()));
                        Ok((step, false))
                    })?;
                    return Ok(match step {
                        PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => InteractiveJobCloseStep::Pending { progress },
                        PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. } => InteractiveJobCloseStep::Blocked,
                    });
                }
            }
        }
        if self.closing && self.pending_completion_rejection.is_none() && self.raw_input.is_none() {
            if let Some(payload) = self.payload.as_mut().filter(|payload| payload.original.is_some() || self.retirement.is_some()) {
                let demand = semio_plugin::app::plugin_typed_owner_demand(&payload.original, &self.retirement, grant.maximum_copy_bytes)?;
                return semio_framework_job::close_original_effect_step(cx, demand, |effect| {
                    semio_plugin::app::plugin_typed_owner_original(&mut payload.original, &mut self.retirement, effect)
                });
            }
        }
        semio_framework_job::close_original_granted_step(cx,|grant|{
            let mut work=0;
            let result=(||->Result<_, semio_framework_value::ValueError>{
            let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
            let pending=|copied_items,released_bytes|semio_framework_job::InteractiveJobCloseStep::Pending {progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items,released_bytes,..Default::default()}};
            if !self.closing {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            if self.pending_completion_rejection.is_some() {
                if maximum_items == 0 {
                    return Ok(pending(0,0));
                }
                work=1;
                self.pending_completion_rejection = None;
                return Ok(pending(1,0));
            }
            if maximum_items == 0 {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            if let Some(input) = self.raw_input.as_mut() {
                let step = input.close_step(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,..grant})?;
                work=u128::from(step.progress().copied_items>0);
                if input.terminal_is_empty() {
                    self.raw_input = None;
                }
                return Ok(match step {
                    semio_framework_job::InteractiveJobCloseStep::Complete {progress} => semio_framework_job::InteractiveJobCloseStep::Pending {progress},
                    other => other,
                });
            }
            if self.payload.take().is_some() {
                work=1;
                return Ok(pending(1,0));
            }
            if let Some(decoder)=self.decoder.as_mut() {
                let bytes=decoder.raw.capacity();
                if bytes>maximum_bytes {return Ok(pending(0,0));}
                work=1;
                drop(std::mem::take(&mut decoder.raw));self.decoder=None;
                return Ok(pending(1,bytes));
            }
            Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()})
            })();
            semio_framework_value::RetainedWorkOutcome{result,work}
        })
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.payload.is_none() && self.retirement.is_none() && self.raw_input.is_none() && self.decoder.is_none() && self.outcomes.terminal_is_empty()
    }
}

pub struct DrawingGestureOperationJobFactory {
    pub(super) keys: Vec<semio_framework::ToolFactoryKey>,
}

impl DrawingGestureOperationJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: DRAWING_GESTURE_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller_id, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for DrawingGestureOperationJobFactory {
    type Payload = DrawingGestureOperationPayload;
    type Job = DrawingGestureOperationJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        DRAWING_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(DRAWING_GESTURE_RAW_BYTES, 32, 1, 16_384, 7_500, 1, 1)
    }

    semio_plugin::retained_command::typed_tool_job_factory_admission!(|operation, payload| Ok::<_, String>(DrawingGestureOperationJob::new(payload)));
}

impl semio_plugin::ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory {
    type Owner = semio_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_GESTURE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = DRAWING_GESTURE_PUBLICATION_CONTRACTS;
}

/// 🏗️ Admits the gesture job on the original Canvas request.
pub(super) fn drawing_gesture_tool_job(request: &mut Option<GestureRequest>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
    use semio_plugin::__value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Drawing gesture request is absent"))?;
    if !original.source_binding.matches(cx) {
        return Err(ValueError::invariant("Drawing gesture construction requires its original source").into());
    }
    if original.command.command_id() != original.tool_id {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing gesture command does not match its exact registered tool").into());
    }
    if original.context.view_state.is_none() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing gesture commands require one concrete Canvas window instance").into());
    }
    if original.checkpoint.is_some() || original.input.as_ref().is_some_and(|input| input.declared_bytes() > DRAWING_GESTURE_RAW_BYTES) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing gesture retained ingress rejects a checkpoint or oversized wire owner").into());
    }
    if let Some(input) = original.input.as_ref() {
        original.command.admitted_encoded_bytes(input, cx)?;
    }
    let capacity = std::mem::size_of::<DrawingGestureOperationJob>();
    let demand = semio_framework::action_bus::ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: capacity, release_bytes: 0, depth: 1 };
    let owner = request.as_mut().ok_or_else(|| ValueError::invariant("original Drawing gesture request is absent"))?;
    let built = owner.admit_tool(cx, demand, |inputs, scope| {
        scope.admit_owner(semio_plugin::__value::RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() })?;
        let mut request = inputs.take().expect("admitted original Drawing gesture request");
        let input = request.input.take();
        let payload = DrawingGestureOperationPayload {
            source_identity: SceneIdentity { instance: request.app_instance_id, base: request.operation.base_revision.0, generation: request.operation.generation.0, revision: request.canonical_base_revision },
            original: Some(GestureRequest::new(request)),
        };
        let job = match input {
            Some(input) => DrawingGestureOperationJob::from_wire(payload, input),
            None => DrawingGestureOperationJob::new(payload),
        };
        Ok((semio_plugin::ErasedToolJob::from_owned(Box::new(job)), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
    })?;
    if built { semio_plugin::ArtifactOwnedToolJobRequest::finish_tool(request, output, cx) } else { Ok(false) }
}
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧵️GestureOperationJobs
