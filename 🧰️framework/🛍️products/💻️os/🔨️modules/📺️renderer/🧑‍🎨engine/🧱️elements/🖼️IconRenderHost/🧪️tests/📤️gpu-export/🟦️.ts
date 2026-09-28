/** @emoji 🖼️ Chromium and Sharp validate the request-sized transparent export contract. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { chromium } from "playwright";
import sharp from "sharp";
import { describe, expect, it } from "vitest";

const host = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const fixture = JSON.parse(readFileSync(resolve(host, "🧫️fixtures/📤️gpu-export/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(host, "🧬️schema/📤️gpu-export/🔣️.json"), "utf8"));

describe("🖼️ Icon GPU PNG export", () => {
  it("validates the neutral request and publication contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("matches browser PNG pixels at a width requiring GPU row padding", async () => {
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const png = await page.evaluate((fixture) => {
        const canvas = document.createElement("canvas");
        canvas.width = fixture.width;
        canvas.height = fixture.height;
        const context = canvas.getContext("2d")!;
        for (const row of fixture.rectangles) {
          context.fillStyle = `rgba(${row.rgba[0]},${row.rgba[1]},${row.rgba[2]},${row.rgba[3] / 255})`;
          context.fillRect(...row.bounds as [number, number, number, number]);
        }
        return canvas.toDataURL("image/png").split(",")[1];
      }, fixture);
      const { data, info } = await sharp(Buffer.from(png, "base64")).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
      expect([info.width, info.height]).toEqual([fixture.width, fixture.height]);
      for (const probe of fixture.probes) {
        const offset = (probe.position[1] * info.width + probe.position[0]) * 4;
        expect([...data.subarray(offset, offset + 4)]).toEqual(probe.rgba);
      }
    } finally {
      await browser.close();
    }
  }, 30_000);
});

