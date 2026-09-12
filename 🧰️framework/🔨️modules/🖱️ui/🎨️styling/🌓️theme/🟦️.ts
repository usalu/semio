// #region 🧲️Header
/** @emoji 🎨️ `@semio-tech/ui-styling` centralizes palette CSS, Tailwind entry, and the shared typography preset for ui consumers. */
// #endregion 🧲️Header

export { tailwindConfig, tailwindConfig as default } from "../💨️tailwind/🎨️tailwind.config.ts";
export {
  STYLING_BOARD_PALETTES,
  STYLING_CANVAS_FONTS,
  STYLING_CANVAS_PALETTES,
  STYLING_MAP_PALETTES,
  STYLING_METRICS,
  STYLING_OPACITIES,
  STYLING_PRESENCE_PALETTES,
  STYLING_RADII,
  STYLING_SEMIO_THEME,
  STYLING_STROKES,
  STYLING_TOKENS,
  type StylingAppearanceName,
  type StylingTokenKey,
} from "../🤖️generated/🔤️tokens/🟦️.ts";
import { ephemeralMap, ephemeralBox, ephemeralSet } from "@semio-tech/framework";
import { STYLING_BOARD_PALETTES, STYLING_METRICS, STYLING_SEMIO_THEME, STYLING_TOKENS, type StylingAppearanceName, type StylingTokenKey } from "../🤖️generated/🔤️tokens/🟦️.ts";

//#region 🔖️ThemeModel
/** @emoji 🎨️ `UiTheme` model: paint-ref resolver, parse/serialize, shared by the token generator and the runtime theme engine. MUST stay dependency-free from `🤖️generated/🔤️tokens/🟦️.ts` (the generator produces that file; importing it here would create a cycle). */
//#region 🔖️types
/** @emoji 🖌️ A single paint reference: a primitive token, a literal hex, or a blend of two tokens. */
export interface ThemePaintRef {
  token?: string;
  hex?: string;
  alpha?: number;
  mix?: [string, string, number];
}

/** @emoji 🖌️ Resolved sRGB8888 color. */
export type Rgba8 = [number, number, number, number];

/** @emoji 🗂️ The four paint groups every appearance carries. */
export type ThemePaletteGroup = "board" | "map" | "canvas" | "chrome";

/** @emoji 🌓️ Light/dark palette dimension within a theme. */
export type ThemeAppearanceName = "light" | "dark";

/** @emoji 🖼️ Optional runtime icon appearance overrides keyed by compile-time icon ids. */
export interface UiThemeIcons {
  readonly aliases?: Readonly<Partial<Record<string, string>>>;
  readonly variants?: Readonly<Partial<Record<string, string>>>;
  readonly themedAliases?: Readonly<Partial<Record<string, string>>>;
  readonly themedVariants?: Readonly<Partial<Record<string, string>>>;
}

const THEME_PALETTE_GROUPS: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];

const THEME_APPEARANCE_NAMES: readonly ThemeAppearanceName[] = ["light", "dark"];

/** @emoji 🎨️ A named, fully editable design-token set (colors, spacing, fonts, strokes, radii, opacities, metrics, and light/dark appearance paints). */
export interface UiTheme {
  readonly id: string;
  readonly label: string;
  readonly colors: Record<string, string>;
  readonly spacing: Record<string, string>;
  readonly fontStacks: Record<string, string>;
  readonly canvasFonts: Record<string, string>;
  readonly strokes: Record<string, number | number[]>;
  readonly radii: Record<string, number>;
  readonly opacities: Record<string, number>;
  readonly metrics: Record<string, Record<string, number | number[]>>;
  readonly appearances: Record<ThemeAppearanceName, Record<ThemePaletteGroup, Record<string, ThemePaintRef>>>;
  readonly icons?: UiThemeIcons;
}
//#endregion 🔖️types

