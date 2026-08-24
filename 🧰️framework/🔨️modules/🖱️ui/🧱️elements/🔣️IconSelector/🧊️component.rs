//! 🔎️ wgpu render function for the IconSelector element — extracted from `widgets` mod's inline
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module
//! of `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod icon_selector;`
//! right before `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since
//! rustc resolves a nested inline-module's `#[path]` as if the parent had its own on-disk directory,
//! which fails for a genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::icon_selector::render_icon_select;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling item this needs (`WidgetContext`, `draw_text`);
//! `crate::wgpu::chrome`/`crate::wgpu::geometry`/`crate::wgpu::input` are the other top-level engine mods `widgets`
//! itself also depends on.

use crate::wgpu::chrome::{ICON_TINY, chrome_item_bg, push_control_border, push_icon};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::widgets::{WidgetContext, draw_text};

pub(crate) fn render_icon_select<E: Clone>(id: &str, value: &str, _uniform: bool, _classifier_kind: &str, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, chrome_item_bg(ctx.theme, false, ctx.input.hovered_id.as_deref() == Some(id)));
    let content_x = bounds.x + ctx.theme.padding_standard;
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(value).is_some() {
            push_icon(ctx.draw, icons, value, content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, ctx.theme.text_element);
        } else {
            draw_text(ctx, value, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
        }
    } else {
        draw_text(ctx, value, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: on_change, control_id: Some(id.to_string()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
}
