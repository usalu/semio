/** 🪢️ Everything that hangs: a pet dangling from the learner's hand, a pet under its parachute and a pet on a rope that is reeled in — one constrained step for all three, the spring that carries the grip, the throw at the release, and the descent under a canopy.
 *
 * Positions are stage pixels with the y axis pointing down, velocities are pixels per second and time is whole ticks
 * of 1/64 s. Everything is built from `+ − × ÷`, `sqrt` and comparisons (and {@link atanTurns}, {@link sinTurns},
 * {@link springStep}, which are built from the same), evaluated in the order written here with literal constants, so
 * the Rust twin reproduces every result bit for bit.
 *
 * The core is {@link swingStep}: one Verlet step of a point on a rod or a rope below an anchor that may move. The
 * constraint force acts along the radius of the tick before and its size is the smaller root of a quadratic (SHAKE),
 * which makes the step second order and symplectic: a free swing keeps its amplitude for minutes and follows the
 * pendulum's differential equation closely. The usual shortcut — step freely, then pull the point back onto the
 * circle — is only first order and eats a quarter of the energy of a swing in its first half period at this tick
 * length; it is deliberately absent. A swing dies by the `damping` factor alone, which is what makes it tunable.
 *
 * A held pet, per tick: `hangStep(hang, pointer, length)`, then `leanOf(hang.grip, hang.bob, length)` for the tilt of
 * its drawing; at the release `throwVelocity(hang, samples)` with the last pointer positions, one per tick. A falling
 * pet, per tick: `chuteOpens(vy, height)`; {@link CHUTE_REFLEX} ticks of plain falling later `canopyOf(feet, vx, vy,
 * chute)`, then `chuteStep(canopy, chute, target, remaining, wind)` until it lands. A pet on a rope: `reelStep`.
 *
 * @see https://en.wikipedia.org/wiki/Constraint_(computational_chemistry)#The_SHAKE_algorithm — SHAKE (Ryckaert, Ciccotti and Berendsen 1977), the constraint step of {@link swingStep}
 * @see https://theorangeduck.com/page/spring-roll-call#critical — the half-life form of a critically damped spring, behind the follow spring
 * @see https://android.googlesource.com/platform/frameworks/native/+/master/libs/input/VelocityTracker.cpp — the least-squares release velocity of a touch, the model of {@link ringVelocity}
 * @see ../🎞️animation/🟦️.ts — `springStep`
 * @see ../🏞️terrain/🟦️.ts — `GRAVITY`, `FALL_SPEED`
 * @see ../📐️trigonometry/🟦️.ts — `atanTurns`, `sinTurns`
 * @see ../../🧬️schema/🟦️.ts — `Point`, `Ticks`, `Turns`
 */

import type { Canopy, Grip, Hang, Point, Ticks, Turns } from "../../🧬️schema/🟦️.ts";
import { springStep } from "../🎞️animation/🟦️.ts";
import { FALL_SPEED, GRAVITY } from "../🏞️terrain/🟦️.ts";
import { atanTurns, sinTurns } from "../📐️trigonometry/🟦️.ts";

//#region 🔖️Constants
/** 📏️ The rod of a held pet as a share of its height: the feet hang 0.8 heights below the grip (38 px for a pet of 48). */
export const HANG_ROD = 0.8;

/** 🧲️ The gravity a held pet swings under, in pixels per second squared: four times `GRAVITY`, because the hand that drags it accelerates many times harder than a fall; an ordinary drag then leans it by 15° to 50° instead of pinning it to the cone (a flick still reaches it), and a swing on a rod of 38 px takes 0.46 s. */
export const HANG_GRAVITY = 7200;

/** 🍯️ The share of its speed a held pet keeps per tick: its swing halves every 13.5 ticks and has died below 3° after about two seconds. */
export const HANG_DAMPING = 0.95;

/** 🍦️ The cosine of the widest lean of a held pet: its feet stay at least half a rod below the grip, so it never leans beyond 60°. */
export const HANG_CONE = 0.5;

/** 🧷️ The stiffness of the spring that carries the grip towards the pointer, in 1/s²: with {@link FOLLOW_DAMPING} a critically damped spring of a half-life of 0.06 s. A resting grip has covered half of a jump of its target after 4 ticks and never overshoots; behind a pointer in steady motion it trails by 70 ms of travel. */
export const FOLLOW_STIFFNESS = 534;

