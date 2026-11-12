/** 🌀️ Surfaces of revolution: ellipsoid (scaled sphere), capsule (hull of two spheres) and revolved profiles, with smooth normals and hard creases where the profile turns by more than 35 degrees.
 * @see ../🧱️builder/🟦️.ts */
import type { CapsuleSolid, EllipsoidSolid, Operation, Quality, RevolveSolid, Vec2 } from "../../🧬️schema/🟦️.ts";
import { TAU, add, cross, mul, normalize, sub, length, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { CREASE_COS, MeshBuilder, gridSurface, type Geometry } from "../🧱️builder/🟦️.ts";
import { signedArea, triangulatePolygon } from "../🔺️polygon/🟦️.ts";
import { circleSegments } from "../📏️segments/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

type Row = { a: number; r: number; na: number; nr: number; join: boolean };

export function revolveFrame(axis: readonly number[]): { a: V3; e1: V3; e2: V3 } {
  const a = normalize(axis, [0, 1, 0]);
  let e1 = sub([1, 0, 0], mul(a, a[0]));
  if (length(e1) < 1e-6) e1 = sub([0, 0, 1], mul(a, a[2]));
  e1 = normalize(e1);
  return { a, e1, e2: cross(a, e1) };
}

function sweepRows(builder: MeshBuilder, rows: Row[], origin: readonly number[], axis: readonly number[], k: number, sweep: number, segments: number): void {
  const { a, e1, e2 } = revolveFrame(axis);
  const full = sweep >= TAU - 1e-9;
  const columns = full ? segments : Math.max(1, Math.ceil((segments * sweep) / TAU));
  const direction = (theta: number): V3 => add(mul(e1, portable.cos(theta) * k), mul(e2, portable.sin(theta)));
  const normalDirection = (theta: number): V3 => add(mul(e1, portable.cos(theta) / k), mul(e2, portable.sin(theta)));
  gridSurface(builder, rows.length, columns, full, (i, j) => {
    const theta = (full ? TAU * j / columns : (sweep * j) / columns);
    const row = rows[i]!;
    const p = add(add(origin as V3, mul(a, row.a)), mul(direction(theta), row.r));
    const n = row.r === 0 ? mul(a, Math.sign(row.na) || 1) : normalize(add(mul(a, row.na), mul(normalDirection(theta), row.nr)));
    return { p, n };
  }, (i) => rows[i]!.join);
}

function sweepCaps(builder: MeshBuilder, pace: Pace, polygon: readonly Vec2[], origin: readonly number[], axis: readonly number[], k: number, sweep: number): void {
  const { a, e1, e2 } = revolveFrame(axis);
  const ccw = signedArea(polygon) > 0 ? [...polygon] : [...polygon].reverse();
  const triangles = triangulatePolygon(ccw, [], pace);
  for (const [theta, outward] of [[0, -1], [sweep, 1]] as const) {
    const direction = add(mul(e1, portable.cos(theta) * k), mul(e2, portable.sin(theta)));
    const n = mul(normalize(cross(a, direction)), outward);
    const ids = ccw.map(([axial, radial]) => builder.vertex(add(add(origin as V3, mul(a, axial)), mul(direction, radial)), n));
    for (let t = 0; t < triangles.length; t += 3) {
      const [i, j, l] = [triangles[t]!, triangles[t + 1]!, triangles[t + 2]!];
      if (outward > 0) builder.triangle(ids[i]!, ids[j]!, ids[l]!);
      else builder.triangle(ids[i]!, ids[l]!, ids[j]!);
    }
  }
}

export function tessellateEllipsoid(solid: EllipsoidSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const [rx, ry, rz] = solid.radii;
  const segments = circleSegments(Math.max(rx, ry, rz), quality);
  const latitudes = Math.max(2, Math.ceil(segments / 2));
  const builder = new MeshBuilder(new Pace(operation));
  gridSurface(builder, latitudes + 1, segments, true, (i, j) => {
    const beta = -Math.PI / 2 + (Math.PI * i) / latitudes;
    const theta = (TAU * j) / segments;
    const pole = i === 0 || i === latitudes;
    const c = pole ? 0 : portable.cos(beta);
    const s = i === 0 ? -1 : pole ? 1 : portable.sin(beta);
    const x = c * portable.sin(theta), z = c * portable.cos(theta);
    return { p: [rx * x, ry * s, rz * z], n: normalize([x / rx, s / ry, z / rz]) };
  });
  return builder.build();
}

export function tessellateCapsule(solid: CapsuleSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const { r0, r1 } = solid;
  const delta = sub(solid.p1, solid.p0);
  const d = length(delta);
  const axis = d < 1e-12 ? ([0, 1, 0] as V3) : mul(delta, 1 / d);
  const k = solid.lateralScale ?? 1;
  const segments = circleSegments(Math.max(r0, r1) * Math.max(1, k), quality);
  const quarter = Math.max(2, Math.ceil(segments / 4));
  const eps = 1e-12;
  const rows: Row[] = [];
  const sphere = (radius: number, offset: number, from: number, to: number, joinLast: boolean) => {
    if (radius <= eps) return;
    const steps = Math.max(1, Math.ceil((quarter * (to - from)) / (Math.PI / 2)));
    for (let i = 0; i <= steps; i++) {
      const beta = from + ((to - from) * i) / steps;
      const pole = (i === 0 && from <= -Math.PI / 2 + 1e-12) || (i === steps && to >= Math.PI / 2 - 1e-12);
      rows.push({ a: offset + radius * portable.sin(beta), r: pole ? 0 : radius * portable.cos(beta), na: portable.sin(beta), nr: pole ? 0 : portable.cos(beta), join: i < steps || joinLast });
    }
  };
  const contained = d + Math.min(r0, r1) <= Math.max(r0, r1) + eps;
  let origin: V3 = [...solid.p0];
  if (contained) {
    const big = r0 >= r1 ? r0 : r1;
    origin = [...(r0 >= r1 ? solid.p0 : solid.p1)];
    sphere(big, 0, -Math.PI / 2, Math.PI / 2, false);
  } else {
    const betaStar = portable.asin(Math.max(-1, Math.min(1, (r0 - r1) / d)));
    if (r0 <= eps) rows.push({ a: 0, r: 0, na: -1, nr: 0, join: true });
    else sphere(r0, 0, -Math.PI / 2, betaStar, true);
    if (r1 <= eps) rows.push({ a: d, r: 0, na: 1, nr: 0, join: false });
    else sphere(r1, d, betaStar, Math.PI / 2, false);
  }
  const builder = new MeshBuilder(new Pace(operation));
  sweepRows(builder, rows, origin, axis, k, TAU, segments);
  return builder.build();
}

/** 🛤️ The profile as the tessellator walks it: radial clamped at zero, consecutive duplicates dropped, a first point equal to the last closing a loop, and for open profiles the end points projected onto the axis (the flat end discs). Orientation is not applied. */
export function revolveOutline(profile: readonly Vec2[]): { points: Vec2[]; loop: boolean } {
  const eps = 1e-12;
  const points: Vec2[] = [];
  for (const p of profile) {
    const last = points[points.length - 1];
    if (!last || portable.hypot(p[0] - last[0], p[1] - last[1]) > eps) points.push([p[0], Math.max(0, p[1])]);
  }
  const first = points[0]!, last = points[points.length - 1]!;
  const loop = points.length > 3 && portable.hypot(first[0] - last[0], first[1] - last[1]) <= eps;
  if (loop) points.pop();
  else {
    if (points[0]![1] > eps) points.unshift([points[0]![0], 0]);
    if (points[points.length - 1]![1] > eps) points.push([points[points.length - 1]![0], 0]);
  }
  return { points, loop };
}

export function tessellateRevolve(solid: RevolveSolid, quality: Quality, operation: Operation = silentOperation): Geometry {
  const sweep = Math.min(solid.sweepRad ?? TAU, TAU);
  const k = solid.lateralScale ?? 1;
  const { points, loop } = revolveOutline(solid.profile);
  if (signedArea(points) > 0) points.reverse();
  const count = points.length;
  const edgeNormal = (i: number): Vec2 => {
    const a = points[i]!, b = points[(i + 1) % count]!;
    const da = b[0] - a[0], dr = b[1] - a[1];
    const l = portable.hypot(da, dr) || 1;
    return [-dr / l, da / l];
  };
  const rows: Row[] = [];
  const row = (p: Vec2, n: Vec2, join: boolean) => rows.push({ a: p[0], r: p[1], na: n[0], nr: n[1], join });
  for (let i = 0; i < count; i++) {
    const p = points[i]!;
    const before = loop || i > 0 ? edgeNormal((i + count - 1) % count) : null;
    const after = loop || i < count - 1 ? edgeNormal(i) : null;
    if (before && after) {
      if (before[0] * after[0] + before[1] * after[1] >= CREASE_COS) {
        const m: Vec2 = [before[0] + after[0], before[1] + after[1]];
        const l = portable.hypot(m[0], m[1]) || 1;
        row(p, [m[0] / l, m[1] / l], true);
      } else {
        row(p, before, false);
        row(p, after, true);
      }
    } else row(p, (before ?? after)!, i < count - 1);
  }
  if (loop) {
    const f = rows[0]!;
    rows[rows.length - 1]!.join = true;
    rows.push({ ...f, join: false });
  }
  const maxRadius = Math.max(...points.map((p) => p[1]), 1e-9) * Math.max(1, k);
  const segments = circleSegments(maxRadius, quality);
  const pace = new Pace(operation);
  const builder = new MeshBuilder(pace);
  sweepRows(builder, rows, solid.origin, solid.axis, k, sweep, segments);
  if (sweep < TAU - 1e-9) sweepCaps(builder, pace, points, solid.origin, solid.axis, k, sweep);
  return builder.build();
}
