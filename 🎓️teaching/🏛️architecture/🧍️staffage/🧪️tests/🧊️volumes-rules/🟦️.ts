/** 🧊️ The rules of the per-scale, per-mode volumes: `checkVolumes` and `measureVolumes` on the sample figure, with one invalid vector per rule and independent oracles for the geometry they measure.
 *
 * THE REFERENCES are the hand-computed volumes, areas and silhouettes of the cuboids of the fixture, and shapely (unions and Hausdorff distances of projected triangles) and trimesh (volumes by point containment,
 * outer areas) in `🐍️.py`, fed with the very triangles of the kernel. The thresholds are pinned to the design (`../../🧫️fixtures/🧊️volumes-rules/🔣️.json`).
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️volumes-rules/🔣️.json — the vectors */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { Box3, BoxGeometry, ExtrudeGeometry, Matrix4, Quaternion, Shape, SphereGeometry, Triangle, Vector2, Vector3 } from "three";
import type { Asset, AuthoredVolume, ScaleDenominator } from "../../🧬️schema/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { VOLUME_LIMITS, VOLUME_RULES, bandVolumes, checkVolumes, compareSolidLists, contactPlanes, exposedArea, fixedTriangleCount, isClosed, featureOf, measureVolumes, outerArea, silhouetteAgreement, unionVolume, type VolumeRule } from "../../🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { insideAny } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { centreLineOf } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts";
import { minimumWidth, slendernessOf, sortedExtents } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { CUT, Mask, bodyOutline, projectedBodyDistance, TILE_CELLS, extentOf, measureAsset, measureView, referenceOf, worldSolids, type World } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Operation = { op: "replace" | "add" | "remove"; path: string; value?: unknown };
type Boxes = { centre: number[]; size: number[]; sectionFaces?: string[] }[];
type Vectors = {
  ellipticMember: {radiusXM:number;radiusYM:number;lengthM:number;azimuthDeg:number};
  subpixelShaft: { triangle: number[][]; points: number[][]; degenerate: number[][]; probeRadiusM: number };
  totalSurface: { stockM: [number, number, number]; areaM2: number; wrongAreaM2: number };
  breathApertures: { diameterM: number; pigeonRadiiM: [number, number, number]; ovalProfileM: [number, number][]; normalThicknessM: number; cases: { kind: string; variant: string; finding: boolean }[] };
  areaBasis: { name: string; undeclared: Operation[]; declared: Operation[] };
  outlines: { name: string; scale: ScaleDenominator; patch: Operation[]; measures: string[] };
  exposed: { name: string; boxes: Boxes; target: number; planes?: { point: number[]; normal: number[]; toleranceM: number }[]; area: number }[];
  bands: { excavator: { bar: { size: number[] }; shortenM: number; distanceM: number }; cell: number; distance: number; cases: { name: string; fine: { centre: number[]; size: number[] }[]; coarse: { centre: number[]; size: number[] }[]; eroded: number; dilated: number; fineVolume: number }[] };
  winding: { cell: number; outer: number; area: number; evenOddArea: number };
  envelopeArea: { pocketFraction: number; name: string; scale: ScaleDenominator; region: string; patch: Operation[]; fineScale: number; finePatch: Operation[] };
  retained: { cell: number; toleranceM2: number; cases: { view: "elevation" | "front" | "plan"; axis: number; far: number; near: number }[]; sampleSection: { scale: number; view: "elevation" | "front" | "plan"; plane: number; label: { en: string; de: string } } };
  emitters: { scale: ScaleDenominator; region: string; solid: string; untagged: Operation[]; removed: Operation[] };
  stroke: { pen: number; cases: { name: string; size: number[]; centres: number[][]; area: number }[] };
  burial: { table: { centre: number[]; size: number[] }; laptop: { centre: number[]; size: number[] }; area: number; probeM: number; toleranceM2: number };
  outer: { solids: { centre: number[]; size: number[] }[]; area: number; toleranceFraction: number };
  porous: { scale: ScaleDenominator; patch: Operation[] };
  sheetStock: { spanM: number; depthM: number; clothM: number; narrowM: number; inflatedM: number };
  sweptRod: { radiusM: number; stationsM: [number, number, number][]; lengthM: number; diameterM: number };
  hollow: { solid: unknown; smallestM: number };
  follow: { bar: { centre: number[]; size: number[] }; cases: { name: string; material?: string; drawn: { bone: string; material?: string; closed: boolean; filled: boolean; rings: number[][][]; penM: number }[]; widthM: number }[]; hoop: { radiusM: number; tubeM: number; penM: number; solid: { kind: "revolve"; profile: number[][] }; outline: number[][] }; bodyEdge: { plate: { centre: number[]; size: number[] }; wire: { centre: number[]; size: number[] }; outline: number[][]; penM: number; edgeAreaM2: number; penAreaM2: number; voidAreaM2: number; rects: number[][] } };
  minimumWidth: { cases: { name: string; points: number[][]; widthM: number }[] };
  tangent: { cell: number; roof: { centre: number[]; size: number[] }; cases: { name: string; bar: { centre: number[]; size: number[] }; gapM: number; rowCells: number; closed: boolean }[] };
  envelope: { bars: { centre: number[]; size: number[] }[]; prism: { centre: number[]; size: number[] }; feature: number };
  viewsPosition: { scale: number; axis: string; elevationM: number[]; widthM: number; limitM: number; cases: { name: string; shiftM: number; finding: boolean }[] };
  typified: { rods: { count: number; first: number; pitch: number; size: number[] }; posts: { count: number; first: number; pitch: number; size: number[] }; feature: number; hullIou: number };
  support: { missing: { name: string; rule: VolumeRule; patch: Operation[] }; wrongAnchor: { name: string; code: string; patch: Operation[] }; valid: { name: string; patch: Operation[] } };
  rules: string[];
  limits: Record<string, unknown>;
  baselinePasses: Record<string, number[]>;
  baselineCovered: Record<string, number[]>;
  invalid: { name: string; rule: VolumeRule; scale: ScaleDenominator; measure?: string; patch: Operation[] }[];
  copied: { valid: { name: string; scales: number[]; patch: Operation[] }; referencesOnly: { name: string; scale: number; patch: Operation[] }; lists: { fine: unknown[]; cases: { name: string; kind: string; denominator: number; fineDenominator: number; coarse: unknown[] }[] } };
  boxes: { cell: number; cases: { name: string; boxes: { centre: number[]; size: number[] }[]; unionVolume: number; meshVolume: number; outerArea: number | null; view: "elevation" | "front" | "plan"; silhouetteArea: number }[] };
};
type Oracle = Record<string, { unionVolume?: number; meshVolume: number; outerArea?: number; triangles: number; iou?: number; hausdorff?: number; area?: number }>;

const vectors = fixture<Vectors>("🧊️volumes-rules");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️volumes-rules/🐍️.py");

function patch(document: any, operations: readonly Operation[]): any {
  const root = structuredClone(document);
  for (const { op, path, value } of operations) {
    const segments = path.split("/").slice(1);
    let node = root;
    const at = (n: any, s: string) => (Array.isArray(n) ? (s.startsWith("id=") ? n.findIndex((i: any) => i.id === s.slice(3)) : Number(s)) : s);
    for (const segment of segments.slice(0, -1)) node = node[at(node, segment)];
    const key = at(node, segments[segments.length - 1]!);
    if (op === "remove") Array.isArray(node) ? node.splice(key as number, 1) : delete node[key];
    else if (op === "add" && Array.isArray(node)) node.push(structuredClone(value));
    else node[key] = structuredClone(value);
  }
  return root;
}

function cuboid(id: string, centre: readonly number[], size: readonly number[]): World {
  const [hx, hy, hz] = size.map((s) => s / 2) as [number, number, number];
  const corners = [-1, 1].flatMap((x) => [-1, 1].flatMap((y) => [-1, 1].map((z) => [centre[0]! + x * hx, centre[1]! + y * hy, centre[2]! + z * hz])));
  const faces = [[0, 1, 3, 2], [4, 6, 7, 5], [0, 4, 5, 1], [2, 3, 7, 6], [0, 2, 6, 4], [1, 5, 7, 3]];
  const indices = faces.flatMap(([a, b, c, d]) => [a!, b!, c!, a!, c!, d!]);
  return { id, positions: Float64Array.from(corners.flat()), indices: Uint32Array.from(indices) };
}

const mesh = (w: World) => ({ positions: [...w.positions], indices: [...w.indices] });
const relative = (value: number, reference: number): number => Math.abs(value - reference) / Math.abs(reference);

const mechanical = patch(sample, (vectors as unknown as { mechanical: { patch: Operation[] } }).mechanical.patch) as Asset;
const baseline = measureVolumes(sample);
const baselineFindings = checkVolumes(sample);
const value = (list: readonly { name: string; scale?: number; value: number }[], name: string, scale: number): number | undefined => list.find((m) => m.name === name && m.scale === scale)?.value;

describe("🧊️ the limits and rules of the volumes are those of the design", () => {
  it("limits closed-solid containment to the relevant component of a translated assembly", () => {
    type Point = [number, number, number];
    const { boxes, samples, copies } = fixture<{ closedPartBounds: {
      boxes: number[][]; samples: { at: Point; skip: number[]; inside: boolean; tested: number[] }[];
      copies: { count: number; stepY: number; samples: Point[] };
    } }>("🧊️volumes-rules").closedPartBounds;
    const bounds = boxes.map(v => new Box3(new Vector3().fromArray(v), new Vector3().fromArray(v, 3)));
    const solids = bounds.map((box, id) => {
      const size = box.getSize(new Vector3()), centre = box.getCenter(new Vector3());
      const g = new BoxGeometry(size.x, size.y, size.z).translate(centre.x, centre.y, centre.z);
      return { id: String(id), positions: Float64Array.from(g.attributes.position!.array), indices: Uint32Array.from(g.index!.array) };
    });
    for (const q of samples) {
      const visited: number[] = [];
      expect(bounds.some((box, i) => !q.skip.includes(i) && box.containsPoint(new Vector3(...q.at)))).toBe(q.inside);
      expect(insideAny(solids, q.at, (_, i) => { visited.push(i); return q.skip.includes(i); })).toBe(q.inside);
      expect(visited).toEqual(q.tested);
    }
    const base = solids[0]!, stride = base.positions.length;
    const positions = new Float64Array(stride * copies.count), indices = new Uint32Array(base.indices.length * copies.count);
    for (let k = 0; k < copies.count; k++) {
      for (let i = 0; i < stride; i++) positions[k * stride + i] = base.positions[i]! + (i % 3 === 1 ? k * copies.stepY : 0);
      for (let i = 0; i < base.indices.length; i++) indices[k * base.indices.length + i] = base.indices[i]! + k * stride / 3;
    }
    const merged = { id: "rungs", positions, indices };
    const separate = { ...merged, separateCopies: { coordinatesPerCopy: stride, indices: base.indices } };
    for (const q of copies.samples) {
      const inside = Array.from({length: copies.count}, (_, i) => bounds[0]!.containsPoint(new Vector3(q[0], q[1] - i * copies.stepY, q[2]))).some(Boolean);
      expect(insideAny([merged], q)).toBe(inside);
      expect(insideAny([separate], q)).toBe(inside);
    }
  });

  it("lists the fourteen rule ids and the thresholds of the design, unweakened", () => {
    expect([...VOLUME_RULES]).toEqual(vectors.rules);
    expect(JSON.parse(JSON.stringify(VOLUME_LIMITS))).toEqual(vectors.limits);
    expect(featureOf(50)).toBeCloseTo(0.0075, 9);
    expect(featureOf(100)).toBeCloseTo(0.015, 9);
    expect(featureOf(200)).toBeCloseTo(0.03, 9);
    expect(featureOf(500)).toBeCloseTo(0.075, 9);
  });

  it("gives every finding a rule, a path, an English and a German message and the failing measurements", () => {
    for (const rule of VOLUME_RULES) expect(typeof rule).toBe("string");
    expect(baselineFindings.length).toBeGreaterThan(0);
    for (const finding of baselineFindings) {
      expect(VOLUME_RULES).toContain(finding.rule);
      expect(/^(volumes|representations)\./.test(finding.path)).toBe(true);
      expect(finding.message.en.length * finding.message.de.length).toBeGreaterThan(0);
      for (const m of finding.measurements) expect(m.ok).toBe(false);
    }
  });
});

