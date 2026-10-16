/** 🌀️ Curve sampling
 *
 * Arc, circle, ellipse and NURBS frames, display samples, lengths and edge sampling.
 */

// #region 🔌️Adapters
import { type EdgeCurve, type Vec3 } from "@semio-tech/framework-3d-js";
import { type EdgeRecord, type VertexRecord } from "../🔷️brep-records/🟦️.ts";
import { vec3Add, vec3Cross, vec3Distance, vec3Dot, vec3Length, vec3Normalize, vec3Scale, vec3Sub } from "../📏️vec3/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🌀️EdgeGeometry
/** 🔵️ Plane frame for a circular arc through `start` and `end` about `center` (CCW in `u×v`). */
export interface ArcPlaneFrame {
  readonly center: Vec3;
  readonly radius: number;
  readonly normal: Vec3;
  readonly u: Vec3;
  readonly v: Vec3;
}

/** 🔵️ Builds arc plane basis; `null` when radius vanishes. */
export function arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null {
  const rs = vec3Sub(start, center);
  const re = vec3Sub(end, center);
  const radius = vec3Length(rs);
  if (radius < 1e-9) return null;
  let normal = vec3Cross(rs, re);
  if (vec3Length(normal) < 1e-9) normal = [0, 0, 1];
  else normal = vec3Normalize(normal);
  const u = vec3Normalize(rs);
  const v = vec3Cross(normal, u);
  return { center, radius, normal, u, v };
}

/** 🔵️ Positive CCW sweep radians from `start` to `end` in the arc plane. */
export function arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number {
  const re = vec3Sub(end, frame.center);
  let sweep = Math.atan2(vec3Dot(re, frame.v), vec3Dot(re, frame.u));
  if (sweep < 0) sweep += Math.PI * 2;
  if (sweep < 1e-9) sweep = Math.PI * 2;
  return sweep;
}

/** 🔵️ Tessellates a circular arc through `start` and `end` about `center` (positive CCW sweep). */
export function arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments = 32): readonly Vec3[] {
  const frame = arcPlaneFrame(center, start, end);
  if (!frame) return [start, end];
  const sweep = arcSweepRadians(frame, end);
  const n = Math.max(2, segments);
  const pts: Vec3[] = [];
  for (let i = 0; i <= n; i++) {
    const a = (i / n) * sweep;
    pts.push(vec3Add(frame.center, vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(a)), vec3Scale(frame.v, frame.radius * Math.sin(a)))));
  }
  return pts;
}

/** 🔵️ Plane frame from center and one on-circle point (Z-up fallback when chord is vertical). */
export function arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null {
  const rs = vec3Sub(onCircle, center);
  const radius = vec3Length(rs);
  if (radius < 1e-9) return null;
  const u = vec3Normalize(rs);
  let axis: Vec3 = [0, 0, 1];
  if (Math.abs(vec3Dot(u, axis)) > 0.99) axis = [0, 1, 0];
  const v = vec3Normalize(vec3Cross(axis, u));
  const normal = vec3Normalize(vec3Cross(u, v));
  return { center, radius, normal, u, v };
}

/** 🔵️ On-circle arc end from pick direction (same sweep as preview / `arcSamplePoints`, not raw cursor). */
export function arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3 {
  const frame = arcPlaneFrame(center, start, pick);
  if (!frame) return pick;
  const sweep = arcSweepRadians(frame, pick);
  return vec3Add(frame.center, vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(sweep)), vec3Scale(frame.v, frame.radius * Math.sin(sweep))));
}

/** 🔵️ End point on arc at `angleDeg` from `start` about `center`. */
export function arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null {
  const frame = arcFrameFromRadiusPoint(center, start);
  if (!frame) return null;
  const radians = (angleDeg * Math.PI) / 180;
  return vec3Add(frame.center, vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(radians)), vec3Scale(frame.v, frame.radius * Math.sin(radians))));
}

/** ⭕️ Tessellates a full circle on plane `normal` through `center`. */
export function circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments = 64): readonly Vec3[] {
  const frame = arcFrameFromRadiusPoint(center, vec3Add(center, vec3Scale(vec3Normalize(normal), radius)));
  if (!frame) return [center];
  const n = Math.max(8, segments);
  const pts: Vec3[] = [];
  for (let i = 0; i <= n; i++) {
    const a = (i / n) * Math.PI * 2;
    pts.push(vec3Add(frame.center, vec3Add(vec3Scale(frame.u, frame.radius * Math.cos(a)), vec3Scale(frame.v, frame.radius * Math.sin(a)))));
  }
  return pts;
}

/** 🥚️ Tessellates an ellipse in the plane of `normal` / `majorAxis`. */
export function ellipseSamplePoints(center: Vec3, normal: Vec3, majorAxis: Vec3, majorRadius: number, minorRadius: number, segments = 64): readonly Vec3[] {
  const u = vec3Normalize(majorAxis);
  const v = vec3Normalize(vec3Cross(normal, u));
  const n = Math.max(8, segments);
  const pts: Vec3[] = [];
  for (let i = 0; i <= n; i++) {
    const a = (i / n) * Math.PI * 2;
    pts.push(vec3Add(center, vec3Add(vec3Scale(u, majorRadius * Math.cos(a)), vec3Scale(v, minorRadius * Math.sin(a)))));
  }
  return pts;
}

