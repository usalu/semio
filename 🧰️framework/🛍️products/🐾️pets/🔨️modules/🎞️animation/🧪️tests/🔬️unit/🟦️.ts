/** 🎞️ Unit suite of the animation module: easing against d3-ease, track and clip sampling, pose blending, the one-tick spring with the gaze constants, the lid of a blink, and the shared vectors of both Protocol v2 cases.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧫️fixtures/🎞️animation-sampling/🔣️.json
 * @see ../../../../🧫️fixtures/🪀️spring-settling/🔣️.json
 */
import { easeCubicIn, easeCubicInOut, easeCubicOut, easeLinear, easeQuadIn, easeQuadOut } from "d3-ease";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import type { Clip, Ease, Species, Track } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { type Pose, restPose, solveRig } from "../../../🦴️rig/🟦️.ts";
import { lerp, smoothstep } from "../../../📐️trigonometry/🟦️.ts";
import { speciesIssues } from "../../../✅️validation/🟦️.ts";
import { BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS, type Spring, blendPose, clipTicks, easeBezier, lidAt, sampleClip, sampleTrack, springStep } from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TOLERANCE = 1e-9;
const THIRD = 1 / 3;

/** 🪜️ Into how many steps the unit interval is cut where an ease is compared with d3-ease at the level of the run. */
const STEPS = sampled(64, 1024, 4096);

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type SamplingVectors = {
  readonly easings: readonly { readonly id: string; readonly ease: Ease; readonly amounts: readonly number[]; readonly expected: readonly number[] }[];
  readonly tracks: readonly { readonly id: string; readonly track: Track; readonly phases: readonly number[]; readonly expected: readonly number[] }[];
  readonly lengths: readonly { readonly id: string; readonly seconds: number; readonly expected: number }[];
  readonly species: Species;
  readonly poses: readonly { readonly id: string; readonly clip: string; readonly ticks: readonly number[]; readonly expected: readonly Pose[] }[];
  readonly blends: readonly { readonly id: string; readonly from: Pose; readonly to: Pose; readonly amounts: readonly number[]; readonly expected: readonly Pose[] }[];
  readonly lids: { readonly ticks: readonly number[]; readonly expected: { readonly blinkTicks: number; readonly closures: readonly number[] } };
};
type Released = { readonly id: string; readonly position: number; readonly velocity: number; readonly target: number; readonly stiffness: number; readonly damping: number };
type SettlingVectors = {
  readonly steps: readonly (Released & { readonly expected: Spring })[];
  readonly runs: readonly (Released & { readonly ticks: readonly number[]; readonly expected: readonly Spring[] })[];
  readonly stables: readonly (Released & { readonly ticks: number; readonly expected: Spring })[];
  readonly unstables: readonly { readonly id: string; readonly stiffness: number; readonly damping: number }[];
  readonly gaze: { readonly stiffness: number; readonly damping: number; readonly band: number; readonly settleTicks: number; readonly overshootBound: number; readonly horizon: number; readonly jumps: readonly { readonly id: string; readonly position: number; readonly target: number; readonly expected: { readonly settled: number; readonly overshoot: number } }[] };
};

const SAMPLING = fixture<SamplingVectors>("🎞️animation-sampling");
const SETTLING = fixture<SettlingVectors>("🪀️spring-settling");
const SPECIES = SAMPLING.species;

/** 🎬️ A clip of the committed species. */
function clipOf(id: string): Clip {
  return SPECIES.clips.find((clip) => clip.id === id)!;
}

/** 🛤️ A track of the body's `x` through the given keys. */
function trackOf(keys: Track["keys"], channel: Track["channel"] = "x"): Track {
  return { bone: "body", channel, keys };
}

/** ⏭️ A spring after `ticks` single steps. */
function after(spring: Omit<Released, "id">, ticks: number): Spring {
  let state: Spring = { position: spring.position, velocity: spring.velocity };
  for (let tick = 0; tick < ticks; tick++) state = springStep(state.position, state.velocity, spring.target, spring.stiffness, spring.damping);
  return state;
}

