// #region input
//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::wgpu::geometry::Rect;
use std::rc::Rc;

use std::collections::HashMap;

type TreeDragPayload = HashMap<String, String>;

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
    pub text_buffer: String,
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
            text_buffer: String::new(),
            cursor_pos: 0,
            hit_targets: Vec::new(),
            pending_events: Vec::new(),
            pending_keys: Vec::new(),
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
        self.drag = DragState { active: true, button, start_x: x, start_y: y, current_x: x, current_y: y, target_id, axis, kind, points: vec![[x, y]] };
    }

    pub fn update_drag(&mut self, x: f32, y: f32) {
        if self.drag.active {
            self.drag.current_x = x;
            self.drag.current_y = y;
            self.drag.points.push([x, y]);
        }
    }

    pub fn end_drag(&mut self) -> DragState {
        let drag = self.drag.clone();
        self.drag = DragState::default();
        drag
    }

    pub fn drain_events(&mut self) -> Vec<E> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn drain_keys(&mut self) -> Vec<KeyAction> {
        std::mem::take(&mut self.pending_keys)
    }

    pub fn queue_event(&mut self, event: E) {
        self.pending_events.push(event);
    }

    pub fn queue_key(&mut self, action: KeyAction) {
        self.pending_keys.push(action);
    }

    pub fn focus_input(&mut self, id: &str, value: &str) {
        self.focused_id = Some(id.to_string());
        self.text_buffer = value.to_string();
        self.cursor_pos = value.len();
    }

    pub fn blur_input(&mut self) {
        self.focused_id = None;
        self.text_buffer.clear();
        self.cursor_pos = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_pos <= self.text_buffer.len() {
            self.text_buffer.insert(self.cursor_pos, ch);
            self.cursor_pos += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn delete_forward(&mut self) {
        if self.cursor_pos < self.text_buffer.len() {
            self.text_buffer.remove(self.cursor_pos);
        }
    }

    pub fn move_cursor(&mut self, delta: i32) {
        let len = self.text_buffer.len() as i32;
        self.cursor_pos = ((self.cursor_pos as i32) + delta).clamp(0, len) as usize;
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
