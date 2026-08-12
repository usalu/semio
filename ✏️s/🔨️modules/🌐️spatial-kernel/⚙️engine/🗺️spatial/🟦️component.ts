// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/kernel-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/kernel-3d-js";
// #endregion 🧲️Header


import { EdgeRef, FaceRef, Model, ShellRef, SolidRef, VertexRef, WireRef } from "../📐️geometry/🟦️component.ts";



// #region 📦️🗺️spatial
// #region 🧮️Diff
export type AnchorRecordDiff = { readonly id: AnchorRef } & Partial<Pick<AnchorRecord, "position" | "attachment">>;
export type VertexRecordDiff = { readonly id: VertexRef } & Partial<Pick<VertexRecord, "position">>;
export type EdgeRecordDiff = { readonly id: EdgeRef } & Partial<Pick<EdgeRecord, "vertexIds" | "curve">>;
export type WireRecordDiff = { readonly id: WireRef } & Partial<Pick<WireRecord, "edgeIds">>;
export type FaceRecordDiff = { readonly id: FaceRef } & Partial<Pick<FaceRecord, "wireIds" | "surface">>;
export type ShellRecordDiff = { readonly id: ShellRef } & Partial<Pick<ShellRecord, "faceIds">>;
export type SolidRecordDiff = { readonly id: SolidRef } & Partial<Pick<SolidRecord, "shellIds" | "solid">>;
/** @emoji 🧮️ Forward patch bucket for one geometry table (`added` / `modified` / `removed` arrays). */
export interface EntityDiff<TRec, TDiff, TId extends string> {
  readonly added?: readonly TRec[];
  readonly modified?: readonly TDiff[];
  readonly removed?: readonly TId[];
}

/** @emoji 🧮️ Serializable model diff applied by `applyModelDiff`. */
export interface ModelDiff {
  readonly anchors?: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef>;
  readonly vertices?: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef>;
  readonly edges?: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef>;
  readonly wires?: EntityDiff<WireRecord, WireRecordDiff, WireRef>;
  readonly faces?: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef>;
  readonly shells?: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef>;
  readonly solids?: EntityDiff<SolidRecord, SolidRecordDiff, SolidRef>;
}

export const EMPTY_MODEL_DIFF: ModelDiff = {};

function isEntityDiffEmpty<TRec, TDiff, TId extends string>(e: EntityDiff<TRec, TDiff, TId> | undefined): boolean {
  if (!e) return true;
  const a = e.added?.length ?? 0;
  const m = e.modified?.length ?? 0;
  const r = e.removed?.length ?? 0;
  return a === 0 && m === 0 && r === 0;
}

/** @emoji 🧮️ True when `diff` has no geometry mutations. */
export function isEmptyModelDiff(d: ModelDiff | undefined): boolean {
  if (!d) return true;
  return isEntityDiffEmpty(d.anchors) && isEntityDiffEmpty(d.vertices) && isEntityDiffEmpty(d.edges) && isEntityDiffEmpty(d.wires) && isEntityDiffEmpty(d.faces) && isEntityDiffEmpty(d.shells) && isEntityDiffEmpty(d.solids);
}

function cloneRec<T>(r: T): T {
  return JSON.parse(JSON.stringify(r)) as T;
}

function applyEntityDiff<T extends { id: string }, TDiff extends { id: string }>(bucket: Record<string, T>, section: EntityDiff<T, TDiff, string> | undefined, inverse: EntityDiff<T, TDiff, string>): void {
  if (!section) return;
  if (section.removed) {
    for (const id of section.removed) {
      const cur = bucket[id];
      if (!cur) continue;
      if (!inverse.added) inverse.added = [];
      (inverse.added as T[]).push(cloneRec(cur));
      delete bucket[id];
    }
  }
  if (section.added) {
    for (const rec of section.added) {
      const id = rec.id;
      bucket[id] = cloneRec(rec as T);
      if (!inverse.removed) inverse.removed = [];
      (inverse.removed as string[]).push(id);
    }
  }
  if (section.modified) {
    for (const md of section.modified) {
      const id = md.id;
      const cur = bucket[id];
      if (!cur) continue;
      const back: Record<string, unknown> = { id };
      const curO = cur as Record<string, unknown>;
      const mdO = md as Record<string, unknown>;
      for (const fk of Object.keys(mdO)) {
        if (fk === "id") continue;
        back[fk] = curO[fk];
        curO[fk] = mdO[fk];
      }
      if (!inverse.modified) inverse.modified = [];
      (inverse.modified as TDiff[]).push(back as TDiff);
    }
  }
}

