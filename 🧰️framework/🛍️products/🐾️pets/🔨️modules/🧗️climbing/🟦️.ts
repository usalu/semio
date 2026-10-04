/** 🧗️ How pets get to a perch they can neither walk nor hop to: up and down the free side walls of the surveyed elements, on a ladder leant against a wall, and on a grappling rope hooked to an edge above.
 *
 * Positions are viewport pixels with the y axis pointing down, speeds are pixels per second and time is whole ticks of
 * 1/64 s. Lengths of a body are fractions of the species `Size` (`width` across, `height` up from the feet); the
 * starting values are those of the research brief for pets of 40 to 56 px. Everything is built from `+ − × ÷`, `sqrt`,
 * `abs`, `floor`, `min`, `max` and comparisons, evaluated in the order written here, so the Rust twin reproduces every
 * result bit for bit.
 *
 * Nothing here keeps state. An actor on a wall is its pitch and the height of its feet, on a ladder the distance it
 * has climbed along the rails, on a rope the length of rope left; the stage stores those numbers and asks this module
 * for the next one. Travel is clipped by distance, never by time: the phase of the climbing clip is a function of
 * where the hands are ({@link climbPhase}, {@link ladderPhase}), so limbs never slide on what they hold.
 *
 * Whatever a rope or a ladder touches is not in its own way: a keep-out that holds the point a line stands on or
 * ends at (the box of the surface under a hook, the box behind the rim a ladder leans on) is passed over when the
 * line is tested ({@link ladderFor}, {@link shotFor}).
 *
 * A wall, per tick: `climbStep(y, goal, ticks)` with the ticks since the climb began, then `gripStep(grip, "climb")`;
 * at {@link rimOf} the mantle, `mantlePath(pitch, size, ticks ÷ MANTLE_TICKS)`. A ladder: `ladderStep(travel, goal,
 * ticks)` and `ladderAt(ladder, travel)`, then {@link hoistPath} from the exit to {@link ladderLanding}. A rope:
 * `hookStep(shot.muzzle, shot.hook, HOOK_SPEED, ticks)` until {@link hookTicks}, then from `haulOf(shot, shot.length,
 * size)` on `haulStep(shot, haul, ticks, size)` until `REEL_LEAST` heights of rope are left, then {@link hoistPath}
 * to {@link landingFor}. After every survey {@link wallHolds}, {@link ladderHolds} and {@link shotHolds} say whether
 * what carries an actor is still there.
 *
 * The pitches of one wall line — the side edges of a column of cards, the same side, no more than `WALL_FOLLOW`
 * apart — are one way up and down: a climber lunges across the gap between two of them when its hands reach the
 * lowest hold of the upper one within `CROSS_REACH` heights from the top of the lower one ({@link crossable},
 * {@link lungePath}), and {@link wallPath} is the whole way, tick by tick, from taking hold to the mantle.
 *
 * @see ../🏞️terrain/🟦️.ts — `wallsOf`, `segmentHits`, `strideTo`
 * @see ../🪢️swing/🟦️.ts — `reelStep`, the swing on a rope that is reeled in, and `REEL_LEAST`, the rope a haul ends with
 * @see ../📐️trigonometry/🟦️.ts — `clamp`, `smoothstep`
 * @see ../../🧬️schema/🟦️.ts — `Gear`, `Ladder`, `Perch`, `Pitch`, `Point`, `Rect`, `Shot`, `Size`
 */

import { type Gear, type Ladder, type Perch, type Pitch, type Point, type Rect, type Shot, type Size, TICKS_PER_SECOND, type Ticks } from "../../🧬️schema/🟦️.ts";
import { type Fall, WALL_LIP, segmentHits, strideTo } from "../🏞️terrain/🟦️.ts";
import { clamp, smoothstep } from "../📐️trigonometry/🟦️.ts";
import { REEL_LEAST, reelStep } from "../🪢️swing/🟦️.ts";

//#region 🔖️Constants
/** ✋️ How high above its feet an actor holds on, as a fraction of its height: the mantle begins when the hands reach the rim. */
export const HAND_HEIGHT = 0.85;

/** ⬆️ The speed of climbing up a wall in pixels per second (0.62 body heights). */
export const CLIMB_RISE = 30;

/** ⬇️ The speed of climbing down a wall in pixels per second. */
export const CLIMB_DESCENT = 40;

/** 🛫️ Over how many ticks a climb on a wall gathers its speed. */
export const CLIMB_RAMP = 6;

/** 🤲️ The distance between two holds on a wall as a fraction of the height; two holds, one per hand, are one cycle of the climbing clip. */
export const GRIP_SPACING = 0.3;

/** 🔋️ The grip a rested actor has, in ticks of climbing (6 s): enough for 150 px of wall and the mantle. */
export const GRIP_BUDGET = 384;

/** 💪️ The grip one tick of climbing, mantling or hanging over a rim costs. */
export const GRIP_CLIMB = 1;

/** 🐒️ The grip one tick of hanging still or sliding costs. */
export const GRIP_HANG = 0.25;

/** 🛌️ The grip one tick on a perch gives back. */
export const GRIP_REST = 2;

/** 🦷️ How many pixels of a pitch must lie under the hands: the hands never hold the very end of a free stretch. */
export const GRIP_BITE = 8;

/** 🧲️ How far a wall may move sideways between two surveys, in pixels, and still carry its climber. */
export const WALL_FOLLOW = 6;

/** 🛝️ The speed a slide down a wall starts with, in pixels per second. */
export const SLIDE_START = 30;

/** 📈️ The acceleration of a slide in pixels per second squared. */
export const SLIDE_GAIN = 600;

/** 🚒️ The fastest slide in pixels per second. */
export const SLIDE_SPEED = 160;

/** 💨️ The sideways speed with which a wall that moved or vanished throws its climber off, in pixels per second, away from the wall. */
export const SLIP_PUSH = 140;

/** 🤸️ The upward speed of that throw in pixels per second. */
export const SLIP_LIFT = 160;

/** 🤏️ The ticks of taking hold of a wall from its foot. */
export const WALL_GRAB_TICKS = 8;

/** 🦥️ The ticks of lowering oneself over a rim onto the wall below. */
export const WALL_HANG_TICKS = 10;

/** 🏋️ The ticks of a mantle from a wall over its rim onto the perch. */
export const MANTLE_TICKS = 28;

/** 📥️ How far beyond half a body width inside the rim a mantle ends, in pixels. */
export const MANTLE_INSET = 8;

/** 🐫️ How far above its end a hoist rises before it settles, as a fraction of the height. */
export const HOIST_HUMP = 0.1;

/** 🌅️ The share of a hoist over which the body rises; the way across begins when this share of the hoist is left. */
export const HOIST_RISE = 0.65;

/** 📐️ The lean of a ladder its owner aims for: foot distance ÷ rise, the 4 : 1 rule of real ladders. */
export const LADDER_LEAN = 0.25;

/** 🗼️ The steepest lean a ladder stands at. */
export const LADDER_STEEP = 0.14;

/** 🛬️ The flattest lean a ladder stands at. */
export const LADDER_FLAT = 0.4;

