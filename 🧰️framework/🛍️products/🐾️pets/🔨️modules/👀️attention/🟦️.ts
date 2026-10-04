/** 👁️ Attention: what a pet does with its eyes, its head and the way it faces — how visible it wants to be in front of the page while it flies or climbs, the way it ought to face and the turn towards it, the gaze spring and the manners its mood gives it, the blink schedule, perking up at a pointer that has come to rest, and the lean and the squeeze a frame draws — and what the learner's hand does to one pet: the gestures the pointer makes round it (circling, stroking) and what they set off, the shrug of a pet that has had enough, and the occasions of the hand that move its mood (picked up, dangled, shaken, put down).
 *
 * Whatever the pointer does to one pet belongs here; its horizons (`POINTER_TICKS`, `TURN_REST`, `PERK_LINGER`) are read by the clock and by the frame.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ../👆️gesture/🟦️.ts — the gestures recognised here
 * @see ../💗️feeling/🟦️.ts — moods, states and tricks
 * @see ./🦀️.rs — the Rust twin
 */

import { type Actor, type Cue, type Point, type Rect, type Slug, type Species, type Stage, type Ticks } from "../../🧬️schema/🟦️.ts";
import { BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS, clipTicks, springStep } from "../🎞️animation/🟦️.ts";
import { randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { hoverStep, noHover, noShaking, pressStep, shakeStep, type Guards, type PressSignal } from "../👆️gesture/🟦️.ts";
import { RELEASE_WEIGHTS, releaseVelocity } from "../🪢️swing/🟦️.ts";
import { giveBack, hold, letGo, lift, toss, tossTarget } from "../🚶️locomotion/🟦️.ts";
import { performed, settled, shownMood, stateAfterTrick, tricksFor } from "../💗️feeling/🟦️.ts";
import { smoothstep } from "../📐️trigonometry/🟦️.ts";
import { WALL_LIP } from "../🏞️terrain/🟦️.ts";
import { ORIGIN, TURN_TICKS, actorKey, blinkAt, clipAt, douse, enter, facingTo, feel, ignite, indexOf, perform, present, purr, shift, standing, turn, type Body, type Draft } from "../📝️draft/🟦️.ts";
import { lookOffset, type Pose } from "../🦴️rig/🟦️.ts";
import { dwellOf, needsAfter } from "../🧠️behavior/🟦️.ts";

//#region 🔖️Constants
export const POINTER_TICKS = 256;
const GAZE_REACH = 32;
const GAZE_REST = 0.000244140625;
const GAZE_CALM = 0.015625;
const GAZE_AHEAD = 0.3;
const GAZE_SULK = 0.5;
const GAZE_FALL = 0.8;
const EYE_HEIGHT = 0.6;
const BLINK_AGAIN = 7;
const GAZE_SAD = 0.5;
const GAZE_SLEEPY = 0.3;
export const PURR_TICKS = 192;
const WAKE_REACH = 1.5;
const SHY_OPACITY = 0.35;
export const TURN_REST = 56;
const TURN_CLEAR = 12;
const LEAN_TURN = 5;
const LEAN_REACH = 1.5;
const LEAN_NOD = 1;
export const PERK_LINGER = 32;
const PERK_URGE = 0.5;
const PERK_COST = 0.6;
const TRAIL = RELEASE_WEIGHTS.length;
const DANGLE_TICKS = 64;
//#endregion 🔖️Constants

//#region 🔖️Presence
/** 😴️ Whether the pointer is close enough to an actor to keep it awake: within 1.5 × its height of the middle of its body. */
export function watched(pointer: Point | null, actor: Actor, kind: Species): boolean {
  if (pointer === null) return false;
  const height = kind.size.height;
  const dx = pointer.x - actor.x;
  const dy = pointer.y - (actor.y - height / 2);
  const reach = WAKE_REACH * height;
  return dx * dx + dy * dy <= reach * reach;
}

/** 🫣️ How visible an actor wants to be: see-through (0.35) while it flies — in the air or under its parachute — or climbs or hangs — on a wall, a ladder or a rope — in front of what the page keeps free (a keep-out: a control, text, an element the host marks, the body of a surface, the focus), so that what lies beneath stays readable; whole wherever it stands — on a perch or a head —, in the learner's hand and while nothing of the page is beneath it. Its box is the size box of its species at its feet less `WALL_LIP` on every side (a keep-out reaches a few pixels beyond what it keeps, so a climber beside a card's side covers nothing of the card). The pointer has no say in it: a pet under a resting pointer looks up and perks instead ({@link perk}). */
export function presenceOf(keepouts: readonly Rect[], actor: Actor, kind: Species): number {
  const footing = actor.footing;
  if (footing === "perch" || footing === "head" || footing === "hand") return 1;
  const x0 = actor.x - kind.size.width / 2 + WALL_LIP;
  const x1 = actor.x + kind.size.width / 2 - WALL_LIP;
  const y0 = actor.y - kind.size.height + WALL_LIP;
  const y1 = actor.y - WALL_LIP;
  for (const keepout of keepouts) if (keepout.width > 0 && keepout.height > 0 && x0 < keepout.x + keepout.width && keepout.x < x1 && y0 < keepout.y + keepout.height && keepout.y < y1) return SHY_OPACITY;
  return 1;
}
//#endregion 🔖️Presence

//#region 🔖️Turning
/** ↪️ The way an actor ought to face at tick `now`, or 0 when nothing says: towards its goal while it walks or hops; towards its partner while it waits for it, acts with it or shows it a trick, away from it while it sulks; and, standing idle by itself on a perch outside a time of concentration, towards the pointer while that is worth a look (it moved within the last 4 s) and clearly on one side — more than 12 px beyond its body — once the actor has rested for 56 ticks after its last turn, so a pointer that crosses over and back never makes it flip back and forth. */
export function headingOf(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): 1 | 0 | -1 {
  const actor = actors[index]!;
  const activity = actor.activity;
  if (activity === "walk" || activity === "hop") return actor.goal > actor.x ? 1 : actor.goal < actor.x ? -1 : 0;
  if (actor.partner !== null) {
    if (activity !== "idle" && activity !== "greet" && activity !== "cuddle" && activity !== "squabble" && activity !== "sulk" && activity !== "trick") return 0;
    const other = indexOf(actors, actor.partner);
    if (other < 0) return 0;
    const towards = facingTo(actors[other]!.x, actor.x);
    return activity === "sulk" ? (towards === 1 ? -1 : 1) : towards;
  }
  if (activity !== "idle" || actor.footing !== "perch" || actor.leaving || stage.quiet || stage.pointer === null || now - stage.pointed >= POINTER_TICKS || now < actor.faced + TURN_REST) return 0;
  const across = stage.pointer.x - actor.x;
  const clear = kinds[index]!.size.width / 2 + TURN_CLEAR;
  return across > clear ? 1 : across < 0 - clear ? -1 : 0;
}

/** 👂️ An actor turns to whoever is at `x` at tick `now` (the learner who clicked it): `turn` towards it. */
export function heed(draft: Draft, index: number, x: number, now: Ticks): void {
  const body = draft.actors[index]!;
  turn(body, facingTo(x, body.x), now);
}
//#endregion 🔖️Turning

//#region 🔖️Gaze
/** 👀️ Where an actor wants its pupils at tick `now`, between −1 and 1 on both axes of the screen, with the manners of the mood it shows: a sad pet keeps its eyes down (at least half way), a sleepy one lowered (at least 0.3), a grumpy one averts them from the pointer.
 *
 * Asleep: the centre. Sulking: ahead and down. Else the pointer while it moved within the last 4 s and the actor
 * neither walks nor flies; else its partner; else, standing, the nearest glance point; else down while falling and
 * straight ahead otherwise. Points are looked at from 0.6 of the height above the feet with `lookOffset`.
 */
function gazeGoal(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): Point {
  const actor = actors[index]!;
  const activity = actor.activity;
  if (activity === "sleep") return ORIGIN;
  if (activity === "sulk") return { x: GAZE_AHEAD * actor.facing, y: GAZE_SULK };
  const feeling = settled(actor.feeling, kinds[index]!.mood, now);
  const mood = shownMood(feeling.mood, feeling.intensity);
  const goal = sought(stage, actors, kinds, index, now);
  const averted = mood === "grumpy" && goal.pointer ? { x: 0 - goal.x, y: goal.y } : { x: goal.x, y: goal.y };
  const floor = mood === "sad" ? GAZE_SAD : mood === "sleepy" ? GAZE_SLEEPY : 0 - 1;
  return averted.y < floor ? { x: averted.x, y: floor } : averted;
}

/** 🎯️ What an actor would look at at tick `now` without the manners of its mood, and whether that is the pointer (see {@link gazeGoal}). */
function sought(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): Point & { readonly pointer: boolean } {
  const actor = actors[index]!;
  const activity = actor.activity;
  const eye = { x: actor.x, y: actor.y - kinds[index]!.size.height * EYE_HEIGHT };
  const moving = activity === "walk" || activity === "hop" || activity === "fall";
  if (!moving && stage.pointer !== null && now - stage.pointed < POINTER_TICKS) return { ...lookOffset(eye, stage.pointer, GAZE_REACH), pointer: true };
  return { ...glanceOf(stage, actors, kinds, index, eye, moving), pointer: false };
}

/** 🧿️ What an actor looks at when the pointer is not worth a look: its partner; else, standing, the nearest glance point; else down while falling and straight ahead otherwise. */
function glanceOf(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, eye: Point, moving: boolean): Point {
  const actor = actors[index]!;
  const activity = actor.activity;
  if (actor.partner !== null) {
    const other = indexOf(actors, actor.partner);
    if (other >= 0) return lookOffset(eye, { x: actors[other]!.x, y: actors[other]!.y - kinds[other]!.size.height * EYE_HEIGHT }, GAZE_REACH);
  }
  if (!moving && stage.glances.length > 0) {
    let nearest = stage.glances[0]!;
    let least = Infinity;
    for (const glance of stage.glances) {
      const dx = glance.x - eye.x;
      const dy = glance.y - eye.y;
      const distance = dx * dx + dy * dy;
      if (distance < least) {
        nearest = glance;
        least = distance;
      }
    }
    return lookOffset(eye, nearest, GAZE_REACH);
  }
  if (activity === "fall") return { x: 0, y: GAZE_FALL };
  return { x: GAZE_AHEAD * actor.facing, y: 0 };
}

/** 🧘️ Whether the pupils of an actor rest on what it wants to look at at tick `now`: on the goal, without velocity. */
export function gazeRests(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): boolean {
  const gaze = actors[index]!.gaze;
  if (gaze.vx !== 0 || gaze.vy !== 0) return false;
  const goal = gazeGoal(stage, actors, kinds, index, now);
  return gaze.x === goal.x && gaze.y === goal.y;
}

/** 🔭️ One tick of the gaze of an actor: both axes spring towards the goal; within 1/4096 of it and slower than 1/64 per second the pupils snap onto it and rest. A sleeper's gaze eases to the centre the same way: the lean of its head follows the gaze, so nothing may jump behind the shut lids either. */
export function look(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const gaze = body.gaze;
  const goal = gazeGoal(draft, draft.actors, draft.kinds, index, now);
  if (gaze.x === goal.x && gaze.y === goal.y && gaze.vx === 0 && gaze.vy === 0) return;
  const across = springStep(gaze.x, gaze.vx, goal.x, GAZE_STIFFNESS, GAZE_DAMPING);
  const down = springStep(gaze.y, gaze.vy, goal.y, GAZE_STIFFNESS, GAZE_DAMPING);
  const rests = Math.abs(across.position - goal.x) <= GAZE_REST && Math.abs(down.position - goal.y) <= GAZE_REST && Math.abs(across.velocity) <= GAZE_CALM && Math.abs(down.velocity) <= GAZE_CALM;
  body.gaze = rests ? { x: goal.x, y: goal.y, vx: 0, vy: 0 } : { x: across.position, y: down.position, vx: across.velocity, vy: down.velocity };
}
//#endregion 🔖️Gaze

//#region 🔖️Manners
/** 😉️ The blink schedule of an actor: when a blink has run its 12 ticks the next one is drawn, 2…6 s ahead, or, one time in six, 7 ticks ahead (a double blink). A sleeper's lids stay shut and draw nothing. */
export function wink(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.activity === "sleep" || now < body.blink + BLINK_TICKS) return;
  const words = randomWords(actorKey(draft, index), 2);
  body.blink = unitOf(words[1]!) * 6 < 1 ? now + BLINK_AGAIN : blinkAt(now, unitOf(words[0]!));
}

