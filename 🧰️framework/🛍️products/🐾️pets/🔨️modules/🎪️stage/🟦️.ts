/** 🎪️ The stage: the pure fold that makes pets alive. `advance` folds events into a stage, `frameOf` projects a stage into what a render target draws; the same seed and the same events yield the same frames, bit for bit.
 *
 * NORMATIVE ORDER (the Rust twin follows it step by step). Actors are kept in `menagerie.species` order. One tick:
 * 1. per actor, in that order: fade (out while leaving and not walking, gone at 0; otherwise towards its presence —
 *    whole, or see-through while the pointer rests on it) → turning round (an actor that does not face its heading
 *    faces it at once and its drawing follows over 8 ticks, up to `faced`; a walker strides only once it faces its
 *    goal squarely) → motion (`walk`: on every beat the end at the goal or before a neighbour, else `strideTo`;
 *    `hop`: `hopStep` + `landingOf`, a floating gait glides straight; `fall`: `fallStep` + `landingOf`; a sleeper
 *    wakes when the pointer is near; an idle one perks up when the pointer has come to rest beside it) → gaze spring
 *    → blink schedule → mood → the end of its activity when `tick ≥ until`;
 * 2. the stage: a pair whose partners both wait begins its encounter; on every whole second a new pair may be drawn,
 *    and then whoever is wanted but not on stage arrives when a perch has room.
 *
 * DISTANCE. Actors of one surface never stand in each other. Whoever arrives, lands or picks a goal keeps a
 * comfortable gap (8 px between the bodies); a walker stops before a neighbour (6 px); a ride on a perch that shrank
 * sets the actors apart again; and whoever does not fit — a perch holds as many as fit with the comfortable gap — is
 * crowded out: it fades where it stood and, while it is wanted, arrives anew on a perch with room. Newcomers spread
 * out: they arrive on a perch that holds the fewest, on the ground — the lowest perches of the stage — only when no
 * higher one is as empty. And when a survey brings new ground, the scenery has changed: whoever stood on ground that
 * vanished is gone with it and arrives anew (without new ground it falls), and whoever shares a perch moves to one
 * that holds nobody.
 *
 * THE POINTER. Outside a time of concentration a pet that stands idle by itself attends to the pointer while it is
 * worth a look (it moved within the last 4 s): its pupils follow it, the bone that carries its first eye leans
 * after the pupils, it turns round when the pointer is clearly behind it (at most once a second), and when the
 * pointer has come to rest beside it for half a second and it is curious enough it greets it, which costs curiosity.
 * Pupils and lean follow the gaze, which reaches half of its way at 32 px from the eyes: `lookOffset(…, 32)`.
 *
 * DRAWS. One key per draw, `[seed, stream, counter]`, words in the order listed:
 * - actor (`stream` = index of the species, `counter` = `actor.draws`, which starts at the tick of its arrival):
 *   arrival and thaw `[idle dwell, idle clip, first blink]`; coming to rest `[idle dwell, idle clip]`; decision
 *   `randomPick` on word 0, then `[·, dwell, clip, goal or target]`; blink `[gap, double]`; poke and perking up
 *   `[dwell, clip]`; sulk `[dwell, clip]`;
 * - stage (`stream` = `STAGE_STREAM`, `counter` = `stage.draws`): arrival `[perch, place, facing]`; pairing
 *   `randomPick` on word 0, then `[·, chance]`; encounter `[kind, span, clip of the first, clip of the second]`.
 *
 * LAZINESS. Needs are settled when an activity ends (`needsAfter` over its whole span) and rapports when they are
 * touched (`rapportFaded` since `stage.met`, the tick of the last change of any rapport), so neither depends on how
 * time is cut into `ticked` events. Ticks in which nothing can change are jumped over: an actor at rest (no motion,
 * no turn, no fade, gaze and mood on their targets) only has scheduled changes — the end of its activity, the end of
 * its blink, the pointer losing its interest, the next whole second on which a pair may be drawn or somebody who
 * waits off stage may arrive.
 *
 * @see ../🧠️behavior/🟦️.ts — limits, weights, dwells, encounters, needs, rapport
 * @see ../🏞️terrain/🟦️.ts — perches, strides, falls, hops
 * @see ../🎞️animation/🟦️.ts — clips, springs, blinks
 * @see ../🦴️rig/🟦️.ts — poses and matrices
 * @see ../🎲️randomness/🟦️.ts — counter-based draws
 * @see ../../🧬️schema/🟦️.ts — `Stage`, `Actor`, `StageEvent`, `Frame`
 * @see ./🦀️.rs — the Rust twin
 */

