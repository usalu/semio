/** 🪞️ Derived-view detection: a front or plan view that is a mirrored, rotated or uniformly scaled and shifted copy of the elevation (or of another view or band) is not a handcrafted view.
 *
 * The evaluated rings of two views are sampled to point sets (vertices and edge midpoints), centred on their bounding box and scaled to a unit longest side. The distance is the symmetric Hausdorff distance under the best of a small candidate set of transforms
 * (identity, mirror about each axis, rotations of 90, 180 and 270 degrees, and the mirrored rotations). A distance below {@link NEAR_MATCH} is flagged, below {@link COPY} for the same view of two bands.
 *
 * @see ./🧰️.ts — the matrix column
 * @see ../../🧫️fixtures/🔭️multiview-coverage/🔣️.json — the allowlist of genuinely symmetric bodies
 */
import type { Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedShape, Point } from "../../🔨️modules/🔗️contracts/🟦️.ts";

/** 📏️ The normalised Hausdorff distance below which two different views count as the same drawing. */
export const NEAR_MATCH = 0.03;

/** 📏️ The normalised distance below which the same view of two bands counts as a copy. */
export const COPY = 0.005;

/** 🔢️ The most points a sample keeps per view. */
export const MAX_POINTS = 240;

/** 🔄️ The candidate transforms: a 2x2 matrix each, applied to the unit-normalised point set. */
export const TRANSFORMS: readonly { readonly name: string; readonly m: readonly [number, number, number, number] }[] = [
  { name: "identity", m: [1, 0, 0, 1] },
  { name: "mirror-x", m: [-1, 0, 0, 1] },
  { name: "mirror-y", m: [1, 0, 0, -1] },
  { name: "rotate-180", m: [-1, 0, 0, -1] },
  { name: "rotate-90", m: [0, -1, 1, 0] },
  { name: "rotate-270", m: [0, 1, -1, 0] },
  { name: "mirror-rotate-90", m: [0, 1, 1, 0] },
  { name: "mirror-rotate-270", m: [0, -1, -1, 0] },
];

/** ✂️ The unit-normalised point set of the non-ground shapes: vertices and edge midpoints, centred and scaled by the longest side; undefined when nothing is drawn. */
export function pointSet(shapes: readonly EvaluatedShape[]): Point[] | undefined {
  const raw: Point[] = [];
  for (const evaluated of shapes) {
    if (evaluated.shape.role === "ground") continue;
    for (const ring of evaluated.rings)
      ring.forEach((p, i) => {
        raw.push(p);
        const next = ring[i + 1] ?? (evaluated.closed ? ring[0] : undefined);
        if (next !== undefined) raw.push({ x: (p.x + next.x) / 2, y: (p.y + next.y) / 2 });
      });
  }
  if (raw.length === 0) return undefined;
  const xs = raw.map((p) => p.x);
  const ys = raw.map((p) => p.y);
  const side = Math.max(Math.max(...xs) - Math.min(...xs), Math.max(...ys) - Math.min(...ys));
  if (!(side > 0)) return undefined;
  const cx = (Math.max(...xs) + Math.min(...xs)) / 2;
  const cy = (Math.max(...ys) + Math.min(...ys)) / 2;
  const stride = Math.max(1, Math.ceil(raw.length / MAX_POINTS));
  return raw.filter((_, i) => i % stride === 0).map((p) => ({ x: (p.x - cx) / side, y: (p.y - cy) / side }));
}

function directed(a: readonly Point[], b: readonly Point[]): number {
  let worst = 0;
  for (const p of a) {
    let best = Number.POSITIVE_INFINITY;
    for (const q of b) best = Math.min(best, (p.x - q.x) ** 2 + (p.y - q.y) ** 2);
    worst = Math.max(worst, best);
  }
  return Math.sqrt(worst);
}

/** 📐️ The symmetric Hausdorff distance of two point sets. */
export const hausdorff = (a: readonly Point[], b: readonly Point[]): number => Math.max(directed(a, b), directed(b, a));

/** 🪞️ The best transform of `b` onto `a` and its distance. */
export function bestTransform(a: readonly Point[], b: readonly Point[]): { readonly name: string; readonly distance: number } {
  let best = { name: "identity", distance: Number.POSITIVE_INFINITY };
  for (const { name, m } of TRANSFORMS) {
    const moved = b.map((p) => ({ x: m[0] * p.x + m[1] * p.y, y: m[2] * p.x + m[3] * p.y }));
    const distance = hausdorff(a, moved);
    if (distance < best.distance) best = { name, distance };
  }
  return best;
}

/** 🚩️ One flagged near-match. */
export type DerivedFlag = { readonly view: Viewpoint; readonly against: string; readonly transform: string; readonly distance: number };

/** 🔎️ The flags of the front and plan views of one drawing: each equal (under a candidate transform) to the elevation, the front to the plan, or the same view of another band (`others`, keyed by a band label, only the copy threshold applies). */
export function derivedFlags(views: Readonly<Record<Viewpoint, readonly EvaluatedShape[]>>, others: Readonly<Record<string, Readonly<Record<Viewpoint, readonly EvaluatedShape[]>>>> = {}): DerivedFlag[] {
  const sets = Object.fromEntries((["elevation", "front", "plan"] as const).map((view) => [view, pointSet(views[view])])) as Record<Viewpoint, Point[] | undefined>;
  const flags: DerivedFlag[] = [];
  const compare = (view: Viewpoint, against: string, other: Point[] | undefined, threshold: number, transforms = true): void => {
    const mine = sets[view];
    if (mine === undefined || other === undefined) return;
    const best = transforms ? bestTransform(mine, other) : { name: "identity", distance: hausdorff(mine, other) };
    if (best.distance < threshold) flags.push({ view, against, transform: best.name, distance: Math.round(best.distance * 1e5) / 1e5 });
  };
  compare("front", "elevation", sets.elevation, NEAR_MATCH);
  compare("plan", "elevation", sets.elevation, NEAR_MATCH);
  compare("plan", "front", sets.front, NEAR_MATCH);
  for (const [band, other] of Object.entries(others))
    for (const view of ["front", "plan"] as const) compare(view, `${band} ${view}`, pointSet(other[view]), COPY, false);
  return flags;
}
