/** 💗️ Unit suite of the feeling module: how a mood comes, holds, fades and travels, what it does to the face, the wishes and the encounters of a pet, how the states of a species follow each other, which tricks are on offer, how reactions are found and folded, the shared vectors of both Protocol v2 cases, and the arithmetic the core is allowed to use.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧫️fixtures/💗️feeling-dynamics/🔣️.json
 * @see ../../../../🧫️fixtures/⚗️chemistry-rules/🔣️.json
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACTIVITIES, CUES, MOODS, type Activity, type Cooling, type Feeling, type Menagerie, type Mood, type Rapport, type Reaction, type Slug, type Species, type Trick } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { randomWords, unitOf, weightedIndex } from "../../../🎲️randomness/🟦️.ts";
import { encounterShares } from "../../../🧠️behavior/🟦️.ts";
import {
  APPRAISALS,
  CHEMISTRY_BEAT,
  CONTAGION_BEAT,
  CONTAGION_REACH,
  type Character,
  type Chemistry,
  type Consequence,
  DROWSY_ENERGY,
  EFFECT_AMOUNT,
  LEAN_TICKS,
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
  type Occasion,
  PRONENESS,
  PRONE_DECAY,
  PRONE_GAIN,
  type Sighting,
  type Standing,
  TRICK_AMOUNT,
  type Trial,
  WHIM_FAVOR,
  WILLING_MOODS,
  appraised,
  atRest,
  calmsAt,
  caught,
  clickTrick,
  drawsOf,
  drowsed,
  encounterBias,
  everyTicks,
  faceOf,
  heldTicks,
  impulse,
  ladderOf,
  lastingTicks,
  matches,
  moodWeights,
  nearby,
  performed,
  pronenessOf,
  reactionsOf,
  rungsOf,
  seen,
  settled,
  settlesAt,
  showTrick,
  shownMood,
  spiritsOf,
  stateAfterTrick,
  stateAt,
  stateEnds,
  stepRung,
  stepState,
  stirred,
  swayedShares,
  trialsOf,
  tricksFor,
  valenceOf,
  whimTrick,
} from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TOLERANCE = 1e-9;

/** 🎯️ How many random feelings a law of the moods is held to at the level of the run. */
const FEELINGS = sampled(200, 2000, 20000);

/** 🎬️ How many random stages a law of the chemistry is held to at the level of the run. */
const STAGES = sampled(40, 400, 4000);

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type Named = Feeling & { readonly id: string };
type Beat = { readonly tick: number; readonly sightings: readonly Sighting[]; readonly units: readonly number[]; readonly rapports: readonly Rapport[] };
type FeelingVectors = {
  readonly tables: readonly { readonly id: string; readonly expected: Record<string, unknown> }[];
  readonly priorities: readonly { readonly id: string; readonly expected: { readonly during: readonly (readonly number[])[]; readonly after: readonly (readonly number[])[] } }[];
  readonly impulses: readonly { readonly id: string; readonly resting: Mood; readonly feeling: Feeling; readonly events: readonly { readonly tick: number; readonly mood: Mood; readonly amount: number }[]; readonly expected: readonly Feeling[] }[];
  readonly decays: readonly { readonly id: string; readonly feeling: Feeling; readonly resting: Mood; readonly ticks: readonly number[]; readonly expected: { readonly views: readonly Feeling[]; readonly settles: number } }[];
  readonly jumps: readonly { readonly id: string; readonly feeling: Feeling; readonly resting: Mood; readonly cuts: readonly number[]; readonly expected: { readonly stepped: Feeling; readonly direct: Feeling } }[];
  readonly contagion: readonly { readonly id: string; readonly restings: readonly Mood[]; readonly feelings: readonly Feeling[]; readonly sociabilities: readonly number[]; readonly affinities: readonly (readonly number[])[]; readonly near: readonly (readonly boolean[])[]; readonly beats: number; readonly expected: readonly (readonly Feeling[])[] }[];
  readonly faces: readonly { readonly id: string; readonly feeling: Feeling; readonly expected: unknown }[];
  readonly appraisals: readonly { readonly id: string; readonly character: Character; readonly feelings: readonly Feeling[]; readonly tick: number; readonly energies: readonly number[]; readonly expected: { readonly proneness: readonly number[]; readonly occasions: Record<Occasion, readonly Feeling[]>; readonly tricks: Record<string, readonly Feeling[]>; readonly drowsy: readonly (readonly Feeling[])[] } }[];
  readonly wishes: readonly { readonly id: string; readonly intensity: number; readonly expected: Record<Mood, readonly number[]> }[];
  readonly leanings: readonly { readonly id: string; readonly first: Feeling; readonly second: Feeling; readonly shares: readonly number[]; readonly expected: { readonly leaning: unknown; readonly swayed: readonly number[] } }[];
};
type ChemistryVectors = {
  readonly menagerie: Menagerie;
  readonly relations: readonly { readonly id: string; readonly first: Sighting; readonly second: Sighting; readonly reaches: readonly number[]; readonly expected: { readonly nearby: readonly boolean[]; readonly above: boolean; readonly below: boolean; readonly beside: boolean; readonly any: boolean } }[];
  readonly states: readonly { readonly id: string; readonly species: Slug; readonly state: Slug; readonly since: number; readonly ticks: readonly number[]; readonly expected: { readonly standings: readonly Standing[]; readonly ends: number | null; readonly lasts: number } }[];
  readonly ladders: readonly { readonly id: Slug; readonly expected: { readonly ladder: readonly Slug[]; readonly rungs: Record<string, readonly Slug[]>; readonly steps: Record<string, { readonly up: Slug; readonly down: Slug; readonly stay: Slug }>; readonly after: Record<string, Record<string, Slug>> } }[];
  readonly tricks: readonly { readonly id: Slug; readonly feelings: readonly Named[]; readonly clicks: readonly number[]; readonly units: readonly number[]; readonly expected: { readonly offers: Record<string, Record<string, Record<string, readonly Slug[]>>>; readonly clicks: Record<string, readonly (Slug | null)[]>; readonly whims: Record<string, Record<string, readonly (Slug | null)[]>>; readonly shows: Record<string, Record<string, readonly (Slug | null)[]>> } }[];
  readonly reachability: readonly { readonly id: Slug; readonly expected: { readonly edges: readonly (readonly [Slug, Slug])[]; readonly reachable: readonly Slug[]; readonly unreachable: readonly Slug[] } }[];
  readonly matching: readonly { readonly id: string; readonly sightings: readonly Sighting[]; readonly coolings: readonly Cooling[]; readonly tick: number; readonly rapports: readonly Rapport[]; readonly expected: readonly Trial[] }[];
  readonly reactions: readonly { readonly id: string; readonly beats: readonly Beat[]; readonly expected: readonly Chemistry[] }[];
};

const FELT = fixture<FeelingVectors>("💗️feeling-dynamics");
const SET = fixture<ChemistryVectors>("⚗️chemistry-rules");
const MENAGERIE = SET.menagerie;
const CALM: Feeling = { mood: "content", intensity: MOOD_REST, since: 0 };
const HOST: Character = { mood: "content", temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 } };

/** 🧬️ One species of the committed menagerie. */
function kind(id: Slug): Species {
  return MENAGERIE.species.find((species) => species.id === id)!;
}

/** 🔬️ Where an answer first differs from an expected one, or `null`: numbers within 1e-9, everything else exactly, arrays and objects member by member. */
function difference(actual: unknown, expected: unknown, where: string): string | null {
  if (typeof expected === "number" && typeof actual === "number") return Math.abs(actual - expected) <= TOLERANCE ? null : `${where}: ${actual} is not ${expected}`;
  if (Array.isArray(expected)) {
    if (!Array.isArray(actual) || actual.length !== expected.length) return `${where}: ${JSON.stringify(actual)} is not a list of ${expected.length}`;
    for (let index = 0; index < expected.length; index++) {
      const found = difference(actual[index], expected[index], `${where}[${index}]`);
      if (found !== null) return found;
    }
    return null;
  }
  if (expected !== null && typeof expected === "object") {
    if (actual === null || typeof actual !== "object" || Array.isArray(actual)) return `${where}: ${JSON.stringify(actual)} is not an object`;
    const keys = Object.keys(expected);
    if (Object.keys(actual).length !== keys.length) return `${where}: the keys ${Object.keys(actual).join(", ")} are not ${keys.join(", ")}`;
    for (const key of keys) {
      if (!(key in actual)) return `${where}: ${key} is missing`;
      const found = difference((actual as Record<string, unknown>)[key], (expected as Record<string, unknown>)[key], `${where}.${key}`);
      if (found !== null) return found;
    }
    return null;
  }
  return actual === expected ? null : `${where}: ${JSON.stringify(actual)} is not ${JSON.stringify(expected)}`;
}

