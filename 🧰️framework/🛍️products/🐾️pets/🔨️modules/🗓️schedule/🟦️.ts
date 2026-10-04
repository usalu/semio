/** 🗓️ The schedule of the stage: the changes that are due at a known tick and depend on more than one actor — the next whole second on which a pair may be drawn (`pairingTick`), the next on which somebody who waits off stage may arrive (`arrivalTick`), the next beat on which moods may travel or the chemistry may act (`beatTick`), the next tick on which the mischief of the stage does something (`prankTick`, with the pets that could play with a marked element of the page, `prospectsOf`) — and whether the gestures of the pointer must see the coming tick (`sampling`).
 *
 * The clock jumps to these ticks (`lull`) and the frame reports them (`wake`). A new time horizon of the stage is a function here and a line in both of them.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ../🪄️mischief/🟦️.ts — the topic match, the stations, the gates and the lift of mischief
 * @see ./🦀️.rs — the Rust twin
 */

import { MOODS, TICKS_PER_SECOND, type Actor, type Fixture, type Menagerie, type Perch, type Pitch, type Slug, type Species, type Stage, type Ticks } from "../../🧬️schema/🟦️.ts";
import { CHEMISTRY_BEAT, CONTAGION_BEAT, MOOD_PRIORITIES, MOOD_REST, MOOD_SPREADS, calmsAt, heldTicks, settled, stateAt, trialsOf, type Sighting } from "../💗️feeling/🟦️.ts";
import { hoverOf, indexOf } from "../📝️draft/🟦️.ts";
import { circled } from "../👆️gesture/🟦️.ts";
import { MODE_LIMITS } from "../🧠️behavior/🟦️.ts";
import { clingOf, footOf, rimOf } from "../🧗️climbing/🟦️.ts";
import { LIFT_RETURNS, LIFT_TICKS, allowedFrom, fixtureFor, stationFor, type Circumstances } from "../🪄️mischief/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";

//#region 🔖️Constants
const WARMUP = 1280;
//#endregion 🔖️Constants

//#region 🔖️Schedule
/** 🙌️ Whether an actor can be drawn into an encounter or a prank at tick `tick`: standing idle and whole on a perch, without a partner, not leaving, and not held by a circle the pointer makes round it (`circled`). */
export function sociable(actor: Actor, tick: Ticks): boolean {
  return actor.activity === "idle" && actor.partner === null && !actor.leaving && actor.opacity === 1 && actor.footing === "perch" && !circled(actor.hover, tick);
}

/** 📅️ The first whole second at or after tick `from` on which the stage may draw a pair, or −1 while it may not: the mode has encounters, it is not a quiet time, nobody has a partner, at least two actors are sociable, and the warm-up (20 s before the first encounter) or the gap of the mode since the last change of a rapport has passed. */
export function pairingTick(stage: Stage, from: Ticks): Ticks {
  const limits = MODE_LIMITS[stage.mode];
  if (limits.encounterGap === 0 || stage.quiet) return -1;
  let free = 0;
  for (const actor of stage.actors) {
    if (actor.partner !== null) return -1;
    if (sociable(actor, from)) free++;
  }
  if (free < 2) return -1;
  const open = stage.met === 0 ? WARMUP : stage.met + limits.encounterGap;
  const first = from > open ? from : open;
  return Math.floor((first + TICKS_PER_SECOND - 1) / TICKS_PER_SECOND) * TICKS_PER_SECOND;
}

/** 🚏️ The first whole second at or after tick `from` on which a species that is wanted but not on stage may arrive, or −1 while nobody waits, no perch exists or the stage is still (a still stage lets them arrive with its events). */
export function arrivalTick(menagerie: Menagerie, stage: Stage, from: Ticks): Ticks {
  if (stage.mode === "still" || stage.perches.length === 0) return -1;
  for (const kind of menagerie.species) {
    if (stage.wanted.includes(kind.id) && indexOf(stage.actors, kind.id) < 0) return Math.floor((from + TICKS_PER_SECOND - 1) / TICKS_PER_SECOND) * TICKS_PER_SECOND;
  }
  return -1;
}