import { ACTIVITIES, TICKS_PER_SECOND, type Activity, type Actor, type ActorFrame, type Clip, type EyeFrame, type Frame, type Menagerie, type Perch, type PetMode, type Point, type Rapport, type Slug, type Species, type Stage, type StageEvent, type Surface, type Surveyed, type Ticks } from "../../🧬️schema/🟦️.ts";
import { BLINK_TICKS, GAZE_DAMPING, GAZE_STIFFNESS, blendPose, clipTicks, lidAt, sampleClip, springStep } from "../🎞️animation/🟦️.ts";
import { STAGE_STREAM, randomPick, randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { HOP_DISTANCE, HOP_HEIGHT, fallStep, hopLanding, hopOf, hopStep, landingOf, perchAt, perchesOf, strideTo, type Flight, type Hop } from "../🏞️terrain/🟦️.ts";
import { clamp, smoothstep } from "../📐️trigonometry/🟦️.ts";
import { lookOffset, pupilReach, restPose, solveRig, type Pose } from "../🦴️rig/🟦️.ts";
import { MODE_LIMITS, activityWeights, affinityOf, dwellOf, encounterOf, moodOf, needsAfter, needsOf, rapportAfter, rapportFaded } from "../🧠️behavior/🟦️.ts";

//#region 🔖️Constants
const FADE_STEP = 0.0625;
const POINTER_TICKS = 256;
const GAZE_REACH = 32;
const GAZE_REST = 0.000244140625;
const GAZE_CALM = 0.015625;
const GAZE_AHEAD = 0.3;
const GAZE_SULK = 0.5;
const GAZE_FALL = 0.8;
const EYE_HEIGHT = 0.6;
const BLINK_LOW = 128;
const BLINK_HIGH = 384;
const BLINK_AGAIN = 7;
const MOOD_EASE = 0.03125;
const MOOD_REST = 0.0009765625;
const POKE_REACH = 1.2;
const POKE_CHEER = 0.3;
const WAKE_REACH = 1.5;
const BLEND_TICKS = 8;
const BREATH_STAGGER = 37;
const PATIENCE = 1920;
const WARMUP = 1280;
const ENCOUNTER_REACH = 12;
const ENCOUNTER_RISE = 3;
const PAIR_WEIGHT = 0.25;
const MEET_GAP = 6;
const COMFORT_GAP = 8;
const LEAVE_REACH = 3;
const STROLL_LEAST = 0.75;
const GLIDE_TICKS = 256;
const SHY_OPACITY = 0.35;
const SHY_STEP = 0.03125;
const SHY_REACH = 4;
const TURN_TICKS = 8;
const TURN_REST = 56;
const TURN_CLEAR = 12;
const LEAN_TURN = 5;
const LEAN_REACH = 1.5;
const LEAN_NOD = 1;
const PERK_LINGER = 32;
const PERK_URGE = 0.5;
const PERK_COST = 0.6;
const CONTACT = 0.0078125;
const ORIGIN: Point = { x: 0, y: 0 };
const NO_CLIPS: readonly Slug[] = [];
const NO_RAPPORTS: readonly Rapport[] = [];
//#endregion 🔖️Constants

//#region 🔖️Draft
type Body = { -readonly [field in keyof Actor]: Actor[field] };

type Draft = { -readonly [field in Exclude<keyof Stage, "actors">]: Stage[field] } & { actors: Body[]; kinds: Species[]; streams: number[] };

type Room = { readonly perch: Perch; readonly stretches: readonly number[]; readonly span: number; readonly crowd: number };

type Launch = { readonly x: number; readonly vx: number; readonly vy: number; readonly ticks: Ticks };

/** 🏟️ An empty stage for a seed: tick 0, mode `calm`, not quiet, nothing surveyed, nobody summoned. */
export function openStage(seed: number): Stage {
  return { seed: seed >>> 0, tick: 0, mode: "calm", quiet: false, width: 0, height: 0, pointer: null, pointed: 0, glances: [], surfaces: [], keepouts: [], perches: [], wanted: [], actors: [], rapports: [], met: 0, draws: 0 };
}

/** 📝️ A working copy of a stage: its actors copied, in `menagerie.species` order, each beside its species and its stream; an actor of a species the menagerie does not have is dropped. */
function draftOf(menagerie: Menagerie, stage: Stage): Draft {
  const actors: Body[] = [];
  const kinds: Species[] = [];
  const streams: number[] = [];
  for (let stream = 0; stream < menagerie.species.length; stream++) {
    const kind = menagerie.species[stream]!;
    for (const actor of stage.actors) {
      if (actor.species !== kind.id) continue;
      actors.push({ ...actor });
      kinds.push(kind);
      streams.push(stream);
      break;
    }
  }
  return {
    seed: stage.seed,
    tick: stage.tick,
    mode: stage.mode,
    quiet: stage.quiet,
    width: stage.width,
    height: stage.height,
    pointer: stage.pointer,
    pointed: stage.pointed,
    glances: stage.glances,
    surfaces: stage.surfaces,
    keepouts: stage.keepouts,
    perches: stage.perches,
    wanted: stage.wanted,
    rapports: stage.rapports,
    met: stage.met,
    draws: stage.draws,
    actors,
    kinds,
    streams,
  };
}

/** 📦️ The stage a draft has become. */
function sealed(draft: Draft): Stage {
  return {
    seed: draft.seed,
    tick: draft.tick,
    mode: draft.mode,
    quiet: draft.quiet,
    width: draft.width,
    height: draft.height,
    pointer: draft.pointer,
    pointed: draft.pointed,
    glances: draft.glances,
    surfaces: draft.surfaces,
    keepouts: draft.keepouts,
    perches: draft.perches,
    wanted: draft.wanted,
    actors: draft.actors,
    rapports: draft.rapports,
    met: draft.met,
    draws: draft.draws,
  };
}
//#endregion 🔖️Draft

//#region 🔖️Lookups
/** 🎈️ How high a species floats above its perch: its hover when its gait is `float`, else 0. */
function hoverOf(kind: Species): number {
  return kind.locomotion.gait === "float" && kind.locomotion.hover !== undefined ? kind.locomotion.hover : 0;
}

/** 🔎️ The index of the actor of a species on stage, or −1. */
function indexOf(actors: readonly Actor[], species: Slug): number {
  for (let index = 0; index < actors.length; index++) if (actors[index]!.species === species) return index;
  return -1;
}

/** 🪵️ The surface with an id, or `null`. */
function surfaceOf(surfaces: readonly Surface[], id: string): Surface | null {
  for (const surface of surfaces) if (surface.id === id) return surface;
  return null;
}

/** 🗃️ The clips a species plays for an activity: its own, else the hop clips for the walk of a hopping gait, else the idle clips (a pet that has no motion of its own for something keeps breathing), else none. */
function clipsOf(kind: Species, activity: Activity): readonly Slug[] {
  const own = kind.repertoire[activity];
  if (own !== undefined && own.length > 0) return own;
  const hops = kind.repertoire.hop;
  if (activity === "walk" && kind.locomotion.gait === "hop" && hops !== undefined && hops.length > 0) return hops;
  const idle = kind.repertoire.idle;
  return idle === undefined ? NO_CLIPS : idle;
}

/** 🎞️ The clip a unit draw picks for an activity, uniformly among {@link clipsOf}; `null` when there is none. */
function clipAt(kind: Species, activity: Activity, unit: number): Slug | null {
  const clips = clipsOf(kind, activity);
  if (clips.length === 0) return null;
  const index = Math.floor(unit * clips.length);
  return clips[index < clips.length ? index : clips.length - 1]!;
}

/** 📼️ The clip of a species with an id, or `null`. */
function clipOf(kind: Species, id: Slug | null): Clip | null {
  if (id === null) return null;
  for (const clip of kind.clips) if (clip.id === id) return clip;
  return null;
}

/** 🫁️ The clip a species breathes with — its first idle clip, which loops under everything it does —, or `null` when it has none. */
function breathOf(kind: Species): Clip | null {
  const idle = kind.repertoire.idle;
  return idle === undefined || idle.length === 0 ? null : clipOf(kind, idle[0]!);
}

/** ➡️ The way an actor at `x` faces when it faces `towards`: right when that is not to its left. */
function facingTo(towards: number, x: number): 1 | -1 {
  return towards >= x ? 1 : -1;
}

/** 😑️ The tick of the blink a unit draw schedules after tick `now`: 2…6 s ahead. */
function blinkAt(now: Ticks, unit: number): Ticks {
  return now + BLINK_LOW + Math.floor((BLINK_HIGH - BLINK_LOW) * unit);
}

/** 🔑️ The key of the next draw of an actor; the draw is counted. */
function actorKey(draft: Draft, index: number): number[] {
  const body = draft.actors[index]!;
  const counter = body.draws >>> 0;
  body.draws = (counter + 1) >>> 0;
  return [draft.seed, draft.streams[index]!, counter];
}

/** 🗝️ The key of the next draw of the stage; the draw is counted. */
function stageKey(draft: Draft): number[] {
  const counter = draft.draws >>> 0;
  draft.draws = (counter + 1) >>> 0;
  return [draft.seed, STAGE_STREAM, counter];
}

/** 😴️ Whether the pointer is close enough to an actor to keep it awake: within 1.5 × its height of the middle of its body. */
function watched(pointer: Point | null, actor: Actor, kind: Species): boolean {
  if (pointer === null) return false;
  const height = kind.size.height;
  const dx = pointer.x - actor.x;
  const dy = pointer.y - (actor.y - height / 2);
  const reach = WAKE_REACH * height;
  return dx * dx + dy * dy <= reach * reach;
}

/** 🫣️ How visible an actor wants to be: see-through (0.35) while the pointer rests on it — inside its box, grown by 4 px — so that whoever points can see what lies beneath, else whole. */
function presenceOf(pointer: Point | null, actor: Actor, kind: Species): number {
  if (pointer === null) return 1;
  const reach = kind.size.width / 2 + SHY_REACH;
  const across = pointer.x - actor.x;
  if (across > reach || across < 0 - reach) return 1;
  return pointer.y >= actor.y - kind.size.height - SHY_REACH && pointer.y <= actor.y + SHY_REACH ? SHY_OPACITY : 1;
}

/** ↪️ The way an actor ought to face at tick `now`, or 0 when nothing says: towards its goal while it walks or hops; towards its partner while it waits for it or acts with it, away from it while it sulks; and, standing idle by itself on a perch outside a time of concentration, towards the pointer while that is worth a look (it moved within the last 4 s) and clearly on one side — more than 12 px beyond its body — once the actor has rested for 56 ticks after its last turn, so a pointer that crosses over and back never makes it flip back and forth. */
function headingOf(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): 1 | 0 | -1 {
  const actor = actors[index]!;
  const activity = actor.activity;
  if (activity === "walk" || activity === "hop") return actor.goal > actor.x ? 1 : actor.goal < actor.x ? -1 : 0;
  if (actor.partner !== null) {
    if (activity !== "idle" && activity !== "greet" && activity !== "cuddle" && activity !== "squabble" && activity !== "sulk") return 0;
    const other = indexOf(actors, actor.partner);
    if (other < 0) return 0;
    const towards = facingTo(actors[other]!.x, actor.x);
    return activity === "sulk" ? (towards === 1 ? -1 : 1) : towards;
  }
  if (activity !== "idle" || actor.perch === null || actor.leaving || stage.quiet || stage.pointer === null || now - stage.pointed >= POINTER_TICKS || now < actor.faced + TURN_REST) return 0;
  const across = stage.pointer.x - actor.x;
  const clear = kinds[index]!.size.width / 2 + TURN_CLEAR;
  return across > clear ? 1 : across < 0 - clear ? -1 : 0;
}

/** 🙃️ An actor turns to face `way`: it faces that way at once, and its drawing follows — squeezed through a line — over the next 8 ticks, up to `faced`. An actor that is still turning turns back from where its drawing is: the turn back takes as long as the turn has run. */
function turn(body: Body, way: 1 | -1, now: Ticks): void {
  if (way === body.facing) return;
  const left = body.faced > now ? body.faced - now : 0;
  body.facing = way;
  body.faced = now + TURN_TICKS - left;
}

/** ↔️ How far apart the feet of two actors on one perch must stay for their bodies not to overlap: half of both widths. */
function shoulders(one: Species, other: Species): number {
  return (one.size.width + other.size.width) / 2;
}

/** 🎵️ The ticks of one hop of a hopping gait while it walks with `clip` — the gait covers ground in whole hops — and 1 for every other gait. */
function beatOf(kind: Species, clip: Clip | null): Ticks {
  return kind.locomotion.gait === "hop" && clip !== null ? clipTicks(clip) : 1;
}
//#endregion 🔖️Lookups

//#region 🔖️Gaze
/** 👀️ Where an actor wants its pupils at tick `now`, between −1 and 1 on both axes of the screen.
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
  const eye = { x: actor.x, y: actor.y - kinds[index]!.size.height * EYE_HEIGHT };
  const moving = activity === "walk" || activity === "hop" || activity === "fall";
  if (!moving && stage.pointer !== null && now - stage.pointed < POINTER_TICKS) return lookOffset(eye, stage.pointer, GAZE_REACH);
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
function gazeRests(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number, now: Ticks): boolean {
  const gaze = actors[index]!.gaze;
  if (gaze.vx !== 0 || gaze.vy !== 0) return false;
  const goal = gazeGoal(stage, actors, kinds, index, now);
  return gaze.x === goal.x && gaze.y === goal.y;
}

/** 🔭️ One tick of the gaze of an actor: both axes spring towards the goal; within 1/4096 of it and slower than 1/64 per second the pupils snap onto it and rest. A sleeper's gaze eases to the centre the same way: the lean of its head follows the gaze, so nothing may jump behind the shut lids either. */
function look(draft: Draft, index: number, now: Ticks): void {
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

//#region 🔖️Activities
/** 🔀️ An actor takes up an activity at tick `now`: its needs are settled over the span of the one it leaves. */
function shift(draft: Draft, index: number, activity: Activity, now: Ticks): void {
  const body = draft.actors[index]!;
  body.needs = needsAfter(body.needs, body.activity, now - body.since, draft.kinds[index]!.temperament);
  body.activity = activity;
  body.since = now;
}

/** 🛋️ An actor comes to rest: idle where it stands for a drawn dwell with a drawn idle clip, without a partner. */
function settle(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const words = randomWords(actorKey(draft, index), 2);
  shift(draft, index, "idle", now);
  body.until = now + dwellOf("idle", draft.mode, unitOf(words[0]!));
  body.clip = clipAt(draft.kinds[index]!, "idle", unitOf(words[1]!));
  body.partner = null;
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
}

/** ✂️ An actor lets go of its partner: both links are cut, and a partner that was on its way, waiting or in the middle of the encounter comes to rest (one that sulks keeps sulking). */
function release(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.partner === null) return;
  const other = indexOf(draft.actors, body.partner);
  body.partner = null;
  if (other < 0 || draft.actors[other]!.partner !== body.species) return;
  draft.actors[other]!.partner = null;
  if (draft.actors[other]!.activity !== "sulk" && !draft.actors[other]!.leaving) settle(draft, other, now);
}

/** 🚪️ An actor leaves the stage for good: it is taken out of the draft. */
function remove(draft: Draft, index: number): void {
  draft.actors.splice(index, 1);
  draft.kinds.splice(index, 1);
  draft.streams.splice(index, 1);
}

/** 🏡️ An actor that has nowhere left to be is gone at once (always `false`): it lets go of its partner and arrives anew, spaced from the others, once it is wanted and a perch has room. */
function vanish(draft: Draft, index: number, now: Ticks): boolean {
  release(draft, index, now);
  remove(draft, index);
  return false;
}

/** 🚷️ An actor has no room where it is: it lets go of its partner and of its perch and fades out on the spot. While it is wanted it arrives anew once a perch has room. */
function crowdOut(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  release(draft, index, now);
  if (body.activity === "walk" || body.activity === "hop" || body.activity === "fall") {
    shift(draft, index, "idle", now);
    body.clip = clipAt(draft.kinds[index]!, "idle", 0);
  }
  body.leaving = true;
  body.perch = null;
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
}

/** 🈳️ The place nearest to `x` on a perch where an actor stands a comfortable gap (8 px between the bodies) away from everybody else on that surface, or `null` when the perch has no such place. */
function vacancy(draft: Draft, index: number, perch: Perch, x: number): number | null {
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  if (perch.x1 - half < perch.x0 + half) return null;
  let stretches: number[] = [perch.x0 + half, perch.x1 - half];
  for (let other = 0; other < draft.actors.length; other++) {
    if (other === index || draft.actors[other]!.perch !== perch.surface) continue;
    const room = shoulders(draft.kinds[other]!, kind) + COMFORT_GAP;
    stretches = carve(stretches, draft.actors[other]!.x - room, draft.actors[other]!.x + room);
  }
  let nearest: number | null = null;
  let least = Infinity;
  for (let at = 0; at < stretches.length; at += 2) {
    const place = clamp(x, stretches[at]!, stretches[at + 1]!);
    const distance = Math.abs(place - x);
    if (distance < least) {
      nearest = place;
      least = distance;
    }
  }
  return nearest;
}

/** 🕳️ An actor loses its perch and starts to fall. */
function drop(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  release(draft, index, now);
  shift(draft, index, "fall", now);
  body.perch = null;
  body.vx = 0;
  body.vy = 0;
  body.until = now + dwellOf("fall", draft.mode, 0);
  body.clip = clipAt(draft.kinds[index]!, "fall", 0);
}

/** 🛬️ An actor touches down on a perch at `x`: it stands inside the stretch it fits on and lands for the length of its landing clip, or 0.3 s when that clip loops or is missing. When somebody stands there already it comes down beside them, at the nearest place that keeps a comfortable gap; when that place is farther away than its own width, or the perch is full, it lands where it fell and is crowded out. */
function touch(draft: Draft, index: number, perch: Perch, x: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  const aimed = clamp(x, perch.x0 + half, perch.x1 - half);
  const place = vacancy(draft, index, perch, aimed);
  const fits = place !== null && Math.abs(place - aimed) <= kind.size.width;
  shift(draft, index, "land", now);
  body.perch = perch.surface;
  body.x = fits ? place : aimed;
  body.y = perch.y - hoverOf(kind);
  body.vx = 0;
  body.vy = 0;
  body.goal = body.x;
  body.clip = clipAt(kind, "land", 0);
  const clip = clipOf(kind, body.clip);
  body.until = now + (clip !== null && !clip.loop ? clipTicks(clip) : dwellOf("land", draft.mode, 0));
  if (!fits && !body.leaving) crowdOut(draft, index, now);
}

/** 🥾️ Where a walk from `x` towards `goal` really ends: a hopping gait covers ground in whole hops of its clip (speed × the length of the clip), as many as fit before the goal; every other gait goes all the way. */
function paced(kind: Species, clip: Slug | null, x: number, goal: number): number {
  const beat = beatOf(kind, clipOf(kind, clip));
  if (beat === 1) return goal;
  const hop = (Math.max(kind.locomotion.speed, 0) * beat) / TICKS_PER_SECOND;
  if (!(hop > 0)) return x;
  const hops = Math.floor(Math.abs(goal - x) / hop);
  return goal >= x ? x + hops * hop : x - hops * hop;
}

/** 🚶️ An actor sets out for a goal on its perch; it turns towards it first when it faces the other way. */
function stroll(draft: Draft, index: number, goal: number, clip: Slug | null, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "walk", now);
  body.goal = goal;
  body.until = now + dwellOf("walk", draft.mode, 0);
  body.clip = clip;
}

