/** 🪄️ Locomotion, mischief: posting a pet to a spot, popping it back, pushing and unpushing it and what is left unlifted.
 *
 * @see ../🧳️trips/🟦️.ts — the trips a post starts
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { type Foothold, type Ticks, type Trip } from "../../../🧬️schema/🟦️.ts";
import { freeAmong, grown, meets } from "../../🚧️clearance/🟦️.ts";
import { COMFORT_GAP, bodiesOf, extentAt, obstaclesFor, wallLeanOf } from "../../📏️spacing/🟦️.ts";
import { clipAt, hoverOf, indexOf, release, settle, shift, turn, type Draft } from "../../📝️draft/🟦️.ts";
import { GRIP_HANG, clingOf } from "../../🧗️climbing/🟦️.ts";
import { LIFT_TICKS } from "../../🪄️mischief/🟦️.ts";
import type { Post } from "../../🗓️schedule/🟦️.ts";
import { POST_UNITS } from "../📐️constants/🟦️.ts";
import { dusted, ungear } from "../⚓️footing/🟦️.ts";
import { tripOf, type Outing, gripNow, standOf, walked, outingsTo } from "../🪜️outings/🟦️.ts";
import { inside, embark, rest } from "../🧳️trips/🟦️.ts";

//#region 🔖️Mischief
/** 🧘️ Whether an outing that ends holding on to a wall leaves its actor the grip to hang there still for `span` ticks (`GRIP_HANG` per tick). What is left afterwards may not last for the way back up: a pet that played spent its grip, and slides down its wall when it is over. */
function holding(outing: Outing, span: Ticks): boolean {
  return outing.trip.grip >= GRIP_HANG * span;
}

/** 📬️ An actor that stands on a perch sets out at tick `now` for its post, to work there for `span` ticks after it arrives: to a post on a wall on the first clear outing {@link outingsTo} offers (over the rim from the perch on top, up from a perch beside the wall or from the exit of its own ladder raised against it, by a perch on the way when nothing reaches it from where it stands; lunges included) that leaves it the grip for that ({@link holding}); to a post on its perch on a walk along it — a trip, claimed like every other, that ends standing there; where that walk is not clear it does not go, since it never passes another on its perch. With `slip`, where no way to a post elsewhere is clear — the post lies out of reach of its gear — it slips over in a puff ({@link popTo}), the last resort the stage tries only when no pet that could play can get to its post with its gear. The tick it stands at its post — the end of its trip, or `now` when it stands there already or slipped over —, or `null` when it cannot get there at all (then nothing changes). */
export function postTo(draft: Draft, index: number, post: Post, side: 1 | -1, span: Ticks, slip: boolean, now: Ticks): Ticks | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (post.kind === "wall") {
    for (const outing of outingsTo(draft, index, { kind: "wall", pitch: post.pitch, y: post.y }, POST_UNITS, now)) if (holding(outing, span) && embark(draft, index, outing, now)) return outing.trip.from + outing.trip.steps.length - 1;
  } else if (body.footing === "perch" && body.perch === post.perch.surface) {
    const steps = walked(standOf(body, post.perch), kind, post.x, "walk");
    if (steps !== null && steps.length === 0) return now;
    const trip: Trip | null = steps === null ? null : { owner: body.species, from: now + 1, steps, pitches: [], ladder: null, ending: "perch", landing: post.perch.surface, grip: body.grip };
    return trip !== null && embark(draft, index, { trip, raise: null, stands: 0, rope: null }, now) ? trip.from + trip.steps.length - 1 : null;
  }
  return slip && popTo(draft, index, post, side, span, now) ? now : null;
}

