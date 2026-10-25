/** 🧗️ Locomotion, outings: every outing a pet with gear can take from a stand — the climb onto a wall, the ladder and the rope, the slide and the rests on the way — and the legs of a trip assembled from them.
 *
 * @see ../⚓️footing/🟦️.ts — the release of gear
 * @see ../🦀️.rs — the Rust twin
 * @see ../../🎪️stage/🟦️.ts — the façade of the stage and the normative order of a tick
 */
import { type Activity, type Extent, type Foothold, type Ladder, type Perch, type Pitch, type Rope, type Shot, type Slug, type Species, type Ticks, type Trip } from "../../../🧬️schema/🟦️.ts";
import { grown, meets } from "../../🚧️clearance/🟦️.ts";
import { landingOf, perchAt, strideTo } from "../../🏞️terrain/🟦️.ts";
import { COMFORT_GAP, bodiesOf, extentAt, wallLeanOf } from "../../📏️spacing/🟦️.ts";
import { clamp, smoothstep } from "../../📐️trigonometry/🟦️.ts";
import { TURN_TICKS, facingTo, type Body, type Draft } from "../../📝️draft/🟦️.ts";
import { GRIP_BUDGET, GRIP_CLIMB, GRIP_HANG, HOOK_RETURN, HOOK_SPEED, LADDER_DISMOUNT_TICKS, LADDER_MOUNT_TICKS, LADDER_RAISE_TICKS, ROPE_AIM_TICKS, ROPE_HOIST_TICKS, ROPE_MARGIN, ROPE_MISS_CHANCE, ROPE_SHRUG_TICKS, ROPE_TUG_TICKS, WALL_GRAB_TICKS, chainOf, clingOf, clings, footOf, gripFor, haulOf, haulStep, hoistPath, hookTicks, ladderAt, ladderExit, ladderLanding, ladderLength, ladderStep, ladderTo, landingFor, ledgeOf, missOf, rimFor, rimOf, routeOf, shotFor, sighted, slideStep, wallCost, wallPath, type LadderStand, type WallHold } from "../../🧗️climbing/🟦️.ts";
import { REEL_LEAST } from "../../🪢️swing/🟦️.ts";
import { LONGEST, REST_LEAST, REST_HALVINGS, MAX_LADDERS } from "../📐️constants/🟦️.ts";

//#region 🔖️Gear
/** 🎒️ Whether a species gets around with gear: it owns what it takes to climb, a ladder or a grappling gun, and it does not float — a floater changes its lanes instead. */
export function geared(kind: Species): boolean {
  return kind.locomotion.gait !== "float" && (kind.gear.includes("climb") || kind.gear.includes("ladder") || kind.gear.includes("grapple"));
}

/** 🗻️ The trip of an owner, or `null`. */
export function tripOf(trips: readonly Trip[], owner: Slug): Trip | null {
  for (const trip of trips) if (trip.owner === owner) return trip;
  return null;
}

/** 🪺️ Where on a perch the feet of a body `half` wide keep the whole body on it, `[x0 + half, x1 − half]`; `null` on a perch narrower than the body. */
function roomOn(perch: Perch, half: number): readonly [number, number] | null {
  const low = perch.x0 + half;
  const high = perch.x1 - half;
  return low <= high ? [low, high] : null;
}

/** ⛺️ A trip as planned, with what taking it sets up: the ladder it raises (`null`: none) and the tick that ladder stands up, and the grappling rope it fires (`null`: none). */
export type Outing = { readonly trip: Trip; readonly raise: LadderStand | null; readonly stands: Ticks; readonly rope: Rope | null };

/** 🧿️ Where an actor can set out for with its gear: a perch (wherever its way arrives on it) or a spot on a wall — a pitch and the height of the feet there, or, with `null`, a height the stage draws. */
export type Venture = { readonly kind: "perch"; readonly perch: Perch } | { readonly kind: "wall"; readonly pitch: Pitch; readonly y: number | null };

/** 💸️ The grip a foothold of a trip costs: `GRIP_HANG` sliding down a wall, `GRIP_CLIMB` for anything else on a wall, nothing off a wall. */
export function effortOf(step: Foothold): number {
  return step.footing !== "wall" ? 0 : step.activity === "slide" ? GRIP_HANG : GRIP_CLIMB;
}

/** 🔋️ What is left at tick `now` of the grip of an actor that rests on a wall: hanging still costs `GRIP_HANG` per tick since it began to rest (`since`), never below nothing. */
export function gripNow(body: Body, now: Ticks): number {
  return Math.max(body.grip - GRIP_HANG * (now - body.since), 0);
}

/** 🧩️ Where a leg of a trip sets out from: the perch an actor stands on, where its feet are there, the way it faces and the grip it has — the actor itself before its first leg, where the leg before put it before every other. */
export type Stand = { readonly perch: Perch; readonly x: number; readonly y: number; readonly facing: 1 | -1; readonly grip: number };

