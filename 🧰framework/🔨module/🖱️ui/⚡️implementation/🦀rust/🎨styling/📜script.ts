#!/usr/bin/env bun
/** @emoji ⚙️ Reads `framework/ui/styling/🔣tokens.json`; emits palette CSS, TS, C#, Rust, and Python styling artifacts. */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, getWorkspaceRoot } from "../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { parseUiTheme, resolveThemeMetrics, resolveThemePaint, type ThemePaintRef, type UiTheme } from "../../../../../../🧰framework/🔨module/🖱️ui/⚡️implementation/🟦typescript/🎨styling/📦index.ts";

/** @emoji 🧭 `import.meta.dir` is a Bun-only extension; fall back to `import.meta.url` so this module loads
 * under Vitest (which transforms it outside the Bun runtime) for the inline 🌓Levels generator tests. */
const stylingRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));
const tokensPath = join(stylingRoot, "🔣tokens.json");
const generatedCssDir = join(stylingRoot, "🤖generated");
const tsStylingDir = join(stylingRoot, "..", "..", "🟦typescript", "🎨styling");
const netPaletteDir = join(stylingRoot, "net", "Elements.Styling", "Generated");
const pyGeneratedPath = join(stylingRoot, "..", "..", "🐍python", "🎨styling", "🎨styling", "🤖generated.py");
const rustGeneratedPath = join(stylingRoot, "🤖generated.rs");
const repoRoot = getWorkspaceRoot();

/** @emoji 📁 Canonical `framework/ui/asset` directory (fonts, cursors, …). */
export const ELEMENTS_ASSETS_ROOT = join(stylingRoot, "..", "asset");
const elementsAssetsRoot = ELEMENTS_ASSETS_ROOT;
const composeNetPaletteDir = join(repoRoot, "compose", "client", "lib", "net", "Elements.Styling", "Generated");

const GOOGLE_FONTS_UA = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const GOOGLE_FONT_QUERIES: Record<string, string> = {
  "font/anta": "Anta",
  "font/kelly-slab": "Kelly Slab",
  "font/share-tech-mono": "Share Tech Mono",
  "font/noto-emoji": "Noto Emoji",
};

type Rgba8 = [number, number, number, number];

type PaintRef = ThemePaintRef;

interface Tokens {
  version: number;
  colors: Record<string, string>;
  spacing: Record<string, string>;
  fontStacks: Record<string, string>;
  fontFaces: { family: string; src: string }[];
  canvasFonts?: Record<string, string>;
  strokes?: Record<string, number | number[]>;
  radii?: Record<string, number>;
  opacities?: Record<string, number>;
  metrics?: Record<string, Record<string, number | number[]>>;
  levels?: StylingLevels;
  appearances?: Record<string, Record<string, Record<string, PaintRef>>>;
}

/** @emoji 🌓 Knobs driving the formula-derived 6-level UI surface system (`base..menu`); see contract at
 * `.🦑repo/🎫tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`. */
interface StylingLevels {
  names: readonly string[];
  shadeStepPercent: number;
  elementStepPercent: number;
  hoverStepPercent: number;
  glassAlphaStep: number;
  glassBlurStepPx: number;
  glassSaturate: number;
  veilAlphaExtraSteps: number;
  zStep: number;
}

const APPEARANCE_NAMES = ["light", "dark"] as const;

function colorKeyToCssVar(key: string): string {
  return `--color-${key.replaceAll("_", "-")}`;
}

function toPascalCase(s: string): string {
  return s
    .split(/[^a-zA-Z0-9]+/)
    .filter(Boolean)
    .map((p) => p[0]!.toUpperCase() + p.slice(1))
    .join("");
}

function toSnakeCase(s: string): string {
  return s.replace(/[A-Z]/g, (m) => `_${m.toLowerCase()}`);
}

function toScreamingSnake(s: string): string {
  return toSnakeCase(s).toUpperCase();
}

function loadTokens(): Tokens {
  const raw = readFileSync(tokensPath, "utf8");
  return JSON.parse(raw) as Tokens;
}

/** @emoji 📏 Derives dag component width as twice the IO channel column width. */
function resolveMetrics(metrics: Tokens["metrics"]): NonNullable<Tokens["metrics"]> {
  return resolveThemeMetrics(metrics ?? {}) as NonNullable<Tokens["metrics"]>;
}

function resolvePaint(colors: Record<string, string>, ref: PaintRef): Rgba8 {
  return resolveThemePaint(colors, ref);
}

function srgbByteToLinear(c: number): number {
  const x = c / 255;
  return x <= 0.04045 ? x / 12.92 : ((x + 0.055) / 1.055) ** 2.4;
}

function rgba8ToLinear(rgba: Rgba8): [number, number, number, number] {
  return [srgbByteToLinear(rgba[0]), srgbByteToLinear(rgba[1]), srgbByteToLinear(rgba[2]), rgba[3] / 255];
}

function linearToSrgbByte(x: number): number {
  const c = x <= 0.0031308 ? x * 12.92 : 1.055 * x ** (1 / 2.4) - 0.055;
  return Math.round(Math.min(1, Math.max(0, c)) * 255);
}

//#region 🌓Levels
/** @emoji 🌓 sRGB(linear) → Oklab, per Björn Ottosson's reference matrices (https://bottosson.github.io/posts/oklab/). */
function linearToOklab(r: number, g: number, b: number): [number, number, number] {
  const l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
  const m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
  const s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
  const l_ = Math.cbrt(l);
  const m_ = Math.cbrt(m);
  const s_ = Math.cbrt(s);
  return [0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_, 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_, 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_];
}

