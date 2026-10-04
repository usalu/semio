/** 🎨 Themes: the TypeScript twin of `semio-viz-theme`. The categorical, sequential and diverging
 * palettes are not restated here — they are read from the one generator that also writes
 * `semio-tokens.sty`, so the LaTeX and the TypeScript subject cannot drift apart.
 * @see ../../🎨print-design-token-paints/🟦️.ts
 * @see ../../../🖋️latex/semio-viz-theme.sty
 */
import colorContract from "../../📸️snapshot/📊️chart/🎨️color/🔣️.json";
import { oklabMix } from "../../../../../🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts";
import { loadPrintDesignTokens, renderPrintLatexTokenStylesheet, renderVisualizationPalette, type PrintTheme } from "../../../🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts";

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
/** 🎨️ Parses owned CSS named, hexadecimal, RGB and HSL paints into byte channels. */
export function vizParseColor(color: string): VizRgba {
  const value = color.trim().toLowerCase(), names = colorContract["x-semio-named-colors"] as Readonly<Record<string,string>>;
  if (value === "transparent") return [0,0,0,0];
  const text = names[value] ?? (/^#[\da-f]{3,4}$|^#[\da-f]{6}(?:[\da-f]{2})?$/.test(value) ? value.slice(1) : undefined);
  if (text !== undefined) {
    const expanded = text.length <= 4 ? [...text].map(character=>character+character).join("") : text;
    return [Number.parseInt(expanded.slice(0,2),16),Number.parseInt(expanded.slice(2,4),16),Number.parseInt(expanded.slice(4,6),16),expanded.length===8?Number.parseInt(expanded.slice(6,8),16):255];
  }
  const functional = /^(rgba?|hsla?)\((.*)\)$/.exec(value);
  const invalid = (): never => { throw new Error("invalid colour: " + color); };
  if (!functional) return invalid();
  const parts = functional[2]!.trim().split(/\s*[,/]\s*|\s+/).filter(Boolean);
  if (parts.length < 3 || parts.length > 4) return invalid();
  const numeric = (part: string,percent: number): number => { if(!/^[+-]?(?:\d*\.\d+|\d+\.?\d*)(?:e[+-]?\d+)?%?$/.test(part))return invalid();const number=Number.parseFloat(part);return part.endsWith("%")?number*percent/100:number; };
  const bound = (number: number,max: number) => Math.max(0,Math.min(max,number));
  const alpha = parts.length===4 ? bound(numeric(parts[3]!,1),1)*255 : 255;
  if (functional[1]!.startsWith("rgb")) return [bound(numeric(parts[0]!,255),255),bound(numeric(parts[1]!,255),255),bound(numeric(parts[2]!,255),255),alpha];
  if(!parts[1]!.endsWith("%")||!parts[2]!.endsWith("%"))return invalid();
  const angle = /^([+-]?(?:\d*\.\d+|\d+\.?\d*)(?:e[+-]?\d+)?)(deg|rad|grad|turn)?$/.exec(parts[0]!);
  if(!angle)return invalid();
  const hue = ((Number(angle[1])*(angle[2]==="rad"?180/Math.PI:angle[2]==="grad"?0.9:angle[2]==="turn"?360:1)%360)+360)%360;
  const saturation = bound(numeric(parts[1]!,1),1),light = bound(numeric(parts[2]!,1),1);
  const chroma = (1-Math.abs(2*light-1))*saturation, secondary = chroma*(1-Math.abs(hue/60%2-1)),base=light-chroma/2;
  const channels = hue<60?[chroma,secondary,0]:hue<120?[secondary,chroma,0]:hue<180?[0,chroma,secondary]:hue<240?[0,secondary,chroma]:hue<300?[secondary,0,chroma]:[chroma,0,secondary];
  return [(channels[0]!+base)*255,(channels[1]!+base)*255,(channels[2]!+base)*255,alpha];
}

/** 🎨️ Formats bytes back to `#rrggbb`, dropping a fully opaque alpha. */
export function vizFormatColor(color: VizRgba): string {
  const byte = (value: number): string => Math.max(0, Math.min(255, Math.round(value))).toString(16).padStart(2, "0");
  return `#${byte(color[0])}${byte(color[1])}${byte(color[2])}${color[3] >= 255 ? "" : byte(color[3])}`;
}

/** 🫧️ Keeps authored transparency precise while opaque interpolation remains hexadecimal. */
function interpolatedColor(color: VizRgba): string {
  if (color[3] >= 255) return vizFormatColor(color);
  return "rgba("+color.slice(0,3).map(value=>Math.round(Math.max(0,Math.min(255,value)))).join(",")+","+Math.max(0,color[3]/255)+")";
}

/** 🪟️ Missing color channels of transparent endpoints inherit the visible endpoint. */
function interpolationColors(a: string,b: string): readonly [VizRgba,VizRgba] {
  let from=vizParseColor(a),to=vizParseColor(b);
  if(from[3]===0)from=[to[0],to[1],to[2],0];
  if(to[3]===0)to=[from[0],from[1],from[2],0];
  return [from,to];
}

/** 🎨️ Linear sRGB interpolation between two colours, d3's `interpolateRgb` at gamma 1. */
export function vizInterpolateRgb(a: string, b: string): (t: number) => string {
  const [from,to] = interpolationColors(a,b);
  return (t) => interpolatedColor([from[0] + (to[0] - from[0]) * t, from[1] + (to[1] - from[1]) * t, from[2] + (to[2] - from[2]) * t, from[3] + (to[3] - from[3]) * t]);
}

/** 🎨️ The authored color spaces shared by numerical and native chart inference. */
export type VizColorSpace = "rgb" | "lab" | "hcl" | "oklab";

/** 🎨️ Converts byte sRGB to CIE Lab with the D50 reference white used by the grammar. */
function colorLab(color: VizRgba): readonly [number, number, number] {
  const linear = (byte: number) => byte / 255 <= 0.04045 ? byte / (255 * 12.92) : ((byte / 255 + 0.055) / 1.055) ** 2.4;
  const [r,g,b] = color.slice(0,3).map(linear);
  const axis = (value: number) => value > (6/29) ** 3 ? Math.cbrt(value) : value / (3 * (6/29) ** 2) + 4/29;
  const y = axis(0.2225045 * r! + 0.7168786 * g! + 0.0606169 * b!);
  const x = r === g && g === b ? y : axis((0.4360747 * r! + 0.3850649 * g! + 0.1430804 * b!) / 0.96422);
  const z = r === g && g === b ? y : axis((0.0139322 * r! + 0.0971045 * g! + 0.7141733 * b!) / 0.82521);
  return [116 * y - 16, 500 * (x - y), 200 * (y - z)];
}

/** 🎨️ Resolves CIE Lab back into byte sRGB for print and scene output. */
function labColor(lab: readonly number[], alpha: number): VizRgba {
  const inverse = (value: number) => value > 6/29 ? value ** 3 : 3 * (6/29) ** 2 * (value - 4/29);
  const middle = (lab[0]! + 16) / 116;
  const x = 0.96422 * inverse(middle + lab[1]! / 500), y = inverse(middle), z = 0.82521 * inverse(middle - lab[2]! / 200);
  const byte = (value: number) => 255 * (value <= 0.0031308 ? 12.92 * value : 1.055 * value ** (1/2.4) - 0.055);
  return [byte(3.1338561*x - 1.6168667*y - 0.4906146*z), byte(-0.9787684*x + 1.9161415*y + 0.033454*z), byte(0.0719453*x - 0.2289914*y + 1.4052427*z), alpha];
}

/** 🌈️ Interpolates an authored pair in the same four color spaces as the native grammar. */
export function vizInterpolateColor(space: VizColorSpace, a: string, b: string): (t: number) => string {
  if (space === "rgb") return vizInterpolateRgb(a,b);
  const [from,to] = interpolationColors(a,b);
  if (space === "oklab") return value => { const mixed=oklabMix([...from],[...to],value);return interpolatedColor([mixed[0],mixed[1],mixed[2],from[3]+(to[3]-from[3])*value]); };
  const start = colorLab(from), end = colorLab(to);
  const mix = (a: number,b: number,value: number) => a + (b-a)*value;
  if (space === "lab") return value => interpolatedColor(labColor(start.map((part,index)=>mix(part,end[index]!,value)),mix(from[3],to[3],value)));
  if (space !== "hcl") throw new Error("unknown color interpolator: " + space);
  const polar = (lab: readonly number[]) => { const chroma = Math.hypot(lab[1]!,lab[2]!); return { hue:chroma === 0 ? Number.NaN : Math.atan2(lab[2]!,lab[1]!), chroma:lab[0]! <= 0 || lab[0]! >= 100 ? Number.NaN : chroma }; };
  const left = polar(start), right = polar(end);
  const hue = Number.isNaN(left.hue) ? right.hue : left.hue;
  const target = Number.isNaN(right.hue) ? hue : right.hue;
  let difference = target-hue; if(difference>Math.PI)difference-=2*Math.PI;else if(difference< -Math.PI)difference+=2*Math.PI;
  const chroma = Number.isNaN(left.chroma) ? right.chroma : left.chroma;
  const targetChroma = Number.isNaN(right.chroma) ? chroma : right.chroma;
  return value => { const angle = hue+difference*value, radius = mix(chroma,targetChroma,value); return interpolatedColor(labColor([mix(start[0]!,end[0]!,value),Number.isNaN(angle)?0:Math.cos(angle)*radius,Number.isNaN(angle)?0:Math.sin(angle)*radius],mix(from[3],to[3],value))); };
}

/** 🎨️ A piecewise interpolator over the stops of a named scheme. */
export function vizSchemeInterpolator(palette: VizPalette, scheme: string, space: VizColorSpace = "rgb"): (t: number) => string {
  const stops = palette.schemes[scheme];
  if (stops === undefined) throw new Error(`unknown scheme ${scheme}; the tokens define ${Object.keys(palette.schemes).join(", ")}`);
  const n = stops.length - 1;
  return (t) => {
    const clamped = Math.max(0, Math.min(1, t));
    const i = Math.max(0, Math.min(n - 1, Math.floor(clamped * n)));
    return vizInterpolateColor(space, stops[i]!, stops[i + 1]!)(clamped * n - i);
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
  readonly chrome: Readonly<Record<string, string>>;
  readonly fonts: Readonly<Record<string, readonly string[]>>;
  readonly typography: Readonly<Record<string, number>>;
};

/** 🎨️ `\SemioVizTheme`: resolves the named theme for one appearance out of the design tokens. */
export function vizTheme(appearance: PrintTheme, name = "semio"): VizTheme {
  const tokens = loadPrintDesignTokens();
  const strokes: Record<string, number> = {};
  for (const [key, value] of Object.entries(tokens.strokes ?? {})) if (typeof value === "number") strokes[key] = value;
  const fonts: Record<string, readonly string[]> = {};
  for (const [key, value] of Object.entries(tokens.fontStacks ?? {})) fonts[key] = [...value.matchAll(/"([^"]+)"|'([^']+)'|([^,]+)/g)].map(match => (match[1] ?? match[2] ?? match[3]!).trim());
  const typography: Record<string, number> = {};
  for (const [key, value] of Object.entries(tokens.metrics?.typography ?? {})) if (typeof value === "number") typography[key] = value;
  const chrome: Record<string,string> = {};
  for(const line of renderPrintLatexTokenStylesheet(tokens).split("\n")){const match=/^\\definecolor\{semio-chrome-(light|dark)-([^}]+)\}\{HTML\}\{([0-9a-fA-F]{6})\}$/.exec(line);if(match?.[1]===appearance)chrome[match[2]!]="#"+match[3]!.toLowerCase();}
  return { name, appearance, palette: vizPalette(appearance), strokes, chrome, fonts, typography };
}
//#endregion 🔖️Theme
