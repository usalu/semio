// #region layout
//! 🧮️ Flex stack layout for widget trees.

use crate::wgpu::component::ui::{UiControlNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::geometry::Rect;
use crate::wgpu::theme::Theme;
use ui_contract::SpaceToken;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

/// 📐️ The [`SpaceToken`] a legacy declarative `UiStackNode.gap` string names. The declarative
/// `UiNode` path is in-crate chrome with no React counterpart (plugin content arrives as
/// `UiNodeRecord`s carrying a real `LayoutSpec`), so these three names are the whole vocabulary —
/// they are mapped ONTO the shared ramp here instead of carrying private pixel literals, which is
/// what let `gap`'s `tight` (4px) and `padding`'s `tight` (6px) drift apart before.
pub fn gap_space_token(token: Option<&str>) -> SpaceToken {
    match token {
        Some("loose") => SpaceToken::Md,
        Some("tight") => SpaceToken::Xs,
        named => named_space_token(named).unwrap_or(SpaceToken::Xs),
    }
}

/// 📐️ A [`SpaceToken`]'s own wire name, the vocabulary `reconcile::space_token` stamps onto a
/// record-mounted node's legacy strings so that channel stays lossless for diffing.
pub fn named_space_token(token: Option<&str>) -> Option<SpaceToken> {
    match token? {
        "none" | "0" => Some(SpaceToken::None),
        "xs" => Some(SpaceToken::Xs),
        "sm" => Some(SpaceToken::Sm),
        "md" => Some(SpaceToken::Md),
        "lg" => Some(SpaceToken::Lg),
        "xl" => Some(SpaceToken::Xl),
        "xxl" => Some(SpaceToken::Xxl),
        _ => None,
    }
}

/// 📐️ The inverse of [`named_space_token`].
pub fn space_token_name(token: SpaceToken) -> &'static str {
    match token {
        SpaceToken::None => "none",
        SpaceToken::Xs => "xs",
        SpaceToken::Sm => "sm",
        SpaceToken::Md => "md",
        SpaceToken::Lg => "lg",
        SpaceToken::Xl => "xl",
        SpaceToken::Xxl => "xxl",
    }
}

/// 📐️ The [`SpaceToken`] a legacy declarative `UiStackNode.padding` string names — one step looser
/// than [`gap_space_token`] at `tight`/`loose`, the same relation the two private literal tables had.
pub fn padding_space_token(token: Option<&str>) -> SpaceToken {
    match token {
        Some("tight") => SpaceToken::Sm,
        Some("loose") => SpaceToken::Lg,
        named => named_space_token(named).unwrap_or(SpaceToken::Xs),
    }
}

/// 📐️ A declarative gap string in px. `None`/`"standard"` stay theme-resolved (the theme's own
/// `--ui-spacing` step); every named token resolves through [`SpaceToken::px`], the ONE ramp React's
/// `spaceTokenRem` uses.
pub fn gap_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        None | Some("standard") => theme.gap_standard,
        named => gap_space_token(named).px(),
    }
}

pub fn padding_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        None | Some("standard") => theme.padding_standard,
        named => padding_space_token(named).px(),
    }
}

pub fn layout_vertical(bounds: Rect, gap: f32, padding: f32, child_heights: &[f32]) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_heights.len().saturating_sub(1) as f32);
    let total_children: f32 = child_heights.iter().sum();
    let mut y = inner.y;
    let mut rects = Vec::with_capacity(child_heights.len());
    let available = (inner.h - total_gap - total_children).max(0.0);
    let extra_per_child = if child_heights.is_empty() { 0.0 } else { available / child_heights.len() as f32 };
    for &height in child_heights {
        let h = height + extra_per_child;
        rects.push(Rect::new(inner.x, y, inner.w, h));
        y += h + gap;
    }
    rects
}

pub fn layout_horizontal(bounds: Rect, gap: f32, padding: f32, child_widths: &[f32]) -> Vec<Rect> {
    let inner = bounds.inset(padding);
    let total_gap = gap * (child_widths.len().saturating_sub(1) as f32);
    let total_children: f32 = child_widths.iter().sum();
    let mut x = inner.x;
    let mut rects = Vec::with_capacity(child_widths.len());
    let available = (inner.w - total_gap - total_children).max(0.0);
    let extra_per_child = if child_widths.is_empty() { 0.0 } else { available / child_widths.len() as f32 };
    for &width in child_widths {
        let w = width + extra_per_child;
        rects.push(Rect::new(x, inner.y, w, inner.h));
        x += w + gap;
    }
    rects
}

