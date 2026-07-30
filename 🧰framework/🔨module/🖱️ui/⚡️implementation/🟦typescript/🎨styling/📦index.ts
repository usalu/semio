// #region 🧲Header
/** @emoji 🎨 `@semio-tech/ui-styling` centralizes palette CSS, Tailwind entry, and the shared typography preset for ui consumers. */
// #endregion 🧲Header

export { tailwindConfig, tailwindConfig as default } from "../../../../../../🧰framework/🔨module/🖱️ui/⚡️implementation/🦀rust/🎨styling/🎨tailwind/🎨tailwind.config.ts";
export {
  STYLING_BOARD_PALETTES,
  STYLING_CANVAS_FONTS,
  STYLING_CANVAS_PALETTES,
  STYLING_MAP_PALETTES,
  STYLING_METRICS,
  STYLING_OPACITIES,
  STYLING_RADII,
  STYLING_SEMIO_THEME,
  STYLING_STROKES,
  STYLING_TOKENS,
  type StylingAppearanceName,
  type StylingTokenKey,
} from "../../../../../../🧰framework/🔨module/🖱️ui/⚡️implementation/🟦typescript/🎨styling/🟦tokens.generated.ts";
import { STYLING_BOARD_PALETTES, STYLING_METRICS, STYLING_SEMIO_THEME, STYLING_TOKENS, type StylingAppearanceName, type StylingTokenKey } from "../../../../../../🧰framework/🔨module/🖱️ui/⚡️implementation/🟦typescript/🎨styling/🟦tokens.generated.ts";

//#region 🔖ThemeModel
/** @emoji 🎨 `UiTheme` model: paint-ref resolver, parse/serialize, shared by the token generator and the runtime theme engine. MUST stay dependency-free from `🟦tokens.generated.ts` (the generator produces that file; importing it here would create a cycle). */
//#region 🔖types
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

/** @emoji 🌓 Light/dark palette dimension within a theme. */
export type ThemeAppearanceName = "light" | "dark";

/** @emoji 🖼 Optional runtime icon appearance overrides keyed by compile-time icon ids. */
export interface UiThemeIcons {
  readonly aliases?: Readonly<Partial<Record<string, string>>>;
  readonly variants?: Readonly<Partial<Record<string, string>>>;
  readonly themedAliases?: Readonly<Partial<Record<string, string>>>;
  readonly themedVariants?: Readonly<Partial<Record<string, string>>>;
}

const THEME_PALETTE_GROUPS: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];

const THEME_APPEARANCE_NAMES: readonly ThemeAppearanceName[] = ["light", "dark"];

/** @emoji 🎨 A named, fully editable design-token set (colors, spacing, fonts, strokes, radii, opacities, metrics, and light/dark appearance paints). */
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
//#endregion 🔖types

//#region 🔖resolveTheme
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

/** @emoji 📏 Derives dag component width as twice the IO channel column width (mirrors the 🔣tokens.json authoring shortcut). */
export function resolveThemeMetrics(metrics: UiTheme["metrics"]): UiTheme["metrics"] {
  const out = structuredClone(metrics ?? {});
  const dag = out.dag;
  if (dag && typeof dag.ioColumnWidth === "number") {
    dag.componentWidth = dag.ioColumnWidth * 2;
  }
  return out;
}

/** @emoji 🎨 Resolves every paint in one appearance of a theme to sRGB8888, grouped by palette. */
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
//#endregion 🔖resolveTheme

//#region 🔖parseTheme
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
  const out: UiThemeIcons = {};
  if ("aliases" in obj) out.aliases = parseStringMap(obj.aliases, `${path}.aliases`);
  if ("variants" in obj) out.variants = parseStringMap(obj.variants, `${path}.variants`);
  if ("themedAliases" in obj) out.themedAliases = parseStringMap(obj.themedAliases, `${path}.themedAliases`);
  if ("themedVariants" in obj) out.themedVariants = parseStringMap(obj.themedVariants, `${path}.themedVariants`);
  return out;
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

/** @emoji 🔎 Strictly parses and validates a `UiTheme` (unknown token refs / missing palette groups throw). Every paint is resolved once to surface broken refs immediately. */
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

/** @emoji 💾 Serializes a `UiTheme` to canonical JSON. */
export function serializeUiTheme(theme: UiTheme): string {
  return JSON.stringify(theme, null, 2);
}
//#endregion 🔖parseTheme

