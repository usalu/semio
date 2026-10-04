/** 🧠️ Unit suite of the behaviour module: what the modes allow, what an idle pet feels like doing, how long things last, how encounters turn out, how bonds and drives move, who is on stage, the shared vectors of both Protocol v2 cases, and the arithmetic the core is allowed to use.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧫️fixtures/🧠️behavior-choice/🔣️.json
 * @see ../../../../🧫️fixtures/🤝️bond-dynamics/🔣️.json
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACTIVITIES, PET_MODES, TICKS_PER_SECOND, type Activity, type Actor, type Bond, type Cast, type Menagerie, type Needs, type PetMode, type Rapport, type Species, type Temperament } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { CAST_STREAM, randomPick, randomUnit, randomWords, weightedIndex } from "../../../🎲️randomness/🟦️.ts";
import { AFFINITY_FLOOR, ENCOUNTERS, MODE_LIMITS, RAPPORT_SPAN, type Situation, activityWeights, affinityOf, castOf, dwellOf, encounterOf, encounterShares, followersOf, needsAfter, needsOf, rapportAfter, rapportFaded } from "../../🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TOLERANCE = 1e-9;
const SECOND = TICKS_PER_SECOND;

/** 🎯️ How many keyed draws a weighted pick is compared on at the level of the run. */
const DRAWS = sampled(100, 500, 5000);

/** 🪜️ Into how many steps the unit of a draw is cut where the range of an idle dwell is swept at the level of the run. */
const STEPS = sampled(40, 200, 2000);

/** 🧗️ Into how many steps the unit of a draw is cut where a dwell must not shrink, for every activity in every mode, at the level of the run. */
const RISES = sampled(16, 64, 512);

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
}

type Circumstance = { readonly id: string; readonly mode: PetMode; readonly quiet: boolean; readonly movers: number; readonly fidgeters: number; readonly roam: boolean; readonly hops: boolean; readonly crowd: number; readonly watched: boolean; readonly whims: boolean; readonly fidgets: boolean; readonly needs: Needs };
type ChoiceVectors = {
  readonly limits: readonly { readonly id: PetMode; readonly expected: unknown }[];
  readonly weights: readonly (Circumstance & { readonly expected: readonly number[] })[];
  readonly picks: readonly { readonly id: string; readonly weights: readonly number[]; readonly units: readonly number[]; readonly expected: readonly number[] }[];
  readonly decisions: readonly (Circumstance & { readonly seed: number; readonly stream: number; readonly count: number; readonly expected: { readonly activities: readonly Activity[] } })[];
  readonly dwells: readonly { readonly id: string; readonly activity: Activity; readonly mode: PetMode; readonly units: readonly number[]; readonly expected: readonly number[] }[];
  readonly encounters: readonly { readonly id: string; readonly affinity: number; readonly units: readonly number[]; readonly expected: { readonly shares: readonly number[]; readonly kinds: readonly string[] } }[];
  readonly graph: readonly { readonly id: string; readonly expected: { readonly followers: Record<string, readonly Activity[]>; readonly reachable: Record<string, readonly Activity[]>; readonly components: number } }[];
  readonly casts: readonly { readonly id: string; readonly cast: Cast; readonly capacity: number; readonly seed: number; readonly epochs: readonly number[]; readonly expected: readonly (readonly string[])[] }[];
};
type BondVectors = {
  readonly affinities: readonly { readonly id: string; readonly bonds: readonly Bond[]; readonly rapports: readonly Rapport[]; readonly pairs: readonly (readonly [string, string])[]; readonly expected: readonly number[] }[];
  readonly rapportSteps: readonly { readonly id: string; readonly drift: number; readonly expected: Record<string, number> }[];
  readonly rapportFading: readonly { readonly id: string; readonly drift: number; readonly ticks: readonly number[]; readonly expected: readonly number[] }[];
  readonly histories: readonly { readonly id: string; readonly affinity: number; readonly events: readonly { readonly after: number; readonly activity: Activity }[]; readonly expected: { readonly drifts: readonly number[]; readonly affinities: readonly number[] } }[];
  readonly needs: readonly { readonly id: string; readonly needs: Needs; readonly activity: Activity; readonly ticks: number; readonly temperament: Temperament; readonly expected: Needs }[];
  readonly days: readonly { readonly id: string; readonly temperament: Temperament; readonly spans: readonly { readonly activity: Activity; readonly ticks: number }[]; readonly expected: readonly Needs[] }[];
};

const CHOICE = fixture<ChoiceVectors>("🧠️behavior-choice");
const BONDS = fixture<BondVectors>("🤝️bond-dynamics");
const RESTED: Needs = { energy: 0.9, sociability: 0.5, curiosity: 0.6 };
const OPEN: Situation = { mode: "calm", quiet: false, movers: 0, fidgeters: 0, roam: true, hops: true, crowd: 0, watched: false, whims: false };

/** 🧸️ An actor with nothing but needs. */
function feeling(needs: Needs): Actor {
  return { needs } as Actor;
}