/** 🧍️ An actor waits for its partner: idle, turning towards it, for as long as the stage has patience. */
function attend(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "idle", now);
  body.until = now + PATIENCE;
  body.clip = clipAt(draft.kinds[index]!, "idle", 0);
  body.goal = body.x;
}

/** 😤️ An actor sulks for a drawn span, turning its back on its partner. */
function sulk(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const words = randomWords(actorKey(draft, index), 2);
  shift(draft, index, "sulk", now);
  body.until = now + dwellOf("sulk", draft.mode, unitOf(words[0]!));
  body.clip = clipAt(draft.kinds[index]!, "sulk", unitOf(words[1]!));
}
//#endregion 🔖️Activities

//#region 🔖️Rapport
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

/** 🕊️ The end of a sulk: once the partner has stopped sulking too, the two mend their rapport by a little; the actor lets go of its partner either way. */
function reconcile(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.partner === null) return;
  const other = indexOf(draft.actors, body.partner);
  const pending = other >= 0 && draft.actors[other]!.activity === "sulk" && draft.actors[other]!.partner === body.species;
  if (other >= 0 && !pending) {
    recall(draft, now);
    bond(draft, index, other, "sulk");
  }
  body.partner = null;
}
//#endregion 🔖️Rapport

//#region 🔖️Decision
/** 🪂️ Whether the arc of a hop keeps clear of what must stay free: on every tick on which the feet are above both ends of the hop (lower down the body is beside the things it hops between), the box of the body touches no keep-out and stays below the top of the stage. */
function soars(draft: Draft, kind: Species, from: Point, to: Point, hop: Hop): boolean {
  const half = kind.size.width / 2;
  const ridge = Math.min(from.y, to.y);
  let flight: Flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left > 1; left--) {
    flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    if (flight.y >= ridge) continue;
    const top = flight.y - kind.size.height;
    if (top < 0) return false;
    for (const keepout of draft.keepouts) {
      if (keepout.width > 0 && keepout.height > 0 && Math.max(flight.x - half, keepout.x) < Math.min(flight.x + half, keepout.x + keepout.width) && Math.max(top, keepout.y) < Math.min(flight.y, keepout.y + keepout.height)) return false;
    }
  }
  return true;
}

/** 🦘️ Where an actor could hop to: per other perch (in perch order) the place nearest to it that keeps half a body from the ends and a comfortable gap from the actors there, when `hopOf` grants the hop, the flight really ends on that perch and its arc covers nothing that must stay free ({@link soars}); a floating gait glides there in a straight line at twice its speed when the place is within the reach of a hop and four seconds. */
function hopsOf(draft: Draft, index: number): Launch[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  const hover = hoverOf(kind);
  const launches: Launch[] = [];
  for (const perch of draft.perches) {
    if (perch.surface === body.perch && perch.x0 <= body.x && body.x <= perch.x1) continue;
    const low = perch.x0 + half;
    const high = perch.x1 - half;
    if (high < low) continue;
    const inset = Math.min(half, (high - low) / 2);
    const x = clamp(body.x, low + inset, high - inset);
    let taken = false;
    for (let other = 0; other < draft.actors.length; other++) {
      if (other === index || draft.actors[other]!.perch !== perch.surface) continue;
      if (Math.abs(draft.actors[other]!.x - x) < shoulders(draft.kinds[other]!, kind) + COMFORT_GAP) taken = true;
    }
    if (taken) continue;
    if (kind.locomotion.gait === "float") {
      const dx = x - body.x;
      const dy = perch.y - hover - body.y;
      if (Math.abs(dx) > HOP_DISTANCE || Math.abs(dy) > HOP_HEIGHT) continue;
      const ticks = Math.floor((Math.sqrt(dx * dx + dy * dy) * TICKS_PER_SECOND) / (2 * Math.max(kind.locomotion.speed, 1)) + 0.5);
      if (ticks < 1 || ticks > GLIDE_TICKS) continue;
      launches.push({ x, vx: (dx * TICKS_PER_SECOND) / ticks, vy: (dy * TICKS_PER_SECOND) / ticks, ticks });
      continue;
    }
    const from = { x: body.x, y: body.y };
    const to = { x, y: perch.y };
    const hop = hopOf(from, to);
    if (hop === null) continue;
    const landing = hopLanding(draft.perches, from, to, hop);
    if (landing === null || landing.surface !== perch.surface || landing.x0 !== perch.x0 || !soars(draft, kind, from, to, hop)) continue;
    launches.push({ x, vx: hop.vx, vy: hop.vy, ticks: hop.ticks });
  }
  return launches;
}

/** 🛣️ The stretch `[low, high]` of its perch an actor can walk on without its body coming closer than `gap` to anybody else on that surface (`except` aside; a walker counts for the whole way it still has to go). The stretch always holds the place where the actor stands, and is that place alone for an actor without a perch. */
function clearway(draft: Draft, index: number, gap: number, except: number): readonly [number, number] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const perch = body.perch === null ? null : perchAt(draft.perches, body.perch, body.x);
  if (perch === null) return [body.x, body.x];
  let low = perch.x0 + kind.size.width / 2;
  let high = perch.x1 - kind.size.width / 2;
  for (let other = 0; other < draft.actors.length; other++) {
    const neighbour = draft.actors[other]!;
    if (other === index || other === except || neighbour.perch !== body.perch) continue;
    const room = shoulders(draft.kinds[other]!, kind) + gap;
    const walks = neighbour.activity === "walk";
    const left = walks && neighbour.goal < neighbour.x ? neighbour.goal : neighbour.x;
    const right = walks && neighbour.goal > neighbour.x ? neighbour.goal : neighbour.x;
    if (right <= body.x) low = Math.max(low, right + room);
    else if (left >= body.x) high = Math.min(high, left - room);
    else {
      low = body.x;
      high = body.x;
    }
  }
  return [Math.min(low, body.x), Math.max(high, body.x)];
}

