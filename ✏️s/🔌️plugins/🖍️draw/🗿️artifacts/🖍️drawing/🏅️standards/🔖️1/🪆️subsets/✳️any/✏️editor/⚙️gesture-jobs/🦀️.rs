//! ⚙️ Drawing gesture jobs: the gesture operation payload, the retained command decoder, the interactive gesture job and its tool job factory.

use super::*;

//#region 🧵️GestureOperationJobs
pub(super) struct DrawingGestureOperationPayload {
    pub(super) source_identity: SceneIdentity,
    pub(super) command: DrawingCommand,
    pub(super) snapshot: std::sync::Arc<DrawingSnapshot>,
    pub(super) config: std::sync::Arc<NoConfig>,
    pub(super) window_config: DrawingCanvasWindowConfig,
    pub(super) window_transient: DrawingCanvasWindowTransient,
    pub(super) view_state: semio_framework_plugin::ViewModel,
    pub(super) history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    pub(super) interaction_state: std::sync::Arc<protocol::InteractionState>,
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) operation_context: semio_framework_plugin::AppOperationContext,
    pub(super) active_utility_id: String,
    pub(super) completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<DrawingPlayApp>>,
}

/// 📦️ Validates the retained wire pages of one gesture against the typed command the host already
/// decoded: the pages are the command's OWN binary op encoding (`app_commands!` `OpBinary`, the same
/// bytes `ArtifactRetainedCommandJob` decodes for the bounded lane — since 09-15 the host crosses a
/// pack, not `["verb", {...}]` JSON text). Fail-closed exactly like the framework lane: the bytes
/// must decode to a command whose id is the registered verb, so truncation and a variant swap reject
/// before dispatch. No byte-equal re-encode: the host's pack encoder is not obliged to emit the
/// Rust encoder's canonical layout (float/int spelling, defaulted fields).
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
    pub pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<DrawingPlayApp>>>,
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
        DrawingGestureOperationJob { payload: Some(payload), pending_completion_rejection: None, raw_input: None, raw_page_cursor: 0, raw_byte_cursor: 0, decoder: None, raw_validated: true, completed: false, closing: false, outcomes: semio_s_artifact_draw_drawing_host::outcome::DrawingJobOutcomes::new() }
    }

    pub(super) fn from_wire(payload: DrawingGestureOperationPayload, input: semio_framework_action_bus::RetainedToolWireInput) -> Self {
        let expected_verb = payload.command.command_id();
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
                context.consume_fuel(page.len().max(1) as u64);
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
            let emit = payload.instance_owner.with_mut::<DrawingInstanceOperationOwner, _>(|owner| owner.dispatch(payload,context));
            let (emit, transient) = match emit {
                Ok(Some(output)) => output,
                Ok(None) => {
                    context.consume_fuel(1);
                    return DrawingJobTurn::Yield;
                }
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return self.outcomes.fault("drawing.gesture-dispatch-refused");
                }
            };
            let window_transient = (transient != payload.window_transient)
                .then(|| canvas_window::transient::addressed(&payload.view_state, transient))
                .transpose();
            let ephemeral = match window_transient {
                Ok(Some(mutation)) => semio_framework_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() },
                Ok(None) => semio_framework_plugin::EphemeralEmit::default(),
                Err(error) => {
                    if let Err(rejected) = payload.completion.complete(Err(error), semio_framework_plugin::EphemeralEmit::default()) {
                        self.pending_completion_rejection = Some(rejected);
                    }
                    return self.outcomes.fault("drawing.gesture-transient-refused");
                }
            };
            if let Err(rejected) = payload.completion.complete(Ok(emit), ephemeral) {
                self.pending_completion_rejection = Some(rejected);
                return self.outcomes.fault("drawing.gesture-completion-rejected");
            }
            self.completed = true;
            context.consume_fuel(1);
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
        if self.payload.is_some() {
            return Ok(unit);
        }
        if let Some(decoder) = self.decoder.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: decoder.raw.capacity(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }
}

impl DrawingGestureOperationJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        let turn = self.turn(context);
        self.outcomes.lend(turn, context)
    }
}

