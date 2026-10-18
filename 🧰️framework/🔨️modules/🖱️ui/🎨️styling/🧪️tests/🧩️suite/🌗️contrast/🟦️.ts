import { describe, it, expect } from "bun:test";
import { contrastRatio, WCAG_AA_LARGE_CONTRAST, WCAG_AA_CONTRAST, WCAG_AAA_CONTRAST, wcagContrastGrade, type Rgba8, contrastRatioRgba, themePaintContrast, type WcagContrastGrade, themePaintContrastPairs, THEME_CHROME_CONTRAST_PAIRS, type UiTheme, semioTheme, resolveThemeAppearancePalettes } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { contrastRatio01, hexToRgb01 } from "../🧮️color-math/🟦️.ts";

/** ♿️ The theme editor prints a LIVE WCAG verdict for every user-customized appearance paint
 * (`📌️ChromePanels/🟦️.tsx`'s `themeContrastBadgeText`). The generated default palette is gated ≥ AA by
 * `🧪️levels-oklabmix/🟦️.ts`; an arbitrary custom theme has no such gate, which is exactly the hole this
 * closes. `contrastRatio01` above is this suite's OWN independent implementation (0-1 float pipeline,
 * written for the palette laws long before these exports existed) and is used here as the oracle. */
describe("custom theme contrast verdict", () => {
  const cases: readonly (readonly [string, string])[] = [
    ["#ffffff", "#000000"],
    ["#000000", "#ffffff"],
    ["#767676", "#ffffff"],
    ["#7b827d", "#101010"],
    ["#1f2a44", "#c8d2e0"],
    ["#c8c8c8", "#ffffff"],
    ["#123456", "#123456"],
  ];

  it("agrees with this suite's own independent 0-1 luminance pipeline on every pair", () => {
    for (const [a, b] of cases) expect(contrastRatio(a, b)).toBeCloseTo(contrastRatio01(hexToRgb01(a), hexToRgb01(b)), 10);
  });

  it("is symmetric and bounded by the spec's 1:1 and 21:1 extremes", () => {
    for (const [a, b] of cases) {
      expect(contrastRatio(a, b)).toBeCloseTo(contrastRatio(b, a), 12);
      expect(contrastRatio(a, b)).toBeGreaterThanOrEqual(1);
      expect(contrastRatio(a, b)).toBeLessThanOrEqual(21);
    }
    expect(contrastRatio("#ffffff", "#000000")).toBeCloseTo(21, 6);
    expect(contrastRatio("#123456", "#123456")).toBeCloseTo(1, 10);
  });

  it("grades at the WCAG 2.2 thresholds, inclusive at each floor", () => {
    expect([WCAG_AA_LARGE_CONTRAST, WCAG_AA_CONTRAST, WCAG_AAA_CONTRAST]).toEqual([3, 4.5, 7]);
    expect(wcagContrastGrade(WCAG_AAA_CONTRAST)).toBe("aaa");
    expect(wcagContrastGrade(WCAG_AAA_CONTRAST - 0.01)).toBe("aa");
    expect(wcagContrastGrade(WCAG_AA_CONTRAST)).toBe("aa");
    expect(wcagContrastGrade(WCAG_AA_CONTRAST - 0.01)).toBe("aaLarge");
    expect(wcagContrastGrade(WCAG_AA_LARGE_CONTRAST)).toBe("aaLarge");
    expect(wcagContrastGrade(WCAG_AA_LARGE_CONTRAST - 0.01)).toBe("fail");
    expect(wcagContrastGrade(Number.NaN)).toBe("fail");
  });

  it("themePaintContrast ignores alpha, rounds to the 0.01 the badge prints, and flags sub-AA pairs", () => {
    const opaque: Rgba8 = [255, 255, 255, 255];
    const translucent: Rgba8 = [255, 255, 255, 3];
    const ink: Rgba8 = [0, 0, 0, 255];
    expect(contrastRatioRgba(translucent, ink)).toBeCloseTo(contrastRatioRgba(opaque, ink), 12);
    const good = themePaintContrast(opaque, ink);
    expect(good.ratio).toBe(21);
    expect(good.grade).toBe("aaa");
    expect(good.passesBodyText).toBe(true);
    const bad = themePaintContrast([200, 200, 200, 255], opaque);
    expect(bad.passesBodyText).toBe(false);
    expect(bad.grade).toBe("fail");
    expect(Number.isInteger(Math.round(bad.ratio * 100) - bad.ratio * 100)).toBe(true);
  });
});