/** 📏️ Every channel of every bone of two poses agrees within the tolerance. */
function expectPose(actual: Pose, expected: Pose): void {
  expect(actual.length).toBe(expected.length);
  actual.forEach((bone, index) => {
    for (const channel of ["x", "y", "rotation", "scaleX", "scaleY"] as const) expect(Math.abs(bone[channel] - expected[index]![channel])).toBeLessThanOrEqual(TOLERANCE);
  });
}

describe("easeBezier — CSS cubic-bezier by bisection", () => {
  it("is 0 at and below 0 and 1 at and above 1, whatever the curve", () => {
    for (const ease of [[0.42, 0, 0.58, 1], [0.34, 1.56, 0.64, 1], [0, 1, 1, 0], [1, 0, 0, 1]] as const) {
      for (const amount of [-1, -1e-300, 0]) expect(easeBezier(ease, amount)).toBe(0);
      for (const amount of [1, 1 + 1e-12, 7]) expect(easeBezier(ease, amount)).toBe(1);
    }
  });

  it("reproduces d3-ease where a polynomial ease is a Bézier with a linear abscissa", () => {
    const twins: readonly (readonly [Ease, (amount: number) => number])[] = [
      [[THIRD, THIRD, 2 * THIRD, 2 * THIRD], easeLinear],
      [[THIRD, 0, 2 * THIRD, 0], easeCubicIn],
      [[THIRD, 1, 2 * THIRD, 1], easeCubicOut],
      [[THIRD, 0, 2 * THIRD, THIRD], easeQuadIn],
      [[THIRD, 2 * THIRD, 2 * THIRD, 1], easeQuadOut],
    ];
    for (const [ease, reference] of twins) for (let step = 0; step <= STEPS; step++) expect(Math.abs(easeBezier(ease, step / STEPS) - reference(step / STEPS))).toBeLessThanOrEqual(1e-13);
  });

  it("keeps the CSS ease-in-out in the family of d3's cubic in-out: symmetric, rising and never 0.1 apart", () => {
    const ease: Ease = [0.42, 0, 0.58, 1];
    let previous = 0;
    for (let step = 1; step <= 256; step++) {
      const amount = step / 256;
      const eased = easeBezier(ease, amount);
      expect(eased).toBeGreaterThan(previous);
      expect(Math.abs(eased + easeBezier(ease, 1 - amount) - 1)).toBeLessThanOrEqual(1e-13);
      expect(Math.abs(eased - easeCubicInOut(amount))).toBeLessThan(0.1);
      if (step % 128 !== 0) expect(Math.sign(eased - amount)).toBe(Math.sign(easeCubicInOut(amount) - amount));
      previous = eased;
    }
  });

  it("leaves [0, 1] with curves that anticipate or overshoot", () => {
    expect(easeBezier([0.34, 1.56, 0.64, 1], 0.7)).toBeGreaterThan(1);
    expect(easeBezier([0.36, 0, 0.66, -0.56], 0.3)).toBeLessThan(0);
  });

  it("stays on the curve where the abscissa stands still", () => {
    expect(Math.abs(easeBezier([1, 0, 0, 1], 0.5) - 0.5)).toBeLessThan(1e-5);
  });

  it("is the Hermite ease of `smoothstep` for the control points 1/3, 0, 2/3, 1", () => {
    for (let step = 0; step <= 256; step++) expect(Math.abs(easeBezier([THIRD, 0, 2 * THIRD, 1], step / 256) - smoothstep(step / 256))).toBeLessThanOrEqual(1e-13);
  });
});

