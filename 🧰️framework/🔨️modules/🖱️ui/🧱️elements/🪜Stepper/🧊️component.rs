//! 🔎️ wgpu render function for the Stepper element — extracted from `widgets` mod's inline body
//! (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired as a CRATE-ROOT sibling module of
//! `crate::wgpu::widgets` (declared `#[cfg(feature = "wgpu-engine")] #[path = "..."] mod stepper;` right before
//! `pub mod widgets` in lib.rs — deliberately NOT nested inside `widgets { }`, since rustc resolves a
//! nested inline-module's `#[path]` as if the parent had its own on-disk directory, which fails for a
//! genuinely inline `mod widgets { }` block). `widgets` mod pulls this back in via
//! `use crate::wgpu::stepper::render_number_stepper;` so its own unqualified call sites keep working.
//! `crate::wgpu::widgets::{...}` reaches the sibling items this needs (`WidgetContext`, `StepperMeta`,
//! `draw_text`, `register_input_meta`); `crate::wgpu::input_element` supplies `render_input` (the Input
//! element, used as the stepper's center text box); `crate::wgpu::chrome`/`crate::wgpu::input` are the other
//! top-level engine mods `widgets` itself also depends on.
//! Not to be confused with the `Steps` element (a progress-indicator, unrelated concept).

use crate::wgpu::widgets::{draw_text, register_input_meta, StepperMeta, WidgetContext};
use crate::wgpu::chrome::push_control_border;
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{HitKind, HitTarget};
use crate::wgpu::input_element::render_input;

#[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
pub(crate) fn render_number_stepper<E: Clone>(id: &str, value: f64, step: f64, _uniform: bool, on_absolute: Option<E>, on_delta: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let seg = bounds.w / 3.0;
    let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
    let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
    let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
    let hair = ctx.theme.stroke_hairline;
    push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.input_bg);
    ctx.draw.push_solid([bounds.x + seg, bounds.y, hair, bounds.h], ctx.theme.border_normal);
    ctx.draw.push_solid([bounds.x + seg * 2.0, bounds.y, hair, bounds.h], ctx.theme.border_normal);
    let minus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.minus"));
    let plus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.plus"));
    if minus_hovered {
        ctx.draw.push_solid([minus.x, minus.y, minus.w, minus.h], ctx.theme.button_hover);
    }
    if plus_hovered {
        ctx.draw.push_solid([plus.x, plus.y, plus.w, plus.h], ctx.theme.button_hover);
    }
    draw_text(ctx, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    let text = format!("{value:.3}");
    let input_id = format!("{id}.input");
    register_input_meta(ctx, &input_id, &text, None, on_absolute.clone());
    render_input(&input_id, &text, None, center, ctx);
    draw_text(ctx, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
    if let (Some(maps), Some(on_absolute), Some(on_delta)) = (ctx.interaction_maps.as_deref_mut(), on_absolute, on_delta) {
        maps.stepper_metas.insert(id.to_string(), StepperMeta { on_absolute, on_delta, step, value });
    }
    ctx.input.register_hit(HitTarget { rect: minus, event: None, control_id: Some(format!("{id}.minus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    ctx.input.register_hit(HitTarget { rect: plus, event: None, control_id: Some(format!("{id}.plus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
}
