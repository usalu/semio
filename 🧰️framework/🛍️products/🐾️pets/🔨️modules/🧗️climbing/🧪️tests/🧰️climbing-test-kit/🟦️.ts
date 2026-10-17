/** 🧰️ Shared fixtures and builders of the climbing unit suites: the stage sample size, the pet sizes, the seeded stream, perch, pitch and rectangle builders, the climb path and the floor, card and wall fixtures. */
import { type Perch, type Pitch, type Point, type Rect, type Size } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { climbStep, ladderFor, shotFor } from "../../🟦️.ts";

export type Ladder = NonNullable<ReturnType<typeof ladderFor>>;
export type Shot = NonNullable<ReturnType<typeof shotFor>>;

/** 🎚️ How many generated stages each sweep of placements judges at the level of the run. */
export const STAGES = sampled(60, 600, 6000);

/** 🐾️ The reference pet of the research brief and the smallest and the largest of the menagerie. */
export const PET: Size = { width: 40, height: 48 };
export const SMALL: Size = { width: 34, height: 40 };
export const LARGE: Size = { width: 46, height: 56 };

/** 🎰️ A deterministic stream of integers in `[0, bound)` (a 32-bit linear congruential generator). */
export function stream(seed: number): (bound: number) => number {
  let state = seed >>> 0;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return Math.floor((state / 4294967296) * bound);
  };
}

/** 🪺️ A perch. */
export function perch(id: string, x0: number, x1: number, y: number): Perch {
  return { surface: id, x0, x1, y };
}

/** 🧱️ A pitch. */
export function pitch(id: string, surface: string, side: 1 | -1, x: number, y0: number, y1: number): Pitch {
  return { wall: id, surface, side, x, y0, y1 };
}

/** 🚧️ A keep-out box. */
export function keepout(x: number, y: number, width: number, height: number): Rect {
  return { x, y, width, height };
}

/** 📦️ The box the survey reports for a card: its own, grown by 4 px on both sides. */
export function solid(x0: number, x1: number, y0: number, y1: number): Rect {
  return keepout(x0 - 4, y0, x1 - x0 + 8, y1 - y0);
}

/** 🧮️ The heights of a climb after every tick until it has arrived. */
export function climbed(y: number, goal: number): number[] {
  const heights: number[] = [];
  let height = y;
  for (let tick = 0; height !== goal && tick < 2000; tick++) {
    height = climbStep(height, goal, tick);
    heights.push(height);
  }
  return heights;
}

/** 🫙️ Whether a box has an area and holds a point, its edges included: what a ladder stands on or leans against, which is not in its way. */
export function holding(rect: Rect, point: Point): boolean {
  return rect.width > 0 && rect.height > 0 && rect.x <= point.x && point.x <= rect.x + rect.width && rect.y <= point.y && point.y <= rect.y + rect.height;
}

/** 🛤️ The x of a segment at height `y`. */
export function along(from: Point, to: Point, y: number): number {
  return from.x + ((to.x - from.x) * (y - from.y)) / (to.y - from.y);
}

export const FLOOR = perch("floor", 0, 640, 480);
export const CARD = perch("card", 306, 494, 300);
export const CARD_BOX = solid(300, 500, 300, 480);
export const LEFT = pitch("card-left", "card", -1, 300, 300, 480);
export const RIGHT = pitch("card-right", "card", 1, 500, 300, 480);
