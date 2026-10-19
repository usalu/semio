//! 🛎️ Frame actions: runtime dispatch cursor, frame action owners, deferred cursor, maintenance authority and runtime apply.
use super::*;

#[path = "🎮️interaction-state/🦀️.rs"]
mod interaction_state;
pub use interaction_state::*;

//#region 📮️RuntimeMailbox

pub struct RuntimeDispatchCursor {
    pub events: ui_host::DrainedEvents,
    pub generation: u64,
    discrete_index: usize,
    pub original: Option<RuntimeDispatchSource>,
    pub inflight: Option<ui_render::DispatchEvent>,
}

pub struct RuntimeDispatchSource{original:semio_framework_job::OriginalCallerSourceLoan,pub epoch:u64}
impl RuntimeDispatchSource{
    pub fn admit(cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Self>,semio_framework_job::OriginalTurnFailure>{let epoch=cx.original_source_epoch().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch source requires its actual admitted input epoch"))?;let parent=cx.original_source_parent()?;Ok(parent.admit_loan(cx)?.map(|original|Self{original,epoch}))}
    fn original_clock_source(&self)->Result<(u64,fn()->Option<u64>,u64),semio_framework_value::ValueError>{self.original.original_clock_source()}
    fn validate_opportunity(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{let(binding,clock,deadline)=self.original_clock_source()?;if cx.original_clock_binding()!=Some(binding)||!std::ptr::fn_addr_eq(clock,cx.clock_source())||deadline!=cx.original_absolute_deadline_us(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch source changed original bound clock or absolute deadline").into())}if !cx.original_source_epoch().is_some_and(|epoch|epoch>=self.epoch){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch cursor incoming epoch precedes its admitted intent").into())}Ok(())}
    fn validate_context(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{self.validate_opportunity(cx)?;self.original.validate_context(cx)}
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.validate_opportunity(cx)?;let parent=cx.original_source_parent()?;let _=self.original.close_step(parent,cx)?;Ok(self.original.terminal_is_empty())}
}

impl RuntimeDispatchCursor {
    pub fn new(mut events: ui_host::DrainedEvents,original:RuntimeDispatchSource) -> Self {
        events.metrics = None;
        Self { events, generation: frame_latency::latest_frame_generation(), discrete_index: 0,original:Some(original),inflight:None }
    }

    pub fn new_for_generation(mut events: ui_host::DrainedEvents, generation: u64,original:RuntimeDispatchSource) -> Self {
        events.metrics = None;
        Self { events, generation, discrete_index: 0,original:Some(original),inflight:None }
    }

    pub fn take_next(&mut self) -> Option<ui_render::DispatchEvent> {
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

    pub fn validate_original(&self,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{let original=self.original.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"dispatch cursor original source has closed"))?;original.validate_context(cx)}

    pub fn events_terminal_is_empty(&self)->bool{self.inflight.is_none()&&self.events.pointer_move.is_none()&&self.events.metrics.is_none()&&self.events.discrete.iter().all(Option::is_none)}

    pub fn close_inflight_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(event)=self.inflight.as_ref()else{return Ok(true)};self.validate_original(cx)?;let remaining=cx.retained_grant();let released_bytes=dispatch_event_retained_capacity(event);let depth=match event{ui_render::DispatchEvent::Accessibility{..}=>4,_=>2};if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_release_bytes<released_bytes||remaining.maximum_depth<depth{return Ok(false)}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})?;cx.consume_fuel(1);drop(self.inflight.take());Ok(false)
    }

    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !self.close_inflight_step(cx)?{return Ok(false)}
        if self.terminal_is_empty(){return Ok(true)}
        if !self.events_terminal_is_empty(){
            self.validate_original(cx)?;let remaining=cx.retained_grant();if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_depth==0{return Ok(false)}
            if self.events.pointer_move.is_some()||self.events.metrics.is_some(){cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);self.events.pointer_move=None;self.events.metrics=None;return Ok(false)}
            let Some(index)=self.events.discrete.iter().position(Option::is_some)else{return Ok(false)};let event=&self.events.discrete[index].as_ref().unwrap().event;let released_bytes=dispatch_event_retained_capacity(event);let depth=match event{ui_render::DispatchEvent::Accessibility{..}=>4,_=>2};if remaining.maximum_release_bytes<released_bytes||remaining.maximum_depth<depth{return Ok(false)}
            drop(self.events.discrete[index].take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.original.as_mut().unwrap();if original.close_step(cx)?{self.original=None}Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self)->bool{self.events_terminal_is_empty()&&self.original.is_none()}
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
pub struct FrameActionEnvelope {
    pub descriptor: ActionDescriptor,
    pub receipt: Option<semio_framework_ui_action::ActionQueueReceipt>,
    pub cancelled: bool,
}