/** 🙋️ An idle actor decides what to do next: one `randomPick` over `activityWeights`, then its dwell, its clip and, for a walk, a goal on the clear way of its perch (within the stroll of the mode, at least three quarters of a body away — a hopping gait at least one whole hop —, never past or into anybody; without such a goal it stays idle instead) or, for a hop, one of its launches. An actor that feels like a hop where no perch is in reach wanders off instead when another perch has room for it: it leaves, and arrives anew a moment later. */
function decide(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const limits = MODE_LIMITS[draft.mode];
  const width = kind.size.width;
  shift(draft, index, "idle", now);
  let movers = 0;
  let fidgeters = 0;
  let crowd = 0;
  for (let other = 0; other < draft.actors.length; other++) {
    if (other === index) continue;
    const activity = draft.actors[other]!.activity;
    if (activity === "walk" || activity === "hop") movers++;
    if (activity === "fidget") fidgeters++;
    if (body.perch !== null && draft.actors[other]!.perch === body.perch && !draft.actors[other]!.leaving) crowd++;
  }
  const perch = body.perch === null ? null : perchAt(draft.perches, body.perch, body.x);
  const [low, high] = clearway(draft, index, COMFORT_GAP, -1);
  const restless = !draft.quiet && movers < limits.movers && limits.hop > 0 && perch !== null;
  const launches = restless ? hopsOf(draft, index) : [];
  const elsewhere = restless && launches.length === 0 && roomsFor(draft, kind).some((room) => room.perch !== perch);
  const weights = activityWeights(body, kind, { mode: draft.mode, quiet: draft.quiet, movers, fidgeters, roam: Math.max(body.x - low, high - body.x) >= STROLL_LEAST * width, hops: launches.length > 0 || elsewhere, crowd, watched: watched(draft.pointer, body, kind) });
  const key = actorKey(draft, index);
  const pick = randomPick(key, weights);
  const words = randomWords(key, 4);
  const activity = pick < 0 ? "idle" : ACTIVITIES[pick]!;
  const dwell = unitOf(words[1]!);
  const clip = clipAt(kind, activity, unitOf(words[2]!));
  const place = unitOf(words[3]!);
  if (activity === "walk") {
    const reach = limits.stroll * width;
    const least = STROLL_LEAST * width;
    let goal = clamp(low + (high - low) * place, body.x - reach, body.x + reach);
    if (Math.abs(goal - body.x) < least) goal = body.x - low > high - body.x ? body.x - least : body.x + least;
    goal = paced(kind, clip, body.x, clamp(goal, low, high));
    if (kind.locomotion.gait === "hop" ? goal !== body.x : Math.abs(goal - body.x) >= least) {
      stroll(draft, index, goal, clip, now);
      return;
    }
    body.clip = clipAt(kind, "idle", unitOf(words[2]!));
    body.until = now + dwellOf("idle", draft.mode, dwell);
    return;
  }
  if (activity === "hop") {
    if (launches.length === 0) {
      leave(draft, index, now);
      return;
    }
    const chosen = Math.floor(place * launches.length);
    const launch = launches[chosen < launches.length ? chosen : launches.length - 1]!;
    shift(draft, index, "hop", now);
    body.perch = null;
    body.vx = launch.vx;
    body.vy = launch.vy;
    body.goal = launch.x;
    body.until = now + launch.ticks;
    body.clip = clip;
    return;
  }
  body.activity = activity;
  body.clip = clip;
  const played = clipOf(kind, clip);
  body.until = now + (activity === "fidget" && played !== null && !played.loop ? clipTicks(played) : dwellOf(activity, draft.mode, dwell));
}

/** 🏁️ The end of what an actor does when its time is up: an idle one decides (or gives up waiting for its partner), a squabbler sulks, a sulker reconciles, and everyone else comes to rest. Walks, hops and falls end by their motion. */
function conclude(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const activity = body.activity;
  if (activity === "walk" || activity === "hop" || activity === "fall") return;
  if (activity === "idle" && body.partner === null) {
    decide(draft, index, now);
    return;
  }
  if (activity === "squabble" && body.partner !== null) {
    sulk(draft, index, now);
    return;
  }
  if (activity === "idle") release(draft, index, now);
  if (activity === "sulk") reconcile(draft, index, now);
  settle(draft, index, now);
}
//#endregion 🔖️Decision

//#region 🔖️Motion
/** 🚧️ Whether somebody stands in the way of the next `ticks` strides of a walker: another actor on its surface, ahead of it, whose body its own would come closer to than 6 px (bodies that are that close already count as touching within 1/128 px). */
function hindered(draft: Draft, index: number, ticks: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const way = body.goal > body.x ? 1 : -1;
  const far = (Math.max(kind.locomotion.speed, 0) * ticks) / TICKS_PER_SECOND;
  const ahead = clamp(body.goal, body.x - far, body.x + far);
  for (let other = 0; other < draft.actors.length; other++) {
    const neighbour = draft.actors[other]!;
    if (other === index || neighbour.perch !== body.perch || (neighbour.x - body.x) * way <= 0) continue;
    if ((neighbour.x - ahead) * way < shoulders(draft.kinds[other]!, kind) + MEET_GAP - CONTACT) return true;
  }
  return false;
}

/** 👣️ One tick of a walk. On every beat — every tick for a walking or floating gait, the start of every hop for a hopping one — the walk ends when the goal is no farther than one stride (the actor stands on it then) or when somebody is in the way of the next beat; otherwise a stride towards the goal follows. A hopping gait that reaches its goal within a hop, or finds somebody in its way in mid-hop, finishes that hop on the spot. The walk also ends when the stage loses patience. At its end a leaver starts to fade, an actor with a partner waits for it and anyone else comes to rest. */
function stride(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const beat = beatOf(kind, clipOf(kind, body.clip));
  let done = now >= body.until;
  if (!done && (now - body.since) % beat === 0) {
    if (Math.abs(body.goal - body.x) <= Math.max(kind.locomotion.speed, 0) / TICKS_PER_SECOND) {
      body.x = body.goal;
      done = true;
    } else done = hindered(draft, index, beat);
  }
  if (!done) {
    if (beat > 1 && hindered(draft, index, 1)) body.goal = body.x;
    body.x = strideTo(body.x, body.goal, kind.locomotion.speed);
    return;
  }
  if (body.leaving) {
    shift(draft, index, "idle", now);
    body.clip = clipAt(kind, "idle", 0);
    body.goal = body.x;
    return;
  }
  if (body.partner !== null) {
    attend(draft, index, now);
    return;
  }
  settle(draft, index, now);
}

/** 🎯️ The perch a flight towards `x` ends on: among the perches that carry `x`, the one nearest to the height `y`; `null` when the target is gone. */
function aim(perches: readonly Perch[], x: number, y: number): Perch | null {
  let nearest: Perch | null = null;
  let least = Infinity;
  for (const perch of perches) {
    if (x < perch.x0 || x > perch.x1) continue;
    const distance = Math.abs(perch.y - y);
    if (distance < least) {
      nearest = perch;
      least = distance;
    }
  }
  return nearest;
}

/** ⬇️ One tick of a fall: `fallStep`, then the swept landing at the height of the feet plus the hover; below the stage box the actor is gone and arrives anew. `false` when the actor is gone. */
function plunge(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const hover = hoverOf(kind);
  const fallen = fallStep(body.y, body.vy);
  const landing = landingOf(draft.perches, body.x, body.y + hover, fallen.y + hover);
  if (landing !== null) {
    touch(draft, index, landing, body.x, now);
    return true;
  }
  body.y = fallen.y;
  body.vy = fallen.vy;
  if (body.y - kind.size.height <= draft.height) return true;
  return vanish(draft, index, now);
}

/** 🛫️ One tick of a hop: a floating gait glides straight, any other flies the arc of `hopStep` and lands on the first perch it crosses on its way down; on its last tick it touches down on the perch it aimed at, and falls when that perch is gone. `false` when the actor is gone. */
function fly(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const left = body.until - now + 1;
  if (left > 1) {
    if (kind.locomotion.gait === "float") {
      body.x = body.x + body.vx / TICKS_PER_SECOND;
      body.y = body.y + body.vy / TICKS_PER_SECOND;
      return true;
    }
    const next = hopStep(body.x, body.y, body.vx, body.vy, ORIGIN, left);
    const landing = landingOf(draft.perches, next.x, body.y, next.y);
    if (landing !== null) {
      touch(draft, index, landing, next.x, now);
      return true;
    }
    body.x = next.x;
    body.y = next.y;
    body.vy = next.vy;
    return true;
  }
  const target = aim(draft.perches, body.goal, kind.locomotion.gait === "float" ? body.y + hoverOf(kind) : fallStep(body.y, body.vy).y);
  if (target !== null) {
    touch(draft, index, target, body.goal, now);
    return true;
  }
  shift(draft, index, "fall", now);
  body.vx = 0;
  body.vy = kind.locomotion.gait === "float" ? 0 : body.vy;
  body.until = now + dwellOf("fall", draft.mode, 0);
  body.clip = clipAt(kind, "fall", 0);
  return plunge(draft, index, now);
}

/** 😉️ The blink schedule of an actor: when a blink has run its 12 ticks the next one is drawn, 2…6 s ahead, or, one time in six, 7 ticks ahead (a double blink). A sleeper's lids stay shut and draw nothing. */
function wink(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.activity === "sleep" || now < body.blink + BLINK_TICKS) return;
  const words = randomWords(actorKey(draft, index), 2);
  body.blink = unitOf(words[1]!) * 6 < 1 ? now + BLINK_AGAIN : blinkAt(now, unitOf(words[0]!));
}

/** 🎭️ One tick of the mood of an actor: a thirty-second of the way to the mood of its activity, onto it once closer than 1/1024. */
function cheer(body: Body): void {
  const want = moodOf(body.activity);
  if (body.mood === want) return;
  const next = body.mood + (want - body.mood) * MOOD_EASE;
  body.mood = Math.abs(want - next) < MOOD_REST ? want : next;
}

