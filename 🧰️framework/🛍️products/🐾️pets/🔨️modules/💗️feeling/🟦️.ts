/** 💗️ What a pet feels, in numbers: its mood and how it comes and goes, what that does to its face and its wishes, how moods travel between pets, the lasting states of a species, the tricks on offer, and the chemistry between species. Pure functions over the schema; the stage folds them.
 *
 * Every table is an array in a defined order (`MOODS`: content, happy, playful, curious, proud, sleepy, grumpy, sad,
 * scared; `ACTIVITIES`; `[greet, cuddle, squabble]`), every constant a literal and every expression written once in a
 * fixed order (no reassociation), so the Rust twin yields the same bits. Time is whole ticks; rates are per second
 * and meet time as `(rate × ticks) ÷ 64`. Nothing here draws: whoever needs chance is handed units.
 *
 * A {@link Feeling} is an anchor, not a clock: the stage stores what {@link impulse} (or {@link atRest}) returns and
 * reads the present through {@link settled}, which is a closed form over any number of ticks — so however time is
 * cut into `ticked` events, the same feelings come out.
 *
 * @see ../🧠️behavior/🟦️.ts — `activityWeights`, `encounterShares`, `rapportAfter`: what the multipliers of this module apply to; `affinityOf`, which a reaction's `affinity` bounds
 * @see ../🎲️randomness/🟦️.ts — `weightedIndex`, the one weighted pick
 * @see ../📐️trigonometry/🟦️.ts — `clamp`
 * @see ../../🧬️schema/🟦️.ts — `Mood`, `Cue`, `SpeciesState`, `Trick`, `Trait`, `Effect`, `Reaction`, `Species`, `Menagerie`
 */

import { ACTIVITIES, MOODS, TICKS_PER_SECOND, type Activity, type Cooling, type Cue, type Effect, type Feeling, type Menagerie, type Mood, type Rapport, type Reaction, type Slug, type Species, type SpeciesState, type Ticks, type Trait, type Trick } from "../../🧬️schema/🟦️.ts";
import { weightedIndex } from "../🎲️randomness/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";
import { affinityOf } from "../🧠️behavior/🟦️.ts";

//#region 🔖️Moods
/** 🥇️ How much a mood outranks the others, in `MOODS` order: scared 6, grumpy 5, sad 4, proud, happy and playful 3, curious 2, sleepy 1 (it follows the energy need), content 0. A mood of a higher rank replaces a lower one at once. */
export const MOOD_PRIORITIES: readonly number[] = [0, 3, 3, 2, 3, 1, 5, 4, 6];

/** ⏳️ For how many ticks a freshly stirred mood neither fades nor gives way to a mood of its own rank or below (2 s): faces do not flicker. */
export const MOOD_HOLD: Ticks = 128;

/** 🌫️ The intensity below which a mood is no longer felt: it has faded and the resting mood of the species returns. An impulse fainter than this never replaces a mood. */
export const MOOD_FAINT = 0.05;

/** 🛋️ How strongly a pet feels the resting mood of its species when nothing stirs it. A feeling at or below this level is calm: it gives way to any other mood, catches the moods of others and spreads none. */
export const MOOD_REST = 0.25;

/** 🌅️ By how much per second the resting mood returns up to {@link MOOD_REST} after another mood has faded (8 s from nothing). */
export const MOOD_RISE = 0.03125;

/** 💪️ How many times stronger than the present intensity an impulse of a lower rank must be to replace a mood whose hold is over. */
export const MOOD_OVERRIDE = 1.2;

/** 📉️ By how much per second a mood fades once its hold is over, in `MOODS` order: from full strength happy and grumpy last 64 s, playful and proud 32 s, curious 16 s, sleepy and sad 128 s, scared 6.4 s (content, when stirred above its rest, 32 s). */
export const MOOD_DECAYS: readonly number[] = [0.03125, 0.015625, 0.03125, 0.0625, 0.03125, 0.0078125, 0.015625, 0.0078125, 0.15625];

/** 🧲️ The factor on the fading of the resting mood of a species: a pet stays twice as long in the mood it is prone to. */
export const PRONE_DECAY = 0.5;

/** 🔢️ The place of a mood in `MOODS`, the row of every table of this module. */
function moodIndex(mood: Mood): number {
  return MOODS.indexOf(mood);
}

/** 🏅️ The rank of a mood ({@link MOOD_PRIORITIES}). */
function rankOf(mood: Mood): number {
  return MOOD_PRIORITIES[moodIndex(mood)]!;
}

/** 😌️ The feeling of a pet that nothing has stirred yet: the resting mood of its species at {@link MOOD_REST}. */
export function atRest(resting: Mood, tick: Ticks): Feeling {
  return { mood: resting, intensity: MOOD_REST, since: tick };
}

/** 🪞️ The mood a pet is in for everybody who asks "is it grumpy?": its mood while that is stirred above {@link MOOD_REST}, `content` while it is calm — whatever the resting mood of its species, a pet at rest is nobody's grump. Tricks and reactions ask this. */
export function shownMood(mood: Mood, intensity: number): Mood {
  return intensity > MOOD_REST ? mood : "content";
}

/** ⚡️ The feeling after an impulse of `amount` (0…1) of a mood at `tick`; `feeling` is the present one ({@link settled} at that tick), and the result is what the stage stores.
 *
 * An impulse of nothing changes nothing. The same mood is reinforced: its intensity rises by `amount` (at most 1)
 * and its hold begins anew. An impulse of `content` soothes: a mood that is worse than content (sleepy, curious,
 * scared, grumpy, sad by their valence) loses `amount` and is over when less than {@link MOOD_FAINT} is left; joy is
 * never taken away. Any other mood replaces the present one — at `amount`, with a fresh hold — when the impulse is
 * at least {@link MOOD_FAINT} and the present feeling is calm (at most {@link MOOD_REST}), or the new mood outranks
 * it ({@link MOOD_PRIORITIES}), or the hold is over and the new mood is of the same rank or more than
 * {@link MOOD_OVERRIDE} times as strong as what is left. Otherwise the impulse is lost.
 */
export function impulse(feeling: Feeling, mood: Mood, amount: number, tick: Ticks): Feeling {
  if (!(amount > 0)) return feeling;
  if (mood === feeling.mood) {
    const raised = feeling.intensity + amount;
    return { mood, intensity: raised < 1 ? raised : 1, since: tick };
  }
  if (mood === "content") {
    if (!(MOOD_VALENCES[moodIndex(feeling.mood)]! < MOOD_VALENCES[0]!)) return feeling;
    const eased = feeling.intensity - amount;
    return eased >= MOOD_FAINT ? { mood: feeling.mood, intensity: eased, since: feeling.since } : { mood: feeling.mood, intensity: 0, since: tick };
  }
  if (amount < MOOD_FAINT) return feeling;
  const fresh: Feeling = { mood, intensity: amount < 1 ? amount : 1, since: tick };
  if (feeling.intensity <= MOOD_REST) return fresh;
  const rank = rankOf(mood);
  const held = rankOf(feeling.mood);
  if (rank > held) return fresh;
  if (tick - feeling.since < MOOD_HOLD) return feeling;
  return rank === held || amount > MOOD_OVERRIDE * feeling.intensity ? fresh : feeling;
}

