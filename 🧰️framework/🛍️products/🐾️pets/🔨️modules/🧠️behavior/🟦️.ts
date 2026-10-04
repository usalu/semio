/** 🧠️ The character of a pet in numbers: what a mode allows, how much a pet feels like each activity, how long it stays with it, how two pets meet, and how their bond and their drives move. Pure functions over the schema; the stage folds them.
 *
 * Every table is an array in `ACTIVITIES` order (idle, fidget, walk, hop, fall, land, sleep, greet, cuddle, squabble,
 * sulk, then hang, tumble, glide, aim, reel, climb, mantle, slide, carry, trick, purr, dizzy, shrug, scoot, push),
 * every constant a literal and every expression written once in a fixed order (no reassociation), so the Rust twin
 * yields the same bits. Time is whole ticks; rates are per second and meet time as `rate × (ticks ÷ 64)`.
 *
 * Only `idle` decides freely: an idle pet draws its next activity from {@link activityWeights} (idle again, fidget,
 * walk, hop, sleep or a trick on a whim); every other activity is entered by what happens on stage and ends where
 * {@link followersOf} says. Encounters are drawn from {@link encounterShares}; a squabble always ends in a sulk and a
 * sulk in mending. Of the fifteen later activities only the trick weighs in that draw: the learner's hand, gear,
 * the other tricks and mischief begin them.
 *
 * @see ../🎪️stage/🟦️.ts — the fold that calls all of this
 * @see ../🎲️randomness/🟦️.ts — `randomWords`, `weightedIndex`, the reserved streams
 * @see ../📐️trigonometry/🟦️.ts — `clamp`
 * @see ../../🧬️schema/🟦️.ts — `Activity`, `Actor`, `Cast`, `Menagerie`, `Needs`, `PetMode`, `Rapport`, `Species`, `Temperament`
 * @see ./🦀️.rs — the Rust twin
 */

import { ACTIVITIES, TICKS_PER_SECOND, type Activity, type Actor, type Cast, type Menagerie, type Needs, type PetMode, type Rapport, type Slug, type Species, type Temperament, type Ticks } from "../../🧬️schema/🟦️.ts";
import { CAST_STREAM, randomWords, weightedIndex } from "../🎲️randomness/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";

//#region 🔖️Modes
/** 🚦️ What a mode allows and how eager it makes a pet: how many actors may walk or hop at once (`movers`) and fidget at once (`fidgeters`), the range of an idle dwell in ticks, the weights of fidget, walk, hop, sleep and a trick on a whim beside an idle weight of 1, the longest walk in body widths (`stroll`), the least ticks between two encounters (`encounterGap`, 0 = never) and the chance of one per second for a pair of full sociability (`encounterRate`). */
export type Limits = {
  readonly movers: number;
  readonly fidgeters: number;
  readonly idleLow: Ticks;
  readonly idleHigh: Ticks;
  readonly fidget: number;
  readonly walk: number;
  readonly hop: number;
  readonly sleep: number;
  readonly whim: number;
  readonly stroll: number;
  readonly encounterGap: Ticks;
  readonly encounterRate: number;
};

/** 🎚️ The limits of every mode: `still` allows nothing, `calm` is slightly active (one mover, long idle dwells, a fidget now and then, a short walk now and then, a trick on a whim rarely, an encounter every couple of minutes), `lively` is busy but not frantic. */
export const MODE_LIMITS: { readonly [mode in PetMode]: Limits } = {
  still: { movers: 0, fidgeters: 0, idleLow: 0, idleHigh: 0, fidget: 0, walk: 0, hop: 0, sleep: 0, whim: 0, stroll: 0, encounterGap: 0, encounterRate: 0 },
  calm: { movers: 1, fidgeters: 1, idleLow: 384, idleHigh: 1280, fidget: 0.4, walk: 0.2, hop: 0.12, sleep: 6, whim: 0.04, stroll: 4, encounterGap: 5760, encounterRate: 0.04 },
  lively: { movers: 2, fidgeters: 2, idleLow: 192, idleHigh: 640, fidget: 1, walk: 0.6, hop: 0.3, sleep: 3, whim: 0.25, stroll: 6, encounterGap: 1920, encounterRate: 0.12 },
};
//#endregion 🔖️Modes