/** 👀️ Every actor that stays on stage as the chemistry sees it at tick `tick`: its species, the state it is in and for how many ticks it has held it, the mood it feels and how strongly, its activity and trick, and the box of its species at its feet. */
export function sightingsOf(menagerie: Menagerie, stage: Stage, tick: Ticks): Sighting[] {
  const sightings: Sighting[] = [];
  for (const actor of stage.actors) {
    const kind = menagerie.species.find((species) => species.id === actor.species);
    if (actor.leaving || kind === undefined) continue;
    const standing = stateAt(kind, actor.state, actor.stateSince, tick);
    const feeling = settled(actor.feeling, kind.mood, tick);
    sightings.push({ species: actor.species, state: standing.state, held: tick - standing.since, mood: feeling.mood, intensity: feeling.intensity, activity: actor.activity, trick: actor.trick, x: actor.x, y: actor.y, width: kind.size.width, height: kind.size.height });
  }
  return sightings;
}

/** ⏭️ The first multiple of `beat` at or after tick `from`. */
function beatAfter(from: Ticks, beat: Ticks): Ticks {
  return Math.floor((from + beat - 1) / beat) * beat;
}

/** ⚗️ The first tick at or after `from` on which the chemistry of a stage at rest can find something due, or −1: `from` itself when a reaction is due already, else the earliest end of a cooling and the earliest tick a state has been held long enough for a trait that asks it. Everything else a reaction reads changes only on horizons of its own (a state that gives way, a mood that calms, the end of an activity) or with motion. */
function dueTick(menagerie: Menagerie, stage: Stage, from: Ticks): Ticks {
  const sightings = sightingsOf(menagerie, stage, from);
  if (trialsOf(menagerie, sightings, stage.coolings, from, stage.rapports).length > 0) return from;
  let due = -1;
  for (const cooling of stage.coolings) if (cooling.until > from && (due < 0 || cooling.until < due)) due = cooling.until;
  for (const reaction of menagerie.chemistry) {
    for (const trait of reaction.unless === undefined ? [reaction.when, reaction.near] : [reaction.when, reaction.near, reaction.unless]) {
      if (trait.held === undefined) continue;
      for (const sighting of sightings) {
        const ripe = from - sighting.held + heldTicks(trait.held);
        if (ripe > from && (due < 0 || ripe < due)) due = ripe;
      }
    }
  }
  return due;
}

/** 🥁️ The first beat at or after tick `from` on which moods may travel or the chemistry may act, or −1 while neither can: a contagion beat while at least two actors stay on stage and one of them shows a mood that spreads (until it calms, `calmsAt`); a chemistry beat while the menagerie has chemistry, at least two actors stay on stage, the stage is not still, it is no time of concentration and a reaction can be due by then ({@link dueTick}). */
export function beatTick(menagerie: Menagerie, stage: Stage, from: Ticks): Ticks {
  if (stage.mode === "still") return -1;
  let staying = 0;
  let stirred = false;
  for (const actor of stage.actors) {
    if (actor.leaving) continue;
    staying++;
    const kind = menagerie.species.find((species) => species.id === actor.species);
    if (kind !== undefined && MOOD_SPREADS[MOODS.indexOf(actor.feeling.mood)]! > 0 && calmsAt(actor.feeling, kind.mood) > from) stirred = true;
  }
  if (staying < 2) return -1;
  const contagion = stirred ? beatAfter(from, CONTAGION_BEAT) : -1;
  const due = menagerie.chemistry.length > 0 && !stage.quiet ? dueTick(menagerie, stage, from) : -1;
  const chemistry = due < 0 ? -1 : beatAfter(due, CHEMISTRY_BEAT);
  return contagion < 0 ? chemistry : chemistry < 0 || contagion < chemistry ? contagion : chemistry;
}

/** 🖱️ Whether the gestures of the pointer must see tick `from`: play is permitted, a pointer is on stage, no press is open, the stage is neither still nor quiet, and the pointer moved or the page scrolled on the tick before. */
export function sampling(stage: Stage, from: Ticks): boolean {
  return stage.play && stage.pointer !== null && stage.press.phase === "idle" && stage.mode !== "still" && !stage.quiet && (from - stage.pointed <= 1 || from - stage.scrolled <= 1);
}
//#endregion 🔖️Schedule

