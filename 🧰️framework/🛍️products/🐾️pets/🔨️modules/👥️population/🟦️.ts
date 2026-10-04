/** 👥️ Population: who is on stage and where the ground is — the survey (perches cut anew, grounded actors riding their surfaces and seated apart again), arrivals and spreading out over new ground, the summons, and the liveliness (freezing and thawing).
 *
 * What a survey brings and what it does to the company belongs here.
 * A part of the stage, not of the package: `@semio-tech/pets` exports nothing of it.
 *
 * @see ../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 * @see ./🦀️.rs — the Rust twin
 */

import { type Fixture, type Ladder, type Menagerie, type Perch, type PetMode, type Point, type Slug, type Species, type Surface, type Surveyed, type Ticks, type Wall } from "../../🧬️schema/🟦️.ts";
import { MARGIN, evicted, freeAt, seatOf, shifted, type Body as Solid } from "../🚧️clearance/🟦️.ts";
import { randomWords, unitOf } from "../🎲️randomness/🟦️.ts";
import { landingOf, perchesOf, wallsOf } from "../🏞️terrain/🟦️.ts";
import { bodiesOf, extentOf, quarters, roomsFor } from "../📏️spacing/🟦️.ts";
import { clamp } from "../📐️trigonometry/🟦️.ts";
import { actorKey, blinkAt, clipAt, crowdOn, feel, hoverOf, indexOf, release, remove, settle, shift, stageKey, surfaceOf, type Body, type Draft } from "../📝️draft/🟦️.ts";
import { courseOf, drop, halt, leave, poof, tripOf, unfoot, unlifted, vanish } from "../🚶️locomotion/🟦️.ts";
import { MODE_LIMITS, dwellOf, needsOf } from "../🧠️behavior/🟦️.ts";
import { atRest } from "../💗️feeling/🟦️.ts";
import { COLD, IDLE, circled, noHover } from "../👆️gesture/🟦️.ts";
import { GRIP_BUDGET, clingOf, ladderHolds, slipOf, spillOf, wallHolds } from "../🧗️climbing/🟦️.ts";

//#region 🔖️Constants
const STRANDED = 2;
const WALL_ROOM = 8;
const WALL_LEAST = 0.5;

/** 📌️ How many pixels an edge of a marked element may move between two surveys while its copy is out before the prank ends: what the render target lets it move before it gives the element back. */
export const FIXTURE_SLACK = 0.5;
//#endregion 🔖️Constants

//#region 🔖️Riding
/** 📐️ The perches and the pitches of the stage for everyone who is on it or wanted: perches with clearance = the tallest of them plus its hover and minimum = the widest, so that every perch carries any of them; pitches (`wallsOf`) with a band of the widest plus `WALL_ROOM` beside the wall and at least `WALL_LEAST` of the smallest height long, so that every pitch takes any of them and is worth a climb; none when nobody is there. */
function measure(menagerie: Menagerie, draft: Draft): void {
  let tallest = 0;
  let widest = 0;
  let smallest = Infinity;
  for (const kind of menagerie.species) {
    if (!draft.wanted.includes(kind.id) && indexOf(draft.actors, kind.id) < 0) continue;
    tallest = Math.max(tallest, kind.size.height + hoverOf(kind));
    widest = Math.max(widest, kind.size.width);
    smallest = Math.min(smallest, kind.size.height);
  }
  draft.perches = perchesOf(draft.surfaces, draft.keepouts, draft.width, draft.height, tallest, widest);
  draft.pitches = smallest === Infinity ? [] : wallsOf(draft.walls, draft.keepouts, draft.width, draft.height, widest + WALL_ROOM, WALL_LEAST * smallest);
}

/** 🟰️ Whether two lists of flat records (perches, pitches, boxes) hold the same values in the same order. */
function alike(one: readonly object[], other: readonly object[]): boolean {
  if (one.length !== other.length) return false;
  for (let index = 0; index < one.length; index++) {
    const left = one[index] as Readonly<Record<string, unknown>>;
    const right = other[index] as Readonly<Record<string, unknown>>;
    for (const key of Object.keys(left)) if (left[key] !== right[key]) return false;
  }
  return true;
}