//#region 🔖️resolveTheme
function parseHex6(hex: string): [number, number, number] {
  const s = hex.trim().replace(/^#/, "");
  if (s.length === 3) {
    return [Number.parseInt(s[0]! + s[0], 16), Number.parseInt(s[1]! + s[1], 16), Number.parseInt(s[2]! + s[2], 16)];
  }
  const v = Number.parseInt(s, 16);
  return [(v >> 16) & 0xff, (v >> 8) & 0xff, v & 0xff];
}

function tokenHexOrThrow(colors: Record<string, string>, key: string): string {
  const v = colors[key];
  if (!v) {
    throw new Error(`theme colors[${key}] missing`);
  }
  return v;
}

function blendHex(a: string, b: string, ratioA: number): string {
  const [ar, ag, ab] = parseHex6(a);
  const [br, bg, bb] = parseHex6(b);
  const t = Math.min(1, Math.max(0, ratioA));
  const r = Math.round(ar * t + br * (1 - t));
  const g = Math.round(ag * t + bg * (1 - t));
  const bl = Math.round(ab * t + bb * (1 - t));
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${bl.toString(16).padStart(2, "0")}`;
}

/** @emoji 🖌️ Resolves a paint ref (token / hex / mix) against a theme's primitive colors to sRGB8888. */
export function resolveThemePaint(colors: Record<string, string>, ref: ThemePaintRef): Rgba8 {
  let hex: string;
  let alpha = ref.alpha ?? 1;
  if (ref.mix) {
    const [a, b, ratio] = ref.mix;
    const bHex = b === "transparent" ? "#000000" : tokenHexOrThrow(colors, b);
    hex = blendHex(tokenHexOrThrow(colors, a), bHex, ratio);
    if (b === "transparent" && ref.alpha === undefined) {
      alpha = 1 - ratio;
    }
  } else if (ref.hex) {
    hex = ref.hex;
  } else if (ref.token) {
    hex = tokenHexOrThrow(colors, ref.token);
  } else {
    throw new Error("paint ref needs token, hex, or mix");
  }
  const [r, g, b] = parseHex6(hex);
  return [r, g, b, Math.round(alpha * 255)];
}

/** @emoji 📏️ Derives dag component width as twice the IO channel column width (mirrors the 🔣️.json authoring shortcut). */
export function resolveThemeMetrics(metrics: UiTheme["metrics"]): UiTheme["metrics"] {
  const out = structuredClone(metrics ?? {});
  const dag = out.dag;
  if (dag && typeof dag.ioColumnWidth === "number") {
    dag.componentWidth = dag.ioColumnWidth * 2;
  }
  return out;
}

/** @emoji 🎨️ Resolves every paint in one appearance of a theme to sRGB8888, grouped by palette. */
export function resolveThemeAppearancePalettes(theme: UiTheme, appearance: ThemeAppearanceName): Record<ThemePaletteGroup, Record<string, Rgba8>> {
  const groups = theme.appearances[appearance];
  const out = {} as Record<ThemePaletteGroup, Record<string, Rgba8>>;
  for (const group of THEME_PALETTE_GROUPS) {
    const paints = groups[group] ?? {};
    const resolved: Record<string, Rgba8> = {};
    for (const [paintName, ref] of Object.entries(paints)) {
      resolved[paintName] = resolveThemePaint(theme.colors, ref);
    }
    out[group] = resolved;
  }
  return out;
}
//#endregion 🔖️resolveTheme

//#region 🔖️parseTheme
function isPlainObject(v: unknown): v is Record<string, unknown> {
  return typeof v === "object" && v !== null && !Array.isArray(v);
}

function requireRecord(value: unknown, path: string): Record<string, unknown> {
  if (!isPlainObject(value)) {
    throw new Error(`theme.${path} must be an object`);
  }
  return value;
}

function requireString(value: unknown, path: string): string {
  if (typeof value !== "string") {
    throw new Error(`theme.${path} must be a string`);
  }
  return value;
}

function requireStringMap(value: unknown, path: string): Record<string, string> {
  const obj = requireRecord(value, path);
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(obj)) {
    out[k] = requireString(v, `${path}.${k}`);
  }
  return out;
}

function parsePaintRef(value: unknown, path: string): ThemePaintRef {
  const obj = requireRecord(value, path);
  const ref: ThemePaintRef = {};
  if (obj.token !== undefined) ref.token = requireString(obj.token, `${path}.token`);
  if (obj.hex !== undefined) ref.hex = requireString(obj.hex, `${path}.hex`);
  if (obj.alpha !== undefined) {
    if (typeof obj.alpha !== "number") throw new Error(`theme.${path}.alpha must be a number`);
    ref.alpha = obj.alpha;
  }
  if (obj.mix !== undefined) {
    if (!Array.isArray(obj.mix) || obj.mix.length !== 3) throw new Error(`theme.${path}.mix must be [tokenA, tokenB, ratio]`);
    ref.mix = [
      requireString(obj.mix[0], `${path}.mix[0]`),
      requireString(obj.mix[1], `${path}.mix[1]`),
      typeof obj.mix[2] === "number"
        ? obj.mix[2]
        : (() => {
            throw new Error(`theme.${path}.mix[2] must be a number`);
          })(),
    ];
  }
  if (!ref.token && !ref.hex && !ref.mix) {
    throw new Error(`theme.${path} needs token, hex, or mix`);
  }
  return ref;
}

function parsePaletteGroup(value: unknown, path: string): Record<string, ThemePaintRef> {
  const obj = requireRecord(value, path);
  const out: Record<string, ThemePaintRef> = {};
  for (const [k, v] of Object.entries(obj)) {
    out[k] = parsePaintRef(v, `${path}.${k}`);
  }
  return out;
}

function parseStringMap(value: unknown, path: string): Record<string, string> {
  const obj = requireRecord(value, path);
  const out: Record<string, string> = {};
  for (const [k, v] of Object.entries(obj)) {
    if (typeof v !== "string") {
      throw new Error(`theme.${path}.${k} must be a string`);
    }
    out[k] = v;
  }
  return out;
}

function parseThemeIcons(value: unknown, path: string): UiThemeIcons {
  const obj = requireRecord(value, path);
  return {
    ...(obj.aliases === undefined ? {} : { aliases: parseStringMap(obj.aliases, `${path}.aliases`) }),
    ...(obj.variants === undefined ? {} : { variants: parseStringMap(obj.variants, `${path}.variants`) }),
    ...(obj.themedAliases === undefined ? {} : { themedAliases: parseStringMap(obj.themedAliases, `${path}.themedAliases`) }),
    ...(obj.themedVariants === undefined ? {} : { themedVariants: parseStringMap(obj.themedVariants, `${path}.themedVariants`) }),
  };
}

function parseAppearance(value: unknown, path: string): Record<ThemePaletteGroup, Record<string, ThemePaintRef>> {
  const obj = requireRecord(value, path);
  const out = {} as Record<ThemePaletteGroup, Record<string, ThemePaintRef>>;
  for (const group of THEME_PALETTE_GROUPS) {
    if (!(group in obj)) {
      throw new Error(`theme.${path} is missing palette group "${group}"`);
    }
    out[group] = parsePaletteGroup(obj[group], `${path}.${group}`);
  }
  return out;
}

function parseNumberOrArray(value: unknown, path: string): number | number[] {
  if (typeof value === "number") return value;
  if (Array.isArray(value) && value.every((x) => typeof x === "number")) return value as number[];
  throw new Error(`theme.${path} must be a number or number[]`);
}

function parseNumberMap(value: unknown, path: string): Record<string, number | number[]> {
  const obj = requireRecord(value, path);
  const out: Record<string, number | number[]> = {};
  for (const [k, v] of Object.entries(obj)) {
    out[k] = parseNumberOrArray(v, `${path}.${k}`);
  }
  return out;
}

function parseMetrics(value: unknown, path: string): UiTheme["metrics"] {
  const obj = requireRecord(value, path);
  const out: UiTheme["metrics"] = {};
  for (const [k, v] of Object.entries(obj)) {
    out[k] = parseNumberMap(v, `${path}.${k}`);
  }
  return out;
}

/** @emoji 🔎️ Strictly parses and validates a `UiTheme` (unknown token refs / missing palette groups throw). Every paint is resolved once to surface broken refs immediately. */
export function parseUiTheme(json: unknown): UiTheme {
  const obj = requireRecord(json, "");
  const colors = requireStringMap(obj.colors, "colors");
  const radiiObj = requireRecord(obj.radii, "radii");
  const radii: Record<string, number> = {};
  for (const [k, v] of Object.entries(radiiObj)) {
    if (typeof v !== "number") throw new Error(`theme.radii.${k} must be a number`);
    radii[k] = v;
  }
  const opacitiesObj = requireRecord(obj.opacities, "opacities");
  const opacities: Record<string, number> = {};
  for (const [k, v] of Object.entries(opacitiesObj)) {
    if (typeof v !== "number") throw new Error(`theme.opacities.${k} must be a number`);
    opacities[k] = v;
  }
  const appearancesObj = requireRecord(obj.appearances, "appearances");
  const appearances = {} as UiTheme["appearances"];
  for (const appearance of THEME_APPEARANCE_NAMES) {
    if (!(appearance in appearancesObj)) {
      throw new Error(`theme.appearances is missing "${appearance}"`);
    }
    appearances[appearance] = parseAppearance(appearancesObj[appearance], `appearances.${appearance}`);
  }
  const theme: UiTheme = {
    id: requireString(obj.id, "id"),
    label: requireString(obj.label, "label"),
    colors,
    spacing: requireStringMap(obj.spacing, "spacing"),
    fontStacks: requireStringMap(obj.fontStacks, "fontStacks"),
    canvasFonts: requireStringMap(obj.canvasFonts, "canvasFonts"),
    strokes: parseNumberMap(obj.strokes, "strokes"),
    radii,
    opacities,
    metrics: parseMetrics(obj.metrics, "metrics"),
    appearances,
    ...(obj.icons !== undefined ? { icons: parseThemeIcons(obj.icons, "icons") } : {}),
  };
  // Resolve every paint once so unknown token refs fail loudly at parse time.
  for (const appearance of THEME_APPEARANCE_NAMES) {
    resolveThemeAppearancePalettes(theme, appearance);
  }
  return theme;
}

/** @emoji 💾️ Serializes a `UiTheme` to canonical JSON. */
export function serializeUiTheme(theme: UiTheme): string {
  return JSON.stringify(theme, null, 2);
}
//#endregion 🔖️parseTheme

//#region 🧪️ThemeModelTests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../🧪️tests/🧪️theme-resolve/🟦️.ts");
  await registerTests1(import.meta.vitest, { parseUiTheme, resolveThemeAppearancePalettes, resolveThemeMetrics, resolveThemePaint, serializeUiTheme }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️ThemeModelTests
//#endregion 🔖️ThemeModel

//#region 🔖️sizing
//#region 🔑️SizeVars
/** @emoji 🔑️ Canonical DOM size CSS variable names. */
export const STYLING_SIZE_VAR = {
  uiSpacing: "--ui-spacing",
  spacingSingle: "--spacing-single",
  spacingDouble: "--spacing-double",
  sizeTiny: "--size-tiny",
  sizeSmall: "--size-small",
  sizeWorkbench: "--size-workbench",
  sizeMedium: "--size-medium",
  sizeLarge: "--size-large",
  sizeHuge: "--size-huge",
  sizeMega: "--size-mega",
  sizeGiga: "--size-giga",
  sizeXl: "--size-xl",
  sizeTera: "--size-tera",
  sizePeta: "--size-peta",
  layoutLabel: "--layout-label",
  layoutPopoverMin: "--layout-popover-min",
  layoutPanelMin: "--layout-panel-min",
  layoutPanelRail: "--layout-panel-rail",
  layoutPanelMax: "--layout-panel-max",
  layoutCommandMax: "--layout-command-max",
  layoutEngagementMax: "--layout-engagement-max",
  layoutDeckWidth: "--layout-deck-width",
  layoutDeckHeight: "--layout-deck-height",
  strokeHairline: "--stroke-hairline",
} as const;

/** @emoji 🔑️ Returns a `var(--…)` reference for a DOM size token. */
export function sizeVar(key: keyof typeof STYLING_SIZE_VAR): string {
  return `var(${STYLING_SIZE_VAR[key]})`;
}

/** @emoji 🔑️ Compact-mode reference root (px) for headless layout math at default 16px root. */
export const STYLING_COMPACT_ROOT_PX = 16;

const COMPACT_UI_SPACING_REM = 0.2;

/** @emoji 📐️ Converts a ui-spacing multiplier to rem length. */
export function uiSpacingRem(multiplier: number): string {
  return `${multiplier * COMPACT_UI_SPACING_REM}rem`;
}

/** @emoji 📐️ Converts a ui-spacing multiplier to px at the compact reference root. */
export function uiSpacingPx(multiplier: number, rootPx = STYLING_COMPACT_ROOT_PX): number {
  return multiplier * COMPACT_UI_SPACING_REM * rootPx;
}

/** @emoji 📐️ DOM layout multipliers (multiples of `--ui-spacing`) from 🔣️.json. */
export const STYLING_DOM = STYLING_METRICS.dom;

/** @emoji 📐️ Resolves a DOM metric key to px at the compact reference root. */
export function domSizePx(key: keyof typeof STYLING_DOM, rootPx = STYLING_COMPACT_ROOT_PX): number {
  return uiSpacingPx(STYLING_DOM[key], rootPx);
}

/** @emoji 📐️ Reads a resolved CSS size variable from the document (browser only). */
export function readSizeVarPx(varName: string, element?: Element | null): number {
  if (typeof document === "undefined") {
    return 0;
  }
  const host = element ?? document.documentElement;
  const raw = getComputedStyle(host).getPropertyValue(varName.startsWith("--") ? varName : `--${varName}`);
  return Number.parseFloat(raw) || 0;
}
//#endregion 🔑️SizeVars
//#endregion 🔖️sizing

//#region 🔖️resolve
//#region 🔑️TokenRefs
/** @emoji 🔑️ Builds a primitive palette CSS variable reference (`var(--color-<key>)`). */
export function tokenVar(key: StylingTokenKey | string): string {
  return `var(--color-${key.replaceAll("_", "-")})`;
}

/** @emoji 🔑️ Builds a semantic UI CSS variable reference (`var(--<name>)`). */
export function semanticVar(name: string): string {
  const trimmed = name.startsWith("--") ? name.slice(2) : name;
  return `var(--${trimmed})`;
}

/** @emoji 🔑️ Builds a Tailwind `@theme inline` color alias (`var(--color-<name>)`). */
export function themeColorVar(name: string): string {
  return `var(--color-${name.replaceAll("_", "-")})`;
}

/** @emoji 🧭️ Permanent X/Y/Z paints for gumball and view/projection gizmos — primary / secondary / tertiary, never active/hover chrome. */
export const SPATIAL_AXIS_COLOR_REFS = {
  x: tokenVar("primary"),
  y: tokenVar("secondary"),
  z: tokenVar("tertiary"),
} as const;

/** @emoji 🧭️ Resolved `#rrggbb` axis paints for spatial manipulators and navigation cubes. */
export function resolveSpatialAxisColors(): { readonly x: string; readonly y: string; readonly z: string } {
  return {
    x: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.x, "primary"),
    y: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.y, "secondary"),
    z: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.z, "tertiary"),
  };
}

