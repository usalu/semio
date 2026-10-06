/** 🪢️ Subject adapter of the swing-dynamics case: the swing module of `@semio-tech/pets` advances every committed swing, grip, drag, release and reel tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🪢️swing/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import {
  coneClamp,
  FOLLOW_DAMPING,
  FOLLOW_STIFFNESS,
  followStep,
  HANG_CONE,
  HANG_DAMPING,
  HANG_GRAVITY,
  HANG_ROD,
  hangOf,
  hangStep,
  leanOf,
  REEL_CAP,
  REEL_DAMPING,
  REEL_LEAST,
  REEL_RAMP,
  REEL_SPEED,
  reelStep,
  RELEASE_DIVISOR,
  RELEASE_STALE,
  RELEASE_WEIGHTS,
  releaseVelocity,
  ringVelocity,
  swingStep,
  THROW_LEAST,
  THROW_MOST,
  THROW_RISE,
  THROW_SHARE,
  throwVelocity,
} from "../../🔨️modules/🪢️swing/🟦️.ts";
import type { Grip, Hang, Point } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🪢️swing-dynamics/🔣️.json";
const SETTLED = 1 / 120;

type Swing = { readonly id: string; readonly anchor: Point; readonly length: number; readonly gravity: number; readonly bob: Point; readonly previous: Point };
type Vectors = {
  readonly rods: readonly (Swing & { readonly damping: number; readonly ticks: readonly number[] })[];
  readonly drifts: readonly (Swing & { readonly damping: number; readonly ticks: number; readonly window: number })[];
  readonly anchors: readonly { readonly id: string; readonly length: number; readonly gravity: number; readonly path: readonly Point[]; readonly ticks: readonly number[] }[];
  readonly ropes: readonly (Swing & { readonly ticks: number })[];
  readonly reels: readonly (Swing & { readonly rate: number; readonly rope: boolean; readonly ticks: readonly number[] })[];
  readonly guards: readonly { readonly id: string; readonly anchor: Point; readonly bob: Point; readonly previous: Point; readonly length: number; readonly least: number; readonly ticks: readonly number[] }[];
  readonly cones: readonly { readonly id: string; readonly anchor: Point; readonly bob: Point; readonly length: number }[];
  readonly follows: readonly { readonly id: string; readonly grip: Grip; readonly target: Point; readonly ticks: readonly number[] }[];
  readonly drags: readonly { readonly id: string; readonly length: number; readonly feet: Point; readonly pointer: readonly Point[]; readonly ticks: readonly number[] }[];
  readonly releases: readonly { readonly id: string; readonly samples: readonly Point[] }[];
  readonly throws: readonly { readonly id: string; readonly hang: Hang; readonly samples: readonly Point[] }[];
  readonly steps: readonly { readonly id: string; readonly before: Point; readonly now: Point; readonly bob: Point; readonly previous: Point; readonly length: number; readonly gravity: number; readonly damping: number; readonly rope: boolean }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

const VIEW = new DataView(new ArrayBuffer(8));

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🎚️ The tuning constants of the hang, the follow spring, the release, the throw and the reel. */
function constants(): Record<string, number | readonly number[]> {
  return {
    hangRod: HANG_ROD,
    hangGravity: HANG_GRAVITY,
    hangDamping: HANG_DAMPING,
    hangCone: HANG_CONE,
    followStiffness: FOLLOW_STIFFNESS,
    followDamping: FOLLOW_DAMPING,
    releaseWeights: RELEASE_WEIGHTS,
    releaseDivisor: RELEASE_DIVISOR,
    releaseStale: RELEASE_STALE,
    throwShare: THROW_SHARE,
    throwLeast: THROW_LEAST,
    throwMost: THROW_MOST,
    throwRise: THROW_RISE,
    reelSpeed: REEL_SPEED,
    reelRamp: REEL_RAMP,
    reelLeast: REEL_LEAST,
    reelCap: REEL_CAP,
    reelDamping: REEL_DAMPING,
  };
}

/** 🏃️ The positions of a point after each of `ticks` ticks of `step`, which is given the point, its place one tick earlier and the tick being stepped (1 first). */
function run(bob: Point, previous: Point, ticks: number, step: (bob: Point, previous: Point, tick: number) => Point): Point[] {
  const path: Point[] = [];
  let now = bob;
  let before = previous;
  for (let tick = 1; tick <= ticks; tick++) {
    const next = step(now, before, tick);
    before = now;
    now = next;
    path.push(now);
  }
  return path;
}

/** 🎯️ The entries of a path at the committed ticks. */
function at<Entry>(path: readonly Entry[], ticks: readonly number[]): Entry[] {
  return ticks.map((tick) => path[tick - 1]!);
}

/** 🏔️ The least height of a free swing below its anchor in every window of ticks, and where it ends. */
function drift(vector: Vectors["drifts"][number]): { lowest: number[]; end: Point } {
  const path = run(vector.bob, vector.previous, vector.ticks, (bob, previous) => swingStep(vector.anchor, vector.anchor, bob, previous, vector.length, vector.gravity, vector.damping, false));
  const lowest: number[] = [];
  for (let start = 0; start < path.length; start += vector.window) {
    let least = path[start]!.y - vector.anchor.y;
    for (let index = start + 1; index < start + vector.window && index < path.length; index++) if (path[index]!.y - vector.anchor.y < least) least = path[index]!.y - vector.anchor.y;
    lowest.push(least);
  }
  return { lowest, end: path[path.length - 1]! };
}