/** 🔄️ One tick of turning round: an actor that faces its way squarely and does not face its heading turns to it ({@link turn}). `true` while a walker must not stride yet: while it turns, and on the tick its turn ends — its walk begins anew at that tick, so its clip starts with its first stride. */
function swivel(draft: Draft, index: number, now: Ticks): boolean {
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
function hail(draft: Draft, index: number, x: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const words = randomWords(actorKey(draft, index), 2);
  shift(draft, index, "greet", now);
  turn(body, facingTo(x, body.x), now);
  body.until = now + dwellOf("greet", draft.mode, unitOf(words[0]!));
  body.clip = clipAt(draft.kinds[index]!, "greet", unitOf(words[1]!));
  body.goal = body.x;
  body.partner = null;
}

/** 🐿️ An idle actor perks up when the pointer has come to rest beside it: outside a time of concentration, on the tick the pointer has not moved for half a second while it is within 1.5 × the actor's height of the middle of its body but not on it (there the actor turns see-through instead: whoever points wants to see what lies beneath), an actor that stands by itself on a perch and whose curiosity — as it stands now — is at least 0.5 greets the pointer ({@link hail}) and spends 0.6 of its curiosity on it (as far as it has any): more than the 0.5 it can have left to spare, so no pet greets twice in a row. Curiosity grows back at rest, faster for a curious species — ten seconds to a minute: that is the pause between two greetings, and a species that is not curious by temperament needs as long before its first one. */
function perk(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const pointer = draft.pointer;
  if (draft.quiet || pointer === null || now !== draft.pointed + PERK_LINGER || body.partner !== null || body.leaving || body.perch === null || !watched(pointer, body, kind) || presenceOf(pointer, body, kind) !== 1) return;
  if (!(needsAfter(body.needs, "idle", now - body.since, kind.temperament).curiosity >= PERK_URGE)) return;
  hail(draft, index, pointer.x, now);
  body.needs = { energy: body.needs.energy, sociability: body.needs.sociability, curiosity: Math.max(body.needs.curiosity - PERK_COST, 0) };
}

/** 🎬️ One tick of one actor, in the normative order; `false` when the actor is gone. */
function act(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  if (body.leaving) {
    if (body.activity !== "walk") {
      const opacity = body.opacity - FADE_STEP;
      if (opacity <= 0) {
        remove(draft, index);
        return false;
      }
      body.opacity = opacity;
    }
  } else {
    const presence = presenceOf(draft.pointer, body, draft.kinds[index]!);
    if (body.opacity < presence) body.opacity = body.opacity + FADE_STEP < presence ? body.opacity + FADE_STEP : presence;
    else if (body.opacity > presence) body.opacity = body.opacity - SHY_STEP > presence ? body.opacity - SHY_STEP : presence;
  }
  const turns = swivel(draft, index, now);
  if (body.activity === "walk") {
    if (!turns) stride(draft, index, now);
  } else if (body.activity === "hop") {
    if (!fly(draft, index, now)) return false;
  } else if (body.activity === "fall") {
    if (!plunge(draft, index, now)) return false;
  } else if (body.activity === "sleep") {
    if (watched(draft.pointer, body, draft.kinds[index]!)) settle(draft, index, now);
  } else if (body.activity === "idle") perk(draft, index, now);
  look(draft, index, now);
  wink(draft, index, now);
  cheer(body);
  if (!body.leaving && now >= body.until) conclude(draft, index, now);
  return true;
}
//#endregion 🔖️Motion

//#region 🔖️Encounters
/** 🙌️ Whether an actor can be drawn into an encounter: standing idle and whole on a perch, without a partner, not leaving. */
function sociable(actor: Actor): boolean {
  return actor.activity === "idle" && actor.partner === null && !actor.leaving && actor.opacity === 1 && actor.perch !== null;
}

/** 📅️ The first whole second at or after tick `from` on which the stage may draw a pair, or −1 while it may not: the mode has encounters, it is not a quiet time, nobody has a partner, at least two actors are sociable, and the warm-up (20 s before the first encounter) or the gap of the mode since the last change of a rapport has passed. */
function pairingTick(stage: Stage, from: Ticks): Ticks {
  const limits = MODE_LIMITS[stage.mode];
  if (limits.encounterGap === 0 || stage.quiet) return -1;
  let free = 0;
  for (const actor of stage.actors) {
    if (actor.partner !== null) return -1;
    if (sociable(actor)) free++;
  }
  if (free < 2) return -1;
  const open = stage.met === 0 ? WARMUP : stage.met + limits.encounterGap;
  const first = from > open ? from : open;
  return Math.floor((first + TICKS_PER_SECOND - 1) / TICKS_PER_SECOND) * TICKS_PER_SECOND;
}

/** 💌️ On a whole second the stage may pair two sociable actors that stand near each other (no farther apart than 12 of their mean widths, no more than 3 in height): one stage draw picks a pair, weighted `(0.25 + |authored affinity|) × mean sociability` (pairs that feel something for each other meet more often than strangers, and whoever has just had company lets others go first), and decides with the chance `rate × mean sociability`. Sociability is the need as it stands now. The pair then approaches; when the mode has room for one mover only, the more sociable of the two walks. */
function pair(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  if (pairingTick(draft, now) !== now) return;
  const limits = MODE_LIMITS[draft.mode];
  let movers = 0;
  for (const body of draft.actors) if (body.activity === "walk" || body.activity === "hop") movers++;
  if (movers >= limits.movers) return;
  const firsts: number[] = [];
  const seconds: number[] = [];
  const drives: number[] = [];
  const socials: number[] = [];
  const weights: number[] = [];
  for (let first = 0; first < draft.actors.length; first++) {
    const one = draft.actors[first]!;
    if (!sociable(one)) continue;
    const eager = needsAfter(one.needs, "idle", now - one.since, draft.kinds[first]!.temperament).sociability;
    for (let second = first + 1; second < draft.actors.length; second++) {
      const two = draft.actors[second]!;
      if (!sociable(two)) continue;
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
  if (one.perch !== two.perch) return false;
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

/** 🤝️ Two actors become partners and come together until their bodies are 6 px apart (what reaches beyond the box of a body — an arm, a ray — then just touches): both walk to the middle (`walker` 0), or only the first (1) or the second (2) walks while the other waits, and whoever need not move waits at once. Each stays on its own perch. */
function approach(draft: Draft, first: number, second: number, walker: number, now: Ticks): void {
  const one = draft.actors[first]!;
  const two = draft.actors[second]!;
  one.partner = two.species;
  two.partner = one.species;
  const apart = (draft.kinds[first]!.size.width + draft.kinds[second]!.size.width) / 2 + MEET_GAP;
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

/** 🎉️ Partners that both wait begin their encounter: the rapports fade up to now, one stage draw picks the kind from their affinity (`encounterOf`), the span they share and a clip for each; they face each other and their rapport moves. */
function meet(menagerie: Menagerie, draft: Draft, now: Ticks): void {
  for (let first = 0; first < draft.actors.length; first++) {
    const one = draft.actors[first]!;
    if (one.partner === null || one.activity !== "idle") continue;
    const second = indexOf(draft.actors, one.partner);
    if (second <= first) continue;
    const two = draft.actors[second]!;
    if (two.activity !== "idle" || two.partner !== one.species) continue;
    recall(draft, now);
    const words = randomWords(stageKey(draft), 4);
    const kind = encounterOf(affinityOf(menagerie, draft.rapports, one.species, two.species), unitOf(words[0]!));
    const span = dwellOf(kind, draft.mode, unitOf(words[1]!));
    shift(draft, first, kind, now);
    shift(draft, second, kind, now);
    one.until = now + span;
    two.until = now + span;
    one.clip = clipAt(draft.kinds[first]!, kind, unitOf(words[2]!));
    two.clip = clipAt(draft.kinds[second]!, kind, unitOf(words[3]!));
    bond(draft, first, second, kind);
  }
}
//#endregion 🔖️Encounters

//#region 🔖️Time
/** 🚏️ The first whole second at or after tick `from` on which a species that is wanted but not on stage may arrive, or −1 while nobody waits, no perch exists or the stage is still (a still stage lets them arrive with its events). */
function arrivalTick(menagerie: Menagerie, stage: Stage, from: Ticks): Ticks {
  if (stage.mode === "still" || stage.perches.length === 0) return -1;
  for (const kind of menagerie.species) {
    if (stage.wanted.includes(kind.id) && indexOf(stage.actors, kind.id) < 0) return Math.floor((from + TICKS_PER_SECOND - 1) / TICKS_PER_SECOND) * TICKS_PER_SECOND;
  }
  return -1;
}

/** 💤️ How many of the next `left` ticks change nothing but the tick: 0 while any actor moves, turns or is about to, fades or has pupils or a mood off their targets, else the ticks before the earliest scheduled change — among them, while a pointer is on stage, the tick an actor may turn to it again and the tick it has come to rest for half a second. */
function lull(menagerie: Menagerie, draft: Draft, left: Ticks): Ticks {
  const next = draft.tick + 1;
  let horizon = next + left;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const activity = body.activity;
    if (body.leaving || body.opacity !== presenceOf(draft.pointer, body, draft.kinds[index]!) || activity === "walk" || activity === "hop" || activity === "fall") return 0;
    if (body.mood !== moodOf(activity) || next <= body.faced || !gazeRests(draft, draft.actors, draft.kinds, index, next)) return 0;
    const heading = headingOf(draft, draft.actors, draft.kinds, index, next);
    if (heading !== 0 && heading !== body.facing) return 0;
    if (activity === "sleep") {
      if (watched(draft.pointer, body, draft.kinds[index]!)) return 0;
    } else horizon = Math.min(horizon, body.blink + BLINK_TICKS);
    horizon = Math.min(horizon, body.until);
    if (draft.pointer !== null && next < body.faced + TURN_REST) horizon = Math.min(horizon, body.faced + TURN_REST);
  }
  if (draft.pointer !== null && next <= draft.pointed + PERK_LINGER) horizon = Math.min(horizon, draft.pointed + PERK_LINGER);
  if (draft.pointer !== null && next - draft.pointed < POINTER_TICKS) horizon = Math.min(horizon, draft.pointed + POINTER_TICKS);
  const pairing = pairingTick(draft, next);
  if (pairing >= 0) horizon = Math.min(horizon, pairing);
  const arrival = arrivalTick(menagerie, draft, next);
  if (arrival >= 0) horizon = Math.min(horizon, arrival);
  const skip = horizon - next;
  return skip > 0 ? Math.min(skip, left) : 0;
}

/** 🥁️ One tick of the stage, in the normative order. */
function step(menagerie: Menagerie, draft: Draft): void {
  const now = draft.tick + 1;
  draft.tick = now;
  for (let index = 0; index < draft.actors.length; ) if (act(draft, index, now)) index++;
  meet(menagerie, draft, now);
  pair(menagerie, draft, now);
  if (arrivalTick(menagerie, draft, now) === now) spawn(menagerie, draft);
}

/** ⏭️ Time passes: tick by tick while something can change, in one jump over every lull, over a still stage and over an empty one that nobody waits to enter. */
function pass(menagerie: Menagerie, draft: Draft, ticks: Ticks): void {
  let left = ticks > 0 ? Math.floor(ticks) : 0;
  while (left > 0) {
    if (draft.mode === "still" || (draft.actors.length === 0 && arrivalTick(menagerie, draft, draft.tick + 1) < 0)) {
      draft.tick = draft.tick + left;
      return;
    }
    const skip = lull(menagerie, draft, left);
    if (skip > 0) {
      draft.tick = draft.tick + skip;
      left = left - skip;
      continue;
    }
    step(menagerie, draft);
    left = left - 1;
  }
}
//#endregion 🔖️Time

//#region 🔖️Events
/** 📐️ The perches of the stage for everyone who is on it or wanted: clearance = the tallest of them plus its hover, minimum = the widest, so that every perch carries any of them. */
function measure(menagerie: Menagerie, draft: Draft): void {
  let tallest = 0;
  let widest = 0;
  for (const kind of menagerie.species) {
    if (!draft.wanted.includes(kind.id) && indexOf(draft.actors, kind.id) < 0) continue;
    tallest = Math.max(tallest, kind.size.height + hoverOf(kind));
    widest = Math.max(widest, kind.size.width);
  }
  draft.perches = perchesOf(draft.surfaces, draft.keepouts, draft.width, draft.height, tallest, widest);
}

/** 🚡️ A grounded actor rides its surface: it moves with the surface's left end and height, and is held inside the nearest perch of that surface (fading in anew when that carries it farther than its own width). When that surface has no perch left it stays on a perch of another surface that lies exactly under its feet (an element that was replaced by its like); without one it falls — unless the survey also brought new ground (`uprooted`: the scenery changed, and a fall would pass in front of whatever is there now) or the stage is still: then it is gone with its ground and arrives anew. Answers how far the perch pulled the actor from where its surface carried it (0 for an actor in the air or falling), or −1 when the actor is gone. */
function carry(draft: Draft, index: number, before: readonly Surface[], uprooted: boolean, now: Ticks): number {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const surface = body.perch;
  if (surface === null) return 0;
  const old = surfaceOf(before, surface);
  const fresh = surfaceOf(draft.surfaces, surface);
  const moved = old === null || fresh === null ? 0 : fresh.x0 - old.x0;
  const x = body.x + moved;
  const half = kind.size.width / 2;
  let best: Perch | null = null;
  let least = Infinity;
  for (const perch of draft.perches) {
    if (perch.surface !== surface) continue;
    const low = perch.x0 + half;
    const high = perch.x1 - half;
    const gap = x < low ? low - x : x > high ? x - high : 0;
    if (gap < least) {
      best = perch;
      least = gap;
    }
  }
  if (best === null) {
    best = landingOf(draft.perches, body.x, body.y + hoverOf(kind), body.y + hoverOf(kind));
    least = 0;
  }
  if (best === null) {
    if (uprooted || draft.mode === "still") {
      vanish(draft, index, now);
      return -1;
    }
    drop(draft, index, now);
    return 0;
  }
  body.perch = best.surface;
  body.x = clamp(x, best.x0 + half, best.x1 - half);
  body.y = best.y - hoverOf(kind);
  body.goal = clamp(body.goal + moved, best.x0 + half, best.x1 - half);
  if (least > kind.size.width && !body.leaving && draft.mode !== "still") body.opacity = 0;
  return least;
}

/** 🪑️ Nobody stands in anybody after a ride. Per perch, the actors it holds (leavers aside) must fit with a comfortable gap between their bodies (the sum of their widths plus 8 px per neighbour pair within the width of the perch; one actor always fits): while they do not, the one its perch pulled farthest (`strains`) — among equals the last in `menagerie.species` order — is crowded out where it stood (`places`), as visible as it was (`shown`), and arrives anew once a perch has room. The others are then set apart, each moved as little as possible, left to right and back: the feet of neighbours end at least as far apart as before the ride, held between half of both widths (bodies that touch) and 8 px more. On a still stage whoever is crowded out is gone at once. */
function seat(draft: Draft, places: readonly number[], strains: readonly number[], shown: readonly number[]): void {
  const now = draft.tick;
  const gone: number[] = [];
  for (const perch of draft.perches) {
    const members: number[] = [];
    for (let index = 0; index < draft.actors.length; index++) {
      const body = draft.actors[index]!;
      if (body.leaving || body.perch !== perch.surface || body.x < perch.x0 || body.x > perch.x1) continue;
      let at = members.length;
      while (at > 0 && places[members[at - 1]!]! > places[index]!) at--;
      members.splice(at, 0, index);
    }
    while (members.length > 1) {
      let need = (members.length - 1) * COMFORT_GAP;
      for (const member of members) need = need + draft.kinds[member]!.size.width;
      if (need <= perch.x1 - perch.x0) break;
      let worst = 0;
      for (let at = 1; at < members.length; at++) {
        const strain = strains[members[at]!]!;
        const most = strains[members[worst]!]!;
        if (strain > most || (strain === most && members[at]! > members[worst]!)) worst = at;
      }
      const index = members[worst]!;
      members.splice(worst, 1);
      draft.actors[index]!.x = places[index]!;
      draft.actors[index]!.opacity = shown[index]!;
      crowdOut(draft, index, now);
      gone.push(index);
    }
    for (let at = 1; at < members.length; at++) {
      const body = draft.actors[members[at]!]!;
      const before = members[at - 1]!;
      const hard = shoulders(draft.kinds[before]!, draft.kinds[members[at]!]!);
      const apart = clamp(places[members[at]!]! - places[before]!, hard, hard + COMFORT_GAP);
      if (body.x - draft.actors[before]!.x < apart) body.x = draft.actors[before]!.x + apart;
    }
    for (let at = members.length - 1; at >= 0; at--) {
      const body = draft.actors[members[at]!]!;
      const high = perch.x1 - draft.kinds[members[at]!]!.size.width / 2;
      if (body.x > high) body.x = high;
      if (at < members.length - 1) {
        const after = members[at + 1]!;
        const hard = shoulders(draft.kinds[after]!, draft.kinds[members[at]!]!);
        const apart = clamp(places[after]! - places[members[at]!]!, hard, hard + COMFORT_GAP);
        if (draft.actors[after]!.x - body.x < apart) body.x = draft.actors[after]!.x - apart;
      }
      if (body.activity !== "walk") body.goal = body.x;
    }
  }
  if (draft.mode !== "still") return;
  gone.sort((one, other) => other - one);
  for (const index of gone) remove(draft, index);
}

/** 🆕️ Whether the surfaces of the stage hold one that was not there `before`: new ground. */
function widened(draft: Draft, before: readonly Surface[]): boolean {
  for (const surface of draft.surfaces) if (surfaceOf(before, surface.id) === null) return true;
  return false;
}

/** 🎠️ Every grounded actor rides its surface from where the surfaces were `before` (`uprooted`: new ground came with them), and is then seated so that nobody stands in anybody. */
function ride(draft: Draft, before: readonly Surface[], uprooted: boolean): void {
  const places: number[] = [];
  const strains: number[] = [];
  const shown: number[] = [];
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    const old = body.perch === null ? null : surfaceOf(before, body.perch);
    const fresh = body.perch === null ? null : surfaceOf(draft.surfaces, body.perch);
    const place = body.x + (old === null || fresh === null ? 0 : fresh.x0 - old.x0);
    const opacity = body.opacity;
    const strain = carry(draft, index, before, uprooted, draft.tick);
    if (strain < 0) continue;
    places.push(place);
    strains.push(strain);
    shown.push(opacity);
    index++;
  }
  seat(draft, places, strains, shown);
}

/** 🔪️ Stretches (pairs of ends) without the interval `(low, high)`. */
function carve(stretches: readonly number[], low: number, high: number): number[] {
  const kept: number[] = [];
  for (let index = 0; index < stretches.length; index += 2) {
    const x0 = stretches[index]!;
    const x1 = stretches[index + 1]!;
    if (high <= x0 || low >= x1) kept.push(x0, x1);
    else {
      if (low > x0) kept.push(x0, low);
      if (high < x1) kept.push(high, x1);
    }
  }
  return kept;
}

/** 🧑‍🤝‍🧑️ How many actors stand on a perch: on its surface, within its ends. */
function crowdOn(draft: Draft, perch: Perch): number {
  let crowd = 0;
  for (const body of draft.actors) if (body.perch === perch.surface && body.x >= perch.x0 && body.x <= perch.x1) crowd++;
  return crowd;
}

/** 🛏️ Where a species could arrive: per perch that carries it the stretches it can stand on a comfortable gap away from the actors already there (half of both widths plus 8 px), and how many stand there. A perch without such a stretch is full. */
function roomsFor(draft: Draft, kind: Species): Room[] {
  const rooms: Room[] = [];
  const half = kind.size.width / 2;
  for (const perch of draft.perches) {
    if (perch.x1 - half < perch.x0 + half) continue;
    let stretches: number[] = [perch.x0 + half, perch.x1 - half];
    for (let other = 0; other < draft.actors.length; other++) {
      if (draft.actors[other]!.perch !== perch.surface) continue;
      const gap = shoulders(draft.kinds[other]!, kind) + COMFORT_GAP;
      stretches = carve(stretches, draft.actors[other]!.x - gap, draft.actors[other]!.x + gap);
    }
    if (stretches.length === 0) continue;
    let span = 0;
    for (let index = 0; index < stretches.length; index += 2) span = span + (stretches[index + 1]! - stretches[index]!);
    rooms.push({ perch, stretches, span, crowd: crowdOn(draft, perch) });
  }
  return rooms;
}

/** 🏘️ The rooms a newcomer chooses among, so that a company spreads out over what the stage offers: those that hold the fewest actors, and of these the ones on the ground — the lowest perches of the stage — only when no higher one is as empty. Pets stand on things before they stand beneath them. */
function quarters(draft: Draft, rooms: readonly Room[]): Room[] {
  let fewest = Infinity;
  for (const room of rooms) if (room.crowd < fewest) fewest = room.crowd;
  let ground = 0 - Infinity;
  for (const perch of draft.perches) if (perch.y > ground) ground = perch.y;
  const emptiest: Room[] = [];
  const raised: Room[] = [];
  for (const room of rooms) {
    if (room.crowd !== fewest) continue;
    emptiest.push(room);
    if (room.perch.y < ground) raised.push(room);
  }
  return raised.length > 0 ? raised : emptiest;
}

/** 🌟️ A wanted species arrives: one stage draw picks one of its {@link quarters} (uniformly), a place on it a comfortable gap away from everybody there and the way it faces; one actor draw its first idle dwell, idle clip and blink. It fades in (on a still stage it is simply there). While no perch has room it waits off stage: nothing is drawn, and it is tried again with every survey, every summons and every whole second. */
function arrive(draft: Draft, kind: Species, stream: number): void {
  const rooms = quarters(draft, roomsFor(draft, kind));
  if (rooms.length === 0) return;
  const words = randomWords(stageKey(draft), 3);
  const chosen = Math.floor(unitOf(words[0]!) * rooms.length);
  const room = rooms[chosen < rooms.length ? chosen : rooms.length - 1]!;
  let along = unitOf(words[1]!) * room.span;
  let x = room.stretches[0]!;
  for (let index = 0; index < room.stretches.length; index += 2) {
    const length = room.stretches[index + 1]! - room.stretches[index]!;
    x = room.stretches[index]! + Math.min(along, length);
    if (along <= length) break;
    along = along - length;
  }
  const now = draft.tick;
  const counter = now >>> 0;
  const draws = randomWords([draft.seed, stream, counter], 3);
  const body: Body = {
    species: kind.id,
    perch: room.perch.surface,
    x,
    y: room.perch.y - hoverOf(kind),
    vx: 0,
    vy: 0,
    facing: unitOf(words[2]!) < 0.5 ? 1 : -1,
    faced: now,
    activity: "idle",
    since: now,
    until: now + dwellOf("idle", draft.mode, unitOf(draws[0]!)),
    goal: x,
    partner: null,
    clip: clipAt(kind, "idle", unitOf(draws[1]!)),
    gaze: { x: 0, y: 0, vx: 0, vy: 0 },
    blink: blinkAt(now, unitOf(draws[2]!)),
    mood: moodOf("idle"),
    needs: needsOf(kind.temperament),
    opacity: draft.mode === "still" ? 1 : 0,
    leaving: false,
    draws: (counter + 1) >>> 0,
  };
  let at = 0;
  while (at < draft.streams.length && draft.streams[at]! < stream) at++;
  draft.actors.splice(at, 0, body);
  draft.kinds.splice(at, 0, kind);
  draft.streams.splice(at, 0, stream);
}

/** 📣️ Every wanted species that is not on stage arrives, in `menagerie.species` order — as long as the company on stage, leavers included, is smaller than the wanted one: a newcomer waits until whoever it replaces is gone, so a stage never holds more actors than were summoned. */
function spawn(menagerie: Menagerie, draft: Draft): void {
  if (draft.perches.length === 0) return;
  for (let stream = 0; stream < menagerie.species.length && draft.actors.length < draft.wanted.length; stream++) {
    const kind = menagerie.species[stream]!;
    if (draft.wanted.includes(kind.id) && indexOf(draft.actors, kind.id) < 0) arrive(draft, kind, stream);
  }
}

/** 🌬️ The company spreads out over new ground (a survey brought a surface that was not there before). Every perch that holds more than one actor gives up all but the first of them (in `menagerie.species` order), as far as perches that hold nobody can carry them — each such perch takes one: whoever stands idle by itself on the shared perch leaves (on a still stage it is gone at once) and, still wanted, arrives anew where nobody stands. So a company that gathered on the only edge a screen offered — the footer line under an introduction — does not stay parked there when the next screen brings cards. A time of concentration leaves everybody where they are. */
function spread(draft: Draft): void {
  if (draft.quiet) return;
  const now = draft.tick;
  const vacant: Perch[] = [];
  for (const perch of draft.perches) if (crowdOn(draft, perch) === 0) vacant.push(perch);
  const gone: number[] = [];
  for (const perch of draft.perches) {
    let first = true;
    for (let index = 0; index < draft.actors.length && vacant.length > 0; index++) {
      const body = draft.actors[index]!;
      if (body.leaving || body.perch !== perch.surface || body.x < perch.x0 || body.x > perch.x1) continue;
      if (first) {
        first = false;
        continue;
      }
      if (body.activity !== "idle" || body.partner !== null) continue;
      const width = draft.kinds[index]!.size.width;
      let home = -1;
      for (let at = 0; at < vacant.length && home < 0; at++) if (vacant[at]!.x1 - vacant[at]!.x0 >= width) home = at;
      if (home < 0) continue;
      vacant.splice(home, 1);
      if (draft.mode === "still") gone.push(index);
      else leave(draft, index, now);
    }
  }
  gone.sort((one, other) => other - one);
  for (const index of gone) remove(draft, index);
}

/** 🗺️ The stage was measured anew: perches are cut again, grounded actors ride their surfaces, fall or — when the survey brought new ground — are gone with ground that vanished, and whoever waits for a perch arrives; over new ground the company then spreads out ({@link spread}), and on a still stage whoever it moved arrives at once. */
function survey(menagerie: Menagerie, draft: Draft, event: Surveyed): void {
  const before = draft.surfaces;
  draft.width = event.width;
  draft.height = event.height;
  draft.surfaces = event.surfaces;
  draft.keepouts = event.keepouts;
  const uprooted = widened(draft, before);
  measure(menagerie, draft);
  ride(draft, before, uprooted);
  spawn(menagerie, draft);
  if (!uprooted) return;
  spread(draft);
  spawn(menagerie, draft);
}

/** 👋️ An actor starts to leave: it lets go of its partner and walks to the nearer end of its perch when that is within three of its widths and nobody stands in the way, then fades; farther away, hemmed in or in the air, it fades where it is. */
function leave(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  body.leaving = true;
  if (body.perch === null) return;
  const perch = perchAt(draft.perches, body.perch, body.x);
  const half = kind.size.width / 2;
  const low = perch === null ? body.x : perch.x0 + half;
  const high = perch === null ? body.x : perch.x1 - half;
  const end = body.x - low <= high - body.x ? low : high;
  const [clearLow, clearHigh] = clearway(draft, index, MEET_GAP, -1);
  const clip = clipAt(kind, "walk", 0);
  const goal = paced(kind, clip, body.x, end);
  if (goal !== body.x && end >= clearLow && end <= clearHigh && Math.abs(end - body.x) <= LEAVE_REACH * kind.size.width) {
    stroll(draft, index, goal, clip, now);
    return;
  }
  shift(draft, index, "idle", now);
  body.clip = clipAt(kind, "idle", 0);
  body.goal = body.x;
}

/** 🎟️ The species that belong on stage changed: whoever is no longer wanted leaves (on a still stage it is gone at once), a leaver that is wanted again stays when it still stands on a perch (one that was crowded out goes on fading and arrives anew), perches are cut for the new company and the newly wanted arrive. */
function summon(menagerie: Menagerie, draft: Draft, wanted: readonly Slug[]): void {
  const now = draft.tick;
  draft.wanted = wanted;
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    const stays = wanted.includes(body.species);
    if (stays && body.leaving && body.perch !== null) {
      body.leaving = false;
      settle(draft, index, now);
    }
    if (!stays && !body.leaving) {
      if (draft.mode === "still") {
        remove(draft, index);
        continue;
      }
      leave(draft, index, now);
    }
    index++;
  }
  measure(menagerie, draft);
  ride(draft, draft.surfaces, false);
  spawn(menagerie, draft);
}

/** 🧊️ The stage turns still: leavers and whoever is in the air are gone (the wanted among them arrive anew at once, on a perch with room), and everyone else is idle at rest, whole, without a partner, with centred pupils. */
function freeze(draft: Draft, now: Ticks): void {
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    if (body.leaving || body.perch === null) {
      remove(draft, index);
      continue;
    }
    shift(draft, index, "idle", now);
    body.faced = now;
    body.until = now;
    body.clip = null;
    body.partner = null;
    body.vx = 0;
    body.vy = 0;
    body.goal = body.x;
    body.gaze = { x: 0, y: 0, vx: 0, vy: 0 };
    body.mood = moodOf("idle");
    body.opacity = 1;
    index++;
  }
}

/** 🎚️ The liveliness changed. Still freezes the stage; leaving still gives every actor a fresh idle dwell, idle clip and blink; a mode that allows fewer movers lets the walkers beyond its limit come to rest (hops in flight count first). */
function tune(menagerie: Menagerie, draft: Draft, mode: PetMode): void {
  if (mode === draft.mode) return;
  const before = draft.mode;
  const now = draft.tick;
  draft.mode = mode;
  if (mode === "still") {
    freeze(draft, now);
    spawn(menagerie, draft);
    return;
  }
  if (before === "still") {
    for (let index = 0; index < draft.actors.length; index++) {
      const body = draft.actors[index]!;
      const words = randomWords(actorKey(draft, index), 3);
      body.since = now;
      body.until = now + dwellOf("idle", mode, unitOf(words[0]!));
      body.clip = clipAt(draft.kinds[index]!, "idle", unitOf(words[1]!));
      body.blink = blinkAt(now, unitOf(words[2]!));
    }
    return;
  }
  let movers = 0;
  for (const body of draft.actors) if (!body.leaving && body.activity === "hop") movers++;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (body.leaving || body.activity !== "walk") continue;
    movers++;
    if (movers <= MODE_LIMITS[mode].movers) continue;
    release(draft, index, now);
    settle(draft, index, now);
  }
}

