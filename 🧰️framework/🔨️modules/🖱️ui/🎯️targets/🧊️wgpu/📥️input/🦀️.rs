// #region input
//! 🖱️ Pointer and keyboard input state for hit testing.

use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::geometry::Rect;
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
        if self.len == PENDING_KEY_CAPACITY || bytes > PENDING_KEY_BYTE_CAPACITY || self.bytes.checked_add(bytes).is_none_or(|next| next > PENDING_KEY_BYTE_CAPACITY) {
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
        if self.len == PENDING_KEY_CAPACITY || bytes > PENDING_KEY_BYTE_CAPACITY || self.bytes.checked_add(bytes).is_none_or(|next| next > PENDING_KEY_BYTE_CAPACITY) {
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

    pub fn claim_actions(&mut self, byte_credits: &[usize]) -> Result<crate::wgpu::BoundedActionClaimBatch, BoundedActionFault> {
        self.pending_actions.claim_batch(byte_credits)
    }

    pub fn reserve_claimed_action<'a>(&'a mut self, claim: crate::wgpu::BoundedActionClaim, controller_id: &str, action: &str) -> Result<crate::wgpu::BoundedClaimedActionReservation<'a>, BoundedActionFault> {
        self.pending_actions.reserve_claimed(claim, controller_id, action)
    }

    pub fn draft_claimed_action(&self, claim: crate::wgpu::BoundedActionClaim, controller_id: &str, action: &str) -> Result<crate::wgpu::BoundedClaimedActionDraft, BoundedActionFault> {
        self.pending_actions.draft_claimed(claim, controller_id, action)
    }

    pub fn publish_prepared_claimed_action(&mut self, prepared: crate::wgpu::PreparedClaimedAction) -> Result<(), BoundedActionFault> {
        self.pending_actions.publish_prepared_claimed(prepared)
    }

    pub fn publish_prepared_claimed_actions(&mut self, prepared: crate::wgpu::PreparedClaimedActionBatch) -> Result<(), BoundedActionFault> {
        self.pending_actions.publish_prepared_claimed_batch(prepared)
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


//#region 🎯️RetainedHitRegistry
/// 🎯️ One interactive retained node, projected onto this region's flat pointer registry.
///
/// 🩸️ The shell's registry used to hold the chrome and NOTHING of any retained window body: a
/// pointer inside a published `mounted_layout` row rect answered the WINDOW's
/// `HitKind::ScrollRegion`, so no row action dispatched, no wheel reached a scene and no hover
/// resolved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6). `rect` is minted by the SAME accumulated
/// origin the retained paint walk paints at, so a row can never again be drawn at one rectangle and
/// hit-tested at another.
#[derive(Clone, Debug, PartialEq)]
#[cfg(feature = "wgpu-engine")]
pub struct RetainedHitRegistration {
    pub node: crate::wgpu::arena::NodeId,
    pub rect: Rect,
    pub kind: HitKind,
    pub control_id: String,
    pub action: Option<ActionDescriptor>,
    pub drag_axis: Option<DragAxis>,
    pub drag_data: Option<HashMap<String, String>>,
}

#[cfg(feature = "wgpu-engine")]
impl RetainedHitRegistration {
    /// 🎯️ The registry entry a host pushes into its own `InputState`.
    pub fn to_hit_target(&self) -> HitTarget<ActionDescriptor> {
        HitTarget { rect: self.rect, event: self.action.clone(), control_id: Some(self.control_id.clone()), kind: self.kind, drag_axis: self.drag_axis, drag_data: self.drag_data.clone() }
    }
}

/// 🌳️ Which row of its owning `Tree` a synthesized `Stack` is, re-derived from that tree's own
/// still-intact spec by key — the same discriminator `mounted_layout::tree_row_kind` measures with,
/// so the row a pointer resolves and the row the layout published are one classification.
#[cfg(feature = "wgpu-engine")]
enum RetainedTreeRow<'a> {
    Section(&'a crate::wgpu::component::ui::UiTreeSectionNode),
    Item(&'a crate::wgpu::component::ui::UiTreeItemNode),
}

#[cfg(feature = "wgpu-engine")]
fn retained_tree_row<'a>(tree: &'a crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId) -> Option<RetainedTreeRow<'a>> {
    let crate::wgpu::tree::NodeKey::Explicit(key) = &tree.node(id)?.key else { return None };
    let owner = crate::wgpu::mounted_layout::owning_tree_spec(tree, id)?;
    if let Some(section) = owner.sections.iter().find(|section| &section.id == key) {
        return Some(RetainedTreeRow::Section(section));
    }
    owner.sections.iter().find_map(|section| crate::wgpu::mounted_layout::find_tree_item(&section.items, key, 0)).map(RetainedTreeRow::Item)
}