/** @emoji 🌓 Oklab → sRGB(linear), inverse of {@link linearToOklab}. */
function oklabToLinear(L: number, a: number, b: number): [number, number, number] {
  const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
  const s_ = L - 0.0894841775 * a - 1.291485548 * b;
  const l = l_ ** 3;
  const m = m_ ** 3;
  const s = s_ ** 3;
  return [4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s, -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s, -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s];
}

/** @emoji 🌓 Mixes two resolved paints in Oklab space (srgb → linear → oklab → lerp → back), `t=0` returns `a`, `t=1` returns `b`. Alpha lerps linearly. Powers the formula-derived level/element paint ladders (see contract's CSS MECHANISM section). */
export function oklabMix(a: Rgba8, b: Rgba8, t: number): Rgba8 {
  const la = rgba8ToLinear(a);
  const lb = rgba8ToLinear(b);
  const oa = linearToOklab(la[0], la[1], la[2]);
  const ob = linearToOklab(lb[0], lb[1], lb[2]);
  const mixed: [number, number, number] = [oa[0] * (1 - t) + ob[0] * t, oa[1] * (1 - t) + ob[1] * t, oa[2] * (1 - t) + ob[2] * t];
  const [lr, lg, lbl] = oklabToLinear(mixed[0], mixed[1], mixed[2]);
  const alpha = la[3] * (1 - t) + lb[3] * t;
  return [linearToSrgbByte(lr), linearToSrgbByte(lg), linearToSrgbByte(lbl), Math.round(alpha * 255)];
}

const LEVELS_DEFAULT: StylingLevels = {
  names: ["base", "window", "pane", "panel", "dialog", "menu"],
  shadeStepPercent: 5,
  elementStepPercent: 6,
  hoverStepPercent: 12,
  glassAlphaStep: 0.12,
  glassBlurStepPx: 8,
  glassSaturate: 1.45,
  veilAlphaExtraSteps: 1,
  zStep: 10,
};

/** @emoji 🌓 Injects the 6 formula-derived `level<Name>` background paints and `element<Name>` element paints
 * (k=0..5, `base..menu`) into one appearance's `chrome` group, mutating it in place. `bg(k) = mix_oklab(base,
 * foreground, k*shadeStep)`, `element(k) = mix_oklab(gray, foreground, k*elementStep)` — see contract CSS MECHANISM. */
function injectLevelPaints(levels: StylingLevels, gray: Rgba8, chrome: Record<string, Rgba8>): void {
  const base = chrome.base;
  const foreground = chrome.foreground;
  if (!base || !foreground) {
    throw new Error("levels: chrome group needs base and foreground paints resolved before injecting level paints");
  }
  const shadeStep = levels.shadeStepPercent / 100;
  const elementStep = levels.elementStepPercent / 100;
  levels.names.forEach((name, k) => {
    chrome[`level${toPascalCase(name)}`] = oklabMix(base, foreground, k * shadeStep);
    chrome[`element${toPascalCase(name)}`] = oklabMix(gray, foreground, k * elementStep);
  });
}
//#endregion 🌓Levels

function rustF32(x: number): string {
  return `${x.toFixed(8).replace(/\.?0+$/, "")}_f32`;
}

function rustF64Lit(v: number): string {
  return Number.isInteger(v) ? `${v}.0` : String(v);
}

/** @emoji 🎨 Resolves every appearance's paint refs to Rgba8, then injects the formula-derived level/element
 * paints (see {@link injectLevelPaints}) into each appearance's `chrome` group so every existing emitter
 * (TS/Rust/Python, keyed off object entries) carries them automatically. */
export function resolveAppearances(tokens: Tokens): Record<string, Record<string, Record<string, Rgba8>>> {
  const out: Record<string, Record<string, Record<string, Rgba8>>> = {};
  for (const [appearanceName, groups] of Object.entries(tokens.appearances ?? {})) {
    out[appearanceName] = {};
    for (const [groupName, paints] of Object.entries(groups)) {
      out[appearanceName]![groupName] = {};
      for (const [paintName, ref] of Object.entries(paints)) {
        out[appearanceName]![groupName]![paintName] = resolvePaint(tokens.colors, ref);
      }
    }
  }
  const levels = tokens.levels ?? LEVELS_DEFAULT;
  const gray = resolvePaint(tokens.colors, { token: "gray" });
  for (const appearanceChrome of Object.values(out)) {
    const chrome = appearanceChrome!.chrome;
    if (chrome) {
      injectLevelPaints(levels, gray, chrome);
    }
  }
  return out;
}