/** 🛂️ The stand of an actor on `perch` as it is: its feet, the way it faces and its grip (whole on a perch). */
export function standOf(body: Body, perch: Perch): Stand {
  return { perch, x: body.x, y: body.y, facing: body.facing, grip: body.grip };
}

/** 🧲️ The way an actor faces while it holds on to a pitch or climbs a ladder of the wall of `side`: towards the wall (a wall's `side` is the side of its element it bounds, so the wall is on the other side of the climber). */
function wallward(leaning: { readonly side: 1 | -1 }): 1 | -1 {
  return leaning.side > 0 ? -1 : 1;
}

/** 🚸️ The footholds of a walk along the perch of `stand` to `to`, doing `activity` (walking, or carrying its ladder) at the speed of `kind`, facing the way it goes; when that is against the way it faces it first stands for `TURN_TICKS` while its drawing turns round. `null` when it cannot walk there: no speed, or not within 4096 ticks. */
export function walked(stand: Stand, kind: Species, to: number, activity: Activity): Foothold[] | null {
  const steps: Foothold[] = [];
  if (to === stand.x) return steps;
  if (!(kind.locomotion.speed > 0)) return null;
  const facing = facingTo(to, stand.x);
  if (facing !== stand.facing) for (let tick = 0; tick < TURN_TICKS; tick++) steps.push({ x: stand.x, y: stand.y, footing: "perch", perch: stand.perch.surface, activity, facing, hold: -1 });
  let x = stand.x;
  for (let tick = 0; x !== to; tick++) {
    if (tick >= LONGEST) return null;
    x = strideTo(x, to, kind.locomotion.speed);
    steps.push({ x, y: stand.y, footing: "perch", perch: stand.perch.surface, activity, facing, hold: -1 });
  }
  return steps;
}

/** 🪵️ The footholds of a climb along the ladder `stand` from `travel` to `goal` (`ladderStep`, the feet at `ladderAt`), facing the wall it leans against, appended to `steps`. */
function rungs(stand: LadderStand, travel: number, goal: number, steps: Foothold[]): void {
  let along = travel;
  for (let tick = 0; along !== goal && tick < LONGEST; tick++) {
    along = ladderStep(along, goal, tick);
    const feet = ladderAt(stand, along);
    steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "climb", facing: wallward(stand), hold: -1 });
  }
}

/** 🧱️ An outing along the wall line `chain` whose first foothold comes at tick `from`: the footholds `lead` that bring the actor to where it takes hold of `chain[start]` (`entry`: a grab beside the wall from its feet at (`x`, `y`) — on a perch or at the exit of a ladder —; a hang over the rim from the ledge on top; or clinging there already with its feet at (`x`, `y`)), the way along the line to `goal` on `chain[end]` (`wallPath`: climbing and lunging, then mantling), facing the wall all along, and its end — over the rim onto the perch that crowns `chain[end]` (`mantle`), stepping off onto `foot` at its height and onto as much of it as the whole body needs (eased over `WALL_GRAB_TICKS`, still holding the wall), or resting on the wall. `null` when the grip it has when the way begins (`grip`) does not last for it (`wallCost`, and the step off), when the perch it ends on has no room for its body there, or when there is no way at all. */
export function scaling(draft: Draft, index: number, lead: readonly Foothold[], chain: readonly Pitch[], start: number, entry: "grab" | "hang" | "cling", x: number, y: number, end: number, goal: number, mantle: boolean, foot: Perch | null, grip: number, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const half = kind.size.width / 2;
  const path = wallPath(chain, start, entry, x, y, end, goal, mantle, kind.size);
  const steps: Foothold[] = [...lead];
  for (const clamber of path) steps.push({ x: clamber.x, y: clamber.y, footing: "wall", perch: null, activity: clamber.work === "mantle" ? "mantle" : "climb", facing: wallward(chain[clamber.hold]!), hold: clamber.hold });
  let ending: "perch" | "wall" = "wall";
  let landing = "";
  let cost = wallCost(path);
  if (mantle) {
    const top = rimFor(chain[end]!, draft.perches, kind.size);
    const room = top === null ? null : roomOn(top, half);
    const ledge = ledgeOf(chain[end]!, kind.size);
    if (top === null || room === null || ledge < room[0] || ledge > room[1]) return null;
    ending = "perch";
    landing = top.surface;
  } else if (foot !== null) {
    const room = roomOn(foot, half);
    if (room === null) return null;
    const off = clingOf(chain[end]!, kind.size);
    const to = clamp(off, room[0], room[1]);
    if (to !== off) {
      for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? to : off + (to - off) * smoothstep(tick / WALL_GRAB_TICKS), y: foot.y, footing: "wall", perch: null, activity: "climb", facing: wallward(chain[end]!), hold: end });
      cost = cost + WALL_GRAB_TICKS * GRIP_CLIMB;
    }
    ending = "perch";
    landing = foot.surface;
  }
  if (steps.length === 0 || cost > grip) return null;
  return { trip: { owner: body.species, from, steps, pitches: chain, ladder: null, ending, landing, grip: grip - cost }, raise: null, stands: 0, rope: null };
}

