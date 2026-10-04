/** 💞️ Sociability: what pets do with each other and with the learner who clicks them — the pairing on whole seconds, coming together, the encounter as the moods of the two tip it (and the encounter the chemistry promised them), showing off a trick to a friend, the sulk after a squabble and its mending, the rapport that moves and fades, moods that travel between neighbours, and the click and the deed of the learner that warm a pet up.
 *
 * Whatever happens between two actors, or between an actor and the learner's click, belongs here.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ../💗️feeling/🟦️.ts — `encounterBias`, `caught`, `clickTrick`
 * @see ../👆️gesture/🟦️.ts — `warmthAfter`, the heat of repeated attention
 * @see ./🦀️.rs — the Rust twin
 */

import { type Activity, type Deed, type Menagerie, type Pledge, type Rapport, type Slug, type Ticks } from "../../🧬️schema/🟦️.ts";
import { randomPick, randomWords, unitOf, weightedIndex } from "../🎲️randomness/🟦️.ts";
import { PURR_TICKS, hail, heed, loose, shrug } from "../👀️attention/🟦️.ts";
import { warmthAfter } from "../👆️gesture/🟦️.ts";
import { CONTAGION_REACH, LEAN_TICKS, caught, clickTrick, encounterBias, showTrick, swayedShares, whimTrick } from "../💗️feeling/🟦️.ts";
import { SEAM } from "../🚧️clearance/🟦️.ts";
import { COMFORT_GAP, clearway } from "../📏️spacing/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";
import { actorKey, astir, clipAt, facingTo, feel, indexOf, perform, present, purr, shift, stageKey, standing, type Draft } from "../📝️draft/🟦️.ts";
import { pairingTick, sociable } from "../🗓️schedule/🟦️.ts";
import { attend, paced, stroll } from "../🚶️locomotion/🟦️.ts";
import { AFFINITY_FLOOR, ENCOUNTERS, MODE_LIMITS, RAPPORT_SPAN, affinityOf, dwellOf, encounterShares, needsAfter, rapportAfter, rapportFaded, type Encounter } from "../🧠️behavior/🟦️.ts";

//#region 🔖️Constants
const ENCOUNTER_REACH = 12;
const ENCOUNTER_RISE = 3;
const PAIR_WEIGHT = 0.25;
const NO_RAPPORTS: readonly Rapport[] = [];
//#endregion 🔖️Constants

//#region 🔖️Rapport
/** 😤️ An actor sulks for a drawn span, turning its back on its partner. */
export function sulk(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const words = randomWords(actorKey(draft, index), 2);
  shift(draft, index, "sulk", now);
  body.until = now + dwellOf("sulk", draft.mode, unitOf(words[0]!));
  body.clip = clipAt(draft.kinds[index]!, "sulk", unitOf(words[1]!));
}

/** 🍂️ Every rapport fades over the ticks since the last change of any of them; what has faded to 0 is forgotten. `met` becomes `now`. */
function recall(draft: Draft, now: Ticks): void {
  const elapsed = now - draft.met;
  if (elapsed > 0 && draft.rapports.length > 0) {
    const faded: Rapport[] = [];
    for (const rapport of draft.rapports) {
      const drift = rapportFaded(rapport.drift, elapsed);
      if (drift !== 0) faded.push({ between: rapport.between, drift });
    }
    draft.rapports = faded;
  }
  draft.met = now;
}

/** 🪢️ The rapport of two actors after something between them (`rapportAfter`); a new pair is listed last, the earlier species of the menagerie first, and a drift of 0 is forgotten. */
function bond(draft: Draft, first: number, second: number, activity: Activity): void {
  const early = first < second ? first : second;
  const late = first < second ? second : first;
  const a = draft.actors[early]!.species;
  const b = draft.actors[late]!.species;
  const kept: Rapport[] = [];
  let found = false;
  for (const rapport of draft.rapports) {
    if (rapport.between[0] !== a || rapport.between[1] !== b) {
      kept.push(rapport);
      continue;
    }
    found = true;
    const drift = rapportAfter(rapport.drift, activity);
    if (drift !== 0) kept.push({ between: rapport.between, drift });
  }
  if (!found) {
    const drift = rapportAfter(0, activity);
    if (drift !== 0) kept.push({ between: [a, b], drift });
  }
  draft.rapports = kept;
}