/// 🖱️ The `HitKind`/control id an engine surface canvas registers under. `World3d` is its own kind;
/// every other bespoke-dispatch surface reuses the `ScrollRegion` + `.pane`/`.map` convention
/// `ShellState::scroll_region_is_scene_surface` already reads, so a wheel over one propagates to the
/// scene instead of scrolling the window that hosts it.
#[cfg(feature = "wgpu-engine")]
fn retained_scene_hit(scene: &crate::wgpu::component::ui::UiComponentSceneNode) -> (HitKind, String) {
    use crate::wgpu::component::ui::SurfaceKind;
    match scene.component_kind {
        SurfaceKind::World3d => (HitKind::World3d, scene.surface_id.clone()),
        SurfaceKind::NodeGraph | SurfaceKind::Board2d => (HitKind::ScrollRegion, format!("{}.pane", scene.surface_id)),
        SurfaceKind::TiledMap => (HitKind::ScrollRegion, format!("{}.map", scene.surface_id)),
        _ => (HitKind::Generic, scene.surface_id.clone()),
    }
}

/// 🎯️ Projects ONE laid-out retained node onto a registry entry, or `None` for a node with no
/// interaction semantics of its own (plain containers, text, separators, a `Tree`'s own frame —
/// the rows carry that one). `rect` is the node's absolute painted rect; a tree row's entry is
/// clipped to its OWN band (`metrics.row_height`) because a row's published height also covers the
/// nested rows it reveals, and those register entries of their own.
#[cfg(feature = "wgpu-engine")]
pub fn retained_hit_registration(tree: &crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId, rect: Rect, metrics: &crate::wgpu::layout::TreeRowMetrics) -> Option<RetainedHitRegistration> {
    use crate::wgpu::component::ui::UiNode;
    let node = tree.node(id)?;
    // 🕳️ A mounted-but-unplaced node (a collapsed branch's row, a `Tree` item action that the row
    // geometry never gives a rect of its own) must never take the pointer from the node actually
    // drawn there.
    if !node.spec.0.presence().visible() || rect.w <= 0.0 || rect.h <= 0.0 {
        return None;
    }
    let entry = |kind: HitKind, control_id: String, action: Option<ActionDescriptor>, rect: Rect| {
        Some(RetainedHitRegistration { node: id, rect, kind, control_id, action, drag_axis: None, drag_data: None })
    };
    match &node.spec.0 {
        UiNode::Stack(stack) => match retained_tree_row(tree, id) {
            Some(RetainedTreeRow::Section(section)) => {
                let band = Rect::new(rect.x, rect.y, rect.w, crate::wgpu::layout::tree_section_header_height(section, metrics).min(rect.h));
                if band.h <= 0.0 {
                    return None;
                }
                entry(HitKind::TreeItem, format!("section.chevron.{}", section.id), stack.activate.clone(), band)
            }
            Some(RetainedTreeRow::Item(item)) => {
                let band = Rect::new(rect.x, rect.y, rect.w, metrics.row_height.min(rect.h));
                if band.h <= 0.0 {
                    return None;
                }
                let draggable = item.draggable.unwrap_or(false);
                let drag_data = item.drag_data.clone().filter(|data| draggable && !data.is_empty());
                Some(RetainedHitRegistration {
                    node: id,
                    rect: band,
                    kind: HitKind::TreeItem,
                    control_id: format!("tree.label.{}", item.id),
                    action: stack.activate.clone().or_else(|| item.action.clone()),
                    drag_axis: draggable.then_some(DragAxis::Both),
                    drag_data,
                })
            }
            None => {
                let activate = stack.activate.clone()?;
                let control_id = stack.id.clone().filter(|id| !id.is_empty()).unwrap_or_else(|| activate.action.clone());
                entry(HitKind::Button, control_id, Some(activate), rect)
            }
        },
        UiNode::Button(button) => entry(HitKind::Button, button.id.clone().unwrap_or_else(|| button.action.action.clone()), Some(button.action.clone()), rect),
        UiNode::Input(input) => entry(HitKind::Input, input.id.clone(), None, rect),
        UiNode::Select(select) => entry(HitKind::Select, select.id.clone(), None, rect),
        UiNode::Toggle(toggle) => entry(HitKind::Toggle, toggle.id.clone(), None, rect),
        UiNode::Slider(slider) => Some(RetainedHitRegistration { node: id, rect, kind: HitKind::Slider, control_id: slider.id.clone(), action: None, drag_axis: Some(DragAxis::Horizontal), drag_data: None }),
        UiNode::ComponentScene(scene) => {
            let (kind, control_id) = retained_scene_hit(scene);
            entry(kind, control_id, None, rect)
        }
        _ => None,
    }
}
//#endregion 🎯️RetainedHitRegistry

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs"]
mod tests;

#[cfg(all(test, feature = "wgpu-engine"))]
#[path = "../../../🧪️tests/🎯️retained-hit-targets/🦀️.rs"]
mod retained_hit_target_tests;
// #endregion input