//#region 🧪LevelsTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  function relativeLuminance(rgba: Rgba8): number {
    const [r, g, b] = rgba8ToLinear(rgba);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  }

  function contrastRatio(a: Rgba8, b: Rgba8): number {
    const la = relativeLuminance(a);
    const lb = relativeLuminance(b);
    const lighter = Math.max(la, lb);
    const darker = Math.min(la, lb);
    return (lighter + 0.05) / (darker + 0.05);
  }

  function oklabL(rgba: Rgba8): number {
    const [r, g, b] = rgba8ToLinear(rgba);
    return linearToOklab(r, g, b)[0];
  }

  function toHex(rgba: Rgba8): string {
    return `#${rgba
      .slice(0, 3)
      .map((c) => c.toString(16).padStart(2, "0"))
      .join("")}`;
  }

  describe("levels: oklabMix", () => {
    it("t=0 returns a and t=1 returns b unchanged", () => {
      const a: Rgba8 = [10, 20, 30, 255];
      const b: Rgba8 = [200, 100, 50, 255];
      expect(oklabMix(a, b, 0)).toEqual(a);
      expect(oklabMix(a, b, 1)).toEqual(b);
    });
  });

  describe("levels: derived appearance paints", () => {
    const tokens = loadTokens();
    const levels = tokens.levels ?? LEVELS_DEFAULT;
    const resolved = resolveAppearances(tokens);
    const levelKeys = levels.names.map((n) => `level${toPascalCase(n)}`);
    const elementKeys = levels.names.map((n) => `element${toPascalCase(n)}`);

    it("injects exactly 6 level + 6 element paints into every appearance's chrome group", () => {
      for (const appearance of ["light", "dark"] as const) {
        const chrome = resolved[appearance]!.chrome!;
        for (const key of [...levelKeys, ...elementKeys]) {
          expect(chrome[key]).toBeDefined();
        }
      }
    });

    it("monotonic lightness per appearance across the 6 levels (light darkens, dark lightens)", () => {
      const lightL = levelKeys.map((k) => oklabL(resolved.light!.chrome![k]!));
      const darkL = levelKeys.map((k) => oklabL(resolved.dark!.chrome![k]!));
      for (let i = 1; i < lightL.length; i++) {
        expect(lightL[i]!).toBeLessThanOrEqual(lightL[i - 1]!);
      }
      for (let i = 1; i < darkL.length; i++) {
        expect(darkL[i]!).toBeGreaterThanOrEqual(darkL[i - 1]!);
      }
    });

    it("contrast against foreground stays >= 4.5:1 at every level, both appearances", () => {
      for (const appearance of ["light", "dark"] as const) {
        const chrome = resolved[appearance]!.chrome!;
        for (const key of levelKeys) {
          expect(contrastRatio(chrome[key]!, chrome.foreground!)).toBeGreaterThanOrEqual(4.5);
        }
      }
    });

    it("monotone alpha ladder: alpha(k) = 1 - k*glassAlphaStep, strictly decreasing", () => {
      const alphas = levels.names.map((_, k) => 1 - k * levels.glassAlphaStep);
      for (let i = 1; i < alphas.length; i++) {
        expect(alphas[i]!).toBeLessThan(alphas[i - 1]!);
      }
      expect(alphas[0]).toBe(1);
      expect(alphas.at(-1)!).toBeCloseTo(1 - 5 * levels.glassAlphaStep, 10);
    });

    it("monotone blur ladder: blur(k) = k*glassBlurStepPx, strictly increasing", () => {
      const blurs = levels.names.map((_, k) => k * levels.glassBlurStepPx);
      for (let i = 1; i < blurs.length; i++) {
        expect(blurs[i]!).toBeGreaterThan(blurs[i - 1]!);
      }
      expect(blurs[0]).toBe(0);
    });

    it("pinned hex snapshot: light appearance level backgrounds", () => {
      const chrome = resolved.light!.chrome!;
      expect(levelKeys.map((k) => toHex(chrome[k]!))).toEqual(["#f7f3e3", "#e9e6d7", "#dad9cc", "#cccdc1", "#bec0b5", "#b0b4aa"]);
    });

    it("pinned hex snapshot: dark appearance level backgrounds", () => {
      const chrome = resolved.dark!.chrome!;
      expect(levelKeys.map((k) => toHex(chrome[k]!))).toEqual(["#001117", "#061a1f", "#112328", "#1c2d31", "#27373a", "#324143"]);
    });
  });
}
//#endregion 🧪LevelsTests

function paletteGroupNames(resolvedAppearances: ReturnType<typeof resolveAppearances>): string[] {
  return Object.keys(resolvedAppearances.light ?? {}).sort();
}

function emitPaletteFonts(tokens: Tokens): string {
  const assetBase = "/asset";
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣tokens.json — run `bun ./📜script.ts generate`. */"];
  for (const face of tokens.fontFaces) {
    const fam = face.family.includes(" ") ? JSON.stringify(face.family) : `"${face.family}"`;
    lines.push("@font-face {");
    lines.push(`  font-family: ${fam};`);
    lines.push(`  src: url("${assetBase}/${face.src}") format("woff2");`);
    lines.push("  font-weight: normal;");
    lines.push("  font-style: normal;");
    lines.push("  font-display: swap;");
    lines.push("}");
    lines.push("");
  }
  return lines.join("\n");
}

