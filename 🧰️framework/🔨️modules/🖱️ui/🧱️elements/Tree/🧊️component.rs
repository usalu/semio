//! 🔎️ wgpu render/measure functions for the Tree element — extracted from `widgets` mod's inline
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module
//! of `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod tree_element;`
//! right before `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since
//! rustc resolves a nested inline-module's `#[path]` as if the parent had its own on-disk directory,
//! which fails for a genuinely inline `mod widgets { }` block). Named `tree_element` rather than the
//! naive `tree` because a crate-root `pub mod tree` already exists (the retained scene-graph
//! arena/`UiTree`, unrelated) — same disambiguation `input_element` used against `crate::wgpu::input`.
//! `widgets` mod pulls the render/measure entry points back in via one `use crate::wgpu::tree_element::{...};`
//! so its own unqualified call sites (including `WidgetNode::Tree`'s dispatch arm) keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `TreeSection`,
//! `TreeItem`, the `TREE_*` layout constants, the small `tree_gutter_width`/`tree_icon_id`/
//! `tree_row_collapsed`/`tree_draw_chevron`/`tree_draw_guides` helpers, `draw_icon`, `draw_text`,
//! `measure_text_width`, `render_widget` — all promoted to `pub(crate)` in this same pass since they
//! were previously `widgets`-module-private); `crate::wgpu::geometry`/`crate::wgpu::input`/`crate::wgpu::text`/
//! `crate::wgpu::theme` are the other top-level engine mods `widgets` itself also depends on.

use crate::wgpu::widgets::{
    draw_icon, draw_text, measure_text_width, render_widget, tree_draw_chevron, tree_draw_guides, tree_gutter_width, tree_icon_id, tree_row_collapsed, TreeItem, TreeSection, WidgetContext, TREE_ICON_SIZE, TREE_INDENT_PER_LEVEL, TREE_ROW_HEIGHT,
    TREE_SECTION_GAP, TREE_TOGGLE_WIDTH,
};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::UiTreeActionPlacement;
use std::collections::HashMap;

pub(crate) fn measure_tree_sections_width<E>(sections: &[TreeSection<E>], atlas: &mut FontAtlas, theme: &Theme) -> f32 {
    let collapsed = HashMap::new();
    measure_tree_sections_width_state(sections, atlas, theme, &collapsed, 0)
}

pub(crate) fn measure_tree_sections_width_state<E>(sections: &[TreeSection<E>], atlas: &mut FontAtlas, theme: &Theme, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
    let mut max_w = 0.0f32;
    for section in sections {
        let section_key = format!("section.{}", section.id);
        let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
        if let Some(label) = &section.label {
            let w = atlas.measure_text(label, theme.font_size_small).0 + tree_gutter_width(0) + TREE_ICON_SIZE + theme.gap_standard * 2.0;
            max_w = max_w.max(w);
        }
        if !section_collapsed {
            for item in &section.items {
                max_w = max_w.max(measure_tree_item_width(item, atlas, theme, collapsed, depth));
            }
        }
    }
    max_w.max(120.0)
}

pub(crate) fn measure_tree_item_width<E>(item: &TreeItem<E>, atlas: &mut FontAtlas, theme: &Theme, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
    if item.dimmed {
        return 0.0;
    }
    let mut w = tree_gutter_width(depth) + TREE_ICON_SIZE + theme.gap_standard + atlas.measure_text(&item.label, theme.font_size_body).0 + theme.gap_standard;
    if let Some(description) = &item.description {
        w += atlas.measure_text(description, theme.font_size_small).0 + theme.gap_standard;
    }
    for action in &item.actions {
        w += TREE_ICON_SIZE + theme.padding_standard;
        if let Some(label) = &action.label {
            w += atlas.measure_text(label, theme.font_size_small).0 + theme.gap_standard;
        }
    }
    if item.control.is_some() {
        w += 120.0 + theme.gap_standard;
    }
    let key = format!("tree.{}", item.id);
    let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
    if !item_collapsed {
        for child in &item.children {
            w = w.max(measure_tree_item_width(child, atlas, theme, collapsed, depth + 1));
        }
    }
    w
}

pub(crate) fn measure_tree_sections<E>(sections: &[TreeSection<E>]) -> f32 {
    let collapsed = HashMap::new();
    measure_tree_sections_state(sections, &collapsed)
}

