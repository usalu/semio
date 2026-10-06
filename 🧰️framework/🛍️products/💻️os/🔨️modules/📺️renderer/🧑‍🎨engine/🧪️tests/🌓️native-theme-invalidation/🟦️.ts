
import { chromium, type Browser, type Page } from "playwright";
import { afterAll, beforeAll, describe, expect, it, vi } from "vitest";
import { resolveElementsSurfaceChromeDark } from "../../../../../../../🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx";
import { resolveWgpuHostAppearance, WGPU_PREFERS_DARK_MEDIA_QUERY } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";
import fixture from "../../🧫️fixtures/🌓️native-theme-invalidation/🔣️.json";

type ThemeTransition = { readonly from: "light" | "dark"; readonly to: "light" | "dark"; readonly reactDark: boolean };

describe("🌓️ native theme invalidation oracle", () => {
  const transitions = fixture.transitions as readonly ThemeTransition[];

  it("validates the neutral transition contract", () => {
    expect(fixture.mediaQuery).toBe(WGPU_PREFERS_DARK_MEDIA_QUERY);
  });

  it("answers system appearance exactly like the React surface chrome resolver", () => {
    for (const transition of transitions) {
      vi.stubGlobal("window", { matchMedia: (query: string) => ({ matches: query === fixture.mediaQuery && transition.reactDark }) });
      expect(resolveElementsSurfaceChromeDark("system"), transition.to).toBe(transition.reactDark);
      expect(resolveWgpuHostAppearance({ matchMedia: window.matchMedia }).systemDark, transition.to).toBe(transition.reactDark);
      expect(resolveElementsSurfaceChromeDark("light")).toBe(false);
      expect(resolveElementsSurfaceChromeDark("dark")).toBe(true);
    }
    vi.unstubAllGlobals();
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

    for (const transition of transitions) {
      it(`${transition.from}-to-${transition.to}`, async () => {
        await page.emulateMedia({ colorScheme: transition.to });
        expect(await page.evaluate(query => matchMedia(query).matches, fixture.mediaQuery)).toBe(transition.reactDark);
      });
    }
  });
});
