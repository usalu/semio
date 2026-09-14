/** 🎨 Themes: the TypeScript twin of `semio-viz-theme`. The categorical, sequential and diverging
 * palettes are not restated here — they are read from the one generator that also writes
 * `semio-tokens.sty`, so the LaTeX and the TypeScript subject cannot drift apart.
 * @see ../../🎨print-design-token-paints/🟦️.ts
 * @see ../../../🖋️latex/semio-viz-theme.sty
 */
import { loadPrintDesignTokens, renderVisualizationPalette, type PrintTheme } from "../../🎨print-design-token-paints/🟦️.ts";

//#region 🔖️Palette
/** 🎨️ An sRGB colour with an alpha channel, in the byte range the scene graph uses. */
export type VizRgba = readonly [number, number, number, number];

/** 🎨️ The resolved palettes of one appearance. */
export type VizPalette = {
  readonly appearance: PrintTheme;
  /** 🎨️ The categorical wheel: one hex per presence hue, in token order. */
  readonly categorical: readonly string[];
  /** 🎨️ Named multi-stop schemes, sequential and diverging, as hex stops. */
  readonly schemes: Readonly<Record<string, readonly string[]>>;
};

function parsePalette(lines: readonly string[], appearance: PrintTheme): VizPalette {
  const categorical: string[] = [];
  const schemes: Record<string, string[]> = {};
  const colorPattern = new RegExp(`^\\\\definecolor\\{semio-presence-${appearance}-(\\d+)\\}\\{HTML\\}\\{([0-9A-Fa-f]{6})\\}$`);
  const schemePattern = new RegExp(`^([a-z-]+)\\s*/\\s*${appearance}\\s*=\\s*\\{\\s*([0-9A-Fa-f,]+)\\s*\\}\\s*,$`);
  for (const line of lines) {
    const color = colorPattern.exec(line.trim());
    if (color !== null) {
      categorical[Number(color[1])] = `#${color[2]!.toLowerCase()}`;
      continue;
    }
    const scheme = schemePattern.exec(line.trim());
    if (scheme !== null) schemes[scheme[1]!] = scheme[2]!.split(",").map((stop) => `#${stop.toLowerCase()}`);
  }
  if (categorical.length === 0) throw new Error(`the design tokens carry no presence hues for the ${appearance} appearance`);
  return { appearance, categorical, schemes };
}

/** 🎨️ Loads the palettes of one appearance from the design tokens. */
export function vizPalette(appearance: PrintTheme): VizPalette {
  return parsePalette(renderVisualizationPalette(loadPrintDesignTokens()), appearance);
}

/** 🎨️ The categorical colour of one series index; the wheel cycles. */
export function vizCategoricalColor(palette: VizPalette, index: number): string {
  return palette.categorical[((index % palette.categorical.length) + palette.categorical.length) % palette.categorical.length]!;
}
//#endregion 🔖️Palette