/** 📏️ Holds an answer to an expected one ({@link difference}). */
function agree(actual: unknown, expected: unknown, where: string): void {
  expect(difference(actual, expected, where)).toBeNull();
}

/** 🧊️ A value frozen all the way down: whatever changes it throws. */
function frozen<Value>(value: Value): Value {
  if (value !== null && typeof value === "object") {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}

/** 🎲️ A stream of units from the product's own counter-based draws, so every run of the suite sees the same samples. */
function units(seed: number, count: number): number[] {
  return randomWords([seed, 77, 0], count).map(unitOf);
}

/** 🫀️ A random feeling with a dyadic intensity (sixty-fourths), so its arithmetic is exact. */
function drawn(marks: readonly number[], index: number): Feeling {
  return { mood: MOODS[Math.floor(marks[index]! * MOODS.length)]!, intensity: Math.floor(marks[index + 1]! * 65) / 64, since: Math.floor(marks[index + 2]! * 500) };
}

/** 👀️ An actor of the committed menagerie as the chemistry sees it, standing on the ground line. */
function sight(species: Slug, state: Slug, x: number, more: Partial<Sighting> = {}): Sighting {
  const size = kind(species).size;
  return { species, state, held: 0, mood: "content", intensity: MOOD_REST, activity: "idle", trick: null, x, y: 300, width: size.width, height: size.height, ...more };
}

describe("the tables of the moods", () => {
  it("carry one entry per mood, in MOODS order, and one multiplier per activity", () => {
    for (const table of [MOOD_PRIORITIES, MOOD_DECAYS, MOOD_VALENCES, MOOD_LIDS, MOOD_SLANTS, MOOD_DROPS, MOOD_AFFINITIES, MOOD_SPREADS, MOOD_SHARES, MOOD_WEIGHTS, PRONENESS, WILLING_MOODS]) expect(table.length).toBe(MOODS.length);
    for (const row of MOOD_WEIGHTS) expect(row.length).toBe(ACTIVITIES.length);
    for (const row of MOOD_SHARES) expect(row.length).toBe(3);
    for (const row of PRONENESS) expect(row.length).toBe(4);
  });

  it("rank the moods as the design says: scared, grumpy, sad, the three joys alike, curious, sleepy, content", () => {
    const rank = (mood: Mood): number => MOOD_PRIORITIES[MOODS.indexOf(mood)]!;
    expect(rank("scared")).toBeGreaterThan(rank("grumpy"));
    expect(rank("grumpy")).toBeGreaterThan(rank("sad"));
    expect(rank("sad")).toBeGreaterThan(rank("proud"));
    expect(rank("proud")).toBe(rank("happy"));
    expect(rank("happy")).toBe(rank("playful"));
    expect(rank("playful")).toBeGreaterThan(rank("curious"));
    expect(rank("curious")).toBeGreaterThan(rank("sleepy"));
    expect(rank("sleepy")).toBeGreaterThan(rank("content"));
  });

  it("keep every number in its range", () => {
    for (const decay of MOOD_DECAYS) expect(decay).toBeGreaterThan(0);
    for (const valence of MOOD_VALENCES) expect(Math.abs(valence)).toBeLessThanOrEqual(1);
    for (const lid of MOOD_LIDS) expect(lid >= 0 && lid < 1).toBe(true);
    for (const spread of MOOD_SPREADS) expect(spread >= 0 && spread < 1).toBe(true);
    for (const row of MOOD_WEIGHTS) for (const weight of row) expect(weight).toBeGreaterThanOrEqual(0);
    for (const row of MOOD_SHARES) for (const share of row) expect(share).toBeGreaterThan(0);
    expect(MOOD_FAINT).toBeLessThan(MOOD_REST);
    expect(MOOD_REST).toBeLessThan(1);
    expect(MOOD_OVERRIDE).toBeGreaterThan(1);
    expect(PRONE_DECAY > 0 && PRONE_DECAY <= 1).toBe(true);
    expect(PRONE_GAIN).toBeGreaterThanOrEqual(1);
    expect(MOOD_HOLD % CONTAGION_BEAT).toBe(0);
    expect(CONTAGION_BEAT).toBe(CHEMISTRY_BEAT);
    expect(CONTAGION_REACH).toBeGreaterThan(0);
    expect(LEAN_TICKS).toBeGreaterThan(0);
    expect(EFFECT_AMOUNT > MOOD_REST && EFFECT_AMOUNT <= 1).toBe(true);
    expect(WHIM_FAVOR).toBeGreaterThan(1);
  });

  it("leave content, the calm mood, without any effect", () => {
    expect(MOOD_WEIGHTS[0]!.every((weight) => weight === 1)).toBe(true);
    expect(MOOD_SHARES[0]).toEqual([1, 1, 1]);
    expect(MOOD_AFFINITIES[0]).toBe(0);
    expect(MOOD_SPREADS[0]).toBe(0);
  });

  it("only move what a pet starts by itself", () => {
    const own: readonly Activity[] = ["fidget", "walk", "hop", "sleep", "aim", "climb", "carry", "trick", "push"];
    for (const row of MOOD_WEIGHTS) ACTIVITIES.forEach((activity, index) => expect(own.includes(activity) || row[index] === 1, activity).toBe(true));
  });

  it("give every occasion at least one impulse, of a mood that exists and an amount a pet can feel", () => {
    for (const occasion of OCCASIONS) expect(APPRAISALS.some((stir) => stir.occasion === occasion), occasion).toBe(true);
    for (const stir of APPRAISALS) {
      expect(OCCASIONS.includes(stir.occasion)).toBe(true);
      expect(MOODS.includes(stir.mood)).toBe(true);
      expect(stir.amount > 0 && stir.amount <= 1).toBe(true);
      if (stir.mood !== "content") expect(stir.amount).toBeGreaterThan(MOOD_REST);
    }
    expect(APPRAISALS.find((stir) => stir.occasion === "tricked")!.amount).toBe(TRICK_AMOUNT);
  });

  it("are the committed tables", () => {
    const tables: Record<string, unknown> = {
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
    for (const vector of FELT.tables) agree(tables, vector.expected, vector.id);
  });
});

describe("an impulse", () => {
  it("reinforces the same mood up to 1 and begins its hold anew", () => {
    expect(impulse({ mood: "happy", intensity: 0.5, since: 10 }, "happy", 0.25, 90)).toEqual({ mood: "happy", intensity: 0.75, since: 90 });
    expect(impulse({ mood: "happy", intensity: 0.75, since: 10 }, "happy", 0.5, 90)).toEqual({ mood: "happy", intensity: 1, since: 90 });
  });

  it("replaces a calm feeling with anything a pet can feel", () => {
    for (const mood of MOODS) {
      if (mood === "content") continue;
      expect(impulse(CALM, mood, 0.25, 7)).toEqual({ mood, intensity: 0.25, since: 7 });
      expect(impulse({ mood: "scared", intensity: MOOD_REST, since: 7 }, mood, MOOD_FAINT, 7).mood).toBe(mood === "scared" ? "scared" : mood);
    }
    expect(impulse(CALM, "happy", MOOD_FAINT / 2, 7)).toBe(CALM);
  });

  it("lets a higher rank in at once, an equal one after the hold and a lower one only when it is much stronger", () => {
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(impulse(happy, "scared", 0.25, 1).mood).toBe("scared");
    expect(impulse(happy, "playful", 0.5, MOOD_HOLD - 1)).toBe(happy);
    expect(impulse(happy, "playful", 0.5, MOOD_HOLD).mood).toBe("playful");
    expect(impulse(happy, "curious", 0.75, MOOD_HOLD)).toBe(happy);
    expect(impulse(happy, "curious", 0.75 * MOOD_OVERRIDE, MOOD_HOLD)).toBe(happy);
    expect(impulse(happy, "curious", 1, MOOD_HOLD).mood).toBe("curious");
    expect(impulse(happy, "curious", 1, MOOD_HOLD - 1)).toBe(happy);
  });

  it("soothes with content whatever is worse than content, and never takes joy away", () => {
    const sad: Feeling = { mood: "sad", intensity: 0.75, since: 40 };
    expect(impulse(sad, "content", 0.25, 99)).toEqual({ mood: "sad", intensity: 0.5, since: 40 });
    expect(impulse(sad, "content", 0.75, 99)).toEqual({ mood: "sad", intensity: 0, since: 99 });
    expect(settled(impulse(sad, "content", 1, 99), "content", 99).mood).toBe("content");
    for (const mood of ["happy", "playful", "proud"] as const) {
      const joy: Feeling = { mood, intensity: 0.75, since: 0 };
      expect(impulse(joy, "content", 1, 500)).toBe(joy);
    }
    for (const mood of ["curious", "sleepy", "grumpy", "sad", "scared"] as const) expect(impulse({ mood, intensity: 0.75, since: 0 }, "content", 0.5, 1).intensity).toBe(0.25);
  });

  it("of nothing changes nothing", () => {
    const feeling: Feeling = { mood: "curious", intensity: 0.5, since: 3 };
    for (const mood of MOODS) for (const amount of [0, -0.5, Number.NaN]) expect(impulse(feeling, mood, amount, 500)).toBe(feeling);
  });

  it("keeps every intensity between 0 and 1 and never changes the feeling it is given", () => {
    const marks = units(1, FEELINGS * 6);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = frozen(drawn(marks, index * 6));
      const after = impulse(feeling, MOODS[Math.floor(marks[index * 6 + 3]! * MOODS.length)]!, marks[index * 6 + 4]! * 1.5, feeling.since + Math.floor(marks[index * 6 + 5]! * 400));
      expect(after.intensity >= 0 && after.intensity <= 1).toBe(true);
      expect(after.since).toBeGreaterThanOrEqual(feeling.since);
    }
  });

  it("answers the committed contest of ranks and the committed stories", () => {
    const contest = (tick: number): number[][] => MOODS.map((present) => MOODS.map((stirring) => (stirring !== present && impulse({ mood: present, intensity: 0.8, since: 0 }, stirring, 0.5, tick).mood === stirring ? 1 : 0)));
    for (const vector of FELT.priorities) agree({ during: contest(MOOD_HOLD / 2), after: contest(MOOD_HOLD) }, vector.expected, vector.id);
    for (const vector of FELT.impulses) {
      let feeling = vector.feeling;
      const feelings = vector.events.map((event) => (feeling = impulse(settled(feeling, vector.resting, event.tick), event.mood, event.amount, event.tick)));
      agree(feelings, vector.expected, vector.id);
    }
  });
});

describe("a feeling over time", () => {
  it("holds for two seconds, fades at the rate of its mood and gives way to the resting mood, which rises back to its rest", () => {
    const scared: Feeling = { mood: "scared", intensity: 0.5, since: 100 };
    expect(settled(scared, "content", 100 + MOOD_HOLD)).toBe(scared);
    expect(settled(scared, "content", 100 + MOOD_HOLD + 64)).toEqual({ mood: "scared", intensity: 0.5 - 0.15625, since: 164 });
    const gone = settled(scared, "content", 100 + MOOD_HOLD + 192);
    expect(gone.mood).toBe("content");
    expect(gone.intensity).toBeLessThan(MOOD_REST);
    expect(settled(scared, "content", 100000)).toEqual({ mood: "content", intensity: MOOD_REST, since: settlesAt(scared, "content") });
  });

  it("lets a pet stay twice as long in the mood it is prone to, down to its rest and no further", () => {
    const proud: Feeling = { mood: "proud", intensity: 0.75, since: 0 };
    const elsewhere = settled(proud, "content", MOOD_HOLD + 640);
    const prone = settled(proud, "proud", MOOD_HOLD + 640);
    expect(0.75 - prone.intensity).toBeCloseTo((0.75 - elsewhere.intensity) * PRONE_DECAY, 12);
    expect(settled(proud, "proud", 1000000)).toEqual({ mood: "proud", intensity: MOOD_REST, since: settlesAt(proud, "proud") });
    expect(settled({ mood: "proud", intensity: 0, since: 0 }, "proud", 64).intensity).toBe(MOOD_RISE);
  });

  it("shows its mood until no more than the rest level is left, and content from calmsAt on", () => {
    const marks = units(11, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      const calm = calmsAt(feeling, resting);
      const shown = (tick: number): Mood => {
        const present = settled(feeling, resting, tick);
        return shownMood(present.mood, present.intensity);
      };
      expect(shown(calm), `${JSON.stringify(feeling)} ${resting}`).toBe("content");
      if (calm > feeling.since) {
        expect(shown(calm - 1), `${JSON.stringify(feeling)} ${resting}`).toBe(feeling.mood);
        expect(shown(feeling.since)).toBe(feeling.mood);
      }
      for (const later of [1, 64, 6400]) expect(shown(calm + later)).toBe("content");
    }
    expect(calmsAt({ mood: "scared", intensity: 0.5, since: 100 }, "content")).toBe(100 + MOOD_HOLD + 103);
    expect(calmsAt({ mood: "proud", intensity: 0.75, since: 0 }, "proud")).toBe(settlesAt({ mood: "proud", intensity: 0.75, since: 0 }, "proud"));
    expect(calmsAt({ mood: "content", intensity: 0.9, since: 7 }, "happy")).toBe(7);
    expect(calmsAt({ mood: "grumpy", intensity: MOOD_REST, since: 9 }, "content")).toBe(9);
  });

  it("is at rest from the first tick for a pet that nothing has stirred", () => {
    for (const mood of MOODS) {
      const rest = atRest(mood, 40);
      expect(rest).toEqual({ mood, intensity: MOOD_REST, since: 40 });
      expect(settlesAt(rest, mood)).toBe(40);
      for (const tick of [0, 40, 41, 4000, 4000000]) expect(settled(rest, mood, tick)).toBe(rest);
    }
  });

  it("comes out the same however time is cut, and settling twice at one tick changes nothing", () => {
    const marks = units(2, FEELINGS * 6);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = frozen(drawn(marks, index * 6));
      const resting = index % 3 === 0 ? feeling.mood : MOODS[Math.floor(marks[index * 6 + 3]! * MOODS.length)]!;
      const middle = feeling.since + Math.floor(marks[index * 6 + 4]! * 9000);
      const end = middle + Math.floor(marks[index * 6 + 5]! * 9000);
      const direct = settled(feeling, resting, end);
      expect(settled(settled(feeling, resting, middle), resting, end), `${feeling.mood} ${feeling.intensity} ${resting} ${middle} ${end}`).toEqual(direct);
      expect(settled(direct, resting, end)).toEqual(direct);
      expect(direct.intensity >= 0 && direct.intensity <= 1).toBe(true);
    }
  });

  it("settles exactly at the tick settlesAt names, and not a tick sooner", () => {
    const marks = units(3, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = index % 3 === 0 ? feeling.mood : MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      const rest = settlesAt(feeling, resting);
      const there = settled(feeling, resting, rest);
      expect([there.mood, there.intensity], `${feeling.mood} ${feeling.intensity} ${resting}`).toEqual([resting, MOOD_REST]);
      const before = settled(feeling, resting, rest - 1);
      if (rest > feeling.since) expect(before.mood === resting && before.intensity === MOOD_REST, `${feeling.mood} ${feeling.intensity} ${resting}`).toBe(false);
    }
  });

  it("treats an intensity that is no number in [0, 1] as no feeling at all, and always answers", () => {
    for (const intensity of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 1.5, -0.5]) {
      for (const mood of ["happy", "content"] as const) {
        const broken: Feeling = { mood, intensity, since: 100 };
        expect(settlesAt(broken, "content")).toBe(100);
        expect(settled(broken, "content", 100)).toEqual({ mood: "content", intensity: MOOD_REST, since: 100 });
        expect(settled(broken, "content", 50).since).toBeLessThanOrEqual(100);
      }
    }
  });

  it("never grows by itself except back up to its rest", () => {
    const marks = units(4, FEELINGS * 4);
    for (let index = 0; index < FEELINGS; index++) {
      const feeling = drawn(marks, index * 4);
      const resting = MOODS[Math.floor(marks[index * 4 + 3]! * MOODS.length)]!;
      let last = feeling;
      for (let tick = feeling.since; tick < feeling.since + 12000; tick += 257) {
        const now = settled(feeling, resting, tick);
        if (now.mood === last.mood && now.intensity > last.intensity) expect(now.mood === resting && now.intensity <= MOOD_REST).toBe(true);
        last = now;
      }
    }
  });

  it("answers the committed feelings over their ticks and the committed cuts of time", () => {
    for (const vector of FELT.decays) agree({ views: vector.ticks.map((tick) => settled(vector.feeling, vector.resting, tick)), settles: settlesAt(vector.feeling, vector.resting) }, vector.expected, vector.id);
    for (const vector of FELT.jumps) {
      let stepped = vector.feeling;
      for (const cut of vector.cuts) stepped = settled(stepped, vector.resting, cut);
      const direct = settled(vector.feeling, vector.resting, vector.cuts[vector.cuts.length - 1]!);
      expect(stepped, vector.id).toEqual(direct);
      agree({ stepped, direct }, vector.expected, vector.id);
    }
  });
});