/** 🧬️ A species with or without a fidget clip. */
function kind(fidgets: boolean): Species {
  return { repertoire: fidgets ? { fidget: ["fidget"] } : {} } as Species;
}

/** ⚖️ The weights of a committed situation. */
function weighed(vector: Circumstance): number[] {
  return activityWeights(feeling(vector.needs), kind(vector.fidgets), { mode: vector.mode, quiet: vector.quiet, movers: vector.movers, fidgeters: vector.fidgeters, roam: vector.roam, hops: vector.hops, crowd: vector.crowd, watched: vector.watched, whims: vector.whims });
}

/** 🏷️ A weight by the name of its activity. */
function weightOf(weights: readonly number[], activity: Activity): number {
  return weights[ACTIVITIES.indexOf(activity)]!;
}

/** 🎪️ A menagerie that carries nothing but bonds. */
function bonded(bonds: readonly Bond[]): Menagerie {
  return { bonds } as Menagerie;
}

describe("mode limits", () => {
  it("still allows nothing at all", () => {
    expect(Object.values(MODE_LIMITS.still).every((value) => value === 0)).toBe(true);
  });

  it("calm lets one actor move and dwells 6 to 20 seconds, lively two and 3 to 10 seconds", () => {
    expect(MODE_LIMITS.calm.movers).toBe(1);
    expect(MODE_LIMITS.lively.movers).toBe(2);
    expect([MODE_LIMITS.calm.idleLow, MODE_LIMITS.calm.idleHigh]).toEqual([6 * SECOND, 20 * SECOND]);
    expect([MODE_LIMITS.lively.idleLow, MODE_LIMITS.lively.idleHigh]).toEqual([3 * SECOND, 10 * SECOND]);
  });

  it("keeps encounters at least 90 seconds apart in calm and 30 seconds in lively", () => {
    expect(MODE_LIMITS.calm.encounterGap).toBeGreaterThanOrEqual(90 * SECOND);
    expect(MODE_LIMITS.lively.encounterGap).toBeGreaterThanOrEqual(30 * SECOND);
    expect(MODE_LIMITS.calm.encounterRate).toBeLessThan(MODE_LIMITS.lively.encounterRate);
  });

  it("makes calm calmer than lively in every weight", () => {
    for (const weight of ["fidget", "walk", "hop", "whim"] as const) expect(MODE_LIMITS.calm[weight]).toBeLessThan(MODE_LIMITS.lively[weight]);
    expect(MODE_LIMITS.calm.stroll).toBeLessThanOrEqual(MODE_LIMITS.lively.stroll);
  });

  it("has limits for every mode of the schema", () => {
    for (const mode of PET_MODES) expect(MODE_LIMITS[mode]).toBeDefined();
  });
});

