//! 🔎️ wgpu render functions for the Select element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod select;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls these back in via
//! `use crate::wgpu::select::{render_select, render_select_menu};` so its own unqualified call sites keep
//! working. `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`,
//! `SelectItem`, `draw_text`, `draw_text_on`); `crate::wgpu::chrome`/`crate::wgpu::input`/`crate::wgpu::theme` are the
//! other top-level engine mods `widgets` itself also depends on.
//!
//! 🎹️ Since ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o this module also owns the
//! element's **behaviour** contract — popup geometry (`🔖️Geometry`) and the keyboard/typeahead state
//! machine (`🔖️Keyboard`) — as pure functions both renderers' call sites share: `paint`'s retained
//! popup rows, `events`' key routing, and the immediate-mode kit below all resolve the same numbers
//! from here, so none of the three can drift. Every rule is a direct port of React's own first-party
//! Select (`🧱️elements/🔽️Select/🟦️.tsx`: `resolveSelectPlacement` at :245-265, `SelectTrigger`'s
//! `onKeyDown` at :395-410, `SelectContent`'s at :641-672, `moveActive` at :435-452,
//! `findTypeaheadOption` at :455-466), not of Radix, which this element stopped wrapping.

use crate::wgpu::chrome::push_control_border;
use crate::wgpu::component::layout::MeasureSelectItem;
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::theme::{Level, Theme};
use crate::wgpu::widgets::{draw_text, draw_text_on, SelectItem, WidgetContext};

pub(crate) trait SelectItemView {
    fn value(&self) -> &str;
    fn label(&self) -> &str;
}

impl SelectItemView for SelectItem {
    fn value(&self) -> &str {
        &self.value
    }

    fn label(&self) -> &str {
        &self.label
    }
}

impl SelectItemView for MeasureSelectItem {
    fn value(&self) -> &str {
        &self.value
    }

    fn label(&self) -> &str {
        &self.label
    }
}

pub use semio_framework_ui_select_behavior::*;

pub(crate) fn render_select<E: Clone, T: SelectItemView>(id: &str, value: &str, items: &[T], placeholder: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let open = *ctx.open_selects.get(id).unwrap_or(&false);
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = if hovered { ctx.theme.button_hover } else { ctx.theme.input_bg };
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let label = items.iter().find(|item| item.value() == value).map_or(placeholder.unwrap_or("Select…"), |item| item.label());
    draw_text(ctx, label, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    if let Some(icons) = ctx.icons {
        crate::wgpu::chrome::push_icon(
            ctx.draw,
            icons,
            "chevron-down",
            bounds.x + bounds.w - ctx.theme.padding_standard - crate::wgpu::chrome::ICON_TINY,
            bounds.y + (bounds.h - crate::wgpu::chrome::ICON_TINY) * 0.5,
            crate::wgpu::chrome::ICON_TINY,
            ctx.theme.text_element,
        );
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Select, drag_axis: None, drag_data: None });
    if open {
        render_select_menu(id, value, items, bounds, ctx);
    }
}