/** 🔄️ One tick of turning round: an actor that faces its way squarely and does not face its heading turns to it ({@link turn}). `true` while a walker must not stride yet: while it turns, and on the tick its turn ends — its walk begins anew at that tick, so its clip starts with its first stride. */
export function swivel(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  if (now < body.faced) return true;
  if (now === body.faced) {
    if (body.activity === "walk") body.since = now;
    return true;
  }
  const heading = headingOf(draft, draft.actors, draft.kinds, index, now);
  if (heading === 0 || heading === body.facing) return false;
  turn(body, heading, now);
  return true;
}

/** 🫡️ An actor greets towards `x`: one actor draw picks the span and the clip, it turns to face `x` and lets go of any goal and partner. */
export function hail(draft: Draft, index: number, x: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const words = randomWords(actorKey(draft, index), 2);
  shift(draft, index, "greet", now);
  turn(body, facingTo(x, body.x), now);
  body.until = now + dwellOf("greet", draft.mode, unitOf(words[0]!));
  body.clip = clipAt(draft.kinds[index]!, "greet", unitOf(words[1]!));
  body.goal = body.x;
  body.partner = null;
}

/** 🐿️ An idle actor perks up when the pointer has come to rest on it or beside it: outside a time of concentration, on the tick the pointer has not moved for half a second while it is within 1.5 × the actor's height of the middle of its body (it looks up at it all the while: its gaze follows the pointer), an actor that stands by itself on a perch and whose curiosity — as it stands now — is at least 0.5 greets the pointer ({@link hail}) and spends 0.6 of its curiosity on it (as far as it has any): more than the 0.5 it can have left to spare, so no pet greets twice in a row. Curiosity grows back at rest, faster for a curious species — ten seconds to a minute: that is the pause between two greetings, and a species that is not curious by temperament needs as long before its first one. */
export function perk(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const pointer = draft.pointer;
  if (draft.quiet || pointer === null || now !== draft.pointed + PERK_LINGER || body.partner !== null || body.leaving || body.footing !== "perch" || !watched(pointer, body, kind)) return;
  if (!(needsAfter(body.needs, "idle", now - body.since, kind.temperament).curiosity >= PERK_URGE)) return;
  hail(draft, index, pointer.x, now);
  body.needs = { energy: body.needs.energy, sociability: body.needs.sociability, curiosity: Math.max(body.needs.curiosity - PERK_COST, 0) };
}
//#endregion 🔖️Manners