/** 🛋️ The damping of the spring that carries the grip, in 1/s, the companion of {@link FOLLOW_STIFFNESS}. */
export const FOLLOW_DAMPING = 46;

/** ⚖️ The weights of the last seven pointer samples, oldest first, whose sum divided by {@link RELEASE_DIVISOR} is the slope at the newest sample of the parabola fitted to all seven by least squares, in pixels per tick. They add up to 0. */
export const RELEASE_WEIGHTS = [7, -2, -7, -8, -5, 2, 13] as const;

/** ➗️ What the weighted sum of {@link RELEASE_WEIGHTS} is divided by. */
export const RELEASE_DIVISOR = 28;

/** 🥶️ How many equal samples at the end mean the pointer has stopped (47 ms): its velocity then counts as zero. */
export const RELEASE_STALE = 3;

/** 🤝️ How much of the pointer's velocity the grip has not caught up with yet is handed to a thrown pet on top of its own. */
export const THROW_SHARE = 0.5;

/** 🐌️ The slowest throw in pixels per second; below it a release is a plain letting go, without velocity. */
export const THROW_LEAST = 70;

/** 🚀️ The fastest throw in pixels per second; a faster release keeps its direction and is slowed to this. A throw straight up at this speed rises 114 px. */
export const THROW_MOST = 640;

/** 🎈️ The fastest a throw may rise, in pixels per second; the upward part beyond it is cut. */
export const THROW_RISE = 520;

/** 💥️ The impact speed in pixels per second above which a landing is hard: reached from rest after a fall of 100 px, about two body heights. A parachute opens for it. */
export const HARD_LANDING = 600;

/** 🚪️ The least falling speed in pixels per second at which a parachute opens: reached after 9 ticks (20 px) of falling from rest, so a pet that merely steps off a low perch never flashes a canopy. */
export const CHUTE_OPENING = 240;

/** 🪟️ The least height above the landing in pixels a parachute needs to be of any use. */
export const CHUTE_HEADROOM = 56;

/** ⏱️ How many ticks a pet keeps falling after it decided to open its parachute — it looks up and tugs the cord. The most sensitive number of the descent (MECH §2.2): every tick of it is a tick of falling at the full gravity. */
export const CHUTE_REFLEX = 6;

/** 📉️ The share of the gap to its terminal speed a canopy keeps per tick: `exp(−1 ÷ (64 × 0.18))`, a time constant of 0.18 s. */
export const CHUTE_FACTOR = 0.9168553557320289;

/** 🍂️ The terminal speed under a canopy in pixels per second per pixel of body height: 96 px/s for a pet of 48. */
export const CHUTE_DESCENT = 2;

/** 🧭️ How hard a canopy steers towards its landing spot: the sideways speed it wants per pixel of distance, in 1/s. */
export const CHUTE_STEER_GAIN = 1.2;

/** ⛵️ The fastest a canopy drifts sideways on purpose, in pixels per second per pixel of body height. */
export const CHUTE_STEER_SPEED = 1.2;

/** 🛞️ The share of the gap to the wanted sideways speed a canopy closes per tick. */
export const CHUTE_STEER_EASE = 0.06;

/** 🧵️ The cords of a parachute as a share of the body height: the pet hangs 0.9 heights below the canopy. */
export const CHUTE_ROD = 0.9;

/** 🪶️ The gravity a pet sways under below its canopy, in pixels per second squared: half of `GRAVITY`, a sway of 1.4 s. */
export const CHUTE_GRAVITY = 900;

/** 🌫️ The share of its speed a pet under a canopy keeps per tick; it acts on the speed over the stage, so a canopy that drifts sideways drags its pet behind it, by 9° at its fastest drift. */
export const CHUTE_DAMPING = 0.97;

/** 🛬️ The height above the landing, as a share of the body height, below which a canopy flares: the descent slows evenly to half of its speed at the touch. */
export const CHUTE_FLARE = 0.25;

/** 💨️ The strongest sideways push of the wind on a canopy in pixels per second. */
export const CHUTE_WIND = 10;

/** 🎐️ How fast the wind swings back and forth, in turns per second. */
export const CHUTE_WIND_RATE = 0.35;

/** 🎣️ How fast a swinging pet reels its rope in, in pixels per second, once it has spun up. */
export const REEL_SPEED = 60;

