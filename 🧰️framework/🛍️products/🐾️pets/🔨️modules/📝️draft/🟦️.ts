/** ✏️ The draft of the stage: the working copy that `advance` folds events into — the actors in `menagerie.species` order, each beside its species and its random stream — with what every part of the stage needs of it: the lookups of surfaces, clips and actors, the keys of the counter-based draws, the stretches of a perch, and the smallest changes of an actor (taking up an activity, coming to rest, letting go of its partner, leaving the draft).
 *
 * What the fold must know beside the stage itself belongs to `Draft`; what a new part of the stage needs of every other part belongs here, below all of them.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { type Activity, type Actor, type Clip, type Feeling, type Menagerie, type Perch, type Point, type Slug, type Species, type Stage, type Surface, type Ticks, type Trick } from "../../🧬️schema/🟦️.ts";
import { clipTicks } from "../🎞️animation/🟦️.ts";
import { STAGE_STREAM, randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { emitterEnds } from "../✨️effects/🟦️.ts";
import { appraised, settled, stateAt, type Occasion, type Standing } from "../💗️feeling/🟦️.ts";
import { dwellOf, needsAfter } from "../🧠️behavior/🟦️.ts";

//#region 🔖️Constants
const BLINK_LOW = 128;
const BLINK_HIGH = 384;
export const ORIGIN: Point = { x: 0, y: 0 };
const NO_CLIPS: readonly Slug[] = [];
//#endregion 🔖️Constants

//#region 🔖️Draft
export type Body = { -readonly [field in keyof Actor]: Actor[field] };

export type Draft = { -readonly [field in Exclude<keyof Stage, "actors">]: Stage[field] } & { actors: Body[]; kinds: Species[]; streams: number[]; readonly menagerie: Menagerie };

export type Room = { readonly perch: Perch; readonly stretches: readonly number[]; readonly span: number; readonly crowd: number };

export type Launch = { readonly x: number; readonly y: number; readonly surface: string; readonly vx: number; readonly vy: number; readonly ticks: Ticks };

/** 📝️ A working copy of a stage: its actors copied, in `menagerie.species` order, each beside its species and its stream, and the menagerie it plays (its bonds); an actor of a species the menagerie does not have is dropped. */
export function draftOf(menagerie: Menagerie, stage: Stage): Draft {
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
    over: stage.over,
    glances: stage.glances,
    surfaces: stage.surfaces,
    keepouts: stage.keepouts,
    walls: stage.walls,
    fixtures: stage.fixtures,
    perches: stage.perches,
    pitches: stage.pitches,
    wanted: stage.wanted,
    rapports: stage.rapports,
    met: stage.met,
    draws: stage.draws,
    play: stage.play,
    mischief: stage.mischief,
    stirred: stage.stirred,
    scrolled: stage.scrolled,
    press: stage.press,
    touched: stage.touched,
    shaking: stage.shaking,
    trail: stage.trail,
    coolings: stage.coolings,
    pledges: stage.pledges,
    ladders: stage.ladders,
    lift: stage.lift,
    rested: stage.rested,
    poofs: stage.poofs,
    puffs: stage.puffs,
    claims: stage.claims,
    courses: stage.courses,
    trips: stage.trips,
    origin: stage.origin,
    actors,
    kinds,
    streams,
    menagerie,
  };
}

/** 📦️ The stage a draft has become. */
export function sealed(draft: Draft): Stage {
  return {
    seed: draft.seed,
    tick: draft.tick,
    mode: draft.mode,
    quiet: draft.quiet,
    width: draft.width,
    height: draft.height,
    pointer: draft.pointer,
    pointed: draft.pointed,
    over: draft.over,
    glances: draft.glances,
    surfaces: draft.surfaces,
    keepouts: draft.keepouts,
    walls: draft.walls,
    fixtures: draft.fixtures,
    perches: draft.perches,
    pitches: draft.pitches,
    wanted: draft.wanted,
    actors: draft.actors,
    rapports: draft.rapports,
    met: draft.met,
    draws: draft.draws,
    play: draft.play,
    mischief: draft.mischief,
    stirred: draft.stirred,
    scrolled: draft.scrolled,
    press: draft.press,
    touched: draft.touched,
    shaking: draft.shaking,
    trail: draft.trail,
    coolings: draft.coolings,
    pledges: draft.pledges,
    ladders: draft.ladders,
    lift: draft.lift,
    rested: draft.rested,
    poofs: draft.poofs,
    puffs: draft.puffs,
    claims: draft.claims,
    courses: draft.courses,
    trips: draft.trips,
    origin: draft.origin,
  };
}
//#endregion 🔖️Draft