/** 👆️ Someone tapped the stage: the nearest grounded actor within 1.2 × its height of the tap (measured from the middle of its body) cheers up and, unless it is busy with its partner, greets towards the tap ({@link hail}); a sulker is reconciled first. A still stage ignores it, and so does a time of concentration: hushed actors rest. */
function poke(draft: Draft, x: number, y: number): void {
  if (draft.mode === "still" || draft.quiet) return;
  const now = draft.tick;
  let nearest = -1;
  let least = Infinity;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (body.leaving || body.perch === null) continue;
    const height = draft.kinds[index]!.size.height;
    const dx = x - body.x;
    const dy = y - (body.y - height / 2);
    const distance = dx * dx + dy * dy;
    const reach = POKE_REACH * height;
    if (distance > reach * reach || distance >= least) continue;
    nearest = index;
    least = distance;
  }
  if (nearest < 0) return;
  const body = draft.actors[nearest]!;
  body.mood = Math.min(1, body.mood + POKE_CHEER);
  if (body.partner !== null && body.activity !== "sulk") return;
  if (body.activity === "sulk") reconcile(draft, nearest, now);
  hail(draft, nearest, x, now);
}

/** 📨️ One event folded into the draft. */
function apply(menagerie: Menagerie, draft: Draft, event: StageEvent): void {
  if (event.kind === "ticked") pass(menagerie, draft, event.ticks);
  else if (event.kind === "pointed") {
    draft.pointer = { x: event.x, y: event.y };
    draft.pointed = draft.tick;
  } else if (event.kind === "unpointed") draft.pointer = null;
  else if (event.kind === "glanced") draft.glances = event.points;
  else if (event.kind === "surveyed") survey(menagerie, draft, event);
  else if (event.kind === "summoned") summon(menagerie, draft, event.species);
  else if (event.kind === "tuned") tune(menagerie, draft, event.mode);
  else if (event.kind === "hushed") draft.quiet = event.quiet;
  else poke(draft, event.x, event.y);
}