describe("activityWeights", () => {
  it("answers in ACTIVITIES order and only ever lets an idle pet choose idle, fidget, walk, hop, sleep or a trick on a whim", () => {
    const weights = activityWeights(feeling({ energy: 0.3, sociability: 0.5, curiosity: 0.6 }), kind(true), { ...OPEN, whims: true });
    expect(weights).toHaveLength(ACTIVITIES.length);
    const chosen: readonly Activity[] = ["idle", "fidget", "walk", "hop", "sleep", "trick"];
    for (const activity of ACTIVITIES) if (!chosen.includes(activity)) expect(weightOf(weights, activity), activity).toBe(0);
    for (const activity of chosen) expect(weightOf(weights, activity)).toBeGreaterThan(0);
    expect(ACTIVITIES.slice(11)).toEqual(["hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"]);
  });

  it("is slightly active by default: a rested pet in calm most likely stays idle, then fidgets, then walks", () => {
    const weights = activityWeights(feeling(RESTED), kind(true), OPEN);
    const total = weights.reduce((left, right) => left + right, 0);
    expect(weightOf(weights, "idle") / total).toBeGreaterThan(0.5);
    expect(weightOf(weights, "fidget")).toBeGreaterThan(weightOf(weights, "walk"));
    expect(weightOf(weights, "walk")).toBeGreaterThan(weightOf(weights, "hop"));
    expect(weightOf(weights, "sleep")).toBe(0);
  });

  it("is busier in lively than in calm", () => {
    const calm = activityWeights(feeling(RESTED), kind(true), OPEN);
    const lively = activityWeights(feeling(RESTED), kind(true), { ...OPEN, mode: "lively" });
    for (const activity of ["fidget", "walk", "hop"] as const) expect(weightOf(lively, activity)).toBeGreaterThan(weightOf(calm, activity));
  });

  it("leaves a still pet nothing but idle", () => {
    expect(activityWeights(feeling({ energy: 0, sociability: 1, curiosity: 1 }), kind(true), { ...OPEN, mode: "still", whims: true })).toEqual(ACTIVITIES.map((activity) => (activity === "idle" ? 1 : 0)));
  });

  it("lets a pet perform on a whim only when a trick of its own is on offer: rarely in calm, more often in lively, by its drive", () => {
    const whim = (mode: PetMode, energy: number, whims: boolean): number => weightOf(activityWeights(feeling({ energy, sociability: 0.5, curiosity: 0.6 }), kind(true), { ...OPEN, mode, whims }), "trick");
    expect(whim("calm", 0.9, false)).toBe(0);
    expect(whim("calm", 0.9, true)).toBe(MODE_LIMITS.calm.whim * (0.5 + 0.5 * 0.9));
    expect(whim("lively", 0.9, true)).toBeGreaterThan(whim("calm", 0.9, true));
    expect(whim("lively", 0.2, true)).toBeLessThan(whim("lively", 0.9, true));
    expect(whim("still", 0.9, true)).toBe(0);
    const total = activityWeights(feeling(RESTED), kind(true), { ...OPEN, whims: true }).reduce((left, right) => left + right, 0);
    expect(whim("calm", 0.9, true) / total).toBeLessThan(0.03);
  });

  it("leaves a quiet pet only idle and sleep", () => {
    const rested = activityWeights(feeling(RESTED), kind(true), { ...OPEN, quiet: true, whims: true });
    expect(rested).toEqual(ACTIVITIES.map((activity) => (activity === "idle" ? 1 : 0)));
    const tired = activityWeights(feeling({ energy: 0.3, sociability: 0.5, curiosity: 0.6 }), kind(true), { ...OPEN, quiet: true });
    expect(weightOf(tired, "sleep")).toBeGreaterThan(0);
    expect(weightOf(tired, "sleep")).toBeCloseTo(3 * weightOf(activityWeights(feeling({ energy: 0.3, sociability: 0.5, curiosity: 0.6 }), kind(true), OPEN), "sleep"), 12);
    expect(tired.filter((weight) => weight > 0)).toHaveLength(2);
  });

  it("lets nobody walk or hop while the mode's movers are busy, and nobody fidget while its fidgeters are", () => {
    const busy = activityWeights(feeling(RESTED), kind(true), { ...OPEN, movers: 1 });
    expect([weightOf(busy, "walk"), weightOf(busy, "hop")]).toEqual([0, 0]);
    expect(weightOf(busy, "fidget")).toBeGreaterThan(0);
    const fidgety = activityWeights(feeling(RESTED), kind(true), { ...OPEN, fidgeters: 1 });
    expect(weightOf(fidgety, "fidget")).toBe(0);
    const lively = activityWeights(feeling(RESTED), kind(true), { ...OPEN, mode: "lively", movers: 1, fidgeters: 1 });
    expect(weightOf(lively, "walk")).toBeGreaterThan(0);
    expect(weightOf(lively, "fidget")).toBeGreaterThan(0);
  });

  it("needs room to walk, a perch in reach to hop and a clip to fidget", () => {
    expect(weightOf(activityWeights(feeling(RESTED), kind(true), { ...OPEN, roam: false }), "walk")).toBe(0);
    expect(weightOf(activityWeights(feeling(RESTED), kind(true), { ...OPEN, hops: false }), "hop")).toBe(0);
    expect(weightOf(activityWeights(feeling(RESTED), kind(false), OPEN), "fidget")).toBe(0);
  });

  it("makes a pet the more restless the more company stands on its surface, and changes nothing else", () => {
    const alone = activityWeights(feeling(RESTED), kind(true), OPEN);
    for (const crowd of [1, 2, 5]) {
      const crowded = activityWeights(feeling(RESTED), kind(true), { ...OPEN, crowd });
      expect(weightOf(crowded, "hop")).toBe(weightOf(alone, "hop") * (1 + crowd));
      expect(crowded.map((weight, index) => (index === ACTIVITIES.indexOf("hop") ? 0 : weight))).toEqual(alone.map((weight, index) => (index === ACTIVITIES.indexOf("hop") ? 0 : weight)));
    }
    expect(weightOf(activityWeights(feeling(RESTED), kind(true), { ...OPEN, crowd: 5, hops: false }), "hop")).toBe(0);
    expect(weightOf(activityWeights(feeling(RESTED), kind(true), { ...OPEN, crowd: 5, quiet: true }), "hop")).toBe(0);
    expect(weightOf(activityWeights(feeling(RESTED), kind(true), { ...OPEN, crowd: 5, movers: 1 }), "hop")).toBe(0);
  });

  it("makes a pet sleepier the less energy it has, never above 0.6 of it and never while it is watched", () => {
    const sleep = (energy: number, watched: boolean): number => weightOf(activityWeights(feeling({ energy, sociability: 0.5, curiosity: 0.6 }), kind(true), { ...OPEN, watched }), "sleep");
    expect(sleep(1, false)).toBe(0);
    expect(sleep(0.6, false)).toBe(0);
    expect(sleep(0.4, false)).toBeGreaterThan(0);
    expect(sleep(0.2, false)).toBeGreaterThan(sleep(0.4, false));
    expect(sleep(0, false)).toBe(MODE_LIMITS.calm.sleep);
    expect(sleep(0, true)).toBe(0);
  });

  it("makes a curious pet walk more and a bored one less", () => {
    const walk = (curiosity: number): number => weightOf(activityWeights(feeling({ energy: 0.8, sociability: 0.5, curiosity }), kind(true), OPEN), "walk");
    expect(walk(1)).toBeGreaterThan(walk(0.5));
    expect(walk(0.5)).toBeGreaterThan(walk(0));
    expect(walk(0)).toBeGreaterThan(0);
  });

  it("answers the committed situations", () => {
    for (const vector of CHOICE.weights) {
      const weights = weighed(vector);
      expect(weights, vector.id).toHaveLength(vector.expected.length);
      weights.forEach((weight, index) => expect(Math.abs(weight - vector.expected[index]!), `${vector.id}[${index}]`).toBeLessThanOrEqual(TOLERANCE));
    }
  });
});

