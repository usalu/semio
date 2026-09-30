import { formatUiNumber, roundUiNumber } from "../🔢️number-format/🟦️.ts";

/** 🧿️ Share of a slider's span within which a pointer value is pulled onto a detent — the twin of `SLIDER_SNAP_RADIUS`. */
export const SLIDER_SNAP_RADIUS = 0.03;

/** 🔬️ The largest precision a renderer honours — the twin of `UI_NUMBER_PRECISION_MAX`. */
export const UI_NUMBER_PRECISION_MAX = 15;

/** 👆️ Clamp, quantize onto the step ladder from `min`, then pull onto the nearest snap within the radius (first snap wins a tie); the twin of `slider_pointer_value`. */
export function sliderPointerValue(value: number, min: number, max: number, step: number, snaps: readonly number[]): number {
  const upper = Math.max(min, max);
  const clamped = Math.min(upper, Math.max(min, value));
  const stepped = step > 0 ? Math.min(upper, Math.max(min, min + Math.round((clamped - min) / step) * step)) : clamped;
  const radius = Math.abs(max - min) * SLIDER_SNAP_RADIUS;
  let nearest: number | null = null;
  let distance = Number.POSITIVE_INFINITY;
  for (const snap of snaps) {
    const gap = Math.abs(snap - clamped);
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

/** 📄️ How many ladder rungs a large arrow or a page key with no detent ahead moves a slider — the twin of `SLIDER_PAGE_STEPS`. */
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

/** ⌨️ The keyboard law of every numeric control: arrows walk the step ladder from `min` (from 0 without one; `large` walks {@link SLIDER_PAGE_STEPS} rungs) and never snap, page keys jump to the adjacent detent or walk ten rungs, Home/End go to the bounds (keeping the value without one), results clamp to the bounds; the twin of `ui_number_key_value`. */
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
  switch (key) {
    case "increment": return walk(large ? SLIDER_PAGE_STEPS : 1, true);
    case "decrement": return walk(large ? SLIDER_PAGE_STEPS : 1, false);
    case "pageUp": return sliderAdjacentSnap(current, snaps, true) ?? walk(SLIDER_PAGE_STEPS, true);
    case "pageDown": return sliderAdjacentSnap(current, snaps, false) ?? walk(SLIDER_PAGE_STEPS, false);
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
