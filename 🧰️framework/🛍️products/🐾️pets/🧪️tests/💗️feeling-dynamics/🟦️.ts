/** 💗️ Subject adapter of the feeling-dynamics case: the pets feeling module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/💗️feeling/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Feeling, MOODS, type Mood, type Trick } from "../../🧬️schema/🟦️.ts";
import {
  APPRAISALS,
  CONTAGION_BEAT,
  CONTAGION_REACH,
  type Character,
  DROWSY_ENERGY,
  MOOD_AFFINITIES,
  MOOD_DECAYS,
  MOOD_DROPS,
  MOOD_FAINT,
  MOOD_HOLD,
  MOOD_LIDS,
  MOOD_OVERRIDE,
  MOOD_PRIORITIES,
  MOOD_REST,
  MOOD_RISE,
  MOOD_SHARES,
  MOOD_SLANTS,
  MOOD_SPREADS,
  MOOD_VALENCES,
  MOOD_WEIGHTS,
  OCCASIONS,
  PRONENESS,
  PRONE_DECAY,
  PRONE_GAIN,
  TRICK_AMOUNT,
  appraised,
  caught,
  drowsed,
  encounterBias,
  faceOf,
  impulse,
  moodWeights,
  performed,
  pronenessOf,
  settled,
  settlesAt,
  spiritsOf,
  swayedShares,
  valenceOf,
} from "../../🔨️modules/💗️feeling/🟦️.ts";

const VECTORS = "shared://💗️feeling-dynamics/🔣️.json";

type Vectors = {
  readonly tables: readonly { readonly id: string }[];
  readonly priorities: readonly { readonly id: string }[];
  readonly impulses: readonly { readonly id: string; readonly resting: Mood; readonly feeling: Feeling; readonly events: readonly { readonly tick: number; readonly mood: Mood; readonly amount: number }[] }[];
  readonly decays: readonly { readonly id: string; readonly feeling: Feeling; readonly resting: Mood; readonly ticks: readonly number[] }[];
  readonly jumps: readonly { readonly id: string; readonly feeling: Feeling; readonly resting: Mood; readonly cuts: readonly number[] }[];
  readonly contagion: readonly Crowd[];
  readonly faces: readonly { readonly id: string; readonly feeling: Feeling }[];
  readonly appraisals: readonly { readonly id: string; readonly character: Character; readonly feelings: readonly Feeling[]; readonly tick: number; readonly energies: readonly number[] }[];
  readonly wishes: readonly { readonly id: string; readonly intensity: number }[];
  readonly leanings: readonly { readonly id: string; readonly first: Feeling; readonly second: Feeling; readonly shares: readonly number[] }[];
};

type Crowd = { readonly id: string; readonly restings: readonly Mood[]; readonly feelings: readonly Feeling[]; readonly sociabilities: readonly number[]; readonly affinities: readonly (readonly number[])[]; readonly near: readonly (readonly boolean[])[]; readonly beats: number };

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 📋️ Every table and constant of the module, under the names the oracle uses. */
function tables(): unknown {
  return {
    priorities: MOOD_PRIORITIES,
    hold: MOOD_HOLD,
    faint: MOOD_FAINT,
    rest: MOOD_REST,
    rise: MOOD_RISE,
    override: MOOD_OVERRIDE,
    decays: MOOD_DECAYS,
    proneDecay: PRONE_DECAY,
    valences: MOOD_VALENCES,
    lids: MOOD_LIDS,
    slants: MOOD_SLANTS,
    drops: MOOD_DROPS,
    occasions: OCCASIONS,
    appraisals: APPRAISALS,
    proneness: PRONENESS,
    proneGain: PRONE_GAIN,
    trickAmount: TRICK_AMOUNT,
    drowsyEnergy: DROWSY_ENERGY,
    weights: MOOD_WEIGHTS,
    affinities: MOOD_AFFINITIES,
    shares: MOOD_SHARES,
    spreads: MOOD_SPREADS,
    contagionBeat: CONTAGION_BEAT,
    contagionReach: CONTAGION_REACH,
  };
}

/** 🥇️ Which impulse of 0.5 replaces which mood of 0.8: a row per present mood, a column per impulse, at a tick inside the hold or at the one that ends it. */
function contest(tick: number): number[][] {
  return MOODS.map((present) => MOODS.map((stirring) => (stirring !== present && impulse({ mood: present, intensity: 0.8, since: 0 }, stirring, 0.5, tick).mood === stirring ? 1 : 0)));
}

