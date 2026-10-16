/** 📦️ Axis-aligned boxes
 *
 * Boxes of solids and points, their corners, intersection, volume, union and difference.
 */

// #region 🔌️Adapters
import { type Vec3 } from "@semio-tech/framework-3d-js";
import { Model } from "../../📐️geometry/🧊️model/🟦️.ts";
import { type SolidPrimitive, type SolidRecord } from "../🔷️brep-records/🟦️.ts";
import { vec3Add, vec3Normalize, vec3Scale } from "../📏️vec3/🟦️.ts";
import { derivedSolidPoints } from "../🔺️face-geometry/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🌀️EdgeGeometry
export function solidPrimitiveAabb(solid: SolidPrimitive): { readonly min: Vec3; readonly max: Vec3 } {
  if (solid.kind === "box") {
    const ax = Math.min(solid.cornerA[0], solid.cornerB[0]);
    const ay = Math.min(solid.cornerA[1], solid.cornerB[1]);
    const bx = Math.max(solid.cornerA[0], solid.cornerB[0]);
    const by = Math.max(solid.cornerA[1], solid.cornerB[1]);
    const z0 = Math.min(solid.cornerA[2], solid.cornerB[2]);
    const z1 = z0 + solid.height;
    return { min: [ax, ay, z0], max: [bx, by, z1] };
  }
  if (solid.kind === "sphere") {
    const r = solid.radius;
    return {
      min: [solid.center[0] - r, solid.center[1] - r, solid.center[2] - r],
      max: [solid.center[0] + r, solid.center[1] + r, solid.center[2] + r],
    };
  }
  const ax = vec3Normalize(solid.axis);
  const h = solid.height;
  const r = solid.kind === "cone" ? Math.max(solid.radius, solid.radiusTop ?? 0) : solid.radius;
  const end = vec3Add(solid.base, vec3Scale(ax, h));
  return {
    min: [Math.min(solid.base[0], end[0]) - r, Math.min(solid.base[1], end[1]) - r, Math.min(solid.base[2], end[2]) - r],
    max: [Math.max(solid.base[0], end[0]) + r, Math.max(solid.base[1], end[1]) + r, Math.max(solid.base[2], end[2]) + r],
  };
}

/** 📐️ Axis-aligned bounds of a solid from shell vertices when present, else analytic `SolidPrimitive`. */
export function modelObjectAabb(model: Model, solid: SolidRecord): { readonly min: Vec3; readonly max: Vec3 } | null {
  const points = derivedSolidPoints(model, solid);
  if (points.length === 0) return solid.solid ? solidPrimitiveAabb(solid.solid) : null;
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  for (const p of points) {
    minX = Math.min(minX, p[0]);
    minY = Math.min(minY, p[1]);
    minZ = Math.min(minZ, p[2]);
    maxX = Math.max(maxX, p[0]);
    maxY = Math.max(maxY, p[1]);
    maxZ = Math.max(maxZ, p[2]);
  }
  const ez = 1e-6;
  return {
    min: [minX, minY, minZ],
    max: [Math.max(maxX, minX + ez), Math.max(maxY, minY + ez), Math.max(maxZ, minZ + ez)],
  };
}

type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** 📐️ Eight corners of an axis-aligned box. */
export function aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[] {
  return [
    [min[0], min[1], min[2]],
    [max[0], min[1], min[2]],
    [max[0], max[1], min[2]],
    [min[0], max[1], min[2]],
    [min[0], min[1], max[2]],
    [max[0], min[1], max[2]],
    [max[0], max[1], max[2]],
    [min[0], max[1], max[2]],
  ];
}

/** 📐️ Overlap of two axis-aligned bounds (or `null`). */
export function aabbIntersect(a: Aabb, b: Aabb): Aabb | null {
  const min: Vec3 = [Math.max(a.min[0], b.min[0]), Math.max(a.min[1], b.min[1]), Math.max(a.min[2], b.min[2])];
  const max: Vec3 = [Math.min(a.max[0], b.max[0]), Math.min(a.max[1], b.max[1]), Math.min(a.max[2], b.max[2])];
  if (min[0] >= max[0] || min[1] >= max[1] || min[2] >= max[2]) return null;
  return { min, max };
}

export function aabbVolume(a: Aabb): number {
  return Math.max(0, a.max[0] - a.min[0]) * Math.max(0, a.max[1] - a.min[1]) * Math.max(0, a.max[2] - a.min[2]);
}