/** 🕯️ The first tick at which a mood that is not the resting one is no longer felt: `since` when it is faint already (or its intensity is no number in [0, 1]), else the first tick after its hold at which less than {@link MOOD_FAINT} is left. */
function fadesAt(feeling: Feeling): Ticks {
  if (!(feeling.intensity >= MOOD_FAINT && feeling.intensity <= 1)) return feeling.since;
  const rate = MOOD_DECAYS[moodIndex(feeling.mood)]!;
  let wait = Math.floor(((feeling.intensity - MOOD_FAINT) * TICKS_PER_SECOND) / rate) + 1;
  while (wait > 1 && feeling.intensity - (rate * (wait - 1)) / TICKS_PER_SECOND < MOOD_FAINT) wait = wait - 1;
  while (!(feeling.intensity - (rate * wait) / TICKS_PER_SECOND < MOOD_FAINT)) wait = wait + 1;
  return feeling.since + MOOD_HOLD + wait;
}

/** 🍂️ The feeling as it stands at `tick`, a closed form over any number of ticks: settling it again at the same tick returns it unchanged, and settling in two steps equals settling in one.
 *
 * A mood other than the resting one keeps its intensity through its hold and then loses `MOOD_DECAYS × seconds`;
 * from the tick less than {@link MOOD_FAINT} is left ({@link fadesAt}) the pet is back in the resting mood of its
 * species, which rises from 0 by {@link MOOD_RISE} per second up to {@link MOOD_REST}. The resting mood itself,
 * stirred above its rest, fades half as fast ({@link PRONE_DECAY}) down to {@link MOOD_REST}; below it (soothed), it
 * rises back. The answer is anchored at `tick` — `since` is set so that the same future follows: `tick − MOOD_HOLD`
 * for a fading mood, `tick` for a rising one, and the tick it came to rest ({@link settlesAt}) for a feeling at
 * rest; an unchanged feeling is returned as it is.
 */
export function settled(feeling: Feeling, resting: Mood, tick: Ticks): Feeling {
  const rested = settlesAt(feeling, resting);
  if (tick >= rested) return feeling.mood === resting && feeling.intensity === MOOD_REST ? feeling : { mood: resting, intensity: MOOD_REST, since: rested };
  const rate = MOOD_DECAYS[moodIndex(feeling.mood)]!;
  if (feeling.mood === resting) {
    if (feeling.intensity > MOOD_REST) {
      const wait = tick - feeling.since - MOOD_HOLD;
      return wait > 0 ? { mood: resting, intensity: feeling.intensity - (rate * PRONE_DECAY * wait) / TICKS_PER_SECOND, since: tick - MOOD_HOLD } : feeling;
    }
    const span = tick - feeling.since;
    return span > 0 ? { mood: resting, intensity: feeling.intensity + (MOOD_RISE * span) / TICKS_PER_SECOND, since: tick } : feeling;
  }
  const wait = tick - feeling.since - MOOD_HOLD;
  const left = wait > 0 ? feeling.intensity - (rate * wait) / TICKS_PER_SECOND : feeling.intensity;
  if (left >= MOOD_FAINT) return wait > 0 ? { mood: feeling.mood, intensity: left, since: tick - MOOD_HOLD } : feeling;
  const span = tick - fadesAt(feeling);
  return { mood: resting, intensity: span > 0 ? (MOOD_RISE * span) / TICKS_PER_SECOND : 0, since: tick };
}

/** 🏁️ The tick from which {@link settled} answers the resting mood at {@link MOOD_REST} and nothing changes any more: the horizon of a feeling for whoever lets the stage rest. An intensity that is no number in [0, 1] is no feeling at all: it is at rest from `since` on. */
export function settlesAt(feeling: Feeling, resting: Mood): Ticks {
  if (!(feeling.intensity >= 0 && feeling.intensity <= 1)) return feeling.since;
  if (feeling.mood !== resting) {
    let rise = Math.floor((MOOD_REST * TICKS_PER_SECOND) / MOOD_RISE);
    while (rise > 1 && !((MOOD_RISE * (rise - 1)) / TICKS_PER_SECOND < MOOD_REST)) rise = rise - 1;
    while ((MOOD_RISE * rise) / TICKS_PER_SECOND < MOOD_REST) rise = rise + 1;
    return fadesAt(feeling) + rise;
  }
  if (feeling.intensity > MOOD_REST) {
    const rate = MOOD_DECAYS[moodIndex(feeling.mood)]! * PRONE_DECAY;
    let wait = Math.floor(((feeling.intensity - MOOD_REST) * TICKS_PER_SECOND) / rate);
    while (wait > 1 && !(feeling.intensity - (rate * (wait - 1)) / TICKS_PER_SECOND > MOOD_REST)) wait = wait - 1;
    while (wait < 1 || feeling.intensity - (rate * wait) / TICKS_PER_SECOND > MOOD_REST) wait = wait + 1;
    return feeling.since + MOOD_HOLD + wait;
  }
  if (feeling.intensity === MOOD_REST) return feeling.since;
  let span = Math.floor(((MOOD_REST - feeling.intensity) * TICKS_PER_SECOND) / MOOD_RISE);
  while (span > 1 && !(feeling.intensity + (MOOD_RISE * (span - 1)) / TICKS_PER_SECOND < MOOD_REST)) span = span - 1;
  while (span < 1 || feeling.intensity + (MOOD_RISE * span) / TICKS_PER_SECOND < MOOD_REST) span = span + 1;
  return feeling.since + span;
}

/** 🌤️ The first tick from which a feeling shows `content` ({@link shownMood} of what {@link settled} answers): `since` when it shows content already — a calm feeling, or content itself —, else the first tick after its hold at which no more than {@link MOOD_REST} is left (its resting mood fades at half the rate). The stage's horizon for whatever the shown mood steers. */
export function calmsAt(feeling: Feeling, resting: Mood): Ticks {
  if (!(feeling.intensity > MOOD_REST && feeling.intensity <= 1) || feeling.mood === "content") return feeling.since;
  const rate = feeling.mood === resting ? MOOD_DECAYS[moodIndex(feeling.mood)]! * PRONE_DECAY : MOOD_DECAYS[moodIndex(feeling.mood)]!;
  let wait = Math.floor(((feeling.intensity - MOOD_REST) * TICKS_PER_SECOND) / rate);
  while (wait > 1 && !(feeling.intensity - (rate * (wait - 1)) / TICKS_PER_SECOND > MOOD_REST)) wait = wait - 1;
  while (wait < 1 || feeling.intensity - (rate * wait) / TICKS_PER_SECOND > MOOD_REST) wait = wait + 1;
  return feeling.since + MOOD_HOLD + wait;
}
//#endregion 🔖️Moods

