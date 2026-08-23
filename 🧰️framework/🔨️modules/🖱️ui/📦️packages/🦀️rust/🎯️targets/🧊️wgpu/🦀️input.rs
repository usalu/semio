// #region input
//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::wgpu::geometry::Rect;
#[cfg(test)]
use crate::wgpu::ActionDescriptor;
use crate::wgpu::{BoundedAction, BoundedActionBatchReservation, BoundedActionFault, BoundedActionQueue, BoundedActionReservation};
use std::rc::Rc;

use std::collections::HashMap;

type TreeDragPayload = HashMap<String, String>;
const HIT_TARGET_CAPACITY: usize = 8_192;
const PENDING_KEY_CAPACITY: usize = 64;
const PENDING_KEY_BYTE_CAPACITY: usize = 4 * 1024;
const DRAG_POINT_CAPACITY: usize = 4_096;
const INPUT_TEXT_PAGE_BYTES: usize = 16 * 1024;

#[derive(Clone, Debug)]
pub struct HitTarget<E> {
    pub rect: Rect,
    pub event: Option<E>,
    pub control_id: Option<String>,
    pub kind: HitKind,
    pub drag_axis: Option<DragAxis>,
    pub drag_data: Option<HashMap<String, String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAxis {
    Horizontal,
    Vertical,
    Both,
    Ring,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDropPosition {
    Before,
    After,
    Inside,
}

#[derive(Clone, Debug)]
pub struct TreeDragState {
    pub source_id: String,
    pub drag_data: TreeDragPayload,
    pub x: f32,
    pub y: f32,
    pub drop_target_id: Option<String>,
    pub drop_position: TreeDropPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitKind {
    Button,
    Toggle,
    Input,
    Select,
    Slider,
    TreeItem,
    TreeDropTarget,
    PanelTab,
    NavbarItem,
    Window,
    World3d,
    PanelResize,
    DockSplit,
    DockJoinCorner,
    ScrollRegion,
    ContextMenu,
    DropdownItem,
    Generic,
}

#[derive(Clone, Debug, Default)]
pub struct PointerModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl PointerModifiers {
    pub fn ctrl_or_meta(&self) -> bool {
        self.ctrl || self.meta
    }
}

#[derive(Clone, Debug, Default)]
pub struct DragState {
    pub active: bool,
    pub button: i16,
    pub start_x: f32,
    pub start_y: f32,
    pub current_x: f32,
    pub current_y: f32,
    pub target_id: Option<String>,
    pub axis: Option<DragAxis>,
    pub kind: Option<HitKind>,
    pub points: Vec<[f32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyAction {
    Char(String),
    Backspace,
    Delete,
    Enter,
    Escape,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Function(u8),
    Tab,
    Space(bool),
}

struct FixedKeyQueue {
    slots: Box<[Option<KeyAction>; PENDING_KEY_CAPACITY]>,
    head: usize,
    len: usize,
    bytes: usize,
}

impl Default for FixedKeyQueue {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0 }
    }
}

impl FixedKeyQueue {
    fn key_bytes(key: &KeyAction) -> usize {
        match key {
            KeyAction::Char(value) => value.len(),
            _ => 0,
        }
    }

    fn push_back(&mut self, key: KeyAction) -> Result<(), KeyAction> {
        let bytes = Self::key_bytes(&key);
        if self.len == PENDING_KEY_CAPACITY || bytes > PENDING_KEY_BYTE_CAPACITY || self.bytes.checked_add(bytes).map_or(true, |next| next > PENDING_KEY_BYTE_CAPACITY) {
            return Err(key);
        }
        let index = (self.head + self.len) % PENDING_KEY_CAPACITY;
        self.slots[index] = Some(key);
        self.len += 1;
        self.bytes += bytes;
        Ok(())
    }

    fn push_front(&mut self, key: KeyAction) -> Result<(), KeyAction> {
        let bytes = Self::key_bytes(&key);
        if self.len == PENDING_KEY_CAPACITY || bytes > PENDING_KEY_BYTE_CAPACITY || self.bytes.checked_add(bytes).map_or(true, |next| next > PENDING_KEY_BYTE_CAPACITY) {
            return Err(key);
        }
        self.head = (self.head + PENDING_KEY_CAPACITY - 1) % PENDING_KEY_CAPACITY;
        self.slots[self.head] = Some(key);
        self.len += 1;
        self.bytes += bytes;
        Ok(())
    }

    fn pop_front(&mut self) -> Option<KeyAction> {
        if self.len == 0 {
            return None;
        }
        let key = self.slots[self.head].take();
        self.head = (self.head + 1) % PENDING_KEY_CAPACITY;
        self.len -= 1;
        if let Some(key) = key.as_ref() {
            self.bytes -= Self::key_bytes(key);
        }
        key
    }

    fn pop_back(&mut self) -> Option<KeyAction> {
        if self.len == 0 {
            return None;
        }
        let index = (self.head + self.len - 1) % PENDING_KEY_CAPACITY;
        let key = self.slots[index].take();
        self.len -= 1;
        if let Some(key) = key.as_ref() {
            self.bytes -= Self::key_bytes(key);
        }
        key
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct InputState<E> {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    pub wheel_delta: f32,
    pub modifiers: PointerModifiers,
    pub drag: DragState,
    pub hovered_id: Option<String>,
    pub focused_id: Option<String>,
    pub text_buffer: ui_contract::TextEditAuthority,
    text_view: String,
    text_view_start: usize,
    text_projection_pending: bool,
    text_fault: Option<ui_contract::TextEditFault>,
    action_fault: Option<BoundedActionFault>,
    pub cursor_pos: usize,
    pub hit_targets: Vec<HitTarget<E>>,
    pending_actions: BoundedActionQueue,
    pending_keys: FixedKeyQueue,
    pub right_click_pos: Option<(f32, f32)>,
}

impl<E> Default for InputState<E> {
    fn default() -> Self {
        Self {
            pointer_x: 0.0,
            pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            wheel_delta: 0.0,
            modifiers: PointerModifiers::default(),
            drag: DragState::default(),
            hovered_id: None,
            focused_id: None,
            text_buffer: ui_contract::TextEditAuthority::default(),
            text_view: String::new(),
            text_view_start: 0,
            text_projection_pending: false,
            text_fault: None,
            action_fault: None,
            cursor_pos: 0,
            hit_targets: Vec::with_capacity(HIT_TARGET_CAPACITY),
            pending_actions: BoundedActionQueue::default(),
            pending_keys: FixedKeyQueue::default(),
            right_click_pos: None,
        }
    }
}

impl<E: Clone> InputState<E> {
    pub fn clear_frame(&mut self) {
        self.hit_targets.clear();
        self.wheel_delta = 0.0;
        self.right_click_pos = None;
    }

    pub fn register_hit(&mut self, target: HitTarget<E>) {
        if self.hit_targets.len() == HIT_TARGET_CAPACITY {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return;
        }
        self.hit_targets.push(target);
    }

    pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {
        self.hit_targets.iter().rev().find(|target| target.rect.contains(x, y))
    }

    pub fn update_hover(&mut self, x: f32, y: f32) {
        self.pointer_x = x;
        self.pointer_y = y;
        self.hovered_id = self.hit_at(x, y).and_then(|hit| hit.control_id.clone());
    }

    pub fn begin_drag(&mut self, x: f32, y: f32, button: i16, target_id: Option<String>, axis: Option<DragAxis>, kind: Option<HitKind>) {
        let mut points = Vec::with_capacity(DRAG_POINT_CAPACITY);
        points.push([x, y]);
        self.drag = DragState { active: true, button, start_x: x, start_y: y, current_x: x, current_y: y, target_id, axis, kind, points };
    }

    pub fn update_drag(&mut self, x: f32, y: f32) {
        if self.drag.active {
            self.drag.current_x = x;
            self.drag.current_y = y;
            if self.drag.points.len() == DRAG_POINT_CAPACITY {
                if let Some(last) = self.drag.points.last_mut() {
                    *last = [x, y];
                }
            } else {
                self.drag.points.push([x, y]);
            }
        }
    }

    pub fn end_drag(&mut self) -> DragState {
        let drag = self.drag.clone();
        self.drag = DragState::default();
        drag
    }

    pub fn take_action_step(&mut self) -> Result<Option<BoundedAction>, BoundedActionFault> {
        if let Some(fault) = self.action_fault.take() {
            return Err(fault);
        }
        Ok(self.pending_actions.pop_front())
    }

    #[cfg(test)]
    pub fn drain_events(&mut self) -> Vec<ActionDescriptor> {
        let mut events = Vec::new();
        while let Some(action) = self.take_action_step().expect("action authority live") {
            events.push(action.into_descriptor().expect("bounded action materializes"));
        }
        events
    }

    pub fn take_key_step(&mut self) -> Option<KeyAction> {
        self.pending_keys.pop_front()
    }

    pub fn reserve_action<'a>(&'a mut self, controller_id: &str, action: &str, byte_credits: usize) -> Result<BoundedActionReservation<'a>, BoundedActionFault> {
        self.pending_actions.reserve(controller_id, action, byte_credits)
    }

    pub fn reserve_actions(&mut self, item_credits: usize, byte_credits: usize) -> Result<BoundedActionBatchReservation<'_>, BoundedActionFault> {
        self.pending_actions.reserve_batch(item_credits, byte_credits)
    }

    pub fn claim_action(&mut self, byte_credits: usize) -> Result<crate::wgpu::BoundedActionClaim, BoundedActionFault> {
        self.pending_actions.claim(byte_credits)
    }

    pub fn reserve_claimed_action<'a>(&'a mut self, claim: crate::wgpu::BoundedActionClaim, controller_id: &str, action: &str) -> Result<crate::wgpu::BoundedClaimedActionReservation<'a>, BoundedActionFault> {
        self.pending_actions.reserve_claimed(claim, controller_id, action)
    }