describe("weightedIndex", () => {
  it("picks in proportion to the weights", () => {
    expect(weightedIndex([1, 1, 2], 0)).toBe(0);
    expect(weightedIndex([1, 1, 2], 0.2499)).toBe(0);
    expect(weightedIndex([1, 1, 2], 0.25)).toBe(1);
    expect(weightedIndex([1, 1, 2], 0.5)).toBe(2);
    expect(weightedIndex([1, 1, 2], 0.999999)).toBe(2);
  });

  it("never picks a weight that is not positive and says −1 when none is", () => {
    for (let step = 0; step < 100; step++) expect([1, 3]).toContain(weightedIndex([-1, 2, 0, 4], step / 100));
    expect(weightedIndex([], 0.5)).toBe(-1);
    expect(weightedIndex([0, 0], 0.5)).toBe(-1);
    expect(weightedIndex([-1], 0.5)).toBe(-1);
  });

  it("is what randomPick does with the unit of its key", () => {
    const weights = [0.5, 0, 1.25, 3, 0, 0.125];
    for (let counter = 0; counter < DRAWS; counter++) {
      const key = [20261002, 3, counter];
      expect(weightedIndex(weights, randomUnit(key))).toBe(randomPick(key, weights));
    }
  });

  it("answers the committed picks and decisions", () => {
    for (const vector of CHOICE.picks) expect(vector.units.map((unit) => weightedIndex(vector.weights, unit)), vector.id).toEqual(vector.expected);
    for (const vector of CHOICE.decisions) {
      const weights = weighed(vector);
      expect(Array.from({ length: vector.count }, (_, counter) => ACTIVITIES[randomPick([vector.seed, vector.stream, counter], weights)]), vector.id).toEqual(vector.expected.activities);
    }
  });
});

describe("dwellOf", () => {
  it("keeps an idle dwell inside the range of its mode", () => {
    for (let step = 0; step < STEPS; step++) {
      const unit = step / STEPS;
      expect(dwellOf("idle", "calm", unit)).toBeGreaterThanOrEqual(6 * SECOND);
      expect(dwellOf("idle", "calm", unit)).toBeLessThan(20 * SECOND);
      expect(dwellOf("idle", "lively", unit)).toBeGreaterThanOrEqual(3 * SECOND);
      expect(dwellOf("idle", "lively", unit)).toBeLessThan(10 * SECOND);
    }
    expect(dwellOf("idle", "still", 0.7)).toBe(0);
  });

  it("lets encounters last 2 to 5 seconds, a sulk 3 to 6, a sleep 20 to 60 and a landing 0.3", () => {
    for (const activity of ["greet", "cuddle", "squabble"] as const) {
      expect(dwellOf(activity, "calm", 0)).toBe(2 * SECOND);
      expect(dwellOf(activity, "calm", 0.999999)).toBe(5 * SECOND - 1);
    }
    expect([dwellOf("sulk", "calm", 0), dwellOf("sulk", "lively", 0.999999)]).toEqual([3 * SECOND, 6 * SECOND - 1]);
    expect([dwellOf("sleep", "calm", 0), dwellOf("sleep", "calm", 0.999999)]).toEqual([20 * SECOND, 60 * SECOND - 1]);
    expect(dwellOf("land", "calm", 0.5)).toBe(19);
  });

  it("is a whole number of ticks that never shrinks as the unit grows", () => {
    for (const activity of ACTIVITIES) {
      for (const mode of PET_MODES) {
        let previous = -1;
        for (let step = 0; step < RISES; step++) {
          const dwell = dwellOf(activity, mode, step / RISES);
          expect(Number.isInteger(dwell)).toBe(true);
          expect(dwell).toBeGreaterThanOrEqual(previous);
          previous = dwell;
        }
      }
    }
  });

  it("answers the committed dwells", () => {
    for (const vector of CHOICE.dwells) expect(vector.units.map((unit) => dwellOf(vector.activity, vector.mode, unit)), vector.id).toEqual(vector.expected);
  });
});