/** 🪑️ The least rise of a ladder as a fraction of the height; lower edges are hopped onto. */
export const LADDER_SHORT = 0.8;

/** 🦒️ The greatest rise of a ladder as a fraction of the height: a telescopic ladder, carried folded and pulled out to eight heights — what it takes to reach the cards of a page from the footer line below them (the real quiz page at 1440 × 900 leaves 7.5 heights of its tallest climber between them). */
export const LADDER_TALL = 8;

/** 👇️ How far under the rim, or above the lower end of the free stretch it leans on, a ladder touches its wall, in pixels (0.15 heights of a 48 px pet): a measure of the ladder, the same for whoever climbs it. */
export const LADDER_TUCK = 7;

/** 🦌️ How far the rails reach beyond that contact, in pixels; drawn, never climbed. */
export const LADDER_HORNS = 14;

/** 🪜️ The distance between two rungs in pixels (0.22 heights of a 48 px pet); two rungs are one cycle of the climbing clip. */
export const RUNG_SPACING = 10.5;

/** 🦶️ Half the footprint of a ladder on its perch in pixels: the foot stays this far from both ends. */
export const LADDER_FOOTING = 10;

/** 🥖️ Half the thickness of a ladder in pixels: keep-outs are grown by it when its line is tested. */
export const LADDER_GIRTH = 6;

/** 🚀️ The speed of climbing up a ladder in pixels per second (2.5 rungs of a 48 px pet). */
export const LADDER_RISE = 26;

/** 🪂️ The speed of climbing down a ladder in pixels per second. */
export const LADDER_DESCENT = 34;

/** 🏁️ Over how many ticks a climb on a ladder gathers its speed. */
export const LADDER_RAMP = 6;

/** 🚪️ How far before the top of a ladder its climber steps over onto the perch, as a fraction of the height. */
export const LADDER_EXIT = 0.4;

/** 🧷️ How far the top of a ladder may move between two surveys, in pixels, and be followed. */
export const LADDER_FOLLOW = 8;

/** 🌋️ How far the perch under a ladder may move up or down between two surveys, in pixels, before the ladder falls. */
export const LADDER_SHIFT = 12;

/** 🐎️ The ticks of stepping onto a ladder. */
export const LADDER_MOUNT_TICKS = 8;

/** 🛹️ The ticks of stepping from a ladder over the rim onto the perch. */
export const LADDER_DISMOUNT_TICKS = 24;

/** 🏗️ The ticks of raising a ladder from the perch against its wall, and of lowering it again. */
export const LADDER_RAISE_TICKS = 30;

/** 💤️ The ticks after its last use at which a standing ladder is taken away (20 s). */
export const LADDER_IDLE = 1280;

/** ⌛️ The ticks a ladder stands at most (120 s). */
export const LADDER_LIFE = 7680;

/** 🌀️ The stiffness of the spring that topples a ladder about its foot. */
export const TOPPLE_STIFFNESS = 150;

/** 🧽️ The damping of that spring. */
export const TOPPLE_DAMPING = 24;

/** 👋️ The sideways speed with which a toppling ladder throws its climber clear, in pixels per second, away from the wall. */
export const TOPPLE_PUSH = 70;

/** 🩴️ The rise above the foot of a ladder, as a fraction of the height, below which its climber steps off a toppling ladder instead of falling. */
export const TOPPLE_STEP = 0.5;

/** 🏹️ The speed of a flying hook in pixels per second (10 px per tick). */
export const HOOK_SPEED = 640;

/** ↩️ The speed of a hook that is pulled back after a miss, in pixels per second. */
export const HOOK_RETURN = 1280;

/** 🧵️ The shortest rope as a fraction of the height. */
export const ROPE_SHORT = 0.8;

/** 🧶️ The longest rope as a fraction of the height: what the grappling gun shoots, eight heights — the cards of a page stand that far above the footer line its pets live on. */
export const ROPE_LONG = 8;

/** 🌄️ The least rise of a rope per pixel of its length (about 25°): a flatter rope would scrape the face of a card. */
export const ROPE_ELEVATION = 0.42;

/** 📏️ By how many pixels keep-outs are grown when the line of a rope is tested. */
export const ROPE_MARGIN = 2;

/** 🔀️ What a pixel of sideways distance costs beside a pixel of rope when the hook point is chosen. */
export const ROPE_DETOUR = 0.3;

/** 🪝️ How far the edge under a hook may move up or down between two surveys, in pixels, before the hook loses it. */
export const ROPE_FOLLOW = 8;

/** 🏔️ The least height of a perch above the feet for a rope, as a fraction of the height; lower perches are hopped onto. */
export const ROPE_RISE = 0.8;

/** 🔫️ How far in front of the feet the muzzle of the gun is, as a fraction of the width. */
export const MUZZLE_FORWARD = 0.3;

/** 🎚️ How far above the feet the muzzle is, as a fraction of the height; the hands hold the rope there. */
export const MUZZLE_HEIGHT = 0.65;

/** 📌️ How far beyond half a body width inside the end of a perch a hook bites, in pixels. */
export const HOOK_INSET = 4;

/** 🎈️ How far above the edge a hook bites, in pixels. */
export const HOOK_LIFT = 1;

/** ⚡️ The widest slant of a rope that is reeled in straight, sideways distance ÷ rise; a wider one swings. */
export const ZIP_SLANT = 0.35;

/** 🚡️ The speed of reeling straight up a rope in pixels per second. */
export const ZIP_SPEED = 150;

/** 🎢️ Over how many ticks a zip gathers its speed. */
export const ZIP_RAMP = 10;

/** 🎯️ The ticks of aiming before a shot. */
export const ROPE_AIM_TICKS = 14;

/** 💥️ The ticks of the recoil after a shot. */
export const ROPE_RECOIL_TICKS = 6;

/** 🪢️ The ticks of the tug that tests a hook before reeling. */
export const ROPE_TUG_TICKS = 4;

/** 🛗️ The ticks of the last hand-over-hand metre and the mantle onto the perch. */
export const ROPE_HOIST_TICKS = 46;

/** 🤷️ The ticks of the shrug after a miss. */
export const ROPE_SHRUG_TICKS = 40;

/** 🐸️ The farthest lunge from one pitch of a wall line to the next, as a fraction of the height: the hands reach from the top of the lower pitch to the lowest hold of the upper one (`GRIP_BITE` above its end) across a gap of up to this many heights less the bite, and drop as far. */
export const CROSS_REACH = 1.5;

/** 🦗️ The ticks of a lunge from one pitch of a wall line to the next, up or down. */
export const LUNGE_TICKS = 16;

/** 🎲️ The chance of a shot that is aimed past the edge for charm. */
export const ROPE_MISS_CHANCE = 0.1;

/** 🙈️ How far past the end of the perch such a shot is aimed, in pixels. */
export const ROPE_MISS_OVERSHOOT = 14;

/** ⏲️ The ticks before the next shot after a miss (4 s). */
export const ROPE_REST = 256;

