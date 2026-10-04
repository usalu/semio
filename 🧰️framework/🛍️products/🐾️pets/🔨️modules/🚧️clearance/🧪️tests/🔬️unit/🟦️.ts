/** 🚧️ Unit suite of the clearance module: every function against cases written from the design, against brute force on a lattice, and a reference world in which the rules of the module, and nothing else, keep six to ten pets apart while they walk, hop, glide, fall, land on heads, are dragged and thrown by a hand and lose, shrink and move their perches.
 *
 * The world is exported: `clearance_world.ts` of ticket 2026/10/02/QUIZ-PETS runs it for the published metrics and ablations.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../🏞️terrain/🟦️.ts — the perches, falls and hops the world moves by
 * @see ../../../🎲️randomness/🟦️.ts — the draws of the world
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Perch, Point, Size } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { randomWords, unitOf } from "../../../🎲️randomness/🟦️.ts";
import { fallStep, hopLanding, hopOf, hopStep, landingOf, strideTo } from "../../../🏞️terrain/🟦️.ts";
import {
  type Body,
  type Claim,
  DELAY,
  type Extent,
  FOREVER,
  LANE_LIFT,
  LEAN,
  MARGIN,
  PATIENCE,
  PUSHES,
  type Posture,
  SCOOT_HASTE,
  SEAM,
  SLIDE_OFF_GAIN,
  SLIDE_OFF_LIMIT,
  SLIDE_OFF_SPEED,
  STEERINGS,
  TALLY,
  type Tally,
  UPRIGHT,
  bodyOf,
  canopied,
  claimClear,
  claimOf,
  columnOver,
  evicted,
  freeAmong,
  freeAt,
  grown,
  guardedStride,
  headUnder,
  laneLift,
  leaning,
  liftOnto,
  meets,
  mustPoof,
  nearMisses,
  obstaclesOf,
  orderKept,
  orderOf,
  overlaps,
  perMillion,
  pruned,
  pushedOut,
  released,
  restsOn,
  scootFraction,
  scooted,
  seatOf,
  shifted,
  sliceAt,
  slideOf,
  slideSide,
  slideStride,
  slotIn,
  spotOn,
  tallied,
  united,
  vaults,
} from "../../🟦️.ts";

//#region 🔖️Reference world
/** 📜️ The rules of the world, each one a switch. The core rules, without any of which bodies overlap: `guard` (a walker stops before a body), `corridors` (and before a claim), `vetting` (a plan is taken only when its claim is clear), `rests` (a claim names where its owner stays, so nobody arrives there later), `projection` (a held pet is pushed out of what it is dragged into), `eviction` (whoever a moving surface carried into somebody poofs) and `poof` (whoever is in the air without a legal continuation poofs). The quality rules, without which pets poof or fall more often: `heads` (a head is a platform), `steering` (air control, bounce and letting go) and `seating` (a perch group is re-seated after a survey). */
export type Rules = {
  readonly guard: boolean;
  readonly corridors: boolean;
  readonly vetting: boolean;
  readonly rests: boolean;
  readonly projection: boolean;
  readonly poof: boolean;
  readonly eviction: boolean;
  readonly heads: boolean;
  readonly steering: boolean;
  readonly seating: boolean;
};

/** 🟢️ Every rule in force. */
export const RULES: Rules = { guard: true, corridors: true, vetting: true, rests: true, projection: true, poof: true, eviction: true, heads: true, steering: true, seating: true };

/** 🦶️ One tick of a planned path: the feet, the velocity in pixels per second and the posture at the end of that tick. */
type Step = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number; readonly posture: Posture };

/** 🗺️ A plan: its steps (the first one is the place it starts from), its claim, and what it ends on: the surface of a perch or the owner of a head. */
type Plan = { readonly steps: readonly Step[]; readonly claim: Claim; readonly ending: "perch" | "head"; readonly landing: string; readonly steer: number };

/** 🐶️ A pet of the world. */
type Pet = {
  readonly slug: string;
  readonly size: Size;
  readonly hover: number;
  readonly speed: number;
  readonly chute: boolean;
  x: number;
  y: number;
  vx: number;
  vy: number;
  posture: Posture;
  mode: "stand" | "walk" | "plan" | "drop" | "held" | "ride" | "scoot" | "gone";
  perch: string | null;
  goal: number;
  until: number;
  waited: number;
  steps: readonly Step[];
  from: number;
  ending: "perch" | "head";
  landing: string;
  side: number;
  slid: number;
};

/** ✋️ The simulated hand: whom it holds, where it is and was a tick ago, where it is heading, how fast and for how long. */
type Hand = { readonly slug: string; x: number; y: number; px: number; py: number; tx: number; ty: number; readonly pace: number; left: number };

/** 🔢️ What happened in a world, counted. */
export type Counts = { surveys: number; grabs: number; hops: number; glides: number; lanes: number; falls: number; heads: number; steered: number; chutes: number; refusals: number; delays: number; stalls: number; timeouts: number; evictions: number; crowded: number; seated: number; spilled: number; disorders: number; brushes: number };

/** 🌍️ The reference world. */
export type World = { readonly seed: number; readonly rules: Rules; readonly span: number; tick: number; draws: number; perches: Perch[]; spare: Perch[]; readonly pets: Pet[]; claims: Claim[]; hand: Hand | null; tally: Tally; poofs: number; waits: number; orders: Map<string, string[]>; readonly counts: Counts };

const WIDTH = 960;
const FLOOR = 520;
const NEAR = 2;
const STREAM = 0x0c1ea4a4;
const FALL_LIMIT = 900;
const CHUTE_SPEED = 480;
const CHUTE_DECAY = 0.91685535573;

/** 🎲️ The next draw of a world in [0, 1): counter-based, so a world is a pure function of its seed and rules. */
function draw(world: World): number {
  const unit = unitOf(randomWords([world.seed, STREAM, world.draws], 1)[0]!);
  world.draws++;
  return unit;
}

/** 🗜️ A value held between two bounds. */
function held(value: number, low: number, high: number): number {
  return Math.min(Math.max(value, low), high);
}

/** 🧊️ The box of a pet at a place in a posture. */
function extentAt(pet: Pet, x: number, y: number, posture: Posture): Extent {
  return bodyOf(pet.slug, { x, y }, pet.size, pet.hover, posture, MARGIN).extent;
}

/** 🧸️ The box of a pet as it is. */
function extentOf(pet: Pet): Extent {
  return extentAt(pet, pet.x, pet.y, pet.posture);
}

/** 👪️ The bodies of a world: every pet that has not vanished, in the order of the pets. */
export function bodiesOf(world: World): Body[] {
  const bodies: Body[] = [];
  for (const pet of world.pets) if (pet.mode !== "gone") bodies.push({ owner: pet.slug, extent: extentOf(pet) });
  return bodies;
}

/** 🪵️ The perch of a surface, or `null`. */
function perchOf(world: World, surface: string | null): Perch | null {
  for (const perch of world.perches) if (perch.surface === surface) return perch;
  return null;
}

/** 🩳️ The perches as far as the feet of a body may go on them whose footprint reaches `half` to either side. */
function narrowed(perches: readonly Perch[], half: number): Perch[] {
  const fitting: Perch[] = [];
  for (const perch of perches) if (perch.x0 + half <= perch.x1 - half) fitting.push({ surface: perch.surface, x0: perch.x0 + half, x1: perch.x1 - half, y: perch.y });
  return fitting;
}

/** 🦵️ Whether a pet stands on a perch (and not in the air, in the hand or on a head). */
function grounded(pet: Pet): boolean {
  return pet.mode === "stand" || pet.mode === "walk" || pet.mode === "scoot";
}

/** 😴️ A pet comes to rest where it is and dwells for a while. */
function settle(world: World, pet: Pet, now: number): void {
  pet.mode = "stand";
  pet.goal = pet.x;
  pet.waited = 0;
  pet.until = now + 20 + Math.floor(draw(world) * 100);
}

/** 🕳️ A pet loses its footing: it needs a plan for the way down, starting with the given velocity. */
function unfoot(pet: Pet, vx: number, vy: number): void {
  pet.mode = "drop";
  pet.perch = null;
  pet.posture = UPRIGHT;
  pet.vx = vx;
  pet.vy = vy;
  pet.waited = 0;
}

/** 💨️ A pet vanishes at once; it arrives anew a second later. */
function poof(world: World, pet: Pet, now: number): void {
  world.claims = released(world.claims, pet.slug);
  pet.mode = "gone";
  pet.perch = null;
  pet.posture = UPRIGHT;
  pet.until = now + 64;
  world.poofs++;
}

/** 🚪️ A vanished pet arrives at a free spot of a perch it draws; `false` when that perch has none. */
function arrive(world: World, pet: Pet, now: number): boolean {
  const perch = world.perches[Math.floor(draw(world) * world.perches.length)]!;
  const half = pet.size.width / 2;
  const x = perch.x0 + half + draw(world) * Math.max(perch.x1 - perch.x0 - pet.size.width, 0);
  const feet = perch.y - pet.hover;
  const spot = spotOn(perch, x, extentAt(pet, x, feet, UPRIGHT), half, obstaclesOf(pet.slug, bodiesOf(world), world.claims, now));
  if (spot === null) return false;
  pet.x = spot;
  pet.y = feet;
  pet.perch = perch.surface;
  pet.posture = UPRIGHT;
  settle(world, pet, now);
  return true;
}

/** 🌱️ A world for a seed: a floor, six shelves, six to ten pets of widths 28…58 and heights 40…56, a fifth of them floaters, half of them with a parachute. `span` is how many ticks a slice of its claims holds (1: the path itself). */
export function openWorld(seed: number, rules: Rules, span: number): World {
  const world: World = {
    seed,
    span,
    rules,
    tick: 0,
    draws: 0,
    perches: [{ surface: "floor", x0: 0, x1: WIDTH, y: FLOOR }],
    spare: [],
    pets: [],
    claims: [],
    hand: null,
    tally: TALLY,
    poofs: 0,
    waits: 0,
    orders: new Map(),
    counts: { surveys: 0, grabs: 0, hops: 0, glides: 0, lanes: 0, falls: 0, heads: 0, steered: 0, chutes: 0, refusals: 0, delays: 0, stalls: 0, timeouts: 0, evictions: 0, crowded: 0, seated: 0, spilled: 0, disorders: 0, brushes: 0 },
  };
  for (let shelf = 0; shelf < 6; shelf++) {
    const x0 = Math.floor(draw(world) * 660);
    const width = 140 + Math.floor(draw(world) * 240);
    world.perches.push({ surface: `shelf-${shelf}`, x0, x1: Math.min(x0 + width, WIDTH), y: FLOOR - 64 * (1 + Math.floor(draw(world) * 5)) });
  }
  const count = 6 + Math.floor(draw(world) * 5);
  for (let index = 0; index < count; index++) {
    const size = { width: 28 + Math.floor(draw(world) * 31), height: 40 + Math.floor(draw(world) * 17) };
    const speed = 24 + Math.floor(draw(world) * 49);
    const hover = draw(world) < 0.2 ? 6 + Math.floor(draw(world) * 13) : 0;
    const pet: Pet = { slug: `pet-${index}`, size, hover, speed, chute: draw(world) < 0.5, x: 0, y: 0, vx: 0, vy: 0, posture: UPRIGHT, mode: "gone", perch: null, goal: 0, until: 0, waited: 0, steps: [], from: 0, ending: "perch", landing: "", side: 1, slid: 0 };
    world.pets.push(pet);
    if (!arrive(world, pet, 0)) pet.until = 64;
  }
  return world;
}

/** 🧾️ The claim of a path, tick by tick; without the rule of rests it names no place where its owner stays. */
function claimed(world: World, pet: Pet, steps: readonly Step[], now: number): Claim {
  const extents: Extent[] = [];
  for (const step of steps) extents.push(extentAt(pet, step.x, step.y, step.posture));
  return claimOf(pet.slug, now, extents, world.span, world.rules.rests ? extents[extents.length - 1]! : null);
}