//#region 🧪ThemeModelTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  const MINIMAL_THEME: UiTheme = {
    id: "test",
    label: "Test",
    colors: { primary: "#ff0000", gray: "#808080" },
    spacing: { compact: "0.25rem" },
    fontStacks: { sans: "sans-serif" },
    canvasFonts: {},
    strokes: { edgeBase: 2 },
    radii: { chrome: 0 },
    opacities: { glassBlur: 1 },
    metrics: { dag: { ioColumnWidth: 10 } },
    appearances: {
      light: { board: { edgeStroke: { token: "gray" } }, map: {}, canvas: {}, chrome: {} },
      dark: { board: { edgeStroke: { token: "primary" } }, map: {}, canvas: {}, chrome: {} },
    },
  };

  describe("theme resolve", () => {
    it("resolveThemePaint resolves a token ref", () => {
      expect(resolveThemePaint(MINIMAL_THEME.colors, { token: "primary" })).toEqual([255, 0, 0, 255]);
    });

    it("resolveThemePaint resolves a mix ref", () => {
      const [r, g, b, a] = resolveThemePaint(MINIMAL_THEME.colors, { mix: ["primary", "gray", 0.5] });
      expect(a).toBe(255);
      expect(r).toBeGreaterThan(g);
    });

    it("resolveThemePaint resolves a literal hex ref with alpha", () => {
      expect(resolveThemePaint(MINIMAL_THEME.colors, { hex: "#00ff00", alpha: 0.5 })).toEqual([0, 255, 0, 128]);
    });

    it("resolveThemeMetrics derives dag.componentWidth", () => {
      const resolved = resolveThemeMetrics(MINIMAL_THEME.metrics);
      expect(resolved.dag!.componentWidth).toBe(20);
    });

    it("resolveThemeAppearancePalettes resolves board paints per appearance", () => {
      const light = resolveThemeAppearancePalettes(MINIMAL_THEME, "light");
      const dark = resolveThemeAppearancePalettes(MINIMAL_THEME, "dark");
      expect(light.board.edgeStroke).toEqual([128, 128, 128, 255]);
      expect(dark.board.edgeStroke).toEqual([255, 0, 0, 255]);
    });
  });

  describe("theme parse", () => {
    it("round-trips a valid theme through serialize/parse", () => {
      const parsed = parseUiTheme(JSON.parse(serializeUiTheme(MINIMAL_THEME)));
      expect(parsed).toEqual(MINIMAL_THEME);
    });

    it("throws on an unknown color token ref", () => {
      const broken = { ...MINIMAL_THEME, appearances: { ...MINIMAL_THEME.appearances, light: { ...MINIMAL_THEME.appearances.light, board: { edgeStroke: { token: "nope" } } } } };
      expect(() => parseUiTheme(broken)).toThrow();
    });

    it("throws when a palette group is missing", () => {
      const broken = JSON.parse(serializeUiTheme(MINIMAL_THEME));
      delete broken.appearances.light.chrome;
      expect(() => parseUiTheme(broken)).toThrow(/chrome/);
    });

    it("throws when appearances.dark is missing", () => {
      const broken = JSON.parse(serializeUiTheme(MINIMAL_THEME));
      delete broken.appearances.dark;
      expect(() => parseUiTheme(broken)).toThrow(/dark/);
    });
  });
}
//#endregion 🧪ThemeModelTests
//#endregion 🔖ThemeModel

//#region 🔖sizing
//#region 🔑SizeVars
/** @emoji 🔑 Canonical DOM size CSS variable names. */
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

/** @emoji 🔑 Returns a `var(--…)` reference for a DOM size token. */
export function sizeVar(key: keyof typeof STYLING_SIZE_VAR): string {
  return `var(${STYLING_SIZE_VAR[key]})`;
}

/** @emoji 🔑 Compact-mode reference root (px) for headless layout math at default 16px root. */
export const STYLING_COMPACT_ROOT_PX = 16;

const COMPACT_UI_SPACING_REM = 0.2;

/** @emoji 📐 Converts a ui-spacing multiplier to rem length. */
export function uiSpacingRem(multiplier: number): string {
  return `${multiplier * COMPACT_UI_SPACING_REM}rem`;
}