describe("the face of a feeling", () => {
  it("is the face of content for a content pet and for a mood of no intensity", () => {
    const content = faceOf(CALM);
    expect(content).toEqual({ bend: 0.3, lid: 0.1, slant: 0, drop: 0 });
    for (const mood of MOODS) expect(faceOf({ mood, intensity: 0, since: 0 })).toEqual(content);
    expect(spiritsOf(CALM)).toBe(0.3);
  });

  it("is the face of the mood at full strength, and stays inside what a face can show", () => {
    MOODS.forEach((mood, index) => {
      const face = faceOf({ mood, intensity: 1, since: 0 });
      agree(face, { bend: MOOD_VALENCES[index], lid: MOOD_LIDS[index], slant: MOOD_SLANTS[index], drop: MOOD_DROPS[index] }, mood);
      expect(valenceOf(mood)).toBe(MOOD_VALENCES[index]);
      for (const intensity of [0.1, 0.25, 0.5, 0.9]) {
        const part = faceOf({ mood, intensity, since: 0 });
        expect(Math.abs(part.bend) <= 1 && part.lid >= 0 && part.lid <= 1 && Math.abs(part.slant) <= 14 && Math.abs(part.drop) <= 2).toBe(true);
        expect(spiritsOf({ mood, intensity, since: 0 })).toBe(part.bend);
      }
    });
  });

  it("eases back without a jump when a mood fades", () => {
    for (const mood of MOODS) {
      const last = faceOf({ mood, intensity: MOOD_FAINT, since: 0 });
      const rest = faceOf({ mood: "content", intensity: 0, since: 0 });
      expect(Math.abs(last.bend - rest.bend)).toBeLessThan(0.06);
      expect(Math.abs(last.lid - rest.lid)).toBeLessThan(0.03);
      expect(Math.abs(last.slant - rest.slant)).toBeLessThan(0.8);
      expect(Math.abs(last.drop - rest.drop)).toBeLessThan(0.11);
    }
  });

  it("answers the committed faces", () => {
    for (const vector of FELT.faces) agree({ valence: valenceOf(vector.feeling.mood), spirits: spiritsOf(vector.feeling), face: faceOf(vector.feeling) }, vector.expected, vector.id);
  });
});