//#region 🔖️Mischief
/** 📮️ Where a pet works on a marked element of the page: on the wall `pitch` with its feet at height `y`, or on `perch` with its feet at `x`. */
export type Post = { readonly kind: "wall"; readonly pitch: Pitch; readonly y: number } | { readonly kind: "perch"; readonly perch: Perch; readonly x: number };

/** 🔭️ A pet that could play with a fixture: the actor, the fixture, its post beside it, the way it would shove the copy (`side`: 1 to the right) and how far it may (`room`: the room beyond the fixture's far end, at most the pet's own width). */
export type Prospect = { readonly actor: Slug; readonly fixture: Fixture; readonly post: Post; readonly side: 1 | -1; readonly room: number };

/** 🛎️ Where a species whose actor stands with its feet at (`x`, `y`) works on `fixture`, or `null`. The stations (`stationFor`) are every perch beside the fixture — there with its feet half its width beside the station, on the perch and its body clear of the fixture — and, with climbing gear and no floating gait, every free stretch of wall its hands hold at a height from which its body lies beside the fixture (its feet at the fixture's lower edge, held between `rimOf` and `footOf`). The one that leaves the copy the most of the room it may have (beyond the far end, at most its own width) wins, then the one nearest to where the actor stands (across plus up or down), then the perches before the walls, each in the survey's order. How it gets there is the pet's affair (`postTo`). */
function postOf(kind: Species, fixture: Fixture, x: number, y: number, perches: readonly Perch[], pitches: readonly Pitch[], width: number): { readonly post: Post; readonly side: 1 | -1; readonly room: number } | null {
  const size = kind.size;
  const half = size.width / 2;
  const top = fixture.y;
  const bottom = fixture.y + fixture.height;
  let best: { readonly post: Post; readonly side: 1 | -1; readonly room: number } | null = null;
  let farness = 0;
  for (const perch of perches) {
    const station = stationFor(fixture, [], [perch], width);
    if (station === null || perch.x1 - perch.x0 < size.width) continue;
    const stand = clamp(station.x - station.side * half, perch.x0 + half, perch.x1 - half);
    const room = Math.min(station.room, size.width);
    const far = Math.abs(stand - x) + Math.abs(perch.y - hoverOf(kind) - y);
    if ((station.side > 0 ? stand + half > fixture.x : stand - half < fixture.x + fixture.width) || (best !== null && (room < best.room || (room === best.room && far >= farness)))) continue;
    best = { post: { kind: "perch", perch, x: stand }, side: station.side, room };
    farness = far;
  }
  if (!kind.gear.includes("climb") || kind.locomotion.gait === "float") return best;
  for (const pitch of pitches) {
    const rim = rimOf(pitch, size);
    const foot = footOf(pitch, size);
    const feet = clamp(bottom, rim, foot);
    const wall = rim > foot || !(feet - size.height < bottom && feet > top) ? null : stationFor(fixture, [pitch], [], width);
    if (wall === null) continue;
    const room = Math.min(wall.room, size.width);
    const far = Math.abs(clingOf(pitch, size) - x) + Math.abs(feet - y);
    if (best !== null && (room < best.room || (room === best.room && far >= farness))) continue;
    best = { post: { kind: "wall", pitch, y: feet }, side: wall.side, room };
    farness = far;
  }
  return best;
}

