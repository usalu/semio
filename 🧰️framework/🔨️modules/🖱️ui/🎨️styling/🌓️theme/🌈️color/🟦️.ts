import { type StylingTokenKey, STYLING_TOKENS } from "../../🤖️generated/🔤️tokens/🟦️.ts";
import { ephemeralMap } from "@semio-tech/framework-transient";
import { currentStylingAppearanceName, stylingAppearanceRootElement } from "../🎛️active-theme/🟦️.ts";

//#region 🔑️TokenRefs
/** 🧱️ Builds a primitive palette CSS variable reference (`var(--color-<key>)`). */
export function tokenVar(key: StylingTokenKey | string): string {
  return `var(--color-${key.replaceAll("_", "-")})`;
}

/** 📘️ Builds a semantic UI CSS variable reference (`var(--<name>)`). */
export function semanticVar(name: string): string {
  const trimmed = name.startsWith("--") ? name.slice(2) : name;
  return `var(--${trimmed})`;
}

/** 🌈️ Builds a Tailwind `@theme inline` color alias (`var(--color-<name>)`). */
export function themeColorVar(name: string): string {
  return `var(--color-${name.replaceAll("_", "-")})`;
}

/** 📐️ Permanent X/Y/Z paints for gumball and view/projection gizmos — primary / secondary / tertiary, never active/hover chrome. */
export const SPATIAL_AXIS_COLOR_REFS = {
  x: tokenVar("primary"),
  y: tokenVar("secondary"),
  z: tokenVar("tertiary"),
} as const;

/** 🧭️ Resolved `#rrggbb` axis paints for spatial manipulators and navigation cubes. */
export function resolveSpatialAxisColors(): { readonly x: string; readonly y: string; readonly z: string } {
  return {
    x: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.x, "primary"),
    y: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.y, "secondary"),
    z: resolveColorHex(SPATIAL_AXIS_COLOR_REFS.z, "tertiary"),
  };
}

/** 🔑️ Returns the canonical hex for a palette token key (headless-safe). */
export function tokenHex(key: StylingTokenKey | string): string {
  return STYLING_TOKENS[key as StylingTokenKey] ?? STYLING_TOKENS.gray;
}
//#endregion 🔑️TokenRefs

const _resolveCache = ephemeralMap<string, string>("framework.modules.ui.styling.packages.typescript.index.ts._resolveCache");
export const _readableForegroundCache = ephemeralMap<string, string>("framework.modules.ui.styling.packages.typescript.index.ts._readableForegroundCache");

/** 🔄️ Clears the color resolve cache (theme switches / tests). */
export function clearColorResolveCache(): void {
  _resolveCache.clear();
  _readableForegroundCache.clear();
}

export function normalizeHex(hex: string): string {
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

export function hexChannel(hex: string, start: number): number {
  return Number.parseInt(hex.slice(start, start + 2), 16);
}

function rgbToHex(r: number, g: number, b: number): string {
  const clamp = (n: number) => Math.min(255, Math.max(0, Math.round(n)));
  return `#${clamp(r).toString(16).padStart(2, "0")}${clamp(g).toString(16).padStart(2, "0")}${clamp(b).toString(16).padStart(2, "0")}`;
}

/** 🎨️ Linear sRGB blend between two palette token keys (headless color-mix approximation). */
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
  if (semantic === "foreground" || semantic === "color-foreground" || semantic === "border-emphasized-color" || semantic === "color-emphasized") {
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
  const host = stylingAppearanceRootElement();
  if (!host) {
    return "";
  }
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (host.classList.contains("dark")) {
    el.classList.add("dark");
  }
  host.appendChild(el);
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

/** 🔎️ Resolves a CSS color expression or hex literal to `#rrggbb`, using palette fallback in headless mode. */
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

/** 🏷️ Resolves a semantic CSS custom property (e.g. `--foreground`) to `#rrggbb`. */
export function resolveSemanticColorHex(cssVar: string, fallbackKey: StylingTokenKey | string = "gray"): string {
  const name = cssVar.startsWith("--") ? cssVar : `--${cssVar}`;
  return resolveBackgroundColorHex(`var(${name})`, fallbackKey);
}

/** 🖌️ Resolves a CSS background-color expression to `#rrggbb`. */
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

/** 📦️ Resolves a CSS color expression to RGBA8888 for canvas WASM theme payloads. */
export function resolveColorRgba(ref: string, fallbackKey: StylingTokenKey | string = "gray", alpha = 255): [number, number, number, number] {
  const hex = normalizeHex(resolveColorHex(ref, fallbackKey));
  const a = hex.length === 9 ? Number.parseInt(hex.slice(7, 9), 16) : alpha;
  return [hexChannel(hex, 1), hexChannel(hex, 3), hexChannel(hex, 5), a];
}

/** ♻️ Converts `#rrggbb` to a Three.js-friendly hex number (`0xrrggbb`). */
export function hexToThreeColor(hex: string): number {
  const norm = normalizeHex(hex);
  return Number.parseInt(norm.slice(1, 7), 16);
}

/** 🧊️ Resolves a CSS color expression to a Three.js-friendly hex number. */
export function resolveThreeColor(ref: string, fallbackKey: StylingTokenKey | string = "gray"): number {
  return hexToThreeColor(resolveColorHex(ref, fallbackKey));
}
