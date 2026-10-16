/** ⚓️ Anchor placement
 *
 * Closest-point and surface placement queries, anchor position evaluation and placement from picked entities.
 */

// #region 🔌️Adapters
import { type Vec3 } from "@semio-tech/framework-3d-js";
import { Model } from "../../📐️geometry/🧊️model/🟦️.ts";
import { type AnchorAttachment, type AnchorRecord, type EdgeRef, type FaceRecord, type FaceRef, geom, type SolidRecord, type SolidRef, type VertexRef, type WireRecord, type WireRef } from "../🔷️brep-records/🟦️.ts";
import { vec3Add, vec3Cross, vec3Distance, vec3Dot, vec3Length, vec3Normalize, vec3Scale, vec3Sub } from "../📏️vec3/🟦️.ts";
import { edgeSamplePoints } from "../🌀️curve-sampling/🟦️.ts";
import { derivedFacePoints, derivedPointCentroid } from "../🔺️face-geometry/🟦️.ts";
import { modelObjectAabb } from "../📦️aabb/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🌀️EdgeGeometry
function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

function uniqueAnchorCurvePoints(points: readonly Vec3[]): readonly Vec3[] {
  if (points.length <= 1) return points;
  const out: Vec3[] = [points[0]!];
  for (let i = 1; i < points.length; i++) {
    const prev = out[out.length - 1]!;
    const next = points[i]!;
    if (vec3Distance(prev, next) > 1e-9) out.push(next);
  }
  return out;
}

function closestPointOnSegment(a: Vec3, b: Vec3, point: Vec3): { readonly point: Vec3; readonly t: number; readonly distance: number } {
  const ab = vec3Sub(b, a);
  const len2 = vec3Dot(ab, ab);
  if (len2 < 1e-12) return { point: a, t: 0, distance: vec3Distance(a, point) };
  const t = clamp01(vec3Dot(vec3Sub(point, a), ab) / len2);
  const hit = vec3Add(a, vec3Scale(ab, t));
  return { point: hit, t, distance: vec3Distance(hit, point) };
}

function closestPointOnPolyline(points: readonly Vec3[], point: Vec3): { readonly point: Vec3; readonly t: number } | null {
  const path = uniqueAnchorCurvePoints(points);
  if (path.length === 0) return null;
  if (path.length === 1) return { point: path[0]!, t: 0 };
  let total = 0;
  const lengths: number[] = [];
  for (let i = 1; i < path.length; i++) {
    const length = vec3Distance(path[i - 1]!, path[i]!);
    lengths.push(length);
    total += length;
  }
  let best: { readonly point: Vec3; readonly t: number; readonly distance: number } | null = null;
  let prefix = 0;
  for (let i = 1; i < path.length; i++) {
    const segment = closestPointOnSegment(path[i - 1]!, path[i]!, point);
    const length = lengths[i - 1]!;
    const normalized = total > 1e-9 ? (prefix + length * segment.t) / total : 0;
    if (!best || segment.distance < best.distance) best = { point: segment.point, t: normalized, distance: segment.distance };
    prefix += length;
  }
  return best ? { point: best.point, t: best.t } : null;
}

function curvePointAtNormalizedT(points: readonly Vec3[], t: number): Vec3 | null {
  const path = uniqueAnchorCurvePoints(points);
  if (path.length === 0) return null;
  if (path.length === 1) return path[0]!;
  let total = 0;
  const lengths: number[] = [];
  for (let i = 1; i < path.length; i++) {
    const length = vec3Distance(path[i - 1]!, path[i]!);
    lengths.push(length);
    total += length;
  }
  let remaining = clamp01(t) * total;
  for (let i = 1; i < path.length; i++) {
    const length = lengths[i - 1]!;
    if (remaining <= length || i === path.length - 1) {
      const segT = length > 1e-9 ? remaining / length : 0;
      return vec3Add(path[i - 1]!, vec3Scale(vec3Sub(path[i]!, path[i - 1]!), segT));
    }
    remaining -= length;
  }
  return path[path.length - 1]!;
}

