//! 🔎️ wgpu render function for the Slider element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod slider;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::slider::render_slider;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `SliderMeta`);
//! `crate::wgpu::geometry`/`crate::wgpu::input`/`crate::wgpu::theme` are the other top-level engine mods `widgets`
//! itself also depends on. The detent law lives in the UI contract (`slider_pointer_value`); this module
//! owns how every wgpu slider applies it and paints one tick per snap (`slider_tick_rects`).
//! Not to be confused with the `Ring` element (a circular value control) — `Slider` is a linear track.

use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
use crate::wgpu::theme::Rgba;
use crate::wgpu::widgets::{draw_text_on, SliderMeta, WidgetContext};

/// 🧲️ One tick per detent on `rail`, at the snap's share of the span along the value track — which is
/// left-to-right in every flow, as the thumb is (`layout::slider_presentation`; an rtl flow only moves the
/// readout cell) — centred on the rail, `thickness` wide and `height` tall. Every wgpu slider paints its
/// ticks through this one law, the twin of React's `slider-tick` spans.
pub(crate) fn slider_tick_rects(rail: Rect, min: f64, max: f64, snaps: &[f64], thickness: f32, height: f32) -> impl Iterator<Item = Rect> + '_ {
    let span = (max - min).max(f64::EPSILON);
    snaps.iter().map(move |snap| {
        let share = ((snap - min) / span).clamp(0.0, 1.0) as f32;
        Rect::new(rail.x + rail.w * share - thickness * 0.5, rail.y + (rail.h - height) * 0.5, thickness, height)
    })
}

/// 🎚️ The value a pointer at `raw` commits: the shared contract law (clamp, step ladder, detents).
pub(crate) fn slider_pointer_value(raw: f64, min: f64, max: f64, step: f64, snaps: &[f64]) -> f64 {
    ui_contract::slider_pointer_value(raw, min, max, if step.is_finite() { step } else { 0.0 }, snaps.iter().copied())
}

#[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
pub(crate) fn render_slider<E: Clone>(id: &str, value: f64, min: f64, max: f64, step: f64, snaps: &[f64], ready: Option<f64>, disabled: bool, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
    let presentation = crate::wgpu::layout::slider_control_presentation(bounds, value, min, max, None, ctx.theme.gap_standard, ui_contract::FlowInline::Ltr).slider;
    let range = (max - min).max(f64::EPSILON);
    let mut t = ((value - min) / range).clamp(0.0, 1.0);
    if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
        let dx = ctx.input.drag.current_x - ctx.input.drag.start_x;
        t = (t as f32 + dx / presentation.track_cell.w.max(1.0)).clamp(0.0, 1.0) as f64;
    }
    let selectable_max = ready.map_or(max, |extent| extent.clamp(min, max));
    let live = slider_pointer_value(min + t * range, min, max, step, snaps).clamp(min, selectable_max);
    let live_presentation = crate::wgpu::layout::slider_control_presentation(bounds, live, min, max, None, ctx.theme.gap_standard, ui_contract::FlowInline::Ltr).slider;
    if !disabled {
        if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
            if let Some(on_change) = on_change {
                maps.slider_metas.insert(id.to_string(), SliderMeta { on_change, min, max: selectable_max, step, value, bounds_x: presentation.track_cell.x, bounds_w: presentation.track_cell.w });
            }
            maps.slider_live_values.insert(id.to_string(), live);
        }
    }
    ctx.draw.push_rounded([live_presentation.rail.x, live_presentation.rail.y, live_presentation.rail.w, live_presentation.rail.h], dim(ctx.theme.muted), live_presentation.rail.h * 0.5);
    ctx.draw.push_rounded([live_presentation.range.x, live_presentation.range.y, live_presentation.range.w, live_presentation.range.h], dim(ctx.theme.text_element), live_presentation.range.h * 0.5);
    for tick in slider_tick_rects(live_presentation.rail, min, max, snaps, ctx.theme.stroke_hairline.max(1.0), live_presentation.thumb.h * 0.75) {
        ctx.draw.push_solid([tick.x, tick.y, tick.w, tick.h], dim(ctx.theme.text_element.with_alpha(ctx.theme.text_element.a * 0.6)));
    }
    let value_t = ((live - min) / range).clamp(0.0, 1.0) as f32;
    if let Some(ready_extent) = ready {
        let ready_t = ((ready_extent.clamp(min, max) - min) / range).clamp(0.0, 1.0) as f32;
        if ready_t > value_t {
            let ready_x = live_presentation.rail.x + live_presentation.rail.w * value_t;
            let ready_w = live_presentation.rail.w * (ready_t - value_t);
            ctx.draw.push_rounded([ready_x, live_presentation.rail.y, ready_w, live_presentation.rail.h], dim(Rgba::from_token(&ui_styling::colors::SECONDARY)), live_presentation.rail.h * 0.5);
        }
    }
    ctx.draw.push_rounded([live_presentation.thumb.x, live_presentation.thumb.y, live_presentation.thumb.w, live_presentation.thumb.h], dim(ctx.theme.text_element), live_presentation.thumb.h * 0.5);
    let text = ui_contract::format_ui_number(live);
    let width = ctx.atlas.measure_text(&text, ctx.theme.font_size_small).0;
    draw_text_on(ctx.draw, ctx.atlas, &text, live_presentation.value_cell.x + (live_presentation.value_cell.w - width).max(0.0), live_presentation.value_cell.y + (live_presentation.value_cell.h + ctx.theme.font_size_small) * 0.5, ctx.theme.font_size_small, dim(ctx.theme.text_element));
    if !disabled {
        ctx.input.register_hit(HitTarget { rect: presentation.track_cell, event: None, control_id: Some(id.to_string()), kind: HitKind::Slider, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
    }
}
