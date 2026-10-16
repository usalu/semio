/** 🧳️ Locomotion, trips: setting out on a trip with gear, embarking, travelling leg by leg, alighting, resting, clinging and halting where the actor is.
 *
 * @see ../🪜️outings/🟦️.ts — the outings a trip is made of
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { TICKS_PER_SECOND, type Claim, type Extent, type Foothold, type Ladder, type Perch, type Pitch, type Species, type Ticks, type Trip } from "../../../🧬️schema/🟦️.ts";
import { randomWords, unitOf } from "../../🎲️randomness/🟦️.ts";
import { claimClear, claimOf, released } from "../../🚧️clearance/🟦️.ts";
import { bodiesOf } from "../../📏️spacing/🟦️.ts";
import { actorKey, clipAt, feel, release, settle, shift, turn, type Draft } from "../../📝️draft/🟦️.ts";
import { dwellOf } from "../../🧠️behavior/🟦️.ts";
import { GRIP_BUDGET, GRIP_HANG, HOOK_RETURN, HOOK_SPEED, LADDER_IDLE, LADDER_LIFE, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_REST, chainOf, footOf, hookTicks, rimOf, routeOf } from "../../🧗️climbing/🟦️.ts";
import { CLAIM_SPAN, REST_RETRY, EXPLORE_SHARE, MAX_LADDERS } from "../📐️constants/🟦️.ts";
import { dropped, unfoot, unpacked, freed } from "../⚓️footing/🟦️.ts";
import { plunge } from "../🏃️motion/🟦️.ts";
import { tripOf, type Outing, type Venture, effortOf, gripNow, scaling, waysOff, reserveOf, lasting, bodyAt, spaced, ladderWays, slideOuting, outingsTo } from "../🪜️outings/🟦️.ts";

/** 🌉️ Whether an actor of `kind` standing at `x` on `from` can get to `to` with its gear at tick `now`, in one leg or by one perch on the way (`routeOf` over the ladders that stand free, a whole grip), as {@link outingsTo} would plan it. */
export function reaches(draft: Draft, kind: Species, x: number, from: Perch, to: Perch, now: Ticks): boolean {
  const standing: Ladder[] = [];
  for (const ladder of draft.ladders) if (ladder.rider === null && ladder.since <= now && ladder.until > now) standing.push(ladder);
  if (routeOf(x, from, to, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) !== null) return true;
  for (const between of draft.perches) {
    if (between === from || between === to || routeOf(x, from, between, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) === null) continue;
    if (routeOf((between.x0 + between.x1) / 2, between, to, kind.gear, kind.size, GRIP_BUDGET, draft.pitches, standing, draft.keepouts) !== null) return true;
  }
  return false;
}

/** 🚩️ An actor sets out at tick `now` on the first outing it can take to one of `ventures`: from the venture the unit `pick` points at, round and round, every outing in the order {@link outingsTo} prefers it (with the units `rest` = `[place, height, miss]`), the first one that is clear ({@link embark}). `false` when none is. B5 sends a pusher to its station this way (a `wall` venture with the height of the station). */
export function setOut(draft: Draft, index: number, ventures: readonly Venture[], pick: number, rest: readonly number[], now: Ticks): boolean {
  const count = ventures.length;
  if (count === 0) return false;
  const first = Math.min(Math.floor(pick * count), count - 1);
  for (let turn = 0; turn < count; turn++) {
    for (const outing of outingsTo(draft, index, ventures[(first + turn) % count]!, rest, now)) if (embark(draft, index, outing, now)) return true;
  }
  return false;
}

/** 📦️ Whether the body of an actor of `kind` with its feet at a foothold lies wholly inside the stage. */
export function inside(draft: Draft, kind: Species, step: Foothold): boolean {
  const half = kind.size.width / 2;
  return step.x - half >= 0 && step.x + half <= draft.width && step.y - kind.size.height >= 0 && step.y <= draft.height;
}

