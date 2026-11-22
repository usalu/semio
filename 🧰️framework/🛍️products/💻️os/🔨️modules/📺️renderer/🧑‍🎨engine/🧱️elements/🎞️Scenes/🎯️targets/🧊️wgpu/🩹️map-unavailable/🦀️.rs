//! 🩹️ The given-up tiled map: React's `rendererUnavailable` overlay, a background-filled column with the alert text over a Retry button.

use super::*;
use semio_renderer_wgpu_engine_board::tiled_map::{map_unavailable_layout, stage_map_unavailable_notice, MapUnavailableNotice};
use semio_renderer_wgpu_scene_state::MapChromeLabels;
use semio_ui_wgpu_widget_kit::draw_text;
use semio_ui_wgpu_layout::chrome::{measure_action_item, push_icon, ICON_TINY};

/// 🖌️ Paints the opaque notice over `bounds` and stages its rects for the retry pointer route and the accessibility projection. The Retry button is drawn without a widget hit so the map surface keeps the pointer.
///
/// See `🧱️elements/🧭️TiledMapHost/🟦️.tsx`.
pub fn render_map_unavailable(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, labels: MapChromeLabels) {
    let theme = ctx.theme;
    let message_size = ctx.atlas.measure_text(labels.canvas_unavailable, theme.font_size_body);
    let retry_width = measure_action_item(ctx.atlas, theme, true, Some(labels.retry));
    let (message_rect, retry_rect) = map_unavailable_layout(bounds, message_size, retry_width, theme.control_height, theme.gap_standard);
    ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], theme.background, theme.border_radius);
    draw_text(ctx, labels.canvas_unavailable, message_rect.x, message_rect.y + message_rect.h, theme.font_size_body, theme.text_muted);
    ctx.draw.push_rounded([retry_rect.x, retry_rect.y, retry_rect.w, retry_rect.h], theme.button, theme.border_radius);
    let pad = theme.padding_standard;
    if let Some(icons) = ctx.icons {
        push_icon(ctx.draw, icons, "rotate-ccw", retry_rect.x + pad, retry_rect.y + (retry_rect.h - ICON_TINY) * 0.5, ICON_TINY, theme.text);
    }
    let label_height = ctx.atlas.measure_text(labels.retry, theme.font_size_small).1;
    draw_text(ctx, labels.retry, retry_rect.x + pad + ICON_TINY + theme.gap_standard, retry_rect.y + (retry_rect.h + label_height) * 0.5, theme.font_size_small, theme.text);
    stage_map_unavailable_notice(&scene.host_id, MapUnavailableNotice { message: labels.canvas_unavailable.to_string(), retry_label: labels.retry.to_string(), message_rect, retry_rect });
}