/** 🚃️ The positions of a point that hangs at rest below the first place of a path while its anchor travels the path. */
function anchored(vector: Vectors["anchors"][number]): Point[] {
  const start = { x: vector.path[0]!.x, y: vector.path[0]!.y + vector.length };
  return at(
    run(start, start, vector.path.length - 1, (bob, previous, tick) => swingStep(vector.path[tick - 1]!, vector.path[tick]!, bob, previous, vector.length, vector.gravity, 1, false)),
    vector.ticks,
  );
}

/** 🚦️ The states of a reeled swing at the committed ticks and its fastest step in pixels per second. */
function guarded(vector: Vectors["guards"][number]): { states: { x: number; y: number; length: number }[]; fastest: number } {
  const horizon = Math.max(...vector.ticks);
  const states: { x: number; y: number; length: number }[] = [];
  let bob = vector.bob;
  let previous = vector.previous;
  let length = vector.length;
  let fastest = 0;
  for (let age = 0; age < horizon; age++) {
    const reel = reelStep(vector.anchor, bob, previous, length, vector.least, age);
    const speed = Math.sqrt((reel.bob.x - bob.x) * (reel.bob.x - bob.x) + (reel.bob.y - bob.y) * (reel.bob.y - bob.y)) * 64;
    if (speed > fastest) fastest = speed;
    previous = bob;
    bob = reel.bob;
    length = reel.length;
    states.push({ x: bob.x, y: bob.y, length });
  }
  return { states: at(states, vector.ticks), fastest };
}

/** 🐕️ A grip after each committed number of ticks on its way to its target. */
function followed(vector: Vectors["follows"][number]): Grip[] {
  const states: Grip[] = [];
  let grip = vector.grip;
  let reached = 0;
  for (const ticks of vector.ticks) {
    for (; reached < ticks; reached++) grip = followStep(grip, vector.target);
    states.push(grip);
  }
  return states;
}

/** 🤹️ The leans of a held pet along a pointer path at the committed ticks, their peak, and the tick from which the lean stays below 3°. */
function dragged(vector: Vectors["drags"][number]): { leans: number[]; peak: number; settled: number } {
  const leans: number[] = [];
  let hang = hangOf(vector.feet, vector.length);
  let peak = 0;
  let settled = 1;
  for (let tick = 1; tick <= vector.pointer.length; tick++) {
    hang = hangStep(hang, vector.pointer[tick - 1]!, vector.length);
    const lean = leanOf(hang.grip, hang.bob, vector.length);
    leans.push(lean);
    if (Math.abs(lean) > peak) peak = Math.abs(lean);
    if (Math.abs(lean) >= SETTLED) settled = tick + 1;
  }
  return { leans: at(leans, vector.ticks), peak, settled };
}

/** 🧬️ The bit patterns of both coordinates of a point. */
function pattern(point: Point): { x: string; y: string } {
  return { x: bits(point.x), y: bits(point.y) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: constants() }) },
    "rod-swings": {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          vectors(ctx).rods.map((vector) => [
            vector.id,
            at(
              run(vector.bob, vector.previous, Math.max(...vector.ticks), (bob, previous) => swingStep(vector.anchor, vector.anchor, bob, previous, vector.length, vector.gravity, vector.damping, false)),
              vector.ticks,
            ),
          ]),
        ),
      }),
    },
    "amplitude-drift": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).drifts.map((vector) => [vector.id, drift(vector)])) }) },
    "moving-anchors": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).anchors.map((vector) => [vector.id, anchored(vector)])) }) },
    "slack-ropes": {
      subject: (ctx) => ({
        projection: Object.fromEntries(vectors(ctx).ropes.map((vector) => [vector.id, run(vector.bob, vector.previous, vector.ticks, (bob, previous) => swingStep(vector.anchor, vector.anchor, bob, previous, vector.length, vector.gravity, 1, true))])),
      }),
    },
    "reeled-swings": {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          vectors(ctx).reels.map((vector) => [
            vector.id,
            at(
              run(vector.bob, vector.previous, Math.max(...vector.ticks), (bob, previous, tick) => swingStep(vector.anchor, vector.anchor, bob, previous, vector.length - (vector.rate * tick) / 64, vector.gravity, 1, vector.rope)),
              vector.ticks,
            ),
          ]),
        ),
      }),
    },
    "reel-guards": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).guards.map((vector) => [vector.id, guarded(vector)])) }) },
    "cone-clamps": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).cones.map((vector) => [vector.id, coneClamp(vector.anchor, vector.bob, vector.length)])) }) },
    "follow-springs": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).follows.map((vector) => [vector.id, followed(vector)])) }) },
    "drag-leans": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).drags.map((vector) => [vector.id, dragged(vector)])) }) },
    "release-velocities": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).releases.map((vector) => [vector.id, { ring: ringVelocity(vector.samples), release: releaseVelocity(vector.samples) }])) }) },
    "throw-velocities": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).throws.map((vector) => [vector.id, throwVelocity(vector.hang, vector.samples)])) }) },
    "bit-patterns": {
      subject: (ctx) => ({
        projection: Object.fromEntries(vectors(ctx).steps.map((vector) => [vector.id, pattern(swingStep(vector.before, vector.now, vector.bob, vector.previous, vector.length, vector.gravity, vector.damping, vector.rope))])),
      }),
    },
  },
});