/** 🏔️ An outing of an actor at `stand` along the wall line `chain`, its first foothold at tick `from`: it walks along its perch to where it takes hold of `chain[start]` — beside the wall as near to `hold` as its whole body stays on the perch, or to the ledge on top for a hold over the rim — and goes on as {@link scaling} says, with the grip it has there. `null` when its body has no room where it must stand. */
function fromPerch(draft: Draft, index: number, stand: Stand, chain: readonly Pitch[], start: number, hold: WallHold, end: number, goal: number, mantle: boolean, foot: Perch | null, from: Ticks): Outing | null {
  const kind = draft.kinds[index]!;
  const room = roomOn(stand.perch, kind.size.width / 2);
  if (room === null) return null;
  const at = hold.over ? ledgeOf(chain[start]!, kind.size) : clamp(hold.x, room[0], room[1]);
  if (at < room[0] || at > room[1]) return null;
  const lead = walked(stand, kind, at, "walk");
  if (lead === null) return null;
  return scaling(draft, index, lead, chain, start, hold.over ? "hang" : "grab", at, stand.y, end, goal, mantle, foot, stand.grip, from);
}

/** 🏗️ The footholds that bring an actor at `stand` onto the ladder `ladder` and up to its exit (`ladderExit`), its first foothold at tick `from`: the walk to the foot — carrying its own ladder there and raising it over `LADDER_RAISE_TICKS` when it `raise`s it —, stepping onto it over `LADDER_MOUNT_TICKS` and the climb along its rails; with the tick the ladder it raises stands up (0 when it raises none). `null` when it cannot stand at the foot. */
function ladderLead(draft: Draft, index: number, stand: Stand, ladder: LadderStand, raise: boolean, from: Ticks): { readonly steps: Foothold[]; readonly stands: Ticks } | null {
  const kind = draft.kinds[index]!;
  const room = roomOn(stand.perch, kind.size.width / 2);
  if (room === null) return null;
  const at = clamp(ladder.foot.x, room[0], room[1]);
  const lead = walked(stand, kind, at, raise ? "carry" : "walk");
  if (lead === null) return null;
  const steps: Foothold[] = [...lead];
  const facing = wallward(ladder);
  let stands = 0;
  if (raise) {
    for (let tick = 0; tick < LADDER_RAISE_TICKS; tick++) steps.push({ x: at, y: stand.y, footing: "perch", perch: stand.perch.surface, activity: "carry", facing, hold: -1 });
    stands = from + steps.length;
  }
  for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? ladder.foot.x : at + (ladder.foot.x - at) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  rungs(ladder, 0, ladderExit(ladder, kind.size), steps);
  return { steps, stands };
}

/** 🪤️ An outing of an actor at `stand` that raises its own ladder against `chain[start]` to take hold of the wall from it (`lean`, of `ladderTo`), its first foothold at tick `from`: up the ladder ({@link ladderLead}), then a grab from its exit onto the wall and on along the line as {@link scaling} says. The ladder is its own, raised for this trip. `null` when there is no such way. */
function leanOuting(draft: Draft, index: number, stand: Stand, chain: readonly Pitch[], start: number, lean: LadderStand, end: number, goal: number, mantle: boolean, foot: Perch | null, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const up = ladderLead(draft, index, stand, lean, true, from);
  if (up === null) return null;
  const exit = ladderAt(lean, ladderExit(lean, kind.size));
  const outing = scaling(draft, index, up.steps, chain, start, "grab", exit.x, exit.y, end, goal, mantle, foot, stand.grip, from);
  if (outing === null) return null;
  const trip = outing.trip;
  return { trip: { owner: trip.owner, from: trip.from, steps: trip.steps, pitches: trip.pitches, ladder: body.species, ending: trip.ending, landing: trip.landing, grip: trip.grip }, raise: lean, stands: up.stands, rope: null };
}