/** ⏰️ The ticks before the next shot after two misses in a row (60 s). */
export const ROPE_SULK = 3840;

const PATIENCE = 1024;
const LONGEST = 4096;
//#endregion 🔖️Constants

//#region 🔖️Types
/** 🧤️ Where the feet of an actor are once it has taken hold of a pitch, and whether it came over the rim from the perch on top (`over`) or from a perch at the foot of the wall. */
export type WallHold = { readonly x: number; readonly y: number; readonly over: boolean };

/** 🏃️ What an actor on a wall is doing with its grip: climbing, hanging still or resting on a perch. */
export type Effort = "climb" | "hang" | "rest";

/** 🤾️ A velocity in pixels per second with which an actor is thrown into the air. */
export type Toss = { readonly vx: number; readonly vy: number };

/** 🎣️ An actor that is reeled in: the rope left between its hands and the hook, where its hands are and where they were a tick before, and where its feet are. */
export type Haul = { readonly rope: number; readonly hand: Point; readonly before: Point; readonly x: number; readonly y: number };

/** 🧰️ A ladder as it stands: the wall it leans against and the side that wall faces, the surface it stands on, its foot on that surface and its top on the wall — the stage's `Ladder` without its owner, its lifetime and its rider. */
export type LadderStand = Pick<Ladder, "wall" | "surface" | "side" | "foot" | "top">;

/** 🖐️ What the hands do at one tick of a way along a wall: take hold from a perch beside it, lower the body over the rim from the perch on top, climb, lunge to the next pitch of the wall line, or mantle over the rim onto the perch on top. */
export type Handwork = "grab" | "hang" | "climb" | "lunge" | "mantle";

/** 👣️ One tick of a way along a wall: where the feet are at its end, which pitch of the wall line the hands hold or reach for (its index in the line) and what they do. */
export type Clamber = { readonly x: number; readonly y: number; readonly hold: number; readonly work: Handwork };

/** 🗺️ One way from a perch to another: where on its perch the actor walks to first (`at`) and what it uses from there — a ladder that stands (`up` or down), a wall line (the pitch it takes hold of and the hold it takes, the pitch at whose rim or foot its climb ends and the height it ends at there), a ladder of its own raised on the spot, or a shot of its grappling gun. */
export type Leg =
  | { readonly means: "ladder"; readonly at: number; readonly ladder: LadderStand; readonly up: boolean }
  | { readonly means: "wall"; readonly at: number; readonly pitch: Pitch; readonly hold: WallHold; readonly exit: Pitch; readonly goal: number }
  | { readonly means: "raise"; readonly at: number; readonly ladder: LadderStand }
  | { readonly means: "grapple"; readonly at: number; readonly shot: Shot };
//#endregion 🔖️Types

//#region 🔖️Measures
/** 🧮️ The least whole number that is not below `value`, by `floor` alone. */
function ceiling(value: number): number {
  return 0 - Math.floor(0 - value);
}

/** 🍰️ The part of `value` beyond its whole number, in [0, 1). */
function fraction(value: number): number {
  return value - Math.floor(value);
}

/** 🐢️ The speed in the tick that begins `ticks` ticks after a start, gathered over `ramp` ticks: `speed × smoothstep((ticks + 1) ÷ ramp)`. */
function gathered(speed: number, ticks: Ticks, ramp: Ticks): number {
  return speed * smoothstep((ticks + 1) / ramp);
}

/** 📦️ Whether `rect` has an area and holds `point`, its edges included. */
function holds(rect: Rect, point: Point): boolean {
  return rect.width > 0 && rect.height > 0 && rect.x <= point.x && point.x <= rect.x + rect.width && rect.y <= point.y && point.y <= rect.y + rect.height;
}

/** 🔭️ Whether the segment from `from` to `to` hits none of the keep-outs grown by `margin`, passing over every keep-out that holds `first` or `second`: what a line stands on or ends at is not in its way. */
export function sighted(from: Point, to: Point, keepouts: readonly Rect[], margin: number, first: Point, second: Point): boolean {
  for (const keepout of keepouts) if (!holds(keepout, first) && !holds(keepout, second) && segmentHits(from, to, keepout, margin)) return false;
  return true;
}
//#endregion 🔖️Measures

//#region 🔖️Hoisting
/** 🦘️ The feet at `phase` 0…1 of a hoist from `from` over an edge onto `to`: the body rises over the first `HOIST_RISE` of the way to `HOIST_HUMP` heights above its end, crosses over the last `HOIST_RISE` of it and settles onto the end over the rest, each part eased by `smoothstep`; `from` itself at 0 and before, `to` itself at 1 and after. A descent over an edge is the same path with the phase running back. */
export function hoistPath(from: Point, to: Point, height: number, phase: number): Point {
  if (phase >= 1) return { x: to.x, y: to.y };
  const hump = HOIST_HUMP * height;
  const rise = smoothstep(phase / HOIST_RISE);
  const across = smoothstep((phase - (1 - HOIST_RISE)) / HOIST_RISE);
  const settle = smoothstep((phase - HOIST_RISE) / (1 - HOIST_RISE));
  return { x: from.x + (to.x - from.x) * across, y: from.y + (to.y - hump - from.y) * rise + hump * settle };
}
//#endregion 🔖️Hoisting

//#region 🔖️Walls
/** 🦎️ The x of the feet of an actor that clings to `pitch`: half its width out on the air side of the wall. */
export function clingOf(pitch: Pitch, size: Size): number {
  return pitch.x + (pitch.side * size.width) / 2;
}

/** 🏝️ The x of the spot on top of the wall where a mantle ends: half a width and `MANTLE_INSET` inside the rim. */
export function ledgeOf(pitch: Pitch, size: Size): number {
  return pitch.x - pitch.side * (size.width / 2 + MANTLE_INSET);
}

/** 🧢️ The height of the feet at which the hands reach the top of `pitch`: the highest an actor climbs on it. */
export function rimOf(pitch: Pitch, size: Size): number {
  return pitch.y0 + HAND_HEIGHT * size.height;
}

/** 🥾️ The height of the feet at which the hands hold `GRIP_BITE` above the lower end of `pitch`: the lowest an actor hangs on it. */
export function footOf(pitch: Pitch, size: Size): number {
  return pitch.y1 - GRIP_BITE + HAND_HEIGHT * size.height;
}

/** 🦀️ Whether an actor with its feet at height `y` has its hands on `pitch`: between {@link rimOf} and {@link footOf}, both included. */
export function clings(pitch: Pitch, y: number, size: Size): boolean {
  return rimOf(pitch, size) <= y && y <= footOf(pitch, size);
}

/** 👑️ Whether `perch` lies on top of `pitch`: on the surface the wall belongs to, at the height the pitch begins at, and carrying the ledge beside the rim. */
export function crowns(perch: Perch, pitch: Pitch, size: Size): boolean {
  const ledge = ledgeOf(pitch, size);
  return perch.surface === pitch.surface && perch.y === pitch.y0 && perch.x0 <= ledge && ledge <= perch.x1;
}