/** 🧗️ Whoever holds on to a ladder or a wall rides with it after a survey moved the terrain. A ladder whose foot and top still stand (`ladderHolds`, for the size of its owner) follows and carries its climber at the same share of its length; one that no longer stands is gone, and its climber steps off where it is low (`spillOf`) or is thrown clear of the wall, with a fright (`startled`). A climber on a wall moves up or down with the top of its wall (`walls`: the walls before the survey) and keeps hold where a pitch of that wall still carries it (`wallHolds`), half its width out from it; else the wall throws it off (`slipOf`), with a fright. */
function holdOn(menagerie: Menagerie, draft: Draft, walls: readonly Wall[], now: Ticks): void {
  const kept: Ladder[] = [];
  for (const ladder of draft.ladders) {
    const owner = menagerie.species.find((kind) => kind.id === ladder.owner);
    const next = owner === undefined ? null : ladderHolds(ladder, draft.perches, draft.pitches, draft.keepouts, owner.size);
    const rider = ladder.rider === null ? -1 : indexOf(draft.actors, ladder.rider);
    const climber = rider >= 0 && draft.actors[rider]!.footing === "ladder" ? draft.actors[rider]! : null;
    if (next !== null) {
      kept.push({ owner: ladder.owner, wall: next.wall, surface: next.surface, side: next.side, foot: next.foot, top: next.top, since: ladder.since, until: ladder.until, rider: ladder.rider });
      if (climber === null) continue;
      const share = clamp((climber.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0, 1);
      climber.x = next.foot.x + (next.top.x - next.foot.x) * share;
      climber.y = next.foot.y + (next.top.y - next.foot.y) * share;
      continue;
    }
    if (climber === null) continue;
    const toss = spillOf(ladder, climber.y, draft.kinds[rider]!.size);
    feel(draft, rider, "startled", now);
    unfoot(draft, rider, toss === null ? 0 : toss.vx, toss === null ? 0 : toss.vy, "fall", now);
  }
  draft.ladders = kept;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const pitch = body.pitch;
    if (body.footing !== "wall" || pitch === null) continue;
    const size = draft.kinds[index]!.size;
    const old = walls.find((wall) => wall.id === pitch.wall);
    const fresh = draft.walls.find((wall) => wall.id === pitch.wall);
    const y = old === undefined || fresh === undefined ? body.y : body.y + (fresh.y0 - old.y0);
    const next = wallHolds(pitch, draft.pitches, y, size);
    if (next !== null) {
      body.pitch = next;
      body.x = clingOf(next, size);
      body.y = y;
      continue;
    }
    const toss = slipOf(pitch);
    feel(draft, index, "startled", now);
    unfoot(draft, index, toss.vx, toss.vy, "fall", now);
  }
}

/** 🚡️ A grounded actor rides its surface: it moves rigidly with the surface's left end and height (its goal with it), however far that is and wherever that leaves it — the seating sets it right ({@link seat}). When its surface has no perch left it stays on a perch of another surface that lies exactly under its feet (an element that was replaced by its like); without one it falls — unless the survey also brought new ground (`uprooted`: the scenery changed, and a fall would pass in front of whatever is there now) or the stage is still: then it is gone with its ground and arrives anew. `false` when the actor is gone. */
function carry(draft: Draft, index: number, before: readonly Surface[], uprooted: boolean, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const surface = body.perch;
  if (body.footing !== "perch" || surface === null) return true;
  const old = surfaceOf(before, surface);
  const fresh = surfaceOf(draft.surfaces, surface);
  let held = false;
  for (const perch of draft.perches) if (perch.surface === surface) held = true;
  if (fresh !== null && held) {
    const moved = old === null ? 0 : fresh.x0 - old.x0;
    body.x = body.x + moved;
    body.goal = body.goal + moved;
    body.y = fresh.y - hoverOf(kind);
    return true;
  }
  const like = landingOf(draft.perches, body.x, body.y + hoverOf(kind), body.y + hoverOf(kind));
  if (like !== null) {
    body.perch = like.surface;
    return true;
  }
  if (uprooted || draft.mode === "still") return vanish(draft, index, now);
  drop(draft, index, now);
  return true;
}