//#region 🔖️Choice
/** 🧭️ What an idle pet finds around it when it decides: the mode, whether it is a time of concentration, how many other actors walk or hop and how many fidget right now, whether its perch has room for a walk (`roam`), whether another perch is open to it — in reach of a hop, or with room to wander off to (`hops`) —, how many others stand on its surface (`crowd`), whether the pointer is close enough to keep it awake (`watched`) and whether a trick of its own is on offer for a whim (`whims`: its species has one for its state and its mood). */
export type Situation = {
  readonly mode: PetMode;
  readonly quiet: boolean;
  readonly movers: number;
  readonly fidgeters: number;
  readonly roam: boolean;
  readonly hops: boolean;
  readonly crowd: number;
  readonly watched: boolean;
  readonly whims: boolean;
};

const QUIET_DROWSE = 3;
const DROWSY = 0.6;

/** ⚖️ How much an idle actor feels like each activity, in `ACTIVITIES` order; 0 = not eligible.
 *
 * `idle` weighs 1. With `drive = 0.5 + 0.5 × energy` and `urge = 0.25 + curiosity` of the actor's needs and the
 * limits of the mode: `fidget = limits.fidget × drive × (0.5 + curiosity)` while fewer than `limits.fidgeters` others
 * fidget and the species has a fidget clip; `walk = limits.walk × drive × urge` while its perch has room and fewer
 * than `limits.movers` others move; `hop = limits.hop × energy × urge × (1 + crowd)` while another perch is open to
 * it and fewer than `limits.movers` others move — company on its surface makes a pet restless, so a crowd thins out
 * by itself where there is room elsewhere; `sleep = limits.sleep × tired²` unless watched, with
 * `tired = (0.6 − energy) ÷ 0.6` held in [0, 1] (a pet with more than 0.6 of its energy never dozes off);
 * `trick = limits.whim × drive` while a trick of its own is on offer for a whim — the pet's own initiative, rare in
 * calm and never in still. Quiet removes fidget, walk, hop and trick and triples sleep. Everything else is entered by
 * events and weighs 0.
 */
export function activityWeights(actor: Actor, species: Species, situation: Situation): number[] {
  const limits = MODE_LIMITS[situation.mode];
  const energy = actor.needs.energy;
  const curiosity = actor.needs.curiosity;
  const drive = 0.5 + 0.5 * energy;
  const urge = 0.25 + curiosity;
  const tired = clamp((DROWSY - energy) / DROWSY, 0, 1);
  const awake = !situation.quiet;
  const free = situation.movers < limits.movers;
  const fidgets = species.repertoire.fidget;
  const fidget = awake && situation.fidgeters < limits.fidgeters && fidgets !== undefined && fidgets.length > 0 ? limits.fidget * drive * (0.5 + curiosity) : 0;
  const walk = awake && free && situation.roam ? limits.walk * drive * urge : 0;
  const hop = awake && free && situation.hops ? limits.hop * energy * urge * (1 + situation.crowd) : 0;
  const sleep = situation.watched ? 0 : limits.sleep * tired * tired * (situation.quiet ? QUIET_DROWSE : 1);
  const trick = awake && situation.whims ? limits.whim * drive : 0;
  return [1, fidget, walk, hop, 0, 0, sleep, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, trick, 0, 0, 0, 0, 0];
}

const DWELL_LOW: readonly Ticks[] = [0, 96, 1920, 96, 640, 19, 1280, 128, 128, 128, 192, 1920, 640, 1280, 32, 640, 1920, 32, 640, 1920, 128, 192, 96, 64, 320, 320];
const DWELL_HIGH: readonly Ticks[] = [0, 96, 1920, 96, 640, 19, 3840, 320, 320, 320, 384, 1920, 640, 1280, 64, 640, 1920, 32, 640, 1920, 128, 384, 160, 96, 320, 640];

/** ⏳️ How many ticks an activity lasts for a unit draw: `low + floor((high − low) × unit)`.
 *
 * `idle` takes its range from the mode (6…20 s calm, 3…10 s lively, 0 still); `sleep` lasts 20…60 s, `greet`,
 * `cuddle` and `squabble` 2…5 s, `sulk` 3…6 s and `land` 0.3 s. `fidget` (1.5 s) is the span of a fidget whose clip
 * loops, and `walk` (30 s), `hop` (1.5 s) and `fall` (10 s) are the patience of the stage with motion that ends itself.
 * Of the later activities `aim` lasts 0.5…1 s, `mantle` 0.5 s, `purr` 3…6 s, `dizzy` 1.5…2.5 s, `shrug` 1…1.5 s,
 * `push` 5…10 s and `trick` 2 s where its clip does not say; `hang`, `climb` and `carry` (30 s), `glide` (20 s),
 * `tumble`, `reel` and `slide` (10 s) and `scoot` (5 s) are again the patience of the stage.
 */