function emitPaletteTheme(tokens: Tokens): string {
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣tokens.json — run `bun ./📜script.ts generate`. */", "@theme {", "  /* Primary brand colors */"];
  for (const [k, v] of Object.entries(tokens.colors)) {
    lines.push(`  ${colorKeyToCssVar(k)}: ${v};`);
  }
  lines.push("  /* Font interfaces */");
  lines.push(`  --font-sans: ${tokens.fontStacks.sans};`);
  lines.push(`  --font-serif: ${tokens.fontStacks.serif};`);
  lines.push(`  --font-mono: ${tokens.fontStacks.mono};`);
  lines.push("  /* Layout spacing */");
  for (const [k, v] of Object.entries(tokens.spacing)) {
    lines.push(`  --spacing-${k.replaceAll("_", "-")}: ${v};`);
  }
  lines.push("}");
  lines.push("");
  return lines.join("\n");
}

function emitJsonConst(name: string, value: unknown, indent = ""): string {
  return `${indent}export const ${name} = ${JSON.stringify(value, null, 2).replaceAll("\n", `\n${indent}`)} as const;\n`;
}

/** @emoji 🎨 Builds the default "semio" `UiTheme` verbatim from 🔣tokens.json (the paint refs stay unresolved). */
function buildSemioUiTheme(tokens: Tokens): UiTheme {
  return {
    id: "semio",
    label: "semio",
    colors: tokens.colors,
    spacing: tokens.spacing,
    fontStacks: tokens.fontStacks,
    canvasFonts: tokens.canvasFonts ?? {},
    strokes: tokens.strokes ?? {},
    radii: tokens.radii ?? {},
    opacities: tokens.opacities ?? {},
    metrics: resolveMetrics(tokens.metrics),
    appearances: (tokens.appearances ?? {}) as UiTheme["appearances"],
  };
}

function emitTypeScriptTokens(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣tokens.json — run `bun ./📜script.ts generate`. */", ""];
  lines.push("export const STYLING_TOKENS = {");
  for (const [k, v] of Object.entries(tokens.colors)) {
    lines.push(`  "${k}": "${v}",`);
  }
  lines.push("} as const;");
  lines.push("");
  lines.push("export type StylingTokenKey = keyof typeof STYLING_TOKENS;");
  lines.push("");
  lines.push(emitJsonConst("STYLING_STROKES", tokens.strokes ?? {}));
  lines.push(emitJsonConst("STYLING_RADII", tokens.radii ?? {}));
  lines.push(emitJsonConst("STYLING_OPACITIES", tokens.opacities ?? {}));
  lines.push(emitJsonConst("STYLING_METRICS", resolveMetrics(tokens.metrics)));
  lines.push(emitJsonConst("STYLING_LEVELS", tokens.levels ?? LEVELS_DEFAULT));
  lines.push(emitJsonConst("STYLING_CANVAS_FONTS", tokens.canvasFonts ?? {}));
  for (const group of paletteGroupNames(resolvedAppearances)) {
    const groupPalettes: Record<string, Record<string, number[]>> = {};
    for (const [appearanceName, groups] of Object.entries(resolvedAppearances)) {
      const paints = groups[group];
      if (!paints) {
        continue;
      }
      groupPalettes[appearanceName] = {};
      for (const [paintName, rgba] of Object.entries(paints)) {
        groupPalettes[appearanceName]![paintName] = [...rgba];
      }
    }
    lines.push(emitJsonConst(`STYLING_${group.toUpperCase()}_PALETTES`, groupPalettes));
    if (group === "board") {
      lines.push("export type StylingAppearanceName = keyof typeof STYLING_BOARD_PALETTES;");
    }
    lines.push("");
  }
  lines.push(emitJsonConst("STYLING_SEMIO_THEME", buildSemioUiTheme(tokens)));
  return lines.join("\n");
}

function emitCSharp(tokens: Tokens): string {
  const lines: string[] = ["// <auto-generated />", "// Generated from framework/ui/styling/🔣tokens.json — run `bun ./📜script.ts generate`.", "using System;", "", "namespace Elements.Styling;", "", "public static class Palette", "{"];
  for (const [k, v] of Object.entries(tokens.colors)) {
    lines.push(`  public const string ${toPascalCase(k)} = "${v}";`);
  }
  lines.push("}");
  lines.push("");
  //#region PaletteColor
  lines.push("public static class PaletteColor");
  lines.push("{");
  lines.push("  public static System.Drawing.Color FromHex(string hex)");
  lines.push("  {");
  lines.push("    if (string.IsNullOrWhiteSpace(hex))");
  lines.push("    {");
  lines.push("      return System.Drawing.Color.Transparent;");
  lines.push("    }");
  lines.push("    var h = hex.TrimStart('#');");
  lines.push("    try");
  lines.push("    {");
  lines.push("      return h.Length switch");
  lines.push("      {");
  lines.push("        3 => System.Drawing.Color.FromArgb(255, Convert.ToInt32(new string(h[0], 2), 16), Convert.ToInt32(new string(h[1], 2), 16), Convert.ToInt32(new string(h[2], 2), 16)),");
  lines.push("        6 => System.Drawing.Color.FromArgb(255, Convert.ToInt32(h.Substring(0, 2), 16), Convert.ToInt32(h.Substring(2, 2), 16), Convert.ToInt32(h.Substring(4, 2), 16)),");
  lines.push("        8 => System.Drawing.Color.FromArgb(Convert.ToInt32(h.Substring(0, 2), 16), Convert.ToInt32(h.Substring(2, 2), 16), Convert.ToInt32(h.Substring(4, 2), 16), Convert.ToInt32(h.Substring(6, 2), 16)),");
  lines.push("        _ => System.Drawing.Color.Transparent,");
  lines.push("      };");
  lines.push("    }");
  lines.push("    catch (FormatException)");
  lines.push("    {");
  lines.push("      return System.Drawing.Color.Transparent;");
  lines.push("    }");
  lines.push("  }");
  lines.push("");
  lines.push("  public static string ToHex(System.Drawing.Color color)");
  lines.push("  {");
  lines.push('    return $"#{color.A:X2}{color.R:X2}{color.G:X2}{color.B:X2}";');
  lines.push("  }");
  lines.push("");
  for (const k of Object.keys(tokens.colors)) {
    const name = toPascalCase(k);
    lines.push(`  public static System.Drawing.Color ${name} => FromHex(Palette.${name});`);
  }
  lines.push("}");
  lines.push("");
  //#endregion
  lines.push("public static class Strokes");
  lines.push("{");
  for (const [k, v] of Object.entries(tokens.strokes ?? {})) {
    if (Array.isArray(v)) {
      lines.push(`  public static readonly double[] ${toPascalCase(k)} = [${v.join(", ")}];`);
    } else {
      lines.push(`  public const double ${toPascalCase(k)} = ${v};`);
    }
  }
  lines.push("}");
  lines.push("");
  lines.push("public static class Radii");
  lines.push("{");
  for (const [k, v] of Object.entries(tokens.radii ?? {})) {
    lines.push(`  public const double ${toPascalCase(k)} = ${v};`);
  }
  lines.push("}");
  lines.push("");
  return lines.join("\n");
}

function emitRust(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
  const lines: string[] = ["// @emoji 🎨 Auto-generated from framework/ui/styling/🔣tokens.json — do not edit by hand.", ""];
  for (const [group, values] of Object.entries({ strokes: tokens.strokes, radii: tokens.radii, opacities: tokens.opacities })) {
    if (!values) {
      continue;
    }
    lines.push(`pub mod ${group} {`);
    for (const [k, v] of Object.entries(values)) {
      const name = toScreamingSnake(k);
      if (Array.isArray(v)) {
        lines.push(`    pub const ${name}: &[f64] = &[${v.map((x) => rustF64Lit(x)).join(", ")}];`);
      } else if (group === "opacities" && Number.isInteger(v) && v > 1) {
        lines.push(`    pub const ${name}: u8 = ${v};`);
      } else {
        lines.push(`    pub const ${name}: f64 = ${rustF64Lit(v)};`);
      }
    }
    lines.push("}");
    lines.push("");
  }
  lines.push("pub mod metrics {");
  for (const [section, values] of Object.entries(resolveMetrics(tokens.metrics))) {
    lines.push(`    pub mod ${section} {`);
    for (const [k, v] of Object.entries(values)) {
      const name = toScreamingSnake(k);
      if (Array.isArray(v)) {
        lines.push(`        pub const ${name}: &[f64] = &[${v.map((x) => rustF64Lit(x)).join(", ")}];`);
      } else if (section === "board" && k === "maxWorldClipTiles") {
        lines.push(`        pub const ${name}: u32 = ${v};`);
      } else if (section === "map" && (k === "labelMaxMin" || k === "labelMaxMax")) {
        lines.push(`        pub const ${name}: u32 = ${v};`);
      } else {
        lines.push(`        pub const ${name}: f64 = ${rustF64Lit(v)};`);
      }
    }
    lines.push("    }");
  }
  lines.push("}");
  lines.push("");
  lines.push("pub mod canvas_fonts {");
  for (const [k, v] of Object.entries(tokens.canvasFonts ?? {})) {
    lines.push(`    pub const ${toScreamingSnake(k)}: &str = ${JSON.stringify(v)};`);
  }
  lines.push("}");
  lines.push("");
  //#region 🌓Levels
  {
    const levels = tokens.levels ?? LEVELS_DEFAULT;
    lines.push("pub mod levels {");
    lines.push(`    pub const NAMES: &[&str] = &[${levels.names.map((n) => JSON.stringify(n)).join(", ")}];`);
    lines.push(`    pub const SHADE_STEP_PERCENT: f64 = ${rustF64Lit(levels.shadeStepPercent)};`);
    lines.push(`    pub const ELEMENT_STEP_PERCENT: f64 = ${rustF64Lit(levels.elementStepPercent)};`);
    lines.push(`    pub const HOVER_STEP_PERCENT: f64 = ${rustF64Lit(levels.hoverStepPercent)};`);
    lines.push(`    pub const GLASS_ALPHA_STEP: f64 = ${rustF64Lit(levels.glassAlphaStep)};`);
    lines.push(`    pub const GLASS_BLUR_STEP_PX: f64 = ${rustF64Lit(levels.glassBlurStepPx)};`);
    lines.push(`    pub const GLASS_SATURATE: f64 = ${rustF64Lit(levels.glassSaturate)};`);
    lines.push(`    pub const VEIL_ALPHA_EXTRA_STEPS: u32 = ${levels.veilAlphaExtraSteps};`);
    lines.push(`    pub const Z_STEP: f64 = ${rustF64Lit(levels.zStep)};`);
    lines.push("}");
    lines.push("");
  }
  //#endregion 🌓Levels
  for (const group of paletteGroupNames(resolvedAppearances)) {
    lines.push(`pub struct ${toPascalCase(group)}Palette {`);
    const sample = resolvedAppearances.light?.[group];
    if (sample) {
      for (const key of Object.keys(sample)) {
        lines.push(`    pub ${toSnakeCase(key)}: [f32; 4],`);
      }
    }
    lines.push("}");
    lines.push("");
    for (const appearanceName of APPEARANCE_NAMES) {
      const paints = resolvedAppearances[appearanceName]?.[group];
      if (!paints) {
        continue;
      }
      const constName = `${group.toUpperCase()}_${appearanceName.toUpperCase()}`;
      lines.push(`pub const ${constName}: ${toPascalCase(group)}Palette = ${toPascalCase(group)}Palette {`);
      for (const [key, rgba] of Object.entries(paints)) {
        const lin = rgba8ToLinear(rgba);
        lines.push(`    ${toSnakeCase(key)}: [${lin.map((x) => rustF32(x)).join(", ")}],`);
      }
      lines.push("};");
      lines.push("");
    }
  }
  return lines.join("\n");
}

function emitPython(tokens: Tokens, resolvedAppearances: ReturnType<typeof resolveAppearances>): string {
  const lines: string[] = [
    '"""@emoji 🎨 Auto-generated from framework/ui/styling/🔣tokens.json — do not edit by hand."""',
    "from __future__ import annotations",
    "from dataclasses import dataclass",
    "from typing import Final",
    "",
    "STYLING_TOKENS: Final[dict[str, str]] = {",
  ];
  for (const [k, v] of Object.entries(tokens.colors)) {
    lines.push(`    ${JSON.stringify(k)}: ${JSON.stringify(v)},`);
  }
  lines.push("}");
  lines.push("");
  lines.push(`STYLING_STROKES: Final[dict[str, float | list[float]]] = ${JSON.stringify(tokens.strokes ?? {}, null, 4)}`);
  lines.push(`STYLING_RADII: Final[dict[str, float]] = ${JSON.stringify(tokens.radii ?? {}, null, 4)}`);
  lines.push(`STYLING_OPACITIES: Final[dict[str, float]] = ${JSON.stringify(tokens.opacities ?? {}, null, 4)}`);
  lines.push(`STYLING_METRICS: Final[dict[str, dict[str, float | list[float]]]] = ${JSON.stringify(resolveMetrics(tokens.metrics), null, 4)}`);
  lines.push("");
  for (const group of paletteGroupNames(resolvedAppearances)) {
    lines.push("@dataclass(frozen=True, slots=True)");
    lines.push(`class ${toPascalCase(group)}Palette:`);
    const sample = resolvedAppearances.light?.[group];
    if (sample) {
      for (const key of Object.keys(sample)) {
        lines.push(`    ${toSnakeCase(key)}: tuple[int, int, int, int]`);
      }
    }
    lines.push("");
    for (const appearanceName of APPEARANCE_NAMES) {
      const paints = resolvedAppearances[appearanceName]?.[group];
      if (!paints) {
        continue;
      }
      const constName = `${group.toUpperCase()}_${appearanceName.toUpperCase()}`;
      const fields = Object.entries(paints)
        .map(([k, rgba]) => `${toSnakeCase(k)}=(${rgba.join(", ")})`)
        .join(", ");
      lines.push(`${constName}: Final[${toPascalCase(group)}Palette] = ${toPascalCase(group)}Palette(${fields})`);
    }
    lines.push("");
  }
  return lines.join("\n");
}

function googleFontsCssUrl(family: string): string {
  const query = family.trim().replaceAll(" ", "+");
  return `https://fonts.googleapis.com/css2?family=${query}:wght@400&display=swap`;
}

function parseGoogleFontWoff2Map(css: string): Map<string, string> {
  const map = new Map<string, string>();
  let subset: string | undefined;
  for (const line of css.split("\n")) {
    const comment = line.match(/^\s*\/\*\s*([^*]+?)\s*\*\/\s*$/);
    if (comment) {
      subset = comment[1]!.trim().toLowerCase();
      continue;
    }
    const urlMatch = line.match(/url\((https:[^)]+\.woff2)\)/);
    if (!urlMatch) {
      continue;
    }
    const url = urlMatch[1]!;
    if (subset) {
      map.set(subset, url);
      subset = undefined;
      continue;
    }
    const indexMatch = url.match(/\.(\d+)\.woff2/);
    if (indexMatch) {
      map.set(indexMatch[1]!, url);
    }
  }
  return map;
}

