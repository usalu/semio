/** 📷️ Chromium Canvas and Sharp validate straight-alpha PNG export bytes. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { chromium } from "playwright";
import sharp from "sharp";
import { describe, expect, it } from "vitest";

const host = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const fixture = JSON.parse(readFileSync(resolve(host, "🧫️fixtures/📤️png-export/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(host, "🧬️schema/📤️png-export/🔣️.json"), "utf8"));

describe("📷️ Icon PNG export", () => {
  it("validates the neutral alpha normalization contract", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("matches Canvas PNG publication for premultiplied framebuffer pixels", async () => {
    const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate((rows: readonly { premultiplied: readonly [number, number, number, number] }[]) => {
        const canvas = document.createElement("canvas");
        canvas.width = rows.length;
        canvas.height = 1;
        const gl = canvas.getContext("webgl", { alpha: true, premultipliedAlpha: true, preserveDrawingBuffer: true, antialias: false });
        if (!gl) throw new Error("WebGL is required for the Canvas PNG oracle");
        gl.enable(gl.SCISSOR_TEST);
        rows.forEach((row, index) => {
          gl.scissor(index, 0, 1, 1);
          gl.clearColor(...row.premultiplied.map((channel: number) => channel / 255) as [number, number, number, number]);
          gl.clear(gl.COLOR_BUFFER_BIT);
        });
        const pixels = new Uint8Array(rows.length * 4);
        gl.readPixels(0, 0, rows.length, 1, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
        const png = canvas.toDataURL("image/png").split(",")[1];
        return { pixels: [...pixels], png };
      }, fixture.cases);
      expect(observed.pixels).toEqual(fixture.cases.flatMap((row: any) => row.premultiplied));
      const decoded = await sharp(Buffer.from(observed.png, "base64")).ensureAlpha().raw().toBuffer();
      expect([...decoded]).toEqual(fixture.cases.flatMap((row: any) => row.straight));
    } finally {
      await browser.close();
    }
  }, 30_000);
});
