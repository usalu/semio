//! 🎚️ Slider geometry and key handling: the slider axis position and value, the dial, the pointer value and snap, the slider and number key values, the text input keys, the number display and the colour hex parsing. See [`super`].

use super::*;

//#region 🎨️Props
/// 🧿️ Share of a slider's axis within which a pointer value is pulled onto a detent. Arrow keys never snap
/// (a stuck arrow key is worse than a missed detent); page keys stop on the detents they reach.
pub const SLIDER_SNAP_RADIUS: f64 = 0.03;

/// 🗺️ Where `value` sits on a slider's axis, `0` at `min` to `1` at `max` (clamped): its share of the span, or
/// of the log span on a `log` axis. A degenerate or (for `log`) non-positive travel answers `0`.
pub fn slider_axis_position(value: f64, min: f64, max: f64, scale: UiNumberScale) -> f64 {
    let (low, high, at) = match scale {
        UiNumberScale::Linear => (min, max, value),
        UiNumberScale::Log if min > 0.0 && max > 0.0 && value > 0.0 => (min.ln(), max.ln(), value.ln()),
        UiNumberScale::Log => return if min > 0.0 && value >= max { 1.0 } else { 0.0 },
    };
    let span = high - low;
    if span.is_finite() && span > 0.0 {
        ((at - low) / span).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// 🔭️ The value at `position` (`0..=1`, clamped) of a slider's axis — the inverse of [`slider_axis_position`]:
/// `min` and `max` exactly at the ends, `min × (max / min)^position` between them on a `log` axis.
pub fn slider_axis_value(position: f64, min: f64, max: f64, scale: UiNumberScale) -> f64 {
    let position = if position.is_finite() { position.clamp(0.0, 1.0) } else { 0.0 };
    if position == 0.0 {
        return min;
    }
    if position == 1.0 {
        return max;
    }
    match scale {
        UiNumberScale::Log if min > 0.0 && max > 0.0 => (min.ln() + position * (max.ln() - min.ln())).exp().clamp(min.min(max), max.max(min)),
        _ => min + position * (max - min),
    }
}

/// 🧭️ The needle angle of a dial at axis `position`, in radians counter-clockwise from three o'clock: one full
/// turn over the travel, its centre (`0.5`) at `0`, the ends at `±π`.
pub fn dial_angle(position: f64) -> f64 {
    (position.clamp(0.0, 1.0) - 0.5) * std::f64::consts::TAU
}

/// 🌀️ The axis position a pointer at `angle` (radians counter-clockwise from three o'clock, any turn) points at
/// on a dial — the inverse of [`dial_angle`], the seam at nine o'clock.
pub fn dial_position(angle: f64) -> f64 {
    if !angle.is_finite() {
        return 0.5;
    }
    let turns = angle / std::f64::consts::TAU + 0.5;
    turns - turns.floor()
}

/// 👆️ Resolves a raw pointer value (the [`slider_axis_value`] under the pointer): clamped to the bounds,
/// quantized onto the `step` ladder from `min` (cleaned to the decimals of `min` and `step`), then pulled onto the
/// nearest snap when it lies within [`SLIDER_SNAP_RADIUS`] of the axis — measured on the log axis for `log` — the
/// first snap winning a tie.
pub fn slider_pointer_value(value: f64, min: f64, max: f64, step: f64, snaps: impl IntoIterator<Item = f64>, scale: UiNumberScale) -> f64 {
    let clamped = value.clamp(min, max.max(min));
    let digits = decimal_digits(min).max(decimal_digits(step)).min(12);
    let stepped = if step > 0.0 { crate::round_ui_number(min + ((clamped - min) / step).round() * step, digits).clamp(min, max.max(min)) } else { clamped };
    let logarithmic = scale == UiNumberScale::Log && min > 0.0 && max > 0.0;
    let axis = |value: f64| if logarithmic { value.ln() } else { value };
    let radius = (axis(max) - axis(min)).abs() * SLIDER_SNAP_RADIUS;
    snaps.into_iter().map(|snap| (snap, (axis(snap) - axis(clamped)).abs())).filter(|(_, distance)| *distance <= radius).min_by(|left, right| left.1.total_cmp(&right.1)).map_or(stepped, |(snap, _)| snap)
}

/// ⏭️ The next detent strictly above (`forward`) or below `current` — what a page key jumps to.
pub fn slider_adjacent_snap(current: f64, snaps: impl IntoIterator<Item = f64>, forward: bool) -> Option<f64> {
    let candidates = snaps.into_iter().filter(|snap| if forward { *snap > current } else { *snap < current });
    if forward {
        candidates.min_by(f64::total_cmp)
    } else {
        candidates.max_by(f64::total_cmp)
    }
}

/// 📄️ How many steps a large arrow (`Shift`) or a page key with no detent ahead moves a numeric control.
pub const SLIDER_PAGE_STEPS: f64 = 10.0;

/// 🎹️ The keys a slider takes, once a renderer has mapped its physical key (direction, `dir`, inversion) onto
/// the value axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderKey {
    Decrement,
    Increment,
    PageDown,
    PageUp,
    Home,
    End,
}

/// 🪜️ The keyboard law every renderer shares for a bounded slider — [`ui_number_key_value`] with both bounds.
#[allow(clippy::too_many_arguments, reason = "the twin of the TypeScript law's positional facets")]
pub fn slider_key_value(current: f64, min: f64, max: f64, step: f64, precision: Option<u16>, display_factor: Option<f64>, snaps: impl IntoIterator<Item = f64>, key: SliderKey, large: bool) -> f64 {
    ui_number_key_value(current, Some(min), Some(max), step, precision, display_factor, snaps, key, large)
}

/// ⌨️ The keyboard law of every numeric control (slider, dial, stepper, number field, vector axis). The step is `step`,
/// else (a non-positive or non-finite one) `10^-precision` display units converted back by the display factor, else 1.
/// Arrows walk one step of the ladder from `min` (from 0 without one; `large` walks [`SLIDER_PAGE_STEPS`]); from an
/// off-ladder value the first rung beyond it is one step, and they never stop on a detent off their path, so a detent
/// never traps them. Page keys jump to the next/previous detent, else walk [`SLIDER_PAGE_STEPS`] steps. Home and End
/// go to the bounds (and keep the value without one). A walked value is cleaned to the decimals of the ladder origin and
/// step (`0.2 + 0.1` lands on `0.3`), then — with a `precision` — rounded half away from zero at that precision in
/// display units and divided back by the factor, then clamped to the bounds; a value landing within ladder tolerance of
/// a detent lands on the detent exactly (a rung of a `π/180` ladder from `-π` becomes `π/2` itself).
#[allow(clippy::too_many_arguments, reason = "the twin of the TypeScript law's positional facets")]
pub fn ui_number_key_value(current: f64, min: Option<f64>, max: Option<f64>, step: f64, precision: Option<u16>, display_factor: Option<f64>, snaps: impl IntoIterator<Item = f64>, key: SliderKey, large: bool) -> f64 {
    let min = min.filter(|min| min.is_finite());
    let max = max.filter(|max| max.is_finite()).map(|max| min.map_or(max, |min| max.max(min)));
    let clamp = |value: f64| value.max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY));
    let factor = display_factor.filter(|factor| factor.is_finite() && *factor > 0.0);
    let origin = min.unwrap_or(0.0);
    let step = if step.is_finite() && step > 0.0 {
        step
    } else {
        precision.map_or(1.0, |precision| crate::round_ui_number(10f64.powi(-i32::from(precision.min(UI_NUMBER_PRECISION_MAX))), precision) / factor.unwrap_or(1.0))
    };
    let digits = decimal_digits(origin).max(decimal_digits(step)).min(12);
    let snaps: Vec<f64> = snaps.into_iter().filter(|snap| snap.is_finite()).collect();
    let settle = |value: f64| snaps.iter().copied().find(|snap| (value - snap).abs() <= 1e-9 * step * ((snap - origin) / step).abs().max(1.0)).unwrap_or(value);
    let shown = |value: f64| match precision {
        None => value,
        Some(precision) => {
            let rounded = crate::round_ui_number(ui_number_display(value, factor), precision);
            factor.map_or(rounded, |factor| rounded / factor)
        }
    };
    let walk = |steps: f64, forward: bool| {
        let position = (current - origin) / step;
        let nearest = position.round();
        let base = if (position - nearest).abs() <= 1e-9 * nearest.abs().max(1.0) {
            nearest
        } else if forward {
            position.floor()
        } else {
            position.ceil()
        };
        settle(clamp(shown(crate::round_ui_number(origin + (if forward { base + steps } else { base - steps }) * step, digits))))
    };
    let steps = if large { SLIDER_PAGE_STEPS } else { 1.0 };
    match key {
        SliderKey::Increment => walk(steps, true),
        SliderKey::Decrement => walk(steps, false),
        SliderKey::PageUp => slider_adjacent_snap(current, snaps.iter().copied(), true).map_or_else(|| walk(SLIDER_PAGE_STEPS, true), clamp),
        SliderKey::PageDown => slider_adjacent_snap(current, snaps.iter().copied(), false).map_or_else(|| walk(SLIDER_PAGE_STEPS, false), clamp),
        SliderKey::Home => min.unwrap_or(current),
        SliderKey::End => max.unwrap_or(current),
    }
}

