/** 🔺️ Face and solid geometry
 *
 * Points, centroids, normals and contact fusion of kernel faces and solids, plus the scalar-axis helpers.
 */

// #region 🔌️Adapters
import { type Vec3 } from "@semio-tech/framework-3d-js";
import { Model } from "../../📐️geometry/🧊️model/🟦️.ts";
import { type FaceRecord, type FaceRef, geom, type SolidRecord, type SolidRef } from "../🔷️brep-records/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🌀️EdgeGeometry
export function derivedFacePoints(model: Model, face: FaceRecord): readonly Vec3[] {
  const points = face.wireIds.flatMap((wireId) => {
    const wire = geom(model).wires[wireId];
    return (wire?.edgeIds ?? []).flatMap((edgeId) => {
      const edge = geom(model).edges[edgeId];
      return (edge?.vertexIds ?? []).map((vertexId) => geom(model).vertices[vertexId]?.position).filter((p): p is Vec3 => Boolean(p));
    });
  });
  return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

export function derivedPointCentroid(points: readonly Vec3[]): Vec3 | null {
  if (points.length === 0) return null;
  const sum = points.reduce((acc, p) => [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]] as unknown as Vec3, [0, 0, 0] as unknown as Vec3);
  return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length] as unknown as Vec3;
}

export function derivedSolidPoints(model: Model, cell: SolidRecord): readonly Vec3[] {
  const points = cell.shellIds.flatMap((shellId) => {
    const shell = geom(model).shells[shellId];
    return (shell?.faceIds ?? []).flatMap((faceId: FaceRef) => {
      const face = geom(model).faces[faceId];
      return face ? derivedFacePoints(model, face) : [];
    });
  });
  return [...new Map(points.map((p) => [p.join(","), p])).values()];
}
// #endregion 🌀️EdgeGeometry

// #region 🧱️PrimitivePreviewGeometry
export function kernelFacePoints(model: Model, face: FaceRecord): readonly Vec3[] {
  const g = geom(model);
  const pts: Vec3[] = [];
  for (const wid of face.wireIds) {
    for (const eid of g.wires[wid]?.edgeIds ?? []) {
      for (const vid of g.edges[eid]?.vertexIds ?? []) {
        const p = g.vertices[vid]?.position;
        if (p) pts.push(p);
      }
    }
  }
  return [...new Map(pts.map((p) => [p.join(","), p])).values()];
}

/** 📍️ Face vertex centroid for preview primitive operations. */
export function faceCentroid(model: Model, face: FaceRecord): Vec3 | null {
  const pts = kernelFacePoints(model, face);
  if (!pts.length) return null;
  let x = 0;
  let y = 0;
  let z = 0;
  for (const p of pts) {
    x += p[0];
    y += p[1];
    z += p[2];
  }
  const n = pts.length;
  return [x / n, y / n, z / n];
}

function kernelFaceNormalFromId(faceId: string): Vec3 | null {
  if (faceId.includes("face-top")) return [0, 0, 1];
  if (faceId.includes("face-bottom")) return [0, 0, -1];
  if (faceId.includes("face-x0")) return [-1, 0, 0];
  if (faceId.includes("face-x1")) return [1, 0, 0];
  if (faceId.includes("face-y0")) return [0, -1, 0];
  if (faceId.includes("face-y1")) return [0, 1, 0];
  return null;
}

/** 📐️ Unit face normal from surface or boundary winding. */
export function faceNormal(model: Model, face: FaceRecord): Vec3 | null {
  if (face.surface?.kind === "plane") {
    const n = face.surface.normal;
    const len = Math.hypot(n[0], n[1], n[2]);
    return len > 1e-9 ? ([n[0] / len, n[1] / len, n[2] / len] as Vec3) : null;
  }
  const pts = kernelFacePoints(model, face);
  if (pts.length >= 3) {
    let nx = 0;
    let ny = 0;
    let nz = 0;
    for (let i = 0; i < pts.length; i++) {
      const p0 = pts[i]!;
      const p1 = pts[(i + 1) % pts.length]!;
      nx += (p0[1] - p1[1]) * (p0[2] + p1[2]);
      ny += (p0[2] - p1[2]) * (p0[0] + p1[0]);
      nz += (p0[0] - p1[0]) * (p0[1] + p1[1]);
    }
    const len = Math.hypot(nx, ny, nz);
    if (len > 1e-9) return [nx / len, ny / len, nz / len];
  }
  return kernelFaceNormalFromId(String(face.id));
}

/** 🧱️ Face ids referenced by one solid shell graph. */
export function solidFaceIds(model: Model, solidId: string): readonly FaceRef[] {
  const g = geom(model);
  const solid = g.solids[solidId];
  if (!solid) return [];
  const out: FaceRef[] = [];
  const seen = new Set<string>();
  for (const shellId of solid.shellIds) {
    for (const faceId of g.shells[shellId]?.faceIds ?? []) {
      const key = String(faceId);
      if (seen.has(key)) continue;
      seen.add(key);
      out.push(faceId);
    }
  }
  return out;
}