/** @emoji 🧮️ Applies `diff` to `model` in place; returns an inverse `ModelDiff` for `applyModelDiff` again. */
export function applyModelDiff(model: Model, diff: ModelDiff): ModelDiff {
  const inv: ModelDiff = {};
  const aInv: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef> = {};
  const vInv: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef> = {};
  const eInv: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
  const wInv: EntityDiff<WireRecord, WireRecordDiff, WireRef> = {};
  const fInv: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef> = {};
  const sInv: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef> = {};
  const cInv: EntityDiff<SolidRecord, SolidRecordDiff, SolidRef> = {};
  applyEntityDiff(model.anchors as Record<string, AnchorRecord>, diff.anchors, aInv);
  applyEntityDiff(model.vertices as Record<string, VertexRecord>, diff.vertices, vInv);
  applyEntityDiff(model.edges as Record<string, EdgeRecord>, diff.edges, eInv);
  applyEntityDiff(model.wires as Record<string, WireRecord>, diff.wires, wInv);
  applyEntityDiff(model.faces as Record<string, FaceRecord>, diff.faces, fInv);
  applyEntityDiff(model.shells as Record<string, ShellRecord>, diff.shells, sInv);
  applyEntityDiff(model.solids as Record<string, SolidRecord>, diff.solids, cInv);
  const movedVertexIds = diff.vertices?.modified?.map((row) => row.id) ?? [];
  let nurbsEdgeSyncApplied = false;
  if (movedVertexIds.length > 0) {
    const nurbsSync = modelDiffSyncNurbsThroughEdgesForMovedVertices(model, movedVertexIds);
    if (!isEmptyModelDiff(nurbsSync)) {
      const eInvSync: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
      applyEntityDiff(model.edges as Record<string, EdgeRecord>, nurbsSync.edges, eInvSync);
      if (!isEntityDiffEmpty(eInvSync)) {
        nurbsEdgeSyncApplied = true;
        inv.edges = inv.edges
          ? {
              added: [...(inv.edges.added ?? []), ...(eInvSync.added ?? [])],
              modified: [...(inv.edges.modified ?? []), ...(eInvSync.modified ?? [])],
              removed: [...(inv.edges.removed ?? []), ...(eInvSync.removed ?? [])],
            }
          : eInvSync;
      }
    }
  }
  if (!isEntityDiffEmpty(aInv)) inv.anchors = aInv;
  if (!isEntityDiffEmpty(vInv)) inv.vertices = vInv;
  if (!isEntityDiffEmpty(eInv)) inv.edges = eInv;
  if (!isEntityDiffEmpty(wInv)) inv.wires = wInv;
  if (!isEntityDiffEmpty(fInv)) inv.faces = fInv;
  if (!isEntityDiffEmpty(sInv)) inv.shells = sInv;
  if (!isEntityDiffEmpty(cInv)) inv.solids = cInv;
  if (!isEmptyModelDiff(diff) || nurbsEdgeSyncApplied) model.bump();
  return inv;
}

// #region 🔌️SpatialKernelInterface
export type Aabb = { readonly min: Vec3; readonly max: Vec3 };