/** 🛤️ The way of an actor that clings to `chain[start]` with its feet at (`x`, `y`) and `grip` left down the ladder `ladder` that leans against a pitch of its line, its first foothold at tick `from`: along the line to the height of the ladder's exit, over onto the ladder there (eased over `WALL_GRAB_TICKS`), down its rails and off onto as much of the perch at its foot as the whole body needs. `null` when the ladder leans on no pitch of the line, its exit is out of the hands' reach there, its foot stands on no perch with room, or the grip does not last for the way along the line. */
function ladderDown(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, grip: number, ladder: Ladder, from: Ticks): Outing | null {
  const kind = draft.kinds[index]!;
  const size = kind.size;
  let at = -1;
  for (let line = 0; line < chain.length && at < 0; line++) if (chain[line]!.wall === ladder.wall && chain[line]!.side === ladder.side && chain[line]!.y0 <= ladder.top.y && ladder.top.y <= chain[line]!.y1) at = line;
  const exit = ladderExit(ladder, size);
  const door = ladderAt(ladder, exit);
  if (at < 0 || !clings(chain[at]!, door.y, size)) return null;
  let ground: Perch | null = null;
  for (const perch of draft.perches) if (ground === null && perch.surface === ladder.surface && perch.y === ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) ground = perch;
  const below = ground === null ? null : roomOn(ground, size.width / 2);
  if (ground === null || below === null) return null;
  const along = scaling(draft, index, [], chain, start, "cling", x, y, at, door.y, false, null, grip, from);
  if (along === null) return null;
  const facing = wallward(ladder);
  const steps: Foothold[] = [...along.trip.steps];
  const off = clingOf(chain[at]!, size);
  for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? door.x : off + (door.x - off) * smoothstep(tick / WALL_GRAB_TICKS), y: door.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  rungs(ladder, exit, 0, steps);
  const foot = clamp(ladder.foot.x, below[0], below[1]);
  if (foot !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? foot : ladder.foot.x + (foot - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  return { trip: { owner: along.trip.owner, from, steps, pitches: chain, ladder: ladder.owner, ending: "perch", landing: ground.surface, grip: along.trip.grip }, raise: null, stands: 0, rope: null };
}

/** 🛗️ The ways an actor that clings to `chain[start]` with its feet at (`x`, `y`) and `grip` left gets off its wall line at tick `now`, the cheapest first (among equals: down before up, the nearer pitch first): down the line onto a perch that passes the foot of one of its pitches, up the line over the rim onto the perch that crowns one of them, or down a ladder that stands against one of them ({@link ladderDown}) — the way a pet that came up by a ladder goes back. */
export function waysOff(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, grip: number, now: Ticks): Outing[] {
  const kind = draft.kinds[index]!;
  const ways: Outing[] = [];
  for (let at = start; at < chain.length; at++) {
    for (const perch of draft.perches) {
      const hold = gripFor(perch, chain[at]!, kind.size);
      if (hold === null || hold.over) continue;
      const way = scaling(draft, index, [], chain, start, "cling", x, y, at, perch.y, false, perch, grip, now + 1);
      if (way !== null) ways.push(way);
    }
  }
  for (let at = start; at >= 0; at--) {
    if (rimFor(chain[at]!, draft.perches, kind.size) === null) continue;
    const way = scaling(draft, index, [], chain, start, "cling", x, y, at, rimOf(chain[at]!, kind.size), true, null, grip, now + 1);
    if (way !== null) ways.push(way);
  }
  for (const ladder of draft.ladders) {
    if (ladder.since > now || ladder.until <= now) continue;
    const way = ladderDown(draft, index, chain, start, x, y, grip, ladder, now + 1);
    if (way !== null) ways.push(way);
  }
  const sorted: Outing[] = [];
  for (const way of ways) {
    let at = sorted.length;
    while (at > 0 && sorted[at - 1]!.trip.grip < way.trip.grip) at--;
    sorted.splice(at, 0, way);
  }
  return sorted;
}

/** 🪫️ The grip the cheapest way off its wall line costs an actor that clings to `chain[start]` with its feet at (`x`, `y`); nothing when it has no way off — then it can only slide. */
export function reserveOf(draft: Draft, index: number, chain: readonly Pitch[], start: number, x: number, y: number, now: Ticks): number {
  const ways = waysOff(draft, index, chain, start, x, y, GRIP_BUDGET, now);
  return ways.length === 0 ? 0 : GRIP_BUDGET - ways[0]!.trip.grip;
}

/** ⚖️ Whether an outing that ends resting on a wall leaves its actor the grip for a rest of `REST_LEAST` ticks there and the cheapest way off the wall line of its last pitch after it. */
export function lasting(draft: Draft, index: number, outing: Outing, now: Ticks): boolean {
  const trip = outing.trip;
  const last = trip.steps[trip.steps.length - 1]!;
  const pitch = trip.pitches[last.hold]!;
  const chain = chainOf(pitch, draft.pitches, draft.kinds[index]!.size);
  return trip.grip >= reserveOf(draft, index, chain, chain.indexOf(pitch), last.x, last.y, now) + GRIP_HANG * REST_LEAST;
}

/** 🦺️ The body of the owner of `trip`, of `kind`, at one of its footholds: upright, or leaning towards the wall its hands hold there (`wallLeanOf`) — the body the stage keeps apart once it stands there. */
export function bodyAt(kind: Species, trip: Trip, step: Foothold): Extent {
  return extentAt(trip.owner, kind, step, wallLeanOf(kind, step.footing, step.hold >= 0 ? trip.pitches[step.hold]! : null, step.x), null);
}

/** 🛏️ Whether an outing that ends resting on a wall leaves its actor `COMFORT_GAP` away from every other body there: walls are spaced like perches. */
export function spaced(draft: Draft, index: number, outing: Outing): boolean {
  const trip = outing.trip;
  const room = grown(bodyAt(draft.kinds[index]!, trip, trip.steps[trip.steps.length - 1]!), COMFORT_GAP);
  for (const other of bodiesOf(draft.actors, draft.kinds)) if (other.owner !== trip.owner && meets(room, other.extent)) return false;
  return true;
}

/** 🏕️ The outings of an actor at `stand` to a spot on the wall line of `pitch`, their first foothold at tick `from`: for every pitch of the line it takes hold of from its perch — from below beside the wall, over the rim from the perch on top, or, when it climbs and carries a ladder, from the exit of its own ladder raised against that pitch (`ladderTo`) —, the way to the spot — the one asked for (`y` on `pitch`), or one drawn with the units `place` and `height`: a pitch of the line on its way (from the one it takes hold of to the end of the line it climbs towards) and a height on it; else nearer and nearer to where it took hold on that pitch, halving the way up to `REST_HALVINGS` times — when what is left of its grip there lasts for a rest and the cheapest way off the line ({@link lasting}) and its body keeps the spacing of a perch from everybody else ({@link spaced}). */
function restSpots(draft: Draft, index: number, stand: Stand, pitch: Pitch, y: number | null, place: number, height: number, from: Ticks, now: Ticks): Outing[] {
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const chain = chainOf(pitch, draft.pitches, size);
  const leans = kind.gear.includes("ladder") && kind.gear.includes("climb");
  const outings: Outing[] = [];
  for (let start = 0; start < chain.length; start++) {
    const holds: { readonly hold: WallHold; readonly lean: LadderStand | null }[] = [];
    const hold = gripFor(stand.perch, chain[start]!, size);
    if (hold !== null) holds.push({ hold, lean: null });
    const lean = leans ? ladderTo(stand.perch, chain[start]!, draft.keepouts, size) : null;
    if (lean !== null) holds.push({ hold: { x: clingOf(chain[start]!, size), y: ladderAt(lean, ladderExit(lean, size)).y, over: false }, lean });
    for (const entered of holds) {
      const over = entered.hold.over;
      const entry = over ? rimOf(chain[start]!, size) : entered.hold.y;
      const spots: (readonly [number, number])[] = [];
      if (y !== null) spots.push([chain.indexOf(pitch), y]);
      else {
        const ahead = over ? chain.length - start : start + 1;
        const offset = Math.min(Math.floor(place * ahead), ahead - 1);
        const at = over ? start + offset : start - offset;
        const near = at === start ? entry : over ? rimOf(chain[at]!, size) : footOf(chain[at]!, size);
        const far = over ? footOf(chain[at]!, size) : rimOf(chain[at]!, size);
        spots.push([at, near + (far - near) * height]);
        const end = over ? footOf(chain[start]!, size) : rimOf(chain[start]!, size);
        let share = 1;
        for (let halving = 0; halving < REST_HALVINGS; halving++) {
          share = share / 2;
          spots.push([start, entry + (end - entry) * share]);
        }
      }
      for (const [at, spot] of spots) {
        const outing = entered.lean === null ? fromPerch(draft, index, stand, chain, start, entered.hold, at, spot, false, null, from) : leanOuting(draft, index, stand, chain, start, entered.lean, at, spot, false, null, from);
        if (outing === null || !lasting(draft, index, outing, now) || !spaced(draft, index, outing)) continue;
        outings.push(outing);
        break;
      }
    }
  }
  return outings;
}

/** 🪜️ An outing over the ladder `ladder` of `owner` from the perch of `stand` to `to`, its first foothold at tick `from`. Up: onto the ladder and up to where it steps over ({@link ladderLead}: carrying its own ladder there and raising it with `raise`) and over the rim onto the ledge (`hoistPath` to `ladderLanding` over `LADDER_DISMOUNT_TICKS`). Down: the walk to the ledge on top, the way over the rim onto the ladder (that step over backwards), the climb down to its foot and the step off onto as much of the perch as the whole body needs. It faces the wall the ladder leans against while it is on the ladder. `null` when a perch has no room for its body where the way needs it. */
function ladderOuting(draft: Draft, index: number, stand: Stand, ladder: LadderStand, owner: Slug, up: boolean, to: Perch, raise: boolean, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const start = roomOn(stand.perch, size.width / 2);
  const goal = roomOn(to, size.width / 2);
  if (start === null || goal === null) return null;
  const exit = ladderExit(ladder, size);
  const landing = ladderLanding(ladder, size);
  const top = ladderAt(ladder, exit);
  const facing = wallward(ladder);
  const steps: Foothold[] = [];
  let stands = 0;
  if (up) {
    if (landing.x < goal[0] || landing.x > goal[1]) return null;
    const lead = ladderLead(draft, index, stand, ladder, raise, from);
    if (lead === null) return null;
    steps.push(...lead.steps);
    stands = lead.stands;
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "mantle", facing, hold: -1 });
    }
  } else {
    if (landing.x < start[0] || landing.x > start[1]) return null;
    const lead = walked(stand, kind, landing.x, "walk");
    if (lead === null) return null;
    steps.push(...lead);
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, 1 - tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    }
    rungs(ladder, exit, 0, steps);
    const off = clamp(ladder.foot.x, goal[0], goal[1]);
    if (off !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? off : ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
  }
  return { trip: { owner: body.species, from, steps, pitches: [], ladder: owner, ending: "perch", landing: to.surface, grip: stand.grip }, raise: raise ? ladder : null, stands, rope: null };
}