describe("the activity graph", () => {
  it("lets every activity reach every other one", () => {
    for (const start of ACTIVITIES) {
      const seen = new Set<Activity>([start]);
      const queue: Activity[] = [start];
      for (let head = 0; head < queue.length; head++) for (const follower of followersOf(queue[head]!)) if (!seen.has(follower)) queue.push(follower), seen.add(follower);
      expect([...seen].sort(), start).toEqual([...ACTIVITIES].sort());
    }
  });

  it("lists followers in ACTIVITIES order, once each, and lets idle follow everything", () => {
    for (const activity of ACTIVITIES) {
      const followers = followersOf(activity);
      expect(followers, activity).toEqual(ACTIVITIES.filter((candidate) => followers.includes(candidate)));
      expect(followers).toContain("idle");
    }
  });

  it("only lets a squabble end in a sulk, and flights in a landing", () => {
    expect(ACTIVITIES.filter((activity) => followersOf(activity).includes("sulk"))).toEqual(["squabble"]);
    expect(ACTIVITIES.filter((activity) => followersOf(activity).includes("land"))).toEqual(["hop", "fall", "tumble", "glide"]);
    expect(ACTIVITIES.filter((activity) => !followersOf(activity).includes("hang"))).toEqual(["hang"]);
    expect(followersOf("idle")).toEqual(expect.arrayContaining(["fidget", "walk", "hop", "sleep", "greet", "cuddle", "squabble", "fall"]));
  });

  it("is the committed graph", () => {
    const committed = CHOICE.graph[0]!.expected;
    for (const activity of ACTIVITIES) expect(followersOf(activity), activity).toEqual(committed.followers[activity]);
    expect(committed.components).toBe(1);
  });
});

describe("encounters", () => {
  it("shares out a whole: greet, cuddle and squabble sum to 1 and none is negative", () => {
    for (let step = -100; step <= 100; step++) {
      const shares = encounterShares(step / 100);
      expect(shares).toHaveLength(ENCOUNTERS.length);
      expect(Math.abs(shares[0]! + shares[1]! + shares[2]! - 1)).toBeLessThanOrEqual(1e-12);
      for (const share of shares) expect(share).toBeGreaterThanOrEqual(0);
    }
  });

  it("lets friends mostly cuddle and never squabble", () => {
    for (const affinity of [0.4, 0.6, 0.9, 1]) {
      const [greet, cuddle, squabble] = encounterShares(affinity);
      expect(cuddle).toBeGreaterThan(0.5);
      expect(cuddle).toBeGreaterThan(greet!);
      expect(squabble).toBe(0);
    }
  });

  it("lets rivals mostly squabble and never cuddle, with a greeting left even at the floor", () => {
    for (const affinity of [-0.3, -0.45, AFFINITY_FLOOR, -1]) {
      const [greet, cuddle, squabble] = encounterShares(affinity);
      expect(squabble).toBeGreaterThan(0.5);
      expect(cuddle).toBe(0);
      expect(greet).toBeGreaterThan(0);
    }
  });

  it("lets everyone else mostly greet, with a small dispute now and then between strangers", () => {
    for (const affinity of [-0.2, 0, 0.2, 0.39]) expect(encounterShares(affinity)[0]).toBeGreaterThan(0.5);
    expect(encounterShares(0)[2]).toBeGreaterThan(0);
    expect(encounterShares(0)[2]).toBeLessThan(0.15);
    expect(encounterShares(0.3)[1]).toBeGreaterThan(0);
  });

  it("draws the kind from the shares", () => {
    const tally = (affinity: number): Record<string, number> => {
      const counts: Record<string, number> = { greet: 0, cuddle: 0, squabble: 0 };
      for (let step = 0; step < 1000; step++) counts[encounterOf(affinity, (step + 0.5) / 1000)]!++;
      return counts;
    };
    expect(tally(0.8).cuddle).toBe(820);
    expect(tally(-0.5).squabble).toBe(800);
    expect(tally(0).greet).toBe(920);
    expect(encounterOf(0.8, 0)).toBe("greet");
    expect(encounterOf(-0.5, 0.999)).toBe("squabble");
  });

  it("answers the committed encounters", () => {
    for (const vector of CHOICE.encounters) {
      encounterShares(vector.affinity).forEach((share, index) => expect(Math.abs(share - vector.expected.shares[index]!), vector.id).toBeLessThanOrEqual(TOLERANCE));
      expect(vector.units.map((unit) => encounterOf(vector.affinity, unit)), vector.id).toEqual(vector.expected.kinds);
    }
  });
});