/** @emoji 🔑️ Returns the canonical hex for a palette token key (headless-safe). */
export function tokenHex(key: StylingTokenKey | string): string {
  return STYLING_TOKENS[key as StylingTokenKey] ?? STYLING_TOKENS.gray;
}
//#endregion 🔑️TokenRefs

//#region 🎨️Resolve
const _resolveCache = ephemeralMap<string, string>("framework.modules.ui.styling.packages.typescript.index.ts._resolveCache");
const _readableForegroundCache = ephemeralMap<string, string>("framework.modules.ui.styling.packages.typescript.index.ts._readableForegroundCache");

/** @emoji 🔄️ Clears the color resolve cache (theme switches / tests). */
export function clearColorResolveCache(): void {
  _resolveCache.clear();
  _readableForegroundCache.clear();
}

function normalizeHex(hex: string): string {
  const raw = hex.trim();
  if (/^#[0-9a-f]{3}$/iu.test(raw)) {
    const r = raw[1]!;
    const g = raw[2]!;
    const b = raw[3]!;
    return `#${r}${r}${g}${g}${b}${b}`.toLowerCase();
  }
  if (/^#[0-9a-f]{6}$/iu.test(raw)) {
    return raw.toLowerCase();
  }
  if (/^#[0-9a-f]{8}$/iu.test(raw)) {
    return raw.toLowerCase();
  }
  return raw;
}

function isHexColor(value: string): boolean {
  return /^#[0-9a-f]{3,8}$/iu.test(value.trim());
}

function hexChannel(hex: string, start: number): number {
  return Number.parseInt(hex.slice(start, start + 2), 16);
}

function rgbToHex(r: number, g: number, b: number): string {
  const clamp = (n: number) => Math.min(255, Math.max(0, Math.round(n)));
  return `#${clamp(r).toString(16).padStart(2, "0")}${clamp(g).toString(16).padStart(2, "0")}${clamp(b).toString(16).padStart(2, "0")}`;
}

/** @emoji 🎨️ Linear sRGB blend between two palette token keys (headless color-mix approximation). */
export function blendTokenHex(keyA: StylingTokenKey | string, keyB: StylingTokenKey | string, ratioA: number): string {
  const a = normalizeHex(tokenHex(keyA));
  const b = normalizeHex(tokenHex(keyB));
  const t = Math.min(1, Math.max(0, ratioA));
  const r = hexChannel(a, 1) * t + hexChannel(b, 1) * (1 - t);
  const g = hexChannel(a, 3) * t + hexChannel(b, 3) * (1 - t);
  const bl = hexChannel(a, 5) * t + hexChannel(b, 5) * (1 - t);
  return rgbToHex(r, g, bl);
}