describe("sampleTrack — keys, eases and ends", () => {
  const keys = [{ at: 0, value: 2 }, { at: 0.25, value: -6, ease: [0.42, 0, 0.58, 1] as Ease }, { at: 0.75, value: 10 }, { at: 1, value: 2 }];

  it("yields each key's value exactly on its phase and holds the end values outside 0 … 1", () => {
    const track = trackOf(keys);
    for (const key of keys) expect(sampleTrack(track, key.at)).toBe(key.value);
    expect(sampleTrack(track, -3)).toBe(2);
    expect(sampleTrack(track, 4)).toBe(2);
  });

  it("is linear between keys without an ease and eased by the earlier key with one", () => {
    const track = trackOf(keys);
    expect(sampleTrack(track, 0.125)).toBe(lerp(2, -6, 0.5));
    expect(sampleTrack(track, 0.375)).toBe(lerp(-6, 10, easeBezier([0.42, 0, 0.58, 1], 0.25)));
    expect(sampleTrack(track, 0.5)).toBeCloseTo(2, 12);
    expect(sampleTrack(track, 0.875)).toBe(lerp(10, 2, 0.5));
  });

  it("answers a track of one key with that key and a track without keys with the rest of its channel", () => {
    expect(sampleTrack(trackOf([{ at: 0, value: 7 }]), 0.4)).toBe(7);
    for (const channel of ["x", "y", "rotation"] as const) expect(sampleTrack(trackOf([], channel), 0.4)).toBe(0);
    for (const channel of ["scaleX", "scaleY"] as const) expect(sampleTrack(trackOf([], channel), 0.4)).toBe(1);
  });

  it("never divides by an empty stretch between two keys on one phase", () => {
    const track = trackOf([{ at: 0, value: 0 }, { at: 0.5, value: 4 }, { at: 0.5, value: 8 }, { at: 1, value: 0 }]);
    for (let step = 0; step <= 64; step++) expect(Number.isFinite(sampleTrack(track, step / 64))).toBe(true);
    expect(sampleTrack(track, 0.5)).toBe(8);
  });
});

describe("clipTicks — seconds to whole ticks", () => {
  it("rounds half up and never falls below one tick", () => {
    const lasting = (seconds: number): Clip => ({ id: "clip", seconds, loop: false, tracks: [] });
    expect(clipTicks(lasting(3))).toBe(192);
    expect(clipTicks(lasting(0.6))).toBe(38);
    expect(clipTicks(lasting(1.4))).toBe(90);
    expect(clipTicks(lasting(0.0390625))).toBe(3);
    expect(clipTicks(lasting(0.0234375))).toBe(2);
    expect(clipTicks(lasting(0.001))).toBe(1);
    expect(clipTicks(lasting(0))).toBe(1);
  });
});

