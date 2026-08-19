//! 🔎️ wgpu render function for the Slider element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod slider;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::slider::render_slider;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `SliderMeta`);
//! `crate::wgpu::geometry`/`crate::wgpu::input`/`crate::wgpu::theme` are the other top-level engine mods `widgets`
//! itself also depends on. `quantize_step` moved alongside `render_slider` as a private helper since
//! it has no other caller anywhere in the crate.
//! Not to be confused with the `Ring` element (a circular value control) — `Slider` is a linear track.

use crate::wgpu::widgets::{SliderMeta, WidgetContext};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
use crate::wgpu::theme::Rgba;

async fn quantize_step(value: f64, step: f64, min: f64) -> f64 {
    if step <= 0.0 {
        return value;
    }
    min + ((value - min) / step).round() * step
}

#[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
pub(crate) async fn render_slider<E: Clone>(id: &str, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let track_y = bounds.y + bounds.h * 0.5;
    let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
    ctx.draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], dim(ctx.theme.separator), 2.0);
    let range = (max - min).max(f64::EPSILON);
    let mut t = ((value - min) / range).clamp(0.0, 1.0);
    if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - ctx.input.drag.start_x;
        t = (t as f32 + dx / bounds.w.max(1.0)).clamp(0.0, 1.0) as f64;
    }
    let selectable_max = ready.map(|extent| extent.clamp(min, max)).unwrap_or(max);
    let live = quantize_step(min + t * range, step, min).clamp(min, selectable_max);
    if !disabled {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            if let Some(on_change) = on_change {
                maps.slider_metas.insert(id.to_string(), SliderMeta { on_change, min, max: selectable_max, step, value, bounds_x: bounds.x, bounds_w: bounds.w });
            }
            maps.slider_live_values.insert(id.to_string(), live);
        }
    }
    let value_t = ((live - min) / range).clamp(0.0, 1.0) as f32;
    if let Some(ready_extent) = ready {
        let ready_t = ((ready_extent.clamp(min, max) - min) / range).clamp(0.0, 1.0) as f32;
        if ready_t > value_t {
            let ready_x = bounds.x + bounds.w * value_t;
            let ready_w = bounds.w * (ready_t - value_t);
            ctx.draw.push_rounded([ready_x, track_y - 2.0, ready_w, 4.0], dim(Rgba::new(0.03433981, 0.63759687, 0.52099557, 1.0)), 2.0);
        }
    }
    let knob_x = bounds.x + bounds.w * value_t;
    ctx.draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], dim(ctx.theme.accent), 6.0);
    if !disabled {
        ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Slider, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
    }
}
