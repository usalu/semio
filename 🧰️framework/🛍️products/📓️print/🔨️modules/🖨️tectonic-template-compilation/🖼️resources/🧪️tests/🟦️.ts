import { test, expect } from "bun:test";
import sharp from "sharp";
import { renderPrintSvgResourceV1 } from "../🟦️.ts";
import cases from "./🔣️.json" with { type: "json" };

for (const row of cases.cases) test("authored SVG renders to the finite language-neutral PNG contract", async () => {
  const bytes = await renderPrintSvgResourceV1(new TextEncoder().encode(row.svg), row.width, new AbortController().signal);
  const actual = await sharp(bytes).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  expect([actual.info.width, actual.info.height]).toEqual([row.width, row.height]);
  expect([...actual.data.subarray(0, 4)]).toEqual(row.pixel);
  const oracle = await sharp({ create: { width: row.width, height: row.height, channels: 4, background: { r: row.pixel[0]!, g: row.pixel[1]!, b: row.pixel[2]!, alpha: row.pixel[3]! / 255 } } }).raw().toBuffer();
  expect(actual.data.equals(oracle)).toBe(true);
});
test("SVG resource conversion honors cancellation and refuses external references", async () => {
  const controller = new AbortController(); controller.abort();
  await expect(renderPrintSvgResourceV1(new TextEncoder().encode(cases.cases[0]!.svg), 80, controller.signal)).rejects.toThrow();
  await expect(renderPrintSvgResourceV1(new TextEncoder().encode('<svg><image href="https://example.com/image.png"/></svg>'), 80, new AbortController().signal)).rejects.toThrow("Self-contained");
});

test("tall SVG output is rejected before raster allocation", async () => {
  await expect(renderPrintSvgResourceV1(new TextEncoder().encode('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1 1000"></svg>'), 4096, new AbortController().signal)).rejects.toThrow("dimensions");
});