/// ⏎️ The keys the draft of a text field answers, once a renderer has read its physical key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputKey {
    Enter,
    Escape,
}

/// 🗜️ The modifiers held with a [`TextInputKey`]: `primary` is the platform's command modifier (Ctrl, ⌘ on macOS).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextInputModifiers {
    pub primary: bool,
    pub shift: bool,
    pub alt: bool,
}

/// ✍️ What a text field does with a key of its draft.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextInputKeyAction {
    /// ↵️ The field inserts a line break at the caret — its own editing, nothing is dispatched.
    Newline,
    /// ✅️ The draft is committed (`Trigger::Commit`) without waiting for focus to leave.
    Commit,
    /// ⏮️ The draft is dropped: the field shows the published value again.
    Revert,
}

/// 🖊️ The keyboard law of every text field that holds a draft (`commit: "blur"`) — the text counterpart of
/// [`ui_number_key_value`] (ticket 26/09/30 design §22.7), pinned by `🧫️text-controls`. Escape reverts the draft. A
/// single-line field commits on Enter. A multi-line field ([`InputKind::LongText`]) inserts a line on Enter, with or
/// without Shift, and commits on Ctrl/⌘+Enter. A single-line Shift+Enter and every Alt chord are not the field's
/// (`None`): Alt+Enter is a shell chord and never reaches a draft. Leaving the field commits in both — that is the commit
/// convention, not a key.
pub fn text_input_key(kind: InputKind, key: TextInputKey, modifiers: TextInputModifiers) -> Option<TextInputKeyAction> {
    if modifiers.alt {
        return None;
    }
    match (key, kind == InputKind::LongText) {
        (TextInputKey::Escape, _) => Some(TextInputKeyAction::Revert),
        (TextInputKey::Enter, true) if modifiers.primary => Some(TextInputKeyAction::Commit),
        (TextInputKey::Enter, true) => Some(TextInputKeyAction::Newline),
        (TextInputKey::Enter, false) if modifiers.shift => None,
        (TextInputKey::Enter, false) => Some(TextInputKeyAction::Commit),
    }
}

