//! 🔎️ wgpu render function for the Toggle element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod toggle;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::toggle::render_toggle;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `draw_text`);
//! `crate::wgpu::chrome`/`crate::wgpu::input` are the other top-level engine mods `widgets` itself also depends
//! on; `crate::wgpu::IconName` is the crate-root re-export of the generated icon enum.

use crate::wgpu::chrome::{item_bg, item_text, push_control_border, push_icon, ICON_TINY};
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::widgets::{draw_text, WidgetContext};
use crate::wgpu::IconName;

pub(crate) fn render_toggle<E: Clone>(id: &str, icon_id: IconName, pressed: bool, text: Option<&str>, bounds: crate::wgpu::geometry::Rect, ctx: &mut WidgetContext<'_, E>) {
    let hovered = ctx.input.hovered_id.as_deref() == Some(id);
    let bg = item_bg(ctx.theme, pressed, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut content_x = bounds.x + ctx.theme.padding_standard;
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(icon_id.as_str()).is_some() {
            push_icon(ctx.draw, icons, icon_id.as_str(), content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(ctx.theme, pressed, hovered));
            content_x += ICON_TINY + ctx.theme.gap_standard;
        }
    }
    if let Some(text) = text {
        draw_text(ctx, text, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, item_text(ctx.theme, pressed, hovered));
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
}