/** 🏆️ The first perch that crowns `pitch`: where a mantle from it ends; `null` when the rim carries nobody. */
export function rimFor(pitch: Pitch, perches: readonly Perch[], size: Size): Perch | null {
  for (const perch of perches) if (crowns(perch, pitch, size)) return perch;
  return null;
}

/** 🫴️ The hold an actor on `perch` takes on `pitch`, or `null` when it cannot: from the perch that crowns the pitch it lowers itself over the rim (`over`, feet at {@link rimOf}); from any other perch it must be able to stand at {@link clingOf} with its hands on the pitch, and keeps its height. */
export function gripFor(perch: Perch, pitch: Pitch, size: Size): WallHold | null {
  const x = clingOf(pitch, size);
  const rim = rimOf(pitch, size);
  if (crowns(perch, pitch, size)) return clings(pitch, rim, size) ? { x, y: rim, over: true } : null;
  return perch.x0 <= x && x <= perch.x1 && clings(pitch, perch.y, size) ? { x, y: perch.y, over: false } : null;
}

/** 🧐️ The pitch that still carries an actor at height `y` after a survey: the first of `pitches` on the same wall and side, no more than `WALL_FOLLOW` aside of `pitch`, that the actor clings to; `null` when the wall moved away, vanished or is no longer free there — the actor slips. */
export function wallHolds(pitch: Pitch, pitches: readonly Pitch[], y: number, size: Size): Pitch | null {
  for (const next of pitches) if (next.wall === pitch.wall && next.side === pitch.side && Math.abs(next.x - pitch.x) <= WALL_FOLLOW && clings(next, y, size)) return next;
  return null;
}

/** 🍌️ The velocity with which an actor that lost `pitch` is thrown off: `SLIP_PUSH` away from the wall and `SLIP_LIFT` up. */
export function slipOf(pitch: Pitch): Toss {
  return { vx: pitch.side * SLIP_PUSH, vy: 0 - SLIP_LIFT };
}

/** 🔌️ The grip one tick later: climbing costs `GRIP_CLIMB`, hanging `GRIP_HANG`, never below 0; resting gives `GRIP_REST` back, never beyond `GRIP_BUDGET`. */
export function gripStep(grip: number, effort: Effort): number {
  return effort === "rest" ? Math.min(grip + GRIP_REST, GRIP_BUDGET) : Math.max(grip - (effort === "climb" ? GRIP_CLIMB : GRIP_HANG), 0);
}

/** 🐜️ The height of the feet one tick later on the way to `goal`, `ticks` ticks after the climb began: up at `CLIMB_RISE`, down at `CLIMB_DESCENT`, gathered over `CLIMB_RAMP` ticks, and the goal itself as soon as it is within one step. */
export function climbStep(y: number, goal: number, ticks: Ticks): number {
  return strideTo(y, goal, gathered(goal < y ? CLIMB_RISE : CLIMB_DESCENT, ticks, CLIMB_RAMP));
}

/** ⏱️ How many ticks the climb from `y` to `goal` takes; `GRIP_BUDGET + 1` when no rested grip lasts for it. */
export function climbTicks(y: number, goal: number): Ticks {
  let height = y;
  let ticks = 0;
  while (height !== goal && ticks <= GRIP_BUDGET) {
    height = climbStep(height, goal, ticks);
    ticks++;
  }
  return ticks;
}

/** 🎡️ The phase 0…1 of the climbing clip at height `y` on `pitch`: one cycle per two holds of `GRIP_SPACING` heights, counted up the wall from its top, so the hands meet the same spots of the wall on every pass. */
export function climbPhase(pitch: Pitch, y: number, size: Size): number {
  return fraction((pitch.y0 - y) / (2 * GRIP_SPACING * size.height));
}

/** 🧈️ One tick of sliding down a wall, velocity first: `vy ← min(max(vy, SLIDE_START) + SLIDE_GAIN ÷ 64, SLIDE_SPEED)`, then `y ← min(y + vy ÷ 64, floor)` — the slide ends at `floor`, the perch below or {@link footOf}. */
export function slideStep(y: number, vy: number, floor: number): Fall {
  const speed = Math.min(Math.max(vy, SLIDE_START) + SLIDE_GAIN / TICKS_PER_SECOND, SLIDE_SPEED);
  return { y: Math.min(y + speed / TICKS_PER_SECOND, floor), vy: speed };
}

/** 🧘️ The feet at `phase` 0…1 of the mantle from `pitch` over its rim onto the perch that crowns it: {@link hoistPath} from the highest hold to the ledge. */
export function mantlePath(pitch: Pitch, size: Size, phase: number): Point {
  return hoistPath({ x: clingOf(pitch, size), y: rimOf(pitch, size) }, { x: ledgeOf(pitch, size), y: pitch.y0 }, size.height, phase);
}

/** 🌉️ Whether an actor of `size` lunges from `from` to `to`: the two stand on one wall line (the same side, no more than `WALL_FOLLOW` apart), one lies wholly above the other, each holds the actor somewhere (its rim not below its foot), and the hands cross the gap from the top of the lower one to the lowest hold of the upper one, `GRIP_BITE` above its end, within `CROSS_REACH` heights. */
export function crossable(from: Pitch, to: Pitch, size: Size): boolean {
  if (from.side !== to.side || Math.abs(to.x - from.x) > WALL_FOLLOW) return false;
  const upward = to.y1 <= from.y0;
  if (!upward && !(from.y1 <= to.y0)) return false;
  const upper = upward ? to : from;
  const lower = upward ? from : to;
  return rimOf(upper, size) <= footOf(upper, size) && rimOf(lower, size) <= footOf(lower, size) && lower.y0 - upper.y1 + GRIP_BITE <= CROSS_REACH * size.height;
}

/** 🐇️ The feet at `phase` 0…1 of a lunge from `from` to `to`: {@link hoistPath} from the hold of `from` nearest to `to` (its rim when `to` lies above, its foot when below) to the hold of `to` nearest to `from` (its foot, or its rim), half a width out from each wall. */
export function lungePath(from: Pitch, to: Pitch, size: Size, phase: number): Point {
  const upward = to.y1 <= from.y0;
  return hoistPath({ x: clingOf(from, size), y: upward ? rimOf(from, size) : footOf(from, size) }, { x: clingOf(to, size), y: upward ? footOf(to, size) : rimOf(to, size) }, size.height, phase);
}

/** 🔗️ Whether `chain` holds a pitch with the same values as `pitch`. */
function listed(chain: readonly Pitch[], pitch: Pitch): boolean {
  for (const entry of chain) if (entry.wall === pitch.wall && entry.surface === pitch.surface && entry.side === pitch.side && entry.x === pitch.x && entry.y0 === pitch.y0 && entry.y1 === pitch.y1) return true;
  return false;
}

