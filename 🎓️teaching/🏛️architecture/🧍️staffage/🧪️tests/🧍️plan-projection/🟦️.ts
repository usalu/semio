/** 🧍️ Plan projection: plan shapes follow their bone by its x travel, a `followTip` capsule stretches along a limb, and the handcrafted plan drawings of the standing adult, the walking commuter, the seated office worker and the city car track poses.
 *
 * THE REFERENCE is gl-matrix `mat2d` (an independent forward kinematics of the rig) and, for the cast depth, jsdom reading the exported SVG.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧍️plan-projection/🔣️.json — the vectors
 */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { glMatrix, mat2d } from "gl-matrix";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Bone, Shape } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedShape } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { exportCast } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import type { CastIndex } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

glMatrix.setMatrixArrayType(Array);

type Extent = { minX: number; maxX: number };
type Vectors = {
  asset: string;
  scale: number;
  shapes: Shape[];
  cases: { name: string; pose: string; thighTipTravelM: number; extents: Record<string, Extent> }[];
  catalog: {
    assets: Record<string, string>;
    scales: number[];
    armSpanM: { min: number; max: number; assets: string[]; pose: Record<string, string> };
    car: { pose: string; lengthM: Extent2; widthM: Extent2 };
    stride: { asset: string; from: string; to: string; shape: string; minTravelM: number };
    lean: { asset: string; from: string; to: string; shape: string; minTravelM: number };
  };
  casts: { manifest: unknown; depthM: Record<string, { min: number; max: number }> };
};
type Extent2 = { min: number; max: number };

const vectors = fixture<Vectors>("🧍️plan-projection");
const readAsset = (path: string): Asset => JSON.parse(readFileSync(path, "utf8")).asset as Asset;
const sample = readAsset(join(STAFFAGE_ROOT, "🧫️fixtures/👤️sample-figure/🔣️.json"));
const catalogAsset = (id: string): Asset => readAsset(join(STAFFAGE_ROOT, "🗂️catalog", vectors.catalog.assets[id]!, id, "🔣️.json"));
const authoredAt = (asset: Asset, denominator: number): AuthoredRepresentation => asset.representations[String(denominator) as keyof Asset["representations"]] as AuthoredRepresentation;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;

const extentOf = (evaluated: EvaluatedShape): { minX: number; maxX: number; minY: number; maxY: number } => {
  const points = evaluated.rings.flat();
  return { minX: Math.min(...points.map((p) => p.x)), maxX: Math.max(...points.map((p) => p.x)), minY: Math.min(...points.map((p) => p.y)), maxY: Math.max(...points.map((p) => p.y)) };
};

const radians = (degrees: number): number => (degrees * Math.PI) / 180;

function worldOf(asset: Asset, pose: string, boneId: string): mat2d {
  const offsets = asset.poses.find((candidate) => candidate.id === pose)?.bones ?? {};
  const chain: Bone[] = [];
  for (let bone: Bone | undefined = asset.rig.bones.find((b) => b.id === boneId); bone !== undefined; bone = asset.rig.bones.find((b) => b.id === bone!.parent)) chain.unshift(bone);
  const matrix = mat2d.create();
  for (const bone of chain) {
    const offset = offsets[bone.id] ?? {};
    let angle = 0;
    let dx = 0;
    let dy = 0;
    if (bone.joint.type === "revolute") angle = Math.min(Math.max(offset.angleDeg ?? 0, bone.joint.minDeg ?? -Infinity), bone.joint.maxDeg ?? Infinity);
    if (bone.joint.type === "prismatic") {
      const axis = radians(bone.joint.axisDeg ?? 0);
      const s = Math.min(Math.max((offset.dx ?? 0) * Math.cos(axis) + (offset.dy ?? 0) * Math.sin(axis), bone.joint.minM ?? -Infinity), bone.joint.maxM ?? Infinity);
      dx = s * Math.cos(axis);
      dy = s * Math.sin(axis);
    }
    mat2d.translate(matrix, matrix, [bone.x + dx, bone.y + dy]);
    mat2d.rotate(matrix, matrix, radians(bone.angleDeg + angle));
  }
  return matrix;
}

const pointAt = (matrix: mat2d, x: number): { x: number; y: number } => ({ x: matrix[0]! * x + matrix[4]!, y: matrix[1]! * x + matrix[5]! });

function travel(asset: Asset, pose: string, boneId: string, along: number): number {
  const bone = asset.rig.bones.find((b) => b.id === boneId)!;
  return pointAt(worldOf(asset, pose, boneId), along * bone.lengthM).x - pointAt(worldOf(asset, "", boneId), along * bone.lengthM).x;
}

