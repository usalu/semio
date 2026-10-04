/** 🚧️ Clearance: the machinery that keeps the bodies of pets apart, so that at the end of every tick no two of them overlap.
 *
 * A **body** is the solid box of an actor ({@link bodyOf}): its size box, the hover beneath a floater, what its
 * posture adds (the lean of a held pet, an open canopy) and a margin on every side. The hard invariant is
 * `overlaps(bodies) = []` ({@link overlaps}): two bodies overlap when their open boxes intersect, so bodies that
 * touch are apart. Everything that *places* a body ({@link slotIn}, {@link guardedStride}, {@link seatOf},
 * {@link pushedOut}, {@link liftOnto}) leaves a {@link SEAM} of 1/64 px beside its neighbour; the predicates
 * ({@link meets}, {@link freeAt}, {@link claimClear}) are exact. The seam is what makes the invariant robust: the
 * rounding of a sum can never turn two placed bodies into overlapping ones.
 *
 * A **claim** is the swept corridor of a planned motion ({@link claimOf}): the box of the body for every range of
 * ticks of the plan, and the place where it rests afterwards. Whoever flies, falls, glides, climbs or reels plans
 * first, has the plan vetted ({@link claimClear}) and then follows it; everybody else respects the claim.
 *
 * The rules, and why they make the invariant hold (the stage updates its actors one after another, each one
 * against the present places of all others):
 *
 * 1. *Guarded motion.* Whoever moves without a plan (walks, scoots, slides off a head, follows the hand) commits
 *    a place only when it is free against every other body and against every claim from now on
 *    ({@link obstaclesOf}: the corridors that are left and the places where plans come to rest).
 * 2. *Vetted plans.* A plan is accepted only when {@link claimClear} holds: tick by tick its box meets nobody —
 *    a body that has no plan stays where it is, a body with a plan is where its own claim says — and the place
 *    where it comes to rest stays free of everything that arrives later.
 * 3. *Guarded transitions.* Arriving, landing and stepping onto a head happen at free places only
 *    ({@link spotOn}, {@link columnOver}, {@link headUnder}).
 * 4. *Outside changes use the same machinery.* A survey never teleports anybody: a perch group is re-seated by
 *    {@link seatOf} and scoots there by one common fraction ({@link scootFraction}); whoever a moving surface
 *    carried into somebody else is named by {@link evicted}.
 * 5. *Last resort.* When no legal continuation is left ({@link mustPoof}) the pet vanishes at once and arrives
 *    anew at a free place; a vanished pet is not a body. There is no translucent overlap.
 *
 * Coordinates are viewport pixels with the y axis pointing down, time is whole ticks of 1/64 s. Only `+ − × ÷`,
 * `abs`, `floor`, `min`, `max` and comparisons are used, in the order written here, lists are walked in their
 * given order and every sort is a stable insertion, so the Rust twin reproduces every result bit for bit.
 *
 * @see ../🏞️terrain/🟦️.ts — perches, falls and hops whose paths are claimed here
 * @see ../../🧬️schema/🟦️.ts — `Point`, `Size`, `Perch`, `Slug`, `Ticks`
 * @see https://cdn.aaai.org/ojs/18726/18726-52-22369-1-10-20210928.pdf — D. Silver, Cooperative Pathfinding: the reservation table in space and time
 * @see https://en.wikipedia.org/wiki/Isotonic_regression — pool-adjacent-violators, the seating of {@link seatOf}
 */

import { type Claim, type Extent, type Perch, type Point, type Size, type Slice, type Slug, TICKS_PER_SECOND, type Ticks } from "../../🧬️schema/🟦️.ts";

export type { Claim, Extent, Slice } from "../../🧬️schema/🟦️.ts";

//#region 🔖️Constants
/** 🫧️ The margin of a body on every side in pixels: two bodies that touch keep the comfortable gap of 8 px between their size boxes. */
export const MARGIN = 4;

/** 🧵️ The hair every placement leaves beside a neighbour, in pixels (1/64, exact in binary): far above the rounding of any sum of coordinates, far below anything visible. */
export const SEAM = 0.015625;

/** ♾️ The last tick there is (2³² − 1): the end of a body that stays, in the time line of {@link claimClear}. */
export const FOREVER = 4294967295;

/** 🧭️ The air control a fall may try, in pixels per second added to its horizontal speed, in the order tried: straight first, then the nearest ones. */
export const STEERINGS: readonly number[] = [0, 30, -30, 60, -60, 90, -90, 130, -130, 180, -180, 240, -240];

