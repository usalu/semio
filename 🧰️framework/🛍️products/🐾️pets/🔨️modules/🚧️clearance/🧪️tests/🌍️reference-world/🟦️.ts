/** 📘️ The reference world of the clearance unit suite: a pure, seeded fold in which the rules of the module, and nothing else, keep six to ten pets apart while they walk, hop, glide, fall, land on heads, are dragged and thrown by a hand and lose, shrink and move their perches.
 *
 * The world is exported: `clearance_world.ts` of ticket 2026/10/02/QUIZ-PETS runs it for the published metrics and ablations.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../🏞️terrain/🟦️.ts — the perches, falls and hops the world moves by
 * @see ../../../🎲️randomness/🟦️.ts — the draws of the world
 */
import type { Perch, Point, Size } from "../../../../🧬️schema/🟦️.ts";
import { randomWords, unitOf } from "../../../🎲️randomness/🟦️.ts";
import { fallStep, hopLanding, hopOf, hopStep, landingOf, strideTo } from "../../../🏞️terrain/🟦️.ts";
import { type Body, type Claim, DELAY, type Extent, MARGIN, PATIENCE, PUSHES, type Posture, SCOOT_HASTE, STEERINGS, TALLY, type Tally, UPRIGHT, bodyOf, canopied, claimClear, claimOf, columnOver, evicted, freeAmong, guardedStride, headUnder, laneLift, leaning, liftOnto, mustPoof, nearMisses, obstaclesOf, orderKept, orderOf, pruned, pushedOut, released, restsOn, scootFraction, scooted, seatOf, slideOf, slideSide, spotOn, tallied } from "../../🟦️.ts";

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
