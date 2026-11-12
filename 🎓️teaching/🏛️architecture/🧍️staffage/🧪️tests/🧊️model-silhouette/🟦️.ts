/** 🧊️ Silhouette agreement: the volume projected into the elevation, front and plan agrees with the authored shapes of the same pose, measured as the IoU of the filled areas (open strokes stroked with their pen width) and the bounding box against `dimensions`.
 *
 * THE REFERENCES are `polygon-clipping` (exact union and intersection areas of the projected triangles and the authored polygons), an owned scanline rasteriser (the method of the authoring aid `volume-check`) that must agree with it, and the physical `dimensions` of the asset. The gate is the authoring gate: at most 8 % per axis and IoU of at least 0.80 per view.
 *
 * @see ./🥒️.feature — the scenarios
 */
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { CAMERAS } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import type { Asset, VolumeSolid } from "../../🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { sampleFigure } from "../🧰️support/🧊️.ts";

type P = [number, number];
type Polygon = P[];
const VIEWPOINTS = ["elevation", "front", "plan"] as const;
const SCALE = 50;
const COARSE = { chordM: 0.01, minSegments: 12, maxSegments: 24 };
const GATE = { bbox: 0.08, iou: 0.8, agreement: 0.03 };
const IDS = ["dining-table", "single-bed", "glulam-beam"] as const;

const stroke = (line: readonly { x: number; y: number }[], width: number): Polygon[] => {
  const h = width / 2;
  const out: Polygon[] = [];
  for (let i = 0; i + 1 < line.length; i++) {
    const [a, b] = [line[i]!, line[i + 1]!];
    const l = Math.hypot(b.x - a.x, b.y - a.y);
    const [ex, ey] = l < 1e-12 ? [h, 0] : [((b.x - a.x) / l) * h, ((b.y - a.y) / l) * h];
    const [nx, ny] = [-ey, ex];
    out.push([[a.x - ex + nx, a.y - ey + ny], [b.x + ex + nx, b.y + ey + ny], [b.x + ex - nx, b.y + ey - ny], [a.x - ex - nx, a.y - ey - ny]]);
  }
  return out;
};

type Vec = { readonly x: number; readonly y: number; readonly z: number };
const world = (frame: { o: Vec; u: Vec; v: Vec; w: Vec }, p: readonly number[]): [number, number, number] => [frame.o.x + p[0]! * frame.u.x + p[1]! * frame.v.x + p[2]! * frame.w.x, frame.o.y + p[0]! * frame.u.y + p[1]! * frame.v.y + p[2]! * frame.w.y, frame.o.z + p[0]! * frame.u.z + p[1]! * frame.v.z + p[2]! * frame.w.z];

/** 🖼️ Authored polygons and projected volume triangles of one pose, per viewpoint. */
function shapesOf(asset: Asset): Record<(typeof VIEWPOINTS)[number], { readonly authored: Polygon[]; readonly volume: Polygon[]; readonly extent: [number, number, number][] }> & { readonly bbox: number[] } {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: SCALE } });
  const pens = asset.representations[SCALE] as unknown as { pens: Record<string, number | null> };
  const triangles: [number, number, number][][] = [];
  const solids = asset.volume.solids.filter((s: VolumeSolid) => s.maxDenominator === undefined || SCALE <= s.maxDenominator);
  for (const solid of solids) {
    const g = tessellateSolid({ ...solid, node: solid.bone } as never, COARSE);
    const frame = state.bones[solid.bone]!.frame as never;
    const pts = Array.from({ length: g.positions.length / 3 }, (_, i) => world(frame, [g.positions[i * 3]!, g.positions[i * 3 + 1]!, g.positions[i * 3 + 2]!]));
    for (let t = 0; t < g.indices.length; t += 3) triangles.push([pts[g.indices[t]!]!, pts[g.indices[t + 1]!]!, pts[g.indices[t + 2]!]!]);
  }
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const t of triangles) for (const p of t) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); }
  const out: any = { bbox: [0, 1, 2].map((k) => hi[k]! - lo[k]!) };
  for (const view of VIEWPOINTS) {
    const cam = CAMERAS[view];
    const pick = (p: readonly number[], axis: "x" | "y" | "z"): number => p[axis === "x" ? 0 : axis === "y" ? 1 : 2]!;
    const project = (p: readonly number[]): P => [cam.paperX.sign * pick(p, cam.paperX.axis), cam.paperUp.sign * pick(p, cam.paperUp.axis)];
    const authored: Polygon[] = [];
    for (const s of state.shapes[view]) {
      if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
      if (s.closed) for (const ring of s.rings) authored.push(ring.map((p) => [p.x, p.y] as P));
      else for (const ring of s.rings) authored.push(...stroke(ring, (((s.shape.penMm ?? pens.pens[s.shape.role]) ?? 0) * SCALE) / 1000));
    }
    out[view] = { authored, volume: triangles.map((t) => t.map(project) as Polygon) };
  }
  return out;
}