/** 📈️ Over how many ticks the reel spins up to {@link REEL_SPEED}. */
export const REEL_RAMP = 10;

/** 🤏️ The shortest rope a pet swings on, as a share of its height; reeling stops there, because a pendulum that keeps shortening spins up without bound. */
export const REEL_LEAST = 0.9;

/** 🚦️ The fastest a pet moves on a reeled rope, in pixels per second: 8.125 px per tick. */
export const REEL_CAP = 520;

/** 🕰️ The share of its speed a pet on a rope keeps per tick: a swing halves in three seconds. */
export const REEL_DAMPING = 0.9965;

const TAUT = 1.000001;
//#endregion 🔖️Constants

//#region 🔖️Types
/** 🎒️ The measures of a species' parachute in pixels and pixels per second: its terminal speed, the fastest sideways drift it steers with, the height it flares at and the length of its cords. */
export type Chute = { readonly terminal: number; readonly reach: number; readonly flare: number; readonly length: number };

/** 🧶️ A pet on a rope after one tick of reeling: where it is and how long the rope is now. */
export type Reel = { readonly bob: Point; readonly length: number };
//#endregion 🔖️Types

//#region 🔖️Swing
/** ⛓️ Where a point that hangs on a rod or a rope of `length` is one tick later, while its anchor moves from `anchorBefore` to `anchorNow`; `bob` is where the point is and `previous` where it was a tick ago.
 *
 * The point first flies freely: it keeps `damping` of its step and falls by `gravity ÷ 4096` (pixels per second
 * squared times the square of a tick). Then it is pulled back along the radius it had before the step, from
 * `anchorBefore` to `bob`, by the smaller root of the quadratic that puts it at `length` from `anchorNow` — so the
 * result lies exactly that far from the anchor whenever such a point exists, and as near to it as that line comes
 * otherwise (an anchor that jumped sideways by more than the rod). A slack rope (`rope` set and the free point
 * within `length` of the anchor) pulls nothing; a rod always does. Because the pull is central, shortening `length`
 * from tick to tick keeps the angular momentum about the anchor, like a pendulum on a shortened string. The damping
 * acts on the step of the tick before, half a tick earlier than a drag would: the swing is that of a pendulum with
 * the drag `128·(1 − damping) ÷ (1 + damping)` per second under a gravity `2 ÷ (1 + damping)` times as strong.
 */
export function swingStep(anchorBefore: Point, anchorNow: Point, bob: Point, previous: Point, length: number, gravity: number, damping: number, rope: boolean): Point {
  const freeX = bob.x + (bob.x - previous.x) * damping;
  const freeY = bob.y + (bob.y - previous.y) * damping + gravity / 4096;
  const radiusX = bob.x - anchorBefore.x;
  const radiusY = bob.y - anchorBefore.y;
  const reachX = freeX - anchorNow.x;
  const reachY = freeY - anchorNow.y;
  const radius = radiusX * radiusX + radiusY * radiusY;
  const along = reachX * radiusX + reachY * radiusY;
  const reach = reachX * reachX + reachY * reachY;
  if (rope && reach <= length * length) return { x: freeX, y: freeY };
  const root = along * along - radius * (reach - length * length);
  const pull = (along - Math.sqrt(root > 0 ? root : 0)) / (radius > 1e-9 ? radius : 1e-9);
  return { x: freeX - pull * radiusX, y: freeY - pull * radiusY };
}
//#endregion 🔖️Swing

//#region 🔖️Hand
/** 🔻️ The feet of a held pet kept on their rod and inside the cone below the grip: a point whose direction from `anchor` leans beyond {@link HANG_CONE} is put on the edge of the cone on its own side at `length`, a point farther than `length` is drawn in along its direction, and every other point — one on the grip itself too — is returned as it is. */
export function coneClamp(anchor: Point, bob: Point, length: number): Point {
  const dx = bob.x - anchor.x;
  const dy = bob.y - anchor.y;
  const span = dx * dx + dy * dy;
  if (dy < 0 || dy * dy < HANG_CONE * HANG_CONE * span) {
    const drop = HANG_CONE * length;
    const side = Math.sqrt(length * length - drop * drop);
    return { x: dx < 0 ? anchor.x - side : anchor.x + side, y: anchor.y + drop };
  }
  if (span <= length * length * TAUT) return bob;
  const taut = length / Math.sqrt(span);
  return { x: anchor.x + dx * taut, y: anchor.y + dy * taut };
}

