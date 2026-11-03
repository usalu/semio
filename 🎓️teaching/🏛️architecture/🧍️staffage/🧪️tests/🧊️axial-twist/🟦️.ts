/** 🧊️ Axial twist: `twistDeg` turns a revolute bone and its subtree about its own longitudinal axis (slew, swivel, steering); elevation, front and plan are projections of the same three-dimensional chain, clips interpolate it and the validator bounds it.
 *
 * THE REFERENCE is numpy (three-dimensional bone chain with rotation matrices) and shapely (extents) in `../🧊️lateral-spread/🐍️.py`, which solves spread and twist together.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️axial-twist/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, BoneOffset, Pose, Shape, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { solveFrames } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool, python } from "../🧰️support/🧰️.ts";
import { ringArea } from "../../🔨️modules/🔺️polygon/🟦️.ts";
import { catalog, extentOf, type Extent } from "../🧊️three-views/🧰️.ts";
requireTool("uv");

type Part = Partial<Extent> & { area?: number };
type Measured = Extent & { area: number };
type Vectors = {
  extentToleranceM: number;
  asset: string;
  scale: number;
  poses: Record<string, { bones: Record<string, BoneOffset> }>;
  shapes: Record<Viewpoint, Shape[]>;
  cases: { name: string; pose: string; transitionAt?: number; extents: Record<string, Part>; rings?: Record<string, { count: number; vertices?: number; area: number }> }[];
  frames: { name: string; pose: string; bone: string; originWorld: { x: number; y: number }; tipWorld: { x: number; y: number }; originZ: number; tipZ: number }[];
  limits: { name: string; bone: string; joint: "revolute" | "fixed" | "prismatic"; twistMinDeg?: number; twistMaxDeg?: number; twistDeg: number; refused: boolean }[];
  fullTurnJoint: { bone: string; twistMinDeg: number; twistMaxDeg: number };
};

const vectors = fixture<Vectors>("🧊️axial-twist");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;
const stand = sample.poses.find((pose) => pose.id === "stand")!;
const root = { x: 0, y: 0.96, angleDeg: 0 };

const asset: Asset = (() => {
  const authored = sample.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
  const poses: Pose[] = [...sample.poses, ...Object.entries(vectors.poses).map(([id, pose]) => ({ ...stand, id, bones: pose.bones }))];
  const rig = { ...sample.rig, bones: sample.rig.bones.map((bone) => (bone.id === vectors.fullTurnJoint.bone ? { ...bone, joint: { ...bone.joint, twistMinDeg: vectors.fullTurnJoint.twistMinDeg, twistMaxDeg: vectors.fullTurnJoint.twistMaxDeg } } : bone)) };
  return { ...sample, rig, poses, representations: { ...sample.representations, [String(vectors.scale)]: { ...authored, geometry: vectors.shapes } } };
})();

const stateOf = (c: Vectors["cases"][number]): EvaluatedState =>
  c.transitionAt === undefined ? evaluate(asset, { kind: "pose", pose: c.pose }, select(vectors.scale)) : evaluate(asset, { kind: "transition", from: "stand", to: c.pose, durationS: 1, timeS: c.transitionAt }, select(vectors.scale));

const extents = (state: EvaluatedState): Record<string, Measured> => {
  const out: Record<string, Measured> = {};
  for (const view of ["elevation", "front", "plan"] as const) for (const evaluated of state.shapes[view]) out[`${view}/${evaluated.shape.id}`] = { ...extentOf([evaluated], true)!, area: evaluated.rings.reduce((sum, ring) => sum + ringArea(ring), 0) };
  return out;
};

const expectPart = (label: string, actual: Measured, expected: Part): void => {
  for (const [edge, value] of Object.entries(expected)) expect(Math.abs(actual[edge as keyof Measured] - value), `${label} ${edge}: ${actual[edge as keyof Measured]} vs ${value}`).toBeLessThanOrEqual(vectors.extentToleranceM);
};

const offsetsOf = (pose: string): Record<string, BoneOffset> => vectors.poses[pose]!.bones;

describe("🧊️ the vectors of the axial twist", () => {
  for (const c of vectors.cases)
    it(c.name, () => {
      const got = extents(stateOf(c));
      for (const [key, part] of Object.entries(c.extents)) expectPart(key, got[key]!, part);
      const state = stateOf(c);
      for (const [key, expected] of Object.entries(c.rings ?? {})) {
        const [view, id] = key.split("/") as [Viewpoint, string];
        const rings = state.shapes[view].find((evaluated) => evaluated.shape.id === id)!.rings;
        expect(rings.length, `${key} rings`).toBe(expected.count);
        if (expected.vertices !== undefined) expect(rings[0]!.length, `${key} vertices`).toBe(expected.vertices);
        expect(Math.abs(rings.reduce((sum, ring) => sum + ringArea(ring), 0) - expected.area), `${key} area`).toBeLessThanOrEqual(1e-5);
      }
    });

  it("returns no frames when no bone is twisted", () => {
    expect(solveFrames(asset.rig, {}, root)).toBeNull();
  });

  it.each(vectors.frames)("places $name at the hand-computed projected and lateral position", (d) => {
    const frame = solveFrames(asset.rig, offsetsOf(d.pose), root)![d.bone]!;
    const length = sample.rig.bones.find((bone) => bone.id === d.bone)!.lengthM;
    expect(frame.affine[4], "origin x").toBeCloseTo(d.originWorld.x, 5);
    expect(frame.affine[5], "origin y").toBeCloseTo(d.originWorld.y, 5);
    expect(frame.affine[0] * length + frame.affine[4], "tip x").toBeCloseTo(d.tipWorld.x, 5);
    expect(frame.affine[1] * length + frame.affine[5], "tip y").toBeCloseTo(d.tipWorld.y, 5);
    expect(frame.originZ, "origin z").toBeCloseTo(d.originZ, 5);
    expect(frame.tipZ, "tip z").toBeCloseTo(d.tipZ, 5);
  });

  it("accumulates the twist of a parent and a child about the same axis", () => {
    const a = solveFrames(asset.rig, { torso: { twistDeg: 30 }, head: { twistDeg: 60 } }, root)!.head!;
    const b = solveFrames(asset.rig, { torso: { twistDeg: 90 } }, root)!.head!;
    expect(a.affine[0]).toBeCloseTo(b.affine[0], 9);
    expect(a.affine[3]).toBeCloseTo(b.affine[3], 9);
  });
});

describe("🧊️ the validator bounds the twist", () => {
  const jointIssues = (candidate: Asset) => validateAsset(candidate).filter((issue) => issue.code === "joint-limit");
  const baseline = jointIssues(sample).length;

  it.each(vectors.limits)("$name", (limit) => {
    const bones = sample.rig.bones.map((bone) => (bone.id === limit.bone ? { ...bone, joint: limit.joint === "revolute" ? { ...bone.joint, ...(limit.twistMinDeg === undefined ? {} : { twistMinDeg: limit.twistMinDeg }), ...(limit.twistMaxDeg === undefined ? {} : { twistMaxDeg: limit.twistMaxDeg }) } : limit.joint === "fixed" ? { type: "fixed" as const } : { type: "prismatic" as const, axisDeg: 0, minM: -1, maxM: 1 } } : bone));
    const poses = sample.poses.map((pose) => (pose.id === "stand" ? { ...pose, bones: { ...pose.bones, [limit.bone]: { twistDeg: limit.twistDeg } } } : pose));
    const issues = jointIssues({ ...sample, rig: { ...sample.rig, bones }, poses });
    expect(issues.length - baseline > 0, issues.map((issue) => issue.path).join(",")).toBe(limit.refused);
  });
});

describe("🧊️ a third-party library recomputes the twisted chain and extents", () => {
  it("solves the chain with numpy and measures the shapes with shapely", { timeout: 300_000 }, () => {
    const cases: Record<string, unknown> = {};
    const ours: Record<string, { extents: Record<string, Measured>; chain: Record<string, { origin: number[]; tip: number[] }> }> = {};
    for (const c of vectors.cases.filter((candidate) => candidate.transitionAt === undefined)) {
      const offsets = offsetsOf(c.pose);
      cases[c.pose] = { offsets, root, lift: 0.96, shapes: { elevation: vectors.shapes.elevation, front: vectors.shapes.front, plan: vectors.shapes.plan } };
      const frames = solveFrames(asset.rig, offsets, root) ?? {};
      ours[c.pose] = {
        extents: extents(stateOf(c)),
        chain: Object.fromEntries(
          Object.entries(frames).map(([id, f]) => {
            const length = sample.rig.bones.find((bone) => bone.id === id)!.lengthM;
            return [id, { origin: [f.affine[4], f.affine[5], f.originZ], tip: [f.affine[0] * length + f.affine[4], f.affine[1] * length + f.affine[5], f.tipZ] }];
          }),
        ),
      };
    }
    const directory = mkdtempSync(join(tmpdir(), "staffage-twist-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ rig: sample.rig.bones.map((bone) => ({ id: bone.id, parent: bone.parent, x: bone.x, y: bone.y, angleDeg: bone.angleDeg, lengthM: bone.lengthM })), cases }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️lateral-spread/🐍️.py"), ["shapely", "numpy"], [input])) as Record<string, { bones: Record<string, { origin: number[]; tip: number[] }>; extents: Record<string, Measured> }>;
      for (const [pose, got] of Object.entries(ours)) {
        for (const [bone, chain] of Object.entries(got.chain))
          for (let i = 0; i < 3; i++) {
            expect(reference[pose]!.bones[bone]!.origin[i], `${pose} ${bone} origin ${i}`).toBeCloseTo(chain.origin[i]!, 6);
            expect(reference[pose]!.bones[bone]!.tip[i], `${pose} ${bone} tip ${i}`).toBeCloseTo(chain.tip[i]!, 6);
          }
        for (const [key, extent] of Object.entries(reference[pose]!.extents)) expectPart(`${pose} ${key} (shapely)`, got.extents[key]!, extent);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ the tower crane slews its jib", () => {
  const crane = catalog().find((candidate) => candidate.id === "tower-crane")!;
  const evaluated = (pose: string) => evaluate(crane, { kind: "pose", pose }, select(100));

  it("turns the jib of the parked weathervane pose by 60 degrees: half as long in the side view, 55 sin 60 deg to the side", () => {
    const parked = evaluated("parked-weathervane");
    const jib = parked.bones.jib!;
    expect(Math.abs(jib.tip.x - jib.origin.x), "projected jib length").toBeCloseTo(55 * Math.cos((60 * Math.PI) / 180), 6);
    const frame = solveFrames(crane.rig, crane.poses.find((pose) => pose.id === "parked-weathervane")!.bones, { x: 0, y: 0, angleDeg: 0 })!.jib!;
    expect(frame.tipZ - frame.originZ, "lateral reach").toBeCloseTo(55 * Math.sin((60 * Math.PI) / 180), 6);
    const e = extentOf(parked.shapes.elevation)!;
    const flat = extentOf(evaluated("lifting-load").shapes.elevation)!;
    expect(e.maxX - e.minX, "the side view shrinks").toBeLessThan(flat.maxX - flat.minX);
  });

  it("keeps the other poses of the crane unslewed", () => {
    for (const pose of ["lifting-load", "max-reach", "skip-at-ground"]) expect(solveFrames(crane.rig, crane.poses.find((candidate) => candidate.id === pose)!.bones, { x: 0, y: 0, angleDeg: 0 })).toBeNull();
  });
});
