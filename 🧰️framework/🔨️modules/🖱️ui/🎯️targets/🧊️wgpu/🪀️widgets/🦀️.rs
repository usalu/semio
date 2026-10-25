// #region widgets
//! 🧩️ Generic widget tree — layout, measurement, and drawing.

use semio_framework_ui_declaration::ui::UiTreeWindow;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState};
use semio_framework_ui_wgpu_layout::layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
use semio_framework_ui_text::{FontAtlas, TextFace};
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_declaration::IconName;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use std::collections::HashMap;

const PANEL_HEADER: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::PANEL_HEADER_HEIGHT_UI_SPACING) as f32;
/// 🌳️ The ONE tree row pitch, straight off `dom.treeRowUiSpacing` — the token React's `Tree` rows
/// carry as `h-workbench`, and the same number `Theme::tree_row_height` hands the retained
/// layout/paint/hit path (`layout::TreeRowMetrics`).
pub use semio_framework_ui_theme::metrics::TREE_ROW_HEIGHT;
pub const TREE_INDENT_PER_LEVEL: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_INDENT_PER_LEVEL_UI_SPACING) as f32;
pub const TREE_TOGGLE_WIDTH: f32 = (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::dom::TREE_TOGGLE_UI_SPACING) as f32;
pub const TREE_ICON_SIZE: f32 = semio_framework_ui_wgpu_layout::chrome::ICON_TREE_ROW;
pub const TREE_SECTION_GAP: f32 = 8.0;

pub fn measure_widget<E>(atlas: &mut FontAtlas, theme: &Theme, node: &WidgetNode<E>) -> (f32, f32) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(theme, gap.as_deref());
            let padding = padding_for_token(theme, padding.as_deref()) * 2.0;
            let vertical = direction != "horizontal";
            let mut total_main = 0.0f32;
            let mut max_cross = 0.0f32;
            for (index, child) in children.iter().enumerate() {
                let (w, h) = measure_widget(atlas, theme, child);
                if vertical {
                    total_main += h;
                    max_cross = max_cross.max(w);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                } else {
                    total_main += w;
                    max_cross = max_cross.max(h);
                    if index + 1 < children.len() {
                        total_main += gap;
                    }
                }
            }
            if vertical {
                (max_cross + padding, total_main + padding)
            } else {
                (total_main + padding, max_cross + padding)
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { theme.font_size_emphasized } else { theme.font_size_body };
            let (w, _) = atlas.measure_text(value, size);
            let lines = wrap_text(atlas, value, w.max(120.0), size);
            (w.max(120.0), lines.len() as f32 * size * 1.35)
        }
        WidgetNode::Separator => (theme.control_height.max(1.0), 1.0 + theme.gap_standard),
        WidgetNode::Button { .. } | WidgetNode::Input { .. } | WidgetNode::Select { .. } | WidgetNode::Toggle { .. } | WidgetNode::Slider { .. } | WidgetNode::NumberStepper { .. } | WidgetNode::IconSelect { .. } => {
            (theme.control_height, theme.control_height)
        }
        WidgetNode::KeyValue { entries } => {
            let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        WidgetNode::Ring { .. } => (80.0, 80.0),
        WidgetNode::Field { label, child, .. } => {
            let label_h = theme.font_size_small;
            let gap = gap_for_token(theme, Some("standard"));
            let (cw, ch) = measure_control(atlas, theme, child);
            (cw.max(atlas.measure_text(label, theme.font_size_small).0), label_h + gap + ch)
        }
        WidgetNode::Section { children, label, .. } => {
            let mut height = PANEL_HEADER;
            let mut max_w = 0.0f32;
            if label.is_some() {
                max_w = max_w.max(160.0);
            }
            for child in children {
                let (w, h) = measure_widget(atlas, theme, child);
                max_w = max_w.max(w);
                height += h + theme.gap_standard;
            }
            (max_w.max(120.0), height)
        }
        WidgetNode::Tree { sections, .. } => (measure_tree_sections_width(sections, atlas, theme), measure_tree_sections(sections)),
        WidgetNode::Progress { .. } => (120.0, semio_framework_ui_wgpu_layout::chrome::SIZE_TINY.max(theme.gap_standard)),
        WidgetNode::Image { src, alt, .. } => match semio_framework_ui_wgpu_paint::ui_image_natural_size(src.as_str()) {
            Some((natural_w, natural_h)) => {
                let height = (natural_h as f32).min(semio_framework_ui_wgpu_paint::UI_IMAGE_MAX_BOX_HEIGHT);
                let scale = if natural_h > 0 { height / natural_h as f32 } else { 1.0 };
                ((natural_w as f32 * scale).max(1.0), height.max(1.0))
            }
            None => {
                let label = alt.as_deref().unwrap_or("");
                (atlas.measure_text(label, theme.font_size_small).0.max(120.0), theme.control_height)
            }
        },
        WidgetNode::Group { children, label, .. } => {
            let mut height = PANEL_HEADER;
            let mut max_w = atlas.measure_text(label, theme.font_size_body).0 + TREE_TOGGLE_WIDTH + theme.gap_standard;
            for child in children {
                let (w, h) = measure_widget(atlas, theme, child);
                max_w = max_w.max(w);
                height += h + theme.gap_standard;
            }
            (max_w.max(120.0), height)
        }
        WidgetNode::ComponentScene { .. } | WidgetNode::ExternalSlot { .. } => (120.0, theme.control_height),
    }
}