/** 🪑️ The grounded actors of every perch are seated after a ride. Each belongs to the perch of its surface it is nearest to; whoever its perch would have to carry farther than `STRANDED` of its widths has lost its ground and falls (on a still stage it is gone and arrives anew). Per perch the others are seated by `seatOf` — everybody moves as little as possible, keeps its order, keeps a seam from its neighbours and stands with its footprint on the perch —, in the order of their right to stay: whoever its perch strains least first (among equals in `menagerie.species` order), leavers last. Whoever does not fit leaves the perch: off it, it falls; on it, it vanishes in a puff and arrives anew where there is room. Whoever has to move scoots to its seat, guarded, together with the others of its surface ({@link scoot}); on a still stage, where nothing moves, it is at its seat at once when that is free and vanishes otherwise. */
function seat(draft: Draft): void {
  const now = draft.tick;
  const stranded: string[] = [];
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (body.footing !== "perch" || body.perch === null) continue;
    const half = draft.kinds[index]!.size.width / 2;
    let least = Infinity;
    for (const candidate of draft.perches) {
      if (candidate.surface !== body.perch) continue;
      const gap = body.x < candidate.x0 + half ? candidate.x0 + half - body.x : body.x > candidate.x1 - half ? body.x - (candidate.x1 - half) : 0;
      if (gap < least) least = gap;
    }
    if (least > STRANDED * draft.kinds[index]!.size.width) stranded.push(body.species);
  }
  for (const species of stranded) {
    const index = indexOf(draft.actors, species);
    if (draft.mode === "still") vanish(draft, index, now);
    else drop(draft, index, now);
  }
  for (const perch of draft.perches) {
    const members: number[] = [];
    const strains: number[] = [];
    for (let index = 0; index < draft.actors.length; index++) {
      const body = draft.actors[index]!;
      if (body.footing !== "perch" || body.perch !== perch.surface) continue;
      const half = draft.kinds[index]!.size.width / 2;
      let nearest: Perch | null = null;
      let least = Infinity;
      for (const candidate of draft.perches) {
        if (candidate.surface !== perch.surface) continue;
        const gap = body.x < candidate.x0 + half ? candidate.x0 + half - body.x : body.x > candidate.x1 - half ? body.x - (candidate.x1 - half) : 0;
        if (gap < least) {
          nearest = candidate;
          least = gap;
        }
      }
      if (nearest !== perch) continue;
      const strain = body.leaving ? Infinity : least;
      let at = members.length;
      while (at > 0 && strains[at - 1]! > strain) at--;
      members.splice(at, 0, index);
      strains.splice(at, 0, strain);
    }
    if (members.length === 0) continue;
    for (const member of members) {
      const body = draft.actors[member]!;
      const half = draft.kinds[member]!.size.width / 2;
      if (body.activity === "walk") body.goal = clamp(body.goal, perch.x0 + half, perch.x1 - half);
    }
    const seating = seatOf(
      members.map((member) => ({ owner: draft.actors[member]!.species, extent: extentOf(draft.actors[member]!, draft.kinds[member]!) })),
      perch,
      MARGIN,
    );
    for (const owner of seating.leavers) {
      const index = indexOf(draft.actors, owner);
      if (index < 0) continue;
      const body = draft.actors[index]!;
      if (body.x < perch.x0 || body.x > perch.x1) drop(draft, index, now);
      else poof(draft, index, now);
    }
    for (const place of seating.seats) {
      if (place.shift === 0) continue;
      const index = indexOf(draft.actors, place.owner);
      if (index < 0) continue;
      const body = draft.actors[index]!;
      const half = draft.kinds[index]!.size.width / 2;
      const target = clamp(body.x + place.shift, perch.x0 + half, perch.x1 - half);
      if (draft.mode === "still") {
        const bodies = bodiesOf(draft.actors, draft.kinds);
        const seated = shifted(bodies[index]!.extent, target - body.x, 0);
        if (freeAt(seated, place.owner, bodies, draft.claims, now)) {
          body.x = target;
          body.goal = body.x;
        } else vanish(draft, index, now);
        continue;
      }
      release(draft, index, now);
      if (body.activity !== "scoot") shift(draft, index, "scoot", now);
      body.clip = clipAt(draft.kinds[index]!, "scoot", 0);
      body.goal = target;
      body.until = now + dwellOf("scoot", draft.mode, 0);
    }
  }
}