/** 🤲️ How many rounds {@link pushedOut} gets to free a held body. */
export const PUSHES = 4;

/** ⏳️ How many ticks a blocked walker waits before it gives its goal up for a new one. */
export const PATIENCE = 40;

/** ⏱️ How many ticks a plan may be put off (the pet stays where it is, as long as that place is free) before {@link mustPoof} takes the stay away. */
export const DELAY = 32;

/** 🛝️ The speed at which a pet starts to slide off a head, in pixels per second. */
export const SLIDE_OFF_SPEED = 32;

/** 📈️ What a slide gains per tick, in pixels per second. */
export const SLIDE_OFF_GAIN = 2;

/** 🚀️ The fastest slide, in pixels per second. */
export const SLIDE_OFF_LIMIT = 128;

/** 🙇️ How far a body that hangs at full tilt reaches out on either side, as a share of its height. */
export const LEAN = 0.25;

/** 🛫️ How far above its own altitude the high lane of a floater lies, as a share of its height plus the margin. */
export const LANE_LIFT = 0.9;

/** 🏃️ How much faster than it walks a pet scoots to its seat. */
export const SCOOT_HASTE = 1.5;
//#endregion 🔖️Constants

//#region 🔖️Types
/** 🐾️ The solid box of one actor, named by the slug of its species. */
export type Body = { readonly owner: Slug; readonly extent: Extent };

/** 🧍️ What a posture adds to the size box of a species, in pixels: to the left, to the right and above. */
export type Posture = { readonly left: number; readonly right: number; readonly above: number };

/** 👥️ Two owners whose bodies are too close, in the order of the list they were found in. */
export type Pair = { readonly first: Slug; readonly second: Slug };

/** 🪑️ How far a seated body moves along its perch, in pixels (negative: to the left). */
export type Seat = { readonly owner: Slug; readonly shift: number };

/** 🛋️ A seating of a perch: the seats from left to right, and who has to leave because the perch cannot hold them, the lowest priority first. */
export type Seating = { readonly seats: readonly Seat[]; readonly leavers: readonly Slug[] };

/** ⛸️ One tick of sliding off a head: how far the rider moves, and the side (1 right, −1 left) it slides to from now on. */
export type Slide = { readonly stride: number; readonly side: number };

/** 🧮️ What a run of the invariant has counted: its ticks, its actor-ticks (bodies summed over the ticks), the ticks with an overlap (must stay 0), the ticks with a near miss, the poofs and the ticks somebody waited. */
export type Tally = { readonly ticks: number; readonly actors: number; readonly overlaps: number; readonly nears: number; readonly poofs: number; readonly waits: number };
//#endregion 🔖️Types

//#region 🔖️Bodies
/** 🧘️ The posture that adds nothing: standing, walking, sitting. */
export const UPRIGHT: Posture = { left: 0, right: 0, above: 0 };

/** 🤸️ The posture of a body that hangs or tumbles tilted: `LEAN × height × |sine|` on either side, `sine` being the sine of its tilt (1 for a held pet, which may swing all the way). */
export function leaning(height: number, sine: number): Posture {
  const reach = LEAN * height * Math.abs(sine);
  return { left: reach, right: reach, above: 0 };
}

/** 🪂️ The posture of a body under an open canopy: the canopy sits centred on top of the size box, so it adds its height above and half of what it is wider on either side. */
export function canopied(size: Size, canopy: Size): Posture {
  const reach = Math.max((canopy.width - size.width) / 2, 0);
  return { left: reach, right: reach, above: Math.max(canopy.height, 0) };
}

/** 📐️ The body of an actor whose feet are at `feet`: the size box centred above the feet, the `hover` beneath them down to the perch, what the posture adds, and `margin` on every side. */
export function bodyOf(owner: Slug, feet: Point, size: Size, hover: number, posture: Posture, margin: number): Body {
  const half = size.width / 2;
  return { owner, extent: { x0: feet.x - half - posture.left - margin, y0: feet.y - size.height - posture.above - margin, x1: feet.x + half + posture.right + margin, y1: feet.y + hover + margin } };
}

/** ➡️ A box moved by `dx` and `dy`. */
export function shifted(extent: Extent, dx: number, dy: number): Extent {
  return { x0: extent.x0 + dx, y0: extent.y0 + dy, x1: extent.x1 + dx, y1: extent.y1 + dy };
}

