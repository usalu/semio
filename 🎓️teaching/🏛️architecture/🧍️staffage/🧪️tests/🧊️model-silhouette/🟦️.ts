/** 🧊️ Silhouette agreement: the volume projected into the elevation, front and plan agrees with the authored shapes of the same pose, measured as the IoU of the filled areas (open strokes stroked with their pen width) and the bounding box against `dimensions`.
 *
 * THE REFERENCES are `polygon-clipping` (exact union and intersection areas of the projected triangles and the authored polygons), an owned scanline rasteriser (the method of the authoring aid `volume-check`) that must agree with it, and the physical `dimensions` of the asset. The gate is the authoring gate: at most 8 % per axis and IoU of at least 0.80 per view.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { bodyOf } from "../🧰️support/🧊️volumes/🟦️.ts";
import { readFileSync } from "node:fs";
import { Box3, ExtrudeGeometry, Matrix4, Shape, Vector2, Vector3 } from "three";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { featureRule, thinnestDrawn } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { copyMesh, worldSolids } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { restFrames, solveChain } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { worldFrameOf, type FrameLike } from "../../🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { CAMERAS } from "../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import type { Asset, Bone, Rig, VolumeSolid } from "../../🧬️schema/🟦️.ts";
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
  const solids = bodyOf(asset, SCALE);
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
const lattice = (v: number): number => Number((Math.round(v / SNAP) * SNAP).toFixed(10));
const orient = (ring: Polygon): Polygon => {
  const snapped = ring.map(([x, y]) => [lattice(x), lattice(y)] as P).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
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
for (const entry of await assetPathsFromFiles()) if ((IDS as readonly string[]).includes(entry.folder)) {
  const candidate = (await readStreamedAsset(entry)).asset;
  if (candidate.volumes?.[`${SCALE}` as "50"]?.status === "authored") assets.push(candidate);
}

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

it("measures both faces of the cloth in the furled and open poses", () => {
  const asset: Asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🪑️objects/market-parasol/🔣️.json", import.meta.url), "utf8")).asset;
  const rest = restFrames(asset.rig);
  const matrix = (frame: FrameLike) => {
    const { o, u, v, w } = worldFrameOf(frame);
    return new Matrix4().makeBasis(new Vector3(...u), new Vector3(...v), new Vector3(...w)).setPosition(...o);
  };
  for (const scale of [50, 100, 200, 500] as const) {
    const volume = asset.volumes[scale];
    if (volume.status !== "authored") throw Error("Missing parasol scale");
    for (const pose of ["open-still", "furled"]) {
      const { bones, shapes } = evaluate(asset, { kind: "pose", pose }, { kind:"scale", scale:{ denominator:scale } });
      for (const list of ["body", "thermal"] as const) {
        const meshes = worldSolids(asset, bones, scale, list);
        const canopy = new Box3();
        for (const gore of volume[list]) {
          if (!("kind" in gore) || gore.kind !== "extrude" || !/^(panel-|th-panel-|th-underside-)/.test(gore.id)) continue;
          const expected = new Box3();
          const palette = (gore.weights ?? [{ bone: gore.bone, weight: 1 }]).map(({bone, weight}) => ({
            weight, matrix: matrix(bones[bone]!.frame).multiply(matrix(rest[bone]!.frame).invert()).multiply(matrix(rest[gore.bone]!.frame)),
          }));
          for (const [z, x] of gore.profile) for (const y of [gore.from, gore.to]) {
            const p = new Vector3();
            for (const joint of palette) p.addScaledVector(new Vector3(x, y, z).applyMatrix4(joint.matrix), joint.weight);
            expected.expandByPoint(p);
          }
          const actual = new Box3();
          const vertices = meshes.find(m => m.id === gore.id)!.positions;
          for (let i = 0; i < vertices.length; i += 3) actual.expandByPoint(new Vector3().fromArray(vertices, i));
          expect(actual.max.distanceTo(expected.max)).toBeLessThan(1e-6);
          expect(actual.min.distanceTo(expected.min)).toBeLessThan(1e-6);
          canopy.union(actual);
        }
        if (list === "body") {
          const drawn = new Box3();
          for (const part of shapes.elevation) if (part.shape.id.startsWith("panel-")) {
            for (const ring of part.rings) for (const pt of ring) drawn.expandByPoint(new Vector3(pt.x, pt.y, 0));
          }
          expect(Math.abs(drawn.max.x-canopy.max.x)).toBeLessThan(.001);
          expect(Math.abs(drawn.min.x-canopy.min.x)).toBeLessThan(.001);
        }
        const width = canopy.getSize(new Vector3()).x;
        if (pose === "furled") expect(width).toBeLessThan(0.65);
        else expect(width).toBeGreaterThan(2.4);
      }
    }
  }
});

it("reports a missing fold joint on a drawn extrusion", () => {
  const asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🪑️objects/market-parasol/🔣️.json", import.meta.url), "utf8")).asset;
  const drawing = asset.representations["50"].geometry.elevation.find((shape: {id:string}) => shape.id === "panel-f");
  drawing.solid.weights = [{ bone: "missing-fold-joint", weight: 1 }];
  const findings = validateAsset(asset);
  expect(findings.some(issue => issue.path.endsWith("weights[0].bone") && issue.code === "volume-weights-bone")).toBe(true);
});

it("keeps radial folds sewn through crown tilt and changing aperture", () => {
  const bone = (id: string, parent: string | null): Bone => ({ id, parent, x: 0, y: 0, angleDeg: 0, lengthM: 1, title: { en: id, de: id }, joint: { type: "revolute" } });
  const facets = ["fold-a", "fold-b", "fold-c", "fold-d"];
  const rig: Rig = {
    kind: "mechanical", root: "crown",
    bones: [bone("crown", null), { ...bone("left", "crown"), abductionDeg: 12 }, bone("right", "crown"), ...facets.map(id => bone(id, "crown"))],
    radialFolds: [{ left: "left", right: "right", facets, cutAngleDeg: 48 }],
  };
  const angle = 12 * Math.PI / 180;
  for (const opening of [0, 1, 8, 16, 32, 48]) {
    const frames = solveChain(rig, { crown: { angleDeg: -15, twistDeg: 25 }, right: { abductionDeg: 12 - opening } }, { x: 1, y: 2, angleDeg: 65 });
    let previous = new Vector3(...frames.left!.frame.u);
    for (const id of facets) {
      const { u, v, w } = frames[id]!.frame;
      const first = new Vector3(...u), normal = new Vector3(...v), transverse = new Vector3(...w);
      expect(first.distanceTo(previous)).toBeLessThan(1e-7);
      expect(first.clone().cross(normal).distanceTo(transverse)).toBeLessThan(1e-7);
      const last = first.clone().multiplyScalar(Math.cos(angle)).addScaledVector(transverse, -Math.sin(angle));
      expect(first.clone().cross(last).length() / 2).toBeCloseTo(Math.sin(angle) / 2, 10);
      previous = last;
    }
    expect(previous.distanceTo(new Vector3(...frames.right!.frame.u))).toBeLessThan(1e-7);
  }
});

it("measures the projected edge of a folded sheet for the minimum printed feature", () => {
  const asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🛋️furniture/height-adjustable-desk/🔣️.json", import.meta.url), "utf8")).asset;
  const points = [[0,0],[.6,0],[.6,.34],[0,.34]];
  const panel = { id:"test-panel", bone:"top", role:"body", kind:"poly", points, filled:true, closed:true, tone:.4, z:2, solid:{kind:"extruded", sweep:"w", z0:.115, z1:.117} };
  asset.representations[500].geometry = { elevation:[panel], front:[panel], plan:[panel] };
  const reference = new ExtrudeGeometry(new Shape(points.map(([x,y]) => new Vector2(x,y))), {depth:.002, bevelEnabled:false});
  reference.computeBoundingBox();
  const thickness = reference.boundingBox!.getSize(new Vector3()).z;
  expect(thinnestDrawn(asset,500).get("top")).toBeCloseTo(thickness,8);
  reference.dispose();
});

it("evicts cold tessellations by retained bytes while preserving recently used meshes", () => {
  const quality = { chordM: .0001, minSegments: 64, maxSegments: 64 };
  const solid = (radius: number): VolumeSolid => ({ id:"cap", bone:"shaft", material:"metal", role:"body", tone:.5, kind:"ellipsoid", radii:[radius,.15,.12] });
  const cold = copyMesh(solid(.13), quality);
  const hot = copyMesh(solid(.14), quality);
  const bytes = hot.positions.byteLength + hot.indices.byteLength;
  const count = Math.ceil(40 * 1024 * 1024 / bytes);
  for (let i = 0; i < count; i++) {
    copyMesh(solid(.2 + i / 100000), quality);
    if (i % 16 === 0) expect(copyMesh(solid(.14), quality)).toBe(hot);
  }
  const regenerated = copyMesh(solid(.13), quality);
  expect(regenerated).not.toBe(cold);
  expect(regenerated.positions).toEqual(cold.positions);
  expect(regenerated.indices).toEqual(cold.indices);
});

it("uses both inverse bind transforms for a weighted cloth profile and its drawing", () => {
  const asset = JSON.parse(readFileSync(new URL("../../🗂️catalog/🪑️objects/market-parasol/🔣️.json", import.meta.url), "utf8")).asset;
  const solid = asset.volumes[50].body.find((s: { id: string }) => s.id === "panel-right");
  const weights = [{ bone: "rib-right", weight: .25 }, { bone: "rib-b", weight: .75 }];
  solid.weights = weights;
  asset.representations[50].geometry.elevation.find((s: { id: string }) => s.id === solid.id).solid.weights = weights;
  const { bones, shapes } = evaluate(asset, { kind: "pose", pose: "furled" }, { kind: "scale", scale: { denominator: 50 } });
  const rest = restFrames(asset.rig);
  const asMatrix = (frame: FrameLike) => {
    const { o, u, v, w } = worldFrameOf(frame);
    return new Matrix4().makeBasis(new Vector3(...u), new Vector3(...v), new Vector3(...w)).setPosition(...o);
  };
  const palette = weights.map(({bone,weight}) => ({ weight, transform: asMatrix(bones[bone]!.frame).multiply(asMatrix(rest[bone]!.frame).invert()).multiply(asMatrix(rest[solid.bone]!.frame)) }));
  const expected = new Box3();
  for (const [z,x] of solid.profile) for (const y of [solid.from,solid.to]) {
    const point = new Vector3();
    for (const { weight, transform } of palette) point.addScaledVector(new Vector3(x,y,z).applyMatrix4(transform),weight);
    expected.expandByPoint(point);
  }
  const mesh = worldSolids(asset, bones, 50, "body").find(s => s.id === solid.id)!;
  const actual = new Box3();
  for (let i = 0; i < mesh.positions.length; i += 3) actual.expandByPoint(new Vector3().fromArray(mesh.positions,i));
  expect(actual.min.distanceTo(expected.min) + actual.max.distanceTo(expected.max)).toBeLessThan(1e-6);
  const rings = shapes.elevation.find(s => s.shape.id === solid.id)!.rings;
  const projected = new Box3().setFromPoints(rings.flatMap(r => r.map(p => new Vector3(p.x,p.y,0))));
  expect(projected.max.x).toBeCloseTo(expected.max.x,6);
  expect(projected.min.y).toBeCloseTo(expected.min.y,6);
});

it("retains the measured horn stations at print scales and detects deformed substitutes", () => {
  const goat = JSON.parse(readFileSync(new URL("../../🗂️catalog/🐕️animals/goat/🔣️.json", import.meta.url), "utf8")).asset;
  const fine = goat.volumes[50].body.find((s: VolumeSolid) => s.id === "horn-n");
  const coarse = goat.volumes[500].body.find((s: VolumeSolid) => s.id === "horn-n");
  const index = goat.volumes[500].body.indexOf(coarse);
  const findings = () => featureRule(goat,[500]).findings.filter(f => f.path === `volumes.500.body[${index}]`);
  const point = (xyz: [number,number,number]) => new Vector3(...xyz);
  expect(point(coarse.sections[0].at).distanceTo(point(fine.sections[0].at))).toBe(0);
  expect(point(coarse.sections.at(-1).at).distanceTo(point(fine.sections.at(-1).at))).toBe(0);
  expect(coarse.sections.length).toBeLessThan(fine.sections.length);
  coarse.id = "near-horn";
  expect(findings()).toHaveLength(0);
  const station = coarse.sections[1];
  const radius = station.rx;
  station.rx *= 0.5;
  expect(findings()).toHaveLength(1);
  station.rx = radius;
  coarse.transform = { translation: [0.01,0,0] };
  expect(findings()).toHaveLength(1);
  delete coarse.transform;
  coarse.material = "short-coat";
  expect(findings()).toHaveLength(1);
});

it("measures the load-bearing and working sections of the household models at every scale", () => {
  const fixture = new URL("../../🧫️fixtures/🧊️catalog-silhouette/🔣️.json",import.meta.url);
  const {workingDimensions} = JSON.parse(readFileSync(fixture,"utf8")) as {workingDimensions: {asset:string;solid:string;axis:number;metres:number;at?:Record<string,number>}[]};
  const assets = new Map<string,Asset>();
  for (const expected of workingDimensions) {
    let asset = assets.get(expected.asset);
    if (!asset) {
      asset = JSON.parse(readFileSync(new URL(`../../🗂️catalog/${expected.asset}/🔣️.json`,import.meta.url),"utf8")).asset as Asset;
      assets.set(expected.asset,asset);
    }
    for (const scale of [50,100,200,500] as const) {
      const volume = asset.volumes?.[scale];
      if (volume?.status !== "authored") throw new Error(`Missing 1:${scale} volume of ${expected.asset}`);
      const piece = volume.body.find(s => s.id === expected.solid);
      if (!piece) throw new Error(`Missing ${expected.solid} at 1:${scale}`);
      const mesh = copyMesh(piece,{chordM:.0002,minSegments:32,maxSegments:32});
      const width = new Box3().setFromArray(mesh.positions).getSize(new Vector3()).getComponent(expected.axis);
      expect(width,`${expected.asset} ${piece.id} at 1:${scale}`).toBeCloseTo(expected.at?.[scale] ?? expected.metres,5);
    }
  }
});
