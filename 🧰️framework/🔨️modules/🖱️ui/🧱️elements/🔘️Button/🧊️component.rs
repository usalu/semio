//! 🔎️ wgpu render function for the Button element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod button;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::button::render_button;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `draw_text`);
//! `crate::wgpu::chrome`/`crate::wgpu::input` are the other top-level engine mods `widgets` itself also depends
//! on; `crate::wgpu::IconName` is the crate-root re-export of the generated icon enum.

use crate::wgpu::IconName;
use crate::wgpu::chrome::{ICON_TINY, item_bg, item_text, push_control_border, push_icon};
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::widgets::{WidgetContext, draw_text};

pub(crate) fn render_button<E: Clone>(id: Option<&String>, icon_id: Option<IconName>, label: &str, event: Option<E>, bounds: crate::wgpu::geometry::Rect, ctx: &mut WidgetContext<'_, E>) {
    let control_id = id.cloned().or_else(|| Some(label.to_string()));
    let hovered = ctx.input.hovered_id == control_id;
    let bg = item_bg(ctx.theme, false, hovered);
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
    let mut text_x = bounds.x + ctx.theme.padding_standard;
    let icon_key = icon_id.filter(|id| *id != IconName::CircleDot).map(IconName::as_str).unwrap_or(label);
    if let Some(icons) = ctx.icons {
        if icons.icon_uv(icon_key).is_some() {
            push_icon(ctx.draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(ctx.theme, false, hovered));
            text_x += ICON_TINY + ctx.theme.gap_standard;
        }
    }
    draw_text(ctx, label, text_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, item_text(ctx.theme, false, hovered));
    ctx.input.register_hit(HitTarget { rect: bounds, event, control_id, kind: HitKind::Button, drag_axis: None, drag_data: None });
}