/** 🎈️ A box grown by `margin` on every side. */
export function grown(extent: Extent, margin: number): Extent {
  return { x0: extent.x0 - margin, y0: extent.y0 - margin, x1: extent.x1 + margin, y1: extent.y1 + margin };
}

/** 🔗️ The smallest box that holds both boxes. */
export function united(one: Extent, other: Extent): Extent {
  return { x0: Math.min(one.x0, other.x0), y0: Math.min(one.y0, other.y0), x1: Math.max(one.x1, other.x1), y1: Math.max(one.y1, other.y1) };
}

/** 💥️ Whether two boxes overlap: their open interiors intersect, so boxes that only touch do not. */
export function meets(one: Extent, other: Extent): boolean {
  return one.x0 < other.x1 && other.x0 < one.x1 && one.y0 < other.y1 && other.y0 < one.y1;
}

/** 🚨️ The executable invariant: every pair of bodies that overlap, by ascending position of the first and then of the second in the list. It must be empty at the end of every tick. */
export function overlaps(bodies: readonly Body[]): Pair[] {
  const pairs: Pair[] = [];
  for (let first = 0; first < bodies.length; first++) {
    for (let second = first + 1; second < bodies.length; second++) {
      if (meets(bodies[first]!.extent, bodies[second]!.extent)) pairs.push({ first: bodies[first]!.owner, second: bodies[second]!.owner });
    }
  }
  return pairs;
}

/** 😬️ Every pair of bodies that do not overlap but are closer than `margin` to each other, in the order of {@link overlaps}. */
export function nearMisses(bodies: readonly Body[], margin: number): Pair[] {
  const pairs: Pair[] = [];
  for (let first = 0; first < bodies.length; first++) {
    for (let second = first + 1; second < bodies.length; second++) {
      const one = bodies[first]!.extent;
      const other = bodies[second]!.extent;
      if (!meets(one, other) && meets(grown(one, margin), other)) pairs.push({ first: bodies[first]!.owner, second: bodies[second]!.owner });
    }
  }
  return pairs;
}
//#endregion 🔖️Bodies

//#region 🔖️Free places
/** 🧱️ Everything `owner` has to stay clear of from `tick` on: the bodies of all others in list order, then for every claim of another owner in list order its slices that are not over yet (`until ≥ tick`) and the place where it comes to rest. */
export function obstaclesOf(owner: Slug, bodies: readonly Body[], claims: readonly Claim[], tick: Ticks): Extent[] {
  const obstacles: Extent[] = [];
  for (const body of bodies) if (body.owner !== owner) obstacles.push(body.extent);
  for (const claim of claims) {
    if (claim.owner === owner) continue;
    for (const slice of claim.slices) if (slice.until >= tick) obstacles.push(slice.extent);
    if (claim.rest !== null) obstacles.push(claim.rest);
  }
  return obstacles;
}

/** 🆓️ Whether a box overlaps none of the obstacles. */
export function freeAmong(extent: Extent, obstacles: readonly Extent[]): boolean {
  for (const obstacle of obstacles) if (meets(extent, obstacle)) return false;
  return true;
}

/** ✅️ Whether `owner` may be at `extent` from `tick` on: the box overlaps no other body, no corridor that is left of another owner's claim and no place where such a claim comes to rest. */
export function freeAt(extent: Extent, owner: Slug, bodies: readonly Body[], claims: readonly Claim[], tick: Ticks): boolean {
  return freeAmong(extent, obstaclesOf(owner, bodies, claims, tick));
}

/** ✂️ `stretches` (pairs of ends, ascending) without the open interval `(low, high)`; an end that is only touched stays, as a single point if need be. */
function carved(stretches: readonly number[], low: number, high: number): number[] {
  const kept: number[] = [];
  for (let index = 0; index < stretches.length; index += 2) {
    const x0 = stretches[index]!;
    const x1 = stretches[index + 1]!;
    if (high <= x0 || low >= x1) kept.push(x0, x1);
    else {
      if (low >= x0) kept.push(x0, low);
      if (high <= x1) kept.push(high, x1);
    }
  }
  return kept;
}

