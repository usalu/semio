/** 🚧️ Subject adapter of the clearance-proof case: the constants and every function of `🔨️modules/🚧️clearance` of `@semio-tech/pets` answer every committed vector.
 *
 * Boxes are committed as rows `[x0, y0, x1, y1]` and claims as what they are made of (`owner`, `from`, `span`, the
 * path as one box per tick, `rest`); the adapter builds the claims with `claimOf` and answers boxes as rows again.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🚧️clearance/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
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
  UPRIGHT,
  bodyOf,
  canopied,
  claimClear,
  claimOf,
  columnOver,
  evicted,
  freeAt,
  guardedStride,
  headUnder,
  leaning,
  liftOnto,
  mustPoof,
  nearMisses,
  obstaclesOf,
  orderKept,
  orderOf,
  overlaps,
  pruned,
  pushedOut,
  released,
  restsOn,
  scootFraction,
  scooted,
  seatOf,
  sliceAt,
  slideOf,
  slideSide,
  slideStride,
  slotIn,
  spotOn,
  vaults,
} from "../../🔨️modules/🚧️clearance/🟦️.ts";
import type { Perch, Point, Size } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🚧️clearance-proof/🔣️.json";

type Row = readonly [number, number, number, number];
type Owned = { readonly owner: string; readonly box: Row };
type Planned = { readonly owner: string; readonly from: number; readonly span: number; readonly path: readonly Row[]; readonly rest: Row | null };
type Stance = { readonly kind: "upright" } | { readonly kind: "leaning"; readonly height: number; readonly sine: number } | { readonly kind: "canopied"; readonly canopy: Size } | { readonly kind: "plain"; readonly left: number; readonly right: number; readonly above: number };
type Place = { readonly id: string; readonly x: number; readonly box: Row; readonly obstacles: readonly Row[] } & ({ readonly kind: "slot"; readonly low: number; readonly high: number } | { readonly kind: "spot"; readonly perch: Perch; readonly foot: number } | { readonly kind: "column"; readonly perch: Perch; readonly foot: number; readonly drop: number });
type Order = { readonly id: string } & ({ readonly kind: "rank"; readonly bodies: readonly Owned[] } | { readonly kind: "kept"; readonly before: readonly string[]; readonly after: readonly string[] } | { readonly kind: "vault"; readonly hopper: Row; readonly rise: number; readonly hurdle: Row });
type Head = { readonly id: string } & (
  | { readonly kind: "landing"; readonly before: Row; readonly after: Row; readonly owner: string; readonly bodies: readonly Owned[] }
  | { readonly kind: "resting" | "side"; readonly rider: Row; readonly host: Row }
  | { readonly kind: "strides"; readonly ticks: readonly number[] }
  | { readonly kind: "slide"; readonly rider: Row; readonly side: number; readonly ticks: number; readonly low: number; readonly high: number; readonly obstacles: readonly Row[] }
);
type Resort = { readonly id: string; readonly bodies: readonly Owned[]; readonly claims: readonly Planned[]; readonly tick: number } & ({ readonly kind: "poof"; readonly owner: string; readonly stay: Row | null; readonly plans: readonly Planned[] } | { readonly kind: "evict" });
type Vectors = {
  readonly bodies: readonly { readonly id: string; readonly feet: Point; readonly size: Size; readonly hover: number; readonly posture: Stance; readonly margin: number }[];
  readonly crowds: readonly { readonly id: string; readonly bodies: readonly Owned[]; readonly margin: number }[];
  readonly spots: readonly Place[];
  readonly strides: readonly { readonly id: string; readonly box: Row; readonly stride: number; readonly obstacles: readonly Row[] }[];
  readonly orders: readonly Order[];
  readonly claims: readonly { readonly id: string; readonly plan: Planned; readonly bodies: readonly Owned[]; readonly claims: readonly Planned[]; readonly ticks: readonly number[]; readonly probe: Row; readonly prober: string }[];
  readonly seatings: readonly { readonly id: string; readonly bodies: readonly Owned[]; readonly perch: Perch; readonly margin: number; readonly stride: number }[];
  readonly heads: readonly Head[];
  readonly pushes: readonly { readonly id: string; readonly box: Row; readonly obstacles: readonly Row[]; readonly iterations: number }[];
  readonly resorts: readonly Resort[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 📦️ A committed row as a box. */
