//! 📶️ Progress, celebrate and widget painters: the presence overlay, the progress bars, the celebrate strokes and the focus ring, loading border, stack frame, separator, checkbox and slider painters. See [`super`].

use super::*;

// #region paint
/// 🎯️ Per-variant paint dispatcher for one retained node, given `(origin_x, origin_y)` — the
/// absolute position of *this node's parent's* content-box origin (so `origin + node.layout.{x,y}`
/// is this node's own absolute top-left, matching taffy's parent-relative `Layout::location`).
#[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
/// 🧭️ The one shared presence overlay every `UiNode` variant gets for free, drawn centrally by
/// `paint_node` after that variant's own paint: `previewed`/`disabled` fills underneath nothing extra
/// (disabled reads as a scrim so it composes over whatever the variant already drew), a `status` ring
/// (loading spin / waiting dash / finished solid — mutually exclusive, `idle` draws nothing), an
/// outset accent ring for `selected`, and a breathing pulse ring for `introducing`. `hover` has no
/// dedicated draw call here — it's folded into `flags` before dispatch (see `paint_node`) so every
/// variant's own hover-aware fill (already reading `NodeFlags::HOVERED`) picks it up for free.
pub fn presence_overlay(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: &UiPresence, selected: bool) {
    if presence.state == UiState::Disabled {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel.with_alpha(0.35));
    }
    let ring_color = if selected { theme.selected } else { theme.border_normal };
    match presence.status {
        UiStatus::Loading => paint_loading_border(draw, bounds, ring_color, theme),
        UiStatus::Waiting => paint_waiting_border(draw, bounds, ring_color, theme),
        UiStatus::Finished => draw.push_finished_border([bounds.x, bounds.y, bounds.w, bounds.h], ring_color, theme.border_radius, theme.stroke_hairline),
        UiStatus::Idle => {}
    }
    if selected {
        let ring = Rect::new(bounds.x - 1.0, bounds.y - 1.0, bounds.w + 2.0, bounds.h + 2.0);
        push_chrome_border(draw, ring, theme.stroke_hairline, theme.accent, true, true, true, true);
    } else if presence.state == UiState::Previewed {
        push_chrome_border(draw, bounds, theme.stroke_hairline, theme.accent, true, true, true, true);
    }
    if presence.state == UiState::Introducing {
        draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], theme.accent, theme.border_radius, theme.stroke_hairline);
    }
    if presence.state == UiState::Celebrating {
        let turns = celebrate_turns(draw.clock_seconds(), theme.celebrate_duration_seconds);
        draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], celebrate_color(theme, turns), theme.border_radius, celebrate_stroke(theme, turns));
    }
}

/// ☑️ Checkbox selection represents checkedness, which is painted by the compact control itself.
pub fn node_selection_outline(node: &UiNode) -> bool {
    node.presence().selected && !matches!(node, UiNode::Toggle(toggle) if toggle.appearance == ui_contract::ToggleAppearance::Checkbox)
}

//#region 📶️Progress
/// 📶️ The share of the track an indeterminate sweep covers. The sweep is painted centred and still:
/// this target's CPU paint pass carries no motion clock and no reduced-motion signal, so a motionless
/// busy band is the one indeterminate rendering that honours `prefers-reduced-motion: reduce` by
/// construction — the bar never animates on a host that asked it not to.
pub const PROGRESS_INDETERMINATE_SHARE: f32 = 1.0 / 3.0;

/// 📶️ Track and fill rects of one retained progress bar inside `bounds`: a track `padding_standard`
/// tall, centred vertically, filled from the left to `ui_contract::progress_fraction` while determinate,
/// or carrying the centred [`PROGRESS_INDETERMINATE_SHARE`] sweep while `total` is `None`.
///
/// 📶️ React's bar is `h-tiny w-full` (`🗣️Interpreter/🟦️.tsx:2114`) — `--size-tiny`, not the
/// one-spacing-unit sliver this used to draw.
pub fn progress_bar_rects(node: &UiProgressNode, bounds: Rect, theme: &Theme) -> ([f32; 4], [f32; 4]) {
    progress_bar_rects_of(node.completed, node.total, bounds, theme)
}

