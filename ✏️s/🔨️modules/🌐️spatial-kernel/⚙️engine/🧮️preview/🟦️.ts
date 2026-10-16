// #region 🧲️Header
/** 🧮️ Kernel-independent spatial preview mathematics and model diff builders. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type MeshTransfer, type Vec3 } from "@semio-tech/framework-3d-js";
import type { ModelDiff, SpatialPreviewKernel } from "../🗺️spatial/🟦️.ts";
import { type EdgeRef, type FaceRef, type ShellRef, type SolidPrimitive, type SolidRef, type VertexRef, type WireRef } from "./🔷️brep-records/🟦️.ts";
import { constrainMovePoint, vec3Add, vec3Cross, vec3Distance, vec3Dot, vec3Length, vec3Normalize, vec3Scale, vec3Sub } from "./📏️vec3/🟦️.ts";
import { arcEndFromAngle, arcEndOnCircle, arcFrameFromRadiusPoint, arcPlaneFrame, arcSamplePoints, arcSweepRadians, circleFromCenterRadiusPoint, circleSamplePoints, edgeCurveLength, edgeSamplePoints, ellipseSamplePoints, nurbsCurveFromPoles, nurbsDisplaySamplePoints, polylineLength } from "./🌀️curve-sampling/🟦️.ts";
import { clampPointAlongDirection, faceCentroid, faceNormal, facePlaneGroupKey, fuseSolidsToExternalFaces, kernelFacePoints, projectPointOnScalarAxis, scalarTopOnAxis, solidFaceIds } from "./🔺️face-geometry/🟦️.ts";
import { aabbCornerPoints, aabbFromPoints, aabbIntersect, modelObjectAabb, solidPrimitiveAabb } from "./📦️aabb/🟦️.ts";
import { anchorPlacementFromEntity, evaluateAnchorPosition } from "./⚓️anchor-placement/🟦️.ts";
import { computeBoxPreviewLayout, transformPointsForPreviewKind } from "./🎚️preview-params/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🌀️EdgeGeometry
export function meshFaceModelDiff(mesh: MeshTransfer, idTag: string): ModelDiff {
  const pos = mesh.position;
  const ind = mesh.index;
  if (ind.length < 3 || pos.length < 9) return {};
  const i0 = ind[0]!;
  const i1 = ind[1]!;
  const i2 = ind[2]!;
  const a = [pos[i0 * 3]!, pos[i0 * 3 + 1]!, pos[i0 * 3 + 2]!] as Vec3;
  const b = [pos[i1 * 3]!, pos[i1 * 3 + 1]!, pos[i1 * 3 + 2]!] as Vec3;
  const c = [pos[i2 * 3]!, pos[i2 * 3 + 1]!, pos[i2 * 3 + 2]!] as Vec3;
  const ctr: Vec3 = [(a[0] + b[0] + c[0]) / 3, (a[1] + b[1] + c[1]) / 3, (a[2] + b[2] + c[2]) / 3];
  const eps = 0.04;
  const pfx = `cm-${idTag}`;
  const v0 = `${pfx}-w0` as VertexRef;
  const v1 = `${pfx}-w1` as VertexRef;
  const v2 = `${pfx}-w2` as VertexRef;
  const e0 = `${pfx}-e0` as EdgeRef;
  const e1 = `${pfx}-e1` as EdgeRef;
  const e2 = `${pfx}-e2` as EdgeRef;
  const wireId = `${pfx}-wire` as WireRef;
  const faceId = `${pfx}-face` as FaceRef;
  return {
    vertices: {
      added: [
        { id: v0, position: [ctr[0] + eps, ctr[1], ctr[2]] },
        { id: v1, position: [ctr[0], ctr[1] + eps, ctr[2]] },
        { id: v2, position: [ctr[0], ctr[1], ctr[2] + eps] },
      ],
    },
    edges: {
      added: [
        { id: e0, vertexIds: [v0, v1] },
        { id: e1, vertexIds: [v1, v2] },
        { id: e2, vertexIds: [v2, v0] },
      ],
    },
    wires: { added: [{ id: wireId, edgeIds: [e0, e1, e2] }] },
    faces: {
      added: [{ id: faceId, wireIds: [wireId] }],
    },
  };
}

/** 📦️ Full axis-aligned box model: 8 vertices, 12 edges, 6 wires, 6 faces, one shell, one solid. */
export function boxModelDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, solid: SolidRef): ModelDiff {
  const ax = Math.min(input.cornerA[0], input.cornerB[0]);
  const ay = Math.min(input.cornerA[1], input.cornerB[1]);
  const bx = Math.max(input.cornerA[0], input.cornerB[0]);
  const by = Math.max(input.cornerA[1], input.cornerB[1]);
  const z0 = Math.min(input.cornerA[2], input.cornerB[2]);
  const z1 = z0 + Math.max(Math.abs(input.height), 1e-9);
  const pfx = `box-${solid}`;
  const v000 = `${pfx}-v000` as VertexRef;
  const v100 = `${pfx}-v100` as VertexRef;
  const v110 = `${pfx}-v110` as VertexRef;
  const v010 = `${pfx}-v010` as VertexRef;
  const v001 = `${pfx}-v001` as VertexRef;
  const v101 = `${pfx}-v101` as VertexRef;
  const v111 = `${pfx}-v111` as VertexRef;
  const v011 = `${pfx}-v011` as VertexRef;
  const eb0 = `${pfx}-eb0` as EdgeRef;
  const eb1 = `${pfx}-eb1` as EdgeRef;
  const eb2 = `${pfx}-eb2` as EdgeRef;
  const eb3 = `${pfx}-eb3` as EdgeRef;
  const et0 = `${pfx}-et0` as EdgeRef;
  const et1 = `${pfx}-et1` as EdgeRef;
  const et2 = `${pfx}-et2` as EdgeRef;
  const et3 = `${pfx}-et3` as EdgeRef;
  const ev0 = `${pfx}-ev0` as EdgeRef;
  const ev1 = `${pfx}-ev1` as EdgeRef;
  const ev2 = `${pfx}-ev2` as EdgeRef;
  const ev3 = `${pfx}-ev3` as EdgeRef;
  const wb = `${pfx}-wire-bottom` as WireRef;
  const wt = `${pfx}-wire-top` as WireRef;
  const wy0 = `${pfx}-wire-y0` as WireRef;
  const wx1 = `${pfx}-wire-x1` as WireRef;
  const wy1 = `${pfx}-wire-y1` as WireRef;
  const wx0 = `${pfx}-wire-x0` as WireRef;
  const fb = `${pfx}-face-bottom` as FaceRef;
  const ft = `${pfx}-face-top` as FaceRef;
  const fy0 = `${pfx}-face-y0` as FaceRef;
  const fx1 = `${pfx}-face-x1` as FaceRef;
  const fy1 = `${pfx}-face-y1` as FaceRef;
  const fx0 = `${pfx}-face-x0` as FaceRef;
  const shell = `${pfx}-shell` as ShellRef;
  return {
    vertices: {
      added: [
        { id: v000, position: [ax, ay, z0] },
        { id: v100, position: [bx, ay, z0] },
        { id: v110, position: [bx, by, z0] },
        { id: v010, position: [ax, by, z0] },
        { id: v001, position: [ax, ay, z1] },
        { id: v101, position: [bx, ay, z1] },
        { id: v111, position: [bx, by, z1] },
        { id: v011, position: [ax, by, z1] },
      ],
    },
    edges: {
      added: [
        { id: eb0, vertexIds: [v000, v100] },
        { id: eb1, vertexIds: [v100, v110] },
        { id: eb2, vertexIds: [v110, v010] },
        { id: eb3, vertexIds: [v010, v000] },
        { id: et0, vertexIds: [v001, v101] },
        { id: et1, vertexIds: [v101, v111] },
        { id: et2, vertexIds: [v111, v011] },
        { id: et3, vertexIds: [v011, v001] },
        { id: ev0, vertexIds: [v000, v001] },
        { id: ev1, vertexIds: [v100, v101] },
        { id: ev2, vertexIds: [v110, v111] },
        { id: ev3, vertexIds: [v010, v011] },
      ],
    },
    wires: {
      added: [
        { id: wb, edgeIds: [eb0, eb1, eb2, eb3] },
        { id: wt, edgeIds: [et0, et1, et2, et3] },
        { id: wy0, edgeIds: [eb0, ev1, et0, ev0] },
        { id: wx1, edgeIds: [eb1, ev2, et1, ev1] },
        { id: wy1, edgeIds: [eb2, ev3, et2, ev2] },
        { id: wx0, edgeIds: [eb3, ev0, et3, ev3] },
      ],
    },
    faces: {
      added: [
        { id: fb, wireIds: [wb] },
        { id: ft, wireIds: [wt] },
        { id: fy0, wireIds: [wy0] },
        { id: fx1, wireIds: [wx1] },
        { id: fy1, wireIds: [wy1] },
        { id: fx0, wireIds: [wx0] },
      ],
    },
    shells: { added: [{ id: shell, faceIds: [fb, ft, fy0, fx1, fy1, fx0] }] },
    solids: {
      added: [
        {
          id: solid,
          shellIds: [shell],
          solid: { kind: "box", cornerA: [ax, ay, z0], cornerB: [bx, by, z0], height: z1 - z0 } satisfies SolidPrimitive,
        },
      ],
    },
  };
}
// #endregion 🌀️EdgeGeometry