/// 🖥️ What a numeric control shows for a stored value: `stored × factor` (the factor `1` when absent).
pub fn ui_number_display(stored: f64, factor: Option<f64>) -> f64 {
    factor.map_or(stored, |factor| stored * factor)
}

/// 🔤️ The text a numeric control shows for a stored value: its [`ui_number_display`] at `precision` fraction
/// digits ([`crate::format_ui_number_fixed`]) or, without one, in the twelve-digit format.
pub fn ui_number_display_text(stored: f64, factor: Option<f64>, precision: Option<u16>) -> String {
    let shown = ui_number_display(stored, factor);
    precision.map_or_else(|| crate::format_ui_number(shown), |precision| crate::format_ui_number_fixed(shown, precision))
}

/// ↩️ The stored value a typed display number means: a candidate (the current value, a detent) whose display text
/// equals the typed number's keeps its exact stored value — so retyping `90` on a degree dial keeps `π/2` — and any
/// other is the typed number (rounded to `precision`) divided back by the factor.
pub fn ui_number_typed_value(typed: f64, factor: Option<f64>, precision: Option<u16>, candidates: impl IntoIterator<Item = f64>) -> f64 {
    let shown = precision.map_or_else(|| crate::format_ui_number(typed), |precision| crate::format_ui_number_fixed(typed, precision));
    let rounded = precision.map_or(typed, |precision| crate::round_ui_number(typed, precision));
    candidates.into_iter().find(|candidate| candidate.is_finite() && ui_number_display_text(*candidate, factor, precision) == shown).unwrap_or_else(|| factor.map_or(rounded, |factor| rounded / factor))
}