/** 🕊️ The end of a sulk: the actor feels better (`mended`) and, once the partner has stopped sulking too, the two mend their rapport by a little; the actor lets go of its partner either way. */
export function reconcile(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.partner === null) return;
  const other = indexOf(draft.actors, body.partner);
  const pending = other >= 0 && draft.actors[other]!.activity === "sulk" && draft.actors[other]!.partner === body.species;
  if (other >= 0 && !pending) {
    recall(draft, now);
    bond(draft, index, other, "sulk");
  }
  feel(draft, index, "mended", now);
  body.partner = null;
}

/** 🎚️ The chemistry shifts the rapport of two actors by `delta` at tick `now`: the rapports fade up to now first, the drift stays inside `[−RAPPORT_SPAN, RAPPORT_SPAN]`, a new pair is listed last with the earlier species of the menagerie first, and a drift of 0 is forgotten. */
export function nudge(draft: Draft, first: number, second: number, delta: number, now: Ticks): void {
  recall(draft, now);
  const early = first < second ? first : second;
  const late = first < second ? second : first;
  const a = draft.actors[early]!.species;
  const b = draft.actors[late]!.species;
  const kept: Rapport[] = [];
  let found = false;
  for (const rapport of draft.rapports) {
    if (rapport.between[0] !== a || rapport.between[1] !== b) {
      kept.push(rapport);
      continue;
    }
    found = true;
    const drift = clamp(rapport.drift + delta, 0 - RAPPORT_SPAN, RAPPORT_SPAN);
    if (drift !== 0) kept.push({ between: rapport.between, drift });
  }
  if (!found) {
    const drift = clamp(delta, 0 - RAPPORT_SPAN, RAPPORT_SPAN);
    if (drift !== 0) kept.push({ between: [a, b], drift });
  }
  draft.rapports = kept;
}

/** 💍️ The chemistry promises two species an encounter at tick `now`: the next time they are paired within {@link LEAN_TICKS} it is this one. A promise the pair had is replaced; promises that ran out are forgotten. */
export function pledge(draft: Draft, one: Slug, other: Slug, encounter: Encounter, now: Ticks): void {
  const kept: Pledge[] = [];
  for (const entry of draft.pledges) if (entry.until > now && !joins(entry.between, one, other)) kept.push(entry);
  kept.push({ between: [one, other], encounter, until: now + LEAN_TICKS });
  draft.pledges = kept;
}

/** 🔗️ Whether a pair names the two species, in either order. */
function joins(between: readonly [Slug, Slug], one: Slug, other: Slug): boolean {
  return (between[0] === one && between[1] === other) || (between[0] === other && between[1] === one);
}

/** 🎫️ The encounter the chemistry promised a pair that meets at tick `now`, or `null`: a promise that is redeemed, and every promise that ran out, is forgotten. */
function redeem(draft: Draft, one: Slug, other: Slug, now: Ticks): Encounter | null {
  let promised: Encounter | null = null;
  const kept: Pledge[] = [];
  for (const entry of draft.pledges) {
    if (entry.until <= now) continue;
    if (promised === null && joins(entry.between, one, other)) promised = entry.encounter;
    else kept.push(entry);
  }
  if (kept.length !== draft.pledges.length) draft.pledges = kept;
  return promised;
}
//#endregion 🔖️Rapport

