/** ⚓️ Neutral text placement vectors compared with Chromium's native canvas. */
import { expect, test } from "vitest";
import { build } from "esbuild";
import { chromium } from "playwright";
import { resolve } from "node:path";
import fixture from "../../🧫️fixtures/⚓️alignment/🔣️.json";

test("scene text placement matches native canvas pixels", async () => {
  const bundle = await build({ entryPoints: [resolve(import.meta.dirname, "../../../🟦️.ts")], bundle: true, write: false, format: "iife", globalName: "drawing", platform: "browser", define: { "import.meta.vitest": "false" } });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.addScriptTag({ content: bundle.outputFiles![0]!.text });
    const matches = await page.evaluate((cases) => {
      const draw = (globalThis as unknown as { drawing: { paintDrawingScene: (context: CanvasRenderingContext2D, scene: unknown) => void } }).drawing;
      return cases.map((entry) => {
        const canvases = [document.createElement("canvas"), document.createElement("canvas")];
        for (const canvas of canvases) { canvas.width = 180; canvas.height = 100; }
        const actual = canvases[0]!.getContext("2d")!, reference = canvases[1]!.getContext("2d")!;
        draw.paintDrawingScene(actual, { width: 180, height: 100, nodes: [{ transform: [1, 0, 0, 1, 4, 3], node: { kind: "text", x: 90, y: 45, content: entry.text, size: 16, anchor: entry.anchor, baseline: entry.baseline, ...("font" in entry ? { font: entry.font } : {}) }, fill: { kind: "solid", color: [0.2, 0.4, 0.6, 1] } }] });
        reference.translate(4, 3);
        reference.font = `16px ${"font" in entry ? entry.font : "sans-serif"}`;
        reference.textAlign = entry.anchor === "middle" ? "center" : entry.anchor === "end" ? "right" : "left";
        reference.textBaseline = entry.baseline as CanvasTextBaseline;
        reference.fillStyle = "rgba(51,102,153,1)";
        entry.text.split("\n").forEach((line, index) => reference.fillText(line, 90, 45 + index * 16 * 1.2));
        const a = actual.getImageData(0, 0, 180, 100).data, b = reference.getImageData(0, 0, 180, 100).data;
        return a.every((value, index) => value === b[index]);
      });
    }, fixture.cases);
    expect(matches).toEqual(fixture.cases.map(() => true));
  } finally {
    await browser.close();
  }
}, 30000);
