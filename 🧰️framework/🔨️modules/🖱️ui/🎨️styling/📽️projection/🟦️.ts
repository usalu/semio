import { existsSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { oklabMix, rgba8ToLinear, linearToOklab, type Rgba8 } from "../🌗️mixing/🟦️.ts";
import { parseUiTheme, resolveThemeMetrics, resolveThemePaint, type ThemePaintRef, type UiTheme } from "../📦️packages/🟦️typescript/🟦️.ts";
import { SEMIO_ASSET_ROUTE } from "../../../🖼️assets/🔍️resolver/🌐️delivery/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts";
import { loadFontCatalog, resolveFontSource } from "../🔤️fonts/🟦️.ts";

const moduleRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));
const stylingOwnerRoot = resolve(moduleRoot, "..");
const tokensPath = join(stylingOwnerRoot, "🔣️.json");
const generatedCssDir = join(stylingOwnerRoot, "🤖️generated");
const paletteDir = join(stylingOwnerRoot, "🎨️palette");
const tokenDir = join(stylingOwnerRoot, "🔤️tokens");
const netGeneratedPath = join(paletteDir, "🔷️.cs");
const pyGeneratedPath = join(tokenDir, "🐍️.py");
const rustGeneratedPath = join(tokenDir, "🦀️.rs");
const obsoleteGeneratedRoots = [join(stylingOwnerRoot, "🔷️net"), join(stylingOwnerRoot, "📦️packages", "🐍️python", "🎨️styling", "🔤️tokens")];
const adaptersManifestPath = join(stylingOwnerRoot, "🛂️adapters.manifest.json");

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
  presence?: StylingPresence;
  appearances?: Record<string, Record<string, Record<string, PaintRef>>>;
}

/** @emoji 👥️ The 12-hue session-color wheel (contract freeze §C7.5) driving `--presence-0..11` CSS vars,
 * `presence::HUES/LIGHT/DARK` (Rust), and `STYLING_PRESENCE_PALETTES` (TS) — the pure twins in
 * `👥️PresenceBar/{🧊️component.rs,🟦️.tsx}` derive every peer's color from these three numbers
 * plus the hub-assigned palette index. */
interface StylingPresence {
  hues: readonly number[];
  light: { s: number; l: number };
  dark: { s: number; l: number };
}

/** @emoji 🌓️ Knobs driving the formula-derived 6-level UI surface system (`base..menu`); see contract at
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`. */
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

/** @emoji 📏️ Derives dag component width as twice the IO channel column width. */
function resolveMetrics(metrics: Tokens["metrics"]): NonNullable<Tokens["metrics"]> {
  return resolveThemeMetrics(metrics ?? {}) as NonNullable<Tokens["metrics"]>;
}

function resolvePaint(colors: Record<string, string>, ref: PaintRef): Rgba8 {
  return resolveThemePaint(colors, ref);
}

//#region 🌓️Levels

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

/** @emoji 👥️ Fallback presence palette (contract freeze §C7.5) — used when a `*.theme.json` predates the
 * `presence` block; `🔣️.json` and every premade theme carry their own copy of these same values. */
const PRESENCE_DEFAULT: StylingPresence = {
  hues: [0, 210, 120, 30, 270, 180, 330, 60, 240, 150, 300, 90],
  light: { s: 0.68, l: 0.32 },
  dark: { s: 0.72, l: 0.62 },
};