/** 🔌️ Precise `SpatialPreviewKernel` (delegates to module functions). */
export class PreciseSpatialKernelMath implements SpatialPreviewKernel {
  vec3Add = vec3Add;
  vec3Sub = vec3Sub;
  vec3Scale = vec3Scale;
  vec3Dot = vec3Dot;
  vec3Cross = vec3Cross;
  vec3Length = vec3Length;
  vec3Distance = vec3Distance;
  vec3Normalize = vec3Normalize;
  arcPlaneFrame = arcPlaneFrame;
  arcSweepRadians = arcSweepRadians;
  arcSamplePoints = arcSamplePoints;
  arcFrameFromRadiusPoint = arcFrameFromRadiusPoint;
  arcEndOnCircle = arcEndOnCircle;
  arcEndFromAngle = arcEndFromAngle;
  circleSamplePoints = circleSamplePoints;
  ellipseSamplePoints = ellipseSamplePoints;
  nurbsDisplaySamplePoints = nurbsDisplaySamplePoints;
  polylineLength = polylineLength;
  edgeCurveLength = edgeCurveLength;
  edgeSamplePoints = edgeSamplePoints;
  circleFromCenterRadiusPoint = circleFromCenterRadiusPoint;
  nurbsCurveFromPoles = nurbsCurveFromPoles;
  aabbFromPoints = (pts: readonly Vec3[]) => aabbFromPoints(pts, 0);
  aabbCornerPoints = aabbCornerPoints;
  aabbIntersect = aabbIntersect;
  solidPrimitiveAabb = solidPrimitiveAabb;
  modelObjectAabb = modelObjectAabb;
  boxModelDiff = boxModelDiff;
  meshFaceModelDiff = meshFaceModelDiff;
  evaluateAnchorPosition = evaluateAnchorPosition;
  anchorPlacementFromEntity = anchorPlacementFromEntity;
  computeBoxPreviewLayout = computeBoxPreviewLayout;
  transformPointsForPreviewKind = transformPointsForPreviewKind;
  constrainMovePoint = constrainMovePoint;
  facePoints = kernelFacePoints;
  faceCentroid = faceCentroid;
  faceNormal = faceNormal;
  solidFaceIds = solidFaceIds;
  fuseSolidsToExternalFaces = fuseSolidsToExternalFaces;
  facePlaneGroupKey = facePlaneGroupKey;
  projectPointOnScalarAxis = projectPointOnScalarAxis;
  scalarTopOnAxis = scalarTopOnAxis;
  clampPointAlongDirection = clampPointAlongDirection;
  abs = Math.abs;
  min2 = (a: number, b: number) => (a < b ? a : b);
  max2 = (a: number, b: number) => (a > b ? a : b);
  minN = (nums: readonly number[]) => nums.reduce((m, n) => (n < m ? n : m), nums[0] ?? 0);
  maxN = (nums: readonly number[]) => nums.reduce((m, n) => (n > m ? n : m), nums[0] ?? 0);
  hypot3 = (x: number, y: number, z: number) => Math.hypot(x, y, z);
  atan2 = Math.atan2;
  cos = Math.cos;
  sin = Math.sin;
  randomTag = (prefix: string) => `${prefix}-${crypto.randomUUID().slice(0, 8)}`;
}

export const preciseSpatialKernelMath = new PreciseSpatialKernelMath();
// #endregion 🧮️SpatialKernelMath