/** 📜️ The claim of a trip: the body at every foothold ({@link bodyAt}), `CLAIM_SPAN` ticks to a slice, resting on the last one — nowhere when it lets go in the air there. */
function tripClaim(draft: Draft, index: number, trip: Trip): Claim {
  const kind = draft.kinds[index]!;
  const extents: Extent[] = [];
  for (const step of trip.steps) extents.push(bodyAt(kind, trip, step));
  return claimOf(trip.owner, trip.from, extents, CLAIM_SPAN, trip.ending === "air" ? null : extents[extents.length - 1]!);
}

/** 👟️ The goal of a walk of a trip that begins with the foothold `at`: where the run of walking or carrying footholds on a perch that it begins ends. */
function walkEnd(trip: Trip, at: number): number {
  let goal = trip.steps[at]!.x;
  for (let next = at; next < trip.steps.length; next++) {
    const step = trip.steps[next]!;
    if (step.footing !== "perch" || (step.activity !== "walk" && step.activity !== "carry")) break;
    goal = step.x;
  }
  return goal;
}

/** ⛵️ An actor takes an outing at tick `now` — only when its body stays inside the stage at every foothold, the ladder it raises is the stage's second at most and its own first, the ladder it climbs stands, is free (or its own already) and stands `LADDER_IDLE` beyond the trip, and the claim of the trip is clear (`claimClear`). It lets go of its partner; on a wall it sets out with what its rest left of its grip; the claim and the trip are on the table; the ladder it raises stands up when the raise is done and is taken away `LADDER_IDLE` after the trip, `LADDER_LIFE` after it stood at the latest; the ladder it climbs is its own until it gets off and stands `LADDER_IDLE` longer (`LADDER_LIFE` at most); its rope is out (fired when the aim is done); it faces the way of its first foothold and walks for the end of the walk it begins with ({@link walkEnd}); its time is up when the trip ends. `false` when it does not take it. */
export function embark(draft: Draft, index: number, outing: Outing, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const trip = outing.trip;
  const owner = body.species;
  for (const step of trip.steps) if (!inside(draft, kind, step)) return false;
  const end = trip.from + trip.steps.length - 1;
  if (outing.raise !== null) {
    if (draft.ladders.length >= MAX_LADDERS || draft.ladders.some((ladder) => ladder.owner === owner)) return false;
  } else if (trip.ladder !== null) {
    const ladder = draft.ladders.find((entry) => entry.owner === trip.ladder);
    if (ladder === undefined || (ladder.rider !== null && ladder.rider !== owner) || ladder.since > now || ladder.since + LADDER_LIFE < end + LADDER_IDLE) return false;
  }
  const claim = tripClaim(draft, index, trip);
  if (!claimClear(claim, bodiesOf(draft.actors, draft.kinds), draft.claims)) return false;
  release(draft, index, now);
  if (body.footing === "wall") body.grip = gripNow(body, now);
  const raise = outing.raise;
  if (raise !== null) draft.ladders = [...draft.ladders, { owner, wall: raise.wall, surface: raise.surface, side: raise.side, foot: raise.foot, top: raise.top, since: outing.stands, until: Math.min(outing.stands + LADDER_LIFE, end + LADDER_IDLE), rider: owner }];
  else if (trip.ladder !== null) draft.ladders = draft.ladders.map((ladder) => (ladder.owner === trip.ladder ? { owner: ladder.owner, wall: ladder.wall, surface: ladder.surface, side: ladder.side, foot: ladder.foot, top: ladder.top, since: ladder.since, until: Math.min(ladder.since + LADDER_LIFE, Math.max(ladder.until, end + LADDER_IDLE)), rider: owner } : ladder));
  draft.claims = [...released(draft.claims, owner), claim];
  draft.courses = dropped(draft.courses, owner);
  draft.trips = [...unpacked(draft.trips, owner), trip];
  body.rope = outing.rope;
  body.until = end;
  const first = trip.steps[0]!;
  body.goal = first.footing === "perch" && (first.activity === "walk" || first.activity === "carry") ? walkEnd(trip, 0) : body.x;
  turn(body, first.facing, now);
  return true;
}