/** 🎯️ The place nearest to `x` between `low` and `high` (ends included) at which a body can be: `extent` is its box when its feet are at `x`, and at the answered feet position the box, moved along, keeps a {@link SEAM} from every obstacle whose height it shares. Among two places equally near the left one; `null` when there is none. */
export function slotIn(x: number, extent: Extent, low: number, high: number, obstacles: readonly Extent[]): number | null {
  if (!(low <= high)) return null;
  let stretches: number[] = [low, high];
  for (const obstacle of obstacles) {
    if (!(extent.y0 < obstacle.y1 && obstacle.y0 < extent.y1)) continue;
    stretches = carved(stretches, x + (obstacle.x0 - extent.x1 - SEAM), x + (obstacle.x1 - extent.x0 + SEAM));
  }
  let nearest: number | null = null;
  let least = Infinity;
  for (let index = 0; index < stretches.length; index += 2) {
    const place = Math.min(Math.max(x, stretches[index]!), stretches[index + 1]!);
    const distance = Math.abs(place - x);
    if (distance < least) {
      nearest = place;
      least = distance;
    }
  }
  return nearest;
}

/** 📍️ The free spot on a perch nearest to `x` for a body whose box is `extent` when its feet are at `x`: its footprint (`foot` px to either side of the feet) stays on the perch. `null` when the perch has no room. */
export function spotOn(perch: Perch, x: number, extent: Extent, foot: number, obstacles: readonly Extent[]): number | null {
  return slotIn(x, extent, perch.x0 + foot, perch.x1 - foot, obstacles);
}

/** 🏛️ The free column over a perch nearest to `x` for a landing: the place at which the body can come straight down by `drop` px (from `extent`, its box with the feet at `x`, to the perch) without touching anybody on the way, its footprint on the perch. `null` when there is none. */
export function columnOver(perch: Perch, x: number, extent: Extent, drop: number, foot: number, obstacles: readonly Extent[]): number | null {
  return slotIn(x, { x0: extent.x0, y0: extent.y0, x1: extent.x1, y1: extent.y1 + Math.max(drop, 0) }, perch.x0 + foot, perch.x1 - foot, obstacles);
}
//#endregion 🔖️Free places

//#region 🔖️Claims
/** 🛤️ The claim of a planned motion: `extents[k]` is the box of the body at the end of tick `from + k`, a slice holds `span` ticks (at least 1) and is the smallest box around their boxes, and `rest` is where the owner stays afterwards. With `span = 1` the claim is the path itself; a wider span costs fewer boxes and claims a little more room. */
export function claimOf(owner: Slug, from: Ticks, extents: readonly Extent[], span: Ticks, rest: Extent | null): Claim {
  const width = Math.max(Math.floor(span), 1);
  const slices: Slice[] = [];
  for (let start = 0; start < extents.length; start += width) {
    const end = Math.min(start + width, extents.length);
    let hull = extents[start]!;
    for (let at = start + 1; at < end; at++) hull = united(hull, extents[at]!);
    slices.push({ from: from + start, until: from + end - 1, extent: hull });
  }
  return { owner, slices, rest };
}

/** 🔎️ Where a claim has its owner at `tick`: the slice that holds the tick, the rest after the last slice, and `null` before the first slice (the owner is still where its body is) or after a plan that ends off stage. */
export function sliceAt(claim: Claim, tick: Ticks): Extent | null {
  if (claim.slices.length === 0 || tick < claim.slices[0]!.from) return null;
  for (const slice of claim.slices) if (tick <= slice.until) return slice.extent;
  return claim.rest;
}

/** 🕰️ The time line of one owner as slices without a gap: its body until its claim begins (for ever without a claim), the slices of the claim, then its rest for ever. */
function spansOf(body: Extent | null, claim: Claim | null): Slice[] {
  const spans: Slice[] = [];
  if (claim === null || claim.slices.length === 0) {
    if (body !== null) spans.push({ from: 0, until: FOREVER, extent: body });
    return spans;
  }
  const first = claim.slices[0]!.from;
  const last = claim.slices[claim.slices.length - 1]!.until;
  if (body !== null && first > 0) spans.push({ from: 0, until: first - 1, extent: body });
  for (const slice of claim.slices) spans.push(slice);
  if (claim.rest !== null && last < FOREVER) spans.push({ from: last + 1, until: FOREVER, extent: claim.rest });
  return spans;
}

/** ⚔️ Whether two time lines hold boxes that overlap at one and the same tick. */
function collide(mine: readonly Slice[], theirs: readonly Slice[]): boolean {
  for (const one of mine) {
    for (const other of theirs) {
      if (one.from <= other.until && other.from <= one.until && meets(one.extent, other.extent)) return true;
    }
  }
  return false;
}

