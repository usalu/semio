//! 🛎️ Frame actions: runtime dispatch cursor, frame action owners, deferred cursor, maintenance authority and runtime apply.
use super::*;

//#region 📮️RuntimeMailbox

pub(crate) struct RuntimeDispatchCursor {
    pub(crate) events: ui_host::DrainedEvents,
    pub(crate) generation: u64,
    discrete_index: usize,
    pub(crate) original: Option<RuntimeDispatchSource>,
    pub(crate) inflight: Option<ui_render::DispatchEvent>,
}

pub(crate) struct RuntimeDispatchSource{original:semio_framework_job::OriginalCallerSourceLoan,epoch:u64}
impl RuntimeDispatchSource{
    pub(crate) fn admit(cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Self>,semio_framework_job::OriginalTurnFailure>{let epoch=cx.original_source_epoch().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch source requires its actual admitted input epoch"))?;let parent=cx.original_source_parent()?;Ok(parent.admit_loan(cx)?.map(|original|Self{original,epoch}))}
    fn original_clock_source(&self)->Result<(u64,fn()->Option<u64>,u64),semio_framework_value::ValueError>{self.original.original_clock_source()}
    fn validate_opportunity(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{let(binding,clock,deadline)=self.original_clock_source()?;if cx.original_clock_binding()!=Some(binding)||!std::ptr::fn_addr_eq(clock,cx.clock_source())||deadline!=cx.original_absolute_deadline_us(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch source changed original bound clock or absolute deadline").into())}if !cx.original_source_epoch().is_some_and(|epoch|epoch>=self.epoch){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch cursor incoming epoch precedes its admitted intent").into())}Ok(())}
    fn validate_context(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{self.validate_opportunity(cx)?;self.original.validate_context(cx)}
    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.validate_opportunity(cx)?;let parent=cx.original_source_parent()?;let _=self.original.close_step(parent,cx)?;Ok(self.original.terminal_is_empty())}
}

impl RuntimeDispatchCursor {
    pub(crate) fn new(mut events: ui_host::DrainedEvents,original:RuntimeDispatchSource) -> Self {
        events.metrics = None;
        Self { events, generation: frame_latency::latest_frame_generation(), discrete_index: 0,original:Some(original),inflight:None }
    }

    pub(crate) fn new_for_generation(mut events: ui_host::DrainedEvents, generation: u64,original:RuntimeDispatchSource) -> Self {
        events.metrics = None;
        Self { events, generation, discrete_index: 0,original:Some(original),inflight:None }
    }

    pub(crate) fn take_next(&mut self) -> Option<ui_render::DispatchEvent> {
        while self.discrete_index < self.events.discrete.len() && self.events.discrete[self.discrete_index].is_none() {
            self.discrete_index += 1;
        }
        let pointer_generation = self.events.pointer_move.as_ref().map(|sample| sample.generation);
        let discrete_generation = self.events.discrete.get(self.discrete_index).and_then(Option::as_ref).map(|event| event.generation);
        if pointer_generation.is_some() && (discrete_generation.is_none() || pointer_generation < discrete_generation) {
            let sample = self.events.pointer_move.take().expect("present retained pointer");
            return Some(ui_render::DispatchEvent::PointerMove { pointer: sample.pointer, x: sample.x, y: sample.y, modifiers: sample.modifiers });
        }
        if self.discrete_index < self.events.discrete.len() {
            let index = self.discrete_index;
            self.discrete_index += 1;
            if let Some(event) = self.events.discrete[index].take() {
                return Some(event.event);
            }
        }
        None
    }

    fn validate_original(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch cursor original source has closed"))?;original.validate_context(cx)}

    fn events_terminal_is_empty(&self)->bool{self.inflight.is_none()&&self.events.pointer_move.is_none()&&self.events.metrics.is_none()&&self.events.discrete.iter().all(Option::is_none)}

    pub(crate) fn close_inflight_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(event)=self.inflight.as_ref()else{return Ok(true)};self.validate_original(cx)?;let remaining=cx.retained_grant();let released_bytes=dispatch_event_retained_capacity(event);let depth=match event{ui_render::DispatchEvent::Accessibility{..}=>4,_=>2};if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_release_bytes<released_bytes||remaining.maximum_depth<depth{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})?;cx.consume_fuel(1);drop(self.inflight.take());Ok(false)
    }

    pub(crate) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !self.close_inflight_step(cx)?{return Ok(false)}
        if self.terminal_is_empty(){return Ok(true)}
        if !self.events_terminal_is_empty(){
            self.validate_original(cx)?;let remaining=cx.retained_grant();if cx.should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(false)}
            if self.events.pointer_move.is_some()||self.events.metrics.is_some(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.events.pointer_move=None;self.events.metrics=None;return Ok(false)}
            let Some(index)=self.events.discrete.iter().position(Option::is_some)else{return Ok(false)};let event=&self.events.discrete[index].as_ref().unwrap().event;let released_bytes=dispatch_event_retained_capacity(event);let depth=match event{ui_render::DispatchEvent::Accessibility{..}=>4,_=>2};if remaining.maximum_release_bytes<released_bytes||remaining.maximum_depth<depth{return Ok(false)}
            drop(self.events.discrete[index].take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.original.as_mut().unwrap();if original.close_step(cx)?{self.original=None}Ok(self.terminal_is_empty())
    }

    pub(crate) fn terminal_is_empty(&self)->bool{self.events_terminal_is_empty()&&self.original.is_none()}
}

