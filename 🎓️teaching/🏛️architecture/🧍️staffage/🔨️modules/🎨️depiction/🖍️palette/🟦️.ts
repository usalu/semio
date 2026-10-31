/** 🖍️ The colour system of the depiction: brand-token themes, the greyscale tone ramp of solid views and the warm/cool diverging ramp of every analytic view.
 *
 * Tokens come from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`: dark `#001117`, light `#f7f3e3`, primary `#ff344f`, secondary `#34d1bf`,
 * tertiary `#fa9500`. Analytic ramps follow the shared contract: compression warm (`#d7301f → #fdae61`), tension cool (`#2c7bb6 → #abd9e9`),
 * zero neutral grey; legend stop positions are normalised `0..1` between `min` and `max`.
 *
 * @see ../../🔗️contracts/🟦️.ts — {@link Legend}
 * @see ../../🏗️structure/🟦️.ts — `STRUCTURAL_COLORS`
 */
import type { Text } from "../../../🧬️schema/🟦️.ts";
import type { DisplayRange, Legend } from "../../🔗️contracts/🟦️.ts";
import { STRUCTURAL_COLORS } from "../../🏗️structure/🟦️.ts";

/** 🌗️ The two drawing themes of the line and solid views. */
export type Theme = "dark" | "light";

/** 🌗️ Background, ink and tone-ramp ends of a theme. */
export type ThemeColours = { readonly background: string; readonly ink: string; readonly softInk: string; readonly toneDark: string; readonly toneLight: string };

/** 🌗️ The brand themes. */
export const THEMES: Readonly<Record<Theme, ThemeColours>> = {
  dark: { background: "#001117", ink: "#f7f3e3", softInk: "#7b827d", toneDark: "#2e3c3d", toneLight: "#f1edde" },
  light: { background: "#f7f3e3", ink: "#001117", softInk: "#7b827d", toneDark: "#243235", toneLight: "#d3d2c5" },
};

/** 🏷️ A bilingual text. */
export const tx = (en: string, de: string): Text => ({ en, de });

function parse(hex: string): [number, number, number] {
  const v = Number.parseInt(hex.slice(1), 16);
  return [(v >> 16) & 255, (v >> 8) & 255, v & 255];
}

/** 🥣️ The linear sRGB mix of two `#rrggbb` colours at fraction `t` of the second. */
export function mix(a: string, b: string, t: number): string {
  const [r0, g0, b0] = parse(a);
  const [r1, g1, b1] = parse(b);
  const hex = (x: number): string => Math.round(x).toString(16).padStart(2, "0");
  return `#${hex(r0 + (r1 - r0) * t)}${hex(g0 + (g1 - g0) * t)}${hex(b0 + (b1 - b0) * t)}`;
}

/** 💡️ Relative luminance of a `#rrggbb` colour. */
export function luminance(hex: string): number {
  const lin = (c: number): number => ((c /= 255) <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
  const [r, g, b] = parse(hex);
  return 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
}

/** 🩶️ The greyscale of a tone `0..1` (0 darkest material, 1 lightest) in a theme. */
export const toneColour = (theme: Theme, tone: number): string => mix(THEMES[theme].toneDark, THEMES[theme].toneLight, Math.max(0, Math.min(1, tone)));

const OVERLAY = {
  light: { applied: mix("#ff344f", "#001117", 0.18), reaction: mix("#34d1bf", "#001117", 0.45), weight: "#555f5d", joint: mix("#fa9500", "#001117", 0.3), vapour: mix("#34d1bf", "#2c7bb6", 0.45) },
  dark: { applied: "#ff6177", reaction: "#34d1bf", weight: "#a2a59d", joint: "#fa9500", vapour: "#5fe0cf" },
} as const;

/** 🏹️ Overlay colours of the analytic views (applied load, reaction, weight, joint moment, vapour), legible on the theme's paper. */
export const overlayColours = (theme: Theme): (typeof OVERLAY)[Theme] => OVERLAY[theme];

/** 🎚️ A diverging scale: `centre` maps to neutral grey, `min` and `max` to the strong colours. */
export type Scale = { readonly min: number; readonly centre: number; readonly max: number };

/** 🌈️ The diverging colour of a value; the `high` side is warm for temperatures and cool for tension; on the dark theme the strong ends are lifted toward the ink for contrast. */
export function divergingColour(value: number, scale: Scale, highWarm: boolean, theme: Theme = "light"): string {
  const c = STRUCTURAL_COLORS;
  const d = value - scale.centre;
  if (Math.abs(d) < 1e-12) return c.zero;
  const span = d > 0 ? scale.max - scale.centre : scale.centre - scale.min;
  const f = span <= 0 ? 1 : Math.min(1, Math.abs(d) / span);
  const warm = d > 0 === highWarm;
  const [weak, strong] = warm ? [c.compressionWeak, c.compressionStrong] : [c.tensionWeak, c.tensionStrong];
  return mix(weak, theme === "dark" ? mix(strong, "#f7f3e3", 0.25) : strong, f);
}

/** 🧮️ A symmetric-or-fixed scale for a set of values: auto spans the largest deviation from the centre (at least `minSpan`), fixed uses the given range, shared uses the documented comparison range `shared` of the quantity (auto when the caller has none). */
export function scaleOf(values: readonly number[], centre: number, display: DisplayRange, minSpan: number, shared?: { readonly min: number; readonly max: number }): Scale {
  const range = display.mode === "shared" ? (shared === undefined ? ({ mode: "auto" } as const) : ({ mode: "fixed", ...shared } as const)) : display;
  if (range.mode === "fixed") return { min: range.min, centre: Math.min(Math.max(centre, range.min), range.max), max: range.max };
  const m = Math.max(minSpan, ...values.map((v) => Math.abs(v - centre)));
  return { min: centre - m, centre, max: centre + m };
}

/** 🏷️ A legend with nine colour stops sampled along the scale; stop positions are values in the legend unit. */
export function legendOf(spec: { readonly title: Text; readonly quantity: string; readonly unit: string; readonly convention: Text; readonly interpretation: Text; readonly scale: Scale; readonly colourAt: (value: number) => string; readonly range?: DisplayRange["mode"]; readonly conditions?: Text; readonly assumptions?: readonly Text[] }): Legend {
  const { min, max, centre } = spec.scale;
  const values = [...Array.from({ length: 9 }, (_, i) => min + ((max - min) * i) / 8), centre].sort((a, b) => a - b);
  return { title: spec.title, quantity: spec.quantity, unit: spec.unit, convention: spec.convention, min, max, stops: values.map((v) => ({ at: max === min ? 0 : (v - min) / (max - min), color: spec.colourAt(v) })), interpretation: spec.interpretation, range: spec.range ?? "auto", ...(spec.conditions ? { conditions: spec.conditions } : {}), ...(spec.assumptions ? { assumptions: spec.assumptions } : {}) };
}

/** ✍️ Greedy word wrap at a character width. */
export function wrap(text: string, width: number): string[] {
  const lines: string[] = [];
  let line = "";
  for (const word of text.split(/\s+/u)) {
    if (line && line.length + 1 + word.length > width) {
      lines.push(line);
      line = word;
    } else line = line ? `${line} ${word}` : word;
  }
  return line ? [...lines, line] : lines;
}
