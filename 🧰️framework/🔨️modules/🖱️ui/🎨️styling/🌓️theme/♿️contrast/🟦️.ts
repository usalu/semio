import type { Rgba8 } from "../🏛️model/🟦️.ts";
import type { StylingTokenKey } from "../../🤖️generated/🔤️tokens/🟦️.ts";
import { _readableForegroundCache, hexChannel, normalizeHex, resolveBackgroundColorHex, tokenHex } from "../🌈️color/🟦️.ts";

function srgbChannelToLinear(channel: number): number {
  const s = channel / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
}

/** 🌓️ WCAG relative luminance for a resolved `#rrggbb` color (0 = black, 1 = white). */
export function relativeLuminance(hex: string): number {
  const norm = normalizeHex(hex);
  const r = srgbChannelToLinear(hexChannel(norm, 1));
  const g = srgbChannelToLinear(hexChannel(norm, 3));
  const b = srgbChannelToLinear(hexChannel(norm, 5));
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** 🌓️ WCAG 2.2 minimum contrast for normal body text (1.4.3 Contrast Minimum, level AA). */
export const WCAG_AA_CONTRAST = 4.5;

/** 🌓️ WCAG 2.2 minimum contrast for large text (≥18.66 px bold or ≥24 px) and for non-text UI
 * components and graphical objects (1.4.11), level AA. */
export const WCAG_AA_LARGE_CONTRAST = 3;

/** 🌓️ WCAG 2.2 enhanced contrast for normal body text (1.4.6, level AAA). */
export const WCAG_AAA_CONTRAST = 7;

/** 🌓️ How a measured contrast ratio grades against WCAG 2.2 — `"fail"` is below the large-text/UI
 * floor, so it is unreadable at ANY size. */
export type WcagContrastGrade = "aaa" | "aa" | "aaLarge" | "fail";

/**
 * 🌓️ The WCAG 2.2 contrast ratio between two resolved `#rrggbb` colors: `(L₁ + 0.05) / (L₂ + 0.05)`
 * with `L₁` the lighter relative luminance. Always ≥ 1 and ≤ 21, and symmetric in its arguments.
 *
 * @see {@link https://www.w3.org/TR/WCAG22/#dfn-contrast-ratio | WCAG 2.2 — contrast ratio}
 */
export function contrastRatio(a: string, b: string): number {
  const first = relativeLuminance(a);
  const second = relativeLuminance(b);
  const lighter = Math.max(first, second);
  const darker = Math.min(first, second);
  return (lighter + 0.05) / (darker + 0.05);
}

/** 🌓️ {@link contrastRatio} for two resolved RGBA8888 paints, ignoring alpha (a ratio is only defined
 * for the colors that actually land on the pixel; a translucent paint's real contrast depends on what is
 * behind it, which a token table cannot know). */
export function contrastRatioRgba(a: Rgba8, b: Rgba8): number {
  const hex = ([r, g, blue]: Rgba8) => `#${[r, g, blue].map((channel) => Math.max(0, Math.min(255, Math.round(channel))).toString(16).padStart(2, "0")).join("")}`;
  return contrastRatio(hex(a), hex(b));
}

/** 🌓️ Grades a ratio against the three WCAG thresholds. The bands are inclusive at each floor, matching
 * the spec's "at least" wording. */
export function wcagContrastGrade(ratio: number): WcagContrastGrade {
  if (!Number.isFinite(ratio)) return "fail";
  if (ratio >= WCAG_AAA_CONTRAST) return "aaa";
  if (ratio >= WCAG_AA_CONTRAST) return "aa";
  if (ratio >= WCAG_AA_LARGE_CONTRAST) return "aaLarge";
  return "fail";
}

/** 🌓️ One live contrast verdict for a theme-editor row: the ratio a user's custom paint reaches against
 * its own appearance foreground, rounded to the 0.01 the UI prints, and whether it clears body-text AA.
 * `🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts` asserts the GENERATED palette clears the same bar; this is
 * the same law applied live to a palette the generator never saw. */
export function themePaintContrast(paint: Rgba8, foreground: Rgba8): { readonly ratio: number; readonly grade: WcagContrastGrade; readonly passesBodyText: boolean } {
  const ratio = Math.round(contrastRatioRgba(paint, foreground) * 100) / 100;
  return { ratio, grade: wcagContrastGrade(ratio), passesBodyText: ratio >= WCAG_AA_CONTRAST };
}

/** 🌓️ One text-on-surface pair of the `chrome` appearance palette that WCAG 2.2 1.4.3 governs. */
export type ThemeContrastPair = { readonly text: string; readonly surface: string };

/**
 * 🌓️ The `chrome` palette's text-on-surface pairs — the ONLY pairs a contrast verdict is meaningful for.
 * Measuring every paint against `foreground` (the first cut of this check) compared text with text
 * (`activeForeground` vs `foreground` = 1:1) and borders with text, so an untouched theme raised warnings
 * that meant nothing. Borders are non-text decoration and are not paired. `foreground` rides every neutral
 * surface (the `level*` ladder `🧪️levels-oklabmix` gates is the same law), each `*Foreground` rides its own
 * surface.
 */
export const THEME_CHROME_CONTRAST_PAIRS: readonly ThemeContrastPair[] = [
  { text: "foreground", surface: "base" },
  { text: "foreground", surface: "panel" },
  { text: "foreground", surface: "muted" },
  { text: "foreground", surface: "hoverInteractiveFill" },
  { text: "mutedForeground", surface: "base" },
  { text: "mutedForeground", surface: "panel" },
  { text: "accentForeground", surface: "accent" },
  { text: "activeForeground", surface: "activeBase" },
  { text: "activeForeground", surface: "activeHover" },
];

/** 🌓️ Every {@link THEME_CHROME_CONTRAST_PAIRS} pair `paintKey` takes part in (as text OR surface), measured
 * on `palette`, worst ratio first — so editing EITHER side of a pair surfaces the verdict on the row being
 * edited. Pairs whose other paint the palette lacks are skipped; a paint in no pair answers `[]`. */
export function themePaintContrastPairs(palette: Readonly<Record<string, Rgba8>>, paintKey: string): readonly (ThemeContrastPair & { readonly counterpart: string; readonly ratio: number; readonly grade: WcagContrastGrade; readonly passesBodyText: boolean })[] {
  return THEME_CHROME_CONTRAST_PAIRS.filter((pair) => pair.text === paintKey || pair.surface === paintKey)
    .flatMap((pair) => {
      const text = palette[pair.text];
      const surface = palette[pair.surface];
      if (!text || !surface) return [];
      return [{ ...pair, counterpart: pair.text === paintKey ? pair.surface : pair.text, ...themePaintContrast(text, surface) }];
    })
    .sort((a, b) => a.ratio - b.ratio);
}

/** 🏷️ Picks a readable palette foreground hex for text on the given background color expression. */
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