    pub fn release_action_claim(&mut self, claim: crate::wgpu::BoundedActionClaim) -> Result<(), BoundedActionFault> {
        self.pending_actions.release_claim(claim)
    }

    pub fn publish_action(&mut self, controller_id: &str, action: &str, byte_credits: usize, build: impl FnOnce(&mut crate::wgpu::BoundedActionBuilder, &str) -> Result<(), BoundedActionFault>) -> Result<(), BoundedActionFault> {
        let text_view = self.text_view.as_str();
        let mut reservation = self.pending_actions.reserve(controller_id, action, byte_credits)?;
        build(reservation.builder(), text_view)?;
        reservation.publish()
    }

    pub fn record_action_fault(&mut self, fault: BoundedActionFault) {
        self.action_fault = Some(fault);
        self.text_fault = Some(match fault {
            BoundedActionFault::ByteCredits | BoundedActionFault::StringCredits => ui_contract::TextEditFault::ByteCredits,
            _ => ui_contract::TextEditFault::ItemCredits,
        });
    }

    pub fn queue_key(&mut self, action: KeyAction) -> Result<(), KeyAction> {
        if let Err(action) = self.pending_keys.push_back(action) {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return Err(action);
        }
        Ok(())
    }

    pub fn retry_key(&mut self, action: KeyAction) -> Result<(), KeyAction> {
        self.pending_keys.push_front(action)
    }