//#region 🔖️Color
/** 🎨️ Parses `#rgb`, `#rrggbb` or `#rrggbbaa` into bytes. */
export function vizParseColor(hex: string): VizRgba {
  const text = hex.replace(/^#/, "");
  if (text.length === 3) return [Number.parseInt(text[0]! + text[0]!, 16), Number.parseInt(text[1]! + text[1]!, 16), Number.parseInt(text[2]! + text[2]!, 16), 255];
  if (text.length === 6) return [Number.parseInt(text.slice(0, 2), 16), Number.parseInt(text.slice(2, 4), 16), Number.parseInt(text.slice(4, 6), 16), 255];
  if (text.length === 8) return [Number.parseInt(text.slice(0, 2), 16), Number.parseInt(text.slice(2, 4), 16), Number.parseInt(text.slice(4, 6), 16), Number.parseInt(text.slice(6, 8), 16)];
  throw new Error(`invalid colour: ${hex}`);
}

/** 🎨️ Formats bytes back to `#rrggbb`, dropping a fully opaque alpha. */
export function vizFormatColor(color: VizRgba): string {
  const byte = (value: number): string => Math.max(0, Math.min(255, Math.round(value))).toString(16).padStart(2, "0");
  return `#${byte(color[0])}${byte(color[1])}${byte(color[2])}${color[3] >= 255 ? "" : byte(color[3])}`;
}

/** 🎨️ Linear sRGB interpolation between two colours, d3's `interpolateRgb` at gamma 1. */
export function vizInterpolateRgb(a: string, b: string): (t: number) => string {
  const from = vizParseColor(a);
  const to = vizParseColor(b);
  return (t) => vizFormatColor([from[0] + (to[0] - from[0]) * t, from[1] + (to[1] - from[1]) * t, from[2] + (to[2] - from[2]) * t, from[3] + (to[3] - from[3]) * t]);
}

/** 🎨️ A piecewise interpolator over the stops of a named scheme. */
export function vizSchemeInterpolator(palette: VizPalette, scheme: string): (t: number) => string {
  const stops = palette.schemes[scheme];
  if (stops === undefined) throw new Error(`unknown scheme ${scheme}; the tokens define ${Object.keys(palette.schemes).join(", ")}`);
  const n = stops.length - 1;
  return (t) => {
    const clamped = Math.max(0, Math.min(1, t));
    const i = Math.max(0, Math.min(n - 1, Math.floor(clamped * n)));
    return vizInterpolateRgb(stops[i]!, stops[i + 1]!)(clamped * n - i);
  };
}

/** 🎨️ Relative luminance by the WCAG definition — the grayscale-safety test. */
export function vizLuminance(hex: string): number {
  const [r, g, b] = vizParseColor(hex);
  const channel = (value: number): number => {
    const c = value / 255;
    return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
}

/** 🎨️ WCAG contrast ratio between two colours, from 1 to 21. */
export function vizContrastRatio(a: string, b: string): number {
  const la = vizLuminance(a);
  const lb = vizLuminance(b);
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
}
//#endregion 🔖️Color

//#region 🔖️Encodings
/** 🖍️ The dash patterns that carry a series when colour alone cannot, in millimetres. */
export const VIZ_DASH_PATTERNS: readonly (readonly number[])[] = [[], [2, 1], [0.5, 1], [3, 1, 0.5, 1], [1, 1], [4, 1.5], [2, 1, 0.5, 1, 0.5, 1]];

/** 🖍️ The hatch patterns that encode a category in a grayscale print. */
export const VIZ_HATCH_PATTERNS = ["solid", "north-east-lines", "north-west-lines", "horizontal-lines", "vertical-lines", "crosshatch", "dots", "grid"] as const;

export type VizHatchPattern = (typeof VIZ_HATCH_PATTERNS)[number];

/** 🖍️ The dash pattern of one series index; the list cycles. */
export function vizDashPattern(index: number): readonly number[] {
  return VIZ_DASH_PATTERNS[((index % VIZ_DASH_PATTERNS.length) + VIZ_DASH_PATTERNS.length) % VIZ_DASH_PATTERNS.length]!;
}

/** 🖍️ The hatch pattern of one series index; the list cycles. */
export function vizHatchPattern(index: number): VizHatchPattern {
  return VIZ_HATCH_PATTERNS[((index % VIZ_HATCH_PATTERNS.length) + VIZ_HATCH_PATTERNS.length) % VIZ_HATCH_PATTERNS.length]!;
}
//#endregion 🔖️Encodings

//#region 🔖️Theme
/** 🎨️ Everything a renderer needs to draw one chart in one appearance. */
export type VizTheme = {
  readonly name: string;
  readonly appearance: PrintTheme;
  readonly palette: VizPalette;
  readonly strokes: Readonly<Record<string, number>>;
  readonly fonts: Readonly<Record<string, readonly string[]>>;
  readonly typography: Readonly<Record<string, number>>;
};

/** 🎨️ `\SemioVizTheme`: resolves the named theme for one appearance out of the design tokens. */
export function vizTheme(appearance: PrintTheme, name = "semio"): VizTheme {
  const tokens = loadPrintDesignTokens();
  const strokes: Record<string, number> = {};
  for (const [key, value] of Object.entries(tokens.strokes ?? {})) if (typeof value === "number") strokes[key] = value;
  const fonts: Record<string, readonly string[]> = {};
  for (const [key, value] of Object.entries((tokens as { fontStacks?: Record<string, string[]> }).fontStacks ?? {})) fonts[key] = value;
  const typography: Record<string, number> = {};
  for (const [key, value] of Object.entries(tokens.metrics?.typography ?? {})) if (typeof value === "number") typography[key] = value;
  return { name, appearance, palette: vizPalette(appearance), strokes, fonts, typography };
}
//#endregion 🔖️Theme
