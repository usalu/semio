/** 🏞️ Where pets stand, walk, fall and hop: perches cut out of the surveyed surfaces, strides along them, the swept one-way landing and ballistic flight in ticks.
 *
 * Positions are viewport pixels with the y axis pointing down, velocities are pixels per second and time is whole ticks
 * of 1/64 s. Everything is built from `+ − × ÷`, `sqrt`, `abs`, `floor`, `min`, `max` and comparisons, evaluated in
 * the order written here, so the Rust twin reproduces every result bit for bit.
 *
 * Flight is integrated per tick by semi-implicit Euler, velocity first: `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`,
 * then `y ← y + vy ÷ 64` and `x ← x + vx ÷ 64`. Below the terminal speed the height after `k` ticks is therefore
 * `y + vy·k ÷ 64 + GRAVITY·k·(k + 1) ÷ 8192`. {@link hopOf} solves this sum, not the continuous arc, for the launch
 * velocity, so a hop it grants ends on its target after exactly its ticks, and {@link hopStep} writes the target
 * itself on the last tick, which removes the rounding the sum has gathered on the way.
 *
 * A hop in flight, per tick: `hopStep(x, y, vx, vy, to, left)` with the ticks left to fly, counting the one being
 * stepped (`hop.ticks` on the first tick, 1 on the last), then `landingOf(perches, next.x, y, next.y)`; a landing is
 * never found while the actor rises, because perches are one-way platforms. {@link hopLanding} runs exactly this loop
 * ahead of time.
 *
 * @see ../../🧬️schema/🟦️.ts — `Point`, `Rect`, `Surface`, `Perch`, `TICKS_PER_SECOND`
 * @see ./🦀️.rs — the Rust twin
 */

import { type Perch, type Point, type Rect, type Surface, TICKS_PER_SECOND, type Ticks } from "../../🧬️schema/🟦️.ts";

//#region 🔖️Constants
/** 🍎️ The downward acceleration of everything in flight, in pixels per second squared: a drop of 100 px takes a third of a second. */
export const GRAVITY = 1800;

/** 🪨️ The terminal speed of a fall in pixels per second, reached after half a second or 225 px. */
export const FALL_SPEED = 900;

/** 🌈️ How far the apex of a hop rises above the higher of its two ends at least, in pixels. */
export const HOP_CLEARANCE = 12;

/** 📐️ The least apex rise of a hop per pixel of horizontal distance: 3/8 launches a level hop at about 56°. */
export const HOP_STEEPNESS = 0.375;

/** 🏔️ The highest apex of a hop above its launch in pixels; a perch more than `HOP_HEIGHT − HOP_CLEARANCE` above is too high. */
export const HOP_HEIGHT = 84;

/** 📏️ The widest hop in pixels of horizontal distance. */
export const HOP_DISTANCE = 160;

/** ⏳️ The longest hop in ticks (0.75 s). */
export const HOP_TICKS = 48;
//#endregion 🔖️Constants

//#region 🔖️Types
/** 🪂️ A height and a vertical speed: what one tick of falling yields. */
export type Fall = { readonly y: number; readonly vy: number };

/** 🦘️ The launch of a hop: its velocity in pixels per second and the ticks it stays in the air. */
export type Hop = { readonly vx: number; readonly vy: number; readonly ticks: Ticks };

/** 🕊️ An actor in the air: where its feet are and how fast they move, in pixels and pixels per second. */
export type Flight = { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number };

type Stretch = readonly [number, number];
//#endregion 🔖️Types

//#region 🔖️Perches
/** ⛔️ Whether `keepout` reaches into the headroom band `[top, bottom)` above a surface; a box without area blocks nothing. */
function blocks(keepout: Rect, top: number, bottom: number): boolean {
  return keepout.width > 0 && Math.max(top, keepout.y) < Math.min(bottom, keepout.y + keepout.height);
}

/** ✂️ `stretches` without the open interval `(low, high)`, still in ascending order; what only touches an end takes nothing away. */
function cut(stretches: readonly Stretch[], low: number, high: number): Stretch[] {
  const kept: Stretch[] = [];
  for (const [x0, x1] of stretches) {
    if (high <= x0 || low >= x1) kept.push([x0, x1]);
    else {
      if (low > x0) kept.push([x0, low]);
      if (high < x1) kept.push([high, x1]);
    }
  }
  return kept;
}

/** 🪺️ The free stretches of every surface with `clearance ≤ y ≤ height`: its extent clipped to `[0, width]`, minus the x-extent of every keep-out that reaches into the band `[y − clearance, y)` above it (keep-outs in list order), keeping stretches at least `minimum` wide; in surface order, then ascending x. */
export function perchesOf(surfaces: readonly Surface[], keepouts: readonly Rect[], width: number, height: number, clearance: number, minimum: number): Perch[] {
  const perches: Perch[] = [];
  for (const surface of surfaces) {
    if (surface.y < clearance || surface.y > height) continue;
    const left = Math.max(surface.x0, 0);
    const right = Math.min(surface.x1, width);
    if (right <= left) continue;
    let stretches: Stretch[] = [[left, right]];
    for (const keepout of keepouts) if (blocks(keepout, surface.y - clearance, surface.y)) stretches = cut(stretches, keepout.x, keepout.x + keepout.width);
    for (const [x0, x1] of stretches) if (x1 - x0 >= minimum) perches.push({ surface: surface.id, x0, x1, y: surface.y });
  }
  return perches;
}