describe("🧊️ the rules that the sample figure satisfies", () => {
  for (const [rule, scales] of Object.entries(vectors.baselinePasses))
    it(`${rule} reports nothing at 1:${scales.join(", 1:")}`, () => {
      for (const scale of scales) expect(baselineFindings.filter((f) => f.rule === rule && f.scale === scale), `${rule} 1:${scale}`).toEqual([]);
    });

  for (const [rule, scales] of Object.entries(vectors.baselineCovered))
    it(`${rule} covers every item at 1:${scales.join(", 1:")}`, () => {
      for (const scale of scales) expect(value(baseline.filter((m) => m.rule === rule), "uncoveredItems", scale), `${rule} 1:${scale}`).toBe(0);
    });

  it("reports the measurement behind every limit, so that authors see how far off they are", () => {
    const iou = baseline.find((m) => m.rule === "volumes-silhouette" && m.name === "iou:elevation" && m.scale === 100)!;
    expect(iou.limit).toBe(0.9);
    expect(iou.bound).toBe("min");
    expect(iou.ok).toBe(iou.value >= 0.9);
    expect(baseline.some((m) => m.rule === "volumes-silhouette" && m.name.startsWith("p95M:") && m.limit === 2 * featureOf(100))).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-cross-scale" && m.name === "unionVolumeBandExcess" && m.limit === 0)).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-monotonic" && m.name === "triangleCountDelta")).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-thermal-coverage" && m.name.startsWith("areaDeviation:") && m.scale === 50 && m.limit === 0.3)).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-thermal-coverage" && m.name.startsWith("areaDeviation:") && m.scale === 100 && m.limit === 0.3)).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-thermal-coverage" && m.name === "areaDeviationTotal" && m.scale === 200 && m.limit === 0.3)).toBe(true);
    expect(baseline.some((m) => m.rule === "volumes-thermal-coverage" && m.name.startsWith("areaDeviation:") && m.scale === 500 && m.ok)).toBe(true);
  });

  it("is also what validateAsset reports for the analytic rules", () => {
    expect(validateAsset(sample).filter((issue) => ["volumes-availability", "volumes-feature", "volumes-thermal-coverage", "volumes-structure-coverage"].includes(issue.code))).toEqual([]);
  });
});

describe("🧊️ one invalid vector per rule", () => {
  it("has a vector for every rule that has a broken variant", () => {
    expect(new Set(vectors.invalid.map((c) => c.rule))).toEqual(new Set(VOLUME_RULES));
  });

  it.each(vectors.invalid)("$rule: $name", ({ rule, scale, measure, patch: operations }) => {
    const broken = patch((vectors.invalid.find((v) => v.patch === operations) as { base?: string }).base === "mechanical" ? mechanical : sample, operations) as Asset;
    const found = checkVolumes(broken, { scale, rules: [rule] });
    const before = baselineFindings.filter((f) => f.rule === rule && f.scale === scale);
    expect(found.filter((f) => f.rule === rule && f.scale === scale).length, found.map((f) => f.message.en).join("\n")).toBeGreaterThan(measure === undefined ? 0 : before.length - 1);
    if (measure === undefined) expect(found.length).toBeGreaterThan(0);
    for (const f of found) {
      expect(f.message.en.length * f.message.de.length).toBeGreaterThan(0);
      expect(f.rule).toBe(rule);
    }
    if (measure !== undefined) {
      const now = measureVolumes(broken, { scale, rules: [rule] }).find((m) => m.name === measure && m.scale === scale)!;
      const was = baseline.find((m) => m.name === measure && m.scale === scale && m.rule === rule)!;
      expect(now.ok, `${measure} ${now.value} against ${now.limit}`).toBe(false);
      if (was !== undefined) expect(now.bound === "min" ? now.value < was.value : now.value > was.value, `${measure}: ${was.value} -> ${now.value}`).toBe(true);
    }
  });

  it("reports the applied fine-scale body area tolerance in both languages", () => {
    const broken = patch(sample, [{ op: "replace", path: "/thermal/regions/id=feet/areaM2", value: 2 }]) as Asset;
    const finding = checkVolumes(broken, { scale: 50, rules: ["volumes-thermal-coverage"] }).find((f) => f.measurements.some((m) => m.name === "areaDeviation:feet"));
    expect(finding?.measurements[0]?.limit).toBe(0.3);
    expect(finding?.message.en).toContain("limit 30 %");
    expect(finding?.message.de).toContain("Grenze 30 %");
  });

  it("only reports the rules asked for and only the scale asked for", () => {
    const only = checkVolumes(mechanical, { scale: 500, rules: ["volumes-bones", "volumes-bounds"] });
    expect(only.every((f) => f.scale === 500 && (f.rule === "volumes-bones" || f.rule === "volumes-bounds"))).toBe(true);
    expect(only.some((f) => f.rule === "volumes-bones")).toBe(true);
  });
});

describe("🧊️ volumes-copied, the 3D twin of representation-copied", () => {
  it.each(vectors.copied.lists.cases)("$name is $kind", ({ kind, coarse, denominator, fineDenominator }) => {
    expect(compareSolidLists(coarse as never, vectors.copied.lists.fine as never, denominator, fineDenominator).kind).toBe(kind);
  });

  it("flags the sample figure, whose scales are mechanical filtered copies, in every list with the finer scale", () => {
    const found = checkVolumes(mechanical, { rules: ["volumes-copied"] });
    expect(found.some((f) => f.scale === 100 && f.list === "body" && f.data?.kind === "subset")).toBe(true);
    expect(found.some((f) => f.scale === 100 && f.list === "structure" && f.data?.kind === "identical")).toBe(true);
    for (const f of found) expect(f.message.en.length * f.message.de.length).toBeGreaterThan(0);
    expect(validateAsset(mechanical).some((issue) => issue.code === "volumes-copied")).toBe(true);
    expect(validateAsset(sample)).toEqual([]);
  });

  it(`accepts: ${vectors.copied.valid.name}`, () => {
    const fixed = patch(sample, vectors.copied.valid.patch) as Asset;
    expect(checkVolumes(fixed, { rules: ["volumes-copied"] })).toEqual([]);
  });

  it(`does not judge: ${vectors.copied.referencesOnly.name}`, () => {
    const only = patch(sample, vectors.copied.referencesOnly.patch) as Asset;
    expect(checkVolumes(only, { scale: 200, rules: ["volumes-copied"] }).filter((f) => f.list === "thermal")).toEqual([]);
  });
});

