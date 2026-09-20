import { readFileSync } from "node:fs";
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { REFERENCE_IMAGE_DECODED_BYTE_CAPACITY, REFERENCE_IMAGE_READBACK_BYTE_CAPACITY, referenceImageSourceDimensions, referenceImageStripRows, referenceImageTargetSize } from "../../🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts";

const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🖼️reference-image-decode/🔣️.json", import.meta.url), "utf8"));
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🖼️reference-image-decode/🔣️.json", import.meta.url), "utf8"));

describe("reference image decode ownership", () => {
  test("neutral fixture is schema-owned and dimensions share the RGBA budget", () => {
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(fixture.reactRoutedExtensions).toEqual(["png", "jpg", "jpeg", "gif", "webp", "bmp", "avif", "tif", "tiff", "svg", "pdf"]);
    expect(fixture.browserMetadataFormats).toEqual(["image/png", "image/jpeg", "image/gif", "image/webp", "image/bmp", "image/svg+xml"]);
    expect(fixture.browserWorkerBitmapFormats).toEqual(["image/png", "image/jpeg", "image/gif", "image/webp", "image/bmp"]);
    expect(fixture.browserPageBitmapFormats).toEqual(["image/svg+xml"]);
    expect(fixture.browserPageDecodeCapacity).toBe(1);
    expect(fixture.unimplementedWgpuFormats).toEqual(["image/avif", "image/tiff", "application/pdf"]);
    expect(fixture.decodedByteCapacity).toBe(REFERENCE_IMAGE_DECODED_BYTE_CAPACITY);
    expect(fixture.readbackStripByteCapacity).toBe(REFERENCE_IMAGE_READBACK_BYTE_CAPACITY);
    expect(fixture.readbackMode).toBe("row-strip");
    expect(fixture.yieldAfterEveryStrip).toBe(true);
    for (const vector of fixture.cases) {
      const oriented = vector.sourceOrientation >= 5 && vector.sourceOrientation <= 8 ? [vector.sourceHeight, vector.sourceWidth] : [vector.sourceWidth, vector.sourceHeight];
      expect(referenceImageTargetSize(oriented[0], oriented[1])).toEqual([vector.expectedWidth, vector.expectedHeight]);
      expect(referenceImageStripRows(vector.expectedWidth)).toBe(vector.expectedRowsPerStrip);
      expect(Math.ceil(vector.expectedHeight / vector.expectedRowsPerStrip)).toBe(vector.expectedStripCount);
    }
  });

  test("encoded dimensions reject a compressed source before browser decode allocation", () => {
    const png = (width: number, height: number) => {
      const bytes = new Uint8Array(24);
      bytes.set([137, 80, 78, 71, 13, 10, 26, 10]);
      bytes.set([0, 0, 0, 13, 73, 72, 68, 82], 8);
      new DataView(bytes.buffer).setUint32(16, width);
      new DataView(bytes.buffer).setUint32(20, height);
      return bytes;
    };
    expect(referenceImageSourceDimensions(png(4096, 4096))).toEqual({ width: 4096, height: 4096, orientation: 1, mediaType: "image/png" });
    expect(() => referenceImageSourceDimensions(png(4097, 4097))).toThrow("reference-image-source-pixel-credits");
    expect(() => referenceImageSourceDimensions(new Uint8Array([1, 2, 3]))).toThrow("reference-image-format");
  });

  test("EXIF quarter-turn dimensions follow browser from-image orientation", () => {
    const exif = [0xff, 0xe1, 0, 34, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 18, 1, 3, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0];
    const original = [0xff, 0xe1, 0, 34, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 18, 1, 3, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
    const sof = [0xff, 0xc0, 0, 11, 8, 0, 2, 0, 3, 1, 1, 0x11, 0];
    const jpeg = new Uint8Array([0xff, 0xd8, ...exif, ...original, ...sof, 0xff, 0xd9]);
    expect(referenceImageSourceDimensions(jpeg)).toEqual({ width: 2, height: 3, orientation: 6, mediaType: "image/jpeg" });
  });

  test("React image, SVG, and browser raster formats retain bounded metadata", () => {
    const gif = new Uint8Array([71, 73, 70, 56, 57, 97, 2, 0, 3, 0]);
    const webp = new Uint8Array(30);
    webp.set([82, 73, 70, 70], 0);
    webp.set([87, 69, 66, 80, 86, 80, 56, 88], 8);
    webp.set([1, 0, 0], 24);
    webp.set([2, 0, 0], 27);
    const bmp = new Uint8Array(26);
    bmp.set([66, 77]);
    new DataView(bmp.buffer).setInt32(18, 2, true);
    new DataView(bmp.buffer).setInt32(22, -3, true);
    const svg = new TextEncoder().encode(`<svg xmlns="http://www.w3.org/2000/svg" width="2in" height="50%" viewBox="0 0 400 200"></svg>`);
    expect(referenceImageSourceDimensions(gif)).toEqual({ width: 2, height: 3, orientation: 1, mediaType: "image/gif" });
    expect(referenceImageSourceDimensions(webp)).toEqual({ width: 2, height: 3, orientation: 1, mediaType: "image/webp" });
    expect(referenceImageSourceDimensions(bmp)).toEqual({ width: 2, height: 3, orientation: 1, mediaType: "image/bmp" });
    expect(referenceImageSourceDimensions(svg)).toEqual({ width: 192, height: 100, orientation: 1, mediaType: "image/svg+xml" });
  });
});
