// #region paint
//! 🖌️ Retained paint pass. Per-`UiNode`-variant drawing logic mechanically ported from the
//! immediate-mode `widgets::render_*` functions (see that region's doc comment for why it still
//! exists), reading resolved geometry from `tree::LayoutBucket` (accumulating parent-relative
//! offsets while walking, since taffy's `Layout::location` is parent-relative — see that struct's
//! doc comment) instead of the old `bounds: Rect` argument an immediate-mode caller threaded down.
//! Interaction-derived visuals (hover/focus/active/selected) read live `NodeFlags`/`WidgetState`,
//! written each frame by `events::EventRouter` (M5, landed) — no longer default/empty by the time
//! `paint_tree` runs, as an earlier revision of this comment used to caveat. `WidgetState`-backed
//! composites have since gained real paint support too: an open `Select`'s popup expands live
//! (`paint_select`'s `open`/`retained` params, wired by the W2 pass — see
//! `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-ui-wgpu-integration.md`), and a focused
//! `Input`'s caret/selection-highlight render straight from its live `EditState` (`paint_input`,
//! W2 widget-visuals pass). `Tree`'s live scroll offset (`WidgetState::scroll_offset`) remains the
//! one rest-state-only exception — no scrollable-viewport paint exists yet, out of every pass to
//! date's scope.

