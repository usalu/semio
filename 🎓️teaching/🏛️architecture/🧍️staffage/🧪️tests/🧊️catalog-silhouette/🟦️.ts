/** 🧊️ The catalog-wide silhouette guard: every one of the 152 assets, in `presentation.defaultPose` at every authored scale, has a body whose bounding box is within one smallest feature of `dimensions` on every axis, whose Hausdorff distance to the drawing is at most two features and whose projection agrees with each depicted view with an IoU of at least 0.92. One physical body: the side elevation, the front elevation, the plan and the volume are projections of the same body.
 *
 * The drawing side is what `evaluate` draws: filled shapes as areas, open strokes and unfilled outlines at the true width of the member they draw (taken from the volume), a declared section against the volume cut by its plane (the drawing must show every cut face and may show whatever remains visible beyond, thin outlines of the rest).
 *
 * THE REFERENCES are sharp (librsvg rasterises the very polygons of the owned scanline raster; the IoU must agree within 0.06), polygon-clipping (exact areas, for the assets of the fixture), a numpy and shapely mesh-plane section (🐍️.py) for the cut faces, hand-computed areas and widths of the fixture, and the physical `dimensions` of the asset.
 * An exemption of an asset is stated with its physical reason in `exemptions` of the fixture; the fixture states none.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️catalog-silhouette/🔣️.json — the vectors
 */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it, vi } from "vitest";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { SCALE_DENOMINATORS, type Asset } from "../../🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, bodyDimensions, worldSolids, cutLoops, measureAsset, memberOf } from "../🧰️support/🌓️silhouette/🟦️.ts";
import { python } from "../🧰️support/🐍️.ts";
import { STAFFAGE_ROOT, fixture, withScratch } from "../🧰️support/🧰️.ts";
import { exactIou, sharpIou } from "./🔍️oracles/🟦️.ts";

type Vectors = {
  gate: { bbox: number; iou: number; scale: number; oracle: number };
  exemptions: Record<string, { check: string; reason: string }[]>;
  exact: string[];
  cuts: { id: string; solid: Record<string, unknown>; axis: 0 | 1 | 2; plane: number; area: number; tolerance: number }[];
  members: { id: string; cell: number; bodies: number[][]; drawn: number[][]; stroke: [number, number][]; pen: number; cap: number; width: number; shift: number; tolerance: number }[];
};

const vectors = fixture<Vectors>("🧊️catalog-silhouette");
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️catalog-silhouette/🐍️.py");
vi.setConfig({ testTimeout: 900_000 });

const shoelace = (loop: readonly (readonly [number, number, number])[], keep: readonly number[]): number => {
  let s = 0;
  for (let i = 0; i < loop.length; i++) {
    const [a, b] = [loop[i]!, loop[(i + 1) % loop.length]!];
    s += a[keep[0]!]! * b[keep[1]!]! - b[keep[0]!]! * a[keep[1]!]!;
  }
  return Math.abs(s / 2);
};