pub(crate) fn measure_tree_sections_state<E>(sections: &[TreeSection<E>], collapsed: &HashMap<String, bool>) -> f32 {
    let mut height = 0.0;
    for section in sections {
        height += TREE_ROW_HEIGHT;
        let section_key = format!("section.{}", section.id);
        let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
        if !section_collapsed {
            for item in &section.items {
                height += measure_tree_item_height(item, collapsed);
            }
            height += TREE_SECTION_GAP;
        }
    }
    height
}

pub(crate) fn measure_tree_item_height<E>(item: &TreeItem<E>, collapsed: &HashMap<String, bool>) -> f32 {
    if item.dimmed {
        return 0.0;
    }
    let mut height = TREE_ROW_HEIGHT;
    let key = format!("tree.{}", item.id);
    let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
    if !item_collapsed {
        for child in &item.children {
            height += measure_tree_item_height(child, collapsed);
        }
    }
    height
}

pub(crate) fn render_tree<E: Clone>(sections: &[TreeSection<E>], selected_ids: &[String], highlighted_ids: &[String], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let mut y = bounds.y;
    for section in sections {
        let section_key = format!("section.{}", section.id);
        if !ctx.collapsed_sections.contains_key(&section_key) {
            ctx.collapsed_sections.insert(section_key.clone(), !section.default_open);
        }
        let section_collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, section.default_open);
        render_tree_section_header(section, bounds, y, section_collapsed, ctx);
        y += TREE_ROW_HEIGHT;
        if !section_collapsed {
            for item in &section.items {
                y += render_tree_item(item, Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT), ctx, 0, selected_ids, highlighted_ids, &[]);
            }
            y += TREE_SECTION_GAP;
        }
    }
}

pub(crate) fn render_tree_section_header<E: Clone>(section: &TreeSection<E>, bounds: Rect, y: f32, collapsed: bool, ctx: &mut WidgetContext<'_, E>) {
    let row = Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT);
    let gutter_w = TREE_TOGGLE_WIDTH;
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
    tree_draw_chevron(ctx, chevron, gutter);
    ctx.input.register_hit(HitTarget { rect: gutter, event: None, control_id: Some(format!("section.chevron.{}", section.id)), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
    if let Some(label) = &section.label {
        let text_color = if collapsed { ctx.theme.text_muted } else { ctx.theme.text_element };
        let label_x = content.x + ctx.theme.gap_standard;
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv("folder")) {
            draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
        }
        draw_text(ctx, label, label_x + TREE_ICON_SIZE + ctx.theme.gap_standard, content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, text_color);
    }
}