//#region 🔖️Face
/** 🙂️ How far a mood at full strength bends the mouth, from −1 (sad) to 1 (happy), in `MOODS` order: content 0.3, happy 0.8, playful 0.6, curious 0.2, proud 0.6, sleepy 0.1, grumpy −0.5, sad −0.8, scared −0.4. It is the valence of the mood. */
export const MOOD_VALENCES: readonly number[] = [0.3, 0.8, 0.6, 0.2, 0.6, 0.1, -0.5, -0.8, -0.4];

/** 😴️ How far a mood at full strength lowers the upper lids at rest (0 wide open … 1 shut; blinks run on top), in `MOODS` order. */
export const MOOD_LIDS: readonly number[] = [0.1, 0.12, 0, 0, 0.15, 0.55, 0.35, 0.25, 0];

/** 🤨️ How a mood at full strength slants the lid edge over each eye, in degrees and `MOODS` order: positive lowers the inner end (hooded, angry), negative raises it (arched, worried). */
export const MOOD_SLANTS: readonly number[] = [0, 0, -10, -10, 6, 4, 12, -10, -14];

/** 🧍️ How far a mood at full strength lets the bone that carries the first eye sink, in pixels and `MOODS` order: negative lifts it (happy, proud), positive lets it droop (sleepy, sad, scared). */
export const MOOD_DROPS: readonly number[] = [0, -1.5, 0, -1, -1.5, 1.5, 0, 2, 2];

/** 😀️ What a feeling does to the shared face: the bend of the mouth (−1…1), the resting height of the lids (0…1), the slant of the lids in degrees and the drop of the posture in pixels. */
export type Countenance = { readonly bend: number; readonly lid: number; readonly slant: number; readonly drop: number };

/** ➗️ One channel of the face: from what content shows towards what the mood shows at full strength, by the intensity. */
function blend(table: readonly number[], feeling: Feeling): number {
  const calm = table[0]!;
  return calm + (table[moodIndex(feeling.mood)]! - calm) * feeling.intensity;
}

/** 👄️ The valence of a mood, from −1 (sad) to 1 (happy): {@link MOOD_VALENCES}. */
export function valenceOf(mood: Mood): number {
  return MOOD_VALENCES[moodIndex(mood)]!;
}

/** 🎈️ The spirits of a feeling, the one number the mouth of the first round bends with: the valence of content (0.3) moved towards the valence of the mood by its intensity. */
export function spiritsOf(feeling: Feeling): number {
  return blend(MOOD_VALENCES, feeling);
}

/** 🎭️ The face of a feeling: every channel is the face of content moved towards the face of the mood by its intensity, so a fading mood eases back without a jump. */
export function faceOf(feeling: Feeling): Countenance {
  return { bend: blend(MOOD_VALENCES, feeling), lid: blend(MOOD_LIDS, feeling), slant: blend(MOOD_SLANTS, feeling), drop: blend(MOOD_DROPS, feeling) };
}
//#endregion 🔖️Face

//#region 🔖️Appraisal
/** 📅️ What can happen to a pet that moves its mood: the learner said hello (`greeted`), it performed a trick (`tricked`), it is being petted (`purred`), picked up (`lifted`), still held a while later (`dangled`), shaken (`shaken`), it came down hard (`dropped`), softly under its parachute (`floated`) or softly without one (`landed`), another pet landed on its head (`trampled`), another pet greeted it (`welcomed`), cuddled it (`cuddled`), squabbled with it (`squabbled`), its sulk ended (`mended`), it was rained on (`drenched`), clicked once too often (`pestered`), thrown off what it played with (`evicted`), the pointer lingers on it (`watched`), something gave it a fright (`startled`), it woke up (`woken`), another pet showed it a trick (`entertained`), it got where it wanted with its gear — up a wall, over a ladder, up its rope (`climbed`) —, its grappling hook missed the edge (`missed`), it bounced off an edge of the stage in flight (`bonked`). */
export const OCCASIONS = ["greeted", "tricked", "purred", "lifted", "dangled", "shaken", "dropped", "floated", "landed", "trampled", "welcomed", "cuddled", "squabbled", "mended", "drenched", "pestered", "evicted", "watched", "startled", "woken", "entertained", "climbed", "missed", "bonked"] as const;

/** 🗓️ One of {@link OCCASIONS}. */
export type Occasion = (typeof OCCASIONS)[number];

/** 💢️ One impulse an occasion gives: a mood and its amount before the character of the species scales it. */
export type Stir = { readonly occasion: Occasion; readonly mood: Mood; readonly amount: number };

/** 📖️ The appraisal table: which occasion gives which impulses, applied in this order. An impulse of `content` soothes first (a cuddle ends fear, anger and sadness before it makes happy; a hello takes a little of them). */
export const APPRAISALS: readonly Stir[] = [
  { occasion: "greeted", mood: "content", amount: 0.2 },
  { occasion: "greeted", mood: "happy", amount: 0.3 },
  { occasion: "tricked", mood: "playful", amount: 0.4 },
  { occasion: "purred", mood: "content", amount: 0.5 },
  { occasion: "purred", mood: "happy", amount: 0.3 },
  { occasion: "lifted", mood: "scared", amount: 0.3 },
  { occasion: "dangled", mood: "curious", amount: 0.4 },
  { occasion: "shaken", mood: "scared", amount: 0.5 },
  { occasion: "dropped", mood: "grumpy", amount: 0.4 },
  { occasion: "floated", mood: "content", amount: 1 },
  { occasion: "floated", mood: "proud", amount: 0.4 },
  { occasion: "landed", mood: "content", amount: 0.5 },
  { occasion: "landed", mood: "happy", amount: 0.3 },
  { occasion: "trampled", mood: "grumpy", amount: 0.3 },
  { occasion: "welcomed", mood: "happy", amount: 0.3 },
  { occasion: "cuddled", mood: "content", amount: 1 },
  { occasion: "cuddled", mood: "happy", amount: 0.5 },
  { occasion: "squabbled", mood: "grumpy", amount: 0.5 },
  { occasion: "mended", mood: "content", amount: 0.3 },
  { occasion: "drenched", mood: "sad", amount: 0.4 },
  { occasion: "pestered", mood: "grumpy", amount: 0.4 },
  { occasion: "evicted", mood: "scared", amount: 0.4 },
  { occasion: "watched", mood: "curious", amount: 0.4 },
  { occasion: "startled", mood: "scared", amount: 0.4 },
  { occasion: "woken", mood: "content", amount: 1 },
  { occasion: "entertained", mood: "happy", amount: 0.3 },
  { occasion: "climbed", mood: "content", amount: 0.3 },
  { occasion: "climbed", mood: "proud", amount: 0.4 },
  { occasion: "missed", mood: "grumpy", amount: 0.3 },
  { occasion: "bonked", mood: "grumpy", amount: 0.3 },
];