function resolveFontFaceUrl(src: string, woff2ByKey: Map<string, string>): string | undefined {
  const base =
    src
      .split("/")
      .pop()
      ?.replace(/\.woff2$/, "") ?? "";
  if (src.startsWith("font/noto-emoji/")) {
    if (base === "emoji-400") {
      return woff2ByKey.get("2") ?? woff2ByKey.get("0");
    }
    const index = base.replace(/-400$/, "");
    return woff2ByKey.get(index) ?? woff2ByKey.get("9");
  }
  return woff2ByKey.get(base);
}

/** @emoji ⬇️ Downloads token font woff2 files into `framework/ui/asset/font`. */
export async function fetchElementsFonts(): Promise<void> {
  const tokens = loadTokens();
  const cssByFamilyDir = new Map<string, Map<string, string>>();
  for (const [dir, family] of Object.entries(GOOGLE_FONT_QUERIES)) {
    const res = await fetch(googleFontsCssUrl(family), { headers: { "User-Agent": GOOGLE_FONTS_UA } });
    if (!res.ok) {
      throw new Error(`Google Fonts CSS failed for ${family}: ${res.status}`);
    }
    cssByFamilyDir.set(dir, parseGoogleFontWoff2Map(await res.text()));
  }
  let wrote = 0;
  for (const face of tokens.fontFaces) {
    const dirKey = Object.keys(GOOGLE_FONT_QUERIES).find((key) => face.src.startsWith(`${key}/`));
    if (!dirKey) {
      throw new Error(`No Google Fonts mapping for ${face.src}`);
    }
    const woff2ByKey = cssByFamilyDir.get(dirKey);
    if (!woff2ByKey?.size) {
      throw new Error(`No woff2 entries parsed for ${dirKey}`);
    }
    const remoteUrl = resolveFontFaceUrl(face.src, woff2ByKey);
    if (!remoteUrl) {
      throw new Error(`Could not resolve woff2 URL for ${face.src}`);
    }
    const dest = join(elementsAssetsRoot, face.src);
    mkdirSync(dirname(dest), { recursive: true });
    if (existsSync(dest)) {
      continue;
    }
    const fileRes = await fetch(remoteUrl);
    if (!fileRes.ok) {
      throw new Error(`Font download failed for ${face.src}: ${fileRes.status}`);
    }
    const bytes = new Uint8Array(await fileRes.arrayBuffer());
    if (bytes.length < 4 || bytes[0] !== 0x77 || bytes[1] !== 0x4f || bytes[2] !== 0x46 || bytes[3] !== 0x32) {
      throw new Error(`Downloaded bytes for ${face.src} are not woff2 (got ${bytes.length} bytes)`);
    }
    writeFileSync(dest, bytes);
    wrote += 1;
  }
  console.log(`framework/ui/styling: fonts ready under framework/ui/asset (${wrote} downloaded, ${tokens.fontFaces.length} total)`);
}