fn measure_control<E>(atlas: &mut FontAtlas, theme: &Theme, control: &ControlNode<E>) -> (f32, f32) {
    match control {
        ControlNode::Button { .. } | ControlNode::Input { .. } | ControlNode::Select { .. } | ControlNode::Toggle { .. } | ControlNode::Slider { .. } | ControlNode::NumberStepper { .. } | ControlNode::IconSelect { .. } => {
            (theme.control_height, theme.control_height)
        }
        ControlNode::KeyValue { entries } => {
            let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
            (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
        }
        ControlNode::Ring { .. } => (80.0, 80.0),
    }
}

pub fn render_widget<E: Clone>(node: &WidgetNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match node {
        WidgetNode::Stack { direction, gap, padding, children } => {
            let gap = gap_for_token(ctx.theme, gap.as_deref());
            let padding = padding_for_token(ctx.theme, padding.as_deref());
            let vertical = direction != "horizontal";
            let sizes: Vec<f32> = children
                .iter()
                .map(|child| {
                    let (w, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    if vertical {
                        h
                    } else {
                        w
                    }
                })
                .collect();
            let rects = if vertical { layout_vertical(bounds, gap, padding, &sizes) } else { layout_horizontal(bounds, gap, padding, &sizes) };
            for (child, rect) in children.iter().zip(rects.iter()) {
                render_widget(child, *rect, ctx);
            }
        }
        WidgetNode::Text { value, emphasize } => {
            let size = if *emphasize { ctx.theme.font_size_emphasized } else { ctx.theme.font_size_body };
            let color = if *emphasize { ctx.theme.text } else { ctx.theme.text_muted };
            draw_text_wrapped(ctx, value, bounds.x, bounds.y, bounds.w.max(1.0), size, color);
        }
        WidgetNode::Separator => {
            let y = bounds.y + bounds.h * 0.5;
            ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, ctx.theme.separator, 1.0);
        }
        WidgetNode::Button { id, icon_id, label, event } => {
            render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
        }
        WidgetNode::Input { id, input_kind, value, placeholder, commit, min, max, step, accept, on_change } => {
            register_input_meta(ctx, id, input_kind, value, commit.clone(), (*min, *max, *step, accept.clone()), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, false, ctx);
        }
        WidgetNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        WidgetNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        WidgetNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        WidgetNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
            render_slider(id, *value, *min, *max, *step, &[], *ready, *disabled, on_change.clone(), bounds, ctx);
        }
        WidgetNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
        }
        WidgetNode::Ring { id, t, disabled, on_change } => {
            render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx);
        }
        WidgetNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
        }
        WidgetNode::Field { label, child, .. } => {
            let label_h = ctx.theme.font_size_small;
            let gap = gap_for_token(ctx.theme, Some("standard"));
            draw_text(ctx, label, bounds.x, bounds.y + label_h, ctx.theme.font_size_small, ctx.theme.text_muted);
            let child_bounds = Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap);
            render_control(child, child_bounds, ctx);
        }
        WidgetNode::Section { label, children, id, default_open } => {
            let section_key = format!("section.{id}");
            if !ctx.collapsed_sections.contains_key(&section_key) {
                ctx.collapsed_sections.insert(section_key.clone(), !default_open);
            }
            let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, *default_open);
            if label.is_some() {
                let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
                let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
                let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
                tree_draw_chevron(ctx, chevron, chevron_rect, semio_framework_ui_wgpu_layout::chrome::ICON_TINY);
                if let Some(label) = label {
                    draw_text(ctx, label, bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard, bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
                }
                ctx.input.register_hit(HitTarget { rect: header, event: None, control_id: Some(format!("section.chevron.{id}")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            }
            if !collapsed {
                let mut y = bounds.y + PANEL_HEADER;
                for child in children {
                    let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
                    let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
                    render_widget(child, child_bounds, ctx);
                    y += h + ctx.theme.gap_standard;
                }
            }
        }
        WidgetNode::Tree { sections, selected_ids, highlighted_ids, selection_change } => {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                maps.tree_selection_change = selection_change.clone();
            }
            let scroll_id = sections.first().map_or_else(|| format!("tree:{:.0}:{:.0}", bounds.x, bounds.y), |section| format!("tree:{}", section.id));
            let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
            render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                render_tree(sections, selected_ids, highlighted_ids, content, ctx);
            });
        }
        WidgetNode::Progress { completed, total, .. } => render_progress(*completed, *total, bounds, ctx),
        WidgetNode::Image { id, src, alt } => render_image(id, src, alt.as_deref(), bounds, ctx),
        WidgetNode::Group { id, label, default_open, children } => render_group(id, label, *default_open, children, bounds, ctx),
        WidgetNode::ComponentScene { surface_id, hit_kind, hit_control_id } => {
            render_scene_placeholder(surface_id, bounds, ctx);
            ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(hit_control_id.clone()), kind: *hit_kind, drag_axis: None, drag_data: None });
        }
        WidgetNode::ExternalSlot { body_key } => render_external_slot(body_key, bounds, ctx),
    }
}