function planOf(asset: Asset, pose: string, denominator: number): Record<string, EvaluatedShape> {
  const state = evaluate(asset, { kind: "pose", pose }, select(denominator));
  return Object.fromEntries(state.shapes.plan.map((evaluated) => [evaluated.shape.id, evaluated]));
}

/** 📌️ A rigid plan shape is carried at the height of the middle of its elevation twin (same id, same bone); the vectors fix the twins at the pivot of their bones, where a point only turns about the bone, so the footprints are those hand-computed in the vectors (r cos(angle), 0.168573 + 0.945519 x). */
const pivotTwins: Shape[] = [
  { id: "rigid", kind: "rect", bone: "thigh-l", role: "body", z: 1, filled: true, tone: 0.4, x: -0.001, y: -0.001, w: 0.002, h: 0.002 },
  { id: "follower", kind: "rect", bone: "shank-l", role: "body", z: 1, filled: true, tone: 0.4, x: -0.001, y: -0.001, w: 0.002, h: 0.002 },
];

describe("🧍️ plan projection of a limb", () => {
  const patched: Asset = {
    ...sample,
    representations: {
      ...sample.representations,
      [String(vectors.scale)]: { ...authoredAt(sample, vectors.scale), geometry: { ...authoredAt(sample, vectors.scale).geometry, elevation: [...authoredAt(sample, vectors.scale).geometry.elevation, ...pivotTwins], plan: vectors.shapes } },
    },
  };

  for (const c of vectors.cases)
    it(c.name, () => {
      const plan = planOf(patched, c.pose, vectors.scale);
      for (const [id, expected] of Object.entries(c.extents)) {
        const extent = extentOf(plan[id]!);
        expect(extent.minX, `${id} minX`).toBeCloseTo(expected.minX, 5);
        expect(extent.maxX, `${id} maxX`).toBeCloseTo(expected.maxX, 5);
      }
    });

  it("agrees with an independent gl-matrix forward kinematics on the thigh tip travel", () => {
    for (const c of vectors.cases) expect(travel(sample, c.pose, "thigh-l", 1)).toBeCloseTo(c.thighTipTravelM, 5);
  });

  it("keeps the y extent of a plan shape (plan y is authored, only x travels)", () => {
    const plan = planOf(patched, "stride-left-forward", vectors.scale);
    const limb = extentOf(plan.limb!);
    expect(limb.minY).toBeCloseTo(0.09 - 0.08, 2);
    expect(limb.maxY).toBeCloseTo(0.09 + 0.08, 2);
  });
});

