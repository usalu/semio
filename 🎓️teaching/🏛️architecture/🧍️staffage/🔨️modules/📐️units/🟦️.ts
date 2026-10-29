/** 📐️ Units, drawing scales, paper conversion, line weights and quantity formatting for the staffage archive.
 *
 * Model space is SI metres; paper space is millimetres. `mmPerMetre = 1000 / denominator`, so a 1.80 m person measures 3.6, 9, 18 and 36 mm at
 * 1:500, 1:200, 1:100 and 1:50. Custom scales snap to the nearest preset by log-distance for line weights and level of detail.
 *
 * @see ../../🧬️schema/🟦️.ts — `ShapeRole`, `Lod`, `Language`
 * @see ../🔗️contracts/🟦️.ts — `DrawingScale`
 * @see https://www.iso.org/standard/4125.html — ISO 128-20 line conventions and pen series
 */
import type { Language, Lod, ShapeRole } from "../../🧬️schema/🟦️.ts";
import type { DrawingScale } from "../🔗️contracts/🟦️.ts";

/** 🖨️ The four standard drawing scale denominators, smallest scale first. */
export const SCALE_PRESETS = [500, 200, 100, 50] as const;

/** 🖥️ CSS pixels per paper millimetre at 100 % viewport zoom (96 dpi). */
export const CSS_PX_PER_MM = 96 / 25.4;

/** 🔤️ Annotation text height in paper millimetres, constant in paper space. */
export const ANNOTATION_TEXT_MM = 2.5;

/** 🏷️ Label text height in paper millimetres, constant in paper space. */
export const LABEL_TEXT_MM = 1.8;

const LINE_WEIGHTS: Readonly<Record<ShapeRole, readonly (number | null)[]>> = {
  body: [0.18, 0.25, 0.35, 0.5],
  detail: [0.13, 0.13, 0.18, 0.25],
  glazing: [0.13, 0.13, 0.18, 0.25],
  texture: [null, 0.13, 0.13, 0.13],
  ground: [0.25, 0.35, 0.5, 0.7],
};

/** 🎯️ The index into {@link SCALE_PRESETS} nearest to the denominator by `|log(d) − log(preset)|`; ties go to the larger scale (lower denominator). */
export function presetIndex(scale: DrawingScale): number {
  const logD = Math.log(scale.denominator);
  let best = 0;
  let bestDistance = Number.POSITIVE_INFINITY;
  for (let index = SCALE_PRESETS.length - 1; index >= 0; index--) {
    const distance = Math.abs(logD - Math.log(SCALE_PRESETS[index]!));
    if (distance < bestDistance) {
      best = index;
      bestDistance = distance;
    }
  }
  return best;
}

/** 📏️ Paper millimetres per model metre at the scale (`1000 / denominator`). */
export function mmPerMetre(scale: DrawingScale): number {
  return 1000 / scale.denominator;
}

/** 📄️ Paper millimetres a model length occupies at the scale. */
export function metresToMm(metres: number, scale: DrawingScale): number {
  return metres * mmPerMetre(scale);
}

/** 🌍️ Model metres a paper length stands for at the scale. */
export function mmToMetres(mm: number, scale: DrawingScale): number {
  return mm / mmPerMetre(scale);
}

/** 🔭️ The level of detail at the scale: denominator > 200 → 0, > 100 → 1, > 50 → 2, otherwise 3. */
export function lodFor(scale: DrawingScale): Lod {
  const d = scale.denominator;
  return d > 200 ? 0 : d > 100 ? 1 : d > 50 ? 2 : 3;
}

/** ✒️ The ISO 128 pen width in paper millimetres for a role at the scale, or `null` when the role is not drawn (texture at 1:500). */
export function lineWeightMm(scale: DrawingScale, role: ShapeRole): number | null {
  return LINE_WEIGHTS[role][presetIndex(scale)] ?? null;
}

/** 🔤️ The constant paper text height for a text kind. */
export function textSizeMm(kind: "annotation" | "label" = "annotation"): number {
  return kind === "annotation" ? ANNOTATION_TEXT_MM : LABEL_TEXT_MM;
}

/** ⚖️ The base units {@link formatQuantity} understands; the prefix is chosen from the magnitude. */
export const QUANTITY_UNITS = ["N", "Pa", "N·m", "°C", "W/m²", "W"] as const;

/** ⚖️ One of {@link QUANTITY_UNITS}. */
export type QuantityUnit = (typeof QUANTITY_UNITS)[number];

/** 🧾️ A number and its unit symbol after prefix selection. */
export type QuantityParts = { readonly value: number; readonly unit: string; readonly text: string };

const PREFIXES: Readonly<Record<QuantityUnit, readonly (readonly [number, string])[]>> = {
  N: [[1e3, "kN"], [0, "N"]],
  Pa: [[1e6, "MPa"], [1e3, "kPa"], [0, "Pa"]],
  "N·m": [[1e3, "kN·m"], [0, "N·m"]],
  "°C": [[0, "°C"]],
  "W/m²": [[0, "W/m²"]],
  W: [[1e3, "kW"], [0, "W"]],
};

/** 🔢️ Decimal digits that keep `significant` digits of a magnitude, never negative. */
export function decimalsFor(magnitude: number, significant: number): number {
  if (magnitude === 0 || !Number.isFinite(magnitude)) return Math.max(0, significant - 1);
  return Math.max(0, significant - 1 - Math.floor(Math.log10(magnitude)));
}

/** 🔠️ Fixed-point text with language decimal separator (`.` English, `,` German) and no negative zero. */
export function formatNumber(value: number, decimals: number, language: Language): string {
  const fixed = value.toFixed(decimals);
  const text = Number(fixed) === 0 ? fixed.replace("-", "") : fixed;
  return language === "de" ? text.replace(".", ",") : text;
}

/** 🧮️ A quantity expressed with the sensible prefix chosen on the rounded magnitude: N → kN from 1000, Pa → kPa → MPa, N·m → kN·m, W → kW; smaller magnitudes stay in the base unit. */
export function quantityParts(value: number, unit: QuantityUnit, language: Language, significant = 3): QuantityParts {
  const rounded = Number(Math.abs(value).toPrecision(significant));
  const [threshold, symbol] = PREFIXES[unit].find(([limit]) => rounded >= limit)!;
  const scaled = value / (threshold === 0 ? 1 : threshold);
  const decimals = value === 0 ? 0 : decimalsFor(Number(Math.abs(scaled).toPrecision(significant)), significant);
  return { value: Number(scaled.toFixed(decimals)), unit: symbol, text: `${formatNumber(scaled, decimals, language)}\u00a0${symbol}` };
}

/** 🧾️ `quantityParts(...).text`: e.g. `1.25 kN`, `12,5 kPa`, `21.4 °C`. */
export function formatQuantity(value: number, unit: QuantityUnit, language: Language, significant = 3): string {
  return quantityParts(value, unit, language, significant).text;
}