//#region 🧩️KitCompositeParity
/// 📶️ The kit's progress bar — the SAME track/fill rects `paint::progress_bar_rects` hands the
/// retained ladder, so the two kits cannot drift on geometry, the indeterminate share, or the
/// `muted`/`accent` token pair.
fn render_progress<E>(completed: f64, total: Option<f64>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let (track, fill) = semio_framework_ui_wgpu_paint::progress_bar_rects_of(completed, total, bounds, ctx.theme);
    ctx.draw.push_rounded(track, ctx.theme.muted, ctx.theme.border_radius);
    if fill[2] > 0.0 {
        ctx.draw.push_rounded(fill, ctx.theme.accent, ctx.theme.border_radius);
    }
}

/// 🖼️ A decodable (`data:`) source draws its real bitmap at React's `object-contain` rect; every
/// other source is a host fetch this crate has no authority to perform, so it shows the placeholder
/// panel plus `alt` — which is also what React's `<img>` shows while it is pending or broken.
fn render_image<E>(id: &str, src: &str, alt: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let decoded = matches!(semio_framework_ui_wgpu_paint::admit_ui_image(src), semio_framework_ui_wgpu_paint::UiImageAdmission::Ready).then(|| semio_framework_ui_wgpu_paint::ui_image_natural_size(src)).flatten();
    if let Some((natural_w, natural_h)) = decoded {
        let content = semio_framework_ui_wgpu_paint::ui_image_content_rect(bounds, natural_w, natural_h);
        ctx.draw.push_raster_quad(src, [content.x, content.y, content.w, content.h], [0.0, 0.0, 1.0, 1.0], 1.0);
        return;
    }
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    let label = alt.unwrap_or(id);
    draw_text(ctx, label, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_small) * 0.5 - 2.0, ctx.theme.font_size_small, ctx.theme.text_muted);
}