//#region 🔖️Hand
/** 🧺️ What a pet may be doing on its perch for the hand to set it off on something else: everything it does by itself, a greeting, a trick, a purr, being dizzy and shrugging. */
const LOOSE: readonly Actor["activity"][] = ["idle", "fidget", "walk", "land", "sleep", "greet", "trick", "purr", "dizzy", "shrug"];

/** 🙌️ Whether an actor is free for what the learner's hand sets off: it stands on a perch, is not leaving, has no partner and does nothing it may not drop. */
export function loose(actor: Actor): boolean {
  return actor.footing === "perch" && actor.perch !== null && !actor.leaving && actor.partner === null && LOOSE.includes(actor.activity);
}

/** 📦️ The body the gestures of the pointer are read against: the box of the species, its feet at the actor's feet (`x` the middle, `y` the bottom). */
export function boxOf(actor: Actor, kind: Species): Rect {
  return { x: actor.x - kind.size.width / 2, y: actor.y - kind.size.height, width: kind.size.width, height: kind.size.height };
}

/** 📌️ The actor a press at (`x`, `y`) belongs to: among those whose box holds the point and that are not leaving, the one drawn last (the lowest feet, then the last species id); `null` when it hits nobody. */
export function touchedAt(draft: Draft, x: number, y: number): Slug | null {
  let hit: Actor | null = null;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const box = boxOf(body, draft.kinds[index]!);
    if (body.leaving || x < box.x || x > box.x + box.width || y < box.y || y > box.y + box.height) continue;
    if (hit === null || body.y > hit.y || (body.y === hit.y && body.species > hit.species)) hit = body;
  }
  return hit === null ? null : hit.species;
}

