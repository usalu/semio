import { chromium, type Browser, type Page } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { resolveWgpuHostPlatformPrefs, WGPU_PLATFORM_PREFS_MEDIA_QUERIES } from "@semio-tech/framework-renderer-wgpu/boot-descriptor";
import fixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎛️platform-prefs/🔣️.json" with { type: "json" };

const CASES = [
  { name: "calm", emulate: {}, expected: { forcedColors: false, prefersContrastMore: false, prefersDark: false, prefersReducedMotion: false } },
  { name: "high contrast", emulate: { forcedColors: "active" as const }, expected: { forcedColors: true, prefersContrastMore: false, prefersDark: false, prefersReducedMotion: false } },
  { name: "more contrast", emulate: { contrast: "more" as const }, expected: { forcedColors: false, prefersContrastMore: true, prefersDark: false, prefersReducedMotion: false } },
  { name: "dark and reduced motion", emulate: { colorScheme: "dark" as const, reducedMotion: "reduce" as const }, expected: { forcedColors: false, prefersContrastMore: false, prefersDark: true, prefersReducedMotion: true } },
];

describe("🎛️ host platform preferences", () => {
  it("answers false for all four when matchMedia is unavailable", () => {
    expect(resolveWgpuHostPlatformPrefs({})).toEqual(CASES[0]!.expected);
  });

  it("asks the media queries of the language-agnostic fixture, the very ones the React shell and the Rust driver are pinned to", () => {
    expect(WGPU_PLATFORM_PREFS_MEDIA_QUERIES).toEqual(fixture.queries);
    expect(resolveWgpuHostPlatformPrefs({})).toEqual(fixture.calm);
  });

  it("asks exactly the four documented media queries", () => {
    const asked: string[] = [];
    resolveWgpuHostPlatformPrefs({ matchMedia: (query: string) => ({ matches: asked.push(query) > 0 }) as MediaQueryList });
    expect(asked.sort()).toEqual(Object.values(WGPU_PLATFORM_PREFS_MEDIA_QUERIES).sort());
  });

  describe("Chromium media-query authority", () => {
    let browser: Browser;
    let page: Page;

    beforeAll(async () => {
      browser = await chromium.launch({ headless: true });
      page = await browser.newPage();
    }, 60_000);

    afterAll(async () => {
      await browser?.close();
    });

    for (const row of CASES) {
      it(row.name, async () => {
        await page.emulateMedia({ forcedColors: "none", contrast: "no-preference", colorScheme: "light", reducedMotion: "no-preference", ...row.emulate });
        const queries = WGPU_PLATFORM_PREFS_MEDIA_QUERIES;
        const real = await page.evaluate((q) => ({ forcedColors: matchMedia(q.forcedColors).matches, prefersContrastMore: matchMedia(q.prefersContrastMore).matches, prefersDark: matchMedia(q.prefersDark).matches, prefersReducedMotion: matchMedia(q.prefersReducedMotion).matches }), queries);
        expect(real).toEqual(row.expected);
        const stub = (query: string) => ({ matches: Object.entries(queries).some(([key, text]) => text === query && row.expected[key as keyof typeof row.expected]) }) as MediaQueryList;
        expect(resolveWgpuHostPlatformPrefs({ matchMedia: stub })).toEqual(row.expected);
      });
    }
  });
});
