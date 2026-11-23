//! 🧮️ Table, block list and diff view scenes.
//!
//! The table render with cell payloads, sort, stepper and editable cells and their accessibility staging, and the block list and diff view render.

extern crate semio_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::FrameworkWidgetContext;
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use semio_ui_wgpu_input::KeyAction;
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::Rgba;
use semio_ui_wgpu_widget_kit::draw_text;
use semio_ui_wgpu_widget_kit::draw_text_face;
use semio_ui_wgpu_layout::chrome::foreground_on_fill;
use semio_ui_wgpu_layout::chrome::push_icon;
use semio_ui_wgpu_input::HitKind;
use semio_ui_wgpu_input::HitTarget;
use semio_ui_text::TextFace;
use semio_ui_theme::Theme;
use semio_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_ui_wgpu_widget_kit::WidgetNode;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_declaration::ui::SurfaceKind;
use semio_ui_declaration::ui::UiComponentSceneNode;
use semio_renderer_wgpu_scene_state::{BLOCK_LIST_PALETTE_DRAG_MIME, BlockListAccessibilityControl, BlockListChromeLabels, SceneListHit, SceneListTransferSession, SceneListTransferSource, SceneListTransferStart, SceneModifiers, TableButtonAccessibilityCell, TableEditableTextAccessibilityCell, TableEditableTextFocusTarget, TableStepperAccessibilityCell, TableStepperFocusTarget, draw_ink_rect_outline, list_visible_row_capacity, render_placeholder, reserve_list_rows, scene_action, scene_hovered_control_id, scene_input_theme, scroll_key, scroll_offset, write_scene_action};
use semio_renderer_wgpu_scene_feeds::{accepted_block_list_accessibility_controls, accepted_table_button_accessibility_cells, stage_block_list_accessibility_controls, stage_table_button_accessibility_cells, stage_table_editable_text_accessibility_cells, stage_table_stepper_accessibility_cells};

use semio_ui_wgpu_layout::chrome::{item_bg, item_text, push_control_border, SharedBorderState, ICON_TINY};
use semio_ui_wgpu_widget_kit::{draw_text_wrapped, measure_text_width, register_input_meta, WidgetContext};

/// 🧩️ The three immediate widgets a scene host draws over its surface (text, button, input); every other node kind is the retained painter's.
pub(crate) fn render_widget<E: Clone>(node: &WidgetNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    match node {
        WidgetNode::Text { value, emphasize } => {
            let (size, color) = if *emphasize { (ctx.theme.font_size_emphasized, ctx.theme.text) } else { (ctx.theme.font_size_body, ctx.theme.text_muted) };
            draw_text_wrapped(ctx, value, bounds.x, bounds.y, bounds.w.max(1.0), size, color);
        }
        WidgetNode::Button { id, icon_id, label, event } => {
            let control_id = id.clone().or_else(|| Some(label.clone()));
            let hovered = ctx.input.hovered_id == control_id;
            let (bg, ink) = (item_bg(ctx.theme, false, hovered), item_text(ctx.theme, false, hovered));
            push_control_border(ctx.draw, bounds, ctx.theme, if hovered { ctx.theme.border_emphasized } else { ctx.theme.border_normal }, bg, if hovered { SharedBorderState::Hover } else { SharedBorderState::Normal });
            let mut text_x = bounds.x + ctx.theme.padding_standard;
            let icon_key = icon_id.filter(|icon| *icon != IconName::CircleDot).map_or(label.as_str(), |icon| icon.as_str());
            if let Some(icons) = ctx.icons.filter(|icons| icons.icon_uv(icon_key).is_some()) {
                push_icon(ctx.draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, ink);
                text_x += ICON_TINY + ctx.theme.gap_standard;
            }
            draw_text(ctx, label, text_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ink);
            ctx.input.register_hit(HitTarget { rect: bounds, event: event.clone(), control_id, kind: HitKind::Button, drag_axis: None, drag_data: None });
        }
        WidgetNode::Input { id, input_kind, value, placeholder, commit, min, max, step, accept, on_change } => {
            register_input_meta(ctx, id, input_kind, value, commit.clone(), (*min, *max, *step, accept.clone()), on_change.clone());
            let focused = ctx.input.focused_id.as_deref() == Some(id.as_str());
            push_control_border(ctx.draw, bounds, ctx.theme, if focused { ctx.theme.border_emphasized } else { ctx.theme.border_normal }, ctx.theme.input_bg, if focused { SharedBorderState::Focus } else { SharedBorderState::Normal });
            let (display, muted) = if focused { (ctx.input.text_view().to_string(), false) } else if value.is_empty() { (placeholder.clone().unwrap_or_default(), true) } else { (value.clone(), false) };
            let text_x = bounds.x + 8.0;
            draw_text(ctx, &display, text_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, if muted { ctx.theme.text_muted } else { ctx.theme.text });
            if focused {
                let cursor = ctx.input.text_view_cursor();
                let cursor_x = text_x + measure_text_width(ctx, &display[..cursor], ctx.theme.font_size_body);
                ctx.draw.push_solid([cursor_x, bounds.y + 6.0, 1.0, bounds.h - 12.0], ctx.theme.text);
            }
            ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.clone()), kind: HitKind::Input, drag_axis: None, drag_data: None });
        }
        _ => {}
    }
}

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs"]
mod table_and_paint2d;
pub use table_and_paint2d::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧱️block-list-and-diff/🦀️.rs"]
mod block_list_and_diff;
pub use block_list_and_diff::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