describe("bonds", () => {
  const bonds: Bond[] = [
    { between: ["sunny", "solary"], affinity: 0.9 },
    { between: ["sunny", "cloudy"], affinity: -0.4 },
    { between: ["solary", "cloudy"], affinity: -0.7 },
  ];

  it("reads the authored affinity in either order, 0 for strangers and for a species with itself", () => {
    expect(affinityOf(bonded(bonds), [], "sunny", "solary")).toBe(0.9);
    expect(affinityOf(bonded(bonds), [], "solary", "sunny")).toBe(0.9);
    expect(affinityOf(bonded(bonds), [], "sunny", "housy")).toBe(0);
    expect(affinityOf(bonded(bonds), [{ between: ["sunny", "sunny"], drift: 0.3 }], "sunny", "sunny")).toBe(0);
  });

  it("adds the drift of the rapport and stays between the floor and 1", () => {
    expect(affinityOf(bonded(bonds), [{ between: ["cloudy", "sunny"], drift: 0.1 }], "sunny", "cloudy")).toBeCloseTo(-0.3, 12);
    expect(affinityOf(bonded(bonds), [{ between: ["sunny", "solary"], drift: 0.5 }], "sunny", "solary")).toBe(1);
    expect(affinityOf(bonded(bonds), [], "solary", "cloudy")).toBe(AFFINITY_FLOOR);
    expect(affinityOf(bonded(bonds), [{ between: ["solary", "cloudy"], drift: -0.5 }], "cloudy", "solary")).toBe(AFFINITY_FLOOR);
    expect(affinityOf(bonded([]), [{ between: ["a", "b"], drift: -0.5 }], "a", "b")).toBe(-0.5);
  });

  it("moves a rapport by the step of the encounter and nothing else", () => {
    expect(rapportAfter(0, "greet")).toBe(0.05);
    expect(rapportAfter(0, "cuddle")).toBe(0.1);
    expect(rapportAfter(0, "squabble")).toBe(-0.15);
    expect(rapportAfter(-0.15, "sulk")).toBeCloseTo(-0.05, 12);
    for (const activity of ["idle", "fidget", "walk", "hop", "fall", "land", "sleep"] as const) expect(rapportAfter(0.2, activity)).toBe(0.2);
    expect(rapportAfter(0.45, "cuddle")).toBe(RAPPORT_SPAN);
    expect(rapportAfter(-0.45, "squabble")).toBe(-RAPPORT_SPAN);
  });

  it("keeps disputes small: a squabble followed by its sulk costs a twentieth", () => {
    let drift = 0;
    for (let round = 1; round <= 20; round++) {
      drift = rapportAfter(rapportAfter(drift, "squabble"), "sulk");
      expect(drift).toBeCloseTo(Math.max(-0.05 * round, -RAPPORT_SPAN + 0.1), 9);
      expect(affinityOf(bonded([{ between: ["a", "b"], affinity: -0.5 }]), [{ between: ["a", "b"], drift }], "a", "b")).toBeGreaterThanOrEqual(AFFINITY_FLOOR);
    }
  });

  it("lets a rapport fade to nothing: a tenth in ten minutes, from either side, and never past 0", () => {
    expect(rapportFaded(0.1, 0)).toBe(0.1);
    expect(rapportFaded(0.1, 300 * SECOND)).toBeCloseTo(0.05, 12);
    expect(rapportFaded(0.1, 600 * SECOND)).toBe(0);
    expect(rapportFaded(-0.15, 600 * SECOND)).toBeCloseTo(-0.05, 12);
    expect(rapportFaded(-0.15, 900 * SECOND)).toBe(0);
    expect(rapportFaded(0.5, 3000 * SECOND + 1)).toBe(0);
    expect(Object.is(rapportFaded(-0.1, 100000 * SECOND), 0)).toBe(true);
    for (let ticks = 0; ticks < 40000; ticks += 997) expect(Math.abs(rapportFaded(0.3, ticks + 997))).toBeLessThanOrEqual(Math.abs(rapportFaded(0.3, ticks)));
  });

  it("answers the committed affinities, steps, fadings and histories", () => {
    for (const vector of BONDS.affinities) vector.pairs.forEach((pair, index) => expect(Math.abs(affinityOf(bonded(vector.bonds), vector.rapports, pair[0], pair[1]) - vector.expected[index]!), `${vector.id} ${pair.join("–")}`).toBeLessThanOrEqual(TOLERANCE));
    for (const vector of BONDS.rapportSteps) for (const activity of ACTIVITIES) expect(Math.abs(rapportAfter(vector.drift, activity) - vector.expected[activity]!), `${vector.id} ${activity}`).toBeLessThanOrEqual(TOLERANCE);
    for (const vector of BONDS.rapportFading) vector.ticks.forEach((ticks, index) => expect(Math.abs(rapportFaded(vector.drift, ticks) - vector.expected[index]!), `${vector.id} ${ticks}`).toBeLessThanOrEqual(TOLERANCE));
    for (const vector of BONDS.histories) {
      let drift = 0;
      vector.events.forEach((event, index) => {
        drift = rapportAfter(rapportFaded(drift, event.after), event.activity);
        expect(Math.abs(drift - vector.expected.drifts[index]!), `${vector.id}[${index}]`).toBeLessThanOrEqual(TOLERANCE);
        expect(Math.abs(affinityOf(bonded([{ between: ["a", "b"], affinity: vector.affinity }]), [{ between: ["a", "b"], drift }], "b", "a") - vector.expected.affinities[index]!), `${vector.id}[${index}]`).toBeLessThanOrEqual(TOLERANCE);
      });
    }
  });
});