/** @emoji 📐 Converts a ui-spacing multiplier to px at the compact reference root. */
export function uiSpacingPx(multiplier: number, rootPx = STYLING_COMPACT_ROOT_PX): number {
  return multiplier * COMPACT_UI_SPACING_REM * rootPx;
}

/** @emoji 📐 DOM layout multipliers (multiples of `--ui-spacing`) from 🔣tokens.json. */
export const STYLING_DOM = STYLING_METRICS.dom;

/** @emoji 📐 Resolves a DOM metric key to px at the compact reference root. */
export function domSizePx(key: keyof typeof STYLING_DOM, rootPx = STYLING_COMPACT_ROOT_PX): number {
  return uiSpacingPx(STYLING_DOM[key], rootPx);
}

/** @emoji 📐 Reads a resolved CSS size variable from the document (browser only). */
export function readSizeVarPx(varName: string, element?: Element | null): number {
  if (typeof document === "undefined") {
    return 0;
  }
  const host = element ?? document.documentElement;
  const raw = getComputedStyle(host).getPropertyValue(varName.startsWith("--") ? varName : `--${varName}`);
  return Number.parseFloat(raw) || 0;
}
//#endregion 🔑SizeVars
//#endregion 🔖sizing

//#region 🔖resolve
//#region 🔑TokenRefs
/** @emoji 🔑 Builds a primitive palette CSS variable reference (`var(--color-<key>)`). */
export function tokenVar(key: StylingTokenKey | string): string {
  return `var(--color-${key.replaceAll("_", "-")})`;
}

/** @emoji 🔑 Builds a semantic UI CSS variable reference (`var(--<name>)`). */
export function semanticVar(name: string): string {
  const trimmed = name.startsWith("--") ? name.slice(2) : name;
  return `var(--${trimmed})`;
}

/** @emoji 🔑 Builds a Tailwind `@theme inline` color alias (`var(--color-<name>)`). */
export function themeColorVar(name: string): string {
  return `var(--color-${name.replaceAll("_", "-")})`;
}

/** @emoji 🧭 Permanent X/Y/Z paints for gumball and view/projection gizmos — primary / secondary / tertiary, never active/hover chrome. */
export const SPATIAL_AXIS_COLOR_REFS = {
  x: tokenVar("primary"),
  y: tokenVar("secondary"),
  z: tokenVar("tertiary"),
} as const;

/** @emoji 🧭 Resolved `#rrggbb` axis paints for spatial manipulators and navigation cubes. */
export function resolveSpatialAxisColors(): { readonly x: string; readonly y: string; readonly z: string } {
  return {
    x: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.x, "primary"),
    y: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.y, "secondary"),
    z: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.z, "tertiary"),
  };
}

/** @emoji 🔑 Returns the canonical hex for a palette token key (headless-safe). */
export function tokenHex(key: StylingTokenKey | string): string {
  return STYLING_TOKENS[key as StylingTokenKey] ?? STYLING_TOKENS.gray;
}
//#endregion 🔑TokenRefs

//#region 🎨Resolve
const _resolveCache = new Map<string, string>();
const _readableForegroundCache = new Map<string, string>();

/** @emoji 🔄 Clears the color resolve cache (theme switches / tests). */
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

/** @emoji 🎨 Linear sRGB blend between two palette token keys (headless color-mix approximation). */
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

/** @emoji 🎨 Resolves a CSS color expression or hex literal to `#rrggbb`, using palette fallback in headless mode. */
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

/** @emoji 🎨 Resolves a semantic CSS custom property (e.g. `--foreground`) to `#rrggbb`. */
export function resolveSemanticColorHex(cssVar: string, fallbackKey: StylingTokenKey | string = "gray"): string {
  const name = cssVar.startsWith("--") ? cssVar : `--${cssVar}`;
  return resolveBackgroundColorHex(`var(${name})`, fallbackKey);
}

/** @emoji 🎨 Resolves a CSS background-color expression to `#rrggbb`. */
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

/** @emoji 🎨 Resolves a CSS color expression to RGBA8888 for canvas WASM theme payloads. */
export function resolveColorRgba(ref: string, fallbackKey: StylingTokenKey | string = "gray", alpha = 255): [number, number, number, number] {
  const hex = normalizeHex(resolveColorHex(ref, fallbackKey));
  const a = hex.length === 9 ? Number.parseInt(hex.slice(7, 9), 16) : alpha;
  return [hexChannel(hex, 1), hexChannel(hex, 3), hexChannel(hex, 5), a];
}

