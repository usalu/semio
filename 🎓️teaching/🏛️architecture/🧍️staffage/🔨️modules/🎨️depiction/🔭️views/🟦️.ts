/** 🔭️ The three orthographic cameras of every asset and the error of a view that has no drawing: the one place the projection convention lives in code.
 *
 * World frame (asset-local and scene frame are the same): metres, x ahead, y up, z lateral with the left of the asset at +z, ground at y = 0. With x ahead and y up and z to the left the triple is left-handed.
 * Each camera maps a world point to paper-up coordinates `(x, y)`, y up (the depiction flips y to paper millimetres, y down):
 *
 * | viewpoint | seen from | looking along | paper x | paper up | nearer to the viewer |
 * |---|---|---|---|---|---|
 * | `elevation` (side, the rig plane) | the asset's right, −z | +z | x (ahead is right) | y | smaller z |
 * | `front` | ahead, +x | −x | z (the asset's left is paper right) | y | larger x |
 * | `plan` (top) | above, +y | −y | x | z (the asset's left is paper up) | larger y |
 *
 * A viewpoint changes only this basis; pose, time, root, scale, environment and attachments never depend on it.
 *
 * @see ../../🧮️evaluation/🟦️.ts — `projected`, the placement of front and plan shapes
 * @see ../🧊️depth/🟦️.ts — the depth convention (nearer is painted later)
 */
import type { Viewpoint } from "../../../🧬️schema/🟦️.ts";
import type { Point, Vec3 } from "../../🔗️contracts/🟦️.ts";

/** 🎥️ The orthographic camera of one viewpoint: which world axes become paper x and paper up (with sign), and the world direction the viewer looks along. */
export type Camera = { readonly viewpoint: Viewpoint; readonly seenFrom: string; readonly paperX: { readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }; readonly paperUp: { readonly axis: "x" | "y" | "z"; readonly sign: 1 | -1 }; readonly looking: Vec3 };

/** 🎥️ The three cameras, keyed by viewpoint. */
export const CAMERAS: Readonly<Record<Viewpoint, Camera>> = {
  elevation: { viewpoint: "elevation", seenFrom: "the right of the asset (-z)", paperX: { axis: "x", sign: 1 }, paperUp: { axis: "y", sign: 1 }, looking: { x: 0, y: 0, z: 1 } },
  front: { viewpoint: "front", seenFrom: "ahead of the asset (+x)", paperX: { axis: "z", sign: 1 }, paperUp: { axis: "y", sign: 1 }, looking: { x: -1, y: 0, z: 0 } },
  plan: { viewpoint: "plan", seenFrom: "above the asset (+y)", paperX: { axis: "x", sign: 1 }, paperUp: { axis: "z", sign: 1 }, looking: { x: 0, y: -1, z: 0 } },
};

/** 📐️ The paper-up coordinates `(x, y)`, metres, of a world point in a viewpoint. */
export function project(viewpoint: Viewpoint, p: Vec3): Point {
  const camera = CAMERAS[viewpoint];
  return { x: camera.paperX.sign * p[camera.paperX.axis], y: camera.paperUp.sign * p[camera.paperUp.axis] };
}

/** 🧊️ The distance of a world point toward the viewer, up to a constant: larger is nearer and is painted later. */
export function nearness(viewpoint: Viewpoint, p: Vec3): number {
  const { looking } = CAMERAS[viewpoint];
  return -(p.x * looking.x + p.y * looking.y + p.z * looking.z);
}

/** 🚫️ Thrown when the drawing selected for a viewpoint has no shapes there: the view is missing and nothing is substituted, flattened or left blank. */
export class ViewMissingError extends Error {
  readonly viewpoint: Viewpoint;
  readonly asset: string;

  constructor(asset: string, viewpoint: Viewpoint) {
    super(`asset "${asset}" has no drawn shapes in the ${viewpoint} view of the selected representation; no other view is substituted`);
    this.name = "ViewMissingError";
    this.viewpoint = viewpoint;
    this.asset = asset;
  }
}