//#region 🔖️Lookups
/** 🎈️ How high a species floats above its perch: its hover when its gait is `float`, else 0. */
export function hoverOf(kind: Species): number {
  return kind.locomotion.gait === "float" && kind.locomotion.hover !== undefined ? kind.locomotion.hover : 0;
}

/** 🔎️ The index of the actor of a species on stage, or −1. */
export function indexOf(actors: readonly Actor[], species: Slug): number {
  for (let index = 0; index < actors.length; index++) if (actors[index]!.species === species) return index;
  return -1;
}

/** 🪵️ The surface with an id, or `null`. */
export function surfaceOf(surfaces: readonly Surface[], id: string): Surface | null {
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
export function clipAt(kind: Species, activity: Activity, unit: number): Slug | null {
  const clips = clipsOf(kind, activity);
  if (clips.length === 0) return null;
  const index = Math.floor(unit * clips.length);
  return clips[index < clips.length ? index : clips.length - 1]!;
}

/** 📼️ The clip of a species with an id, or `null`. */
export function clipOf(kind: Species, id: Slug | null): Clip | null {
  if (id === null) return null;
  for (const clip of kind.clips) if (clip.id === id) return clip;
  return null;
}

/** 🫁️ The clip a species breathes with — its first idle clip, which loops under everything it does —, or `null` when it has none. */
export function breathOf(kind: Species): Clip | null {
  const idle = kind.repertoire.idle;
  return idle === undefined || idle.length === 0 ? null : clipOf(kind, idle[0]!);
}

/** ➡️ The way an actor at `x` faces when it faces `towards`: right when that is not to its left. */
export function facingTo(towards: number, x: number): 1 | -1 {
  return towards >= x ? 1 : -1;
}

/** 🔃️ Over how many ticks the drawing of an actor that turns round follows it, squeezed through a line. */
export const TURN_TICKS = 8;

/** 🙃️ An actor turns to face `way` at tick `now`: it faces that way at once, and its drawing follows — squeezed through a line — over the next `TURN_TICKS` ticks, up to `faced`. An actor that is still turning turns back from where its drawing is: the turn back takes as long as the turn has run. */
export function turn(body: Body, way: 1 | -1, now: Ticks): void {
  if (way === body.facing) return;
  const left = body.faced > now ? body.faced - now : 0;
  body.facing = way;
  body.faced = now + TURN_TICKS - left;
}

/** 😑️ The tick of the blink a unit draw schedules after tick `now`: 2…6 s ahead. */
export function blinkAt(now: Ticks, unit: number): Ticks {
  return now + BLINK_LOW + Math.floor((BLINK_HIGH - BLINK_LOW) * unit);
}

/** 🔑️ The key of the next draw of an actor; the draw is counted. */
export function actorKey(draft: Draft, index: number): number[] {
  const body = draft.actors[index]!;
  const counter = body.draws >>> 0;
  body.draws = (counter + 1) >>> 0;
  return [draft.seed, draft.streams[index]!, counter];
}

/** 🗝️ The key of the next draw of the stage; the draw is counted. */
export function stageKey(draft: Draft): number[] {
  const counter = draft.draws >>> 0;
  draft.draws = (counter + 1) >>> 0;
  return [draft.seed, STAGE_STREAM, counter];
}

/** ↔️ How far apart the feet of two actors on one perch must stay for their bodies not to overlap: half of both widths. */
export function shoulders(one: Species, other: Species): number {
  return (one.size.width + other.size.width) / 2;
}

/** 🎵️ The ticks of one hop of a hopping gait while it walks with `clip` — the gait covers ground in whole hops — and 1 for every other gait. */
export function beatOf(kind: Species, clip: Clip | null): Ticks {
  return kind.locomotion.gait === "hop" && clip !== null ? clipTicks(clip) : 1;
}

/** 🏃️ Whether an actor moves of its own accord: it walks or hops, or it is on a trip with its gear — what the movers a mode allows count. */
export function astir(draft: Draft, body: Body): boolean {
  return body.activity === "walk" || body.activity === "hop" || draft.trips.some((trip) => trip.owner === body.species);
}
//#endregion 🔖️Lookups

//#region 🔖️Activities
/** 🔀️ An actor takes up an activity at tick `now`: its needs are settled over the span of the one it leaves, and a trick or a purr it leaves stops making particles (a trick that is cut short leaves no state behind). */
export function shift(draft: Draft, index: number, activity: Activity, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (body.activity === "trick") {
    douse(draft, index, trickOf(kind, body.trick)?.emitter, body.since, now);
    body.trick = null;
  } else if (body.activity === "purr") douse(draft, index, kind.purr.emitter, body.since, now);
  body.needs = needsAfter(body.needs, body.activity, now - body.since, kind.temperament);
  body.activity = activity;
  body.since = now;
}

/** 🛋️ An actor comes to rest: idle where it stands for a drawn dwell with a drawn idle clip, without a partner. */
export function settle(draft: Draft, index: number, now: Ticks): void {
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
export function release(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.partner === null) return;
  const other = indexOf(draft.actors, body.partner);
  body.partner = null;
  if (other < 0 || draft.actors[other]!.partner !== body.species) return;
  draft.actors[other]!.partner = null;
  if (draft.actors[other]!.activity !== "sulk" && !draft.actors[other]!.leaving) settle(draft, other, now);
}

/** 🚪️ An actor leaves the stage for good: it is taken out of the draft, with its claim, its course and its trip, the ladder it climbs is free again, and the press no longer belongs to it. */
export function remove(draft: Draft, index: number): void {
  const species = draft.actors[index]!.species;
  if (draft.claims.some((claim) => claim.owner === species)) draft.claims = draft.claims.filter((claim) => claim.owner !== species);
  if (draft.courses.some((course) => course.owner === species)) draft.courses = draft.courses.filter((course) => course.owner !== species);
  if (draft.trips.some((trip) => trip.owner === species)) draft.trips = draft.trips.filter((trip) => trip.owner !== species);
  if (draft.ladders.some((ladder) => ladder.rider === species)) draft.ladders = draft.ladders.map((ladder) => (ladder.rider === species ? { ...ladder, rider: null } : ladder));
  if (draft.touched === species) draft.touched = null;
  draft.actors.splice(index, 1);
  draft.kinds.splice(index, 1);
  draft.streams.splice(index, 1);
}
//#endregion 🔖️Activities

//#region 🔖️Mind
/** 🃏️ The trick of a species with an id, or `null`. */
export function trickOf(kind: Species, id: Slug | null): Trick | null {
  if (id === null) return null;
  for (const trick of kind.tricks) if (trick.id === id) return trick;
  return null;
}

/** 🫀️ What an actor feels at tick `now`: its feeling settled to that tick against the resting mood of its species. Nothing is stored: a feeling is an anchor that only events move. */
export function present(draft: Draft, index: number, now: Ticks): Feeling {
  return settled(draft.actors[index]!.feeling, draft.kinds[index]!.mood, now);
}

/** 🚩️ The state an actor is in at tick `now` and the tick it began: the state it entered, given way along `lasts` and `then` as far as time says. */
export function standing(draft: Draft, index: number, now: Ticks): Standing {
  const body = draft.actors[index]!;
  return stateAt(draft.kinds[index]!, body.state, body.stateSince, now);
}

/** 🌊️ An actor feels what an occasion gives it at tick `now`, as its species takes it (`appraised`). */
export function feel(draft: Draft, index: number, occasion: Occasion, now: Ticks): void {
  draft.actors[index]!.feeling = appraised(present(draft, index, now), occasion, draft.kinds[index]!, now);
}

/** 🌫️ The plumes of an actor without those of which nothing is alive any more at `now` (`emitterEnds`); a plume of an emitter its species does not have is gone. */
function aired(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const kept = body.emitters.filter((plume) => {
    const emitter = kind.emitters.find((entry) => entry.id === plume.emitter);
    if (emitter === undefined) return false;
    const ends = emitterEnds(emitter, plume.since, plume.until);
    return ends === null || ends > now;
  });
  if (kept.length !== body.emitters.length) body.emitters = kept;
}

/** 💨️ An actor starts to run an emitter of its species at tick `now`: a plume that lasts until its cause ends. Nothing starts without an emitter or for one the species does not have. */
export function ignite(draft: Draft, index: number, emitter: Slug | undefined, now: Ticks): void {
  aired(draft, index, now);
  if (emitter === undefined || !draft.kinds[index]!.emitters.some((entry) => entry.id === emitter)) return;
  const body = draft.actors[index]!;
  body.emitters = [...body.emitters, { emitter, since: now, until: null }];
}

/** 🧯️ The cause of a plume ended at tick `now`: the latest running plume of the emitter that began at `since` stops making particles; what it made lives on. */
export function douse(draft: Draft, index: number, emitter: Slug | undefined, since: Ticks, now: Ticks): void {
  aired(draft, index, now);
  if (emitter === undefined) return;
  const body = draft.actors[index]!;
  for (let at = body.emitters.length - 1; at >= 0; at--) {
    const plume = body.emitters[at]!;
    if (plume.emitter !== emitter || plume.since !== since || plume.until !== null) continue;
    body.emitters = body.emitters.map((entry, place) => (place === at ? { emitter: entry.emitter, since: entry.since, until: now } : entry));
    return;
  }
}

/** 🔦️ An actor enters a state at tick `since` (a state it is in begins anew): the emitter of the state it leaves stops at `now`, the emitter of the new one starts, and the state it was in just before `since` becomes its `former` one, from which the drawing blends. A state the species does not have changes nothing. */
export function enter(draft: Draft, index: number, state: Slug, since: Ticks, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const next = kind.states.find((entry) => entry.id === state);
  if (next === undefined) return;
  douse(draft, index, kind.states.find((entry) => entry.id === body.state)?.emitter, body.stateSince, now);
  body.former = since > body.stateSince ? stateAt(kind, body.state, body.stateSince, since - 1).state : body.state;
  body.state = state;
  body.stateSince = since;
  if (next.emitter !== undefined) {
    aired(draft, index, now);
    body.emitters = [...body.emitters, { emitter: next.emitter, since, until: null }];
  }
}

/** ⌛️ An actor's state catches up with time at tick `now`: when a lasting state has given way ({@link standing}), the actor enters the state it is in now, from the tick that one began, and the emitters follow. */
export function stand(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const current = standing(draft, index, now);
  if (current.state !== body.state || current.since !== body.stateSince) enter(draft, index, current.state, current.since, now);
}

/** 🪄️ An actor performs a trick at tick `now`: it plays the trick's clip once — for the length of the clip, or the trick's dwell when the clip loops or is missing —, standing where it is, with the particles of the trick's emitter. Its partner stays: a pet that shows off to another performs for it. */
export function perform(draft: Draft, index: number, trick: Trick, now: Ticks): void {
  const body = draft.actors[index]!;
  shift(draft, index, "trick", now);
  const clip = clipOf(draft.kinds[index]!, trick.clip);
  body.trick = trick.id;
  body.clip = trick.clip;
  body.until = now + (clip !== null && !clip.loop ? clipTicks(clip) : dwellOf("trick", draft.mode, 0));
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  ignite(draft, index, trick.emitter, now);
}

/** 😻️ An actor purrs at tick `now` for at least `ticks` more and at least one whole loop of its purr clip: the clip loops with the particles of its purr emitter; a purr that runs is only drawn out. */
export function purr(draft: Draft, index: number, ticks: Ticks, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const loop = clipOf(kind, kind.purr.clip);
  const span = loop !== null && clipTicks(loop) > ticks ? clipTicks(loop) : ticks;
  if (body.activity === "purr") {
    if (body.until < now + span) body.until = now + span;
    return;
  }
  shift(draft, index, "purr", now);
  body.clip = kind.purr.clip;
  body.until = now + span;
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  ignite(draft, index, kind.purr.emitter, now);
}
//#endregion 🔖️Mind

//#region 🔖️Stretches
/** 🔪️ Stretches (pairs of ends) without the interval `(low, high)`. */
export function carve(stretches: readonly number[], low: number, high: number): number[] {
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
export function crowdOn(draft: Draft, perch: Perch): number {
  let crowd = 0;
  for (const body of draft.actors) if (body.perch === perch.surface && body.x >= perch.x0 && body.x <= perch.x1) crowd++;
  return crowd;
}
//#endregion 🔖️Stretches