describe("the appraisal of what happens to a pet", () => {
  it("scales an impulse by the temperament and by the resting mood of the species", () => {
    const lively: Character = { mood: "playful", temperament: { energy: 1, sociability: 0, curiosity: 0.5 } };
    expect(pronenessOf(lively, "playful")).toBe(1.5 * PRONE_GAIN);
    expect(pronenessOf(lively, "sleepy")).toBe(0.5);
    expect(pronenessOf(lively, "grumpy")).toBe(1.5);
    expect(pronenessOf(lively, "happy")).toBe(0.5);
    expect(pronenessOf(lively, "curious")).toBe(1);
    expect(pronenessOf(lively, "scared")).toBe(1);
    expect(stirred(CALM, "playful", 0.5, lively, 9)).toEqual({ mood: "playful", intensity: 1, since: 9 });
    expect(stirred(CALM, "happy", 0.5, lively, 9)).toEqual({ mood: "happy", intensity: 0.25, since: 9 });
  });

  it("is never negative for any temperament", () => {
    for (const energy of [0, 0.5, 1]) for (const sociability of [0, 0.5, 1]) for (const curiosity of [0, 0.5, 1]) for (const mood of MOODS) expect(pronenessOf({ mood: "content", temperament: { energy, sociability, curiosity } }, mood)).toBeGreaterThanOrEqual(0);
  });

  it("makes a hello happy, a lift scary, a hard landing grumpy and a soft one under a parachute proud", () => {
    expect(appraised(CALM, "greeted", HOST, 5).mood).toBe("happy");
    expect(appraised(CALM, "lifted", HOST, 5).mood).toBe("scared");
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "shaken", HOST, 6).intensity).toBe(0.8);
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "dropped", HOST, 300).mood).toBe("grumpy");
    expect(appraised(appraised(CALM, "lifted", HOST, 5), "floated", HOST, 70).mood).toBe("proud");
    expect(appraised(CALM, "trampled", HOST, 5).mood).toBe("grumpy");
    expect(appraised(CALM, "drenched", HOST, 5).mood).toBe("sad");
    expect(appraised(CALM, "watched", HOST, 5).mood).toBe("curious");
  });

  it("ends fear, anger and sadness with a cuddle, and cheers a sad pet with two hellos", () => {
    for (const mood of ["scared", "grumpy", "sad"] as const) expect(appraised({ mood, intensity: 1, since: 0 }, "cuddled", HOST, 10).mood).toBe("happy");
    const sad: Feeling = { mood: "sad", intensity: 0.6, since: 0 };
    const once = appraised(sad, "greeted", HOST, 300);
    expect(once.mood).toBe("sad");
    expect(once.intensity).toBeLessThan(sad.intensity);
    expect(appraised(once, "greeted", HOST, 310).mood).toBe("happy");
  });

  it("lets a sulk end in peace and a sleeper wake up content", () => {
    const sulking = appraised(CALM, "squabbled", HOST, 0);
    expect(sulking).toEqual({ mood: "grumpy", intensity: 0.5, since: 0 });
    expect(appraised(sulking, "mended", { mood: "grumpy", temperament: HOST.temperament }, 400).intensity).toBeCloseTo(0.2, 12);
    expect(appraised(sulking, "mended", HOST, 400).intensity).toBeCloseTo(0.5 - 0.3 * PRONE_GAIN, 12);
    expect(settled(appraised({ mood: "sleepy", intensity: 0.9, since: 0 }, "woken", HOST, 50), "content", 50).mood).toBe("content");
  });

  it("leaves the mood a trick names, or playfulness when it names none", () => {
    expect(performed(CALM, { mood: "proud" } as Trick, HOST, 3)).toEqual({ mood: "proud", intensity: TRICK_AMOUNT, since: 3 });
    expect(performed(CALM, {} as Trick, HOST, 3)).toEqual({ mood: "playful", intensity: TRICK_AMOUNT, since: 3 });
  });

  it("lets sleepiness follow the energy need: a calm pet grows sleepy, a stirred one only when it is worn out", () => {
    expect(drowsed(CALM, DROWSY_ENERGY, 5)).toBe(CALM);
    expect(drowsed(CALM, 0.3, 5)).toEqual({ mood: "sleepy", intensity: 0.5, since: 5 });
    expect(drowsed({ mood: "sleepy", intensity: 0.5, since: 5 }, 0, 9)).toEqual({ mood: "sleepy", intensity: 1, since: 9 });
    const sleepy: Feeling = { mood: "sleepy", intensity: 0.75, since: 5 };
    expect(drowsed(sleepy, 0.3, 9)).toBe(sleepy);
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(drowsed(happy, 0.3, 1000)).toBe(happy);
    expect(drowsed(happy, 0, MOOD_HOLD - 1)).toBe(happy);
    expect(drowsed(happy, 0, 1000)).toEqual({ mood: "sleepy", intensity: 1, since: 1000 });
  });

  it("answers the committed characters", () => {
    for (const vector of FELT.appraisals) {
      const kinds: readonly (Mood | undefined)[] = [undefined, ...MOODS];
      agree(
        {
          proneness: MOODS.map((mood) => pronenessOf(vector.character, mood)),
          occasions: Object.fromEntries(OCCASIONS.map((occasion) => [occasion, vector.feelings.map((feeling) => appraised(feeling, occasion, vector.character, vector.tick))])),
          tricks: Object.fromEntries(kinds.map((mood) => [mood ?? "none", vector.feelings.map((feeling) => performed(feeling, { mood } as Trick, vector.character, vector.tick))])),
          drowsy: vector.feelings.map((feeling) => vector.energies.map((energy) => drowsed(feeling, energy, vector.tick))),
        },
        vector.expected,
        vector.id,
      );
    }
  });
});