//#region 🌳️TreeRowGeometry
/// 🌳️ The ONE tree row geometry every consumer of this target derives from: `mounted_layout`
/// (which publishes the rects `events::hit_test` reads), `paint`'s retained tree walk, and the
/// interactive sync. Before this existed the layout measured a tree row as a bare `Stack` with no
/// arena children — `padding_standard * 2 = 6.4 px` — while the painter stepped its own private
/// cursor by a hardcoded `24.0`, so the rectangles a pointer hit and the rows a reader saw were
/// different geometry (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-tree-row-hit-test-2026-09-12.md`).
///
/// `row_height` is `dom.treeRowUiSpacing` off the theme — the same `--size-workbench` React's
/// `Tree` rows carry as `h-workbench` (`🧱️elements/🌳️Tree/🟦️.tsx`'s `treeRowHeightPx`), so a
/// section header row, an item row and a React row are one number, never three literals.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TreeRowMetrics {
    pub row_height: f32,
    pub header_height: f32,
    pub control_width: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub gap: f32,
    pub drag_handle_extent: f32,
    pub inline: ui_contract::FlowInline,
    standard_row_height: f32,
    standard_control_width: f32,
    standard_control_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeDragRole {
    Sort,
    Transfer,
}

/// 🌳️ React's property-row value column: `controlValueColumnUiSpacing × --ui-spacing`.
pub const TREE_ROW_CONTROL_WIDTH: f32 = (ui_styling::metrics::dom::CONTROL_VALUE_COLUMN_UI_SPACING * ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX) as f32;

/// 🌳️ React's compact property-tree value column, measured from the actual mounted DOM.
pub const TREE_ROW_COMPACT_CONTROL_WIDTH: f32 = 104.0;

/// 🌳️ Depth past which a tree's own spec is refused rather than recursed — the same ceiling
/// `paint::RETAINED_TREE_DEPTH` walks with, so neither side can out-recurse the other.
pub const TREE_ROW_MAX_DEPTH: usize = 64;

impl TreeRowMetrics {
    pub fn from_theme(theme: &Theme) -> Self {
        Self::for_presentation(theme, ui_contract::TreePresentation::Standard)
    }

    pub fn for_presentation(theme: &Theme, presentation: ui_contract::TreePresentation) -> Self {
        let mut metrics = Self {
            row_height: theme.tree_row_height,
            header_height: theme.tree_row_height,
            control_width: TREE_ROW_CONTROL_WIDTH,
            control_height: theme.control_height,
            control_height_small: theme.control_height_small,
            gap: theme.gap_standard,
            drag_handle_extent: crate::wgpu::chrome::ICON_TREE_ROW,
            inline: ui_contract::FlowInline::Ltr,
            standard_row_height: theme.tree_row_height,
            standard_control_width: TREE_ROW_CONTROL_WIDTH,
            standard_control_height: theme.control_height,
        };
        metrics = metrics.with_presentation(presentation);
        metrics
    }

    pub fn with_presentation(mut self, presentation: ui_contract::TreePresentation) -> Self {
        self.row_height = match presentation {
            ui_contract::TreePresentation::Standard => self.standard_row_height,
            ui_contract::TreePresentation::Compact => crate::wgpu::chrome::SIZE_TINY * 1.5,
        };
        self.header_height = self.row_height;
        self.control_width = match presentation {
            ui_contract::TreePresentation::Standard => self.standard_control_width,
            ui_contract::TreePresentation::Compact => TREE_ROW_COMPACT_CONTROL_WIDTH,
        };
        self.control_height = self.standard_control_height;
        self
    }

    pub fn with_inline(mut self, inline: ui_contract::FlowInline) -> Self {
        self.inline = inline;
        self
    }

    pub fn for_item(mut self, item: &UiTreeItemNode) -> Self {
        self.control_height = match item.control.as_ref() {
            Some(UiControlNode::Input(_) | UiControlNode::Select(_)) => self.control_height_small,
            Some(_) => self.standard_control_height,
            None => 0.0,
        };
        self.row_height = self.row_height.max(self.control_height);
        self
    }
}

pub fn tree_drag_role(item: &UiTreeItemNode) -> Option<TreeDragRole> {
    item.draggable.unwrap_or(false).then_some(if item.drag_data.is_some() { TreeDragRole::Transfer } else { TreeDragRole::Sort })
}