/** 🚂️ One tick of an actor on a trip at tick `now`: it takes its foothold of the tick — where its feet are and how fast they moved, what carries it (its perch, which may be a perch on the way between two legs, a wall, a ladder or a rope), the pitch its hands hold, the grip it has left (every foothold on a wall costs its effort), the way it faces (it turns round where a foothold faces the other way), what it does (a walk goes for where its run of walking footholds ends; it feels the miss of its hook when it shrugs, `missed`) —; on the haul its rope runs from the hook to its hands, a missed rope is in again once the hook is back and a rope it climbed once it stands on a perch again; at its last foothold it arrives ({@link alight}). An actor whose activity something else changed since the last tick (a click, a trick, a mode that stops walkers) gives its trip up there ({@link halt}). `false` when the actor is gone. */
export function travel(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const trip = tripOf(draft.trips, body.species);
  if (trip === null) return true;
  const at = now - trip.from;
  if (at < 0) return true;
  const last = trip.steps.length - 1;
  const was = at > 0 ? trip.steps[(at > last ? last : at) - 1]! : null;
  if (was !== null && body.activity !== was.activity) {
    halt(draft, index, now);
    return true;
  }
  const step = trip.steps[at > last ? last : at]!;
  body.vx = (step.x - body.x) * TICKS_PER_SECOND;
  body.vy = (step.y - body.y) * TICKS_PER_SECOND;
  body.x = step.x;
  body.y = step.y;
  body.footing = step.footing;
  body.perch = step.perch;
  body.pitch = step.hold >= 0 ? trip.pitches[step.hold]! : null;
  body.grip = Math.max(body.grip - effortOf(step), 0);
  if (step.activity !== body.activity) {
    shift(draft, index, step.activity, now);
    body.clip = clipAt(kind, step.activity, 0);
    if (step.activity === "walk" || step.activity === "carry") body.goal = walkEnd(trip, at > last ? last : at);
    if (step.activity === "shrug") feel(draft, index, "missed", now);
  }
  if (step.facing !== body.facing) turn(body, step.facing, now);
  const rope = body.rope;
  if (rope !== null && !rope.caught && now >= rope.since + hookTicks(rope.shot.muzzle, rope.shot.hook, HOOK_SPEED) + hookTicks(rope.shot.hook, rope.shot.muzzle, HOOK_RETURN)) body.rope = null;
  else if (rope !== null && rope.caught && now > rope.since && step.footing === "perch" && step.activity !== "aim") body.rope = null;
  else if (rope !== null && step.footing === "rope") {
    const hand = { x: step.x + rope.shot.facing * MUZZLE_FORWARD * kind.size.width, y: step.y - MUZZLE_HEIGHT * kind.size.height };
    const dx = rope.shot.hook.x - hand.x;
    const dy = rope.shot.hook.y - hand.y;
    body.rope = { shot: rope.shot, since: rope.since, length: Math.sqrt(dx * dx + dy * dy), hand, before: rope.hand, caught: rope.caught };
  }
  if (at < last) return true;
  return alight(draft, index, trip, now);
}