/// 📶️ [`progress_bar_rects`] keyed by the two numbers it actually reads, so the `🪀️widgets` kit's
/// own `Progress` arm (ticket 26/09/17 packet W15a) prices the identical geometry without having to
/// build a whole `UiProgressNode` — one bar law covers both kits.
pub fn progress_bar_rects_of(completed: f64, total: Option<f64>, bounds: Rect, _theme: &Theme) -> ([f32; 4], [f32; 4]) {
    let height = SIZE_TINY.min(bounds.h).max(0.0);
    let track = [bounds.x, bounds.y + (bounds.h - height) * 0.5, bounds.w.max(0.0), height];
    let fill = match ui_contract::progress_fraction(completed, total) {
        Some(fraction) => [track[0], track[1], track[2] * fraction as f32, height],
        None => {
            let width = track[2] * PROGRESS_INDETERMINATE_SHARE;
            [track[0] + (track[2] - width) * 0.5, track[1], width, height]
        }
    };
    (track, fill)
}

//#region 🎉️Celebrate
// 🎉️ React celebrates an element with TWO infinite keyframes over one `--celebrate-border-duration`
// (1.2 s): `celebrate-border-spin` drives a conic-gradient angle 0→360° linearly, and
// `celebrate-border-burst` drives the ring's thickness `--stroke-hairline` → `--stroke-focus` →
// hairline on an ease-in-out (`🎨️styling/🖌️ui/🎨️.css:1218-1231, 1249-1268`). Both are pure functions
// of the frame clock, so they port exactly; what does NOT port is the conic gradient itself, which a
// shader-less ring cannot express — see the packet report's gap list.

/// 🎉️ Where in the celebrate cycle `seconds` sits, as turns in `[0, 1)`. `duration <= 0` freezes at
/// the start rather than dividing by zero.
pub fn celebrate_turns(seconds: f32, duration: f32) -> f32 {
    if !(duration > 0.0) || !seconds.is_finite() {
        return 0.0;
    }
    let turns = seconds / duration;
    turns - turns.floor()
}

/// 🎉️ The ring thickness at `turns` — React's `celebrate-border-burst`: hairline at 0 % and 100 %,
/// `--stroke-focus` at 50 %, eased in and out (the raised cosine `ease-in-out` traces).
pub fn celebrate_stroke(theme: &Theme, turns: f32) -> f32 {
    let eased = 0.5 - 0.5 * (turns * std::f32::consts::TAU).cos();
    theme.stroke_hairline + (theme.stroke_focus - theme.stroke_hairline) * eased
}

/// 🎉️ The conic gradient sampled at `turns` — React's `--celebrate-conic` cycles
/// primary → secondary → tertiary → primary, so three equal thirds with a wrap back to the first.
pub fn celebrate_color(theme: &Theme, turns: f32) -> Rgba {
    let stops = theme.celebrate;
    let scaled = turns.clamp(0.0, 1.0) * stops.len() as f32;
    let index = (scaled.floor() as usize).min(stops.len() - 1);
    let next = stops[(index + 1) % stops.len()];
    let from = stops[index];
    let t = scaled - index as f32;
    Rgba::new(from.r + (next.r - from.r) * t, from.g + (next.g - from.g) * t, from.b + (next.b - from.b) * t, from.a + (next.a - from.a) * t)
}
//#endregion 🎉️Celebrate

/// ⌨️ Whether a focus ring paints on this node — React's `focus-visible:border-accent`
/// (`🔨️modules/📝️form-control-presentation/🟦️.ts:14`), which is a `:focus-visible` selector and so
/// never fires for a pointer-focused control. `NodeFlags::FOCUS_VISIBLE` is the bit `EventRouter`
/// stamps beside `FOCUSED` only when a KEY moved focus, so this is the whole distinction: the ring
/// is keyboard-only on both renderers. `FOCUSED` alone still drives caret/edit state and hit
/// routing, which are modality-independent.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn focus_ring_visible(flags: NodeFlags) -> bool {
    flags.contains(NodeFlags::FOCUSED) && flags.contains(NodeFlags::FOCUS_VISIBLE)
}

/// 📶️ Paints a progress bar from theme tokens only: `muted` track, `accent` fill at the theme's own
/// corner radius — React's `ProgressView` is `bg-muted` + `bg-accent` (`🗣️Interpreter/🟦️.tsx:
/// 2114-2116`), and its `rounded-full` resolves to `--radius-full`, which this design system pins to
/// `0` like every other radius, so a hardcoded pill shape was wgpu-only. The track stood in with
/// `separator` until `Theme` gained a real `--muted` field (ticket 26/09/17 packet W2k).
pub fn paint_progress(node: &UiProgressNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let (track, fill) = progress_bar_rects(node, bounds, theme);
    draw.push_rounded(track, theme.muted, theme.border_radius);
    if fill[2] > 0.0 {
        draw.push_rounded(fill, theme.accent, theme.border_radius);
    }
}
//#endregion 📶️Progress

