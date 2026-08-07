//! 🔎️ wgpu render function for the Ring element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod ring;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::ring::render_ring;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`RingMeta`, `WidgetContext`);
//! `crate::wgpu::geometry`/`crate::wgpu::input` are the other top-level engine mods `widgets` itself also depends on.

use crate::wgpu::widgets::{RingMeta, WidgetContext};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};

pub(crate) fn render_ring<E: Clone>(id: &str, t: f64, disabled: bool, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let cx = bounds.x + bounds.w * 0.5;
    let cy = bounds.y + bounds.h * 0.5;
    let radius = bounds.w.min(bounds.h) * 0.4;
    let segments = 48usize;
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let angle = std::f32::consts::TAU * i as f32 / segments as f32;
        points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
    }
    for window in points.windows(2) {
        ctx.draw.push_line(window[0][0], window[0][1], window[1][0], window[1][1], ctx.theme.separator, 2.0);
    }
    let mut knob_t = t;
    if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - cx;
        let dy = ctx.input.drag.current_y - cy;
        knob_t = (dy.atan2(dx) as f64 / std::f64::consts::TAU).rem_euclid(1.0);
    }
    if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
        maps.ring_metas.insert(id.to_string(), RingMeta { on_change, disabled, center_x: cx, center_y: cy, radius });
        maps.ring_live_values.insert(id.to_string(), knob_t);
    }
    let knob_angle = std::f32::consts::TAU * knob_t as f32;
    let kx = cx + knob_angle.cos() * radius;
    let ky = cy + knob_angle.sin() * radius;
    let accent = if disabled { ctx.theme.text_muted } else { ctx.theme.accent };
    ctx.draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
    if !disabled {
        ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Slider, drag_axis: Some(DragAxis::Ring), drag_data: None });
    }
}