function orthonormalBasis(normal: Vec3): { readonly u: Vec3; readonly v: Vec3 } {
  const n = vec3Normalize(normal);
  const seed: Vec3 = Math.abs(n[2]) < 0.9 ? [0, 0, 1] : [1, 0, 0];
  const u = vec3Normalize(vec3Cross(seed, n));
  const v = vec3Normalize(vec3Cross(n, u));
  return { u, v };
}

export function faceNormalFromPoints(points: readonly Vec3[]): Vec3 | null {
  if (points.length < 3) return null;
  for (let i = 2; i < points.length; i++) {
    const normal = vec3Cross(vec3Sub(points[i - 1]!, points[0]!), vec3Sub(points[i]!, points[0]!));
    if (vec3Length(normal) > 1e-9) return vec3Normalize(normal);
  }
  return null;
}

function projectPointToPlane(point: Vec3, origin: Vec3, normal: Vec3): Vec3 {
  const n = vec3Normalize(normal);
  return vec3Sub(point, vec3Scale(n, vec3Dot(vec3Sub(point, origin), n)));
}

function planePlacement(origin: Vec3, normal: Vec3, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
  const hit = projectPointToPlane(point, origin, normal);
  const basis = orthonormalBasis(normal);
  const delta = vec3Sub(hit, origin);
  return { point: hit, u: vec3Dot(delta, basis.u), v: vec3Dot(delta, basis.v) };
}

function cylinderPlacement(origin: Vec3, axis: Vec3, radius: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
  const axisN = vec3Normalize(axis);
  const basis = orthonormalBasis(axisN);
  const delta = vec3Sub(point, origin);
  const axial = vec3Dot(delta, axisN);
  const radialRaw = vec3Sub(delta, vec3Scale(axisN, axial));
  const radialDir = vec3Length(radialRaw) > 1e-9 ? vec3Normalize(radialRaw) : basis.u;
  const hit = vec3Add(origin, vec3Add(vec3Scale(axisN, axial), vec3Scale(radialDir, radius)));
  return { point: hit, u: Math.atan2(vec3Dot(radialDir, basis.v), vec3Dot(radialDir, basis.u)), v: axial };
}

function spherePlacement(center: Vec3, radius: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
  const delta = vec3Sub(point, center);
  const dir = vec3Length(delta) > 1e-9 ? vec3Normalize(delta) : ([1, 0, 0] as Vec3);
  const hit = vec3Add(center, vec3Scale(dir, radius));
  return { point: hit, u: Math.atan2(dir[1], dir[0]), v: Math.asin(Math.max(-1, Math.min(1, dir[2]))) };
}

function conePlacement(apex: Vec3, axis: Vec3, semiAngle: number, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } {
  const axisN = vec3Normalize(axis);
  const basis = orthonormalBasis(axisN);
  const delta = vec3Sub(point, apex);
  const height = Math.max(0, vec3Dot(delta, axisN));
  const radialRaw = vec3Sub(delta, vec3Scale(axisN, height));
  const radialDir = vec3Length(radialRaw) > 1e-9 ? vec3Normalize(radialRaw) : basis.u;
  const radius = Math.tan(semiAngle) * height;
  const hit = vec3Add(apex, vec3Add(vec3Scale(axisN, height), vec3Scale(radialDir, radius)));
  return { point: hit, u: Math.atan2(vec3Dot(radialDir, basis.v), vec3Dot(radialDir, basis.u)), v: height };
}

function wireCurvePoints(model: Model, wire: WireRecord): readonly Vec3[] {
  const points: Vec3[] = [];
  for (const edgeId of wire.edgeIds) {
    const edge = geom(model).edges[edgeId];
    if (!edge) continue;
    for (const point of uniqueAnchorCurvePoints(edgeSamplePoints(geom(model).vertices, edge, 64))) {
      const prev = points[points.length - 1];
      if (!prev || vec3Distance(prev, point) > 1e-9) points.push(point);
    }
  }
  return points;
}