/** 🍂️ The way down from where a pet is with a velocity (`steer` px/s of it being air control): tick by tick `fallStep` and a sideways drift, held inside the stage; a parachute opens once it falls fast; it ends on the first perch its footprint comes down on or on the first head, whichever is higher. `null` when it ends on nothing within the limit. */
function fallPlan(world: World, pet: Pet, speed: Point, steer: number, statics: readonly Body[], perches: readonly Perch[], now: number): Plan | null {
  const half = pet.size.width / 2;
  const canopy = canopied(pet.size, { width: pet.size.width * 1.6, height: pet.size.height * 0.6 });
  const steps: Step[] = [{ x: pet.x, y: pet.y, vx: speed.x, vy: speed.y, posture: UPRIGHT }];
  let x = pet.x;
  let y = pet.y;
  let vx = speed.x;
  let vy = speed.y;
  let posture = UPRIGHT;
  let open = false;
  for (let tick = 1; tick <= FALL_LIMIT; tick++) {
    const before = extentAt(pet, x, y, posture);
    if (pet.chute && !open && vy >= CHUTE_SPEED) {
      open = true;
      posture = canopy;
    }
    const fallen = fallStep(y, vy);
    const sink = open ? pet.size.height * 2 + (vy - pet.size.height * 2) * CHUTE_DECAY : fallen.vy;
    const nextY = open ? y + sink / 64 : fallen.y;
    let nextX = x + vx / 64;
    if (nextX < half || nextX > WIDTH - half) {
      nextX = held(nextX, half, WIDTH - half);
      vx = 0;
    }
    const perch = landingOf(perches, nextX, y + pet.hover, nextY + pet.hover);
    const after = extentAt(pet, nextX, nextY, posture);
    const host = world.rules.heads ? headUnder(before, after, pet.slug, statics) : null;
    const onPerch = perch === null ? Infinity : perch.y - pet.hover;
    const onHead = host === null ? Infinity : nextY + liftOnto(after, host.extent);
    if (perch !== null || host !== null) {
      const head = host !== null && onHead <= onPerch;
      steps.push({ x: nextX, y: head ? onHead : onPerch, vx: 0, vy: 0, posture: UPRIGHT });
      return { steps, claim: claimed(world, pet, steps, now), ending: head ? "head" : "perch", landing: head ? host.owner : perch!.surface, steer };
    }
    steps.push({ x: nextX, y: nextY, vx, vy: sink, posture });
    x = nextX;
    y = nextY;
    vy = sink;
  }
  return null;
}

/** 🛫️ A pet takes a plan: from now on it follows its steps, and its claim is on the table. */
function install(world: World, pet: Pet, plan: Plan, now: number): void {
  pet.mode = "plan";
  pet.perch = null;
  pet.posture = UPRIGHT;
  pet.steps = plan.steps;
  pet.from = now;
  pet.ending = plan.ending;
  pet.landing = plan.landing;
  world.claims = released(world.claims, pet.slug);
  world.claims.push(plan.claim);
}

/** 🪂️ A pet without footing looks for its way down, in the order tried: as it flies, towards the free column over the perch it would land on, with the air control of `STEERINGS`, and, when it was thrown, bounced back at half its speed or simply let go (again with the air control); the first plan that is clear is taken. While none is, it stays where it is for at most `DELAY` ticks, as long as that place is free; when nothing legal is left it poofs. */
function plunge(world: World, pet: Pet, now: number): void {
  const bodies = bodiesOf(world);
  const statics: Body[] = [];
  for (const body of bodies) {
    let planned = false;
    for (const claim of world.claims) if (claim.owner === body.owner) planned = true;
    if (!planned && body.owner !== pet.slug) statics.push(body);
  }
  const half = pet.size.width / 2;
  const perches = narrowed(world.perches, half);
  const plans: Plan[] = [];
  let chosen: Plan | null = null;
  const straight = fallPlan(world, pet, { x: pet.vx, y: pet.vy }, 0, statics, perches, now);
  if (straight !== null) {
    plans.push(straight);
    if (!world.rules.vetting || claimClear(straight.claim, bodies, world.claims)) chosen = straight;
  }
  if (chosen === null && world.rules.steering) {
    const tries: { readonly speed: Point; readonly steer: number }[] = [];
    if (straight !== null && straight.ending === "perch") {
      const perch = perchOf(world, straight.landing)!;
      const column = columnOver(perch, pet.x, extentAt(pet, pet.x, pet.y, UPRIGHT), perch.y - pet.hover - pet.y, half, obstaclesOf(pet.slug, bodies, world.claims, now));
      const steer = column === null ? 0 : held(((column - pet.x) * 64) / (straight.steps.length - 1), -240, 240);
      if (steer !== 0) tries.push({ speed: { x: pet.vx + steer, y: pet.vy }, steer });
    }
    for (let index = 1; index < STEERINGS.length; index++) tries.push({ speed: { x: pet.vx + STEERINGS[index]!, y: pet.vy }, steer: STEERINGS[index]! });
    if (pet.vx !== 0 || pet.vy !== 0) {
      tries.push({ speed: { x: 0 - pet.vx / 2, y: pet.vy }, steer: 0 - pet.vx / 2 - pet.vx });
      for (const steer of STEERINGS) tries.push({ speed: { x: steer, y: 0 }, steer: steer - pet.vx });
    }
    for (const attempt of tries) {
      const steered = fallPlan(world, pet, attempt.speed, attempt.steer, statics, perches, now);
      if (steered === null) continue;
      plans.push(steered);
      if (claimClear(steered.claim, bodies, world.claims)) {
        chosen = steered;
        break;
      }
    }
  }
  if (chosen === null) {
    const claims: Claim[] = [];
    for (const plan of plans) claims.push(plan.claim);
    if (!mustPoof(pet.slug, pet.waited < DELAY ? extentOf(pet) : null, claims, bodies, world.claims, now)) {
      pet.waited++;
      world.waits++;
      world.counts.delays++;
      return;
    }
    if (world.rules.poof || plans.length === 0) {
      world.counts.timeouts++;
      poof(world, pet, now);
      return;
    }
    chosen = plans[0]!;
  }
  world.counts.falls++;
  if (chosen.steer !== 0) world.counts.steered++;
  if (chosen.ending === "head") world.counts.heads++;
  for (const step of chosen.steps) {
    if (step.posture !== UPRIGHT) {
      world.counts.chutes++;
      break;
    }
  }
  install(world, pet, chosen, now);
}

/** 🕊️ A planned pet moves on by one step; on the last one it lands: on its perch, or on a head it slides off from. */
function fly(world: World, pet: Pet, now: number): void {
  const index = now - pet.from;
  const step = pet.steps[index]!;
  pet.x = step.x;
  pet.y = step.y;
  pet.vx = step.vx;
  pet.vy = step.vy;
  pet.posture = step.posture;
  if (index < pet.steps.length - 1) return;
  world.claims = released(world.claims, pet.slug);
  pet.posture = UPRIGHT;
  if (pet.ending === "perch") {
    pet.perch = pet.landing;
    settle(world, pet, now);
    return;
  }
  pet.mode = "ride";
  pet.slid = 0;
  const host = world.pets.find((other) => other.slug === pet.landing && other.mode !== "gone");
  pet.side = host === undefined ? 1 : slideSide(extentOf(pet), extentOf(host));
}

/** 🛷️ A pet on a head: it falls when its host no longer carries it, and slides off otherwise. */
function slide(world: World, pet: Pet, now: number): void {
  const host = world.pets.find((other) => other.slug === pet.landing && other.mode !== "gone");
  const extent = extentOf(pet);
  if (host === undefined || !restsOn(extent, extentOf(host))) {
    unfoot(pet, 0, 0);
    return;
  }
  const slid = slideOf(extent, pet.side, pet.slid, 0 - MARGIN, WIDTH + MARGIN, obstaclesOf(pet.slug, bodiesOf(world), world.claims, now));
  pet.x = pet.x + slid.stride;
  if (slid.side === pet.side) pet.slid++;
  else {
    pet.side = slid.side;
    pet.slid = 0;
  }
}

/** 🦘️ The hop of a walker to a spot: the ballistic arc of the terrain, when it really ends on that perch. */
function hopPlans(world: World, pet: Pet, target: Perch, spot: number, now: number): Plan[] {
  const from = { x: pet.x, y: pet.y };
  const to = { x: spot, y: target.y };
  const hop = hopOf(from, to);
  if (hop === null) return [];
  const landing = hopLanding(narrowed(world.perches, pet.size.width / 2), from, to, hop);
  if (landing === null || landing.surface !== target.surface) return [];
  const steps: Step[] = [{ x: pet.x, y: pet.y, vx: hop.vx, vy: hop.vy, posture: UPRIGHT }];
  let flight = { x: pet.x, y: pet.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left >= 1; left--) {
    flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    steps.push({ x: flight.x, y: flight.y, vx: flight.vx, vy: flight.vy, posture: UPRIGHT });
  }
  return [{ steps, claim: claimed(world, pet, steps, now), ending: "perch", landing: target.surface, steer: 0 }];
}

/** 🎈️ The glides of a floater to a spot: a straight line at twice its speed, at its own altitude and in its high lane. */
function glidePlans(world: World, pet: Pet, target: Perch, spot: number, now: number): Plan[] {
  const dx = spot - pet.x;
  const dy = target.y - pet.hover - pet.y;
  if (Math.abs(dx) > 300 || Math.abs(dy) > 200) return [];
  const ticks = Math.max(Math.ceil(Math.max(Math.abs(dx), Math.abs(dy)) / ((2 * pet.speed) / 64)), 1);
  if (ticks > 256) return [];
  const plans: Plan[] = [];
  for (const lane of [0, laneLift(pet.size.height, MARGIN)]) {
    const steps: Step[] = [];
    for (let tick = 0; tick <= ticks; tick++) {
      const ramp = Math.min(1, tick / 8, (ticks - tick) / 8);
      steps.push(tick === ticks ? { x: spot, y: target.y - pet.hover, vx: 0, vy: 0, posture: UPRIGHT } : { x: pet.x + (dx * tick) / ticks, y: pet.y + (dy * tick) / ticks - lane * ramp, vx: 0, vy: 0, posture: UPRIGHT });
    }
    plans.push({ steps, claim: claimed(world, pet, steps, now), ending: "perch", landing: target.surface, steer: 0 });
  }
  return plans;
}

/** 🧳️ A pet tries to travel to another perch: it draws a place near itself on every other perch, takes the free spots nearest to those places, and tries the plans to them (a hop or, floating, its glides) beginning with a perch it draws; only a plan that is clear is taken. `false` when it stays. */
function travel(world: World, pet: Pet, now: number): boolean {
  const half = pet.size.width / 2;
  const bodies = bodiesOf(world);
  const obstacles = obstaclesOf(pet.slug, bodies, world.claims, now);
  const first = Math.floor(draw(world) * world.perches.length);
  const reach = (draw(world) - 0.5) * 240;
  for (let turn = 0; turn < world.perches.length; turn++) {
    const target = world.perches[(first + turn) % world.perches.length]!;
    if (target.surface === pet.perch || target.x1 - target.x0 < pet.size.width) continue;
    const aim = held(pet.x + reach, target.x0 + half, target.x1 - half);
    const spot = spotOn(target, aim, extentAt(pet, aim, target.y - pet.hover, UPRIGHT), half, obstacles);
    if (spot === null) continue;
    const plans = pet.hover > 0 ? glidePlans(world, pet, target, spot, now) : hopPlans(world, pet, target, spot, now);
    for (const plan of plans) {
      if (world.rules.vetting && !claimClear(plan.claim, bodies, world.claims)) continue;
      if (pet.hover > 0) world.counts.glides++;
      else world.counts.hops++;
      if (plan !== plans[0]) world.counts.lanes++;
      install(world, pet, plan, now);
      return true;
    }
  }
  return false;
}

/** 🤔️ A pet that has dwelt long enough travels or sets out for a goal on its perch. */
function decide(world: World, pet: Pet, now: number): void {
  const perch = perchOf(world, pet.perch);
  if (perch === null) {
    unfoot(pet, 0, 0);
    return;
  }
  if (draw(world) < 0.35) {
    if (travel(world, pet, now)) return;
    world.counts.refusals++;
    pet.until = now + 16 + Math.floor(draw(world) * 32);
    return;
  }
  const half = pet.size.width / 2;
  const goal = held(perch.x0 + half + draw(world) * (perch.x1 - perch.x0 - pet.size.width), Math.min(perch.x0 + half, pet.x), Math.max(perch.x1 - half, pet.x));
  pet.goal = world.rules.guard ? pet.x + guardedStride(extentOf(pet), goal - pet.x, obstaclesOf(pet.slug, bodiesOf(world), world.rules.corridors ? world.claims : [], now)) : goal;
  pet.mode = "walk";
  pet.waited = 0;
}