/** 🏁️ An actor comes to the end of its trip at tick `now`: the claim and the trip are done, the ladder it climbed is free and its rope is in; letting go in the air it falls ({@link plunge}); on a wall it rests there with the grip it has left ({@link rest}, for a dwell of `idle`); on a perch it stands — whole of grip — and comes to rest: after a miss for `ROPE_REST` at least, after any other way with its gear proud of it (`climbed`). `false` when the actor is gone. */
function alight(draft: Draft, index: number, trip: Trip, now: Ticks): boolean {
  const body = draft.actors[index]!;
  draft.claims = released(draft.claims, body.species);
  draft.trips = unpacked(draft.trips, body.species);
  freed(draft, body.species, now);
  body.rope = null;
  if (trip.ending === "air") {
    unfoot(draft, index, body.vx, body.vy, "fall", now);
    return plunge(draft, index, now);
  }
  if (trip.ending === "wall") {
    body.footing = "wall";
    body.perch = null;
    body.grip = trip.grip;
    const words = randomWords(actorKey(draft, index), 1);
    rest(draft, index, dwellOf("idle", draft.mode, unitOf(words[0]!)), now);
    return true;
  }
  body.footing = "perch";
  body.perch = trip.landing;
  body.pitch = null;
  body.grip = GRIP_BUDGET;
  settle(draft, index, now);
  if (trip.steps[trip.steps.length - 1]!.activity === "shrug") body.until = Math.max(body.until, now + ROPE_REST);
  else if (trip.steps.some((step) => step.footing !== "perch")) feel(draft, index, "climbed", now);
  return true;
}

/** 🦥️ An actor holds on to its wall or its ladder at tick `now` and rests there for `span` ticks — on a wall no longer than what is left of its grip lasts beside the reserve for the cheapest way off its line (`GRIP_HANG` per tick) —, in the pose of its climb (`climb`, paused: the clip turns with the distance climbed), with the grip it has now; at least a tick. Once its time is up it moves on by itself ({@link cling}). B5 keeps a pusher at its station on a wall this way and then sets it to `push` for the same time. */
export function rest(draft: Draft, index: number, span: Ticks, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  shift(draft, index, "climb", now);
  body.clip = clipAt(kind, "climb", 0);
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  let until = now + span;
  const pitch = body.pitch;
  if (body.footing === "wall" && pitch !== null) {
    const chain = chainOf(pitch, draft.pitches, kind.size);
    const spare = body.grip - reserveOf(draft, index, chain, chain.indexOf(pitch), body.x, body.y, now);
    until = Math.min(until, now + Math.floor(spare / GRIP_HANG));
  }
  body.until = Math.max(until, now + 1);
}

/** 🏞️ A spot on its wall line an actor that clings to `chain[start]` with `grip` left climbs on to (the units `place` and `height` draw the pitch of the line and the height on it), when what is left of its grip there lasts for a rest and the cheapest way off ({@link lasting}) and it keeps its spacing there ({@link spaced}); `null` otherwise. */
function explored(draft: Draft, index: number, chain: readonly Pitch[], start: number, grip: number, place: number, height: number, now: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const size = draft.kinds[index]!.size;
  const at = Math.min(Math.floor(place * chain.length), chain.length - 1);
  const y = rimOf(chain[at]!, size) + (footOf(chain[at]!, size) - rimOf(chain[at]!, size)) * height;
  const outing = scaling(draft, index, [], chain, start, "cling", body.x, body.y, at, y, false, null, grip, now);
  return outing !== null && lasting(draft, index, outing, now) && spaced(draft, index, outing) ? outing : null;
}