/** ⛓️ The wall line an actor of `size` climbs from `pitch` without letting go: `pitch` and every pitch of `pitches` it reaches by lunges from one to the next ({@link crossable}) — above it the one with the lowest end, below it the one with the highest top, the first among equals, never one the line holds already (so it ends on every survey, even one of pitches without length far out, where the arithmetic cannot tell an end from a hold) —, from the top down. */
export function chainOf(pitch: Pitch, pitches: readonly Pitch[], size: Size): Pitch[] {
  const chain: Pitch[] = [pitch];
  for (;;) {
    const top = chain[0]!;
    let above: Pitch | null = null;
    for (const next of pitches) if (next.y1 <= top.y0 && !listed(chain, next) && crossable(top, next, size) && (above === null || next.y1 > above.y1)) above = next;
    if (above === null) break;
    chain.unshift(above);
  }
  for (;;) {
    const bottom = chain[chain.length - 1]!;
    let below: Pitch | null = null;
    for (const next of pitches) if (next.y0 >= bottom.y1 && !listed(chain, next) && crossable(bottom, next, size) && (below === null || next.y0 < below.y0)) below = next;
    if (below === null) break;
    chain.push(below);
  }
  return chain;
}

/** 🪨️ The way of an actor of `size` along the wall line `chain` (its pitches from the top down, as {@link chainOf} answers them), tick by tick. It takes hold of `chain[from]` — from a perch beside the wall where its feet stand at (`x`, `y`), stepping onto the hold over `WALL_GRAB_TICKS` (`grab`); over the rim from the perch on top, lowering itself along the mantle path backwards over `WALL_HANG_TICKS` (`hang`, `x` and `y` are not read); or holding on there already with its feet at height `y` (`cling`) —, climbs on every pitch from where it holds to the end of that pitch on its way (the rim upwards, the foot downwards; every climb gathers its speed anew) and lunges across every gap ({@link lungePath} over `LUNGE_TICKS`) until it reaches `goal` on `chain[to]`, and with `mantle` it mantles over the rim of `chain[to]` onto the ledge ({@link mantlePath} over `MANTLE_TICKS`). A climb that does not arrive within 4096 ticks is cut there. */
export function wallPath(chain: readonly Pitch[], from: number, entry: "grab" | "hang" | "cling", x: number, y: number, to: number, goal: number, mantle: boolean, size: Size): Clamber[] {
  const path: Clamber[] = [];
  const first = chain[from]!;
  const hold = clingOf(first, size);
  let height = y;
  if (entry === "grab") {
    for (let tick = 1; tick <= WALL_GRAB_TICKS; tick++) path.push({ x: tick === WALL_GRAB_TICKS ? hold : x + (hold - x) * smoothstep(tick / WALL_GRAB_TICKS), y, hold: from, work: "grab" });
  } else if (entry === "hang") {
    for (let tick = 1; tick <= WALL_HANG_TICKS; tick++) {
      const feet = mantlePath(first, size, 1 - tick / WALL_HANG_TICKS);
      path.push({ x: feet.x, y: feet.y, hold: from, work: "hang" });
    }
    height = rimOf(first, size);
  }
  let at = from;
  for (;;) {
    const pitch = chain[at]!;
    const cling = clingOf(pitch, size);
    const end = at === to ? goal : at > to ? rimOf(pitch, size) : footOf(pitch, size);
    for (let tick = 0; height !== end && tick < LONGEST; tick++) {
      height = climbStep(height, end, tick);
      path.push({ x: cling, y: height, hold: at, work: "climb" });
    }
    if (at === to) break;
    const next = at > to ? at - 1 : at + 1;
    for (let tick = 1; tick <= LUNGE_TICKS; tick++) {
      const feet = lungePath(pitch, chain[next]!, size, tick / LUNGE_TICKS);
      path.push({ x: feet.x, y: feet.y, hold: next, work: "lunge" });
    }
    height = at > to ? footOf(chain[next]!, size) : rimOf(chain[next]!, size);
    at = next;
  }
  if (mantle) {
    for (let tick = 1; tick <= MANTLE_TICKS; tick++) {
      const feet = mantlePath(chain[to]!, size, tick / MANTLE_TICKS);
      path.push({ x: feet.x, y: feet.y, hold: to, work: "mantle" });
    }
  }
  return path;
}

/** 🪫️ The grip a way along a wall costs: `GRIP_CLIMB` for every tick of it — taking hold, lowering itself over a rim, climbing, lunging and mantling all hold the wall. */
export function wallCost(path: readonly Clamber[]): number {
  return path.length * GRIP_CLIMB;
}
//#endregion 🔖️Walls

//#region 🔖️Ladders
/** 🪡️ The distance between two points. */
function span(from: Point, to: Point): number {
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  return Math.sqrt(dx * dx + dy * dy);
}

/** 🧾️ The keep-outs a ladder that leans on `pitch` has to keep clear of: every one but those that begin within `WALL_LIP` of the wall line on the side of its element — the body of the element whose side the wall is and of what stands under it in the same line, which the ladder leans against and never passes in front of (a survey grows such a body sideways by a few pixels, so the line of a ladder along it would touch it). */
function flanking(keepouts: readonly Rect[], pitch: Pitch): Rect[] {
  const kept: Rect[] = [];
  for (const keepout of keepouts) if (pitch.side < 0 ? keepout.x < pitch.x - WALL_LIP : keepout.x + keepout.width > pitch.x + WALL_LIP) kept.push(keepout);
  return kept;
}

/** 🧍️ The ladder with its foot at `x` on `low` and its top touching `pitch` at the height `y`, or `null` when it cannot stand so: the rise within `LADDER_SHORT`…`LADDER_TALL` heights, the contact on the pitch (no higher than `LADDER_TUCK` under its top, no lower than its lower end), the footprint on the perch, the lean within `LADDER_STEEP`…`LADDER_FLAT` on the air side, and its line clear of every keep-out on the air side of the wall ({@link flanking}) grown by `LADDER_GIRTH`. */
function stood(low: Perch, pitch: Pitch, x: number, y: number, keepouts: readonly Rect[], size: Size): LadderStand | null {
  const top = { x: pitch.x, y };
  const foot = { x, y: low.y };
  const rise = foot.y - top.y;
  const lean = (x - pitch.x) * pitch.side;
  const fits = LADDER_SHORT * size.height <= rise && rise <= LADDER_TALL * size.height && pitch.y0 + LADDER_TUCK <= top.y && top.y <= pitch.y1 && low.x0 + LADDER_FOOTING <= x && x <= low.x1 - LADDER_FOOTING && LADDER_STEEP * rise <= lean && lean <= LADDER_FLAT * rise;
  return fits && sighted(foot, top, flanking(keepouts, pitch), LADDER_GIRTH, foot, top) ? { wall: pitch.wall, surface: low.surface, side: pitch.side, foot, top } : null;
}

/** 🔨️ The ladder an actor of `size` raises on `low` against `pitch` to reach `high`, or `null`: `high` must crown the pitch, the top touches the wall `LADDER_TUCK` under the rim, and the foot stands where the lean is `LADDER_LEAN`, or as near to that as the perch lets it. */
export function ladderFor(low: Perch, high: Perch, pitch: Pitch, keepouts: readonly Rect[], size: Size): LadderStand | null {
  if (!crowns(high, pitch, size)) return null;
  const top = pitch.y0 + LADDER_TUCK;
  const wanted = pitch.x + pitch.side * LADDER_LEAN * (low.y - top);
  return stood(low, pitch, clamp(wanted, low.x0 + LADDER_FOOTING, low.x1 - LADDER_FOOTING), top, keepouts, size);
}

