/** 🏞️ Subject adapter of the terrain-walking case: `perchesOf`, `perchAt`, `nearestPerch` and `strideTo` of `@semio-tech/pets` answer every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🏞️terrain/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "@semio-tech/framework/test/adapter";
import { nearestPerch, perchAt, perchesOf, strideTo } from "../../🔨️modules/🏞️terrain/🟦️.ts";
import { castOn, standing } from "../../🔨️modules/📏️spacing/🟦️.ts";
import type { Menagerie, Perch, Point, Rect, Slug, Surface } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🏞️terrain-walking/🔣️.json";
const MENAGERIE = "shared://🧬️schema-conformance/🔣️.json";

type Vectors = {
  readonly layouts: readonly { readonly id: string; readonly width: number; readonly height: number; readonly clearance: number; readonly minimum: number; readonly surfaces: readonly Surface[]; readonly keepouts: readonly Rect[] }[];
  readonly standings: readonly { readonly id: string; readonly perches: readonly Perch[]; readonly queries: readonly { readonly surface: string; readonly x: number }[] }[];
  readonly nearests: readonly { readonly id: string; readonly perches: readonly Perch[]; readonly points: readonly Point[] }[];
  readonly companies: readonly { readonly id: string; readonly width: number; readonly height: number; readonly surfaces: readonly Surface[]; readonly keepouts: readonly Rect[]; readonly order: readonly Slug[]; readonly cast: { readonly core: readonly Slug[]; readonly rotation: readonly Slug[] }; readonly troupe: readonly Slug[]; readonly holding: readonly Slug[]; readonly avoid: readonly Slug[]; readonly epoch: number }[];
  readonly strides: readonly { readonly id: string; readonly x: number; readonly goal: number; readonly speed: number; readonly ticks: number }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🐾️ The menagerie the companies are drawn from. */
function menagerie(ctx: AdapterContext): Menagerie {
  return (JSON.parse(new TextDecoder().decode(ctx.inputBytes(MENAGERIE))) as { readonly menagerie: Menagerie }).menagerie;
}

/** 🔢️ The index of an answered perch in the list it was answered from, `null` for none. */
function indexOf(perches: readonly Perch[], perch: Perch | null): number | null {
  return perch === null ? null : perches.indexOf(perch);
}

/** 👣️ The x after every tick of a walk. */
function walked(x: number, goal: number, speed: number, ticks: number): number[] {
  const positions: number[] = [];
  let position = x;
  for (let tick = 0; tick < ticks; tick++) {
    position = strideTo(position, goal, speed);
    positions.push(position);
  }
  return positions;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    perches: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).layouts.map((layout) => [layout.id, perchesOf(layout.surfaces, layout.keepouts, layout.width, layout.height, layout.clearance, layout.minimum)])) }) },
    standing: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).standings.map((stand) => [stand.id, stand.queries.map((query) => indexOf(stand.perches, perchAt(stand.perches, query.surface, query.x)))])) }) },
    nearest: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).nearests.map((near) => [near.id, near.points.map((point) => indexOf(near.perches, nearestPerch(near.perches, point.x, point.y)))])) }) },
    company: {
      subject: (ctx) => {
        const pets = menagerie(ctx);
        return { projection: Object.fromEntries(vectors(ctx).companies.map((ground) => [ground.id, { standing: standing(pets, ground, ground.order).map((kind) => kind.id), cast: castOn(pets, ground, { scene: ground.id, ...ground.cast }, ground.troupe, ground.epoch, ground.holding, ground.avoid) }])) };
      },
    },
    strides: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).strides.map((walk) => [walk.id, walked(walk.x, walk.goal, walk.speed, walk.ticks)])) }) },
  },
});