function headlessTokenFromVarRef(ref: string): string | undefined {
  const m = ref.match(/^var\(\s*(--color-[a-z0-9-]+)\s*\)$/iu);
  if (!m) {
    return undefined;
  }
  const key = m[1]!.slice("--color-".length);
  return STYLING_TOKENS[key as StylingTokenKey];
}

const SEMANTIC_HEADLESS_FALLBACK: Partial<Record<string, StylingTokenKey>> = {
  "border-element-color": "gray",
  "border-normal-color": "gray",
  "border-emphasized-color": "dark",
  "hover-interactive-fill": "gray",
  foreground: "dark",
  "color-foreground": "dark",
  "color-muted-foreground": "gray",
  "color-border": "gray",
  "muted-foreground": "gray",
};

function headlessSemanticFromVarRef(ref: string): string | undefined {
  const m = ref.match(/^var\(\s*(--[a-z0-9-]+)\s*\)$/iu);
  if (!m) {
    return undefined;
  }
  const semantic = m[1]!.slice(2);
  if (semantic === "foreground" || semantic === "color-foreground") {
    return tokenHex(currentStylingAppearanceName() === "dark" ? "light" : "dark");
  }
  const key = SEMANTIC_HEADLESS_FALLBACK[semantic];
  return key ? tokenHex(key) : undefined;
}

function cssProbeAvailable(): boolean {
  if (typeof document === "undefined" || import.meta.env?.VITEST) {
    return false;
  }
  try {
    return document.createElement("canvas").getContext("2d") != null;
  } catch {
    return false;
  }
}

function probeCssComputed(property: "color" | "backgroundColor", value: string): string {
  if (typeof document === "undefined") {
    return "";
  }
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) {
    el.classList.add("dark");
  }
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

function cssPaintToHex(css: string, fallback: string): string {
  if (!css || css === "rgba(0, 0, 0, 0)") {
    return fallback;
  }
  if (isHexColor(css)) {
    return normalizeHex(css);
  }
  if (/^rgba?\(/iu.test(css)) {
    const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)/u);
    if (m) {
      return rgbToHex(Number(m[1]), Number(m[2]), Number(m[3]));
    }
  }
  if (typeof document === "undefined") {
    return fallback;
  }
  const canvas = document.createElement("canvas");
  let ctx: CanvasRenderingContext2D | null = null;
  try {
    ctx = canvas.getContext("2d");
  } catch {
    return fallback;
  }
  if (!ctx) {
    return fallback;
  }
  ctx.fillStyle = "#000000";
  ctx.fillStyle = css;
  const converted = ctx.fillStyle;
  if (typeof converted === "string" && isHexColor(converted)) {
    return normalizeHex(converted);
  }
  if (typeof converted === "string" && /^rgba?\(/iu.test(converted)) {
    return cssPaintToHex(converted, fallback);
  }
  return fallback;
}

function resolvePaintExpressionHex(trimmed: string, fallback: string, onResolved?: (hex: string) => void): string {
  if (isHexColor(trimmed)) {
    const hex = normalizeHex(trimmed);
    onResolved?.(hex);
    return hex;
  }
  if (cssProbeAvailable()) {
    const raw = probeCssComputed("backgroundColor", trimmed);
    if (raw && raw !== "rgba(0, 0, 0, 0)") {
      const hex = cssPaintToHex(raw, "");
      if (isHexColor(hex)) {
        onResolved?.(hex);
        return hex;
      }
    }
  }
  const palette = headlessTokenFromVarRef(trimmed) ?? headlessSemanticFromVarRef(trimmed);
  if (palette) {
    onResolved?.(palette);
    return palette;
  }
  onResolved?.(fallback);
  return fallback;
}

/** @emoji 🎨️ Resolves a CSS color expression or hex literal to `#rrggbb`, using palette fallback in headless mode. */
export function resolveColorHex(ref: string, fallbackKey: StylingTokenKey | string = "gray"): string {
  const cacheKey = `${currentStylingAppearanceName()}|${ref}|${fallbackKey}`;
  const cached = _resolveCache.get(cacheKey);
  if (cached !== undefined) {
    return cached;
  }
  const fallback = tokenHex(fallbackKey);
  return resolvePaintExpressionHex(ref.trim(), fallback, (hex) => {
    _resolveCache.set(cacheKey, hex);
  });
}

/** @emoji 🎨️ Resolves a semantic CSS custom property (e.g. `--foreground`) to `#rrggbb`. */
export function resolveSemanticColorHex(cssVar: string, fallbackKey: StylingTokenKey | string = "gray"): string {
  const name = cssVar.startsWith("--") ? cssVar : `--${cssVar}`;
  return resolveBackgroundColorHex(`var(${name})`, fallbackKey);
}

/** @emoji 🎨️ Resolves a CSS background-color expression to `#rrggbb`. */
export function resolveBackgroundColorHex(ref: string, fallbackKey: StylingTokenKey | string = "gray"): string {
  const cacheKey = `bg|${currentStylingAppearanceName()}|${ref}|${fallbackKey}`;
  const cached = _resolveCache.get(cacheKey);
  if (cached !== undefined) {
    return cached;
  }
  const fallback = tokenHex(fallbackKey);
  return resolvePaintExpressionHex(ref.trim(), fallback, (hex) => {
    _resolveCache.set(cacheKey, hex);
  });
}

/** @emoji 🎨️ Resolves a CSS color expression to RGBA8888 for canvas WASM theme payloads. */
export function resolveColorRgba(ref: string, fallbackKey: StylingTokenKey | string = "gray", alpha = 255): [number, number, number, number] {
  const hex = normalizeHex(resolveColorHex(ref, fallbackKey));
  const a = hex.length === 9 ? Number.parseInt(hex.slice(7, 9), 16) : alpha;
  return [hexChannel(hex, 1), hexChannel(hex, 3), hexChannel(hex, 5), a];
}

/** @emoji 🎨️ Converts `#rrggbb` to a Three.js-friendly hex number (`0xrrggbb`). */
export function hexToThreeColor(hex: string): number {
  const norm = normalizeHex(hex);
  return Number.parseInt(norm.slice(1, 7), 16);
}

/** @emoji 🎨️ Resolves a CSS color expression to a Three.js-friendly hex number. */
export function resolveThreeColor(ref: string, fallbackKey: StylingTokenKey | string = "gray"): number {
  return hexToThreeColor(resolveColorHex(ref, fallbackKey));
}