/** 🪵️ The ladder an actor of `size` raises on `low` against `pitch` to take hold of the wall from it, or `null`: the top touches the wall at the lower end of the pitch — as low as a ladder leans on it, so the shortest that reaches it —, the foot stands where the lean is `LADDER_LEAN`, or as near to that as the perch lets it, it stands by the rules of {@link ladderFor}, and a climber at its exit (`ladderExit`) has its hands on the pitch ({@link clings}): from there it steps over onto the wall. */
export function ladderTo(low: Perch, pitch: Pitch, keepouts: readonly Rect[], size: Size): LadderStand | null {
  const top = pitch.y1;
  const wanted = pitch.x + pitch.side * LADDER_LEAN * (low.y - top);
  const stand = stood(low, pitch, clamp(wanted, low.x0 + LADDER_FOOTING, low.x1 - LADDER_FOOTING), top, keepouts, size);
  return stand !== null && clings(pitch, ladderAt(stand, ladderExit(stand, size)).y, size) ? stand : null;
}

/** 🩺️ The ladder as it stands after a survey, or `null` when it topples: its foot keeps its x on a perch of its surface that moved no more than `LADDER_SHIFT` up or down, its top follows a pitch of its wall — touching it under the rim or at the lower end, as ladders lean ({@link ladderFor}, {@link ladderTo}) —, no farther than `LADDER_FOLLOW`, and it still stands by the rules of {@link ladderFor} for its owner's `size`. A ladder leans on its wall, not on what lies on top of it: it stands on when the perch on the rim is gone. */
export function ladderHolds(ladder: LadderStand, perches: readonly Perch[], pitches: readonly Pitch[], keepouts: readonly Rect[], size: Size): LadderStand | null {
  for (const low of perches) {
    if (low.surface !== ladder.surface || Math.abs(low.y - ladder.foot.y) > LADDER_SHIFT) continue;
    for (const pitch of pitches) {
      if (pitch.wall !== ladder.wall || pitch.side !== ladder.side) continue;
      for (const top of [pitch.y0 + LADDER_TUCK, pitch.y1]) {
        const next = stood(low, pitch, ladder.foot.x, top, keepouts, size);
        if (next !== null && span(ladder.top, next.top) <= LADDER_FOLLOW) return next;
      }
    }
  }
  return null;
}

/** 🎋️ The length of a ladder from its foot to its top. */
export function ladderLength(ladder: LadderStand): number {
  return span(ladder.foot, ladder.top);
}

/** 🎼️ The number of rungs of a ladder: one per `RUNG_SPACING` of its length. */
export function ladderRungs(ladder: LadderStand): number {
  return Math.floor(ladderLength(ladder) / RUNG_SPACING);
}

/** 🍕️ The lean of a ladder: the distance of its foot from the wall ÷ its rise. */
export function ladderLean(ladder: LadderStand): number {
  return Math.abs(ladder.foot.x - ladder.top.x) / (ladder.foot.y - ladder.top.y);
}

/** 🛎️ The distance along a ladder at which a climber of `size` steps over onto the perch: `LADDER_EXIT` heights before its top, never before its foot. */
export function ladderExit(ladder: LadderStand, size: Size): number {
  return Math.max(ladderLength(ladder) - LADDER_EXIT * size.height, 0);
}

/** 🐛️ The distance climbed along a ladder one tick later on the way to `goal`, `ticks` ticks after the climb began: up at `LADDER_RISE`, down at `LADDER_DESCENT`, gathered over `LADDER_RAMP` ticks, and the goal itself as soon as it is within one step. */
export function ladderStep(travel: number, goal: number, ticks: Ticks): number {
  return strideTo(travel, goal, gathered(goal > travel ? LADDER_RISE : LADDER_DESCENT, ticks, LADDER_RAMP));
}

/** 📍️ The feet of a climber that has climbed `travel` pixels along a ladder from its foot: the foot itself at 0 and before, the top itself at its length and beyond. */
export function ladderAt(ladder: LadderStand, travel: number): Point {
  const length = ladderLength(ladder);
  if (!(travel > 0)) return { x: ladder.foot.x, y: ladder.foot.y };
  if (travel >= length) return { x: ladder.top.x, y: ladder.top.y };
  const share = travel / length;
  return { x: ladder.foot.x + (ladder.top.x - ladder.foot.x) * share, y: ladder.foot.y + (ladder.top.y - ladder.foot.y) * share };
}

/** 🎠️ The phase 0…1 of the climbing clip after `travel` pixels along a ladder: one cycle per two rungs, so hands and feet meet the rungs. */
export function ladderPhase(travel: number): number {
  return fraction(travel / (2 * RUNG_SPACING));
}

/** 🏖️ Where a climber of `size` stands once it has stepped off the top of a ladder: on the rim the ladder leans against, half a width and `MANTLE_INSET` inside it. */
export function ladderLanding(ladder: LadderStand, size: Size): Point {
  return { x: ladder.top.x - ladder.side * (size.width / 2 + MANTLE_INSET), y: ladder.top.y - LADDER_TUCK };
}

/** 🎳️ What a toppling ladder does to a climber whose feet are at height `y`: `null` when it is less than `TOPPLE_STEP` heights above the foot and steps off, else the velocity that throws it clear of the wall. */
export function spillOf(ladder: LadderStand, y: number, size: Size): Toss | null {
  return ladder.foot.y - y < TOPPLE_STEP * size.height ? null : { vx: ladder.side * TOPPLE_PUSH, vy: 0 };
}
//#endregion 🔖️Ladders

//#region 🔖️Rope
/** 🔦️ The shot from `feet` at the point `x` of the edge of `perch`, or `null`: the actor faces the point, the muzzle is `MUZZLE_FORWARD` widths in front of the feet and `MUZZLE_HEIGHT` heights above them, the hook bites `HOOK_LIFT` above the edge; the rope must be `ROPE_SHORT`…`ROPE_LONG` heights long, rise at least `ROPE_ELEVATION` per pixel of its length and miss every keep-out grown by `ROPE_MARGIN`, except what lies under the hook. A rope no more slanted than `ZIP_SLANT` is reeled in straight, any other swings — unless the swing would carry the actor into a keep-out under the hook (the line straight down from the hook, as long as the rope, misses no keep-out grown by `ROPE_MARGIN` but what lies under the hook): such a rope is hauled in straight too, beside the element it rises along. */
function aimed(feet: Point, perch: Perch, x: number, keepouts: readonly Rect[], size: Size): Shot | null {
  const facing: 1 | -1 = x < feet.x ? -1 : 1;
  const muzzle = { x: feet.x + facing * MUZZLE_FORWARD * size.width, y: feet.y - MUZZLE_HEIGHT * size.height };
  const hook = { x, y: perch.y - HOOK_LIFT };
  const across = Math.abs(hook.x - muzzle.x);
  const rise = muzzle.y - hook.y;
  const length = span(muzzle, hook);
  const under = { x, y: perch.y };
  const reaches = ROPE_SHORT * size.height <= length && length <= ROPE_LONG * size.height && rise >= ROPE_ELEVATION * length;
  if (!reaches || !sighted(muzzle, hook, keepouts, ROPE_MARGIN, under, under)) return null;
  const swings = across > ZIP_SLANT * rise && sighted(hook, { x, y: hook.y + length }, keepouts, ROPE_MARGIN, under, under);
  return { surface: perch.surface, facing, muzzle, hook, length, reel: swings ? "swing" : "zip" };
}