/** 🛡️ What silences the gestures at tick `now`: the pointer over a control, a scroll in the tick before, a time of concentration or a still stage. */
function guardsOf(draft: Draft, now: Ticks): Guards {
  return { control: draft.over === "control", scrolled: now - draft.scrolled <= 1, quiet: draft.quiet, still: draft.mode === "still" };
}

/** 🌀️ One tick of the gestures of the pointer round every actor on its perch (`hoverStep` with the pointer where it was last sampled): a completed gesture is a cue ({@link cued}). Nothing is followed while play is not permitted, while a press is open or without a pointer. */
export function hovers(draft: Draft, now: Ticks): void {
  const pointer = draft.pointer;
  if (!draft.play || pointer === null || draft.press.phase !== "idle") return;
  const guards = guardsOf(draft, now);
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (body.leaving || body.footing !== "perch") continue;
    const step = hoverStep(body.hover, pointer, boxOf(body, draft.kinds[index]!), now, guards);
    body.hover = step.state;
    if (step.cue !== null) cued(draft, index, step.cue, now);
  }
}

/** 🖐️ Every gesture the pointer may be making round the actors is forgotten at tick `now` (the pointer left the stage): the gestures are deaf until then. */
export function unhovered(draft: Draft, now: Ticks): void {
  for (const body of draft.actors) body.hover = noHover(now);
}