describe("what a mood does to the wishes and the encounters of a pet", () => {
  it("multiplies nothing for a calm heart and the whole row at full strength", () => {
    for (const mood of MOODS) {
      expect(moodWeights(mood, 0)).toEqual(ACTIVITIES.map(() => 1));
      expect(moodWeights(mood, 1)).toEqual(MOOD_WEIGHTS[MOODS.indexOf(mood)]);
      expect(moodWeights(mood, 0.5).every((weight) => weight >= 0)).toBe(true);
    }
  });

  it("makes the playful perform, the sleepy sleep and the scared freeze", () => {
    const at = (weights: readonly number[], activity: Activity): number => weights[ACTIVITIES.indexOf(activity)]!;
    expect(at(moodWeights("playful", 1), "trick")).toBe(3);
    expect(at(moodWeights("playful", 0.5), "fidget")).toBe(1.5);
    expect(at(moodWeights("sleepy", 1), "sleep")).toBe(4);
    expect(at(moodWeights("scared", 1), "sleep")).toBe(0);
    expect(at(moodWeights("scared", 1), "trick")).toBe(0);
    expect(at(moodWeights("curious", 1), "climb")).toBe(2);
    expect(at(moodWeights("grumpy", 0.5), "idle")).toBe(1);
  });

  it("lets two grumpy pets squabble sooner and a scared one be comforted by a friend", () => {
    const share = (affinity: number, first: Feeling, second: Feeling, index: number): number => {
      const leaning = encounterBias(first, second);
      const shares = swayedShares(encounterShares(affinity + leaning.affinity), leaning);
      return shares[index]! / (shares[0]! + shares[1]! + shares[2]!);
    };
    const grumpy: Feeling = { mood: "grumpy", intensity: 0.9, since: 0 };
    const scared: Feeling = { mood: "scared", intensity: 0.9, since: 0 };
    const happy: Feeling = { mood: "happy", intensity: 0.9, since: 0 };
    expect(share(0, grumpy, grumpy, 2)).toBeGreaterThan(2 * share(0, CALM, CALM, 2));
    expect(share(0.3, grumpy, grumpy, 2)).toBeGreaterThan(0);
    expect(share(0.3, CALM, CALM, 2)).toBe(0);
    expect(share(0.5, scared, CALM, 1)).toBeGreaterThan(share(0.5, CALM, CALM, 1));
    expect(share(-0.2, scared, CALM, 2)).toBeLessThan(share(-0.2, CALM, CALM, 2));
    expect(share(0.2, happy, happy, 1)).toBeGreaterThan(share(0.2, CALM, CALM, 1));
  });

  it("lets the proud one show off: the prouder of two, the first of equals, nobody at rest", () => {
    const proud = (intensity: number): Feeling => ({ mood: "proud", intensity, since: 0 });
    expect(encounterBias(proud(0.6), CALM).show).toBe(0);
    expect(encounterBias(CALM, proud(0.6)).show).toBe(1);
    expect(encounterBias(proud(0.6), proud(0.9)).show).toBe(1);
    expect(encounterBias(proud(0.9), proud(0.9)).show).toBe(0);
    expect(encounterBias(proud(MOOD_REST), CALM).show).toBe(-1);
    expect(encounterBias(CALM, CALM)).toEqual({ affinity: 0, shares: [1, 1, 1], show: -1 });
  });

  it("answers the committed wishes and leanings", () => {
    for (const vector of FELT.wishes) agree(Object.fromEntries(MOODS.map((mood) => [mood, moodWeights(mood, vector.intensity)])), vector.expected, vector.id);
    for (const vector of FELT.leanings) {
      const leaning = encounterBias(vector.first, vector.second);
      agree({ leaning, swayed: swayedShares(vector.shares, leaning) }, vector.expected, vector.id);
    }
  });
});

describe("a mood that travels", () => {
  it("is adopted by a calm neighbour, weaker than where it came from, with a hold of its own", () => {
    const sleepy: Feeling = { mood: "sleepy", intensity: 1, since: 0 };
    expect(caught(CALM, sleepy, 1, 1, 64)).toEqual({ mood: "sleepy", intensity: 0.8, since: 64 });
    expect(caught(CALM, sleepy, 0, 0.5, 64)).toEqual({ mood: "sleepy", intensity: 0.2, since: 64 });
    expect(caught(CALM, sleepy, -0.6, 0.25, 64)).toBe(CALM);
  });

  it("lifts a neighbour in the same mood towards the stronger one and leaves its hold alone", () => {
    const mine: Feeling = { mood: "playful", intensity: 0.5, since: 30 };
    expect(caught(mine, { mood: "playful", intensity: 1, since: 0 }, 1, 1, 64)).toEqual({ mood: "playful", intensity: 0.8, since: 30 });
    expect(caught(mine, { mood: "playful", intensity: 0.4, since: 0 }, 1, 1, 64)).toBe(mine);
  });

  it("stays where it is when the neighbour is busy with a mood of its own, when it only rests, or when it does not spread", () => {
    const happy: Feeling = { mood: "happy", intensity: 0.75, since: 0 };
    expect(caught(happy, { mood: "scared", intensity: 1, since: 0 }, 1, 1, 64)).toBe(happy);
    expect(caught(CALM, { mood: "sleepy", intensity: MOOD_REST, since: 0 }, 1, 1, 64)).toBe(CALM);
    for (const mood of ["content", "proud", "sad"] as const) expect(caught(CALM, { mood, intensity: 1, since: 0 }, 1, 1, 64)).toBe(CALM);
    expect(caught(CALM, { mood: "playful", intensity: 1, since: 0 }, 1, 0, 64)).toBe(CALM);
  });

  it("never grows on the way", () => {
    const marks = units(5, FEELINGS * 8);
    for (let index = 0; index < FEELINGS; index++) {
      const mine = frozen(drawn(marks, index * 8));
      const theirs = frozen(drawn(marks, index * 8 + 3));
      const after = caught(mine, theirs, marks[index * 8 + 6]! * 1.6 - 0.6, marks[index * 8 + 7]!, 999);
      expect(after.intensity >= 0 && after.intensity <= 1).toBe(true);
      if (after !== mine) {
        expect(after.mood).toBe(theirs.mood);
        expect(after.intensity).toBeLessThanOrEqual(theirs.intensity);
      }
    }
  });

  it("answers the committed crowds beat by beat", () => {
    for (const vector of FELT.contagion) {
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
      agree(beats, vector.expected, vector.id);
    }
  });
});

describe("the states of a species", () => {
  it("last as long as they say and give way to the state they name", () => {
    const wick = kind("wick");
    expect(lastingTicks(wick.states[0]!)).toBe(0);
    expect(lastingTicks(wick.states[1]!)).toBe(640);
    expect(lastingTicks({ id: "blink", name: { en: "", de: "" }, lasts: 0.001 })).toBe(1);
    expect(stateEnds(wick, "burning", 100)).toBeNull();
    expect(stateEnds(wick, "high", 100)).toBe(740);
    expect(stateEnds(wick, "nowhere", 100)).toBeNull();
    expect(stateAt(wick, "high", 100, 739)).toEqual({ state: "high", since: 100 });
    expect(stateAt(wick, "high", 100, 740)).toEqual({ state: "ember", since: 740 });
    expect(stateAt(wick, "burning", 100, 100000000)).toEqual({ state: "burning", since: 100 });
    expect(stateAt(wick, "nowhere", 100, 5000)).toEqual({ state: "burning", since: 100 });
  });

  it("run round a circle of states for as long as anyone likes without walking every lap", () => {
    const wick = kind("wick");
    const lap = 640 + 384 + 96;
    expect(stateAt(wick, "high", 0, lap)).toEqual({ state: "high", since: lap });
    expect(stateAt(wick, "high", 0, lap * 1000000 + 700)).toEqual({ state: "ember", since: lap * 1000000 + 640 });
    expect(stateAt(wick, "sparked", 0, 32 + lap * 123456789 + 1030)).toEqual({ state: "flicker", since: 32 + lap * 123456789 + 1024 });
  });

  it("fall back to the resting state when a lasting state names no state of the species, and never give way to themselves", () => {
    const odd = { states: [{ id: "rest" }, { id: "flash", lasts: 1 }, { id: "lost", lasts: 1, then: "nowhere" }, { id: "loop", lasts: 1, then: "loop" }] } as unknown as Species;
    expect(stateAt(odd, "flash", 0, 64)).toEqual({ state: "rest", since: 64 });
    expect(stateAt(odd, "lost", 0, 640)).toEqual({ state: "rest", since: 64 });
    expect(stateAt(odd, "loop", 0, 640)).toEqual({ state: "loop", since: 0 });
    expect(stateEnds(odd, "loop", 0)).toBeNull();
    expect(stateAt({ states: [] } as unknown as Species, "any", 3, 9)).toEqual({ state: "any", since: 3 });
  });

  it("come out the same however time is cut", () => {
    for (const species of MENAGERIE.species) {
      for (const state of species.states) {
        for (const middle of [0, 31, 640, 1119, 1120, 5000, 77777]) {
          for (const end of [middle, middle + 1, middle + 999, middle + 123456]) {
            const half = stateAt(species, state.id, 100, 100 + middle);
            expect(stateAt(species, half.state, half.since, 100 + end), `${species.id} ${state.id} ${middle} ${end}`).toEqual(stateAt(species, state.id, 100, 100 + end));
          }
        }
      }
    }
  });

  it("answer the committed states over their ticks", () => {
    for (const vector of SET.states) {
      const species = kind(vector.species);
      const state = species.states.find((entry) => entry.id === vector.state);
      agree({ standings: vector.ticks.map((tick) => stateAt(species, vector.state, vector.since, tick)), ends: stateEnds(species, vector.state, vector.since), lasts: state === undefined ? 0 : lastingTicks(state) }, vector.expected, vector.id);
    }
  });
});