/** 👣️ The grip one tick later on its way to `target` (the pointer, held inside the stage): {@link springStep} on each axis with {@link FOLLOW_STIFFNESS} and {@link FOLLOW_DAMPING}. */
export function followStep(grip: Grip, target: Point): Grip {
  const x = springStep(grip.x, grip.vx, target.x, FOLLOW_STIFFNESS, FOLLOW_DAMPING);
  const y = springStep(grip.y, grip.vy, target.y, FOLLOW_STIFFNESS, FOLLOW_DAMPING);
  return { x: x.position, y: y.position, vx: x.velocity, vy: y.velocity };
}

/** 🪝️ A pet at the moment it is picked up: gripped `length` above its feet, everything at rest. The grip then travels to the pointer through {@link followStep}, which is the lift. */
export function hangOf(feet: Point, length: number): Hang {
  return { grip: { x: feet.x, y: feet.y - length, vx: 0, vy: 0 }, bob: feet, previous: feet };
}

/** 🦧️ A held pet one tick later: the grip follows `target`, the feet swing below it on a rod of `length` under {@link HANG_GRAVITY} and {@link HANG_DAMPING}, and {@link coneClamp} keeps them on the rod and inside the cone. */
export function hangStep(hang: Hang, target: Point, length: number): Hang {
  const grip = followStep(hang.grip, target);
  const swung = swingStep(hang.grip, grip, hang.bob, hang.previous, length, HANG_GRAVITY, HANG_DAMPING, false);
  return { grip, bob: coneClamp(grip, swung, length), previous: hang.bob };
}

/** 🗼️ The tilt of a body that hangs from `anchor` with its feet at `bob`, in turns: the rotation that carries a body hanging straight down onto it, in the sense of the rig (positive turns the x axis towards the y axis, clockwise on a screen). 0 straight down, negative while the feet trail to the right, positive to the left; off by at most 2e-6 turns. */
export function leanOf(anchor: Point, bob: Point, length: number): Turns {
  return atanTurns((anchor.x - bob.x) / length, (bob.y - anchor.y) / length);
}
//#endregion 🔖️Hand

//#region 🔖️Release
/** 🧊️ Whether the last {@link RELEASE_STALE} samples are one point: the pointer has stopped. Fewer samples have not stopped. */
function stale(samples: readonly Point[]): boolean {
  const count = samples.length;
  if (count < RELEASE_STALE) return false;
  const newest = samples[count - 1]!;
  for (let back = 2; back <= RELEASE_STALE; back++) {
    const sample = samples[count - back]!;
    if (sample.x !== newest.x || sample.y !== newest.y) return false;
  }
  return true;
}

/** 💍️ The velocity of the pointer at its newest sample in pixels per second, from its positions at the last seven ticks, oldest first: the slope of the parabola fitted to them by least squares, which is the sum of {@link RELEASE_WEIGHTS} times the samples, divided by {@link RELEASE_DIVISOR}, times 64.
 *
 * Every sample is taken relative to the newest one before it is weighted (the weights add up to 0, so the fit is
 * the same), which makes a pointer at rest yield exactly zero wherever it rests. When fewer than seven samples are
 * given the oldest one stands in for the missing ones — the pointer is taken to have rested there — and no sample
 * at all is no velocity. Samples before the last seven are ignored.
 */
export function ringVelocity(samples: readonly Point[]): Point {
  const count = samples.length;
  if (count === 0) return { x: 0, y: 0 };
  const newest = samples[count - 1]!;
  const first = count - RELEASE_WEIGHTS.length;
  let x = 0;
  let y = 0;
  for (let index = 0; index < RELEASE_WEIGHTS.length - 1; index++) {
    const sample = samples[first + index < 0 ? 0 : first + index]!;
    x = x + RELEASE_WEIGHTS[index]! * (sample.x - newest.x);
    y = y + RELEASE_WEIGHTS[index]! * (sample.y - newest.y);
  }
  return { x: (x * 64) / RELEASE_DIVISOR, y: (y * 64) / RELEASE_DIVISOR };
}

