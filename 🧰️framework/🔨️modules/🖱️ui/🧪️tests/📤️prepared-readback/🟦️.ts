/** 📤️ Chromium and Sharp validate the neutral prepared-target readback contract. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { chromium } from "playwright";
import sharp from "sharp";
import { describe, expect, it } from "vitest";

const uiRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const fixture = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/📤️prepared-readback/🔣️.json"), "utf8"));

describe("📤️ Prepared target readback", () => {
  it("validates the shared byte and allocation contract", () => {
    for (const row of fixture.cases) expect(row.rgba).toHaveLength(row.width * row.height * 4);
  });

  it("preserves RGBA and alpha through the independent PNG encoder and decoder", async () => {
    for (const row of fixture.cases) {
      const png = await sharp(Buffer.from(row.rgba), { raw: { width: row.width, height: row.height, channels: 4 } }).png().toBuffer();
      const decoded = await sharp(png).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
      expect([decoded.info.width, decoded.info.height], row.id).toEqual([row.width, row.height]);
      expect([...decoded.data], row.id).toEqual(row.rgba);
    }
  });

  it("reads exact unaligned rows and encoded sRGB bytes from Chromium WebGPU", async () => {
    const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader"] });
    try {
      const page = await browser.newPage();
      await page.route("https://renderer.test/**", (route) => route.fulfill({ contentType: "text/html", body: "<!doctype html><title>Prepared target readback</title>" }));
      await page.goto("https://renderer.test/");
      const observed = await page.evaluate(async (contract) => {
        const adapter = await navigator.gpu.requestAdapter();
        if (!adapter) throw new Error("WebGPU is required for prepared target readback");
        const device = await adapter.requestDevice();
        device.pushErrorScope("validation");
        const rows: number[][] = [];
        for (const row of contract.cases) {
          const texture = device.createTexture({ size: [row.width, row.height], format: row.format, usage: GPUTextureUsage.COPY_SRC | GPUTextureUsage.COPY_DST });
          const source = new Uint8Array(row.rgba);
          if (row.format === "bgra8unorm") for (let offset = 0; offset < source.length; offset += 4) [source[offset], source[offset + 2]] = [source[offset + 2], source[offset]];
          device.queue.writeTexture({ texture }, source, { bytesPerRow: row.width * 4 }, [row.width, row.height]);
          const buffer = device.createBuffer({ size: row.paddedBytesPerRow * row.height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
          const encoder = device.createCommandEncoder();
          encoder.copyTextureToBuffer({ texture }, { buffer, bytesPerRow: row.paddedBytesPerRow }, [row.width, row.height]);
          device.queue.submit([encoder.finish()]);
          await buffer.mapAsync(GPUMapMode.READ);
          const mapped = new Uint8Array(buffer.getMappedRange());
          const rgba: number[] = [];
          for (let y = 0; y < row.height; y++) for (let x = 0; x < row.width; x++) {
            const offset = y * row.paddedBytesPerRow + x * 4;
            rgba.push(...(row.format === "bgra8unorm" ? [mapped[offset + 2], mapped[offset + 1], mapped[offset], mapped[offset + 3]] : [...mapped.subarray(offset, offset + 4)]));
          }
          rows.push(rgba);
          buffer.unmap();
          buffer.destroy();
          texture.destroy();
        }
        const texture = device.createTexture({ size: [1, 1], format: "rgba8unorm", viewFormats: ["rgba8unorm-srgb"], usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
        const buffer = device.createBuffer({ size: contract.alignment, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
        const encoder = device.createCommandEncoder();
        const pass = encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView({ format: "rgba8unorm-srgb" }), loadOp: "clear", storeOp: "store", clearValue: contract.srgbClear.linear }] });
        pass.end();
        encoder.copyTextureToBuffer({ texture }, { buffer, bytesPerRow: contract.alignment }, [1, 1]);
        device.queue.submit([encoder.finish()]);
        await buffer.mapAsync(GPUMapMode.READ);
        const srgb = [...new Uint8Array(buffer.getMappedRange()).subarray(0, 4)];
        buffer.unmap();
        buffer.destroy();
        texture.destroy();
        const error = await device.popErrorScope();
        device.destroy();
        if (error) throw new Error(error.message);
        return { rows, srgb };
      }, fixture);
      expect(observed.rows).toEqual(fixture.cases.map((row: any) => row.rgba));
      expect(observed.srgb).toEqual(fixture.srgbClear.rgba);
    } finally {
      await browser.close();
    }
  }, 30_000);
});