describe("the ladder of states and the tricks of a species", () => {
  it("is the order of the states as authored, stepped one rung at a time and held at both ends", () => {
    const mist = kind("mist");
    expect(ladderOf(mist)).toEqual(["fluffy", "heavy", "raining", "thin"]);
    expect(stepState(mist, "fluffy", 1)).toBe("heavy");
    expect(stepState(mist, "fluffy", -1)).toBe("fluffy");
    expect(stepState(mist, "thin", 1)).toBe("thin");
    expect(stepState(mist, "thin", -3)).toBe("raining");
    expect(stepState(mist, "heavy", 0)).toBe("heavy");
    expect(stepRung(["a", "b"], "c", 1)).toBe("c");
  });

  it("lets a circling trick name its own rungs, so a resting state can lie in the middle", () => {
    const glow = kind("glow");
    const up = glow.tricks.find((trick) => trick.id === "wind-up")!;
    const down = glow.tricks.find((trick) => trick.id === "wind-down")!;
    expect(ladderOf(glow)[0]).toBe("bright");
    expect(rungsOf(glow, up)).toEqual(["dusk", "dim", "bright", "blazing"]);
    expect(stateAfterTrick(glow, "bright", up)).toBe("blazing");
    expect(stateAfterTrick(glow, "blazing", up)).toBe("blazing");
    expect(stateAfterTrick(glow, "bright", down)).toBe("dim");
    expect(stateAfterTrick(glow, "dim", down)).toBe("dusk");
    expect(stateAfterTrick(glow, "dusk", down)).toBe("dusk");
    expect(rungsOf(glow, { ...up, from: ["nowhere"] })).toEqual(ladderOf(glow));
  });

  it("leaves the state a trick names, or the state as it was", () => {
    const wick = kind("wick");
    const trick = (id: Slug): Trick => wick.tricks.find((entry) => entry.id === id)!;
    expect(stateAfterTrick(wick, "burning", trick("whoosh"))).toBe("high");
    expect(stateAfterTrick(wick, "high", trick("blow-out"))).toBe("snuffed");
    expect(stateAfterTrick(wick, "ember", trick("dance"))).toBe("ember");
    expect(stateAfterTrick(wick, "ember", { ...trick("whoosh"), to: "nowhere" })).toBe("ember");
  });

  it("offers a trick for a cue in the states it names, and none of a pet's own to one that is sleepy, grumpy, sad or scared", () => {
    const glow = kind("glow");
    const ids = (tricks: readonly Trick[]): Slug[] => tricks.map((trick) => trick.id);
    expect(ids(tricksFor(glow, "click", "bright", CALM))).toEqual(["flare", "spark"]);
    expect(ids(tricksFor(glow, "click", "dim", CALM))).toEqual(["flare"]);
    expect(ids(tricksFor(glow, "whim", "bright", CALM))).toEqual(["flare", "shimmer"]);
    for (const mood of ["sleepy", "grumpy", "sad", "scared"] as const) {
      expect(tricksFor(glow, "whim", "bright", { mood, intensity: 0.6, since: 0 })).toEqual([]);
      expect(tricksFor(glow, "show", "bright", { mood, intensity: 0.6, since: 0 })).toEqual([]);
      expect(ids(tricksFor(glow, "whim", "bright", { mood, intensity: MOOD_REST, since: 0 }))).toEqual(["flare", "shimmer"]);
      expect(ids(tricksFor(glow, "shake", "bright", { mood, intensity: 0.6, since: 0 }))).toEqual(["sputter"]);
      expect(shownMood(mood, MOOD_REST)).toBe("content");
      expect(shownMood(mood, 0.26)).toBe(mood);
    }
  });

  it("plays the click tricks round and round and picks a whim in proportion, favouring the mood a pet shows", () => {
    const glow = kind("glow");
    expect([0, 1, 2, 3, -1].map((index) => clickTrick(glow, "bright", CALM, index)!.id)).toEqual(["flare", "spark", "flare", "spark", "spark"]);
    expect(clickTrick(kind("leaf"), "green", CALM, 5)!.id).toBe("unfurl");
    expect(clickTrick({ ...glow, tricks: [] }, "bright", CALM, 0)).toBeNull();
    const proud: Feeling = { mood: "proud", intensity: 0.6, since: 0 };
    expect(whimTrick(glow, "bright", CALM, 0.49)!.id).toBe("flare");
    expect(whimTrick(glow, "bright", CALM, 0.5)!.id).toBe("shimmer");
    expect(whimTrick(glow, "bright", proud, 0.74)!.id).toBe("flare");
    expect(whimTrick(glow, "bright", proud, 0.75)!.id).toBe("shimmer");
    expect(showTrick(glow, "bright", proud, 0.99)!.id).toBe("shimmer");
    expect(showTrick(kind("wick"), "burning", CALM, 0.5)).toBeNull();
    expect(showTrick(kind("wick"), "snuffed", CALM, 0.5)!.id).toBe("catch");
    expect(whimTrick(glow, "bright", { mood: "sad", intensity: 0.9, since: 0 }, 0.5)).toBeNull();
    const draws = units(6, sampled(300, 3000, 30000));
    const flares = draws.filter((unit) => whimTrick(glow, "bright", proud, unit)!.id === "flare").length;
    expect(Math.abs(flares / draws.length - WHIM_FAVOR / (WHIM_FAVOR + 1))).toBeLessThan(sampled(0.08, 0.03, 0.01));
    expect(weightedIndex([WHIM_FAVOR, 1], 0.74)).toBe(0);
  });

  it("answers the committed ladders and tricks", () => {
    for (const vector of SET.ladders) {
      const species = kind(vector.id);
      const names = ladderOf(species);
      agree(
        {
          ladder: names,
          rungs: Object.fromEntries(species.tricks.map((trick) => [trick.id, rungsOf(species, trick)])),
          steps: Object.fromEntries([...names, "nowhere"].map((name) => [name, { up: stepState(species, name, 1), down: stepState(species, name, -1), stay: stepState(species, name, 0) }])),
          after: Object.fromEntries(species.tricks.map((trick) => [trick.id, Object.fromEntries(names.map((name) => [name, stateAfterTrick(species, name, trick)]))])),
        },
        vector.expected,
        vector.id,
      );
    }
    for (const vector of SET.tricks) {
      const species = kind(vector.id);
      const names = ladderOf(species);
      agree(
        {
          offers: Object.fromEntries(CUES.map((cue) => [cue, Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, tricksFor(species, cue, name, feeling).map((trick) => trick.id)]))]))])),
          clicks: Object.fromEntries(names.map((name) => [name, vector.clicks.map((index) => clickTrick(species, name, CALM, index)?.id ?? null)])),
          whims: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => whimTrick(species, name, feeling, unit)?.id ?? null)]))])),
          shows: Object.fromEntries(names.map((name) => [name, Object.fromEntries(vector.feelings.map((feeling) => [feeling.id, vector.units.map((unit) => showTrick(species, name, feeling, unit)?.id ?? null)]))])),
        },
        vector.expected,
        vector.id,
      );
    }
  });

  it("reaches every state of every committed species by its tricks, its lasting states or a reaction", () => {
    for (const vector of SET.reachability) {
      const species = kind(vector.id);
      expect(vector.expected.unreachable, vector.id).toEqual([]);
      expect([...vector.expected.reachable].sort(), vector.id).toEqual([...ladderOf(species)].sort());
      for (const [start, end] of vector.expected.edges) {
        const byTrick = species.tricks.some((trick) => (trick.from === undefined || trick.from.includes(start)) && stateAfterTrick(species, start, trick) === end);
        const byTime = stateEnds(species, start, 0) !== null && stateAt(species, start, 0, stateEnds(species, start, 0)!).state === end;
        const byReaction = MENAGERIE.chemistry.some((reaction) => reaction.then.some((effect) => effect.state === end && (reaction[effect.on].species ?? species.id) === species.id &&(reaction[effect.on].state ?? start) === start));
        expect(byTrick || byTime || byReaction, `${vector.id}: ${start} → ${end}`).toBe(true);
      }
    }
  });
});