/** @emoji 🎨 Writes all styling artifacts from {@link 🔣tokens.json}. */
export function generateStylingArtifacts(): void {
  const tokens = loadTokens();
  const resolvedAppearances = resolveAppearances(tokens);
  mkdirSync(generatedCssDir, { recursive: true });
  mkdirSync(tsStylingDir, { recursive: true });
  mkdirSync(netPaletteDir, { recursive: true });
  mkdirSync(composeNetPaletteDir, { recursive: true });
  mkdirSync(dirname(pyGeneratedPath), { recursive: true });
  const fonts = emitPaletteFonts(tokens);
  const theme = emitPaletteTheme(tokens);
  const paletteCss = `${fonts}\n${theme}`;
  writeFileSync(join(generatedCssDir, "palette-fonts.css"), fonts, "utf8");
  writeFileSync(join(generatedCssDir, "palette-🎨theme.css"), theme, "utf8");
  writeFileSync(join(tsStylingDir, "🎨palette.css"), paletteCss, "utf8");
  writeFileSync(join(tsStylingDir, "🟦tokens.generated.ts"), emitTypeScriptTokens(tokens, resolvedAppearances), "utf8");
  const cs = emitCSharp(tokens);
  writeFileSync(join(netPaletteDir, "Palette.g.cs"), cs, "utf8");
  writeFileSync(join(composeNetPaletteDir, "Palette.g.cs"), cs, "utf8");
  writeFileSync(rustGeneratedPath, emitRust(tokens, resolvedAppearances), "utf8");
  writeFileSync(pyGeneratedPath, emitPython(tokens, resolvedAppearances), "utf8");
  validatePremadeThemes();
}