/** 🆕️ Whether the surfaces of the stage hold one that was not there `before`: new ground. */
function widened(draft: Draft, before: readonly Surface[]): boolean {
  for (const surface of draft.surfaces) if (surfaceOf(before, surface.id) === null) return true;
  return false;
}

/** 🎠️ The company after the surfaces moved from where they were `before` (`uprooted`: new ground came with them; `walls`: the walls before; `reshaped`: the perches, the pitches or the keep-outs changed). When the terrain changed, every trip with gear is given up (`halt`: on its perch its owner comes to rest, on a wall or a ladder it holds on, on a rope it falls). Whatever was planned through the air is given up — its owner plans anew in its turn, with the speed it has, from where it is —; whoever the stage no longer holds (its feet beyond the edges of a stage that narrowed) is gone with it and arrives anew; every grounded actor rides its surface ({@link carry}), whoever holds on to a ladder or a wall rides with it ({@link holdOn}) and whoever stands on its head rides along; the side edges of the stage stop whatever a surface carried towards them (its body stays inside the stage); whoever a moving surface carried into somebody who did not move, into a corridor or onto a landing spot vanishes in a puff (`evicted`, the unmoved first); then everybody on a perch is seated ({@link seat}). Nobody is ever moved into anybody. */
function ride(menagerie: Menagerie, draft: Draft, before: readonly Surface[], walls: readonly Wall[], uprooted: boolean, reshaped: boolean): void {
  const now = draft.tick;
  for (let index = 0; index < draft.actors.length && reshaped; index++) {
    const body = draft.actors[index]!;
    if (tripOf(draft.trips, body.species) === null) continue;
    const perched = body.footing === "perch";
    halt(draft, index, now);
    if (perched) settle(draft, index, now);
  }
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    if (courseOf(draft.courses, body.species) !== null) unfoot(draft, index, body.vx, body.vy, body.activity, now);
  }
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    if (body.footing !== "hand" && (body.x < 0 || body.x > draft.width)) vanish(draft, index, now);
    else index++;
  }
  const was: Point[] = draft.actors.map((body) => ({ x: body.x, y: body.y }));
  const species = draft.actors.map((body) => body.species);
  for (let index = 0; index < draft.actors.length; ) if (carry(draft, index, before, uprooted, now)) index++;
  if (reshaped) holdOn(menagerie, draft, walls, now);
  for (const body of draft.actors) {
    if (body.footing !== "head" || body.host === null) continue;
    const host = indexOf(draft.actors, body.host);
    const earlier = species.indexOf(body.host);
    if (host < 0 || earlier < 0) continue;
    body.x = body.x + (draft.actors[host]!.x - was[earlier]!.x);
    body.y = body.y + (draft.actors[host]!.y - was[earlier]!.y);
  }
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const half = draft.kinds[index]!.size.width / 2;
    if (body.footing !== "hand" && half <= draft.width - half) body.x = clamp(body.x, half, draft.width - half);
  }
  const bodies = bodiesOf(draft.actors, draft.kinds);
  const ranked: Solid[] = [];
  const moved: Solid[] = [];
  for (let index = 0; index < draft.actors.length; index++) {
    const earlier = species.indexOf(draft.actors[index]!.species);
    const still = earlier >= 0 && was[earlier]!.x === draft.actors[index]!.x && was[earlier]!.y === draft.actors[index]!.y;
    if (still) ranked.push(bodies[index]!);
    else moved.push(bodies[index]!);
  }
  for (const owner of evicted([...ranked, ...moved], draft.claims, now)) {
    const index = indexOf(draft.actors, owner);
    if (index >= 0) poof(draft, index, now);
  }
  seat(draft);
}
//#endregion 🔖️Riding

