/** 🦴️ The interactive 3D model carries a bone that rests laterally: the model the stage prepares holds the lifted clavicle solid and the arm that hangs from it at the hand-computed places, in a pose and through a clip, and the stage shows it.
 *
 * THE REFERENCE is the hand computation of the shared vectors (`consumers`), confirmed by scipy, numpy and shapely in the library test.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../../../🧫️fixtures/🦴️lateral-rest/🔣️.json — the vectors
 * @see ../../../../🧪️tests/🦴️lateral-rest/🟦️.ts — the library test of every other consumer
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { cleanup, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { Asset, AuthoredRepresentation, AuthoredVolume, Bone, BoneOffset, Clip, Pose, Shape } from "@teaching/staffage";
import { poseModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { prepareModel } from "../../🔨️modules/🧊️spatial/🏭️prepare/🟦️.ts";
import { ModelStage, type StageOptions } from "../../🔨️modules/🧊️spatial/🖥️stage/🟦️.ts";
import { sceneOf } from "../../🔨️modules/🧊️spatial/🟦️.ts";

type Triple = [number, number, number];
type Vectors = {
  base: string;
  scale: number;
  girdle: { bones: Bone[]; reparent: Record<string, Partial<Bone>> };
  shapes: Record<"elevation" | "front" | "plan", Shape[]>;
  cases: { pose: string; offsets: Record<string, BoneOffset> }[];
  consumers: {
    pose: string;
    meshToleranceM: number;
    clip: Clip;
    instants: { timeS: number; tip: Triple }[];
    solids: { body: AuthoredVolume["body"][number]; structure: AuthoredVolume["structure"][number]; thermal: AuthoredVolume["thermal"][number] };
    capsule: { ahead: [number, number]; up: [number, number]; left: [number, number] };
  };
};

const staffage = resolve(import.meta.dirname, "../../../..");
const fixture = <T,>(name: string): T => JSON.parse(readFileSync(resolve(staffage, "🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
const vectors = fixture<Vectors>("🦴️lateral-rest");
const consumers = vectors.consumers;

const fitted: Asset = (() => {
  const sample = fixture<{ asset: Asset }>(vectors.base).asset;
  const stand = sample.poses.find((pose) => pose.id === "stand")!;
  const bones: Bone[] = [...sample.rig.bones.map((bone) => (vectors.girdle.reparent[bone.id] === undefined ? bone : { ...bone, ...vectors.girdle.reparent[bone.id] })), ...vectors.girdle.bones];
  const poses: Pose[] = [...sample.poses, ...vectors.cases.map((c) => ({ ...stand, id: c.pose, bones: c.offsets }))];
  const authored = sample.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
  const volume = sample.volumes![String(vectors.scale) as keyof NonNullable<Asset["volumes"]>] as AuthoredVolume;
  return {
    ...sample,
    rig: { ...sample.rig, bones },
    poses,
    clips: [...sample.clips, consumers.clip],
    thermal: { ...sample.thermal, respiration: { ...sample.thermal.respiration!, motion: [] } },
    representations: { ...sample.representations, [String(vectors.scale)]: { ...authored, geometry: vectors.shapes } },
    volumes: { ...sample.volumes, [String(vectors.scale)]: { ...volume, body: [...volume.body, consumers.solids.body], thermal: [...volume.thermal, consumers.solids.thermal], structure: [...volume.structure, consumers.solids.structure] } },
    exports: [...sample.exports, "glb", "fbx", "model"],
  } as Asset;
})();

const request = { view: "light-solid", scale: vectors.scale, query: { kind: "pose", pose: consumers.pose }, language: "en" } as const;
const moving = { ...request, query: { kind: "clip", clip: consumers.clip.id, timeS: 0 } } as const;
const half = consumers.instants.find((instant) => instant.timeS === 0.5)!;
const point = (world: readonly number[], x: number, y: number, z: number): Triple => [world[0]! * x + world[4]! * y + world[8]! * z + world[12]!, world[1]! * x + world[5]! * y + world[9]! * z + world[13]!, world[2]! * x + world[6]! * y + world[10]! * z + world[14]!];
const expectAt = (label: string, neutral: Triple, staffageAxes: Triple, tolerance: number): void => neutral.forEach((value, index) => expect(Math.abs(value - staffageAxes[2 - index]!), `${label}[${index}]: ${value} vs ${staffageAxes[2 - index]}`).toBeLessThanOrEqual(tolerance));

describe("🦴️ the prepared model of the stage holds the lifted clavicle at its place", () => {
  it("lays the tessellated clavicle solid of the body scene in the hand-computed box and the arm on the clavicle tip", async () => {
    const document = await prepareModel(fitted, request);
    const posed = poseModel(document, consumers.pose, { scene: sceneOf(request) });
    expect(posed.issues).toEqual([]);
    expectAt("clavicle tip", point(posed.world["clavicle-l"]!, 0.17, 0, 0), half.tip, 1e-9);
    expectAt("arm origin", point(posed.world["arm-l"]!, 0, 0, 0), half.tip, 1e-9);
    const scene = document.scenes!.find((candidate) => candidate.id === sceneOf(request))!;
    const carriers = document.nodes.filter((node) => node.mesh !== undefined && scene.nodes.includes(node.id) && (node.id === "clavicle-l" || node.parent === "clavicle-l") && !fitted.rig.bones.some((bone) => bone.id === node.id && bone.id !== "clavicle-l"));
    expect(carriers.length, "the body scene has a mesh on the clavicle").toBeGreaterThan(0);
    const low: Triple = [Infinity, Infinity, Infinity];
    const high: Triple = [-Infinity, -Infinity, -Infinity];
    for (const node of carriers) for (const primitive of document.meshes.find((mesh) => mesh.id === node.mesh)!.primitives.filter((candidate) => candidate.mode === "triangles")) {
      expect(primitive.joints, "the clavicle solid is rigid on its node").toBeUndefined();
      for (let i = 0; i < primitive.positions.length; i += 3) point(posed.world[node.id]!, primitive.positions[i]!, primitive.positions[i + 1]!, primitive.positions[i + 2]!).forEach((value, axis) => {
        low[axis] = Math.min(low[axis]!, value);
        high[axis] = Math.max(high[axis]!, value);
      });
    }
    const got = { left: [low[0], high[0]], up: [low[1], high[1]], ahead: [low[2], high[2]] } as const;
    for (const axis of ["ahead", "up", "left"] as const) for (const end of [0, 1] as const) expect(Math.abs(got[axis][end] - consumers.capsule[axis][end]), `${axis}[${end}]: ${got[axis][end]} vs ${consumers.capsule[axis][end]}`).toBeLessThanOrEqual(consumers.meshToleranceM);
  });

  it("passes the hand-computed tips at a quarter and at half a second of the clip", async () => {
    const document = await prepareModel(fitted, moving);
    for (const instant of consumers.instants) {
      const sampled = poseModel(document, { animation: consumers.clip.id, timeS: instant.timeS }, { scene: sceneOf(moving) });
      expect(sampled.issues).toEqual([]);
      expectAt(`clavicle tip at ${instant.timeS} s`, point(sampled.world["clavicle-l"]!, 0.17, 0, 0), instant.tip, 1e-4);
      expectAt(`arm origin at ${instant.timeS} s`, point(sampled.world["arm-l"]!, 0, 0, 0), instant.tip, 1e-4);
    }
  });
});

describe("🖥️ the stage shows it", () => {
  const hosts: HTMLElement[] = [];
  const stages: ModelStage[] = [];
  beforeEach(() => {
    vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, left: 0, top: 0, right: 160, bottom: 120, width: 160, height: 120, toJSON: () => ({}) });
    vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
  });
  afterEach(() => {
    for (const stage of stages.splice(0)) stage.dispose();
    for (const host of hosts.splice(0)) host.remove();
    cleanup();
    vi.restoreAllMocks();
  });

  it("becomes ready with the lifted shoulders as a pose and as a clip and reports no error", async () => {
    const host = document.createElement("div");
    document.body.append(host);
    hosts.push(host);
    const stage = new ModelStage(host, { ...request, asset: fitted, preset: "front", settleMs: 0 } as StageOptions);
    stages.push(stage);
    const phase = (): string | null => host.querySelector("[data-model-stage]")!.getAttribute("data-state");
    await waitFor(() => expect(phase()).toBe("ready"), { timeout: 60000 });
    expect(host.querySelector('[role="alert"]')?.textContent ?? "").toBe("");
    stage.update({ query: moving.query });
    await waitFor(() => expect(phase()).toBe("ready"), { timeout: 60000 });
    expect(host.querySelector('[role="alert"]')?.textContent ?? "").toBe("");
  });
});