const premadeThemeDir = join(stylingRoot, "theme");

/** @emoji 🔎 Parses and resolves every premade `*.theme.json` so a broken preset fails `generate` instead of shipping. */
function validatePremadeThemes(): void {
  if (!existsSync(premadeThemeDir)) {
    return;
  }
  for (const entry of readdirSync(premadeThemeDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".theme.json")) {
      continue;
    }
    const path = join(premadeThemeDir, entry.name);
    const raw = readFileSync(path, "utf8");
    try {
      parseUiTheme(JSON.parse(raw));
    } catch (err) {
      throw new Error(`framework/ui/styling/theme/${entry.name} is invalid: ${err instanceof Error ? err.message : String(err)}`);
    }
  }
}

class GenerateScript extends BundleScript {
  run(): void {
    generateStylingArtifacts();
    console.log("framework/ui/styling: wrote generated CSS/TS/C#/Rust/Python styling artifacts");
  }
}

class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await fetchElementsFonts();
  }
}

/** 🧪 Runs the in-source `import.meta.vitest` coverage in `🟦vite-elements-assets.ts` (the generic
 * `tileProxyVitePlugin`/`staticDirVitePlugin`/`meshCollectionVitePlugin`/`playgroundAssetVitePlugins`
 * factories among others) — `framework/ui/styling/js/🧪index.test.ts`'s `bun:test` cases run separately via `bun test`. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "🧪vitest.config.ts");
  }
}

const PX_SCAN_ROOTS = ["framework/module/ui/js/react", "framework/module/ui/styling/js", "framework/product/os", "framework/product/os/module/dev", "compose/client/ui", "s/plugin/flow", "s/plugin/cad/renderer", "s/plugin/puzzle", "framework/os/kernel/infinite/world", "s/plugin/gis/2d"] as const;

const PX_SCAN_SKIP = ["/.🦑repo/", "/node_modules/", "/.storybook/", "/fixture/", "tokens.generated.", "session.json", ".plan.md"];

const PX_PATTERNS: { name: string; re: RegExp }[] = [{ name: "tailwind-arbitrary-px", re: /\[(?!9999px)[-0-9]*\.?[0-9]+px\]/ }];

function isPxScanExemptLine(line: string): boolean {
  if (line.includes("--stroke-hairline: 1px")) {
    return true;
  }
  if (line.includes("expect(") || line.includes("toContain(")) {
    return true;
  }
  if (line.includes("@media") && line.includes("px")) {
    return true;
  }
  if (/\b(h|w|min-h|min-w|max-h|max-w)-px\b/.test(line)) {
    return true;
  }
  if (line.includes("rounded-[9999px]")) {
    return true;
  }
  if (line.includes("cursor:") && line.includes("url(")) {
    return true;
  }
  if (/font=["'`]\d/.test(line) && line.includes("px")) {
    return true;
  }
  if (line.includes("transform:") && line.includes("px")) {
    return true;
  }
  if (line.includes("translate3d") || line.includes("translate(")) {
    return true;
  }
  if (line.includes("patchAutoAnimate") || line.includes("innerHTML")) {
    return true;
  }
  return false;
}

function shouldPxScanFile(path: string): boolean {
  if (!/\.(tsx?|css)$/.test(path)) {
    return false;
  }
  return !PX_SCAN_SKIP.some((skip) => path.includes(skip));
}

function collectPxViolations(repoRoot: string): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".repo") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRoot.length + 1);
      if (!shouldPxScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isPxScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of PX_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of PX_SCAN_ROOTS) {
    const abs = join(repoRoot, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}

/** @emoji 🚫 Fails when hardcoded `px` sizing literals remain in scanned source roots. */
class CheckNoPxScript extends BundleScript {
  run(): void {
    const violations = collectPxViolations(repoRoot);
    if (violations.length === 0) {
      console.log("framework/ui/styling: no hardcoded px sizing violations");
      return;
    }
    console.error(`framework/ui/styling: found ${violations.length} hardcoded px sizing violation(s):`);
    for (const v of violations.slice(0, 80)) {
      console.error(`  ${v.file}:${v.line} [${v.kind}] ${v.text}`);
    }
    if (violations.length > 80) {
      console.error(`  … and ${violations.length - 80} more`);
    }
    process.exit(1);
  }
}