/** 🚦️ Whether a plan may be taken: from its first tick on, the box of its slice — and after its last slice its rest, for ever — overlaps nobody at the same tick. Another owner is where its body is until its own claim begins (for ever when it has none), then inside the slices of that claim, then at the rest of that claim; so a plan may cross a corridor before or after its owner passes, but may never come to rest where somebody arrives later. One claim per owner is expected (the first one in the list is the time line of its body); the owner's own body and claims are ignored. */
export function claimClear(claim: Claim, bodies: readonly Body[], claims: readonly Claim[]): boolean {
  const mine = spansOf(null, claim);
  for (const body of bodies) {
    if (body.owner === claim.owner) continue;
    let theirs: Claim | null = null;
    for (const other of claims) {
      if (other.owner === body.owner) {
        theirs = other;
        break;
      }
    }
    if (collide(mine, spansOf(body.extent, theirs))) return false;
  }
  for (let index = 0; index < claims.length; index++) {
    const other = claims[index]!;
    if (other.owner === claim.owner) continue;
    let leading = true;
    for (let before = 0; before < index; before++) if (claims[before]!.owner === other.owner) leading = false;
    let embodied = false;
    for (const body of bodies) if (body.owner === other.owner) embodied = true;
    if (leading && embodied) continue;
    if (collide(mine, spansOf(null, other))) return false;
  }
  return true;
}

/** 🔓️ The claims without those of `owner`: its plan has ended, was given up, or its owner vanished. */
export function released(claims: readonly Claim[], owner: Slug): Claim[] {
  const kept: Claim[] = [];
  for (const claim of claims) if (claim.owner !== owner) kept.push(claim);
  return kept;
}

/** 🧹️ The claims as they stand at `tick`: slices that are over (`until < tick`) are dropped, and with them every claim that has no slice left — its owner has arrived and its body now holds the place. */
export function pruned(claims: readonly Claim[], tick: Ticks): Claim[] {
  const kept: Claim[] = [];
  for (const claim of claims) {
    const slices: Slice[] = [];
    for (const slice of claim.slices) if (slice.until >= tick) slices.push(slice);
    if (slices.length > 0) kept.push({ owner: claim.owner, slices, rest: claim.rest });
  }
  return kept;
}
//#endregion 🔖️Claims

//#region 🔖️Order on a perch
/** 👣️ The farthest a body may move sideways this tick: `stride` (negative: to the left) cut short so that the box stops a {@link SEAM} before the first obstacle that shares its height and lies ahead (its middle beyond the middle of the box). An obstacle the box already overlaps therefore holds it back only on the way further in, never on the way out. Because nobody passes anybody whose height it shares, the order along a perch never changes by walking. */
export function guardedStride(extent: Extent, stride: number, obstacles: readonly Extent[]): number {
  const middle = extent.x0 + extent.x1;
  if (stride > 0) {
    let reach = stride;
    for (const obstacle of obstacles) {
      if (!(extent.y0 < obstacle.y1 && obstacle.y0 < extent.y1)) continue;
      if (!(obstacle.x0 + obstacle.x1 > middle)) continue;
      reach = Math.min(reach, Math.max(obstacle.x0 - extent.x1 - SEAM, 0));
    }
    return reach;
  }
  if (stride < 0) {
    let reach = 0 - stride;
    for (const obstacle of obstacles) {
      if (!(extent.y0 < obstacle.y1 && obstacle.y0 < extent.y1)) continue;
      if (!(obstacle.x0 + obstacle.x1 < middle)) continue;
      reach = Math.min(reach, Math.max(extent.x0 - obstacle.x1 - SEAM, 0));
    }
    return 0 - reach;
  }
  return 0;
}

/** 🦘️ The first condition of a swap by a hop: a hop that rises `rise` px from where `hopper` stands lifts its box over `hurdle`, a seam to spare. The swap itself is a plan like any other: the hop is taken only when {@link claimClear} holds for its claim, which also keeps the landing beyond the neighbour free; otherwise the lower priority turns round or waits ({@link PATIENCE}). */
export function vaults(hopper: Extent, rise: number, hurdle: Extent): boolean {
  return hopper.y1 - rise <= hurdle.y0 - SEAM;
}

/** 🔢️ The owners of the bodies from left to right by the middle of their boxes; equals keep their order in the list. */
export function orderOf(bodies: readonly Body[]): Slug[] {
  const ranked: number[] = [];
  for (let index = 0; index < bodies.length; index++) {
    const middle = bodies[index]!.extent.x0 + bodies[index]!.extent.x1;
    let at = ranked.length;
    while (at > 0 && bodies[ranked[at - 1]!]!.extent.x0 + bodies[ranked[at - 1]!]!.extent.x1 > middle) at--;
    ranked.splice(at, 0, index);
  }
  const owners: Slug[] = [];
  for (const index of ranked) owners.push(bodies[index]!.owner);
  return owners;
}

