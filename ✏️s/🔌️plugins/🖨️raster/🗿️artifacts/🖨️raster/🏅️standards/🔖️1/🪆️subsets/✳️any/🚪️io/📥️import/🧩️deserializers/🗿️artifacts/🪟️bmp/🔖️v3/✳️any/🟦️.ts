/** 🚪️ raster ← bmp (v3) — TypeScript twin of the sibling `🦀️.rs` leaf's decode half.
 *
 * A genuine second implementation, not a re-export: it restates stdio's own `decode_bmp`
 * (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs:175`) for the
 * one storage shape a raster export ever writes — 24bpp `BI_RGB`, single plane, bottom-up or
 * top-down — and rebuilds the canonical RGBA8 canvas the Rust leaf turns into a one-`Pixel`-layer
 * document. Every other bit depth/compression is refused with the reason, never guessed at, which
 * mirrors the Rust codec's own typed `Err`.
 *
 * 🧾️ Alpha is forced opaque on decode, exactly as `decode_bmp`'s own 24bpp branch does: BMP v3
 * carries no alpha channel at all.
 */

import type { RasterCanvas } from "../../../../../../📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️";
import { bmpRowBytes } from "../../../../../../📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️";

const readU16 = (bytes: Uint8Array, at: number): number => bytes[at]! | (bytes[at + 1]! << 8);
const readU32 = (bytes: Uint8Array, at: number): number => (bytes[at]! | (bytes[at + 1]! << 8) | (bytes[at + 2]! << 16) | (bytes[at + 3]! << 24)) >>> 0;
const readI32 = (bytes: Uint8Array, at: number): number => readU32(bytes, at) | 0;

/** 📥️ Reads a 24bpp `BI_RGB` BMP v3 file back into the canonical RGBA8 canvas. */
export function bmpV3ToRasterCanvas(bytes: Uint8Array): RasterCanvas {
  if (bytes.length < 54 || bytes[0] !== 0x42 || bytes[1] !== 0x4d) throw new Error("bmp import: bad signature");
  const dataOffset = readU32(bytes, 10);
  const headerSize = readU32(bytes, 14);
  if (headerSize < 40) throw new Error(`bmp import: unsupported info header size ${headerSize}`);
  const width = readI32(bytes, 18);
  const heightField = readI32(bytes, 22);
  const bitsPerPixel = readU16(bytes, 28);
  const compression = readU32(bytes, 30);
  if (width <= 0 || heightField === 0) throw new Error(`bmp import: unsupported ${width}x${heightField} geometry`);
  if (compression !== 0) throw new Error(`bmp import: unsupported compression ${compression} (this twin implements BI_RGB only)`);
  if (bitsPerPixel !== 24) throw new Error(`bmp import: unsupported bit depth ${bitsPerPixel} (this twin implements 24bpp only)`);
  const topDown = heightField < 0;
  const height = Math.abs(heightField);
  const rowBytes = bmpRowBytes(width, 24);
  const rgba8 = new Uint8Array(width * height * 4);
  for (let fileRow = 0; fileRow < height; fileRow += 1) {
    const rowStart = dataOffset + fileRow * rowBytes;
    if (rowStart + rowBytes > bytes.length) throw new Error("bmp import: pixel data truncated");
    const targetRow = topDown ? fileRow : height - 1 - fileRow;
    for (let x = 0; x < width; x += 1) {
      const source = rowStart + x * 3;
      const target = (targetRow * width + x) * 4;
      rgba8[target] = bytes[source + 2]!;
      rgba8[target + 1] = bytes[source + 1]!;
      rgba8[target + 2] = bytes[source]!;
      rgba8[target + 3] = 255;
    }
  }
  return { width, height, rgba8 };
}

/** 🔡️ Parses the lowercase hex `🧫️fixtures/*.json` stores back into bytes. */
export function bmpHexToBytes(hex: string): Uint8Array {
  if (hex.length % 2 !== 0) throw new Error("bmp import: hex payload has an odd length");
  const out = new Uint8Array(hex.length / 2);
  for (let index = 0; index < out.length; index += 1) out[index] = Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16);
  return out;
}