export function dwellOf(activity: Activity, mode: PetMode, unit: number): Ticks {
  const index = ACTIVITIES.indexOf(activity);
  const low = activity === "idle" ? MODE_LIMITS[mode].idleLow : DWELL_LOW[index]!;
  const high = activity === "idle" ? MODE_LIMITS[mode].idleHigh : DWELL_HIGH[index]!;
  return low + Math.floor((high - low) * unit);
}
//#endregion 🔖️Choice

//#region 🔖️Graph
const AFTER_IDLE: readonly Activity[] = ["idle", "fidget", "walk", "hop", "fall", "sleep", "greet", "cuddle", "squabble", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"];
const AFTER_FIDGET: readonly Activity[] = ["idle", "walk", "fall", "greet", "hang", "trick", "purr", "shrug", "scoot"];
const AFTER_WALK: readonly Activity[] = ["idle", "fall", "greet", "cuddle", "squabble", "hang", "aim", "climb", "trick", "purr", "shrug", "scoot"];
const AFTER_HOP: readonly Activity[] = ["idle", "fall", "land", "hang", "glide", "slide"];
const AFTER_FALL: readonly Activity[] = ["idle", "land", "hang", "glide", "slide"];
const AFTER_LAND: readonly Activity[] = ["idle", "walk", "fall", "greet", "hang", "trick", "purr", "dizzy", "shrug", "scoot"];
const AFTER_SLEEP: readonly Activity[] = ["idle", "walk", "fall", "greet", "hang", "trick", "purr", "shrug", "scoot"];
const AFTER_GREET: readonly Activity[] = ["idle", "walk", "fall", "hang", "trick", "purr", "shrug", "scoot"];
const AFTER_CUDDLE: readonly Activity[] = ["idle", "walk", "fall", "hang", "purr", "scoot"];
const AFTER_SQUABBLE: readonly Activity[] = ["idle", "walk", "fall", "sulk", "hang", "scoot"];
const AFTER_SULK: readonly Activity[] = ["idle", "walk", "fall", "greet", "hang", "trick", "purr", "shrug", "scoot"];
const AFTER_HAND: readonly Activity[] = ["idle", "walk", "fall", "greet", "hang", "trick", "purr", "shrug", "scoot"];
const AFTER_HANG: readonly Activity[] = ["idle", "tumble", "glide"];
const AFTER_TUMBLE: readonly Activity[] = ["idle", "land", "hang", "glide", "slide"];
const AFTER_GLIDE: readonly Activity[] = ["idle", "land", "hang", "slide"];
const AFTER_AIM: readonly Activity[] = ["idle", "fall", "hang", "reel", "shrug", "scoot"];
const AFTER_REEL: readonly Activity[] = ["idle", "fall", "hang", "mantle"];
const AFTER_CLIMB: readonly Activity[] = ["idle", "fall", "hang", "mantle", "slide", "push"];
const AFTER_CARRY: readonly Activity[] = ["idle", "fall", "hang", "climb", "scoot"];
const AFTER_SLIDE: readonly Activity[] = ["idle", "fall", "hang"];
const AFTER_GROUNDED: readonly Activity[] = ["idle", "fall", "hang", "scoot"];
const AFTER_PUSH: readonly Activity[] = ["idle", "walk", "fall", "hang", "tumble", "climb", "mantle", "slide", "scoot"];

/** 🕸️ The activities that may follow an activity on stage, in `ACTIVITIES` order: the activity graph. Every activity is reachable from every other one, and `idle` follows them all (a stage that turns still freezes every actor in it).
 *
 * `idle` → idle, fidget, walk, hop, sleep or a trick by its own choice, greet, a trick, a purr or a shrug by the
 * learner's hand, greet, cuddle or squabble when a partner has arrived, walk towards a partner or off the stage,
 * fall when its perch vanishes. `walk` → idle at its goal, or the encounter it walked into. `hop` → land, or fall
 * when its target is gone. `fall` → land. `squabble` → sulk. `cuddle` → a purr on its own. Whatever a pet does on its
 * perch by itself — `fidget`, `walk`, `land`, `sleep`, `greet`, `sulk`, `trick`, `purr`, `shrug` — the learner's
 * click or gesture can turn into a greeting, a trick, a purr or a shrug. Whoever is summoned away walks off.
 *
 * The body adds: the learner's hand picks a pet up whatever it does (`hang`); whatever stands on a perch falls when
 * its perch vanishes and scoots to a new seat when a survey re-seats it (`scoot` → idle there, or fall). `hang` →
 * `tumble` when it is let go, `glide` when it glides back to where it was picked up. In the air a parachute opens
 * (`fall`, `tumble`, a `hop` planned anew → `glide`), and whatever comes down lands on a perch (`land`, or at rest
 * at once after a parachute) or on a head (`slide`, which ends in a `fall`); a hard landing from far up makes a pet
 * `dizzy`.
 *
 * Gear adds trips that begin in `idle` or at the end of the `walk` to the gear: `carry` the own ladder there →
 * `climb` it; `aim` the grappling gun → `reel` up the rope → `mantle` onto the edge, or `shrug` after a miss; `climb`
 * a wall or a ladder (resting on it in the same pose) → `mantle` over the rim, `slide` down the wall, or `idle` on
 * the perch it steps off onto; whatever holds on falls when its wall, its ladder or its rope gives way, and scoots
 * while it still stands on its perch.
 *
 * Mischief adds `push`: a pet that stands `idle` beside a marked element of the page or holds on to the wall beside
 * it (`climb`) pushes; it ends `idle` on its perch or holding on and climbing, mantling or sliding off its wall,
 * `tumble`s when the learner takes the element back, falls when what carries it gives way, scoots to a new seat,
 * walks off when it is summoned away, and is picked up like everybody else.
 */
export function followersOf(activity: Activity): readonly Activity[] {
  if (activity === "idle") return AFTER_IDLE;
  if (activity === "fidget") return AFTER_FIDGET;
  if (activity === "walk") return AFTER_WALK;
  if (activity === "hop") return AFTER_HOP;
  if (activity === "fall") return AFTER_FALL;
  if (activity === "land") return AFTER_LAND;
  if (activity === "sleep") return AFTER_SLEEP;
  if (activity === "greet") return AFTER_GREET;
  if (activity === "cuddle") return AFTER_CUDDLE;
  if (activity === "squabble") return AFTER_SQUABBLE;
  if (activity === "sulk") return AFTER_SULK;
  if (activity === "trick" || activity === "purr" || activity === "shrug") return AFTER_HAND;
  if (activity === "hang") return AFTER_HANG;
  if (activity === "tumble") return AFTER_TUMBLE;
  if (activity === "glide") return AFTER_GLIDE;
  if (activity === "aim") return AFTER_AIM;
  if (activity === "reel") return AFTER_REEL;
  if (activity === "climb") return AFTER_CLIMB;
  if (activity === "carry") return AFTER_CARRY;
  if (activity === "slide" || activity === "mantle") return AFTER_SLIDE;
  if (activity === "dizzy" || activity === "scoot") return AFTER_GROUNDED;
  return AFTER_PUSH;
}
//#endregion 🔖️Graph

//#region 🔖️Encounters
/** 🫱️ What two pets can do when they meet, in the order of {@link encounterShares}. */
export const ENCOUNTERS = ["greet", "cuddle", "squabble"] as const;

/** 🤗️ One of {@link ENCOUNTERS}. */
export type Encounter = (typeof ENCOUNTERS)[number];

/** 🫂️ The least affinity of two friends: they mostly cuddle when they meet, and a pet with gear sets out for the perch of a friend. */
export const FRIENDS = 0.4;

const RIVALS = -0.3;

/** 🥧️ The chances `[greet, cuddle, squabble]` of an encounter at an affinity; they sum to 1.
 *
 * Friends (affinity ≥ 0.4) mostly cuddle: `0.5 + 0.4 × affinity` (0.66 … 0.9). Rivals (affinity ≤ −0.3) mostly
 * squabble: `0.55 − 0.5 × affinity`, at most 0.95 (0.7 … 0.85 down to the affinity floor). In between a pair mostly
 * greets, with a cuddle now and then when it leans friendly (`0.5 × affinity`) and a small dispute now and then when
 * it does not (`0.08 − 0.5 × affinity`, never below 0). Friends never squabble and rivals never cuddle.
 */
export function encounterShares(affinity: number): number[] {
  const cuddle = affinity >= FRIENDS ? 0.5 + 0.4 * affinity : affinity > 0 ? 0.5 * affinity : 0;
  const squabble = affinity <= RIVALS ? clamp(0.55 - 0.5 * affinity, 0, 0.95) : clamp(0.08 - 0.5 * affinity, 0, 0.95);
  return [1 - cuddle - squabble, cuddle, squabble];
}

/** 🎭️ The encounter a unit draw picks at an affinity: {@link weightedIndex} of {@link encounterShares}. */
export function encounterOf(affinity: number, unit: number): Encounter {
  const index = weightedIndex(encounterShares(affinity), unit);
  return ENCOUNTERS[index < 0 ? 0 : index]!;
}
//#endregion 🔖️Encounters

//#region 🔖️Bonds
/** 🧱️ The lowest effective affinity of any pair: disputes stay small however often two rivals have squabbled. */
export const AFFINITY_FLOOR = -0.6;

/** 📏️ How far shared history can shift a bond away from its authored affinity, in both directions. */
export const RAPPORT_SPAN = 0.5;

const RAPPORT_STEPS: readonly number[] = [0, 0, 0, 0, 0, 0, 0, 0.05, 0.1, -0.15, 0.1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const RAPPORT_FADE_STEP = 0.1;
const RAPPORT_FADE_TICKS = 38400;

/** 🔗️ Whether a pair joins the two species, in either order. */
function joins(pair: readonly [Slug, Slug], a: Slug, b: Slug): boolean {
  return (pair[0] === a && pair[1] === b) || (pair[0] === b && pair[1] === a);
}

/** 💞️ How two species feel about each other right now: the authored affinity of their bond (0 when unlisted) plus the drift of their rapport (0 when they have no history), held inside `[AFFINITY_FLOOR, 1]`; 0 for a species and itself. */
export function affinityOf(menagerie: Menagerie, rapports: readonly Rapport[], a: Slug, b: Slug): number {
  if (a === b) return 0;
  let affinity = 0;
  for (const bond of menagerie.bonds) {
    if (!joins(bond.between, a, b)) continue;
    affinity = bond.affinity;
    break;
  }
  let drift = 0;
  for (const rapport of rapports) {
    if (!joins(rapport.between, a, b)) continue;
    drift = rapport.drift;
    break;
  }
  return clamp(affinity + drift, AFFINITY_FLOOR, 1);
}

/** 🪢️ The drift of a rapport after an encounter: a greet adds 0.05, a cuddle 0.1, a squabble takes 0.15 and the end of a sulk gives 0.1 back (mending); any other activity leaves it. The result stays inside `[−RAPPORT_SPAN, RAPPORT_SPAN]`. */
export function rapportAfter(drift: number, activity: Activity): number {
  return clamp(drift + RAPPORT_STEPS[ACTIVITIES.indexOf(activity)]!, 0 - RAPPORT_SPAN, RAPPORT_SPAN);
}

/** 🍂️ The drift of a rapport `ticks` later without an encounter: it moves towards 0 by `0.1 × ticks ÷ 38400` (a tenth in ten minutes) and stops there. */
export function rapportFaded(drift: number, ticks: Ticks): number {
  const step = (RAPPORT_FADE_STEP * ticks) / RAPPORT_FADE_TICKS;
  return drift > step ? drift - step : drift < 0 - step ? drift + step : 0;
}
//#endregion 🔖️Bonds

//#region 🔖️Needs
const ENERGY_RATES: readonly number[] = [-0.001, -0.01, -0.012, -0.02, 0, 0, 0.02, -0.006, -0.004, -0.012, -0.001, 0, 0, 0, -0.004, -0.012, -0.016, -0.016, -0.002, -0.014, -0.01, 0.004, -0.002, -0.001, -0.012, -0.014];
const SOCIABILITY_RATES: readonly number[] = [0.002, 0.002, 0.002, 0.002, 0, 0.002, 0.002, -0.12, -0.12, -0.12, -0.02, 0, 0, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, -0.06, 0.002, 0.002, 0.002, 0.002];
const CURIOSITY_RATES: readonly number[] = [0.01, -0.08, -0.06, -0.1, 0, 0.01, 0.01, 0, 0, 0, 0.01, 0, 0, -0.04, -0.06, -0.08, -0.08, -0.08, -0.04, -0.06, -0.08, 0.01, 0, 0.01, -0.02, -0.1];

/** 🌱️ The drives a pet arrives with: energy `0.5 + 0.5 × temperament.energy`, sociability and curiosity as its temperament says. */
export function needsOf(temperament: Temperament): Needs {
  return { energy: 0.5 + 0.5 * temperament.energy, sociability: temperament.sociability, curiosity: temperament.curiosity };
}

/** 🔋️ The drives of an actor after `ticks` of an activity, each held inside [0, 1]: `need + rate × (ticks ÷ 64)` with a rate per second and activity.
 *
 * Energy drains with everything but sleep (idle 0.001, walk 0.012, hop 0.02 … per second), slower for an energetic
 * species (`× (1.5 − temperament.energy)`), and returns asleep (0.02). Sociability grows alone (0.002, `× (0.5 +
 * temperament.sociability)`) and is spent in company (0.12 while greeting, cuddling or squabbling). Curiosity grows
 * at rest (0.01, `× (0.5 + temperament.curiosity)`) and is spent on the move (walk 0.06, fidget 0.08, hop 0.1).
 * Of the later activities the strenuous ones drain energy (climb and mantle 0.016, carry and push 0.014, reel and
 * scoot 0.012, trick 0.01) and purring gives a little back (0.004); a purr spends sociability (0.06) on the
 * learner's company; getting somewhere and showing off spend curiosity (push 0.1, reel, climb, mantle and trick
 * 0.08, aim and carry 0.06, glide and slide 0.04, scoot 0.02). In the learner's hand and in a tumble nothing moves.
 */
export function needsAfter(needs: Needs, activity: Activity, ticks: Ticks, temperament: Temperament): Needs {
  const index = ACTIVITIES.indexOf(activity);
  const seconds = ticks / TICKS_PER_SECOND;
  const energy = ENERGY_RATES[index]!;
  const sociability = SOCIABILITY_RATES[index]!;
  const curiosity = CURIOSITY_RATES[index]!;
  return {
    energy: clamp(needs.energy + (energy < 0 ? energy * (1.5 - temperament.energy) : energy) * seconds, 0, 1),
    sociability: clamp(needs.sociability + (sociability > 0 ? sociability * (0.5 + temperament.sociability) : sociability) * seconds, 0, 1),
    curiosity: clamp(needs.curiosity + (curiosity > 0 ? curiosity * (0.5 + temperament.curiosity) : curiosity) * seconds, 0, 1),
  };
}
//#endregion 🔖️Needs

//#region 🔖️Casting
/** 🎠️ The members of a ring that are on at an epoch, `seats` of them at most: read one after the other from the place a word picks (`word mod count`), moved on by one per epoch, so everybody gets its turn and a change of epoch swaps one member. */
function turnsOf(ring: readonly Slug[], seats: number, word: number, epoch: number): Slug[] {
  const count = ring.length;
  const taken: Slug[] = [];
  if (count === 0) return taken;
  const turn = Math.floor(epoch) % count;
  const start = (word % count) + (turn < 0 ? turn + count : turn);
  for (let index = 0; index < count && index < seats; index++) taken.push(ring[(start + index) % count]!);
  return taken;
}

/** 🎟️ The species of a cast that are on stage at an epoch: its core first, then its visitors.
 *
 * The rotation visits: it gets the seats the core leaves free, and one seat whenever the stage holds at least two —
 * also when the core alone would fill or exceed the capacity —, so every species of a cast can appear. The core gets
 * the other seats: all of it, in its order, while it fits; otherwise it takes turns itself. Whoever takes turns is
 * read off a ring ({@link turnsOf}) from a start the seed picks (`randomWords([seed, CAST_STREAM, 0], 2)`: word 0
 * for the core, word 1 for the rotation) and every epoch moves on by one. A species is listed once, the core wins.
 */
export function castOf(cast: Cast, capacity: number, epoch: number, seed: number): Slug[] {
  const room = capacity > 0 ? Math.floor(capacity) : 0;
  const core: Slug[] = [];
  for (const species of cast.core) if (!core.includes(species)) core.push(species);
  const pool: Slug[] = [];
  for (const species of cast.rotation) if (!core.includes(species) && !pool.includes(species)) pool.push(species);
  const spare = room - core.length;
  const least = room >= 2 ? 1 : 0;
  const open = spare > least ? spare : least;
  const guests = open < pool.length ? open : pool.length;
  const seats = room - guests;
  const words = randomWords([seed >>> 0, CAST_STREAM, 0], 2);
  const troupe = core.length <= seats ? core : turnsOf(core, seats, words[0]!, epoch);
  for (const species of turnsOf(pool, guests, words[1]!, epoch)) troupe.push(species);
  return troupe;
}
//#endregion 🔖️Casting
