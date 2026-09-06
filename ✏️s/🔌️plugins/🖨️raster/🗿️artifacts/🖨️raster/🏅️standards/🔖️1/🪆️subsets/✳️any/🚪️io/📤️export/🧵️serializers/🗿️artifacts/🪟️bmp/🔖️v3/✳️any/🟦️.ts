/** 🚪️ raster → bmp (v3) — TypeScript twin of the exact byte shape the sibling `🦀️.rs` leaf emits.
 *
 * A genuine second implementation, not a re-export. The Rust leaf flattens the layer stack with
 * `raster_composite_image`, hands the RGBA8 canvas to stdio's `SemioImageToBmp`
 * (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🦀️.rs`,
 * which pins `rowOrder = BottomUp`, `planes = 1`, `bitsPerPixel = 24`, `compression = BI_RGB`) and
 * writes it with stdio's own `encode_bmp` direct path
 * (`…/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs:481`). Every constant below restates one of
 * those two files, and `🚪️io/🧪️tests/🟦️.ts` + the Rust
 * `bmp_export_matches_the_typescript_parity_fixture` test assert BOTH sides against the SAME
 * `🧫️fixtures/🪟️solid-3x2.json` bytes, so a drift fails in both languages instead of silently.
 *
 * 🧾️ BMP v3 has no alpha channel — the source alpha is dropped by the format, exactly as the Rust
 * side documents. That is the format's loss, not a shortcut taken here.
 */

/** 🖼️ One flattened raster composite: canonical RGBA8, row-major, top-down, `rgba8.length === width * height * 4`. */
export type RasterCanvas = Readonly<{ width: number; height: number; rgba8: Uint8Array }>;

/** 📐️ `row_bytes(width, bpp)` — a BMP scanline is padded up to a 4-byte boundary. */
export const bmpRowBytes = (width: number, bitsPerPixel: number): number => Math.floor((width * bitsPerPixel + 31) / 32) * 4;

const writeU16 = (out: Uint8Array, at: number, value: number): void => {
  out[at] = value & 0xff;
  out[at + 1] = (value >>> 8) & 0xff;
};

const writeU32 = (out: Uint8Array, at: number, value: number): void => {
  out[at] = value & 0xff;
  out[at + 1] = (value >>> 8) & 0xff;
  out[at + 2] = (value >>> 16) & 0xff;
  out[at + 3] = (value >>> 24) & 0xff;
};

/** 📤️ Writes the 24bpp `BI_RGB`, bottom-up BMP v3 file the Rust leaf produces for this canvas. */
export function rasterCanvasToBmpV3(canvas: RasterCanvas): Uint8Array {
  const { width, height, rgba8 } = canvas;
  if (!Number.isInteger(width) || !Number.isInteger(height) || width <= 0 || height <= 0) throw new Error(`bmp export: ${width}x${height} is not a positive integer canvas`);
  if (rgba8.length !== width * height * 4) throw new Error(`bmp export: rgba8 is ${rgba8.length} bytes, expected ${width * height * 4}`);
  const rowBytes = bmpRowBytes(width, 24);
  const pixelBytes = rowBytes * height;
  const fileSize = 14 + 40 + pixelBytes;
  const out = new Uint8Array(fileSize);
  out[0] = 0x42;
  out[1] = 0x4d;
  writeU32(out, 2, fileSize);
  writeU16(out, 6, 0);
  writeU16(out, 8, 0);
  writeU32(out, 10, 54);
  writeU32(out, 14, 40);
  writeU32(out, 18, width);
  writeU32(out, 22, height);
  writeU16(out, 26, 1);
  writeU16(out, 28, 24);
  writeU32(out, 30, 0);
  writeU32(out, 34, pixelBytes);
  writeU32(out, 38, 0);
  writeU32(out, 42, 0);
  writeU32(out, 46, 0);
  writeU32(out, 50, 0);
  for (let fileRow = 0; fileRow < height; fileRow += 1) {
    const sourceRow = height - 1 - fileRow;
    const rowStart = 54 + fileRow * rowBytes;
    for (let x = 0; x < width; x += 1) {
      const source = (sourceRow * width + x) * 4;
      const target = rowStart + x * 3;
      out[target] = rgba8[source + 2]!;
      out[target + 1] = rgba8[source + 1]!;
      out[target + 2] = rgba8[source]!;
    }
  }
  return out;
}

/** 🔡️ Lowercase hex, the shape `🧫️fixtures/*.json` stores and both languages compare on. */
export const bmpBytesToHex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
