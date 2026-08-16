import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
import { oklabMix } from "../../../../🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts";

//#region 🎨️DesignTokenPaints
export type PrintTheme = "light" | "dark";

export type PrintDesignTokens = {
  readonly colors: Record<string, string>;
  readonly spacing: Record<string, string>;
  readonly strokes?: Record<string, number | number[]>;
  readonly opacities?: { readonly glassPanelAlpha?: number };
  readonly levels?: PrintStylingLevels;
  readonly appearances?: Record<string, Record<string, Record<string, PrintPaintReference>>>;
  readonly metrics?: {
    readonly chrome?: {
      readonly controlHeightUiSpacing?: number;
      readonly paddingStandardUiSpacing?: number;
      readonly navbarHeightUiSpacing?: number;
      readonly footerHeightUiSpacing?: number;
      readonly glassPanelBlurPx?: number;
      readonly glassSaturate?: number;
    };
    readonly typography?: {
      readonly textXsPx?: number;
      readonly text2xsPx?: number;
      readonly textSmPx?: number;
    };
  };
};

export type PrintStylingLevels = {
  readonly names: readonly string[];
  readonly shadeStepPercent: number;
  readonly glassAlphaStep: number;
  readonly glassBlurStepPx: number;
  readonly glassSaturate: number;
};

export type PrintPanelGlassStyle = {
  readonly tintHex: string;
  readonly alpha: number;
  readonly blurPixels: number;
  readonly saturation: number;
};

type PrintPaintReference = {
  readonly token?: string;
  readonly hex?: string;
  readonly alpha?: number;
  readonly mix?: readonly [string, string, number];
};

type Rgba8 = [number, number, number, number];

const DEFAULT_LEVELS: PrintStylingLevels = {
  names: ["base", "window", "pane", "panel", "dialog", "menu"],
  shadeStepPercent: 5,
  glassAlphaStep: 0.12,
  glassBlurStepPx: 8,
  glassSaturate: 1.45,
};

const CHROME_PAINT_KEYS = ["base", "borderNormal", "borderEmphasized", "activeBase", "activeForeground", "foreground", "accent"] as const;
const PRINT_LEVEL_SURFACE_KEYS = ["window", "pane", "panel", "dialog", "menu"] as const;
const workspaceRoot = getWorkspaceRoot();
const productRoot = join(workspaceRoot, "🧰️framework", "🛍️products", "📓️print");
const tokensPath = join(workspaceRoot, "🧰️framework", "🔨️modules", "🖱️ui", "🎨️styling", "📦️packages", "🦀️rust", "🔣️tokens.json");
const latexDirectory = join(productRoot, "🖋️latex");
const latexTokensPath = join(latexDirectory, "semio-tokens.sty");

/** 🎨️ Loads the canonical framework design-token document for print rendering. */
export function loadPrintDesignTokens(): PrintDesignTokens {
  return JSON.parse(readFileSync(tokensPath, "utf8")) as PrintDesignTokens;
}