/** 👣️ A walker takes its guarded stride; hindered, it waits, and gives its goal up when its patience is over. */
function stride(world: World, pet: Pet, now: number): void {
  const next = strideTo(pet.x, pet.goal, pet.speed);
  const wanted = next - pet.x;
  let allowed = wanted;
  if (world.rules.guard) allowed = guardedStride(extentOf(pet), wanted, obstaclesOf(pet.slug, bodiesOf(world), world.rules.corridors ? world.claims : [], now));
  if (allowed === wanted) {
    pet.x = next;
    if (pet.x === pet.goal) settle(world, pet, now);
    return;
  }
  pet.x = pet.x + allowed;
  pet.waited++;
  world.waits++;
  world.counts.stalls++;
  if (pet.waited > PATIENCE) settle(world, pet, now);
}

/** 🤏️ A held pet follows the hand, leaning by how fast it is dragged; it is pushed out of whatever it is dragged into, and keeps its place when that fails or would leave the stage. */
function follow(world: World, pet: Pet, now: number): void {
  const hand = world.hand!;
  const posture = leaning(pet.size.height, Math.min(Math.abs(hand.x - hand.px) / 8, 1));
  let x = hand.x;
  let y = hand.y;
  if (world.rules.projection) {
    const push = pushedOut(extentAt(pet, x, y, posture), obstaclesOf(pet.slug, bodiesOf(world), world.claims, now), PUSHES);
    if (push === null) return;
    x = x + push.x;
    y = y + push.y;
    if (x < pet.size.width / 2 || x > WIDTH - pet.size.width / 2 || y > FLOOR - pet.hover) return;
  }
  pet.x = x;
  pet.y = y;
  pet.posture = posture;
}

/** 🧲️ The hand heads for another pet (to drag its pet into it) or for any point of the stage. */
function retarget(world: World, hand: Hand): void {
  const others = world.pets.filter((pet) => pet.mode !== "gone" && pet.slug !== hand.slug);
  if (others.length > 0 && draw(world) < 0.5) {
    const other = others[Math.floor(draw(world) * others.length)]!;
    hand.tx = other.x;
    hand.ty = other.y;
    return;
  }
  hand.tx = 40 + draw(world) * 880;
  hand.ty = 60 + draw(world) * 440;
}

/** ✊️ The hand of the world: now and then it picks any pet up, wherever it is, drags it about for a while and lets it go, thrown with the speed of the hand or just dropped. */
function handle(world: World, now: number): void {
  if (world.hand === null) {
    if (!(draw(world) < 1 / 120)) return;
    const visible = world.pets.filter((pet) => pet.mode !== "gone");
    if (visible.length === 0) return;
    const pet = visible[Math.floor(draw(world) * visible.length)]!;
    world.claims = released(world.claims, pet.slug);
    pet.mode = "held";
    pet.perch = null;
    world.counts.grabs++;
    world.hand = { slug: pet.slug, x: pet.x, y: pet.y, px: pet.x, py: pet.y, tx: pet.x, ty: pet.y, pace: 4 + draw(world) * 10, left: 20 + Math.floor(draw(world) * 100) };
    retarget(world, world.hand);
    return;
  }
  const hand = world.hand;
  const pet = world.pets.find((other) => other.slug === hand.slug)!;
  hand.left--;
  if (hand.left <= 0) {
    const thrown = draw(world) < 0.5;
    unfoot(pet, thrown ? held((hand.x - hand.px) * 64, -640, 640) : 0, thrown ? held((hand.y - hand.py) * 64, -520, 640) : 0);
    world.hand = null;
    return;
  }
  hand.px = hand.x;
  hand.py = hand.y;
  const dx = hand.tx - hand.x;
  const dy = hand.ty - hand.y;
  const far = Math.max(Math.abs(dx), Math.abs(dy));
  if (far <= hand.pace) {
    hand.x = hand.tx;
    hand.y = hand.ty;
    retarget(world, hand);
  } else {
    hand.x = hand.x + (dx * hand.pace) / far;
    hand.y = hand.y + (dy * hand.pace) / far;
  }
  hand.y = Math.min(hand.y, FLOOR - pet.hover);
  hand.x = held(hand.x, pet.size.width / 2, WIDTH - pet.size.width / 2);
}

/** 🏗️ A survey now and then: a shelf vanishes, shrinks, jumps by up to 60 px or appears. Riders ride a shelf that moved, and whoever it carried into somebody poofs; riders of a shelf that vanished fall; whoever is in the air plans anew; the riders of the shelf are seated again and scoot to their seats. */
function survey(world: World, now: number): void {
  if (!(draw(world) < 1 / 300)) return;
  world.counts.surveys++;
  const kind = draw(world);
  const planned = world.pets.filter((pet) => pet.mode === "plan");
  for (const pet of planned) {
    world.claims = released(world.claims, pet.slug);
    unfoot(pet, pet.vx, pet.vy);
  }
  if (world.perches.length === 1 || (kind < 0.25 && world.spare.length > 0)) {
    if (world.spare.length > 0) world.perches.push(world.spare.pop()!);
    return;
  }
  const index = 1 + Math.floor(draw(world) * (world.perches.length - 1));
  const old = world.perches[index]!;
  const riders = world.pets.filter((pet) => grounded(pet) && pet.perch === old.surface);
  if (kind < 0.5) {
    world.perches.splice(index, 1);
    world.spare.push(old);
    for (const pet of riders) unfoot(pet, 0, 0);
    return;
  }
  let fresh: Perch;
  if (kind < 0.75) {
    const width = (0.4 + 0.4 * draw(world)) * (old.x1 - old.x0);
    const x0 = old.x0 + draw(world) * (old.x1 - old.x0 - width);
    fresh = { surface: old.surface, x0, x1: x0 + width, y: old.y };
  } else {
    const x0 = held(old.x0 + (draw(world) - 0.5) * 120, 0, WIDTH - (old.x1 - old.x0));
    const y = held(old.y + (draw(world) - 0.5) * 120, 100, 480);
    fresh = { surface: old.surface, x0, x1: x0 + (old.x1 - old.x0), y };
    const moved: Pet[] = [...riders];
    for (let at = 0; at < moved.length; at++) for (const pet of world.pets) if (pet.mode === "ride" && pet.landing === moved[at]!.slug && !moved.includes(pet)) moved.push(pet);
    for (const pet of moved) {
      pet.x = pet.x + (fresh.x0 - old.x0);
      pet.y = pet.y + (fresh.y - old.y);
      pet.goal = pet.goal + (fresh.x0 - old.x0);
    }
    if (world.rules.eviction) {
      const bodies = bodiesOf(world);
      const ranked = [...bodies.filter((body) => !moved.some((pet) => pet.slug === body.owner)), ...bodies.filter((body) => moved.some((pet) => pet.slug === body.owner))];
      for (const owner of evicted(ranked, world.claims, now)) {
        world.counts.evictions++;
        poof(world, world.pets.find((pet) => pet.slug === owner)!, now);
      }
    }
  }
  world.perches[index] = fresh;
  const members = world.pets.filter((pet) => grounded(pet) && pet.perch === fresh.surface);
  if (!world.rules.seating) {
    for (const pet of members) {
      if (pet.x >= fresh.x0 + pet.size.width / 2 && pet.x <= fresh.x1 - pet.size.width / 2) continue;
      world.counts.spilled++;
      unfoot(pet, 0, 0);
    }
    return;
  }
  const seating = seatOf(members.map((pet) => ({ owner: pet.slug, extent: extentOf(pet) })), fresh, MARGIN);
  for (const owner of seating.leavers) {
    const pet = members.find((member) => member.slug === owner)!;
    if (pet.x < fresh.x0 + pet.size.width / 2 || pet.x > fresh.x1 - pet.size.width / 2) {
      world.counts.spilled++;
      unfoot(pet, 0, 0);
    } else {
      world.counts.crowded++;
      poof(world, pet, now);
    }
  }
  if (!seating.seats.some((seat) => seat.shift !== 0)) return;
  for (const seat of seating.seats) {
    const pet = members.find((member) => member.slug === seat.owner)!;
    if (seat.shift !== 0) world.counts.seated++;
    pet.mode = "scoot";
    pet.goal = pet.x + seat.shift;
    pet.waited = 0;
  }
}

/** 🚌️ Everybody who scoots to a seat moves by the one common share of the way, perch by perch, and only when every place on the way is free; a group that is held up too long gives up: whoever is on the perch stays where it is, whoever is not falls. */
function scoot(world: World, now: number): void {
  for (const perch of [...world.perches]) {
    const members = world.pets.filter((pet) => pet.mode === "scoot" && pet.perch === perch.surface);
    if (members.length === 0) continue;
    const fraction = scootFraction(members.map((pet) => pet.goal - pet.x), (SCOOT_HASTE * 48) / 64);
    const obstacles = obstaclesOf("", bodiesOf(world).filter((body) => !members.some((pet) => pet.slug === body.owner)), world.claims, now);
    const places = members.map((pet) => scooted(pet.x, pet.goal, fraction));
    if (members.every((pet, at) => freeAmong(extentAt(pet, places[at]!, pet.y, UPRIGHT), obstacles))) {
      members.forEach((pet, at) => {
        pet.x = places[at]!;
        if (fraction >= 1) settle(world, pet, now);
      });
      continue;
    }
    world.waits += members.length;
    for (const pet of members) {
      pet.waited++;
      if (pet.waited <= PATIENCE) continue;
      if (pet.x < perch.x0 || pet.x > perch.x1) {
        world.counts.spilled++;
        unfoot(pet, 0, 0);
      } else settle(world, pet, now);
    }
  }
}

/** 🎬️ The turn of one pet. */
function act(world: World, pet: Pet, now: number): void {
  if (pet.mode === "gone") {
    if (now >= pet.until && !arrive(world, pet, now)) pet.until = now + 64;
  } else if (pet.mode === "held") follow(world, pet, now);
  else if (pet.mode === "plan") fly(world, pet, now);
  else if (pet.mode === "drop") plunge(world, pet, now);
  else if (pet.mode === "ride") slide(world, pet, now);
  else if (pet.mode === "walk") stride(world, pet, now);
  else if (pet.mode === "stand" && now >= pet.until) decide(world, pet, now);
}

/** ⏭️ One tick of a world: the claims are pruned, the survey and the hand act, the seated scoot, every pet takes its turn (the first one rotates), and the tick is tallied: overlaps, near misses, poofs, waits, and whether the order on every perch was kept. */
export function tickWorld(world: World): void {
  world.tick++;
  const now = world.tick;
  const poofs = world.poofs;
  const waits = world.waits;
  world.claims = pruned(world.claims, now);
  survey(world, now);
  handle(world, now);
  scoot(world, now);
  for (let turn = 0; turn < world.pets.length; turn++) act(world, world.pets[(now + turn) % world.pets.length]!, now);
  const bodies = bodiesOf(world);
  world.tally = tallied(world.tally, bodies, NEAR, world.poofs - poofs, world.waits - waits);
  if (nearMisses(bodies, NEAR).some((pair) => world.pets.some((pet) => pet.mode === "plan" && (pet.slug === pair.first || pet.slug === pair.second)))) world.counts.brushes++;
  const orders = new Map<string, string[]>();
  for (const perch of world.perches) {
    const order = orderOf(world.pets.filter((pet) => grounded(pet) && pet.perch === perch.surface).map((pet) => ({ owner: pet.slug, extent: extentOf(pet) })));
    const before = world.orders.get(perch.surface);
    if (before !== undefined && !orderKept(before, order)) world.counts.disorders++;
    orders.set(perch.surface, order);
  }
  world.orders = orders;
}

/** 🏁️ A world run for a number of ticks. */
export function runWorld(seed: number, ticks: number, rules: Rules, span: number): World {
  const world = openWorld(seed, rules, span);
  for (let tick = 0; tick < ticks; tick++) tickWorld(world);
  return world;
}
//#endregion 🔖️Reference world

//#region 🔖️Tools of the cases
/** 🔬️ How many generated cases a comparison with brute force takes at the level of the run. */
const CASES = sampled(40, 400, 4000);