/// 🌀️ Shared "this node is loading" affordance for every `UiNode` kind that carries a
/// `loading: Option<bool>` flag (`Button`, `Stack`, `Section`, `Tree`, `TreeItem`). Delegates to
/// `draw::DrawList::push_loading_border`, which already renders a real time-varying (spinning +
/// pulsing) ring via `UI_SHADER`'s `kind == 6` branch fed by `render_frame`'s `time_seconds`
/// uniform (see `UiInstance::loading_border`'s doc comment) — despite older planning docs assuming
/// no animation-clock scaffolding exists anywhere in this crate, `draw`/`shaders` already wired one
/// in at the GPU layer; this helper just standardizes the radius/stroke args every `paint` call site
/// passes into that existing primitive, leaving only `color` (which varies with e.g. selected state)
/// to the caller.
pub fn paint_loading_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
    draw.push_loading_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
}

/// 🌀️ Shared "this node is waiting" affordance mirroring `paint_loading_border`: dashed, slower ring
/// via `draw::DrawList::push_waiting_border` (`UI_SHADER`'s `kind == 7` branch). Callers dispatch
/// `loading` before `waiting` so the more active state wins when both flags are set.
pub fn paint_waiting_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
    draw.push_waiting_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
}

/// 🎴️ A `Stack`'s `activate`/`selected` visual affordances, ported from
/// `framework/renderer/react/ui-interpreter.tsx`'s `case "stack"` (`widgets::WidgetNode::Stack` has
/// neither field to port from — see this region's own doc comment on why `widgets` is an incomplete
/// reference for fixtures like this one): `activate` (React's `"border bg-panel cursor-pointer
/// rounded-md"`) paints a filled `theme.panel` background (brighter, `theme.button_hover`, while
/// `events::EventRouter`'s hover-chain has flagged it `NodeFlags::HOVERED` — see
/// `events::is_plain_stack_container`'s matching hit-test exception for why an activatable Stack can
/// be hovered/clicked at all) with a normal border; `selected` (`"ring-primary border-primary
/// ring-1"`) paints an accent-colored border plus a slightly outset accent ring, approximating the
/// DOM's separate `ring`+`border` layers with this crate's single stroke-rect primitive.
/// `dropAction`'s accept-a-drop affordance has no dedicated visual in the React reference either
/// (`onDragOver`/`onDrop` are behavioral only) — its only paint-visible effect is keeping
/// `NodeFlags::DROP_TARGET` in sync (`sync_interactive_state`, above), consumed by
/// `events`/cursor-derivation, not drawn here.
pub fn paint_stack_frame(stack: &UiStackNode, bounds: Rect, flags: NodeFlags, theme: &Theme, draw: &mut DrawList) {
    let activatable = stack.activate.is_some();
    if !activatable {
        return;
    }
    let hovered = flags.contains(NodeFlags::HOVERED);
    let bg = if hovered { theme.button_hover } else { theme.panel };
    push_control_border(draw, bounds, theme, theme.border_normal, bg);
}

/// ➖️ A `Separator`'s rule — one `--stroke-hairline` across the node's middle, the same weight
/// React's `<hr>` border carries (`strokes.chromeBorderHairline`), never a hardcoded pixel.
pub fn paint_separator(bounds: Rect, theme: &Theme, draw: &mut DrawList) {
    let y = bounds.y + bounds.h * 0.5;
    draw.push_line(bounds.x, y, bounds.x + bounds.w, y, theme.separator, theme.stroke_hairline);
}

/// 🎨️ A colour field's swatch — a square inset at its leading edge, filled with the colour its text parses to
/// (`ui_contract::parse_ui_color_hex`, the `color_input` recipe's law) — and the bounds its hex text paints in beside
/// it. `None` for text that is not a colour, which then paints as plain text.
pub fn color_input_swatch(bounds: Rect, text: &str) -> Option<(Rect, Rgba, Rect)> {
    let rgba = ui_contract::parse_ui_color_hex(text)?;
    let side = (bounds.h - 8.0).max(4.0);
    let swatch = Rect::new(bounds.x + 4.0, bounds.y + (bounds.h - side) * 0.5, side, side);
    let channel = |component: f64| (component * 255.0).round() as u8;
    Some((swatch, Rgba::from_srgb8(channel(rgba[0]), channel(rgba[1]), channel(rgba[2]), channel(rgba[3])), Rect::new(swatch.x + side, bounds.y, (bounds.w - side - 4.0).max(0.0), bounds.h)))
}