/** 📍️ The first perch of `surface` that carries `x`, both ends included; `null` ≙ Rust `None`. */
export function perchAt(perches: readonly Perch[], surface: string, x: number): Perch | null {
  for (const perch of perches) if (perch.surface === surface && perch.x0 <= x && x <= perch.x1) return perch;
  return null;
}

/** 🧲️ The perch whose nearest point is closest to `(x, y)` by squared distance, the first one among equals; `null` without perches. */
export function nearestPerch(perches: readonly Perch[], x: number, y: number): Perch | null {
  let nearest: Perch | null = null;
  let least = Infinity;
  for (const perch of perches) {
    const dx = Math.max(perch.x0 - x, 0, x - perch.x1);
    const dy = perch.y - y;
    const distance = dx * dx + dy * dy;
    if (distance < least) {
      nearest = perch;
      least = distance;
    }
  }
  return nearest;
}
//#endregion 🔖️Perches

//#region 🔖️Walking
/** 👣️ The next x one tick later on the way to `goal` at `speed` pixels per second: a step of `max(speed, 0) ÷ 64`, or the goal itself when it is no farther than that. */
export function strideTo(x: number, goal: number, speed: number): number {
  const step = Math.max(speed, 0) / TICKS_PER_SECOND;
  const gap = goal - x;
  return gap > step ? x + step : gap < 0 - step ? x - step : goal;
}
//#endregion 🔖️Walking

//#region 🔖️Falling
/** ⬇️ One tick of falling, velocity first: `vy ← min(vy + GRAVITY ÷ 64, FALL_SPEED)`, then `y ← y + vy ÷ 64`. */
export function fallStep(y: number, vy: number): Fall {
  const speed = Math.min(vy + GRAVITY / TICKS_PER_SECOND, FALL_SPEED);
  return { y: y + speed / TICKS_PER_SECOND, vy: speed };
}

/** 🛬️ The highest perch crossed at `x` on the way down from `fromY` to `toY`, both heights and both ends of the perch included, the first one among equals; `null` when none is crossed, as always while rising (`toY < fromY`). */
export function landingOf(perches: readonly Perch[], x: number, fromY: number, toY: number): Perch | null {
  let landing: Perch | null = null;
  for (const perch of perches) {
    if (perch.x0 <= x && x <= perch.x1 && fromY <= perch.y && perch.y <= toY && (landing === null || perch.y < landing.y)) landing = perch;
  }
  return landing;
}
//#endregion 🔖️Falling

//#region 🔖️Hopping
/** 🚀️ The ballistic hop from `from` to `to`, or `null` when out of reach.
 *
 * 1. `rise = max(HOP_CLEARANCE − min(dy, 0), |dx| × HOP_STEEPNESS)` is the apex above the launch, so the apex clears
 *    both ends; too high when it exceeds `HOP_HEIGHT`, too far when `|dx|` exceeds `HOP_DISTANCE`.
 * 2. `ticks = floor((sqrt(2 × rise ÷ GRAVITY) + sqrt(2 × (rise + dy) ÷ GRAVITY)) × 64 + 0.5)` is the time of that
 *    arc up and down, rounded to whole ticks; too long when it exceeds `HOP_TICKS`.
 * 3. `vy = (dy − GRAVITY × ticks × (ticks + 1) ÷ 8192) × 64 ÷ ticks` makes the per-tick sum end on `to.y`; a landing
 *    faster than `FALL_SPEED` is a fall, not a hop. `vx = dx × 64 ÷ ticks`.
 *
 * Every limit is tested as "within", so a point that is not a finite number is out of reach as well.
 */
export function hopOf(from: Point, to: Point): Hop | null {
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const rise = Math.max(HOP_CLEARANCE - Math.min(dy, 0), Math.abs(dx) * HOP_STEEPNESS);
  if (!(rise <= HOP_HEIGHT && Math.abs(dx) <= HOP_DISTANCE)) return null;
  const seconds = Math.sqrt((2 * rise) / GRAVITY) + Math.sqrt((2 * (rise + dy)) / GRAVITY);
  const ticks = Math.floor(seconds * TICKS_PER_SECOND + 0.5);
  if (!(ticks <= HOP_TICKS)) return null;
  const vy = ((dy - (GRAVITY * ticks * (ticks + 1)) / (2 * TICKS_PER_SECOND * TICKS_PER_SECOND)) * TICKS_PER_SECOND) / ticks;
  if (!(vy + ticks * (GRAVITY / TICKS_PER_SECOND) <= FALL_SPEED)) return null;
  return { vx: (dx * TICKS_PER_SECOND) / ticks, vy, ticks };
}

/** 🎈️ One tick of a hop towards `to` with `ticks` left to fly, counting this one: the fall step for the height and `x ← x + vx ÷ 64`, and on the last tick (`ticks ≤ 1`) the target itself. */
export function hopStep(x: number, y: number, vx: number, vy: number, to: Point, ticks: Ticks): Flight {
  const fallen = fallStep(y, vy);
  return ticks > 1 ? { x: x + vx / TICKS_PER_SECOND, y: fallen.y, vx, vy: fallen.vy } : { x: to.x, y: to.y, vx, vy: fallen.vy };
}

/** 🎯️ The perch on which `hop` from `from` towards `to` really ends: the first landing of its per-tick flight, which is another perch than the target's when one lies in the way down; `null` when the flight lands nowhere. */
export function hopLanding(perches: readonly Perch[], from: Point, to: Point, hop: Hop): Perch | null {
  let flight: Flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left >= 1; left--) {
    const next = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    const landing = landingOf(perches, next.x, flight.y, next.y);
    if (landing !== null) return landing;
    flight = next;
  }
  return null;
}
//#endregion 🔖️Hopping
