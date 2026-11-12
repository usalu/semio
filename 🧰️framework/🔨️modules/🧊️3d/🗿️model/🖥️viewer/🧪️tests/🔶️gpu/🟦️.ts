/** 🔶️ Real WebGL2: bundles the browser harness, renders the rig with the WebGL2 backend in headless Chromium (Playwright, a test-only dependency; SwiftShader when no GPU) and compares every frame with the software rasteriser; also loses and restores the context. Skipped when no browser can be launched. */
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { fixture } from "../🧰️support/🟦️.ts";

const harness = resolve(dirname(fileURLToPath(import.meta.url)), "../🧰️support/🖥️harness/🟦️.ts");

describe("webgl2 on a real browser", () => {
  it("agrees with the software rasteriser in coverage and colour and recovers from context loss", async (ctx) => {
    let browser: import("playwright").Browser | null = null;
    try {
      const { chromium } = await import("playwright");
      browser = await chromium.launch({ args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
    } catch {
      return ctx.skip("Playwright cannot launch Chromium on this host (browser binaries not installed), so the real WebGL2 path is not exercised");
    }
    try {
      const { build } = await import("esbuild");
      const bundle = await build({ entryPoints: [harness], bundle: true, format: "iife", target: "es2022", write: false, logLevel: "error" });
      const page = await browser.newPage();
      await page.setContent("<!doctype html><body></body>");
      await page.addScriptTag({ content: bundle.outputFiles[0]!.text });
      const result = await page.evaluate((doc) => (window as any).semioGpu.run(doc), fixture("🧱️rig").document);
      expect(result.status).toEqual(["ready", "lost", "restored"]);
      expect(result.recovered).toBe(true);
      expect(result.rows).toHaveLength(10);
      for (const row of result.rows) {
        expect(row.coverageAgreement, `${row.preset}/${row.variant}`).toBeGreaterThan(0.97);
        expect(row.meanAbsDiff, `${row.preset}/${row.variant}`).toBeLessThan(40);
        expect(row.glCovered).toBeGreaterThan(500);
      }
    } finally {
      await browser.close();
    }
  }, 120_000);
});
