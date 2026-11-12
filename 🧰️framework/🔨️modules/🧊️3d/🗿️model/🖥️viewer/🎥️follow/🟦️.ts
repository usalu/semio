/** 🎥️ Follow camera and root motion of the viewer: which point the orbit target tracks while an animation plays, how it is smoothed, and how the horizontal travel of the root node is kept or removed.
 *
 * Follow: `off` leaves the camera target alone; `{ node }` tracks the world translation of that node; `bounds` the centre of the exact world bounds of the posed geometry. The tracked point is smoothed per frame with an exponential filter of time constant `smoothingS` seconds:
 * `target += (point - target) * (1 - exp(-dtS / smoothingS))`, where `dtS` is the animation time that elapsed since the previous frame (a time constant of 0, a frame after a seek or a change of mode snaps onto the point). The result is deterministic for a given sequence of times and writes into a preallocated {@link FollowState}.
 * Root motion: `travel` keeps the pose as sampled; `in-place` subtracts, from the root node and every node below it, the horizontal (x and z, the up axis is y) change of the root node's local translation against its rest translation at the current time, so the figure stays at its rest position horizontally while vertical motion is kept. The root node is `rootNode` or else the first node without parent; a named node that has a parent is ignored.
 * @see ../🎞️frame/🟦️.ts */
import type { Slug } from "../../🧬️schema/🟦️.ts";
import type { V3 } from "../../🧮️math/🟦️.ts";
import type { Frame, Scene } from "../🧬️contract/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

/** 🧲️ What the camera target follows. */
export type ViewerFollow = "off" | "bounds" | { readonly node: Slug };
/** 🚶️ `travel` keeps the root translation of the animation, `in-place` removes its horizontal part. */
export type RootMotion = "in-place" | "travel";

/** 📌️ The smoothed target (in place) and whether it holds a valid position yet. */
export type FollowState = { readonly target: V3; ready: boolean };
/** 🎛️ The follow part of one frame request: the mode, the state it updates, the elapsed animation time and the smoothing time constant in seconds. */
export type FollowInput = { readonly mode: ViewerFollow; readonly state: FollowState; readonly dtS: number; readonly smoothingS: number };

/** 🆕️ A state that snaps onto the first tracked point. */
export const createFollowState = (): FollowState => ({ target: [0, 0, 0], ready: false });

/** 📍️ Writes the tracked point of `follow` under the frame's pose into `out`; false when there is nothing to track (mode off, unknown node, empty scene). Allocates nothing. */
export function followPoint(scene: Scene, frame: Frame, follow: ViewerFollow, out: V3, positions: (primitive: number) => Float64Array): boolean {
  if (follow === "off") return false;
  if (follow !== "bounds") {
    const at = scene.nodeIndex.get(follow.node);
    if (at === undefined) return false;
    out[0] = frame.matrices[at * 16 + 12]!;
    out[1] = frame.matrices[at * 16 + 13]!;
    out[2] = frame.matrices[at * 16 + 14]!;
    return true;
  }
  let x0 = Infinity, y0 = Infinity, z0 = Infinity, x1 = -Infinity, y1 = -Infinity, z1 = -Infinity;
  for (const p of scene.primitives) {
    const w = positions(p.index);
    for (let i = 0; i < p.vertexCount; i++) {
      const x = w[i * 3]!, y = w[i * 3 + 1]!, z = w[i * 3 + 2]!;
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (y < y0) y0 = y;
      if (y > y1) y1 = y;
      if (z < z0) z0 = z;
      if (z > z1) z1 = z;
    }
  }
  if (x0 > x1) return false;
  out[0] = (x0 + x1) / 2;
  out[1] = (y0 + y1) / 2;
  out[2] = (z0 + z1) / 2;
  return true;
}

/** 🧮️ Moves `state.target` towards `point` by the exponential step of the module docs (snapping when the state is not ready, `dtS` is not positive while not ready, or `smoothingS` is 0). */
export function smoothFollow(state: FollowState, point: V3, dtS: number, smoothingS: number): void {
  const alpha = !state.ready || smoothingS <= 0 ? 1 : dtS <= 0 ? 0 : 1 - portable.exp(-dtS / smoothingS);
  for (let k = 0; k < 3; k++) state.target[k] = state.target[k]! + (point[k]! - state.target[k]!) * alpha;
  state.ready = true;
}

/** 🌳️ The index of the root node of `scene`: `rootNode` when it exists and has no parent, else the first node without parent; -1 when there is none. */
export function rootIndexOf(scene: Scene, rootNode?: Slug): number {
  const nodes = scene.document.nodes;
  if (rootNode !== undefined) {
    const at = scene.nodeIndex.get(rootNode);
    if (at !== undefined && nodes[at]!.parent === null) return at;
  }
  for (let i = 0; i < nodes.length; i++) if (nodes[i]!.parent === null) return i;
  return -1;
}

/** 🛤️ Applies `in-place` root motion to the world matrices of `frame`: subtracts `(dx, dz)`, the horizontal change of the root's local translation against its rest translation, from the translation of the root and of every node below it. Allocates nothing. */
export function removeRootTravel(scene: Scene, frame: Frame, root: number, local: readonly number[]): void {
  const nodes = scene.document.nodes;
  const rest = nodes[root]!.translation;
  const dx = local[0]! - rest[0]!, dz = local[2]! - rest[2]!;
  if (dx === 0 && dz === 0) return;
  for (let i = 0; i < nodes.length; i++) {
    let at = i;
    while (at !== root && nodes[at]!.parent !== null) at = scene.nodeIndex.get(nodes[at]!.parent!)!;
    if (at !== root) continue;
    frame.matrices[i * 16 + 12] = frame.matrices[i * 16 + 12]! - dx;
    frame.matrices[i * 16 + 14] = frame.matrices[i * 16 + 14]! - dz;
  }
}