describe("🧊️ exposed thermal area (C1), bands (C3), winding and tiling (C7), supports (C6), instancing and fixed quality (C4)", () => {
  const FRAME = { u: { x: 1, y: 0, z: 0 }, v: { x: 0, y: 1, z: 0 }, w: { x: 0, y: 0, z: 1 } };
  const worlds = (boxes: Boxes): World[] => boxes.map((b, i) => ({ ...cuboid(`b${i}`, b.centre, b.size), frame: FRAME, ...(b.sectionFaces === undefined ? {} : { sectionFaces: b.sectionFaces }) }));

  it.each(vectors.exposed)("$name: the exposed area is the hand value", ({ boxes, target, planes, area }) => {
    const solids = worlds(boxes);
    const own = exposedArea([solids[target]!], solids, (planes ?? []).map((p) => ({ point: p.point as never, normal: p.normal as never, toleranceM: p.toleranceM })), (other) => other === solids[target]);
    expect(Math.abs(own - area), `${own} against ${area}`).toBeLessThan(1e-9);
  });

  it("the unit cube is eroded and dilated by the distance as the hand formula says", () => {
    for (const c of vectors.bands.cases) {
      const toWorld = (list: { centre: number[]; size: number[] }[]) => list.map((b, i) => cuboid(`b${i}`, b.centre, b.size));
      const r = bandVolumes(toWorld(c.fine), toWorld(c.coarse), vectors.bands.distance);
      expect(relative(r.fine, c.fineVolume), "fine").toBeLessThan(0.03);
      expect(relative(r.eroded, c.eroded), "eroded").toBeLessThan(0.08);
      expect(relative(r.dilated, c.dilated), "dilated").toBeLessThan(0.05);
    }
  });

  it("fills a self-crossing polygon by the non-zero winding rule", () => {
    const { cell, outer, area } = vectors.winding;
    const star = [0, 2, 4, 1, 3].map((k) => ({ x: outer * Math.sin((2 * Math.PI * k) / 5), y: outer * Math.cos((2 * Math.PI * k) / 5) }));
    const mask = new Mask({ minX: -1.2, minY: -1.2, maxX: 1.2, maxY: 1.2 }, cell);
    mask.fill([star]);
    const filled = mask.cells.reduce((sum, x) => sum + x, 0) * cell * cell;
    expect(relative(filled, area)).toBeLessThan(0.01);
  });

  it("compares an envelope prism with the convex hull of the bars it replaces", () => {
    const { bars, prism, feature } = vectors.envelope;
    const lattice = bars.map((b, i) => ({ ...cuboid(`bar${i}`, b.centre, b.size), bone: "mast" }));
    const envelope = [{ ...cuboid("prism", prism.centre, prism.size), bone: "mast", envelope: true }];
    const plain = silhouetteAgreement(envelope, lattice, "elevation", feature);
    const hulled = silhouetteAgreement(envelope, lattice, "elevation", feature, 0, { hullBones: new Set(["mast"]) });
    expect(plain.iou).toBeLessThan(0.7);
    expect(hulled.iou).toBeGreaterThan(0.98);
  });

  it("compares a typified row with the row it stands for through their common envelope", () => {
    const { rods, posts, feature, hullIou } = vectors.typified;
    const row = (prefix: string, r: { count: number; first: number; pitch: number; size: number[] }, envelope: boolean): World[] => Array.from({ length: r.count }, (_, k) => ({ ...cuboid(`${prefix}${k}`, [r.first + k * r.pitch, r.size[1]! / 2, 0], r.size), bone: "balustrade", ...(envelope ? { envelope: true } : {}) }));
    const [fine, coarse] = [row("rod", rods, false), row("post", posts, true)];
    const plain = silhouetteAgreement(coarse, fine, "elevation", feature);
    const hulled = silhouetteAgreement(coarse, fine, "elevation", feature, 0, { hullBones: new Set(["balustrade"]) });
    expect(plain.rawIou).toBeLessThan(0.5);
    expect(plain.p95M).toBeGreaterThan(VOLUME_LIMITS.crossDistanceFeatures * feature * 0.5);
    expect(Math.abs(hulled.rawIou - hullIou)).toBeLessThan(0.01);
    expect(hulled.p95M).toBeLessThanOrEqual(hulled.cellM);
    const undeclared = silhouetteAgreement(row("post", posts, false), fine, "elevation", feature, 0, { hullBones: new Set(["balustrade"]) });
    expect(undeclared.rawIou).toBeLessThan(0.5);
    const directory = mkdtempSync(join(tmpdir(), "staffage-typified-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases: [{ name: "typified", meshes: coarse.map(mesh), other: fine.map(mesh), view: "elevation", hull: true }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [input])) as Record<string, { iou: number; iouHull: number }>;
      expect(Math.abs(reference.typified!.iouHull - hullIou), "shapely convex hulls").toBeLessThan(1e-6);
      expect(reference.typified!.iou, "shapely rows").toBeLessThan(0.5);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("measures the same IoU and distances in tiles as on one raster", () => {
    const [a, b] = [cuboid("a", [0, 0, 0], [3, 2, 1]), cuboid("b", [0.3, 0.1, 0], [3, 2, 1])];
    const one = silhouetteAgreement([a], [b], "elevation", 0.2);
    const tiled = silhouetteAgreement([a], [b], "elevation", 0.2, 0, { tileCells: 40 });
    expect(Math.abs(one.iou - tiled.iou)).toBeLessThan(1e-9);
    expect(Math.abs(one.p95M - tiled.p95M)).toBeLessThanOrEqual(one.cellM);
    expect(TILE_CELLS).toBeGreaterThan(100);
  });

  it("measures a view of the sample the same in tiles", () => {
    const pose = sample.presentation.defaultPose;
    const state = evaluate(sample, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 100 } });
    const solids = worldSolids(sample, state.bones as never, 100);
    const [whole, tiles] = [measureView(sample, state, solids, "elevation", 100, undefined, 0.015), measureView(sample, state, solids, "elevation", 100, undefined, 0.015, 60)];
    expect(Math.abs(whole.iou - tiles.iou)).toBeLessThan(0.01);
    expect(Math.abs(whole.p95M - tiles.p95M)).toBeLessThanOrEqual(2 * whole.cell);
  });

  it("shares reference geometry and preserves thermal area across separated and touching translated boxes", () => {
    const volume = sample.volumes["50"] as AuthoredVolume;
    const state = evaluate(sample, { kind: "pose", pose: sample.presentation.defaultPose });
    for (const spacing of [0.1, 0.2]) {
      const first = volume.body[0]!;
      const box: AuthoredVolume["body"][number] = { id: "links", bone: first.bone, material: first.material, role: "body", tone: 0.5, kind: "box", size: [0.1, 0.1, 0.1], repeat: { count: 38, translation: [spacing, 0, 0] } };
      const source: Asset = { ...sample, volumes: { ...sample.volumes, "50": { ...volume, body: [box], thermal: [{ id: "outer", of: "links", region: "skin" }] } } };
      const body = worldSolids(source, state.bones, 50);
      const thermal = worldSolids(source, state.bones, 50, "thermal", body);
      const uncut = referenceOf("elevation", body, undefined);
      expect(uncut.full).toBe(uncut.cut);
      expect(uncut.visible).toBe(uncut.cut);
      const section = referenceOf("elevation", body, { plane: 0, retained: "far", label: { en: "Section", de: "Schnitt" } });
      expect(section.full.length).toBe(body[0]!.indices.length / 3);
      expect(thermal[0]!.positions).toBe(body[0]!.positions);
      expect(thermal[0]!.indices).toBe(body[0]!.indices);
      expect(Boolean(body[0]!.separateCopies)).toBe(spacing > 0.1);
      const expanded = { id: "expanded", positions: body[0]!.positions, indices: body[0]!.indices, frame: body[0]!.frame };
      expect(isClosed(body[0]!)).toBe(isClosed(expanded));
      expect(outerArea(thermal)).toBeCloseTo(outerArea([expanded]), 10);
      const triangle = new Triangle();
      let threeArea = 0;
      for (let i = 0; i < expanded.indices.length; i += 3) {
        triangle.a.fromArray(expanded.positions, 3 * expanded.indices[i]!);
        triangle.b.fromArray(expanded.positions, 3 * expanded.indices[i + 1]!);
        triangle.c.fromArray(expanded.positions, 3 * expanded.indices[i + 2]!);
        threeArea += triangle.getArea();
      }
      expect(outerArea(thermal)).toBeCloseTo(threeArea, 10);
      expect(outerArea(thermal)).toBeCloseTo(38 * 0.06, 10);
    }
  });

  it("instances the copies of a repeat exactly as the kernel tessellates them", () => {
    const pose = sample.presentation.defaultPose;
    const state = evaluate(sample, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 50 } });
    const body = (sample.volumes["50"] as AuthoredVolume).body;
    const solid = body.find((x) => x.repeat !== undefined)!;
    const world = worldSolids(sample, state.bones, 50).find((entry) => entry.id === solid.id)!;
    const reference = tessellateSolid({ ...solid, node: solid.bone } as never, qualityOf(sample));
    const { o, u, v, w } = state.bones[solid.bone]!.frame;
    const basis = new Matrix4().makeBasis(new Vector3(u.x, u.y, u.z), new Vector3(v.x, v.y, v.z), new Vector3(w.x, w.y, w.z)).setPosition(o.x, o.y, o.z);
    const expected = new Vector3();
    const actual = new Vector3();

    expect(world.indices.length).toBeGreaterThan(0);
    expect(world.indices).toEqual(Uint32Array.from(reference.indices));
    expect(world.positions.length).toBe(reference.positions.length);
    for (let i = 0; i < reference.positions.length; i += 3) {
      expected.fromArray(reference.positions, i).applyMatrix4(basis);
      actual.fromArray(world.positions, i);
      expect(actual.distanceTo(expected)).toBeLessThan(1e-9);
    }
  });

  it("counts triangles at one fixed quality whatever the radius", () => {
    const capsule = (r: number) => ({ id: "c", kind: "capsule", bone: "b", material: "m", role: "body", tone: 0.5, p0: [0, 0, 0], p1: [1, 0, 0], r0: r, r1: r }) as never;
    expect(fixedTriangleCount([capsule(0.05)])).toBe(fixedTriangleCount([capsule(0.5)]));
  });

  it("requires exactly one support solid per support anchor and checks the anchor name", () => {
    const missing = patch(sample, vectors.support.missing.patch) as Asset;
    expect(checkVolumes(missing, { scale: 100, rules: ["volumes-structure-coverage"] }).some((f) => f.data?.anchor === "contact-l")).toBe(true);
    const wrong = patch(sample, vectors.support.wrongAnchor.patch) as Asset;
    expect(validateAsset(wrong).some((issue) => issue.code === vectors.support.wrongAnchor.code)).toBe(true);
    const valid = patch(sample, vectors.support.valid.patch) as Asset;
    expect(checkVolumes(valid, { rules: ["volumes-structure-coverage"] }).filter((f) => f.data?.anchor !== undefined)).toEqual([]);
  });

  it("checks the physical extent of a declared physiological breath aperture", () => {
    const v = vectors.breathApertures;
    for (const c of v.cases) {
      const a = structuredClone(sample);
      const rep = a.representations["50"];
      if (rep.status !== "authored") throw Error("expected print drawing");
      const emitter = a.thermal.emitters.find(e => e.id === "nose-breath")!;
      Object.assign(emitter, { diameterM: v.diameterM, kind: c.variant === "device" ? "vent" : "breath" });
      if (c.variant === "unused-breath") Object.assign(a.thermal,{respiration:undefined});
      const anchor = a.anchors.find(x => x.id === emitter.anchor)!;
      const source = { id: "outlet-test", bone: c.variant === "wrong-bone" ? "torso" : anchor.bone, role: "body", material: "skin", tone: 0.5,
        region: c.variant === "non-tissue" ? "unknown" : "head", tags: c.variant === "untagged" ? [emitter.id] : ["emitter", c.variant === "unknown" ? "other-outlet" : emitter.id],
        transform: { translation: [anchor.x, anchor.y, c.variant === "away-from-surface" ? 1 : 0] } };
      const radii = c.variant === "bead" ? [v.diameterM/2,v.diameterM/2,v.diameterM/2] : v.pigeonRadiiM.map(x => c.variant === "undersized" ? x/2 : x);
      const solid = c.kind === "ellipsoid" ? { ...source, kind: c.kind, radii } : { ...source, kind: c.kind, plane: "xy", profile: v.ovalProfileM, from: 0, to: c.variant === "longitudinal" ? 0.02 : v.normalThicknessM };
      const list = c.variant === "body-list" ? "body" : "thermal";
      const skin = {id:"outlet-skin",kind:"box",bone:anchor.bone,role:"body",material:"skin",tone:0.5,region:"head",size:[0.02,0.02,0.002], transform:{translation:[anchor.x,anchor.y,-0.001]}};
      const asset = patch(a,[{op:"add",path:"/volumes/50/thermal/-",value:skin},{op:"add",path:"/volumes/50/"+list+"/-",value:solid}]) as Asset;
      const failures = checkVolumes(asset,{scale:50,rules:["volumes-feature"]}).filter(f => f.message.en.includes('"outlet-test"'));
      expect(failures.length,c.kind+" "+c.variant).toBe(Number(c.finding));
      if (c.variant === "physical") {
        const mesh = c.kind === "ellipsoid" ? new SphereGeometry(1,32,16).scale(...v.pigeonRadiiM) : new ExtrudeGeometry(new Shape(v.ovalProfileM.map(([x,y])=>new Vector2(x,y))),{depth:v.normalThicknessM,bevelEnabled:false});
        mesh.computeBoundingBox();
        const size = mesh.boundingBox!.getSize(new Vector3());
        expect(Math.max(size.x,size.y)).toBeCloseTo(v.diameterM,8);
        expect(size.z).toBeCloseTo(v.normalThicknessM,8);
        mesh.dispose();
      }
    }
  });

  it("measures a slender member against its independently specified elliptical section", () => {
    const f=vectors.ellipticMember, b=sample.rig.bones[1]!, material=sample.materials[0]!.id;
    const angle=f.azimuthDeg*Math.PI/180, radius=Math.hypot(f.radiusXM*Math.sin(angle),f.radiusYM*Math.cos(angle));
    const shape=new Shape();
    shape.absellipse(0,0,f.radiusXM,f.radiusYM,0,Math.PI*2,false,angle);
    const geometry=new ExtrudeGeometry(shape,{depth:f.lengthM,curveSegments:180,bevelEnabled:false});
    const bounds=new Box3().setFromBufferAttribute(geometry.attributes.position! as never);
    expect(bounds.max.y).toBeCloseTo(radius,5);
    const area=Math.PI*f.radiusXM*f.radiusYM;
    const q=new Quaternion().setFromAxisAngle(new Vector3(0,1,0),Math.PI/2).multiply(new Quaternion().setFromAxisAngle(new Vector3(0,0,1),angle));
    const stations=[0,f.lengthM].map(x=>({at:[x,0,0],rx:f.radiusXM,ry:f.radiusYM,rotation:q.toArray()}));
    const shaft={id:"physical-shaft",member:b.id,bone:b.id,kind:"loft",material,part:"member",role:"body",tone:.9,sections:stations,capStart:true,capEnd:true};
    const section={material,areaM2:area,secondMomentM4:area*radius*radius/4,extremeFibreM:radius};
    for(const [change,want] of [[{},0],[{areaM2:area*.4},1],[{secondMomentM4:section.secondMomentM4*1.7},1],[{material:"other"},1]] as const) {
      const asset=patch(sample,[{op:"add",path:"/rig/bones/1/section",value:{...section,...change}},
        {op:"add",path:"/volumes/50/structure/-",value:shaft}]) as Asset;
      const issues=checkVolumes(asset,{scale:50,rules:["volumes-feature"]}).filter(x=>x.message.en.includes('"physical-shaft"'));
      expect(issues.length).toBe(want);
    }
    geometry.dispose();
  });

  it("retains the physical gauge of a broad sheet bound to its thermal material, and lets an unbound broad sheet pass only when it declares `sheet`", () => {
    const f = vectors.sheetStock;
    const region = sample.thermal.regions[0]!;
    const material = region.material;
    const bone = sample.rig.bones[1]!.id;
    const size = (depth: number, thickness: number, bound: boolean, declared = false) => {
      const profile = [[0,0],[f.spanM,0],[0,depth]];
      const sheet = {id:"canopy-sheet",kind:"extrude",bone,material,role:"body",tone:0.5,plane:"zx",profile,from:0,to:thickness,...(declared ? {sheet:true} : {})};
      const layer = { ...region.layers[0]!, id:"sheet-stock", material, thicknessM:f.clothM };
      const thermal = {id:"sheet-surface",of:sheet.id,region:region.id,layer:layer.id};
      const asset = patch(sample,[{op:"add",path:"/volumes/50/body/-",value:sheet},
        {op:"replace",path:"/thermal/regions/0/layers",value:[layer]},
        ...(bound ? [{op:"add" as const,path:"/volumes/50/thermal/-",value:thermal}] : [])]) as Asset;
      const shape = new Shape(profile.map(([x,y]) => new Vector2(x,y)));
      const geometry = new ExtrudeGeometry(shape,{depth:thickness,bevelEnabled:false});
      geometry.computeBoundingBox();
      expect(geometry.boundingBox!.getSize(new Vector3()).z).toBeCloseTo(thickness,8);
      geometry.dispose();
      return checkVolumes(asset,{scale:50,rules:["volumes-feature"]}).some(i=>i.message.en.includes('"canopy-sheet"'));
    };
    expect(size(f.depthM,f.clothM,true)).toBe(false);
    expect(size(f.depthM,f.inflatedM,true)).toBe(true);
    expect(size(f.narrowM,f.clothM,true)).toBe(true);
    expect(size(f.depthM,f.clothM,false)).toBe(true);
    expect(size(f.depthM,f.clothM,false,true)).toBe(false);
    expect(size(f.narrowM,f.clothM,false,true)).toBe(true);
    expect(size(f.depthM,f.inflatedM,true,true)).toBe(true);
  });

  it("keeps the measured tissue diameter when the drawing merges its region", () => {
    const tissue = { id: "tissue", bone: "head", kind: "ellipsoid", radii: [0.001, 0.001, 0.001], material: "skin", tone: 0.5, role: "body", region: "head" };
    const individual = patch(sample, [{ op: "add", path: "/volumes/50/thermal/-", value: tissue }]) as Asset;
    const rule = (input: Asset) => checkVolumes(input, { scale: 50, rules: ["volumes-feature"] }).filter((f) => f.message.en.includes('"tissue"'));
    expect(rule(individual)).toHaveLength(1);
    const grouped = structuredClone(individual);
    const drawing = grouped.representations["50"];
    if (drawing.status !== "authored") throw new Error("expected authored representation");
    for (const shapes of Object.values(drawing.geometry)) {
      for (const shape of shapes) {
        if (shape.bone === "head" && shape.region === "head") {
          Object.assign(shape, { region: undefined, regions: ["head", "torso"] });
        }
      }
    }
    expect(rule(grouped)).toEqual([]);
  });

  it("exempts texture and slender solids from the feature rule and bounds the rest by the drawing", () => {
    const flagged = patch(sample, [{ op: "replace", path: "/volumes/50/body/id=nose/radii", value: [0.001, 0.001, 0.001] }, { op: "add", path: "/volumes/50/body/id=nose/texture", value: true }]) as Asset;
    expect(checkVolumes(flagged, { scale: 50, rules: ["volumes-feature"] })).toEqual([]);
    const bare = patch(sample, [{ op: "replace", path: "/volumes/50/body/id=nose/radii", value: [0.001, 0.001, 0.001] }]) as Asset;
    expect(checkVolumes(bare, { scale: 50, rules: ["volumes-feature"] }).length).toBeGreaterThan(0);
  });
});

