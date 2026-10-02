/** 🤝️ Subject adapter of the bond-dynamics case: the pets behaviour module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🧠️behavior/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { ACTIVITIES, type Activity, type Bond, type Menagerie, type Needs, type Rapport, type Slug, type Temperament } from "../../🧬️schema/🟦️.ts";
import { affinityOf, needsAfter, needsOf, rapportAfter, rapportFaded } from "../../🔨️modules/🧠️behavior/🟦️.ts";

const VECTORS = "shared://🤝️bond-dynamics/🔣️.json";

type Vectors = {
  readonly affinities: readonly { readonly id: string; readonly bonds: readonly Bond[]; readonly rapports: readonly Rapport[]; readonly pairs: readonly (readonly [Slug, Slug])[] }[];
  readonly rapportSteps: readonly { readonly id: string; readonly drift: number }[];
  readonly rapportFading: readonly { readonly id: string; readonly drift: number; readonly ticks: readonly number[] }[];
  readonly histories: readonly { readonly id: string; readonly affinity: number; readonly events: readonly { readonly after: number; readonly activity: Activity }[] }[];
  readonly needs: readonly { readonly id: string; readonly needs: Needs; readonly activity: Activity; readonly ticks: number; readonly temperament: Temperament }[];
  readonly days: readonly { readonly id: string; readonly temperament: Temperament; readonly spans: readonly { readonly activity: Activity; readonly ticks: number }[] }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(VECTORS))) as Vectors;
}

/** 🎪️ A menagerie that carries nothing but bonds. */
function bonded(bonds: readonly Bond[]): Menagerie {
  return { bonds } as Menagerie;
}

/** 📖️ The drift and the affinity of the pair `a`–`b` after every event of a history. */
function history(affinity: number, events: readonly { readonly after: number; readonly activity: Activity }[]): { drifts: number[]; affinities: number[] } {
  const menagerie = bonded([{ between: ["a", "b"], affinity }]);
  const drifts: number[] = [];
  const affinities: number[] = [];
  let drift = 0;
  for (const event of events) {
    drift = rapportAfter(rapportFaded(drift, event.after), event.activity);
    drifts.push(drift);
    affinities.push(affinityOf(menagerie, [{ between: ["a", "b"], drift }], "b", "a"));
  }
  return { drifts, affinities };
}

/** 🌗️ The needs of a temperament at its arrival and after every span. */
function day(temperament: Temperament, spans: readonly { readonly activity: Activity; readonly ticks: number }[]): Needs[] {
  const states: Needs[] = [needsOf(temperament)];
  for (const span of spans) states.push(needsAfter(states[states.length - 1]!, span.activity, span.ticks, temperament));
  return states;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    affinities: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).affinities.map((vector) => [vector.id, vector.pairs.map((pair) => affinityOf(bonded(vector.bonds), vector.rapports, pair[0], pair[1]))])) }) },
    "rapport-steps": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).rapportSteps.map((vector) => [vector.id, Object.fromEntries(ACTIVITIES.map((activity) => [activity, rapportAfter(vector.drift, activity)]))])) }) },
    "rapport-fading": { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).rapportFading.map((vector) => [vector.id, vector.ticks.map((ticks) => rapportFaded(vector.drift, ticks))])) }) },
    histories: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).histories.map((vector) => [vector.id, history(vector.affinity, vector.events)])) }) },
    needs: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).needs.map((vector) => [vector.id, needsAfter(vector.needs, vector.activity, vector.ticks, vector.temperament)])) }) },
    days: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).days.map((vector) => [vector.id, day(vector.temperament, vector.spans)])) }) },
  },
});