describe("🧊️ the section of a solid by a plane", () => {
  it.each(vectors.cuts)("$id: the cut face has the hand-computed area and equals the numpy and shapely section", async ({ solid, axis, plane, area, tolerance }) => {
    const g = tessellateSolid(solid as never, { chordM: 0.005, minSegments: 48, maxSegments: 96 });
    const world = { id: String(solid.id), positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
    const keep = [0, 1, 2].filter((k) => k !== axis);
    const sorted = cutLoops(world, axis, plane).map((loop) => shoelace(loop, keep)).sort((p, q) => q - p);
    const own = sorted.reduce((sum, a, i) => sum + (i === 0 ? a : -a), 0);
    expect(Math.abs(own - area) / area, `${own} against ${area}`).toBeLessThanOrEqual(tolerance);
    const triangles = Array.from({ length: world.indices.length / 3 }, (_, t) => [0, 1, 2].flatMap((k) => [world.positions[world.indices[t * 3 + k]! * 3]!, world.positions[world.indices[t * 3 + k]! * 3 + 1]!, world.positions[world.indices[t * 3 + k]! * 3 + 2]!]));
    await withScratch(async (directory) => {
      const file = join(directory, "mesh.json");
      writeFileSync(file, JSON.stringify({ triangles, axis, plane }));
      const reference = JSON.parse(python(ORACLE, "geometry", [file])) as { area: number };
      expect(Math.abs(own - reference.area) / reference.area, `${own} against shapely ${reference.area}`).toBeLessThanOrEqual(1e-4);
    });
  });
});

describe("🧊️ the width of the member a stroke draws", () => {
  it.each(vectors.members)("$id", ({ cell, bodies, drawn, stroke, pen, cap, width, shift, tolerance }) => {
    const box = { minX: -0.1, minY: -0.8, maxX: 1.1, maxY: 0.8 };
    const mask = (rects: number[][]): Mask => {
      const m = new Mask(box, cell);
      for (const [x0, y0, x1, y1] of rects) m.fill([[{ x: x0!, y: y0! }, { x: x1!, y: y0! }, { x: x1!, y: y1! }, { x: x0!, y: y1! }]]);
      return m;
    };
    const got = memberOf({ x: stroke[0]![0], y: stroke[0]![1] }, { x: stroke[1]![0], y: stroke[1]![1] }, [mask(bodies)], mask(drawn), pen, cap);
    expect(Math.abs(got.width - width), `width ${got.width}`).toBeLessThanOrEqual(tolerance);
    expect(Math.abs(got.shift - shift), `shift ${got.shift}`).toBeLessThanOrEqual(tolerance);
  });
});

const entries = await assetPathsFromFiles();
const exempt = (asset: string, check: string): boolean => (vectors.exemptions[asset] ?? []).some((e) => e.check === check);

describe("🧊️ every asset of the catalog is one body in its three views and its volume", () => {
  it("finds every asset of the catalog, each once, and excuses none", () => {
    expect(new Set(entries.map((e) => e.folder)).size).toBe(entries.length);
    expect(entries.length).toBeGreaterThanOrEqual(152);
    expect(Object.keys(vectors.exemptions)).toEqual([]);
    expect({ bbox: GATE.bbox, iou: GATE.iou, scale: GATE.scale }).toEqual({ bbox: vectors.gate.bbox, iou: vectors.gate.iou, scale: vectors.gate.scale });
  });

  describe.each(entries.map((e) => [e.folder, e] as const))("%s", (_id, entry) => {
    it("fills its dimensions within one smallest feature and agrees with the body of every authored scale in the elevation, the front and the plan, as sharp rasterises it", { timeout: 900_000 }, async () => {
      const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
      const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
      const scales = SCALE_DENOMINATORS.filter((d) => (asset.volumes as Record<string, { status: string }>)[String(d)]?.status === "authored");
      expect(scales.length, `${asset.id} has no authored volume`).toBeGreaterThan(0);
      for (const scale of scales) {
        const feature = featureOf(scale);
        const m = asset.representations[`${scale}`].status === "authored" ? measureAsset(asset, scale, undefined, undefined, feature) : undefined;
        const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones, scale));
        if (!exempt(asset.id, "bbox")) bbox.forEach((b, k) => expect(Math.abs(b - dims[k]!), `${asset.id} 1:${scale} axis ${k}: ${b} m against ${dims[k]} m`).toBeLessThanOrEqual(feature));
        if (m === undefined) continue;
        for (const view of VIEWPOINTS) {
          const measured = m.views[view];
          if (!exempt(asset.id, view)) {
            expect(measured.iou, `${asset.id} 1:${scale} ${view}${m.sections[view] === undefined ? "" : " (section)"}`).toBeGreaterThanOrEqual(vectors.gate.iou);
            expect(measured.hausdorffM, `${asset.id} 1:${scale} ${view} Hausdorff`).toBeLessThanOrEqual(2 * feature);
          }
          const reference = await sharpIou(measured);
          expect(Math.abs(reference - measured.iou), `${asset.id} 1:${scale} ${view}: sharp ${reference} against owned ${measured.iou}`).toBeLessThanOrEqual(vectors.gate.oracle);
          if (vectors.exact.includes(asset.id)) {
            const exact = exactIou(measured);
            expect(Math.abs(exact - measured.iou), `${asset.id} 1:${scale} ${view}: polygon-clipping ${exact} against owned ${measured.iou}`).toBeLessThanOrEqual(vectors.gate.oracle);
          }
        }
      }
    });
  });
});