/// 🧾️ Routes a renderer-local completion to the exact owner that admitted it.
pub fn settle_renderer_action_receipt(receipt: semio_framework_ui_action::ActionQueueReceipt, outcome: semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome<'_>) {
    match receipt.source {
        semio_framework_ui_action::ActionQueueReceiptSource::CanvasTextEditor => semio_framework_os_renderer_wgpu_engine_text::text_editor::settle_text_editor_action_receipt(receipt, outcome),
        semio_framework_ui_action::ActionQueueReceiptSource::RetainedInput => {
            let discard = matches!(outcome, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Refused(reason) if semio_framework_os_renderer_wgpu_shell_prefs::dispatch_refusal_discards_input_draft(reason));
            semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::settle_retained_input_receipt(receipt, discard);
        }
    }
}

/// 🛟️ Settles the checked-out receipt even when its async dispatch future is cancelled.
pub struct FrameActionReceiptOwner {
    pub receipt: Option<semio_framework_ui_action::ActionQueueReceipt>,
}

impl FrameActionReceiptOwner {
    pub fn settle(&mut self, outcome: semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome<'_>) {
        if let Some(receipt) = self.receipt.take() {
            settle_renderer_action_receipt(receipt, outcome);
        }
    }
}

impl Drop for FrameActionReceiptOwner {
    fn drop(&mut self) {
        self.settle(semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
    }
}

impl FrameActionEnvelope {
    pub fn cancel(self) {
        if let Some(receipt) = self.receipt {
            settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
        }
    }
}

impl From<semio_framework_ui_action::QueuedActionDescriptor> for FrameActionEnvelope {
    fn from(queued: semio_framework_ui_action::QueuedActionDescriptor) -> Self {
        Self { descriptor: queued.descriptor, receipt: queued.receipt, cancelled: false }
    }
}

/// 📦️ Fixed FIFO ownership for admitted frame actions; refusal returns the identical action.
struct FrameActionBatchOwner {
    slots: [Option<FrameActionEnvelope>; semio_framework_ui_action::ACTION_BATCH_ITEM_CAPACITY],
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

pub struct FrameActionOwners {
    pub slots: [Option<FrameActionEnvelope>; WORLD3D_DEADLINE_CAPACITY],
    head: usize,
    pub len: usize,
    pub batch: Option<FrameActionBatchOwner>,
    closing: Option<semio_framework_value_retirement::retirement::controlled::ControlledRetirement<(String, String, Option<semio_framework_value::DslValue>)>>,
    closing_receipt: Option<semio_framework_ui_action::ActionQueueReceipt>,
}

impl Default for FrameActionOwners {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0, batch: None, closing: None, closing_receipt: None }
    }
}

enum FrameActionBatchStage {
    Pending,
    Published,
    Fault,
}

