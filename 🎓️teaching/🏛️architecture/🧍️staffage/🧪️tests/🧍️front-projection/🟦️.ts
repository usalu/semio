/** 🧍️ Front projection: front shapes follow their bone by its y travel, the radiator and the split air conditioner are drawn from the front, and a front cast is sized to the front.
 *
 * THE REFERENCE is gl-matrix `mat2d` (an independent forward kinematics of the rig), hand-computed catalogue dimensions and, for the cast sizes, the exported index.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧍️front-projection/🔣️.json — the vectors
 */
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { glMatrix, mat2d } from "gl-matrix";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Bone, Shape } from "../../🧬️schema/🟦️.ts";
import { exportCast } from "../../🔨️modules/📤️export/🎭️cast/🟦️.ts";
import type { CastIndex } from "../../🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { loadEnvironments } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { solveThermalAt } from "../../🔨️modules/🌡️thermal/🟦️.ts";
import { STAFFAGE_ROOT, fixture } from "../🧰️support/🧰️.ts";

glMatrix.setMatrixArrayType(Array);

type Box = { minX: number; maxX: number; minY: number; maxY: number };
type Range = { min: number; max: number };
type Vectors = {
  scale: number;
  shapes: Shape[];
  cases: { name: string; pose: string; shankOriginRelativeTravelYM: number; followerMinusFixedMinX: number; followerMinusFixedMinY: number }[];
  catalog: {
    assets: Record<string, string>;
    scales: number[];
    outline: { asset: string; shape: string; scale: number; xM: Range; yM: Range };
    bands: { asset: string; pose: string; regions: string[]; minSpreadK: number };
  };
  casts: { manifest: { casts: Record<string, unknown> }; widthM: Record<string, Range>; heightM: Record<string, Range> };
};

const vectors = fixture<Vectors>("🧍️front-projection");
const readAsset = (path: string): Asset => JSON.parse(readFileSync(path, "utf8")).asset as Asset;
const sample = readAsset(join(STAFFAGE_ROOT, "🧫️fixtures/👤️sample-figure/🔣️.json"));
const catalogAsset = (id: string): Asset => readAsset(join(STAFFAGE_ROOT, "🗂️catalog", vectors.catalog.assets[id]!, id, "🔣️.json"));
const authoredAt = (asset: Asset, denominator: number): AuthoredRepresentation => asset.representations[String(denominator) as keyof Asset["representations"]] as AuthoredRepresentation;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;
const radians = (degrees: number): number => (degrees * Math.PI) / 180;

function worldOf(asset: Asset, pose: string, boneId: string): mat2d {
  const offsets = asset.poses.find((candidate) => candidate.id === pose)?.bones ?? {};
  const chain: Bone[] = [];
  for (let bone: Bone | undefined = asset.rig.bones.find((b) => b.id === boneId); bone !== undefined; bone = asset.rig.bones.find((b) => b.id === bone!.parent)) chain.unshift(bone);
  const matrix = mat2d.create();
  for (const bone of chain) {
    const offset = offsets[bone.id] ?? {};
    const angle = bone.joint.type === "revolute" ? Math.min(Math.max(offset.angleDeg ?? 0, bone.joint.minDeg ?? -Infinity), bone.joint.maxDeg ?? Infinity) : 0;
    mat2d.translate(matrix, matrix, [bone.x, bone.y]);
    mat2d.rotate(matrix, matrix, radians(bone.angleDeg + angle));
  }
  return matrix;
}

const frontOf = (asset: Asset, pose: string, denominator: number) => {
  const state = evaluate(asset, { kind: "pose", pose }, select(denominator));
  return Object.fromEntries(state.shapes.front.map((evaluated) => [evaluated.shape.id, evaluated]));
};

const extentOf = (rings: readonly (readonly { x: number; y: number }[])[]): Box => {
  const points = rings.flat();
  return { minX: Math.min(...points.map((p) => p.x)), maxX: Math.max(...points.map((p) => p.x)), minY: Math.min(...points.map((p) => p.y)), maxY: Math.max(...points.map((p) => p.y)) };
};

