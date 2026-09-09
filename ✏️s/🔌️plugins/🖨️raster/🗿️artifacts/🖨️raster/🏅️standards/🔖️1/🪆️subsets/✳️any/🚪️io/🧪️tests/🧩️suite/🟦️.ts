/** 🧪️ raster io — cross-language parity for the `🪟️bmp` leaves.
 *
 * `🧫️fixtures/*.json` is the single shared oracle: the Rust `🚪️io/🦀️.rs` test
 * `bmp_export_matches_the_typescript_parity_fixture` asserts the SAME `bmpHex` from
 * `raster_composite_image` → stdio's `SemioImageToBmp` → stdio's `encode_bmp`, and
 * `bmp_import_matches_the_typescript_parity_fixture` asserts the same `rgba8` comes back out of the
 * import leaf. A disagreement between the two implementations therefore fails on both sides instead
 * of drifting silently — which is exactly what "the raster BMP hop is real" has to mean.
 */

import { describe, expect, test } from "bun:test";
import { bmpBytesToHex, rasterCanvasToBmpV3 } from "../../📤️export/🧵️serializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️";
import { bmpHexToBytes, bmpV3ToRasterCanvas } from "../../📥️import/🧩️deserializers/🗿️artifacts/🪟️bmp/🔖️v3/✳️any/🟦️";

type Fixture = Readonly<{ schema: string; width: number; height: number; rgba8: number[]; bmpHex: string }>;

const FIXTURES = ["🪟️solid-3x2.json", "🌈️gradient-5x3.json"] as const;

async function load(name: string): Promise<Fixture> {
  return (await Bun.file(new URL(`../../🧫️fixtures/${name}`, import.meta.url)).json()) as Fixture;
}

describe("raster io — bmp v3 parity", () => {
  for (const name of FIXTURES) {
    test(`${name}: the TypeScript writer reproduces the Rust bytes`, async () => {
      const fixture = await load(name);
      expect(fixture.schema).toBe("semio.raster.io.bmp-parity.v1");
      const bytes = rasterCanvasToBmpV3({ width: fixture.width, height: fixture.height, rgba8: Uint8Array.from(fixture.rgba8) });
      expect(bmpBytesToHex(bytes)).toBe(fixture.bmpHex);
      expect(bytes[0]).toBe(0x42);
      expect(bytes[1]).toBe(0x4d);
    });

    test(`${name}: the TypeScript reader recovers the canvas the Rust import leaf builds`, async () => {
      const fixture = await load(name);
      const canvas = bmpV3ToRasterCanvas(bmpHexToBytes(fixture.bmpHex));
      expect([canvas.width, canvas.height]).toEqual([fixture.width, fixture.height]);
      expect(Array.from(canvas.rgba8)).toEqual(fixture.rgba8);
    });

    test(`${name}: writer ∘ reader is the identity on this canvas`, async () => {
      const fixture = await load(name);
      const canvas = bmpV3ToRasterCanvas(bmpHexToBytes(fixture.bmpHex));
      expect(bmpBytesToHex(rasterCanvasToBmpV3(canvas))).toBe(fixture.bmpHex);
    });
  }

  test("a bit depth this dialect never writes is refused with the reason, never guessed at", async () => {
    const fixture = await load("🪟️solid-3x2.json");
    const bytes = bmpHexToBytes(fixture.bmpHex);
    bytes[28] = 32;
    expect(() => bmpV3ToRasterCanvas(bytes)).toThrow("unsupported bit depth 32");
  });
});