/** 🗳️ The pets that could play with a marked element of the page at tick `tick`, in `menagerie.species` order and each with its fixtures in the survey's order: an actor that stands idle and whole on a perch, without a partner, not leaving, not circled and on no trip ({@link sociable}), of a species with a ground that covers the fixture's key (`fixtureFor` — the key is all it reads of the element besides its box), with a post beside it ({@link postOf}); none while as many actors move (walk, hop or travel) as the mode allows, since the way to a post moves one more. */
export function prospectsOf(menagerie: Menagerie, stage: Stage, tick: Ticks): Prospect[] {
  const prospects: Prospect[] = [];
  if (stage.fixtures.length === 0) return prospects;
  let movers = 0;
  for (const actor of stage.actors) if (actor.activity === "walk" || actor.activity === "hop" || stage.trips.some((trip) => trip.owner === actor.species)) movers++;
  if (movers >= MODE_LIMITS[stage.mode].movers) return prospects;
  for (const actor of stage.actors) {
    const kind = menagerie.species.find((species) => species.id === actor.species);
    if (kind === undefined || !sociable(actor, tick) || actor.perch === null || stage.trips.some((trip) => trip.owner === actor.species)) continue;
    for (const fixture of fixtureFor(kind.grounds, stage.fixtures)) {
      const way = postOf(kind, fixture, actor.x, actor.y, stage.perches, stage.pitches, stage.width);
      if (way !== null) prospects.push({ actor: actor.species, fixture, post: way.post, side: way.side, room: way.room });
    }
  }
  return prospects;
}

/** 🚥️ What the gates of mischief see on a stage at tick `tick`: the learner's consent (`mischief`; the render target grants it only where the pointer is fine, so the gate of the pointer stands open here), the width of the stage, its mode and whether it is a time of concentration, whether a prank is under way, the learner's last stir — the latest of an input, a move of the pointer and a scroll — and the tick the last prank ended. */
export function circumstancesOf(stage: Stage, tick: Ticks): Circumstances {
  return { permitted: stage.mischief, fine: true, width: stage.width, mode: stage.mode, quiet: stage.quiet, lifting: stage.lift !== null, tick, stirred: Math.max(stage.stirred, stage.pointed, stage.scrolled), rested: stage.rested };
}

/** 🪄️ The first tick at or after `from` on which the mischief of a stage does something, or −1 while it waits for nothing but motion and the horizons of its actors. Without a prank: the tick the gates open (`allowedFrom`) while somebody could play ({@link prospectsOf}). With one (`lift`), the tick it begins and the tick it ends — and `from` itself while its pet is on its way and gone, leaving or off its trip before the tick it was to arrive, and while the copy is out and its pusher is gone, leaving or no longer pushes before the copy has begun to slide home (`LIFT_RETURNS`). After the learner threw the pusher off: −1 while it is in the air, under its parachute or in the hand; once it is down again, the first tick at which sheepishness (sad) can show — at once, unless its fright or anger still shows (a mood that outranks sadness, above its rest), then the tick that mood calms (`calmsAt`) —; `from` once it is gone. */
export function prankTick(menagerie: Menagerie, stage: Stage, from: Ticks): Ticks {
  const lift = stage.lift;
  if (lift === null) {
    const opens = allowedFrom(circumstancesOf(stage, from));
    if (opens === null || prospectsOf(menagerie, stage, from).length === 0) return -1;
    return opens > from ? opens : from;
  }
  const index = indexOf(stage.actors, lift.pusher);
  const pusher = index < 0 ? null : stage.actors[index]!;
  const age = from - lift.since;
  if (age > LIFT_TICKS) {
    const kind = menagerie.species.find((species) => species.id === lift.pusher);
    if (pusher === null || kind === undefined) return from;
    if (pusher.footing === "air" || pusher.footing === "chute" || pusher.footing === "hand") return -1;
    const feeling = settled(pusher.feeling, kind.mood, from);
    if (!(feeling.intensity > MOOD_REST) || MOOD_PRIORITIES[MOODS.indexOf(feeling.mood)]! <= MOOD_PRIORITIES[MOODS.indexOf("sad")]!) return from;
    const calm = calmsAt(pusher.feeling, kind.mood);
    return calm > from ? calm : from;
  }
  if (age === LIFT_TICKS || age === 0) return from;
  if (age < 0) return pusher === null || pusher.leaving || (age < -1 && !stage.trips.some((trip) => trip.owner === lift.pusher)) ? from : lift.since;
  return (pusher === null || pusher.leaving || pusher.activity !== "push") && age < LIFT_RETURNS ? from : lift.since + LIFT_TICKS;
}
//#endregion 🔖️Mischief