pub(crate) fn render_tree_item<E: Clone>(item: &TreeItem<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>, depth: u32, selected_ids: &[String], highlighted_ids: &[String], is_last_at_level: &[bool]) -> f32 {
    if item.dimmed {
        return 0.0;
    }
    let key = format!("tree.{}", item.id);
    if !ctx.collapsed_sections.contains_key(&key) {
        ctx.collapsed_sections.insert(key.clone(), !item.default_open);
    }
    let collapsed = tree_row_collapsed(ctx.collapsed_sections, &key, item.default_open);
    let expandable = !item.children.is_empty();
    let gutter_w = tree_gutter_width(depth);
    let row = Rect::new(bounds.x, bounds.y, bounds.w, TREE_ROW_HEIGHT);
    let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
    let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
    let hovered = ctx.input.hovered_id.as_deref().is_some_and(|id| id.strip_prefix("tree.label.").is_some_and(|v| v == item.id));
    let selected = item.selected || selected_ids.iter().any(|id| id == &item.id);
    let highlighted = item.highlighted || highlighted_ids.iter().any(|id| id == &item.id);
    tree_draw_guides(ctx, gutter, depth, is_last_at_level);
    if expandable {
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        let chevron_rect = Rect::new(gutter.x + depth as f32 * TREE_INDENT_PER_LEVEL, gutter.y, TREE_TOGGLE_WIDTH, gutter.h);
        tree_draw_chevron(ctx, chevron, chevron_rect);
        ctx.input.register_hit(HitTarget { rect: chevron_rect, event: None, control_id: Some(format!("tree.chevron.{}", item.id)), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
    }
    if selected {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.selected, ctx.theme.border_radius);
    } else if highlighted || hovered {
        ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.row_hover, ctx.theme.border_radius);
    }
    let mut label_x = content.x + ctx.theme.gap_standard;
    let icon_id = tree_icon_id(item, expandable);
    let text_color = if selected || highlighted {
        ctx.theme.active_foreground
    } else if hovered {
        ctx.theme.border_emphasized
    } else if item.dimmed {
        ctx.theme.text_muted
    } else {
        ctx.theme.text_element
    };
    if let Some(icon_id) = icon_id {
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
            draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
            label_x += TREE_ICON_SIZE + ctx.theme.gap_standard;
        }
    }
    draw_text(ctx, &item.label, label_x, content.y + (content.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, text_color);
    if let Some(description) = &item.description {
        let label_w = measure_text_width(ctx, &item.label, ctx.theme.font_size_body);
        draw_text(ctx, description, label_x + label_w + ctx.theme.gap_standard, content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text_muted);
    }
    let mut actions_x = content.x + content.w - ctx.theme.gap_standard;
    for (index, action) in item.actions.iter().enumerate().rev() {
        if action.placement == UiTreeActionPlacement::Menu {
            continue;
        }
        let label_w = action.label.as_ref().map_or(0.0, |label| measure_text_width(ctx, label, ctx.theme.font_size_small) + ctx.theme.gap_standard);
        let action_w = TREE_ICON_SIZE + ctx.theme.padding_standard + label_w;
        actions_x -= action_w;
        let action_rect = Rect::new(actions_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5 - 2.0, action_w, TREE_ICON_SIZE + 4.0);
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(action.icon_id.as_str())) {
            let action_color = if hovered { ctx.theme.border_emphasized } else { ctx.theme.text_element };
            draw_icon(ctx, uv, action_rect.x + 2.0, action_rect.y + 2.0, TREE_ICON_SIZE, action_color);
        }
        if hovered {
            if let Some(label) = &action.label {
                draw_text(ctx, label, action_rect.x + TREE_ICON_SIZE + 4.0, action_rect.y + (TREE_ICON_SIZE + ctx.theme.font_size_small) * 0.5, ctx.theme.font_size_small, ctx.theme.text_muted);
            }
        }
        ctx.input.register_hit(HitTarget { rect: action_rect, event: Some(action.event.clone()), control_id: Some(format!("tree.action.{}.{}", item.id, index)), kind: HitKind::Button, drag_axis: None, drag_data: None });
    }
    if let Some(hover) = &item.hover_event {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            maps.tree_hover_commands.insert(item.id.clone(), hover.clone());
        }
    }
    if let Some(unhover) = &item.unhover_event {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            maps.tree_unhover_commands.insert(item.id.clone(), unhover.clone());
        }
    }
    if let Some(control) = &item.control {
        let control_w = 120.0;
        let control_rect = Rect::new(content.x + content.w - control_w - ctx.theme.gap_standard, content.y + (content.h - ctx.theme.control_height) * 0.5, control_w, ctx.theme.control_height);
        render_widget(control, control_rect, ctx);
    }
    let label_rect = Rect::new(label_x, content.y, content.x + content.w - label_x - ctx.theme.gap_standard, content.h);
    ctx.input.register_hit(HitTarget {
        rect: label_rect,
        event: item.event.clone(),
        control_id: Some(format!("tree.label.{}", item.id)),
        kind: HitKind::TreeItem,
        drag_axis: if item.draggable { Some(DragAxis::Both) } else { None },
        drag_data: if item.draggable && !item.drag_data.is_empty() { Some(item.drag_data.clone()) } else { None },
    });
    let mut height = TREE_ROW_HEIGHT;
    if !collapsed {
        for (index, child) in item.children.iter().enumerate() {
            let mut child_is_last = is_last_at_level.to_vec();
            child_is_last.push(index + 1 == item.children.len());
            let child_bounds = Rect::new(bounds.x, bounds.y + height, bounds.w, TREE_ROW_HEIGHT);
            height += render_tree_item(child, child_bounds, ctx, depth + 1, selected_ids, highlighted_ids, &child_is_last);
        }
    }
    height
}