function closestPointOnAabbSurface(min: Vec3, max: Vec3, point: Vec3): Vec3 {
  const clamped: [number, number, number] = [Math.max(min[0], Math.min(max[0], point[0])), Math.max(min[1], Math.min(max[1], point[1])), Math.max(min[2], Math.min(max[2], point[2]))];
  const dx = Math.min(Math.abs(clamped[0] - min[0]), Math.abs(max[0] - clamped[0]));
  const dy = Math.min(Math.abs(clamped[1] - min[1]), Math.abs(max[1] - clamped[1]));
  const dz = Math.min(Math.abs(clamped[2] - min[2]), Math.abs(max[2] - clamped[2]));
  if (dx <= dy && dx <= dz) clamped[0] = Math.abs(clamped[0] - min[0]) <= Math.abs(max[0] - clamped[0]) ? min[0] : max[0];
  else if (dy <= dz) clamped[1] = Math.abs(clamped[1] - min[1]) <= Math.abs(max[1] - clamped[1]) ? min[1] : max[1];
  else clamped[2] = Math.abs(clamped[2] - min[2]) <= Math.abs(max[2] - clamped[2]) ? min[2] : max[2];
  return clamped as Vec3;
}

function facePlacement(model: Model, face: FaceRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number } | null {
  if (face.surface?.kind === "plane") return planePlacement(face.surface.origin, face.surface.normal, point);
  if (face.surface?.kind === "cylinder") return cylinderPlacement(face.surface.origin, face.surface.axis, face.surface.radius, point);
  if (face.surface?.kind === "sphere") return spherePlacement(face.surface.center, face.surface.radius, point);
  if (face.surface?.kind === "cone") return conePlacement(face.surface.apex, face.surface.axis, face.surface.semiAngle, point);
  const points = derivedFacePoints(model, face);
  const origin = derivedPointCentroid(points);
  const normal = faceNormalFromPoints(points);
  if (!origin || !normal) return null;
  return planePlacement(origin, normal, point);
}

function pointOnFaceAt(model: Model, faceId: FaceRef, u: number, v: number): Vec3 | null {
  const face = geom(model).faces[faceId];
  if (!face) return null;
  if (face.surface?.kind === "plane") {
    const basis = orthonormalBasis(face.surface.normal);
    return vec3Add(face.surface.origin, vec3Add(vec3Scale(basis.u, u), vec3Scale(basis.v, v)));
  }
  if (face.surface?.kind === "cylinder") {
    const axisN = vec3Normalize(face.surface.axis);
    const basis = orthonormalBasis(axisN);
    const radial = vec3Add(vec3Scale(basis.u, Math.cos(u)), vec3Scale(basis.v, Math.sin(u)));
    return vec3Add(face.surface.origin, vec3Add(vec3Scale(axisN, v), vec3Scale(radial, face.surface.radius)));
  }
  if (face.surface?.kind === "sphere") {
    return vec3Add(face.surface.center, [Math.cos(v) * Math.cos(u) * face.surface.radius, Math.cos(v) * Math.sin(u) * face.surface.radius, Math.sin(v) * face.surface.radius]);
  }
  if (face.surface?.kind === "cone") {
    const axisN = vec3Normalize(face.surface.axis);
    const basis = orthonormalBasis(axisN);
    const radial = vec3Add(vec3Scale(basis.u, Math.cos(u)), vec3Scale(basis.v, Math.sin(u)));
    const radius = Math.tan(face.surface.semiAngle) * v;
    return vec3Add(face.surface.apex, vec3Add(vec3Scale(axisN, v), vec3Scale(radial, radius)));
  }
  const points = derivedFacePoints(model, face);
  const origin = derivedPointCentroid(points);
  const normal = faceNormalFromPoints(points);
  if (!origin || !normal) return null;
  const basis = orthonormalBasis(normal);
  return vec3Add(origin, vec3Add(vec3Scale(basis.u, u), vec3Scale(basis.v, v)));
}