/** @emoji 🗺️ Color scan walks its own root set: same as {@link PX_SCAN_ROOTS} minus `framework/ui/styling` (the source of truth
 * for tokens), plus `.storybook` and the whole `compose/client` tree (superseding the narrower `compose/client/ui` root
 * so it isn't walked twice). */
const COLOR_SCAN_ROOTS = [...PX_SCAN_ROOTS.filter((root) => !root.startsWith("framework/module/ui/styling") && root !== "compose/client/ui"), ".storybook", "compose/client"] as const;

/** @emoji 📋 Files with pre-existing hardcoded-color usage surfaced by the full-palette/manual-dark-variant patterns
 * and the widened scan roots — tracked for follow-up migration, not fixed here. */
const COLOR_SCAN_LEGACY_ALLOWLIST = [
  ".storybook/compose/algorithm/kit-store/index.tsx",
  ".storybook/framework/os/index.tsx",
  ".storybook/preview.tsx",
  ".storybook/stories/compose/algorithm/KitStore.stories.tsx",
  ".storybook/stories/ui/Avatar.stories.tsx",
  ".storybook/stories/ui/Footer.stories.tsx",
  ".storybook/stories/ui/Icons.stories.tsx",
  ".storybook/stories/ui/OntologyTree.stories.tsx",
  ".storybook/stories/ui/Tree.stories.tsx",
  ".storybook/stories/ui/ValidationTree.stories.tsx",
  "cad/renderer/js/index.tsx",
  "framework/product/os/rs/lib.rs",
  "framework/product/os/module/infinite/world/r3f/index.tsx",
  "framework/product/os/module/infinite/world/rs/lib.rs",
  "puzzle/2d/rs/lib.rs",
  "puzzle/plugin/rs/lib.rs",
];

const COLOR_SCAN_SKIP = [
  ...PX_SCAN_SKIP.filter((skip) => skip !== "/.storybook/"),
  "/dist/",
  "/.vite/",
  "/.stage/",
  "/renderer-modules/",
  "/plugin-modules/",
  "generated/",
  "🤖generated.rs",
  "🤖generated.py",
  "Palette.g.cs",
  "🎨palette.css",
  ...COLOR_SCAN_LEGACY_ALLOWLIST,
];

const COLOR_PATTERNS: { name: string; re: RegExp }[] = [
  { name: "raw-hex-color", re: /#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?(?:[0-9a-fA-F]{2})?\b/ },
  { name: "raw-rgb-hsl-color", re: /\b(?:rgba?|hsla?)\(\s*[\d.]/ },
  {
    name: "tailwind-palette-color-class",
    re: /\b(?:bg|text|border|ring|fill|stroke|from|via|to|divide|outline|decoration|caret|accent|shadow)-(?:red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose|zinc|gray|slate|neutral|stone)-\d{2,3}\b/,
  },
  { name: "manual-dark-variant-palette", re: /\bdark:(?:bg|text|border|ring|fill|stroke)-/ },
];

function isColorScanExemptLine(line: string): boolean {
  if (line.includes("expect(") || line.includes("toContain(") || line.includes("toBe(")) {
    return true;
  }
  if (/^\s*(\/\/|\*|\/\*)/.test(line)) {
    return true;
  }
  return false;
}

function shouldColorScanFile(path: string): boolean {
  if (!/\.(tsx?|css|rs)$/.test(path)) {
    return false;
  }
  return !COLOR_SCAN_SKIP.some((skip) => path.includes(skip));
}

function collectColorViolations(repoRootPath: string): { file: string; line: number; kind: string; text: string }[] {
  const violations: { file: string; line: number; kind: string; text: string }[] = [];

  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "node_modules" || entry.name === ".repo") {
          continue;
        }
        walk(full);
        continue;
      }
      const rel = full.slice(repoRootPath.length + 1);
      if (!shouldColorScanFile(rel)) {
        continue;
      }
      const lines = readFileSync(full, "utf8").split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i]!;
        if (isColorScanExemptLine(line)) {
          continue;
        }
        for (const { name, re } of COLOR_PATTERNS) {
          if (re.test(line)) {
            violations.push({ file: rel, line: i + 1, kind: name, text: line.trim() });
            break;
          }
        }
      }
    }
  };

  for (const root of COLOR_SCAN_ROOTS) {
    const abs = join(repoRootPath, root);
    if (existsSync(abs)) {
      walk(abs);
    }
  }
  return violations;
}

/** @emoji 🚫 Fails when hardcoded hex/rgb/hsl colors, full-palette Tailwind color classes, or manual `dark:`-variant color utilities remain outside `framework/ui/styling` (design tokens/appearances are the single source of color truth). */
class CheckNoRawColorsScript extends BundleScript {
  run(): void {
    const violations = collectColorViolations(repoRoot);
    if (violations.length === 0) {
      console.log("framework/ui/styling: no hardcoded color violations");
      return;
    }
    console.error(`framework/ui/styling: found ${violations.length} hardcoded color violation(s):`);
    for (const v of violations.slice(0, 80)) {
      console.error(`  ${v.file}:${v.line} [${v.kind}] ${v.text}`);
    }
    if (violations.length > 80) {
      console.error(`  … and ${violations.length - 80} more`);
    }
    process.exit(1);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir)
    .register("generate", GenerateScript)
    .register("fonts", FontsScript)
    .register("test", TestScript)
    .register("check-no-px", CheckNoPxScript)
    .register("check-no-raw-colors", CheckNoRawColorsScript);
  await runBundleScriptMain(router, import.meta.url);
}