describe("🧊️ C17: an unfilled closed outline that the 3D fills counts as filled", () => {
  it("gives the same IoU and boundary distances as the filled shape", () => {
    const outlined = patch(sample, vectors.outlines.patch) as Asset;
    const [was, now] = [measureVolumes(sample, { scale: vectors.outlines.scale, rules: ["volumes-silhouette"] }), measureVolumes(outlined, { scale: vectors.outlines.scale, rules: ["volumes-silhouette"] })];
    for (const name of vectors.outlines.measures) expect(now.find((m) => m.name === name)!.value, name).toBeCloseTo(was.find((m) => m.name === name)!.value, 6);
  });
});

describe("🧊️ C20 and the defects of the tool owner", () => {
  const circle = (r: number, n = 64): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
  const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
  const kernel = (id: string, solid: Record<string, unknown>): World => {
    const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
    return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
  };

  it("measures the outer area of a ring without the disc of another region that touches or overlaps its inner cylinder (sorption wheel, C29)", () => {
    const ring = kernel("ring", { kind: "extrude", plane: "xy", profile: circle(0.31), holes: [circle(0.26).reverse()], from: -0.11, to: 0.11 });
    const alone = exposedArea([ring], [ring], [], () => false);
    expect(alone).toBeGreaterThan(0.95);
    expect(alone).toBeLessThan(1.0);
    for (const radius of [0.26, 0.27]) {
      const disc = kernel("disc", { kind: "extrude", plane: "xy", profile: circle(radius), from: -0.05, to: 0.05 });
      expect(exposedArea([ring], [ring, disc], [], (other) => other === ring)).toBeGreaterThan(0.58);
      expect(exposedArea([ring], [ring], [], () => false), `disc radius ${radius}`).toBe(alone);
    }
  });

  it("treats only closed meshes as burying, so an open skin above a solid buries nothing (pendant lamp)", () => {
    const cube = cuboid("cube", [0, 0, 0], [1, 1, 1]);
    const skin: World = { id: "skin", positions: Float64Array.from([-1, 2, -1, 1, 2, -1, 1, 2, 1, -1, 2, 1]), indices: Uint32Array.from([0, 1, 2, 0, 2, 3]) };
    expect(isClosed(cube)).toBe(true);
    expect(isClosed(skin)).toBe(false);
    expect(exposedArea([cube], [cube, skin], [], (other) => other === cube)).toBeCloseTo(6, 9);
    const loft = kernel("loft", { kind: "loft", sections: [{ at: [0, 0, 0], rx: 0.1, ry: 0.1 }, { at: [1, 0, 0], rx: 0.1, ry: 0.1 }], capStart: false, capEnd: false });
    expect(isClosed(loft)).toBe(false);
    const sampleBody = worldSolids(sample, evaluate(sample, { kind: "pose", pose: sample.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } }).bones as never, 50);
    for (const solid of sampleBody) expect(isClosed(solid), solid.id).toBe(true);
  });

  it("keeps a face 2 mm above a contact plane exposed and drops one on the plane", () => {
    const state = { anchors: { a: { anchor: { id: "a", kind: "contact" }, at: { x: 0, y: 0 }, z: 0 } } } as never;
    const planes = contactPlanes(state, featureOf(50));
    const [above, on] = [cuboid("above", [0, 0.502, 0], [1, 1, 1]), cuboid("on", [0, 0.5, 0], [1, 1, 1])];
    expect(exposedArea([above], [], planes)).toBeCloseTo(6, 9);
    expect(exposedArea([on], [], planes)).toBeCloseTo(5, 9);
  });

  it("exempts a layered thermal shell from the feature rule and reports only per-solid findings", () => {
    const shell = { id: "skin-shell", kind: "ellipsoid", bone: "head", material: "skin", role: "detail", tone: 0.5, radii: [0.002, 0.002, 0.002], region: "head", layer: "none" };
    const layered = patch(sample, [{ op: "add", path: "/volumes/50/thermal/-", value: shell }]) as Asset;
    expect(checkVolumes(layered, { scale: 50, rules: ["volumes-feature"] }).filter((f) => f.path.includes("thermal"))).toEqual([]);
    const aggregate = measureVolumes(sample, { scale: 50, rules: ["volumes-feature"] }).filter((m) => m.name === "minDimensionM");
    expect(aggregate.length).toBe(3);
    for (const m of aggregate) expect(m.ok).toBe(true);
  });

  it("widens open strokes to their pen in the whole-asset view agreement", () => {
    const stroke = { id: "mast", bone: "head", role: "detail", z: 0, filled: false, tone: 0, kind: "poly", points: [[0, 0], [0, 3]], closed: false };
    const taller = patch(sample, [{ op: "add", path: "/representations/100/geometry/front/-", value: stroke }]) as Asset;
    const [was, now] = [measureVolumes(sample, { scale: 100, rules: ["volumes-views-disagree"] }), measureVolumes(taller, { scale: 100, rules: ["volumes-views-disagree"] })];
    expect(now.find((m) => m.name === "extentDifferenceM:y")!.value).toBeGreaterThan(was.find((m) => m.name === "extentDifferenceM:y")!.value + 1);
  });
  it.each(vectors.viewsPosition.cases)("$name", { timeout: 300_000 }, (c) => {
    requireTool("uv");
    const v = vectors.viewsPosition;
    const rect = { id: "footprint", bone: sample.rig.root, role: "body", z: 0, filled: true, tone: 0.5, kind: "rect", x: v.elevationM[0]! + c.shiftM, y: -v.widthM / 2, w: v.elevationM[1]! - v.elevationM[0]!, h: v.widthM };
    const moved = patch(sample, [{ op: "replace", path: `/representations/${v.scale}/geometry/plan`, value: [rect] }]) as Asset;
    const options = { scale: v.scale as ScaleDenominator, rules: ["volumes-views-disagree"] as const };
    const measured = measureVolumes(moved, options);
    const [extent, place] = [measured.find((m) => m.name === `extentDifferenceM:${v.axis}`)!, measured.find((m) => m.name === `positionDifferenceM:${v.axis}`)!];
    expect(extent.value).toBeLessThan(1e-3);
    expect(extent.ok).toBe(true);
    expect(place.value).toBeCloseTo(c.shiftM, 3);
    expect(place.limit).toBeCloseTo(v.limitM, 9);
    const findings = checkVolumes(moved, options).filter((f) => (f.data as { axis?: string } | undefined)?.axis === v.axis);
    expect(findings.length).toBe(c.finding ? 1 : 0);
    if (c.finding) expect((findings[0]!.data as { firstM: number[]; secondM: number[] }).secondM[0]! - (findings[0]!.data as { firstM: number[] }).firstM[0]!).toBeCloseTo(c.shiftM, 3);
    const state = evaluate(moved, { kind: "pose", pose: moved.presentation.defaultPose }, { kind: "scale", scale: { denominator: v.scale } } as never);
    const points = (view: "elevation" | "plan") => (state.shapes[view] as readonly { shape: { role: string }; rings: readonly (readonly { x: number; y: number }[])[] }[]).filter((item) => ["body", "detail", "glazing"].includes(item.shape.role)).flatMap((item) => item.rings.flat().map((q) => [q.x, q.y]));
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: [{ name: "place", bounds: { first: points("elevation"), second: points("plan"), axis: 0 } }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { shift: number; extentDifference: number }>;
      expect(Math.abs(reference.place!.shift - place.value)).toBeLessThan(1e-9);
      expect(Math.abs(reference.place!.extentDifference - extent.value)).toBeLessThan(1e-9);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ C21: small regions", () => {
  it("skips the per-region area comparison of a region below 6 feature squared and keeps it in the total", () => {
    const tiny = patch(sample, [{ op: "replace", path: "/thermal/regions/id=feet/areaM2", value: 0.0001 }]) as Asset;
    const found = measureVolumes(tiny, { scale: 500, rules: ["volumes-thermal-coverage"] });
    const small = measureVolumes(tiny, { scale: 50, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === "areaDeviation:feet")!;
    expect(small.limit).toBe(Number.MAX_VALUE);
    expect(measureVolumes(sample, { scale: 50, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === "areaDeviation:feet")!.limit).toBe(0.3);
    expect(found.some((m) => m.name === "areaDeviationTotal")).toBe(true);
  });
});

describe("📏 Total component area with pose-dependent exposure", () => {
  it("measures the complete stock, excludes internal joints and rejects a wrong declaration", () => {
    const { stockM: [x, y, z], areaM2, wrongAreaM2 } = vectors.totalSurface;
    const candidate = structuredClone(sample);
    const base = { kind: "box", material: "skin", bone: "head", role: "body", tone: 0.5, size: [x / 2, y, z] };
    const parts = [-1, 1].map((dir, i) => ({ ...base, id: "stock-" + i, region: "head", transform: { translation: [dir * x / 4, 0, 0] } }));
    const asset = patch(candidate, [
      { op: "replace", path: "/thermal/regions/id=head/areaM2", value: areaM2 },
      { op: "add", path: "/thermal/regions/id=head/areaBasis", value: "total" },
      { op: "replace", path: "/volumes/50/thermal", value: [...(candidate.volumes["50"] as AuthoredVolume).thermal.filter(q => q.region !== "head"), ...parts] },
    ]) as Asset;
    const measure = (a: Asset) => measureVolumes(a, { scale: 50, rules: ["volumes-thermal-coverage"] });
    const mesh = new BoxGeometry(x, y, z).toNonIndexed();
    const pos = mesh.getAttribute("position");
    let surface = 0;
    for (let i = 0; i < pos.count; i += 3) {
      const vertices = [i, i + 1, i + 2].map(j => new Vector3().fromBufferAttribute(pos, j));
      surface += new Triangle(vertices[0]!, vertices[1]!, vertices[2]!).getArea();
    }
    expect(surface).toBeCloseTo(areaM2, 6);
    expect(measure(asset).find(m => m.name === "info:totalAreaM2:head")?.value).toBeCloseTo(surface, 6);
    const wrong = patch(asset, [{ op: "replace", path: "/thermal/regions/id=head/areaM2", value: wrongAreaM2 }]) as Asset;
    expect(measure(wrong).find(m => m.name === "areaDeviation:head")?.ok).toBe(false);
  });
});

describe("🧊️ C19: areaBasis exchange", () => {
  it("reports a leaf region without areaBasis and exempts a declared one from the exposed-area comparison", () => {
    const undeclared = patch(sample, vectors.areaBasis.undeclared) as Asset;
    expect(validateAsset(undeclared).some((issue) => issue.code === "thermal-area-basis" && issue.path === "thermal.regions.head.areaBasis")).toBe(true);
    const declared = patch(sample, vectors.areaBasis.declared) as Asset;
    expect(validateAsset(declared).some((issue) => issue.code === "thermal-area-basis")).toBe(false);
    expect(measureVolumes(declared, { scale: 50, rules: ["volumes-thermal-coverage"] }).some((m) => m.name === "areaDeviation:head")).toBe(false);
    expect(measureVolumes(sample, { scale: 50, rules: ["volumes-thermal-coverage"] }).some((m) => m.name === "areaDeviation:head")).toBe(true);
  });
});

describe("🧊️ C22: envelope solids and thermal area", () => {
  const areas = (asset: Asset, scale: ScaleDenominator): Map<string, number> => new Map(measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] }).filter((m) => m.name.startsWith("info:outerAreaM2:")).map((m) => [m.name.slice(17), m.value] as const));

  it("gives the region coverage but leaves the porous hull out of the sums", () => {
    const { scale, region, patch: operations } = vectors.envelopeArea;
    const [plain, envelope] = [sample, patch(sample, operations) as Asset];
    const [before, after] = [areas(plain, scale), areas(envelope, scale)];
    expect(before.has(region)).toBe(true);
    expect(after.has(region)).toBe(false);
    for (const [id, value] of before) if (id !== region) expect(Math.abs(after.get(id)! - value), id).toBeLessThan(vectors.envelopeArea.pocketFraction * value);
    expect(checkVolumes(envelope, { scale, rules: ["volumes-thermal-coverage"] }).filter((f) => (f.data as { region?: string } | undefined)?.region === region)).toEqual([]);
    const declared = new Map(sample.thermal.regions.map((r) => [r.id, r.areaM2] as const));
    const rest = [...before].filter(([id]) => id !== region && declared.get(id)! > 0 && sample.thermal.regions.find((r) => r.id === id)!.areaBasis !== "exchange");
    const hand = Math.abs(rest.reduce((s, [, v]) => s + v, 0) / rest.reduce((s, [id]) => s + declared.get(id)!, 0) - 1);
    expect(Math.abs(measureVolumes(envelope, { scale, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === "areaDeviationTotal")!.value - hand)).toBeLessThan(vectors.envelopeArea.pocketFraction);
  });

  it("measures a per-region area at 1:50 only for the regions without envelope solids", () => {
    const envelope = patch(sample, vectors.envelopeArea.finePatch) as Asset;
    const found = measureVolumes(envelope, { scale: vectors.envelopeArea.fineScale as ScaleDenominator, rules: ["volumes-thermal-coverage"] });
    expect(found.some((m) => m.name === `areaDeviation:${vectors.envelopeArea.region}`)).toBe(false);
    expect(found.some((m) => m.name === "areaDeviation:legs")).toBe(true);
  });
});

describe("🧊️ C23: a section declares the retained side", () => {
  const { cell, toleranceM2, cases, sampleSection } = vectors.retained;
  const projectedArea = (groups: readonly (readonly (readonly { x: number; y: number }[])[])[]): number => {
    const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
    for (const g of groups) for (const r of g) for (const p of r) [box.minX, box.minY, box.maxX, box.maxY] = [Math.min(box.minX, p.x), Math.min(box.minY, p.y), Math.max(box.maxX, p.x), Math.max(box.maxY, p.y)];
    const mask = new Mask({ minX: box.minX - 2 * cell, minY: box.minY - 2 * cell, maxX: box.maxX + 2 * cell, maxY: box.maxY + 2 * cell }, cell);
    for (const g of groups) mask.fill(g as never);
    return mask.cells.reduce((s, c) => s + c, 0) * cell * cell;
  };
  const scene = (axis: number): World[] =>
    [[-1, 1, 0], [1, 2, 5], [0, 3, 10]].map(([along, footprint, shift], k) => {
      const [centre, size] = [[0, 0, 0], [1, 1, 1]];
      const [b, c] = [0, 1, 2].filter((i) => i !== axis) as [number, number];
      centre[axis] = along!;
      centre[b] = shift! + footprint! / 2;
      size[b] = footprint!;
      return cuboid(`s${k}`, centre, size);
    });

  it.each(cases)("$view: the retained half of the hand scene, far by default, far and near explicitly", ({ view, far, near }) => {
    const solids = scene(CUT[view].axis);
    const at = (retained?: "far" | "near") => projectedArea(referenceOf(view, solids, { plane: 0, label: { en: "A-A", de: "A-A" }, ...(retained === undefined ? {} : { retained }) }).visible);
    expect(Math.abs(at() - far), "default").toBeLessThan(toleranceM2);
    expect(Math.abs(at("far") - far), "far").toBeLessThan(toleranceM2);
    expect(Math.abs(at("near") - near), "near").toBeLessThan(toleranceM2);
  });

  it("agrees with the silhouette of the halves cut by trimesh and shapely", { timeout: 300_000 }, () => {
    requireTool("uv");
    const input: Record<string, unknown>[] = [];
    const own: Record<string, number> = {};
    for (const c of cases)
      for (const retained of ["far", "near"] as const) {
        const solids = scene(CUT[c.view].axis);
        const keepHigh = (CUT[c.view].retained > 0) === (retained === "far");
        input.push({ name: `${c.view}-${retained}`, meshes: solids.map(mesh), view: c.view, half: { axis: CUT[c.view].axis, plane: 0, keepHigh } });
        own[`${c.view}-${retained}`] = projectedArea(referenceOf(c.view, solids, { plane: 0, label: { en: "A-A", de: "A-A" }, retained }).visible);
      }
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: input }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      for (const [key, value] of Object.entries(own)) expect(Math.abs(value - reference[key]!.area), `${key} ${value} against shapely ${reference[key]!.area}`).toBeLessThan(toleranceM2);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("changes the silhouette measurement of an authored section view", () => {
    const section = (retained?: "far" | "near"): Asset => patch(sample, [{ op: "add", path: `/representations/${sampleSection.scale}/sections`, value: { [sampleSection.view]: { plane: sampleSection.plane, label: sampleSection.label, ...(retained === undefined ? {} : { retained }) } } }]) as Asset;
    const iou = (asset: Asset): number => measureVolumes(asset, { scale: sampleSection.scale as ScaleDenominator, rules: ["volumes-silhouette"] }).find((m) => m.name === `iou:${sampleSection.view}`)!.value;
    expect(iou(section("far"))).toBe(iou(section()));
    expect(iou(section("near"))).not.toBe(iou(section("far")));
  });
});

describe("🧊️ emitters, huge bands and the names of the solids behind a finding", () => {
  const areas = (asset: Asset, scale: ScaleDenominator): Map<string, number> => new Map(measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] }).filter((m) => m.name.startsWith("info:outerAreaM2:")).map((m) => [m.name.slice(17), m.value] as const));

  it("leaves a solid tagged with an emitter id out of the exposed area of its region, as if it were removed", () => {
    const { scale, region, untagged, removed } = vectors.emitters;
    const [tagged, bare, gone] = [areas(sample, scale).get(region)!, areas(patch(sample, untagged) as Asset, scale).get(region)!, areas(patch(sample, removed) as Asset, scale).get(region)!];
    expect(bare).toBeGreaterThan(tagged + 0.01);
    expect(Math.abs(tagged - gone)).toBeLessThan(1e-9);
  });

  it("asks only the start of an emitter to lie in the body box", () => {
    const { scale, untagged } = vectors.emitters;
    const box = (asset: Asset): number => measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === "outsideBodyBoxM")!.value;
    expect(box(sample)).toBeLessThan(box(patch(sample, untagged) as Asset));
  });

  it("keeps a band of at least one cell around a body of 12 m, where the old grid left none", () => {
    const { bar, shortenM, distanceM } = vectors.bands.excavator;
    const [long, short] = [cuboid("long", [0, 0.2, 0], bar.size), cuboid("short", [-shortenM / 2, 0.2, 0], [bar.size[0]! - shortenM, bar.size[1]!, bar.size[2]!])];
    const r = bandVolumes([long], [short], distanceM);
    expect(r.cellM).toBeLessThanOrEqual(r.bandM);
    expect(r.eroded).toBeLessThan(r.fine);
    expect(r.dilated).toBeGreaterThan(r.fine);
    expect(r.coarse).toBeGreaterThanOrEqual(r.eroded);
    expect(r.coarse).toBeLessThanOrEqual(r.dilated);
  });

  it("measures a projected member against subpixel body triangles and edge-on faces", () => {
    const f = vectors.subpixelShaft;
    for (const face of [f.triangle, f.degenerate]) {
      const points = face.map(([x,y]) => ({ x: x!, y: y! }));
      const distance = projectedBodyDistance([[points]], { minX: 0, minY: 0, maxX: 0.4, maxY: 0.1 });
      const v = face.map(([x,y]) => new Vector3(x,y,0));
      const triangle = new Triangle(v[0]!,v[1]!,v[2]!);
      for (const [x,y] of f.points) {
        const probe = new Vector3(x,y,0);
        const expected = triangle.closestPointToPoint(probe,new Vector3()).distanceTo(probe);
        expect(distance({ x: x!, y: y! },f.probeRadiusM)).toBeCloseTo(Math.min(expected,f.probeRadiusM),9);
      }
    }
  });

  it("names the worst member solids of a containment finding", () => {
    const broken = patch(sample, [{ op: "replace", path: "/volumes/50/structure/0/r0", value: 0.6 }, { op: "replace", path: "/volumes/50/structure/0/r1", value: 0.6 }]) as Asset;
    const found = checkVolumes(broken, { scale: 50, rules: ["volumes-structure-coverage"] }).filter((f) => f.message.en.includes("outside the body silhouette"));
    expect(found.length).toBeGreaterThan(0);
    for (const f of found) expect((f.data!["solids"] as readonly string[])[0]).toBe("member-pelvis");
  });
});