/** 🌐️ How many worlds are run with every rule in force at the level of the run, and for how many ticks each: at the exhaustive level the 24 × 30 000 ticks of the research prototype. */
const WORLDS = sampled(2, 6, 24);
const WORLD_TICKS = sampled(1500, 5000, 30000);

/** 🔪️ How many worlds are run with one rule switched off at the exhaustive level, and for how many ticks each: the ablations of the research prototype (the other levels run the short runs of `BITES` only, because the rarer rules bite in a few worlds of twelve). */
const ABLATIONS = sampled(0, 0, 12);
const ABLATION_TICKS = sampled(0, 0, 15000);

/** 🦷️ Per core rule the cheapest run in which the world without that rule lets bodies overlap, found with `clearance_world.ts --teeth 40 --ticks 2500` (the first overlap: guard seed 12 tick 24, corridors seed 9 tick 88, vetting seed 23 tick 23, rests seed 21 tick 81, projection seed 6 tick 7, eviction seed 17 tick 21): the seed and a number of ticks a little beyond the first overlap. */
const BITES: readonly { readonly rule: keyof Rules; readonly seed: number; readonly ticks: number }[] = [
  { rule: "guard", seed: 12, ticks: 40 },
  { rule: "corridors", seed: 9, ticks: 120 },
  { rule: "vetting", seed: 23, ticks: 40 },
  { rule: "rests", seed: 21, ticks: 110 },
  { rule: "projection", seed: 6, ticks: 20 },
  { rule: "eviction", seed: 17, ticks: 40 },
];

/** 🧰️ A box by its four edges. */
function box(x0: number, y0: number, x1: number, y1: number): Extent {
  return { x0, y0, x1, y1 };
}

/** 🏷️ A body by its owner and the four edges of its box. */
function body(owner: string, x0: number, y0: number, x1: number, y1: number): Body {
  return { owner, extent: { x0, y0, x1, y1 } };
}

/** 🎰️ A deterministic stream of integers in `[0, bound)` (a 32-bit linear congruential generator). */
function stream(seed: number): (bound: number) => number {
  let state = seed >>> 0;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return Math.floor((state / 4294967296) * bound);
  };
}

/** 🥶️ A value frozen all the way down, so that a function that wrote into its input would throw. */
function frozen<Value>(value: Value): Value {
  if (typeof value === "object" && value !== null) {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}

/** 🟰️ Whether two boxes share an area, decided by the size of their intersection instead of by comparisons of edges. */
function sharing(one: Extent, other: Extent): boolean {
  return Math.min(one.x1, other.x1) - Math.max(one.x0, other.x0) > 0 && Math.min(one.y1, other.y1) - Math.max(one.y0, other.y0) > 0;
}

/** 🧱️ A few boxes on the quarter-pixel lattice around a body that is 40 wide and 50 tall at (100, 250). */
function scattered(next: (bound: number) => number, count: number): Extent[] {
  const boxes: Extent[] = [];
  for (let index = 0; index < count; index++) {
    const x0 = next(1200) / 4 - 50;
    const y0 = 150 + next(800) / 4;
    boxes.push(box(x0, y0, x0 + 4 + next(240) / 4, y0 + 4 + next(240) / 4));
  }
  return boxes;
}

/** 🛤️ The boxes of a 40 × 50 body that moves by a fixed step per tick. */
function marching(x: number, y: number, dx: number, dy: number, ticks: number): Extent[] {
  const extents: Extent[] = [];
  for (let tick = 0; tick < ticks; tick++) extents.push(box(x + dx * tick, y + dy * tick, x + dx * tick + 40, y + dy * tick + 50));
  return extents;
}
//#endregion 🔖️Tools of the cases

describe("constants", () => {
  it("are the starting values of the design", () => {
    expect([MARGIN, SEAM, FOREVER, PUSHES, PATIENCE, DELAY]).toEqual([4, 1 / 64, 2 ** 32 - 1, 4, 40, 32]);
    expect([SLIDE_OFF_SPEED, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, LEAN, LANE_LIFT, SCOOT_HASTE]).toEqual([32, 2, 128, 0.25, 0.9, 1.5]);
    expect(STEERINGS).toEqual([0, 30, -30, 60, -60, 90, -90, 130, -130, 180, -180, 240, -240]);
    expect(TALLY).toEqual({ ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 });
  });
});

describe("bodies", () => {
  const size = { width: 40, height: 48 };

  it("builds the box around the feet from size, hover, posture and margin", () => {
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, UPRIGHT, 4)).toEqual({ owner: "a", extent: { x0: 76, y0: 248, x1: 124, y1: 304 } });
    expect(bodyOf("a", { x: 100, y: 290 }, size, 10, UPRIGHT, 4).extent).toEqual({ x0: 76, y0: 238, x1: 124, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, leaning(48, 1), 4).extent).toEqual({ x0: 64, y0: 248, x1: 136, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, canopied(size, { width: 64, height: 28 }), 4).extent).toEqual({ x0: 64, y0: 220, x1: 136, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, { left: 3, right: 14, above: 0 }, 0).extent).toEqual({ x0: 77, y0: 252, x1: 134, y1: 300 });
  });

  it("widens a leaning body by a quarter of its height times the sine, and a canopied one by what the canopy overhangs", () => {
    expect(leaning(48, 1)).toEqual({ left: 12, right: 12, above: 0 });
    expect(leaning(48, -0.5)).toEqual({ left: 6, right: 6, above: 0 });
    expect(leaning(48, 0)).toEqual(UPRIGHT);
    expect(canopied(size, { width: 64, height: 28 })).toEqual({ left: 12, right: 12, above: 28 });
    expect(canopied(size, { width: 30, height: 20 })).toEqual({ left: 0, right: 0, above: 20 });
    expect(leaning(48, 1).left).toBe(LEAN * 48);
  });

  it("lets neighbours with the comfortable gap of 8 px between their size boxes touch without overlapping", () => {
    const one = bodyOf("a", { x: 100, y: 300 }, size, 0, UPRIGHT, MARGIN);
    const apart = bodyOf("b", { x: 100 + (40 + 30) / 2 + 2 * MARGIN, y: 300 }, { width: 30, height: 40 }, 0, UPRIGHT, MARGIN);
    const closer = bodyOf("b", { x: 100 + (40 + 30) / 2 + 2 * MARGIN - 1 / 1024, y: 300 }, { width: 30, height: 40 }, 0, UPRIGHT, MARGIN);
    expect(apart.extent.x0).toBe(one.extent.x1);
    expect(overlaps([one, apart])).toEqual([]);
    expect(overlaps([one, closer])).toEqual([{ first: "a", second: "b" }]);
  });

  it("meets only where open boxes intersect", () => {
    const one = box(0, 0, 10, 10);
    expect(meets(one, box(10, 0, 20, 10))).toBe(false);
    expect(meets(one, box(0, 10, 10, 20))).toBe(false);
    expect(meets(one, box(9.999, 9.999, 20, 20))).toBe(true);
    expect(meets(one, box(2, 2, 3, 3))).toBe(true);
    expect(meets(box(2, 2, 3, 3), one)).toBe(true);
    expect(meets(one, box(-5, 4, 15, 6))).toBe(true);
    expect(meets(one, box(11, 0, 20, 10))).toBe(false);
    expect(meets(one, one)).toBe(true);
  });

  it("moves, grows and unites boxes", () => {
    expect(shifted(box(1, 2, 3, 4), 10, -1)).toEqual(box(11, 1, 13, 3));
    expect(grown(box(1, 2, 3, 4), 0.5)).toEqual(box(0.5, 1.5, 3.5, 4.5));
    expect(united(box(1, 2, 3, 4), box(-1, 3, 2, 9))).toEqual(box(-1, 2, 3, 9));
  });

  it("names every overlapping pair, by the first and then the second in the list", () => {
    const bodies = [body("a", 0, 0, 10, 10), body("b", 5, 5, 15, 15), body("c", 8, 8, 20, 20), body("d", 100, 100, 110, 110)];
    expect(overlaps(bodies)).toEqual([
      { first: "a", second: "b" },
      { first: "a", second: "c" },
      { first: "b", second: "c" },
    ]);
    expect(overlaps([bodies[3]!, bodies[2]!, bodies[0]!])).toEqual([{ first: "c", second: "a" }]);
    expect(overlaps([])).toEqual([]);
    expect(overlaps([bodies[0]!])).toEqual([]);
  });

  it("names as near misses the pairs that are apart but closer than the margin", () => {
    const bodies = [body("a", 0, 0, 10, 10), body("b", 11, 0, 20, 10), body("c", 5, 5, 8, 8), body("e", 11, 11, 20, 20)];
    expect(nearMisses(bodies, 2)).toEqual([
      { first: "a", second: "b" },
      { first: "a", second: "e" },
      { first: "b", second: "e" },
    ]);
    expect(nearMisses(bodies, 1.5)).toEqual(nearMisses(bodies, 2));
    expect(nearMisses(bodies, 1)).toEqual([]);
    expect(nearMisses(bodies, 0)).toEqual([]);
  });

  it("agrees with the area of the intersection on generated bodies", () => {
    const next = stream(11);
    for (let round = 0; round < CASES; round++) {
      const bodies = scattered(next, 8).map((extent, index) => ({ owner: `b${index}`, extent }));
      const expected: { first: string; second: string }[] = [];
      for (let first = 0; first < bodies.length; first++) for (let second = first + 1; second < bodies.length; second++) if (sharing(bodies[first]!.extent, bodies[second]!.extent)) expected.push({ first: bodies[first]!.owner, second: bodies[second]!.owner });
      expect(overlaps(bodies)).toEqual(expected);
    }
  });
});

