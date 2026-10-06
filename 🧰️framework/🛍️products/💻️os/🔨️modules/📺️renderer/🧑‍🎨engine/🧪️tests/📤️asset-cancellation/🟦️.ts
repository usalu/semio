/** 🛑️ Chromium AbortController validates exact-request cancellation isolation. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { chromium } from "playwright";
import { describe, expect, it } from "vitest";
const engine = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const world = resolve(engine, "../../♾️infinite/🌍️world");
const fixture = JSON.parse(readFileSync(resolve(world, "🧫️fixtures/📤️asset-cancellation/🔣️.json"), "utf8"));

describe("🛑️ Exact asset request cancellation", () => {
  
  it("matches browser abort isolation before and during an active request", async () => {
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate((stages: readonly string[]) => stages.map((stage) => {
        const first = new AbortController();
        const unrelated = new AbortController();
        let notifications = 0;
        if (stage === "fetching") first.signal.addEventListener("abort", () => notifications++);
        first.abort();
        first.abort();
        const replacement = new AbortController();
        first.abort();
        return { cancelled: [first.signal.aborted, unrelated.signal.aborted], replacementCancelled: replacement.signal.aborted, notifications };
      }), fixture.stages);
      observed.forEach((row, index) => {
        expect(row.cancelled).toEqual(fixture.expected.cancelled);
        expect(row.replacementCancelled).toBe(fixture.expected.replacementCancelled);
        expect(row.notifications).toBe(fixture.stages[index] === "fetching" ? 1 : 0);
      });
    } finally {
      await browser.close();
    }
  }, 30_000);
});