describe("sampleClip — poses in rig order", () => {
  it("leaves every channel without a track at rest", () => {
    const pose = sampleClip(SPECIES, clipOf("wave"), 20);
    expect(pose.length).toBe(SPECIES.bones.length);
    pose.forEach((bone, index) => {
      if (SPECIES.bones[index]!.id === "arm-right") expect(bone).toEqual({ x: 0, y: 0, rotation: sampleTrack(clipOf("wave").tracks[0]!, 20 / 90), scaleX: 1, scaleY: 1 });
      else expect(bone).toEqual({ x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1 });
    });
  });

  it("starts the breathing loop in the rest pose", () => {
    expect(sampleClip(SPECIES, clipOf("breathe"), 0)).toEqual(restPose(SPECIES));
  });

  it("wraps a looping clip and counts ticks before its beginning as its beginning", () => {
    const clip = clipOf("stroll");
    const length = clipTicks(clip);
    for (const tick of [0, 1, 7, 19, 37]) {
      expect(sampleClip(SPECIES, clip, tick + length)).toEqual(sampleClip(SPECIES, clip, tick));
      expect(sampleClip(SPECIES, clip, tick + 5 * length)).toEqual(sampleClip(SPECIES, clip, tick));
    }
    expect(sampleClip(SPECIES, clip, -9)).toEqual(sampleClip(SPECIES, clip, 0));
  });

  it("holds the last key of a clip that plays once", () => {
    const clip = clipOf("lunge");
    const length = clipTicks(clip);
    const held = sampleClip(SPECIES, clip, length);
    expect(held[0]!.x).toBe(3);
    expect(held[SPECIES.bones.findIndex((bone) => bone.id === "tuft")]!.rotation).toBe(12);
    for (const tick of [length + 1, 2 * length, 100000]) expect(sampleClip(SPECIES, clip, tick)).toEqual(held);
    expect(sampleClip(SPECIES, clip, -4)).toEqual(sampleClip(SPECIES, clip, 0));
  });

  it("skips a track on a bone the species lacks and lets the later of two tracks on one channel win", () => {
    const clip: Clip = { id: "odd", seconds: 1, loop: false, tracks: [trackOf([{ at: 0, value: 1 }, { at: 1, value: 1 }]), { bone: "wing", channel: "y", keys: [{ at: 0, value: 9 }, { at: 1, value: 9 }] }, trackOf([{ at: 0, value: 5 }, { at: 1, value: 5 }])] };
    const pose = sampleClip(SPECIES, clip, 10);
    expect(pose.length).toBe(SPECIES.bones.length);
    expect(pose[1]).toEqual({ x: 5, y: 0, rotation: 0, scaleX: 1, scaleY: 1 });
    expect(pose.filter((bone) => bone.y !== 0).length).toBe(0);
  });

  it("feeds `solveRig`: the rest pose of a clip solves like the rig's own rest pose, a posed bone moves its matrix", () => {
    expect(solveRig(SPECIES, sampleClip(SPECIES, clipOf("breathe"), 0))).toEqual(solveRig(SPECIES, restPose(SPECIES)));
    const solved = solveRig(SPECIES, sampleClip(SPECIES, clipOf("stroll"), 9));
    expect(solved.length).toBe(SPECIES.bones.length * 6);
    expect(solved.every((entry) => Number.isFinite(entry))).toBe(true);
    expect(solved).not.toEqual(solveRig(SPECIES, restPose(SPECIES)));
  });
});

describe("blendPose — cross-fade of two poses", () => {
  const from = restPose(SPECIES);
  const to = sampleClip(SPECIES, clipOf("stroll"), 9);

  it("is the first pose itself at and below 0 and the second itself at and above 1", () => {
    expect(blendPose(from, to, 0)).toBe(from);
    expect(blendPose(from, to, -2)).toBe(from);
    expect(blendPose(from, to, 1)).toBe(to);
    expect(blendPose(from, to, 3)).toBe(to);
  });

  it("blends every channel of every bone linearly in between", () => {
    const blended = blendPose(from, to, 0.25);
    expect(blended.length).toBe(from.length);
    blended.forEach((bone, index) => {
      for (const channel of ["x", "y", "rotation", "scaleX", "scaleY"] as const) expect(bone[channel]).toBe(lerp(from[index]![channel], to[index]![channel], 0.25));
    });
    expect(blendPose(to, to, 0.6)).toEqual(to);
  });
});