/** 🔔️ What a gesture of the learner sets off in an actor that is free for it ({@link loose}) at tick `now`: stroking makes it purr (feeling `purred`) — or perform its stroke trick when it has one on offer and does not purr yet —, circling clockwise or counter-clockwise has it perform the first trick its species offers for that cue in its state and mood (the trick steps it up or down its rungs when it ends); a circle no trick answers is only noticed (`watched`). */
export function cued(draft: Draft, index: number, cue: Cue, now: Ticks): void {
  const body = draft.actors[index]!;
  if (!loose(body)) return;
  const kind = draft.kinds[index]!;
  const offer = tricksFor(kind, cue, standing(draft, index, now).state, present(draft, index, now))[0];
  if (cue === "stroke" && (offer === undefined || body.activity === "purr")) {
    feel(draft, index, "purred", now);
    purr(draft, index, PURR_TICKS, now);
    return;
  }
  if (offer === undefined) {
    feel(draft, index, "watched", now);
    return;
  }
  perform(draft, index, offer, now);
}

/** 🤷️ An actor has had enough of the learner's attention at tick `now`: it feels pestered, shrugs and turns away from `x` for the length of its shrug clip (the shrug's dwell when that loops or is missing). */
export function shrug(draft: Draft, index: number, x: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  feel(draft, index, "pestered", now);
  shift(draft, index, "shrug", now);
  turn(body, facingTo(x, body.x) === 1 ? -1 : 1, now);
  body.clip = clipAt(kind, "shrug", 0);
  const clip = kind.clips.find((entry) => entry.id === body.clip);
  body.until = now + (clip !== undefined && !clip.loop ? clipTicks(clip) : dwellOf("shrug", draft.mode, 0));
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
}