/** 🧬️ What of a species shapes its feelings: its resting mood and its temperament. Every `Species` is one. */
export type Character = Pick<Species, "mood" | "temperament">;

/** 🧮️ How the temperament of a species scales an impulse, per mood in `MOODS` order as `[base, energy, sociability, curiosity]`: the scale is `base + energy × temperament.energy + …`. The energetic lean playful, the sociable happy and sad, the curious curious; low energy leans sleepy and low sociability grumpy; content, proud and scared are taken as they come. */
export const PRONENESS: readonly (readonly number[])[] = [
  [1, 0, 0, 0],
  [0.5, 0, 1, 0],
  [0.5, 1, 0, 0],
  [0.5, 0, 0, 1],
  [1, 0, 0, 0],
  [1.5, -1, 0, 0],
  [1.5, 0, -1, 0],
  [0.5, 0, 1, 0],
  [1, 0, 0, 0],
];

/** 📈️ The factor on every impulse of the resting mood of a species: a pet is half again as easily stirred into the mood it is prone to. */
export const PRONE_GAIN = 1.5;

/** 🎚️ How strongly a species takes an impulse of a mood: the scale of its temperament ({@link PRONENESS}), times {@link PRONE_GAIN} for its resting mood. */
export function pronenessOf(character: Character, mood: Mood): number {
  const row = PRONENESS[moodIndex(mood)]!;
  const lean = row[0]! + row[1]! * character.temperament.energy + row[2]! * character.temperament.sociability + row[3]! * character.temperament.curiosity;
  return character.mood === mood ? lean * PRONE_GAIN : lean;
}

/** 🌊️ {@link impulse} as a species takes it: the amount times its {@link pronenessOf}, held inside [0, 1]. This is how the mood effect of a reaction lands. */
export function stirred(feeling: Feeling, mood: Mood, amount: number, character: Character, tick: Ticks): Feeling {
  return impulse(feeling, mood, clamp(amount * pronenessOf(character, mood), 0, 1), tick);
}

/** 🧐️ The feeling after an occasion: every impulse of its rows in {@link APPRAISALS}, in table order, each as the species takes it ({@link stirred}). */
export function appraised(feeling: Feeling, occasion: Occasion, character: Character, tick: Ticks): Feeling {
  let present = feeling;
  for (const stir of APPRAISALS) if (stir.occasion === occasion) present = stirred(present, stir.mood, stir.amount, character, tick);
  return present;
}

/** 🪄️ The amount of the impulse a trick leaves: of the mood the trick names, or playful when it names none (the `tricked` row of {@link APPRAISALS}). */
export const TRICK_AMOUNT = 0.4;

/** 🎉️ The feeling after a trick was performed: an impulse of {@link TRICK_AMOUNT} of the mood the trick leaves (`Trick.mood`), as the species takes it; playful when the trick names none. */
export function performed(feeling: Feeling, trick: Trick, character: Character, tick: Ticks): Feeling {
  return trick.mood === undefined ? appraised(feeling, "tricked", character, tick) : stirred(feeling, trick.mood, TRICK_AMOUNT, character, tick);
}

/** 🥱️ The energy below which a pet grows sleepy — the same 0.6 below which the behaviour module lets it doze off. */
export const DROWSY_ENERGY = 0.6;

/** 💤️ The feeling after the energy need was looked at: sleepy follows it. With `tired = (0.6 − energy) ÷ 0.6` held in [0, 1], a pet that is less sleepy than tired gets the difference as an impulse of sleepy — which, being of low rank, takes a calm mood, and a stirred one only once its hold is over and the pet is more than {@link MOOD_OVERRIDE} times as tired as it is stirred; a rested pet is left as it is and its sleepiness fades by itself, or at once when it is `woken`. */
export function drowsed(feeling: Feeling, energy: number, tick: Ticks): Feeling {
  const tired = clamp((DROWSY_ENERGY - energy) / DROWSY_ENERGY, 0, 1);
  const felt = feeling.mood === "sleepy" ? feeling.intensity : 0;
  return tired > felt ? impulse(feeling, "sleepy", tired - felt, tick) : feeling;
}
//#endregion 🔖️Appraisal

//#region 🔖️Wishes
/** ⚖️ What a mood at full strength does to how much a pet feels like each activity: a multiplier per activity in `ACTIVITIES` order, one row per mood in `MOODS` order. Only what a pet starts by itself is moved — fidget, walk, hop, sleep, the gear (aim, climb, carry), trick and push; everything entered by events stays at 1.
 *
 * Happy: a little more of everything, less sleep. Playful: fidget ×2, walk ×1.5, hop ×2, trick ×3, push ×2. Curious:
 * walk ×2, gear ×2. Proud: fidget ×1.5, trick ×2. Sleepy: sleep ×4, everything else a quarter to a half. Grumpy:
 * stomps about (walk ×1.5) and plays little. Sad: sits still and sleeps more. Scared: freezes — no fidget, sleep,
 * gear, trick or push, a quarter of the walks and hops.
 */