/** 🎨️ Renders the canonical LaTeX stylesheet text from framework design-token paints. */
export function renderPrintLatexTokenStylesheet(tokens: PrintDesignTokens = loadPrintDesignTokens()): string {
  const lines: string[] = ["% Generated from framework/ui/styling/🔣️tokens.json — run `bun ./📜️script.ts generate`.", "\\NeedsTeXFormat{LaTeX2e}", "\\ProvidesPackage{semio-tokens}[2026/07/06 v0.1.0 semio design tokens]", "\\RequirePackage{xcolor}", ""];
  for (const [key, hex] of Object.entries(tokens.colors)) lines.push(`\\definecolor{${latexColorKey(key)}}{HTML}{${hex.replace(/^#/, "")}}`);
  lines.push("");
  const unitFactor = remFactor(tokens.spacing.compact ?? "0.2rem");
  const unitEm = `${+unitFactor.toFixed(3)}em`;
  lines.push(`\\newcommand{\\semio@spacing@unit}{${unitEm}}`);
  lines.push(`\\newcommand{\\semio@spacing@single}{${unitEm}}`);
  lines.push(`\\newcommand{\\semio@spacing@double}{${+(unitFactor * 2).toFixed(3)}em}`);
  const hairline = typeof tokens.strokes?.chromeBorderHairline === "number" ? tokens.strokes.chromeBorderHairline * 0.75 : typeof tokens.strokes?.gridLarge === "number" ? tokens.strokes.gridLarge * 0.75 : 0.75;
  const strokeDefault = typeof tokens.strokes?.chromeBorderDefault === "number" ? tokens.strokes.chromeBorderDefault * 0.75 : typeof tokens.strokes?.edgeBase === "number" ? tokens.strokes.edgeBase * 0.75 : 1.5;
  const strokeFocus = typeof tokens.strokes?.chromeBorderFocus === "number" ? tokens.strokes.chromeBorderFocus * 0.75 : typeof tokens.strokes?.dagNodeSelected === "number" ? tokens.strokes.dagNodeSelected * 0.75 : 1.75;
  lines.push(`\\newcommand{\\semio@stroke@hairline}{${hairline}pt}`);
  lines.push(`\\newcommand{\\semio@stroke@default}{${strokeDefault}pt}`);
  lines.push(`\\newcommand{\\semio@stroke@focus}{${strokeFocus}pt}`);
  lines.push("");
  const chromeMetrics = tokens.metrics?.chrome;
  if (chromeMetrics) {
    lines.push(`\\newcommand{\\semio@chrome@titlebar@height}{${+(unitFactor * (chromeMetrics.controlHeightUiSpacing ?? 7)).toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@padding}{${+(unitFactor * (chromeMetrics.paddingStandardUiSpacing ?? 1)).toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@navbar@height}{${+(unitFactor * (chromeMetrics.navbarHeightUiSpacing ?? 9)).toFixed(3)}em}`);
    lines.push(`\\newcommand{\\semio@chrome@footer@height}{${+(unitFactor * (chromeMetrics.footerHeightUiSpacing ?? 9)).toFixed(3)}em}`);
    lines.push("\\newcommand{\\semio@chrome@icon@scale}{1}");
    lines.push("\\newcommand{\\semio@chrome@icon@scale@footer}{1}");
  }
  const typography = tokens.metrics?.typography;
  if (typography) {
    lines.push(`\\newcommand{\\semio@chrome@font@chip}{${+((typography.text2xsPx ?? 9.6) * 0.75).toFixed(3)}pt}`);
    lines.push(`\\newcommand{\\semio@chrome@font@body}{${+((typography.textSmPx ?? 12.8) * 0.75).toFixed(3)}pt}`);
  }
  lines.push("");
  for (const theme of ["light", "dark"] as const) {
    const chrome = tokens.appearances?.[theme]?.chrome;
    if (!chrome) continue;
    for (const key of CHROME_PAINT_KEYS) {
      const paint = chrome[key];
      if (paint) lines.push(`\\definecolor{semio-chrome-${theme}-${latexChromePaintKey(key)}}{HTML}{${resolvePaint(tokens.colors, paint).replace(/^#/, "")}}`);
    }
    const basePaint = chrome.base;
    if (!basePaint) throw new Error(`tokens.appearances.${theme}.chrome.base missing`);
    lines.push(`\\definecolor{semio-chrome-${theme}-canvas}{HTML}{${resolvePaint(tokens.colors, basePaint).replace(/^#/, "")}}`);
    for (const name of PRINT_LEVEL_SURFACE_KEYS) lines.push(`\\definecolor{semio-chrome-${theme}-${name}}{HTML}{${levelSurfaceHex(tokens, theme, name).replace(/^#/, "")}}`);
  }
  return `${lines.join("\n")}\n`;
}

/** 🎨️ Writes the generated LaTeX design-token stylesheet. */
export function writePrintLatexTokenStylesheet(): void {
  mkdirSync(latexDirectory, { recursive: true });
  writeFileSync(latexTokensPath, renderPrintLatexTokenStylesheet(), "utf8");
}