    pub fn focus_input_owned(&mut self, id: String, value: String) {
        if id.len() > 1024 {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return;
        }
        if value.len() > INPUT_TEXT_PAGE_BYTES {
            self.text_fault = Some(ui_contract::TextEditFault::ByteCredits);
            return;
        }
        if let Err(fault) = self.text_buffer.replace_owned(value) {
            self.text_fault = Some(fault);
            return;
        }
        self.focused_id = Some(id);
        self.reset_text_view();
    }

    pub fn focus_id_owned(&mut self, id: String) {
        if id.len() > 1024 {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return;
        }
        if let Err(fault) = self.text_buffer.replace_owned(String::new()) {
            self.text_fault = Some(fault);
            return;
        }
        self.focused_id = Some(id);
        self.reset_text_view();
    }

    fn reset_text_view(&mut self) {
        self.text_view.clear();
        self.text_view_start = 0;
        self.text_projection_pending = false;
        self.cursor_pos = 0;
    }

    pub fn blur_input(&mut self) {
        self.focused_id = None;
        self.text_fault = self.text_buffer.replace_owned(String::new()).err();
        self.reset_text_view();
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_pos <= self.text_buffer.len() {
            let _ = self.text_buffer.enqueue_owned(self.text_buffer.generation(), ch.to_string(), self.cursor_pos, self.cursor_pos);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            if let Ok(start) = self.text_buffer.root().previous_boundary(self.cursor_pos) {
                let _ = self.text_buffer.enqueue_owned(self.text_buffer.generation(), String::new(), start, self.cursor_pos);
            }
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.text_buffer.len() {
            if let Ok(end) = self.text_buffer.root().next_boundary(self.cursor_pos) {
                let _ = self.text_buffer.enqueue_owned(self.text_buffer.generation(), String::new(), self.cursor_pos, end);
            }
        }
    }

    pub fn move_cursor(&mut self, delta: i32) {
        if delta < 0 {
            self.cursor_pos = self.text_buffer.root().previous_boundary(self.cursor_pos).unwrap_or(self.cursor_pos);
        } else if delta > 0 {
            self.cursor_pos = self.text_buffer.root().next_boundary(self.cursor_pos).unwrap_or(self.cursor_pos);
        }
    }

    pub fn drive_text_step(&mut self) -> Result<bool, ui_contract::TextEditFault> {
        if let Some(fault) = self.text_fault.take() {
            return Err(fault);
        }
        if self.text_projection_pending {
            if let Some(view) = self.text_buffer.step_projection(1)? {
                self.text_view = view;
                self.text_projection_pending = false;
            }
            return Ok(true);
        }
        match self.text_buffer.step(self.text_buffer.generation(), 1, false)? {
            ui_contract::TextEditProgress::Published { caret } => {
                self.cursor_pos = caret;
                let start = self.text_buffer.root().boundary_at_or_before(self.cursor_pos.saturating_sub(2048))?;
                self.text_buffer.start_projection(start, 4096)?;
                self.text_view_start = start;
                self.text_projection_pending = true;
                Ok(true)
            }
            ui_contract::TextEditProgress::Idle => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn text_view(&self) -> &str {
        &self.text_view
    }

    pub fn text_view_cursor(&self) -> usize {
        self.cursor_pos.saturating_sub(self.text_view_start).min(self.text_view.len())
    }

    pub fn close_step(&mut self) -> Result<bool, ui_contract::TextEditFault> {
        if self.hit_targets.pop().is_some() {
            return Ok(false);
        }
        if self.pending_actions.pop_back().is_some() {
            return Ok(false);
        }
        if !self.pending_actions.close_claim_step() {
            return Ok(false);
        }
        if self.pending_keys.pop_back().is_some() {
            return Ok(false);
        }
        if self.drag.points.pop().is_some() {
            return Ok(false);
        }
        if self.drag.target_id.take().is_some() || self.hovered_id.take().is_some() {
            return Ok(false);
        }
        if self.focused_id.take().is_some() {
            return Ok(false);
        }
        if !self.text_view.is_empty() {
            self.text_view.clear();
            return Ok(false);
        }
        self.text_projection_pending = false;
        self.text_fault = None;
        self.action_fault = None;
        self.text_buffer.close_step(1)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.hit_targets.is_empty()
            && self.pending_actions.is_empty()
            && self.pending_keys.is_empty()
            && self.drag.points.is_empty()
            && self.drag.target_id.is_none()
            && self.hovered_id.is_none()
            && self.focused_id.is_none()
            && self.text_view.is_empty()
            && !self.text_projection_pending
            && self.action_fault.is_none()
            && self.text_buffer.terminal_is_empty()
    }
}

#[derive(Clone)]
pub struct PointerCallbacks {
    pub on_move: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_button: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
    pub on_wheel: Rc<dyn Fn(f32, f32, f32, PointerModifiers)>,
    pub on_key: Rc<dyn Fn(KeyAction, PointerModifiers)>,
    pub on_context_menu: Rc<dyn Fn(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_at_prefers_content_registered_after_scroll_region() {
        let mut input = InputState::<()>::default();
        let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
        let row = Rect::new(0.0, 24.0, 200.0, 24.0);
        input.register_hit(HitTarget { rect: scroll, event: None, control_id: Some("scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        input.register_hit(HitTarget { rect: row, event: None, control_id: Some("tree.label.item-1".into()), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
        let hit = input.hit_at(10.0, 36.0).expect("row point should hit");
        assert_eq!(hit.control_id.as_deref(), Some("tree.label.item-1"));
        assert_eq!(hit.kind, HitKind::TreeItem);
    }

    #[test]
    fn event_queue_has_fixed_credits_and_transfers_one_fifo_item() {
        let mut input = InputState::<ActionDescriptor>::default();
        for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
            let action = format!("event-{index}");
            input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("queue credit");
        }
        assert_eq!(input.pending_actions.len(), crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY);
        for expected in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
            assert_eq!(input.take_action_step().expect("authority").expect("event").into_descriptor().expect("materialized").action, format!("event-{expected}"));
        }
        assert!(input.take_action_step().expect("authority").is_none());
    }

    #[test]
    fn event_queue_close_retires_one_owned_slot_per_step() {
        let mut input = InputState::<ActionDescriptor>::default();
        input.reserve_action("controller", "first", 128).expect("first").publish().expect("first queue");
        input.reserve_action("controller", "second", 128).expect("second").publish().expect("second queue");
        assert!(!input.close_step().expect("first retirement step"));
        assert_eq!(input.pending_actions.len(), 1);
        assert!(!input.close_step().expect("second retirement step"));
        assert!(input.pending_actions.is_empty());
    }

    #[test]
    fn saturated_reservation_does_not_consume_semantic_source_and_retry_keeps_fifo() {
        let mut input = InputState::<ActionDescriptor>::default();
        for index in 0..crate::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY {
            let action = format!("event-{index}");
            input.reserve_action("controller", &action, 128).expect("reservation").publish().expect("publish");
        }
        let semantic_source = String::from("retry-owned");
        let result = input.publish_action("controller", "retry", 128, |builder, _| {
            builder.begin_object(None)?;
            builder.string(Some("value"), &semantic_source)?;
            builder.end_container()
        });
        assert_eq!(result, Err(BoundedActionFault::ItemCredits));
        assert_eq!(semantic_source, "retry-owned");
        assert_eq!(input.take_action_step().expect("authority").expect("first").into_descriptor().expect("descriptor").action, "event-0");
        input
            .publish_action("controller", "retry", 128, |builder, _| {
                builder.begin_object(None)?;
                builder.string(Some("value"), &semantic_source)?;
                builder.end_container()
            })
            .expect("retry publication");
        let mut last = None;
        while let Some(action) = input.take_action_step().expect("authority") {
            last = Some(action.into_descriptor().expect("descriptor"));
        }
        assert_eq!(last.expect("last").action, "retry");
    }

    #[test]
    fn action_fault_is_observed_before_another_queued_owner() {
        let mut input = InputState::<ActionDescriptor>::default();
        input.reserve_action("controller", "queued", 128).expect("reservation").publish().expect("publish");
        input.record_action_fault(BoundedActionFault::ByteCredits);
        assert!(matches!(input.take_action_step(), Err(BoundedActionFault::ByteCredits)));
        assert_eq!(input.take_action_step().expect("authority").expect("queued").into_descriptor().expect("descriptor").action, "queued");
    }
}
// #endregion input