/** 🧷️ Whether an order was kept: the owners that are in both lists follow each other in `after` as they did in `before`. */
export function orderKept(before: readonly Slug[], after: readonly Slug[]): boolean {
  let last = -1;
  for (const owner of before) {
    let at = -1;
    for (let index = 0; index < after.length; index++) {
      if (after[index] === owner) {
        at = index;
        break;
      }
    }
    if (at < 0) continue;
    if (at < last) return false;
    last = at;
  }
  return true;
}
//#endregion 🔖️Order on a perch

//#region 🔖️Seating
/** 💺️ The seating of a perch after a survey: every body moves as little as possible (least squares) while the order from left to right is kept, neighbours keep a {@link SEAM} between their boxes and every box without its `margin` lies on the perch.
 *
 * `bodies` are the bodies that stand on the perch, the highest priority first. While they do not fit side by side
 * the last one in the list leaves. For the others, sorted by the middle of their boxes, let `o₁ = 0` and
 * `oᵢ₊₁ = oᵢ + widthᵢ + SEAM`; with `qᵢ = x0ᵢ − oᵢ` "apart and in order" reads "`q` never decreases" and "on the
 * perch" reads `perch.x0 − margin ≤ q ≤ perch.x1 + margin − widthₙ − oₙ`. The nearest such `q` is the isotonic
 * regression of `q` (pool adjacent violators: a block is the mean of its members, two neighbouring blocks merge
 * while the left mean is larger than the right one), clamped into the bounds. A seating that is valid already is
 * answered with shifts of exactly 0.
 */
export function seatOf(bodies: readonly Body[], perch: Perch, margin: number): Seating {
  const members: number[] = [];
  for (let index = 0; index < bodies.length; index++) {
    const middle = bodies[index]!.extent.x0 + bodies[index]!.extent.x1;
    let at = members.length;
    while (at > 0 && bodies[members[at - 1]!]!.extent.x0 + bodies[members[at - 1]!]!.extent.x1 > middle) at--;
    members.splice(at, 0, index);
  }
  const leavers: Slug[] = [];
  const low = perch.x0 - margin;
  let offsets: number[] = [];
  let high = low;
  while (members.length > 0) {
    offsets = [0];
    for (let at = 1; at < members.length; at++) {
      const before = bodies[members[at - 1]!]!.extent;
      offsets.push(offsets[at - 1]! + (before.x1 - before.x0) + SEAM);
    }
    const last = bodies[members[members.length - 1]!]!.extent;
    high = perch.x1 + margin - (last.x1 - last.x0) - offsets[members.length - 1]!;
    if (low <= high) break;
    let worst = 0;
    for (let at = 1; at < members.length; at++) if (members[at]! > members[worst]!) worst = at;
    leavers.push(bodies[members[worst]!]!.owner);
    members.splice(worst, 1);
  }
  const sums: number[] = [];
  const counts: number[] = [];
  for (let at = 0; at < members.length; at++) {
    sums.push(bodies[members[at]!]!.extent.x0 - offsets[at]!);
    counts.push(1);
    while (sums.length > 1 && sums[sums.length - 2]! / counts[counts.length - 2]! > sums[sums.length - 1]! / counts[counts.length - 1]!) {
      const sum = sums.pop()!;
      const count = counts.pop()!;
      sums[sums.length - 1] = sums[sums.length - 1]! + sum;
      counts[counts.length - 1] = counts[counts.length - 1]! + count;
    }
  }
  const seats: Seat[] = [];
  let at = 0;
  for (let block = 0; block < sums.length; block++) {
    const mean = Math.min(Math.max(sums[block]! / counts[block]!, low), high);
    for (let member = 0; member < counts[block]!; member++) {
      seats.push({ owner: bodies[members[at]!]!.owner, shift: mean - (bodies[members[at]!]!.extent.x0 - offsets[at]!) });
      at++;
    }
  }
  return { seats, leavers };
}