describe("where two bodies are to each other", () => {
  const box = (x: number, y: number, width: number, height: number): Sighting => ({ species: "glow", state: "bright", held: 0, mood: "content", intensity: MOOD_REST, activity: "idle", trick: null, x, y, width, height });

  it("measures the gap between the boxes, across and upright together", () => {
    const pet = box(100, 300, 40, 30);
    expect(nearby(pet, box(140, 300, 40, 30), 0)).toBe(true);
    expect(nearby(pet, box(150, 300, 40, 30), 9.5)).toBe(false);
    expect(nearby(pet, box(150, 300, 40, 30), 10)).toBe(true);
    expect(nearby(pet, box(100, 230, 40, 30), 39.5)).toBe(false);
    expect(nearby(pet, box(100, 230, 40, 30), 40)).toBe(true);
    expect(nearby(pet, box(170, 230, 40, 30), 49.5)).toBe(false);
    expect(nearby(pet, box(170, 230, 40, 30), 50)).toBe(true);
  });

  it("calls the first above or below the second when they share a column, beside it when they share only a row", () => {
    const pet = box(100, 300, 40, 30);
    const over = box(110, 240, 40, 30);
    expect([seen(over, pet, "above"), seen(over, pet, "below"), seen(over, pet, "beside"), seen(over, pet, "any")]).toEqual([true, false, false, true]);
    expect([seen(pet, over, "above"), seen(pet, over, "below"), seen(pet, over, "beside")]).toEqual([false, true, false]);
    const neighbour = box(150, 295, 40, 30);
    expect([seen(neighbour, pet, "above"), seen(neighbour, pet, "below"), seen(neighbour, pet, "beside")]).toEqual([false, false, true]);
    const diagonal = box(170, 240, 40, 30);
    expect([seen(diagonal, pet, "above"), seen(diagonal, pet, "below"), seen(diagonal, pet, "beside"), seen(diagonal, pet, "any")]).toEqual([false, false, false, true]);
  });

  it("is symmetric where it should be: near and beside both ways, above one way and below the other", () => {
    const marks = units(7, STAGES * 8);
    for (let index = 0; index < STAGES; index++) {
      const place = (offset: number): Sighting => box(Math.floor(marks[offset]! * 400) / 2, Math.floor(marks[offset + 1]! * 400) / 2, 10 + Math.floor(marks[offset + 2]! * 60), 10 + Math.floor(marks[offset + 3]! * 60));
      const first = place(index * 8);
      const second = place(index * 8 + 4);
      for (const reach of [0, 10, 50, 200]) expect(nearby(first, second, reach)).toBe(nearby(second, first, reach));
      expect(seen(first, second, "beside")).toBe(seen(second, first, "beside"));
      expect(seen(first, second, "above")).toBe(seen(second, first, "below"));
      expect(seen(first, second, "above") && seen(first, second, "below")).toBe(false);
      expect(seen(first, second, "beside") && (seen(first, second, "above") || seen(first, second, "below"))).toBe(false);
    }
  });

  it("answers the committed pairs", () => {
    for (const vector of SET.relations) agree({ nearby: vector.reaches.map((reach) => nearby(vector.first, vector.second, reach)), above: seen(vector.first, vector.second, "above"), below: seen(vector.first, vector.second, "below"), beside: seen(vector.first, vector.second, "beside"), any: seen(vector.first, vector.second, "any") }, vector.expected, vector.id);
  });
});

