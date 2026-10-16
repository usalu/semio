//! 🧾️ Typed operation driving, task resume, response dispatch, intents and reserved presence.

use super::*;
impl PluginExchangeOutput{
    /// 🧾️ Keeps original ownership receipts ahead of the next semantic producer.
    pub(super) fn ownership_receipt_pending(&self)->bool{self.original_receiving.is_some()||self.constructor_receipt.is_some()||self.preparation_receipt.is_some()||self.retirement_receipt.is_some()||self.history_command_receipt.is_some()||self.tool_run_receipt.is_some()}
    fn collect_original_receipts<PA:PluginApp>(&mut self,app:&mut PA){self.constructor_receipt=app.take_constructor_receipt();self.preparation_receipt=app.take_typed_operation_preparation_receipt();self.retirement_receipt=app.take_typed_operation_retirement_receipt();self.history_command_receipt=app.take_history_command_receipt();self.tool_run_receipt=app.take_tool_run_receipt();}
    fn original_receipt_family(&self)->Option<OriginalPublicationFamily>{let mut families=[self.constructor_receipt.is_some().then_some(OriginalPublicationFamily::Constructor),self.preparation_receipt.is_some().then_some(OriginalPublicationFamily::Preparation),self.retirement_receipt.is_some().then_some(OriginalPublicationFamily::Retirement),self.history_command_receipt.is_some().then_some(OriginalPublicationFamily::History),self.tool_run_receipt.is_some().then_some(OriginalPublicationFamily::Tool)].into_iter().flatten();let first=families.next()?;families.next().is_none().then_some(first)}
}
/// 🎰️ Highest typed-operation slot occupancy any instance of this guest has reached, and whether that
/// peak still owes its drain witness — the two values that bound the diagnostic below to at most two
/// console lines per distinct peak, i.e. at most `2 × ARTIFACT_LIVE_OUTPUT_SLOTS` for a whole session.
static PEAK_TYPED_OPERATION_SLOTS: AtomicU64 = AtomicU64::new(0);
static TYPED_OPERATION_SLOT_DRAIN_PENDING: AtomicBool = AtomicBool::new(false);
/// 🎰️ Reports the guest's live typed-operation slot occupancy on the console channel the rest of this
/// runtime's `[TRACE]` diagnostics already use — no wire, no protocol frame, and nothing the host may
/// branch on. A browser probe reads `live=0` as the proof every admitted slot reached its release path;
/// a `peak` that rises and never drains is the retirement leak that refused
/// `engagementAbort`/`setCamera`/`openVortexSuggestions` 39 times in the 2026-09-11 puzzle3d battery
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8). Every new peak and its first full drain print
/// unconditionally because that pair IS the leak signal and is self-limiting; every intermediate
/// transition prints only under [`semio_framework_trace::runtime_diagnostics_enabled`].
fn trace_typed_operation_slot_occupancy<PA: PluginApp>(app: &PA, instance: u32) {
    let live = app.live_typed_operation_slots() as u64;
    let previous = PEAK_TYPED_OPERATION_SLOTS.fetch_max(live, Ordering::Relaxed);
    let peak = previous.max(live);
    let raised = live > previous;
    if raised {
        TYPED_OPERATION_SLOT_DRAIN_PENDING.store(true, Ordering::Relaxed);
    }
    let drained = live == 0 && TYPED_OPERATION_SLOT_DRAIN_PENDING.swap(false, Ordering::Relaxed);
    if !raised && !drained && !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS);
}
pub(super) fn advance_typed_operation_output_with_leftover<PA: PluginApp>(app: &mut PA, instance: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(PluginExchangeOutput, Option<TypedOperationLeftover>), crate::PluginLifecycleFailure> {
    let mut output=PluginExchangeOutput::default();output.collect_original_receipts(app);if output.ownership_receipt_pending(){return Ok((output,None))}
    let before=cx.retained_progress();let original=cx.retained_grant();let recipient=cx.original_recipient_identity();
    ::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,cx))?;
    if let Some((grant,progress))=app.take_local_interaction_query_receipt(){
        let remaining=cx.retained_grant();let covered=grant.maximum_items<=remaining.maximum_items&&grant.maximum_copy_bytes<=remaining.maximum_copy_bytes&&grant.maximum_capacity_bytes<=remaining.maximum_capacity_bytes&&grant.maximum_release_bytes<=remaining.maximum_release_bytes&&grant.maximum_depth<=remaining.maximum_depth&&progress.fits(grant);cx.consume_retained(progress)?;
        if !covered{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original interaction receipt exceeds its caller grant").with_retained_progress(progress).into())}
        return Ok((PluginExchangeOutput::default(),None));
    }
    trace_typed_operation_slot_occupancy(app, instance);
    output.collect_original_receipts(app);let after=cx.retained_progress();let subtract=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original publication recipient moved backwards"));let posted=RetainedCloneProgress{copied_items:subtract(after.copied_items,before.copied_items)?,copied_bytes:subtract(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:subtract(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:subtract(after.released_bytes,before.released_bytes)?};if posted!=RetainedCloneProgress::default(){output.original_receiving=Some(OriginalPublicationReceiving{family:output.original_receipt_family(),before,posted,grant:original,recipient});}
    if output.ownership_receipt_pending(){return Ok((output,None));}
    match app.take_reserved_commit_outcome() {
        Some(Ok(result)) => {
            output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::Invocation {
                in_reply_to: 0,
                output: encode_wire_serialized(&result.output),
                diagnostics: encode_wire_serialized(&result.diagnostics),
                ui_scope: encode_wire_serialized(&result.ui_scope),
                history_patch: encode_wire_serialized(&result.history_patch),
                messages: Vec::new(),
                mutations: encode_wire_serialized(&result.mutations),
                inverse_group: encode_wire_serialized(&result.inverse_group),
            })));
            ::semio_framework_async::poll::resolve_ready(push_invocation_side_frames(&mut output.effects, &mut output.events, &result));
        }
        Some(Err(fault)) => {
            let mut frames = Vec::new();
            ::semio_framework_async::poll::resolve_ready(push_app_fault(&mut frames, None, fault));
            for frame in frames.iter() {
                output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(frame)));
            }
        }
        None => {}
    }
    while output.typed_operation_results.len() < TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM {
        let Some(page) = app.take_typed_operation_result_page(instance) else { break };
        output.typed_operation_results.push(page);
    }
    if let Some(effect) = app.take_typed_operation_effect() {
        output.effects.push(encode_wire_serialized(&effect));
    }
    if let Some(event) = app.take_typed_operation_event() {
        output.events.push(encode_wire_serialized(&event));
    }
    if let Some(completion) = ::semio_framework_async::poll::resolve_ready(app.take_typed_operation_completion())? {
        output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::OperationCompleted {
            operation: completion.operation,
            revision: completion.revision,
            ui_scope: encode_wire_serialized(&completion.ui_scope),
            history_patch: completion.history_patch.as_ref().map(encode_wire_serialized).unwrap_or_default(),
        })));
    }
    let mut leftover = None;
    if let Some(progress) = app.take_typed_operation_ui_progress() {
        output.frames.push(::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&protocol::AppFrame::Invocation {
            in_reply_to: 0,
            output: progress.leftover.as_ref().map(|leftover| encode_wire_serialized(&leftover.view)).unwrap_or_default(),
            diagnostics: Vec::new(),
            ui_scope: encode_wire_serialized(&progress.ui_scope),
            history_patch: progress.history_patch.as_ref().map(encode_wire_serialized).unwrap_or_default(),
            messages: Vec::new(),
            mutations: Vec::new(),
            inverse_group: Vec::new(),
        })));
        leftover = progress.leftover;
    }
    app.publish_local_interaction_query_reply(&mut output.frames, 4);
    Ok((output, leftover))
}
pub(super) fn advance_typed_operation_output<PA: PluginApp>(app: &mut PA, instance: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput, crate::PluginLifecycleFailure> {
    advance_typed_operation_output_with_leftover(app, instance, identity,cx).map(|(output, _)| output)
}
/// 🕹️ How many extra typed-operation units `plugin_exchange` drives for the operation a command
/// of that exchange started, so the operation's inline interaction fold (and its leftover
/// `InteractionView`) lands inside the command's own reply. A pick with no durable lane folds
/// within a handful of units (worker step, install, fold); one with a document lane presents a
/// result page first and stops this drive on it.
pub(super) const TYPED_OPERATION_REPLY_LEFTOVER_PUMPS: usize = 16;
/// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — merges a typed operation's inline-folded
/// leftover `InteractionView` into the `Invocation` reply of the command that STARTED that
/// operation, when both are in the same exchange: the reply's `output` is the operation's
/// `{ operationId, generation }` witness, and it becomes `{ operationId, generation,
/// interactionView }` — exactly the envelope `interactionViewFromLeftoverOutput` (`🛠️ShellHelpers`)
/// peels from `response.output` in `applyLeftoverInteractionView`, so the host's leftover
/// selection overlay, Inspection tab and `INTERACTION_STATE_OBSERVED` land for a folded verb the
/// same way they landed when the verb took a host round trip. A reply that answers a different
/// operation, or no typed operation at all, is left untouched.
pub(super) fn merge_typed_operation_leftover_into_reply(frames: &mut [protocol::AppFrame], leftover: &TypedOperationLeftover) -> bool {
    let operation = leftover.operation.to_string();
    for frame in frames.iter_mut() {
        let protocol::AppFrame::Invocation { in_reply_to, output, .. } = frame else { continue };
        if *in_reply_to == 0 || output.is_empty() {
            continue;
        }
        let Ok(DslValue::Object(mut entries)) = store::pack_rt::decode_wire_value(output) else { continue };
        if !entries.iter().any(|(key, value)| key == "operationId" && value.as_str() == Some(operation.as_str())) {
            continue;
        }
        let DslValue::Object(envelope) = &leftover.view else { return false };
        entries.retain(|(key, _)| !envelope.iter().any(|(leftover_key, _)| leftover_key == key));
        entries.extend(envelope.iter().cloned());
        *output = store::pack_rt::encode_wire_value(&DslValue::Object(entries));
        return true;
    }
    false
}
/// 🔁️ What a typed-operation scan found: a runnable instance, or only a busy one.
///
/// A busy instance is NOT runnable work — it is another owner holding the same lock — and the two
/// used to fold into one `bool` that the reactor turn reported as `MoreWork` either way. An idle
/// app that answers `MoreWork` because a mounted worker session held the lock for a microsecond
/// costs the host a whole turn round trip for nothing, so the turn now weighs them separately.
/// See `📓️idle-turns-2026-09-10.md`.
#[derive(Clone, Copy, Default)]
pub struct TypedOperationScan {
    pub runnable: bool,
    pub contended: bool,
}
/// ⏱️ Wall milliseconds one turn may spend driving typed-operation units — the reactor executor's
/// own slice, so the continuation phase and the executor phase cost an interactive turn the same.
pub const TYPED_OPERATION_SLICE_MS: u64 = 8;
/// 🔁️ Units one turn may drive before it hands the host its turn back regardless of the clock.
/// A guest whose host clock is missing still terminates; 256 covers every staged work this
/// runtime admits (the heaviest measured is a 5.7 KB `registerBrushMesh` at ~84).
pub const TYPED_OPERATION_UNITS_PER_TURN: u32 = 256;
/// 📄️ Result pages ONE turn may hand the host, and therefore the widest acknowledgement batch the
/// shell posts back in one crossing.
///
/// 🐛️ A turn used to carry EXACTLY ONE page: [`advance_typed_operation_output`] took a single page
/// and [`TypedOperationGrant::spent`] ended the drive the instant it existed, so every
/// acknowledgeable publication unit of every live operation cost its own host round trip. Measured
/// on the React door of 🧊️generation3d (`📓️shard-message-crossings-2026-09-15.md` §2): **200 ack
/// crossings over 69 operations, every single one carrying exactly ONE ack** — 10.0 of a
/// `flowEvalTick` hop's 36.2 worker crossings, each a full structured-clone round trip to retire 53
/// bytes. A page's ACK is per-page by protocol; the CROSSING that carries it never had to be.
/// `Event::Message` is a list on the wire and the reactor's poll already acknowledges every entry of
/// it, so N presentable pages retire in ONE crossing — the same cardinality change
/// `📓️ui-turn-patch-batching-2026-09-15.md` made for `patch-ack`.
///
/// Each operation still retains exactly one unacknowledged page (`queue_page`), so this bound is
/// over DISTINCT live operations, never over one operation's own sequence.
pub const TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM: usize = 8;
/// 🔁️ What one turn may spend continuing typed operations, and what stops it early.
///
/// [`plugin_continue_typed_operations`] advanced EXACTLY ONE publication unit per call and the
/// reactor calls it once per turn, so every unit of a staged operation cost the host a whole turn
/// round trip. A trivial `compositeEdit` reached its terminal result in 48 turns
/// (`retained_operation_continues_after_command_admission_until_publication_and_retirement`) and a
/// 5.7 KB `registerBrushMesh` in ~84 (`📓️2026-09-12-wave-B22-brush-mesh-upload.md` §1.3). A turn
/// now drives units until one of four bounds, and every one of them is the protocol's own rather
/// than a pacing constant: a result page the host must acknowledge before the operation may
/// continue, the turn's declared frame and effect capacity, [`Self::deadline`], and [`Self::units`].
/// Ticket 26/09/02 wave B24.
#[derive(Clone, Copy)]
pub struct TypedOperationGrant {
    pub units: u32,
    pub max_frames: usize,
    pub max_effects: usize,
    pub deadline_us: Option<u64>,
    pub clock:fn()->Option<u64>,
}
impl TypedOperationGrant {
    /// 🔁️ One unit and nothing more — for a law that steps the continuation deliberately.
    #[cfg(test)]
    pub const UNIT: Self = Self { units: 1, max_frames: usize::MAX, max_effects: usize::MAX, deadline_us: None,clock:semio_framework_job::logical_now_us };
    /// 🔁️ The grant a turn's own [`semio_framework::kernel::Budget`] declares.
    pub fn turn(budget: semio_framework::kernel::Budget,cx:&semio_framework_job::StepContext<'_>) -> Self {
        Self {
            units: TYPED_OPERATION_UNITS_PER_TURN.min(u32::try_from(cx.fuel_remaining()).unwrap_or(u32::MAX)),
            max_frames: budget.max_frames as usize,
            max_effects: budget.max_effects as usize,
            deadline_us:Some(cx.deadline_us()),
            clock:cx.clock_source(),
        }
    }
    /// 🛑️ Whether the collected output has reached what one turn may carry. An EMPTY collection
    /// never counts as reached, so a budget declaring zero frames still drives units rather than
    /// silently restoring the one-unit-per-turn pacing this grant exists to remove.
    pub(super) fn spent(&self, output: &PluginExchangeOutput) -> bool {
        output.original_receiving.is_some() || output.constructor_receipt.is_some() || output.preparation_receipt.is_some() || output.retirement_receipt.is_some() || output.history_command_receipt.is_some() || output.tool_run_receipt.is_some() || output.typed_operation_results.len() >= TYPED_OPERATION_PAGES_PER_TURN_MAXIMUM || (!output.frames.is_empty() && output.frames.len() >= self.max_frames) || (!output.effects.is_empty() && output.effects.len() >= self.max_effects)
    }
    fn expired(&self) -> bool {
        self.deadline_us.is_some_and(|deadline| (self.clock)().is_none_or(|now|now>=deadline))
    }
}
fn pending_typed_operation_instance<PA: PluginApp>(runtime: &PluginRuntime<PA>, start: usize) -> Result<(Option<(usize, u32)>, TypedOperationScan), Fault> {
    let instances = runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?;
    let mut cursor = start;
    let mut first = None;
    let mut contended = false;
    for _ in 0..PLUGIN_RUNTIME_INSTANCE_SLOTS {
        let Some((index, id, cell)) = instances.next_entry_from(cursor) else { break };
        if first == Some(index) {
            break;
        }
        first.get_or_insert(index);
        cursor = (index + 1) % PLUGIN_RUNTIME_INSTANCE_SLOTS;
        if runtime.guest_lifetimes.try_borrow().map_err(|_| plugin_internal_fault("lifecycle authority busy"))?.get(id).is_some_and(|slot| !slot.cell.is_live()) {
            continue;
        }
        match cell.instance.try_lock() {
            Ok(instance) if instance.app.has_runnable_typed_operations() => return Ok((Some((index, id)), TypedOperationScan { runnable: true, contended })),
            Ok(_) => {}
            Err(std::sync::TryLockError::WouldBlock) => contended = true,
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault(format!("runtime operation authority is poisoned for instance {id}"))),
        }
    }
    Ok((None, TypedOperationScan { runnable: false, contended }))
}
/// 📮️ Publishes admitted operations without requiring another user command — as many units of ONE
/// instance as [`TypedOperationGrant`] allows, so a staged work costs the host one round trip per
/// acknowledgeable result page instead of one per unit. Units of a SECOND instance are left to the
/// next turn: the round-robin cursor is the fairness authority and one turn's output carries one
/// receiver.
pub async fn plugin_continue_typed_operations<PA: PluginApp>(runtime: &PluginRuntime<PA>, grant: TypedOperationGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(Option<(u32, PluginExchangeOutput)>, TypedOperationScan), crate::PluginLifecycleFailure> {
    let mut driven: Option<(u32, PluginExchangeOutput)> = None;
    let mut units = 0;
    loop {
        let (next, scan) = pending_typed_operation_instance(runtime, runtime.typed_continuation_cursor.get())?;
        let Some((index, instance)) = next else { return Ok((driven, scan)) };
        if units >= grant.units || driven.as_ref().is_some_and(|(owner, _)| *owner != instance) || (units > 0 && grant.expired()) {
            return Ok((driven, scan));
        }
        runtime.typed_continuation_cursor.set((index + 1) % PLUGIN_RUNTIME_INSTANCE_SLOTS);
        let cell = runtime_instance_cell(runtime, instance)?;
        let output = match cell.instance.try_lock() {
            Ok(mut active) => advance_typed_operation_output(&mut active.app, instance, identity,cx)?,
            Err(std::sync::TryLockError::WouldBlock) => return Ok((driven, TypedOperationScan { runnable: false, contended: true })),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(plugin_internal_fault(format!("runtime operation authority is poisoned for instance {instance}")).into()),
        };
        units += 1;
        let collected = driven.get_or_insert((instance, PluginExchangeOutput::default()));
        assert!(collected.1.preparation_receipt.is_none()&&collected.1.retirement_receipt.is_none()&&collected.1.history_command_receipt.is_none()&&collected.1.tool_run_receipt.is_none()&&collected.1.constructor_receipt.is_none(),"an original receipt stops before another publication unit");
        collected.1.original_receiving=output.original_receiving;
        collected.1.constructor_receipt=output.constructor_receipt;
        collected.1.preparation_receipt=output.preparation_receipt;
        collected.1.retirement_receipt=output.retirement_receipt;
        collected.1.history_command_receipt=output.history_command_receipt;
        collected.1.tool_run_receipt=output.tool_run_receipt;
        collected.1.frames.extend(output.frames);
        collected.1.effects.extend(output.effects);
        collected.1.events.extend(output.events);
        collected.1.typed_operation_results.extend(output.typed_operation_results);
        if grant.spent(&collected.1) {
            let (_, scan) = pending_typed_operation_instance(runtime, runtime.typed_continuation_cursor.get())?;
            return Ok((driven, scan));
        }
    }
}
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): erased `AsyncTask` resolution
/// input for `plugin_resume_task` — mirrors `⚛️reactor`'s own (private) `TaskResumeOutcome`
/// minus its `Fault` case (a task that itself resolved `Err` never reaches `plugin_runtime` at
/// all — `⚛️reactor::drain_task_resumes` frames that straight to the shell). Kept as a SEPARATE
/// type, not a shared one, so the dependency direction stays what it already is everywhere else
/// in this file: `plugin_runtime` never imports `reactor`; `reactor` imports `plugin_runtime`.
pub enum TaskResumeInput {
    Command(Vec<u8>),
    Emit { artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8> },
}
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumes ONE `AsyncTask`
/// follow-up against `instance_id`'s live `dyn PluginApp` — `Command` re-enters through
/// `PluginApp::resume_task_command` (the SAME APPLYING typed-command path a fresh command uses,
/// against CURRENT state — Elm's Msg-from-Cmd), `Emit` through `PluginApp::resume_task_emit`
/// (decode + `dispatch_emit` directly, no re-entry into `handle`). Frames the result EXACTLY
/// like `AppCommand::PureCommand` does (`AppFrame::Emit`, carrying the SAME
/// `take_last_emit_wire()` triple `dispatch_emit` always captures) rather than
/// `AppFrame::Invocation` — a task follow-up is a guest-applied store-authority mutation, not a
/// live client's reply; `in_reply_to: 0` marks it unsolicited (no seq is awaiting it). An
/// app-level `Err` becomes an `Error` frame, exactly like every other dispatch path in this
/// module — this function itself never fails. Returns the SAME `(frames, effects, events)`
/// shape `PluginExchangeOutput` carries so the caller (`⚛️reactor::drain_task_resumes`) feeds
/// `frames` through the SAME `route_app_frame` a live turn's command frames go through.
///
/// 🌉️ Rewritten from a `.map(protocol::encode_app_frame).collect()` into an explicit loop
/// (sync — `Iterator::map` cannot take an async closure, and `encode_app_frame` is genuinely async).
pub async fn plugin_resume_task<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, meta: &ActionMeta, input: TaskResumeInput, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> PluginExchangeOutput {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let dispatched = match input {
        TaskResumeInput::Command(bytes) => {
            with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.resume_task_command(bytes, meta, identity, cx))
            })
            .await
        }
        TaskResumeInput::Emit { artifact_ops, config_ops, draft_ops } => {
            with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                ::semio_framework_async::poll::resolve_ready(instance.app.resume_task_emit(artifact_ops, config_ops, draft_ops, meta, identity, cx))
            })
            .await
        }
    };
    match dispatched {
        Ok(result) => {
            let emit_wire = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
            })
            .await
            .unwrap_or_default();
            let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
            frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
        }
        Err(fault) => push_app_fault(&mut frames, None, fault).await,
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }
}
/// 🔁️ Dispatches one extension invocation's OUTCOME back into the instance that asked for it —
/// `⚛️reactor`'s `Event::Completed` continuation branch is this function's only caller. `action`
/// is the `response_action` the app declared on its `Emit::extension_invocations` entry and is
/// routed through the SAME owner-qualified `PluginApp::handle_command` path a live client's
/// command frame takes (kind discipline, tool proofs, the command log and undo grouping all
/// apply unchanged) — there is no second dispatch path to keep in sync. The address is
/// reconstructed rather than carried: `plugin_id` from the loaded program, `app_id` from the
/// live instance itself, so an app can never name a foreign owner through a response action.
/// Frames the result EXACTLY like `plugin_resume_task` (`AppFrame::Emit`/`Error`, `in_reply_to:
/// 0`): a continuation is guest-applied, unsolicited by any client `seq`.
///
/// 🪟️ A continuation carries the instance's LAST HOST VIEW, exactly like `pending_effects`
/// does. A response action is a window-addressed route whenever the request it answers was one
/// — `reactor::extension_response_args` echoes the request's own `windowId`/`windowKindId` back
/// onto it — and `ArtifactApp::retained_window_transient_target` validates that address against
/// the trusted roster. Dispatching it with no view made every `flowEvalResolve` fault
/// `targeted window transient capture requires an exact ViewModel roster`, so no extension
/// answer ever folded back and the preview never converged
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn plugin_dispatch_response_action<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<PluginExchangeOutput>,semio_framework_job::OriginalTurnFailure> {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let view_state = with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.view().cloned())
    })
    .await
    .ok()
    .flatten();
    let actor = match instance_actor(runtime, instance_id).await {
        Ok(actor) => actor.into(),
        Err(fault) => {
            push_app_fault(&mut frames, None, fault).await;
            let mut output = PluginExchangeOutput::default();
            for frame in &frames { output.frames.push(protocol::encode_app_frame(frame).await); }
            return Ok(Some(output));
        }
    };
    let meta = ActionMeta { actor, instance_id, view_state };
    let dispatched = with_instances_turn_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        ::semio_framework_async::poll::resolve_ready(instance.app.handle_response_action(action, args, original_fault, &meta, identity, cx))
    })
    .await;
    match dispatched {
        Ok(None)=>return Ok(None),
        Ok(Some(result)) => {
            let emit_wire = with_instances_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
            })
            .await
            .unwrap_or_default();
            let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
            frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
            push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
        }
        Err(failure) => return Err(failure),
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    Ok(Some(PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }))
}
/// 🕰️ Host-driven completion for one framework-reserved spawn-job: the job's output joins the app's
/// reserved-commit queue and one typed-operation continuation unit runs at once. A commit that
/// needs more units (composed children, revert walks) stays runnable and the reactor's
/// continuation finishes it on this and later turns. A job the app never admitted is a no-op so
/// fill and other isolated jobs keep their existing JobCompleted path.
pub async fn plugin_complete_reserved_spawned_job<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, job: u64, output: Result<Vec<u8>, Fault>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PluginExchangeOutput,crate::PluginLifecycleFailure> {
    let admitted = with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.admit_reserved_spawned_job(job, output)
    })
    .await;
    match admitted {
        Ok(false) => Ok(PluginExchangeOutput::default()),
        Ok(true) => {
            with_instances_original_mut(runtime, |list| {
                let mut instance = find_instance(list, instance_id)?;
                advance_typed_operation_output(&mut instance.app, instance_id, identity,cx)
            })
            .await
        }
        Err(fault) => Err(fault.into()),
    }
}
/// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): dispatches every `intents` entry for
/// `instance_id` against the app's live `dyn PluginApp` via `PluginApp::handle_intent_frame` —
/// the ONLY dispatch path this takes, which resolves a typed `Command` through
/// `ArtifactApp::command_from_intent` and routes it through the SAME internal
/// `dispatch_typed_command_inner` every other command path uses (kind discipline, command log,
/// undo grouping, `AsyncTask` spawning all apply automatically). `intents` is ALREADY
/// revision-guarded by the caller (`⚛️reactor::poll`'s `PatchTracker::revision` +
/// `ui_runtime::is_stale_intent`, at the reactor because it owns the reconciler that owns the
/// revision) — this function never re-checks staleness, matching `plugin_exchange`'s own division
/// of labor (host-level routing vs. app-level dispatch).
///
/// Frames the result EXACTLY like `plugin_resume_task` (`AppFrame::Emit`/`Error`, `in_reply_to:
/// 0`): an intent is guest-applied, not a live client's `Command`-frame reply awaiting a specific
/// `seq` — the reply IS the next `UiPatch` revision bump the caller's own `dirty_render` pass
/// produces this same turn, already acknowledged by the existing `PatchAck` event; `intent.seq`
/// is the dedupe/log key a caller may record, not an `in_reply_to`. A per-intent `Fault` (e.g. an
/// `ActionId.version` mismatch, or a `View`/`🐚️Shell`-kind action returning operations) rides
/// `AppFrame::Error` — never silently dropped, never a hard stop for the remaining intents in the
/// same batch.
pub async fn plugin_dispatch_intents<PA: PluginApp, const N: usize>(runtime: &PluginRuntime<PA>, instance_id: u32, intents: &ui_contract::UiFixedList<ui_contract::UiIntent, N>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<PluginExchangeOutput>,semio_framework_job::OriginalTurnFailure> {
    let mut frames: Vec<protocol::AppFrame> = Vec::new();
    let mut effect_bytes: Vec<Vec<u8>> = Vec::new();
    let mut event_bytes: Vec<Vec<u8>> = Vec::new();
    let actor: semio_framework_value::SharedUtf8 = instance_actor(runtime, instance_id).await?.into();
    for intent in intents {
        let dispatched = with_instances_turn_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            let context = instance.surface_contexts.get(intent.surface.as_ref()).ok_or_else(|| plugin_internal_fault("intent surface has no host context"))?;
            let meta = ActionMeta { actor: actor.clone(), instance_id, view_state: Some(context.view_state) };
            ::semio_framework_async::poll::resolve_ready(instance.app.handle_intent_frame(intent, &meta, identity, cx))
        })
        .await;
        match dispatched {
            Ok(None)=>return Ok(None),
            Ok(Some(result)) => {
                let emit_wire = with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    Ok(::semio_framework_async::poll::resolve_ready(instance.app.take_last_emit_wire()).unwrap_or_default())
                })
                .await
                .unwrap_or_default();
                let EmitWire { document: document_ops, config: config_ops, draft: draft_ops, children: child_ops } = emit_wire;
                frames.push(protocol::AppFrame::Emit { in_reply_to: 0, document_ops, config_ops, draft_ops, output: encode_wire_serialized(&result.output), diagnostics: encode_wire_serialized(&result.diagnostics), child_ops });
                push_invocation_side_frames(&mut effect_bytes, &mut event_bytes, &result).await;
            }
            Err(failure) => return Err(failure),
        }
    }
    let mut frame_bytes = Vec::with_capacity(frames.len());
    for frame in frames.iter() {
        frame_bytes.push(protocol::encode_app_frame(frame).await);
    }
    Ok(Some(PluginExchangeOutput {
        original_receiving: None,
        constructor_receipt: None,
        preparation_receipt: None,
        retirement_receipt: None,
        history_command_receipt: None,
        tool_run_receipt: None,
        frames: frame_bytes,
        effects: effect_bytes,
        events: event_bytes,
        retry_command: None,
        command_terminal_fault: None,
        presence_pending: None,
        presence_terminal: None,
        presence_terminal_fault: None,
        typed_operation_results: Vec::new(),
    }))
}
pub async fn plugin_reserve_presence_ingress<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.app.reserve_presence_ingress(seq)
    })
    .await
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_admit_reserved_presence<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    admission: PresenceRosterAdmission,
    own_color: Option<u8>,
    item_count: u32,
    page: semio_framework::kernel::FixedCommandPage,
    now_ms: i64,
) -> Result<u64, (Fault, PresenceRosterAdmission, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), admission, page)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(fault) => return Err((fault, admission, page)),
    };
    let generation = admission.generation();
    let cursor = match protocol::PresenceCommandCursor::admit_page(admission.sequence(), own_color, item_count, page) {
        Ok(cursor) => cursor,
        Err((error, rejected)) => {
            instance.app.reject_presence_ingress(admission, rejected, error.into_fault());
            return Ok(generation);
        }
    };
    instance.app.admit_presence_ingress(admission, cursor, now_ms);
    Ok(generation)
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_push_reserved_presence_page<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    generation: u64,
    page_index: u32,
    page: semio_framework::kernel::FixedCommandPage,
) -> Result<(), (Fault, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), page)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(fault) => return Err((fault, page)),
    };
    instance.app.push_presence_ingress(generation, page_index, page)
}
#[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
pub async fn plugin_reject_reserved_presence<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    instance_id: u32,
    admission: PresenceRosterAdmission,
    command: semio_framework::kernel::FixedCommandPage,
    fault: Fault,
) -> Result<(), (Fault, semio_framework::kernel::FixedCommandPage)> {
    let instances = match runtime.instances.try_borrow_mut() {
        Ok(instances) => instances,
        Err(_) => return Err((plugin_internal_fault("runtime instance authority is busy"), command)),
    };
    let mut instance = match find_instance(&instances, instance_id) {
        Ok(instance) => instance,
        Err(error) => return Err((error, command)),
    };
    instance.app.reject_presence_ingress(admission, command, fault);
    Ok(())
}
/// 🔀️ The single bidirectional entry point behind WIT `exchange` (see `📜️wit/📜️world.wit`'s
/// `interface plugin` doc) — decodes each `protocol::AppCommand` in `commands`, dispatches it
/// against `instance_id`, and returns every produced `protocol::AppFrame`/effect/event.
///
/// 🚧️ Wave 1 scope — documented here rather than silently: `AppCommand::CommandText` is stubbed
/// (`Error{code:"unsupported"}`, a later wave wires real headless op-text scripts);
/// `AppCommand::ArtifactCommand` only accepts the six history verbs (`DOCUMENT_COMMAND_ACTION_IDS`),
/// mapped onto the existing magic action-name interception rather than a real typed
/// `store::os_store::store_schema::ArtifactCommand` wire codec; the unsolicited outbox (backbone-driven
/// `AppFrame::DocumentChanged`, a persistent per-instance frame queue surviving across calls) is
/// NOT implemented — only `pending_effects` is drained, once, at the end of the batch, whenever a
/// dispatched command mutated the document, mirroring exactly where `plugin_refresh_ui` already
/// calls it.
///
/// 🧬️ Channel v12 (A4) retired `AppCommand::{Hello, RefreshUi, AttachBackbone, DetachBackbone,
/// Bye}` — lifecycle is now `Event::InstanceOpen`/`InstanceClose` at the reactor level, surface
/// rendering is driven by `surface-visible`/`hidden` instead of `RefreshUi` probes, and the
/// backbone attach/detach commands are gone with no replacement command in this wave. Those arms
/// are deleted here rather than adapted (design-abi.md §4).
pub async fn plugin_acknowledge_typed_operation_result<PA: PluginApp>(runtime: &PluginRuntime<PA>, token: TypedOperationResultToken) -> Result<bool, Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, token.receiver)?;
        instance.app.acknowledge_typed_operation_result(token)
    })
    .await
}
/// 🏃️ Drives a self-waking guest future to completion inside one wasm poll. `resolve_ready`
/// samples once; framework-reserved routes yield through [`plugin_job_yield_once`] before their
/// commit, so a single poll would drop the job and the host would see `command-complete` without
/// [`commit_framework_history_route`].
pub fn drive_self_waking_ready<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, Waker};
    let waker = Waker::noop();
    let mut cx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    for _ in 0..1_048_576 {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => continue,
        }
    }
    panic!("drive_self_waking_ready: exceeded the self-wake bound");
}
pub(super) fn media_export_handle_from_wire(handle: protocol::MediaExportHandleWire) -> ArtifactMediaExportHandle {
    ArtifactMediaExportHandle {
        app_instance_id: handle.app_instance_id,
        parent_document_id: handle.parent_document_id,
        operation_id: semio_framework_job::OperationId(handle.operation_id),
        base_revision: semio_framework_job::RevisionId(handle.base_revision),
        generation: semio_framework_job::Generation(handle.generation),
    }
}
pub(super) fn media_export_handle_into_wire(handle:ArtifactMediaExportHandle)->protocol::MediaExportHandleWire{protocol::MediaExportHandleWire{app_instance_id:handle.app_instance_id,parent_document_id:handle.parent_document_id,operation_id:handle.operation_id.0,base_revision:handle.base_revision.0,generation:handle.generation.0}}