//#region 🔖️Encounters
/** 💌️ On a whole second the stage may pair two sociable actors that stand near each other (no farther apart than 12 of their mean widths, no more than 3 in height): one stage draw picks a pair, weighted `(0.25 + |authored affinity|) × mean sociability` (pairs that feel something for each other meet more often than strangers, and whoever has just had company lets others go first), and decides with the chance `rate × mean sociability`. Sociability is the need as it stands now. The pair then approaches; when the mode has room for one mover only, the more sociable of the two walks. */
export function pair(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  if (pairingTick(draft, now) !== now) return;
  const limits = MODE_LIMITS[draft.mode];
  let movers = 0;
  for (const body of draft.actors) if (astir(draft, body)) movers++;
  if (movers >= limits.movers) return;
  const firsts: number[] = [];
  const seconds: number[] = [];
  const drives: number[] = [];
  const socials: number[] = [];
  const weights: number[] = [];
  for (let first = 0; first < draft.actors.length; first++) {
    const one = draft.actors[first]!;
    if (!sociable(one, now)) continue;
    const eager = needsAfter(one.needs, "idle", now - one.since, draft.kinds[first]!.temperament).sociability;
    for (let second = first + 1; second < draft.actors.length; second++) {
      const two = draft.actors[second]!;
      if (!sociable(two, now)) continue;
      const width = (draft.kinds[first]!.size.width + draft.kinds[second]!.size.width) / 2;
      if (Math.abs(one.x - two.x) > ENCOUNTER_REACH * width || Math.abs(one.y - two.y) > ENCOUNTER_RISE * width || parted(draft, first, second)) continue;
      const keen = needsAfter(two.needs, "idle", now - two.since, draft.kinds[second]!.temperament).sociability;
      const social = (eager + keen) / 2;
      firsts.push(first);
      seconds.push(second);
      drives.push(eager >= keen ? 1 : 2);
      socials.push(social);
      weights.push((PAIR_WEIGHT + Math.abs(affinityOf(menagerie, NO_RAPPORTS, one.species, two.species))) * social);
    }
  }
  if (firsts.length === 0) return;
  const key = stageKey(draft);
  const chosen = randomPick(key, weights);
  if (chosen < 0 || !(unitOf(randomWords(key, 2)[1]!) < limits.encounterRate * socials[chosen]!)) return;
  approach(draft, firsts[chosen]!, seconds[chosen]!, limits.movers - movers >= 2 ? 0 : drives[chosen]!, now);
}

/** 🚻️ Whether somebody stands between two actors of one surface, so that they could not come together without walking through it. */
function parted(draft: Draft, first: number, second: number): boolean {
  const one = draft.actors[first]!;
  const two = draft.actors[second]!;
  if (one.footing !== "perch" || two.footing !== "perch" || one.perch !== two.perch) return false;
  const low = Math.min(one.x, two.x);
  const high = Math.max(one.x, two.x);
  for (let other = 0; other < draft.actors.length; other++) {
    const between = draft.actors[other]!;
    if (other !== first && other !== second && between.perch === one.perch && between.x >= low && between.x <= high) return true;
  }
  return false;
}

/** 🧭️ Where an actor may walk to on its perch to meet its partner (the actor at `partner`), as close to `x` as its clear way and its gait allow. */
function reachable(draft: Draft, index: number, partner: number, x: number): number {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const [low, high] = clearway(draft, index, COMFORT_GAP, partner);
  return paced(kind, clipAt(kind, "walk", 0), body.x, clamp(x, low, high));
}

/** 🤝️ Two actors become partners and come together until their boxes are as far apart as both their encounter poses reach out (the sum of both species' `reach`, never less than the comfortable gap of two bodies that touch, and a seam), so that no pose of theirs ever touches the other: both walk to the middle (`walker` 0), or only the first (1) or the second (2) walks while the other waits, and whoever need not move waits at once. Each stays on its own perch. */
function approach(draft: Draft, first: number, second: number, walker: number, now: Ticks): void {
  const one = draft.actors[first]!;
  const two = draft.actors[second]!;
  one.partner = two.species;
  two.partner = one.species;
  const apart = (draft.kinds[first]!.size.width + draft.kinds[second]!.size.width) / 2 + Math.max(draft.kinds[first]!.reach + draft.kinds[second]!.reach, COMFORT_GAP) + SEAM;
  const side = facingTo(two.x, one.x);
  const close = Math.abs(two.x - one.x) <= apart;
  const middle = (one.x + two.x) / 2;
  const near = close || walker === 2 ? one.x : reachable(draft, first, second, walker === 1 ? two.x - side * apart : middle - (side * apart) / 2);
  const far = close || walker === 1 ? two.x : reachable(draft, second, first, walker === 2 ? one.x + side * apart : middle + (side * apart) / 2);
  if (near === one.x) attend(draft, first, now);
  else stroll(draft, first, near, clipAt(draft.kinds[first]!, "walk", 0), now);
  if (far === two.x) attend(draft, second, now);
  else stroll(draft, second, far, clipAt(draft.kinds[second]!, "walk", 0), now);
}

