// #region 🧲️Header
/** @emoji 🔬️ `Paint2dHost` paint witness laws: the `data-layers-json`/`data-assets-json` a raster surface
 * publishes, pinned against the language-neutral fixture and cross-checked against `pngjs`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { PNG } from "pngjs";
import { describe, expect, it } from "vitest";
import { paint2dAssetExtent, paint2dPaintWitnessDom } from "../../🟦️.tsx";
import contract from "../../🧫️fixtures/🔬️paint-witness/🔣️.json";
// #endregion 🔌️Adapters

//#region 🔬️PaintWitness
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../..");

describe("paint-2d paint witness", () => {
  /** ⚖️ LAW: every fixture case publishes exactly the declared layer forest and asset extents. */
  it.each(contract.cases)("$name", ({ documentSyncJson, assetsJson, layersJson, assets }) => {
    const witness = paint2dPaintWitnessDom(documentSyncJson, assetsJson);
    expect(witness.layersJson).toBe(layersJson);
    expect(JSON.parse(witness.assetsJson)).toEqual(assets);
    expect(witness.assetsJson.includes("iVBORw0KGgo")).toBe(false);
  });

  /** ⚖️ LAW: the curated raster demo's committed media reads as a real image through the witness, and
   * `pngjs` — a full decoder, not an IHDR peek — agrees on its extent. */
  it.each(contract.committedAssets)("reads $path at its real extent", ({ path, mime, bytes, width, height }) => {
    const file = readFileSync(resolve(repoRoot, path));
    const decoded = PNG.sync.read(file);
    const published = paint2dPaintWitnessDom("{\"layers\":[]}", JSON.stringify({ media: { mime, data: file.toString("base64") } })).assetsJson;
    expect(JSON.parse(published).media).toEqual({ mime, bytes, width, height });
    expect({ bytes: file.length, width: decoded.width, height: decoded.height }).toEqual({ bytes, width, height });
    expect(published.length).toBeLessThan(128);
  });

  /** ⚖️ LAW: the IHDR peek and `pngjs` agree for every fixture PNG, including the 2×2 placeholder. */
  it("agrees with pngjs on every PNG payload the fixture carries", () => {
    const payloads = contract.cases.flatMap((entry) => Object.values(JSON.parse(entry.assetsJson || "{}") ?? {}) as { mime?: string; data?: string }[]).filter((asset) => asset.mime === "image/png" && typeof asset.data === "string" && asset.data.startsWith("iVBOR"));
    expect(payloads.length).toBeGreaterThan(0);
    for (const asset of payloads) {
      const decoded = PNG.sync.read(Buffer.from(asset.data!, "base64"));
      expect(paint2dAssetExtent(asset as { mime: string; data: string })).toEqual({ mime: "image/png", bytes: Buffer.from(asset.data!, "base64").length, width: decoded.width, height: decoded.height });
    }
  });
});
//#endregion 🔬️PaintWitness