describe("🧍️ handcrafted plan drawings of the catalog", () => {
  for (const id of Object.keys(vectors.catalog.assets))
    it(`${id} draws a plan at every authored scale`, () => {
      const asset = catalogAsset(id);
      for (const scale of vectors.catalog.scales) {
        const authored = authoredAt(asset, scale);
        expect(authored.geometry.plan.length, `1:${scale}`).toBeGreaterThan(0);
        const plan = planOf(asset, asset.presentation.defaultPose, scale);
        expect(Object.keys(plan).length).toBe(authored.geometry.plan.length);
      }
    });

  it("gives a person a plan arm span of about half a metre at every scale that draws arms", () => {
    const { min, max, assets, pose } = vectors.catalog.armSpanM;
    const measured: Record<string, { span: number; rings: number[][][] }> = {};
    for (const id of assets) {
      const asset = catalogAsset(id);
      for (const scale of [50, 100]) {
        const arms = Object.values(planOf(asset, pose[id]!, scale)).filter((evaluated) => evaluated.shape.id.startsWith("upper-arm"));
        expect(arms.length, `${id} 1:${scale}`).toBe(2);
        const span = Math.max(...arms.map((a) => extentOf(a).maxY)) - Math.min(...arms.map((a) => extentOf(a).minY));
        expect(span, `${id} 1:${scale}`).toBeGreaterThan(min);
        expect(span, `${id} 1:${scale}`).toBeLessThan(max);
        measured[`${id}/${scale}`] = { span, rings: arms.flatMap((a) => a.rings.map((ring) => ring.map((p) => [p.x, p.y]))) };
      }
    }
    if (!requireTool("uv")) return;
    const directory = mkdtempSync(join(tmpdir(), "staffage-plan-span-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify(Object.fromEntries(Object.entries(measured).map(([key, value]) => [key, value.rings]))));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧍️plan-projection/🐍️.py"), "geometry", [input])) as Record<string, { minY: number; maxY: number }>;
      for (const [key, value] of Object.entries(measured)) expect(reference[key]!.maxY - reference[key]!.minY, `${key} (shapely)`).toBeCloseTo(value.span, 6);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("gives the city car its true length and width in plan", () => {
    const { pose, lengthM, widthM } = vectors.catalog.car;
    const asset = catalogAsset("city-car");
    for (const scale of vectors.catalog.scales) {
      const shell = extentOf(planOf(asset, pose, scale)["body-shell"]!);
      expect(shell.maxX - shell.minX, `1:${scale} length`).toBeGreaterThan(lengthM.min);
      expect(shell.maxX - shell.minX, `1:${scale} length`).toBeLessThan(lengthM.max);
      const all = Object.values(planOf(asset, pose, scale)).map(extentOf);
      const width = Math.max(...all.map((e) => e.maxY)) - Math.min(...all.map((e) => e.minY));
      expect(width, `1:${scale} width`).toBeGreaterThan(widthM.min - 0.2);
      expect(width, `1:${scale} width`).toBeLessThan(widthM.max);
    }
  });

  it("moves the commuter's foot in plan exactly as the independent kinematics predicts", () => {
    const { asset: id, from, to, shape: shoe, minTravelM } = vectors.catalog.stride;
    const asset = catalogAsset(id);
    const reps = authoredAt(asset, 50).geometry.plan!;
    const authored = reps.find((s) => s.id === shoe)!;
    expect(authored.kind).toBe("capsule");
    const capsule = authored as Extract<Shape, { kind: "capsule" }>;
    const heights: number[] = [];
    for (const pose of [from, to]) {
      const extent = extentOf(planOf(asset, pose, 50)[shoe]!);
      const dx0 = travel(asset, pose, capsule.bone, 0);
      const dx1 = travel(asset, pose, capsule.bone, 1);
      expect(extent.maxX, `${pose} maxX`).toBeCloseTo(Math.max(capsule.x0 + dx0 + capsule.r0, capsule.x1 + dx1 + capsule.r1), 2);
      expect(extent.minX, `${pose} minX`).toBeCloseTo(Math.min(capsule.x0 + dx0 - capsule.r0, capsule.x1 + dx1 - capsule.r1), 2);
      heights.push(extent.maxX);
    }
    expect(Math.abs(heights[0]! - heights[1]!)).toBeGreaterThan(minTravelM);
  });

  it("moves the seated worker's shoulders in plan when leaning back", () => {
    const { asset: id, from, to, shape, minTravelM } = vectors.catalog.lean;
    const asset = catalogAsset(id);
    const before = extentOf(planOf(asset, from, 50)[shape]!);
    const after = extentOf(planOf(asset, to, 50)[shape]!);
    expect(before.minX - after.minX).toBeGreaterThan(minTravelM);
    expect(after.minX).toBeLessThan(before.minX);
  });
});

describe("🧍️ cast export of plan casts", { timeout: 60_000 }, () => {
  let root: string;
  let indexes: readonly CastIndex[];

  beforeAll(async () => {
    root = mkdtempSync(join(tmpdir(), "staffage-plan-"));
    indexes = await exportCast(vectors.casts.manifest, Object.keys(vectors.casts.depthM), root);
  }, 120_000);

  afterAll(() => rmSync(root, { recursive: true, force: true }));

  it("sizes a plan cast to the plan and an elevation cast to the elevation", () => {
    for (const [id, { min, max }] of Object.entries(vectors.casts.depthM)) {
      const index = indexes.find((candidate) => candidate.cast === id)!;
      const depth = index.boundsM.maxY - index.boundsM.minY;
      expect(depth, id).toBeGreaterThan(min);
      expect(depth, id).toBeLessThan(max);
      const cast = (vectors.casts.manifest as { casts: Record<string, { asset: string; pose: string; scale: number }> }).casts[id]!;
      const figure = evaluate(catalogAsset(cast.asset), { kind: "pose", pose: cast.pose }, select(cast.scale)).shapes.elevation.filter((evaluated) => evaluated.shape.role !== "ground").flatMap((evaluated) => evaluated.rings.flat());
      const height = Math.max(...figure.map((p) => p.y)) - Math.min(...figure.map((p) => p.y));
      expect(Math.abs(index.logical.heightM - height), `${id}: heightM is the physical figure height in the side elevation, whatever the viewpoint of the cast`).toBeLessThan(0.01);
      if (index.viewpoint === "plan") expect(index.logical.heightM, `${id}: heightM is the physical figure height, not the plan depth`).toBeGreaterThan(depth + 0.3);
      else expect(Math.abs(index.logical.heightM - depth), id).toBeLessThan(0.15);
    }
  });
});