/// 🎨️ `rgba` — sRGB components in `0..=1` with straight alpha, a missing component 0 and a missing alpha 1, a
/// non-finite one 0 — as a lowercase `#rrggbb`, or `#rrggbbaa` with `alpha`; each channel is `round(c × 255)`.
pub fn ui_color_hex(rgba: &[f64], alpha: bool) -> String {
    let channel = |index: usize| {
        let component = rgba.get(index).copied().unwrap_or(if index == 3 { 1.0 } else { 0.0 });
        ((if component.is_finite() { component.clamp(0.0, 1.0) } else { 0.0 }) * 255.0).round() as u8
    };
    std::iter::once("#".to_string()).chain((0..if alpha { 4 } else { 3 }).map(|index| format!("{:02x}", channel(index)))).collect()
}

/// 🧪️ Reads a hex colour a user typed — an optional `#`, then 3, 4, 6 or 8 hex digits in any case, surrounding
/// space ignored, the short forms doubling each digit — as sRGB components in `0..=1` (`channel / 255`), alpha 1
/// when absent. `None` for anything else.
pub fn parse_ui_color_hex(text: &str) -> Option<[f64; 4]> {
    let trimmed = text.trim();
    let digits = trimmed.strip_prefix('#').unwrap_or(trimmed);
    let width = match digits.len() {
        3 | 4 => 1,
        6 | 8 => 2,
        _ => return None,
    };
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let mut rgba = [0.0, 0.0, 0.0, 1.0];
    for (index, slot) in rgba.iter_mut().enumerate().take(digits.len() / width) {
        let channel = u8::from_str_radix(&digits[index * width..(index + 1) * width], 16).ok()?;
        *slot = f64::from(if width == 1 { channel * 17 } else { channel }) / 255.0;
    }
    Some(rgba)
}

/// 🔟️ The fraction digits `value` prints with in the twelve-digit format (an `e-n` exponent adds `n`).
fn decimal_digits(value: f64) -> u16 {
    let text = crate::format_ui_number(value);
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text.as_str(), "0"));
    let fraction = mantissa.split_once('.').map_or(0, |(_, fraction)| fraction.len() as i64);
    u16::try_from((fraction - exponent.parse::<i64>().unwrap_or(0)).max(0)).unwrap_or(u16::MAX)
}
//#endregion 🎨️Props