/** 🎚️ The share of its way every member of a group covers this tick when the one with the longest way moves `stride` px: `stride ÷ max |shift|`, at most 1 (and 1 when nobody has a way left). Because being apart, in order and on the perch are linear conditions, every mixture of two valid seatings is valid: a group that moves by one common share never overlaps within itself. */
export function scootFraction(shifts: readonly number[], stride: number): number {
  let farthest = 0;
  for (const shift of shifts) farthest = Math.max(farthest, Math.abs(shift));
  if (!(farthest > 0)) return 1;
  return farthest > stride ? Math.max(stride, 0) / farthest : 1;
}

/** 🚶️ Where a scoot from `from` towards `to` stands after covering `fraction` of the way: `to` itself once the fraction reaches 1. */
export function scooted(from: number, to: number, fraction: number): number {
  return fraction >= 1 ? to : from + (to - from) * fraction;
}
//#endregion 🔖️Seating

//#region 🔖️Heads
/** 🎩️ The body on whose top a descending body lands, like a perch crossing: between `before` and `after` (the box of the faller at the end of the last tick and of this one) its lower edge comes down onto the upper edge of a body whose width it shares at `after`. The highest such body, the first one among equals; `null` when it lands on nobody — never while rising, and never on a body it was beside already. */
export function headUnder(before: Extent, after: Extent, owner: Slug, bodies: readonly Body[]): Body | null {
  let host: Body | null = null;
  for (const body of bodies) {
    if (body.owner === owner) continue;
    const top = body.extent.y0;
    if (!(before.y1 <= top && after.y1 > top - SEAM && after.y1 >= before.y1)) continue;
    if (!(after.x0 < body.extent.x1 && body.extent.x0 < after.x1)) continue;
    if (host === null || top < host.extent.y0) host = body;
  }
  return host;
}

/** 🛗️ How far a box has to move down (negative: up) to stand on a head: its lower edge a {@link SEAM} above the upper edge of `host`. */
export function liftOnto(extent: Extent, host: Extent): number {
  return host.y0 - SEAM - extent.y1;
}

/** 🪨️ Whether `rider` still stands on `host`: it shares its width and its lower edge lies within two seams above the upper edge of the host. Checked every tick; a rider whose host walked off, fell or vanished falls again. */
export function restsOn(rider: Extent, host: Extent): boolean {
  return rider.x0 < host.x1 && host.x0 < rider.x1 && rider.y1 <= host.y0 && host.y0 - rider.y1 <= 2 * SEAM;
}

/** ↔️ The side a rider slides off to: 1 (right) when the middle of its box is at or beyond the middle of the host, else −1 (left). */
export function slideSide(rider: Extent, host: Extent): number {
  return rider.x0 + rider.x1 >= host.x0 + host.x1 ? 1 : -1;
}

/** 🎿️ How far a slide moves in its tick number `ticks` (0 for the first), in pixels: it starts at {@link SLIDE_OFF_SPEED}, gains {@link SLIDE_OFF_GAIN} per tick and never exceeds {@link SLIDE_OFF_LIMIT}. */
export function slideStride(ticks: Ticks): number {
  return Math.min(SLIDE_OFF_SPEED + SLIDE_OFF_GAIN * ticks, SLIDE_OFF_LIMIT) / TICKS_PER_SECOND;
}

/** 🛷️ One tick of sliding off a head towards `side`: the stride of {@link slideStride}, kept between the edges `low` and `high` of the stage and guarded against the obstacles. A rider that cannot move at all turns round and tries the other side from the next tick on. Once it no longer {@link restsOn} its host it falls. */
export function slideOf(rider: Extent, side: number, ticks: Ticks, low: number, high: number, obstacles: readonly Extent[]): Slide {
  const wanted = slideStride(ticks);
  const bounded = side > 0 ? Math.min(wanted, Math.max(high - rider.x1, 0)) : 0 - Math.min(wanted, Math.max(rider.x0 - low, 0));
  const stride = guardedStride(rider, bounded, obstacles);
  return stride === 0 ? { stride: 0, side: 0 - side } : { stride, side };
}
//#endregion 🔖️Heads