describe("free places", () => {
  const bodies = [body("a", 0, 0, 10, 10), body("b", 20, 0, 30, 10)];
  const claims: Claim[] = [
    { owner: "c", slices: [{ from: 5, until: 5, extent: box(40, 0, 50, 10) }, { from: 6, until: 9, extent: box(60, 0, 70, 10) }], rest: box(80, 0, 90, 10) },
    { owner: "a", slices: [{ from: 5, until: 9, extent: box(200, 0, 210, 10) }], rest: box(220, 0, 230, 10) },
  ];

  it("gathers the bodies of the others, the corridors that are left and the rests", () => {
    expect(obstaclesOf("a", bodies, claims, 5)).toEqual([box(20, 0, 30, 10), box(40, 0, 50, 10), box(60, 0, 70, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("a", bodies, claims, 6)).toEqual([box(20, 0, 30, 10), box(60, 0, 70, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("a", bodies, claims, 10)).toEqual([box(20, 0, 30, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("c", bodies, claims, 10)).toEqual([box(0, 0, 10, 10), box(20, 0, 30, 10), box(220, 0, 230, 10)]);
    expect(obstaclesOf("nobody", [], [], 0)).toEqual([]);
  });

  it("calls a place free when no body, no corridor that is left and no rest of another owner overlaps it", () => {
    expect(freeAt(box(10, 0, 20, 10), "x", bodies, claims, 5)).toBe(true);
    expect(freeAt(box(5, 0, 15, 10), "x", bodies, claims, 5)).toBe(false);
    expect(freeAt(box(5, 0, 15, 10), "a", bodies, claims, 5)).toBe(true);
    expect(freeAt(box(45, 0, 55, 10), "x", bodies, claims, 5)).toBe(false);
    expect(freeAt(box(45, 0, 55, 10), "x", bodies, claims, 6)).toBe(true);
    expect(freeAt(box(85, 0, 95, 10), "x", bodies, claims, 1000)).toBe(false);
    expect(freeAt(box(85, 0, 95, 10), "c", bodies, claims, 1000)).toBe(true);
    expect(freeAmong(box(10, 0, 20, 10), [box(0, 0, 10, 10), box(20, 0, 30, 10)])).toBe(true);
    expect(freeAmong(box(10, 0, 20.5, 10), [box(0, 0, 10, 10), box(20, 0, 30, 10)])).toBe(false);
  });

  const extent = box(80, 250, 120, 300);

  it("finds the nearest place that keeps a seam from every obstacle of its height", () => {
    expect(slotIn(100, extent, 20, 400, [])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 20, 400, [box(60, 260, 90, 300)])).toBe(110 + SEAM);
    expect(slotIn(100, extent, 20, 400, [box(90, 260, 110, 300)])).toBe(70 - SEAM);
    expect(slotIn(100, extent, 20, 400, [box(110, 100, 150, 200)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 200, 150, 250)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 300, 150, 350)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 200, 150, 250.25)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 150, 400, [])).toBe(150);
    expect(slotIn(100, extent, 20, 60, [])).toBe(60);
  });

  it("answers a single point when that is all there is, and nothing when there is no room", () => {
    expect(slotIn(100, extent, 90 - SEAM, 160, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 90, 110, [box(110, 260, 150, 300)])).toBeNull();
    expect(slotIn(100, extent, 90, 170, [box(110, 260, 150, 300)])).toBeNull();
    expect(slotIn(100, extent, 90, 170 + SEAM, [box(110, 260, 150, 300)])).toBe(170 + SEAM);
    expect(slotIn(100, extent, 120, 110, [])).toBeNull();
    expect(slotIn(100, extent, Number.NaN, 110, [])).toBeNull();
  });

  it("keeps the footprint on the perch and looks down the whole column for a landing", () => {
    const perch: Perch = { surface: "s", x0: 50, x1: 300, y: 300 };
    expect(spotOn(perch, 100, extent, 16, [])).toBe(100);
    expect(spotOn(perch, 40, shifted(extent, -60, 0), 16, [])).toBe(66);
    expect(spotOn(perch, 400, shifted(extent, 300, 0), 16, [])).toBe(284);
    expect(spotOn({ surface: "s", x0: 50, x1: 70, y: 300 }, 60, shifted(extent, -40, 0), 16, [])).toBeNull();
    expect(spotOn(perch, 100, extent, 16, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    const high = box(80, 100, 120, 150);
    const below = [box(110, 200, 150, 250)];
    expect(spotOn(perch, 100, high, 16, below)).toBe(100);
    expect(columnOver(perch, 100, high, 150, 16, below)).toBe(90 - SEAM);
    expect(columnOver(perch, 100, high, 50, 16, below)).toBe(100);
    expect(columnOver(perch, 100, high, -30, 16, below)).toBe(100);
  });

  it("agrees with a search over every place of a 1/64 px lattice", () => {
    const next = stream(23);
    for (let round = 0; round < CASES; round++) {
      const x = 100 + next(400) / 4;
      const half = 10 + next(80) / 4;
      const own = box(x - half, 250, x + half, 300);
      const obstacles = scattered(next, 1 + next(5));
      const low = next(400) / 4;
      const high = low + next(800) / 4;
      let best: number | null = null;
      for (let step = 0; step <= (high - low) * 64; step++) {
        const place = low + step / 64;
        const there = shifted(own, place - x, 0);
        if (obstacles.some((obstacle) => sharing({ x0: there.x0 - SEAM, y0: there.y0, x1: there.x1 + SEAM, y1: there.y1 }, obstacle))) continue;
        if (best === null || Math.abs(place - x) < Math.abs(best - x)) best = place;
      }
      expect(slotIn(x, own, low, high, obstacles), `round ${round}`).toBe(best);
    }
  });
});

describe("claims", () => {
  it("cuts a path into slices of a span, each the smallest box around its ticks", () => {
    const path = marching(0, 0, 10, 1, 10);
    const exact = claimOf("a", 100, path, 1, path[9]!);
    expect(exact.owner).toBe("a");
    expect(exact.slices.length).toBe(10);
    expect(exact.slices[3]).toEqual({ from: 103, until: 103, extent: path[3] });
    expect(exact.rest).toEqual(path[9]);
    const coarse = claimOf("a", 100, path, 4, null);
    expect(coarse.slices).toEqual([
      { from: 100, until: 103, extent: box(0, 0, 70, 53) },
      { from: 104, until: 107, extent: box(40, 4, 110, 57) },
      { from: 108, until: 109, extent: box(80, 8, 130, 59) },
    ]);
    expect(coarse.rest).toBeNull();
    expect(claimOf("a", 7, path, 0, null).slices.length).toBe(10);
    expect(claimOf("a", 7, path, 2.9, null).slices.length).toBe(5);
    expect(claimOf("a", 7, [], 3, null).slices).toEqual([]);
  });

  it("answers where a claim has its owner at a tick", () => {
    const path = marching(0, 0, 10, 0, 6);
    const claim = claimOf("a", 10, path, 2, box(500, 0, 540, 50));
    expect(sliceAt(claim, 9)).toBeNull();
    expect(sliceAt(claim, 10)).toEqual(box(0, 0, 50, 50));
    expect(sliceAt(claim, 13)).toEqual(box(20, 0, 70, 50));
    expect(sliceAt(claim, 15)).toEqual(box(40, 0, 90, 50));
    expect(sliceAt(claim, 16)).toEqual(box(500, 0, 540, 50));
    expect(sliceAt(claimOf("a", 10, path, 2, null), 16)).toBeNull();
    expect(sliceAt(claimOf("a", 10, [], 2, box(0, 0, 1, 1)), 16)).toBeNull();
  });

  it("refuses a plan that runs into a body that stays, and ignores the owner's own body", () => {
    const path = marching(0, 0, 10, 0, 20);
    const plan = claimOf("a", 0, path, 1, path[19]!);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50)], [])).toBe(true);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50), body("b", 100, 60, 140, 110)], [])).toBe(true);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50), body("b", 100, 40, 140, 90)], [])).toBe(false);
    expect(claimClear(plan, [body("b", 230, 0, 270, 50)], [])).toBe(true);
    expect(claimClear(plan, [body("b", 229, 0, 269, 50)], [])).toBe(false);
  });

  it("lets two plans cross one place at different ticks and refuses them at the same tick", () => {
    const across = claimOf("a", 20, marching(0, 100, 10, 0, 21), 1, box(200, 100, 240, 150));
    const down = (from: number): Claim => claimOf("b", from, marching(100, -100, 0, 10, 31), 1, box(100, 200, 140, 250));
    expect(claimClear(down(0), [], [across])).toBe(true);
    expect(claimClear(down(10), [], [across])).toBe(false);
    expect(claimClear(down(30), [], [across])).toBe(true);
    expect(claimClear(across, [], [down(0)])).toBe(true);
    expect(claimClear(across, [], [down(10)])).toBe(false);
    expect(claimClear(across, [], [down(30)])).toBe(true);
  });

  it("refuses a rest where somebody arrives later, and a path through somebody's rest", () => {
    const late = claimOf("a", 0, marching(0, 0, 10, 0, 51), 1, box(500, 0, 540, 50));
    const early = claimOf("b", 0, marching(500, -300, 0, 10, 31), 1, box(500, 0, 540, 50));
    expect(claimClear(early, [], [late])).toBe(false);
    expect(claimClear(claimOf("b", 0, marching(500, -300, 0, 10, 31), 1, null), [], [late])).toBe(true);
    expect(claimClear(claimOf("b", 0, marching(560, -300, 0, 10, 31), 1, box(560, 0, 600, 50)), [], [late])).toBe(true);
    const through = claimOf("b", 100, marching(500, -300, 0, 10, 61), 1, box(500, 300, 540, 350));
    expect(claimClear(through, [], [late])).toBe(false);
    expect(claimClear(through, [], [claimOf("a", 0, marching(0, 0, 10, 0, 51), 1, null)])).toBe(true);
  });

  it("takes a planned body to be where its claim says, and where it stands until that claim begins", () => {
    const leaving = claimOf("a", 0, marching(100, 0, 10, 0, 31), 1, box(400, 0, 440, 50));
    const landing = claimOf("b", 0, marching(100, -300, 0, 10, 31), 1, box(100, 0, 140, 50));
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [leaving])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [])).toBe(false);
    const later = claimOf("a", 40, marching(300, 0, 10, 0, 11), 1, box(400, 0, 440, 50));
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [later])).toBe(false);
    expect(claimClear(landing, [], [later])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [claimOf("c", 0, marching(900, 0, 0, 0, 5), 1, null), leaving])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [leaving, claimOf("a", 0, marching(100, 0, 0, 0, 40), 1, null)])).toBe(false);
  });

  it("agrees with a comparison of every tick of every time line", () => {
    const next = stream(37);
    const horizon = 60;
    /** 🧵️ The box of an owner at every tick of the horizon: its body until its claim begins, its slices, then its rest. */
    const line = (own: Extent | null, claim: Claim | null): (Extent | null)[] => {
      const boxes: (Extent | null)[] = [];
      for (let tick = 0; tick <= horizon; tick++) {
        if (claim === null || claim.slices.length === 0) boxes.push(own);
        else if (tick < claim.slices[0]!.from) boxes.push(own);
        else boxes.push(sliceAt(claim, tick));
      }
      return boxes;
    };
    let clear = 0;
    for (let round = 0; round < CASES; round++) {
      const others = 1 + next(4);
      const bodies: Body[] = [];
      const claims: Claim[] = [];
      for (let index = 0; index < others; index++) {
        const owner = `o${index}`;
        const x = next(60) * 4;
        const y = next(40) * 4;
        if (next(4) > 0) bodies.push(body(owner, x, y, x + 40, y + 50));
        if (next(2) > 0) {
          const path = marching(x, y, next(9) - 4, next(9) - 4, 1 + next(30));
          claims.push(claimOf(owner, next(20), path, 1 + next(4), next(3) > 0 ? path[path.length - 1]! : null));
        }
      }
      const from = next(20);
      const path = marching(next(60) * 4, next(40) * 4, next(9) - 4, next(9) - 4, 1 + next(30));
      const plan = claimOf("me", from, path, 1 + next(4), next(3) > 0 ? path[path.length - 1]! : null);
      let expected = true;
      const mine = line(null, plan);
      for (let index = 0; index < others; index++) {
        const owner = `o${index}`;
        const theirs = line(bodies.find((entry) => entry.owner === owner)?.extent ?? null, claims.find((entry) => entry.owner === owner) ?? null);
        for (let tick = from; tick <= horizon; tick++) if (mine[tick] !== null && theirs[tick] !== null && sharing(mine[tick]!, theirs[tick]!)) expected = false;
      }
      if (expected) clear++;
      expect(claimClear(plan, bodies, claims), `round ${round}`).toBe(expected);
    }
    expect(clear).toBeGreaterThan(CASES / 20);
    expect(clear).toBeLessThan(CASES - CASES / 20);
  });

  it("releases the claims of an owner and prunes what is over", () => {
    const one = claimOf("a", 0, marching(0, 0, 1, 0, 10), 5, box(9, 0, 49, 50));
    const two = claimOf("b", 3, marching(0, 0, 1, 0, 4), 2, null);
    expect(released([one, two], "a")).toEqual([two]);
    expect(released([one, two], "c")).toEqual([one, two]);
    expect(pruned([one, two], 0)).toEqual([one, two]);
    expect(pruned([one, two], 5)).toEqual([{ owner: "a", slices: [one.slices[1]], rest: one.rest }, { owner: "b", slices: [two.slices[1]], rest: null }]);
    expect(pruned([one, two], 7)).toEqual([{ owner: "a", slices: [one.slices[1]], rest: one.rest }]);
    expect(pruned([one, two], 10)).toEqual([]);
  });
});