use crate::wgpu::arena::NodeId;
use crate::wgpu::chrome::{chrome_item_bg, item_bg, item_text, push_chrome_border, push_control_border, push_icon, ICON_TINY};
use crate::wgpu::component::ui::{
    UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueNode, UiNode, UiNumberStepperNode, UiPresence, UiRingNode, UiSectionNode, UiSelectItem,
    UiSelectNode, UiSliderNode, UiStackNode, UiState, UiStatus, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::geometry::Rect;
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::{Level, Rgba, Theme};
use crate::wgpu::tree::{EditState, NodeFlags, NodeKey, UiTree};
use crate::wgpu::widgets::{draw_text_on, wrap_text};
use crate::wgpu::IconName;
use crate::wgpu::Label;
use crate::wgpu::UiTreeActionPlacement;

const PANEL_HEADER: f32 = 24.0;
const TREE_ROW_HEIGHT: f32 = 24.0;
const TREE_INDENT_PER_LEVEL: f32 = 10.0;
const TREE_TOGGLE_WIDTH: f32 = 14.0;
const TREE_ICON_SIZE: f32 = 14.0;
pub const RETAINED_NODE_TEXT_MAX_BYTES: usize = 4 * 1024 * 1024;
const RETAINED_NODE_COLLECTION_ITEMS: usize = 256;
const RETAINED_NODE_FIXED_OUTPUT_ITEMS: usize = 8;
const RETAINED_NODE_FIXED_OUTPUT_BYTES: usize = 64 * 1024;
const RETAINED_TREE_DEPTH: usize = 64;

/// ✒️ Retained byte, glyph, line, and pen authority shared by node and Shell text.
#[derive(Default)]
pub struct RetainedGlyphCursor {
    byte: usize,
    line: usize,
    pen_x: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedGlyphStep {
    Pending,
    Complete,
    Fault,
}

impl RetainedGlyphCursor {
    pub fn reset(&mut self) {
        self.byte = 0;
        self.line = 0;
        self.pen_x = 0.0;
    }

    pub fn byte(&self) -> usize {
        self.byte
    }

    pub fn close_step(&mut self) -> bool {
        if self.byte != 0 || self.line != 0 || self.pen_x != 0.0 {
            self.reset();
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.byte == 0 && self.line == 0 && self.pen_x == 0.0
    }
}

/// ✒️ Advances one UTF-8 scalar and at most one exactly admitted glyph.
pub fn paint_retained_glyph_step(value: &str, bounds: Rect, size: f32, color: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    if value.len() > RETAINED_NODE_TEXT_MAX_BYTES || !value.is_char_boundary(cursor.byte) {
        return RetainedGlyphStep::Fault;
    }
    if cursor.byte == value.len() {
        return RetainedGlyphStep::Complete;
    }
    let Some(ch) = value[cursor.byte..].chars().next() else { return RetainedGlyphStep::Fault };
    let Some(next_byte) = cursor.byte.checked_add(ch.len_utf8()) else { return RetainedGlyphStep::Fault };
    if ch == '\n' {
        let Some(next_line) = cursor.line.checked_add(1) else { return RetainedGlyphStep::Fault };
        cursor.byte = next_byte;
        cursor.line = next_line;
        cursor.pen_x = 0.0;
        return RetainedGlyphStep::Pending;
    }
    if draw.begin_retained_output(1, std::mem::size_of::<crate::wgpu::draw::UiInstance>()).is_err() {
        return RetainedGlyphStep::Fault;
    }
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let glyph = atlas.ensure_glyph(ch, size);
    let (atlas_x, atlas_y, width, height, advance, bearing_x, bearing_y) = (glyph.atlas_x, glyph.atlas_y, glyph.width, glyph.height, glyph.advance, glyph.bearing_x, glyph.bearing_y);
    if cursor.pen_x > 0.0 && cursor.pen_x + advance > bounds.w.max(1.0) {
        let Some(next_line) = cursor.line.checked_add(1) else {
            let _ = draw.finish_retained_output();
            return RetainedGlyphStep::Fault;
        };
        cursor.line = next_line;
        cursor.pen_x = 0.0;
    }
    let baseline = bounds.y + size + size * 1.35 * cursor.line as f32;
    let x = bounds.x + cursor.pen_x + bearing_x;
    let y = baseline - height as f32 - bearing_y;
    let uv = [atlas_x as f32 / atlas_w, atlas_y as f32 / atlas_h, (atlas_x + width) as f32 / atlas_w, (atlas_y + height) as f32 / atlas_h];
    draw.push_glyph([x, y, (width as f32).max(1.0), (height as f32).max(1.0)], color, uv);
    if draw.finish_retained_output().is_err() {
        return RetainedGlyphStep::Fault;
    }
    cursor.byte = next_byte;
    cursor.pen_x += advance;
    RetainedGlyphStep::Pending
}

/// 🖌️ Generation-qualified cursor for one retained node's byte, glyph, line, and chrome output.
pub(crate) struct RetainedNodePaintCursor {
    node: Option<NodeId>,
    glyph: RetainedGlyphCursor,
    origin_x: f32,
    origin_y: f32,
    chrome: bool,
    phase: u16,
    item: usize,
    section: usize,
    depth: usize,
    path: [usize; RETAINED_TREE_DEPTH],
    row_y: f32,
    visited: usize,
    ascending: bool,
    selected: Option<usize>,
}

impl Default for RetainedNodePaintCursor {
    fn default() -> Self {
        Self { node: None, glyph: RetainedGlyphCursor::default(), origin_x: 0.0, origin_y: 0.0, chrome: false, phase: 0, item: 0, section: 0, depth: 0, path: [0; RETAINED_TREE_DEPTH], row_y: 0.0, visited: 0, ascending: false, selected: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RetainedNodePaintStep {
    Pending,
    Complete,
    Fault,
}

impl RetainedNodePaintCursor {
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.node.is_none()
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.node.take().is_some() {
            self.reset_progress();
            return false;
        }
        self.glyph.close_step()
    }

    fn begin(&mut self, node: NodeId, origin_x: f32, origin_y: f32) {
        self.node = Some(node);
        self.origin_x = origin_x;
        self.origin_y = origin_y;
        self.reset_progress();
    }

    fn finish(&mut self) -> RetainedNodePaintStep {
        self.node = None;
        self.reset_progress();
        RetainedNodePaintStep::Complete
    }

    fn reset_progress(&mut self) {
        self.glyph.reset();
        self.chrome = false;
        self.phase = 0;
        self.item = 0;
        self.section = 0;
        self.depth = 0;
        self.path = [0; RETAINED_TREE_DEPTH];
        self.row_y = 0.0;
        self.visited = 0;
        self.ascending = false;
        self.selected = None;
    }

    fn advance(&mut self, phase: u16) {
        self.phase = phase;
        self.glyph.reset();
        self.chrome = false;
        self.item = 0;
    }
}

fn retained_fixed_output(draw: &mut DrawList, paint: impl FnOnce(&mut DrawList)) -> Result<(), ()> {
    draw.begin_retained_output(RETAINED_NODE_FIXED_OUTPUT_ITEMS, RETAINED_NODE_FIXED_OUTPUT_BYTES)?;
    paint(draw);
    draw.finish_retained_output().map(|_| ())
}

fn retained_presence_step(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: &UiPresence) -> RetainedNodePaintStep {
    if retained_fixed_output(draw, |draw| presence_overlay(draw, bounds, theme, presence)).is_err() {
        RetainedNodePaintStep::Fault
    } else {
        RetainedNodePaintStep::Complete
    }
}

fn retained_text_node_step(value: &str, bounds: Rect, size: f32, color: Rgba, atlas: &mut FontAtlas, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    match paint_retained_glyph_step(value, bounds, size, color, atlas, draw, &mut cursor.glyph) {
        RetainedGlyphStep::Pending => RetainedNodePaintStep::Pending,
        RetainedGlyphStep::Complete => RetainedNodePaintStep::Complete,
        RetainedGlyphStep::Fault => RetainedNodePaintStep::Fault,
    }
}

fn retained_tree_item_at<'a>(tree: &'a UiTreeNode, cursor: &RetainedNodePaintCursor) -> Option<&'a UiTreeItemNode> {
    if cursor.depth == 0 {
        return None;
    }
    let section = tree.sections.get(cursor.section)?;
    let mut item = section.items.get(cursor.path[0])?;
    for level in 1..cursor.depth {
        item = item.items.as_ref()?.get(cursor.path[level])?;
    }
    Some(item)
}

fn retained_tree_sibling_count(tree: &UiTreeNode, cursor: &RetainedNodePaintCursor) -> Option<usize> {
    if cursor.depth == 0 {
        return None;
    }
    if cursor.depth == 1 {
        return Some(tree.sections.get(cursor.section)?.items.len());
    }
    let mut parent = tree.sections.get(cursor.section)?.items.get(cursor.path[0])?;
    for level in 1..cursor.depth - 1 {
        parent = parent.items.as_ref()?.get(cursor.path[level])?;
    }
    parent.items.as_ref().map(Vec::len)
}

fn retained_control_text(control: &UiControlNode) -> Option<&str> {
    match control {
        UiControlNode::Input(node) => Some(if node.value.is_empty() { node.placeholder.as_ref().map(Label::as_str).unwrap_or("") } else { node.value.as_str() }),
        UiControlNode::Select(node) => node.placeholder.as_ref().map(Label::as_str),
        UiControlNode::Toggle(node) => node.text.as_ref().map(Label::as_str),
        UiControlNode::Button(node) => Some(node.label.as_str()),
        UiControlNode::KeyValue(node) => node.entries.first().map(|entry| entry.value.as_str()),
        UiControlNode::Slider(node) => node.unit.as_deref(),
        UiControlNode::NumberStepper(_) | UiControlNode::Ring(_) => None,
        UiControlNode::IconSelect(node) => Some(node.value.as_str()),
    }
}

fn retained_control_is_bounded(control: &UiControlNode) -> bool {
    match control {
        UiControlNode::Select(node) => node.items.len() <= RETAINED_NODE_COLLECTION_ITEMS,
        UiControlNode::KeyValue(node) => node.entries.len() <= RETAINED_NODE_COLLECTION_ITEMS,
        UiControlNode::Input(_) | UiControlNode::Toggle(_) | UiControlNode::Button(_) | UiControlNode::Slider(_) | UiControlNode::NumberStepper(_) | UiControlNode::Ring(_) | UiControlNode::IconSelect(_) => true,
    }
}

fn retained_tree_node_step(tree: &UiTreeNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList, cursor: &mut RetainedNodePaintCursor) -> RetainedNodePaintStep {
    if tree.sections.len() > RETAINED_NODE_COLLECTION_ITEMS {
        return RetainedNodePaintStep::Fault;
    }
    match cursor.phase {
        0 => {
            let result = retained_fixed_output(draw, |draw| draw.push_scissor(bounds));
            cursor.row_y = bounds.y;
            cursor.advance(1);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        1 => {
            let Some(section) = tree.sections.get(cursor.section) else {
                cursor.advance(10);
                return RetainedNodePaintStep::Pending;
            };
            if section.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            if !section.presence.visible() {
                cursor.section += 1;
                cursor.advance(1);
                return RetainedNodePaintStep::Pending;
            }
            if !cursor.chrome {
                cursor.chrome = true;
                let Some(label) = section.label.as_ref() else { return RetainedNodePaintStep::Pending };
                let color = if section.default_open.unwrap_or(true) { theme.text_element } else { theme.text_muted };
                let result = retained_fixed_output(draw, |draw| {
                    if let Some(icons) = icons {
                        push_icon(draw, icons, "folder", bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, cursor.row_y + (PANEL_HEADER - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, color);
                    }
                });
                let _ = label;
                return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
            }
            let Some(label) = section.label.as_ref() else {
                cursor.advance(2);
                return RetainedNodePaintStep::Pending;
            };
            let color = if section.default_open.unwrap_or(true) { theme.text_element } else { theme.text_muted };
            let label_bounds = Rect::new(bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard + TREE_ICON_SIZE + theme.gap_standard, cursor.row_y, bounds.w, PANEL_HEADER);
            match retained_text_node_step(label.as_str(), label_bounds, theme.font_size_small, color, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.row_y += PANEL_HEADER;
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        2 => {
            let Some(section) = tree.sections.get(cursor.section) else { return RetainedNodePaintStep::Fault };
            if section.items.is_empty() {
                cursor.section += 1;
                cursor.advance(1);
            } else {
                cursor.depth = 1;
                cursor.path[0] = 0;
                cursor.advance(3);
            }
            RetainedNodePaintStep::Pending
        }
        3 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            if item.items.as_ref().is_some_and(|items| items.len() > RETAINED_NODE_COLLECTION_ITEMS)
                || item.actions.as_ref().is_some_and(|actions| actions.len() > RETAINED_NODE_COLLECTION_ITEMS)
                || item.control.as_ref().is_some_and(|control| !retained_control_is_bounded(control))
            {
                return RetainedNodePaintStep::Fault;
            }
            let Some(visited) = cursor.visited.checked_add(1) else { return RetainedNodePaintStep::Fault };
            if visited > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            cursor.visited = visited;
            if !item.presence.visible() {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            }
            let row = Rect::new(bounds.x, cursor.row_y, bounds.w, TREE_ROW_HEIGHT);
            let selected = item.presence.selected;
            let previewed = item.presence.state == UiState::Previewed;
            let result = retained_fixed_output(draw, |draw| {
                if selected {
                    draw.push_rounded([row.x, row.y, row.w, row.h], theme.selected, theme.border_radius);
                } else if previewed {
                    draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
                }
                let ring_color = if selected { theme.selected } else { theme.border_normal };
                match item.presence.status {
                    UiStatus::Loading => paint_loading_border(draw, row, ring_color, theme),
                    UiStatus::Waiting => paint_waiting_border(draw, row, ring_color, theme),
                    UiStatus::Finished => draw.push_finished_border([row.x, row.y, row.w, row.h], ring_color, theme.border_radius, theme.stroke_hairline),
                    UiStatus::Idle => {}
                }
                if matches!(item.presence.state, UiState::Introducing | UiState::Celebrating) {
                    draw.push_introducing_border([row.x, row.y, row.w, row.h], theme.accent, theme.border_radius, theme.stroke_hairline);
                }
                let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
                if item.items.as_ref().is_some_and(|items| !items.is_empty()) {
                    if let Some(icons) = icons {
                        let chevron = if item.default_open.unwrap_or(false) { "chevron-down" } else { "chevron-right" };
                        push_icon(draw, icons, chevron, indent - TREE_TOGGLE_WIDTH, row.y + (TREE_ROW_HEIGHT - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
                    }
                }
                if let (Some(icons), Some(icon_id)) = (icons, item.icon_id) {
                    let color = if selected || previewed { theme.active_foreground } else { theme.text_element };
                    push_icon(draw, icons, icon_id.as_str(), indent, row.y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, color);
                }
            });
            cursor.advance(4);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        4 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            if !item.presence.visible() {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            }
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
            let selected = item.presence.selected;
            let previewed = item.presence.state == UiState::Previewed;
            let color = if selected || previewed { theme.active_foreground } else { theme.text_element };
            let color = if item.dimmed.unwrap_or(false) || item.presence.state == UiState::Disabled { color.with_alpha(color.a * 0.5) } else { color };
            match retained_text_node_step(item.label.as_str(), Rect::new(label_x, cursor.row_y, bounds.w, TREE_ROW_HEIGHT), theme.font_size_body, color, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(5);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        5 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            let Some(description) = item.description.as_deref() else {
                cursor.advance(6);
                return RetainedNodePaintStep::Pending;
            };
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let offset = item.label.as_str().len().min(RETAINED_NODE_COLLECTION_ITEMS) as f32 * theme.font_size_body * 0.5;
            match retained_text_node_step(description, Rect::new(indent + TREE_ICON_SIZE + theme.gap_standard + offset, cursor.row_y, bounds.w, TREE_ROW_HEIGHT), theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(6);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        6 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            let actions = item.actions.as_deref().unwrap_or(&[]);
            if cursor.item >= actions.len() {
                cursor.advance(7);
                return RetainedNodePaintStep::Pending;
            }
            let action = &actions[actions.len() - cursor.item - 1];
            cursor.item += 1;
            if action.placement() == UiTreeActionPlacement::Menu {
                return RetainedNodePaintStep::Pending;
            }
            let x = bounds.x + bounds.w - theme.gap_standard - cursor.item as f32 * (TREE_ICON_SIZE + theme.padding_standard);
            let result = retained_fixed_output(draw, |draw| {
                if let Some(icons) = icons {
                    push_icon(draw, icons, action.icon_id.as_str(), x, cursor.row_y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, theme.text_element);
                }
            });
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        7 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            let Some(control) = item.control.as_ref() else {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            };
            let control_rect = Rect::new(bounds.x + bounds.w - 120.0 - theme.gap_standard, cursor.row_y + (TREE_ROW_HEIGHT - theme.control_height) * 0.5, 120.0, theme.control_height);
            if !cursor.chrome {
                let result = retained_fixed_output(draw, |draw| push_control_border(draw, control_rect, theme, theme.border_normal, theme.input_bg));
                cursor.chrome = true;
                return if result.is_err() { RetainedNodePaintStep::Fault } else { RetainedNodePaintStep::Pending };
            }
            if let UiControlNode::Select(select) = control {
                if cursor.selected.is_none() && cursor.item < select.items.len() {
                    if select.items[cursor.item].value == select.value {
                        cursor.selected = Some(cursor.item);
                    }
                    cursor.item += 1;
                    return RetainedNodePaintStep::Pending;
                }
            }
            let value = match control {
                UiControlNode::Select(select) => cursor.selected.and_then(|index| select.items.get(index)).map(|item| item.label.as_str()).or_else(|| select.placeholder.as_ref().map(Label::as_str)),
                control => retained_control_text(control),
            };
            let Some(value) = value else {
                cursor.advance(8);
                return RetainedNodePaintStep::Pending;
            };
            match retained_text_node_step(value, control_rect, theme.font_size_small, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(8);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            }
        }
        8 => {
            let Some(item) = retained_tree_item_at(tree, cursor) else { return RetainedNodePaintStep::Fault };
            if item.presence.visible() {
                cursor.row_y += TREE_ROW_HEIGHT;
            }
            cursor.selected = None;
            if item.default_open.unwrap_or(false) && item.items.as_ref().is_some_and(|items| !items.is_empty()) {
                if cursor.depth >= RETAINED_TREE_DEPTH {
                    return RetainedNodePaintStep::Fault;
                }
                cursor.path[cursor.depth] = 0;
                cursor.depth += 1;
                cursor.advance(3);
            } else {
                cursor.ascending = true;
                cursor.advance(9);
            }
            RetainedNodePaintStep::Pending
        }
        9 => {
            if !cursor.ascending || cursor.depth == 0 {
                return RetainedNodePaintStep::Fault;
            }
            let Some(siblings) = retained_tree_sibling_count(tree, cursor) else { return RetainedNodePaintStep::Fault };
            let level = cursor.depth - 1;
            if cursor.path[level] + 1 < siblings {
                cursor.path[level] += 1;
                cursor.advance(3);
            } else {
                cursor.path[level] = 0;
                cursor.depth -= 1;
                if cursor.depth == 0 {
                    cursor.section += 1;
                    cursor.advance(1);
                }
            }
            RetainedNodePaintStep::Pending
        }
        10 => {
            let result = retained_fixed_output(draw, DrawList::pop_scissor);
            cursor.advance(11);
            if result.is_err() {
                RetainedNodePaintStep::Fault
            } else {
                RetainedNodePaintStep::Pending
            }
        }
        _ => retained_presence_step(draw, bounds, theme, &tree.presence),
    }
}

/// 🎨️ Advances one exact glyph or one independently pre-admitted bounded widget child.
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
pub(crate) fn paint_node_step(
    tree: &UiTree,
    id: NodeId,
    origin_x: f32,
    origin_y: f32,
    theme: &Theme,
    atlas: &mut FontAtlas,
    icons: Option<&IconAtlas>,
    has_scene_host: bool,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    if cursor.node.is_none() {
        cursor.begin(id, origin_x, origin_y);
        return RetainedNodePaintStep::Pending;
    }
    if cursor.node != Some(id) || cursor.origin_x != origin_x || cursor.origin_y != origin_y {
        return RetainedNodePaintStep::Fault;
    }
    let Some(node) = tree.node(id) else { return RetainedNodePaintStep::Fault };
    let Some(layout) = tree.accepted_layout(id) else { return RetainedNodePaintStep::Fault };
    let bounds = Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height);
    let presence = node.spec.0.presence();
    if !presence.visible() {
        return cursor.finish();
    }
    let mut flags = node.flags;
    if presence.state != UiState::Disabled {
        flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
    }
    let step = match &node.spec.0 {
        UiNode::Text(text) => {
            if cursor.phase == 0 {
                let emphasize = text.emphasize.unwrap_or(false);
                let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
                let color = if emphasize { theme.text } else { theme.text_muted };
                match retained_text_node_step(text.value.as_str(), bounds, size, color, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(1);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Stack(stack) => {
            if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| paint_stack_frame(stack, bounds, flags, theme, draw));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Separator(_) => {
            if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| paint_separator(bounds, theme, draw));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Button(button) => match cursor.phase {
            0 => {
                let hovered = flags.contains(NodeFlags::HOVERED);
                let result = retained_fixed_output(draw, |draw| {
                    push_control_border(draw, bounds, theme, if flags.contains(NodeFlags::FOCUSED) { theme.border_emphasized } else { theme.border_normal }, item_bg(theme, false, hovered));
                    if let Some(icons) = icons {
                        push_icon(draw, icons, button.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
                    }
                });
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            }
            1 => match retained_text_node_step(button.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            },
            _ => retained_presence_step(draw, bounds, theme, presence),
        },
        UiNode::Input(input_node) => {
            let display = node.state.edit.as_ref().map(|edit| edit.text.as_str()).filter(|text| !text.is_empty()).unwrap_or_else(|| {
                if input_node.value.is_empty() {
                    input_node.placeholder.as_ref().map(Label::as_str).unwrap_or("")
                } else {
                    input_node.value.as_str()
                }
            });
            match cursor.phase {
                0 => {
                    let result = retained_fixed_output(draw, |draw| {
                        push_control_border(draw, bounds, theme, if flags.contains(NodeFlags::FOCUSED) { theme.border_emphasized } else { theme.border_normal }, theme.input_bg);
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                1 => match retained_text_node_step(display, bounds, theme.font_size_body, if input_node.value.is_empty() { theme.text_muted } else { theme.text }, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                },
                _ => retained_presence_step(draw, bounds, theme, presence),
            }
        }
        UiNode::Select(select) => {
            if select.items.len() > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            match cursor.phase {
                0 => {
                    let hovered = flags.contains(NodeFlags::HOVERED);
                    let result = retained_fixed_output(draw, |draw| {
                        push_control_border(draw, bounds, theme, if flags.contains(NodeFlags::FOCUSED) { theme.border_emphasized } else { theme.border_normal }, if hovered { theme.button_hover } else { theme.input_bg });
                        if let Some(icons) = icons {
                            push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
                        }
                    });
                    cursor.advance(1);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                1 if cursor.item < select.items.len() => {
                    if select.items[cursor.item].value == select.value {
                        cursor.selected = Some(cursor.item);
                    }
                    cursor.item += 1;
                    RetainedNodePaintStep::Pending
                }
                1 => {
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                2 => {
                    let label = cursor.selected.and_then(|index| select.items.get(index)).map(|item| item.label.as_str()).or_else(|| select.placeholder.as_ref().map(Label::as_str)).unwrap_or("Select…");
                    match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.advance(if node.state.open { 3 } else { 6 });
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                }
                3 => {
                    let menu = Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, select.items.len() as f32 * theme.control_height + 4.0);
                    let result = retained_fixed_output(draw, |draw| {
                        draw.push_glass([menu.x, menu.y, menu.w, menu.h], theme.border_radius, theme.glass(Level::Menu));
                    });
                    cursor.advance(4);
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                4 if cursor.item < select.items.len() => {
                    let relative = select_popup_row_rect(bounds.w, bounds.h, cursor.item, theme);
                    let row = Rect::new(bounds.x + relative.x, bounds.y + relative.y, relative.w, relative.h);
                    let item = &select.items[cursor.item];
                    let result = retained_fixed_output(draw, |draw| {
                        if item.value == select.value {
                            draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
                        }
                    });
                    cursor.phase = 5;
                    cursor.glyph.reset();
                    if result.is_err() {
                        RetainedNodePaintStep::Fault
                    } else {
                        RetainedNodePaintStep::Pending
                    }
                }
                4 => {
                    cursor.advance(6);
                    RetainedNodePaintStep::Pending
                }
                5 => {
                    let Some(item) = select.items.get(cursor.item) else { return RetainedNodePaintStep::Fault };
                    let relative = select_popup_row_rect(bounds.w, bounds.h, cursor.item, theme);
                    let row = Rect::new(bounds.x + relative.x + 8.0, bounds.y + relative.y, relative.w - 8.0, relative.h);
                    match retained_text_node_step(item.label.as_str(), row, theme.font_size_body, theme.text, atlas, draw, cursor) {
                        RetainedNodePaintStep::Complete => {
                            cursor.item += 1;
                            cursor.phase = 4;
                            cursor.glyph.reset();
                            RetainedNodePaintStep::Pending
                        }
                        step => step,
                    }
                }
                _ => retained_presence_step(draw, bounds, theme, presence),
            }
        }
        UiNode::Toggle(toggle) => match cursor.phase {
            0 => {
                let pressed = toggle.presence.selected;
                let hovered = flags.contains(NodeFlags::HOVERED);
                let result = retained_fixed_output(draw, |draw| {
                    push_control_border(draw, bounds, theme, if flags.contains(NodeFlags::FOCUSED) { theme.border_emphasized } else { theme.border_normal }, item_bg(theme, pressed, hovered));
                    if let Some(icons) = icons {
                        push_icon(draw, icons, toggle.icon_id.as_str(), bounds.x + theme.padding_standard, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
                    }
                });
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            }
            1 => {
                let label = toggle.text.as_ref().map(Label::as_str).unwrap_or("");
                match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            }
            _ => retained_presence_step(draw, bounds, theme, presence),
        },
        UiNode::KeyValue(key_value) => {
            if key_value.entries.len() > RETAINED_NODE_COLLECTION_ITEMS {
                return RetainedNodePaintStep::Fault;
            }
            let Some(entry) = key_value.entries.get(cursor.item) else {
                cursor.advance(2);
                return match retained_presence_step(draw, bounds, theme, presence) {
                    RetainedNodePaintStep::Complete => cursor.finish(),
                    step => step,
                };
            };
            let row = Rect::new(bounds.x, bounds.y + cursor.item as f32 * theme.control_height, bounds.w, theme.control_height);
            if cursor.phase == 0 {
                match retained_text_node_step(entry.label.as_str(), row, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.phase = 1;
                        cursor.glyph.reset();
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                let value_bounds = Rect::new(row.x + row.w * 0.4, row.y, row.w * 0.6, row.h);
                match retained_text_node_step(entry.value.as_str(), value_bounds, theme.font_size_small, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.item += 1;
                        cursor.phase = 0;
                        cursor.glyph.reset();
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            }
        }
        UiNode::Slider(slider) => match cursor.phase {
            0 => {
                let track_y = bounds.y + bounds.h * 0.5;
                let range = (slider.max - slider.min).max(f64::EPSILON);
                let knob_x = bounds.x + bounds.w * ((slider.value - slider.min) / range).clamp(0.0, 1.0) as f32;
                let result = retained_fixed_output(draw, |draw| {
                    draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], theme.separator, 2.0);
                    draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], theme.accent, 6.0);
                });
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            }
            1 => {
                let label = slider.unit.as_deref().unwrap_or("");
                match retained_text_node_step(label, bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            }
            _ => retained_presence_step(draw, bounds, theme, presence),
        },
        UiNode::NumberStepper(stepper) => match cursor.phase {
            0 => {
                let result = retained_fixed_output(draw, |draw| push_control_border(draw, bounds, theme, theme.border_normal, theme.input_bg));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            }
            1 => {
                let label = if stepper.uniform { format!("{:.3}", stepper.value) } else { UI_INSPECTOR_MIXED_PLACEHOLDER.to_string() };
                match retained_text_node_step(&label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            }
            _ => retained_presence_step(draw, bounds, theme, presence),
        },
        UiNode::Ring(ring) => {
            let segments = 48usize;
            if cursor.item < segments {
                let cx = bounds.x + bounds.w * 0.5;
                let cy = bounds.y + bounds.h * 0.5;
                let radius = bounds.w.min(bounds.h) * 0.4;
                let a0 = std::f32::consts::TAU * cursor.item as f32 / segments as f32;
                let a1 = std::f32::consts::TAU * (cursor.item + 1) as f32 / segments as f32;
                let result = retained_fixed_output(draw, |draw| draw.push_line(cx + a0.cos() * radius, cy + a0.sin() * radius, cx + a1.cos() * radius, cy + a1.sin() * radius, theme.separator, 2.0));
                cursor.item += 1;
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| {
                    let cx = bounds.x + bounds.w * 0.5;
                    let cy = bounds.y + bounds.h * 0.5;
                    let radius = bounds.w.min(bounds.h) * 0.4;
                    let angle = std::f32::consts::TAU * ring.t as f32;
                    draw.push_rounded([cx + angle.cos() * radius - 6.0, cy + angle.sin() * radius - 6.0, 12.0, 12.0], theme.accent, 6.0);
                });
                cursor.phase = 1;
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::IconSelect(select) => match cursor.phase {
            0 => {
                let result = retained_fixed_output(draw, |draw| push_control_border(draw, bounds, theme, theme.border_normal, theme.input_bg));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            }
            1 => match retained_text_node_step(select.value.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                RetainedNodePaintStep::Complete => {
                    cursor.advance(2);
                    RetainedNodePaintStep::Pending
                }
                step => step,
            },
            _ => retained_presence_step(draw, bounds, theme, presence),
        },
        UiNode::Field(field) => {
            let value = match cursor.phase {
                0 => Some((field.label.as_str(), theme.text_muted)),
                1 => field.description.as_deref().map(|value| (value, theme.text_muted)),
                2 => field.error.as_deref().map(|value| (value, theme.error)),
                _ => None,
            };
            if let Some((value, color)) = value {
                match retained_text_node_step(value, bounds, theme.font_size_small, color, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(cursor.phase + 1);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else if cursor.phase < 3 {
                cursor.advance(cursor.phase + 1);
                RetainedNodePaintStep::Pending
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Section(section) => {
            if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| {
                    if let Some(icons) = icons {
                        push_icon(draw, icons, if section.default_open.unwrap_or(true) { "chevron-down" } else { "chevron-right" }, bounds.x, bounds.y, ICON_TINY, theme.text_element);
                    }
                });
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 1 {
                let label = section.label.as_ref().map(Label::as_str).unwrap_or("");
                match retained_text_node_step(label, bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Group(group) => {
            if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| {
                    if let Some(icons) = icons {
                        push_icon(draw, icons, if group.default_open.unwrap_or(true) { "chevron-down" } else { "chevron-right" }, bounds.x, bounds.y, ICON_TINY, theme.text_element);
                    }
                });
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 1 {
                match retained_text_node_step(group.label.as_str(), bounds, theme.font_size_body, theme.text, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::Tree(tree_node) => retained_tree_node_step(tree_node, bounds, theme, atlas, icons, draw, cursor),
        UiNode::Image(image) => {
            if has_scene_host {
                retained_presence_step(draw, bounds, theme, presence)
            } else if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 1 {
                let label = image.alt.as_ref().map(Label::as_str).unwrap_or(image.id.as_str());
                match retained_text_node_step(label, bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::ComponentScene(scene) => {
            if has_scene_host {
                retained_presence_step(draw, bounds, theme, presence)
            } else if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 1 {
                match retained_text_node_step(scene.surface_id.as_str(), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
        UiNode::ExternalSlot(slot) => {
            if cursor.phase == 0 {
                let result = retained_fixed_output(draw, |draw| draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel, theme.border_radius));
                cursor.advance(1);
                if result.is_err() {
                    RetainedNodePaintStep::Fault
                } else {
                    RetainedNodePaintStep::Pending
                }
            } else if cursor.phase == 1 {
                match retained_text_node_step(slot.body_key.as_str(), bounds, theme.font_size_small, theme.text_muted, atlas, draw, cursor) {
                    RetainedNodePaintStep::Complete => {
                        cursor.advance(2);
                        RetainedNodePaintStep::Pending
                    }
                    step => step,
                }
            } else {
                retained_presence_step(draw, bounds, theme, presence)
            }
        }
    };
    match step {
        RetainedNodePaintStep::Complete => cursor.finish(),
        step => step,
    }
}

/// 🖼️ Top-level entry point: unconditionally walks and (re)paints every node reachable from `root`,
/// clearing `DIRTY_PAINT` as it visits (mirroring `flex::LayoutEngine::write_back`'s clear-as-you-go
/// pattern) but never touching `DIRTY_LAYOUT`/`SUBTREE_DIRTY` — clearing those is `flex`'s job and
/// `flex::LayoutEngine::compute` already runs (and clears them) before paint each frame, per the
/// intended pipeline. Deliberately has **no internal early-out**: `DrawList` only supports a full
/// clear-and-rebuild (no API to remove/replace a single dirty subtree's prior draw calls while
/// leaving clean siblings' draw calls in place), so a genuinely incremental repaint isn't safe to
/// build yet. Whether to call `paint_tree` at all this frame — i.e. "was anything dirty" — is a
/// decision a later milestone's `engine` facade owns (it already knows from driving `flex::compute`
/// and `reconcile::apply_tree`), not something `paint_tree` decides for itself.
/// 🎬️ `has_scene_host` gates the `ComponentScene`/`Image` leaf arms below (see `paint_node`'s own
/// match): when the caller's `engine::Ui::frame` has a real `scene_slots::SceneHost` for this tick,
/// those leaves paint NOTHING here — the host paints the real content into the same rect right after
/// this call, in `Ui::frame`'s `collect_scene_slots` loop — instead of this pass drawing placeholder
/// chrome that the host would then have to paint over. With no host (`false`), behavior is unchanged
/// from before this parameter existed: `paint_component_scene`/`paint_image`'s own placeholder chrome.
pub(crate) fn paint_tree(tree: &mut UiTree, root: NodeId, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    sync_interactive_state(tree, root, theme);
    paint_node(tree, root, 0.0, 0.0, theme, atlas, icons, has_scene_host, draw);
    clear_dirty_paint(tree, root);
}

fn clear_dirty_paint(tree: &mut UiTree, id: NodeId) {
    if let Some(node) = tree.node_mut(id) {
        node.flags.set(NodeFlags::DIRTY_PAINT, false);
    }
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        clear_dirty_paint(tree, child);
    }
}

//#region 🔖️InteractiveStateSync
// 🔗️ W2 wiring: a paint-owned pre-pass, mutable (unlike `paint_node`'s own read-only walk below),
// run once per `paint_tree` call before painting anything — writes derived state `flex`/`reconcile`
// have no way to produce for composite widgets they don't fully own the interactive geometry of:
//  - an open `Select`'s synthesized item-row `Button`s (`reconcile::children_of`'s `Select` arm)
//    get real per-row `LayoutBucket` rects here (`flex::style_for`'s fallback leaf style gives every
//    one of them a zero-size rect — neither `Select` nor its rows are a flex container `flex` grants
//    space to), computed with the exact geometry `paint_select` itself paints the popup at (see
//    `select_popup_row_rect`), so `events::hit_test` can actually find and click them.
//  - a `Stack`'s `NodeFlags::DROP_TARGET` bit is kept in sync with its own `drop_action` field
//    (`events::nearest_accepting_drop_target` walks the bubble chain for this flag).
//  - a `Tree`'s synthesized per-row `Stack`s (`reconcile::children_of`'s `Tree` arm) get real
//    per-row rects too (same zero-size root cause as `Select`'s rows), computed with the exact
//    row-height/indent math `paint_tree_item` paints rows at, and their `NodeFlags::DRAG_SOURCE` bit
//    is kept in sync with the *original* `UiTreeItemNode`'s `draggable` field (`reconcile` never
//    drops fields, only clones them into `WidgetSpec` — see that module's own doc comment).

const RETAINED_SYNC_COLLECTION_ITEMS: usize = 256;
const RETAINED_SYNC_OUTPUTS: usize = RETAINED_SYNC_COLLECTION_ITEMS * 2;
const RETAINED_SYNC_DEPTH: usize = 64;
const RETAINED_SYNC_KEY_BYTES: usize = 256;

#[derive(Clone, Copy)]
struct RetainedSyncKey {
    bytes: [u8; RETAINED_SYNC_KEY_BYTES],
    len: u16,
}

impl RetainedSyncKey {
    fn try_from_str(value: &str) -> Option<Self> {
        if value.len() > RETAINED_SYNC_KEY_BYTES {
            return None;
        }
        let mut bytes = [0; RETAINED_SYNC_KEY_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

#[derive(Clone, Copy)]
struct RetainedSyncTreeRecord {
    key: RetainedSyncKey,
    parent: Option<usize>,
    retained: Option<NodeId>,
    y: f32,
    height: f32,
    draggable: bool,
    section: bool,
}

#[derive(Clone, Copy)]
struct RetainedSyncTreeFrame {
    record: usize,
    next_item: usize,
    items_pointer: usize,
    items_len: usize,
    height: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RetainedInteractiveSyncPhase {
    Bind,
    SelectItem,
    SelectScan,
    SelectWrite,
    StackWrite,
    TreeSection,
    TreeItem,
    TreeApplyPrepare,
    TreeApplyScan,
    TreeApplyWrite,
    TreeClose,
}

/// 🧭️ Retains one mounted interaction-layout synchronization authority.
pub(crate) struct RetainedInteractiveSyncCursor {
    node: Option<NodeId>,
    phase: RetainedInteractiveSyncPhase,
    item: usize,
    child_scan: Option<NodeId>,
    matched: Option<NodeId>,
    select_width: f32,
    select_height: f32,
    tree_section: usize,
    tree_section_y: f32,
    tree_depth: usize,
    tree_frames: [Option<RetainedSyncTreeFrame>; RETAINED_SYNC_DEPTH],
    tree_records: [Option<RetainedSyncTreeRecord>; RETAINED_SYNC_OUTPUTS],
    tree_record_len: usize,
    tree_item_count: usize,
    tree_apply: usize,
}

impl Default for RetainedInteractiveSyncCursor {
    fn default() -> Self {
        Self {
            node: None,
            phase: RetainedInteractiveSyncPhase::Bind,
            item: 0,
            child_scan: None,
            matched: None,
            select_width: 0.0,
            select_height: 0.0,
            tree_section: 0,
            tree_section_y: 0.0,
            tree_depth: 0,
            tree_frames: [None; RETAINED_SYNC_DEPTH],
            tree_records: [None; RETAINED_SYNC_OUTPUTS],
            tree_record_len: 0,
            tree_item_count: 0,
            tree_apply: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RetainedInteractiveSyncStep {
    Pending,
    Complete,
    Fault,
}

impl RetainedInteractiveSyncCursor {
    fn finish(&mut self) -> RetainedInteractiveSyncStep {
        self.node = None;
        self.phase = RetainedInteractiveSyncPhase::Bind;
        self.item = 0;
        self.child_scan = None;
        self.matched = None;
        self.select_width = 0.0;
        self.select_height = 0.0;
        self.tree_section = 0;
        self.tree_section_y = 0.0;
        self.tree_depth = 0;
        self.tree_item_count = 0;
        self.tree_apply = 0;
        RetainedInteractiveSyncStep::Complete
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.tree_record_len > 0 {
            self.tree_record_len -= 1;
            self.tree_records[self.tree_record_len] = None;
            return false;
        }
        if self.tree_frames[self.tree_depth].take().is_some() {
            if self.tree_depth > 0 {
                self.tree_depth -= 1;
            }
            return false;
        }
        if self.node.take().is_some() {
            self.phase = RetainedInteractiveSyncPhase::Bind;
            self.child_scan = None;
            self.matched = None;
            return false;
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.node.is_none() && self.tree_record_len == 0 && self.tree_frames[0].is_none()
    }
}

fn retained_sync_tree_item_step(cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    let Some(frame) = cursor.tree_frames[cursor.tree_depth] else { return RetainedInteractiveSyncStep::Fault };
    if frame.next_item >= frame.items_len {
        let complete = frame.height;
        let record = frame.record;
        let Some(output) = cursor.tree_records[record].as_mut() else { return RetainedInteractiveSyncStep::Fault };
        output.height = complete;
        cursor.tree_frames[cursor.tree_depth] = None;
        if cursor.tree_depth == 0 {
            cursor.tree_section_y += complete;
            cursor.tree_section += 1;
            cursor.phase = RetainedInteractiveSyncPhase::TreeSection;
        } else {
            cursor.tree_depth -= 1;
            let Some(parent) = cursor.tree_frames[cursor.tree_depth].as_mut() else { return RetainedInteractiveSyncStep::Fault };
            parent.height += complete;
        }
        return RetainedInteractiveSyncStep::Pending;
    }
    let item_index = frame.next_item;
    let Some(next_item) = item_index.checked_add(1) else { return RetainedInteractiveSyncStep::Fault };
    let Some(active) = cursor.tree_frames[cursor.tree_depth].as_mut() else { return RetainedInteractiveSyncStep::Fault };
    active.next_item = next_item;
    let Some(item) = (unsafe { (frame.items_pointer as *const UiTreeItemNode).add(item_index).as_ref() }) else { return RetainedInteractiveSyncStep::Fault };
    if !item.presence.visible() {
        return RetainedInteractiveSyncStep::Pending;
    }
    let Some(item_count) = cursor.tree_item_count.checked_add(1).filter(|count| *count <= RETAINED_SYNC_COLLECTION_ITEMS) else { return RetainedInteractiveSyncStep::Fault };
    let Some(record_index) = cursor.tree_record_len.checked_add(1).filter(|count| *count <= RETAINED_SYNC_OUTPUTS).map(|_| cursor.tree_record_len) else { return RetainedInteractiveSyncStep::Fault };
    let Some(key) = RetainedSyncKey::try_from_str(&item.id) else { return RetainedInteractiveSyncStep::Fault };
    cursor.tree_records[record_index] = Some(RetainedSyncTreeRecord { key, parent: Some(frame.record), retained: None, y: frame.height, height: TREE_ROW_HEIGHT, draggable: item.draggable.unwrap_or(false), section: false });
    cursor.tree_record_len += 1;
    cursor.tree_item_count = item_count;
    let children = item.items.as_deref().filter(|items| item.default_open.unwrap_or(false) && !items.is_empty());
    if let Some(children) = children {
        let Some(depth) = cursor.tree_depth.checked_add(1).filter(|depth| *depth < RETAINED_SYNC_DEPTH) else { return RetainedInteractiveSyncStep::Fault };
        cursor.tree_depth = depth;
        cursor.tree_frames[depth] = Some(RetainedSyncTreeFrame { record: record_index, next_item: 0, items_pointer: children.as_ptr() as usize, items_len: children.len(), height: TREE_ROW_HEIGHT });
    } else {
        let Some(parent) = cursor.tree_frames[cursor.tree_depth].as_mut() else { return RetainedInteractiveSyncStep::Fault };
        parent.height += TREE_ROW_HEIGHT;
    }
    RetainedInteractiveSyncStep::Pending
}

/// 🧩️ Advances one retained synchronization item, child lookup, layout output, or close owner.
pub(crate) fn sync_interactive_state_node_step(tree: &mut UiTree, id: NodeId, theme: &Theme, cursor: &mut RetainedInteractiveSyncCursor) -> RetainedInteractiveSyncStep {
    if cursor.node.is_none() {
        cursor.node = Some(id);
        cursor.phase = RetainedInteractiveSyncPhase::Bind;
        return RetainedInteractiveSyncStep::Pending;
    }
    if cursor.node != Some(id) {
        return RetainedInteractiveSyncStep::Fault;
    }
    match cursor.phase {
        RetainedInteractiveSyncPhase::Bind => {
            let Some(node) = tree.node(id) else { return RetainedInteractiveSyncStep::Fault };
            match &node.spec.0 {
                UiNode::Select(select) if node.state.open => {
                    if select.items.len() > RETAINED_SYNC_COLLECTION_ITEMS {
                        return RetainedInteractiveSyncStep::Fault;
                    }
                    let Some(layout) = tree.accepted_layout(id) else { return RetainedInteractiveSyncStep::Fault };
                    cursor.select_width = layout.width;
                    cursor.select_height = layout.height;
                    cursor.phase = RetainedInteractiveSyncPhase::SelectItem;
                    RetainedInteractiveSyncStep::Pending
                }
                UiNode::Stack(_) => {
                    cursor.phase = RetainedInteractiveSyncPhase::StackWrite;
                    RetainedInteractiveSyncStep::Pending
                }
                UiNode::Tree(tree_node) => {
                    if tree_node.sections.len() > RETAINED_SYNC_COLLECTION_ITEMS || tree.accepted_layout(id).is_none() {
                        return RetainedInteractiveSyncStep::Fault;
                    }
                    cursor.phase = RetainedInteractiveSyncPhase::TreeSection;
                    RetainedInteractiveSyncStep::Pending
                }
                _ => cursor.finish(),
            }
        }
        RetainedInteractiveSyncPhase::SelectItem => {
            let Some(select) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => Some(select),
                _ => None,
            }) else {
                return RetainedInteractiveSyncStep::Fault;
            };
            if cursor.item >= select.items.len() {
                return cursor.finish();
            }
            cursor.child_scan = tree.node(id).and_then(|node| node.first_child);
            cursor.matched = None;
            cursor.phase = RetainedInteractiveSyncPhase::SelectScan;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectScan => {
            let Some(child) = cursor.child_scan else { return RetainedInteractiveSyncStep::Fault };
            let matches = tree
                .node(id)
                .and_then(|node| match &node.spec.0 {
                    UiNode::Select(select) => select.items.get(cursor.item),
                    _ => None,
                })
                .is_some_and(|item| matches!(tree.node(child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == &item.value));
            if matches {
                cursor.matched = Some(child);
                cursor.phase = RetainedInteractiveSyncPhase::SelectWrite;
            } else {
                cursor.child_scan = tree.node(child).and_then(|node| node.next_sibling);
            }
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::SelectWrite => {
            let Some(child) = cursor.matched.take() else { return RetainedInteractiveSyncStep::Fault };
            let rect = select_popup_row_rect(cursor.select_width, cursor.select_height, cursor.item, theme);
            let Some(node) = tree.node_mut(child) else { return RetainedInteractiveSyncStep::Fault };
            node.layout.x = rect.x;
            node.layout.y = rect.y;
            node.layout.width = rect.w;
            node.layout.height = rect.h;
            let Some(item) = cursor.item.checked_add(1) else { return RetainedInteractiveSyncStep::Fault };
            cursor.item = item;
            cursor.phase = RetainedInteractiveSyncPhase::SelectItem;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::StackWrite => {
            let accepts_drop = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Stack(stack) => Some(stack.drop_action.is_some()),
                _ => None,
            });
            let Some(accepts_drop) = accepts_drop else { return RetainedInteractiveSyncStep::Fault };
            let Some(node) = tree.node_mut(id) else { return RetainedInteractiveSyncStep::Fault };
            node.flags.set(NodeFlags::DROP_TARGET, accepts_drop);
            cursor.finish()
        }
        RetainedInteractiveSyncPhase::TreeSection => {
            let Some((sections_len, width)) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Tree(tree_node) => tree.accepted_layout(id).map(|layout| (tree_node.sections.len(), layout.width)),
                _ => None,
            }) else {
                return RetainedInteractiveSyncStep::Fault;
            };
            if cursor.tree_section >= sections_len {
                cursor.tree_apply = 0;
                cursor.phase = RetainedInteractiveSyncPhase::TreeApplyPrepare;
                return RetainedInteractiveSyncStep::Pending;
            }
            let Some(section) = tree.node(id).and_then(|node| match &node.spec.0 {
                UiNode::Tree(tree_node) => tree_node.sections.get(cursor.tree_section),
                _ => None,
            }) else {
                return RetainedInteractiveSyncStep::Fault;
            };
            let Some(record_index) = cursor.tree_record_len.checked_add(1).filter(|count| *count <= RETAINED_SYNC_OUTPUTS).map(|_| cursor.tree_record_len) else { return RetainedInteractiveSyncStep::Fault };
            let Some(key) = RetainedSyncKey::try_from_str(&section.id) else { return RetainedInteractiveSyncStep::Fault };
            let header = if section.label.is_some() { PANEL_HEADER } else { 0.0 };
            cursor.tree_records[record_index] = Some(RetainedSyncTreeRecord { key, parent: None, retained: None, y: cursor.tree_section_y, height: header, draggable: false, section: true });
            cursor.tree_record_len += 1;
            cursor.tree_depth = 0;
            cursor.tree_frames[0] = Some(RetainedSyncTreeFrame { record: record_index, next_item: 0, items_pointer: section.items.as_ptr() as usize, items_len: section.items.len(), height: header });
            cursor.select_width = width;
            cursor.phase = RetainedInteractiveSyncPhase::TreeItem;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeItem => retained_sync_tree_item_step(cursor),
        RetainedInteractiveSyncPhase::TreeApplyPrepare => {
            if cursor.tree_apply >= cursor.tree_record_len {
                cursor.phase = RetainedInteractiveSyncPhase::TreeClose;
                return RetainedInteractiveSyncStep::Pending;
            }
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return RetainedInteractiveSyncStep::Fault };
            let parent = match record.parent {
                Some(parent) => cursor.tree_records.get(parent).and_then(|record| record.as_ref()).and_then(|record| record.retained),
                None => Some(id),
            };
            let Some(parent) = parent else { return RetainedInteractiveSyncStep::Fault };
            cursor.child_scan = tree.node(parent).and_then(|node| node.first_child);
            cursor.matched = None;
            cursor.phase = RetainedInteractiveSyncPhase::TreeApplyScan;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeApplyScan => {
            let Some(child) = cursor.child_scan else { return RetainedInteractiveSyncStep::Fault };
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return RetainedInteractiveSyncStep::Fault };
            let Some(target) = record.key.as_str() else { return RetainedInteractiveSyncStep::Fault };
            if matches!(tree.node(child).map(|node| &node.key), Some(NodeKey::Explicit(key)) if key == target) {
                cursor.matched = Some(child);
                cursor.phase = RetainedInteractiveSyncPhase::TreeApplyWrite;
            } else {
                cursor.child_scan = tree.node(child).and_then(|node| node.next_sibling);
            }
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeApplyWrite => {
            let Some(child) = cursor.matched.take() else { return RetainedInteractiveSyncStep::Fault };
            let Some(record) = cursor.tree_records[cursor.tree_apply] else { return RetainedInteractiveSyncStep::Fault };
            let Some(node) = tree.node_mut(child) else { return RetainedInteractiveSyncStep::Fault };
            node.layout.x = 0.0;
            node.layout.y = record.y;
            node.layout.width = cursor.select_width;
            node.layout.height = record.height;
            if !record.section {
                node.flags.set(NodeFlags::DRAG_SOURCE, record.draggable);
            }
            let Some(output) = cursor.tree_records[cursor.tree_apply].as_mut() else { return RetainedInteractiveSyncStep::Fault };
            output.retained = Some(child);
            let Some(next) = cursor.tree_apply.checked_add(1) else { return RetainedInteractiveSyncStep::Fault };
            cursor.tree_apply = next;
            cursor.phase = RetainedInteractiveSyncPhase::TreeApplyPrepare;
            RetainedInteractiveSyncStep::Pending
        }
        RetainedInteractiveSyncPhase::TreeClose => {
            if cursor.tree_record_len > 0 {
                cursor.tree_record_len -= 1;
                cursor.tree_records[cursor.tree_record_len] = None;
                RetainedInteractiveSyncStep::Pending
            } else {
                cursor.finish()
            }
        }
    }
}

/// 🔎️ Finds `parent`'s retained child keyed `key` — `reconcile`'s synthesized `Select`/`Tree` rows
/// are keyed by stable identity (`item.value`/`section.id`/`item.id`, see `reconcile::children_of`),
/// so this is how this pass re-associates a declarative row (`UiSelectItem`/`UiTreeItemNode`) with
/// its already-existing retained `NodeId`, robust to reconcile's insertion-order quirks (a re-used
/// matched child physically keeps its old sibling-list position — see that module's own doc comment
/// on why key lookup, not positional indexing, is the safe way to do this).
fn find_child_by_key(tree: &UiTree, parent: NodeId, key: &NodeKey) -> Option<NodeId> {
    tree.children(parent).find(|&child| tree.node(child).map(|n| &n.key) == Some(key))
}

fn sync_interactive_state(tree: &mut UiTree, id: NodeId, theme: &Theme) {
    sync_interactive_state_node(tree, id, theme);
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        sync_interactive_state(tree, child, theme);
    }
}

pub(crate) fn sync_interactive_state_node(tree: &mut UiTree, id: NodeId, theme: &Theme) {
    let accepted = tree.accepted_layout(id);
    let select_open: Option<(Vec<UiSelectItem>, f32, f32)> = tree.node(id).and_then(|node| match &node.spec.0 {
        UiNode::Select(select) if node.state.open => accepted.map(|layout| (select.items.clone(), layout.width, layout.height)),
        _ => None,
    });
    if let Some((items, select_w, select_h)) = select_open {
        sync_select_popup_rows(tree, id, &items, select_w, select_h, theme);
    }

    let stack_drop_target: Option<bool> = tree.node(id).and_then(|node| match &node.spec.0 {
        UiNode::Stack(stack) => Some(stack.drop_action.is_some()),
        _ => None,
    });
    if let Some(accepts_drop) = stack_drop_target {
        if let Some(node) = tree.node_mut(id) {
            node.flags.set(NodeFlags::DROP_TARGET, accepts_drop);
        }
    }

    if tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::Tree(_))) {
        sync_tree_row_layout(tree, id);
    }
}

/// 📐️ One popup row's `(x, y, w, h)` **relative to the `Select`'s own top-left** — shared by
/// `sync_select_popup_rows` (writes it into the row's retained `LayoutBucket`) and `paint_select`
/// (paints it), so the two can never drift apart. Mirrors `widgets::render_select_menu`'s literal
/// geometry: the popup sits `select_h + 2.0` below the trigger, each row inset `2.0`,
/// `theme.control_height` tall.
fn select_popup_row_rect(select_w: f32, select_h: f32, index: usize, theme: &Theme) -> Rect {
    let item_h = theme.control_height;
    let menu_y = select_h + 2.0;
    Rect::new(2.0, menu_y + 2.0 + index as f32 * item_h, (select_w - 4.0).max(0.0), item_h)
}

fn sync_select_popup_rows(tree: &mut UiTree, select_id: NodeId, items: &[UiSelectItem], select_w: f32, select_h: f32, theme: &Theme) {
    for (index, item) in items.iter().enumerate() {
        let Some(row_id) = find_child_by_key(tree, select_id, &NodeKey::Explicit(item.value.clone())) else { continue };
        let rect = select_popup_row_rect(select_w, select_h, index, theme);
        if let Some(node) = tree.node_mut(row_id) {
            node.layout.x = rect.x;
            node.layout.y = rect.y;
            node.layout.width = rect.w;
            node.layout.height = rect.h;
        }
    }
}

/// 🌳️ Gives each of a `Tree`'s synthesized per-section `Stack`s (`reconcile::children_of`'s `Tree`
/// arm, keyed by `section.id`) real `LayoutBucket` geometry, cumulative down the tree exactly like
/// `paint_tree_widget`'s own procedural walk (header height, then each item's row height including
/// any expanded nested rows).
fn sync_tree_row_layout(tree: &mut UiTree, tree_id: NodeId) {
    let Some(tree_node) = tree.node(tree_id).and_then(|node| match &node.spec.0 {
        UiNode::Tree(tree_node) => Some(tree_node.clone()),
        _ => None,
    }) else {
        return;
    };
    let width = tree.accepted_layout(tree_id).map_or(0.0, |layout| layout.width);
    let mut section_y = 0.0;
    for section in &tree_node.sections {
        let Some(section_id) = find_child_by_key(tree, tree_id, &NodeKey::Explicit(section.id.clone())) else { continue };
        let header_offset = if section.label.is_some() { PANEL_HEADER } else { 0.0 };
        let mut item_y = header_offset;
        for item in &section.items {
            item_y += sync_tree_item_layout(tree, section_id, item, item_y, width);
        }
        if let Some(node) = tree.node_mut(section_id) {
            node.layout.x = 0.0;
            node.layout.y = section_y;
            node.layout.width = width;
            node.layout.height = item_y;
        }
        section_y += item_y;
    }
}

/// 🌳️ Recursive per-item counterpart of `sync_tree_row_layout`, one level down — writes `item`'s own
/// retained row `Stack` geometry (found by `item.id`, `reconcile::tree_item_row`'s key) at
/// `y_offset` relative to `parent` (its retained parent row/section), then recurses into any
/// expanded nested `items` relative to *this* row, mirroring `paint_tree_item`'s identical
/// recursion. Also keeps `NodeFlags::DRAG_SOURCE` synced with `item.draggable` (see
/// `events::is_plain_stack_container`/`set_drag_payload` for the two consumers of that bit). Returns
/// the total height (own row + any expanded nested rows) consumed, for the caller's own cursor.
fn sync_tree_item_layout(tree: &mut UiTree, parent: NodeId, item: &UiTreeItemNode, y_offset: f32, width: f32) -> f32 {
    if !item.presence.visible() {
        return 0.0;
    }
    let Some(item_id) = find_child_by_key(tree, parent, &NodeKey::Explicit(item.id.clone())) else {
        return TREE_ROW_HEIGHT;
    };
    let expandable = item.items.as_ref().is_some_and(|items| !items.is_empty());
    let expanded = expandable && item.default_open.unwrap_or(false);
    let mut nested_height = 0.0;
    if expanded {
        for nested in item.items.as_ref().unwrap() {
            nested_height += sync_tree_item_layout(tree, item_id, nested, TREE_ROW_HEIGHT + nested_height, width);
        }
    }
    let total_height = TREE_ROW_HEIGHT + nested_height;
    if let Some(node) = tree.node_mut(item_id) {
        node.layout.x = 0.0;
        node.layout.y = y_offset;
        node.layout.width = width;
        node.layout.height = total_height;
        node.flags.set(NodeFlags::DRAG_SOURCE, item.draggable.unwrap_or(false));
    }
    total_height
}
//#endregion 🔖️InteractiveStateSync

/// 🎯️ Per-variant paint dispatcher for one retained node, given `(origin_x, origin_y)` — the
/// absolute position of *this node's parent's* content-box origin (so `origin + node.layout.{x,y}`
/// is this node's own absolute top-left, matching taffy's parent-relative `Layout::location`).
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
/// 🧭️ The one shared presence overlay every `UiNode` variant gets for free, drawn centrally by
/// `paint_node` after that variant's own paint: `previewed`/`disabled` fills underneath nothing extra
/// (disabled reads as a scrim so it composes over whatever the variant already drew), a `status` ring
/// (loading spin / waiting dash / finished solid — mutually exclusive, `idle` draws nothing), an
/// outset accent ring for `selected`, and a breathing pulse ring for `introducing`. `hover` has no
/// dedicated draw call here — it's folded into `flags` before dispatch (see `paint_node`) so every
/// variant's own hover-aware fill (already reading `NodeFlags::HOVERED`) picks it up for free.
fn presence_overlay(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: &UiPresence) {
    if presence.state == UiState::Disabled {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel.with_alpha(0.35));
    }
    let ring_color = if presence.selected { theme.selected } else { theme.border_normal };
    match presence.status {
        UiStatus::Loading => paint_loading_border(draw, bounds, ring_color, theme),
        UiStatus::Waiting => paint_waiting_border(draw, bounds, ring_color, theme),
        UiStatus::Finished => draw.push_finished_border([bounds.x, bounds.y, bounds.w, bounds.h], ring_color, theme.border_radius, theme.stroke_hairline),
        UiStatus::Idle => {}
    }
    if presence.selected {
        let ring = Rect::new(bounds.x - 1.0, bounds.y - 1.0, bounds.w + 2.0, bounds.h + 2.0);
        push_chrome_border(draw, ring, theme.stroke_hairline, theme.accent, true, true, true, true);
    } else if presence.state == UiState::Previewed {
        // 🔍️ Inset (not outset, unlike `selected`'s ring) hairline so the two stay distinguishable
        // when composed — a previewed-and-selected element still reads as selected via the outset ring.
        push_chrome_border(draw, bounds, theme.stroke_hairline, theme.accent, true, true, true, true);
    }
    if presence.state == UiState::Introducing {
        draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], theme.accent, theme.border_radius, theme.stroke_hairline);
    }
    // 🎉️ `Celebrating` reuses the introducing breathing-pulse ring — `Theme` has no primary/secondary/
    // tertiary triad to cycle through, so `theme.accent` is the honest static reduction of the CSS
    // spinning tri-color ring for this shader-less renderer; a true conic tri-color ring is out of scope.
    if presence.state == UiState::Celebrating {
        draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], theme.accent, theme.border_radius, theme.stroke_hairline);
    }
}

pub(crate) fn paint_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    paint_node_self(tree, id, origin_x, origin_y, theme, atlas, icons, has_scene_host, draw);
    let Some(node) = tree.node(id) else { return };
    let Some(layout) = tree.accepted_layout(id) else { return };
    if !node.spec.0.presence().visible() {
        return;
    }
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    match &node.spec.0 {
        UiNode::Stack(_) | UiNode::Field(_) | UiNode::Section(_) | UiNode::Group(_) => paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw),
        UiNode::Select(select) => paint_select(select, Rect::new(abs_x, abs_y, layout.width, layout.height), node.flags, node.state.open, Some((tree, id)), theme, atlas, icons, draw),
        UiNode::KeyValue(key_value) => paint_key_value(key_value, Rect::new(abs_x, abs_y, layout.width, layout.height), theme, atlas, draw),
        UiNode::IconSelect(select) => paint_icon_select(select, Rect::new(abs_x, abs_y, layout.width, layout.height), node.flags, theme, atlas, icons, draw),
        UiNode::Tree(tree_node) => paint_tree_widget(tree_node, Rect::new(abs_x, abs_y, layout.width, layout.height), theme, atlas, icons, draw),
        _ => {}
    }
}

pub(crate) fn paint_node_self(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    let Some(node) = tree.node(id) else { return };
    let Some(layout) = tree.accepted_layout(id) else { return };
    let presence = node.spec.0.presence();
    if !presence.visible() {
        return;
    }
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    let bounds = Rect::new(abs_x, abs_y, layout.width, layout.height);
    if matches!(presence.status, UiStatus::Loading | UiStatus::Waiting) {
        draw.push_solid([bounds.x + theme.padding_standard, bounds.y + theme.padding_standard, (bounds.w - theme.padding_standard * 2.0).max(0.0), (bounds.h - theme.padding_standard * 2.0).max(0.0)], theme.button_hover);
        presence_overlay(draw, bounds, theme, presence);
        return;
    }
    // 🖱️ Authored `presence.hover` (default false) composes with live pointer hover: every variant's
    // own paint already reads `NodeFlags::HOVERED` for its hover-aware fill, so folding the authored
    // flag in here — suppressed while disabled, matching `events::EventRouter`'s own suppression —
    // makes it effective everywhere for free, with no per-variant paint changes.
    let mut flags = node.flags;
    if presence.state != UiState::Disabled {
        flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
    }
    match &node.spec.0 {
        UiNode::Stack(stack) => {
            paint_stack_frame(stack, bounds, flags, theme, draw);
        }
        UiNode::Text(text) => paint_text(text, bounds, theme, atlas, draw),
        UiNode::Separator(_) => paint_separator(bounds, theme, draw),
        UiNode::Button(button) => paint_button(button, bounds, flags, theme, atlas, icons, draw),
        UiNode::Input(input) => paint_input(input, node.state.edit.as_ref(), bounds, flags, theme, atlas, draw),
        UiNode::Select(_) => {}
        UiNode::Toggle(toggle) => paint_toggle(toggle, bounds, flags, theme, atlas, icons, draw),
        UiNode::KeyValue(_) => {}
        UiNode::Slider(slider) => paint_slider(slider, bounds, theme, atlas, draw),
        UiNode::NumberStepper(stepper) => paint_number_stepper(stepper, bounds, flags, theme, atlas, draw),
        UiNode::Ring(ring) => paint_ring(ring, bounds, theme, draw),
        UiNode::IconSelect(_) => {}
        UiNode::Field(field) => {
            paint_field(field, bounds, theme, atlas, draw);
        }
        UiNode::Section(section) => {
            paint_section(section, bounds, theme, atlas, icons, draw);
        }
        UiNode::Group(group) => {
            paint_group(group, bounds, theme, atlas, icons, draw);
        }
        UiNode::Tree(_) => {}
        // 🎬️ With a `SceneHost` registered this tick, leave these two rects untouched here —
        // `engine::Ui::frame`'s `collect_scene_slots` loop paints the real content right after this
        // pass returns. With no host, fall back to the unchanged placeholder chrome.
        UiNode::Image(image) => {
            if !has_scene_host {
                paint_image(image, bounds, theme, atlas, draw);
            }
        }
        UiNode::ComponentScene(scene) => {
            if !has_scene_host {
                paint_component_scene(scene, bounds, theme, draw);
            }
        }
        UiNode::ExternalSlot(slot) => paint_external_slot(slot, bounds, theme, atlas, draw),
    }
    presence_overlay(draw, bounds, theme, presence);
}

/// 🌀️ Shared "this node is loading" affordance for every `UiNode` kind that carries a
/// `loading: Option<bool>` flag (`Button`, `Stack`, `Section`, `Tree`, `TreeItem`). Delegates to
/// `draw::DrawList::push_loading_border`, which already renders a real time-varying (spinning +
/// pulsing) ring via `UI_SHADER`'s `kind == 6` branch fed by `render_frame`'s `time_seconds`
/// uniform (see `UiInstance::loading_border`'s doc comment) — despite older planning docs assuming
/// no animation-clock scaffolding exists anywhere in this crate, `draw`/`shaders` already wired one
/// in at the GPU layer; this helper just standardizes the radius/stroke args every `paint` call site
/// passes into that existing primitive, leaving only `color` (which varies with e.g. selected state)
/// to the caller.
fn paint_loading_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
    draw.push_loading_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
}

/// 🌀️ Shared "this node is waiting" affordance mirroring `paint_loading_border`: dashed, slower ring
/// via `draw::DrawList::push_waiting_border` (`UI_SHADER`'s `kind == 7` branch). Callers dispatch
/// `loading` before `waiting` so the more active state wins when both flags are set.
fn paint_waiting_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
    draw.push_waiting_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
}

/// 🎴️ A `Stack`'s `activate`/`selected` visual affordances, ported from
/// `framework/renderer/react/ui-interpreter.tsx`'s `case "stack"` (`widgets::WidgetNode::Stack` has
/// neither field to port from — see this region's own doc comment on why `widgets` is an incomplete
/// reference for fixtures like this one): `activate` (React's `"border bg-panel cursor-pointer
/// rounded-md"`) paints a filled `theme.panel` background (brighter, `theme.button_hover`, while
/// `events::EventRouter`'s hover-chain has flagged it `NodeFlags::HOVERED` — see
/// `events::is_plain_stack_container`'s matching hit-test exception for why an activatable Stack can
/// be hovered/clicked at all) with a normal border; `selected` (`"ring-primary border-primary
/// ring-1"`) paints an accent-colored border plus a slightly outset accent ring, approximating the
/// DOM's separate `ring`+`border` layers with this crate's single stroke-rect primitive.
/// `dropAction`'s accept-a-drop affordance has no dedicated visual in the React reference either
/// (`onDragOver`/`onDrop` are behavioral only) — its only paint-visible effect is keeping
/// `NodeFlags::DROP_TARGET` in sync (`sync_interactive_state`, above), consumed by
/// `events`/cursor-derivation, not drawn here.
fn paint_stack_frame(stack: &UiStackNode, bounds: Rect, flags: NodeFlags, theme: &Theme, draw: &mut DrawList) {
    let activatable = stack.activate.is_some();
    if !activatable {
        return;
    }
    let hovered = flags.contains(NodeFlags::HOVERED);
    let bg = if hovered { theme.button_hover } else { theme.panel };
    push_control_border(draw, bounds, theme, theme.border_normal, bg);
}

/// 🧱️ `Stack`'s own paint (beyond `paint_node`'s separate `paint_stack_frame` call for its
/// `activate`/`selected` affordance) is a no-operation — it's pure layout; this just recurses into its
/// retained children, each offset by this node's absolute top-left. Also reused by `Field`/`Section`,
/// whose single/`children` nested `UiNode`s reconcile already expands into retained children (see
/// `reconcile::children_of`) — `paint_stack_frame` doesn't apply to either (neither carries
/// `activate`/`selected`).
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
fn paint_stack(tree: &UiTree, id: NodeId, abs_x: f32, abs_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
    let children: Vec<NodeId> = tree.children(id).collect();
    for child in children {
        paint_node(tree, child, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
    }
}

fn paint_text(node: &UiTextNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let emphasize = node.emphasize.unwrap_or(false);
    let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
    let color = if emphasize { theme.text } else { theme.text_muted };
    let lines = wrap_text(atlas, node.value.as_str(), bounds.w.max(1.0), size);
    let line_h = size * 1.35;
    for (index, line) in lines.iter().enumerate() {
        draw_text_on(draw, atlas, line, bounds.x, bounds.y + line_h * index as f32 + size, size, color);
    }
}

fn paint_separator(bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let y = bounds.y + bounds.h * 0.5;
    draw.push_line(bounds.x, y, bounds.x + bounds.w, y, theme.separator, 1.0);
}

fn paint_button(node: &UiButtonNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    // 🚫️ `disabled:opacity-50` is the shared dimming convention this codebase's React reference
    // (`ui/js/react/index.tsx`'s form controls) uses for every disabled interactive control; ported
    // here via `Rgba::with_alpha` since `paint` has no CSS to lean on. A disabled control also can't
    // be hovered — `widgets::render_button` has no `disabled` concept at all (see this region's own
    // doc comment on why `widgets` is an incomplete reference for this specific fixture), so this is
    // an independent, `UiButtonNode.disabled`-driven fix rather than a widgets port.
    let disabled = node.presence.state == UiState::Disabled;
    let hovered = !disabled && flags.contains(NodeFlags::HOVERED);
    // 🎯️ `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`,
    // applied to every form-control primitive including `Button`) — `widgets::render_button` never
    // implemented a focus ring either (only `render_input` did), so this is another independent
    // React-sourced fix, mirroring `paint_input`'s own established border-swap convention.
    let focused = !disabled && flags.contains(NodeFlags::FOCUSED);
    let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
    let bg = dim(item_bg(theme, false, hovered));
    let border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, dim(border), bg);
    let mut text_x = bounds.x + theme.padding_standard;
    let icon_key = if node.icon_id == IconName::CircleDot { node.label.as_str() } else { node.icon_id.as_str() };
    if let Some(icons) = icons {
        if icons.icon_uv(icon_key).is_some() {
            push_icon(draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, dim(item_text(theme, false, hovered)));
            text_x += ICON_TINY + theme.gap_standard;
        }
    }
    draw_text_on(draw, atlas, node.label.as_str(), text_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, dim(item_text(theme, false, hovered)));
}

/// ↔ Local mirror of `events::selection_bounds` — `anchor..caret` as `(start, end)` regardless of
/// which is smaller (see `tree::EditState`'s own doc comment). Duplicated rather than imported
/// across the `paint`/`events` module boundary for a one-line pure function; keep the two in sync
/// if `EditState`'s selection convention ever changes.
fn edit_selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ `edit` is `node.state.edit` (see `tree::WidgetState`'s doc comment: `Some` only while this
/// `Input` is focused and has a live typing buffer). While present, the live `EditState::text`
/// (with any in-progress IME `composition` spliced in at the caret for preview) wins over the
/// declarative `node.value` — the same "focused buffer governs" contract `events::FocusState`
/// already establishes — since caret/selection coordinates are only meaningful against the exact
/// string they were computed from. Neither `widgets::render_input` nor React's native `<input>`
/// (whose caret/selection are rendered by the browser itself, not by application code — there is no
/// CSS/JSX to port for their exact geometry) has anything to port from, so caret/selection styling
/// (`theme.accent`) is this pass's own independent choice, kept consistent with `paint_input`'s own
/// pre-existing `border_emphasized`-on-focus convention.
fn paint_input(node: &UiInputNode, edit: Option<&EditState>, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let focused = flags.contains(NodeFlags::FOCUSED);
    let border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, theme.input_bg);
    let text_x = bounds.x + 8.0;
    let text_baseline_y = bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0;
    if let Some(edit) = focused.then_some(edit).flatten() {
        let (start, end) = edit_selection_bounds(edit.anchor, edit.caret);
        if start != end {
            let (x0, _) = atlas.measure_text(&edit.text[..start], theme.font_size_body);
            let (x1, _) = atlas.measure_text(&edit.text[..end], theme.font_size_body);
            let sel_h = theme.font_size_body * 1.2;
            let sel_y = bounds.y + (bounds.h - sel_h) * 0.5;
            draw.push_solid([text_x + x0, sel_y, (x1 - x0).max(1.0), sel_h], theme.accent.with_alpha(0.3));
        }
        let mut display = edit.text.clone();
        if let Some(composition) = &edit.composition {
            display.insert_str(edit.caret, composition);
        }
        draw_text_on(draw, atlas, &display, text_x, text_baseline_y, theme.font_size_body, theme.text);
        let (caret_x, _) = atlas.measure_text(&edit.text[..edit.caret], theme.font_size_body);
        let caret_h = theme.font_size_body * 1.2;
        let caret_y = bounds.y + (bounds.h - caret_h) * 0.5;
        draw.push_solid([text_x + caret_x, caret_y, 1.0, caret_h], theme.accent);
        return;
    }
    let (display, muted): (&str, bool) = if node.value.is_empty() { (node.placeholder.as_ref().map(Label::as_str).unwrap_or(""), true) } else { (node.value.as_str(), false) };
    draw_text_on(draw, atlas, display, text_x, text_baseline_y, theme.font_size_body, if muted { theme.text_muted } else { theme.text });
}

/// 🔽️ `retained` is `Some((tree, id))` for a real top-level `Select` node (able to read its
/// synthesized item rows' live `NodeFlags::HOVERED` for the popup's row-hover highlight) and `None`
/// for an inline `Select` painted via `paint_control` (a `TreeItem`'s embedded control — no per-
/// control `NodeId` exists for that yet, same caveat `paint_control`'s own doc comment already
/// makes, so it always paints closed regardless of `open`). W2 wiring: `open` (from
/// `tree::WidgetState::open`, toggled by `events::EventRouter::toggle_select_popup`) now has a real
/// data source, closing the gap this function's own doc comment used to describe — when `true`, the
/// popup paints below the trigger with the exact geometry `select_popup_row_rect` also writes into
/// the rows' `LayoutBucket` (see `sync_select_popup_rows`), so clicking a row actually hit-tests.
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
fn paint_select(node: &UiSelectNode, bounds: Rect, flags: NodeFlags, open: bool, retained: Option<(&UiTree, NodeId)>, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ `SelectTrigger`'s own `formControlFocusBorderClass` (`ui/js/react/index.tsx`) swaps its
    // border to `border-accent` on `focus-visible` — mirrored via the same border-swap convention
    // `paint_input`/`paint_button` already use, since `widgets::render_select` never implemented one.
    let focused = flags.contains(NodeFlags::FOCUSED);
    let bg = if hovered { theme.button_hover } else { theme.input_bg };
    let border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, bg);
    let label = node.items.iter().find(|item| item.value == node.value).map_or_else(|| node.placeholder.as_ref().map(Label::as_str).unwrap_or("Select…"), |item| item.label.as_str());
    draw_text_on(draw, atlas, label, bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    if let Some(icons) = icons {
        push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    }
    if !open {
        return;
    }
    let row_children: Vec<NodeId> = retained.map(|(tree, id)| tree.children(id).collect()).unwrap_or_default();
    let item_h = theme.control_height;
    let menu_h = node.items.len() as f32 * item_h + 4.0;
    let menu = Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, menu_h);
    draw.push_glass([menu.x, menu.y, menu.w, menu.h], theme.border_radius, theme.glass(Level::Menu));
    for (index, item) in node.items.iter().enumerate() {
        let relative = select_popup_row_rect(bounds.w, bounds.h, index, theme);
        let row = Rect::new(bounds.x + relative.x, bounds.y + relative.y, relative.w, relative.h);
        let row_hovered = retained.zip(row_children.get(index)).is_some_and(|((tree, _), &row_id)| tree.node(row_id).is_some_and(|n| n.flags.contains(NodeFlags::HOVERED)));
        if row_hovered || item.value == node.value {
            draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
        }
        draw_text_on(draw, atlas, item.label.as_str(), row.x + 8.0, row.y + 18.0, theme.font_size_body, theme.text);
    }
}

fn paint_toggle(node: &UiToggleNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let pressed = node.presence.selected;
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ Same `formControlFocusBorderClass` border-swap as `paint_button`/`paint_select` — the icon-
    // button variant `Toggle` renders through (`ui/js/react/index.tsx`) carries it too.
    let focused = flags.contains(NodeFlags::FOCUSED);
    let bg = item_bg(theme, pressed, hovered);
    let border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, bg);
    let mut content_x = bounds.x + theme.padding_standard;
    if let Some(icons) = icons {
        if icons.icon_uv(node.icon_id.as_str()).is_some() {
            push_icon(draw, icons, node.icon_id.as_str(), content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
            content_x += ICON_TINY + theme.gap_standard;
        }
    }
    if let Some(text) = &node.text {
        draw_text_on(draw, atlas, text.as_str(), content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, item_text(theme, pressed, hovered));
    }
}

fn paint_key_value(node: &UiKeyValueNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let label_w = node.entries.iter().map(|entry| atlas.measure_text(entry.label.as_str(), theme.font_size_small).0).fold(0.0f32, f32::max);
    let value_x = bounds.x + label_w + theme.gap_standard * 2.0;
    let row_h = theme.control_height;
    for (index, entry) in node.entries.iter().enumerate() {
        let y = bounds.y + index as f32 * row_h;
        draw_text_on(draw, atlas, entry.label.as_str(), bounds.x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
        draw_text_on(draw, atlas, &entry.value, value_x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
    }
}

fn paint_slider(node: &UiSliderNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let track_y = bounds.y + bounds.h * 0.5;
    draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], theme.separator, 2.0);
    let range = (node.max - node.min).max(f64::EPSILON);
    let t = ((node.value - node.min) / range).clamp(0.0, 1.0);
    let knob_x = bounds.x + bounds.w * t as f32;
    draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], theme.accent, 6.0);
    // 📏️ `ui-interpreter.tsx`'s `case "slider"` is the ground truth for the unit-label readout
    // (`WidgetNode::Slider` has no `unit` field at all, so there's nothing to port from `widgets`
    // here either): `{control.value} {control.unit}`, muted small text, trailing the track. React
    // lays it out as a sibling flex item outside the slider's own box; `paint` has no extra layout
    // space to claim (that's `flex`'s call, out of scope here), so this right-aligns inside the
    // slider's own bounds as the closest in-bounds approximation.
    if let Some(unit) = &node.unit {
        let text = format!("{} {unit}", node.value);
        let (w, _) = atlas.measure_text(&text, theme.font_size_small);
        draw_text_on(draw, atlas, &text, bounds.x + bounds.w - w, track_y + theme.font_size_small * 0.5 - 2.0, theme.font_size_small, theme.text_muted);
    }
}

fn paint_number_stepper(node: &UiNumberStepperNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let seg = bounds.w / 3.0;
    let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
    let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
    let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
    let hair = theme.stroke_hairline;
    // 🖱️ `Stepper`'s minus/plus `<Button variant="outline">`s (`ui/js/react/index.tsx`) each carry
    // their own `hover:bg-muted`/`focus-visible:bg-muted`/`formControlFocusBorderClass`; this retained
    // model has no per-segment `NodeId` (the whole stepper is one hit-testable node — see this
    // function's caller, `paint_control`'s doc comment, for the same one-`NodeId`-per-composite
    // caveat), so the closest in-model approximation tints the shared outer bg/border for hover/focus,
    // which the nested center-segment border below then repaints back to `input_bg`/`border_normal`
    // (the center "value" segment isn't a button — it never carries React's own hover/focus fill).
    let hovered = flags.contains(NodeFlags::HOVERED);
    let focused = flags.contains(NodeFlags::FOCUSED);
    let outer_bg = if hovered { theme.button_hover } else { theme.input_bg };
    let outer_border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, outer_border, outer_bg);
    draw.push_solid([bounds.x + seg, bounds.y, hair, bounds.h], theme.border_normal);
    draw.push_solid([bounds.x + seg * 2.0, bounds.y, hair, bounds.h], theme.border_normal);
    // 🔲️ `widgets::render_number_stepper` renders the center value segment through a full
    // `render_input` call, which nests its own `push_control_border` box around the value —
    // `golden_number_stepper_known_gap`'s doc comment measured this as the exact 14-vs-19-instance
    // divergence (the missing nested border box). Ported verbatim here to close that gap.
    push_control_border(draw, center, theme, theme.border_normal, theme.input_bg);
    draw_text_on(draw, atlas, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, theme.font_size_body, theme.text);
    // 🔀️ `uniform: false` means the selection's values disagree (`ui-interpreter.tsx`'s
    // `case "numberStepper"`: `value: control.uniform ? control.value : undefined, mixed: !control.uniform`
    // fed into `<Stepper mixed>`, which shows `mixedLabel` — `UI_INSPECTOR_MIXED_PLACEHOLDER`'s Rust
    // side of that same string) instead of a formatted number. `widgets::render_number_stepper`
    // ignores `uniform` entirely (both branches of its `if uniform {..} else {..}` format the same
    // way — a `widgets`-side gap this doesn't port from, since there's nothing correct to port).
    let (text, text_color) = if node.uniform { (format!("{:.3}", node.value), theme.text) } else { (UI_INSPECTOR_MIXED_PLACEHOLDER.to_string(), theme.text_muted) };
    draw_text_on(draw, atlas, &text, center.x + 8.0, center.y + (center.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, text_color);
    draw_text_on(draw, atlas, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, theme.font_size_body, theme.text);
}

fn paint_ring(node: &UiRingNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let cx = bounds.x + bounds.w * 0.5;
    let cy = bounds.y + bounds.h * 0.5;
    let radius = bounds.w.min(bounds.h) * 0.4;
    let segments = 48usize;
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
    }
    for window in points.windows(2) {
        draw.push_line(window[0][0], window[0][1], window[1][0], window[1][1], theme.separator, 2.0);
    }
    let disabled = node.presence.state == UiState::Disabled;
    let knob_angle = std::f32::consts::TAU * node.t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    let accent = if disabled { theme.text_muted } else { theme.accent };
    draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
}

fn paint_icon_select(node: &UiIconSelectNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let hovered = flags.contains(NodeFlags::HOVERED);
    // 🎯️ Same border-swap-on-focus convention as `paint_button`/`paint_select`/`paint_toggle` — the
    // real `IconSelector` (`ui/js/react/index.tsx`) nests a `Select` for its mode picker, which
    // inherits `formControlFocusBorderClass` the same way.
    let focused = flags.contains(NodeFlags::FOCUSED);
    let border = if focused { theme.border_emphasized } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, chrome_item_bg(theme, false, hovered));
    let content_x = bounds.x + theme.padding_standard;
    let has_icon = icons.and_then(|icons| icons.icon_uv(&node.value)).is_some();
    if let (true, Some(icons)) = (has_icon, icons) {
        push_icon(draw, icons, &node.value, content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    } else {
        draw_text_on(draw, atlas, &node.value, content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    }
}

/// 📝️ A `Field`'s label (+ required marker) and description/error text; its `child` control is a
/// retained child painted separately by `paint_stack`. Layout intent ported from `ui/js/react/index.tsx`'s
/// `Field` component (`widgets::render_widget`'s `WidgetNode::Field` arm only draws the bare label —
/// no description/required/error at all — so those three are an independent port from the React
/// reference, not from `widgets`): label (+ `*` required marker in `theme.error`) on the first line,
/// description muted-small below it, error (in `theme.error`) below that. `reconcile`/`flex` don't
/// yet reserve the child control's layout slot below this text (see `golden_field_known_gap`'s doc
/// comment — a documented `flex` gap, out of scope here), so these lines are positioned relative to
/// `bounds.y` only; they'll land correctly once that flex gap is fixed.
fn paint_field(node: &UiFieldNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    let label_size = theme.font_size_small;
    draw_text_on(draw, atlas, node.label.as_str(), bounds.x, bounds.y + label_size, label_size, theme.text_muted);
    let mut y = bounds.y + label_size;
    if node.required.unwrap_or(false) {
        let (label_w, _) = atlas.measure_text(node.label.as_str(), label_size);
        draw_text_on(draw, atlas, "*", bounds.x + label_w + 2.0, y, label_size, theme.error);
    }
    if let Some(description) = &node.description {
        y += label_size + theme.gap_standard * 0.5;
        draw_text_on(draw, atlas, description, bounds.x, y, label_size, theme.text_muted);
    }
    if let Some(error) = &node.error {
        y += label_size + theme.gap_standard * 0.5;
        draw_text_on(draw, atlas, error, bounds.x, y, label_size, theme.error);
    }
}

/// 📂️ A `Section`'s header chevron+label; its `children` are retained children painted separately by
/// `paint_stack`. Collapsed state still reads `default_open` directly — no `WidgetState`-backed
/// toggle persistence exists for `Section` yet (unlike `Select`'s popup open/closed state and
/// `Input`'s live edit buffer, both wired by now — see `WidgetState`'s own doc comment).
fn paint_section(node: &UiSectionNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let Some(label) = &node.label else { return };
    let collapsed = !node.default_open.unwrap_or(true);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    if let Some(icons) = icons {
        push_icon(draw, icons, chevron, bounds.x, bounds.y + (PANEL_HEADER - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    }
    draw_text_on(draw, atlas, label.as_str(), bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, bounds.y + (PANEL_HEADER + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
}

/** @emoji 🌿️ Same header chrome as {@link paint_section} (chevron + label), for a `Group`'s always-
 * present `label` — used when a nested subtree (e.g. `Origin`) is painted directly in the native
 * retained tree rather than pre-expanded into `UiTreeItemNode.items`. */
fn paint_group(node: &UiGroupNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let collapsed = !node.default_open.unwrap_or(true);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    if let Some(icons) = icons {
        push_icon(draw, icons, chevron, bounds.x, bounds.y + (PANEL_HEADER - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
    }
    draw_text_on(draw, atlas, node.label.as_str(), bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, bounds.y + (PANEL_HEADER + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
}

fn paint_tree_widget(node: &UiTreeNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    draw.push_scissor(bounds);
    let mut y = bounds.y;
    for section in &node.sections {
        if let Some(label) = &section.label {
            // 🗂️ `widgets::render_tree_section_header` draws a folder icon before the label and
            // dims the label to `text_muted` only while collapsed (`text_element` otherwise) —
            // ported here; previously this always used `text_muted` regardless of collapsed state.
            let collapsed = !section.default_open.unwrap_or(true);
            let text_color = if collapsed { theme.text_muted } else { theme.text_element };
            let label_x = bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard;
            if let Some(icons) = icons {
                push_icon(draw, icons, "folder", label_x, y + (PANEL_HEADER - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
            }
            draw_text_on(draw, atlas, label.as_str(), label_x + TREE_ICON_SIZE + theme.gap_standard, y + (PANEL_HEADER + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, text_color);
            y += PANEL_HEADER;
        }
        for item in &section.items {
            y = paint_tree_item(item, bounds.x, bounds.w, y, 1, node, theme, atlas, icons, draw, &[]);
        }
    }
    draw.pop_scissor();
    // 🧭️ Status/selected/introducing rings for the whole `Tree` are drawn once, centrally, by
    // `paint_node`'s shared `presence_overlay` — not duplicated here.
}

/// 🌳️ Recursive row painter for one `Tree` item (and, if expanded, its nested `items`). Ports every
/// piece of `widgets::render_tree_item`'s visual structure that depends only on static retained data
/// (ancestor guide lines, selected/highlighted text color, description text, always-visible actions,
/// an inline `control`) — anything that depends on *live* hover/drag/focus state (row hover fill,
/// hover-revealed actions, hover-highlighted action icons, drag guides) stays out of scope: there is
/// no per-tree-row `NodeId`/`NodeFlags` yet (`reconcile::children_of` doesn't expand `Tree` into
/// retained item children — see `paint_select`'s neighboring doc comment for the same root cause), so
/// there is nowhere to read a live per-row hover/drag flag from until that reconcile expansion lands.
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
fn paint_tree_item(item: &UiTreeItemNode, x: f32, width: f32, y: f32, depth: u32, tree_node: &UiTreeNode, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList, is_last_at_level: &[bool]) -> f32 {
    if !item.presence.visible() {
        return y;
    }
    let row = Rect::new(x, y, width, TREE_ROW_HEIGHT);
    let selected = item.presence.selected;
    let previewed = item.presence.state == UiState::Previewed;
    let dimmed = item.dimmed.unwrap_or(false) || item.presence.state == UiState::Disabled;
    if selected {
        draw.push_rounded([row.x, row.y, row.w, row.h], theme.selected, theme.border_radius);
    } else if previewed {
        draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
    }
    let ring_color = if selected { theme.selected } else { theme.border_normal };
    match item.presence.status {
        UiStatus::Loading => paint_loading_border(draw, row, ring_color, theme),
        UiStatus::Waiting => paint_waiting_border(draw, row, ring_color, theme),
        UiStatus::Finished => draw.push_finished_border([row.x, row.y, row.w, row.h], ring_color, theme.border_radius, theme.stroke_hairline),
        UiStatus::Idle => {}
    }
    if item.presence.state == UiState::Introducing || item.presence.state == UiState::Celebrating {
        draw.push_introducing_border([row.x, row.y, row.w, row.h], theme.accent, theme.border_radius, theme.stroke_hairline);
    }
    paint_tree_guides(draw, x, row.y, row.h, depth, is_last_at_level, theme);
    let indent = x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
    let expandable = item.items.as_ref().is_some_and(|items| !items.is_empty());
    if expandable {
        if let Some(icons) = icons {
            let chevron = if item.default_open.unwrap_or(false) { "chevron-down" } else { "chevron-right" };
            push_icon(draw, icons, chevron, indent - TREE_TOGGLE_WIDTH, row.y + (TREE_ROW_HEIGHT - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
        }
    }
    // 🎨️ `widgets::render_tree_item`'s `text_color`: selected/previewed rows use `active_foreground`
    // for both icon tint and label (previously this always used `text_element`/`theme.text`);
    // `dimmed` (the eye-toggle "hidden in scene" domain flag, or `presence.state == Disabled`) halves
    // its alpha without skipping the row — it stays visible and clickable to un-hide/re-enable.
    let text_color = if selected || previewed { theme.active_foreground } else { theme.text_element };
    let text_color = if dimmed { text_color.with_alpha(text_color.a * 0.5) } else { text_color };
    if let (Some(icons), Some(icon_id)) = (icons, item.icon_id) {
        push_icon(draw, icons, icon_id.as_str(), indent, row.y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
    }
    let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
    draw_text_on(draw, atlas, item.label.as_str(), label_x, row.y + (TREE_ROW_HEIGHT + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, text_color);
    if let Some(description) = &item.description {
        let (label_w, _) = atlas.measure_text(item.label.as_str(), theme.font_size_body);
        draw_text_on(draw, atlas, description, label_x + label_w + theme.gap_standard, row.y + (TREE_ROW_HEIGHT + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
    }
    let mut actions_x = row.x + row.w - theme.gap_standard;
    if let Some(icons) = icons {
        for action in item.actions.iter().flatten().rev() {
            if action.placement() == UiTreeActionPlacement::Menu {
                continue;
            }
            actions_x -= TREE_ICON_SIZE + theme.padding_standard;
            push_icon(draw, icons, action.icon_id.as_str(), actions_x, row.y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, theme.text_element);
        }
    }
    // 🎛️ An inline per-row control (e.g. a small toggle/select embedded in a tree row), static data
    // already present on `UiTreeItemNode` that the old paint pass never rendered at all.
    if let Some(control) = &item.control {
        let control_w = 120.0;
        let control_rect = Rect::new(row.x + row.w - control_w - theme.gap_standard, row.y + (row.h - theme.control_height) * 0.5, control_w, theme.control_height);
        paint_control(control, control_rect, theme, atlas, icons, draw);
    }
    let mut next_y = y + TREE_ROW_HEIGHT;
    if expandable && item.default_open.unwrap_or(false) {
        for (index, child) in item.items.as_ref().unwrap().iter().enumerate() {
            let mut child_is_last = is_last_at_level.to_vec();
            child_is_last.push(index + 1 == item.items.as_ref().unwrap().len());
            next_y = paint_tree_item(child, x, width, next_y, depth + 1, tree_node, theme, atlas, icons, draw, &child_is_last);
        }
    }
    next_y
}

/// 📏️ Ancestor connector lines for one tree row, ported from `widgets::tree_draw_guides` — adjusted
/// for `paint_tree_item`'s `depth` starting at `1` for top-level items (`widgets`' `render_tree_item`
/// starts its own `depth` at `0`), so every `widgets_depth` reference there is this function's
/// `depth - 1`.
fn paint_tree_guides(draw: &mut DrawList, row_x: f32, row_y: f32, row_h: f32, depth: u32, is_last_at_level: &[bool], theme: &Theme) {
    let hair = theme.stroke_hairline.max(1.0);
    let guide_color = theme.border_normal;
    for level in 0..depth.saturating_sub(1) {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = row_x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        draw.push_solid([x, row_y, hair, row_h], guide_color);
    }
    if depth > 1 {
        let x = row_x + (depth - 2) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = row_y + row_h * 0.5;
        draw.push_solid([x, row_y, hair, mid_y - row_y], guide_color);
        draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
    }
}

/// 🎛️ Adapter from a `TreeItem`'s inline `UiControlNode` payload (a narrower enum than `UiNode` —
/// see `component::ui::UiControlNode`'s own doc comment) to the matching `paint_*` function; mirrors
/// `paint_node`'s `UiNode` dispatch table one level down. No per-control `NodeId` exists for an inline
/// tree-row control yet, so it always paints at rest (`NodeFlags::empty()`) — same interactive-state
/// caveat as the rest of this function's caller.
fn paint_control(control: &UiControlNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
    let flags = NodeFlags::empty();
    match control {
        UiControlNode::Button(node) => paint_button(node, bounds, flags, theme, atlas, icons, draw),
        UiControlNode::Input(node) => paint_input(node, None, bounds, flags, theme, atlas, draw),
        UiControlNode::Select(node) => paint_select(node, bounds, flags, false, None, theme, atlas, icons, draw),
        UiControlNode::Toggle(node) => paint_toggle(node, bounds, flags, theme, atlas, icons, draw),
        UiControlNode::KeyValue(node) => paint_key_value(node, bounds, theme, atlas, draw),
        UiControlNode::Slider(node) => paint_slider(node, bounds, theme, atlas, draw),
        UiControlNode::NumberStepper(node) => paint_number_stepper(node, bounds, flags, theme, atlas, draw),
        UiControlNode::Ring(node) => paint_ring(node, bounds, theme, draw),
        UiControlNode::IconSelect(node) => paint_icon_select(node, bounds, flags, theme, atlas, icons, draw),
    }
}

/// 🖼️ `paint_node`'s caller (`paint_tree`) only reaches this when `has_scene_host` is `false` this
/// tick — a real `scene_slots::SceneHost` paints the actual image content instead (see `paint_node`'s
/// `UiNode::Image` arm). No host-side texture-upload queue exists in `ui_wgpu` itself even so (that
/// lives in the renderer's `program_bridge`/`engine_canvas`, outside this crate's scope); paints a
/// raster quad keyed by `src` on the chance a caller-owned `RasterTextureTable` already has that key
/// uploaded, falling back to `alt` text when there's nothing to show yet.
fn paint_image(node: &UiImageNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    if node.src.is_empty() {
        if let Some(alt) = &node.alt {
            draw_text_on(draw, atlas, alt.as_str(), bounds.x + 4.0, bounds.y + 16.0, theme.font_size_small, theme.text_muted);
        }
        return;
    }
    draw.push_raster_quad(&node.src, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
}

/// 🎬️ `paint_node`'s caller only reaches this when `has_scene_host` is `false` this tick — with a
/// real `scene_slots::SceneHost` registered, `engine::Ui::frame`'s `collect_scene_slots` loop paints
/// the actual scene surface (canvas2d/world3d/node-graph/…) into this same rect right after this
/// pass returns (see `paint_node`'s `UiNode::ComponentScene` arm), so this placeholder chrome is
/// purely the no-host fallback — "there's something visible in that rect" rather than nothing.
fn paint_component_scene(node: &UiComponentSceneNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let _ = &node.surface_id;
    push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
}

/// 🧩️ Same placeholder-chrome treatment as `paint_component_scene`: the plugin body itself is a host
/// concern (`program_bridge`), out of scope here; label the slot with its `body_key` for now.
fn paint_external_slot(node: &UiExternalSlotNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
    push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
    draw_text_on(draw, atlas, &node.body_key, bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, theme.text_muted);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::component::layout::ActionDescriptor;
    use crate::wgpu::component::ui::{UiFieldNode, UiNumberStepperNode, UiSectionNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTreeItemAction, UiTreeSectionNode};
    use crate::wgpu::draw::{KIND_GLYPH, KIND_LOADING_BORDER, KIND_SOLID, KIND_WAITING_BORDER};
    use crate::wgpu::flex::LayoutEngine;
    use crate::wgpu::tree::EditState;

    fn action() -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
    }

    fn text(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    fn stack(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    fn loading_button(id: &str) -> UiNode {
        UiNode::Button(UiButtonNode {
            id: Some(id.into()),
            icon_id: IconName::CircleDot,
            label: Label::data(id),
            action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
            style: None,
            presence: UiPresence::status(UiStatus::Loading),
            menu: None,
        })
    }

    fn waiting_button(id: &str) -> UiNode {
        UiNode::Button(UiButtonNode {
            id: Some(id.into()),
            icon_id: IconName::CircleDot,
            label: Label::data(id),
            action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
            style: None,
            presence: UiPresence::status(UiStatus::Waiting),
            menu: None,
        })
    }

    fn setup(ui: &UiNode) -> (UiTree, NodeId, Theme, FontAtlas) {
        let mut tree = UiTree::new();
        tree.apply_tree(ui);
        let root = tree.root.unwrap();
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let mut engine = LayoutEngine::new();
        engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
        (tree, root, theme, atlas)
    }

    #[test]
    fn painting_a_text_node_emits_glyph_instances() {
        let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(total_instances > 0, "text node should emit at least one glyph instance");
    }

    #[test]
    fn painting_a_stack_recurses_into_every_child() {
        let ui = stack(vec![text("a"), UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }), text("b")]);
        let (mut tree, root, theme, mut atlas) = setup(&ui);
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        let total_vectors: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
        assert!(total_instances > 0, "text children should have emitted glyphs");
        assert!(total_vectors > 0, "separator child should have emitted a line");
    }

    #[test]
    fn paint_tree_clears_dirty_paint_but_leaves_layout_dirt_flags_untouched() {
        let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let after_first = tree.node(root).unwrap().flags;
        assert!(!after_first.contains(NodeFlags::DIRTY_PAINT));
        assert!(!after_first.contains(NodeFlags::DIRTY_LAYOUT), "paint must not touch DIRTY_LAYOUT, that's flex's job");
        assert!(!after_first.contains(NodeFlags::SUBTREE_DIRTY), "flex::compute already cleared SUBTREE_DIRTY before paint ran");

        // Second call must be a no-operation w.r.t. these flags — repeat of the M3 SUBTREE_DIRTY bug class
        // (calling twice shouldn't set or double-clear something it shouldn't).
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
        let after_second = tree.node(root).unwrap().flags;
        assert_eq!(after_first, after_second);
    }

    #[test]
    fn painting_a_loading_button_emits_a_loading_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&loading_button("save"));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
        assert!(has_loading_border, "loading button should emit a KIND_LOADING_BORDER instance");
    }

    #[test]
    fn painting_a_waiting_button_emits_a_waiting_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&waiting_button("save"));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
        assert!(has_waiting_border, "waiting button should emit a KIND_WAITING_BORDER instance");
    }

    // 🚫️ `painting_a_loading_and_waiting_button_prefers_the_loading_border` deleted: `status` is now a
    // single `UiStatus` enum, so "loading and waiting both set" is unrepresentable — which is the point.

    //#region 🔖️FidelityFixes
    // 🩹️ One test per fidelity gap this pass closed (see `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`),
    // additive to the pre-existing tests above.

    fn button(id: &str, disabled: bool) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(id), action: action(), style: None, presence: UiPresence::disabled_if(disabled), menu: None })
    }

    #[test]
    fn painting_a_disabled_button_dims_its_border_alpha() {
        let (mut tree, root, theme, mut atlas) = setup(&button("btn", true));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let dimmed = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.color[3] - theme.border_normal.a * 0.5).abs() < 0.01);
        assert!(dimmed, "a disabled button should paint its border at half alpha");
    }

    fn loading_section(id: &str) -> UiNode {
        UiNode::Section(UiSectionNode { id: id.into(), label: Some(Label::data("Sec")), default_open: Some(true), presence: UiPresence::status(UiStatus::Loading), children: vec![text("child")], menu: None })
    }

    #[test]
    fn painting_a_loading_section_emits_a_loading_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&loading_section("sec"));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
        assert!(has_loading_border, "a loading section should emit a KIND_LOADING_BORDER instance");
    }

    fn waiting_section(id: &str) -> UiNode {
        UiNode::Section(UiSectionNode { id: id.into(), label: Some(Label::data("Sec")), default_open: Some(true), presence: UiPresence::status(UiStatus::Waiting), children: vec![text("child")], menu: None })
    }

    #[test]
    fn painting_a_waiting_section_emits_a_waiting_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&waiting_section("sec"));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
        assert!(has_waiting_border, "a waiting section should emit a KIND_WAITING_BORDER instance");
    }

    fn loading_stack(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::status(UiStatus::Loading),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
            menu: None,
        })
    }

    #[test]
    fn painting_a_loading_stack_emits_a_loading_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&loading_stack(vec![text("a")]));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
        assert!(has_loading_border, "a loading stack should emit a KIND_LOADING_BORDER instance");
    }

    fn waiting_stack(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: Some("none".into()),
            padding: Some("none".into()),
            id: None,
            presence: UiPresence::status(UiStatus::Waiting),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
            menu: None,
        })
    }

    #[test]
    fn painting_a_waiting_stack_emits_a_waiting_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&waiting_stack(vec![text("a")]));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
        assert!(has_waiting_border, "a waiting stack should emit a KIND_WAITING_BORDER instance");
    }

    fn loading_tree() -> UiNode {
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
            presence: UiPresence::status(UiStatus::Loading),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        })
    }

    fn waiting_tree() -> UiNode {
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item"))] }],
            presence: UiPresence::status(UiStatus::Waiting),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        })
    }

    #[test]
    fn painting_a_loading_tree_emits_a_loading_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&loading_tree());
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
        assert!(has_loading_border, "a loading tree should emit a KIND_LOADING_BORDER instance");
    }

    #[test]
    fn painting_a_waiting_tree_emits_a_waiting_border_instance() {
        let (mut tree, root, theme, mut atlas) = setup(&waiting_tree());
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
        assert!(has_waiting_border, "a waiting tree should emit a KIND_WAITING_BORDER instance");
    }

    fn stepper(id: &str, value: f64, uniform: bool) -> UiNode {
        UiNode::NumberStepper(UiNumberStepperNode { id: id.into(), value, step: 1.0, uniform, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })
    }

    #[test]
    fn painting_a_uniform_number_stepper_nests_a_border_around_its_center_value() {
        let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        // Outer control border (bg + 4 edges = 5) + 2 divider lines + nested center-value border
        // (bg + 4 edges = 5) + minus/"2.000"/plus glyphs (1 + 5 + 1 = 7) = 19 — the exact instance
        // count `golden_number_stepper_known_gap`'s doc comment measured `widgets::render_number_stepper`
        // emitting (vs this region's pre-fix 14), now matched by porting the nested border.
        assert_eq!(total, 19, "uniform NumberStepper should now nest a border around its center value, matching widgets' 19-instance output");
    }

    #[test]
    fn painting_a_mixed_number_stepper_shows_the_mixed_placeholder_in_muted_color() {
        let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, false));
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let has_muted_glyph =
            draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| {
                (instance.params[2] - KIND_GLYPH).abs() < 0.01 && (instance.color[0] - theme.text_muted.r).abs() < 0.001 && (instance.color[1] - theme.text_muted.g).abs() < 0.001 && (instance.color[2] - theme.text_muted.b).abs() < 0.001
            });
        assert!(has_muted_glyph, "a non-uniform (mixed) NumberStepper should paint its center value's glyphs in theme.text_muted (the 'Mixed' placeholder)");
    }

    fn slider(id: &str, unit: Option<&str>) -> UiNode {
        UiNode::Slider(UiSliderNode { id: id.into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: unit.map(String::from), on_change: action(), presence: UiPresence::default(), menu: None })
    }

    #[test]
    fn painting_a_slider_with_a_unit_emits_extra_glyphs_for_the_readout() {
        let (mut plain_tree, plain_root, theme, mut plain_atlas) = setup(&slider("sl", None));
        let mut plain_draw = DrawList::default();
        paint_tree(&mut plain_tree, plain_root, &theme, &mut plain_atlas, None, false, &mut plain_draw);

        let (mut unit_tree, unit_root, theme2, mut unit_atlas) = setup(&slider("sl", Some("mm")));
        let mut unit_draw = DrawList::default();
        paint_tree(&mut unit_tree, unit_root, &theme2, &mut unit_atlas, None, false, &mut unit_draw);

        let plain_total: usize = plain_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        let unit_total: usize = unit_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(unit_total > plain_total, "a slider with a unit should paint extra glyphs for its value+unit readout");
    }

    fn field(description: Option<&str>, required: bool, error: Option<&str>) -> UiNode {
        UiNode::Field(UiFieldNode {
            id: "f".into(),
            label: Label::data("Label"),
            description: description.map(String::from),
            required: Some(required),
            error: error.map(String::from),
            child: Box::new(text("child")),
            presence: UiPresence::default(),
            menu: None,
        })
    }

    #[test]
    fn painting_a_field_with_description_required_and_error_emits_extra_glyphs() {
        let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&field(None, false, None));
        let mut bare_draw = DrawList::default();
        paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

        let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&field(Some("desc"), true, Some("bad")));
        let mut rich_draw = DrawList::default();
        paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

        let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(rich_total > bare_total, "description/required-marker/error should each add glyph instances beyond the bare label");
    }

    fn tree_with_item_description() -> UiNode {
        let mut item = UiTreeItemNode::base("i1", Label::data("Item One"));
        item.description = Some("desc".into());
        item.actions = Some(vec![UiTreeItemAction { icon_id: IconName::Sparkles, label: None, action: action(), placement: None }]);
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        })
    }

    fn tree_with_bare_item() -> UiNode {
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", Label::data("Item One"))] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        })
    }

    #[test]
    fn painting_a_tree_item_with_description_emits_more_than_a_bare_item() {
        let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&tree_with_bare_item());
        let mut bare_draw = DrawList::default();
        paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

        let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&tree_with_item_description());
        let mut rich_draw = DrawList::default();
        paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

        let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(rich_total > bare_total, "a tree item's description text should paint extra glyphs beyond a bare item (icons are None here, so its always-visible action doesn't add its own icon instance in this fixture)");
    }
    //#endregion 🔖️FidelityFixes

    //#region 🔖️W2InteractivityFixes
    // 🔽️🎴️🌳️ Tests for `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 pass: Select popup painting
    // (`paint_select` + `sync_select_popup_rows`), Stack `activate`/`selected`/`drop_action`
    // (`paint_stack_frame` + `sync_interactive_state`'s `DROP_TARGET` sync), and Tree row real layout
    // + `DRAG_SOURCE` sync (`sync_tree_row_layout`/`sync_tree_item_layout`).

    fn select(id: &str, value: &str) -> UiNode {
        UiNode::Select(UiSelectNode {
            id: id.into(),
            value: value.into(),
            items: vec![UiSelectItem { value: "a".into(), label: Label::data("Alpha") }, UiSelectItem { value: "b".into(), label: Label::data("Beta") }],
            placeholder: None,
            on_change: action(),
            presence: UiPresence::default(),
            menu: None,
        })
    }

    #[test]
    fn painting_an_open_select_popup_emits_more_instances_than_a_closed_one_and_highlights_the_value() {
        let (mut tree, root, theme, mut atlas) = setup(&select("sel", "b"));
        let mut closed_draw = DrawList::default();
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut closed_draw);
        let closed_total: usize = closed_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        tree.node_mut(root).unwrap().state.open = true;
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let mut open_draw = DrawList::default();
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut open_draw);
        let open_total: usize = open_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        assert!(open_total > closed_total, "an open Select should paint its popup rows in addition to the closed trigger");
        // "Beta" (value "b") is the current value — its row should paint a `row_hover`-colored
        // highlight rect (a KIND_ROUNDED instance in exactly `theme.row_hover`).
        let has_selected_highlight = open_draw
            .layers
            .iter()
            .flat_map(|layer| layer.ui_instances.iter())
            .any(|instance| (instance.color[0] - theme.row_hover.r).abs() < 0.001 && (instance.color[1] - theme.row_hover.g).abs() < 0.001 && (instance.color[2] - theme.row_hover.b).abs() < 0.001);
        assert!(has_selected_highlight, "the popup row matching the Select's current value should paint a row_hover highlight");
    }

    #[test]
    fn opening_a_selects_popup_gives_its_synthesized_item_rows_real_hit_testable_layout() {
        let (mut tree, root, theme, mut atlas) = setup(&select("sel", "a"));
        tree.node_mut(root).unwrap().state.open = true;
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let row_a = find_child_by_key(&tree, root, &NodeKey::Explicit("a".into())).expect("reconcile should have synthesized a retained row for item \"a\"");
        let row_b = find_child_by_key(&tree, root, &NodeKey::Explicit("b".into())).expect("reconcile should have synthesized a retained row for item \"b\"");
        let bucket_a = &tree.node(row_a).unwrap().layout;
        let bucket_b = &tree.node(row_b).unwrap().layout;
        assert!(bucket_a.width > 0.0 && bucket_a.height > 0.0, "an open Select's row should get real (non-zero) layout so events::hit_test can find it");
        assert!(bucket_b.y > bucket_a.y, "row \"b\" should be laid out below row \"a\"");
    }

    fn drop_stack(drop_action: Option<ActionDescriptor>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("dz".into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children: vec![text("child")], menu: None })
    }

    #[test]
    fn a_stacks_drop_target_flag_tracks_its_drop_action() {
        let (mut tree, root, theme, mut atlas) = setup(&drop_stack(Some(action())));
        let mut draw = DrawList::default();
        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack with a drop_action should be flagged NodeFlags::DROP_TARGET");

        let (mut plain_tree, plain_root, plain_theme, mut plain_atlas) = setup(&drop_stack(None));
        let mut plain_draw = DrawList::default();
        paint_tree(&mut plain_tree, plain_root, &plain_theme, &mut plain_atlas, None, false, &mut plain_draw);
        assert!(!plain_tree.node(plain_root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack without a drop_action must not be flagged DROP_TARGET");
    }

    fn activatable_stack(selected: bool) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some("card".into()),
            presence: UiPresence::selected(selected),
            activate: Some(action()),
            drop_action: None,
            drop_overlay: None,
            children: vec![text("child")],
            menu: None,
        })
    }

    #[test]
    fn an_activatable_stack_paints_a_frame_and_a_selected_one_paints_an_extra_ring() {
        let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&stack(vec![text("child")]));
        let mut bare_draw = DrawList::default();
        paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);
        let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        let (mut card_tree, card_root, theme2, mut card_atlas) = setup(&activatable_stack(false));
        let mut card_draw = DrawList::default();
        paint_tree(&mut card_tree, card_root, &theme2, &mut card_atlas, None, false, &mut card_draw);
        let card_total: usize = card_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(card_total > bare_total, "an activatable Stack should paint a bg+border frame a bare Stack doesn't");

        let (mut selected_tree, selected_root, theme3, mut selected_atlas) = setup(&activatable_stack(true));
        let mut selected_draw = DrawList::default();
        paint_tree(&mut selected_tree, selected_root, &theme3, &mut selected_atlas, None, false, &mut selected_draw);
        let selected_total: usize = selected_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
        assert!(selected_total > card_total, "a selected activatable Stack should paint an extra ring border beyond the plain activate frame");
    }

    fn tree_with_draggable_item() -> UiNode {
        let mut item = UiTreeItemNode::base("i1", Label::data("Item One"));
        item.draggable = Some(true);
        UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        })
    }

    #[test]
    fn a_trees_draggable_item_gets_real_row_layout_and_the_drag_source_flag() {
        let (mut tree, root, theme, mut atlas) = setup(&tree_with_draggable_item());
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        let section = find_child_by_key(&tree, root, &NodeKey::Explicit("s1".into())).expect("reconcile should have synthesized a retained row for section \"s1\"");
        let row = find_child_by_key(&tree, section, &NodeKey::Explicit("i1".into())).expect("reconcile should have synthesized a retained row for item \"i1\"");
        let bucket = &tree.node(row).unwrap().layout;
        assert!(bucket.width > 0.0 && bucket.height > 0.0, "a Tree row should get real (non-zero) layout so events::hit_test can find it");
        assert!(tree.node(row).unwrap().flags.contains(NodeFlags::DRAG_SOURCE), "a draggable Tree item's row should be flagged NodeFlags::DRAG_SOURCE");
    }
    //#endregion 🔖️W2InteractivityFixes

    //#region 🔖️W2WidgetVisuals
    // 🖱️✍️🎯️ Tests for `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 widget-visuals pass: a
    // focused `Input`'s caret/selection-highlight (`paint_input`, sourced from `tree::EditState`),
    // and the `formControlFocusBorderClass`-matching focus ring ported onto every remaining
    // focusable control kind (`Button`/`Select`/`Toggle`/`NumberStepper`/`IconSelect`, plus a
    // `NumberStepper` hover tint) that only `paint_input` had before this pass.
    fn input(id: &str, value: &str) -> UiNode {
        UiNode::Input(UiInputNode { id: id.into(), input_kind: "text".into(), value: value.into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: action(), presence: UiPresence::default(), menu: None })
    }

    fn focus(tree: &mut UiTree, id: NodeId) {
        tree.node_mut(id).unwrap().flags.set(NodeFlags::FOCUSED, true);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    fn has_solid_instance_colored(draw: &DrawList, color: Rgba) -> bool {
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| {
            (instance.params[2] - KIND_SOLID).abs() < 0.01 && (instance.color[0] - color.r).abs() < 0.001 && (instance.color[1] - color.g).abs() < 0.001 && (instance.color[2] - color.b).abs() < 0.001 && (instance.color[3] - color.a).abs() < 0.001
        })
    }

    #[test]
    fn painting_an_unfocused_input_emits_no_caret_or_selection() {
        let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
        tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        assert!(!has_solid_instance_colored(&draw, theme.accent), "an unfocused Input must not paint a caret even if a stale EditState lingers on it");
        assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "an unfocused Input must not paint a selection highlight");
    }

    #[test]
    fn painting_a_focused_input_with_a_collapsed_selection_emits_a_caret_line_but_no_highlight() {
        let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
        focus(&mut tree, root);
        tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 });
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        assert!(has_solid_instance_colored(&draw, theme.accent), "a focused Input should paint its caret as a theme.accent solid instance");
        assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a collapsed selection (anchor == caret) must not paint a highlight rect");
    }

    #[test]
    fn painting_a_focused_input_with_a_real_selection_emits_a_translucent_highlight() {
        let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello world"));
        focus(&mut tree, root);
        tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello world".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        assert!(has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a real anchor..caret selection should paint a theme.accent-at-0.3-alpha highlight rect");
    }

    #[test]
    fn painting_a_focused_input_shows_the_live_edit_buffer_text_not_the_stale_declarative_value() {
        let (mut tree, root, theme, mut atlas) = setup(&input("in", "old"));
        focus(&mut tree, root);
        tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "a much longer buffer".into(), caret: 20, anchor: 20, composition: None, scroll_x: 0.0 });
        let mut draw = DrawList::default();

        let (mut stale_tree, stale_root, stale_theme, mut stale_atlas) = setup(&input("in", "old"));
        let mut stale_draw = DrawList::default();
        paint_tree(&mut stale_tree, stale_root, &stale_theme, &mut stale_atlas, None, false, &mut stale_draw);
        let stale_total: usize = stale_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
        let focused_total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

        assert!(focused_total > stale_total, "a focused Input with a live EditState should paint its live (longer) buffer text, not the stale shorter declarative value");
    }

    fn toggle(id: &str) -> UiNode {
        UiNode::Toggle(UiToggleNode { id: id.into(), icon_id: IconName::CircleDot, text: Some(Label::data("Toggle")), on_change: action(), presence: UiPresence::default(), menu: None })
    }

    fn icon_select(id: &str) -> UiNode {
        UiNode::IconSelect(UiIconSelectNode { id: id.into(), value: "star".into(), uniform: true, classifier_kind: "generic".into(), on_change: action(), presence: UiPresence::default(), menu: None })
    }

    /// 🎯️ Shared assertion for the border-swap-on-focus fix: an otherwise-identical pair of trees,
    /// one with `NodeFlags::FOCUSED` set on the root, should differ in at least one border instance's
    /// color (`theme.border_emphasized` replacing `theme.border_normal`) — mirrors
    /// `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`).
    fn assert_focus_swaps_border_color(make: impl Fn() -> UiNode, label: &str) {
        let (mut unfocused_tree, unfocused_root, theme, mut unfocused_atlas) = setup(&make());
        let mut unfocused_draw = DrawList::default();
        paint_tree(&mut unfocused_tree, unfocused_root, &theme, &mut unfocused_atlas, None, false, &mut unfocused_draw);

        let (mut focused_tree, focused_root, focused_theme, mut focused_atlas) = setup(&make());
        focus(&mut focused_tree, focused_root);
        let mut focused_draw = DrawList::default();
        paint_tree(&mut focused_tree, focused_root, &focused_theme, &mut focused_atlas, None, false, &mut focused_draw);

        assert!(!has_solid_instance_colored(&unfocused_draw, theme.border_emphasized), "{label}: an unfocused control must not paint its border_emphasized color");
        assert!(has_solid_instance_colored(&focused_draw, theme.border_emphasized), "{label}: a focused control should swap its border to theme.border_emphasized");
    }

    #[test]
    fn painting_a_focused_button_swaps_its_border_to_border_emphasized() {
        assert_focus_swaps_border_color(|| button("btn", false), "Button");
    }

    #[test]
    fn painting_a_focused_select_swaps_its_border_to_border_emphasized() {
        assert_focus_swaps_border_color(|| select("sel", "a"), "Select");
    }

    #[test]
    fn painting_a_focused_toggle_swaps_its_border_to_border_emphasized() {
        assert_focus_swaps_border_color(|| toggle("tog"), "Toggle");
    }

    #[test]
    fn painting_a_focused_number_stepper_swaps_its_outer_border_to_border_emphasized() {
        assert_focus_swaps_border_color(|| stepper("ns", 2.0, true), "NumberStepper");
    }

    #[test]
    fn painting_a_focused_icon_select_swaps_its_border_to_border_emphasized() {
        assert_focus_swaps_border_color(|| icon_select("ic"), "IconSelect");
    }

    #[test]
    fn painting_a_hovered_number_stepper_tints_its_outer_background() {
        let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
        tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true);
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let mut draw = DrawList::default();

        paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

        assert!(has_solid_instance_colored(&draw, theme.button_hover), "a hovered NumberStepper should tint its shared minus/plus background to theme.button_hover");
    }
    //#endregion 🔖️W2WidgetVisuals

    //#region 🧵️RetainedPaintLaws
    fn retained_glyph_count(draw: &DrawList) -> usize {
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).count()
    }

    #[test]
    fn retained_text_paint_emits_at_most_one_glyph_per_grant() {
        let (tree, root, theme, mut atlas) = setup(&text("ab"));
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        let before = retained_glyph_count(&draw);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(retained_glyph_count(&draw), before + 1);
        assert_eq!(cursor.glyph.byte(), 1);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(retained_glyph_count(&draw), before + 2);
        assert_eq!(cursor.glyph.byte(), 2);
    }

    #[test]
    fn retained_multi_megabyte_input_advances_one_scalar_per_grant() {
        let value = "x".repeat(2 * 1_024 * 1_024);
        let (tree, root, theme, mut atlas) = setup(&input("huge-input", &value));
        let identity = match tree.node(root).map(|node| &node.spec.0) {
            Some(UiNode::Input(input)) => input.value.as_ptr(),
            _ => panic!("retained input owner"),
        };
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        let before = retained_glyph_count(&draw);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(retained_glyph_count(&draw), before + 1);
        assert_eq!(cursor.glyph.byte(), 1);
        assert_eq!(
            tree.node(root).and_then(|node| match &node.spec.0 {
                UiNode::Input(input) => Some(input.value.as_ptr()),
                _ => None,
            }),
            Some(identity)
        );
    }

    #[test]
    fn retained_select_max_plus_one_refuses_before_output_without_moving_tree_owner() {
        let mut authored = select("select-max-plus-one", "0");
        let UiNode::Select(select) = &mut authored else { panic!("select fixture") };
        select.items = (0..=RETAINED_NODE_COLLECTION_ITEMS).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect();
        let (tree, root, theme, mut atlas) = setup(&authored);
        let identity = match tree.node(root).map(|node| &node.spec.0) {
            Some(UiNode::Select(select)) => select.items.as_ptr(),
            _ => panic!("retained select owner"),
        };
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Fault);
        assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len()).sum::<usize>(), 0);
        assert_eq!(
            tree.node(root).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => Some(select.items.as_ptr()),
                _ => None,
            }),
            Some(identity)
        );
    }

    #[test]
    fn retained_select_sync_writes_at_most_one_row_per_grant() {
        let (mut tree, root, theme, _) = setup(&select("retained-sync-select", "a"));
        let Some(root_node) = tree.node_mut(root) else { panic!("retained select root") };
        root_node.state.open = true;
        let children: Vec<NodeId> = tree.children(root).collect();
        let mut cursor = RetainedInteractiveSyncCursor::default();
        let mut complete = false;
        for _ in 0..64 {
            let before: Vec<(f32, f32, f32, f32)> = children.iter().filter_map(|child| tree.node(*child).map(|node| (node.layout.x, node.layout.y, node.layout.width, node.layout.height))).collect();
            let step = sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor);
            let changed = children.iter().zip(before.iter()).filter(|(child, before)| tree.node(**child).is_some_and(|node| (node.layout.x, node.layout.y, node.layout.width, node.layout.height) != **before)).count();
            assert!(changed <= 1);
            if step == RetainedInteractiveSyncStep::Complete {
                complete = true;
                break;
            }
            assert_ne!(step, RetainedInteractiveSyncStep::Fault);
        }
        assert!(complete);
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn retained_select_sync_max_plus_one_fault_closes_exact_cursor_owner() {
        let mut authored = select("retained-sync-max-plus-one", "0");
        let UiNode::Select(select) = &mut authored else { panic!("retained select fixture") };
        select.items = (0..=RETAINED_SYNC_COLLECTION_ITEMS).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect();
        let (mut tree, root, theme, _) = setup(&authored);
        let Some(root_node) = tree.node_mut(root) else { panic!("retained select root") };
        root_node.state.open = true;
        let identity = match tree.node(root).map(|node| &node.spec.0) {
            Some(UiNode::Select(select)) => select.items.as_ptr(),
            _ => panic!("retained select owner"),
        };
        let mut cursor = RetainedInteractiveSyncCursor::default();
        assert_eq!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Pending);
        assert_eq!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Fault);
        assert!(!cursor.close_step());
        assert!(cursor.close_step());
        assert!(cursor.terminal_is_empty());
        assert_eq!(
            tree.node(root).and_then(|node| match &node.spec.0 {
                UiNode::Select(select) => Some(select.items.as_ptr()),
                _ => None,
            }),
            Some(identity)
        );
    }

    #[test]
    fn retained_tree_sync_abandonment_releases_one_record_or_depth_owner_per_grant() {
        let mut nested = UiTreeItemNode::base("nested", Label::data("Nested"));
        nested.default_open = Some(true);
        nested.items = Some(vec![UiTreeItemNode::base("leaf", Label::data("Leaf"))]);
        let authored = UiNode::Tree(UiTreeNode {
            sections: vec![UiTreeSectionNode { id: "section".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }],
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        });
        let (mut tree, root, theme, _) = setup(&authored);
        let mut cursor = RetainedInteractiveSyncCursor::default();
        for _ in 0..32 {
            assert_ne!(sync_interactive_state_node_step(&mut tree, root, &theme, &mut cursor), RetainedInteractiveSyncStep::Fault);
            if cursor.tree_record_len >= 3 {
                break;
            }
        }
        assert!(cursor.tree_record_len >= 3);
        while !cursor.terminal_is_empty() {
            let records = cursor.tree_record_len;
            let frames = cursor.tree_frames.iter().flatten().count();
            assert!(!cursor.close_step());
            let closed = records.saturating_sub(cursor.tree_record_len) + frames.saturating_sub(cursor.tree_frames.iter().flatten().count());
            assert!(closed <= 1);
        }
        assert!(cursor.close_step());
    }

    #[test]
    fn retained_text_multi_megabyte_max_plus_one_preserves_tree_owner_identity() {
        let authored = text(&"x".repeat(RETAINED_NODE_TEXT_MAX_BYTES + 1));
        let mut tree = UiTree::new();
        tree.apply_tree(&authored);
        let Some(root) = tree.root else { panic!("retained text root") };
        let identity = match tree.node(root).map(|node| &node.spec.0) {
            Some(UiNode::Text(text)) => text.value.as_str().as_ptr(),
            _ => panic!("retained text owner"),
        };
        let theme = Theme::default();
        let mut atlas = FontAtlas::builtin();
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Fault);
        let after = match tree.node(root).map(|node| &node.spec.0) {
            Some(UiNode::Text(text)) => text.value.as_str().as_ptr(),
            _ => panic!("retained text owner"),
        };
        assert_eq!(after, identity);
        assert_eq!(retained_glyph_count(&draw), 0);
        assert!(!cursor.terminal_is_empty());
    }

    #[test]
    fn retained_text_cancel_close_preserves_exact_terminal_witness() {
        let (tree, root, theme, mut atlas) = setup(&text("cancel"));
        let mut draw = DrawList::default();
        let mut cursor = RetainedNodePaintCursor::default();
        assert_eq!(paint_node_step(&tree, root, 0.0, 0.0, &theme, &mut atlas, None, false, &mut draw, &mut cursor), RetainedNodePaintStep::Pending);
        assert!(!cursor.terminal_is_empty());
        assert!(!cursor.close_step());
        assert!(cursor.terminal_is_empty());
        assert!(cursor.close_step());
    }
    //#endregion 🧵️RetainedPaintLaws
}
// #endregion paint