/** ✋️ The learner picked an actor up at tick `now`: it is frightened a little (`lifted`). */
export function lifted(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "lifted", now);
}

/** 🪀️ An actor still hangs in the learner's hand a while after it was picked up, at tick `now`: it grows curious (`dangled`). */
export function dangled(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "dangled", now);
}

/** 🫨️ The learner shook the actor it holds at tick `now`: it is frightened (`shaken`), and the first trick its species offers for a shake in its state and mood happens to it at once — no clip, the actor hangs in the hand: it enters the state the trick leaves, the trick's particles burst out and it feels as the trick leaves it. */
export function shaken(draft: Draft, index: number, now: Ticks): void {
  const kind = draft.kinds[index]!;
  feel(draft, index, "shaken", now);
  const current = standing(draft, index, now);
  const trick = tricksFor(kind, "shake", current.state, present(draft, index, now))[0];
  if (trick === undefined) return;
  const after = stateAfterTrick(kind, current.state, trick);
  if (after !== current.state || trick.to !== undefined) enter(draft, index, after, now, now);
  ignite(draft, index, trick.emitter, now);
  douse(draft, index, trick.emitter, now, now);
  draft.actors[index]!.feeling = performed(present(draft, index, now), trick, kind, now);
}

/** 🧱️ An actor came down hard at tick `now` (a fall that its parachute did not soften): it is cross (`dropped`). */
export function dropped(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "dropped", now);
}

/** 🪂️ An actor floated down under its parachute and touched down at tick `now`: it is content and proud of it (`floated`). */
export function floated(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "floated", now);
}

/** 🍃️ An actor touched down softly without a parachute at tick `now`: content and glad (`landed`). */
export function landed(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "landed", now);
}

/** 🦶️ Another actor landed on this one's head at tick `now`: it is cross (`trampled`). */
export function trampled(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "trampled", now);
}

/** 🏃️ An actor was thrown off what it played with at tick `now` (the learner took a fixture back): it is frightened (`evicted`). */
export function evicted(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "evicted", now);
}

/** 😱️ Something gave an actor a fright at tick `now` (`startled`). */
export function startled(draft: Draft, index: number, now: Ticks): void {
  feel(draft, index, "startled", now);
}

/** 🤲️ What the press machine decided at tick `now` for the actor `touched` the press belongs to, the hand's part: a pick-up (`lift`) lifts it by its scruff — when play is permitted and it is neither leaving nor in the hand already —, it feels picked up and its shake is followed from now on; the release of a picked-up pet (`drop`) lets it go with the pointer's velocity (`releaseVelocity` of the trail); a pick-up that is called off (`abort`) gives it back. Clicks and resting presses are the mind's. */
export function grasp(draft: Draft, signal: PressSignal | null, touched: Slug | null, now: Ticks): void {
  if (signal === null || touched === null) return;
  const index = indexOf(draft.actors, touched);
  if (index < 0) return;
  const body = draft.actors[index]!;
  if (signal === "lift") {
    if (!draft.play || body.leaving || body.footing === "hand") return;
    lift(draft, index, now);
    draft.shaking = noShaking(now);
    lifted(draft, index, now);
    return;
  }
  if (body.footing !== "hand") return;
  if (signal === "drop") letGo(draft, index, releaseVelocity(draft.trail), now);
  else if (signal === "abort") giveBack(draft, index, now);
}