//#region 🔖️Arrival
/** 🌟️ A wanted species arrives: one stage draw picks one of its {@link quarters} (uniformly), a place on it a comfortable gap away from everybody there and the way it faces; one actor draw its first idle dwell, idle clip and blink. It fades in (on a still stage it is simply there), standing on its perch, feeling the resting mood of its species at rest, in the first state of its species, untouched and empty-handed. While no perch has room it waits off stage: nothing is drawn, and it is tried again with every survey, every summons and every whole second. */
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
    host: null,
    pitch: null,
    grip: GRIP_BUDGET,
    footing: "perch",
    x,
    y: room.perch.y - hoverOf(kind),
    vx: 0,
    vy: 0,
    tilt: 0,
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
    needs: needsOf(kind.temperament),
    opacity: draft.mode === "still" ? 1 : 0,
    leaving: false,
    draws: (counter + 1) >>> 0,
    feeling: atRest(kind.mood, now),
    state: kind.states.length > 0 ? kind.states[0]!.id : "",
    stateSince: now,
    former: kind.states.length > 0 ? kind.states[0]!.id : "",
    trick: null,
    warmth: COLD,
    hover: noHover(0),
    hang: null,
    chute: null,
    rope: null,
    emitters: kind.states.length > 0 && kind.states[0]!.emitter !== undefined ? [{ emitter: kind.states[0]!.emitter, since: now, until: null }] : [],
  };
  let at = 0;
  while (at < draft.streams.length && draft.streams[at]! < stream) at++;
  draft.actors.splice(at, 0, body);
  draft.kinds.splice(at, 0, kind);
  draft.streams.splice(at, 0, stream);
}

/** 📣️ Every wanted species that is not on stage arrives, in `menagerie.species` order — as long as the company on stage, leavers included, is smaller than the wanted one: a newcomer waits until whoever it replaces is gone, so a stage never holds more actors than were summoned. */
export function spawn(menagerie: Menagerie, draft: Draft): void {
  if (draft.perches.length === 0) return;
  for (let stream = 0; stream < menagerie.species.length && draft.actors.length < draft.wanted.length; stream++) {
    const kind = menagerie.species[stream]!;
    if (draft.wanted.includes(kind.id) && indexOf(draft.actors, kind.id) < 0) arrive(draft, kind, stream);
  }
}

/** 🌬️ The company spreads out over new ground (a survey brought a surface that was not there before). Every perch that holds more than one actor gives up all but the first of them (in `menagerie.species` order), as far as perches that hold nobody can carry them — each such perch takes one: whoever stands idle by itself on the shared perch and is not circled by the pointer leaves (on a still stage it is gone at once) and, still wanted, arrives anew where nobody stands. So a company that gathered on the only edge a screen offered — the footer line under an introduction — does not stay parked there when the next screen brings cards. A time of concentration leaves everybody where they are. */
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
      if (body.activity !== "idle" || body.partner !== null || circled(body.hover, now)) continue;
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
//#endregion 🔖️Arrival

//#region 🔖️Events
/** 🧭️ Whether the fixture `id` of `before` is gone from `after` or lies there with an edge more than {@link FIXTURE_SLACK} pixels from where it was. */
function astray(before: readonly Fixture[], after: readonly Fixture[], id: string): boolean {
  const was = before.find((fixture) => fixture.id === id);
  const now = after.find((fixture) => fixture.id === id);
  if (was === undefined || now === undefined) return true;
  return Math.abs(now.x - was.x) > FIXTURE_SLACK || Math.abs(now.y - was.y) > FIXTURE_SLACK || Math.abs(now.x + now.width - was.x - was.width) > FIXTURE_SLACK || Math.abs(now.y + now.height - was.y - was.height) > FIXTURE_SLACK;
}