/** 🎋️ The ways on of an actor that holds on to the ladder `ladder` it climbs, the nearer first: up to where it steps over and over the rim onto the perch that crowns the wall it leans against, or down to its foot and onto as much of the perch it stands on as the whole body needs. */
export function ladderWays(draft: Draft, index: number, ladder: Ladder, now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const half = size.width / 2;
  const travel = clamp((body.y - ladder.foot.y) / (ladder.top.y - ladder.foot.y), 0, 1) * ladderLength(ladder);
  const exit = ladderExit(ladder, size);
  const landing = ladderLanding(ladder, size);
  const ways: Outing[] = [];
  let crown: Perch | null = null;
  for (const pitch of draft.pitches) if (crown === null && pitch.wall === ladder.wall && pitch.side === ladder.side) crown = rimFor(pitch, draft.perches, size);
  const above = crown === null ? null : roomOn(crown, half);
  const facing = wallward(ladder);
  if (crown !== null && above !== null && landing.x >= above[0] && landing.x <= above[1]) {
    const steps: Foothold[] = [];
    rungs(ladder, travel, exit, steps);
    const top = ladderAt(ladder, exit);
    for (let tick = 1; tick <= LADDER_DISMOUNT_TICKS; tick++) {
      const feet = hoistPath(top, landing, size.height, tick / LADDER_DISMOUNT_TICKS);
      steps.push({ x: feet.x, y: feet.y, footing: "ladder", perch: null, activity: "mantle", facing, hold: -1 });
    }
    ways.push({ trip: { owner: body.species, from: now + 1, steps, pitches: [], ladder: ladder.owner, ending: "perch", landing: crown.surface, grip: body.grip }, raise: null, stands: 0, rope: null });
  }
  let ground: Perch | null = null;
  for (const perch of draft.perches) if (ground === null && perch.surface === ladder.surface && perch.y === ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1) ground = perch;
  const below = ground === null ? null : roomOn(ground, half);
  if (ground !== null && below !== null) {
    const steps: Foothold[] = [];
    rungs(ladder, travel, 0, steps);
    const off = clamp(ladder.foot.x, below[0], below[1]);
    if (off !== ladder.foot.x) for (let tick = 1; tick <= LADDER_MOUNT_TICKS; tick++) steps.push({ x: tick === LADDER_MOUNT_TICKS ? off : ladder.foot.x + (off - ladder.foot.x) * smoothstep(tick / LADDER_MOUNT_TICKS), y: ladder.foot.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    if (steps.length === 0) steps.push({ x: body.x, y: body.y, footing: "ladder", perch: null, activity: "climb", facing, hold: -1 });
    const way: Outing = { trip: { owner: body.species, from: now + 1, steps, pitches: [], ladder: ladder.owner, ending: "perch", landing: ground.surface, grip: body.grip }, raise: null, stands: 0, rope: null };
    if (ways.length > 0 && travel < exit - travel) ways.unshift(way);
    else ways.push(way);
  }
  return ways;
}

/** 🪝️ An outing up a grappling rope from the perch of `stand` to `to`, its first foothold at tick `from`: the walk to `at`, aiming over `ROPE_AIM_TICKS`, the flight of the hook (`hookTicks` at `HOOK_SPEED`), the tug over `ROPE_TUG_TICKS`, the haul up the rope (`haulStep` from `haulOf` until `REEL_LEAST` heights of rope are left) and the hoist onto the perch over `ROPE_HOIST_TICKS` (to `landingFor`, onto as much of the perch as the whole body needs), facing the way it shoots from the aim on. With `miss` the shot is aimed past the edge (`missOf`, where that line stays inside the stage and is clear but for what lies under the hook, as for a hit: `sighted`): the hook flies there and is pulled back (`HOOK_RETURN`) and the actor shrugs over `ROPE_SHRUG_TICKS` where it stands. The rope is fired when the aim is done. `null` when the perch it hoists itself onto has no room for its body. */
function ropeOuting(draft: Draft, index: number, stand: Stand, shot: Shot, to: Perch, at: number, miss: boolean, from: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const size = kind.size;
  const goal = roomOn(to, size.width / 2);
  if (goal === null) return null;
  const lead = walked(stand, kind, at, "walk");
  if (lead === null) return null;
  const perch = stand.perch.surface;
  const facing = shot.facing;
  const steps: Foothold[] = [...lead];
  for (let tick = 0; tick < ROPE_AIM_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  const fire = from + steps.length;
  const point = missOf(shot, to);
  const under = { x: shot.hook.x, y: to.y };
  const missing = miss && point.x >= 0 && point.x <= draft.width && sighted(shot.muzzle, point, draft.keepouts, ROPE_MARGIN, under, under);
  const target = missing ? point : shot.hook;
  const out = hookTicks(shot.muzzle, target, HOOK_SPEED);
  for (let tick = 0; tick < out; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  const dx = target.x - shot.muzzle.x;
  const dy = target.y - shot.muzzle.y;
  const rope: Rope = { shot: missing ? { surface: shot.surface, facing: shot.facing, muzzle: shot.muzzle, hook: target, length: shot.length, reel: shot.reel } : shot, since: fire, length: Math.sqrt(dx * dx + dy * dy), hand: shot.muzzle, before: shot.muzzle, caught: !missing };
  if (missing) {
    const back = hookTicks(target, shot.muzzle, HOOK_RETURN);
    for (let tick = 0; tick < back; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
    for (let tick = 0; tick < ROPE_SHRUG_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "shrug", facing, hold: -1 });
    return { trip: { owner: body.species, from, steps, pitches: [], ladder: null, ending: "perch", landing: perch, grip: stand.grip }, raise: null, stands: 0, rope };
  }
  for (let tick = 0; tick < ROPE_TUG_TICKS; tick++) steps.push({ x: at, y: stand.perch.y, footing: "perch", perch, activity: "aim", facing, hold: -1 });
  let haul = haulOf(shot, shot.length, size);
  const least = REEL_LEAST * size.height;
  for (let tick = 0; haul.rope > least && tick < LONGEST; tick++) {
    haul = haulStep(shot, haul, tick, size);
    steps.push({ x: haul.x, y: haul.y, footing: "rope", perch: null, activity: "reel", facing, hold: -1 });
  }
  const landed = landingFor(shot, to, size);
  const spot = { x: clamp(landed.x, goal[0], goal[1]), y: landed.y };
  for (let tick = 1; tick <= ROPE_HOIST_TICKS; tick++) {
    const feet = hoistPath({ x: haul.x, y: haul.y }, spot, size.height, tick / ROPE_HOIST_TICKS);
    steps.push({ x: feet.x, y: feet.y, footing: "rope", perch: null, activity: "mantle", facing, hold: -1 });
  }
  return { trip: { owner: body.species, from, steps, pitches: [], ladder: null, ending: "perch", landing: to.surface, grip: stand.grip }, raise: null, stands: 0, rope };
}

/** 🛝️ The slide of an actor down the wall it clings to, from where it is, with `grip` left: sliding (`slideStep`) down to the highest perch below its feet it reaches before its hands leave the wall at its foot (`footOf`), and stepping off onto as much of it as the whole body needs — or down to the foot of the wall, where it lets go. `null` when it is there already. */
export function slideOuting(draft: Draft, index: number, grip: number, now: Ticks): Outing | null {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  const pitch = body.pitch;
  if (pitch === null) return null;
  const bottom = footOf(pitch, kind.size);
  const below = landingOf(draft.perches, body.x, body.y, bottom);
  const floor = below === null ? bottom : below.y;
  const steps: Foothold[] = [];
  let slide = { y: body.y, vy: 0 };
  const facing = wallward(pitch);
  for (let tick = 0; slide.y !== floor && tick < LONGEST; tick++) {
    slide = slideStep(slide.y, slide.vy, floor);
    steps.push({ x: body.x, y: slide.y, footing: "wall", perch: null, activity: "slide", facing, hold: 0 });
  }
  let ending: "perch" | "air" = "air";
  let landing = "";
  const room = below === null ? null : roomOn(below, kind.size.width / 2);
  if (below !== null && room !== null) {
    const to = clamp(body.x, room[0], room[1]);
    if (to !== body.x) for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) steps.push({ x: tick === WALL_GRAB_TICKS ? to : body.x + (to - body.x) * smoothstep(tick / WALL_GRAB_TICKS), y: below.y, footing: "wall", perch: null, activity: "slide", facing, hold: 0 });
    ending = "perch";
    landing = below.surface;
  }
  if (steps.length === 0) return null;
  let cost = 0;
  for (const step of steps) cost = cost + effortOf(step);
  return { trip: { owner: body.species, from: now + 1, steps, pitches: [pitch], ladder: null, ending, landing, grip: Math.max(grip - cost, 0) }, raise: null, stands: 0, rope: null };
}

/** 📶️ `outing` put among `outings` (quickest first, the fewest ticks) after every one as quick as it. */
function ranked(outings: Outing[], outing: Outing): void {
  let at = outings.length;
  while (at > 0 && outings[at - 1]!.trip.steps.length > outing.trip.steps.length) at--;
  outings.splice(at, 0, outing);
}

/** 🔗️ Two outings one after the other, the second setting out where the first ended: its footholds after those of the first (their holds counted on past the pitches of the first), the pitches of both, the ladder and the rope either uses, the end and the grip of the second. `null` when both use a ladder or both a rope: a trip uses one ladder and one rope at most. */
function joined(first: Outing, second: Outing): Outing | null {
  if ((first.trip.ladder !== null && second.trip.ladder !== null) || (first.rope !== null && second.rope !== null)) return null;
  const offset = first.trip.pitches.length;
  const steps: Foothold[] = [...first.trip.steps];
  for (const step of second.trip.steps) steps.push(step.hold < 0 ? step : { x: step.x, y: step.y, footing: step.footing, perch: step.perch, activity: step.activity, facing: step.facing, hold: step.hold + offset });
  return { trip: { owner: first.trip.owner, from: first.trip.from, steps, pitches: [...first.trip.pitches, ...second.trip.pitches], ladder: first.trip.ladder ?? second.trip.ladder, ending: second.trip.ending, landing: second.trip.landing, grip: second.trip.grip }, raise: first.raise ?? second.raise, stands: first.raise !== null ? first.stands : second.stands, rope: first.rope ?? second.rope };
}

/** 🧷️ The outings in one leg of an actor at `stand` to a venture, their first foothold at tick `from` (`now` the tick it decides at), the quickest first (the fewest ticks, in the order `routeOf` offers them among equals). To a perch: every way `routeOf` offers it with its gear (with the grip of the stand, over the ladders that stand, are free and stand on), as a trip — a ladder of its own only while fewer than two stand and none of its own; a shot from a place where its whole body stays on its perch (shot anew from there when the offered one is not), aimed past the edge with `miss`. To a spot on a wall: {@link restSpots}, with the units `place` and `height` of `units`. */
function legsTo(draft: Draft, index: number, stand: Stand, venture: Venture, units: readonly number[], miss: boolean, from: Ticks, now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (venture.kind === "wall") return restSpots(draft, index, stand, venture.pitch, venture.y, units[0]!, units[1]!, from, now);
  const to = venture.perch;
  const standing: Ladder[] = [];
  for (const ladder of draft.ladders) if (ladder.rider === null && ladder.since <= now && ladder.until > now) standing.push(ladder);
  const legs = routeOf(stand.x, stand.perch, to, kind.gear, kind.size, stand.grip, draft.pitches, standing, draft.keepouts) ?? [];
  const outings: Outing[] = [];
  for (const leg of legs) {
    let outing: Outing | null = null;
    if (leg.means === "ladder") {
      const owner = standing.find((ladder) => ladder === leg.ladder)?.owner;
      if (owner !== undefined) outing = ladderOuting(draft, index, stand, leg.ladder, owner, leg.up, to, false, from);
    } else if (leg.means === "wall") {
      const chain = chainOf(leg.pitch, draft.pitches, kind.size);
      outing = fromPerch(draft, index, stand, chain, chain.indexOf(leg.pitch), leg.hold, chain.indexOf(leg.exit), leg.goal, !leg.hold.over, leg.hold.over ? to : null, from);
    } else if (leg.means === "raise") {
      if (draft.ladders.length < MAX_LADDERS && !draft.ladders.some((ladder) => ladder.owner === body.species)) outing = ladderOuting(draft, index, stand, leg.ladder, body.species, true, to, true, from);
    } else {
      const room = roomOn(stand.perch, kind.size.width / 2);
      const at = room === null ? leg.at : clamp(leg.at, room[0], room[1]);
      const shot = at === leg.at ? leg.shot : shotFor({ x: at, y: stand.perch.y }, [to], draft.keepouts, kind.size);
      if (room !== null && shot !== null) outing = ropeOuting(draft, index, stand, shot, to, at, miss, from);
    }
    if (outing !== null) ranked(outings, outing);
  }
  return outings;
}

/** 🗺️ The outings of an actor that stands on a perch to a venture at tick `now`, in the order it prefers them, the quickest first: the ways in one leg ({@link legsTo}; a shot aimed past the edge when the unit `miss` is below `ROPE_MISS_CHANCE`); only when there is none, the ways in two legs — a leg to another perch (by a ladder, a wall or a rope, never a miss), then a leg from where that one ends to the venture ({@link joined}), planned whole, so a pet whose gear reaches nothing from where it stands goes there by a perch on the way. The units `[place, height, miss]` are draws of the decision on the gear stream. */
export function outingsTo(draft: Draft, index: number, venture: Venture, units: readonly number[], now: Ticks): Outing[] {
  const body = draft.actors[index]!;
  const kind = draft.kinds[index]!;
  if (!geared(kind) || body.footing !== "perch" || body.perch === null) return [];
  const from = perchAt(draft.perches, body.perch, body.x);
  if (from === null) return [];
  const stand = standOf(body, from);
  const direct = legsTo(draft, index, stand, venture, units, units[2]! < ROPE_MISS_CHANCE, now + 1, now);
  if (direct.length > 0) return direct;
  const outings: Outing[] = [];
  for (const between of draft.perches) {
    if (between === from || (venture.kind === "perch" && between === venture.perch)) continue;
    for (const first of legsTo(draft, index, stand, { kind: "perch", perch: between }, units, false, now + 1, now)) {
      const last = first.trip.steps[first.trip.steps.length - 1]!;
      const arrival: Stand = { perch: between, x: last.x, y: last.y, facing: last.facing, grip: first.trip.grip };
      for (const second of legsTo(draft, index, arrival, venture, units, false, first.trip.from + first.trip.steps.length, now)) {
        const both = joined(first, second);
        if (both !== null) ranked(outings, both);
      }
    }
  }
  return outings;
}
//#endregion 🔖️Gear