describe("springStep — one tick of semi-implicit Euler", () => {
  it("moves the velocity first and the position with the new velocity", () => {
    expect(springStep(0, 0, 1, 512, 32)).toEqual({ position: 0.125, velocity: 8 });
    expect(springStep(4, 3, 9, 0, 0)).toEqual({ position: 4.046875, velocity: 3 });
    expect(springStep(2, -1, 0, 100, 0)).toEqual({ position: 1.935546875, velocity: -4.125 });
  });

  it("applies the exact one-tick matrix of the gaze spring", () => {
    expect(GAZE_STIFFNESS).toBe(512);
    expect(GAZE_DAMPING).toBe(32);
    for (const [offset, velocity] of [[1, 0], [0, 1], [-0.375, 2.5], [0.25, -7]] as const) {
      expect(springStep(offset, velocity, 0, GAZE_STIFFNESS, GAZE_DAMPING)).toEqual({ position: 0.875 * offset + 0.0078125 * velocity, velocity: -8 * offset + 0.5 * velocity });
    }
  });

  it("stays exactly on its target when it rests there", () => {
    for (const target of [0, 0.1, -37.25, 1e-9, 123456.789]) expect(after({ position: target, velocity: 0, target, stiffness: GAZE_STIFFNESS, damping: GAZE_DAMPING }, 500)).toEqual({ position: target, velocity: 0 });
  });

  it("settles the gaze within a quarter of a second and barely overshoots", () => {
    const gaze = SETTLING.gaze;
    expect(gaze.stiffness).toBe(GAZE_STIFFNESS);
    expect(gaze.damping).toBe(GAZE_DAMPING);
    for (const jump of gaze.jumps) {
      const travel = jump.target - jump.position;
      let state: Spring = { position: jump.position, velocity: 0 };
      let settled = 1;
      let overshoot = 0;
      for (let tick = 1; tick <= gaze.horizon; tick++) {
        state = springStep(state.position, state.velocity, jump.target, GAZE_STIFFNESS, GAZE_DAMPING);
        const share = (state.position - jump.target) / travel;
        if (Math.abs(share) > gaze.band) settled = tick + 1;
        if (share > overshoot) overshoot = share;
      }
      expect(settled).toBe(jump.expected.settled);
      expect(settled).toBeLessThanOrEqual(gaze.settleTicks);
      expect(overshoot).toBeGreaterThan(0);
      expect(overshoot).toBeLessThanOrEqual(gaze.overshootBound);
      expect(Math.abs(overshoot - jump.expected.overshoot)).toBeLessThanOrEqual(TOLERANCE);
    }
  });

  it("dies out inside the stated stability region and keeps moving outside it", () => {
    const region = (stiffness: number, damping: number): boolean => stiffness > 0 && damping > 0 && stiffness / 4096 + damping / 32 < 4;
    for (const vector of SETTLING.stables) {
      expect(region(vector.stiffness, vector.damping)).toBe(true);
      const faded = after(vector, 4096);
      expect(Math.abs(faded.position - vector.target)).toBeLessThan(1e-6);
      expect(Math.abs(faded.velocity)).toBeLessThan(1e-6);
    }
    for (const vector of SETTLING.unstables) {
      expect(region(vector.stiffness, vector.damping)).toBe(false);
      const kept = after({ position: 1, velocity: 1, target: 0, stiffness: vector.stiffness, damping: vector.damping }, 256);
      expect(Math.abs(kept.position) + Math.abs(kept.velocity)).toBeGreaterThan(0.1);
    }
  });
});

describe("lidAt — the lid of a blink", () => {
  it("is open before, at the beginning and from the end on, and shut at the 4th and 5th tick", () => {
    expect(BLINK_TICKS).toBe(12);
    for (const tick of [-5, 0, BLINK_TICKS, BLINK_TICKS + 1, 1000]) expect(lidAt(tick)).toBe(0);
    expect(lidAt(4)).toBe(1);
    expect(lidAt(5)).toBe(1);
  });

  it("closes faster than it opens, each way by the Hermite ease", () => {
    for (let tick = 1; tick <= 4; tick++) {
      expect(lidAt(tick)).toBeGreaterThan(lidAt(tick - 1));
      expect(lidAt(tick)).toBe(smoothstep(tick / 4));
    }
    for (let tick = 6; tick <= BLINK_TICKS; tick++) {
      expect(lidAt(tick)).toBeLessThan(lidAt(tick - 1));
      if (tick < BLINK_TICKS) expect(lidAt(tick)).toBe(1 - smoothstep((tick - 5) / 7));
    }
    expect(lidAt(2)).toBe(0.5);
    expect(lidAt(7)).toBeGreaterThan(0.5);
  });

  it("shows a fully shut lid to a stage drawn at every other tick, whichever parity it draws", () => {
    for (const parity of [0, 1]) expect(Math.max(...Array.from({ length: 8 }, (_, index) => lidAt(parity + 2 * index)))).toBe(1);
  });
});