/** @emoji ⚡️ Fast approximate preview math (sync); subset of `SpatialKernel`. */
export interface SpatialPreviewKernel {
  vec3Add(a: Vec3, b: Vec3): Vec3;
  vec3Sub(a: Vec3, b: Vec3): Vec3;
  vec3Scale(a: Vec3, s: number): Vec3;
  vec3Dot(a: Vec3, b: Vec3): number;
  vec3Cross(a: Vec3, b: Vec3): Vec3;
  vec3Length(a: Vec3): number;
  vec3Distance(a: Vec3, b: Vec3): number;
  vec3Normalize(a: Vec3): Vec3;
  arcPlaneFrame(center: Vec3, start: Vec3, end: Vec3): ArcPlaneFrame | null;
  arcSweepRadians(frame: ArcPlaneFrame, end: Vec3): number;
  arcSamplePoints(center: Vec3, start: Vec3, end: Vec3, segments?: number): readonly Vec3[];
  arcFrameFromRadiusPoint(center: Vec3, onCircle: Vec3): ArcPlaneFrame | null;
  arcEndOnCircle(center: Vec3, start: Vec3, pick: Vec3): Vec3;
  arcEndFromAngle(center: Vec3, start: Vec3, angleDeg: number): Vec3 | null;
  circleSamplePoints(center: Vec3, normal: Vec3, radius: number, segments?: number): readonly Vec3[];
  ellipseSamplePoints(center: Vec3, normal: Vec3, majorAxis: Vec3, majorRadius: number, minorRadius: number, segments?: number): readonly Vec3[];
  nurbsDisplaySamplePoints(poles: readonly Vec3[], segmentsPerSpan?: number): readonly Vec3[];
  polylineLength(points: readonly Vec3[]): number;
  edgeCurveLength(curve: EdgeCurve | undefined, ends: readonly Vec3[]): number;
  edgeSamplePoints(vertices: Readonly<Record<string, VertexRecord>>, edge: EdgeRecord, segments?: number): readonly Vec3[];
  circleFromCenterRadiusPoint(center: Vec3, radiusPoint: Vec3): { readonly center: Vec3; readonly normal: Vec3; readonly radius: number } | null;
  nurbsCurveFromPoles(poles: readonly Vec3[], through?: boolean): EdgeCurve | null;
  aabbFromPoints(points: readonly Vec3[]): Aabb | null;
  aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[];
  aabbIntersect(a: Aabb, b: Aabb): Aabb | null;
  solidPrimitiveAabb(solid: SolidPrimitive): Aabb;
  modelObjectAabb(model: Model, solid: SolidRecord): Aabb | null;
  boxModelDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }, solid: SolidRef): ModelDiff;
  meshFaceModelDiff(mesh: MeshTransfer, idTag: string): ModelDiff;
  evaluateAnchorPosition(model: Model, anchor: AnchorRecord): Vec3;
  anchorPlacementFromEntity(model: Model, kind: AnchorAttachment["kind"], id: string, point: Vec3): { readonly position: Vec3; readonly attachment: AnchorAttachment } | null;
  computeBoxPreviewLayout(cornerA: Vec3, cornerB: Vec3, height: number): { readonly position: Vec3; readonly scale: Vec3 };
  transformPointsForPreviewKind(previewKind: string, params: Record<string, unknown>): (point: Vec3) => Vec3;
  constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal?: Vec3): Vec3;
  facePoints(model: Model, face: FaceRecord): readonly Vec3[];
  faceCentroid(model: Model, face: FaceRecord): Vec3 | null;
  faceNormal(model: Model, face: FaceRecord): Vec3 | null;
  solidFaceIds(model: Model, solidId: string): readonly FaceRef[];
  fuseSolidsToExternalFaces(
    model: Model,
    solidRefs: readonly SolidRef[],
    options?: { readonly hullSolidId?: string; readonly contactPairs?: readonly (readonly [string, string])[]; readonly maxSeparation?: number },
  ): { readonly hullSolid: SolidRef; readonly externalFaces: readonly FaceRef[] };
  facePlaneGroupKey(normal: Vec3, centroid: Vec3): string;
  projectPointOnScalarAxis(base: Vec3, axis: Vec3, raw: Vec3): { readonly projected: Vec3; readonly t: number };
  scalarTopOnAxis(base: Vec3, axis: Vec3, height: number, signedT: number): Vec3;
  clampPointAlongDirection(anchor: Vec3, target: Vec3, length: number): Vec3;
  abs(x: number): number;
  min2(a: number, b: number): number;
  max2(a: number, b: number): number;
  minN(nums: readonly number[]): number;
  maxN(nums: readonly number[]): number;
  hypot3(x: number, y: number, z: number): number;
  atan2(y: number, x: number): number;
  cos(a: number): number;
  sin(a: number): number;
  randomTag(prefix: string): string;
}

/** @emoji 🔌️ Precise BREP kernel: preview math + construction, tessellation, derived views. */
export interface SpatialKernel extends SpatialPreviewKernel {
  readonly id: string;
  readonly operations: readonly string[];
  createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef>;
  volume(solid: SolidRef): Promise<number>;
  tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer>;
  query?(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown>;
  executeAction?(
    actionId: string,
    params: Record<string, unknown>,
    args: Record<string, unknown>,
    ctx: {
      readonly model: Model;
      readonly preview: SpatialPreviewKernel;
      readonly activeModelDefinitionId?: string | null;
    },
  ): Promise<ActionResult> | ActionResult;
  executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }>;
  extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef>;
  offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void>;
  createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }>;
  extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }>;
  offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }>;
  vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number>;
  edgeLength(e: EdgeRef, model: Model): Promise<number>;
  faceArea(f: FaceRef, model: Model): Promise<number>;
  syncSolidsFromModel(model: Model): Promise<void>;
  solidVolume(c: SolidRef): Promise<number>;
  adjacentSolids(solid: SolidRef, model: Model): Promise<readonly SolidRef[]>;
  sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]>;
}

/** @emoji 🧱️ Appends a tessellated commit as one mesh `face` on `Model` (in-memory scene growth). */
export function appendCommittedMeshFaceToModel(model: Model, mesh: MeshTransfer, idTag: string, math: SpatialPreviewKernel): void {
  applyModelDiff(model, math.meshFaceModelDiff(mesh, idTag));
}

/** @emoji 🔌️ Optional query context for kernel adapters. */
export interface KernelQueryContext {
  readonly model: Model;
  readonly activeModelDefinitionId?: string | null;
}
// #endregion 🔌️SpatialKernelInterface

// #endregion 📦️🗺️spatial
