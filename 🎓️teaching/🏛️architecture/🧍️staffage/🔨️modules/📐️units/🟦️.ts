/** 📐️ Units, drawing scales, paper conversion, line weights and quantity formatting for the staffage archive.
 *
 * Model space is SI metres; paper space is millimetres. `mmPerMetre = 1000 / denominator`, so a 1.80 m person measures 3.6, 9, 18 and 36 mm at
 * 1:500, 1:200, 1:100 and 1:50 and at every custom scale alike. Pens, hatches and detail belong to the individually authored representations,
 * never to this module.
 *
 * @see ../../🧬️schema/🟦️.ts — `Language`
 * @see ../🔗️contracts/🟦️.ts — `DrawingScale`
 * @see https://www.iso.org/standard/4125.html — ISO 128-20 line conventions and pen series
 */
import type { Language } from "../../🧬️schema/🟦️.ts";
import type { DrawingScale } from "../🔗️contracts/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🖨️ The four standard drawing scale denominators, smallest scale first. */
export const SCALE_PRESETS = [500, 200, 100, 50] as const;

/** 🖥️ CSS pixels per paper millimetre at 100 % viewport zoom (96 dpi). */
export const CSS_PX_PER_MM = 96 / 25.4;

/** 🔤️ Annotation text height in paper millimetres, constant in paper space. */
export const ANNOTATION_TEXT_MM = 2.5;

/** ✒️ The thin pen of annotations (dimension lines, scale bars, legend frames) in paper millimetres: ISO 128 thin line 0.13 mm. */
export const ANNOTATION_PEN_MM = 0.13;

/** 🏷️ Label text height in paper millimetres, constant in paper space. */
export const LABEL_TEXT_MM = 1.8;

/** 📄️ Paper millimetres a real length occupies at a denominator: `metres · 1000 / denominator`. */
export function paperMm(metres: number, denominator: number): number {
  return (metres * 1000) / denominator;
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

/** 🔍️ The on-screen size in CSS millimetres of a scene of `widthMm` × `heightMm` paper millimetres at a viewport zoom (1 = print size). Zoom is a pure viewport magnification: it never enters evaluation, representation selection or depiction. */
export function viewportMm(widthMm: number, heightMm: number, zoom: number): { readonly widthMm: number; readonly heightMm: number } {
  return { widthMm: widthMm * zoom, heightMm: heightMm * zoom };
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
  return Math.max(0, significant - 1 - Math.floor(portable.log10(magnitude)));
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

const HELVETICA = "278 278 355 556 556 889 667 191 333 333 389 584 278 333 278 278 556 556 556 556 556 556 556 556 556 556 278 278 584 584 584 556 1015 667 667 722 722 667 611 778 722 278 500 667 556 833 722 778 667 778 722 667 611 722 667 944 667 667 611 278 278 278 469 556 333 556 556 500 556 556 278 556 556 222 222 500 222 833 556 556 556 556 333 500 278 556 500 722 500 500 500 334 260 334 584".split(" ").map(Number);
const HELVETICA_BOLD = "278 333 474 556 556 889 722 238 333 333 389 584 278 333 278 278 556 556 556 556 556 556 556 556 556 556 333 333 584 584 584 611 975 722 722 722 722 667 611 778 722 278 556 722 611 833 722 778 667 778 722 667 611 722 667 944 667 667 611 333 278 333 584 556 333 556 611 556 611 556 333 611 611 278 278 556 278 889 611 611 611 611 389 556 333 611 556 778 556 556 500 389 280 389 584".split(" ").map(Number);
const SYMBOL_ADVANCE: Readonly<Record<string, readonly [number, number]>> = { "ß": [611, 611], "·": [278, 278], "–": [556, 556], "—": [1000, 1000], "×": [584, 584], "°": [400, 400], "µ": [556, 611], "²": [333, 333], "³": [333, 333], " ": [278, 278], "…": [1000, 1000], "±": [584, 584], "÷": [584, 584], "„": [333, 500], "“": [333, 500], "”": [333, 500], "’": [222, 278] };
const UNKNOWN_ADVANCE = 1000;

/** 🔡️ The width in millimetres of a text set in the font of the drawings (Helvetica, with Arial as its metric twin, regular or bold) at a height: the standard Adobe Helvetica advance widths, accents take their base letter, symbols outside the font count as one em. Used to reserve room for text and to wrap captions.
 * @see https://adobe-type-tools.github.io/font-info/ — Adobe core 14 font metrics */
export function textWidthMm(text: string, sizeMm: number, bold = false): number {
  let units = 0;
  for (const char of text) {
    const symbol = SYMBOL_ADVANCE[char];
    const code = char.charCodeAt(0);
    if (symbol !== undefined) units += symbol[bold ? 1 : 0];
    else if (code >= 32 && code <= 126) units += (bold ? HELVETICA_BOLD : HELVETICA)[code - 32]!;
    else {
      const base = char.normalize("NFD")[0]!;
      const baseCode = base.charCodeAt(0);
      units += baseCode >= 32 && baseCode <= 126 && base !== char ? (bold ? HELVETICA_BOLD : HELVETICA)[baseCode - 32]! : UNKNOWN_ADVANCE;
    }
  }
  return (units * sizeMm) / 1000;
}