/// ☑️ The compact binary-control outline and check use a fixed number of draw scalars.
pub fn paint_checkbox(node: &semio_framework_ui_declaration::ui::UiToggleNode, bounds: Rect, flags: NodeFlags, theme: &Theme, inline: ui_contract::FlowInline, draw: &mut DrawList) {
    let side = SIZE_TINY.min(bounds.h).min(bounds.w);
    let rect = Rect::new(if inline.is_rtl() { bounds.x + bounds.w - side } else { bounds.x }, bounds.y + (bounds.h - side) * 0.5, side, side);
    let border = if focus_ring_visible(flags) { theme.accent } else { theme.border_normal };
    push_control_border(draw, rect, theme, border, if node.presence.selected { theme.text } else { theme.input_bg });
    if node.presence.selected {
        let color = theme.panel;
        draw.push_line(rect.x + side * 0.2, rect.y + side * 0.5, rect.x + side * 0.43, rect.y + side * 0.73, color, theme.stroke_hairline * 1.5);
        draw.push_line(rect.x + side * 0.43, rect.y + side * 0.73, rect.x + side * 0.82, rect.y + side * 0.25, color, theme.stroke_hairline * 1.5);
    }
}

/// 🎚️ A slider's travel at `value`: a track's rail, filled range, one tick per detent at its axis position and the thumb —
/// or a dial's face, one radial tick per detent, the needle at the value's angle and its thumb on the rim. Shared by the
/// retained painter and the test painter, so both draw every detent the same way.
pub fn paint_slider_travel(node: &semio_framework_ui_declaration::ui::UiSliderNode, value: f64, presentation: semio_framework_ui_wgpu_layout::layout::SliderPresentation, theme: &Theme, draw: &mut DrawList) {
    let tick_color = theme.text_element.with_alpha(theme.text_element.a * 0.6);
    match node.appearance {
        ui_contract::SliderAppearance::Dial => {
            let face = semio_framework_ui_wgpu_controls::slider::dial_face(presentation.track_cell);
            let diameter = face.radius * 2.0;
            draw.push_rounded([face.cx - face.radius, face.cy - face.radius, diameter, diameter], theme.text_element, face.radius);
            let inset = theme.stroke_hairline.max(1.0);
            draw.push_rounded([face.cx - face.radius + inset, face.cy - face.radius + inset, diameter - inset * 2.0, diameter - inset * 2.0], theme.muted, face.radius - inset);
            for [x0, y0, x1, y1] in semio_framework_ui_wgpu_controls::slider::dial_tick_lines(face, node) {
                draw.push_line(x0, y0, x1, y1, tick_color, theme.stroke_hairline.max(1.0));
            }
            let angle = ui_contract::dial_angle(node.axis_position(value));
            let (x, y) = semio_framework_ui_wgpu_controls::slider::dial_point(face, angle, 0.82);
            draw.push_line(face.cx, face.cy, x, y, theme.accent, (face.radius * 0.14).max(1.5));
            let knob = (face.radius * 0.3).max(2.0);
            draw.push_rounded([x - knob * 0.5, y - knob * 0.5, knob, knob], theme.text_element, knob * 0.5);
        }
        ui_contract::SliderAppearance::Track => {
            draw.push_rounded([presentation.rail.x, presentation.rail.y, presentation.rail.w, presentation.rail.h], theme.muted, presentation.rail.h * 0.5);
            draw.push_rounded([presentation.range.x, presentation.range.y, presentation.range.w, presentation.range.h], theme.text_element, presentation.range.h * 0.5);
            for tick in semio_framework_ui_wgpu_controls::slider::slider_tick_rects(presentation.rail, node.min, node.max, node.scale, &node.snaps, theme.stroke_hairline.max(1.0), presentation.thumb.h * 0.75) {
                draw.push_solid([tick.x, tick.y, tick.w, tick.h], tick_color);
            }
            draw.push_rounded([presentation.thumb.x, presentation.thumb.y, presentation.thumb.w, presentation.thumb.h], theme.text_element, presentation.thumb.h * 0.5);
        }
    }
}
// #endregion paint