describe("needs", () => {
  const even: Temperament = { energy: 0.5, sociability: 0.5, curiosity: 0.5 };

  it("lets a pet arrive rested, with the sociability and curiosity of its temperament", () => {
    expect(needsOf({ energy: 0.6, sociability: 0.7, curiosity: 0.2 })).toEqual({ energy: 0.8, sociability: 0.7, curiosity: 0.2 });
    expect(needsOf({ energy: 0, sociability: 0, curiosity: 0 }).energy).toBe(0.5);
    expect(needsOf({ energy: 1, sociability: 1, curiosity: 1 }).energy).toBe(1);
  });

  it("changes nothing in no time", () => {
    const needs = { energy: 0.4, sociability: 0.6, curiosity: 0.8 };
    for (const activity of ACTIVITIES) expect(needsAfter(needs, activity, 0, even)).toEqual(needs);
  });

  it("drains energy awake, faster on the move, and returns it asleep", () => {
    const start = { energy: 0.5, sociability: 0.5, curiosity: 0.5 };
    const after = (activity: Activity): number => needsAfter(start, activity, 10 * SECOND, even).energy;
    expect(after("idle")).toBeLessThan(0.5);
    expect(after("walk")).toBeLessThan(after("idle"));
    expect(after("hop")).toBeLessThan(after("walk"));
    expect(after("sleep")).toBeCloseTo(0.7, 12);
    expect(needsAfter(start, "idle", 10 * SECOND, { ...even, energy: 0.9 }).energy).toBeGreaterThan(needsAfter(start, "idle", 10 * SECOND, { ...even, energy: 0.1 }).energy);
  });

  it("spends sociability in company and curiosity on the move, and grows both at rest", () => {
    const start = { energy: 0.5, sociability: 0.5, curiosity: 0.5 };
    for (const activity of ["greet", "cuddle", "squabble"] as const) expect(needsAfter(start, activity, 3 * SECOND, even).sociability).toBeCloseTo(0.14, 12);
    expect(needsAfter(start, "idle", 60 * SECOND, even).sociability).toBeGreaterThan(0.5);
    for (const activity of ["walk", "fidget", "hop"] as const) expect(needsAfter(start, activity, 2 * SECOND, even).curiosity).toBeLessThan(0.5);
    expect(needsAfter(start, "idle", 10 * SECOND, even).curiosity).toBeGreaterThan(0.5);
  });

  it("holds every need inside 0 and 1 however long something lasts", () => {
    for (const activity of ACTIVITIES) {
      for (const start of [0, 0.5, 1]) {
        const needs = needsAfter({ energy: start, sociability: start, curiosity: start }, activity, 1000000, even);
        for (const need of [needs.energy, needs.sociability, needs.curiosity]) {
          expect(need).toBeGreaterThanOrEqual(0);
          expect(need).toBeLessThanOrEqual(1);
        }
      }
    }
  });

  it("answers the committed needs and days", () => {
    const near = (left: Needs, right: Needs, where: string): void => {
      for (const need of ["energy", "sociability", "curiosity"] as const) expect(Math.abs(left[need] - right[need]), `${where} ${need}`).toBeLessThanOrEqual(TOLERANCE);
    };
    for (const vector of BONDS.needs) near(needsAfter(vector.needs, vector.activity, vector.ticks, vector.temperament), vector.expected, vector.id);
    for (const vector of BONDS.days) {
      let needs = needsOf(vector.temperament);
      near(needs, vector.expected[0]!, vector.id);
      vector.spans.forEach((span, index) => {
        needs = needsAfter(needs, span.activity, span.ticks, vector.temperament);
        near(needs, vector.expected[index + 1]!, `${vector.id}[${index}]`);
      });
    }
  });
});

