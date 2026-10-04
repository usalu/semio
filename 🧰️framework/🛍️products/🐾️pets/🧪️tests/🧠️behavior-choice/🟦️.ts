/** 🧠️ Subject adapter of the behavior-choice case: the pets behaviour module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧠️behavior/🟦️.ts
 * @see ../../🔨️modules/🎲️randomness/🟦️.ts — `randomPick`, `weightedIndex`
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { ACTIVITIES, type Activity, type Actor, type Cast, type Needs, type PetMode, type Species } from "../../🧬️schema/🟦️.ts";
import { randomPick, weightedIndex } from "../../🔨️modules/🎲️randomness/🟦️.ts";
import { MODE_LIMITS, activityWeights, castOf, dwellOf, encounterOf, encounterShares, followersOf } from "../../🔨️modules/🧠️behavior/🟦️.ts";

const VECTORS = "shared://🧠️behavior-choice/🔣️.json";

type Circumstance = { readonly id: string; readonly mode: PetMode; readonly quiet: boolean; readonly movers: number; readonly fidgeters: number; readonly roam: boolean; readonly hops: boolean; readonly crowd: number; readonly watched: boolean; readonly whims: boolean; readonly fidgets: boolean; readonly needs: Needs };

type Vectors = {
  readonly limits: readonly { readonly id: PetMode }[];
  readonly weights: readonly Circumstance[];
  readonly picks: readonly { readonly id: string; readonly weights: readonly number[]; readonly units: readonly number[] }[];
  readonly decisions: readonly (Circumstance & { readonly seed: number; readonly stream: number; readonly count: number })[];
  readonly dwells: readonly { readonly id: string; readonly activity: Activity; readonly mode: PetMode; readonly units: readonly number[] }[];
  readonly encounters: readonly { readonly id: string; readonly affinity: number; readonly units: readonly number[] }[];
  readonly graph: readonly { readonly id: string }[];
  readonly casts: readonly { readonly id: string; readonly cast: Cast; readonly capacity: number; readonly seed: number; readonly epochs: readonly number[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** ⚖️ The weights of a committed situation: an actor with the committed needs and a species with or without a fidget. */
function weighed(vector: Circumstance): number[] {
  const actor = { needs: vector.needs } as Actor;
  const species = { repertoire: vector.fidgets ? { fidget: ["fidget"] } : {} } as Species;
  return activityWeights(actor, species, { mode: vector.mode, quiet: vector.quiet, movers: vector.movers, fidgeters: vector.fidgeters, roam: vector.roam, hops: vector.hops, crowd: vector.crowd, watched: vector.watched, whims: vector.whims });
}

/** 🙋️ What an actor decides at the counters `0 … count − 1` of its stream, and how often it decides what. */
function decided(vector: Circumstance & { readonly seed: number; readonly stream: number; readonly count: number }): { activities: Activity[]; counts: Record<string, number> } {
  const weights = weighed(vector);
  const activities = Array.from({ length: vector.count }, (_, counter) => ACTIVITIES[randomPick([vector.seed, vector.stream, counter], weights)]!);
  return { activities, counts: Object.fromEntries(ACTIVITIES.map((activity) => [activity, activities.filter((chosen) => chosen === activity).length])) };
}

/** 🕸️ The activity graph: the followers of every activity, what a breadth-first search reaches from each one, and the number of strongly connected components (1 when everything reaches everything). */
function graph(): { followers: Record<string, readonly Activity[]>; reachable: Record<string, Activity[]>; components: number } {
  const reachable: Record<string, Activity[]> = {};
  for (const start of ACTIVITIES) {
    const seen = new Set<Activity>([start]);
    const queue: Activity[] = [start];
    for (let head = 0; head < queue.length; head++) {
      for (const follower of followersOf(queue[head]!)) {
        if (seen.has(follower)) continue;
        seen.add(follower);
        queue.push(follower);
      }
    }
    reachable[start] = ACTIVITIES.filter((activity) => seen.has(activity));
  }
  const classes = new Set(ACTIVITIES.map((activity) => ACTIVITIES.filter((other) => reachable[activity]!.includes(other) && reachable[other]!.includes(activity)).join(" ")));
  return { followers: Object.fromEntries(ACTIVITIES.map((activity) => [activity, followersOf(activity)])), reachable, components: classes.size };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    limits: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).limits.map((vector) => [vector.id, MODE_LIMITS[vector.id]])) }) },
    weights: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).weights.map((vector) => [vector.id, weighed(vector)])) }) },
    picks: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).picks.map((vector) => [vector.id, vector.units.map((unit) => weightedIndex(vector.weights, unit))])) }) },
    decisions: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).decisions.map((vector) => [vector.id, decided(vector)])) }) },
    dwells: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).dwells.map((vector) => [vector.id, vector.units.map((unit) => dwellOf(vector.activity, vector.mode, unit))])) }) },
    encounters: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).encounters.map((vector) => [vector.id, { shares: encounterShares(vector.affinity), kinds: vector.units.map((unit) => encounterOf(vector.affinity, unit)) }])) }) },
    reachability: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).graph.map((vector) => [vector.id, graph()])) }) },
    casts: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).casts.map((vector) => [vector.id, vector.epochs.map((epoch) => castOf(vector.cast, vector.capacity, epoch, vector.seed))])) }) },
  },
});
