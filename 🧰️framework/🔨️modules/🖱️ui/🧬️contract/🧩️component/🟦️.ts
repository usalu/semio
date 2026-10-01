import type { UiNumberBound, UiNumberLimits, UiNumberScale } from "@semio-tech/framework";
import { formatUiNumber, formatUiNumberFixed, roundUiNumber } from "../🔢️number-format/🟦️.ts";

/** 🧿️ Share of a slider's axis within which a pointer value is pulled onto a detent — the twin of `SLIDER_SNAP_RADIUS`. */
export const SLIDER_SNAP_RADIUS = 0.03;

/** 📐️ Where `value` sits on a slider's axis, `0` at `min` to `1` at `max` (clamped), by span or log span; the twin of `slider_axis_position`. */
export function sliderAxisPosition(value: number, min: number, max: number, scale: UiNumberScale = "linear"): number {
  let low = min;
  let high = max;
  let at = value;
  if (scale === "log") {
    if (!(min > 0 && max > 0 && value > 0)) return min > 0 && value >= max ? 1 : 0;
    low = Math.log(min);
    high = Math.log(max);
    at = Math.log(value);
  }
  const span = high - low;
  return Number.isFinite(span) && span > 0 ? Math.min(1, Math.max(0, (at - low) / span)) : 0;
}

/** 🔭️ The value at `position` (clamped) of a slider's axis — `min`/`max` exactly at the ends, `min × (max / min)^position` on a log axis; the twin of `slider_axis_value`. */
export function sliderAxisValue(position: number, min: number, max: number, scale: UiNumberScale = "linear"): number {
  const at = Number.isFinite(position) ? Math.min(1, Math.max(0, position)) : 0;
  if (at === 0) return min;
  if (at === 1) return max;
  if (scale === "log" && min > 0 && max > 0) return Math.min(Math.max(min, max), Math.max(Math.min(min, max), Math.exp(Math.log(min) + at * (Math.log(max) - Math.log(min)))));
  return min + at * (max - min);
}

/** 🧭️ A dial's needle angle at axis `position`, radians counter-clockwise from three o'clock, the travel's centre at `0`; the twin of `dial_angle`. */
export function dialAngle(position: number): number {
  return (Math.min(1, Math.max(0, position)) - 0.5) * 2 * Math.PI;
}

/** 🌀️ The axis position a pointer at `angle` points at on a dial, the seam at nine o'clock; the twin of `dial_position`. */
export function dialPosition(angle: number): number {
  if (!Number.isFinite(angle)) return 0.5;
  const turns = angle / (2 * Math.PI) + 0.5;
  return turns - Math.floor(turns);
}

/** 🙅️ Whether `value` lies beyond `bound` on the side `below` names (an excluded limit refuses itself); the twin of `UiNumberBound::refuses`. */
export function uiNumberBoundRefuses(bound: UiNumberBound, value: number, below: boolean): boolean {
  if (below) return bound.exclusive ? value <= bound.value : value < bound.value;
  return bound.exclusive ? value >= bound.value : value > bound.value;
}

/** 🪤️ The hard bound a typed `value` crosses — `limits` when declared, else the inclusive `min`/`max` (unlabelled) — the lower one first, `null` for an admitted or non-finite value; the twin of `ui_number_crossed_bound`. */
export function uiNumberCrossedBound(value: number, min: number | null | undefined, max: number | null | undefined, limits: UiNumberLimits | null | undefined): UiNumberBound | null {
  if (!Number.isFinite(value)) return null;
  const lower = limits ? limits.min : min == null ? null : { value: min, exclusive: false, refusal: null };
  const upper = limits ? limits.max : max == null ? null : { value: max, exclusive: false, refusal: null };
  if (lower && uiNumberBoundRefuses(lower, value, true)) return lower;
  if (upper && uiNumberBoundRefuses(upper, value, false)) return upper;
  return null;
}

/** 🖥️ What a numeric control shows for a stored value: `stored × factor` (factor 1 when absent); the twin of `ui_number_display`. */
export function uiNumberDisplay(stored: number, factor: number | null | undefined): number {
  return factor == null ? stored : stored * factor;
}