describe("the chemistry of a menagerie", () => {
  const eclipse = (): Sighting[] => [sight("mist", "heavy", 100, { y: 190 }), sight("glow", "bright", 100)];
  const happened = (chemistry: Chemistry): Slug[] => chemistry.consequences.map((consequence) => consequence.reaction);
  const odd = (id: Slug, then: Reaction["then"]): Reaction => ({ id, when: { species: "leaf" }, near: { species: "wick" }, within: 100, every: 1, then });

  it("asks an actor for the species, the state and how long it has held it, the activity, the trick and the mood it shows", () => {
    const actor = sight("glow", "dim", 0, { held: 320, mood: "grumpy", intensity: 0.5, activity: "trick", trick: "flare" });
    expect(matches({ species: "glow" }, actor)).toBe(true);
    expect(matches({}, actor)).toBe(true);
    expect(matches({ state: "dim", mood: "grumpy" }, actor)).toBe(true);
    expect(matches({ species: "mist" }, actor)).toBe(false);
    expect(matches({ species: "glow", state: "dim", mood: "grumpy", activity: "trick", trick: "flare" }, actor)).toBe(true);
    expect(matches({ species: "glow", trick: "spark" }, actor)).toBe(false);
    expect(matches({ species: "glow", trick: "flare" }, { ...actor, activity: "idle", trick: null })).toBe(false);
    expect(matches({ species: "glow", state: "bright" }, actor)).toBe(false);
    expect(matches({ species: "glow", activity: "idle" }, actor)).toBe(false);
    expect(matches({ species: "glow", mood: "content" }, actor)).toBe(false);
    expect(matches({ species: "glow", mood: "grumpy" }, { ...actor, intensity: MOOD_REST })).toBe(false);
    expect(matches({ species: "glow", mood: "content" }, { ...actor, intensity: MOOD_REST })).toBe(true);
    expect(matches({ state: "dim", held: 5 }, actor)).toBe(true);
    expect(matches({ state: "dim", held: 5 }, { ...actor, held: 319 })).toBe(false);
    expect([heldTicks(5), heldTicks(0.001), heldTicks(1.5)]).toEqual([320, 1, 96]);
  });

  it("finds what is due in pair order and applies it: a state, moods with their amounts, a shift of the rapport", () => {
    const beat = reactionsOf(MENAGERIE, eclipse(), 0, [], [], []);
    expect(beat.consequences).toEqual<Consequence[]>([
      { reaction: "eclipse", on: "glow", other: "mist", state: "dim", mood: "grumpy", amount: 0.5, rapport: 0, encounter: null, trick: null, activity: null },
      { reaction: "eclipse", on: "mist", other: "glow", state: null, mood: "playful", amount: EFFECT_AMOUNT, rapport: 0, encounter: null, trick: null, activity: null },
      { reaction: "eclipse", on: "mist", other: "glow", state: null, mood: null, amount: 0, rapport: -0.05, encounter: null, trick: null, activity: null },
    ]);
    expect(beat.coolings).toEqual<Cooling[]>([{ reaction: "eclipse", when: "mist", near: "glow", until: 1280 }]);
    expect(beat.drawn).toBe(0);
    expect(everyTicks(MENAGERIE.chemistry[0]!)).toBe(1280);
  });

  it("lets a reaction cool for its pair, and tries it again when its time is up", () => {
    const first = reactionsOf(MENAGERIE, eclipse(), 0, [], [], []);
    expect(reactionsOf(MENAGERIE, eclipse(), 1279, first.coolings, [], []).consequences).toEqual([]);
    expect(reactionsOf(MENAGERIE, eclipse(), 1279, first.coolings, [], []).coolings).toEqual(first.coolings);
    const again = reactionsOf(MENAGERIE, eclipse(), 1280, first.coolings, [], []);
    expect(happened(again)).toEqual(["eclipse", "eclipse", "eclipse"]);
    expect(again.coolings).toEqual([{ reaction: "eclipse", when: "mist", near: "glow", until: 2560 }]);
  });

  it("takes one unit per reaction with a chance, in order, and counts a missing unit as bad luck", () => {
    const stage = [sight("glow", "blazing", 100), sight("wick", "snuffed", 150)];
    const trials = trialsOf(MENAGERIE, stage, [], 0, []);
    expect(trials).toEqual<Trial[]>([{ reaction: 4, when: 0, near: 1, chancy: true }]);
    expect(drawsOf(trials)).toBe(1);
    expect(drawsOf(trialsOf(MENAGERIE, eclipse(), [], 0, []))).toBe(0);
    const lucky = reactionsOf(MENAGERIE, stage, 0, [], [0.499, 0], []);
    expect(happened(lucky)).toEqual(["relight", "relight", "relight"]);
    expect(lucky.drawn).toBe(1);
    for (const chances of [[0.5], [0.9], []]) {
      const unlucky = reactionsOf(MENAGERIE, stage, 0, [], chances, []);
      expect(unlucky.consequences).toEqual([]);
      expect(unlucky.drawn).toBe(1);
      expect(unlucky.coolings).toEqual([{ reaction: "relight", when: "glow", near: "wick", until: 1920 }]);
    }
  });

  it("applies nothing twice in a beat: the first state, mood, trick and activity an actor gets, the first shift and promise a pair gets", () => {
    const stage = [...eclipse(), sight("leaf", "lush", 160)];
    const beat = reactionsOf(MENAGERIE, stage, 0, [], [], []);
    expect(happened(beat)).toEqual(["eclipse", "eclipse", "eclipse", "old-friends", "old-friends"]);
    expect(beat.coolings.map((cooling) => cooling.reaction)).toEqual(["eclipse", "dazzle", "old-friends"]);
    expect(beat.consequences.slice(3).map((consequence) => [consequence.on, consequence.activity, consequence.encounter])).toEqual([
      ["leaf", "walk", null],
      ["glow", null, "cuddle"],
    ]);
    const sleeper = [sight("leaf", "green", 100, { activity: "sleep" }), sight("mist", "fluffy", 160, { y: 250 })];
    const lulled = reactionsOf(MENAGERIE, sleeper, 0, [], [], []).consequences;
    expect(lulled.map((consequence) => [consequence.reaction, consequence.on, consequence.mood, consequence.rapport])).toEqual([
      ["lullaby", "mist", "sleepy", 0],
      ["lullaby", "leaf", null, 0.02],
    ]);
    const twice = { ...MENAGERIE, chemistry: [odd("first", [{ on: "near", activity: "sleep" }, { on: "near", activity: "walk" }]), odd("second", [{ on: "near", activity: "hop" }])] };
    expect(reactionsOf(twice, [sight("leaf", "green", 84), sight("wick", "burning", 122)], 0, [], [], []).consequences.map((consequence) => consequence.activity)).toEqual(["sleep"]);
  });

  it("judges everybody as it stood when the beat began", () => {
    const stage = [sight("mist", "raining", 100, { y: 200 }), sight("leaf", "green", 84), sight("wick", "burning", 122)];
    const beat = reactionsOf(MENAGERIE, stage, 0, [], [], []);
    expect(happened(beat)).toEqual(["soak", "downpour", "downpour", "downpour"]);
    expect(beat.coolings.map((cooling) => cooling.reaction)).toEqual(["soak", "downpour", "encore"]);
    expect(beat.consequences.filter((consequence) => consequence.trick !== null)).toEqual([]);
  });

  it("drops a state the species does not have and a trick it does not offer in its state", () => {
    const stage = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    const tried = (then: Reaction["then"]): Chemistry => reactionsOf({ ...MENAGERIE, chemistry: [odd("odd", then)] }, stage, 0, [], [], []);
    expect(tried([{ on: "near", state: "frozen" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", trick: "juggle" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", trick: "catch" }]).consequences).toEqual([]);
    expect(tried([{ on: "near", rapport: 0 }]).consequences).toEqual([]);
    expect(tried([{ on: "near", state: "frozen", trick: "dance", mood: "happy", amount: 0 }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "wick", other: "leaf", state: null, mood: "happy", amount: 0, rapport: 0, encounter: null, trick: "dance", activity: null }]);
    expect(tried([{ on: "when", encounter: "cuddle" }, { on: "near", encounter: "squabble" }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "leaf", other: "wick", state: null, mood: null, amount: 0, rapport: 0, encounter: "cuddle", trick: null, activity: null }]);
    expect(tried([{ on: "when", activity: "fidget" }]).consequences).toEqual<Consequence[]>([{ reaction: "odd", on: "leaf", other: "wick", state: null, mood: null, amount: 0, rapport: 0, encounter: null, trick: null, activity: "fidget" }]);
    expect(tried([{ on: "near", state: "high" }]).coolings).toEqual([{ reaction: "odd", when: "leaf", near: "wick", until: 64 }]);
  });

  it("lets a side of any species match whoever stands there, the cooling still kept per pair of species", () => {
    const anyone = { ...MENAGERIE, chemistry: [{ ...odd("anyone", [{ on: "near", mood: "happy" }]), when: { mood: "playful" as const }, near: {} }] };
    const stage = [sight("leaf", "green", 84, { mood: "playful", intensity: 0.6 }), sight("wick", "burning", 122), sight("glow", "bright", 40)];
    const trials = trialsOf(anyone, stage, [], 0, []);
    expect(trials.map((trial) => [stage[trial.when]!.species, stage[trial.near]!.species])).toEqual([
      ["leaf", "glow"],
      ["leaf", "wick"],
    ]);
    const beat = reactionsOf(anyone, stage, 0, [], [], []);
    expect(beat.coolings).toEqual([
      { reaction: "anyone", when: "leaf", near: "glow", until: 64 },
      { reaction: "anyone", when: "leaf", near: "wick", until: 64 },
    ]);
    expect(trialsOf(anyone, stage, beat.coolings, 32, [])).toEqual([]);
  });

  it("holds a reaction back while a third actor that matches unless stands near the second", () => {
    const sheltered = { ...MENAGERIE, chemistry: [{ ...odd("shelter", [{ on: "near", mood: "scared" }]), unless: { species: "glow", state: "bright" }, within: 40 }] };
    const pair = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    expect(trialsOf(sheltered, pair, [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 182)], [], 0, [])).toEqual([]);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 200)], [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "dim", 182)], [], 0, [])).toHaveLength(1);
    expect(trialsOf(sheltered, [...pair, sight("glow", "bright", 30)], [], 0, [])).toHaveLength(1);
  });

  it("asks the affinity of the two — the authored bond moved by their rapport — to lie within the bounds", () => {
    const fond = (low: number, high: number): Menagerie => ({ ...MENAGERIE, chemistry: [{ ...odd("fond", [{ on: "near", encounter: "cuddle" }]), affinity: [low, high] }] });
    const pair = [sight("leaf", "green", 84), sight("wick", "burning", 122)];
    expect(trialsOf(fond(0.3, 1), pair, [], 0, [])).toHaveLength(1);
    expect(trialsOf(fond(0.31, 1), pair, [], 0, [])).toEqual([]);
    expect(trialsOf(fond(0.31, 1), pair, [], 0, [{ between: ["wick", "leaf"], drift: 0.25 }])).toHaveLength(1);
    expect(trialsOf(fond(-1, 0.29), pair, [], 0, [])).toEqual([]);
    expect(trialsOf(fond(-1, 0), pair, [], 0, [{ between: ["leaf", "wick"], drift: -0.5 }])).toHaveLength(1);
  });

  it("orders the trials by the species of the menagerie, not by how the actors are listed", () => {
    const marks = units(8, STAGES * 4);
    for (let index = 0; index < Math.min(STAGES, SET.matching.length * 4); index++) {
      const vector = SET.matching[index % SET.matching.length]!;
      const turn = 1 + Math.floor(marks[index]! * (vector.sightings.length - 1));
      const turned = [...vector.sightings.slice(turn), ...vector.sightings.slice(0, turn)];
      const name = (stage: readonly Sighting[], trials: readonly Trial[]): string[] => trials.map((trial) => `${MENAGERIE.chemistry[trial.reaction]!.id}:${stage[trial.when]!.species}:${stage[trial.near]!.species}`);
      const once = new Set(vector.sightings.map((sighting) => sighting.species)).size === vector.sightings.length;
      if (once) expect(name(turned, trialsOf(MENAGERIE, turned, vector.coolings, vector.tick, vector.rapports)), vector.id).toEqual(name(vector.sightings, trialsOf(MENAGERIE, vector.sightings, vector.coolings, vector.tick, vector.rapports)));
    }
  });

  it("never changes what it is given", () => {
    const menagerie = frozen(structuredClone(MENAGERIE));
    for (const vector of SET.reactions) {
      let coolings: readonly Cooling[] = frozen([]);
      for (const beat of vector.beats) coolings = frozen(reactionsOf(menagerie, frozen(structuredClone(beat.sightings)), beat.tick, coolings, frozen([...beat.units]), frozen(structuredClone(beat.rapports))).coolings as Cooling[]);
    }
  });

  it("answers the committed stages and stories", () => {
    for (const vector of SET.matching) agree(trialsOf(MENAGERIE, vector.sightings, vector.coolings, vector.tick, vector.rapports), vector.expected, vector.id);
    for (const vector of SET.reactions) {
      const beats: Chemistry[] = [];
      let coolings: readonly Cooling[] = [];
      for (const beat of vector.beats) {
        const outcome = reactionsOf(MENAGERIE, beat.sightings, beat.tick, coolings, beat.units, beat.rapports);
        coolings = outcome.coolings;
        beats.push(outcome);
      }
      agree(beats, vector.expected, vector.id);
    }
  });
});

describe("the arithmetic of the module", () => {
  it("uses nothing the Rust twin cannot reproduce bit for bit, and never the console", () => {
    const source = readFileSync(resolve(HERE, "../../🟦️.ts"), "utf8");
    expect(source).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
    expect(source).not.toMatch(/\b(Date|performance|console)\b/);
    expect(source).not.toMatch(/\[DEBUG\]/);
  });
});