const SNAPS = [1e-5, 1e-4, 5e-4];
let SNAP = SNAPS[0]!;
const ringArea = (ring: Polygon): number => { let s = 0; for (let k = 0; k < ring.length; k++) { const [a, b] = [ring[k]!, ring[(k + 1) % ring.length]!]; s += a[0] * b[1] - b[0] * a[1]; } return Math.abs(s / 2); };
const orient = (ring: Polygon): Polygon => {
  const snapped = ring.map(([x, y]) => [Math.round(x / SNAP) * SNAP, Math.round(y / SNAP) * SNAP] as P).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
  return snapped.length >= 3 && ringArea(snapped) > 4 * SNAP * SNAP ? snapped : [];
};
const area = (multi: polygonClipping.MultiPolygon): number => multi.reduce((sum, poly) => sum + poly.reduce((a, ring, i) => { let s = 0; for (let k = 0; k < ring.length - 1; k++) s += ring[k]![0] * ring[k + 1]![1] - ring[k + 1]![0] * ring[k]![1]; return a + (i === 0 ? 1 : -1) * Math.abs(s / 2); }, 0), 0);
const union = (polygons: readonly Polygon[]): polygonClipping.MultiPolygon => {
  let parts = polygons.map(orient).filter((r) => r.length >= 3).map((r) => [[...r, r[0]!]] as polygonClipping.Polygon);
  while (parts.length > 1) {
    const next: polygonClipping.MultiPolygon[] = [];
    for (let i = 0; i < parts.length; i += 16) next.push(polygonClipping.union(parts[i]!, ...parts.slice(i + 1, i + 16)));
    parts = next as never;
  }
  return (parts[0] ?? []) as never;
};

/** 🧮️ IoU by the exact union and intersection of polygon-clipping, on the coordinates snapped to 10 micrometres (and, where its sweep line fails on near-coincident edges, to 0.1 and 0.5 millimetres). */
export function exactIou(authored: readonly Polygon[], volume: readonly Polygon[]): number {
  let failure: unknown;
  for (const snap of SNAPS) {
    SNAP = snap;
    try {
      const a = union(authored);
      const v = union(volume);
      const both = polygonClipping.intersection(a, v);
      const either = area(a) + area(v) - area(both);
      return either === 0 ? 1 : area(both) / either;
    } catch (error) {
      failure = error;
    }
  }
  throw failure;
}

/** 📤️ IoU by an owned scanline mask on a shared grid of 800 cells across (the method of the authoring aid). */
export function rasterIou(authored: readonly Polygon[], volume: readonly Polygon[]): number {
  const pts = [...authored, ...volume].flat();
  const [x0, x1] = [Math.min(...pts.map((p) => p[0])), Math.max(...pts.map((p) => p[0]))];
  const [y0, y1] = [Math.min(...pts.map((p) => p[1])), Math.max(...pts.map((p) => p[1]))];
  const cell = Math.max(x1 - x0, y1 - y0) / 800;
  const w = Math.ceil((x1 - x0) / cell) + 1;
  const h = Math.ceil((y1 - y0) / cell) + 1;
  const mask = (polygons: readonly Polygon[]): Uint8Array => {
    const cells = new Uint8Array(w * h);
    for (const ring of polygons) {
      const [ry0, ry1] = [Math.min(...ring.map((p) => p[1])), Math.max(...ring.map((p) => p[1]))];
      for (let j = Math.max(0, Math.ceil((ry0 - y0) / cell - 0.5)); j <= Math.min(h - 1, Math.floor((ry1 - y0) / cell - 0.5)); j++) {
        const y = y0 + (j + 0.5) * cell;
        const xs: number[] = [];
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) { const [a, b] = [ring[k]!, ring[i]!]; if (a[1] <= y !== b[1] <= y) xs.push(a[0] + ((y - a[1]) / (b[1] - a[1])) * (b[0] - a[0])); }
        xs.sort((a, b) => a - b);
        for (let n = 0; n + 1 < xs.length; n += 2) for (let i = Math.max(0, Math.ceil((xs[n]! - x0) / cell - 0.5)); i <= Math.min(w - 1, Math.ceil((xs[n + 1]! - x0) / cell - 0.5) - 1); i++) cells[j * w + i] = 1;
      }
    }
    return cells;
  };
  const [a, v] = [mask(authored), mask(volume)];
  let both = 0;
  let either = 0;
  for (let i = 0; i < a.length; i++) { both += a[i]! & v[i]!; either += a[i]! | v[i]!; }
  return either === 0 ? 1 : both / either;
}

const assets: Asset[] = [sampleFigure()];
for (const entry of await assetPathsFromFiles()) if ((IDS as readonly string[]).includes(entry.folder)) assets.push((await readStreamedAsset(entry)).asset);

describe("🧊️ the volume agrees with the authored silhouettes of the presentation pose", () => {
  describe.each(assets.map((a) => [a.id, a] as const))("%s", (_id, asset) => {
    const measured = shapesOf(asset);
    it("fills its dimensions within the authoring gate on every axis", () => {
      const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
      dims.forEach((d, k) => expect(Math.abs(measured.bbox[k]! - d) / d, `axis ${k}`).toBeLessThanOrEqual(GATE.bbox));
    });
    it.each(VIEWPOINTS)("agrees in the %s: exact IoU of polygon-clipping at least the gate and equal to the owned raster IoU", { timeout: 120_000 }, (view) => {
      const { authored, volume } = measured[view];
      const exact = exactIou(authored, volume);
      expect(exact, `${view} IoU`).toBeGreaterThanOrEqual(GATE.iou);
      expect(Math.abs(rasterIou(authored, volume) - exact), `${view} raster against exact`).toBeLessThanOrEqual(GATE.agreement);
    });
  });
});