/** @emoji 🌓️ Injects the 6 formula-derived `level<Name>` background paints and `element<Name>` element paints
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
//#endregion 🌓️Levels

function rustF32(x: number): string {
  return `${x.toFixed(8).replace(/\.?0+$/, "")}_f32`;
}

function rustF64Lit(v: number): string {
  return Number.isInteger(v) ? `${v}.0` : String(v);
}

/** @emoji 🎨️ Resolves every appearance's paint refs to Rgba8, then injects the formula-derived level/element
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

//#region 🧪️LevelsTests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../🧪️tests/🧪️levels-oklabmix/🟦️.ts");
  await registerTests1(import.meta.vitest, { LEVELS_DEFAULT, join, linearToOklab, loadTokens, oklabMix, resolveAppearances, rgba8ToLinear, toPascalCase }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️LevelsTests

function paletteGroupNames(resolvedAppearances: ReturnType<typeof resolveAppearances>): string[] {
  return Object.keys(resolvedAppearances.light ?? {}).sort();
}

function emitPaletteFonts(tokens: Tokens): string {
  const assetBase = SEMIO_ASSET_ROUTE;
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣️.json — run `bun ./📜️script.ts generate`. */"];
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
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣️.json — run `bun ./📜️script.ts generate`. */", "@theme {", "  /* Primary brand colors */"];
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

/** @emoji 🔢️ Renders an `[0, 1]` fraction as a clean percent literal (`0.68` → `"68"`), avoiding float
 * noise in generated CSS. */
function pct(fraction: number): string {
  return String(Math.round(fraction * 10000) / 100);
}

/** @emoji 👥️ Emits `--presence-0..11` under `:root` (light) and `.dark`, plus their `--color-presence-N`
 * `@theme` aliases (contract freeze §C7.5) — the CSS half of the presence palette; `emitRust`/
 * `emitTypeScriptTokens` emit the pure-twin half consumed by `👥️PresenceBar`. Only the base cycle
 * (`index % 12`, i.e. `k = index / 12 === 0`) gets a CSS var; higher cycles render inline HSL via the
 * `presence_color`/`presenceColor` twins. */
function emitPalettePresence(tokens: Tokens): string {
  const presence = tokens.presence ?? PRESENCE_DEFAULT;
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣️.json — run `bun ./📜️script.ts generate`. */", ":root {"];
  presence.hues.forEach((h, i) => {
    lines.push(`  --presence-${i}: hsl(${h}deg ${pct(presence.light.s)}% ${pct(presence.light.l)}%);`);
  });
  lines.push("}", "", ".dark {");
  presence.hues.forEach((h, i) => {
    lines.push(`  --presence-${i}: hsl(${h}deg ${pct(presence.dark.s)}% ${pct(presence.dark.l)}%);`);
  });
  lines.push("}", "", "@theme {");
  presence.hues.forEach((_, i) => {
    lines.push(`  --color-presence-${i}: var(--presence-${i});`);
  });
  lines.push("}");
  lines.push("");
  return lines.join("\n");
}

function emitJsonConst(name: string, value: unknown, indent = ""): string {
  return `${indent}export const ${name} = ${JSON.stringify(value, null, 2).replaceAll("\n", `\n${indent}`)} as const;\n`;
}

/** @emoji 🎨️ Builds the default "semio" `UiTheme` verbatim from 🔣️.json (the paint refs stay unresolved). */
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
  const lines: string[] = ["/* Generated from framework/ui/styling/🔣️.json — run `bun ./📜️script.ts generate`. */", ""];
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
  lines.push(emitJsonConst("STYLING_PRESENCE_PALETTES", tokens.presence ?? PRESENCE_DEFAULT));
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
  const lines: string[] = ["// <auto-generated />", "// Generated from framework/ui/styling/🔣️.json — run `bun ./📜️script.ts generate`.", "using System;", "", "namespace Semio.Framework.Ui.Styling;", "", "public static class Palette", "{"];
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
  const lines: string[] = ["// @emoji 🎨️ Auto-generated from framework/ui/styling/🔣️.json — do not edit by hand.", ""];
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
  //#region 🌓️Levels
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
  //#endregion 🌓️Levels
  //#region 👥️Presence
  {
    const presence = tokens.presence ?? PRESENCE_DEFAULT;
    lines.push("pub mod presence {");
    lines.push(`    pub const HUES: [u16; 12] = [${presence.hues.map((h) => `${h}`).join(", ")}];`);
    lines.push(`    pub const LIGHT: (f64, f64) = (${rustF64Lit(presence.light.s)}, ${rustF64Lit(presence.light.l)});`);
    lines.push(`    pub const DARK: (f64, f64) = (${rustF64Lit(presence.dark.s)}, ${rustF64Lit(presence.dark.l)});`);
    lines.push("}");
    lines.push("");
  }
  //#endregion 👥️Presence
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
    '"""@emoji 🎨️ Auto-generated from framework/ui/styling/🔣️.json — do not edit by hand."""',
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

type StylingArtifact = { path: string; content: string };
type StylingAdapterManifest = { tokens: string; adapters: readonly { outputs: readonly string[] }[] };

/** @emoji 🧾️ Renders the complete cross-language styling output manifest without writing files. */
export function renderStylingArtifacts(): readonly StylingArtifact[] {
  const tokens = loadTokens();
  const fontsCatalog = loadFontCatalog();
  for (const face of tokens.fontFaces) resolveFontSource(face.src, fontsCatalog);
  const resolvedAppearances = resolveAppearances(tokens);
  const fonts = emitPaletteFonts(tokens);
  const theme = emitPaletteTheme(tokens);
  const presenceCss = emitPalettePresence(tokens);
  const paletteCss = `${fonts}\n${theme}\n${presenceCss}`;
  return [
    { path: join(generatedCssDir, "🔤️palette-fonts", "🎨️.css"), content: fonts },
    { path: join(generatedCssDir, "🌓️palette-theme", "🎨️.css"), content: theme },
    { path: join(generatedCssDir, "🚦️palette-presence", "🎨️.css"), content: presenceCss },
    { path: join(paletteDir, "🎨️.css"), content: paletteCss },
    { path: join(generatedCssDir, "🔤️tokens", "🟦️.ts"), content: emitTypeScriptTokens(tokens, resolvedAppearances) },
    { path: netGeneratedPath, content: emitCSharp(tokens) },
    { path: rustGeneratedPath, content: emitRust(tokens, resolvedAppearances) },
    { path: pyGeneratedPath, content: emitPython(tokens, resolvedAppearances) },
  ];
}

/** @emoji 📋️ Proves the adapter manifest declares exactly the artifacts emitted by the shared renderer. */
function validateStylingOutputManifest(artifacts: readonly StylingArtifact[]): void {
  const manifest = JSON.parse(readFileSync(adaptersManifestPath, "utf8")) as StylingAdapterManifest;
  if (manifest.tokens !== "🔣️.json") throw new Error(`styling adapter manifest tokens must be 🔣️.json, got ${JSON.stringify(manifest.tokens)}`);
  const declared = manifest.adapters.flatMap((adapter) => adapter.outputs).sort();
  const rendered = artifacts.map((artifact) => relative(stylingOwnerRoot, artifact.path)).sort();
  if (new Set(declared).size !== declared.length) throw new Error("styling adapter manifest contains duplicate outputs");
  if (JSON.stringify(declared) !== JSON.stringify(rendered)) throw new Error(`styling adapter manifest mismatch:\ndeclared=${JSON.stringify(declared)}\nrendered=${JSON.stringify(rendered)}`);
}

/** @emoji 🎨️ Writes all styling artifacts from the same byte plan used by the freshness check. */
export function generateStylingArtifacts(): void {
  const artifacts = renderStylingArtifacts();
  validateStylingOutputManifest(artifacts);
  const ownedDirectories = new Set([generatedCssDir, paletteDir, tokenDir, ...artifacts.map((artifact) => dirname(artifact.path)).filter((path) => path !== stylingOwnerRoot)]);
  for (const root of ownedDirectories) {
    if (!existsSync(root)) continue;
    const expected = new Set(artifacts.filter((artifact) => artifact.path.startsWith(`${root}${sep}`)).map((artifact) => join(root, relative(root, artifact.path).split(sep)[0]!)));
    for (const name of readdirSync(root)) if (!expected.has(join(root, name))) rmSync(join(root, name), { recursive: true, force: true });
  }
  for (const artifact of artifacts) {
    writeGeneratedFileIfChanged(artifact.path, artifact.content);
  }
  for (const root of obsoleteGeneratedRoots) rmSync(root, { recursive: true, force: true });
  validatePremadeThemes();
}

/** 🧾️ Emits the canonical multi-language styling byte plan and stale removals without writes. */
export function previewStylingArtifacts(repoRoot: string): string {
    const artifacts = renderStylingArtifacts();
    validateStylingOutputManifest(artifacts);
    const directoryRoots = [...new Set([generatedCssDir, paletteDir, tokenDir, ...artifacts.map((artifact) => dirname(artifact.path)).filter((path) => path !== stylingOwnerRoot)])];
    const nodes = [
      ...directoryRoots.map((path) => ({ bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: relative(repoRoot, path).replaceAll("\\", "/").normalize("NFC") })),
      ...artifacts.map((artifact) => ({ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, artifact.path).replaceAll("\\", "/").normalize("NFC") })),
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const staleRemovals = [...directoryRoots.flatMap((root) => {
      if (!existsSync(root)) return [];
      const expected = new Set(artifacts.filter((artifact) => artifact.path.startsWith(`${root}${sep}`)).map((artifact) => join(root, relative(root, artifact.path).split(sep)[0]!)));
      return readdirSync(root).filter((name) => !expected.has(join(root, name))).map((name) => relative(repoRoot, join(root, name)).replaceAll("\\", "/").normalize("NFC"));
    }), ...obsoleteGeneratedRoots.filter(existsSync).map((path) => relative(repoRoot, path).replaceAll("\\", "/").normalize("NFC"))].sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  return `${JSON.stringify({ contractId: "styling-tokens", nodes, schemaVersion: 1, staleRemovals })}\n`;
}

/** @emoji 🧪️ Fails closed when a declared styling artifact is missing or byte-stale. */
export function checkStylingArtifacts(): void {
  const artifacts = renderStylingArtifacts();
  validateStylingOutputManifest(artifacts);
  validatePremadeThemes();
  const stale = artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => relative(stylingOwnerRoot, artifact.path));
  if (stale.length > 0) throw new Error(`framework/ui/styling generated artifacts are stale:\n${stale.map((path) => `  ${path}`).join("\n")}`);
}

const premadeThemeDir = join(stylingOwnerRoot, "🌓️theme");

/** @emoji 🔎️ Parses every JSON preset owned by the theme directory before generation succeeds. */
function validatePremadeThemes(): void {
  if (!existsSync(premadeThemeDir)) {
    return;
  }
  for (const entry of readdirSync(premadeThemeDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".json")) {
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