/** 🤾️ The throw a velocity at the release becomes, in pixels per second: nothing below {@link THROW_LEAST}, slowed to {@link THROW_MOST} along its direction above it, and then never rising faster than {@link THROW_RISE}. */
export function throwOf(velocity: Point): Point {
  const speed = Math.sqrt(velocity.x * velocity.x + velocity.y * velocity.y);
  if (speed < THROW_LEAST) return { x: 0, y: 0 };
  const scale = speed > THROW_MOST ? THROW_MOST / speed : 1;
  const y = velocity.y * scale;
  return { x: velocity.x * scale, y: y < 0 - THROW_RISE ? 0 - THROW_RISE : y };
}

/** 🖐️ The throw of the pointer alone at the release, from its last samples, one per tick, oldest first: {@link throwOf} of {@link ringVelocity}, and nothing when the pointer had stopped (its last {@link RELEASE_STALE} samples are equal). */
export function releaseVelocity(samples: readonly Point[]): Point {
  return stale(samples) ? { x: 0, y: 0 } : throwOf(ringVelocity(samples));
}

/** 🥏️ The throw of a held pet at the release: the velocity of its feet (their last step times 64) plus {@link THROW_SHARE} of what the pointer does and the grip has not caught up with yet (the pointer's velocity, zero once it stopped, less the grip's), through {@link throwOf}. */
export function throwVelocity(hang: Hang, samples: readonly Point[]): Point {
  const ring = stale(samples) ? { x: 0, y: 0 } : ringVelocity(samples);
  return throwOf({ x: (hang.bob.x - hang.previous.x) * 64 + THROW_SHARE * (ring.x - hang.grip.vx), y: (hang.bob.y - hang.previous.y) * 64 + THROW_SHARE * (ring.y - hang.grip.vy) });
}
//#endregion 🔖️Release

//#region 🔖️Parachute
/** ☄️ The speed in pixels per second at which something that moves at `vy` now would hit a landing `height` pixels below without a parachute: `sqrt(vy² + 2 × GRAVITY × height)`, never beyond `FALL_SPEED`; a landing that is not below counts as reached. */
export function impactSpeed(vy: number, height: number): number {
  const speed = Math.sqrt(vy * vy + 2 * GRAVITY * (height > 0 ? height : 0));
  return speed < FALL_SPEED ? speed : FALL_SPEED;
}

/** 🪂️ Whether a pet that falls at `vy` with a landing `height` pixels below its feet opens its parachute now: it falls at {@link CHUTE_OPENING} or faster, has {@link CHUTE_HEADROOM} or more below it, and would land harder than {@link HARD_LANDING}. From rest that is every drop of 102 px or more. */
export function chuteOpens(vy: number, height: number): boolean {
  return vy >= CHUTE_OPENING && height >= CHUTE_HEADROOM && impactSpeed(vy, height) > HARD_LANDING;
}

/** 🧮️ The measures of the parachute of a species of `height` pixels: {@link CHUTE_DESCENT}, {@link CHUTE_STEER_SPEED}, {@link CHUTE_FLARE} and {@link CHUTE_ROD} times that height. */
export function chuteOf(height: number): Chute {
  return { terminal: CHUTE_DESCENT * height, reach: CHUTE_STEER_SPEED * height, flare: CHUTE_FLARE * height, length: CHUTE_ROD * height };
}

/** 🌂️ A parachute at the tick it opens above a pet whose feet are at `feet` and move at `vx`, `vy`: the canopy holds the cords their length above the feet and moves as the pet does, so nothing sways yet. */
export function canopyOf(feet: Point, vx: number, vy: number, chute: Chute): Canopy {
  return { x: feet.x, y: feet.y - chute.length, vx, vy, bob: feet, previous: { x: feet.x - vx / 64, y: feet.y - vy / 64 } };
}

/** 🦅️ The share of its descent speed a canopy keeps `remaining` pixels above the landing: all of it at `flare` and above, half of it at the touch and below, evenly in between. */
export function flareOf(remaining: number, flare: number): number {
  if (remaining >= flare) return 1;
  if (remaining <= 0) return 0.5;
  return 0.5 + (0.5 * remaining) / flare;
}

/** 🌬️ The sideways push of the wind on a canopy in pixels per second, `ticks` after the stage began: {@link CHUTE_WIND} times the sine of {@link CHUTE_WIND_RATE} turns per second, shifted by `phase` turns so every pet has a wind of its own. */
export function chuteWind(ticks: Ticks, phase: Turns): number {
  return CHUTE_WIND * sinTurns((CHUTE_WIND_RATE * ticks) / 64 + phase);
}

