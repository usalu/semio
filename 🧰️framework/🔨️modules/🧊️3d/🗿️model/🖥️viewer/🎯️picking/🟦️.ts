/** 🎯️ Picking: a world ray against the posed triangles (Möller–Trumbore), nearest hit first. Back faces are ignored unless the material is double-sided; blended materials with zero alpha are skipped.
 *
 * The identity of a hit is node + mesh + primitive + material + triangle (and the node's tags) and, when the primitive carries solid provenance (`Primitive.solids`), the id of the solid that produced the triangle (`null` otherwise).
 * @see ../🎞️frame/🟦️.ts */
import type { Operation, PrimitiveSolid, Slug } from "../../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { MATERIAL_STRIDE, type Frame, type Scene } from "../🧬️contract/🟦️.ts";
import { posedPositions } from "../🎞️frame/🟦️.ts";
import { rayOf, type CameraState, type Ray } from "../📷️camera/🟦️.ts";
import type { V3 } from "../../🧮️math/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

/** 🪧️ The nearest surface along a ray. `material` is the variant-resolved material id, `solid` the id of the solid that produced the triangle or null without provenance. */
export type Hit = { primitive: number; node: Slug; mesh: Slug; material: Slug; tags: readonly Slug[]; solid: Slug | null; triangle: number; distance: number; point: V3; normal: V3 };

/** 🧬️ The solid whose index range contains the primitive-local index `index` (binary search over contiguous ranges), or null. */
export function solidOf(solids: readonly PrimitiveSolid[], index: number): Slug | null {
  let lo = 0, hi = solids.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1, range = solids[mid]!;
    if (index < range.first) hi = mid - 1;
    else if (index >= range.first + range.count) lo = mid + 1;
    else return range.id;
  }
  return null;
}

/** 🔦️ Nearest triangle hit of `ray`, or null; the optional `operation` is polled every few thousand triangles and a cancellation throws `CancelledError` without a hit. */
export function pick(scene: Scene, frame: Frame, ray: Ray, operation: Operation = silentOperation): Hit | null {
  const pace = new Pace(operation);
  let best: Hit | null = null;
  const [ox, oy, oz] = ray.origin, [dx, dy, dz] = ray.direction;
  for (const prim of scene.primitives) {
    if (prim.mode !== "triangles") continue;
    const material = frame.materialMap[prim.materialIndex]!;
    const o = material * MATERIAL_STRIDE;
    if (frame.materials[o + 11] === 1 && frame.materials[o + 3] === 0) continue;
    const doubleSided = frame.materials[o + 10] === 1;
    const w = posedPositions(scene, frame, prim.index);
    for (let t = 0; t < prim.indexCount; t += 3) {
      pace.tick();
      const a = (scene.indices[prim.indexStart + t]! - prim.vertexStart) * 3, b = (scene.indices[prim.indexStart + t + 1]! - prim.vertexStart) * 3, c = (scene.indices[prim.indexStart + t + 2]! - prim.vertexStart) * 3;
      const e1x = w[b]! - w[a]!, e1y = w[b + 1]! - w[a + 1]!, e1z = w[b + 2]! - w[a + 2]!;
      const e2x = w[c]! - w[a]!, e2y = w[c + 1]! - w[a + 1]!, e2z = w[c + 2]! - w[a + 2]!;
      const nx = e1y * e2z - e1z * e2y, ny = e1z * e2x - e1x * e2z, nz = e1x * e2y - e1y * e2x;
      const facing = nx * dx + ny * dy + nz * dz;
      if (Math.abs(facing) < 1e-18 || (facing > 0 && !doubleSided)) continue;
      const px = dy * e2z - dz * e2y, py = dz * e2x - dx * e2z, pz = dx * e2y - dy * e2x;
      const det = e1x * px + e1y * py + e1z * pz;
      if (Math.abs(det) < 1e-18) continue;
      const sx = ox - w[a]!, sy = oy - w[a + 1]!, sz = oz - w[a + 2]!;
      const u = (sx * px + sy * py + sz * pz) / det;
      if (u < 0 || u > 1) continue;
      const qx = sy * e1z - sz * e1y, qy = sz * e1x - sx * e1z, qz = sx * e1y - sy * e1x;
      const v = (dx * qx + dy * qy + dz * qz) / det;
      if (v < 0 || u + v > 1) continue;
      const distance = (e2x * qx + e2y * qy + e2z * qz) / det;
      if (distance < 0 || (best && distance >= best.distance)) continue;
      const l = portable.hypot(nx, ny, nz) || 1;
      const s = facing > 0 ? -1 : 1;
      best = {
        primitive: prim.index, node: prim.node, mesh: prim.mesh, material: scene.materials[material]!.id, tags: prim.tags, solid: solidOf(prim.solids, t), triangle: t / 3, distance,
        point: [ox + dx * distance, oy + dy * distance, oz + dz * distance], normal: [(s * nx) / l, (s * ny) / l, (s * nz) / l],
      };
    }
  }
  return best;
}

/** 🖱️ Picks at normalised device coordinates (`x` right, `y` up, in [-1, 1]) of the picture a camera and aspect produce. */
export const pickAt = (scene: Scene, frame: Frame, camera: CameraState, aspect: number, ndcX: number, ndcY: number, operation: Operation = silentOperation): Hit | null => pick(scene, frame, rayOf(camera, aspect, ndcX, ndcY), operation);
