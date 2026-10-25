/** ✋️ Locomotion, hand: a pet lifted into the learner's hand, held, let go, given back and tossed.
 *
 * @see ../🧳️trips/🟦️.ts — the halt of a trip a lift interrupts
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { TICKS_PER_SECOND, type Claim, type Perch, type Point, type Ticks, type Waypoint } from "../../../🧬️schema/🟦️.ts";
import { MARGIN, PUSHES, claimClear, freeAmong, obstaclesOf, pushedOut, released, shifted, spotOn } from "../../🚧️clearance/🟦️.ts";
import { bodiesOf, extentAt, extentOf, obstaclesFor } from "../../📏️spacing/🟦️.ts";
import { clamp, smoothstep } from "../../📐️trigonometry/🟦️.ts";
import { clipAt, hoverOf, release, settle, shift, type Draft } from "../../📝️draft/🟦️.ts";
import { dwellOf } from "../../🧠️behavior/🟦️.ts";
import { hangOf, hangStep, leanOf } from "../../🪢️swing/🟦️.ts";
import { RETURN_SPEED, RETURN_LEAST, TOSS_TOP, TOSS_TICKS } from "../📐️constants/🟦️.ts";
import { dropped, unfoot, install, ungear } from "../⚓️footing/🟦️.ts";
import { planOf } from "../🛬️courses/🟦️.ts";
import { tripOf } from "../🪜️outings/🟦️.ts";
import { halt } from "../🧳️trips/🟦️.ts";

//#region 🔖️Hand
/** 🚦️ Whoever planned to pass or to come to rest where the body of an actor now is — counting on it to be gone by then — gives its plan up at tick `now`, because the hand took that actor out of its own plan: a flier plans anew in its turn, with the speed it has, from where it is, keeping its activity; a traveller with its gear gives its trip up ({@link halt}) and, on its perch, comes to rest. */
function yielded(draft: Draft, index: number, now: Ticks): void {
  const owner = draft.actors[index]!.species;
  const box = extentOf(draft.actors[index]!, draft.kinds[index]!);
  for (let other = 0; other < draft.actors.length; other++) {
    const flier = draft.actors[other]!;
    if (other === index) continue;
    const own: Claim[] = [];
    for (const claim of draft.claims) if (claim.owner === flier.species) own.push(claim);
    if (own.length === 0 || freeAmong(box, obstaclesOf(owner, [], own, now))) continue;
    if (tripOf(draft.trips, flier.species) === null) unfoot(draft, other, flier.vx, flier.vy, flier.activity, now);
    else {
      halt(draft, other, now);
      if (flier.footing === "perch") settle(draft, other, now);
    }
  }
}

/** 🎓️ The learner — or a deed of the learner (`toss`) — picks an actor up at tick `now`: it lets go of its partner, of its claim and course, of whatever carried it and whatever its gear held ({@link ungear}), and hangs by its scruff (`hangOf` with the species' `grip` as the rod) in the hand (footing `hand`, activity `hang`); where its feet stood when it was picked up from a perch is remembered (`origin`), and the pointer's trail starts anew. Whoever planned to pass where it hangs plans anew ({@link yielded}). */
export function lift(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  release(draft, index, now);
  draft.claims = released(draft.claims, body.species);
  draft.courses = dropped(draft.courses, body.species);
  ungear(draft, index, now);
  draft.origin = body.footing === "perch" ? { x: body.x, y: body.y } : null;
  draft.trail = [];
  if (body.activity !== "hang") shift(draft, index, "hang", now);
  body.clip = clipAt(kind, "hang", 0);
  body.footing = "hand";
  body.perch = null;
  body.host = null;
  body.chute = null;
  body.tilt = 0;
  body.hang = hangOf({ x: body.x, y: body.y }, kind.grip);
  body.vx = 0;
  body.vy = 0;
  body.goal = body.x;
  body.until = now + dwellOf("hang", draft.mode, 0);
  yielded(draft, index, now);
}