describe("order on a perch", () => {
  const extent = box(80, 250, 120, 300);

  it("cuts a stride short a seam before the first obstacle ahead that shares its height", () => {
    expect(guardedStride(extent, 5, [])).toBe(5);
    expect(guardedStride(extent, -5, [])).toBe(-5);
    expect(guardedStride(extent, 0, [box(0, 0, 1000, 1000)])).toBe(0);
    expect(guardedStride(extent, 5, [box(123, 260, 160, 300)])).toBe(3 - SEAM);
    expect(guardedStride(extent, 2, [box(123, 260, 160, 300)])).toBe(2);
    expect(guardedStride(extent, 5, [box(120.01, 260, 160, 300)])).toBe(0);
    expect(guardedStride(extent, -5, [box(40, 260, 77, 300)])).toBe(SEAM - 3);
    expect(guardedStride(extent, -5, [box(123, 260, 160, 300)])).toBe(-5);
    expect(guardedStride(extent, 5, [box(40, 260, 77, 300)])).toBe(5);
    expect(guardedStride(extent, 5, [box(123, 100, 160, 250)])).toBe(5);
    expect(guardedStride(extent, 5, [box(123, 300, 160, 350)])).toBe(5);
    expect(guardedStride(extent, 5, [box(200, 260, 240, 300), box(123, 260, 160, 300), box(122, 100, 160, 200)])).toBe(3 - SEAM);
  });

  it("lets a body out of an obstacle it overlaps, never further in", () => {
    const inside = [box(110, 260, 150, 300)];
    expect(guardedStride(extent, 5, inside)).toBe(0);
    expect(guardedStride(extent, -5, inside)).toBe(-5);
    expect(guardedStride(extent, 5, [box(60, 260, 100, 300)])).toBe(5);
    expect(guardedStride(extent, -5, [box(60, 260, 100, 300)])).toBe(0);
    expect(guardedStride(extent, 5, [box(60, 260, 140, 300)])).toBe(5);
    expect(guardedStride(extent, -5, [box(60, 260, 140, 300)])).toBe(-5);
  });

  it("stops before the corridors and the rests of claims as before bodies", () => {
    const claims: Claim[] = [{ owner: "c", slices: [{ from: 5, until: 5, extent: box(122, 0, 160, 300) }, { from: 6, until: 9, extent: box(124, 0, 160, 300) }], rest: box(126, 250, 160, 300) }];
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 5))).toBe(2 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 6))).toBe(4 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 10))).toBe(6 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("c", [], claims, 5))).toBe(9);
  });

  it("agrees with a walk along a 1/64 px lattice that stops at the first place too close to an obstacle", () => {
    const next = stream(41);
    let hindered = 0;
    for (let round = 0; round < CASES; round++) {
      const x = 150 + next(200) / 4;
      const own = box(x - 20, 250, x + 20, 300);
      const stride = (next(161) - 80) / 4;
      const side = stride < 0 ? -1 : 1;
      const near = x + 20 + next(80) / 4;
      const far = x - 20 - next(80) / 4;
      const flank = side > 0 ? box(near, 240 + next(40), near + 10 + next(80) / 4, 310) : box(far - 10 - next(80) / 4, 240 + next(40), far, 310);
      const obstacles = [...scattered(next, 1 + next(5)), flank].filter((obstacle) => !sharing({ x0: own.x0 - SEAM, y0: own.y0, x1: own.x1 + SEAM, y1: own.y1 }, obstacle));
      let reach = 0;
      for (let step = 1; step <= Math.abs(stride) * 64; step++) {
        const there = shifted(own, (side * step) / 64, 0);
        if (obstacles.some((obstacle) => sharing({ x0: there.x0 - SEAM, y0: there.y0, x1: there.x1 + SEAM, y1: there.y1 }, obstacle))) break;
        reach = step / 64;
      }
      if (reach < Math.abs(stride)) hindered++;
      expect(guardedStride(own, stride, obstacles), `round ${round}`).toBe(reach === 0 ? 0 : side * reach);
    }
    expect(hindered).toBeGreaterThan(CASES / 5);
  });

  it("keeps walkers on one line apart and in their order, which nothing but the guard does", () => {
    const ticks = sampled(400, 4000, 40000);
    for (const guarded of [true, false]) {
      const next = stream(5);
      const walkers: Body[] = [];
      let x = 0;
      for (let index = 0; index < 7; index++) {
        const width = 20 + next(21);
        walkers.push(body(`w${index}`, x, 0, x + width, 40));
        x = x + width + SEAM + next(40) / 8;
      }
      const order = orderOf(walkers);
      let collisions = 0;
      let swaps = 0;
      for (let tick = 0; tick < ticks; tick++) {
        for (let turn = 0; turn < walkers.length; turn++) {
          const index = (tick + turn) % walkers.length;
          const walker = walkers[index]!;
          const wanted = (next(49) - 24) / 8;
          const stride = guarded ? guardedStride(walker.extent, wanted, obstaclesOf(walker.owner, walkers, [], tick)) : wanted;
          walkers[index] = { owner: walker.owner, extent: shifted(walker.extent, stride, 0) };
        }
        if (overlaps(walkers).length > 0) collisions++;
        if (!orderKept(order, orderOf(walkers))) swaps++;
      }
      if (guarded) expect([collisions, swaps]).toEqual([0, 0]);
      else {
        expect(collisions).toBeGreaterThan(ticks / 10);
        expect(swaps).toBeGreaterThan(0);
      }
    }
  });

  it("ranks bodies from left to right and tells whether an order was kept", () => {
    const bodies = [body("c", 50, 0, 60, 10), body("a", 0, 0, 10, 10), body("b", 20, 0, 40, 10), body("d", 25, 0, 35, 10)];
    expect(orderOf(bodies)).toEqual(["a", "b", "d", "c"]);
    expect(orderOf([])).toEqual([]);
    expect(orderKept(["a", "b", "c"], ["a", "b", "c"])).toBe(true);
    expect(orderKept(["a", "b", "c"], ["a", "x", "c"])).toBe(true);
    expect(orderKept(["a", "b", "c"], ["c", "y", "a"])).toBe(false);
    expect(orderKept(["a", "b", "c"], ["b", "a"])).toBe(false);
    expect(orderKept(["a", "b", "c"], [])).toBe(true);
    expect(orderKept([], ["a"])).toBe(true);
  });

  it("allows a swap by a hop only when the rise lifts the hopper over its neighbour", () => {
    const hopper = box(80, 250, 120, 304);
    const hurdle = box(130, 244, 170, 304);
    expect(vaults(hopper, 60 + SEAM, hurdle)).toBe(true);
    expect(vaults(hopper, 60, hurdle)).toBe(false);
    expect(vaults(hopper, 84, box(130, 219, 170, 304))).toBe(false);
    expect(vaults(hopper, 84, box(130, 221, 170, 304))).toBe(true);
  });

  it("never finds a hop over a neighbour clear whose rise does not vault it", () => {
    const size = { width: 30, height: 40 };
    const from = { x: 100, y: 300 };
    let vaulted = 0;
    let refused = 0;
    for (let dx = 60; dx <= 160; dx += 5) {
      const to = { x: from.x + dx, y: 300 };
      const hop = hopOf(from, to);
      if (hop === null) continue;
      const path: Extent[] = [bodyOf("h", from, size, 0, UPRIGHT, MARGIN).extent];
      let flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
      for (let left = hop.ticks; left >= 1; left--) {
        flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
        path.push(bodyOf("h", flight, size, 0, UPRIGHT, MARGIN).extent);
      }
      const plan = claimOf("h", 0, path, 1, path[path.length - 1]!);
      const rise = path[0]!.y1 - Math.min(...path.map((extent) => extent.y1));
      for (let height = 4; height <= 60; height += 4) {
        for (let width = 4; width <= 40; width += 12) {
          const hurdle = bodyOf("n", { x: from.x + dx / 2, y: 300 }, { width, height }, 0, UPRIGHT, MARGIN);
          if (claimClear(plan, [hurdle], [])) {
            vaulted++;
            expect(vaults(path[0]!, rise, hurdle.extent), `dx ${dx} height ${height} width ${width}`).toBe(true);
          } else refused++;
        }
      }
    }
    expect(vaulted).toBeGreaterThan(20);
    expect(refused).toBeGreaterThan(20);
  });
});

describe("seating", () => {
  const perch: Perch = { surface: "s", x0: 100, x1: 400, y: 300 };

  /** 💺️ The boxes after a seating, in the order of its seats. */
  const seated = (bodies: readonly Body[], seating: { readonly seats: readonly { readonly owner: string; readonly shift: number }[] }): Body[] => seating.seats.map((seat) => ({ owner: seat.owner, extent: shifted(bodies.find((entry) => entry.owner === seat.owner)!.extent, seat.shift, 0) }));

  it("leaves a valid seating exactly as it is", () => {
    const bodies = [body("b", 220, 248, 268, 304), body("a", 150, 248, 198, 304), body("c", 268 + SEAM, 248, 300, 304)];
    expect(seatOf(bodies, perch, MARGIN)).toEqual({ seats: [{ owner: "a", shift: 0 }, { owner: "b", shift: 0 }, { owner: "c", shift: 0 }], leavers: [] });
    expect(seatOf([], perch, MARGIN)).toEqual({ seats: [], leavers: [] });
  });

  it("moves two bodies that overlap apart by the same amount, a seam between them", () => {
    const seating = seatOf([body("a", 150, 248, 198, 304), body("b", 190, 248, 238, 304)], perch, MARGIN);
    expect(seating).toEqual({ seats: [{ owner: "a", shift: -(4 + SEAM / 2) }, { owner: "b", shift: 4 + SEAM / 2 }], leavers: [] });
  });

  it("pools a whole run of violators into one block", () => {
    const bodies = [body("a", 200, 248, 240, 304), body("b", 210, 248, 250, 304), body("c", 220, 248, 260, 304), body("d", 300, 248, 340, 304)];
    const after = seated(bodies, seatOf(bodies, perch, MARGIN));
    expect(after.map((entry) => entry.extent.x0)).toEqual([170 - SEAM, 210, 250 + SEAM, 300]);
  });

  it("brings bodies back onto the perch, their margin beyond its ends", () => {
    expect(seatOf([body("a", 60, 248, 108, 304)], perch, MARGIN).seats).toEqual([{ owner: "a", shift: 36 }]);
    expect(seatOf([body("a", 380, 248, 428, 304)], perch, MARGIN).seats).toEqual([{ owner: "a", shift: -24 }]);
    const bodies = [body("a", 300, 248, 348, 304), body("b", 352, 248, 400, 304)];
    const short: Perch = { surface: "s", x0: 100, x1: 380, y: 300 };
    expect(seatOf(bodies, short, MARGIN).seats).toEqual([{ owner: "a", shift: -(12 + SEAM) }, { owner: "b", shift: -16 }]);
    expect(seated(bodies, seatOf(bodies, short, MARGIN))[1]!.extent.x1).toBe(short.x1 + MARGIN);
  });

  it("names whoever does not fit for leaving, the last in the list first", () => {
    const narrow: Perch = { surface: "s", x0: 100, x1: 200, y: 300 };
    const bodies = [body("c", 180, 248, 228, 304), body("a", 90, 248, 138, 304), body("b", 130, 248, 178, 304)];
    const seating = seatOf(bodies, narrow, MARGIN);
    expect(seating.leavers).toEqual(["b"]);
    expect(seating.seats.map((seat) => seat.owner)).toEqual(["a", "c"]);
    expect(overlaps(seated(bodies, seating))).toEqual([]);
    expect(seatOf([body("a", 90, 248, 210, 304)], narrow, MARGIN)).toEqual({ seats: [], leavers: ["a"] });
    expect(seatOf([body("a", 90, 248, 198, 304)], narrow, MARGIN).leavers).toEqual([]);
    expect(seatOf([body("a", 0, 248, 60, 304), body("b", 0, 248, 60, 304), body("c", 0, 248, 60, 304), body("d", 0, 248, 60, 304)], narrow, MARGIN).leavers).toEqual(["d", "c", "b"]);
  });

  it("is the nearest valid seating: it agrees with a search over every way to pool neighbours", () => {
    const next = stream(53);
    let moved = 0;
    for (let round = 0; round < CASES; round++) {
      const count = 1 + next(5);
      const bodies: Body[] = [];
      for (let index = 0; index < count; index++) {
        const x0 = 80 + next(1000) / 4;
        bodies.push(body(`s${index}`, x0, 248, x0 + 30 + next(30), 304));
      }
      const seating = seatOf(bodies, perch, MARGIN);
      if (seating.leavers.length > 0) continue;
      const order = orderOf(bodies).map((owner) => bodies.find((entry) => entry.owner === owner)!);
      expect(seating.seats.map((seat) => seat.owner)).toEqual(order.map((entry) => entry.owner));
      const offsets = [0];
      for (let at = 1; at < count; at++) offsets.push(offsets[at - 1]! + (order[at - 1]!.extent.x1 - order[at - 1]!.extent.x0) + SEAM);
      const wanted = order.map((entry, at) => entry.extent.x0 - offsets[at]!);
      const low = perch.x0 - MARGIN;
      const high = perch.x1 + MARGIN - (order[count - 1]!.extent.x1 - order[count - 1]!.extent.x0) - offsets[count - 1]!;
      let best: number[] = [];
      let least = Infinity;
      for (let cuts = 0; cuts < 1 << (count - 1); cuts++) {
        const values: number[] = [];
        let start = 0;
        for (let at = 0; at < count; at++) {
          if (at < count - 1 && (cuts & (1 << at)) === 0) continue;
          const block = wanted.slice(start, at + 1);
          const mean = Math.min(Math.max(block.reduce((sum, value) => sum + value, 0) / block.length, low), high);
          for (let member = start; member <= at; member++) values.push(mean);
          start = at + 1;
        }
        if (values.some((value, at) => at > 0 && value < values[at - 1]!)) continue;
        const cost = values.reduce((sum, value, at) => sum + (value - wanted[at]!) ** 2, 0);
        if (cost < least - 1e-9) {
          least = cost;
          best = values;
        }
      }
      seating.seats.forEach((seat, at) => expect(Math.abs(seat.shift - (best[at]! - wanted[at]!)), `round ${round} seat ${at}`).toBeLessThan(1e-9));
      if (seating.seats.some((seat) => seat.shift !== 0)) moved++;
      const after = seated(bodies, seating);
      expect(overlaps(after), `round ${round}`).toEqual([]);
      for (let at = 1; at < count; at++) expect(after[at]!.extent.x0 - after[at - 1]!.extent.x1, `round ${round}`).toBeGreaterThan(SEAM - 1e-9);
      expect(after[0]!.extent.x0).toBeGreaterThan(low - 1e-9);
      expect(after[count - 1]!.extent.x1).toBeLessThan(perch.x1 + MARGIN + 1e-9);
      for (const seat of seatOf(after, perch, MARGIN).seats) expect(Math.abs(seat.shift), `round ${round}`).toBeLessThan(1e-9);
    }
    expect(moved).toBeGreaterThan(CASES / 4);
  });

  it("scoots a group by one common share of the way, which never overlaps within the group", () => {
    expect(scootFraction([3, -6, 0], 1.5)).toBe(0.25);
    expect(scootFraction([3, -6, 0], 10)).toBe(1);
    expect(scootFraction([3, -6, 0], 6)).toBe(1);
    expect(scootFraction([0, 0], 1.5)).toBe(1);
    expect(scootFraction([], 1.5)).toBe(1);
    expect(scootFraction([3], 0)).toBe(0);
    expect(scootFraction([3], -2)).toBe(0);
    expect(scootFraction([0], -2)).toBe(1);
    expect(scooted(10, 20, 0.25)).toBe(12.5);
    expect(scooted(10, 20, 0)).toBe(10);
    expect(scooted(10, 20, 1)).toBe(20);
    expect(scooted(10, 20, 1.5)).toBe(20);
    const next = stream(67);
    for (let round = 0; round < CASES; round++) {
      const bodies: Body[] = [];
      let x = 100 + next(200) / 4;
      for (let index = 0; index < 2 + next(4); index++) {
        const width = 30 + next(30);
        bodies.push(body(`s${index}`, x, 248, x + width, 304));
        x = x + width + SEAM + next(160) / 4;
      }
      const shrunk: Perch = { surface: "s", x0: 100 + next(400) / 4, x1: 500 + next(800) / 4, y: 300 };
      const seating = seatOf(bodies, shrunk, MARGIN);
      const members = seating.seats.map((seat) => bodies.find((entry) => entry.owner === seat.owner)!);
      const goals = seating.seats.map((seat, at) => members[at]!.extent.x0 + seat.shift);
      let places = members.map((entry) => entry.extent.x0);
      let fraction = 0;
      for (let tick = 0; tick < 2000 && fraction < 1; tick++) {
        fraction = scootFraction(places.map((place, at) => goals[at]! - place), 1.125);
        places = places.map((place, at) => scooted(place, goals[at]!, fraction));
        const there = members.map((entry, at) => ({ owner: entry.owner, extent: shifted(entry.extent, places[at]! - entry.extent.x0, 0) }));
        expect(overlaps(there), `round ${round} tick ${tick}`).toEqual([]);
      }
      expect(places).toEqual(goals);
    }
  });
});