describe("🧊️ burial is sampled over the area of a face, so no mesh needs tiling", () => {
  it("buries the footprint of a laptop on a table top and not the two big triangles of the whole top", () => {
    const { table, laptop, area, probeM, toleranceM2 } = vectors.burial;
    const [top, base] = [cuboid("table", table.centre, table.size), cuboid("laptop", laptop.centre, laptop.size)];
    expect(Math.abs(exposedArea([top], [top, base], [], (other) => other === top, probeM) - area)).toBeLessThan(toleranceM2);
  });
});

describe("🧊️ C29: the outer area of a region is the surface of the union of its own solids", () => {
  const { solids: boxes, area, toleranceFraction } = vectors.outer;
  const worlds = boxes.map((b, i) => cuboid(`o${i}`, b.centre, b.size));

  it("equals the hand value", () => {
    expect(relative(exposedArea(worlds, worlds, [], () => false, 0.0005), area)).toBeLessThan(toleranceFraction);
  });

  it("equals the surface samples of trimesh", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: [{ name: "union", meshes: worlds.map(mesh), unionOuter: { eps: 0.0005 } }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      expect(relative(exposedArea(worlds, worlds, [], () => false, 0.0005), reference.union!.area)).toBeLessThan(toleranceFraction);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("leaves faces covered by the solids of other regions in, and measures the exposure beside it", () => {
    const named = measureVolumes(sample, { scale: 50, rules: ["volumes-thermal-coverage"] });
    for (const id of ["torso", "legs"]) {
      const [outer, exposed] = [named.find((m) => m.name === `info:outerAreaM2:${id}`)!.value, named.find((m) => m.name === `info:exposedAreaM2:${id}`)!.value];
      expect(exposed).toBeLessThanOrEqual(outer + 1e-9);
      expect(named.some((m) => m.name === `info:exposureTarget:${id}`)).toBe(true);
      expect(named.find((m) => m.name === `info:exposureDeviation:${id}`)!.value).toBeGreaterThanOrEqual(0);
    }
  });
});

describe("🧊️ C30: a hollow part is judged by its overall extents, not by its wall", () => {
  it("measures the section path and wire diameter of a closed loft", () => {
    const { radiusM, stationsM, lengthM, diameterM } = vectors.sweptRod;
    const sections = stationsM.map((at) => ({ at, rx: radiusM, ry: radiusM, exponent: 2 }));
    const rod = { id: "link", kind: "loft", bone: "head", material: "skin", role: "body", tone: 0.5, sections, capStart: false, capEnd: false } as const;
    const [minor, major, length] = sortedExtents(rod);
    const stations = stationsM.map((point) => new Vector3(...point));
    const oracle = stations.slice(1).reduce((sum, point, i) => sum + point.distanceTo(stations[i]!), 0);
    expect(minor).toBeCloseTo(diameterM);
    expect(major).toBeCloseTo(diameterM);
    expect(length).toBeCloseTo(lengthM);
    expect(length).toBeCloseTo(oracle);
  });

  it("reads 80 mm for a cup with a 0.2 mm wall and finds no thin-solid finding for it at 1:50", () => {
    const { solid, smallestM } = vectors.hollow;
    expect(sortedExtents(solid as never)[0]).toBeCloseTo(smallestM, 9);
    const asset = patch(sample, [{ op: "add", path: "/volumes/50/body/0", value: solid }]) as Asset;
    expect(checkVolumes(asset, { scale: 50, rules: ["volumes-feature"] }).filter((f) => f.message.en.includes('"cup"'))).toEqual([]);
  });
});

describe("🧊️ C26: exchange regions are porous masses", () => {
  const { scale, patch: operations } = vectors.porous;
  const porous = patch(sample, operations) as Asset;

  it("makes the outline of a shape bound to an exchange region a soft outline in the silhouette", () => {
    const names = (asset: Asset) => measureVolumes(asset, { scale, rules: ["volumes-silhouette"] }).map((m) => m.name);
    expect(names(sample)).not.toContain("p95SoftM:elevation");
    expect(names(porous)).toContain("p95SoftM:elevation");
  });

  it("compares the union volume of the porous solids with the soft band, never more strictly than before", () => {
    const excess = (asset: Asset) => measureVolumes(asset, { scale, rules: ["volumes-cross-scale"] }).find((m) => m.name === "unionVolumeBandExcess")!.value;
    expect(excess(porous)).toBeLessThanOrEqual(excess(sample) + 1e-9);
  });

  it("lets the solids of an exchange region bury nothing in the exposure measure", () => {
    const exposed = (asset: Asset, id: string) => measureVolumes(asset, { scale: 50, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === `info:exposedAreaM2:${id}`)!.value;
    expect(exposed(porous, "head")).toBeGreaterThanOrEqual(exposed(sample, "head") - 1e-9);
  });
});

describe("🧊️ C24: a slender solid is drawn as a stroke", () => {
  const { pen, cases } = vectors.stroke;
  const slender = (c: (typeof cases)[number]): World => {
    const parts = c.centres.map((centre, i) => cuboid(`s${i}`, centre, c.size));
    const per = parts[0]!.positions.length;
    const positions = new Float64Array(per * parts.length);
    parts.forEach((w, i) => positions.set(w.positions, i * per));
    const indices = Uint32Array.from(parts.flatMap((w, i) => [...w.indices].map((x) => x + (i * per) / 3)));
    return { id: "bar", positions, indices, slender: true, penM: pen, copyVertices: per / 3 };
  };
  const area = (w: World): number => {
    const groups = referenceOf("elevation", [w], undefined).visible;
    const box = { minX: -3, minY: -3, maxX: 3, maxY: 3 };
    const mask = new Mask(box, 0.002);
    for (const g of groups) mask.fill(g);
    return mask.cells.reduce((s, c) => s + c, 0) * 0.002 * 0.002;
  };

  it.each(cases)("$name: the area is the hand value of the pen-widened centre line", (c) => {
    expect(Math.abs(area(slender(c)) - c.area)).toBeLessThan(0.004);
  });

  it("agrees with the buffered centre lines of shapely", { timeout: 300_000 }, () => {
    requireTool("uv");
    const input = cases.map((c) => ({ name: c.name, stroke: { lines: c.centres.map((p) => [[p[0]! - c.size[0]! / 2, p[1]!], [p[0]! + c.size[0]! / 2, p[1]!]]), width: Math.max(pen, c.size[1]!) } }));
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: input }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      for (const c of cases) expect(Math.abs(area(slender(c)) - reference[c.name]!.area), c.name).toBeLessThan(0.004);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ every raster measure is independent of the resolution", () => {
  const CELLS = [600, 900, 1200] as const;
  const variants: { name: string; asset: Asset; scale: ScaleDenominator }[] = [
    { name: "the sample", asset: sample, scale: 200 },
    { name: "a section view", asset: patch(sample, [{ op: "add", path: "/representations/200/sections", value: { front: { plane: vectors.retained.sampleSection.plane, label: vectors.retained.sampleSection.label } } }]) as Asset, scale: 200 },
    { name: "unfilled closed outlines", asset: patch(sample, vectors.outlines.patch) as Asset, scale: vectors.outlines.scale },
  ];

  it.each(variants)("silhouette IoU, p95 and Hausdorff of $name agree at 600, 900 and 1200 cells (IoU within 0.03, p95 within 2.5 cells)", ({ asset, scale }) => {
    for (const view of ["elevation", "front", "plan"] as const) {
      const runs = CELLS.map((cells) => measureAsset(asset, scale, undefined, cells, featureOf(scale)).views[view]);
      const long = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
      const coarse = long / CELLS[0];
      for (const r of runs.slice(1)) {
        expect(Math.abs(r.iou - runs[0]!.iou), `${view} IoU`).toBeLessThan(0.03);
        expect(Math.abs(r.p95M - runs[0]!.p95M), `${view} p95`).toBeLessThanOrEqual(2.5 * coarse);
      }
    }
  });

  it.each(variants)("the banded path of $name (tiles of 96 cells) reads what the whole raster reads", ({ asset, scale }) => {
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
    const solids = worldSolids(asset, state.bones as never, scale, "body");
    const feature = featureOf(scale);
    for (const view of ["elevation", "front", "plan"] as const) {
      const whole = measureView(asset, state, solids, view, scale, 400, feature, Infinity);
      const tiled = measureView(asset, state, solids, view, scale, 400, feature, 96);
      expect(Math.abs(tiled.iou - whole.iou), `${view} IoU`).toBeLessThan(0.01);
      const [cap, cell] = [6 * feature, whole.spread.cellM];
      expect(Math.abs(Math.min(tiled.p95M, cap) - Math.min(whole.p95M, cap)), `${view} p95 up to the margin of a tile`).toBeLessThanOrEqual(3 * cell);
    }
  });

  it("the comparison of two silhouettes agrees tiled and untiled and at three cell sizes", () => {
    const bodies = [50, 200].map((d) => worldSolids(sample, evaluate(sample, { kind: "pose", pose: sample.presentation.defaultPose }, { kind: "scale", scale: { denominator: d as ScaleDenominator } }).bones as never, d, "body"));
    for (const view of ["elevation", "front", "plan"] as const) {
      const runs = [0.03, 0.015, 0.0075].map((feature) => silhouetteAgreement(bodies[1]!, bodies[0]!, view, feature, feature));
      const tiled = silhouetteAgreement(bodies[1]!, bodies[0]!, view, 0.015, 0.015, { tileCells: 32 });
      const plain = runs[1]!;
      expect(Math.abs(tiled.iou - plain.iou), `${view} tiled IoU`).toBeLessThan(1e-6);
      expect(Math.abs(tiled.p95M - plain.p95M), `${view} tiled p95`).toBeLessThanOrEqual(plain.cellM);
      for (const r of runs) expect(Math.abs(r.iou - runs[0]!.iou), `${view} IoU`).toBeLessThan(0.03);
    }
  });
});

describe("🧊️ solids that touch exactly leave no hairline in the silhouette raster", () => {
  const { cell, roof, cases } = vectors.tangent;
  const covered = (c: (typeof cases)[number], seal: boolean): number => {
    const groups = referenceOf("elevation", [cuboid("roof", roof.centre, roof.size), cuboid("bar", c.bar.centre, c.bar.size)], undefined).cut;
    const mask = new Mask({ minX: -1.2, minY: -0.2, maxX: 1.2, maxY: 1.8 }, cell);
    for (const g of groups) mask.fill(g, seal);
    mask.seal();
    return mask.cells.reduce((s, x) => s + x, 0);
  };

  it.each(cases)("$name: the row of cells between the solids is closed only below a quarter cell", (c) => {
    expect(covered(c, true) - covered(c, false)).toBe(c.rowCells);
  });

  it("agrees with the morphological closing of shapely", { timeout: 300_000 }, () => {
    requireTool("uv");
    const rects = (c: (typeof cases)[number]): number[][] => [[-1, 0, 1, roof.size[1]!], [c.bar.centre[0]! - c.bar.size[0]! / 2, c.bar.centre[1]! - c.bar.size[1]! / 2, c.bar.centre[0]! + c.bar.size[0]! / 2, c.bar.centre[1]! + c.bar.size[1]! / 2]];
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: cases.map((c) => ({ name: c.name, closing: { rects: rects(c), radius: cell / 8 } })) }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number; closedArea: number }>;
      for (const c of cases) expect(reference[c.name]!.closedArea - reference[c.name]!.area > 1e-9, c.name).toBe(c.closed);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ C24 refined: a slender solid is drawn at the width of the 2D shape it follows, along its own centre line", () => {
  const { bar, cases, hoop } = vectors.follow;
  const slender = (w: World): World => ({ ...w, bone: "b", slender: true, copyVertices: w.positions.length / 3 });
  const area = (w: World, drawn: Parameters<typeof referenceOf>[3], view: "elevation" | "front" | "plan" = "elevation"): number => {
    const box = { minX: -1.5, minY: -1.5, maxX: 1.5, maxY: 1.5 };
    const mask = new Mask(box, 0.002);
    for (const g of referenceOf(view, [w], undefined, drawn).visible) mask.fill(g);
    return mask.cells.reduce((s, c) => s + c, 0) * 0.002 * 0.002;
  };
  const shapes = (c: (typeof cases)[number]) => c.drawn.map((d) => ({ ...d, rings: d.rings.map((r) => r.map(([x, y]) => ({ x: x!, y: y! }))) }));
  const tube = (): World => {
    const g = tessellateSolid({ id: "hoop", bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...hoop.solid } as never, { chordM: 0.002, minSegments: 12, maxSegments: 96 });
    return { id: "hoop", positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
  };

  it.each(cases)("$name", (c) => {
    const w = { ...slender(cuboid("bar", bar.centre, bar.size)), ...(c.material === undefined ? {} : { material: c.material }) };
    expect(Math.abs(area(w, shapes(c)) - (1 + c.widthM) * c.widthM)).toBeLessThan(0.004);
  });

  it("finds the centre line of a closed hoop: its length is the path and its thickness the tube, and it qualifies as slender", { timeout: 300_000 }, () => {
    requireTool("uv");
    const w = tube();
    const line = centreLineOf(w.positions, w.positions, w.indices)!;
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: [{ name: "hoop", pappus: { mesh: mesh(w), profile: hoop.solid.profile } }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { pathLength: number }>;
      expect(Math.abs(line.length - reference.hoop!.pathLength) / reference.hoop!.pathLength).toBeLessThan(0.05);
      expect(Math.abs(line.thickness - 2 * hoop.tubeM) / (2 * hoop.tubeM)).toBeLessThan(0.2);
      const solid = { id: "hoop", bone: "head", material: "skin", role: "body", tone: 0.5, slender: true, ...hoop.solid } as never;
      expect(slendernessOf(sample, solid)).toBeGreaterThanOrEqual(20);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("draws a hoop as the annulus of shapely: the centre line follows the circle at the pen of the outline it follows", { timeout: 300_000 }, () => {
    requireTool("uv");
    const w = slender(tube());
    const drawn = [{ bone: "b", closed: true, filled: false, rings: [hoop.outline.map(([x, y]) => ({ x: x!, y: y! }))], penM: hoop.penM }];
    const faceOn = Math.max(...(["elevation", "front", "plan"] as const).map((view) => area(w, drawn, view)));
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: [{ name: "ring", annulus: { radius: hoop.radiusM, width: hoop.penM } }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      expect(Math.abs(faceOn - reference.ring!.area) / reference.ring!.area).toBeLessThan(0.08);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
  it("keeps a slender solid along the outline of a body at its true width: the centre line of the outline is the body edge, the pen is convention", { timeout: 300_000 }, () => {
    requireTool("uv");
    const edge = vectors.follow.bodyEdge;
    const [plate, wire] = [{ ...cuboid("plate", edge.plate.centre, edge.plate.size), bone: "b" }, slender(cuboid("wire", edge.wire.centre, edge.wire.size))];
    const rings = [edge.outline.map(([x, y]) => ({ x: x!, y: y! }))];
    const drawn = [{ bone: "b", closed: true, filled: false, rings, penM: edge.penM }];
    const covered = (solids: readonly World[]): number => {
      const mask = new Mask({ minX: -1, minY: -1, maxX: 1, maxY: 1 }, 0.001);
      for (const g of referenceOf("elevation", solids, undefined, drawn).cut) mask.fill(g);
      return mask.cells.reduce((sum, c) => sum + c, 0) * 1e-6;
    };
    expect(bodyOutline("elevation", [plate, wire], rings)).toBe(true);
    expect(bodyOutline("elevation", [wire], rings)).toBe(false);
    const own = covered([plate, wire]);
    expect(Math.abs(own - edge.edgeAreaM2)).toBeLessThan(0.002);
    expect(edge.penAreaM2 - own).toBeGreaterThan(0.035);
    expect(Math.abs(covered([wire]) - edge.voidAreaM2)).toBeLessThan(0.002);
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: [{ name: "edge", closing: { rects: edge.rects, radius: 1e-6 } }] }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      expect(Math.abs(reference.edge!.area - edge.edgeAreaM2)).toBeLessThan(1e-9);
      expect(Math.abs(own - reference.edge!.area)).toBeLessThan(0.002);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ volumes-feature reads the true cross extent of a shape turned off the axes (C13, office-swivel-chair)", () => {
  const { cases } = vectors.minimumWidth;
  const points = (c: (typeof cases)[number]) => c.points.map(([x, y]) => ({ x: x!, y: y! }));

  it.each(cases)("$name: the smallest width is the hand value", (c) => {
    expect(minimumWidth(points(c))).toBeCloseTo(c.widthM, 3);
  });

  it("agrees with the minimum rotated rectangle of shapely and is never larger than the box around the shape", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      writeFileSync(join(directory, "input.json"), JSON.stringify({ cases: cases.map((c) => ({ name: c.name, minrect: c.points })) }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { width: number }>;
      for (const c of cases) {
        expect(minimumWidth(points(c)), c.name).toBeCloseTo(reference[c.name]!.width, 3);
        const [xs, ys] = [c.points.map((p) => p[0]!), c.points.map((p) => p[1]!)];
        expect(minimumWidth(points(c))).toBeLessThanOrEqual(Math.min(Math.max(...xs) - Math.min(...xs), Math.max(...ys) - Math.min(...ys)) + 1e-9);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ cancellation and progress", () => {
  it("reports progress up to the total for the expensive rules", () => {
    const calls: [number, number][] = [];
    checkVolumes(sample, { scale: 100, rules: ["volumes-silhouette"] }, { progress: (done, total) => void calls.push([done, total]) });
    expect(calls.length).toBeGreaterThan(1);
    expect(calls[calls.length - 1]![0]).toBe(calls[calls.length - 1]![1]);
    for (const [done, total] of calls) expect(done).toBeLessThanOrEqual(total);
  });

  it("stops when the signal is aborted", () => {
    const controller = new AbortController();
    controller.abort();
    expect(() => checkVolumes(sample, { scale: 100, rules: ["volumes-silhouette"] }, { signal: controller.signal })).toThrow();
  });
});

describe("🧊️ the geometry the rules measure, on hand-computed cuboids", () => {
  it.each(vectors.boxes.cases)("$name: the owned union volume, outer area and silhouette equal the hand values", ({ boxes, unionVolume: hand, outerArea: outer, view, silhouetteArea }) => {
    const solids = boxes.map((b, i) => cuboid(`b${i}`, b.centre, b.size));
    expect(relative(unionVolume(solids, vectors.boxes.cell), hand)).toBeLessThan(0.02);
    if (outer !== null) expect(relative(outerArea(solids), outer)).toBeLessThan(1e-9);
    const agreement = silhouetteAgreement(solids, solids, view, 0.05);
    expect(agreement.iou).toBe(1);
    expect(agreement.hausdorffM).toBe(0);
    void silhouetteArea;
  });

  it("measures the Hausdorff distance and the IoU of two displaced cuboids", () => {
    const [a, b] = [cuboid("a", [0, 0, 0], [1, 1, 1]), cuboid("b", [0.2, 0, 0], [1, 1, 1])];
    const { iou, hausdorffM, cellM } = silhouetteAgreement([a], [b], "elevation", 0.1);
    expect(Math.abs(iou - 0.8 / 1.2)).toBeLessThan(0.03);
    expect(Math.abs(hausdorffM - 0.2)).toBeLessThanOrEqual(2 * cellM);
  });
});

describe("🧊️ the owned geometry equals shapely and trimesh on the same triangles", () => {
  it("reproduces unions, meshes volumes, outer areas, IoUs and Hausdorff distances", { timeout: 300_000 }, () => {
    requireTool("uv");
    const cases: Record<string, unknown>[] = [];
    const expected: Record<string, { union?: number; outer?: number; iou?: number; hausdorff?: number; cell: number }> = {};
    for (const c of vectors.boxes.cases) {
      const solids = c.boxes.map((b, i) => cuboid(`b${i}`, b.centre, b.size));
      cases.push({ name: c.name, meshes: solids.map(mesh), cell: vectors.boxes.cell, outer: c.outerArea !== null });
      expected[c.name] = { union: unionVolume(solids, vectors.boxes.cell), ...(c.outerArea === null ? {} : { outer: outerArea(solids) }), cell: vectors.boxes.cell };
    }
    const pose = sample.presentation.defaultPose;
    const bodies = new Map<number, readonly World[]>();
    const thermal = new Map<number, readonly World[]>();
    for (const scale of [50, 200] as const) {
      const bones = evaluate(sample, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } }).bones;
      bodies.set(scale, worldSolids(sample, bones as never, scale, "body"));
      thermal.set(scale, worldSolids(sample, bones as never, scale, "thermal"));
    }
    const feature = featureOf(200);
    const box = extentOf(bodies.get(50)!);
    const cell = Math.max(0.04, feature / 2, Math.max(box.max[0] - box.min[0], box.max[2] - box.min[2]) / 400);
    for (const scale of [50, 200]) {
      cases.push({ name: `body-${scale}`, meshes: bodies.get(scale)!.map(mesh), cell });
      expected[`body-${scale}`] = { union: unionVolume(bodies.get(scale)!, cell), cell };
    }
    const torso = (scale: number) => thermal.get(scale)!.filter((w) => ["torso-torso", "pelvis-torso"].includes(w.id));
    cases.push({ name: "torso-50", meshes: torso(50).map(mesh), outer: true });
    expected["torso-50"] = { outer: outerArea(torso(50)), cell };
    for (const view of ["elevation", "front", "plan"] as const) {
      cases.push({ name: `cross-${view}`, meshes: bodies.get(200)!.map(mesh), other: bodies.get(50)!.map(mesh), view });
      const own = silhouetteAgreement(bodies.get(200)!, bodies.get(50)!, view, feature);
      expected[`cross-${view}`] = { iou: own.iou, hausdorff: own.hausdorffM, cell: own.cellM };
    }
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases }));
      const reference = JSON.parse(python(ORACLE, "geometry", [input])) as Oracle;
      for (const c of vectors.boxes.cases) {
        const r = reference[c.name]!;
        expect(relative(r.meshVolume, c.meshVolume), `${c.name} mesh volume`).toBeLessThan(1e-9);
        expect(relative(r.unionVolume!, c.unionVolume), `${c.name} union against the hand value`).toBeLessThan(0.02);
        expect(relative(expected[c.name]!.union!, r.unionVolume!), `${c.name} union against trimesh`).toBeLessThan(0.03);
        if (c.outerArea !== null) expect(relative(r.outerArea!, c.outerArea), `${c.name} outer area`).toBeLessThan(1e-9);
      }
      for (const scale of [50, 200]) expect(relative(expected[`body-${scale}`]!.union!, reference[`body-${scale}`]!.unionVolume!), `body 1:${scale} union`).toBeLessThan(0.03);
      expect(relative(expected["torso-50"]!.outer!, reference["torso-50"]!.outerArea!), "torso outer area").toBeLessThan(0.02);
      for (const view of ["elevation", "front", "plan"]) {
        const [own, r] = [expected[`cross-${view}`]!, reference[`cross-${view}`]!];
        expect(Math.abs(own.iou! - r.iou!), `${view} IoU ${own.iou} against shapely ${r.iou}`).toBeLessThan(0.02);
        expect(Math.abs(own.hausdorff! - r.hausdorff!), `${view} Hausdorff ${own.hausdorff} against shapely ${r.hausdorff}`).toBeLessThanOrEqual(3 * own.cell + 0.005);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ the amended measures equal shapely, trimesh and scipy", () => {
  it("reproduces exposed areas, eroded and dilated volumes, non-zero outlines and tolerant IoUs", { timeout: 300_000 }, () => {
    requireTool("uv");
    const FRAME = { u: { x: 1, y: 0, z: 0 }, v: { x: 0, y: 1, z: 0 }, w: { x: 0, y: 0, z: 1 } };
    const AXIS: Record<string, number[]> = { "+u": [1, 0, 0], "-u": [-1, 0, 0], "+v": [0, 1, 0], "-v": [0, -1, 0], "+w": [0, 0, 1], "-w": [0, 0, -1] };
    const cases: Record<string, unknown>[] = [];
    const own: Record<string, number> = {};
    vectors.exposed.forEach((c, n) => {
      const solids = c.boxes.map((b, i) => ({ ...cuboid(`b${i}`, b.centre, b.size), frame: FRAME, ...(b.sectionFaces === undefined ? {} : { sectionFaces: b.sectionFaces }) }) as World);
      const planes = (c.planes ?? []).map((p) => ({ point: p.point as never, normal: p.normal as never, toleranceM: p.toleranceM }));
      own[`exposed-${n}`] = exposedArea([solids[c.target]!], solids, planes, (other) => other === solids[c.target]);
      const sections = Object.fromEntries(c.boxes.map((b, i) => [String(i), (b.sectionFaces ?? []).map((f) => AXIS[f])]));
      cases.push({ name: `exposed-${n}`, meshes: solids.map(mesh), exposed: { target: c.target, planes: c.planes ?? [], sections, cosine: Math.cos(Math.PI / 4), eps: 5e-4 } });
    });
    const band = vectors.bands.cases[0]!;
    const toWorld = (list: { centre: number[]; size: number[] }[]) => list.map((b, i) => cuboid(`b${i}`, b.centre, b.size));
    const ownBand = bandVolumes(toWorld(band.fine), toWorld(band.coarse), vectors.bands.distance);
    cases.push({ name: "band", band: { fine: toWorld(band.fine).map(mesh), coarse: toWorld(band.coarse).map(mesh), distance: vectors.bands.distance, cell: ownBand.cellM } });
    const outline = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map((k) => ({ r: k % 2 === 0 ? 1 : 0.381966, a: (Math.PI * k) / 5 })).map(({ r, a }) => [r * Math.sin(a), r * Math.cos(a)]);
    cases.push({ name: "outline", outline });
    const [a, b] = [cuboid("a", [0, 0, 0], [1, 1, 1]), cuboid("b", [0.3, 0.1, 0], [1, 1, 1])];
    cases.push({ name: "tolerant", meshes: [mesh(a)], other: [mesh(b)], view: "elevation", tolerance: 0.1 });
    const agreement = silhouetteAgreement([a], [b], "elevation", 0.1, 0.1);
    const directory = mkdtempSync(join(tmpdir(), "staffage-volumes-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ cases }));
      const reference = JSON.parse(python(ORACLE, "geometry", [input])) as Record<string, any>;
      vectors.exposed.forEach((c, n) => {
        expect(Math.abs(reference[`exposed-${n}`].exposedArea - c.area), `${c.name} (trimesh)`).toBeLessThan(1e-9);
        expect(Math.abs(reference[`exposed-${n}`].exposedArea - own[`exposed-${n}`]!), `${c.name} owned`).toBeLessThan(1e-9);
      });
      for (const key of ["fine", "coarse", "dilated", "eroded"] as const) expect(relative(ownBand[key], reference.band[key]), `band ${key}`).toBeLessThan(0.03);
      expect(relative(reference.outline.outlineArea, vectors.winding.area)).toBeLessThan(1e-3);
      expect(Math.abs(agreement.iou - reference.tolerant.iouTolerant), `tolerant IoU ${agreement.iou} against shapely ${reference.tolerant.iouTolerant}`).toBeLessThan(0.03);
      expect(Math.abs(agreement.rawIou - reference.tolerant.iou)).toBeLessThan(0.02);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ authored volumes of the sample", () => {
  it("has an authored volume for every authored representation, with all three lists", () => {
    for (const scale of ["50", "100", "200", "500"] as const) {
      const volume = sample.volumes[scale] as AuthoredVolume;
      expect(volume.status).toBe(sample.representations[scale].status);
      for (const list of ["body", "thermal", "structure"] as const) expect(volume[list].length, `${scale} ${list}`).toBeGreaterThan(0);
    }
  });
});