/** 🔤️ The text a numeric control shows for a stored value, at `precision` or in the twelve-digit format; the twin of `ui_number_display_text`. */
export function uiNumberDisplayText(stored: number, factor: number | null | undefined, precision: number | null | undefined): string {
  const shown = uiNumberDisplay(stored, factor);
  return precision == null ? formatUiNumber(shown) : formatUiNumberFixed(shown, precision);
}

/** ↩️ The stored value a typed display number means: a candidate whose display text equals the typed one keeps its exact stored value, any other is the typed number (rounded to `precision`) divided by the factor; the twin of `ui_number_typed_value`. */
export function uiNumberTypedValue(typed: number, factor: number | null | undefined, precision: number | null | undefined, candidates: readonly number[]): number {
  const shown = precision == null ? formatUiNumber(typed) : formatUiNumberFixed(typed, precision);
  const rounded = precision == null ? typed : roundUiNumber(typed, precision);
  const kept = candidates.find((candidate) => Number.isFinite(candidate) && uiNumberDisplayText(candidate, factor, precision) === shown);
  return kept ?? (factor == null ? rounded : rounded / factor);
}

/** 🔬️ The largest precision a renderer honours — the twin of `UI_NUMBER_PRECISION_MAX`. */
export const UI_NUMBER_PRECISION_MAX = 15;

/** 👆️ Clamp, quantize onto the step ladder from `min` (cleaned to its decimals), then pull onto the nearest snap within the radius of the axis — the log axis for `log` — the first snap winning a tie; the twin of `slider_pointer_value`. */
export function sliderPointerValue(value: number, min: number, max: number, step: number, snaps: readonly number[], scale: UiNumberScale = "linear"): number {
  const upper = Math.max(min, max);
  const clamped = Math.min(upper, Math.max(min, value));
  const digits = Math.min(12, Math.max(decimalDigits(min), decimalDigits(step)));
  const stepped = step > 0 ? Math.min(upper, Math.max(min, roundUiNumber(min + Math.round((clamped - min) / step) * step, digits))) : clamped;
  const logarithmic = scale === "log" && min > 0 && max > 0;
  const axis = (at: number): number => (logarithmic ? Math.log(at) : at);
  const radius = Math.abs(axis(max) - axis(min)) * SLIDER_SNAP_RADIUS;
  let nearest: number | null = null;
  let distance = Number.POSITIVE_INFINITY;
  for (const snap of snaps) {
    const gap = Math.abs(axis(snap) - axis(clamped));
    if (gap <= radius && gap < distance) {
      nearest = snap;
      distance = gap;
    }
  }
  return nearest ?? stepped;
}

/** ⏭️ The next detent strictly above (`forward`) or below `current`; the twin of `slider_adjacent_snap`. */
export function sliderAdjacentSnap(current: number, snaps: readonly number[], forward: boolean): number | null {
  const candidates = snaps.filter((snap) => (forward ? snap > current : snap < current));
  return candidates.length === 0 ? null : forward ? Math.min(...candidates) : Math.max(...candidates);
}

/** 📄️ How many ladder rungs a large arrow or a page key moves a slider — the twin of `SLIDER_PAGE_STEPS`. */
export const SLIDER_PAGE_STEPS = 10;

/** ⌨️ The keys a slider takes on its value axis — the twin of `SliderKey`. */
export type SliderKey = "decrement" | "increment" | "pageDown" | "pageUp" | "home" | "end";

/** 🔟️ The fraction digits `value` prints with in the twelve-digit format (an `e-n` exponent adds `n`). */
function decimalDigits(value: number): number {
  const [mantissa = "", exponent = "0"] = formatUiNumber(value).split("e");
  return Math.max(0, (mantissa.split(".")[1]?.length ?? 0) - Number(exponent));
}

/** 🪜️ The keyboard law of a bounded slider — {@link uiNumberKeyValue} with both bounds; the twin of `slider_key_value`. */
export function sliderKeyValue(current: number, min: number, max: number, step: number, snaps: readonly number[], key: SliderKey, large: boolean): number {
  return uiNumberKeyValue(current, min, max, step, snaps, key, large);
}

