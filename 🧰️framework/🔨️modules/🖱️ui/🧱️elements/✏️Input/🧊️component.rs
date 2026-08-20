//! 🔎️ wgpu render function for the Input element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod input_element;` right
//! before `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc
//! resolves a nested inline-module's `#[path]` as if the parent had its own on-disk directory, which
//! fails for a genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::input_element::render_input;` so its own unqualified call sites keep working.
//! (Module named `input_element`, not `input`, to avoid shadowing the sibling `crate::wgpu::input` mod.)
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `draw_text`,
//! `measure_text_width`); `crate::wgpu::chrome`/`crate::wgpu::input` are the other top-level engine mods
//! `widgets` itself also depends on.

use crate::wgpu::widgets::{draw_text, measure_text_width, WidgetContext};
use crate::wgpu::chrome::push_control_border;
use crate::wgpu::input::{HitKind, HitTarget};

pub(crate) fn render_input<E: Clone>(id: &str, value: &str, placeholder: Option<&str>, bounds: crate::wgpu::geometry::Rect, ctx: &mut WidgetContext<'_, E>) {
    let focused = ctx.input.focused_id.as_deref() == Some(id);
    let border = if focused { ctx.theme.border_emphasized } else { ctx.theme.border_normal };
    push_control_border(ctx.draw, bounds, ctx.theme, border, ctx.theme.input_bg);
    let (display, muted) = if focused {
        (ctx.input.text_buffer.clone(), false)
    } else if value.is_empty() {
        (placeholder.unwrap_or("").to_string(), true)
    } else {
        (value.to_string(), false)
    };
    draw_text(ctx, &display, bounds.x + 8.0, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, if muted { ctx.theme.text_muted } else { ctx.theme.text });
    if focused {
        let cursor_x = bounds.x + 8.0 + measure_text_width(ctx, &display[..ctx.input.cursor_pos.min(display.len())], ctx.theme.font_size_body);
        ctx.draw.push_solid([cursor_x, bounds.y + 6.0, 1.0, bounds.h - 12.0], ctx.theme.text);
    }
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Input, drag_axis: None, drag_data: None });
}