function srgbChannelToLinear(channel: number): number {
  const s = channel / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

/** @emoji 🌓️ WCAG relative luminance for a resolved `#rrggbb` color (0 = black, 1 = white). */
export function relativeLuminance(hex: string): number {
  const norm = normalizeHex(hex);
  const r = srgbChannelToLinear(hexChannel(norm, 1));
  const g = srgbChannelToLinear(hexChannel(norm, 3));
  const b = srgbChannelToLinear(hexChannel(norm, 5));
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** @emoji 🏷️ Picks a readable palette foreground hex for text on the given background color expression. */
export function readableForegroundHex(backgroundRef: string, lightKey: StylingTokenKey | string = "light", darkKey: StylingTokenKey | string = "dark"): string {
  const cacheKey = `${backgroundRef}|${lightKey}|${darkKey}`;
  const cached = _readableForegroundCache.get(cacheKey);
  if (cached !== undefined) {
    return cached;
  }
  const bgHex = resolveBackgroundColorHex(backgroundRef, "gray");
  const result = relativeLuminance(bgHex) > 0.5 ? tokenHex(darkKey) : tokenHex(lightKey);
  _readableForegroundCache.set(cacheKey, result);
  return result;
}

/** @emoji 🌓️ Resolves the active styling appearance name from the document root class list. */
export function currentStylingAppearanceName(): StylingAppearanceName {
  if (typeof document !== "undefined" && document.documentElement.classList.contains("dark")) {
    return "dark";
  }
  return "light";
}

/** @emoji 🖼️ Serializes active theme icon overrides for hosts that rasterize catalog icons at runtime. */
export function serializeThemeIconOverridesJson(): string | undefined {
  const icons = activeUiTheme().icons;
  return icons ? JSON.stringify(icons) : undefined;
}

/** @emoji 🎨️ Serializes the active theme's board palette paints for DAG/flow canvas WASM (`CanvasPalette` JSON). Falls back to the baked semio palette before a theme is set. */
export function serializeCanvasThemeJson(appearanceName: StylingAppearanceName = currentStylingAppearanceName()): string {
  if (_activeUiTheme.current) {
    return JSON.stringify(resolveThemeAppearancePalettes(_activeUiTheme.current, appearanceName).board);
  }
  return JSON.stringify(STYLING_BOARD_PALETTES[appearanceName]);
}

/** @emoji 🎨️ WASM session surface that accepts serialized canvas theme JSON. */
export interface CanvasThemeSession {
  setCanvasThemeJson(json: string): void;
}

/** @emoji 🌓️ Pushes the active theme's canvas palette into a canvas WASM session. */
export function syncSessionCanvasTheme(session: CanvasThemeSession | null | undefined): void {
  if (!session) return;
  try {
    clearColorResolveCache();
    session.setCanvasThemeJson(serializeCanvasThemeJson());
  } catch {
    /* theme tokens not ready */
  }
}
//#endregion 🎨️Resolve

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests2 } = await import("../🧪️tests/🧪️theme-resolve/🟦️.ts");
  await registerTests2(import.meta.vitest, { SPATIAL_AXIS_COLOR_REFS, STYLING_BOARD_PALETTES, blendTokenHex, clearColorResolveCache, readableForegroundHex, relativeLuminance, resolveColorHex, resolveColorRgba, resolveSemanticColorHex, resolveSpatialAxisColors, serializeCanvasThemeJson, syncSessionCanvasTheme, tokenHex, tokenVar }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
//#endregion 🔖️resolve

//#region 🔖️theme
//#region 🔑️Premades
/** @emoji 🎨️ The default "semio" theme, built from 🔣️.json at generate time. */
export function semioTheme(): UiTheme {
  return STYLING_SEMIO_THEME as unknown as UiTheme;
}

const _builtinThemesCache = ephemeralBox<UiTheme[] | undefined>("framework.modules.ui.styling.packages.typescript.index.ts._builtinThemesCache", undefined);

/** @emoji 🎨️ Premade themes bundled with the app: semio plus any `framework/ui/styling/theme/*.theme.json` presets. `import.meta.glob` is a Vite build-time macro — it only exists once actually *called* in the bundled output, so this must call it directly inside a try/catch rather than probe for it first (`import.meta.glob` as a bare property is always `undefined` at runtime, in Vite and everywhere else; a `typeof` guard would never be true). Outside Vite (bun scripts, tests) the call throws and this falls back to semio only. */
export function builtinUiThemes(): readonly UiTheme[] {
  if (_builtinThemesCache.current) {
    return _builtinThemesCache.current;
  }
  const themes: UiTheme[] = [semioTheme()];
  try {
    const modules = import.meta.glob("../theme/*.theme.json", { eager: true, import: "default" }) as Record<string, unknown>;
    for (const raw of Object.values(modules)) {
      themes.push(parseUiTheme(raw));
    }
  } catch {
    /* import.meta.glob unavailable outside Vite */
  }
  _builtinThemesCache.current = themes;
  return themes;
}
//#endregion 🔑️Premades

//#region 🔑️ActiveTheme
const _activeUiTheme = ephemeralBox<UiTheme | undefined>("framework.modules.ui.styling.packages.typescript.index.ts._activeUiTheme", undefined);
const _activeUiThemeSubscribers = ephemeralSet<(theme: UiTheme) => void>("framework.modules.ui.styling.packages.typescript.index.ts._activeUiThemeSubscribers");
/** 🐚️ Per-root applied CSS var names — lets N co-mounted shells each carry their own theme's tokens
 * without clobbering each other's `<div>` inline overrides (only `document.documentElement` is also
 * "the page", for the single page-owning shell / `setActiveUiTheme` callers). */
const _appliedThemeCssPropsByRoot = ephemeralMap<HTMLElement, Set<string>>("framework.modules.ui.styling.packages.typescript.index.ts._appliedThemeCssPropsByRoot");

/** @emoji 🎨️ The currently active theme (defaults to semio before any theme is set). */
export function activeUiTheme(): UiTheme {
  return _activeUiTheme.current ?? semioTheme();
}

/** @emoji 🎨️ Registers a callback invoked whenever the active theme changes. Returns an unsubscribe function. */
export function subscribeActiveUiTheme(callback: (theme: UiTheme) => void): () => void {
  _activeUiThemeSubscribers.add(callback);
  return () => _activeUiThemeSubscribers.delete(callback);
}

function setCssVar(root: HTMLElement, appliedNames: Set<string>, name: string, value: string): void {
  root.style.setProperty(name, value);
  appliedNames.add(name);
}

/** @emoji 🎨️ Applies a theme's colors/spacing/fonts/strokes/glass metrics as inline CSS var overrides on
 * `root`, clearing any previous overrides *this function* applied to that same root first. Applied
 * unconditionally (even for a pristine semio theme, whose values equal the generated CSS defaults) so a
 * semio-based *draft* with edits — which still carries `id: "semio"` until saved — is never mistaken for
 * the untouched default.
 *
 * Bounded scope: only the DOM chrome tokens below are per-root. `activeUiTheme().icons` (render-time icon
 * defaults) and the `resolveColorHex`/`resolveBackgroundColorHex` canvas-paint caches stay page-level —
 * they key off `currentStylingAppearanceName()` with no root, so a co-mounted shell's own theme correctly
 * repaints its DOM chrome (CSS vars inherit from its own root) while any canvas/GPU paint that resolves a
 * hex value still follows whichever theme was applied last, page-wide. */
export function applyUiThemeToRoot(root: HTMLElement, theme: UiTheme): void {
  let appliedNames = _appliedThemeCssPropsByRoot.get(root);
  if (!appliedNames) {
    appliedNames = new Set();
    _appliedThemeCssPropsByRoot.set(root, appliedNames);
  }
  for (const name of appliedNames) {
    root.style.removeProperty(name);
  }
  appliedNames.clear();
  root.dataset.uiTheme = theme.id;
  for (const [key, hex] of Object.entries(theme.colors)) {
    setCssVar(root, appliedNames, `--color-${key.replaceAll("_", "-")}`, hex);
  }
  for (const [key, value] of Object.entries(theme.spacing)) {
    setCssVar(root, appliedNames, `--spacing-${key.replaceAll("_", "-")}`, value);
  }
  if (theme.fontStacks.sans) setCssVar(root, appliedNames, "--font-sans", theme.fontStacks.sans);
  if (theme.fontStacks.serif) setCssVar(root, appliedNames, "--font-serif", theme.fontStacks.serif);
  if (theme.fontStacks.mono) setCssVar(root, appliedNames, "--font-mono", theme.fontStacks.mono);
  const hairline = theme.strokes.chromeBorderHairline;
  if (typeof hairline === "number") setCssVar(root, appliedNames, "--stroke-hairline", `${hairline}px`);
  const chrome = theme.metrics.chrome;
  if (chrome) {
    if (typeof chrome.glassSaturate === "number") setCssVar(root, appliedNames, "--glass-saturate", `${chrome.glassSaturate}`);
    if (typeof chrome.shadeStepPercent === "number") setCssVar(root, appliedNames, "--level-shade-step", `${chrome.shadeStepPercent}%`);
    if (typeof chrome.elementStepPercent === "number") setCssVar(root, appliedNames, "--element-shade-step", `${chrome.elementStepPercent}%`);
    if (typeof chrome.hoverStepPercent === "number") setCssVar(root, appliedNames, "--hover-shade-step", `${chrome.hoverStepPercent}%`);
    if (typeof chrome.glassAlphaStep === "number") setCssVar(root, appliedNames, "--glass-alpha-step", `${chrome.glassAlphaStep}`);
    if (typeof chrome.glassBlurStepPx === "number") setCssVar(root, appliedNames, "--glass-blur-step", `${chrome.glassBlurStepPx / 16}rem`);
  }
  clearColorResolveCache();
}

/** @emoji 🎨️ Removes every CSS var {@link applyUiThemeToRoot} applied to `root` and forgets its registry
 * entry — call on a shell's unmount so a later, unrelated element reused at the same DOM position never
 * inherits a stale theme's inline overrides. */
export function clearUiThemeFromRoot(root: HTMLElement): void {
  const appliedNames = _appliedThemeCssPropsByRoot.get(root);
  if (!appliedNames) return;
  for (const name of appliedNames) {
    root.style.removeProperty(name);
  }
  delete root.dataset.uiTheme;
  _appliedThemeCssPropsByRoot.delete(root);
}

/** @emoji 🎨️ Applies a theme's colors/spacing/fonts/strokes/glass metrics as inline `documentElement` CSS
 * var overrides — the page-owning case of {@link applyUiThemeToRoot}. */
export function applyUiThemeToDocument(theme: UiTheme): void {
  if (typeof document === "undefined") {
    return;
  }
  applyUiThemeToRoot(document.documentElement, theme);
}

/** @emoji 🎨️ Sets the *page-global* active theme, applies it to `document.documentElement`, and notifies
 * subscribers — for the single page-owning shell (`ShellScope.ownsPage`) or a standalone (non-shell) host.
 * A co-mounted, non-page-owning shell must call {@link applyUiThemeToRoot} on its own root instead, or it
 * would fight every other mounted shell over the same document-wide tokens. */
export function setActiveUiTheme(theme: UiTheme): void {
  _activeUiTheme.current = theme;
  applyUiThemeToDocument(theme);
  for (const subscriber of _activeUiThemeSubscribers) {
    subscriber(theme);
  }
}
//#endregion 🔑️ActiveTheme

//#region 🧪️ThemeTests
if (import.meta.vitest) {
  const { registerTests3 } = await import("../🧪️tests/🧪️theme-resolve/🟦️.ts");
  await registerTests3(import.meta.vitest, { STYLING_BOARD_PALETTES, _activeUiTheme, _appliedThemeCssPropsByRoot, activeUiTheme, applyUiThemeToDocument, applyUiThemeToRoot, builtinUiThemes, clearUiThemeFromRoot, semioTheme, serializeCanvasThemeJson, setActiveUiTheme, subscribeActiveUiTheme }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️ThemeTests
//#endregion 🔖️theme

//#region 🔖️icon-render-port
//#region 🔖️IconRenderPort
export type IconRenderFormat = "svg" | "png";

export type IconRenderShape = "rectangle" | "ellipse";

export interface IconRenderCamera {
  readonly position: readonly [number, number, number];
  readonly target: readonly [number, number, number];
  readonly zoom: number;
  readonly fov?: number;
  readonly up?: readonly [number, number, number];
}

export interface IconRenderLights {
  readonly ambientIntensity: number;
  readonly ambientColor: string;
  readonly sunAzimuth: number;
  readonly sunElevation: number;
  readonly sunIntensity: number;
  readonly sunColor: string;
}

export interface IconRenderMaterial {
  readonly color?: string;
  readonly metalness?: number;
  readonly roughness?: number;
  readonly emissive?: string;
  readonly emissiveIntensity?: number;
}

export interface IconRenderRequest {
  readonly assetUrl: string;
  readonly camera: IconRenderCamera;
  readonly lights: IconRenderLights;
  readonly width: number;
  readonly height: number;
  readonly format: IconRenderFormat;
  readonly shape?: IconRenderShape;
  readonly background?: string;
  readonly shadowEnabled?: boolean;
  readonly material?: IconRenderMaterial;
}

export interface IconRenderResult {
  readonly dataUrl: string;
  readonly svgMarkup?: string;
}

export interface IconRenderPort {
  render(request: IconRenderRequest): Promise<IconRenderResult>;
}
//#endregion 🔖️IconRenderPort
//#endregion 🔖️icon-render-port

//#region 🧭️ElementState
/** @emoji 🧭️ The one shared, compile-time-enforced state model every rendered UI element carries:
 * `state` × `status` × `hover` × `selected`. Mirrors the Rust `UiState`/`UiStatus`/`UiPresence`
 * model in `ui_wgpu` (see `framework/ui/wgpu/rs/lib.rs`'s 🔖️Presence region) — string-literal unions here,
 * since this package has no dependency on the Rust crate or its generated bindings. `Hidden` makes
 * every other axis irrelevant: a hidden element is not rendered at all. */
export const UI_STATES = ["introducing", "celebrating", "previewed", "normal", "disabled", "hidden"] as const;
export type UiState = (typeof UI_STATES)[number];

/** @emoji 🧭️ The activity lifecycle of a UI element, orthogonal to {@link UiState} and composable with it. */
export const UI_STATUSES = ["waiting", "loading", "idle", "finished"] as const;
export type UiStatus = (typeof UI_STATUSES)[number];

export interface UiElementState {
  readonly state: UiState;
  readonly status: UiStatus;
  readonly hover: boolean;
  readonly selected: boolean;
}

const DEFAULT_UI_ELEMENT_STATE: UiElementState = { state: "normal", status: "idle", hover: false, selected: false };

/** @emoji 🧭️ Fills in every axis with its default (`normal`/`idle`/`false`/`false`) — the one place that convention lives. */
export function resolveElementState(partial?: Partial<UiElementState>): UiElementState {
  return { ...DEFAULT_UI_ELEMENT_STATE, ...partial };
}

/** @emoji 🙈️ `true` only for `state === "hidden"` — callers must not render/lay out/hit-test the element. */
export function elementStateHidden(s: Pick<UiElementState, "state">): boolean {
  return s.state === "hidden";
}

/** @emoji 🧭️ The one function that emits the shared `data-ui-*` attribute vocabulary every element's
 * markup carries. Two axes carry a SECOND attribute alongside `data-ui-state`/`data-ui-status`:
 * `introducing` also stamps `data-introduced="true"` — the exact attribute `UIIntroduction`'s
 * tour-driven reveal already stamps imperatively (see `framework/ui/js/react/index.tsx`), so an authored
 * `state: "introducing"` and a live tour step converge on the identical CSS rule
 * (`[data-introduced="true"]` in `🖌️ui.css`) with no duplicate styling to maintain. `celebrating`
 * mirrors this exactly with `data-celebrated="true"`, converging with `celebrateElements()`'s
 * transient imperative stamp on the same `[data-celebrated="true"]` rule. All axes are omitted at
 * their default value — the DOM stays clean when nothing is going on. A hidden element gets `{}`:
 * callers must not render it at all, so there is nothing to attribute. */
export type UiElementStateAttributes = {
  readonly "data-ui-state"?: "introducing" | "celebrating" | "previewed" | "disabled";
  readonly "data-introduced"?: "true";
  readonly "data-celebrated"?: "true";
  readonly "data-ui-status"?: "waiting" | "loading" | "finished";
  readonly "data-ui-hover"?: "true";
  readonly "data-ui-selected"?: "true";
};

export function elementStateAttributes(s: UiElementState): UiElementStateAttributes {
  if (elementStateHidden(s)) return {};
  const attrs: { -readonly [K in keyof UiElementStateAttributes]?: UiElementStateAttributes[K] } = {};
  if (s.state === "introducing") {
    attrs["data-ui-state"] = "introducing";
    attrs["data-introduced"] = "true";
  } else if (s.state === "celebrating") {
    attrs["data-ui-state"] = "celebrating";
    attrs["data-celebrated"] = "true";
  } else if (s.state === "previewed" || s.state === "disabled") {
    attrs["data-ui-state"] = s.state;
  }
  if (s.status !== "idle") attrs["data-ui-status"] = s.status;
  if (s.hover) attrs["data-ui-hover"] = "true";
  if (s.selected) attrs["data-ui-selected"] = "true";
  return attrs;
}

/** @emoji 🎨️ The shared precedence resolver for renderers that can't use CSS/data-attributes at all
 * (3D fills, canvas emissive channels — `Orb`/`Geometry`/world-mesh materials): collapses the four
 * axes to a single fill "kind" a caller maps to its own color table. Precedence, most to least
 * specific: `disabled` > `celebrated` > `selected` > `previewed` > `hovered` > `neutral`. `hidden`
 * resolves to `null` — nothing to fill, the caller must not render the element at all. */
export const ELEMENT_FILL_KINDS = ["disabled", "celebrated", "selected", "previewed", "hovered", "neutral"] as const;
export type ElementFillKind = (typeof ELEMENT_FILL_KINDS)[number];

export function resolveElementFillKind(s: UiElementState): ElementFillKind | null {
  if (elementStateHidden(s)) return null;
  if (s.state === "disabled") return "disabled";
  if (s.state === "celebrating") return "celebrated";
  if (s.selected) return "selected";
  if (s.state === "previewed") return "previewed";
  if (s.hover) return "hovered";
  return "neutral";
}
//#endregion 🧭️ElementState

//#region 🔁️AnimationScope
/** @emoji ⏱️ One rule that starts CSS animations, addressed by its resolved selector path. */
export interface CssClockRule {
  selector: string;
  keyframes: readonly string[];
}

/** @emoji 🖌️ One rule that PAINTS with animated custom properties — reads them from a declaration whose
 * own property is not itself a custom property, directly or through a custom-property chain. A rule that
 * only forwards the value into another custom property is not a paint and owns no clock. */
export interface CssPaintRule {
  selector: string;
  properties: readonly string[];
}

/** @emoji 🔬️ What {@link analyzeCssAnimationScope} reads out of one stylesheet. */
export interface CssAnimationScope {
  animatedCustomProperties: readonly string[];
  keyframesByProperty: Readonly<Record<string, readonly string[]>>;
  propertyInheritance: Readonly<Record<string, boolean>>;
  clocks: readonly CssClockRule[];
  paints: readonly CssPaintRule[];
  rootClocks: readonly CssClockRule[];
}

/** @emoji 🌳️ Selectors that address the document root, whose animated inherited custom properties
 * re-resolve every element's computed style on every frame. */
const CSS_DOCUMENT_ROOT_SELECTORS = new Set([":root", "html", ":root:root", "html:root"]);

interface CssBlock {
  prelude: string;
  declarations: readonly (readonly [string, string])[];
  children: readonly CssBlock[];
}

function stripCssComments(css: string): string {
  return css.replace(/\/\*[\s\S]*?\*\//g, "");
}

function parseCssBlocks(css: string): readonly CssBlock[] {
  const root: CssBlock = { prelude: "", declarations: [], children: [] };
  const stack: { block: CssBlock; declarations: [string, string][]; children: CssBlock[] }[] = [{ block: root, declarations: [], children: [] }];
  let buffer = "";
  let depth = 0;
  for (let index = 0; index < css.length; index += 1) {
    const character = css[index]!;
    if (character === "'" || character === '"') {
      const end = css.indexOf(character, index + 1);
      const slice = end === -1 ? css.slice(index) : css.slice(index, end + 1);
      buffer += slice;
      index += slice.length - 1;
      continue;
    }
    if (character === "(") depth += 1;
    else if (character === ")") depth = Math.max(0, depth - 1);
    if (depth > 0) { buffer += character; continue; }
    if (character === "{") {
      const frame = { block: { prelude: buffer.trim(), declarations: [], children: [] } as CssBlock, declarations: [] as [string, string][], children: [] as CssBlock[] };
      stack.push(frame);
      buffer = "";
      continue;
    }
    if (character === "}") {
      const declaration = buffer.trim();
      const frame = stack.pop();
      if (!frame) { buffer = ""; continue; }
      if (declaration.length > 0) {
        const colon = declaration.indexOf(":");
        if (colon > 0) frame.declarations.push([declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()]);
      }
      const closed: CssBlock = { prelude: frame.block.prelude, declarations: frame.declarations, children: frame.children };
      stack.at(-1)?.children.push(closed);
      buffer = "";
      continue;
    }
    if (character === ";") {
      const declaration = buffer.trim();
      const colon = declaration.indexOf(":");
      if (colon > 0) stack.at(-1)?.declarations.push([declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()]);
      buffer = "";
      continue;
    }
    buffer += character;
  }
  return stack[0]?.children ?? [];
}

function cssVarReads(value: string): readonly string[] {
  return [...value.matchAll(/var\(\s*(--[\w-]+)/g)].map((match) => match[1]!);
}

function cssAnimationNames(value: string): readonly string[] {
  const nonNames = new Set(["none", "infinite", "linear", "ease", "ease-in", "ease-out", "ease-in-out", "alternate", "alternate-reverse", "reverse", "normal", "forwards", "backwards", "both", "running", "paused", "step-start", "step-end", "important"]);
  return value.split(",").flatMap((layer) => layer.replace(/!important/g, " ").split(/\s+/).filter((token) => /^[A-Za-z_-][\w-]*$/.test(token) && !nonNames.has(token) && !token.startsWith("--") && !token.startsWith("cubic-bezier") && !token.startsWith("steps")));
}

/** @emoji 🔬️ Reads a stylesheet's animation scope: which custom properties any `@keyframes` animates,
 * how each is registered, which rules start those clocks, and which rules paint with them.
 *
 * Exists because the cost of a CSS animation is the size of the style invalidation it causes, not the
 * number of animations: an animated registered custom property declared `inherits: true` and started on
 * the document root re-resolves the computed style of EVERY element on EVERY frame. Measured on the
 * generation3d procedural example (ticket 2026/09/09, `📓️host-reconcile-silence-2026-09-12.md`): nine
 * such clocks on `:root` cost 112.3 s of style recalculation out of 129.0 s of main-thread task time and
 * starved the plugin host's continuation pump to one macrotask per frame.
 *
 * Nested rules (`@utility x { &::after { … } }`, `@media … { … }`) resolve to a space-joined selector
 * path, so a clock and the paint it drives are comparable by identity.
 * See https://drafts.css-houdini.org/css-properties-values-api/#inherits-descriptor. */
export function analyzeCssAnimationScope(css: string): CssAnimationScope {
  const blocks = parseCssBlocks(stripCssComments(css));
  const keyframesByProperty = new Map<string, Set<string>>();
  const propertyInheritance: Record<string, boolean> = {};
  const clocks: CssClockRule[] = [];
  const directReads = new Map<string, Set<string>>();
  const customPropertyReads = new Map<string, Set<string>>();
  const indirectReads = new Map<string, Set<string>>();
  const walk = (block: CssBlock, path: readonly string[]): void => {
    const prelude = block.prelude;
    if (/^@keyframes\s/.test(prelude)) {
      const name = prelude.replace(/^@keyframes\s+/, "").trim();
      const collect = (node: CssBlock): void => {
        for (const [property] of node.declarations) if (property.startsWith("--")) (keyframesByProperty.get(property) ?? keyframesByProperty.set(property, new Set()).get(property)!).add(name);
        for (const child of node.children) collect(child);
      };
      collect(block);
      return;
    }
    if (/^@property\s/.test(prelude)) {
      const name = prelude.replace(/^@property\s+/, "").trim();
      const inherits = block.declarations.find(([property]) => property === "inherits")?.[1];
      propertyInheritance[name] = inherits?.trim() === "true";
      return;
    }
    const scoping = /^@(media|supports|layer|container|scope)\b/.test(prelude) || prelude.length === 0;
    const selectorPath = scoping ? path : [...path, prelude];
    const selector = selectorPath.join(" ");
    for (const [property, value] of block.declarations) {
      if (property === "animation" || property === "animation-name") {
        const keyframes = cssAnimationNames(value);
        if (keyframes.length > 0) clocks.push({ selector, keyframes });
      }
      const reads = cssVarReads(value);
      if (reads.length === 0) continue;
      const sink = property.startsWith("--") ? customPropertyReads.get(property) ?? customPropertyReads.set(property, new Set()).get(property)! : directReads.get(selector) ?? directReads.set(selector, new Set()).get(selector)!;
      for (const read of reads) sink.add(read);
      if (!property.startsWith("--")) for (const read of reads) (indirectReads.get(selector) ?? indirectReads.set(selector, new Set()).get(selector)!).add(read);
    }
    for (const child of block.children) walk(child, selectorPath);
  };
  for (const block of blocks) walk(block, []);
  const animated = [...keyframesByProperty.keys()].sort();
  const animatedSet = new Set(animated);
  const resolve = (name: string, seen: Set<string>): readonly string[] => {
    if (seen.has(name)) return [];
    seen.add(name);
    if (animatedSet.has(name)) return [name];
    return [...(customPropertyReads.get(name) ?? [])].flatMap((next) => resolve(next, seen));
  };
  const paints: CssPaintRule[] = [];
  for (const [selector, reads] of directReads) {
    const resolved = [...new Set([...reads].flatMap((read) => resolve(read, new Set())))].sort();
    if (resolved.length > 0) paints.push({ selector, properties: resolved });
  }
  return {
    animatedCustomProperties: animated,
    keyframesByProperty: Object.fromEntries(animated.map((name) => [name, [...keyframesByProperty.get(name)!].sort()])),
    propertyInheritance,
    clocks,
    paints: paints.sort((left, right) => left.selector.localeCompare(right.selector)),
    rootClocks: clocks.filter((clock) => clock.selector.split(" ").every((part) => CSS_DOCUMENT_ROOT_SELECTORS.has(part.trim()))),
  };
}
/** @emoji ⚖️ The three laws of {@link analyzeCssAnimationScope}, in the order the fixture states them.
 * `no-document-root-clock` — no rule whose whole selector path addresses the document root may start an
 * animation; `animated-custom-properties-are-non-inherited` — a property any `@keyframes` animates must be
 * registered `inherits: false`, so its frames invalidate one element instead of the whole document;
 * `every-paint-owns-its-clock` — a rule that paints with an animated property must start the animation
 * itself, because a non-inherited phase never reaches it from an ancestor. */
export const CSS_ANIMATION_SCOPE_LAWS = ["no-document-root-clock", "animated-custom-properties-are-non-inherited", "every-paint-owns-its-clock"] as const;
export type CssAnimationScopeLaw = (typeof CSS_ANIMATION_SCOPE_LAWS)[number];

/** @emoji ⚖️ Names which of {@link CSS_ANIMATION_SCOPE_LAWS} a stylesheet's scope breaks, in law order. */
export function cssAnimationScopeViolations(scope: CssAnimationScope): readonly CssAnimationScopeLaw[] {
  const animated = new Set(scope.animatedCustomProperties);
  const started = new Map<string, Set<string>>();
  for (const clock of scope.clocks) for (const name of clock.keyframes) (started.get(clock.selector) ?? started.set(clock.selector, new Set()).get(clock.selector)!).add(name);
  const broken: CssAnimationScopeLaw[] = [];
  if (scope.rootClocks.length > 0) broken.push("no-document-root-clock");
  if ([...animated].some((name) => scope.propertyInheritance[name] !== false)) broken.push("animated-custom-properties-are-non-inherited");
  const unclocked = scope.paints.some((paint) => paint.properties.some((name) => !(scope.keyframesByProperty[name] ?? []).some((keyframes) => started.get(paint.selector)?.has(keyframes) === true)));
  if (unclocked) broken.push("every-paint-owns-its-clock");
  return broken;
}

/** @emoji 🖌️ Every paint that does not start the clock its animated property needs, named with the
 * keyframes it is missing — the actionable half of `every-paint-owns-its-clock`. */
export function cssAnimationScopeUnclockedPaints(scope: CssAnimationScope): readonly { selector: string; property: string; keyframes: readonly string[] }[] {
  const started = new Map<string, Set<string>>();
  for (const clock of scope.clocks) for (const name of clock.keyframes) (started.get(clock.selector) ?? started.set(clock.selector, new Set()).get(clock.selector)!).add(name);
  return scope.paints.flatMap((paint) => paint.properties.filter((name) => !(scope.keyframesByProperty[name] ?? []).some((keyframes) => started.get(paint.selector)?.has(keyframes) === true)).map((property) => ({ selector: paint.selector, property, keyframes: scope.keyframesByProperty[property] ?? [] })));
}
//#endregion 🔁️AnimationScope