/** @emoji 🎨 Converts `#rrggbb` to a Three.js-friendly hex number (`0xrrggbb`). */
export function hexToThreeColor(hex: string): number {
  const norm = normalizeHex(hex);
  return Number.parseInt(norm.slice(1, 7), 16);
}

/** @emoji 🎨 Resolves a CSS color expression to a Three.js-friendly hex number. */
export function resolveThreeColor(ref: string, fallbackKey: StylingTokenKey | string = "gray"): number {
  return hexToThreeColor(resolveColorHex(ref, fallbackKey));
}

function srgbChannelToLinear(channel: number): number {
  const s = channel / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

/** @emoji 🌓 WCAG relative luminance for a resolved `#rrggbb` color (0 = black, 1 = white). */
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

/** @emoji 🌓 Resolves the active styling appearance name from the document root class list. */
export function currentStylingAppearanceName(): StylingAppearanceName {
  if (typeof document !== "undefined" && document.documentElement.classList.contains("dark")) {
    return "dark";
  }
  return "light";
}

/** @emoji 🖼 Serializes active theme icon overrides for hosts that rasterize catalog icons at runtime. */
export function serializeThemeIconOverridesJson(): string | undefined {
  const icons = activeUiTheme().icons;
  return icons ? JSON.stringify(icons) : undefined;
}

/** @emoji 🎨 Serializes the active theme's board palette paints for DAG/flow canvas WASM (`CanvasPalette` JSON). Falls back to the baked semio palette before a theme is set. */
export function serializeCanvasThemeJson(appearanceName: StylingAppearanceName = currentStylingAppearanceName()): string {
  if (_activeUiTheme) {
    return JSON.stringify(resolveThemeAppearancePalettes(_activeUiTheme, appearanceName).board);
  }
  return JSON.stringify(STYLING_BOARD_PALETTES[appearanceName]);
}

/** @emoji 🎨 WASM session surface that accepts serialized canvas theme JSON. */
export interface CanvasThemeSession {
  setCanvasThemeJson(json: string): void;
}

/** @emoji 🌓 Pushes the active theme's canvas palette into a canvas WASM session. */
export function syncSessionCanvasTheme(session: CanvasThemeSession | null | undefined): void {
  if (!session) return;
  try {
    clearColorResolveCache();
    session.setCanvasThemeJson(serializeCanvasThemeJson());
  } catch {
    /* theme tokens not ready */
  }
}
//#endregion 🎨Resolve

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("styling resolve", () => {
    it("tokenVar and tokenHex read generated palette", () => {
      expect(tokenVar("primary")).toBe("var(--color-primary)");
      expect(tokenHex("primary")).toBe("#ff344f");
    });

    it("resolveColorHex resolves palette var refs headlessly", () => {
      clearColorResolveCache();
      expect(resolveColorHex("var(--color-secondary)", "gray")).toBe("#34d1bf");
    });

    it("resolveSpatialAxisColors maps X/Y/Z to primary/secondary/tertiary permanently", () => {
      clearColorResolveCache();
      expect(resolveSpatialAxisColors()).toEqual({ x: "#ff344f", y: "#34d1bf", z: "#fa9500" });
      expect(SPATIAL_AXIS_COLOR_REFS).toEqual({ x: "var(--color-primary)", y: "var(--color-secondary)", z: "var(--color-tertiary)" });
    });

    it("resolveColorHex passes through hex literals", () => {
      clearColorResolveCache();
      expect(resolveColorHex("#abc", "gray")).toBe("#aabbcc");
    });

    it("blendTokenHex mixes two palette keys", () => {
      const mixed = blendTokenHex("primary", "light", 0.28);
      expect(mixed).toMatch(/^#[0-9a-f]{6}$/u);
    });

    it("resolveColorRgba returns byte tuple", () => {
      clearColorResolveCache();
      expect(resolveColorRgba("var(--color-gray)", "gray")).toEqual([123, 130, 125, 255]);
    });

    it("relativeLuminance orders light above dark palette tokens", () => {
      expect(relativeLuminance(tokenHex("light"))).toBeGreaterThan(relativeLuminance(tokenHex("dark")));
    });

    it("readableForegroundHex picks light text on dark fills and dark text on light fills", () => {
      clearColorResolveCache();
      expect(readableForegroundHex("var(--color-dark)")).toBe(tokenHex("light"));
      expect(readableForegroundHex("var(--color-light)")).toBe(tokenHex("dark"));
    });

    it("resolveColorHex resolves semantic element vars to gray not foreground", () => {
      clearColorResolveCache();
      expect(resolveColorHex("var(--color-element)", "gray")).toBe("#7b827d");
      expect(resolveSemanticColorHex("border-element-color", "gray")).toBe("#7b827d");
      expect(resolveColorHex("var(--color-element)", "gray")).not.toBe(tokenHex("dark"));
    });

    it("serializeCanvasThemeJson emits token board palette fields", () => {
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as {
        rasterClear: number[];
        nodeFill: number[];
        nodeStroke: number[];
        nodeStrokeHovered: number[];
        nodeStrokeSelected: number[];
        edgeStroke: number[];
        handleStroke: number[];
        handleStrokeHovered: number[];
        handleFill: number[];
        labelFill: number[];
        labelFillHovered: number[];
        labelHalo: number[];
        gridMinorStroke: number[];
      };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
      expect(parsed.nodeFill).toHaveLength(4);
      expect(parsed.labelFill).toEqual([123, 130, 125, 255]);
      expect(parsed.edgeStroke).toEqual([123, 130, 125, 255]);
      expect(parsed.handleStroke).toEqual([123, 130, 125, 255]);
      expect(parsed.handleStrokeHovered).toEqual(parsed.handleStroke);
      expect(parsed.nodeStrokeSelected).toEqual(STYLING_BOARD_PALETTES.light.nodeStrokeSelected);
      expect(parsed.handleFill[3]).toBe(0);
      expect(parsed.gridMinorStroke[3]).toBeLessThan(255);
      const dark = JSON.parse(serializeCanvasThemeJson("dark")) as { rasterClear: number[]; labelFill: number[] };
      expect(dark.rasterClear).toEqual(STYLING_BOARD_PALETTES.dark.rasterClear);
      expect(dark.rasterClear).not.toEqual(parsed.rasterClear);
      expect(dark.labelFill).toEqual(STYLING_BOARD_PALETTES.dark.labelFill);
      expect(dark.labelFill).not.toEqual(parsed.labelFill);
    });

    it("syncSessionCanvasTheme pushes serialized palette into a session", () => {
      const calls: string[] = [];
      syncSessionCanvasTheme({
        setCanvasThemeJson(json: string) {
          calls.push(json);
        },
      });
      expect(calls.length).toBe(1);
      const parsed = JSON.parse(calls[0]!) as { rasterClear: number[] };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
    });
  });
}
//#endregion 🧪Tests
//#endregion 🔖resolve

//#region 🔖theme
//#region 🔑Premades
/** @emoji 🎨 The default "semio" theme, built from 🔣tokens.json at generate time. */
export function semioTheme(): UiTheme {
  return STYLING_SEMIO_THEME as unknown as UiTheme;
}

let _builtinThemesCache: UiTheme[] | undefined;

/** @emoji 🎨 Premade themes bundled with the app: semio plus any `framework/ui/styling/theme/*.theme.json` presets. `import.meta.glob` is a Vite build-time macro — it only exists once actually *called* in the bundled output, so this must call it directly inside a try/catch rather than probe for it first (`import.meta.glob` as a bare property is always `undefined` at runtime, in Vite and everywhere else; a `typeof` guard would never be true). Outside Vite (bun scripts, tests) the call throws and this falls back to semio only. */
export function builtinUiThemes(): readonly UiTheme[] {
  if (_builtinThemesCache) {
    return _builtinThemesCache;
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
  _builtinThemesCache = themes;
  return themes;
}
//#endregion 🔑Premades

//#region 🔑ActiveTheme
let _activeUiTheme: UiTheme | undefined;
const _activeUiThemeSubscribers = new Set<(theme: UiTheme) => void>();
const _appliedThemeCssProps = new Set<string>();

/** @emoji 🎨 The currently active theme (defaults to semio before any theme is set). */
export function activeUiTheme(): UiTheme {
  return _activeUiTheme ?? semioTheme();
}

/** @emoji 🎨 Registers a callback invoked whenever the active theme changes. Returns an unsubscribe function. */
export function subscribeActiveUiTheme(callback: (theme: UiTheme) => void): () => void {
  _activeUiThemeSubscribers.add(callback);
  return () => _activeUiThemeSubscribers.delete(callback);
}

function setCssVar(root: HTMLElement, name: string, value: string): void {
  root.style.setProperty(name, value);
  _appliedThemeCssProps.add(name);
}

/** @emoji 🎨 Applies a theme's colors/spacing/fonts/strokes/glass metrics as inline `documentElement` CSS var overrides, clearing any previous overrides first. Applied unconditionally (even for a pristine semio theme, whose values equal the generated CSS defaults) so a semio-based *draft* with edits — which still carries `id: "semio"` until saved — is never mistaken for the untouched default. */
export function applyUiThemeToDocument(theme: UiTheme): void {
  if (typeof document === "undefined") {
    return;
  }
  const root = document.documentElement;
  for (const name of _appliedThemeCssProps) {
    root.style.removeProperty(name);
  }
  _appliedThemeCssProps.clear();
  root.dataset.uiTheme = theme.id;
  for (const [key, hex] of Object.entries(theme.colors)) {
    setCssVar(root, `--color-${key.replaceAll("_", "-")}`, hex);
  }
  for (const [key, value] of Object.entries(theme.spacing)) {
    setCssVar(root, `--spacing-${key.replaceAll("_", "-")}`, value);
  }
  if (theme.fontStacks.sans) setCssVar(root, "--font-sans", theme.fontStacks.sans);
  if (theme.fontStacks.serif) setCssVar(root, "--font-serif", theme.fontStacks.serif);
  if (theme.fontStacks.mono) setCssVar(root, "--font-mono", theme.fontStacks.mono);
  const hairline = theme.strokes.chromeBorderHairline;
  if (typeof hairline === "number") setCssVar(root, "--stroke-hairline", `${hairline}px`);
  const chrome = theme.metrics.chrome;
  if (chrome) {
    if (typeof chrome.glassSaturate === "number") setCssVar(root, "--glass-saturate", `${chrome.glassSaturate}`);
    if (typeof chrome.shadeStepPercent === "number") setCssVar(root, "--level-shade-step", `${chrome.shadeStepPercent}%`);
    if (typeof chrome.elementStepPercent === "number") setCssVar(root, "--element-shade-step", `${chrome.elementStepPercent}%`);
    if (typeof chrome.hoverStepPercent === "number") setCssVar(root, "--hover-shade-step", `${chrome.hoverStepPercent}%`);
    if (typeof chrome.glassAlphaStep === "number") setCssVar(root, "--glass-alpha-step", `${chrome.glassAlphaStep}`);
    if (typeof chrome.glassBlurStepPx === "number") setCssVar(root, "--glass-blur-step", `${chrome.glassBlurStepPx / 16}rem`);
  }
  clearColorResolveCache();
}

/** @emoji 🎨 Sets the active theme, applies it to the document, and notifies subscribers. */
export function setActiveUiTheme(theme: UiTheme): void {
  _activeUiTheme = theme;
  applyUiThemeToDocument(theme);
  for (const subscriber of _activeUiThemeSubscribers) {
    subscriber(theme);
  }
}
//#endregion 🔑ActiveTheme

//#region 🧪ThemeTests
if (import.meta.vitest) {
  const { afterEach, describe, expect, it } = import.meta.vitest;

  afterEach(() => {
    _activeUiTheme = undefined;
    if (typeof document !== "undefined") {
      for (const name of [..._appliedThemeCssProps]) {
        document.documentElement.style.removeProperty(name);
      }
      delete document.documentElement.dataset.uiTheme;
    }
    _appliedThemeCssProps.clear();
  });

  describe("theme registry", () => {
    it("builtinUiThemes always includes semio first", () => {
      const themes = builtinUiThemes();
      expect(themes[0]!.id).toBe("semio");
    });

    it("builtinUiThemes discovers the mono premade via import.meta.glob", () => {
      const themes = builtinUiThemes();
      expect(themes.map((t) => t.id)).toContain("mono");
    });

    it("activeUiTheme defaults to semio", () => {
      expect(activeUiTheme().id).toBe("semio");
    });

    it("serializeCanvasThemeJson matches the baked palette before any theme is set", () => {
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as { rasterClear: number[] };
      expect(parsed.rasterClear).toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
    });

    it("setActiveUiTheme changes serializeCanvasThemeJson output and notifies subscribers", () => {
      const mono = builtinUiThemes().find((t) => t.id === "mono");
      if (!mono) throw new Error("mono premade not discovered by builtinUiThemes()");
      const seen: string[] = [];
      const unsubscribe = subscribeActiveUiTheme((t) => seen.push(t.id));
      setActiveUiTheme(mono);
      expect(seen).toEqual(["mono"]);
      const parsed = JSON.parse(serializeCanvasThemeJson("light")) as { rasterClear: number[] };
      expect(parsed.rasterClear).not.toEqual(STYLING_BOARD_PALETTES.light.rasterClear);
      unsubscribe();
    });

    it("applyUiThemeToDocument writes the level knob CSS vars, never the deleted per-tier ones", () => {
      const theme = semioTheme();
      applyUiThemeToDocument(theme);
      const root = document.documentElement;
      const chrome = theme.metrics.chrome;
      if (typeof chrome?.shadeStepPercent === "number") {
        expect(root.style.getPropertyValue("--level-shade-step")).toBe(`${chrome.shadeStepPercent}%`);
      }
      if (typeof chrome?.glassAlphaStep === "number") {
        expect(root.style.getPropertyValue("--glass-alpha-step")).toBe(`${chrome.glassAlphaStep}`);
      }
      for (const deleted of ["--glass-panel-blur", "--glass-panel-alpha", "--glass-menu-alpha", "--glass-window-options-blur", "--glass-window-options-alpha"]) {
        expect(root.style.getPropertyValue(deleted)).toBe("");
      }
    });
  });
}
//#endregion 🧪ThemeTests
//#endregion 🔖theme

//#region 🔖icon-render-port
//#region 🔖IconRenderPort
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
//#endregion 🔖IconRenderPort
//#endregion 🔖icon-render-port

//#region 🧭ElementState
/** @emoji 🧭 The one shared, compile-time-enforced state model every rendered UI element carries:
 * `state` × `status` × `hover` × `selected`. Mirrors the Rust `UiState`/`UiStatus`/`UiPresence`
 * model in `ui_wgpu` (see `framework/ui/wgpu/rs/lib.rs`'s 🔖Presence region) — string-literal unions here,
 * since this package has no dependency on the Rust crate or its generated bindings. `Hidden` makes
 * every other axis irrelevant: a hidden element is not rendered at all. */
export const UI_STATES = ["introducing", "celebrating", "previewed", "normal", "disabled", "hidden"] as const;
export type UiState = (typeof UI_STATES)[number];

/** @emoji 🧭 The activity lifecycle of a UI element, orthogonal to {@link UiState} and composable with it. */
export const UI_STATUSES = ["waiting", "loading", "idle", "finished"] as const;
export type UiStatus = (typeof UI_STATUSES)[number];

export interface UiElementState {
  readonly state: UiState;
  readonly status: UiStatus;
  readonly hover: boolean;
  readonly selected: boolean;
}

const DEFAULT_UI_ELEMENT_STATE: UiElementState = { state: "normal", status: "idle", hover: false, selected: false };

/** @emoji 🧭 Fills in every axis with its default (`normal`/`idle`/`false`/`false`) — the one place that convention lives. */
export function resolveElementState(partial?: Partial<UiElementState>): UiElementState {
  return { ...DEFAULT_UI_ELEMENT_STATE, ...partial };
}

/** @emoji 🙈 `true` only for `state === "hidden"` — callers must not render/lay out/hit-test the element. */
export function elementStateHidden(s: Pick<UiElementState, "state">): boolean {
  return s.state === "hidden";
}

/** @emoji 🧭 The one function that emits the shared `data-ui-*` attribute vocabulary every element's
 * markup carries. Two axes carry a SECOND attribute alongside `data-ui-state`/`data-ui-status`:
 * `introducing` also stamps `data-introduced="true"` — the exact attribute `UIIntroduction`'s
 * tour-driven reveal already stamps imperatively (see `framework/ui/js/react/index.tsx`), so an authored
 * `state: "introducing"` and a live tour step converge on the identical CSS rule
 * (`[data-introduced="true"]` in `🎨ui.css`) with no duplicate styling to maintain. `celebrating`
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

/** @emoji 🎨 The shared precedence resolver for renderers that can't use CSS/data-attributes at all
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
//#endregion 🧭ElementState