/** ⌨️ The keyboard law of every numeric control: arrows walk one rung of the step ladder from `min` (from 0 without one; `large` walks {@link SLIDER_PAGE_STEPS} rungs) and never stop on a detent off their path, page keys walk ten rungs and stop on the first detent they reach, a key landing within ladder tolerance of a detent lands on it exactly, Home/End go to the bounds (keeping the value without one), results clamp to the bounds; the twin of `ui_number_key_value`. */
export function uiNumberKeyValue(current: number, min: number | null, max: number | null, step: number, snaps: readonly number[], key: SliderKey, large: boolean): number {
  const lower = min != null && Number.isFinite(min) ? min : null;
  const upper = max != null && Number.isFinite(max) ? (lower == null ? max : Math.max(lower, max)) : null;
  const clamp = (value: number): number => Math.min(upper ?? Number.POSITIVE_INFINITY, Math.max(lower ?? Number.NEGATIVE_INFINITY, value));
  const origin = lower ?? 0;
  const rung = Number.isFinite(step) && step > 0 ? step : 1;
  const digits = Math.min(12, Math.max(decimalDigits(origin), decimalDigits(rung)));
  const walk = (rungs: number, forward: boolean): number => {
    const position = (current - origin) / rung;
    const nearest = Math.round(position);
    const base = Math.abs(position - nearest) <= 1e-9 * Math.max(1, Math.abs(nearest)) ? nearest : forward ? Math.floor(position) : Math.ceil(position);
    return clamp(roundUiNumber(origin + (forward ? base + rungs : base - rungs) * rung, digits));
  };
  const finite = snaps.filter((snap) => Number.isFinite(snap));
  const settle = (value: number): number => finite.find((snap) => Math.abs(value - snap) <= 1e-9 * rung * Math.max(1, Math.abs((snap - origin) / rung))) ?? value;
  const page = (forward: boolean): number => {
    const target = walk(SLIDER_PAGE_STEPS, forward);
    const reached = finite.filter((snap) => (forward ? snap > current && snap <= target : snap < current && snap >= target));
    return reached.length === 0 ? settle(target) : forward ? Math.min(...reached) : Math.max(...reached);
  };
  switch (key) {
    case "increment": return settle(walk(large ? SLIDER_PAGE_STEPS : 1, true));
    case "decrement": return settle(walk(large ? SLIDER_PAGE_STEPS : 1, false));
    case "pageUp": return page(true);
    case "pageDown": return page(false);
    case "home": return lower ?? current;
    case "end": return upper ?? current;
  }
}

/** 🎨️ `rgba` (sRGB components in 0..1, straight alpha; a missing component 0, a missing alpha 1, a non-finite one 0) as a lowercase `#rrggbb`, or `#rrggbbaa` with `alpha`, each channel `round(c × 255)`; the twin of `ui_color_hex`. */
export function uiColorHex(rgba: readonly number[], alpha: boolean): string {
  const channel = (index: number): string => {
    const component = rgba[index] ?? (index === 3 ? 1 : 0);
    return Math.round((Number.isFinite(component) ? Math.min(1, Math.max(0, component)) : 0) * 255).toString(16).padStart(2, "0");
  };
  return `#${[0, 1, 2, ...(alpha ? [3] : [])].map(channel).join("")}`;
}

/** 🧪️ Reads a typed hex colour — an optional `#`, then 3, 4, 6 or 8 hex digits in any case, surrounding space ignored, the short forms doubling each digit — as components `channel / 255` with alpha 1 when absent, else `null`; the twin of `parse_ui_color_hex`. */
export function parseUiColorHex(text: string): [number, number, number, number] | null {
  const trimmed = text.trim();
  const digits = trimmed.startsWith("#") ? trimmed.slice(1) : trimmed;
  const width = digits.length === 3 || digits.length === 4 ? 1 : digits.length === 6 || digits.length === 8 ? 2 : 0;
  if (width === 0 || !/^[0-9a-fA-F]+$/.test(digits)) return null;
  const rgba: [number, number, number, number] = [0, 0, 0, 1];
  for (let index = 0; index < digits.length / width; index++) {
    const channel = Number.parseInt(digits.slice(index * width, (index + 1) * width), 16);
    rgba[index] = (width === 1 ? channel * 17 : channel) / 255;
  }
  return rgba;
}