function solidPlacement(model: Model, cell: SolidRecord, point: Vec3): { readonly point: Vec3; readonly u: number; readonly v: number; readonly w: number } | null {
  const bounds = modelObjectAabb(model, cell);
  if (!bounds) return null;
  const hit = closestPointOnAabbSurface(bounds.min, bounds.max, point);
  const sx = Math.max(bounds.max[0] - bounds.min[0], 1e-9);
  const sy = Math.max(bounds.max[1] - bounds.min[1], 1e-9);
  const sz = Math.max(bounds.max[2] - bounds.min[2], 1e-9);
  return {
    point: hit,
    u: clamp01((hit[0] - bounds.min[0]) / sx),
    v: clamp01((hit[1] - bounds.min[1]) / sy),
    w: clamp01((hit[2] - bounds.min[2]) / sz),
  };
}

function pointOnSolidAt(model: Model, cellId: SolidRef, u: number, v: number, w: number): Vec3 | null {
  const cell = geom(model).solids[cellId];
  if (!cell) return null;
  const bounds = modelObjectAabb(model, cell);
  if (!bounds) return null;
  const point: Vec3 = [bounds.min[0] + clamp01(u) * (bounds.max[0] - bounds.min[0]), bounds.min[1] + clamp01(v) * (bounds.max[1] - bounds.min[1]), bounds.min[2] + clamp01(w) * (bounds.max[2] - bounds.min[2])];
  return closestPointOnAabbSurface(bounds.min, bounds.max, point);
}

export function evaluateAnchorPosition(model: Model, anchor: AnchorRecord): Vec3 {
  if (anchor.attachment.kind === "vertex") return geom(model).vertices[anchor.attachment.id]?.position ?? anchor.position;
  if (anchor.attachment.kind === "edge") {
    const edge = geom(model).edges[anchor.attachment.id];
    return edge ? (curvePointAtNormalizedT(edgeSamplePoints(geom(model).vertices, edge, 64), anchor.attachment.t) ?? anchor.position) : anchor.position;
  }
  if (anchor.attachment.kind === "wire") {
    const wire = geom(model).wires[anchor.attachment.id];
    return wire ? (curvePointAtNormalizedT(wireCurvePoints(model, wire), anchor.attachment.t) ?? anchor.position) : anchor.position;
  }
  if (anchor.attachment.kind === "face") return pointOnFaceAt(model, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v) ?? anchor.position;
  return pointOnSolidAt(model, anchor.attachment.id, anchor.attachment.u, anchor.attachment.v, anchor.attachment.w) ?? anchor.position;
}

/** ⚓️ Resolves anchor placement on a model entity from a pick point. */
export function anchorPlacementFromEntity(model: Model, kind: AnchorAttachment["kind"], id: string, point: Vec3): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null {
  if (kind === "vertex") {
    const vertex = geom(model).vertices[id];
    return vertex ? { position: vertex.position, attachment: { kind, id: id as VertexRef } } : null;
  }
  if (kind === "edge") {
    const edge = geom(model).edges[id];
    if (!edge) return null;
    const hit = closestPointOnPolyline(edgeSamplePoints(geom(model).vertices, edge, 64), point);
    return hit ? { position: hit.point, attachment: { kind, id: id as EdgeRef, t: hit.t } } : null;
  }
  if (kind === "wire") {
    const wire = geom(model).wires[id];
    if (!wire) return null;
    const hit = closestPointOnPolyline(wireCurvePoints(model, wire), point);
    return hit ? { position: hit.point, attachment: { kind, id: id as WireRef, t: hit.t } } : null;
  }
  if (kind === "face") {
    const face = geom(model).faces[id];
    if (!face) return null;
    const hit = facePlacement(model, face, point);
    return hit ? { position: hit.point, attachment: { kind, id: id as FaceRef, u: hit.u, v: hit.v } } : null;
  }
  const cell = geom(model).solids[id];
  if (!cell) return null;
  const hit = solidPlacement(model, cell, point);
  return hit ? { position: hit.point, attachment: { kind: "solid", id: id as SolidRef, u: hit.u, v: hit.v, w: hit.w } } : null;
}
// #endregion 🌀️EdgeGeometry
// #endregion 🧮️SpatialKernelMath