pub fn tree_drag_handle_rect(row_width: f32, metrics: &TreeRowMetrics) -> Rect {
    let extent = metrics.drag_handle_extent.min(row_width.max(0.0));
    let x = if metrics.inline.is_rtl() { metrics.gap } else { (row_width - metrics.gap - extent).max(0.0) };
    Rect::new(x, (metrics.row_height - metrics.drag_handle_extent) * 0.5, extent, metrics.drag_handle_extent.min(metrics.row_height.max(0.0)))
}

pub fn tree_drag_handle_reservation(metrics: &TreeRowMetrics) -> f32 {
    metrics.drag_handle_extent + metrics.gap * 2.0
}

/// 🌳️ A tree item row's own height: its row plus every expanded, visible nested row beneath it —
/// exactly what `paint::retained_tree_node_step` advances its `row_y` cursor by for that item.
pub fn tree_item_height(item: &UiTreeItemNode, metrics: &TreeRowMetrics) -> f32 {
    tree_item_height_at(item, metrics, 0)
}

fn tree_item_height_at(item: &UiTreeItemNode, metrics: &TreeRowMetrics, depth: usize) -> f32 {
    if !item.presence.visible() {
        return 0.0;
    }
    let mut height = metrics.for_item(item).row_height;
    if depth >= TREE_ROW_MAX_DEPTH || !item.default_open.unwrap_or(false) {
        return height;
    }
    for nested in item.items.iter().flatten() {
        height += tree_item_height_at(nested, metrics, depth + 1);
    }
    height
}

/// 🌳️ A tree section row's own height: its header row (only when it is labelled, matching the
/// painter's own `section.label.is_some()` gate) plus every visible item row.
pub fn tree_section_height(section: &UiTreeSectionNode, metrics: &TreeRowMetrics) -> f32 {
    tree_section_height_with_open(section, metrics, section.default_open.unwrap_or(true))
}

pub fn tree_section_height_with_open(section: &UiTreeSectionNode, metrics: &TreeRowMetrics, open: bool) -> f32 {
    if !section.presence.visible() {
        return 0.0;
    }
    let mut height = tree_section_header_height(section, metrics);
    if open {
        for item in &section.items {
            height += tree_item_height_at(item, metrics, 0);
        }
    }
    height
}

/// 🌳️ A tree section's header band — `row_height` when the section is labelled, `0` when it is not.
pub fn tree_section_header_height(section: &UiTreeSectionNode, metrics: &TreeRowMetrics) -> f32 {
    if section.label.is_some() {
        metrics.header_height
    } else {
        0.0
    }
}

/// 🌳️ The section header band at the edge its block flow paints.
pub fn tree_section_header_band(rect: Rect, header_height: f32, reversed: bool) -> Rect {
    let header_height = header_height.min(rect.h);
    Rect::new(rect.x, if reversed { rect.y + rect.h - header_height } else { rect.y }, rect.w, header_height)
}

pub fn tree_item_chevron_rect(rect: Rect, depth: usize, metrics: &TreeRowMetrics, reversed: bool) -> Rect {
    let row = tree_section_header_band(rect, metrics.row_height, reversed);
    let indent = (ui_styling::metrics::dom::TREE_INDENT_PER_LEVEL_UI_SPACING * ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX) as f32;
    let width = (ui_styling::metrics::dom::TREE_TOGGLE_UI_SPACING * ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX) as f32;
    let offset = depth.saturating_sub(1) as f32 * indent;
    let x = if metrics.inline.is_rtl() { row.x + (row.w - width - offset).max(0.0) } else { row.x + offset };
    Rect::new(x, row.y, width.min(row.w), row.h)
}

/// 🌳️ A whole `Tree` node's height: the sum of its visible sections.
pub fn tree_node_height(node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    node.sections.iter().map(|section| tree_section_height(section, metrics)).sum()
}

/// 🌳️ The rect a row's inline control occupies, **relative to that row's own top-left** — the one
/// definition `paint`'s control draw and `mounted_layout`'s arrange both take it from.
pub fn tree_row_control_rect(row_width: f32, metrics: &TreeRowMetrics) -> Rect {
    tree_row_control_rect_with_height(row_width, metrics.control_height, metrics)
}