describe("heads", () => {
  const host = body("h", 100, 240, 150, 304);

  it("lands a descending body on the top it comes down onto, like a perch crossing", () => {
    const before = box(90, 180, 130, 236);
    const after = box(90, 190, 130, 246);
    expect(headUnder(before, after, "f", [host])).toBe(host);
    expect(liftOnto(after, host.extent)).toBe(-(6 + SEAM));
    expect(shifted(after, 0, liftOnto(after, host.extent)).y1).toBe(240 - SEAM);
    expect(headUnder(before, after, "h", [host])).toBeNull();
    expect(headUnder(shifted(before, -50, 0), shifted(after, -50, 0), "f", [host])).toBeNull();
    expect(headUnder(shifted(before, -30, 0), shifted(after, -29.75, 0), "f", [host])).toBe(host);
    expect(headUnder(shifted(before, -30, 0), shifted(after, -30, 0), "f", [host])).toBeNull();
    expect(headUnder(after, before, "f", [host])).toBeNull();
    expect(headUnder(shifted(before, 0, 14), shifted(after, 0, 14), "f", [host])).toBeNull();
    expect(headUnder(before, shifted(before, 0, 3), "f", [host])).toBeNull();
    expect(headUnder(before, shifted(before, 0, 4), "f", [host])).toBe(host);
    expect(headUnder(shifted(before, 0, 4), shifted(before, 0, 4.25), "f", [host])).toBe(host);
    expect(headUnder(before, after, "f", [])).toBeNull();
  });

  it("chooses the highest head, and the first one among equals", () => {
    const before = box(90, 100, 180, 200);
    const after = box(90, 160, 180, 260);
    const taller = body("t", 150, 220, 200, 304);
    const twin = body("w", 60, 220, 95, 304);
    expect(headUnder(before, after, "f", [host, taller])).toBe(taller);
    expect(headUnder(before, after, "f", [taller, host])).toBe(taller);
    expect(headUnder(before, after, "f", [host, twin, taller])).toBe(twin);
  });

  it("tells whether a rider still stands on its host", () => {
    const rider = box(90, 180, 130, 240 - SEAM);
    expect(restsOn(rider, host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, SEAM), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, -SEAM), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, -2 * SEAM), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, 0, 2 * SEAM), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, -30, 0), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, -29.75, 0), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 60, 0), host.extent)).toBe(false);
  });

  it("slides off to the nearer side, faster and faster up to a limit", () => {
    expect(slideSide(box(110, 0, 150, 10), host.extent)).toBe(1);
    expect(slideSide(box(105, 0, 145, 10), host.extent)).toBe(1);
    expect(slideSide(box(104, 0, 145, 10), host.extent)).toBe(-1);
    expect(slideStride(0)).toBe(0.5);
    expect(slideStride(10)).toBe(0.8125);
    expect(slideStride(48)).toBe(2);
    expect(slideStride(1000)).toBe(2);
    for (let tick = 1; tick < 100; tick++) expect(slideStride(tick)).toBeGreaterThanOrEqual(slideStride(tick - 1));
  });

  it("guards a slide, keeps it on the stage and turns it round where it cannot move", () => {
    const rider = box(90, 180, 130, 240 - SEAM);
    expect(slideOf(rider, 1, 0, 0, 960, [host.extent])).toEqual({ stride: 0.5, side: 1 });
    expect(slideOf(rider, -1, 10, 0, 960, [host.extent])).toEqual({ stride: -0.8125, side: -1 });
    expect(slideOf(rider, 1, 0, 0, 960, [box(130.25, 100, 200, 235)])).toEqual({ stride: 0.25 - SEAM, side: 1 });
    expect(slideOf(rider, 1, 0, 0, 960, [box(130 + SEAM, 100, 200, 235)])).toEqual({ stride: 0, side: -1 });
    expect(slideOf(rider, -1, 0, 0, 960, [box(40, 100, 90, 235)])).toEqual({ stride: 0, side: 1 });
    expect(slideOf(rider, 1, 0, 0, 130, [])).toEqual({ stride: 0, side: -1 });
    expect(slideOf(rider, 1, 0, 0, 130.25, [])).toEqual({ stride: 0.25, side: 1 });
    expect(slideOf(rider, -1, 0, 90, 960, [])).toEqual({ stride: 0, side: 1 });
    expect(slideOf(rider, -1, 0, 89.75, 960, [])).toEqual({ stride: -0.25, side: -1 });
  });
});

describe("held", () => {
  const extent = box(100, 100, 140, 150);

  it("takes the shortest way out of an obstacle, a seam to spare", () => {
    expect(pushedOut(extent, [], PUSHES)).toEqual({ x: 0, y: 0 });
    expect(pushedOut(extent, [box(140, 100, 200, 150), box(0, 150, 300, 200)], PUSHES)).toEqual({ x: 0, y: 0 });
    expect(pushedOut(extent, [box(130, 90, 200, 160)], PUSHES)).toEqual({ x: -(10 + SEAM), y: 0 });
    expect(pushedOut(extent, [box(40, 90, 108, 160)], PUSHES)).toEqual({ x: 8 + SEAM, y: 0 });
    expect(pushedOut(extent, [box(90, 140, 150, 200)], PUSHES)).toEqual({ x: 0, y: -(10 + SEAM) });
    expect(pushedOut(extent, [box(90, 60, 150, 105)], PUSHES)).toEqual({ x: 0, y: 5 + SEAM });
    expect(pushedOut(extent, [box(110, 0, 130, 300)], PUSHES)).toEqual({ x: -(30 + SEAM), y: 0 });
    expect(pushedOut(extent, [box(0, 115, 300, 135)], PUSHES)).toEqual({ x: 0, y: -(35 + SEAM) });
  });

  it("goes on from obstacle to obstacle and gives up when its rounds are over", () => {
    const pair = [box(130, 90, 200, 160), box(80, 60, 125, 105)];
    expect(pushedOut(extent, pair, PUSHES)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, pair, 1)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, [pair[1]!, pair[0]!], 1)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, [box(130, 90, 200, 160), box(60, 90, 95, 160)], PUSHES)).toBeNull();
    const walls = [box(0, 0, 105, 300), box(135, 0, 300, 300)];
    expect(pushedOut(extent, walls, PUSHES)).toBeNull();
    expect(pushedOut(extent, walls, 100)).toBeNull();
    expect(pushedOut(extent, [box(130, 90, 200, 160)], 0)).toBeNull();
    expect(pushedOut(extent, [box(140, 90, 200, 160)], 0)).toEqual({ x: 0, y: 0 });
  });

  it("answers a free place or nothing on generated crowds", () => {
    const next = stream(71);
    let freed = 0;
    let stuck = 0;
    for (let round = 0; round < CASES; round++) {
      const own = box(100, 250, 140, 300);
      const obstacles = scattered(next, 1 + next(4));
      const push = pushedOut(own, obstacles, PUSHES);
      if (push === null) {
        stuck++;
        continue;
      }
      expect(freeAmong(shifted(own, push.x, push.y), obstacles), `round ${round}`).toBe(true);
      if (!freeAmong(own, obstacles)) freed++;
      else expect(push).toEqual({ x: 0, y: 0 });
      if (obstacles.length === 1 && !freeAmong(own, obstacles)) {
        const only = obstacles[0]!;
        const ways = [own.x1 - only.x0, only.x1 - own.x0, own.y1 - only.y0, only.y1 - own.y0];
        expect(Math.abs(push.x) + Math.abs(push.y), `round ${round}`).toBe(Math.min(...ways) + SEAM);
      }
    }
    expect(freed).toBeGreaterThan(CASES / 10);
    expect(stuck).toBeLessThan(CASES / 2);
  });
});