/** 🧵️ The stage after the events, folded in order; the stage passed in is never changed. */
export function advance(menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]): Stage {
  if (events.length === 0) return stage;
  const draft = draftOf(menagerie, stage);
  for (const event of events) apply(menagerie, draft, event);
  return sealed(draft);
}
//#endregion 🔖️Events

//#region 🔖️Frame
/** 🧱️ Whether the clip of an activity replaces the idle loop underneath it (cross-fade) instead of being added on top of it: the gaits, the flights, the landing and sleep bring their own body motion. */
function replaces(activity: Activity): boolean {
  return activity === "walk" || activity === "hop" || activity === "fall" || activity === "land" || activity === "sleep";
}

/** 🎛️ How much of its clip an actor shows: it fades in over the first 8 ticks of the activity and out over the last 8 — of its span, or of the way to its goal when it walks; flights end hard. */
function weightOf(kind: Species, actor: Actor, tick: Ticks): number {
  const activity = actor.activity;
  const into = (tick - actor.since) / BLEND_TICKS;
  const out = activity === "hop" || activity === "fall" ? 1 : activity === "walk" ? (Math.abs(actor.goal - actor.x) * TICKS_PER_SECOND) / (Math.max(kind.locomotion.speed, 1) * BLEND_TICKS) : (actor.until - tick) / BLEND_TICKS;
  return clamp(Math.min(into, out), 0, 1);
}