/** 🎉️ Partners that both wait begin their encounter: the rapports fade up to now, one stage draw picks the kind, the span they share and a clip for each; they face each other, their rapport moves and they feel it (`welcomed`, `cuddled`, `squabbled`).
 *
 * The kind is the encounter the chemistry promised the pair, when it did; else a pick from the shares of their
 * affinity tipped by what the two feel (`encounterBias`: the affinity shifted by their moods, the shares swayed). In
 * a greeting the prouder of two proud pets shows off instead: it performs a trick it offers to `show` (picked with
 * its clip word) for as long as the greeting lasts or the trick needs, and the other is entertained.
 */
export function meet(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  for (let first = 0; first < draft.actors.length; first++) {
    const one = draft.actors[first]!;
    if (one.partner === null || one.activity !== "idle") continue;
    const second = indexOf(draft.actors, one.partner);
    if (second <= first) continue;
    const two = draft.actors[second]!;
    if (two.activity !== "idle" || two.partner !== one.species) continue;
    recall(draft, now);
    const words = randomWords(stageKey(draft), 4);
    const leaning = encounterBias(present(draft, first, now), present(draft, second, now));
    const affinity = clamp(affinityOf(menagerie, draft.rapports, one.species, two.species) + leaning.affinity, AFFINITY_FLOOR, 1);
    const picked = weightedIndex(swayedShares(encounterShares(affinity), leaning), unitOf(words[0]!));
    const kind = redeem(draft, one.species, two.species, now) ?? ENCOUNTERS[picked < 0 ? 0 : picked]!;
    const span = dwellOf(kind, draft.mode, unitOf(words[1]!));
    shift(draft, first, kind, now);
    shift(draft, second, kind, now);
    one.until = now + span;
    two.until = now + span;
    one.clip = clipAt(draft.kinds[first]!, kind, unitOf(words[2]!));
    two.clip = clipAt(draft.kinds[second]!, kind, unitOf(words[3]!));
    bond(draft, first, second, kind);
    const occasion = kind === "greet" ? "welcomed" : kind === "cuddle" ? "cuddled" : "squabbled";
    feel(draft, first, occasion, now);
    feel(draft, second, occasion, now);
    if (kind !== "greet" || leaning.show < 0) continue;
    const shower = leaning.show === 0 ? first : second;
    const watcher = leaning.show === 0 ? second : first;
    const trick = showTrick(draft.kinds[shower]!, standing(draft, shower, now).state, present(draft, shower, now), unitOf(words[leaning.show === 0 ? 2 : 3]!));
    if (trick === null) continue;
    perform(draft, shower, trick, now);
    const end = draft.actors[shower]!.until > now + span ? draft.actors[shower]!.until : now + span;
    draft.actors[shower]!.until = end;
    draft.actors[watcher]!.until = end;
    feel(draft, watcher, "entertained", now);
  }
}
//#endregion 🔖️Encounters

//#region 🔖️Contagion
/** 🦠️ One beat of moods travelling at tick `now`: every actor that stays catches the moods of its neighbours (`caught`) — those whose bodies are no farther apart across than {@link CONTAGION_REACH} of their mean widths and no farther apart upright than their heights together —, givers in `menagerie.species` order, everybody as they stood when the beat began, with its own temperament's sociability and the affinity of the two. A feeling that caught nothing is left as it is. */
export function contagion(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  const count = draft.actors.length;
  if (count < 2) return;
  const felt = draft.actors.map((_, index) => present(draft, index, now));
  for (let catcher = 0; catcher < count; catcher++) {
    const body = draft.actors[catcher]!;
    if (body.leaving) continue;
    const kind = draft.kinds[catcher]!;
    let mine = felt[catcher]!;
    for (let giver = 0; giver < count; giver++) {
      const other = draft.actors[giver]!;
      if (giver === catcher || other.leaving) continue;
      const peer = draft.kinds[giver]!;
      const across = Math.abs(body.x - other.x) - (kind.size.width + peer.size.width) / 2;
      if (across > (CONTAGION_REACH * (kind.size.width + peer.size.width)) / 2 || Math.abs(body.y - other.y) > kind.size.height + peer.size.height) continue;
      mine = caught(mine, felt[giver]!, affinityOf(menagerie, draft.rapports, body.species, other.species), kind.temperament.sociability, now);
    }
    if (mine !== felt[catcher]) body.feeling = mine;
  }
}
//#endregion 🔖️Contagion