function extent(row: Row): Extent {
  return { x0: row[0], y0: row[1], x1: row[2], y1: row[3] };
}

/** 🧾️ A box as the row it is committed as. */
function row(box: Extent): number[] {
  return [box.x0, box.y0, box.x1, box.y1];
}

/** 🐾️ Committed bodies as bodies. */
function embodied(bodies: readonly Owned[]): Body[] {
  return bodies.map((entry) => ({ owner: entry.owner, extent: extent(entry.box) }));
}

/** 🎫️ What a claim is made of as the claim `claimOf` makes of it. */
function claimed(plan: Planned): Claim {
  return claimOf(plan.owner, plan.from, plan.path.map(extent), plan.span, plan.rest === null ? null : extent(plan.rest));
}

/** 🧍️ The posture a vector names. */
function postured(stance: Stance, size: Size): Posture {
  if (stance.kind === "leaning") return leaning(stance.height, stance.sine);
  if (stance.kind === "canopied") return canopied(size, stance.canopy);
  if (stance.kind === "plain") return { left: stance.left, right: stance.right, above: stance.above };
  return UPRIGHT;
}

/** 📍️ The free place a vector asks for. */
function placed(place: Place): number | null {
  const obstacles = place.obstacles.map(extent);
  if (place.kind === "slot") return slotIn(place.x, extent(place.box), place.low, place.high, obstacles);
  if (place.kind === "spot") return spotOn(place.perch, place.x, extent(place.box), place.foot, obstacles);
  return columnOver(place.perch, place.x, extent(place.box), place.drop, place.foot, obstacles);
}

/** 🔢️ The order, the kept order or the vault a vector asks for. */
function ordered(order: Order): string[] | boolean {
  if (order.kind === "rank") return orderOf(embodied(order.bodies));
  if (order.kind === "kept") return orderKept(order.before, order.after);
  return vaults(extent(order.hopper), order.rise, extent(order.hurdle));
}

/** 🚦️ Everything asked about one plan. */
function planned(vector: Vectors["claims"][number]): unknown {
  const plan = claimed(vector.plan);
  const bodies = embodied(vector.bodies);
  const claims = vector.claims.map(claimed);
  const probe = extent(vector.probe);
  return {
    slices: plan.slices.map((slice) => [slice.from, slice.until, ...row(slice.extent)]),
    at: vector.ticks.map((tick) => {
      const box = sliceAt(plan, tick);
      return box === null ? null : row(box);
    }),
    clear: claimClear(plan, bodies, claims),
    free: vector.ticks.map((tick) => freeAt(probe, vector.prober, bodies, claims, tick)),
    obstacles: vector.ticks.map((tick) => obstaclesOf(vector.prober, bodies, claims, tick).length),
    pruned: vector.ticks.map((tick) => pruned(claims, tick).map((claim) => [claim.owner, claim.slices.length])),
    released: released(claims, vector.prober).map((claim) => claim.owner),
  };
}

/** 💺️ The seating of a perch and the first tick of the scoot to it. */
function seated(vector: Vectors["seatings"][number]): unknown {
  const bodies = embodied(vector.bodies);
  const seating = seatOf(bodies, vector.perch, vector.margin);
  const fraction = scootFraction(seating.seats.map((seat) => seat.shift), vector.stride);
  return {
    seats: seating.seats.map((seat) => [seat.owner, seat.shift]),
    leavers: seating.leavers,
    fraction,
    places: seating.seats.map((seat) => {
      const start = bodies.find((entry) => entry.owner === seat.owner)!.extent.x0;
      return scooted(start, start + seat.shift, fraction);
    }),
  };
}