/** 🧩️ One pose on top of another: offsets and rotations add, scale factors multiply. */
function layer(under: Pose, over: Pose): Pose {
  return under.map((bone, index) => {
    const other = over[index]!;
    return { x: bone.x + other.x, y: bone.y + other.y, rotation: bone.rotation + other.rotation, scaleX: bone.scaleX * other.scaleX, scaleY: bone.scaleY * other.scaleY };
  });
}

/** 🤸️ The pose of an actor at a tick: the first idle clip of its species loops underneath on the clock of the stage (staggered by 37 ticks per stream, so a pet that stands still is never frozen and no two breathe in step), and the clip of its activity, weighted by {@link weightOf} from the rest pose, lies on top of it or takes its place. A walker that still turns round has not set out yet: its walk weighs nothing. */
function poseOf(kind: Species, actor: Actor, tick: Ticks, stream: number): Pose {
  const rest = restPose(kind);
  const breath = breathOf(kind);
  const top = clipOf(kind, actor.clip);
  const under = breath === null ? rest : sampleClip(kind, breath, tick + stream * BREATH_STAGGER);
  if (top === null || (breath !== null && top.id === breath.id)) return under;
  const weight = tick < actor.faced && actor.activity === "walk" ? 0 : weightOf(kind, actor, tick);
  const over = blendPose(rest, sampleClip(kind, top, tick - actor.since), weight);
  if (breath === null) return over;
  return layer(replaces(actor.activity) ? blendPose(under, rest, weight) : under, over);
}

/** 🌾️ A pose in which the head leans after the eyes: the bone that carries the first eye of the species turns by 5° and shifts by 1.5 px per unit of the gaze `across` (in the actor's own orientation: ahead is positive) and sinks by 1 px per unit of the gaze `down`. The gaze is a spring, so the lean eases with it. A species without eyes does not lean. */
function leant(kind: Species, pose: Pose, across: number, down: number): Pose {
  const eye = kind.face.eyes[0];
  if (eye === undefined) return pose;
  return pose.map((bone, index) => (kind.bones[index]!.id === eye.bone ? { x: bone.x + LEAN_REACH * across, y: bone.y + LEAN_NOD * down, rotation: bone.rotation + LEAN_TURN * across, scaleX: bone.scaleX, scaleY: bone.scaleY } : bone));
}

/** 🪞️ The factor the drawing of an actor is scaled by across at a tick: 1 once it faces its way squarely, and while it turns round an eased sweep from −1 (the mirror image, which is the way it faced before) through 0 (a line) to 1 over the 8 ticks before `faced`. */
function squeezeOf(actor: Actor, tick: Ticks): number {
  return tick < actor.faced ? 2 * smoothstep((TURN_TICKS - (actor.faced - tick)) / TURN_TICKS) - 1 : 1;
}

/** 🎼️ How many ticks per second the motion of one actor needs at a tick: 64 while it fades (in, out, or see-through under the pointer and back), turns round, walks, flies, lands, plays a clip that does not loop or blends a clip in or out; 32 while a loop, a blink, its pupils or its mood move; 16 while it only sleeps; 0 when nothing of it moves. */
function paceOf(stage: Stage, actors: readonly Actor[], kinds: readonly Species[], index: number): 0 | 16 | 32 | 64 {
  const actor = actors[index]!;
  const kind = kinds[index]!;
  const tick = stage.tick;
  const activity = actor.activity;
  if (actor.leaving || actor.opacity !== presenceOf(stage.pointer, actor, kind) || tick < actor.faced || activity === "walk" || activity === "hop" || activity === "fall" || activity === "land") return 64;
  const breath = breathOf(kind);
  const top = clipOf(kind, actor.clip);
  const layered = top !== null && (breath === null || top.id !== breath.id);
  if (layered && ((!top.loop && tick - actor.since < clipTicks(top)) || tick - actor.since < BLEND_TICKS || actor.until - tick < BLEND_TICKS)) return 64;
  const restless = actor.mood !== moodOf(activity) || !gazeRests(stage, actors, kinds, index, tick + 1);
  if (activity === "sleep") return restless ? 32 : layered || breath !== null ? 16 : 0;
  if (restless || (tick >= actor.blink && tick < actor.blink + BLINK_TICKS)) return 32;
  return (layered && top.loop) || breath !== null ? 32 : 0;
}

/** 🎥️ What a render target draws for a stage: the actors back to front (by `y`, then species id), each with its feet, facing, activity, opacity, one matrix per bone (`solveRig` of {@link poseOf}, leaning after the gaze ({@link leant}) outside a time of concentration, squeezed across by {@link squeezeOf} while the actor turns round), its eyes (pupil offset = gaze × `pupilReach`: the pupil travels up to the outline of the white; mirrored with the actor as drawn; lid from the blink, shut asleep) and its mood; the rate the motion needs and, at rate 0, the tick of the next scheduled change.
 *
 * A still stage shows every actor in its rest pose with open eyes and centred pupils at rate 0 without a wake tick —
 * see-through at once while the pointer rests on it, since nothing eases there — and so does an empty one that nobody
 * waits to enter. At rate 0 otherwise (species without an idle loop, pupils at rest, between blinks) `wake` is the
 * earliest of the next blink, the end of an activity, the pointer losing its interest, having come to rest for half
 * a second or being allowed to turn an actor again, the next whole second on which a pair may be drawn and the next
 * whole second on which somebody who waits off stage may arrive.
 */
export function frameOf(menagerie: Menagerie, stage: Stage): Frame {
  const actors: Actor[] = [];
  const kinds: Species[] = [];
  const streams: number[] = [];
  for (let stream = 0; stream < menagerie.species.length; stream++) {
    const kind = menagerie.species[stream]!;
    for (const actor of stage.actors) {
      if (actor.species !== kind.id) continue;
      actors.push(actor);
      kinds.push(kind);
      streams.push(stream);
      break;
    }
  }
  const tick = stage.tick;
  const still = stage.mode === "still";
  const order = actors.map((_, index) => index).sort((a, b) => actors[a]!.y - actors[b]!.y || (actors[a]!.species < actors[b]!.species ? -1 : actors[a]!.species > actors[b]!.species ? 1 : 0));
  const frames: ActorFrame[] = order.map((index) => {
    const actor = actors[index]!;
    const kind = kinds[index]!;
    const lid = still ? 0 : actor.activity === "sleep" ? 1 : lidAt(tick - actor.blink);
    const squeeze = still ? 1 : squeezeOf(actor, tick);
    const forward = squeeze < 0 ? actor.facing === -1 : actor.facing === 1;
    const across = forward ? actor.gaze.x : 0 - actor.gaze.x;
    const eyes: EyeFrame[] = kind.face.eyes.map((eye) => {
      const span = pupilReach(eye);
      return { x: across * span, y: actor.gaze.y * span, lid };
    });
    const pose = still ? restPose(kind) : poseOf(kind, actor, tick, streams[index]!);
    const bones = solveRig(kind, still || stage.quiet ? pose : leant(kind, pose, across, actor.gaze.y));
    if (squeeze !== 1) for (let entry = 0; entry < bones.length; entry += 2) bones[entry] = bones[entry]! * squeeze;
    const presence = presenceOf(stage.pointer, actor, kind);
    return { species: actor.species, x: actor.x, y: actor.y, facing: actor.facing, activity: actor.activity, opacity: still && presence < actor.opacity ? presence : actor.opacity, bones, eyes, mood: actor.mood };
  });
  let rate: 0 | 16 | 32 | 64 = 0;
  if (!still) {
    for (let index = 0; index < actors.length; index++) {
      const pace = paceOf(stage, actors, kinds, index);
      if (pace > rate) rate = pace;
    }
  }
  let wake: Ticks | null = null;
  const arrival = arrivalTick(menagerie, { ...stage, actors }, tick + 1);
  if (rate === 0 && !still && (actors.length > 0 || arrival >= 0)) {
    let horizon = Infinity;
    for (const actor of actors) {
      horizon = Math.min(horizon, actor.until);
      if (actor.activity !== "sleep") horizon = Math.min(horizon, actor.blink > tick ? actor.blink : actor.blink + BLINK_TICKS);
      if (stage.pointer !== null && tick + 1 < actor.faced + TURN_REST) horizon = Math.min(horizon, actor.faced + TURN_REST);
    }
    if (actors.length > 0 && stage.pointer !== null && tick + 1 <= stage.pointed + PERK_LINGER) horizon = Math.min(horizon, stage.pointed + PERK_LINGER);
    if (actors.length > 0 && stage.pointer !== null && tick + 1 - stage.pointed < POINTER_TICKS) horizon = Math.min(horizon, stage.pointed + POINTER_TICKS);
    const pairing = pairingTick({ ...stage, actors }, tick + 1);
    if (pairing >= 0) horizon = Math.min(horizon, pairing);
    if (arrival >= 0) horizon = Math.min(horizon, arrival);
    wake = horizon > tick ? horizon : tick + 1;
  }
  return { tick, actors: frames, rate, wake };
}
//#endregion 🔖️Frame