impl FrameActionOwners {
    pub fn try_push_queued(&mut self, action: semio_framework_ui_action::QueuedActionDescriptor) -> Result<(), semio_framework_ui_action::QueuedActionDescriptor> {
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

    pub fn try_push_queued_pair(&mut self, first: semio_framework_ui_action::QueuedActionDescriptor, second: semio_framework_ui_action::QueuedActionDescriptor) -> Result<(), (semio_framework_ui_action::QueuedActionDescriptor, semio_framework_ui_action::QueuedActionDescriptor)> {
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

    pub fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor> {
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
        if self.batch.is_some() || expected == 0 || expected > semio_framework_ui_action::ACTION_BATCH_ITEM_CAPACITY || self.len.checked_add(expected).is_none_or(|len| len > self.slots.len()) {
            return false;
        }
        self.batch = Some(FrameActionBatchOwner::new(expected));
        true
    }

    pub fn staged_batch_remaining(&self) -> Option<usize> {
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
                        settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
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

    pub fn pop_front(&mut self) -> Option<FrameActionEnvelope> {
        if self.len == 0 {
            return None;
        }
        let action = self.slots[self.head].take()?;
        self.head = self.head.checked_add(1).map(|index| index % self.slots.len())?;
        self.len = self.len.checked_sub(1)?;
        Some(action)
    }

    fn retain_cancelled_action(&mut self, action: FrameActionEnvelope) -> Result<(), FrameActionEnvelope> {
        if self.closing.is_some() || self.closing_receipt.is_some() { return Err(action); }
        self.closing_receipt = action.receipt;
        let ActionDescriptor { controller_id, action, args } = action.descriptor;
        self.closing = Some(semio_framework_value_retirement::retirement::controlled::ControlledRetirement::retain((controller_id, action, args)));
        Ok(())
    }

    fn close_retained_action_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.closing_receipt.is_some() {
            if !frame_action_metadata(cx)? { return Ok(false); }
            settle_renderer_action_receipt(self.closing_receipt.take().unwrap(), semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
            return Ok(false);
        }
        if let Some(owner) = self.closing.as_mut() {
            if !owner.terminal_is_empty() {
                let demand = owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx, demand, |grant| owner.step(grant))?;
                return Ok(false);
            }
            if !frame_action_metadata(cx)? { return Ok(false); }
            self.closing = None;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if !self.close_retained_action_step(cx)? { return Ok(false); }
        if self.batch.as_ref().is_some_and(|batch| batch.len == 0) {
            if !frame_action_metadata(cx)? { return Ok(false); }
            self.batch = None;
            return Ok(false);
        }
        if self.batch.is_none() && self.len == 0 { return Ok(true); }
        if !frame_action_metadata(cx)? { return Ok(false); }
        let action = if let Some(batch) = self.batch.as_mut() { batch.len -= 1; batch.slots[usize::from(batch.len)].take() } else { self.pop_front() };
        if let Some(action) = action {
            assert!(self.retain_cancelled_action(action).is_ok(), "empty original action retirement slot");
        }
        Ok(false)
    }

    pub fn cancel_correlation(&mut self, token: std::num::NonZeroU64) {
        for action in self.slots.iter_mut().flatten().chain(self.batch.iter_mut().flat_map(|batch| batch.slots.iter_mut().flatten())) {
            if action.receipt.is_some_and(|receipt| receipt.token == token) {
                action.cancelled = true;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0 && self.batch.is_none() && self.closing.is_none() && self.closing_receipt.is_none()
    }
}

pub fn frame_action_metadata(cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    if cx.retirement_should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
    cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
    cx.consume_fuel(1);
    Ok(true)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameInputActionStep {
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

pub fn transfer_frame_input_action(input: &mut InputState<ActionDescriptor>, actions: &mut FrameActionOwners) -> Result<FrameInputActionStep, &'static str> {
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
            settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
        }
        return fail_frame_action_batch(input, actions, "bounded frame input action batch source accounting faulted");
    }
    let receipt = action.receipt();
    let action = match action.into_envelope() {
        Ok(action) => FrameActionEnvelope::from(action),
        Err(_) => {
            if let Some(receipt) = receipt {
                settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
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

pub struct FrameDeferredCursor {
    pub actions: FrameActionOwners,
    pub pump_sync: bool,
    pub flush_tutorial: bool,
    pub shell_maintenance: bool,
    pub settle: bool,
    pub phase: u8,
    pub generation: u64,
    pub cancel: Option<semio_framework_async::CancelToken>,
    cancel_close: Option<semio_framework_async::CancelTokenRetirement>,
    pub closing: bool,
}

/// 🔁️ How often a frame owes the shell's sync pump (`ShellState::pump_sync_events`: the document actors'
/// events, the directory lane, the hub creation door, the agent bridge) — on BOTH builds. The browser build
/// used to skip it (the cadence was native-only), so a browser document actor reached `Live` on the wire
/// while its shell never heard a `Status`, `Presence` or `RemoteMutations` event (ticket 26/09/23 WG7, run s12b).
pub const SHELL_SYNC_PUMP_INTERVAL_MS: f64 = 100.0;

pub enum FrameDeferredWork {
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
    pub fn new(actions: FrameActionOwners, pump_sync: bool, flush_tutorial: bool, shell_maintenance: bool, settle: bool, generation: u64, cancel: semio_framework_async::CancelToken) -> Self {
        Self { actions, pump_sync, flush_tutorial, shell_maintenance, settle, phase: 0, generation, cancel: Some(cancel), cancel_close: None, closing: false }
    }

    pub fn checkout_site(&self) -> &'static str {
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

    pub fn take_next(&mut self) -> Option<FrameDeferredWork> {
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

    pub fn work_is_empty(&self) -> bool {
        self.actions.is_empty() && (self.phase > 0 || !self.shell_maintenance) && (self.phase > 1 || !self.pump_sync) && (self.phase > 2 || !self.flush_tutorial) && (self.phase > 3 || !self.settle)
    }

    pub fn terminal_is_empty(&self) -> bool {
        !self.closing && self.work_is_empty() && self.cancel.is_none() && self.cancel_close.is_none()
    }

    pub fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if !self.actions.close_step(cx)? { return Ok(false); }
        if self.shell_maintenance || self.pump_sync || self.flush_tutorial || self.settle {
            if !frame_action_metadata(cx)? { return Ok(false); }
            if self.shell_maintenance { self.shell_maintenance = false; }
            else if self.pump_sync { self.pump_sync = false; }
            else if self.flush_tutorial { self.flush_tutorial = false; }
            else { self.settle = false; }
            return Ok(false);
        }
        if self.cancel.is_some() {
            if !frame_action_metadata(cx)? { return Ok(false); }
            self.cancel_close = self.cancel.take().map(semio_framework_async::CancelTokenRetirement::from_token);
            return Ok(false);
        }
        if let Some(owner) = self.cancel_close.as_mut() {
            if !owner.terminal_is_empty() {
                if cx.retirement_should_yield() { return Ok(false); }
                let grant = cx.retained_grant();
                let step = if owner.is_original_alias_witness(cx.original_cancel_token()) {
                    owner.return_alias_step(cx.original_cancel_token(), grant)
                } else { owner.close_step(grant) };
                let step = step.map_err(|error| match error {
                    semio_framework_async::CancelTokenRetirementError::Refused(error) => semio_framework_job::OriginalTurnFailure::debit_original(error, cx),
                    semio_framework_async::CancelTokenRetirementError::Blocked(_) => semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "frame deferred cancellation retains an unwitnessed supplier").into(),
                })?;
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(u64::from(step.progress().copied_items != 0));
                return Ok(false);
            }
            if !frame_action_metadata(cx)? { return Ok(false); }
            self.cancel_close = None;
            return Ok(false);
        }
        if self.phase != 4 || self.closing {
            if !frame_action_metadata(cx)? { return Ok(false); }
            self.phase = 4;
            self.closing = false;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn begin_close(&mut self) {
        if !self.terminal_is_empty() { self.closing = true; }
    }

    #[cfg(any(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
    pub fn return_shell_maintenance(&mut self) {
        self.phase = 0;
        self.shell_maintenance = true;
        self.closing = false;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameExecutionOwnerCell<T> {
    pub state: std::sync::atomic::AtomicU8,
    pub owner: std::cell::UnsafeCell<Option<T>>,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Send for FrameExecutionOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Sync for FrameExecutionOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
impl<T> FrameExecutionOwnerCell<T> {
    pub fn new(owner: T) -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), owner: std::cell::UnsafeCell::new(Some(owner)) }
    }

    /// 🔒️ Temporarily borrows the persistent cell without creating or discarding another owner.
    pub fn try_borrow_mut<R>(&self, visit: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).ok()?;
        struct Return<'a>(&'a std::sync::atomic::AtomicU8);
        impl Drop for Return<'_> { fn drop(&mut self) { self.0.store(0, Ordering::Release); } }
        let _return = Return(&self.state);
        unsafe { (&mut *self.owner.get()).as_mut().map(visit) }
    }

    pub fn try_take(&self) -> Option<T> {
        self.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).ok()?;
        unsafe { (&mut *self.owner.get()).take() }
    }

    pub fn try_restore(&self, owner: T) -> Result<(), T> {
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

    pub fn restore_taken(&self, owner: T) {
        unsafe {
            *self.owner.get() = Some(owner);
        }
        self.state.store(0, Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameExecutionRegistry<T> {
    pub state: std::sync::atomic::AtomicU8,
    pub generation: AtomicU64,
    pub owner: std::cell::UnsafeCell<Option<Arc<FrameExecutionOwnerCell<T>>>>,
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
    const RETURNED: u8 = 6;

    pub fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: AtomicU64::new(0), owner: std::cell::UnsafeCell::new(None) }
    }

    pub fn try_publish(&self, generation: u64, owner: Arc<FrameExecutionOwnerCell<T>>) -> Result<(), Arc<FrameExecutionOwnerCell<T>>> {
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

    pub fn try_begin(&self, generation: u64) -> Option<Arc<FrameExecutionOwnerCell<T>>> {
        if self.generation.load(Ordering::Acquire) != generation {
            return None;
        }
        self.state.compare_exchange(Self::QUEUED, Self::RUNNING, Ordering::AcqRel, Ordering::Acquire).ok()?;
        unsafe { (&*self.owner.get()).as_ref().cloned() }
    }

    pub fn abandon(&self, generation: u64) -> bool {
        if self.generation.load(Ordering::Acquire) != generation {
            return false;
        }
        let state = self.state.load(Ordering::Acquire);
        if !matches!(state, Self::QUEUED | Self::RUNNING) {
            return false;
        }
        self.state.compare_exchange(state, Self::ABANDONED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    pub fn return_to_caller(&self, generation: u64) -> bool {
        self.generation.load(Ordering::Acquire) == generation && self.state.compare_exchange(Self::RUNNING, Self::RETURNED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    pub fn reclaim_rejected(&self, generation: u64, fallback: Arc<FrameExecutionOwnerCell<T>>) -> Arc<FrameExecutionOwnerCell<T>> {
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

    pub fn has_returned_owner(&self) -> bool { matches!(self.state.load(Ordering::Acquire), Self::ABANDONED | Self::RETURNED) }

    pub fn try_take_returned(&self) -> Option<(u64, Arc<FrameExecutionOwnerCell<T>>)> {
        let state = self.state.load(Ordering::Acquire);
        if !matches!(state, Self::ABANDONED | Self::RETURNED) { return None; }
        self.state.compare_exchange(state, Self::RECOVERING, Ordering::AcqRel, Ordering::Acquire).ok()?;
        let generation = self.generation.load(Ordering::Acquire);
        let owner = unsafe { (&mut *self.owner.get()).take() }?;
        self.generation.store(0, Ordering::Release);
        self.state.store(0, Ordering::Release);
        Some((generation, owner))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameMaintenanceAuthority {
    pub state: std::sync::atomic::AtomicU8,
    pub generation: AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceAuthority {
    pub fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: AtomicU64::new(0) }
    }

    pub fn try_reserve(&self, generation: u64) -> bool {
        if self.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.generation.store(generation, Ordering::Release);
        true
    }

    pub fn is_live(&self, generation: u64) -> bool {
        self.state.load(Ordering::Acquire) == 1 && self.generation.load(Ordering::Acquire) == generation
    }

    pub fn release(&self, generation: u64) -> bool {
        if !self.is_live(generation) {
            return false;
        }
        self.generation.store(0, Ordering::Release);
        self.state.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }
}

/// 🛑️ Classifies one maintenance terminal guard before any field work moves.
#[cfg(not(target_arch = "wasm32"))]
pub fn frame_maintenance_terminal_fault(cancelled: bool, stale: bool, deadline_exceeded: bool) -> Option<&'static str> {
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

//#endregion 📮️RuntimeMailbox
pub const WORLD3D_DEADLINE_CAPACITY: usize = 256;
pub const WORLD3D_DEADLINE_ID_BYTES: usize = 256;

//#region 📥️FrameBuildInputs
/// 📥️ The fixed scalar `Send`-safe slice of `AppRuntime` this job needs.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameBuildInputs {
    pub wheel_zoom_deadline_ms: f64,
    pub now_ms: f64,
}

/// 📤️ The worker's fixed scalar candidate, revalidated against live state before use.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrameDirectives {
    pub wheel_zoom_deadline_cleared: bool,
}
//#endregion 📥️FrameBuildInputs