/// 🔽️ The open popup. `WidgetContext::viewport_height` is the measured surface the kit flips and
/// clamps against — it used to carry none, so this path passed `0.0` and could only ever place below
/// at full natural height (ticket 26/09/17 packet W2k threaded the viewport through
/// `framework_widget_context`). Everything else — row pitch, insets, hit rects, the clamp, the
/// scroll buttons — comes from `🔖️Geometry` above, shared with the retained path.
pub(crate) fn render_select_menu<E: Clone, T: SelectItemView>(id: &str, value: &str, items: &[T], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let viewport_h = ctx.viewport_height;
    let inset = ctx.theme.padding_standard;
    let font_size = ctx.theme.font_size_body;
    // 🔼️ Resolve this frame's scroll offset BEFORE anything paints: take whatever direction the
    // press handler left pending, price it in `select_scroll_step` pixels against the popup's own
    // viewport, clamp it to what the popup can actually scroll, and write both slots back. A popup
    // that fits clamps straight to `0.0`, so the chevrons stay inert exactly as React's do.
    let pending = ctx.scroll_offsets.remove(&select_scroll_pending_key(id)).unwrap_or(0.0);
    let scroll_key = select_scroll_key(id);
    let stored = ctx.scroll_offsets.get(&scroll_key).copied().unwrap_or(0.0);
    let popup = select_popup_geometry(bounds, items.len(), ctx.theme, viewport_h, stored, pending);
    if popup.scroll == 0.0 {
        ctx.scroll_offsets.remove(&scroll_key);
    } else {
        ctx.scroll_offsets.insert(scroll_key, popup.scroll);
    }
    let mut render_rows = |draw: &mut crate::wgpu::draw::DrawList| {
        let glass = draw.push_glass([popup.menu.x, popup.menu.y, popup.menu.w, popup.menu.h], ctx.theme.border_radius, ctx.theme.glass(Level::Menu));
        draw.begin_glass_content(glass);
        crate::wgpu::chrome::push_chrome_border(draw, popup.menu, ctx.theme.stroke_hairline, ctx.theme.border_normal, true, true, true, true);
        draw.push_scissor(select_popup_viewport_rect(popup));
        for (index, item) in items.iter().enumerate().take(popup.last_row).skip(popup.first_row) {
            let row = select_popup_row_rect(bounds, index, popup, ctx.theme);
            let row_hovered = ctx.input.hit_at(ctx.input.pointer_x, ctx.input.pointer_y).and_then(|h| h.control_id.as_deref()) == Some(&format!("{id}.item.{}", item.value()));
            if row_hovered || item.value() == value {
                draw.push_rounded([row.x, row.y, row.w, row.h], ctx.theme.row_hover, ctx.theme.border_radius);
            }
            draw_text_on(draw, ctx.atlas, item.label(), row.x + inset, row.y + (row.h + font_size) * 0.5 - 2.0, font_size, ctx.theme.text);
            ctx.input.register_hit(HitTarget { rect: select_popup_row_hit_rect(bounds, index, popup, ctx.theme), event: None, control_id: Some(format!("{id}.item.{}", item.value())), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
        draw.pop_scissor();
        // 🔼️ The chevrons paint and register LAST so their bands win the hit resolve (`HitRegistry`
        // resolves the most recently registered target first) over the rows they sit on top of.
        if let (Some(up), Some(down)) = (popup.up, popup.down) {
            let chevron = crate::wgpu::chrome::SIZE_TINY;
            let center_x = popup.menu.x + (popup.menu.w - chevron) * 0.5;
            if let Some(icons) = ctx.icons {
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-up", center_x, up.y + (up.h - chevron) * 0.5, chevron, ctx.theme.text_muted);
                crate::wgpu::chrome::push_icon(draw, icons, "chevron-down", center_x, down.y + (down.h - chevron) * 0.5, chevron, ctx.theme.text_muted);
            }
            ctx.input.register_hit(HitTarget { rect: up, event: None, control_id: Some(select_scroll_control_id(id, true)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
            ctx.input.register_hit(HitTarget { rect: down, event: None, control_id: Some(select_scroll_control_id(id, false)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
        }
        draw.end_glass_content();
    };
    if let Some(overlay) = ctx.overlay.as_deref_mut() {
        render_rows(overlay);
    } else {
        render_rows(ctx.draw);
    }
    if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
        maps.register_select_popup_wheel(id, popup.menu, popup.max_scroll, ctx.input.staged_hits().len());
    }
}

/// 🔼️ Arms one chevron press: the direction is stashed for [`render_select_menu`] to price and
/// clamp on the next paint, which is the only place the item count and the resolved popup height are
/// both in hand. Answers `false` for a control id that is not a scroll chevron.
pub fn arm_select_scroll(scroll_offsets: &mut std::collections::HashMap<String, f32>, control_id: &str) -> bool {
    let Some((id, direction)) = select_scroll_control_parts(control_id) else { return false };
    scroll_offsets.insert(select_scroll_pending_key(id), direction);
    true
}

pub(crate) fn arm_retained_select_scroll_at(tree: &mut crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId, x: f32, y: f32) -> bool {
    let direction = tree.node(id).and_then(|node| node.state.select_popup).and_then(|popup| select_scroll_direction_at(popup, x, y));
    let Some(direction) = direction else { return false };
    let Some(node) = tree.node_mut(id) else { return false };
    node.state.scroll_offset.0 = direction;
    tree.mark_dirty(id, crate::wgpu::tree::NodeFlags::DIRTY_PAINT);
    true
}

pub(crate) fn scroll_retained_select_at(tree: &mut crate::wgpu::tree::UiTree, id: crate::wgpu::arena::NodeId, x: f32, y: f32, delta: f32) -> bool {
    let Some(popup) = tree.node(id).filter(|node| node.state.open).and_then(|node| node.state.select_popup).filter(|popup| popup.menu.contains(x, y)) else { return false };
    if delta.is_finite() && delta != 0.0 {
        if let Some(node) = tree.node_mut(id) {
            node.state.scroll_offset.1 = (node.state.scroll_offset.1 + delta).clamp(0.0, popup.max_scroll);
        }
        tree.mark_dirty(id, crate::wgpu::tree::NodeFlags::DIRTY_PAINT);
    }
    true
}

/// 🧹️ Drops both of one popup's scroll slots — called when the popup closes so a reopened `Select`
/// starts at its first page, like a freshly mounted `SelectContent`, and neither slot outlives it.
pub fn clear_select_scroll(scroll_offsets: &mut std::collections::HashMap<String, f32>, id: &str) {
    scroll_offsets.remove(&select_scroll_key(id));
    scroll_offsets.remove(&select_scroll_pending_key(id));
}