fn dispatch_event_retained_capacity(event:&ui_render::DispatchEvent)->usize{
    use ui_render::{DispatchEvent as E,ImeEvent,AccessibilityEvent};match event{
        E::KeyDown{key,..}|E::KeyUp{key,..}=>key.capacity(),
        E::TextInput{text}|E::Paste{text}|E::TextEditChunk{text,..}|E::Ime(ImeEvent::Update{text,..}|ImeEvent::Commit{text})=>text.capacity(),
        E::Accessibility{target,event}=>target.window_id.capacity()+target.node_key.capacity()+match event{AccessibilityEvent::Value(text)=>text.capacity(),_=>0},
        _=>0,
    }
}

/// 🧾️ Carries a renderer-local receipt beside the unchanged domain action.
struct FrameActionEnvelope {
    descriptor: ActionDescriptor,
    receipt: Option<ui_wgpu::wgpu::ActionQueueReceipt>,
    cancelled: bool,
}

/// 🧾️ Routes a renderer-local completion to the exact owner that admitted it.
pub(crate) fn settle_renderer_action_receipt(receipt: ui_wgpu::wgpu::ActionQueueReceipt, outcome: engine_canvas::TextEditorActionOutcome<'_>) {
    match receipt.source {
        ui_wgpu::wgpu::ActionQueueReceiptSource::CanvasTextEditor => engine_canvas::settle_text_editor_action_receipt(receipt, outcome),
        ui_wgpu::wgpu::ActionQueueReceiptSource::RetainedInput => {
            let discard = matches!(outcome, engine_canvas::TextEditorActionOutcome::Refused(reason) if shell::dispatch_refusal_discards_input_draft(reason));
            interpreter::settle_retained_input_receipt(receipt, discard);
        }
    }
}

/// 🛟️ Settles the checked-out receipt even when its async dispatch future is cancelled.
pub(crate) struct FrameActionReceiptOwner {
    pub(crate) receipt: Option<ui_wgpu::wgpu::ActionQueueReceipt>,
}

impl FrameActionReceiptOwner {
    pub(crate) fn settle(&mut self, outcome: engine_canvas::TextEditorActionOutcome<'_>) {
        if let Some(receipt) = self.receipt.take() {
            settle_renderer_action_receipt(receipt, outcome);
        }
    }
}

impl Drop for FrameActionReceiptOwner {
    fn drop(&mut self) {
        self.settle(engine_canvas::TextEditorActionOutcome::Cancelled);
    }
}

impl FrameActionEnvelope {
    fn cancel(self) {
        if let Some(receipt) = self.receipt {
            settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Cancelled);
        }
    }
}

impl From<ui_wgpu::wgpu::QueuedActionDescriptor> for FrameActionEnvelope {
    fn from(queued: ui_wgpu::wgpu::QueuedActionDescriptor) -> Self {
        Self { descriptor: queued.descriptor, receipt: queued.receipt, cancelled: false }
    }
}

/// 📦️ Fixed FIFO ownership for admitted frame actions; refusal returns the identical action.
struct FrameActionBatchOwner {
    slots: [Option<FrameActionEnvelope>; ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY],
    expected: u8,
    len: u8,
    source_remaining: u8,
    fault: Option<&'static str>,
}

impl FrameActionBatchOwner {
    fn new(expected: usize) -> Self {
        let expected = u8::try_from(expected).expect("validated frame action batch capacity");
        Self { slots: std::array::from_fn(|_| None), expected, len: 0, source_remaining: expected, fault: None }
    }

    fn remaining(&self) -> usize {
        usize::from(self.expected.saturating_sub(self.len))
    }
}

pub(crate) struct FrameActionOwners {
    pub(crate) slots: [Option<FrameActionEnvelope>; WORLD3D_DEADLINE_CAPACITY],
    head: usize,
    pub(crate) len: usize,
    pub(crate) batch: Option<FrameActionBatchOwner>,
}

impl Default for FrameActionOwners {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0, batch: None }
    }
}

enum FrameActionBatchStage {
    Pending,
    Published,
    Fault,
}

impl FrameActionOwners {
    pub(crate) fn try_push_queued(&mut self, action: ui_wgpu::wgpu::QueuedActionDescriptor) -> Result<(), ui_wgpu::wgpu::QueuedActionDescriptor> {
        if self.batch.is_some() || self.len == self.slots.len() {
            return Err(action);
        }
        let Some(index) = self.head.checked_add(self.len).map(|index| index % self.slots.len()) else { return Err(action) };
        let Some(next_len) = self.len.checked_add(1) else { return Err(action) };
        if self.slots[index].is_some() {
            return Err(action);
        }
        self.slots[index] = Some(FrameActionEnvelope::from(action));
        self.len = next_len;
        Ok(())
    }

    pub(crate) fn try_push_queued_pair(&mut self, first: ui_wgpu::wgpu::QueuedActionDescriptor, second: ui_wgpu::wgpu::QueuedActionDescriptor) -> Result<(), (ui_wgpu::wgpu::QueuedActionDescriptor, ui_wgpu::wgpu::QueuedActionDescriptor)> {
        if self.batch.is_some() || self.len.checked_add(2).is_none_or(|len| len > self.slots.len()) {
            return Err((first, second));
        }
        let first_index = (self.head + self.len) % self.slots.len();
        let second_index = (first_index + 1) % self.slots.len();
        if self.slots[first_index].is_some() || self.slots[second_index].is_some() {
            return Err((first, second));
        }
        self.slots[first_index] = Some(FrameActionEnvelope::from(first));
        self.slots[second_index] = Some(FrameActionEnvelope::from(second));
        self.len += 2;
        Ok(())
    }

