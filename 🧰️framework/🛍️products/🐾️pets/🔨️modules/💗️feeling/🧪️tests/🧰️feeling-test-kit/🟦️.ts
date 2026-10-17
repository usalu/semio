/** 🧰️ Shared fixtures and helpers of the feeling unit suites: the fixture reader, the tolerance and sample sizes, the vectors of the module and of the chemistry rules, the calm and host characters, the deep comparison and the seeded draws. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect } from "vitest";
import { MOODS, type Cooling, type Feeling, type Menagerie, type Mood, type Rapport, type Slug, type Species } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { randomWords, unitOf } from "../../../🎲️randomness/🟦️.ts";
import { type Character, type Chemistry, MOOD_REST, type Occasion, type Sighting, type Standing, type Trial } from "../../🟦️.ts";

export const HERE = dirname(fileURLToPath(import.meta.url));
const TOLERANCE = 1e-9;

/** 🎯️ How many random feelings a law of the moods is held to at the level of the run. */
export const FEELINGS = sampled(200, 2000, 20000);

/** 🎬️ How many random stages a law of the chemistry is held to at the level of the run. */
export const STAGES = sampled(40, 400, 4000);

/** 🧫️ One committed fixture of the product. */
function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(HERE, "../../../../🧪️tests/🧫️fixtures", name, "🔣️.json"), "utf8")) as T;
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

export const FELT = fixture<FeelingVectors>("💗️feeling-dynamics");
export const SET = fixture<ChemistryVectors>("⚗️chemistry-rules");
export const MENAGERIE = SET.menagerie;
export const CALM: Feeling = { mood: "content", intensity: MOOD_REST, since: 0 };
export const HOST: Character = { mood: "content", temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 } };

/** 🧬️ One species of the committed menagerie. */
export function kind(id: Slug): Species {
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
export function agree(actual: unknown, expected: unknown, where: string): void {
  expect(difference(actual, expected, where)).toBeNull();
}

/** 🧊️ A value frozen all the way down: whatever changes it throws. */
export function frozen<Value>(value: Value): Value {
  if (value !== null && typeof value === "object") {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}

/** 🎲️ A stream of units from the product's own counter-based draws, so every run of the suite sees the same samples. */
export function units(seed: number, count: number): number[] {
  return randomWords([seed, 77, 0], count).map(unitOf);
}

/** 🫀️ A random feeling with a dyadic intensity (sixty-fourths), so its arithmetic is exact. */
export function drawn(marks: readonly number[], index: number): Feeling {
  return { mood: MOODS[Math.floor(marks[index]! * MOODS.length)]!, intensity: Math.floor(marks[index + 1]! * 65) / 64, since: Math.floor(marks[index + 2]! * 500) };
}

/** 👀️ An actor of the committed menagerie as the chemistry sees it, standing on the ground line. */
export function sight(species: Slug, state: Slug, x: number, more: Partial<Sighting> = {}): Sighting {
  const size = kind(species).size;
  return { species, state, held: 0, mood: "content", intensity: MOOD_REST, activity: "idle", trick: null, x, y: 300, width: size.width, height: size.height, ...more };
}