describe("🧍️ front projection of a bone", () => {
  const patched: Asset = {
    ...sample,
    representations: {
      ...sample.representations,
      [String(vectors.scale)]: { ...authoredAt(sample, vectors.scale), geometry: { ...authoredAt(sample, vectors.scale).geometry, front: vectors.shapes } },
    },
  };

  for (const c of vectors.cases)
    it(c.name, () => {
      const front = frontOf(patched, c.pose, vectors.scale);
      const follower = extentOf(front.follower!.rings);
      const fixed = extentOf(front.fixed!.rings);
      expect(follower.minX - fixed.minX).toBeCloseTo(c.followerMinusFixedMinX, 5);
      expect(follower.minY - fixed.minY).toBeCloseTo(c.followerMinusFixedMinY, 5);
      expect(follower.maxX - follower.minX).toBeCloseTo(0.2, 5);
    });

  it("agrees with an independent gl-matrix forward kinematics on the shank origin y travel", () => {
    const relative = (pose: string): number => worldOf(sample, pose, "shank-l")[5]! - worldOf(sample, pose, "thigh-l")[5]!;
    for (const c of vectors.cases) expect(relative(c.pose) - relative("")).toBeCloseTo(c.shankOriginRelativeTravelYM, 5);
  });
});

describe("🧍️ handcrafted front drawings of the catalog", () => {
  for (const id of Object.keys(vectors.catalog.assets))
    it(`${id} draws a front elevation at every authored scale`, () => {
      const asset = catalogAsset(id);
      for (const scale of vectors.catalog.scales) {
        const authored = authoredAt(asset, scale);
        expect(authored.geometry.front.length, `1:${scale}`).toBeGreaterThan(0);
        expect(Object.keys(frontOf(asset, asset.presentation.defaultPose, scale)).length).toBe(authored.geometry.front.length);
      }
    });

  it("gives the radiator its catalogue front size", () => {
    const { asset, shape, scale, xM, yM } = vectors.catalog.outline;
    const extent = extentOf(frontOf(catalogAsset(asset), "rated", scale)[shape]!.rings);
    expect(extent.minX).toBeCloseTo(xM.min, 3);
    expect(extent.maxX).toBeCloseTo(xM.max, 3);
    expect(extent.minY).toBeCloseTo(yM.min, 3);
    expect(extent.maxY).toBeCloseTo(yM.max, 3);
  });

  it("warms the upper third of the front plate more than the lower third", async () => {
    const { asset, pose, regions, minSpreadK } = vectors.catalog.bands;
    const presets = await loadEnvironments();
    const environment = presets!.presets.find((preset) => preset.id === "indoor-heated-winter")!.environment;
    const field = solveThermalAt(catalogAsset(asset), { kind: "pose", pose }, environment);
    const [upper, mid, lower] = regions.map((region) => field.regions[region]!.surfaceTempC);
    expect(upper! - lower!).toBeGreaterThanOrEqual(minSpreadK);
    expect(upper!).toBeGreaterThan(mid!);
    expect(mid!).toBeGreaterThan(lower!);
  });
});

describe("🧍️ front casts", () => {
  let root = "";
  let indexes: readonly CastIndex[] = [];

  beforeAll(async () => {
    root = mkdtempSync(join(tmpdir(), "staffage-front-"));
    indexes = await exportCast(vectors.casts.manifest, Object.keys(vectors.casts.widthM), root);
  }, 120_000);

  afterAll(() => rmSync(root, { recursive: true, force: true }));

  it("sizes a front cast to the front and a side cast to the section", () => {
    for (const [id, { min, max }] of Object.entries(vectors.casts.widthM)) {
      const index = indexes.find((candidate) => candidate.cast === id)!;
      const width = index.boundsM.maxX - index.boundsM.minX;
      expect(width, `${id} width`).toBeGreaterThan(min);
      expect(width, `${id} width`).toBeLessThan(max);
      expect(index.heightM, `${id} height`).toBeGreaterThan(vectors.casts.heightM[id]!.min);
      expect(index.heightM, `${id} height`).toBeLessThan(vectors.casts.heightM[id]!.max);
    }
  });
});

describe("🧍️ front and plan geometry are mandatory", () => {
  it("refuses a representation with an emptied front or plan drawing", () => {
    const emptied = (view: "front" | "plan"): Asset => ({ ...sample, representations: { ...sample.representations, "100": { ...authoredAt(sample, 100), geometry: { ...authoredAt(sample, 100).geometry, [view]: [] } } } as Asset["representations"] });
    expect(validateAsset(sample, {})).toEqual([]);
    for (const view of ["front", "plan"] as const) {
      const issue = validateAsset(emptied(view), {}).find((candidate) => candidate.code === `${view}-empty`);
      expect(issue?.path, view).toBe(`representations.100.geometry.${view}`);
      expect(issue?.message.de, view).toContain(view === "front" ? "Vorderansicht" : "Draufsicht");
    }
  });
});
