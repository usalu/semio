/** 🎞️ The shared timeline of the playgrounds: where a placement is in its clip at one instant, how that instant is baked into the 3D scene's animation, and that the three playgrounds read one position.
 * Independent checks: `three` interpolates the keyframe tracks (linear and quaternion) that the retimed channels and the original clip are sampled with, and the library's own `evaluate` gives the height of the figure that the composed 3D scene must show at the same instant.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import { Quaternion, QuaternionKeyframeTrack, VectorKeyframeTrack, Vector3 } from "three";
import { evaluate } from "@teaching/staffage";
import type { Channel, ModelDocument } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { poseModel, validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { prepareModel } from "../../🔨️modules/🧊️spatial/🏭️prepare/🟦️.ts";
import { clipDurationS, readTrack, timelineLengthS, windowEndS, type TrackInput } from "../../🔨️modules/🧪️playground/🎞️timeline/🟦️.ts";
import { retimeChannel, sampleChannel, type Window } from "../../🔨️modules/🧪️playground/🧊️model/🎞️retime/🟦️.ts";
import { composePlayground, placeNode, placementOf, TIMELINE_ANIMATION } from "../../🔨️modules/🧪️playground/🧊️model/🧩️compose/🟦️.ts";
import { modelQueryOf } from "../../🔨️modules/🧪️playground/🧊️model/🟦️.tsx";
import { dragPosition, extentM, hitBox, stageGeometry } from "../../🔨️modules/🧪️playground/📐️geometry/🟦️.ts";
import { linePosition } from "../../🔨️modules/🧪️playground/🧱️compose/🟦️.ts";
import { INITIAL_STATE, reduce, sanitize, TRACK_DEFAULTS, STORAGE_VERSION, type Placement } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { VIEWS } from "@teaching/staffage";
import { parseAddress, formatAddress, playgroundFromParams, playgroundToParams, routePath } from "../../🔨️modules/🗺️routing/🟦️.ts";
import { sampleFigure } from "../🧰️support/🧰️.ts";

const track = (patch: Partial<TrackInput> = {}): TrackInput => ({ ...TRACK_DEFAULTS, offsetS: 0, ...patch });

describe("🎞️ reading a placement at a timeline instant", () => {
  it("plays inside its window, holds before and after it, repeats or holds at the clip's ends", () => {
    expect(readTrack(track(), 1, 0.25, 10)).toEqual({ clipTimeS: 0.25, cycleS: 0.25, phase: "running" });
    expect(readTrack(track(), 1, 2.75, 10)).toMatchObject({ clipTimeS: 2.75, phase: "running" });
    expect(readTrack(track(), 1, 2.75, 10).cycleS).toBeCloseTo(0.75, 9);
    expect(readTrack(track({ startS: 2 }), 1, 1, 10)).toEqual({ clipTimeS: 0, cycleS: 0, phase: "waiting" });
    expect(readTrack(track({ startS: 1, durationS: 2 }), 1, 5, 10)).toMatchObject({ clipTimeS: 2, phase: "finished" });
    expect(readTrack(track({ speed: 2 }), 1, 0.75, 10)).toMatchObject({ clipTimeS: 1.5, cycleS: 0.5 });
    expect(readTrack(track({ offsetS: 0.5 }), 1, 0.25, 10).clipTimeS).toBeCloseTo(0.75, 9);
    expect(readTrack(track({ loop: false }), 1, 3, 10)).toMatchObject({ clipTimeS: 1, cycleS: 1 });
    expect(readTrack(track({ holdS: 0.4 }), 1, 7, 10)).toEqual({ clipTimeS: 0.4, cycleS: 0.4, phase: "held" });
    expect(readTrack(track(), 0, 3, 10)).toEqual({ clipTimeS: 0, cycleS: 0, phase: "static" });
  });

  it("sizes the timeline by the longest window unless the reader sets it", () => {
    const rows = [{ ...TRACK_DEFAULTS, clipS: 1 }, { ...TRACK_DEFAULTS, startS: 2, clipS: 4, speed: 2 }, { ...TRACK_DEFAULTS, startS: 1, durationS: 7, clipS: 1 }];
    expect(timelineLengthS(rows, null)).toBe(8);
    expect(timelineLengthS(rows, 20)).toBe(20);
    expect(timelineLengthS([], null)).toBe(1);
    expect(windowEndS(TRACK_DEFAULTS, 6)).toBe(6);
    expect(windowEndS({ startS: 1, durationS: 2 }, 6)).toBe(3);
    expect(clipDurationS(sampleFigure, null)).toBe(sampleFigure.clips.find((clip) => clip.id === sampleFigure.presentation.defaultClip)!.durationS);
    expect(clipDurationS(sampleFigure, { kind: "pose", pose: "stand" })).toBe(0);
    expect(clipDurationS(sampleFigure, { kind: "clip", clip: "gone" })).toBe(clipDurationS(sampleFigure, null));
  });
});

const keyed = (path: "translation" | "rotation", times: readonly number[], values: readonly number[]): Channel => ({ target: { node: "n", path }, interpolation: "linear", times, values });
const move = keyed("translation", [0, 0.25, 0.5, 1], [0, 0, 0, 1, 2, 3, -1, 0.5, 2, 0, 0, 0]);
const half = Math.SQRT1_2;
const turn = keyed("rotation", [0, 0.5, 1], [0, 0, 0, 1, 0, half, 0, half, 0, 0, 0, 1]);

const threeSample = (channel: Channel, timeS: number): number[] => {
  const track = channel.target && "node" in channel.target && channel.target.path === "rotation" ? new QuaternionKeyframeTrack(".quaternion", [...channel.times], [...channel.values]) : new VectorKeyframeTrack(".position", [...channel.times], [...channel.values]);
  return Array.from((track as unknown as { createInterpolant(): { evaluate(timeS: number): ArrayLike<number> } }).createInterpolant().evaluate(timeS));
};
const close = (a: readonly number[], b: readonly number[], eps = 1e-5): void => a.forEach((value, index) => expect(Math.abs(value - b[index]!)).toBeLessThan(eps));
const asChannel = (channel: Channel, keyedTimes: { readonly times: readonly number[]; readonly values: readonly number[] }): Channel => ({ ...channel, times: keyedTimes.times, values: keyedTimes.values });

describe("🕰️ the clip laid on the timeline", () => {
  const timelineS = 8;
  const windows: readonly [string, Window][] = [
    ["a looping window with an offset at double speed", { startS: 1, endS: 4, speed: 2, offsetS: 0.25, loop: true, holdS: null, clipS: 1 }],
    ["a window that runs to the end of the timeline", { startS: 0, endS: timelineS, speed: 1, offsetS: 0, loop: true, holdS: null, clipS: 1 }],
    ["a clip that holds its last frame", { startS: 2, endS: 7, speed: 1, offsetS: 0, loop: false, holdS: null, clipS: 1 }],
    ["a slow window", { startS: 0.5, endS: 6, speed: 0.4, offsetS: 0.1, loop: true, holdS: null, clipS: 1 }],
  ];
  for (const [name, window] of windows) {
    it(`gives, for ${name}, at every instant the value of the clip at the time the rules read (three interpolates both)`, () => {
      for (const channel of [move, turn]) {
        const retimed = asChannel(channel, retimeChannel(channel, window, timelineS));
        expect(retimed.times.every((time, index) => index === 0 || time > retimed.times[index - 1]!)).toBe(true);
        for (let timeS = 0; timeS <= timelineS + 1e-9; timeS += 0.0625) {
          const reading = readTrack({ ...TRACK_DEFAULTS, startS: window.startS, durationS: window.endS - window.startS, speed: window.speed, loop: window.loop, holdS: window.holdS, offsetS: window.offsetS }, window.clipS, timeS, timelineS);
          const wanted = threeSample(channel, reading.cycleS);
          const got = threeSample(retimed, timeS);
          const same = reading.cycleS === 0 && timeS > 0 && Math.abs(timeS % 1) < 1e-9;
          if (!same) close(got, wanted);
        }
      }
    });
  }

  it("freezes a held placement at one clip time for the whole timeline", () => {
    const window: Window = { startS: 0, endS: timelineS, speed: 1, offsetS: 0, loop: true, holdS: 0.375, clipS: 1 };
    const retimed = retimeChannel(move, window, timelineS);
    expect(retimed.times).toEqual([0, timelineS]);
    close(retimed.values.slice(0, 3), threeSample(move, 0.375));
    close(retimed.values.slice(3), threeSample(move, 0.375));
    close([...sampleChannel(move, 0.375)], threeSample(move, 0.375));
    close([...sampleChannel(turn, 0.25)], threeSample(turn, 0.25));
  });
});

const place = (patch: Partial<Placement>): Placement => ({ id: "p", asset: "sample-figure", xM: 0, zM: 0, yawDeg: 0, pick: null, offsetS: 0, mirrored: false, ...TRACK_DEFAULTS, ...patch });

describe("🧊️ the 3D scene of the placements", () => {
  let walk: ModelDocument;
  let stand: ModelDocument;
  const timelineS = 3;
  const build = async (): Promise<ModelDocument> => {
    walk ??= await prepareModel(sampleFigure, { view: "light-solid", scale: 100, language: "en", query: { kind: "clip", clip: "walk", timeS: 0 }, scales: [100] });
    stand ??= await prepareModel(sampleFigure, { view: "light-solid", scale: 100, language: "en", query: { kind: "pose", pose: "stand" }, scales: [100] });
    return composePlayground(
      [
        { id: "a", document: walk, xM: 2, zM: 1, yawDeg: 0, animation: "walk", pose: null, window: { startS: 0, endS: timelineS, speed: 1, offsetS: 0, loop: true, holdS: null, clipS: 1 }, plantNode: sampleFigure.rig.root },
        { id: "b", document: stand, xM: 5, zM: -1, yawDeg: 90, animation: null, pose: "stand", window: null, plantNode: sampleFigure.rig.root },
      ],
      { id: "playground", timelineS, floor: { minXM: 0, maxXM: 8, minZM: -3, maxZM: 3, stepM: 1 } },
    );
  };

  it("is one valid document with every identifier of a placement under its own prefix, one scene per mode and one master animation", async () => {
    const document = await build();
    expect(validateModel(document).filter((issue) => issue.severity === "error")).toEqual([]);
    expect(new Set(document.nodes.map((node) => node.id)).size).toBe(document.nodes.length);
    expect(document.nodes.filter((node) => node.parent === null).map((node) => node.id).sort()).toEqual(["floor", "slot-0", "slot-1"]);
    expect(document.scenes!.map((scene) => scene.id).sort()).toEqual(["s100-body", "s100-structure", "s100-thermal"]);
    expect(document.scenes!.every((scene) => scene.nodes.includes("floor") && scene.nodes.some((node) => node.startsWith("p0-")) && scene.nodes.some((node) => node.startsWith("p1-")))).toBe(true);
    expect(document.animations.map((animation) => animation.id)).toEqual([TIMELINE_ANIMATION]);
    expect(document.variants.map((variant) => variant.id).sort()).toEqual([...VIEWS].sort());
    expect(placementOf("p0-pelvis", ["a", "b"])).toBe("a");
    expect(placementOf("p1-torso", ["a", "b"])).toBe("b");
    expect(placementOf("slot-1", ["a", "b"])).toBeNull();
    expect(placementOf("floor", ["a", "b"])).toBeNull();
  });

  it("puts a placement at (z, 0, x) of the neutral frame and turns a positive yaw to the asset's left", () => {
    expect(placeNode(0, 2, 1, 0).translation).toEqual([1, 0, 2]);
    const quaternion = new Quaternion(...(placeNode(1, 5, -1, 90).rotation as [number, number, number, number]));
    const forward = new Vector3(0, 0, 1).applyQuaternion(quaternion);
    expect(forward.x).toBeCloseTo(1, 9);
    expect(forward.z).toBeCloseTo(0, 9);
  });

  it("shows at every instant the figure the evaluator evaluates: planted on its ground point, at the height of the clip", async () => {
    const document = await build();
    for (const timeS of [0, 1 / 3, 0.5, 0.9, 1.4, 2.7]) {
      const world = poseModel(document, { animation: TIMELINE_ANIMATION, timeS }).world["p0-pelvis"]!;
      const evaluated = evaluate(sampleFigure, { kind: "clip", clip: "walk", timeS });
      const pelvis = evaluated.bones[sampleFigure.rig.root]!;
      expect(world[12]).toBeCloseTo(1 + 0, 3);
      expect(world[14]).toBeCloseTo(2, 3);
      expect(Math.abs(world[13]! - pelvis.origin.y)).toBeLessThan(2e-3);
    }
    const standing = poseModel(document, { animation: TIMELINE_ANIMATION, timeS: 1 }).world["p1-pelvis"]!;
    expect(standing[12]).toBeCloseTo(-1, 3);
    expect(standing[14]).toBeCloseTo(5, 3);
  });

  it("follows the window of the placement: held before its start, paused when held", async () => {
    walk ??= await prepareModel(sampleFigure, { view: "light-solid", scale: 100, language: "en", query: { kind: "clip", clip: "walk", timeS: 0 }, scales: [100] });
    const part = (window: Window) => ({ id: "a", document: walk, xM: 0, zM: 0, yawDeg: 0, animation: "walk", pose: null, window, plantNode: sampleFigure.rig.root });
    const delayed = composePlayground([part({ startS: 1.5, endS: timelineS, speed: 1, offsetS: 0, loop: true, holdS: null, clipS: 1 })], { id: "d", timelineS, floor: null });
    const at = (document: ModelDocument, timeS: number) => poseModel(document, { animation: TIMELINE_ANIMATION, timeS }).world["p0-torso"]!;
    close(at(delayed, 0), at(delayed, 1.4), 1e-6);
    expect(Math.max(...at(delayed, 1.5).map((value, index) => Math.abs(value - at(delayed, 2)[index]!)))).toBeGreaterThan(1e-3);
    const held = composePlayground([part({ startS: 0, endS: timelineS, speed: 1, offsetS: 0, loop: true, holdS: 0.4, clipS: 1 })], { id: "h", timelineS, floor: null });
    close(at(held, 0.1), at(held, 2.9), 1e-6);
    const still = poseModel(held, { animation: TIMELINE_ANIMATION, timeS: 1 }).world["p0-torso"]!;
    const clip = evaluate(sampleFigure, { kind: "clip", clip: "walk", timeS: 0.4 }).bones.torso!;
    expect(Math.abs(still[13]! - clip.origin.y)).toBeLessThan(2e-3);
  });

  it("builds the model query of a placement from its pick, never from a pose or clip the asset no longer has", () => {
    expect(modelQueryOf(sampleFigure, null)).toEqual({ kind: "clip", clip: sampleFigure.presentation.defaultClip, timeS: 0 });
    expect(modelQueryOf(sampleFigure, { kind: "pose", pose: "stand" })).toEqual({ kind: "pose", pose: "stand" });
    expect(modelQueryOf(sampleFigure, { kind: "clip", clip: "gone" })).toEqual({ kind: "clip", clip: sampleFigure.presentation.defaultClip, timeS: 0 });
    expect(modelQueryOf(sampleFigure, { kind: "pose", pose: "gone" }).kind).toBe("clip");
  });
});

describe("📐️ one position in three playgrounds", () => {
  const ground = { lengthM: 24, depthM: 12 };
  it("projects x ahead and z to the left through the section, the plan and the 3D scene", () => {
    expect(linePosition("elevation", { xM: 3, zM: 1.5 }, ground)).toEqual({ horizontalM: 3, liftM: 0, nearness: -1.5 });
    expect(linePosition("front", { xM: 3, zM: 1.5 }, ground)).toEqual({ horizontalM: 7.5, liftM: 0, nearness: 3 });
    expect(linePosition("plan", { xM: 3, zM: 1.5 }, ground)).toEqual({ horizontalM: 3, liftM: 1.5, nearness: 0 });
    expect(placeNode(0, 3, 1.5, 0).translation).toEqual([1.5, 0, 3]);
  });

  it("sizes each stage from the ground and puts z = 0 on the middle row of the plan", () => {
    expect(extentM("front", ground)).toEqual({ acrossM: 12, downM: 0 });
    expect(extentM("plan", ground)).toEqual({ acrossM: 24, downM: 12 });
    const plan = stageGeometry("plan", ground, 2, 40);
    expect(plan.widthPx).toBe(48 + 24 * 40);
    expect(plan.groundPx).toBe(24 + 6 * 40);
    const side = stageGeometry("elevation", ground, 2, 40);
    expect(side.heightPx - side.groundPx).toBe(34);
  });

  it("drags along x in the side elevation, along z in the front elevation and both in the plan (up is the asset's left), snapped and kept on the ground", () => {
    const start = { xM: 3, zM: 0 };
    expect(dragPosition("elevation", start, { x: 21, y: 99 }, 40, ground)).toEqual({ xM: 3.55 });
    expect(dragPosition("front", start, { x: 40, y: 99 }, 40, ground)).toEqual({ zM: 1 });
    expect(dragPosition("plan", start, { x: 40, y: -80 }, 40, ground)).toEqual({ xM: 4, zM: 2 });
    expect(dragPosition("plan", start, { x: -4000, y: 4000 }, 40, ground)).toEqual({ xM: 0, zM: -6 });
  });

  it("grabs standing figures by height and width and a plan figure by its footprint around its ground point", () => {
    const dimensions = { heightM: 1.8, widthM: 0.6, depthM: 0.4, massKg: 70 };
    expect(hitBox("elevation", dimensions, 100)).toEqual({ left: -30, top: -180, width: 60, height: 180 });
    expect(hitBox("front", dimensions, 100)).toEqual({ left: -25, top: -180, width: 50, height: 180 });
    expect(hitBox("plan", dimensions, 100)).toEqual({ left: -30, top: -25, width: 60, height: 50 });
  });
});

describe("💾️ the shared scene and its address", () => {
  it("clamps every track value and keeps the scene's depth for sideways positions", () => {
    let state = reduce(INITIAL_STATE, { type: "placement-added", placement: place({ id: "p", zM: 99, startS: -3, speed: 100, durationS: 0, holdS: -1 }) });
    const [first] = state.playground.placements;
    expect(first).toMatchObject({ zM: INITIAL_STATE.playground.depthM / 2, startS: 0, speed: 8, durationS: 0.1, holdS: 0 });
    state = reduce(state, { type: "placement-changed", id: "p", patch: { loop: false, holdS: 1.25, speed: 0.5 } });
    expect(state.playground.placements[0]).toMatchObject({ loop: false, holdS: 1.25, speed: 0.5 });
    state = reduce(state, { type: "playground-configured", patch: { depthM: 4, timelineS: 0.2 } });
    expect(state.playground.placements[0]!.zM).toBe(2);
    expect(state.playground.timelineS).toBe(1);
    state = reduce(state, { type: "playground-configured", patch: { timelineS: null } });
    expect(state.playground.timelineS).toBeNull();
  });

  it("reads stored scenes defensively: missing track members get their defaults", () => {
    const state = sanitize({ version: STORAGE_VERSION, state: { playground: { depthM: 99999, camera: "nope", timelineS: "x", placements: [{ id: "q", asset: "a-b", speed: "fast", durationS: 5, loop: false }] } } }, VIEWS);
    expect(state.playground.depthM).toBe(200);
    expect(state.playground.camera).toBe("perspective");
    expect(state.playground.timelineS).toBeNull();
    expect(state.playground.placements[0]).toMatchObject({ speed: 1, durationS: 5, loop: false, startS: 0, holdS: null });
  });

  it("keeps the three playgrounds at their own addresses and the scene in every one of them", () => {
    expect(parseAddress("#/playground").route).toEqual({ page: "playground" });
    expect(parseAddress("#/playground/plan").route).toEqual({ page: "playground", tab: "plan" });
    expect(parseAddress("#/playground/3d?cam=front").route).toEqual({ page: "playground", tab: "model" });
    expect(routePath({ page: "playground", tab: "model" })).toBe("/playground/3d");
    expect(routePath({ page: "playground", tab: "section" })).toBe("/playground");
    expect(formatAddress({ page: "playground", tab: "plan" }, new URLSearchParams("len=10"))).toBe("#/playground/plan?len=10");
    const scene = { ...INITIAL_STATE.playground, camera: "plan" as const, depthM: 8, timelineS: 6, placements: [place({ id: "url-0", asset: "park-bench", xM: 2, zM: -1.5, yawDeg: 45, pick: { kind: "clip", clip: "walk" }, offsetS: 0.5, startS: 1, durationS: 3, speed: 2, loop: false, holdS: 0.75 })] };
    const params = playgroundToParams(scene);
    expect(decodeURIComponent(params.get("p")!)).toBe("park-bench~2~clip.walk~0.5~~-1.5~45~1~3~2~n~0.75");
    expect(playgroundFromParams(new URLSearchParams(params.toString()))).toEqual(scene);
  });
});