describe("castOf", () => {
  const cast: Cast = { scene: "home", core: ["sunny", "cloudy", "housy"], rotation: ["windy", "boily", "roofy", "insuly"] };

  it("puts the core first and fills up with the rotation", () => {
    const troupe = castOf(cast, 5, 0, 7);
    expect(troupe.slice(0, 3)).toEqual(cast.core);
    expect(troupe).toHaveLength(5);
    for (const species of troupe.slice(3)) expect(cast.rotation).toContain(species);
  });

  it("never exceeds the capacity and shows nobody without room", () => {
    expect(castOf(cast, 0, 0, 7)).toEqual([]);
    expect(castOf(cast, -3, 0, 7)).toEqual([]);
    expect(castOf(cast, 100, 3, 7)).toHaveLength(7);
    expect(castOf(cast, 4.9, 0, 7)).toHaveLength(4);
  });

  it("keeps one seat for a visitor from two seats on, also when the core alone would fill or exceed them: the core then takes turns itself", () => {
    for (const capacity of [2, 3]) {
      const seen = new Set<string>();
      for (let epoch = 0; epoch < cast.core.length * cast.rotation.length; epoch++) {
        const troupe = castOf(cast, capacity, epoch, 7);
        expect(troupe).toHaveLength(capacity);
        expect(new Set(troupe).size).toBe(capacity);
        for (const species of troupe.slice(0, -1)) expect(cast.core).toContain(species);
        expect(cast.rotation).toContain(troupe.at(-1));
        for (const species of troupe) seen.add(species);
        expect(castOf(cast, capacity, epoch + cast.core.length * cast.rotation.length, 7)).toEqual(troupe);
      }
      expect(seen).toEqual(new Set([...cast.core, ...cast.rotation]));
    }
    expect(castOf(cast, 3, 1, 7)[0]).toBe(castOf(cast, 3, 0, 7)[1]);
    expect(castOf(cast, 4, 5, 7).slice(0, 3)).toEqual(cast.core);
    expect(castOf(cast, 1, 0, 7)).toHaveLength(1);
    expect(cast.core).toContain(castOf(cast, 1, 0, 7)[0]);
    expect(new Set([0, 1, 2].map((epoch) => castOf(cast, 1, epoch, 7)[0]))).toEqual(new Set(cast.core));
  });

  it("gives a core without a rotation every seat, and a rotation without a core as well", () => {
    const alone: Cast = { scene: "s", core: ["a", "b", "c"], rotation: [] };
    expect(castOf(alone, 3, 4, 7)).toEqual(["a", "b", "c"]);
    expect(castOf(alone, 2, 0, 7)).toHaveLength(2);
    expect(castOf(alone, 2, 1, 7)[0]).toBe(castOf(alone, 2, 0, 7)[1]);
    const guests: Cast = { scene: "s", core: [], rotation: ["x", "y", "z"] };
    expect([...castOf(guests, 5, 0, 7)].sort()).toEqual(["x", "y", "z"]);
    expect(castOf(guests, 1, 0, 7)).toHaveLength(1);
  });

  it("shows five of nine in turn and one of eleven visitors on a stage of six, and everybody within the turns of both rings", () => {
    const home: Cast = { scene: "home", core: ["a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9"], rotation: ["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "b10", "b11"] };
    const seen = new Set<string>();
    for (let epoch = 0; epoch < 11; epoch++) {
      const troupe = castOf(home, 6, epoch, 20261002);
      expect(troupe).toHaveLength(6);
      expect(troupe.filter((species) => home.core.includes(species))).toHaveLength(5);
      expect(home.rotation).toContain(troupe[5]);
      const next = castOf(home, 6, epoch + 1, 20261002);
      expect(next.filter((species) => !troupe.includes(species))).toHaveLength(2);
      for (const species of troupe) seen.add(species);
    }
    expect(seen.size).toBe(20);
    const words = randomWords([20261002, CAST_STREAM, 0], 2);
    expect(castOf(home, 6, 0, 20261002)[0]).toBe(home.core[words[0]! % 9]);
    expect(castOf(home, 6, 0, 20261002)[5]).toBe(home.rotation[words[1]! % 11]);
  });

  it("moves the rotation on by one per epoch and comes round again", () => {
    const shown = (epoch: number): string[] => castOf(cast, 5, epoch, 7).slice(3);
    for (let epoch = 0; epoch < 8; epoch++) {
      expect(shown(epoch + 1)[0]).toBe(shown(epoch)[1]);
      expect(shown(epoch + cast.rotation.length)).toEqual(shown(epoch));
    }
    expect(new Set([0, 1, 2, 3].flatMap(shown))).toEqual(new Set(cast.rotation));
  });

  it("is a pure function of its seed, and seeds start the rotation in different places", () => {
    expect(castOf(cast, 5, 2, 99)).toEqual(castOf(cast, 5, 2, 99));
    expect(new Set(Array.from({ length: 40 }, (_, seed) => castOf(cast, 4, 0, seed)[3])).size).toBe(cast.rotation.length);
  });

  it("lists a species once even when a cast names it twice", () => {
    const troupe = castOf({ scene: "s", core: ["a", "b", "a"], rotation: ["b", "c", "d", "c"] }, 6, 1, 3);
    expect(troupe.slice(0, 2)).toEqual(["a", "b"]);
    expect([...troupe].sort()).toEqual(["a", "b", "c", "d"]);
  });

  it("answers the committed casts", () => {
    for (const vector of CHOICE.casts) expect(vector.epochs.map((epoch) => castOf(vector.cast, vector.capacity, epoch, vector.seed)), vector.id).toEqual(vector.expected);
  });
});

describe("the arithmetic of the module", () => {
  it("uses nothing the Rust twin cannot reproduce bit for bit, and never the console", () => {
    const source = readFileSync(resolve(HERE, "../../🟦️.ts"), "utf8");
    expect(source).not.toMatch(/Math\.(sin|cos|tan|atan2|exp|pow|hypot|log|random)\b/);
    expect(source).not.toMatch(/\b(Date|performance|console)\b/);
  });

  it("holds the limits the committed vectors were generated with", () => {
    for (const vector of CHOICE.limits) expect(MODE_LIMITS[vector.id]).toEqual(vector.expected);
  });
});