/// 🗂️ A `Group` folds through the same `collapsed_sections` slot a `Section` does — one fold state
/// per id, so a panel that mixes both kinds cannot end up with two disagreeing open bits.
fn render_group<E: Clone>(id: &str, label: &str, default_open: bool, children: &[WidgetNode<E>], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let section_key = format!("section.{id}");
    if !ctx.collapsed_sections.contains_key(&section_key) {
        ctx.collapsed_sections.insert(section_key.clone(), !default_open);
    }
    let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, default_open);
    let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
    let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
    tree_draw_chevron(ctx, if collapsed { "chevron-right" } else { "chevron-down" }, chevron_rect, semio_framework_ui_wgpu_layout::chrome::ICON_TINY);
    draw_text(ctx, label, bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard, bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    ctx.input.register_hit(HitTarget { rect: header, event: None, control_id: Some(format!("section.chevron.{id}")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    if collapsed {
        return;
    }
    let mut y = bounds.y + PANEL_HEADER;
    for child in children {
        let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
        render_widget(child, Rect::new(bounds.x, y, bounds.w, h), ctx);
        y += h + ctx.theme.gap_standard;
    }
}

/// 🎬️ The no-host placeholder for a scene rect — "there is something in that box" chrome, identical
/// to the retained ladder's own `ComponentScene` fallback.
fn render_scene_placeholder<E>(surface_id: &str, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let _ = surface_id;
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    semio_framework_ui_wgpu_layout::chrome::push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.panel);
}

/// 🔌️ A plugin body slot labelled by its `body_key`; the body itself is the host's concern.
fn render_external_slot<E>(body_key: &str, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
    semio_framework_ui_wgpu_layout::chrome::push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.panel);
    draw_text(ctx, body_key, bounds.x + ctx.theme.padding_standard, bounds.y + (bounds.h + ctx.theme.font_size_small) * 0.5 - 2.0, ctx.theme.font_size_small, ctx.theme.text_muted);
}
//#endregion 🧩️KitCompositeParity

fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match control {
        ControlNode::Button { id, icon_id, label, event } => {
            render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
        }
        ControlNode::Input { id, input_kind, value, placeholder, commit, min, max, step, accept, on_change } => {
            register_input_meta(ctx, id, input_kind, value, commit.clone(), (*min, *max, *step, accept.clone()), on_change.clone());
            render_input(id, value, placeholder.as_deref(), bounds, false, ctx);
        }
        ControlNode::Select { id, value, items, placeholder, on_change } => {
            register_select_meta(ctx, id, on_change.clone());
            render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
        }
        ControlNode::Toggle { id, icon_id, pressed, text, on_change } => {
            register_toggle_meta(ctx, id, *pressed, on_change.clone());
            render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
        }
        ControlNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
        ControlNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
            render_slider(id, *value, *min, *max, *step, &[], *ready, *disabled, on_change.clone(), bounds, ctx);
        }
        ControlNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
            render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
        }
        ControlNode::Ring { id, t, disabled, on_change } => render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx),
        ControlNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
            render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
        }
    }
}

use semio_framework_ui_wgpu_controls::button::render_button;
use semio_framework_ui_wgpu_controls::icon_selector::render_icon_select;
use semio_framework_ui_wgpu_controls::input_element::render_input;
use semio_framework_ui_wgpu_controls::key_value::render_key_value;
use semio_framework_ui_wgpu_controls::ring::render_ring;
use semio_framework_ui_wgpu_controls::select::render_select;
use semio_framework_ui_wgpu_controls::slider::render_slider;
use semio_framework_ui_wgpu_controls::stepper::render_number_stepper;
use semio_framework_ui_wgpu_controls::toggle::render_toggle;
use crate::tree_element::{measure_tree_sections, measure_tree_sections_state, measure_tree_sections_width, render_tree};
use semio_framework_ui_wgpu_widget_kit::{ControlNode, TreeItem, WidgetContext, WidgetNode, draw_icon, draw_text, draw_text_wrapped, register_input_meta, register_select_meta, register_toggle_meta, render_scroll_region, wrap_text};

#[path = "../../../🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod tree_element;


pub fn tree_gutter_width(depth: u32) -> f32 {
    depth as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH
}

pub fn tree_icon_id<E>(item: &TreeItem<E>, expandable: bool) -> Option<&str> {
    item.icon_id.map(IconName::as_str).or(if expandable { Some("folder") } else { None })
}

pub fn tree_row_collapsed(collapsed: &HashMap<String, bool>, key: &str, default_open: bool) -> bool {
    collapsed.get(key).copied().unwrap_or(!default_open)
}

/// 🔽️ One fold chevron centred in `rect`. `size` is the caller's, because React draws a `Tree`
/// ROW's toggle at `size-tiny` — 9.6 px, [`crate::wgpu::chrome::SIZE_TINY`] (`🌳️Tree/🟦️.tsx:4576`) —
/// and a SECTION header's at `size-small`, 16 px, [`crate::wgpu::chrome::ICON_TINY`]
/// (`treeSectionChevronClassName`, `🌳️Tree/🟦️.tsx:251`).
pub fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect, size: f32) {
    if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
        draw_icon(ctx, uv, rect.x + (rect.w - size) * 0.5, rect.y + (rect.h - size) * 0.5, size, ctx.theme.text_muted);
    }
}

pub fn tree_draw_guides<E>(ctx: &mut WidgetContext<'_, E>, gutter: Rect, depth: u32, is_last_at_level: &[bool]) {
    let hair = ctx.theme.stroke_hairline.max(1.0);
    let guide_color = ctx.theme.border_normal;
    for level in 0..depth {
        if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
            continue;
        }
        let x = gutter.x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, gutter.h], guide_color);
    }
    if depth > 0 {
        let x = gutter.x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
        let mid_y = gutter.y + gutter.h * 0.5;
        ctx.draw.push_solid([x, gutter.y, hair, mid_y - gutter.y], guide_color);
        ctx.draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs"]
mod metrics_tests;
// #endregion widgets
