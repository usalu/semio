/** 🧰️ Tools of the cases of the clearance unit suite: how many generated cases a comparison takes at the level of the run, the worlds and ablations that are played, the cheapest runs that bite per rule, and the small builders of boxes, bodies and sweeps.
 *
 * @see ../🌍️reference-world/🟦️.ts — the world the sweeps play
 * @see ../../🟦️.ts — the implementation under test
 */
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { type Body, type Extent } from "../../🟦️.ts";
import type { Rules } from "../🌍️reference-world/🟦️.ts";

//#region 🔖️Tools of the cases
/** 🔬️ How many generated cases a comparison with brute force takes at the level of the run. */
export const CASES = sampled(40, 400, 4000);

/** 🌐️ How many worlds are run with every rule in force at the level of the run, and for how many ticks each: at the exhaustive level the 24 × 30 000 ticks of the research prototype. */
export const WORLDS = sampled(2, 6, 24);

export const WORLD_TICKS = sampled(1500, 5000, 30000);

/** 🔪️ How many worlds are run with one rule switched off at the exhaustive level, and for how many ticks each: the ablations of the research prototype (the other levels run the short runs of `BITES` only, because the rarer rules bite in a few worlds of twelve). */
export const ABLATIONS = sampled(0, 0, 12);

export const ABLATION_TICKS = sampled(0, 0, 15000);

/** 🦷️ Per core rule the cheapest run in which the world without that rule lets bodies overlap, found with `clearance_world.ts --teeth 40 --ticks 2500` (the first overlap: guard seed 12 tick 24, corridors seed 9 tick 88, vetting seed 23 tick 23, rests seed 21 tick 81, projection seed 6 tick 7, eviction seed 17 tick 21): the seed and a number of ticks a little beyond the first overlap. */
export const BITES: readonly { readonly rule: keyof Rules; readonly seed: number; readonly ticks: number }[] = [
  { rule: "guard", seed: 12, ticks: 40 },
  { rule: "corridors", seed: 9, ticks: 120 },
  { rule: "vetting", seed: 23, ticks: 40 },
  { rule: "rests", seed: 21, ticks: 110 },
  { rule: "projection", seed: 6, ticks: 20 },
  { rule: "eviction", seed: 17, ticks: 40 },
];

/** 📦️ A box by its four edges. */
export function box(x0: number, y0: number, x1: number, y1: number): Extent {
  return { x0, y0, x1, y1 };
}

/** 🏷️ A body by its owner and the four edges of its box. */
export function body(owner: string, x0: number, y0: number, x1: number, y1: number): Body {
  return { owner, extent: { x0, y0, x1, y1 } };
}

/** 🎰️ A deterministic stream of integers in `[0, bound)` (a 32-bit linear congruential generator). */
export function stream(seed: number): (bound: number) => number {
  let state = seed >>> 0;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return Math.floor((state / 4294967296) * bound);
  };
}

/** 🥶️ A value frozen all the way down, so that a function that wrote into its input would throw. */
export function frozen<Value>(value: Value): Value {
  if (typeof value === "object" && value !== null) {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}

/** 🟰️ Whether two boxes share an area, decided by the size of their intersection instead of by comparisons of edges. */
export function sharing(one: Extent, other: Extent): boolean {
  return Math.min(one.x1, other.x1) - Math.max(one.x0, other.x0) > 0 && Math.min(one.y1, other.y1) - Math.max(one.y0, other.y0) > 0;
}

/** 🧱️ A few boxes on the quarter-pixel lattice around a body that is 40 wide and 50 tall at (100, 250). */
export function scattered(next: (bound: number) => number, count: number): Extent[] {
  const boxes: Extent[] = [];
  for (let index = 0; index < count; index++) {
    const x0 = next(1200) / 4 - 50;
    const y0 = 150 + next(800) / 4;
    boxes.push(box(x0, y0, x0 + 4 + next(240) / 4, y0 + 4 + next(240) / 4));
  }
  return boxes;
}

/** 🛤️ The boxes of a 40 × 50 body that moves by a fixed step per tick. */
export function marching(x: number, y: number, dx: number, dy: number, ticks: number): Extent[] {
  const extents: Extent[] = [];
  for (let tick = 0; tick < ticks; tick++) extents.push(box(x + dx * tick, y + dy * tick, x + dx * tick + 40, y + dy * tick + 50));
  return extents;
}
//#endregion 🔖️Tools of the cases