/** 🎾️ The learner tossed a pet with a deed at tick `now` (the keyboard's way to see a parachute): a pet that is free for it ({@link loose}) is carried to the top of the stage and let go there (`toss`), when play is permitted and the stage is not still. */
export function tossed(draft: Draft, species: Slug, now: Ticks): void {
  const index = indexOf(draft.actors, species);
  if (index < 0 || !draft.play || draft.mode === "still" || !loose(draft.actors[index]!)) return;
  toss(draft, index, now);
  lifted(draft, index, now);
}

/** ✊️ One tick of the hand at tick `now`: the press machine sees the tick pass (`pressStep` with `ticked`: an armed press becomes a resting one, and a pet that is free for it purrs under it); the pet the learner holds follows the pointer (`hold`; the pointer's last positions, one per tick and newest last, are its trail) and may be shaken (`shakeStep`: {@link shaken}); a while after the pick-up it dangles curiously ({@link dangled}); a pet carried by a toss goes for the top of the stage and is let go when its time is up. */
export function handle(draft: Draft, now: Ticks): void {
  const guards = { control: false, scrolled: false, quiet: draft.quiet, still: draft.mode === "still" };
  const step = pressStep(draft.press, { kind: "ticked" }, now, guards);
  draft.press = step.state;
  const pressed = draft.touched === null ? -1 : indexOf(draft.actors, draft.touched);
  if (step.signal === "hold" && pressed >= 0 && loose(draft.actors[pressed]!)) {
    feel(draft, pressed, "purred", now);
    purr(draft, pressed, PURR_TICKS, now);
  }
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (body.footing !== "hand") continue;
    if (draft.press.phase === "lifted" && index === pressed) {
      const pointer = draft.pointer ?? (draft.trail.length > 0 ? draft.trail[draft.trail.length - 1]! : { x: body.x, y: body.y - draft.kinds[index]!.grip });
      draft.trail = [...(draft.trail.length < TRAIL ? draft.trail : draft.trail.slice(draft.trail.length - TRAIL + 1)), pointer];
      hold(draft, index, pointer, now);
      const hang = body.hang;
      if (hang !== null) {
        const shake = shakeStep(draft.shaking, { x: hang.grip.x, y: hang.grip.y }, draft.kinds[index]!.size.height, now, guards);
        draft.shaking = shake.state;
        if (shake.cue !== null) shaken(draft, index, now);
      }
      if (now === body.since + DANGLE_TICKS) dangled(draft, index, now);
      continue;
    }
    if (now >= body.until) letGo(draft, index, ORIGIN, now);
    else hold(draft, index, tossTarget(draft, index), now);
  }
}
//#endregion 🔖️Hand

//#region 🔖️Drawing
/** 🌾️ A pose in which the head leans after the eyes: the bone that carries the first eye of the species turns by 5° and shifts by 1.5 px per unit of the gaze `across` (in the actor's own orientation: ahead is positive) and sinks by 1 px per unit of the gaze `down`. The gaze is a spring, so the lean eases with it. A species without eyes does not lean. */
export function leant(kind: Species, pose: Pose, across: number, down: number): Pose {
  const eye = kind.face.eyes[0];
  if (eye === undefined) return pose;
  return pose.map((bone, index) => (kind.bones[index]!.id === eye.bone ? { x: bone.x + LEAN_REACH * across, y: bone.y + LEAN_NOD * down, rotation: bone.rotation + LEAN_TURN * across, scaleX: bone.scaleX, scaleY: bone.scaleY } : bone));
}

/** 🪞️ The factor the drawing of an actor is scaled by across at a tick: 1 once it faces its way squarely, and while it turns round an eased sweep from −1 (the mirror image, which is the way it faced before) through 0 (a line) to 1 over the 8 ticks before `faced`. */
export function squeezeOf(actor: Actor, tick: Ticks): number {
  return tick < actor.faced ? 2 * smoothstep((TURN_TICKS - (actor.faced - tick)) / TURN_TICKS) - 1 : 1;
}
//#endregion 🔖️Drawing