/** ➰️ An actor whose rest on a wall or a ladder is over moves on at tick `now`. On a ladder: up or down, the nearer first ({@link ladderWays}); while neither is clear it holds on for `REST_RETRY` more ticks and the ladder stands `LADDER_IDLE` beyond that — but no longer than `LADDER_LIFE`: then it lets go with a fright. On a wall, with what is left of its grip: in a lively stage outside a time of concentration, one time in two (`EXPLORE_SHARE`, an actor draw), to another spot of its line ({@link explored}); else, or when that is not clear, the cheapest way off its line that is clear ({@link waysOff}); while none is clear it holds on for `REST_RETRY` more ticks as long as its grip lasts for that and the cheapest way off; then it slides down ({@link slideOuting}); when even that is not clear it holds on while it still can and at last lets go with a fright (`startled`). `false` when the actor is gone. */
function onward(draft: Draft, index: number, ladder: Ladder | null, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (ladder !== null) {
    for (const way of ladderWays(draft, index, ladder, now)) if (embark(draft, index, way, now)) return true;
    if (ladder.since + LADDER_LIFE <= now + REST_RETRY) {
      feel(draft, index, "startled", now);
      unfoot(draft, index, 0, 0, "fall", now);
      return plunge(draft, index, now);
    }
    draft.ladders = draft.ladders.map((entry) => (entry.owner === ladder.owner ? { owner: entry.owner, wall: entry.wall, surface: entry.surface, side: entry.side, foot: entry.foot, top: entry.top, since: entry.since, until: Math.min(entry.since + LADDER_LIFE, Math.max(entry.until, now + REST_RETRY + LADDER_IDLE)), rider: entry.rider } : entry));
    body.until = now + REST_RETRY;
    return true;
  }
  const pitch = body.pitch!;
  const grip = gripNow(body, now);
  const chain = chainOf(pitch, draft.pitches, kind.size);
  const start = chain.indexOf(pitch);
  const ways = waysOff(draft, index, chain, start, body.x, body.y, grip, now);
  const tries: Outing[] = [];
  if (draft.mode === "lively" && !draft.quiet) {
    const units = randomWords(actorKey(draft, index), 3).map(unitOf);
    if (units[0]! < EXPLORE_SHARE) {
      const spot = explored(draft, index, chain, start, grip, units[1]!, units[2]!, now);
      if (spot !== null) tries.push(spot);
    }
  }
  for (const way of ways) tries.push(way);
  for (const outing of tries) if (embark(draft, index, outing, now)) return true;
  const reserve = ways.length === 0 ? 0 : grip - ways[0]!.trip.grip;
  if (ways.length > 0 && grip - GRIP_HANG * REST_RETRY >= reserve) {
    body.until = now + REST_RETRY;
    return true;
  }
  const slide = slideOuting(draft, index, grip, now);
  if (slide !== null && embark(draft, index, slide, now)) return true;
  if (grip - GRIP_HANG * REST_RETRY > 0) {
    body.until = now + REST_RETRY;
    return true;
  }
  feel(draft, index, "startled", now);
  unfoot(draft, index, 0, 0, "fall", now);
  return plunge(draft, index, now);
}

/** 🐨️ One tick of an actor that holds on to a wall or a ladder without a trip: once its rest is over it moves on ({@link onward}); one that no longer has the wall (`pitch` lost) or the ladder it held lets go and falls. `false` when the actor is gone. */
export function cling(draft: Draft, index: number, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const ladder = body.footing === "ladder" ? (draft.ladders.find((entry) => entry.rider === body.species) ?? null) : null;
  if ((body.footing === "wall" && body.pitch === null) || (body.footing === "ladder" && ladder === null)) {
    unfoot(draft, index, 0, 0, "fall", now);
    return plunge(draft, index, now);
  }
  if (now < body.until) return true;
  return onward(draft, index, ladder, now);
}

/** 🛑️ An actor gives its trip up at tick `now` — the stage moved under it, the learner's hand or another actor's plan took its place, or something set it off on another activity: its claim is released. On its rope it lets go and falls; on a wall or a ladder it holds on where it is, with the grip its trip has left it, and looks for a way on at the next tick ({@link rest}) — a ladder it left for the wall is free again —; on its perch it stays as it is, its rope in and the ladder it was going to climb or raise free. */
export function halt(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (tripOf(draft.trips, body.species) === null) return;
  draft.claims = released(draft.claims, body.species);
  draft.trips = unpacked(draft.trips, body.species);
  if (body.footing === "rope") {
    unfoot(draft, index, body.vx, body.vy, "fall", now);
    return;
  }
  body.rope = null;
  if (body.footing === "ladder") {
    rest(draft, index, 1, now);
    return;
  }
  freed(draft, body.species, now);
  if (body.footing === "wall") rest(draft, index, 1, now);
}
