/** 🪀️ Subject adapter of the spring-settling case: `springStep` and the gaze constants of `@semio-tech/pets` answer every committed vector by single ticks.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🎞️animation/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { GAZE_DAMPING, GAZE_STIFFNESS, type Spring, springStep } from "../../🔨️modules/🎞️animation/🟦️.ts";

const VECTORS = "shared://🪀️spring-settling/🔣️.json";

type Released = { readonly id: string; readonly position: number; readonly velocity: number; readonly target: number; readonly stiffness: number; readonly damping: number };
type Jump = { readonly id: string; readonly position: number; readonly target: number };
type Vectors = {
  readonly steps: readonly Released[];
  readonly runs: readonly (Released & { readonly ticks: readonly number[] })[];
  readonly rests: readonly { readonly id: string; readonly target: number; readonly stiffness: number; readonly damping: number; readonly ticks: number }[];
  readonly stables: readonly (Released & { readonly ticks: number })[];
  readonly gaze: { readonly band: number; readonly horizon: number; readonly jumps: readonly Jump[] };
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** ⏭️ A spring after `ticks` single steps. */
function after(spring: Omit<Released, "id">, ticks: number): Spring {
  let state: Spring = { position: spring.position, velocity: spring.velocity };
  for (let tick = 0; tick < ticks; tick++) state = springStep(state.position, state.velocity, spring.target, spring.stiffness, spring.damping);
  return state;
}

/** 🎯️ From which tick a resting pupil stays within the band around its new target, and its largest overshoot, both as shares of the way. */
function settling(band: number, horizon: number, jump: Jump): { settled: number; overshoot: number } {
  const travel = jump.target - jump.position;
  let state: Spring = { position: jump.position, velocity: 0 };
  let settled = 1;
  let overshoot = 0;
  for (let tick = 1; tick <= horizon; tick++) {
    state = springStep(state.position, state.velocity, jump.target, GAZE_STIFFNESS, GAZE_DAMPING);
    const share = (state.position - jump.target) / travel;
    if (Math.abs(share) > band) settled = tick + 1;
    if (share > overshoot) overshoot = share;
  }
  return { settled, overshoot };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "single-steps": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).steps.map((vector) => [vector.id, after(vector, 1)])) }) },
    "tick-runs": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).runs.map((vector) => [vector.id, vector.ticks.map((ticks) => after(vector, ticks))])) }) },
    resting: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).rests.map((vector) => [vector.id, after({ position: vector.target, velocity: 0, target: vector.target, stiffness: vector.stiffness, damping: vector.damping }, vector.ticks)])) }) },
    "stable-springs": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).stables.map((vector) => [vector.id, after(vector, vector.ticks)])) }) },
    "gaze-settling": {
      subject: (ctx) => {
        const gaze = vectors(ctx).gaze;
        return { projection: { stiffness: GAZE_STIFFNESS, damping: GAZE_DAMPING, jumps: Object.fromEntries(gaze.jumps.map((jump) => [jump.id, settling(gaze.band, gaze.horizon, jump)])) } };
      },
    },
  },
});