/** 🦧️ One tick of an actor in the hand at tick `now`: its grip follows `target` (held inside the stage) and its body swings below it (`hangStep`), tilted by `leanOf`; a body dragged into somebody or into a corridor or landing spot is pushed out of it (`pushedOut`, grip and feet together); when that fails, or would leave the stage (its drawing, or its upright box at its feet), it keeps its place and stops swinging. */
export function hold(draft: Draft, index: number, target: Point, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const hang = body.hang;
  if (hang === null) return;
  const next = hangStep(hang, { x: clamp(target.x, 0, draft.width), y: clamp(target.y, 0, draft.height) }, kind.grip);
  const tilt = leanOf(next.grip, next.bob, kind.grip);
  const box = extentAt(body.species, kind, next.bob, tilt, null);
  const push = pushedOut(box, obstaclesFor(draft, index, now), PUSHES);
  const moved = push === null ? null : shifted(box, push.x, push.y);
  const half = kind.size.width / 2;
  if (push === null || moved === null || moved.x0 + MARGIN < 0 || moved.x1 - MARGIN > draft.width || moved.y0 + MARGIN < 0 || moved.y1 - MARGIN > draft.height || next.bob.x + push.x - half < 0 || next.bob.x + push.x + half > draft.width || next.bob.y + push.y - kind.size.height < 0 || next.bob.y + push.y > draft.height) {
    body.hang = { grip: { x: hang.grip.x, y: hang.grip.y, vx: 0, vy: 0 }, bob: hang.bob, previous: hang.bob };
    return;
  }
  body.hang = { grip: { x: next.grip.x + push.x, y: next.grip.y + push.y, vx: next.grip.vx, vy: next.grip.vy }, bob: { x: next.bob.x + push.x, y: next.bob.y + push.y }, previous: { x: next.previous.x + push.x, y: next.previous.y + push.y } };
  body.x = next.bob.x + push.x;
  body.y = next.bob.y + push.y;
  body.tilt = tilt;
}

/** 🎈️ The hand lets an actor go at tick `now` with the velocity `velocity` (`releaseVelocity` of the pointer's trail, or nothing): it tumbles through the air, righting itself, and plans its way down in its turn — with air control towards a free spot, and its parachute where it would land hard. */
export function letGo(draft: Draft, index: number, velocity: Point, now: Ticks): void {
  draft.trail = [];
  unfoot(draft, index, velocity.x, velocity.y, "tumble", now);
}

/** ↩️ A pick-up is called off at tick `now` (Escape, a lost pointer, a hidden page): the actor glides back, straight and eased, to the free spot nearest to where it was picked up on that perch, at `RETURN_SPEED` and in no fewer than `RETURN_LEAST` ticks, righting itself on the way — on a corridor whose claim is clear; else, or when it was not picked up from a perch, it is simply let go. */
export function giveBack(draft: Draft, index: number, now: Ticks): void {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const origin = draft.origin;
  draft.origin = null;
  const hover = hoverOf(kind);
  let perch: Perch | null = null;
  if (origin !== null) for (const candidate of draft.perches) if (perch === null && candidate.y - hover === origin.y && candidate.x0 <= origin.x && origin.x <= candidate.x1) perch = candidate;
  const spot = origin === null || perch === null ? null : spotOn(perch, origin.x, extentAt(body.species, kind, origin, 0, null), kind.size.width / 2, obstaclesFor(draft, index, now));
  if (origin === null || perch === null || spot === null) {
    letGo(draft, index, { x: 0, y: 0 }, now);
    return;
  }
  const dx = spot - body.x;
  const dy = origin.y - body.y;
  const ticks = Math.max(Math.floor((Math.sqrt(dx * dx + dy * dy) * TICKS_PER_SECOND) / RETURN_SPEED) + 1, RETURN_LEAST);
  const steps: Waypoint[] = [];
  for (let tick = 1; tick <= ticks; tick++) {
    const share = smoothstep(tick / ticks);
    steps.push(tick === ticks ? { x: spot, y: origin.y, vx: 0, vy: 0, tilt: 0, canopy: null } : { x: body.x + dx * share, y: body.y + dy * share, vx: 0, vy: 0, tilt: body.tilt * (1 - share), canopy: null });
  }
  const plan = planOf(draft, index, { owner: body.species, from: now + 1, steps, ending: "perch", landing: perch.surface, touch: 0 });
  if (!claimClear(plan.claim, bodiesOf(draft.actors, draft.kinds), draft.claims)) {
    letGo(draft, index, { x: 0, y: 0 }, now);
    return;
  }
  draft.trail = [];
  release(draft, index, now);
  install(draft, index, plan, "glide", now);
}

/** 🎯️ Where the hand of a toss carries an actor at the top of the stage: above where it was picked up, its body `TOSS_TOP` px below the top edge. */
export function tossTarget(draft: Draft, index: number): Point {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  return { x: body.goal, y: TOSS_TOP + kind.size.height - kind.grip };
}

/** 🎾️ The learner tosses an actor with a deed at tick `now` (the keyboard's way to see a parachute): it is picked up like by the hand ({@link lift}), carried to the top of the stage ({@link tossTarget}) for `TOSS_TICKS` and let go there without a throw. */
export function toss(draft: Draft, index: number, now: Ticks): void {
  lift(draft, index, now);
  draft.origin = null;
  draft.actors[index]!.until = now + TOSS_TICKS;
}
//#endregion 🔖️Hand