/** ♿️ The text-on-surface pair law, replayed from `🧪️tests/🧫️fixtures/♿️chrome-contrast-pairs.json` — whose ratios
 * were computed by an independent Python WCAG implementation, so this is a cross-language oracle too. */
describe("chrome text-on-surface contrast pairs", () => {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/♿️chrome-contrast-pairs.json"), "utf8")) as {
    readonly palette: Record<string, Rgba8>;
    readonly cases: Record<string, { readonly text: string; readonly surface: string; readonly counterpart: string; readonly ratio: number; readonly grade: WcagContrastGrade } | null>;
  };

  it("names the worst pair every paint takes part in, and no pair for a border", () => {
    for (const [paintKey, expected] of Object.entries(fixture.cases)) {
      const worst = themePaintContrastPairs(fixture.palette, paintKey)[0] ?? null;
      if (expected === null) {
        expect(worst).toBeNull();
        continue;
      }
      expect(worst).not.toBeNull();
      expect({ text: worst!.text, surface: worst!.surface, counterpart: worst!.counterpart, grade: worst!.grade }).toEqual({ text: expected.text, surface: expected.surface, counterpart: expected.counterpart, grade: expected.grade });
      expect(worst!.ratio).toBeCloseTo(expected.ratio, 2);
    }
  });

  it("never compares a text paint with another text paint, nor a border with anything", () => {
    for (const pair of THEME_CHROME_CONTRAST_PAIRS) {
      expect(pair.text === "foreground" || pair.text.endsWith("Foreground")).toBe(true);
      expect(pair.surface.endsWith("Foreground") || pair.surface === "foreground" || pair.surface.startsWith("border")).toBe(false);
    }
  });

  /** ♿️ Every SHIPPED theme — the default `semio` theme built from `🎨️styling/🔣️.json` and the `mono` preset
   * `🌓️theme/🔣️.json` — clears WCAG 2.2 AA (4.5:1) on every chrome text/surface pair in BOTH appearances, measured
   * by this module and, as the independent oracle, by the third-party `color` package's WCAG contrast.
   * U4 measured the untouched light default failing `mutedForeground` on `panel` (2.34) and `base` (3.54) and
   * `activeForeground` on `activeHover` (4.45); the dark appearance failed five pairs (ticket 26/09/23 S15). */
  it("every shipped theme passes AA on every chrome text/surface pair, both appearances, against a third-party oracle", async () => {
    const thirdPartyColorSpecifier = "color";
    const { default: Color } = (await import(thirdPartyColorSpecifier)) as { default: (value: string) => { contrast(other: unknown): number } };
    const mono = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/🔣️.json"), "utf8")) as UiTheme;
    const hex = (paint: Rgba8) => `#${paint.slice(0, 3).map((channel) => channel.toString(16).padStart(2, "0")).join("")}`;
    const failures: string[] = [];
    for (const [name, theme] of [["semio", semioTheme()], ["mono", mono]] as const) {
      for (const appearance of ["light", "dark"] as const) {
        const chrome = resolveThemeAppearancePalettes(theme, appearance).chrome;
        for (const pair of THEME_CHROME_CONTRAST_PAIRS) {
          const ratio = contrastRatioRgba(chrome[pair.text]!, chrome[pair.surface]!);
          const oracle = Color(hex(chrome[pair.text]!)).contrast(Color(hex(chrome[pair.surface]!)));
          expect(ratio, `${name} ${appearance} ${pair.text}/${pair.surface} oracle`).toBeCloseTo(oracle, 2);
          if (ratio < WCAG_AA_CONTRAST) failures.push(`${name} ${appearance} ${pair.text}/${pair.surface}=${ratio.toFixed(2)}`);
        }
      }
    }
    expect(failures).toEqual([]);
  });

  it("the shipped default chrome palette is measured on the same pairs (surfaces the ladder gates pass AA with foreground)", () => {
    const theme = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../🌓️theme/🔣️.json"), "utf8"));
    expect(Object.keys(theme.appearances.light.chrome)).toEqual(expect.arrayContaining([...new Set(THEME_CHROME_CONTRAST_PAIRS.flatMap((pair) => [pair.text, pair.surface]))]));
  });
});