impl semio_framework_job::InteractiveJob for DrawingGestureOperationJob {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

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
        semio_framework_job::close_original_granted_step(cx,|grant|{
            let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_release_bytes;
            let pending=|copied_items,released_bytes|semio_framework_job::InteractiveJobCloseStep::Pending {progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items,released_bytes,..Default::default()}};
            if !self.closing {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            if !self.outcomes.terminal_is_empty() {
                return self.outcomes.close_job_step(grant);
            }
            if let Some(rejected) = self.pending_completion_rejection.as_mut() {
                if let Ok(emit) = rejected.emit.as_mut() {
                    match emit.close_child_one(grant) {
                        Ok(Some(semio_framework_plugin::PluginLifecycleStep::Progress(progress) | semio_framework_plugin::PluginLifecycleStep::Complete(progress))) => return Ok(semio_framework_job::InteractiveJobCloseStep::Pending { progress }),
                        Ok(Some(semio_framework_plugin::PluginLifecycleStep::Blocked { .. } | semio_framework_plugin::PluginLifecycleStep::AwaitingInput { .. })) => return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked),
                        Ok(None) => {}
                        Err(_) => return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "interactive job close refused")),
                    }
                }
                if maximum_items == 0 {
                    return Ok(pending(0,0));
                }
                self.pending_completion_rejection = None;
                return Ok(pending(1,0));
            }
            if maximum_items == 0 {
                return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
            }
            if let Some(input) = self.raw_input.as_mut() {
                let step = input.close_step(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,..grant})?;
                if input.terminal_is_empty() {
                    self.raw_input = None;
                }
                return Ok(match step {
                    semio_framework_job::InteractiveJobCloseStep::Complete {progress} => semio_framework_job::InteractiveJobCloseStep::Pending {progress},
                    other => other,
                });
            }
            if self.payload.take().is_some() {
                return Ok(pending(1,0));
            }
            if let Some(decoder)=self.decoder.as_mut() {
                let bytes=decoder.raw.capacity();
                if bytes>maximum_bytes {return Ok(pending(0,0));}
                drop(std::mem::take(&mut decoder.raw));self.decoder=None;
                return Ok(pending(1,bytes));
            }
            Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()})
        })
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(self.close_demands(0)?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.pending_completion_rejection.is_none() && self.payload.is_none() && self.raw_input.is_none() && self.decoder.is_none() && self.outcomes.terminal_is_empty()
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

    semio_framework_plugin::retained_command::typed_tool_job_factory_admission!(|operation, payload| Ok::<_, String>(DrawingGestureOperationJob::new(payload)));
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = DRAWING_GESTURE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = DRAWING_GESTURE_PUBLICATION_CONTRACTS;
}

/// 🏗️ The gesture half of `DrawingPlayApp::build_tool_job` — the Canvas window's live view, config and transient plus the retained wire owner of the command enter one gesture job.
pub(super) fn drawing_gesture_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<DrawingPlayApp>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
    use semio_framework_plugin::__value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
    let original = request.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Drawing gesture request is absent"))?;
    if original.command.command_id() != original.tool_id {
        return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing gesture command does not match its exact registered tool").into());
    }
    if original.context.view_state.is_none() {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing gesture commands require one concrete Canvas window instance").into());
    }
    if original.checkpoint.is_some() || original.input.as_ref().is_some_and(|input| input.declared_bytes() > DRAWING_GESTURE_RAW_BYTES) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing gesture retained ingress rejects a checkpoint or oversized wire owner").into());
    }
    let demand = semio_framework::action_bus::ToolJobAdmissionDemand { items: 1, copy_bytes: 0, capacity_bytes: std::mem::size_of::<DrawingGestureOperationJob>() + std::mem::size_of::<DrawingGestureOperationPayload>(), release_bytes: 0, depth: 1 };
    semio_framework::action_bus::admit_original_tool_construction(cx, output, demand, |_| {
        let mut request = request.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Drawing gesture request is absent"))?;
        let view_state = request.context.view_state.clone().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "Drawing gesture commands require one concrete Canvas window instance"))?;
        let window_config = canvas_window::config::from_snapshot(request.context.window_config.as_ref());
        let window_transient = canvas_window::transient::from_snapshot(request.context.window_transient.as_ref());
        let input = request.input.take();
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = DrawingGestureOperationPayload {
            source_identity: SceneIdentity { instance: request.app_instance_id, base: request.operation.base_revision.0, generation: request.operation.generation.0, revision: request.canonical_base_revision },
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            window_config,
            window_transient,
            view_state,
            history: request.history,
            interaction_state: request.interaction_state,
            instance_owner: request.instance_operation_owner,
            operation_context,
            active_utility_id: request.context.view_state.as_ref().map_or(DRAWING_DEFAULT_UTILITY, drawing_active_utility).to_owned(),
            completion: request.completion,
        };
        let job = match input {
            Some(input) => DrawingGestureOperationJob::from_wire(payload, input),
            None => DrawingGestureOperationJob::new(payload),
        };
        Ok((semio_framework_plugin::ErasedToolJob::from_owned(Box::new(job)), RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: std::mem::size_of::<DrawingGestureOperationJob>(), ..Default::default() }))
    })
}
//#endregion 🧵️GestureOperationJobs
