/** 🪄️ Subject adapter of the mischief-choice case: the pets mischief module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🪄️mischief/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Circumstances, allowed, allowedFrom, chosenFixture, fits, fixtureFor, liftAt, liftEnds, stationFor, thrownOff } from "../../🔨️modules/🪄️mischief/🟦️.ts";

const VECTORS = "shared://🪄️mischief-choice/🔣️.json";

type Box = { readonly id: string; readonly key: string; readonly x: number; readonly y: number; readonly width: number; readonly height: number };
type Described = Box & { readonly value: unknown; readonly correct: boolean; readonly answered: unknown };

type Vectors = {
  readonly matches: readonly { readonly id: string; readonly ground: string; readonly key: string }[];
  readonly candidates: readonly { readonly id: string; readonly grounds: readonly string[]; readonly fixtures: readonly Box[] }[];
  readonly choices: readonly { readonly id: string; readonly count: number; readonly units: readonly number[] }[];
  readonly leaks: readonly { readonly id: string; readonly grounds: readonly string[]; readonly unit: number; readonly items: readonly Described[]; readonly shuffles: readonly (readonly number[])[] }[];
  readonly gates: readonly (Circumstances & { readonly id: string })[];
  readonly stations: readonly { readonly id: string; readonly fixture: Box; readonly pitches: Parameters<typeof stationFor>[1]; readonly perches: Parameters<typeof stationFor>[2]; readonly width: number }[];
  readonly lifts: readonly { readonly id: string; readonly since: number; readonly side: 1 | -1; readonly room: number; readonly span: number; readonly unit: number; readonly first: number; readonly last: number; readonly step: number }[];
  readonly throws: readonly { readonly id: string; readonly pusher: { readonly x: number; readonly y: number }; readonly fixture: Box; readonly unit: number }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🧲️ The id of the fixture a species picks in a host description, `null` when nothing fits; the description is handed over whole, with everything the page knows about its items. */
function picked(grounds: readonly string[], items: readonly Described[], unit: number): string | null {
  return chosenFixture(fixtureFor(grounds, items), unit)?.id ?? null;
}

/** 🙈️ The pick over the plain description and over every permutation of the values, the correctness flags and the answers among its items. */
function leak(grounds: readonly string[], items: readonly Described[], unit: number, shuffles: readonly (readonly number[])[]): { plain: string | null; shuffled: (string | null)[] } {
  return {
    plain: picked(grounds, items, unit),
    shuffled: shuffles.map((shuffle) =>
      picked(
        grounds,
        items.map((item, place) => ({ ...item, value: items[shuffle[place]!]!.value, correct: items[shuffle[place]!]!.correct, answered: items[shuffle[place]!]!.answered })),
        unit,
      ),
    ),
  };
}

/** 🎢️ The copy of a lifted fixture at every `step`-th tick from `first` to `last`: dx, dy, tilt and opacity as four lists, and the tick the lift ends. */
function trajectory(since: number, side: 1 | -1, room: number, span: number, unit: number, first: number, last: number, step: number): { dx: number[]; dy: number[]; tilt: number[]; opacity: number[]; ends: number } {
  const path = { dx: [] as number[], dy: [] as number[], tilt: [] as number[], opacity: [] as number[], ends: liftEnds(since) };
  for (let tick = first; tick <= last; tick += step) {
    const lift = liftAt(since, tick, side, room, span, unit);
    path.dx.push(lift.dx);
    path.dy.push(lift.dy);
    path.tilt.push(lift.tilt);
    path.opacity.push(lift.opacity);
  }
  return path;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    matches: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).matches.map((vector) => [vector.id, fits(vector.ground, vector.key)])) }) },
    candidates: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).candidates.map((vector) => [vector.id, fixtureFor(vector.grounds, vector.fixtures).map((fixture) => fixture.id)])) }) },
    choices: {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          vectors(ctx).choices.map((vector) => [
            vector.id,
            vector.units.map(
              (unit) =>
                chosenFixture(
                  Array.from({ length: vector.count }, (_, position) => position),
                  unit,
                ) ?? -1,
            ),
          ]),
        ),
      }),
    },
    leaks: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).leaks.map((vector) => [vector.id, leak(vector.grounds, vector.items, vector.unit, vector.shuffles)])) }) },
    gates: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).gates.map((vector) => [vector.id, { allowed: allowed(vector), from: allowedFrom(vector) }])) }) },
    stations: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).stations.map((vector) => [vector.id, stationFor(vector.fixture, vector.pitches, vector.perches, vector.width)])) }) },
    lifts: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).lifts.map((vector) => [vector.id, trajectory(vector.since, vector.side, vector.room, vector.span, vector.unit, vector.first, vector.last, vector.step)])) }) },
    throws: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).throws.map((vector) => [vector.id, thrownOff(vector.pusher, vector.fixture, vector.unit)])) }) },
  },
});
