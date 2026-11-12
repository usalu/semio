/** 📽️ Projection of physical structural vectors onto the three drawing planes. Nothing here draws: a force or moment is solved once in world metres
 * and newtons (x ahead, y up, z the asset's left) and every view only reads these projections, so the magnitude of a vector is the same in all views.
 *
 * Plane coordinates are the model coordinates the evaluator places shapes in: elevation `(x, y)`, plan `(x, z)` (paper up is the asset's left), front `(z, y)` (paper right is the asset's left).
 * The viewer looks at the elevation from the right of the asset (-z), at the plan from above (+y) and at the front from ahead (+x).
 *
 * @see ../🟦️.ts — `solveStructure`
 * @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🏗️structural-projection.md — the convention
 */
import type { Viewpoint } from "../../../🧬️schema/🟦️.ts";
import type { Point, Vec3 } from "../../🔗️contracts/🟦️.ts";
import { lookingOf, project } from "../../🎨️depiction/🔭️views/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 📍️ The in-plane part of a world vector as model coordinates of the viewpoint (the camera of `🔭️views`). */
export const planeOf = (viewpoint: Viewpoint, v: Vec3): Point => project(viewpoint, v);

/** 🎯️ The signed component of a world vector along the line of sight: positive points at the viewer (dot symbol), negative away from the viewer (cross symbol). */
export function towardViewer(viewpoint: Viewpoint, v: Vec3): number {
  const looking = lookingOf(viewpoint);
  return -(v.x * looking.x + v.y * looking.y + v.z * looking.z);
}

/** 📐️ The in-plane magnitude of a vector in a view. */
export function inPlaneMagnitude(viewpoint: Viewpoint, v: Vec3): number {
  const p = planeOf(viewpoint, v);
  return portable.hypot(p.x, p.y);
}

/** 📏️ The full magnitude of a vector, identical in every view: `|v|² = in-plane² + out-of-plane²`. */
export const magnitude = (v: Vec3): number => portable.hypot(v.x, v.y, v.z);

/** 🔄️ What one moment vector looks like in a view: `vector` the in-plane components of `M` (drawn as a double-headed arrow, left-hand rule), `curl` the part along the line of sight (drawn as an arc, positive counter-clockwise as seen), `total` the invariant magnitude. */
export type MomentProjection = { readonly vector: Point; readonly curl: number; readonly total: number };

/** 🧭️ Projects a moment `M = r × F` with the solver's components `(mx, my, mz)` (positive about axis k turns i towards j for the cyclic order (i, j, k)). The in-plane part is drawn as an engineering double-headed arrow along its axis components; because x ahead, y up, z left is a left-handed triple the arrow follows the LEFT-hand rule. The part along the line of sight is the curl, positive when it turns counter-clockwise as the viewer sees it: positive `mz` is counter-clockwise in the side elevation, positive `mx` (front elevation) and positive `my` (plan) are clockwise. */
export function projectMoment(viewpoint: Viewpoint, m: Vec3): MomentProjection {
  return { vector: planeOf(viewpoint, m), curl: -towardViewer(viewpoint, m), total: magnitude(m) };
}

/** 🏹️ What one force vector looks like in a view: the in-plane arrow, the out-of-plane component with its symbol and the invariant magnitude. */
export type ForceProjection = { readonly arrow: Point; readonly outOfPlane: number; readonly symbol: "dot" | "cross" | null; readonly total: number };

/** 🧪️ Projects a force vector: a component along the line of sight is explicit as a dot (towards the viewer) or a cross (away). */
export function projectForce(viewpoint: Viewpoint, f: Vec3, tolerance = 1e-9): ForceProjection {
  const out = towardViewer(viewpoint, f);
  const total = magnitude(f);
  return { arrow: planeOf(viewpoint, f), outOfPlane: out, symbol: Math.abs(out) <= tolerance * Math.max(1, total) ? null : out > 0 ? "dot" : "cross", total };
}

/** 🧊️ The tension-side direction of a bending moment `m` on a member along `u`: `-(m × u)`, so a sagging moment (about the lateral axis) puts it on the bone's local -y side. */
export function tensionSide(m: Vec3, u: Vec3): Vec3 {
  return { x: -(m.y * u.z - m.z * u.y), y: -(m.z * u.x - m.x * u.z), z: -(m.x * u.y - m.y * u.x) };
}