/** 🎇️ The best shot of the grappling gun from `feet`, or `null` when no edge is in reach: of every perch at least `ROPE_RISE` heights above the feet the two ends, each `HOOK_INSET` inside — the hook bites the corner of the edge, so a rope from below can rise beside the element the edge tops, where the face of that element is in the way of every other line —, and the point between them nearest to the feet (the middle of a perch too narrow for that) are tried, and the shot with the least `length + ROPE_DETOUR × sideways distance` wins, the first one among equals. Where its body lands once it is up is the hoist's to say (`landingFor`). */
export function shotFor(feet: Point, perches: readonly Perch[], keepouts: readonly Rect[], size: Size): Shot | null {
  let best: Shot | null = null;
  let least = Infinity;
  for (const perch of perches) {
    if (!(feet.y - perch.y >= ROPE_RISE * size.height)) continue;
    const low = perch.x0 + HOOK_INSET;
    const high = perch.x1 - HOOK_INSET;
    const spots = low <= high ? [low, high, clamp(feet.x, low, high)] : [(perch.x0 + perch.x1) / 2];
    for (const spot of spots) {
      const shot = aimed(feet, perch, spot, keepouts, size);
      if (shot === null) continue;
      const cost = shot.length + ROPE_DETOUR * Math.abs(shot.hook.x - shot.muzzle.x);
      if (cost < least) {
        best = shot;
        least = cost;
      }
    }
  }
  return best;
}

/** 🧿️ The shot as it holds after a survey, or `null` when the hook lost its edge: a perch of its surface must still carry the hook's x no more than `ROPE_FOLLOW` above or below, and the line from the muzzle to the hook on that edge must still be clear; the hook follows the edge, the rope takes its new length. */
export function shotHolds(shot: Shot, perches: readonly Perch[], keepouts: readonly Rect[]): Shot | null {
  for (const perch of perches) {
    if (perch.surface !== shot.surface || !(perch.x0 <= shot.hook.x && shot.hook.x <= perch.x1) || Math.abs(perch.y - HOOK_LIFT - shot.hook.y) > ROPE_FOLLOW) continue;
    const hook = { x: shot.hook.x, y: perch.y - HOOK_LIFT };
    const under = { x: hook.x, y: perch.y };
    if (sighted(shot.muzzle, hook, keepouts, ROPE_MARGIN, under, under)) return { surface: shot.surface, facing: shot.facing, muzzle: shot.muzzle, hook, length: span(shot.muzzle, hook), reel: shot.reel };
  }
  return null;
}

/** 🫥️ Where a shot that is meant to miss is aimed: `ROPE_MISS_OVERSHOOT` past the end of `perch` that is nearer to the hook, at the height of the hook — the hook flies there, finds nothing and is pulled back. */
export function missOf(shot: Shot, perch: Perch): Point {
  return { x: shot.hook.x - perch.x0 <= perch.x1 - shot.hook.x ? perch.x0 - ROPE_MISS_OVERSHOOT : perch.x1 + ROPE_MISS_OVERSHOOT, y: shot.hook.y };
}

/** 🕰️ How many ticks a hook flies from `from` to `to` at `speed` pixels per second. */
export function hookTicks(from: Point, to: Point, speed: number): Ticks {
  return ceiling((span(from, to) * TICKS_PER_SECOND) / speed);
}

/** 🪃️ The hook `ticks` ticks after it left `from` for `to` on a straight line at `speed` pixels per second: `from` itself at 0 and before, `to` itself from {@link hookTicks} on. The flight out is `hookStep(muzzle, hook, HOOK_SPEED, ticks)`, the way back after a miss `hookStep(tip, muzzle, HOOK_RETURN, ticks)`. */
export function hookStep(from: Point, to: Point, speed: number, ticks: Ticks): Point {
  const length = span(from, to);
  if (!(ticks > 0)) return { x: from.x, y: from.y };
  if (ticks >= ceiling((length * TICKS_PER_SECOND) / speed)) return { x: to.x, y: to.y };
  const share = (speed * ticks) / TICKS_PER_SECOND / length;
  return { x: from.x + (to.x - from.x) * share, y: from.y + (to.y - from.y) * share };
}

/** 🐟️ The actor whose hands hold the rope of `shot` at `hand` with `rope` pixels left to the hook: its feet hang under the hands as they stood under the muzzle. */
function hung(shot: Shot, rope: number, hand: Point, before: Point, size: Size): Haul {
  return { rope, hand, before, x: hand.x - shot.facing * MUZZLE_FORWARD * size.width, y: hand.y + MUZZLE_HEIGHT * size.height };
}

/** 🐠️ The point of the taut line of `shot` that lies `rope` pixels from the hook towards the muzzle. */
function lined(shot: Shot, rope: number): Point {
  const share = rope / shot.length;
  return { x: shot.hook.x + (shot.muzzle.x - shot.hook.x) * share, y: shot.hook.y + (shot.muzzle.y - shot.hook.y) * share };
}

/** 🎏️ The actor at rest on the taut line of `shot` with `rope` pixels of rope left: with the whole length of the shot it is where it stood when the hook bit, which is how every haul begins. */
export function haulOf(shot: Shot, rope: number, size: Size): Haul {
  const hand = lined(shot, rope);
  return hung(shot, rope, hand, hand, size);
}

/** 🚠️ One tick of reeling straight up the rope of `shot`, `ticks` ticks after the haul began: the rope shortens by `ZIP_SPEED ÷ 64`, gathered over `ZIP_RAMP` ticks, but not below `REEL_LEAST` heights, where the hoist onto the perch begins; a rope that is shorter already keeps its length. The hands stay on the taut line. */
export function zipStep(shot: Shot, haul: Haul, ticks: Ticks, size: Size): Haul {
  const rope = Math.max(haul.rope - gathered(ZIP_SPEED, ticks, ZIP_RAMP) / TICKS_PER_SECOND, Math.min(REEL_LEAST * size.height, haul.rope));
  return hung(shot, rope, lined(shot, rope), haul.hand, size);
}

/** 🎪️ One tick of swinging on the rope of `shot` while it is reeled in, `ticks` ticks after the haul began: the hands are the pendulum of the swing module's `reelStep` under the hook, down to a rope of `REEL_LEAST` heights. */
export function swayStep(shot: Shot, haul: Haul, ticks: Ticks, size: Size): Haul {
  const reel = reelStep(shot.hook, haul.hand, haul.before, haul.rope, REEL_LEAST * size.height, ticks);
  return hung(shot, reel.length, reel.bob, haul.hand, size);
}