/** 💫️ An actor slips over to its post at tick `now` when its gear has no way there: it is gone from where it stood in a puff of dust ({@link dusted}: not a last resort, but a poof the stage counts) and stands at its post at once, see-through — it fades in over the next ticks like an arrival —, facing `side`; on a wall it holds on whole of grip, resting in the pose of its climb for `span` ticks, on a perch it comes to rest. Only when its body there lies inside the stage, a comfortable gap from every other body and off every corridor and landing spot somebody claimed. `false` when it cannot (then nothing changes). */
function popTo(draft: Draft, index: number, post: Post, side: 1 | -1, span: Ticks, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const feet: Foothold = post.kind === "wall" ? { x: clingOf(post.pitch, kind.size), y: post.y, footing: "wall", perch: null, activity: "climb", facing: side, hold: 0 } : { x: post.x, y: post.perch.y - hoverOf(kind), footing: "perch", perch: post.perch.surface, activity: "idle", facing: side, hold: -1 };
  const extent = extentAt(body.species, kind, feet, wallLeanOf(kind, feet.footing, post.kind === "wall" ? post.pitch : null, feet.x), null);
  const room = grown(extent, COMFORT_GAP);
  if (!inside(draft, kind, feet) || !freeAmong(extent, obstaclesFor(draft, index, now)) || bodiesOf(draft.actors, draft.kinds).some((other) => other.owner !== body.species && meets(room, other.extent))) return false;
  dusted(draft, index, now);
  release(draft, index, now);
  ungear(draft, index, now);
  body.x = feet.x;
  body.y = feet.y;
  body.goal = feet.x;
  body.vx = 0;
  body.vy = 0;
  body.tilt = 0;
  body.opacity = 0;
  body.facing = side;
  body.faced = now;
  body.host = null;
  if (post.kind === "wall") {
    body.footing = "wall";
    body.perch = null;
    body.pitch = post.pitch;
    rest(draft, index, span, now);
    body.until = now + span;
    return true;
  }
  body.footing = "perch";
  body.perch = post.perch.surface;
  settle(draft, index, now);
  return true;
}

/** 🤜️ An actor at its post begins to push at tick `now`, until `until`, shoving the way `side` says — when it is ready: idle on its perch, or holding on to its wall at rest in the pose of its climb with the grip to hang there until then; not on a trip, not leaving, without a partner. It faces the way it shoves, stands still and plays its push clip. `false` when it is not ready (then nothing changes). */
export function pushAt(draft: Draft, index: number, side: 1 | -1, until: Ticks, now: Ticks): boolean {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (body.leaving || body.partner !== null || tripOf(draft.trips, body.species) !== null) return false;
  const clinging = body.footing === "wall" && body.activity === "climb" && body.pitch !== null;
  if (!clinging && !(body.footing === "perch" && body.activity === "idle")) return false;
  if (clinging) {
    const grip = gripNow(body, now);
    if (grip < GRIP_HANG * (until - now)) return false;
    body.grip = grip;
  }
  shift(draft, index, "push", now);
  body.clip = clipAt(kind, "push", 0);
  body.until = until;
  body.goal = body.x;
  body.vx = 0;
  body.vy = 0;
  turn(body, side, now);
  return true;
}

/** 🫷️ An actor stops pushing at tick `now` without being thrown off: on its wall it holds on in the pose of its climb and moves on from the next tick ({@link rest}), on its perch it comes to rest. An actor that does not push stays as it is. */
export function unpush(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  if (body.activity !== "push") return;
  if (body.footing === "wall" && body.pitch !== null) rest(draft, index, 1, now);
  else settle(draft, index, now);
}

/** 🧯️ The prank of the stage ends quietly at tick `now` — its element went away or moved, the learner no longer permits mischief, the stage turns still —: a pet on its way to its post goes on to where it was going, a pusher stops ({@link unpush}), the copy is gone and the mode's cooldown counts from now (`rested`). A pusher the learner threw off keeps its prank until it is down again. */
export function unlifted(draft: Draft, now: Ticks): void {
  const lift = draft.lift;
  if (lift === null || now - lift.since > LIFT_TICKS) return;
  draft.lift = null;
  draft.rested = now;
  const index = indexOf(draft.actors, lift.pusher);
  if (index >= 0) unpush(draft, index, now);
}
//#endregion 🔖️Mischief