export const MOOD_WEIGHTS: readonly (readonly number[])[] = [
  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  [1, 1.5, 1.25, 1.25, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 1.25, 1, 1.25, 1, 1, 1.25, 1.5, 1, 1, 1, 1, 1.25],
  [1, 2, 1.5, 2, 1, 1, 0.25, 1, 1, 1, 1, 1, 1, 1, 1.5, 1, 1.5, 1, 1, 1.5, 3, 1, 1, 1, 1, 2],
  [1, 1, 2, 1.5, 1, 1, 0.25, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1.5, 1, 1, 1, 1, 1, 1.5],
  [1, 1.5, 1, 1, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
  [1, 0.25, 0.5, 0.25, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 0.25, 1, 0.25, 1, 1, 0.25, 0.25, 1, 1, 1, 1, 0.25],
  [1, 1, 1.5, 0.5, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 0.5, 1, 0.5, 1, 1, 0.5, 0.25, 1, 1, 1, 1, 0.5],
  [1, 0.25, 0.5, 0.25, 1, 1, 1.5, 1, 1, 1, 1, 1, 1, 1, 0.5, 1, 0.5, 1, 1, 0.5, 0.25, 1, 1, 1, 1, 0.25],
  [1, 0, 0.25, 0.25, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0],
];

/** 🏋️ The multipliers a mood of an intensity puts on `activityWeights`, in `ACTIVITIES` order: 1 moved towards the row of the mood in {@link MOOD_WEIGHTS} by the intensity. Multiply weight by weight; a weight of 0 stays 0. */
export function moodWeights(mood: Mood, intensity: number): number[] {
  const row = MOOD_WEIGHTS[moodIndex(mood)]!;
  const weights: number[] = [];
  for (let index = 0; index < ACTIVITIES.length; index++) weights.push(1 + (row[index]! - 1) * intensity);
  return weights;
}

/** 💞️ By how much a mood at full strength shifts the affinity an encounter is drawn from, in `MOODS` order: happy +0.15, playful +0.1, curious and proud +0.05, content 0, sleepy and scared −0.05, sad −0.1, grumpy −0.25. */
export const MOOD_AFFINITIES: readonly number[] = [0, 0.15, 0.1, 0.05, 0.05, -0.05, -0.25, -0.1, -0.05];

/** 🥧️ What a mood at full strength does to the chances `[greet, cuddle, squabble]` of an encounter, one row per mood in `MOODS` order: the happy and the playful greet and cuddle more, the curious and the proud greet more, nobody picks on the sleepy, the grumpy squabble more and are poor company, the sad and the scared are comforted (cuddle ×2 — which only friends ever do) and left in peace. */
export const MOOD_SHARES: readonly (readonly number[])[] = [
  [1, 1, 1],
  [1.25, 1.25, 1],
  [1.25, 1.25, 1],
  [1.25, 1, 1],
  [1.5, 1, 1],
  [1, 1, 0.5],
  [0.5, 0.5, 1.5],
  [1, 2, 0.5],
  [1, 2, 0.25],
];

/** 🧭️ How the moods of two pets tip their encounter: `affinity` is added to the affinity the encounter is drawn from, `shares` multiplies the chances `[greet, cuddle, squabble]`, and `show` says who shows off instead of greeting (0 the first, 1 the second, −1 nobody). */
export type Leaning = { readonly affinity: number; readonly shares: readonly number[]; readonly show: number };

/** 🤝️ How an encounter of two pets leans by what they feel: the mean of their {@link MOOD_AFFINITIES}, each by its intensity (two grumpy pets squabble sooner); the product of their {@link MOOD_SHARES}, each moved from 1 by its intensity (a scared one is comforted by a friend); and the proud one — the prouder of two, the first when they are equally proud — shows off when it feels it above {@link MOOD_REST}. */
export function encounterBias(first: Feeling, second: Feeling): Leaning {
  const one = moodIndex(first.mood);
  const other = moodIndex(second.mood);
  const shares: number[] = [];
  for (let index = 0; index < 3; index++) shares.push((1 + (MOOD_SHARES[one]![index]! - 1) * first.intensity) * (1 + (MOOD_SHARES[other]![index]! - 1) * second.intensity));
  const vain = first.mood === "proud" && first.intensity > MOOD_REST;
  const boastful = second.mood === "proud" && second.intensity > MOOD_REST;
  const show = vain && (!boastful || first.intensity >= second.intensity) ? 0 : boastful ? 1 : -1;
  return { affinity: (MOOD_AFFINITIES[one]! * first.intensity + MOOD_AFFINITIES[other]! * second.intensity) / 2, shares, show };
}

/** 🍰️ The chances `[greet, cuddle, squabble]` of an encounter as two moods tip them: share by share times the leaning. They no longer sum to 1; `weightedIndex` picks in proportion. */
export function swayedShares(shares: readonly number[], leaning: Leaning): number[] {
  return [shares[0]! * leaning.shares[0]!, shares[1]! * leaning.shares[1]!, shares[2]! * leaning.shares[2]!];
}
//#endregion 🔖️Wishes

//#region 🔖️Contagion
/** 🦠️ How readily a mood travels from one pet to the next, in `MOODS` order: sleepy 0.8 (a yawn), playful 0.6, happy 0.5, curious 0.4, scared 0.3, grumpy 0.2; content, proud and sad stay where they are. Fear and anger spread weakly on purpose. */
export const MOOD_SPREADS: readonly number[] = [0, 0.5, 0.6, 0.4, 0, 0.8, 0.2, 0, 0.3];

/** 🥁️ Every how many ticks moods travel (twice a second). */
export const CONTAGION_BEAT: Ticks = 32;

/** 📏️ Within how many body widths of each other two pets catch each other's moods. */
export const CONTAGION_REACH = 3;

/** 🤧️ The feeling of a pet after one beat beside another, both as they stood when the beat began: moods travel, and never grow on the way.
 *
 * Only a stirred mood travels (above {@link MOOD_REST}). Its pull is `gain = MOOD_SPREADS × sociability × (0.5 +
 * 0.5 × affinity)` — the sociability of the one who catches, the affinity of the two. A pet in the same mood is
 * lifted by `gain × (theirs − mine)` when the other feels it more strongly, and its hold is left alone. A calm pet
 * (at most {@link MOOD_REST}) adopts the mood at `gain × theirs` when that is at least {@link MOOD_FAINT}, with a
 * fresh hold. Anyone else keeps its mood. `gain` is at most 0.8, so nobody ends above the one it caught from.
 * Several neighbours are applied one after the other in species order, each as it stood when the beat began.
 */
export function caught(mine: Feeling, theirs: Feeling, affinity: number, sociability: number, tick: Ticks): Feeling {
  if (!(theirs.intensity > MOOD_REST)) return mine;
  const gain = MOOD_SPREADS[moodIndex(theirs.mood)]! * sociability * (0.5 + 0.5 * affinity);
  if (!(gain > 0)) return mine;
  if (mine.mood === theirs.mood) {
    const lift = gain * (theirs.intensity - mine.intensity);
    return lift > 0 ? { mood: mine.mood, intensity: mine.intensity + lift, since: mine.since } : mine;
  }
  if (mine.intensity > MOOD_REST) return mine;
  const adopted = gain * theirs.intensity;
  return adopted >= MOOD_FAINT ? { mood: theirs.mood, intensity: adopted, since: tick } : mine;
}
//#endregion 🔖️Contagion

//#region 🔖️States
/** 🚩️ The state a pet is in and the tick it began. */
export type Standing = { readonly state: Slug; readonly since: Ticks };

/** 🔦️ The state of a species by its id, or `null`. */
function stateOf(species: Species, id: Slug): SpeciesState | null {
  for (const state of species.states) if (state.id === id) return state;
  return null;
}

/** ⏱️ For how many ticks a state lasts: `floor(lasts × 64 + 0.5)`, at least 1; 0 says it lasts until something changes it. */
export function lastingTicks(state: SpeciesState): Ticks {
  if (state.lasts === undefined) return 0;
  const ticks = Math.floor(state.lasts * TICKS_PER_SECOND + 0.5);
  return ticks > 1 ? ticks : 1;
}

/** ➡️ The state a lasting state gives way to: the one `then` names, or the resting state (the first) when it names none of the species. */
function followerOf(species: Species, state: SpeciesState): Slug {
  return state.then !== undefined && stateOf(species, state.then) !== null ? state.then : species.states[0]!.id;
}

/** 🕰️ The state of a pet at `tick` when it entered `state` at `since`, a closed form over any number of ticks: a state that `lasts` gives way to its `then` when its time is up, that one to its own, and so on.
 *
 * A state without `lasts`, and one that would give way to itself, stays. A chain that runs in a circle is not
 * walked lap by lap: once the walk has taken as many steps as the species has states it is on the circle, whole
 * laps are taken off at once and the rest is walked. An id the species does not know counts as its resting state.
 */
export function stateAt(species: Species, state: Slug, since: Ticks, tick: Ticks): Standing {
  const count = species.states.length;
  if (count === 0) return { state, since };
  let current = stateOf(species, state) ?? species.states[0]!;
  let began = since;
  for (let steps = 0; steps < count + count + 2; steps++) {
    const span = lastingTicks(current);
    if (span === 0) break;
    const next = followerOf(species, current);
    if (next === current.id || tick - began < span) break;
    if (steps === count) {
      let lap = 0;
      let walker = current;
      for (let turn = 0; turn < count; turn++) {
        lap = lap + lastingTicks(walker);
        walker = stateOf(species, followerOf(species, walker))!;
        if (walker.id === current.id) break;
      }
      began = began + Math.floor((tick - began) / lap) * lap;
      continue;
    }
    began = began + span;
    current = stateOf(species, next)!;
  }
  return { state: current.id, since: began };
}

/** 🔚️ The tick at which a state entered at `since` gives way to the next one, or `null` when it stays until something changes it. */
export function stateEnds(species: Species, state: Slug, since: Ticks): Ticks | null {
  const current = stateOf(species, state);
  if (current === null) return null;
  const span = lastingTicks(current);
  return span === 0 || followerOf(species, current) === current.id ? null : since + span;
}

/** 🪜️ The state ladder of a species: its states in the order they are authored, from the resting state up. */
export function ladderOf(species: Species): Slug[] {
  const ladder: Slug[] = [];
  for (const state of species.states) ladder.push(state.id);
  return ladder;
}

/** 🧗️ The rungs a trick steps along: the states it lists in `from`, in that order, when it lists any that the species has; the whole ladder otherwise. A species whose resting state lies in the middle of its ladder (a sun that can dim as well as blaze) says so here. */
export function rungsOf(species: Species, trick: Trick): Slug[] {
  const rungs: Slug[] = [];
  if (trick.from !== undefined) for (const id of trick.from) if (stateOf(species, id) !== null) rungs.push(id);
  return rungs.length > 0 ? rungs : ladderOf(species);
}

/** 👣️ One step along rungs: up for a positive direction, down for a negative one, held at both ends (a further turn is a flourish that changes nothing); a state that is not a rung stays. */
export function stepRung(rungs: readonly Slug[], state: Slug, direction: number): Slug {
  const index = rungs.indexOf(state);
  if (index < 0 || direction === 0) return state;
  const next = direction > 0 ? index + 1 : index - 1;
  return next < 0 || next >= rungs.length ? state : rungs[next]!;
}

/** 🔼️ One step up (positive direction) or down (negative) the {@link ladderOf} a species. */
export function stepState(species: Species, state: Slug, direction: number): Slug {
  return stepRung(ladderOf(species), state, direction);
}

/** 🎬️ The state a trick leaves a pet in: the state `to` names when the species has it; else, for a trick cued by circling, one step up its rungs (`circle`) or down (`countercircle`, when it is not also cued by `circle`); else the state it was in. */
export function stateAfterTrick(species: Species, state: Slug, trick: Trick): Slug {
  if (trick.to !== undefined) return stateOf(species, trick.to) !== null ? trick.to : state;
  const direction = trick.cues.includes("circle") ? 1 : trick.cues.includes("countercircle") ? -1 : 0;
  return direction === 0 ? state : stepRung(rungsOf(species, trick), state, direction);
}
//#endregion 🔖️States

//#region 🔖️Tricks
/** 🙋️ Whether a pet that shows a mood performs by itself (cues `whim` and `show`), in `MOODS` order: the content, happy, playful, curious and proud do; the sleepy, grumpy, sad and scared do not. What the learner asks for is always on offer. */
export const WILLING_MOODS: readonly boolean[] = [true, true, true, true, true, false, false, false, false];

/** 🎪️ The tricks of a species that a cue can set off right now, in authored order: the trick lists the cue, it is on offer in the state (`from` absent or naming it), and — for the pet's own cues `whim` and `show` — the mood it shows ({@link shownMood}) is a willing one ({@link WILLING_MOODS}). */
export function tricksFor(species: Species, cue: Cue, state: Slug, feeling: Feeling): Trick[] {
  const tricks: Trick[] = [];
  if ((cue === "whim" || cue === "show") && !WILLING_MOODS[moodIndex(shownMood(feeling.mood, feeling.intensity))]!) return tricks;
  for (const trick of species.tricks) if (trick.cues.includes(cue) && (trick.from === undefined || trick.from.includes(state))) tricks.push(trick);
  return tricks;
}

/** 👆️ The trick the n-th click of a streak sets off (`index` 0 for the first trick click): the click tricks on offer, in authored order, round and round; `null` when none is on offer. */
export function clickTrick(species: Species, state: Slug, feeling: Feeling, index: number): Trick | null {
  const tricks = tricksFor(species, "click", state, feeling);
  const count = tricks.length;
  if (count === 0) return null;
  const turn = Math.floor(index) % count;
  return tricks[turn < 0 ? turn + count : turn] ?? null;
}

/** ⭐️ How many times likelier a pet picks, among the tricks on offer, one that leaves the mood it shows already: it stays in character. */
export const WHIM_FAVOR = 3;

/** 🎲️ The trick a unit draw picks among those a cue can set off: {@link weightedIndex} over a weight of 1 each, {@link WHIM_FAVOR} for a trick that leaves the mood the pet shows; `null` when none is on offer. */
function pick(species: Species, cue: Cue, state: Slug, feeling: Feeling, unit: number): Trick | null {
  const tricks = tricksFor(species, cue, state, feeling);
  const shown = shownMood(feeling.mood, feeling.intensity);
  const weights: number[] = [];
  for (const trick of tricks) weights.push(trick.mood === shown ? WHIM_FAVOR : 1);
  const index = weightedIndex(weights, unit);
  return index < 0 ? null : tricks[index]!;
}

/** 💭️ The trick a pet performs on a whim, for a unit draw: one of its `whim` tricks on offer, favouring the mood it shows; `null` when it has none or is in no mood for it. */
export function whimTrick(species: Species, state: Slug, feeling: Feeling, unit: number): Trick | null {
  return pick(species, "whim", state, feeling, unit);
}

/** 🎤️ The trick a pet shows another pet, for a unit draw: one of its `show` tricks on offer, favouring the mood it shows; `null` when it has none or is in no mood for it. */
export function showTrick(species: Species, state: Slug, feeling: Feeling, unit: number): Trick | null {
  return pick(species, "show", state, feeling, unit);
}
//#endregion 🔖️Tricks

//#region 🔖️Chemistry
/** 👀️ What the chemistry needs to know of an actor: its species, its state and for how many ticks it has held it, its mood with its intensity, its activity and the trick it performs (`null` when none), and its body — `x` the middle and `y` the feet of a box `width` wide that rises `height` above them (viewport pixels, y pointing down). */
export type Sighting = {
  readonly species: Slug;
  readonly state: Slug;
  readonly held: Ticks;
  readonly mood: Mood;
  readonly intensity: number;
  readonly activity: Activity;
  readonly trick: Slug | null;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
};

/** 🧪️ Every how many ticks the chemistry of a stage is looked at (twice a second). */
export const CHEMISTRY_BEAT: Ticks = 32;

/** 💧️ The amount of the mood an effect gives when it names none. */
export const EFFECT_AMOUNT = 0.6;

/** ⏲️ For how many ticks the encounter a reaction promises a pair is owed (30 s): when the two meet within that time, it is the one named. */
export const LEAN_TICKS: Ticks = 1920;

/** 📐️ Whether two bodies are within `reach` pixels of each other: the gap between the two boxes (0 when they touch or overlap), across and upright taken together, is at most `reach`. */
export function nearby(first: Sighting, second: Sighting, reach: number): boolean {
  const across = Math.abs(first.x - second.x) - (first.width + second.width) / 2;
  const under = first.y - first.height - second.y;
  const over = second.y - second.height - first.y;
  const wide = across > 0 ? across : 0;
  const tall = under > 0 ? under : over > 0 ? over : 0;
  return wide * wide + tall * tall <= reach * reach;
}

/** 🔭️ Whether the first body is where a reaction wants it, seen from the second. `above` and `below`: the two share a column (their widths overlap) and the middle of the first is higher, or lower. `beside`: they share a row (their heights overlap) but no column. `any`: wherever it is. Bodies that share neither column nor row are diagonal to each other and only `any` takes them. */
export function seen(first: Sighting, second: Sighting, where: NonNullable<Reaction["where"]>): boolean {
  if (where === "any") return true;
  const column = Math.abs(first.x - second.x) < (first.width + second.width) / 2;
  if (where === "beside") return !column && first.y - first.height < second.y && second.y - second.height < first.y;
  const rise = first.y - first.height / 2 - (second.y - second.height / 2);
  return column && (where === "above" ? rise < 0 : rise > 0);
}

/** ⌛️ For how many ticks an actor must have held its state for a trait that asks `held` seconds of it: `floor(held × 64 + 0.5)`, at least 1. */
export function heldTicks(held: number): Ticks {
  const ticks = Math.floor(held * TICKS_PER_SECOND + 0.5);
  return ticks > 1 ? ticks : 1;
}

/** 🔎️ Whether an actor is what one side of a reaction asks for: the species (any when the trait names none), and the state — held for at least {@link heldTicks} of `held` —, the mood, the activity and the trick where the trait names them. The mood is the one the actor shows ({@link shownMood}): `content` asks for a calm pet, any other mood for a pet that is stirred into it. */
export function matches(trait: Trait, sighting: Sighting): boolean {
  return (
    (trait.species === undefined || trait.species === sighting.species) &&
    (trait.state === undefined || trait.state === sighting.state) &&
    (trait.held === undefined || sighting.held >= heldTicks(trait.held)) &&
    (trait.mood === undefined || trait.mood === shownMood(sighting.mood, sighting.intensity)) &&
    (trait.activity === undefined || trait.activity === sighting.activity) &&
    (trait.trick === undefined || trait.trick === sighting.trick)
  );
}

/** 🎯️ A reaction that is due for a pair: the index of the reaction in `menagerie.chemistry`, the indices of the two sightings, and whether it takes a unit draw (it has a `chance`). */
export type Trial = { readonly reaction: number; readonly when: number; readonly near: number; readonly chancy: boolean };

/** 💥️ What a reaction does to one actor (`on`, with `other` the second of the pair): the state it enters (`null`: none; the state it is in already begins anew), the mood it gets an impulse of and its amount, the shift of the rapport of the two, the encounter the two are promised, the trick it performs and what it is set off on by itself. */
export type Consequence = {
  readonly reaction: Slug;
  readonly on: Slug;
  readonly other: Slug;
  readonly state: Slug | null;
  readonly mood: Mood | null;
  readonly amount: number;
  readonly rapport: number;
  readonly encounter: NonNullable<Effect["encounter"]> | null;
  readonly trick: Slug | null;
  readonly activity: NonNullable<Effect["activity"]> | null;
};

/** ⚗️ What one beat of chemistry yields: the consequences to apply in order, the coolings to keep for the next beat, and how many of the units were used. */
export type Chemistry = { readonly consequences: readonly Consequence[]; readonly coolings: readonly Cooling[]; readonly drawn: number };

/** 🗂️ The species of a menagerie by its id, or `null`. */
function speciesOf(menagerie: Menagerie, id: Slug): Species | null {
  for (const species of menagerie.species) if (species.id === id) return species;
  return null;
}

/** ❄️ Whether a reaction is still cooling for an ordered pair of species at `tick`. */
function cooling(coolings: readonly Cooling[], reaction: Slug, when: Slug, near: Slug, tick: Ticks): boolean {
  for (const entry of coolings) if (entry.until > tick && entry.reaction === reaction && entry.when === when && entry.near === near) return true;
  return false;
}

/** 🔗️ Whether a list of pairs holds the two species, in either order. */
function joined(pairs: readonly (readonly [Slug, Slug])[], one: Slug, other: Slug): boolean {
  for (const pair of pairs) if ((pair[0] === one && pair[1] === other) || (pair[0] === other && pair[1] === one)) return true;
  return false;
}

/** 🔁️ For how many ticks a reaction cools after its turn: `floor(every × 64 + 0.5)`, at least 1. */
export function everyTicks(reaction: Reaction): Ticks {
  const ticks = Math.floor(reaction.every * TICKS_PER_SECOND + 0.5);
  return ticks > 1 ? ticks : 1;
}

/** 🚧️ Whether a third actor of `order` — neither the `when` nor the `near` actor of a trial — matches `unless` and is {@link nearby} the `near` actor within `within` pixels: then the reaction holds back. */
function barred(unless: Trait, sightings: readonly Sighting[], order: readonly number[], when: number, near: number, within: number): boolean {
  for (const third of order) if (third !== when && third !== near && matches(unless, sightings[third]!) && nearby(sightings[third]!, sightings[near]!, within)) return true;
  return false;
}

/** 🧫️ The reactions that are due at `tick`, in the order their units are drawn and their effects applied: for every ordered pair of actors — the first by the place of its species in `menagerie.species`, then the second likewise — every reaction in authored order whose `when` the first and whose `near` the second {@link matches}, whose bodies are {@link nearby} within `within` and {@link seen} as `where` asks (the first seen from the second), whose `affinity` bounds hold the affinity of the two (`affinityOf` with the `rapports` of the stage), which no third actor bars ({@link barred} by `unless`), and which is not cooling for the pair. Actors of a species the menagerie does not know are passed over. */
export function trialsOf(menagerie: Menagerie, sightings: readonly Sighting[], coolings: readonly Cooling[], tick: Ticks, rapports: readonly Rapport[]): Trial[] {
  const order: number[] = [];
  for (const species of menagerie.species) for (let index = 0; index < sightings.length; index++) if (sightings[index]!.species === species.id) order.push(index);
  const trials: Trial[] = [];
  for (const when of order) {
    const first = sightings[when]!;
    for (const near of order) {
      if (near === when) continue;
      const second = sightings[near]!;
      for (let index = 0; index < menagerie.chemistry.length; index++) {
        const reaction = menagerie.chemistry[index]!;
        if (!matches(reaction.when, first) || !matches(reaction.near, second)) continue;
        if (!nearby(first, second, reaction.within) || !seen(first, second, reaction.where ?? "any")) continue;
        if (reaction.affinity !== undefined) {
          const affinity = affinityOf(menagerie, rapports, first.species, second.species);
          if (affinity < reaction.affinity[0] || affinity > reaction.affinity[1]) continue;
        }
        if (reaction.unless !== undefined && barred(reaction.unless, sightings, order, when, near, reaction.within)) continue;
        if (cooling(coolings, reaction.id, first.species, second.species, tick)) continue;
        trials.push({ reaction: index, when, near, chancy: reaction.chance !== undefined });
      }
    }
  }
  return trials;
}

/** 🎰️ How many units the trials of a beat take: one per trial with a chance, in trial order. The stage draws that many from its stream and hands them to {@link reactionsOf}. */
export function drawsOf(trials: readonly Trial[]): number {
  let draws = 0;
  for (const trial of trials) if (trial.chancy) draws = draws + 1;
  return draws;
}

/** 🔥️ One beat of chemistry: what the reactions of a menagerie do to the actors of a stage as they stand at `tick`.
 *
 * Every trial of {@link trialsOf} has its turn in order, and cools for its pair for {@link everyTicks} from `tick`
 * whether it happens or not — so a reaction is tried at most once in `every` seconds. A reaction with a `chance`
 * takes the next unit and happens when `unit < chance` (a unit that was not handed in counts as 1: it does not
 * happen). Its effects become consequences in authored order: the state when the species of the actor has it, the
 * mood with its amount ({@link EFFECT_AMOUNT} when none is named), the rapport, the encounter, the trick when the
 * species has it and offers it in the state the actor is in, and the activity. Everybody is judged as it stood when
 * the beat began — a state set in this beat sets nothing else off before the next one — and nothing is applied twice
 * in a beat: an actor takes the first state, the first mood, the first trick and the first activity that reach it, a
 * pair the first shift of its rapport and the first promise of an encounter. An effect of which nothing is left is
 * dropped. The coolings returned are those still running, then the new ones in trial order.
 */
export function reactionsOf(menagerie: Menagerie, sightings: readonly Sighting[], tick: Ticks, coolings: readonly Cooling[], units: readonly number[], rapports: readonly Rapport[]): Chemistry {
  const trials = trialsOf(menagerie, sightings, coolings, tick, rapports);
  const kept: Cooling[] = [];
  for (const entry of coolings) if (entry.until > tick) kept.push(entry);
  const consequences: Consequence[] = [];
  const stated: Slug[] = [];
  const moved: Slug[] = [];
  const tricked: Slug[] = [];
  const started: Slug[] = [];
  const shifted: (readonly [Slug, Slug])[] = [];
  const promised: (readonly [Slug, Slug])[] = [];
  let drawn = 0;
  for (const trial of trials) {
    const reaction = menagerie.chemistry[trial.reaction]!;
    const first = sightings[trial.when]!;
    const second = sightings[trial.near]!;
    kept.push({ reaction: reaction.id, when: first.species, near: second.species, until: tick + everyTicks(reaction) });
    if (reaction.chance !== undefined) {
      const unit = drawn < units.length ? units[drawn]! : 1;
      drawn = drawn + 1;
      if (!(unit < reaction.chance)) continue;
    }
    for (const effect of reaction.then) {
      const actor = effect.on === "when" ? first : second;
      const other = effect.on === "when" ? second : first;
      const species = speciesOf(menagerie, actor.species);
      if (species === null) continue;
      let state: Slug | null = null;
      if (effect.state !== undefined && stateOf(species, effect.state) !== null && !stated.includes(actor.species)) {
        state = effect.state;
        stated.push(actor.species);
      }
      let mood: Mood | null = null;
      if (effect.mood !== undefined && !moved.includes(actor.species)) {
        mood = effect.mood;
        moved.push(actor.species);
      }
      let rapport = 0;
      if (effect.rapport !== undefined && effect.rapport !== 0 && !joined(shifted, actor.species, other.species)) {
        rapport = effect.rapport;
        shifted.push([actor.species, other.species]);
      }
      let encounter: NonNullable<Effect["encounter"]> | null = null;
      if (effect.encounter !== undefined && !joined(promised, actor.species, other.species)) {
        encounter = effect.encounter;
        promised.push([actor.species, other.species]);
      }
      let trick: Slug | null = null;
      if (effect.trick !== undefined && !tricked.includes(actor.species)) {
        for (const offered of species.tricks) {
          if (offered.id !== effect.trick || (offered.from !== undefined && !offered.from.includes(actor.state))) continue;
          trick = offered.id;
          tricked.push(actor.species);
          break;
        }
      }
      let activity: NonNullable<Effect["activity"]> | null = null;
      if (effect.activity !== undefined && !started.includes(actor.species)) {
        activity = effect.activity;
        started.push(actor.species);
      }
      if (state === null && mood === null && rapport === 0 && encounter === null && trick === null && activity === null) continue;
      consequences.push({ reaction: reaction.id, on: actor.species, other: other.species, state, mood, amount: mood === null ? 0 : (effect.amount ?? EFFECT_AMOUNT), rapport, encounter, trick, activity });
    }
  }
  return { consequences, coolings: kept, drawn };
}
//#endregion 🔖️Chemistry