/** 📖️ A feeling through its events: settled at the tick of each, then stirred by its impulse. */
function story(vector: Vectors["impulses"][number]): Feeling[] {
  const feelings: Feeling[] = [];
  let feeling = vector.feeling;
  for (const event of vector.events) {
    feeling = impulse(settled(feeling, vector.resting, event.tick), event.mood, event.amount, event.tick);
    feelings.push(feeling);
  }
  return feelings;
}

/** 🦘️ A feeling settled through every cut in turn, and in one step. */
function jump(vector: Vectors["jumps"][number]): { stepped: Feeling; direct: Feeling } {
  let stepped = vector.feeling;
  for (const cut of vector.cuts) stepped = settled(stepped, vector.resting, cut);
  return { stepped, direct: settled(vector.feeling, vector.resting, vector.cuts[vector.cuts.length - 1]!) };
}

/** 🦠️ A crowd through its beats: before every beat everybody is settled, then everybody catches from each neighbour in order, each neighbour as it stood when the beat began. */
function crowd(vector: Crowd): Feeling[][] {
  const beats: Feeling[][] = [];
  let feelings: readonly Feeling[] = vector.feelings;
  for (let beat = 1; beat <= vector.beats; beat++) {
    const tick = beat * CONTAGION_BEAT;
    const present = feelings.map((feeling, index) => settled(feeling, vector.restings[index]!, tick));
    const next = present.map((mine, catcher) => {
      let feeling = mine;
      for (let giver = 0; giver < present.length; giver++) if (giver !== catcher && vector.near[giver]![catcher]!) feeling = caught(feeling, present[giver]!, vector.affinities[giver]![catcher]!, vector.sociabilities[catcher]!, tick);
      return feeling;
    });
    beats.push(next);
    feelings = next;
  }
  return beats;
}

/** 🧐️ The proneness of a character and its feelings after every occasion, every kind of trick and every look at its energy. */
function appraisal(vector: Vectors["appraisals"][number]): unknown {
  const kinds: readonly (Mood | undefined)[] = [undefined, ...MOODS];
  return {
    proneness: MOODS.map((mood) => pronenessOf(vector.character, mood)),
    occasions: Object.fromEntries(OCCASIONS.map((occasion) => [occasion, vector.feelings.map((feeling) => appraised(feeling, occasion, vector.character, vector.tick))])),
    tricks: Object.fromEntries(kinds.map((mood) => [mood ?? "none", vector.feelings.map((feeling) => performed(feeling, { mood } as Trick, vector.character, vector.tick))])),
    drowsy: vector.feelings.map((feeling) => vector.energies.map((energy) => drowsed(feeling, energy, vector.tick))),
  };
}

/** 🤝️ How an encounter of two feelings leans and what that does to shares. */
function leaning(vector: Vectors["leanings"][number]): unknown {
  const bias = encounterBias(vector.first, vector.second);
  return { leaning: bias, swayed: swayedShares(vector.shares, bias) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    tables: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).tables.map((vector) => [vector.id, tables()])) }) },
    priorities: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).priorities.map((vector) => [vector.id, { during: contest(MOOD_HOLD / 2), after: contest(MOOD_HOLD) }])) }) },
    impulses: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).impulses.map((vector) => [vector.id, story(vector)])) }) },
    decays: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).decays.map((vector) => [vector.id, { views: vector.ticks.map((tick) => settled(vector.feeling, vector.resting, tick)), settles: settlesAt(vector.feeling, vector.resting) }])) }) },
    jumps: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).jumps.map((vector) => [vector.id, jump(vector)])) }) },
    contagion: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).contagion.map((vector) => [vector.id, crowd(vector)])) }) },
    faces: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).faces.map((vector) => [vector.id, { valence: valenceOf(vector.feeling.mood), spirits: spiritsOf(vector.feeling), face: faceOf(vector.feeling) }])) }) },
    appraisals: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).appraisals.map((vector) => [vector.id, appraisal(vector)])) }) },
    wishes: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).wishes.map((vector) => [vector.id, Object.fromEntries(MOODS.map((mood) => [mood, moodWeights(mood, vector.intensity)]))])) }) },
    leanings: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).leanings.map((vector) => [vector.id, leaning(vector)])) }) },
  },
});