    pub(crate) fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor> {
        if self.batch.is_some() || self.len == self.slots.len() {
            return Err(action);
        }
        let Some(index) = self.head.checked_add(self.len).map(|index| index % self.slots.len()) else { return Err(action) };
        let Some(next_len) = self.len.checked_add(1) else { return Err(action) };
        if self.slots[index].is_some() {
            return Err(action);
        }
        self.slots[index] = Some(FrameActionEnvelope { descriptor: action, receipt: None, cancelled: false });
        self.len = next_len;
        Ok(())
    }

    fn begin_batch(&mut self, expected: usize) -> bool {
        if self.batch.is_some() || expected == 0 || expected > ui_wgpu::wgpu::action::ACTION_BATCH_ITEM_CAPACITY || self.len.checked_add(expected).is_none_or(|len| len > self.slots.len()) {
            return false;
        }
        self.batch = Some(FrameActionBatchOwner::new(expected));
        true
    }

    pub(crate) fn staged_batch_remaining(&self) -> Option<usize> {
        self.batch.as_ref().map(FrameActionBatchOwner::remaining)
    }

    fn stage_batch_action(&mut self, action: FrameActionEnvelope) -> FrameActionBatchStage {
        let Some(batch) = self.batch.as_mut() else {
            action.cancel();
            return FrameActionBatchStage::Fault;
        };
        if batch.fault.is_some() {
            action.cancel();
            return FrameActionBatchStage::Fault;
        }
        if batch.len == batch.expected || usize::from(batch.len) == batch.slots.len() {
            action.cancel();
            batch.fault = Some("frame input action batch staging exceeded its reserved slice");
            return FrameActionBatchStage::Fault;
        }
        batch.slots[usize::from(batch.len)] = Some(action);
        batch.len += 1;
        if batch.len != batch.expected {
            return FrameActionBatchStage::Pending;
        }
        for offset in 0..usize::from(batch.expected) {
            let Some(index) = self.head.checked_add(self.len).and_then(|index| index.checked_add(offset)).map(|index| index % self.slots.len()) else {
                batch.fault = Some("frame input action batch target index overflowed");
                return FrameActionBatchStage::Fault;
            };
            if self.slots[index].is_some() {
                batch.fault = Some("frame input action batch target was occupied");
                return FrameActionBatchStage::Fault;
            }
        }
        let mut batch = self.batch.take().expect("validated frame action batch");
        for offset in 0..usize::from(batch.expected) {
            let index = (self.head + self.len + offset) % self.slots.len();
            self.slots[index] = batch.slots[offset].take();
        }
        self.len += usize::from(batch.expected);
        FrameActionBatchStage::Published
    }

    fn consume_batch_source(&mut self) -> bool {
        let Some(batch) = self.batch.as_mut() else { return false };
        let Some(source_remaining) = batch.source_remaining.checked_sub(1) else {
            batch.fault = Some("frame input action batch consumed beyond its source slice");
            return false;
        };
        batch.source_remaining = source_remaining;
        true
    }

    fn fail_batch(&mut self, fault: &'static str) -> bool {
        let Some(batch) = self.batch.as_mut() else { return false };
        batch.fault = Some(fault);
        batch.len > 0
    }