function kernelFacesAreContactPair(
  a: { readonly face: FaceRef; readonly solid: string; readonly centroid: Vec3 },
  b: { readonly face: FaceRef; readonly solid: string; readonly centroid: Vec3 },
  contactPairs: readonly (readonly [string, string])[],
  maxSeparation: number,
): boolean {
  if (a.solid === b.solid) return false;
  const aId = String(a.face);
  const bId = String(b.face);
  let suffixMatch = false;
  for (const [suffixA, suffixB] of contactPairs) {
    if (aId.endsWith(suffixA) && bId.endsWith(suffixB)) {
      suffixMatch = true;
      break;
    }
  }
  if (!suffixMatch) return false;
  const sep = Math.hypot(a.centroid[0] - b.centroid[0], a.centroid[1] - b.centroid[1], a.centroid[2] - b.centroid[2]);
  return sep <= maxSeparation;
}

/** 🧱️ Fuses stacked solids and returns external face ids plus hull solid ref. */
export function fuseSolidsToExternalFaces(
  model: Model,
  solidRefs: readonly SolidRef[],
  options?: { readonly hullSolidId?: string; readonly contactPairs?: readonly (readonly [string, string])[]; readonly maxSeparation?: number },
): { readonly hullSolid: SolidRef; readonly externalFaces: readonly FaceRef[] } {
  const contactPairs = options?.contactPairs ?? [
    ["face-top", "face-bottom"],
    ["face-bottom", "face-top"],
  ];
  const maxSeparation = options?.maxSeparation ?? 0.05;
  const hullSolidId = (options?.hullSolidId ?? "from_geometry-hull") as SolidRef;
  const allFaces = solidRefs.flatMap((solidId) => solidFaceIds(model, solidId));
  const internal = new Set<string>();
  for (let i = 0; i < solidRefs.length; i++) {
    for (let j = i + 1; j < solidRefs.length; j++) {
      const solidA = solidRefs[i]!;
      const solidB = solidRefs[j]!;
      for (const faceA of solidFaceIds(model, solidA)) {
        const centroidA = faceCentroid(model, geom(model).faces[faceA]!);
        if (!centroidA) continue;
        for (const faceB of solidFaceIds(model, solidB)) {
          const centroidB = faceCentroid(model, geom(model).faces[faceB]!);
          if (!centroidB) continue;
          if (!kernelFacesAreContactPair({ face: faceA, solid: solidA, centroid: centroidA }, { face: faceB, solid: solidB, centroid: centroidB }, contactPairs, maxSeparation)) continue;
          internal.add(String(faceA));
          internal.add(String(faceB));
        }
      }
    }
  }
  const externalFaces = allFaces.filter((faceId) => !internal.has(String(faceId)));
  const hullSolid: SolidRef = solidRefs.length === 1 ? solidRefs[0]! : hullSolidId;
  return { hullSolid, externalFaces };
}

/** 📐️ Groups coplanar faces for merged object rows. */
export function facePlaneGroupKey(normal: Vec3, centroid: Vec3): string {
  const ax = Math.abs(normal[0]);
  const ay = Math.abs(normal[1]);
  const az = Math.abs(normal[2]);
  if (az >= ax && az >= ay) {
    const sign = normal[2] >= 0 ? "+" : "-";
    return `z:${Math.round(centroid[2] * 1000)}:${sign}`;
  }
  if (ax >= ay) {
    const sign = normal[0] >= 0 ? "+" : "-";
    return `x:${Math.round(centroid[0] * 1000)}:${sign}`;
  }
  const sign = normal[1] >= 0 ? "+" : "-";
  return `y:${Math.round(centroid[1] * 1000)}:${sign}`;
}

/** 📏️ Projects `raw` onto the scalar axis; returns axis parameter `t` and closest point. */
export function projectPointOnScalarAxis(base: Vec3, axis: Vec3, raw: Vec3): { readonly projected: Vec3; readonly t: number } {
  const ax = axis[0];
  const ay = axis[1];
  const az = axis[2];
  const len = Math.hypot(ax, ay, az) || 1;
  const ux = ax / len;
  const uy = ay / len;
  const uz = az / len;
  const t = (raw[0] - base[0]) * ux + (raw[1] - base[1]) * uy + (raw[2] - base[2]) * uz;
  return {
    projected: [base[0] + ux * t, base[1] + uy * t, base[2] + uz * t],
    t,
  };
}

/** 📏️ Point at `height` along `axis` from `base` using signed axis parameter. */
export function scalarTopOnAxis(base: Vec3, axis: Vec3, height: number, signedT: number): Vec3 {
  const ax = axis[0];
  const ay = axis[1];
  const az = axis[2];
  const len = Math.hypot(ax, ay, az) || 1;
  const ux = ax / len;
  const uy = ay / len;
  const uz = az / len;
  const sign = signedT < 0 ? -1 : 1;
  return [base[0] + ux * height * sign, base[1] + uy * height * sign, base[2] + uz * height * sign];
}

/** 📏️ Clamps `target` to `length` units from `anchor` along the anchor→target ray. */
export function clampPointAlongDirection(anchor: Vec3, target: Vec3, length: number): Vec3 {
  const dx = target[0] - anchor[0];
  const dy = target[1] - anchor[1];
  const dz = target[2] - anchor[2];
  const d = Math.hypot(dx, dy, dz);
  if (d < 1e-9) return [target[0], target[1], target[2]];
  const s = length / d;
  return [anchor[0] + dx * s, anchor[1] + dy * s, anchor[2] + dz * s];
}
// #endregion 🧱️PrimitivePreviewGeometry
// #endregion 🧮️SpatialKernelMath