/** 🕊️ A pet under its open parachute one tick later; `target` is the x it steers for, `remaining` the height of its feet above the landing before the step, and `wind` the push of {@link chuteWind}.
 *
 * The descent closes the gap to the terminal speed by {@link CHUTE_FACTOR} (`vy ← terminal + (vy − terminal) ×
 * factor`, the exact step of a linear drag, stable for every speed). The sideways speed closes {@link
 * CHUTE_STEER_EASE} of the gap to the speed it wants, {@link CHUTE_STEER_GAIN} times the distance to the target
 * held within the reach of the chute. The canopy then moves by `vx + wind` and by `vy` times {@link flareOf}, both
 * divided by 64, and the pet swings below it by {@link swingStep} on the cords as a rod, under {@link CHUTE_GRAVITY}
 * and {@link CHUTE_DAMPING}. The speed of the touch is the last step of the canopy times 64.
 */
export function chuteStep(canopy: Canopy, chute: Chute, target: number, remaining: number, wind: number): Canopy {
  const vy = chute.terminal + (canopy.vy - chute.terminal) * CHUTE_FACTOR;
  const pull = CHUTE_STEER_GAIN * (target - canopy.x);
  const wanted = pull < 0 - chute.reach ? 0 - chute.reach : pull > chute.reach ? chute.reach : pull;
  const vx = canopy.vx + (wanted - canopy.vx) * CHUTE_STEER_EASE;
  const x = canopy.x + (vx + wind) / 64;
  const y = canopy.y + (vy * flareOf(remaining, chute.flare)) / 64;
  return { x, y, vx, vy, bob: swingStep(canopy, { x, y }, canopy.bob, canopy.previous, chute.length, CHUTE_GRAVITY, CHUTE_DAMPING, false), previous: canopy.bob };
}
//#endregion 🔖️Parachute

//#region 🔖️Reel
/** 🪀️ A pet that swings on a rope hooked at `anchor` while it reels the rope in, one tick later; `age` counts the ticks reeled before this one and `least` is the shortest rope it swings on ({@link REEL_LEAST} times its height).
 *
 * The rope shortens by the reel speed of the tick — {@link REEL_SPEED} once {@link REEL_RAMP} ticks have passed,
 * the share `(age + 1) ÷ REEL_RAMP` of it before — but never below `least`, and a rope that is shorter than that
 * already keeps its length. The pet then swings by {@link swingStep} on the shortened rope under `GRAVITY` and
 * {@link REEL_DAMPING}; a step longer than {@link REEL_CAP} ÷ 64 pixels is cut to that length along its direction
 * and, where that leaves the pet beyond the rope, drawn in to the rope's length, so a step is never longer than the
 * cap plus what the reel takes in. A pet reeled from 154 px at 85° reaches 722 px/s without the cap and 520 px/s
 * with it; without the least length the shortening pendulum spins up without bound (MECH §3.3).
 */
export function reelStep(anchor: Point, bob: Point, previous: Point, length: number, least: number, age: Ticks): Reel {
  const speed = age + 1 < REEL_RAMP ? (REEL_SPEED * (age + 1)) / REEL_RAMP : REEL_SPEED;
  const floor = length < least ? length : least;
  const wound = length - speed / 64;
  const rope = wound < floor ? floor : wound;
  const swung = swingStep(anchor, anchor, bob, previous, rope, GRAVITY, REEL_DAMPING, true);
  const dx = swung.x - bob.x;
  const dy = swung.y - bob.y;
  const step = dx * dx + dy * dy;
  const cap = REEL_CAP / 64;
  if (step <= cap * cap) return { bob: swung, length: rope };
  const slowed = cap / Math.sqrt(step);
  const x = bob.x + dx * slowed;
  const y = bob.y + dy * slowed;
  const span = (x - anchor.x) * (x - anchor.x) + (y - anchor.y) * (y - anchor.y);
  if (span <= rope * rope) return { bob: { x, y }, length: rope };
  const taut = rope / Math.sqrt(span);
  return { bob: { x: anchor.x + (x - anchor.x) * taut, y: anchor.y + (y - anchor.y) * taut }, length: rope };
}
//#endregion 🔖️Reel
