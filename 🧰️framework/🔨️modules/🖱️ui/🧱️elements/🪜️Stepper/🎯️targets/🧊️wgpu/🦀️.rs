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

use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget};
use crate::input_element::render_input;
use semio_framework_ui_retained_tree::tree::NodeFlags;
use semio_framework_ui_wgpu_widget_kit::{register_input_meta, StepperMeta, WidgetContext};
use semio_framework_ui_wgpu_layout::chrome::{push_control_border, push_icon, ICON_TINY};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::Theme;

/// 🎯️ A stepper value as every wgpu surface prints it: its display value (`stored × display_factor`) at its precision when
/// it has one (the contract's `format_ui_number_fixed`, React's `toFixed` twin), otherwise twelve significant digits.
pub fn stepper_value_text(value: f64, precision: Option<u16>, display_factor: Option<f64>) -> String {
    ui_contract::ui_number_display_text(value, display_factor, precision)
}

#[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
pub fn render_number_stepper<E: Clone>(id: &str, value: f64, step: f64, uniform: bool, on_absolute: Option<E>, on_delta: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
    let segments @ [minus, center, plus] = semio_framework_ui_wgpu_layout::layout::number_stepper_segments(bounds, ui_contract::FlowInline::Ltr, ctx.theme.stroke_hairline);
    ctx.draw.push_scissor(bounds);
    let flags = if ctx.input.focused_id.as_deref() == Some(&format!("{id}.input")) { NodeFlags::FOCUSED } else { NodeFlags::empty() };
    push_stepper_chrome(ctx.draw, bounds, segments, ui_contract::FlowInline::Ltr, flags, None, ctx.theme);
    for (segment, suffix) in [(minus, "minus"), (plus, "plus")] {
        if ctx.input.hovered_id.as_deref() == Some(&format!("{id}.{suffix}")) {
            ctx.draw.push_solid([segment.x, segment.y, segment.w, segment.h], ctx.theme.button_hover);
        }
    }
    push_stepper_icon(ctx.draw, ctx.icons, "minus", minus, ui_contract::FlowInline::Ltr, ctx.theme);
    let text = if uniform { ui_contract::format_ui_number(value) } else { String::new() };
    let placeholder = (!uniform).then_some(semio_framework_ui_declaration::ui::UI_INSPECTOR_MIXED_PLACEHOLDER);
    let input_id = format!("{id}.input");
    register_input_meta(ctx, &input_id, "number", &text, None, (None, None, Some(step), None), on_absolute.clone());
    render_input(&input_id, &text, placeholder, center, true, ctx);
    push_stepper_icon(ctx.draw, ctx.icons, "plus", plus, ui_contract::FlowInline::Ltr, ctx.theme);
    ctx.draw.pop_scissor();
    if let (Some(maps), Some(on_absolute), Some(on_delta)) = (ctx.interaction_maps.as_deref_mut(), on_absolute, on_delta) {
        maps.stepper_metas.insert(id.to_string(), StepperMeta { on_absolute, on_delta, step, value });
    }
    ctx.input.register_hit(HitTarget { rect: minus, event: None, control_id: Some(format!("{id}.minus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    ctx.input.register_hit(HitTarget { rect: plus, event: None, control_id: Some(format!("{id}.plus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
}

/// 🪜️ React Stepper outer chrome and the two logical side-button dividers.
pub fn push_stepper_chrome(draw: &mut DrawList, bounds: Rect, segments: [Rect; 3], inline: ui_contract::FlowInline, flags: NodeFlags, hovered_segment: Option<i8>, theme: &Theme) {
    let border = if flags.contains(NodeFlags::FOCUSED) { theme.accent } else { theme.border_normal };
    push_control_border(draw, bounds, theme, border, theme.input_bg);
    let [minus, _, plus] = segments;
    if hovered_segment == Some(-1) {
        draw.push_solid([minus.x, minus.y, minus.w, minus.h], theme.button_hover);
    } else if hovered_segment == Some(1) {
        draw.push_solid([plus.x, plus.y, plus.w, plus.h], theme.button_hover);
    }
    let hair = theme.stroke_hairline;
    let minus_x = if inline.is_rtl() { minus.x } else { minus.x + minus.w - hair };
    let plus_x = if inline.is_rtl() { plus.x + plus.w - hair } else { plus.x };
    draw.push_solid([minus_x, minus.y, hair, minus.h], theme.border_normal);
    draw.push_solid([plus_x, plus.y, hair, plus.h], theme.border_normal);
}

/// ➕️ Catalog glyph centered in the button content box after its inline divider.
pub fn push_stepper_icon(draw: &mut DrawList, icons: Option<&IconAtlas>, id: &str, bounds: Rect, inline: ui_contract::FlowInline, theme: &Theme) {
    if let Some(icons) = icons {
        let inset = if (id == "minus") != inline.is_rtl() { -theme.stroke_hairline } else { theme.stroke_hairline };
        push_icon(draw, icons, id, bounds.x + (bounds.w - ICON_TINY + inset) * 0.5, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text);
    }
}