/** 🎬️ One tick of the haul up the rope of `shot`, `ticks` ticks after it began: {@link zipStep} on a rope that is reeled in straight, {@link swayStep} on one that swings. */
export function haulStep(shot: Shot, haul: Haul, ticks: Ticks, size: Size): Haul {
  return shot.reel === "zip" ? zipStep(shot, haul, ticks, size) : swayStep(shot, haul, ticks, size);
}

/** 🧭️ How many ticks the haul up the whole rope of `shot` takes until `REEL_LEAST` heights of rope are left; 0 for a rope no longer than that. */
export function haulTicks(shot: Shot, size: Size): Ticks {
  const least = REEL_LEAST * size.height;
  let haul = haulOf(shot, shot.length, size);
  let ticks = 0;
  while (haul.rope > least && ticks < PATIENCE) {
    haul = haulStep(shot, haul, ticks, size);
    ticks++;
  }
  return ticks;
}

/** 🏕️ Where an actor of `size` stands once it has hoisted itself up the rope of `shot` onto `perch`: half a width and `MANTLE_INSET` from the hook towards the middle of the perch, never beyond its ends. */
export function landingFor(shot: Shot, perch: Perch, size: Size): Point {
  const inward = shot.hook.x <= (perch.x0 + perch.x1) / 2 ? 1 : -1;
  return { x: clamp(shot.hook.x + inward * (size.width / 2 + MANTLE_INSET), perch.x0, perch.x1), y: perch.y };
}
//#endregion 🔖️Rope

//#region 🔖️Routes
/** 🛤️ Whether a ladder stands on `perch`: on its surface, at its height, with its foot between its ends. */
function rests(ladder: LadderStand, perch: Perch): boolean {
  return perch.surface === ladder.surface && perch.y === ladder.foot.y && perch.x0 <= ladder.foot.x && ladder.foot.x <= perch.x1;
}

/** 🚏️ The way over a standing ladder from `from` to `to`, or `null`: up when it stands on `from` and leans against a pitch that `to` crowns, down the other way round. */
function ridden(ladder: LadderStand, from: Perch, to: Perch, pitches: readonly Pitch[], size: Size): Leg | null {
  for (const pitch of pitches) {
    if (pitch.wall !== ladder.wall || pitch.side !== ladder.side) continue;
    if (rests(ladder, from) && crowns(to, pitch, size)) return { means: "ladder", at: ladder.foot.x, ladder, up: true };
    if (crowns(from, pitch, size) && rests(ladder, to)) return { means: "ladder", at: ledgeOf(pitch, size), ladder, up: false };
  }
  return null;
}

/** 🧱️ The way over the wall line of `pitch` from `from` to `to`, or `null`: from a hold at the foot of `pitch` up the line to the rim of the nearest pitch on the way that `to` crowns, or over the rim of `pitch` that `from` crowns down the line to the nearest pitch on the way where `to` passes its foot — when `grip` lasts for the whole way ({@link wallPath}, {@link wallCost}). On a single pitch that is the climb and the mantle or the hang at its rim. */
function scaled(pitch: Pitch, from: Perch, to: Perch, size: Size, grip: number, pitches: readonly Pitch[]): Leg | null {
  const hold = gripFor(from, pitch, size);
  if (hold === null) return null;
  const chain = chainOf(pitch, pitches, size);
  const start = chain.indexOf(pitch);
  if (hold.over) {
    for (let at = start; at < chain.length; at++) {
      const landing = gripFor(to, chain[at]!, size);
      if (landing === null || landing.over) continue;
      return wallCost(wallPath(chain, start, "hang", hold.x, hold.y, at, landing.y, false, size)) <= grip ? { means: "wall", at: ledgeOf(pitch, size), pitch, hold, exit: chain[at]!, goal: landing.y } : null;
    }
    return null;
  }
  for (let at = start; at >= 0; at--) {
    if (!crowns(to, chain[at]!, size)) continue;
    const rim = rimOf(chain[at]!, size);
    return wallCost(wallPath(chain, start, "grab", hold.x, hold.y, at, rim, true, size)) <= grip ? { means: "wall", at: hold.x, pitch, hold, exit: chain[at]!, goal: rim } : null;
  }
  return null;
}

/** 🔙️ Where an actor of `size` that stands at `x` on `from` tries to shoot at `to` from, in this order: where it stands; the points of `from` nearest to either end and to the middle of `to`; then back from either end of `to`, a width farther at a time, as long as a rope could still reach across (`ROPE_LONG` heights) — so a shooter whose line the element under `to` blocks steps back until it clears the corner. */
function aims(x: number, from: Perch, to: Perch, size: Size): number[] {
  const stands = [x, clamp(to.x0, from.x0, from.x1), clamp(to.x1, from.x0, from.x1), clamp((to.x0 + to.x1) / 2, from.x0, from.x1)];
  for (let back = size.width; back <= ROPE_LONG * size.height; back = back + size.width) stands.push(clamp(to.x0 - back, from.x0, from.x1), clamp(to.x1 + back, from.x0, from.x1));
  return stands;
}

/** 🗾️ The ways an actor of `size` that stands at `x` on `from` can take to `to` with its `gear` and its `grip`, most preferred first, or `null` when there is none: a ladder that stands between the two (for whoever owns any gear but a parachute), a wall line (gear `climb`; one leg per pitch it takes hold of), a ladder of its own raised against a wall `to` crowns (gear `ladder`, and only where none stands), a shot of its grappling gun at the edge of `to` (gear `grapple`) from the first place of {@link aims} it has one from: where it stands when that works. Every leg is clear of the keep-outs; whether it is clear of the other actors is the stage's to judge. */
export function routeOf(x: number, from: Perch, to: Perch, gear: readonly Gear[], size: Size, grip: number, pitches: readonly Pitch[], ladders: readonly LadderStand[], keepouts: readonly Rect[]): Leg[] | null {
  const legs: Leg[] = [];
  if (gear.includes("climb") || gear.includes("ladder") || gear.includes("grapple")) {
    for (const ladder of ladders) {
      const leg = ridden(ladder, from, to, pitches, size);
      if (leg !== null) legs.push(leg);
    }
  }
  const joined = legs.length > 0;
  if (gear.includes("climb")) {
    for (const pitch of pitches) {
      const leg = scaled(pitch, from, to, size, grip, pitches);
      if (leg !== null) legs.push(leg);
    }
  }
  if (gear.includes("ladder") && !joined) {
    for (const pitch of pitches) {
      const ladder = ladderFor(from, to, pitch, keepouts, size);
      if (ladder !== null) legs.push({ means: "raise", at: ladder.foot.x, ladder });
    }
  }
  if (gear.includes("grapple")) {
    for (const stand of aims(x, from, to, size)) {
      const shot = shotFor({ x: stand, y: from.y }, [to], keepouts, size);
      if (shot === null) continue;
      legs.push({ means: "grapple", at: stand, shot });
      break;
    }
  }
  return legs.length > 0 ? legs : null;
}
//#endregion 🔖️Routes
