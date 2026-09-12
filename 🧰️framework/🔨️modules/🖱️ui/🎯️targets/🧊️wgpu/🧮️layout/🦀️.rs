// #region layout
//! 🧮️ Flex stack layout for widget trees.

use crate::wgpu::component::ui::{UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::geometry::Rect;
use crate::wgpu::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub fn gap_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("tight") => 4.0,
        Some("loose") => 12.0,
        Some("none") | Some("0") => 0.0,
        _ => theme.gap_standard,
    }
}

pub fn padding_for_token(theme: &Theme, token: Option<&str>) -> f32 {
    match token {
        Some("none") | Some("0") => 0.0,
        Some("tight") => 6.0,
        Some("loose") => 16.0,
        _ => theme.padding_standard,
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
    pub gap: f32,
}

/// 🌳️ Width of the inline control a tree row may carry, in row-relative px.
pub const TREE_ROW_CONTROL_WIDTH: f32 = 120.0;

/// 🌳️ Depth past which a tree's own spec is refused rather than recursed — the same ceiling
/// `paint::RETAINED_TREE_DEPTH` walks with, so neither side can out-recurse the other.
pub const TREE_ROW_MAX_DEPTH: usize = 64;

impl TreeRowMetrics {
    pub fn from_theme(theme: &Theme) -> Self {
        Self { row_height: theme.tree_row_height, header_height: theme.tree_row_height, control_width: TREE_ROW_CONTROL_WIDTH, control_height: theme.control_height, gap: theme.gap_standard }
    }
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
    let mut height = metrics.row_height;
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
    if !section.presence.visible() {
        return 0.0;
    }
    let mut height = tree_section_header_height(section, metrics);
    for item in &section.items {
        height += tree_item_height_at(item, metrics, 0);
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

/// 🌳️ A whole `Tree` node's height: the sum of its visible sections.
pub fn tree_node_height(node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    node.sections.iter().map(|section| tree_section_height(section, metrics)).sum()
}

/// 🌳️ The rect a row's inline control occupies, **relative to that row's own top-left** — the one
/// definition `paint`'s control draw and `mounted_layout`'s arrange both take it from.
pub fn tree_row_control_rect(row_width: f32, metrics: &TreeRowMetrics) -> Rect {
    Rect::new((row_width - metrics.control_width - metrics.gap).max(0.0), (metrics.row_height - metrics.control_height) * 0.5, metrics.control_width, metrics.control_height)
}
//#endregion 🌳️TreeRowGeometry

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-layout-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🌳️tree-row-rects/🦀️.rs"]
mod tree_row_rect_tests;
// #endregion layout
