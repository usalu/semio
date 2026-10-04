/** 🪂️ Subject adapter of the parachute-descent case: the parachute of the swing module of `@semio-tech/pets` answers every committed fall, opens every committed canopy and lands every committed drop tick by tick.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🪢️swing/🟦️.ts
 * @see ../../🔨️modules/🏞️terrain/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { FALL_SPEED, fallStep, GRAVITY } from "../../🔨️modules/🏞️terrain/🟦️.ts";
import {
  canopyOf,
  CHUTE_DAMPING,
  CHUTE_DESCENT,
  CHUTE_FACTOR,
  CHUTE_FLARE,
  CHUTE_GRAVITY,
  CHUTE_HEADROOM,
  CHUTE_OPENING,
  CHUTE_REFLEX,
  CHUTE_ROD,
  CHUTE_STEER_EASE,
  CHUTE_STEER_GAIN,
  CHUTE_STEER_SPEED,
  CHUTE_WIND,
  CHUTE_WIND_RATE,
  chuteOf,
  chuteOpens,
  chuteStep,
  chuteWind,
  flareOf,
  HARD_LANDING,
  impactSpeed,
} from "../../🔨️modules/🪢️swing/🟦️.ts";
import type { Canopy, Point } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🪂️parachute-descent/🔣️.json";

type Vectors = {
  readonly impacts: readonly { readonly id: string; readonly vy: number; readonly height: number }[];
  readonly triggers: readonly { readonly id: string; readonly vy: number; readonly height: number }[];
  readonly measures: readonly { readonly id: string; readonly height: number }[];
  readonly flares: readonly { readonly id: string; readonly remaining: number; readonly flare: number }[];
  readonly winds: readonly { readonly id: string; readonly ticks: number; readonly phase: number }[];
  readonly descents: readonly { readonly id: string; readonly height: number; readonly feet: Point; readonly vx: number; readonly vy: number; readonly target: number; readonly remaining: number; readonly phase: number | null; readonly ticks: readonly number[] }[];
  readonly sways: readonly { readonly id: string; readonly height: number; readonly canopy: Point; readonly vx: number; readonly target: number; readonly remaining: number; readonly bob: Point; readonly previous: Point; readonly ticks: readonly number[] }[];
  readonly drops: readonly { readonly id: string; readonly height: number; readonly drop: number; readonly vy: number; readonly chute: boolean }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🎚️ The tuning constants of a fall and of the parachute. */
function constants(): Record<string, number> {
  return {
    gravity: GRAVITY,
    fallSpeed: FALL_SPEED,
    hardLanding: HARD_LANDING,
    chuteOpening: CHUTE_OPENING,
    chuteHeadroom: CHUTE_HEADROOM,
    chuteReflex: CHUTE_REFLEX,
    chuteFactor: CHUTE_FACTOR,
    chuteDescent: CHUTE_DESCENT,
    chuteSteerGain: CHUTE_STEER_GAIN,
    chuteSteerSpeed: CHUTE_STEER_SPEED,
    chuteSteerEase: CHUTE_STEER_EASE,
    chuteRod: CHUTE_ROD,
    chuteGravity: CHUTE_GRAVITY,
    chuteDamping: CHUTE_DAMPING,
    chuteFlare: CHUTE_FLARE,
    chuteWind: CHUTE_WIND,
    chuteWindRate: CHUTE_WIND_RATE,
  };
}

/** 🎯️ The entries of a list of states at the committed ticks. */
function at<Entry>(states: readonly Entry[], ticks: readonly number[]): Entry[] {
  return ticks.map((tick) => states[tick - 1]!);
}

/** ⛱️ A canopy opened above a committed pet and advanced to the last committed tick: its place, its velocity and the feet at the committed ticks. */
function descended(vector: Vectors["descents"][number]): { x: number; y: number; vx: number; vy: number; bob: Point }[] {
  const chute = chuteOf(vector.height);
  const states: { x: number; y: number; vx: number; vy: number; bob: Point }[] = [];
  let canopy = canopyOf(vector.feet, vector.vx, vector.vy, chute);
  for (let tick = 1; tick <= Math.max(...vector.ticks); tick++) {
    canopy = chuteStep(canopy, chute, vector.target, vector.remaining - (canopy.bob.y - vector.feet.y), vector.phase === null ? 0 : chuteWind(tick, vector.phase));
    states.push({ x: canopy.x, y: canopy.y, vx: canopy.vx, vy: canopy.vy, bob: canopy.bob });
  }
  return at(states, vector.ticks);
}

/** 🎐️ The feet of a committed swaying pet below a canopy that descends at its terminal speed, at the committed ticks. */
function swayed(vector: Vectors["sways"][number]): Point[] {
  const chute = chuteOf(vector.height);
  const feet: Point[] = [];
  let canopy: Canopy = { x: vector.canopy.x, y: vector.canopy.y, vx: vector.vx, vy: chute.terminal, bob: vector.bob, previous: vector.previous };
  for (let tick = 1; tick <= Math.max(...vector.ticks); tick++) {
    canopy = chuteStep(canopy, chute, vector.target, vector.remaining, 0);
    feet.push(canopy.bob);
  }
  return at(feet, vector.ticks);
}

/** 🪨️ A whole drop as the stage takes it: fall and ask `chuteOpens`; `CHUTE_REFLEX` ticks after the decision open the canopy and descend under it; stop at the tick the feet reach the landing. */
function dropped(vector: Vectors["drops"][number]): { opened: number; landed: number; touch: number; glide: number; plain: number } {
  const chute = chuteOf(vector.height);
  let y = 0;
  let vy = vector.vy;
  let opened = 0;
  let canopy: Canopy | null = null;
  let tick = 0;
  let touch = 0;
  while (y < vector.drop) {
    tick++;
    const remaining = vector.drop - y;
    if (canopy === null) {
      if (opened === 0 && vector.chute && chuteOpens(vy, remaining)) opened = tick;
      if (opened !== 0 && tick - opened >= CHUTE_REFLEX) canopy = canopyOf({ x: 0, y }, 0, vy, chute);
    }
    if (canopy === null) {
      const fall = fallStep(y, vy);
      y = fall.y;
      vy = fall.vy;
      touch = vy;
    } else {
      const moved = chuteStep(canopy, chute, 0, remaining, 0);
      touch = (moved.y - canopy.y) * 64;
      canopy = moved;
      y = moved.bob.y;
      vy = moved.vy;
    }
  }
  return { opened, landed: tick, touch, glide: vy, plain: impactSpeed(vector.vy, vector.drop) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: constants() }) },
    "impact-speeds": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).impacts.map((vector) => [vector.id, impactSpeed(vector.vy, vector.height)])) }) },
    "chute-triggers": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).triggers.map((vector) => [vector.id, chuteOpens(vector.vy, vector.height)])) }) },
    "chute-measures": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).measures.map((vector) => [vector.id, chuteOf(vector.height)])) }) },
    flares: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).flares.map((vector) => [vector.id, flareOf(vector.remaining, vector.flare)])) }) },
    winds: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).winds.map((vector) => [vector.id, chuteWind(vector.ticks, vector.phase)])) }) },
    "canopy-descents": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).descents.map((vector) => [vector.id, descended(vector)])) }) },
    "canopy-sways": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).sways.map((vector) => [vector.id, swayed(vector)])) }) },
    drops: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).drops.map((vector) => [vector.id, dropped(vector)])) }) },
  },
});
