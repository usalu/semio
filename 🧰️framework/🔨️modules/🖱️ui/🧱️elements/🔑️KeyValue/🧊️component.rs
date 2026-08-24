//! 🔎️ wgpu render function for the KeyValue element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod key_value;` right
//! before `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc
//! resolves a nested inline-module's `#[path]` as if the parent had its own on-disk directory,
//! which fails for a genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::key_value::render_key_value;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `KeyValueEntry`,
//! `draw_text`, `measure_text_width`) — these stay in `widgets` because other widgets share them.
//! Not to be confused with the `Field` element (a single labeled-field wrapper) — `KeyValue` renders
//! a table of label→value rows.

use crate::wgpu::geometry::Rect;
use crate::wgpu::widgets::{KeyValueEntry, WidgetContext, draw_text, measure_text_width};

pub(crate) fn render_key_value<E>(entries: &[KeyValueEntry], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let label_w = entries.iter().map(|e| measure_text_width(ctx, &e.label, ctx.theme.font_size_small)).fold(0.0f32, f32::max);
    let value_x = bounds.x + label_w + ctx.theme.gap_standard * 2.0;
    let row_h = ctx.theme.control_height;
    for (index, entry) in entries.iter().enumerate() {
        let y = bounds.y + index as f32 * row_h;
        draw_text(ctx, &entry.label, bounds.x, y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text_muted);
        draw_text(ctx, &entry.value, value_x, y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text);
    }
}