function aabbUnionVolume(boxes: readonly Aabb[]): number {
  const n = boxes.length;
  if (n === 0) return 0;
  let total = 0;
  for (let mask = 1; mask < 1 << n; mask++) {
    const idx: number[] = [];
    for (let i = 0; i < n; i++) {
      if (mask & (1 << i)) idx.push(i);
    }
    let inter: Aabb | null = boxes[idx[0]!]!;
    for (let k = 1; k < idx.length; k++) {
      inter = inter ? aabbIntersect(inter, boxes[idx[k]!]!) : null;
      if (!inter) break;
    }
    if (!inter) continue;
    const vol = aabbVolume(inter);
    total += idx.length % 2 === 1 ? vol : -vol;
  }
  return Math.max(0, total);
}

/** 📐️ Exact volume of `solid ∩ ⋃ cutters` for axis-aligned bounds (shape-invariant part split). */
export function aabbOverlapUnionVolume(cell: Aabb, cutters: readonly Aabb[]): number {
  const pieces: Aabb[] = [];
  for (const cutter of cutters) {
    const inter = aabbIntersect(cell, cutter);
    if (inter) pieces.push(inter);
  }
  return aabbUnionVolume(pieces);
}

function aabbSubtractSingle(cell: Aabb, hole: Aabb, eps = 1e-9): Aabb[] {
  const inter = aabbIntersect(cell, hole);
  if (!inter || aabbVolume(inter) <= eps) return [cell];
  const out: Aabb[] = [];
  if (cell.min[0] < inter.min[0] - eps) {
    out.push({ min: [cell.min[0], cell.min[1], cell.min[2]], max: [inter.min[0], cell.max[1], cell.max[2]] });
  }
  if (inter.max[0] < cell.max[0] - eps) {
    out.push({ min: [inter.max[0], cell.min[1], cell.min[2]], max: [cell.max[0], cell.max[1], cell.max[2]] });
  }
  if (cell.min[1] < inter.min[1] - eps) {
    out.push({ min: [inter.min[0], cell.min[1], cell.min[2]], max: [inter.max[0], inter.min[1], cell.max[2]] });
  }
  if (inter.max[1] < cell.max[1] - eps) {
    out.push({ min: [inter.min[0], inter.max[1], cell.min[2]], max: [inter.max[0], cell.max[1], cell.max[2]] });
  }
  if (cell.min[2] < inter.min[2] - eps) {
    out.push({ min: [inter.min[0], inter.min[1], cell.min[2]], max: [inter.max[0], inter.max[1], inter.min[2]] });
  }
  if (inter.max[2] < cell.max[2] - eps) {
    out.push({ min: [inter.min[0], inter.min[1], inter.max[2]], max: [inter.max[0], inter.max[1], cell.max[2]] });
  }
  return out.filter((p) => aabbVolume(p) > eps);
}

/** 📐️ Axis-aligned pieces of `solid \\ ⋃(solid ∩ cutter)` (volumetric difference decomposition). */
export function aabbDifferencePieces(cell: Aabb, cutters: readonly Aabb[], volEps = 1e-6): Aabb[] {
  let pieces: Aabb[] = [cell];
  for (const cutter of cutters) {
    const hole = aabbIntersect(cell, cutter);
    if (!hole || aabbVolume(hole) <= volEps) continue;
    const next: Aabb[] = [];
    for (const piece of pieces) next.push(...aabbSubtractSingle(piece, hole, volEps));
    pieces = next.filter((p) => aabbVolume(p) > volEps);
  }
  return pieces;
}

/** 📦️ Axis-aligned bounds from points (optional padding). */
export function aabbFromPoints(points: readonly Vec3[], pad = 0): Aabb | null {
  if (!points.length) return null;
  let minX = points[0]![0];
  let minY = points[0]![1];
  let minZ = points[0]![2];
  let maxX = minX;
  let maxY = minY;
  let maxZ = minZ;
  for (const p of points) {
    minX = Math.min(minX, p[0]);
    minY = Math.min(minY, p[1]);
    minZ = Math.min(minZ, p[2]);
    maxX = Math.max(maxX, p[0]);
    maxY = Math.max(maxY, p[1]);
    maxZ = Math.max(maxZ, p[2]);
  }
  return {
    min: [minX - pad, minY - pad, minZ - pad],
    max: [maxX + pad, maxY + pad, maxZ + pad],
  };
}
// #endregion 🌀️EdgeGeometry
// #endregion 🧮️SpatialKernelMath
