/** 🧊️ The feature edges of glyphs: the outline of a force arrow, moment arc or emitter cone is visible exactly in the variants in which its surface is visible, never under another view.
 *
 * THE REFERENCES are the Khronos `gltf-validator` (no error, no warning), three's `GLTFLoader` (the parsed glTF the viewers read) and the kernel's solid provenance (`Primitive.solids`) that names which solid each primitive belongs to.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { type GlbJson, visibleGlyphs } from "../🧊️model-catalog/🔍️oracles/🟦️.ts";
import { loadGlb, sampleFigure, validateGlb, volumeAssets } from "../🧰️support/🧊️.ts";

const withGlyphs = (assets: readonly Asset[]): Asset[] => assets.filter((a) => ["cantilever-balcony", "electric-fan-heater", "tied-canopy"].includes(a.id));
const assets = [sampleFigure(), ...withGlyphs(volumeAssets())];

describe("🧊️ the feature edges of glyphs follow the variant of their glyph", () => {
  it.each(assets.map((a) => [a.id, a] as const))("%s: validator-clean GLB in which no hidden glyph has a visible outline", async (_id, asset) => {
    const pose = asset.poses.find((p) => p.key !== true)!.id;
    const bytes = await writeGlb(asset, { pose, scale: 50 });
    expect((await validateGlb(bytes, `${asset.id}.glb`)).messages).toEqual([]);
    const { parser } = await loadGlb(bytes);
    const json = parser.json as GlbJson;
    const visible = visibleGlyphs(json);
    let glyphsSeen = 0;
    for (const [variant, entry] of Object.entries(visible)) {
      glyphsSeen += entry.triangles.size;
      for (const solid of entry.lines) expect(entry.triangles.has(solid), `${variant}: outline of ${solid} without its surface`).toBe(true);
    }
    if (asset.id !== sampleFigure().id) expect(glyphsSeen, "the asset has glyphs in some variant").toBeGreaterThan(0);
    const edgeLines = json.meshes.flatMap((m: { primitives: { mode?: number }[] }) => m.primitives).filter((p) => p.mode === 1).length;
    expect(edgeLines).toBeGreaterThan(0);
  });
});
