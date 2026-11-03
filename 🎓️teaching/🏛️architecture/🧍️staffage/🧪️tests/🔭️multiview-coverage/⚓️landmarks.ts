/** ⚓️ Per-bone and per-anchor agreement of the three views of one evaluated state.
 *
 * - Tier one ({@link boneSync}): a bone with exactly one drawn shape in two views must have the same extent along the shared axis in both (elevation and front: height, elevation and plan: forward, front and plan: lateral), within D18.
 * - Tier two ({@link boneSync}, `group`): the same comparison for the union extent of bones with several shapes in a view; reported as `group-drift`, never as a failure.
 * - Anchors ({@link anchorLandmarks}): every contact, attachment and emitter anchor lies inside or on the drawn shapes of its bone, within D18, in every view in which the bone is drawn (a bone without a shape in a view is hidden or absent there and is skipped, D40).
 *
 * @see ./🧰️.ts — the matrix
 */
import type { Anchor, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedShape, EvaluatedState, Point } from "../../🔨️modules/🔗️contracts/🟦️.ts";

/** 🧾️ One disagreement. */
export type BoneFinding = { readonly tier: "bone" | "group"; readonly check: string; readonly bone: string; readonly pair: string; readonly deltaMm: number; readonly message: string };

/** 🧾️ One anchor that does not lie on its bone's drawn shapes. */
export type AnchorFinding = { readonly view: Viewpoint; readonly anchor: string; readonly bone: string; readonly deltaMm: number; readonly message: string };

/** ⚓️ The anchor kinds that must sit on drawn geometry. */
export const LANDMARK_KINDS: readonly Anchor["kind"][] = ["contact", "attachment", "emitter"];

type Range = readonly [number, number];
const rangeOf = (shapes: readonly EvaluatedShape[], axis: "x" | "y"): Range | undefined => {
  const values = shapes.flatMap((shape) => shape.rings.flat().map((p) => p[axis]));
  return values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)];
};

const byBone = (shapes: readonly EvaluatedShape[]): Map<string, EvaluatedShape[]> => {
  const out = new Map<string, EvaluatedShape[]>();
  for (const shape of shapes) if (shape.shape.role !== "ground") (out.get(shape.shape.bone) ?? out.set(shape.shape.bone, []).get(shape.shape.bone)!).push(shape);
  return out;
};

/** 🦴️ The per-bone disagreements of one state: elevation y with front y, elevation x with plan x, front x with plan y. */
export function boneSync(state: EvaluatedState, toleranceM: number): BoneFinding[] {
  const views = { elevation: byBone(state.shapes.elevation), front: byBone(state.shapes.front), plan: byBone(state.shapes.plan) };
  const findings: BoneFinding[] = [];
  const compare = (name: string, a: Viewpoint, aAxis: "x" | "y", b: Viewpoint, bAxis: "x" | "y"): void => {
    for (const [bone, left] of views[a]) {
      const right = views[b].get(bone);
      const ra = rangeOf(left, aAxis);
      const rb = right === undefined ? undefined : rangeOf(right, bAxis);
      if (ra === undefined || rb === undefined) continue;
      const delta = Math.max(Math.abs(ra[0] - rb[0]), Math.abs(ra[1] - rb[1]));
      if (!(delta > toleranceM)) continue;
      const tier = left.length === 1 && right!.length === 1 ? "bone" : "group";
      findings.push({ tier, check: `${tier === "bone" ? "sync-bone" : "group-drift"}-${name}`, bone, pair: `${a}/${b}`, deltaMm: delta * 1000, message: `bone "${bone}" ${a} [${ra[0].toFixed(3)}, ${ra[1].toFixed(3)}] vs ${b} [${rb[0].toFixed(3)}, ${rb[1].toFixed(3)}] m (${left.length} and ${right!.length} shapes)` });
    }
  };
  compare("height", "elevation", "y", "front", "y");
  compare("forward", "elevation", "x", "plan", "x");
  compare("lateral", "front", "x", "plan", "y");
  return findings;
}

const distanceToSegment = (p: Point, a: Point, b: Point): number => {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const length2 = dx * dx + dy * dy;
  const t = length2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / length2));
  return Math.hypot(p.x - (a.x + t * dx), p.y - (a.y + t * dy));
};

const inside = (p: Point, ring: readonly Point[]): boolean => {
  let result = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!;
    const b = ring[j]!;
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) result = !result;
  }
  return result;
};

/** 📏️ The distance in metres from a point to the drawn shapes: 0 inside a closed ring, else to the nearest edge. */
export function distanceToShapes(p: Point, shapes: readonly EvaluatedShape[]): number {
  let best = Number.POSITIVE_INFINITY;
  for (const shape of shapes)
    for (const ring of shape.rings) {
      if (shape.closed && ring.length > 2 && inside(p, ring)) return 0;
      for (let i = 0; i + 1 < ring.length; i++) best = Math.min(best, distanceToSegment(p, ring[i]!, ring[i + 1]!));
      if (shape.closed && ring.length > 2) best = Math.min(best, distanceToSegment(p, ring[ring.length - 1]!, ring[0]!));
      if (ring.length === 1) best = Math.min(best, Math.hypot(p.x - ring[0]!.x, p.y - ring[0]!.y));
    }
  return best;
}

/** ⚓️ The anchors that do not lie on the drawn shapes of their bone in a view where the bone is drawn. */
export function anchorLandmarks(state: EvaluatedState, toleranceM: number): AnchorFinding[] {
  const views = { elevation: byBone(state.shapes.elevation), front: byBone(state.shapes.front), plan: byBone(state.shapes.plan) };
  const findings: AnchorFinding[] = [];
  for (const [id, { anchor, at, z }] of Object.entries(state.anchors)) {
    if (!LANDMARK_KINDS.includes(anchor.kind)) continue;
    const points: Record<Viewpoint, Point> = { elevation: { x: at.x, y: at.y }, front: { x: z, y: at.y }, plan: { x: at.x, y: z } };
    for (const view of ["elevation", "front", "plan"] as const) {
      const shapes = views[view].get(anchor.bone);
      if (shapes === undefined) continue;
      const distance = distanceToShapes(points[view], shapes);
      if (!Number.isFinite(distance)) continue;
      if (distance > toleranceM) findings.push({ view, anchor: id, bone: anchor.bone, deltaMm: distance * 1000, message: `${anchor.kind} anchor "${id}" at (${points[view].x.toFixed(3)}, ${points[view].y.toFixed(3)}) m is ${(distance * 1000).toFixed(0)} mm off the drawn shapes of bone "${anchor.bone}" in the ${view}` });
    }
  }
  return findings;
}
