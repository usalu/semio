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

/// 🧲️ One tick per detent on `rail`, at the snap's axis position (`ui_contract::slider_axis_position`, the log
/// axis for `log`) along the value track — which is left-to-right in every flow, as the thumb is
/// (`layout::slider_presentation`; an rtl flow only moves the readout cell) — centred on the rail, `thickness` wide
/// and `height` tall. Every wgpu track slider paints its ticks through this one law, the twin of React's
/// `slider-tick` spans.
pub(crate) fn slider_tick_rects(rail: Rect, min: f64, max: f64, scale: ui_contract::UiNumberScale, snaps: &[f64], thickness: f32, height: f32) -> impl Iterator<Item = Rect> + '_ {
    snaps.iter().map(move |snap| {
        let share = ui_contract::slider_axis_position(*snap, min, max, scale) as f32;
        Rect::new(rail.x + rail.w * share - thickness * 0.5, rail.y + (rail.h - height) * 0.5, thickness, height)
    })
}

/// 🎚️ The value a pointer at `raw` commits: the shared contract law (clamp, step ladder, detents).
pub(crate) fn slider_pointer_value(raw: f64, min: f64, max: f64, step: f64, snaps: &[f64], scale: ui_contract::UiNumberScale) -> f64 {
    ui_contract::slider_pointer_value(raw, min, max, if step.is_finite() { step } else { 0.0 }, snaps.iter().copied(), scale)
}

/// 🧭️ A dial's face inside a slider's track cell: a circle as tall as the cell, at its start — the twin of React's
/// `slider-dial` square.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DialFace {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
}

/// ⭕️ The face of a dial drawn in `track`.
pub(crate) fn dial_face(track: Rect) -> DialFace {
    let radius = (track.h.min(track.w) * 0.5).max(0.0);
    DialFace { cx: track.x + radius, cy: track.y + track.h * 0.5, radius }
}

/// 📍️ The point at `share` of the radius along a dial's needle angle (`ui_contract::dial_angle`, counter-clockwise from three
/// o'clock, screen y down).
pub(crate) fn dial_point(face: DialFace, angle: f64, share: f32) -> (f32, f32) {
    (face.cx + face.radius * share * angle.cos() as f32, face.cy - face.radius * share * angle.sin() as f32)
}

/// 📡️ One radial tick per detent of a dial, from 62 % to 92 % of its radius at the detent's needle angle — the twin of
/// React's dial `slider-tick` lines.
pub(crate) fn dial_tick_lines(face: DialFace, slider: &crate::wgpu::component::ui::UiSliderNode) -> impl Iterator<Item = [f32; 4]> + '_ {
    slider.snaps.iter().map(move |snap| {
        let angle = ui_contract::dial_angle(slider.axis_position(*snap));
        let (x0, y0) = dial_point(face, angle, 0.62);
        let (x1, y1) = dial_point(face, angle, 0.92);
        [x0, y0, x1, y1]
    })
}

/// 👆️ The value a press or drag at `(x, y)` over a slider's track cell commits: a track reads the pointer's share of the
/// cell, a dial its angle about the face (`ui_contract::dial_position`); the axis maps the share onto the travel and the
/// shared pointer law steps and snaps it.
pub(crate) fn slider_node_pointer_value(slider: &crate::wgpu::component::ui::UiSliderNode, track: Rect, x: f32, y: f32) -> f64 {
    let position = match slider.appearance {
        ui_contract::SliderAppearance::Dial => {
            let face = dial_face(track);
            ui_contract::dial_position(f64::from(face.cy - y).atan2(f64::from(x - face.cx)))
        }
        ui_contract::SliderAppearance::Track => if track.w > 0.0 { f64::from((x - track.x) / track.w).clamp(0.0, 1.0) } else { 0.0 },
    };
    slider_pointer_value(ui_contract::slider_axis_value(position, slider.min, slider.max, slider.scale), slider.min, slider.max, slider.step, &slider.snaps, slider.scale)
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
    let live = slider_pointer_value(min + t * range, min, max, step, snaps, ui_contract::UiNumberScale::Linear).clamp(min, selectable_max);
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
    for tick in slider_tick_rects(live_presentation.rail, min, max, ui_contract::UiNumberScale::Linear, snaps, ctx.theme.stroke_hairline.max(1.0), live_presentation.thumb.h * 0.75) {
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