describe("shared vectors — the committed answers of the Python oracles", () => {
  it("poses a species the product's own validator accepts, with every committed track as one of its clips", () => {
    expect(speciesIssues(SPECIES)).toEqual([]);
    const clips = SAMPLING.tracks.map((vector): Clip => ({ id: vector.id, seconds: 1, loop: vector.track.keys[0]!.value === vector.track.keys[vector.track.keys.length - 1]!.value, tracks: [vector.track] }));
    expect(speciesIssues({ ...SPECIES, clips: [...SPECIES.clips, ...clips.map((clip) => ({ ...clip, id: `track-${clip.id}` }))] })).toEqual([]);
  });

  it("reproduces every easing, track sample, clip length, pose, blend and lid of 🎞️animation-sampling", () => {
    for (const vector of SAMPLING.easings) vector.amounts.forEach((amount, index) => expect(Math.abs(easeBezier(vector.ease, amount) - vector.expected[index]!)).toBeLessThanOrEqual(TOLERANCE));
    for (const vector of SAMPLING.tracks) vector.phases.forEach((phase, index) => expect(Math.abs(sampleTrack(vector.track, phase) - vector.expected[index]!)).toBeLessThanOrEqual(TOLERANCE));
    for (const vector of SAMPLING.lengths) expect(clipTicks({ id: vector.id, seconds: vector.seconds, loop: false, tracks: [] })).toBe(vector.expected);
    for (const vector of SAMPLING.poses) vector.ticks.forEach((tick, index) => expectPose(sampleClip(SPECIES, clipOf(vector.clip), tick), vector.expected[index]!));
    for (const vector of SAMPLING.blends) vector.amounts.forEach((amount, index) => expectPose(blendPose(vector.from, vector.to, amount), vector.expected[index]!));
    expect(SAMPLING.lids.expected.blinkTicks).toBe(BLINK_TICKS);
    SAMPLING.lids.ticks.forEach((tick, index) => expect(Math.abs(lidAt(tick) - SAMPLING.lids.expected.closures[index]!)).toBeLessThanOrEqual(TOLERANCE));
  });

  it("reproduces every step, run and faded spring of 🪀️spring-settling by single ticks", () => {
    for (const vector of SETTLING.steps) {
      const stepped = after(vector, 1);
      expect(Math.abs(stepped.position - vector.expected.position)).toBeLessThanOrEqual(TOLERANCE);
      expect(Math.abs(stepped.velocity - vector.expected.velocity)).toBeLessThanOrEqual(TOLERANCE);
    }
    for (const vector of SETTLING.runs) {
      vector.ticks.forEach((ticks, index) => {
        const reached = after(vector, ticks);
        expect(Math.abs(reached.position - vector.expected[index]!.position)).toBeLessThanOrEqual(TOLERANCE);
        expect(Math.abs(reached.velocity - vector.expected[index]!.velocity)).toBeLessThanOrEqual(TOLERANCE);
      });
    }
    for (const vector of SETTLING.stables) {
      const faded = after(vector, vector.ticks);
      expect(Math.abs(faded.position - vector.expected.position)).toBeLessThanOrEqual(TOLERANCE);
      expect(Math.abs(faded.velocity - vector.expected.velocity)).toBeLessThanOrEqual(TOLERANCE);
    }
  });
});

describe("determinism — the arithmetic the Rust twin can restate", () => {
  it("uses no transcendental function, no clock and no randomness", () => {
    const source = readFileSync(resolve(HERE, "../../🟦️.ts"), "utf8");
    expect(source.match(/Math\.(?!floor\b|min\b|max\b|abs\b|sqrt\b)\w+/g)).toBeNull();
    expect(source.match(/(?<!\/)\*\*|\bDate\b|\bperformance\b|console\./g)).toBeNull();
  });
});
