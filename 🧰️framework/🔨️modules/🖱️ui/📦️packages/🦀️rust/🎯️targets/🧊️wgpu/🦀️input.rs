// #region input
//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::wgpu::geometry::Rect;
use std::rc::Rc;

use std::collections::HashMap;

type TreeDragPayload = HashMap<String, String>;
const HIT_TARGET_CAPACITY: usize = 8_192;
const PENDING_EVENT_CAPACITY: usize = 256;
const PENDING_KEY_CAPACITY: usize = 64;
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
    pub cursor_pos: usize,
    pub hit_targets: Vec<HitTarget<E>>,
    pub pending_events: Vec<E>,
    pub pending_keys: Vec<KeyAction>,
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
            cursor_pos: 0,
            hit_targets: Vec::with_capacity(HIT_TARGET_CAPACITY),
            pending_events: Vec::with_capacity(PENDING_EVENT_CAPACITY),
            pending_keys: Vec::with_capacity(PENDING_KEY_CAPACITY),
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

    pub fn drain_events(&mut self) -> Vec<E> {
        std::mem::replace(&mut self.pending_events, Vec::with_capacity(PENDING_EVENT_CAPACITY))
    }

    pub fn drain_keys(&mut self) -> Vec<KeyAction> {
        std::mem::replace(&mut self.pending_keys, Vec::with_capacity(PENDING_KEY_CAPACITY))
    }

    pub fn queue_event(&mut self, event: E) {
        if self.pending_events.len() == PENDING_EVENT_CAPACITY {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return;
        }
        self.pending_events.push(event);
    }

    pub fn queue_key(&mut self, action: KeyAction) {
        if self.pending_keys.len() == PENDING_KEY_CAPACITY {
            self.text_fault = Some(ui_contract::TextEditFault::ItemCredits);
            return;
        }
        self.pending_keys.push(action);
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
        if self.pending_events.pop().is_some() {
            return Ok(false);
        }
        if self.pending_keys.pop().is_some() {
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
        self.text_buffer.close_step(1)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.hit_targets.is_empty()
            && self.pending_events.is_empty()
            && self.pending_keys.is_empty()
            && self.drag.points.is_empty()
            && self.drag.target_id.is_none()
            && self.hovered_id.is_none()
            && self.focused_id.is_none()
            && self.text_view.is_empty()
            && !self.text_projection_pending
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
}
// #endregion input