describe("the last resort", () => {
  const bodies = [body("a", 0, 0, 40, 50), body("b", 100, 0, 140, 50)];
  const clear = claimOf("a", 0, marching(0, 0, 0, 10, 10), 1, box(0, 90, 40, 140));
  const blocked = claimOf("a", 0, marching(0, 0, 10, 0, 10), 1, box(90, 0, 130, 50));

  it("lets a pet be as long as it may stay or one of its plans is clear", () => {
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", null, [blocked, clear], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", box(70, 0, 110, 50), [blocked, clear], bodies, [], 0)).toBe(false);
  });

  it("poofs a pet that may not stay and has no clear plan", () => {
    expect(mustPoof("a", null, [], bodies, [], 0)).toBe(true);
    expect(mustPoof("a", null, [blocked], bodies, [], 0)).toBe(true);
    expect(mustPoof("a", box(70, 0, 110, 50), [blocked], bodies, [], 0)).toBe(true);
    const corridor = [claimOf("c", 5, marching(0, -100, 0, 20, 6), 1, null)];
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, corridor, 5)).toBe(true);
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, corridor, 11)).toBe(false);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked], bodies, corridor, 5)).toBe(true);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked, clear], bodies, corridor, 5)).toBe(false);
  });

  it("names whoever a change from outside carried into somebody who stays", () => {
    expect(evicted(bodies, [], 0)).toEqual([]);
    expect(evicted([...bodies, body("c", 30, 0, 70, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([body("c", 30, 0, 70, 50), ...bodies], [], 0)).toEqual(["a"]);
    expect(evicted([...bodies, body("c", 30, 0, 70, 50), body("d", 60, 0, 90, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([...bodies, body("c", 30, 0, 110, 50), body("d", 60, 0, 90, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([...bodies, body("d", 60, 0, 90, 50), body("c", 30, 0, 110, 50)], [], 0)).toEqual(["c"]);
  });

  it("evicts a body without a plan from the corridors and rests of those who stay", () => {
    const plan = claimOf("b", 0, marching(100, 0, 20, 0, 10), 1, box(300, 0, 340, 50));
    const rider = body("c", 200, 0, 240, 50);
    expect(evicted([...bodies, rider], [plan], 0)).toEqual(["c"]);
    expect(evicted([...bodies, rider], [plan], 8)).toEqual([]);
    expect(evicted([...bodies, body("c", 310, 0, 350, 50)], [plan], 100)).toEqual(["c"]);
    expect(evicted([...bodies, rider], [plan, claimOf("c", 0, marching(200, 0, 0, 0, 3), 1, null)], 0)).toEqual([]);
    expect(evicted([body("a", 0, 0, 40, 50), body("b", 30, 0, 70, 50), rider], [plan], 0)).toEqual(["b"]);
  });
});

describe("lanes and metrics", () => {
  it("lifts the high lane of a floater by nine tenths of its height and margin", () => {
    expect(laneLift(48, 4)).toBe(LANE_LIFT * 52);
    expect(laneLift(40, 0)).toBe(36);
  });

  it("tallies a run tick by tick", () => {
    const apart = [body("a", 0, 0, 10, 10), body("b", 20, 0, 30, 10)];
    const near = [body("a", 0, 0, 10, 10), body("b", 11, 0, 30, 10)];
    const over = [body("a", 0, 0, 10, 10), body("b", 9, 0, 30, 10), body("c", 9.5, 0, 12, 10)];
    let tally: Tally = TALLY;
    tally = tallied(tally, apart, 2, 0, 1);
    expect(tally).toEqual({ ticks: 1, actors: 2, overlaps: 0, nears: 0, poofs: 0, waits: 1 });
    tally = tallied(tally, near, 2, 1, 0);
    expect(tally).toEqual({ ticks: 2, actors: 4, overlaps: 0, nears: 1, poofs: 1, waits: 1 });
    tally = tallied(tally, over, 2, 2, 3);
    expect(tally).toEqual({ ticks: 3, actors: 7, overlaps: 1, nears: 0 + 1, poofs: 3, waits: 4 });
    expect(TALLY).toEqual({ ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 });
    expect(perMillion(3, 7)).toBe(3000000 / 7);
    expect(perMillion(3, 0)).toBe(0);
  });
});

describe("determinism", () => {
  it("uses nothing but exact operations and has no side effect", () => {
    const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
    const used = [...source.matchAll(/Math\.(\w+)/g)].map((match) => match[1]!);
    expect([...new Set(used)].sort()).toEqual(["abs", "floor", "max", "min"]);
    expect(source).not.toMatch(/\b(Date|performance|console|globalThis|process|Map|Set)\b/);
    expect(source).not.toMatch(/\S \*\* \S/);
  });

  it("never writes into what it is given", () => {
    const bodies = frozen([body("a", 150, 248, 198, 304), body("b", 190, 248, 238, 304), body("c", 400, 100, 440, 150)]);
    const path = frozen(marching(0, 0, 10, 0, 10));
    const claims = frozen([claimOf("c", 0, path, 3, path[9]!)]);
    const perch = frozen<Perch>({ surface: "s", x0: 100, x1: 400, y: 300 });
    const extent = frozen(box(100, 250, 140, 300));
    const point: Point = frozen({ x: 100, y: 300 });
    expect(() => {
      bodyOf("a", point, frozen({ width: 40, height: 48 }), 0, frozen(leaning(48, 1)), MARGIN);
      overlaps(bodies);
      nearMisses(bodies, 2);
      freeAt(extent, "x", bodies, claims, 3);
      slotIn(120, extent, 0, 500, obstaclesOf("x", bodies, claims, 3));
      spotOn(perch, 120, extent, 16, obstaclesOf("x", bodies, claims, 3));
      columnOver(perch, 120, extent, 40, 16, obstaclesOf("x", bodies, claims, 3));
      claimClear(claims[0]!, bodies, claims);
      claimClear(claimOf("x", 0, path, 1, null), bodies, claims);
      released(claims, "c");
      pruned(claims, 5);
      sliceAt(claims[0]!, 5);
      guardedStride(extent, 5, obstaclesOf("x", bodies, claims, 3));
      orderKept(orderOf(bodies), orderOf(bodies));
      seatOf(bodies, perch, MARGIN);
      scootFraction(frozen([1, 2]), 1);
      headUnder(extent, shifted(extent, 0, 5), "x", bodies);
      slideOf(extent, 1, 3, 0, 960, obstaclesOf("x", bodies, claims, 3));
      pushedOut(extent, obstaclesOf("x", bodies, claims, 3), PUSHES);
      mustPoof("x", extent, claims, bodies, claims, 3);
      evicted(bodies, claims, 3);
      tallied(TALLY, bodies, 2, 0, 0);
    }).not.toThrow();
  });
});

describe("the reference world", () => {
  it("keeps every pair of bodies apart at the end of every tick, and the order on every perch", () => {
    const sum = { overlaps: 0, disorders: 0, actors: 0, falls: 0, heads: 0, steered: 0, chutes: 0, hops: 0, glides: 0, grabs: 0, surveys: 0, seated: 0 };
    for (let seed = 1; seed <= WORLDS; seed++) {
      const world = runWorld(seed, WORLD_TICKS, RULES, 1);
      expect(world.tally.ticks).toBe(WORLD_TICKS);
      expect(world.tally.overlaps, `seed ${seed}`).toBe(0);
      expect(world.counts.disorders, `seed ${seed}`).toBe(0);
      expect(world.pets.length).toBeGreaterThanOrEqual(6);
      expect(world.pets.length).toBeLessThanOrEqual(10);
      for (const pet of world.pets) {
        expect(pet.size.width).toBeGreaterThanOrEqual(28);
        expect(pet.size.width).toBeLessThanOrEqual(58);
        expect(pet.size.height).toBeGreaterThanOrEqual(40);
        expect(pet.size.height).toBeLessThanOrEqual(56);
      }
      sum.overlaps += world.tally.overlaps;
      sum.actors += world.tally.actors;
      for (const key of ["falls", "heads", "steered", "chutes", "hops", "glides", "grabs", "surveys", "seated"] as const) sum[key] += world.counts[key];
    }
    expect(sum.actors).toBeGreaterThan(WORLDS * WORLD_TICKS * 5);
    for (const key of ["falls", "heads", "hops", "glides", "grabs", "surveys", "chutes"] as const) expect(sum[key], key).toBeGreaterThan(0);
  });

  it("holds with claims cut into slices of four ticks as well", () => {
    for (let seed = 1; seed <= sampled(1, 3, 6); seed++) {
      const world = runWorld(seed, WORLD_TICKS, RULES, 4);
      expect([world.tally.overlaps, world.counts.disorders], `seed ${seed}`).toEqual([0, 0]);
    }
  });

  it("is judged alike by an overlap test that measures the shared area", () => {
    const world = openWorld(3, RULES, 1);
    const bare = openWorld(3, { ...RULES, guard: false }, 1);
    let shared = 0;
    for (let tick = 0; tick < sampled(600, 3000, 30000); tick++) {
      tickWorld(world);
      tickWorld(bare);
      for (const run of [world, bare]) {
        const bodies = bodiesOf(run);
        let pairs = 0;
        for (let first = 0; first < bodies.length; first++) for (let second = first + 1; second < bodies.length; second++) if (sharing(bodies[first]!.extent, bodies[second]!.extent)) pairs++;
        expect(overlaps(bodies).length).toBe(pairs);
        if (run === bare) shared += pairs;
      }
    }
    expect(world.tally.overlaps).toBe(0);
    expect(shared).toBeGreaterThan(0);
    expect(bare.tally.overlaps).toBeGreaterThan(0);
  });

  it("is a pure function of its seed and rules", () => {
    const ticks = sampled(800, 5000, 30000);
    const one = runWorld(5, ticks, RULES, 1);
    const other = runWorld(5, ticks, RULES, 1);
    expect(other.tally).toEqual(one.tally);
    expect(other.counts).toEqual(one.counts);
    expect(bodiesOf(other)).toEqual(bodiesOf(one));
    expect(bodiesOf(runWorld(6, ticks, RULES, 1))).not.toEqual(bodiesOf(one));
  });

  it.each(BITES)("lets bodies overlap as soon as the rule $rule is switched off", ({ rule, seed, ticks }) => {
    expect(runWorld(seed, ticks, RULES, 1).tally.overlaps).toBe(0);
    expect(runWorld(seed, ticks, { ...RULES, [rule]: false }, 1).tally.overlaps).toBeGreaterThan(0);
    let overlapping = 0;
    for (let other = 1; other <= ABLATIONS; other++) overlapping += runWorld(other, ABLATION_TICKS, { ...RULES, [rule]: false }, 1).tally.overlaps;
    if (ABLATIONS > 0) expect(overlapping).toBeGreaterThan(0);
  });

  it("needs the poof where planning fails: without heads and air control pets poof instead of overlapping, and without the poof as well they overlap", () => {
    const ticks = sampled(600, 6000, 15000);
    const seeds = sampled(1, 6, 12);
    const sum = { kept: 0, poofs: 0, plain: 0, bare: 0 };
    for (let seed = 1; seed <= seeds; seed++) {
      const kept = runWorld(seed, ticks, { ...RULES, heads: false, steering: false }, 1);
      sum.kept += kept.tally.overlaps;
      sum.poofs += kept.tally.poofs;
      sum.plain += runWorld(seed, ticks, RULES, 1).tally.poofs;
      sum.bare += runWorld(seed, ticks, { ...RULES, heads: false, steering: false, poof: false }, 1).tally.overlaps;
    }
    expect(sum.kept).toBe(0);
    expect(sum.poofs).toBeGreaterThan(sum.plain);
    expect(sum.bare).toBeGreaterThan(0);
  });

  it("keeps bodies apart without the quality rules too, at the price of more poofs and falls", () => {
    const ticks = sampled(1000, 6000, 15000);
    const seeds = sampled(1, 6, 12);
    const sum = { poofs: 0, headless: 0, unsteered: 0, spilled: 0, unseated: 0 };
    for (let seed = 1; seed <= seeds; seed++) {
      const plain = runWorld(seed, ticks, RULES, 1);
      const headless = runWorld(seed, ticks, { ...RULES, heads: false }, 1);
      const unsteered = runWorld(seed, ticks, { ...RULES, steering: false }, 1);
      const unseated = runWorld(seed, ticks, { ...RULES, seating: false }, 1);
      for (const world of [plain, headless, unsteered, unseated]) expect([world.tally.overlaps, world.counts.disorders], `seed ${seed}`).toEqual([0, 0]);
      expect(headless.counts.heads).toBe(0);
      expect(unsteered.counts.steered).toBe(0);
      expect(unseated.counts.seated).toBe(0);
      sum.poofs += plain.tally.poofs;
      sum.headless += headless.tally.poofs;
      sum.unsteered += unsteered.tally.poofs;
      sum.spilled += plain.counts.spilled;
      sum.unseated += unseated.counts.spilled;
    }
    if (seeds > 1) {
      expect(sum.headless).toBeGreaterThan(sum.poofs);
      expect(sum.unsteered).toBeGreaterThan(sum.poofs);
      expect(sum.unseated).toBeGreaterThan(sum.spilled);
    }
  });
});