/** 🗺️ The stage was measured anew: perches are cut again, grounded actors ride their surfaces, fall or — when the survey brought new ground — are gone with ground that vanished, and whoever waits for a perch arrives; over new ground the company then spreads out ({@link spread}), and on a still stage whoever it moved arrives at once. The walls and the fixtures of the survey are kept as they come; a prank whose fixture the survey no longer has, or has elsewhere ({@link astray}), ends quietly (`unlifted`). */
export function survey(menagerie: Menagerie, draft: Draft, event: Surveyed): void {
  const fixtures = draft.fixtures;
  const lift = draft.lift;
  if (lift !== null && astray(fixtures, event.fixtures, lift.fixture)) unlifted(draft, draft.tick);
  const before = draft.surfaces;
  const walls = draft.walls;
  const perches = draft.perches;
  const pitches = draft.pitches;
  const keepouts = draft.keepouts;
  draft.width = event.width;
  draft.height = event.height;
  draft.surfaces = event.surfaces;
  draft.keepouts = event.keepouts;
  draft.walls = event.walls;
  draft.fixtures = event.fixtures;
  const uprooted = widened(draft, before);
  measure(menagerie, draft);
  ride(menagerie, draft, before, walls, uprooted, !alike(perches, draft.perches) || !alike(pitches, draft.pitches) || !alike(keepouts, draft.keepouts));
  spawn(menagerie, draft);
  if (!uprooted) return;
  spread(draft);
  spawn(menagerie, draft);
}

/** 🎟️ The species that belong on stage changed: whoever is no longer wanted leaves (on a still stage it is gone at once), a leaver that is wanted again stays when it still stands on a perch (one that was crowded out goes on fading and arrives anew), perches are cut for the new company and the newly wanted arrive. */
export function summon(menagerie: Menagerie, draft: Draft, wanted: readonly Slug[]): void {
  const now = draft.tick;
  draft.wanted = wanted;
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    const stays = wanted.includes(body.species);
    if (stays && body.leaving && body.footing === "perch") {
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
  const perches = draft.perches;
  const pitches = draft.pitches;
  measure(menagerie, draft);
  ride(menagerie, draft, draft.surfaces, draft.walls, false, !alike(perches, draft.perches) || !alike(pitches, draft.pitches));
  spawn(menagerie, draft);
}

/** 🧊️ The stage turns still: the prank with the page is over (`unlifted`; a pusher the learner threw off is let off its sheepishness), leavers and whoever does not stand on a perch — in the air, under a parachute, in the hand, on a head, on a wall, a ladder or a rope — are gone (the wanted among them arrive anew at once, on a perch with room), every trip with gear is given up where it is (the ladders that stand stay as they are), the press is let go, and everyone else is idle at rest, whole, without a partner, with centred pupils, calm (the resting mood of its species at rest), cold and with no gesture under way. */
function freeze(draft: Draft, now: Ticks): void {
  unlifted(draft, now);
  draft.lift = null;
  draft.press = IDLE;
  draft.touched = null;
  draft.trail = [];
  draft.origin = null;
  for (let index = 0; index < draft.actors.length; ) {
    const body = draft.actors[index]!;
    if (body.leaving || body.footing !== "perch") {
      remove(draft, index);
      continue;
    }
    halt(draft, index, now);
    shift(draft, index, "idle", now);
    body.faced = now;
    body.until = now;
    body.clip = null;
    body.partner = null;
    body.vx = 0;
    body.vy = 0;
    body.goal = body.x;
    body.gaze = { x: 0, y: 0, vx: 0, vy: 0 };
    body.feeling = atRest(draft.kinds[index]!.mood, now);
    body.warmth = COLD;
    body.hover = noHover(now);
    body.opacity = 1;
    index++;
  }
}

/** 🎚️ The liveliness changed. Still freezes the stage; leaving still gives every actor a fresh idle dwell, idle clip and blink; a mode that allows fewer movers lets the walkers beyond its limit come to rest (hops in flight and trips with gear that left their perch count first; an actor still on its perch on its way to its gear counts as a walker and gives its trip up, {@link halt}). */
export function tune(menagerie: Menagerie, draft: Draft, mode: PetMode): void {
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
  for (const body of draft.actors) if (!body.leaving && (body.activity === "hop" || (body.footing !== "perch" && tripOf(draft.trips, body.species) !== null))) movers++;
  for (let index = 0; index < draft.actors.length; index++) {
    const body = draft.actors[index]!;
    const setting = body.footing === "perch" && tripOf(draft.trips, body.species) !== null;
    if (body.leaving || (body.activity !== "walk" && !setting)) continue;
    movers++;
    if (movers <= MODE_LIMITS[mode].movers) continue;
    halt(draft, index, now);
    release(draft, index, now);
    settle(draft, index, now);
  }
}
//#endregion 🔖️Events