/** 🪟️ Resolves the token-derived panel-glass paint for one print theme. */
export function resolvePrintPanelGlassStyle(theme: PrintTheme): PrintPanelGlassStyle {
  const tokens = loadPrintDesignTokens();
  const levels = resolveLevels(tokens);
  const panelIndex = levelIndex(levels, "panel");
  return {
    tintHex: levelSurfaceHex(tokens, theme, "panel"),
    alpha: 1 - panelIndex * levels.glassAlphaStep,
    blurPixels: panelIndex * levels.glassBlurStepPx,
    saturation: levels.glassSaturate,
  };
}

function parseHex(hex: string): [number, number, number] {
  const value = hex.trim().replace(/^#/, "");
  if (value.length === 3) return [Number.parseInt(value[0]! + value[0]!, 16), Number.parseInt(value[1]! + value[1]!, 16), Number.parseInt(value[2]! + value[2]!, 16)];
  const integer = Number.parseInt(value, 16);
  return [(integer >> 16) & 0xff, (integer >> 8) & 0xff, integer & 0xff];
}

function hexToRgba8(hex: string): Rgba8 {
  const [red, green, blue] = parseHex(hex);
  return [red, green, blue, 255];
}

function rgba8ToHex([red, green, blue]: Rgba8): string {
  return `#${[red, green, blue].map((value) => value.toString(16).padStart(2, "0")).join("")}`;
}

function resolveLevels(tokens: PrintDesignTokens): PrintStylingLevels {
  return { ...DEFAULT_LEVELS, ...tokens.levels, names: tokens.levels?.names ?? DEFAULT_LEVELS.names };
}

function levelIndex(levels: PrintStylingLevels, name: string): number {
  const index = levels.names.indexOf(name);
  if (index < 0) throw new Error(`tokens.levels.names missing ${name}`);
  return index;
}

function levelSurfaceHex(tokens: PrintDesignTokens, theme: PrintTheme, name: string): string {
  const chrome = tokens.appearances?.[theme]?.chrome;
  if (!chrome?.base || !chrome.foreground) throw new Error(`tokens.appearances.${theme}.chrome needs base and foreground`);
  const levels = resolveLevels(tokens);
  return rgba8ToHex(oklabMix(hexToRgba8(resolvePaint(tokens.colors, chrome.base)), hexToRgba8(resolvePaint(tokens.colors, chrome.foreground)), levelIndex(levels, name) * (levels.shadeStepPercent / 100)));
}

function resolvePaint(colors: Record<string, string>, reference: PrintPaintReference): string {
  if (reference.mix) {
    const [left, right, ratio] = reference.mix;
    const rightHex = right === "transparent" ? "#000000" : colors[right];
    if (!rightHex) throw new Error(`tokens.colors[${right}] missing`);
    return blendHex(colors[left]!, rightHex, ratio);
  }
  if (reference.hex) return reference.hex;
  if (reference.token) {
    const token = colors[reference.token];
    if (!token) throw new Error(`tokens.colors[${reference.token}] missing`);
    return token;
  }
  throw new Error("paint ref needs token, hex, or mix");
}

function blendHex(left: string, right: string, leftRatio: number): string {
  const [leftRed, leftGreen, leftBlue] = parseHex(left);
  const [rightRed, rightGreen, rightBlue] = parseHex(right);
  const ratio = Math.min(1, Math.max(0, leftRatio));
  return `#${[leftRed * ratio + rightRed * (1 - ratio), leftGreen * ratio + rightGreen * (1 - ratio), leftBlue * ratio + rightBlue * (1 - ratio)].map((value) => Math.round(value).toString(16).padStart(2, "0")).join("")}`;
}

function remFactor(rem: string): number {
  const match = rem.match(/^([\d.]+)rem$/);
  return match ? Number.parseFloat(match[1]!) : Number.parseFloat(rem) || 0;
}

function latexColorKey(key: string): string {
  return `semio-${key.replaceAll("_", "-")}`;
}

function latexChromePaintKey(key: string): string {
  return key.replace(/[A-Z]/g, (match) => `-${match.toLowerCase()}`);
}
//#endregion 🎨️DesignTokenPaints