/** 📈️ Centripetal Catmull–Rom samples through `poles` (display / length estimate for nurbs curves). */
export function nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan = 12): readonly Vec3[] {
  if (poles.length < 2) return poles;
  if (poles.length === 2) return poles;
  const pts: Vec3[] = [];
  const n = poles.length;
  for (let i = 0; i < n - 1; i++) {
    const p0 = poles[Math.max(0, i - 1)]!;
    const p1 = poles[i]!;
    const p2 = poles[i + 1]!;
    const p3 = poles[Math.min(n - 1, i + 2)]!;
    const segs = i === n - 2 ? segmentsPerSpan : segmentsPerSpan;
    for (let j = 0; j < (i === n - 2 ? segs : segs); j++) {
      const t = j / segs;
      const t2 = t * t;
      const t3 = t2 * t;
      pts.push([
        0.5 * (2 * p1[0] + (-p0[0] + p2[0]) * t + (2 * p0[0] - 5 * p1[0] + 4 * p2[0] - p3[0]) * t2 + (-p0[0] + 3 * p1[0] - 3 * p2[0] + p3[0]) * t3),
        0.5 * (2 * p1[1] + (-p0[1] + p2[1]) * t + (2 * p0[1] - 5 * p1[1] + 4 * p2[1] - p3[1]) * t2 + (-p0[1] + 3 * p1[1] - 3 * p2[1] + p3[1]) * t3),
        0.5 * (2 * p1[2] + (-p0[2] + p2[2]) * t + (2 * p0[2] - 5 * p1[2] + 4 * p2[2] - p3[2]) * t2 + (-p0[2] + 3 * p1[2] - 3 * p2[2] + p3[2]) * t3),
      ]);
    }
  }
  pts.push(poles[n - 1]!);
  return pts;
}

/** 📏️ Polyline length from sampled points. */
export function polylineLength(points: readonly Vec3[]): number {
  let len = 0;
  for (let i = 1; i < points.length; i++) len += vec3Distance(points[i - 1]!, points[i]!);
  return len;
}

/** 📏️ Curve length from edge curve + boundary vertices (tessellated where non-linear). */
export function edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number {
  if (ends.length < 2) return 0;
  const c = curve ?? { kind: "line" as const };
  if (c.kind === "line") return vec3Distance(ends[0]!, ends[1]!);
  if (c.kind === "arc") {
    const frame = arcPlaneFrame(c.center, ends[0]!, ends[1]!);
    if (!frame) return vec3Distance(ends[0]!, ends[1]!);
    return frame.radius * arcSweepRadians(frame, ends[1]!);
  }
  if (c.kind === "circle") return Math.PI * 2 * c.radius;
  if (c.kind === "ellipse") {
    const h = c.majorRadius - c.minorRadius;
    return Math.PI * (3 * (c.majorRadius + c.minorRadius) - Math.sqrt((3 * c.majorRadius + h) * (c.majorRadius + 3 * h)));
  }
  if (c.kind === "nurbs") return polylineLength(nurbsDisplaySamplePoints(c.poles));
  return vec3Distance(ends[0]!, ends[1]!);
}

/** 🔵️ Samples points along an edge (exact curve tessellation, not vertex chord). */
export function edgeSamplePoints(vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments = 32): readonly Vec3[] {
  const ends = edge.vertexIds.map((id) => vertices[String(id)]?.position).filter((p): p is Vec3 => Boolean(p));
  if (ends.length < 1) return ends;
  const curve = edge.curve;
  if (!curve || curve.kind === "line") {
    if (ends.length >= 2) return ends;
    return ends;
  }
  if (curve.kind === "arc" && ends.length >= 2) return arcSamplePoints(curve.center, ends[0]!, ends[1]!, segments);
  if (curve.kind === "circle") return circleSamplePoints(curve.center, curve.normal, curve.radius, Math.max(segments, 64));
  if (curve.kind === "ellipse") {
    return ellipseSamplePoints(curve.center, curve.normal, curve.majorAxis, curve.majorRadius, curve.minorRadius, Math.max(segments, 64));
  }
  if (curve.kind === "nurbs") {
    const span = curve.through ? Math.max(12, curve.poles.length * 8) : Math.max(4, Math.ceil(segments / 4));
    return nurbsDisplaySamplePoints(curve.poles, span);
  }
  return ends.length >= 2 ? ends : ends;
}

/** ⭕️ Circle params from center and one on-circle point. */
export function circleFromCenterRadiusPoint(center: Vec3, radiusPoint: Vec3): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null {
  const frame = arcFrameFromRadiusPoint(center, radiusPoint);
  if (!frame) return null;
  return { center, normal: frame.normal, radius: frame.radius };
}

/** 📈️ Builds `EdgeCurve` nurbs from poles (`through` = interpolation points, else B-spline control points). */
export function nurbsCurveFromPoles(poles: readonly Vec3[], through = false): EdgeCurve | null {
  if (poles.length < 2) return null;
  const degree = Math.min(3, poles.length - 1);
  return { kind: "nurbs", poles, degree, through };
}
// #endregion 🌀️EdgeGeometry
// #endregion 🧮️SpatialKernelMath