//#region 🔖️Learner
/** 👆️ The learner clicked an actor with the pointer at `x` (a press and a release without movement): the actor warms up (`warmthAfter`) and answers by the tier the heat reaches — a hello (`greeted`, it greets towards the click), the trick of its species for this click (the click tricks it offers in its state and mood, in authored order, round and round; a hello when it has none), a purr (`purred`, drawn out by every further click), and once it has had enough a shrug (`pestered`, it turns away) and then, for a while, nothing but a glance. A sulker is reconciled first and a sleeper wakes (`woken`); an actor that is not free for the hand ({@link loose}: busy with its partner, off its perch or in the middle of what it may not drop) only feels it. Ignored on a still stage, while play is not permitted and by a leaver. */
export function clicked(draft: Draft, species: Slug, x: number): void {
  const index = indexOf(draft.actors, species);
  if (index < 0 || draft.mode === "still" || !draft.play) return;
  const body = draft.actors[index]!;
  if (body.leaving) return;
  const now = draft.tick;
  body.warmth = warmthAfter(body.warmth, now, "click");
  const tier = body.warmth.tier;
  if (tier === "enough" && body.warmth.run > 1) return;
  const reached = loose(body) || (body.activity === "sulk" && body.footing === "perch" && body.perch !== null);
  if (!reached) {
    feel(draft, index, tier === "purr" ? "purred" : tier === "enough" ? "pestered" : "greeted", now);
    return;
  }
  if (body.activity === "sulk") reconcile(draft, index, now);
  if (body.activity === "sleep") feel(draft, index, "woken", now);
  if (tier === "enough") {
    shrug(draft, index, x, now);
    return;
  }
  if (tier === "purr") {
    feel(draft, index, "purred", now);
    purr(draft, index, PURR_TICKS, now);
    heed(draft, index, x, now);
    return;
  }
  const trick = tier === "trick" ? clickTrick(draft.kinds[index]!, standing(draft, index, now).state, present(draft, index, now), body.warmth.tricks - 1) : null;
  if (trick === null) {
    feel(draft, index, "greeted", now);
    hail(draft, index, x, now);
    return;
  }
  perform(draft, index, trick, now);
  heed(draft, index, x, now);
}

/** 🎮️ The learner asked a pet for a deed without the pointer (the keyboard's "Play with the pets"): `hello` greets the learner (`greeted`), `trick` performs the next of its click tricks in authored order (a trick on a whim when it has none, a hello when it has neither), `pet` makes it purr (`purred`). `toss` is the hand's. Only a pet that is free for it ({@link loose}) answers; ignored on a still stage and while play is not permitted. */
export function played(draft: Draft, species: Slug, deed: Deed): void {
  const index = indexOf(draft.actors, species);
  if (index < 0 || draft.mode === "still" || !draft.play || deed === "toss") return;
  const body = draft.actors[index]!;
  if (!loose(body)) return;
  const now = draft.tick;
  const kind = draft.kinds[index]!;
  if (deed === "pet") {
    feel(draft, index, "purred", now);
    purr(draft, index, PURR_TICKS, now);
    return;
  }
  const state = standing(draft, index, now).state;
  const feeling = present(draft, index, now);
  const trick = deed === "trick" ? (clickTrick(kind, state, feeling, body.warmth.tricks) ?? whimTrick(kind, state, feeling, 0)) : null;
  if (trick === null) {
    feel(draft, index, "greeted", now);
    hail(draft, index, body.x + body.facing, now);
    return;
  }
  body.warmth = { heat: body.warmth.heat, since: body.warmth.since, until: body.warmth.until, tier: body.warmth.tier, run: body.warmth.run, tricks: body.warmth.tricks + 1 };
  perform(draft, index, trick, now);
}
//#endregion 🔖️Learner