//#region 🔖️Held
/** 🫸️ How a held body gets out of the obstacles it was dragged into: the sum of the shortest ways out (left, right, up or down, the first one among equals, a {@link SEAM} to spare), taken obstacle by obstacle in list order for at most `iterations` rounds. `null` when it still overlaps something afterwards: then the body keeps the place it had. Without an overlap the answer is no move at all. */
export function pushedOut(extent: Extent, obstacles: readonly Extent[], iterations: number): Point | null {
  let dx = 0;
  let dy = 0;
  for (let round = 0; round < iterations; round++) {
    let clean = true;
    for (const obstacle of obstacles) {
      const x0 = extent.x0 + dx;
      const y0 = extent.y0 + dy;
      const x1 = extent.x1 + dx;
      const y1 = extent.y1 + dy;
      if (!(x0 < obstacle.x1 && obstacle.x0 < x1 && y0 < obstacle.y1 && obstacle.y0 < y1)) continue;
      clean = false;
      const left = x1 - obstacle.x0 + SEAM;
      const right = obstacle.x1 - x0 + SEAM;
      const up = y1 - obstacle.y0 + SEAM;
      const down = obstacle.y1 - y0 + SEAM;
      const least = Math.min(left, right, up, down);
      if (least === left) dx = dx - left;
      else if (least === right) dx = dx + right;
      else if (least === up) dy = dy - up;
      else dy = dy + down;
    }
    if (clean) return { x: dx, y: dy };
  }
  return freeAmong(shifted(extent, dx, dy), obstacles) ? { x: dx, y: dy } : null;
}
//#endregion 🔖️Held

//#region 🔖️Last resort
/** 💨️ Whether a pet has no legal continuation and must poof (vanish at once, arrive anew at a free place): it may not stay — `stay` is the box it would keep, `null` when staying is no option (its patience of {@link DELAY} ticks is used up, or nothing holds it there) — and none of its `plans` is clear. */
export function mustPoof(owner: Slug, stay: Extent | null, plans: readonly Claim[], bodies: readonly Body[], claims: readonly Claim[], tick: Ticks): boolean {
  if (stay !== null && freeAt(stay, owner, bodies, claims, tick)) return false;
  for (const plan of plans) if (claimClear(plan, bodies, claims)) return false;
  return true;
}

/** 🚪️ Who has to poof after a change from outside (a surface that moved its riders), so that the others are apart again: `bodies` in the order of their right to stay — whoever did not move first. A body leaves when it overlaps a body before it that stays, or, having no plan of its own, a corridor that is left or a rest of a claim whose owner stays. The claims of those who leave count no longer. */
export function evicted(bodies: readonly Body[], claims: readonly Claim[], tick: Ticks): Slug[] {
  const gone: Slug[] = [];
  for (let index = 0; index < bodies.length; index++) {
    const body = bodies[index]!;
    let blocked = false;
    for (let before = 0; before < index && !blocked; before++) {
      const other = bodies[before]!;
      if (gone.indexOf(other.owner) < 0 && meets(body.extent, other.extent)) blocked = true;
    }
    let planned = false;
    for (const claim of claims) if (claim.owner === body.owner) planned = true;
    for (let at = 0; at < claims.length && !blocked && !planned; at++) {
      const claim = claims[at]!;
      if (gone.indexOf(claim.owner) >= 0) continue;
      for (const slice of claim.slices) if (slice.until >= tick && meets(body.extent, slice.extent)) blocked = true;
      if (claim.rest !== null && meets(body.extent, claim.rest)) blocked = true;
    }
    if (blocked) gone.push(body.owner);
  }
  return gone;
}
//#endregion 🔖️Last resort

//#region 🔖️Lanes
/** 🛩️ How far above its own altitude a floater of this height glides in its high lane, which it tries when the claim of its glide at its own altitude is not clear: {@link LANE_LIFT} × (height + margin). */
export function laneLift(height: number, margin: number): number {
  return LANE_LIFT * (height + margin);
}
//#endregion 🔖️Lanes

//#region 🔖️Metrics
/** 🥚️ The tally of a run that has not begun. */
export const TALLY: Tally = { ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 };

/** 📊️ The tally after one more tick: the bodies as they are at its end, counted once for {@link overlaps} and once for {@link nearMisses} within `margin`, plus the poofs and the waiting actors of the tick. */
export function tallied(tally: Tally, bodies: readonly Body[], margin: number, poofs: number, waits: number): Tally {
  return {
    ticks: tally.ticks + 1,
    actors: tally.actors + bodies.length,
    overlaps: tally.overlaps + (overlaps(bodies).length > 0 ? 1 : 0),
    nears: tally.nears + (nearMisses(bodies, margin).length > 0 ? 1 : 0),
    poofs: tally.poofs + poofs,
    waits: tally.waits + waits,
  };
}

/** 💯️ A count per million actor-ticks; 0 for a run without any. */
export function perMillion(count: number, actors: number): number {
  return actors > 0 ? (count * 1000000) / actors : 0;
}
//#endregion 🔖️Metrics
