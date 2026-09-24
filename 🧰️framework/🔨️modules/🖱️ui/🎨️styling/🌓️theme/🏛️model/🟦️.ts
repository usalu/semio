/** @emoji 🏛️ `UiTheme` model: paint-ref resolver, parse/serialize, shared by the token generator (`📽️projection`) and the
 * runtime theme engine (`🌓️theme`). Dependency-free by contract: the token generator runs on a fresh clone, before
 * `🤖️generated/🔤️tokens/🟦️.ts` exists, and the theme barrel that re-exports this model imports those tokens. */
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

/** @emoji 🗂️ The six paint groups every appearance carries — `outcome` (error/warning/success/progress
 * status paints) and `diagram` (transparency checkerboard + diagram stroke/seam/accent) exist so the
 * wgpu target reads them from this one source instead of hand-written `Rgba` literals. */
export type ThemePaletteGroup = "board" | "map" | "canvas" | "chrome" | "outcome" | "diagram";

/** @emoji 🌓️ Light/dark palette dimension within a theme. */
export type ThemeAppearanceName = "light" | "dark";

/** @emoji 🖼️ Optional runtime icon appearance overrides keyed by compile-time icon ids. */
export interface UiThemeIcons {
  readonly aliases?: Readonly<Partial<Record<string, string>>>;
  readonly variants?: Readonly<Partial<Record<string, string>>>;
  readonly themedAliases?: Readonly<Partial<Record<string, string>>>;
  readonly themedVariants?: Readonly<Partial<Record<string, string>>>;
}

const THEME_PALETTE_GROUPS: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome", "outcome", "diagram"];

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
