// @vitest-environment node
/** 🧱️ Neutral and Chromium layout oracle for retained frame origin freshness. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🧱️retained-frame-progress/🔣️.json"), "utf8"));

describe("🧱️ retained frame progress and origin", () => {
  it("retains the accepted viewport publication policy", () => {
    expect(fixture.publication).toEqual({ draw: "acceptedOnly", hit: "acceptedOnly", origin: "singleAcceptedViewport" });
  });

  it("matches Chromium when a mounted surface moves without changing size", async () => {
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage({ viewport: { width: 480, height: 360 } });
      const [initial, moved] = await page.evaluate(({ initial, moved }) => {
        document.body.style.margin = "0";
        const surface = document.createElement("button");
        surface.textContent = "Target";
        surface.style.cssText = `position:absolute;left:${initial[0]}px;top:${initial[1]}px;width:${initial[2]}px;height:${initial[3]}px`;
        document.body.append(surface);
        const rect = () => {
          const value = surface.getBoundingClientRect();
          return [value.x, value.y, value.width, value.height];
        };
        const before = rect();
        surface.style.cssText = `position:absolute;left:${moved[0]}px;top:${moved[1]}px;width:${moved[2]}px;height:${moved[3]}px`;
        return [before, rect()];
      }, fixture.sameSizeOriginMove);
      expect(initial).toEqual(fixture.sameSizeOriginMove.initial);
      expect(moved).toEqual(fixture.sameSizeOriginMove.moved);
      expect(moved.slice(2)).toEqual(initial.slice(2));
      expect(moved.slice(0, 2)).toEqual(fixture.sameSizeOriginMove.expectedAcceptedOrigin);
    } finally {
      await browser.close();
    }
  });
});
