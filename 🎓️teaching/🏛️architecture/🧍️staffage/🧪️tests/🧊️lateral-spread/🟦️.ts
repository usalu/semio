/** 🧊️ Lateral spread: `abductionDeg` turns a revolute bone out of the sagittal plane for the front and plan views only; the elevation never changes, children ride on the spread bone, clips interpolate it and the validator bounds it.
 *
 * THE REFERENCE is numpy (three-dimensional bone chain with rotation matrices) and shapely (extents) in 🐍️.py.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️lateral-spread/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, BoneOffset, Pose, Shape, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveFrames, solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool, python } from "../🧰️support/🧰️.ts";
import { catalog, extentOf, type Extent } from "../🧊️three-views/🧰️.ts";
requireTool("uv");

type Vectors = {
  extentToleranceM: number;
  asset: string;
  scale: number;
  poses: Record<string, { bones: Record<string, BoneOffset> }>;
  shapes: Record<Viewpoint, Shape[]>;
  cases: { name: string; pose: string; transitionAt?: number; abductionDeg?: number; extents: Record<string, Partial<Extent>> }[];
  displacements: { name: string; pose: string; bone: string; originWorld: { x: number; y: number }; tipWorld: { x: number; y: number }; originZ: number; tipZ: number }[];
  boneZ: { bone: string; z: number; shapes: Record<Viewpoint, Shape[]>; cases: { name: string; pose: string; extents: Record<string, Partial<Extent>> }[]; limits: { name: string; z: number; refused: boolean }[] };
  limits: { name: string; bone: string; joint: "revolute" | "fixed" | "prismatic"; abductionDeg: number; refused: boolean }[];
};

const vectors = fixture<Vectors>("🧊️lateral-spread");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;
const stand = sample.poses.find((pose) => pose.id === "stand")!;

const withPoses = (base: Asset): Asset => {
  const authored = base.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
  const poses: Pose[] = [...base.poses, ...Object.entries(vectors.poses).map(([id, pose]) => ({ ...stand, id, bones: pose.bones }))];
  return { ...base, poses, representations: { ...base.representations, [String(vectors.scale)]: { ...authored, geometry: vectors.shapes } } };
};
const asset = withPoses(sample);

const stateOf = (c: Vectors["cases"][number]): EvaluatedState =>
  c.transitionAt === undefined ? evaluate(asset, { kind: "pose", pose: c.pose }, select(vectors.scale)) : evaluate(asset, { kind: "transition", from: "stand", to: c.pose, durationS: 1, timeS: c.transitionAt }, select(vectors.scale));

const extents = (state: EvaluatedState): Record<string, Extent> => {
  const out: Record<string, Extent> = {};
  for (const view of ["elevation", "front", "plan"] as const) for (const evaluated of state.shapes[view]) out[`${view}/${evaluated.shape.id}`] = extentOf([evaluated], true)!;
  return out;
};

const expectPart = (label: string, actual: Extent, expected: Partial<Extent>): void => {
  for (const [edge, value] of Object.entries(expected)) expect(Math.abs(actual[edge as keyof Extent] - value), `${label} ${edge}: ${actual[edge as keyof Extent]} vs ${value}`).toBeLessThanOrEqual(vectors.extentToleranceM);
};

const offsetsOf = (pose: string): Record<string, BoneOffset> => (pose === "stand" ? {} : vectors.poses[pose]!.bones);
const root = { x: 0, y: 0.96, angleDeg: 0 };

describe("🧊️ the vectors of the lateral spread", () => {
  for (const c of vectors.cases)
    it(c.name, () => {
      const got = extents(stateOf(c));
      for (const [key, part] of Object.entries(c.extents)) expectPart(key, got[key]!, part);
    });

  it("moves the bone matrices with the spread so every consumer reads the same chain", () => {
    const flat = solveRig(asset.rig, {}, root);
    const spread = solveRig(asset.rig, offsetsOf("arm-60"), root);
    const length = sample.rig.bones.find((bone) => bone.id === "arm-l")!.lengthM;
    expect(flat["arm-l"]![1] * length + flat["arm-l"]![5]).toBeCloseTo(0.88, 6);
    expect(spread["arm-l"]![1] * length + spread["arm-l"]![5]).toBeCloseTo(1.18, 6);
  });

  it("returns no frames when no bone is spread", () => {
    expect(solveFrames(asset.rig, {}, root)).toBeNull();
  });

  it.each(vectors.displacements)("places $name at the hand-computed projected and lateral position", (d) => {
    const frames = solveFrames(asset.rig, offsetsOf(d.pose), root)!;
    const frame = frames[d.bone]!;
    const length = sample.rig.bones.find((bone) => bone.id === d.bone)!.lengthM;
    expect(frame.affine[4], "origin x").toBeCloseTo(d.originWorld.x, 5);
    expect(frame.affine[5], "origin y").toBeCloseTo(d.originWorld.y, 5);
    expect(frame.affine[0] * length + frame.affine[4], "tip x").toBeCloseTo(d.tipWorld.x, 5);
    expect(frame.affine[1] * length + frame.affine[5], "tip y").toBeCloseTo(d.tipWorld.y, 5);
    expect(frame.originZ, "origin z").toBeCloseTo(d.originZ, 5);
    expect(frame.tipZ, "tip z").toBeCloseTo(d.tipZ, 5);
  });

  it("keeps a spread of zero equal to the sagittal pose", () => {
    const base = extents(evaluate(asset, { kind: "pose", pose: "stand" }, select(vectors.scale)));
    const zero = withPoses({ ...sample, poses: [...sample.poses.map((pose) => (pose.id === "stand" ? { ...pose, bones: { ...pose.bones, "arm-l": { ...pose.bones["arm-l"], abductionDeg: 0 } } } : pose))] });
    expect(extents(evaluate(zero, { kind: "pose", pose: "stand" }, select(vectors.scale)))).toEqual(base);
  });
});

describe("🧊️ the validator bounds the spread", () => {
  const jointIssues = (candidate: Asset) => validateAsset(candidate).filter((issue) => issue.code === "joint-limit");
  const baseline = jointIssues(sample).length;

  it.each(vectors.limits)("$name", (limit) => {
    const bones = sample.rig.bones.map((bone) => (bone.id === limit.bone ? { ...bone, joint: limit.joint === "revolute" ? bone.joint : limit.joint === "fixed" ? { type: "fixed" as const } : { type: "prismatic" as const, axisDeg: 0, minM: -1, maxM: 1 } } : bone));
    const poses = sample.poses.map((pose) => (pose.id === "stand" ? { ...pose, bones: { ...pose.bones, [limit.bone]: { abductionDeg: limit.abductionDeg } } } : pose));
    const issues = jointIssues({ ...sample, rig: { ...sample.rig, bones }, poses });
    expect(issues.length - baseline > 0, issues.map((issue) => issue.path).join(",")).toBe(limit.refused);
  });
});

describe("🧊️ looping clips close their spread and twist", () => {
  const closing = (candidate: Asset) => validateAsset(candidate).filter((issue) => issue.code === "loop-closing").length;
  const looping = sample.clips.find((clip) => clip.loop && clip.keys.length > 1 && clip.keys[clip.keys.length - 1]!.t >= clip.durationS)!;
  const open = (field: "abductionDeg" | "twistDeg"): Asset => ({
    ...sample,
    clips: sample.clips.map((clip) => (clip.id === looping.id ? { ...clip, keys: clip.keys.map((key, i) => (i === clip.keys.length - 1 ? { ...key, bones: { ...key.bones, "arm-l": { ...key.bones?.["arm-l"], [field]: 20 } } } : key)) } : clip)),
  });

  it.each(["abductionDeg", "twistDeg"] as const)("refuses a looping clip whose closing key differs in %s", (field) => {
    expect(closing(open(field))).toBeGreaterThan(closing(sample));
  });
});

describe("🧊️ a third-party library recomputes the displacements and extents", () => {
  it("solves the chain with numpy and measures the shapes with shapely", { timeout: 300_000 }, () => {
    const cases: Record<string, unknown> = {};
    const ours: Record<string, { extents: Record<string, Extent>; spread: Record<string, { origin: number[]; tip: number[] }> }> = {};
    for (const c of vectors.cases.filter((candidate) => candidate.transitionAt === undefined)) {
      const offsets = offsetsOf(c.pose);
      cases[c.pose] = { offsets, root, lift: 0.96, shapes: { front: vectors.shapes.front, plan: vectors.shapes.plan } };
      const frames = solveFrames(asset.rig, offsets, root) ?? {};
      ours[c.pose] = {
        extents: extents(stateOf(c)),
        spread: Object.fromEntries(Object.entries(frames).map(([id, f]) => { const length = sample.rig.bones.find((bone) => bone.id === id)!.lengthM; return [id, { origin: [f.affine[4], f.affine[5], f.originZ], tip: [f.affine[0] * length + f.affine[4], f.affine[1] * length + f.affine[5], f.tipZ] }]; })),
      };
    }
    const directory = mkdtempSync(join(tmpdir(), "staffage-lateral-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ rig: sample.rig.bones.map((bone) => ({ id: bone.id, parent: bone.parent, x: bone.x, y: bone.y, angleDeg: bone.angleDeg, lengthM: bone.lengthM })), cases }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️lateral-spread/🐍️.py"), ["shapely", "numpy"], [input])) as Record<string, { bones: Record<string, { origin: number[]; tip: number[] }>; extents: Record<string, Extent> }>;
      for (const [pose, got] of Object.entries(ours)) {
        for (const [bone, spread] of Object.entries(got.spread))
          for (let i = 0; i < 3; i++) {
            expect(reference[pose]!.bones[bone]!.origin[i], `${pose} ${bone} origin ${i}`).toBeCloseTo(spread.origin[i]!, 6);
            expect(reference[pose]!.bones[bone]!.tip[i], `${pose} ${bone} tip ${i}`).toBeCloseTo(spread.tip[i]!, 6);
          }
        for (const [key, extent] of Object.entries(reference[pose]!.extents)) expectPart(`${pose} ${key} (shapely)`, got.extents[key]!, extent);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});


describe("🧊️ a bone beside the symmetry plane (Bone.z)", () => {
  const z = vectors.boneZ;
  const beside = (offset: number): Asset => {
    const base = withPoses(sample);
    const authored = base.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
    const bones = base.rig.bones.map((bone) => (bone.id === z.bone ? { ...bone, z: offset } : bone));
    return { ...base, rig: { ...base.rig, bones }, representations: { ...base.representations, [String(vectors.scale)]: { ...authored, geometry: { ...z.shapes, elevation: vectors.shapes.elevation } } } };
  };

  for (const c of z.cases)
    it(c.name, () => {
      const got = extents(evaluate(beside(z.z), { kind: "pose", pose: c.pose }, select(vectors.scale)));
      for (const [key, part] of Object.entries(c.extents)) expectPart(key, got[key]!, part);
    });

  it("leaves the elevation of a chain without spread untouched by z", () => {
    const flat = extents(evaluate(beside(0), { kind: "pose", pose: "stand" }, select(vectors.scale)));
    expect(extents(evaluate(beside(z.z), { kind: "pose", pose: "stand" }, select(vectors.scale)))["elevation/arm"]).toEqual(flat["elevation/arm"]);
  });

  const boneIssues = (candidate: Asset) => validateAsset(candidate).filter((issue) => issue.code === "bone-lateral");

  it.each(z.limits)("$name", (limit) => {
    expect(boneIssues(beside(limit.z)).length > 0).toBe(limit.refused);
  });
});

describe("🧊️ the herring gull spreads its wings", () => {
  const gull = catalog().find((candidate) => candidate.id === "herring-gull")!;
  const wide = (pose: string, view: "front" | "plan"): { near: number; far: number } => {
    const state = evaluate(gull, { kind: "pose", pose }, select(50));
    const lateral = (id: string): Extent => extentOf(state.shapes[view].filter((evaluated) => evaluated.shape.id === id), true)!;
    const side = (a: string, b: string) => ({ lo: Math.min(lateral(a)[view === "front" ? "minX" : "minY"], lateral(b)[view === "front" ? "minX" : "minY"]), hi: Math.max(lateral(a)[view === "front" ? "maxX" : "maxY"], lateral(b)[view === "front" ? "maxX" : "maxY"]) });
    return { near: -side("near-arm", "near-hand").lo, far: side("far-arm", "far-hand").hi };
  };

  for (const view of ["front", "plan"] as const)
    it(`reaches further sideways in ${view} while soaring and landing than standing, and mirrors the near and far wing`, () => {
      const standing = wide("standing", view);
      for (const pose of ["gliding", "gliding-up", "gliding-down", "approach-a", "flare-key", "landing", "touchdown"]) {
        const spread = wide(pose, view);
        expect(spread.near, `${pose} near ${view}`).toBeGreaterThan(standing.near + 0.1);
        expect(spread.far, `${pose} far ${view}`).toBeGreaterThan(standing.far + 0.1);
        expect(Math.abs(spread.near - spread.far), `${pose} mirrored ${view}`).toBeLessThan(0.08);
      }
    });
});