    fn retire_failed_batch_step(&mut self, input: &mut InputState<ActionDescriptor>) -> Option<Result<FrameInputActionStep, &'static str>> {
        let batch = self.batch.as_mut()?;
        let fault = batch.fault?;
        if batch.len > 0 {
            batch.len -= 1;
            if let Some(action) = batch.slots[usize::from(batch.len)].take() {
                action.cancel();
            }
            return Some(Ok(FrameInputActionStep::Pending));
        }
        let source_remaining = usize::from(batch.source_remaining);
        if source_remaining > 0 {
            match input.take_action_batch_len_step() {
                Ok(Some(remaining)) if remaining == source_remaining => {}
                Err(_) => return Some(Ok(FrameInputActionStep::Pending)),
                _ => return Some(Err("bounded frame input action batch source identity changed while retiring")),
            }
            match input.take_action_step() {
                Ok(Some(action)) => {
                    if let Some(receipt) = action.receipt() {
                        settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Cancelled);
                    }
                    batch.source_remaining -= 1;
                    return Some(Ok(FrameInputActionStep::Pending));
                }
                Err(_) => return Some(Ok(FrameInputActionStep::Pending)),
                Ok(None) => return Some(Err("bounded frame input action batch lost its source owner while retiring")),
            }
        }
        self.batch = None;
        Some(Err(fault))
    }

    pub(crate) fn pop_front(&mut self) -> Option<FrameActionEnvelope> {
        if self.len == 0 {
            return None;
        }
        let action = self.slots[self.head].take()?;
        self.head = self.head.checked_add(1).map(|index| index % self.slots.len())?;
        self.len = self.len.checked_sub(1)?;
        Some(action)
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if let Some(batch) = self.batch.as_mut() {
            if batch.len > 0 {
                batch.len -= 1;
                if let Some(action) = batch.slots[usize::from(batch.len)].take() {
                    action.cancel();
                }
                return false;
            }
            self.batch = None;
            return false;
        }
        if let Some(action) = self.pop_front() {
            action.cancel();
            return false;
        }
        true
    }

    pub(crate) fn cancel_correlation(&mut self, token: std::num::NonZeroU64) {
        for action in self.slots.iter_mut().flatten().chain(self.batch.iter_mut().flat_map(|batch| batch.slots.iter_mut().flatten())) {
            if action.receipt.is_some_and(|receipt| receipt.token == token) {
                action.cancelled = true;
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0 && self.batch.is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FrameInputActionStep {
    Empty,
    Deferred,
    Pending,
    Transferred,
}

fn fail_frame_action_batch(input: &mut InputState<ActionDescriptor>, actions: &mut FrameActionOwners, fault: &'static str) -> Result<FrameInputActionStep, &'static str> {
    if actions.fail_batch(fault) {
        return Ok(FrameInputActionStep::Pending);
    }
    actions.retire_failed_batch_step(input).unwrap_or(Err(fault))
}

pub(crate) fn transfer_frame_input_action(input: &mut InputState<ActionDescriptor>, actions: &mut FrameActionOwners) -> Result<FrameInputActionStep, &'static str> {
    if let Some(step) = actions.retire_failed_batch_step(input) {
        return step;
    }
    let source_remaining = match input.take_action_batch_len_step() {
        Ok(remaining) => remaining,
        Err(_) if actions.staged_batch_remaining().is_some() => return fail_frame_action_batch(input, actions, "bounded frame input action batch source faulted"),
        Err(_) => return Err("bounded frame input action authority faulted"),
    };
    if let Some(target_remaining) = actions.staged_batch_remaining() {
        if source_remaining != Some(target_remaining) {
            return fail_frame_action_batch(input, actions, "bounded frame input action batch identity changed while staged");
        }
    } else {
        let Some(source_remaining) = source_remaining else { return Ok(FrameInputActionStep::Empty) };
        if !actions.begin_batch(source_remaining) {
            return Ok(FrameInputActionStep::Deferred);
        }
    }
    let action = match input.take_action_step() {
        Ok(Some(action)) => action,
        Ok(None) => return fail_frame_action_batch(input, actions, "bounded frame input action batch lost its source owner"),
        Err(_) => return fail_frame_action_batch(input, actions, "bounded frame input action batch source faulted"),
    };
    if !actions.consume_batch_source() {
        if let Some(receipt) = action.receipt() {
            settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Cancelled);
        }
        return fail_frame_action_batch(input, actions, "bounded frame input action batch source accounting faulted");
    }
    let receipt = action.receipt();
    let action = match action.into_envelope() {
        Ok(action) => FrameActionEnvelope::from(action),
        Err(_) => {
            if let Some(receipt) = receipt {
                settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Cancelled);
            }
            return fail_frame_action_batch(input, actions, "bounded frame input action failed materialization");
        }
    };
    match actions.stage_batch_action(action) {
        FrameActionBatchStage::Pending => Ok(FrameInputActionStep::Pending),
        FrameActionBatchStage::Published => Ok(FrameInputActionStep::Transferred),
        FrameActionBatchStage::Fault => Ok(FrameInputActionStep::Pending),
    }
}

pub(crate) struct FrameDeferredCursor {
    pub(crate) actions: FrameActionOwners,
    pub(crate) pump_sync: bool,
    pub(crate) flush_tutorial: bool,
    pub(crate) shell_maintenance: bool,
    pub(crate) settle: bool,
    pub(crate) phase: u8,
    pub(crate) generation: u64,
    pub(crate) cancel: semio_framework_async::CancelToken,
    pub(crate) closing: bool,
}

/// 🔁️ How often a frame owes the shell's sync pump (`ShellState::pump_sync_events`: the document actors'
/// events, the directory lane, the hub creation door, the agent bridge) — on BOTH builds. The browser build
/// used to skip it (the cadence was native-only), so a browser document actor reached `Live` on the wire
/// while its shell never heard a `Status`, `Presence` or `RemoteMutations` event (ticket 26/09/23 WG7, run s12b).
pub(crate) const SHELL_SYNC_PUMP_INTERVAL_MS: f64 = 100.0;

pub(crate) enum FrameDeferredWork {
    ShellMaintenance,
    PumpSync,
    Action(FrameActionEnvelope),
    FlushTutorial,
    /// 🫀️ ONE step of the shell's settle lane — the runtime's half of the pump. It comes LAST in a
    /// frame's deferred order because an input action the same frame carries is what the user is
    /// waiting on, and a settle step may spend a guest crossing.
    Settle,
}

impl FrameDeferredCursor {
    pub(crate) fn new(actions: FrameActionOwners, pump_sync: bool, flush_tutorial: bool, shell_maintenance: bool, settle: bool, generation: u64, cancel: semio_framework_async::CancelToken) -> Self {
        Self { actions, pump_sync, flush_tutorial, shell_maintenance, settle, phase: 0, generation, cancel, closing: false }
    }

    pub(crate) fn checkout_site(&self) -> &'static str {
        if self.phase == 0 && self.shell_maintenance {
            "frame-deferred-shell-maintenance"
        } else if self.phase <= 1 && self.pump_sync {
            "frame-deferred-pump-sync"
        } else if !self.actions.is_empty() {
            "frame-deferred-action"
        } else if self.phase <= 2 && self.flush_tutorial {
            "frame-deferred-tutorial-flush"
        } else if self.phase <= 3 && self.settle {
            "frame-deferred-settle"
        } else {
            "frame-deferred-empty"
        }
    }

    pub(crate) fn take_next(&mut self) -> Option<FrameDeferredWork> {
        if self.phase == 0 {
            self.phase = 1;
            if self.shell_maintenance {
                return Some(FrameDeferredWork::ShellMaintenance);
            }
        }
        if self.phase == 1 {
            self.phase = 2;
            if self.pump_sync {
                return Some(FrameDeferredWork::PumpSync);
            }
        }
        if let Some(action) = self.actions.pop_front() {
            return Some(FrameDeferredWork::Action(action));
        }
        if self.phase == 2 {
            self.phase = 3;
            if self.flush_tutorial {
                return Some(FrameDeferredWork::FlushTutorial);
            }
        }
        if self.phase == 3 {
            self.phase = 4;
            if self.settle {
                return Some(FrameDeferredWork::Settle);
            }
        }
        None
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        !self.closing && self.actions.is_empty() && (self.phase > 0 || !self.shell_maintenance) && (self.phase > 1 || !self.pump_sync) && (self.phase > 2 || !self.flush_tutorial) && (self.phase > 3 || !self.settle)
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if !self.actions.close_step() {
            return false;
        }
        if self.shell_maintenance {
            self.shell_maintenance = false;
            return false;
        }
        if self.pump_sync {
            self.pump_sync = false;
            return false;
        }
        if self.flush_tutorial {
            self.flush_tutorial = false;
            return false;
        }
        if self.settle {
            self.settle = false;
            return false;
        }
        self.phase = 4;
        self.closing = false;
        true
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
    }

    #[cfg(any(not(target_arch = "wasm32"), test))]
    fn return_shell_maintenance(&mut self) {
        self.phase = 0;
        self.shell_maintenance = true;
        self.closing = false;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameMaintenanceOwner {
    pub(crate) interaction: Option<AppInteractionState>,
    pub(crate) cursor: Option<FrameDeferredCursor>,
    pub(crate) deadline_ms: Option<u64>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceOwner {
    pub(crate) fn new(interaction: AppInteractionState, cursor: FrameDeferredCursor) -> Self {
        Self { interaction: Some(interaction), cursor: Some(cursor), deadline_ms: renderer_worker_pool().now_ms().checked_add(8) }
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        self.cursor.as_ref().map(|cursor| cursor.generation)
    }

    pub(crate) fn take_pair(&mut self) -> Option<(AppInteractionState, FrameDeferredCursor)> {
        if self.interaction.is_none() || self.cursor.is_none() {
            return None;
        }
        Some((self.interaction.take()?, self.cursor.take()?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameDeferredExecutionOwner {
    pub(crate) interaction: Option<AppInteractionState>,
    pub(crate) cursor: Option<FrameDeferredCursor>,
    pub(crate) work: Option<FrameDeferredWork>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameDeferredExecutionOwner {
    pub(crate) fn new(interaction: AppInteractionState, cursor: FrameDeferredCursor, work: FrameDeferredWork) -> Self {
        Self { interaction: Some(interaction), cursor: Some(cursor), work: Some(work) }
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        self.cursor.as_ref().map(|cursor| cursor.generation)
    }

    pub(crate) fn cancel_current(&mut self) {
        if let Some(FrameDeferredWork::Action(action)) = self.work.take() {
            action.cancel();
        } else {
            self.work = None;
        }
    }

    pub(crate) fn take_pair(&mut self) -> Option<(AppInteractionState, FrameDeferredCursor)> {
        if self.work.is_some() || self.interaction.is_none() || self.cursor.is_none() {
            return None;
        }
        Some((self.interaction.take()?, self.cursor.take()?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameExecutionOwnerCell<T> {
    pub(crate) state: std::sync::atomic::AtomicU8,
    pub(crate) owner: std::cell::UnsafeCell<Option<T>>,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Send for FrameExecutionOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Sync for FrameExecutionOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
impl<T> FrameExecutionOwnerCell<T> {
    pub(crate) fn new(owner: T) -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), owner: std::cell::UnsafeCell::new(Some(owner)) }
    }

    pub(crate) fn try_take(&self) -> Option<T> {
        self.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).ok()?;
        unsafe { (&mut *self.owner.get()).take() }
    }

    pub(crate) fn try_restore(&self, owner: T) -> Result<(), T> {
        if self.state.load(Ordering::Acquire) != 1 {
            return Err(owner);
        }
        let slot = unsafe { &mut *self.owner.get() };
        if slot.is_some() {
            return Err(owner);
        }
        *slot = Some(owner);
        self.state.store(0, Ordering::Release);
        Ok(())
    }

    pub(crate) fn restore_taken(&self, owner: T) {
        unsafe {
            *self.owner.get() = Some(owner);
        }
        self.state.store(0, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameMaintenanceRefusal {
    pub(crate) owner: Arc<FrameExecutionOwnerCell<FrameMaintenanceOwner>>,
    pub(crate) reservation_live: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceRefusal {
    pub(crate) fn try_take(&self) -> Option<FrameMaintenanceOwner> {
        self.owner.try_take()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameExecutionRegistry<T> {
    pub(crate) state: std::sync::atomic::AtomicU8,
    pub(crate) generation: AtomicU64,
    pub(crate) owner: std::cell::UnsafeCell<Option<Arc<FrameExecutionOwnerCell<T>>>>,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Send for FrameExecutionRegistry<T> {}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Sync for FrameExecutionRegistry<T> {}

#[cfg(not(target_arch = "wasm32"))]
impl<T> FrameExecutionRegistry<T> {
    const WRITING: u8 = 1;
    const QUEUED: u8 = 2;
    const RUNNING: u8 = 3;
    const ABANDONED: u8 = 4;
    const RECOVERING: u8 = 5;

    pub(crate) fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: AtomicU64::new(0), owner: std::cell::UnsafeCell::new(None) }
    }

    pub(crate) fn try_publish(&self, generation: u64, owner: Arc<FrameExecutionOwnerCell<T>>) -> Result<(), Arc<FrameExecutionOwnerCell<T>>> {
        if self.state.compare_exchange(0, Self::WRITING, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(owner);
        }
        unsafe {
            *self.owner.get() = Some(owner);
        }
        self.generation.store(generation, Ordering::Release);
        self.state.store(Self::QUEUED, Ordering::Release);
        Ok(())
    }

    pub(crate) fn try_begin(&self, generation: u64) -> Option<Arc<FrameExecutionOwnerCell<T>>> {
        if self.generation.load(Ordering::Acquire) != generation {
            return None;
        }
        self.state.compare_exchange(Self::QUEUED, Self::RUNNING, Ordering::AcqRel, Ordering::Acquire).ok()?;
        unsafe { (&*self.owner.get()).as_ref().cloned() }
    }

    pub(crate) fn abandon(&self, generation: u64) -> bool {
        if self.generation.load(Ordering::Acquire) != generation {
            return false;
        }
        let state = self.state.load(Ordering::Acquire);
        if !matches!(state, Self::QUEUED | Self::RUNNING) {
            return false;
        }
        self.state.compare_exchange(state, Self::ABANDONED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    pub(crate) fn complete(&self, generation: u64) -> bool {
        if self.generation.load(Ordering::Acquire) != generation || self.state.compare_exchange(Self::RUNNING, Self::WRITING, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let owner = unsafe { (&mut *self.owner.get()).take() };
        drop(owner);
        self.generation.store(0, Ordering::Release);
        self.state.store(0, Ordering::Release);
        true
    }

    pub(crate) fn reclaim_rejected(&self, generation: u64, fallback: Arc<FrameExecutionOwnerCell<T>>) -> Arc<FrameExecutionOwnerCell<T>> {
        if self.generation.load(Ordering::Acquire) != generation {
            return fallback;
        }
        if self.state.compare_exchange(Self::QUEUED, Self::RECOVERING, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return fallback;
        }
        let owner = unsafe { (&mut *self.owner.get()).take() };
        self.generation.store(0, Ordering::Release);
        self.state.store(0, Ordering::Release);
        match owner {
            Some(owner) => owner,
            None => fallback,
        }
    }

    pub(crate) fn try_take_abandoned(&self) -> Option<(u64, Arc<FrameExecutionOwnerCell<T>>)> {
        self.state.compare_exchange(Self::ABANDONED, Self::RECOVERING, Ordering::AcqRel, Ordering::Acquire).ok()?;
        let generation = self.generation.load(Ordering::Acquire);
        let owner = unsafe { (&mut *self.owner.get()).take() }?;
        self.generation.store(0, Ordering::Release);
        self.state.store(0, Ordering::Release);
        Some((generation, owner))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameMaintenanceAuthority {
    pub(crate) state: std::sync::atomic::AtomicU8,
    pub(crate) generation: AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceAuthority {
    pub(crate) fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: AtomicU64::new(0) }
    }

    pub(crate) fn try_reserve(&self, generation: u64) -> bool {
        if self.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.generation.store(generation, Ordering::Release);
        true
    }

    pub(crate) fn is_live(&self, generation: u64) -> bool {
        self.state.load(Ordering::Acquire) == 1 && self.generation.load(Ordering::Acquire) == generation
    }

    pub(crate) fn release(&self, generation: u64) -> bool {
        if !self.is_live(generation) {
            return false;
        }
        self.generation.store(0, Ordering::Release);
        self.state.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }
}

/// 🛑️ Classifies one maintenance terminal guard before any field work moves.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn frame_maintenance_terminal_fault(cancelled: bool, stale: bool, deadline_exceeded: bool) -> Option<&'static str> {
    if cancelled {
        Some("frame maintenance cancelled")
    } else if stale {
        Some("frame maintenance generation became stale")
    } else if deadline_exceeded {
        Some("frame maintenance deadline expired")
    } else {
        None
    }
}

pub(crate) enum RuntimeApply {
    Resize {
        width: f32,
        height: f32,
        dpr: f32,
    },
    DispatchEvents(Option<RuntimeDispatchCursor>),
    ResumeDispatch {
        interaction: Option<AppInteractionState>,
        cursor: Option<RuntimeDispatchCursor>,
    },
    ResumeFrameDeferred {
        interaction: Option<AppInteractionState>,
        cursor: Option<FrameDeferredCursor>,
    },
    #[cfg(not(target_arch = "wasm32"))]
    RestoreInteraction(Option<AppInteractionState>),
    #[cfg(not(target_arch = "wasm32"))]
    PluginReload(Option<Result<Vec<program_bridge::ProgramBridgeEntry>, String>>),
}

impl RuntimeApply {
    pub(crate) fn close_returned_interaction_step(&mut self,runtime:&mut AppRuntime,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        match self{
            Self::ResumeFrameDeferred{interaction,cursor}=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){owner.begin_close();if !owner.close_step(){return Ok(false)}}cursor.take();Ok(true)},
            Self::DispatchEvents(cursor)=>{if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            Self::ResumeDispatch{interaction,cursor}=>{if let Some(returned)=interaction.as_mut(){if let Some(original)=cursor.as_ref(){original.validate_original(cx)?}if !interpreter::close_clipboard_io_step(&mut returned.input,cx)?{return Ok(false)}}if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            #[cfg(not(target_arch="wasm32"))]Self::RestoreInteraction(interaction)=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned)}Ok(true)},
            _=>Ok(false),
        }
    }

    /// 🎟️ Whether applying this completion HANDS THE INTERACTION STATE BACK. The checkout ledger ages
    /// a live checkout against exactly this: while one of these is queued (or a reservation for one is
    /// in flight) the state has an owner, and however long that owner takes it is not a leak. An arm
    /// that carries no state carries no ownership either — the native abandoned-maintenance resume is
    /// `ResumeFrameDeferred { interaction: None, .. }` for that reason.
    pub(crate) fn restores_interaction(&self) -> bool {
        match self {
            Self::ResumeDispatch { interaction, .. } | Self::ResumeFrameDeferred { interaction, .. } => interaction.is_some(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => interaction.is_some(),
            _ => false,
        }
    }

    fn restore_interaction_only(&mut self, runtime: &mut AppRuntime) -> bool {
        match self {
            Self::ResumeDispatch { interaction, cursor } => {
                let Some(returned) = interaction.take() else { return cursor.is_none() };
                runtime.return_interaction(returned);
                cursor.is_none()
            }
            Self::ResumeFrameDeferred { interaction, cursor } => {
                let Some(returned) = interaction.take() else { return cursor.is_none() };
                runtime.return_interaction(returned);
                cursor.is_none()
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => {
                let Some(returned) = interaction.take() else { return true };
                runtime.return_interaction(returned);
                true
            }
            _ => false,
        }
    }

    fn start_dispatch(cursor: &mut Option<RuntimeDispatchCursor>, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        log_debug("[TRACE] start_dispatch enter");
        let Some(cursor_value) = cursor.as_mut() else { return Ok(true) };
        if !cursor_value.close_inflight_step(cx)?{return Ok(false)}
        if cursor_value.events_terminal_is_empty()||cx.is_cancelled(){if !cursor_value.close_step(cx)?{return Ok(false)}cursor.take();return Ok(true)}
        cursor_value.validate_original(cx)?;if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            log_debug("[TRACE] start_dispatch refused: mailbox handle expired");
            return Ok(false);
        };
        let Some(mut interaction) = runtime.check_out_interaction("dispatch-event") else {
            log_debug_once_per_transition("start-dispatch-checkout", true, "[TRACE] start_dispatch refused: interaction state is checked out");
            return Ok(false);
        };
        if !interaction.input.original_clipboard_has_capacity() {
            runtime.return_interaction(interaction);
            return Ok(false);
        }
        match interaction.input.bind_original_clipboard_dispatch(cx,cursor_value.original.as_ref().expect("validated dispatch source").epoch) {
            Ok(true) => {}
            Ok(false) => { runtime.return_interaction(interaction); return Ok(false); }
            Err(error) => { runtime.return_interaction(interaction); return Err(error); }
        }
        log_debug_once_per_transition("start-dispatch-checkout", false, "[TRACE] start_dispatch: interaction state checked out for one event");
        if !mailbox.reserve_interaction_future() {
            log_debug("[TRACE] start_dispatch refused: interaction future credits exhausted");
            interaction.frame_fault = Some("runtime dispatch completion credits exhausted".to_string());
            runtime.return_interaction(interaction);
            return Ok(false);
        }
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let mut cursor_value=cursor.take().expect("dispatch cursor admitted above");cursor_value.inflight=Some(cursor_value.take_next().expect("non-empty dispatch cursor"));
        mailbox.spawn_dispatch_reserved(cursor_value,interaction);
        Ok(true)
    }

    pub(crate) fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        if cursor.is_none() && !runtime.interaction_available() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                if let Some(mut owner) = mailbox.try_take_abandoned_frame_deferred() {
                    owner.cancel_current();
                    let Some((interaction, returned_cursor)) = owner.take_pair() else { return false };
                    runtime.return_interaction(interaction);
                    *cursor = Some(returned_cursor);
                    return false;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if runtime.pending_frame_maintenance_refusal.is_none() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                runtime.pending_frame_maintenance_refusal = mailbox.try_take_abandoned_frame_maintenance();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal.take() {
            let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            if refusal.reservation_live {
                if !mailbox.cancel_interaction_future_reservation() {
                    runtime.pending_frame_maintenance_refusal = Some(refusal);
                    return false;
                }
                refusal.reservation_live = false;
            }
            let Some(mut owner) = refusal.try_take() else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            let Some((interaction, mut returned_cursor)) = owner.take_pair() else {
                if let Err(owner) = refusal.owner.try_restore(owner) {
                    refusal.owner = Arc::new(FrameExecutionOwnerCell::new(owner));
                }
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            returned_cursor.return_shell_maintenance();
            runtime.return_interaction(interaction);
            *cursor = Some(returned_cursor);
            return false;
        }
        if cursor.as_ref().is_some_and(|cursor| cursor.closing) {
            let complete = cursor.as_mut().is_some_and(FrameDeferredCursor::close_step);
            if complete {
                cursor.take();
            }
            return complete;
        }
        let Some(cursor_value) = cursor.as_mut() else { return true };
        if cursor_value.terminal_is_empty() {
            cursor.take();
            return true;
        }
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else { return false };
        let checkout_site = cursor_value.checkout_site();
        let Some(mut interaction) = runtime.check_out_interaction(checkout_site) else { return false };
        if !mailbox.reserve_interaction_future() {
            interaction.frame_fault = Some("frame deferred completion credits exhausted".to_string());
            runtime.return_interaction(interaction);
            return false;
        }
        let Some(mut cursor_value) = cursor.take() else {
            runtime.return_interaction(interaction);
            return false;
        };
        let Some(work) = cursor_value.take_next() else {
            runtime.return_interaction(interaction);
            if !cursor_value.terminal_is_empty() {
                cursor_value.closing = true;
                *cursor = Some(cursor_value);
                return false;
            }
            return true;
        };
        if matches!(work, FrameDeferredWork::ShellMaintenance) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                match mailbox.try_spawn_frame_maintenance_reserved(FrameMaintenanceOwner::new(interaction, cursor_value)) {
                    Ok(()) => return true,
                    Err(refusal) => {
                        runtime.pending_frame_maintenance_refusal = Some(refusal);
                        return false;
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                mailbox.spawn_frame_maintenance_reserved(async move {
                    cursor_value.shell_maintenance = interaction.shell.advance_chrome_maintenance_step();
                    if cursor_value.shell_maintenance {
                        cursor_value.phase = 0;
                    }
                    (interaction, cursor_value)
                });
                return true;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match mailbox.try_spawn_frame_deferred_reserved(FrameDeferredExecutionOwner::new(interaction, cursor_value, work)) {
                Ok(()) => return true,
                Err(mut owner) => {
                    let _ = mailbox.0.completions.lock().expect("runtime completion mailbox lock").cancel_interaction_reservation();
                    owner.cancel_current();
                    let Some((interaction, cursor_value)) = owner.take_pair() else { return false };
                    runtime.return_interaction(interaction);
                    *cursor = Some(cursor_value);
                    return false;
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        mailbox.spawn_frame_deferred_reserved(async move {
            match work {
                FrameDeferredWork::ShellMaintenance => {}
                FrameDeferredWork::PumpSync => {
                    interaction.shell.pump_sync_events().await;
                }
                FrameDeferredWork::Action(action) => {
                    if action.cancelled {
                        action.cancel();
                    } else {
                        let receipt = action.receipt;
                        let mut receipt_owner = FrameActionReceiptOwner { receipt };
                        match interaction.shell.dispatch_gesture_action(action.descriptor).await {
                            Ok(()) => {
                                receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Accepted);
                            }
                            Err(error) => {
                                receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Refused(&error));
                                if let Some(receipt) = receipt {
                                    if receipt.abort_correlation_on_error {
                                        cursor_value.actions.cancel_correlation(receipt.token);
                                    }
                                }
                                log_debug(&format!("[TRACE] frame deferred action failed: {error}"));
                                interaction.shell.note_dispatch_fault(&error);
                            }
                        }
                    }
                }
                FrameDeferredWork::FlushTutorial => interaction.shell.tutorial_flush_pending_document_ops().await,
                FrameDeferredWork::Settle => {
                    interaction.shell.settle_pump_step().await;
                }
            }
            (interaction, cursor_value)
        });
        #[cfg(target_arch = "wasm32")]
        return true;
        #[allow(unreachable_code)]
        false
    }

    fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        if let Self::DispatchEvents(cursor) = self {
            log_debug(&format!("[TRACE] apply_step DispatchEvents present={} terminal-empty={:?}", cursor.is_some(), cursor.as_ref().map(RuntimeDispatchCursor::terminal_is_empty)));
        }
        match self{Self::DispatchEvents(cursor)=>return Self::start_dispatch(cursor,runtime,handle,cx),Self::ResumeDispatch{interaction,cursor}=>{if let Some(returned)=interaction.as_mut(){if let Some(original)=cursor.as_ref(){original.validate_original(cx)?;}if interpreter::drive_clipboard_io_step(&mut returned.input,cx)?{return Ok(false)}}if let Some(returned)=interaction.take(){runtime.return_interaction(returned)}return Self::start_dispatch(cursor,runtime,handle,cx)},_=>{}}
        Ok((||{match self {
            Self::DispatchEvents(_)|Self::ResumeDispatch{..}=>unreachable!(),
            Self::Resize { width, height, dpr } => runtime.resize(*width, *height, *dpr),
            Self::ResumeFrameDeferred { interaction, cursor } => {
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
                }
                return Self::start_frame_deferred(cursor, runtime, handle);
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => {
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::PluginReload(result) => match result.take() {
                Some(Ok(entries)) => {
                    let handle = handle.clone();
                    runtime.submit_interaction(&handle, Some("plugin-boot"), move |mut interaction| async move {
                        interaction.shell.prepare_hot_reload(entries);
                        if let Err(error) = interaction.shell.boot().await {
                            log_debug(&format!("wasm program hot reload failed: {error}"));
                        } else {
                            log_debug("wasm program hot reload complete");
                        }
                        interaction
                    });
                }
                Some(Err(error)) => log_debug(&format!("wasm program reload failed: {error}")),
                None => return true,
            },
        }
        true})())
    }
}
//#endregion 📮️RuntimeMailbox