/// 🎛️ The value-column rect for a control's own authored height. General's Select/Input are small;
/// Stepper/Toggle/Button retain the default chrome height while sharing the same 160px column.
pub fn tree_row_control_rect_with_height(row_width: f32, control_height: f32, metrics: &TreeRowMetrics) -> Rect {
    let x = if metrics.inline.is_rtl() { metrics.gap } else { (row_width - metrics.control_width - metrics.gap).max(0.0) };
    Rect::new(x, (metrics.row_height - control_height) * 0.5, metrics.control_width.min((row_width - metrics.gap).max(0.0)), control_height)
}

/// ☑️ React's compact tree checkbox wrapper is one `size-tiny` column at the inline edge of the
/// value column, while retaining the whole row height as its vertical pointer band.
pub fn tree_checkbox_hit_rect(bounds: Rect, inline: ui_contract::FlowInline) -> Rect {
    let width = crate::wgpu::chrome::SIZE_TINY.min(bounds.w.max(0.0));
    let x = if inline.is_rtl() { bounds.x + bounds.w - width } else { bounds.x };
    Rect::new(x, bounds.y, width, bounds.h)
}

pub fn tree_inline_control_height(control: &UiControlNode, theme: &Theme) -> f32 {
    match control {
        UiControlNode::Input(_) | UiControlNode::Select(_) => theme.control_height_small,
        UiControlNode::Toggle(_)
        | UiControlNode::Button(_)
        | UiControlNode::KeyValue(_)
        | UiControlNode::Slider(_)
        | UiControlNode::NumberStepper(_)
        | UiControlNode::Ring(_)
        | UiControlNode::IconSelect(_) => theme.control_height,
    }
}

pub fn tree_row_control_rect_before_drag(row_width: f32, metrics: &TreeRowMetrics) -> Rect {
    let reservation = tree_drag_handle_reservation(metrics);
    let x = if metrics.inline.is_rtl() { metrics.gap + reservation } else { (row_width - metrics.control_width - metrics.gap - reservation).max(0.0) };
    Rect::new(x, (metrics.row_height - metrics.control_height) * 0.5, metrics.control_width.min((row_width - reservation).max(0.0)), metrics.control_height)
}
//#endregion 🌳️TreeRowGeometry

//#region 🎛️ControlGeometry
/// ➖️🔢️➕️ A `NumberStepper`'s three segments — decrement, value, increment — as thirds of its own
/// box. ONE definition: `paint`'s stepper chrome draws these rects and `events`' press routing
/// hit-tests the same three, so the `−` a user sees and the `−` a press resolves can never drift.
pub fn number_stepper_segments(bounds: Rect) -> [Rect; 3] {
    let segment = bounds.w / 3.0;
    [Rect::new(bounds.x, bounds.y, segment, bounds.h), Rect::new(bounds.x + segment, bounds.y, segment, bounds.h), Rect::new(bounds.x + segment * 2.0, bounds.y, segment, bounds.h)]
}

/// 🎚️ The value a `Slider` press/drag at `x` reports, on the same full-width track
/// `paint`'s slider arm draws (`bounds.x + bounds.w * t`), snapped onto `step` and clamped into
/// `min..=max` — Radix's own `Slider` semantics, which React's `SliderView` delegates to.
pub fn slider_value_at(bounds: Rect, x: f32, min: f64, max: f64, step: f64) -> f64 {
    let span = max - min;
    if !span.is_finite() || span <= 0.0 {
        return min;
    }
    let ratio = if bounds.w > 0.0 { f64::from((x - bounds.x) / bounds.w).clamp(0.0, 1.0) } else { 0.0 };
    let raw = min + ratio * span;
    let snapped = if step.is_finite() && step > 0.0 { min + ((raw - min) / step).round() * step } else { raw };
    snapped.clamp(min, max)
}

/// 💍️ The normalised `t` a `Ring` press/drag at `(x, y)` reports, on the same circle `paint`'s ring
/// arm draws its knob at (`angle = TAU * t`, centre of `bounds`, radius `min(w, h) * 0.4`).
pub fn ring_t_at(bounds: Rect, x: f32, y: f32) -> f64 {
    let angle = f64::from(y - (bounds.y + bounds.h * 0.5)).atan2(f64::from(x - (bounds.x + bounds.w * 0.5)));
    let turns = angle / std::f64::consts::TAU;
    turns - turns.floor()
}
//#endregion 🎛️ControlGeometry

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-layout-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🌳️tree-row-rects/🦀️.rs"]
mod tree_row_rect_tests;
// #endregion layout