/** 🎩️ What a vector asks about heads. */
function headed(head: Head): unknown {
  if (head.kind === "landing") {
    const after = extent(head.after);
    const host = headUnder(extent(head.before), after, head.owner, embodied(head.bodies));
    return host === null ? { host: null, lift: null } : { host: host.owner, lift: liftOnto(after, host.extent) };
  }
  if (head.kind === "strides") return head.ticks.map(slideStride);
  if (head.kind === "slide") {
    const slide = slideOf(extent(head.rider), head.side, head.ticks, head.low, head.high, head.obstacles.map(extent));
    return [slide.stride, slide.side];
  }
  return head.kind === "resting" ? restsOn(extent(head.rider), extent(head.host)) : slideSide(extent(head.rider), extent(head.host));
}

/** 💨️ The last resort a vector asks about. */
function resorted(resort: Resort): boolean | string[] {
  const bodies = embodied(resort.bodies);
  const claims = resort.claims.map(claimed);
  if (resort.kind === "evict") return evicted(bodies, claims, resort.tick);
  return mustPoof(resort.owner, resort.stay === null ? null : extent(resort.stay), resort.plans.map(claimed), bodies, claims, resort.tick);
}

/** 🗂️ The answers to a list of vectors by their ids. */
function answers<Vector extends { readonly id: string }>(list: readonly Vector[], answer: (vector: Vector) => unknown): Record<string, unknown> {
  return Object.fromEntries(list.map((vector) => [vector.id, answer(vector)]));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    constants: { subject: () => ({ projection: { margin: MARGIN, seam: SEAM, forever: FOREVER, steerings: [...STEERINGS], pushes: PUSHES, patience: PATIENCE, delay: DELAY, slideOffSpeed: SLIDE_OFF_SPEED, slideOffGain: SLIDE_OFF_GAIN, slideOffLimit: SLIDE_OFF_LIMIT, lean: LEAN, laneLift: LANE_LIFT, scootHaste: SCOOT_HASTE } }) },
    bodies: { subject: (ctx) => ({ projection: answers(vectors(ctx).bodies, (vector) => row(bodyOf(vector.id, vector.feet, vector.size, vector.hover, postured(vector.posture, vector.size), vector.margin).extent)) }) },
    crowds: {
      subject: (ctx) => ({
        projection: answers(vectors(ctx).crowds, (vector) => ({ overlaps: overlaps(embodied(vector.bodies)).map((pair) => [pair.first, pair.second]), nears: nearMisses(embodied(vector.bodies), vector.margin).map((pair) => [pair.first, pair.second]) })),
      }),
    },
    spots: { subject: (ctx) => ({ projection: answers(vectors(ctx).spots, placed) }) },
    strides: { subject: (ctx) => ({ projection: answers(vectors(ctx).strides, (vector) => guardedStride(extent(vector.box), vector.stride, vector.obstacles.map(extent))) }) },
    orders: { subject: (ctx) => ({ projection: answers(vectors(ctx).orders, ordered) }) },
    claims: { subject: (ctx) => ({ projection: answers(vectors(ctx).claims, planned) }) },
    seatings: { subject: (ctx) => ({ projection: answers(vectors(ctx).seatings, seated) }) },
    heads: { subject: (ctx) => ({ projection: answers(vectors(ctx).heads, headed) }) },
    pushes: {
      subject: (ctx) => ({
        projection: answers(vectors(ctx).pushes, (vector) => {
          const push = pushedOut(extent(vector.box), vector.obstacles.map(extent), vector.iterations);
          return push === null ? null : [push.x, push.y];
        }),
      }),
    },
    resorts: { subject: (ctx) => ({ projection: answers(vectors(ctx).resorts, resorted) }) },
  },
});
