/** 🧊️ Depth ordering of the shapes of one view: which shapes a part that moves behind (or in front of) another really paints under (over).
 *
 * The authored paint order (`z`, then authored position) is the truth at rest: the author drew what hides what. In a pose or at a clip instant a pair of shapes of different bones changes its paint order only when the
 * physical depth along the view direction reverses against the rest pose, by more than {@link DEPTH_TOLERANCE_M}. The depth is the `nearness` of the camera of the view (`../🔭️views`: nearer is higher in the plan,
 * further forward in the front and on the right of the asset, the smaller lateral z, in the elevation) of the nearest point of the shape carried by the 3D frames of its bone (`shapeDepth`), so broad and rotated shapes
 * are ranked by their extent and the lateral position of a bone beside the symmetry plane counts at rest too. Shapes of one bone, ground shapes and pairs whose order did not reverse keep their authored order, so static
 * drawings are unchanged and a flame that sinks below a hob, a hand that swings behind a hip or a leaf that drops behind a pane is hidden by it.
 *
 * @see ../../🧮️evaluation/🟦️.ts — `shapeDepth`, `restShapeDepth`
 * @see ../🔭️views/🟦️.ts — `CAMERAS`, `nearness`
 */
import type { PlanarViewpoint, Shape } from "../../../🧬️schema/🟦️.ts";
import type { EvaluatedShape, EvaluatedState } from "../../🔗️contracts/🟦️.ts";
import { restShapeDepth, shapeDepth } from "../../🧮️evaluation/🟦️.ts";

/** 📏️ The depth difference in metres below which two shapes count as level and keep their authored order. */
export const DEPTH_TOLERANCE_M = 0.002;

const depthOf = (state: EvaluatedState, shape: Shape, viewpoint: PlanarViewpoint): { readonly now: number; readonly then: number } | null =>
  shape.role === "ground" || state.bones[shape.bone] === undefined ? null : { now: shapeDepth(state, shape, viewpoint), then: restShapeDepth(state.asset, shape, viewpoint) };

/** 🎨️ The indices of `items` in paint order, farthest first: the authored order of `items` with every pair that reversed its depth against the rest pose painted the other way round.
 * Constraint cycles (three shapes that each hide the next) are broken deterministically: the unpainted shape with the lowest authored index is painted next, so equal inputs give equal orders. */
export function depthOrder(state: EvaluatedState, viewpoint: PlanarViewpoint, items: readonly EvaluatedShape[]): number[] {
  const identity = items.map((_, index) => index);
  const depths = items.map(({ shape }) => depthOf(state, shape, viewpoint));
  const before: number[][] = identity.map(() => []);
  let reversed = false;
  for (let i = 0; i < items.length; i++)
    for (let j = i + 1; j < items.length; j++) {
      const a = depths[i];
      const b = depths[j];
      const flip = a !== null && b !== null && items[i]!.shape.bone !== items[j]!.shape.bone && a.now - b.now > DEPTH_TOLERANCE_M && a.then - b.then <= DEPTH_TOLERANCE_M;
      if (flip) reversed = true;
      if (flip) before[j]!.push(i);
      else before[i]!.push(j);
    }
  if (!reversed) return identity;
  const waiting = identity.map(() => 0);
  for (const targets of before) for (const target of targets) waiting[target]!++;
  const done = new Set<number>();
  const order: number[] = [];
  while (order.length < items.length) {
    let pick = identity.find((index) => !done.has(index) && waiting[index] === 0);
    pick ??= identity.find((index) => !done.has(index))!;
    done.add(pick);
    order.push(pick);
    for (const target of before[pick]!) waiting[target]!--;
  }
  return order;
}
