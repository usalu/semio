// #region 🧲️Header
/** @emoji 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import { ephemeralBox, ephemeralMap, ephemeralWeakMap } from "@semio-tech/framework";
import type { ArcPlaneFrame, EdgeCurve, EdgeGroup, EdgeInfo, FaceGroup, FaceInfo, MeshTransfer, Vec3 } from "@semio-tech/s-3d-js";
import { emptyMeshTransfer, kernelGeometry, solidRef } from "@semio-tech/s-3d-js";
// #endregion 🧲️Header


import { AnchorAttachment, AnchorRecord, AnchorRef, EdgeRecord, EdgeRef, FaceRecord, FaceRef, Model, ShellRecord, ShellRef, SolidPrimitive, SolidRecord, SolidRef, TypologyRef, VertexRecord, VertexRef, WireRecord, WireRef } from "../📐️geometry/🟦️.ts";



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

/** @emoji 🧮️ Mutable in-progress view of `EntityDiff` used while accumulating an inverse patch. */
interface MutableEntityDiff<TRec, TDiff, TId extends string> {
  added?: TRec[];
  modified?: TDiff[];
  removed?: TId[];
}

/** @emoji 🧮️ Mutable in-progress view of `ModelDiff` used while accumulating an inverse patch. */
interface MutableModelDiff {
  anchors?: EntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef>;
  vertices?: EntityDiff<VertexRecord, VertexRecordDiff, VertexRef>;
  edges?: EntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef>;
  wires?: EntityDiff<WireRecord, WireRecordDiff, WireRef>;
  faces?: EntityDiff<FaceRecord, FaceRecordDiff, FaceRef>;
  shells?: EntityDiff<ShellRecord, ShellRecordDiff, ShellRef>;
  solids?: EntityDiff<SolidRecord, SolidRecordDiff, SolidRef>;
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

function vec3Eq(a: Vec3, b: Vec3): boolean {
  return a[0] === b[0] && a[1] === b[1] && a[2] === b[2];
}

/** @emoji 🧮️ Re-poles through-nurbs edges anchored on `movedVertexIds` so their endpoints track the live vertex positions. */
function modelDiffSyncNurbsThroughEdgesForMovedVertices(model: Model, movedVertexIds: readonly VertexRef[]): ModelDiff {
  const moved = new Set(movedVertexIds.map(String));
  const edgeMods: EdgeRecordDiff[] = [];
  for (const edge of Object.values(model.edges)) {
    const curve = edge.curve;
    if (curve?.kind !== "nurbs" || !curve.through || curve.poles.length < 2) continue;
    const startId = String(edge.vertexIds[0] ?? "");
    const endId = String(edge.vertexIds[1] ?? edge.vertexIds[0] ?? "");
    let poles: Vec3[] | null = null;
    if (startId && moved.has(startId)) {
      const position = model.vertices[startId as VertexRef]?.position;
      if (position) {
        poles = [...curve.poles];
        poles[0] = [position[0], position[1], position[2]];
      }
    }
    if (endId && moved.has(endId)) {
      const position = model.vertices[endId as VertexRef]?.position;
      if (position) {
        poles = poles ? [...poles] : [...curve.poles];
        poles[poles.length - 1] = [position[0], position[1], position[2]];
      }
    }
    if (!poles) continue;
    if (poles.every((point, index) => vec3Eq(point, curve.poles[index]!))) continue;
    edgeMods.push({ id: edge.id, curve: { ...curve, poles } });
  }
  return edgeMods.length ? { edges: { modified: edgeMods } } : EMPTY_MODEL_DIFF;
}

function applyEntityDiff<T extends { id: string }, TDiff extends { id: string }>(bucket: Record<string, T>, section: EntityDiff<T, TDiff, string> | undefined, inverse: MutableEntityDiff<T, TDiff, string>): void {
  if (!section) return;
  if (section.removed) {
    for (const id of section.removed) {
      const cur = bucket[id];
      if (!cur) continue;
      if (!inverse.added) inverse.added = [];
      inverse.added.push(cloneRec(cur));
      delete bucket[id];
    }
  }
  if (section.added) {
    for (const rec of section.added) {
      const id = rec.id;
      bucket[id] = cloneRec(rec as T);
      if (!inverse.removed) inverse.removed = [];
      inverse.removed.push(id);
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
      inverse.modified.push(back as TDiff);
    }
  }
}

/** @emoji 🧮️ Applies `diff` to `model` in place; returns an inverse `ModelDiff` for `applyModelDiff` again. */
export function applyModelDiff(model: Model, diff: ModelDiff): ModelDiff {
  const inv: MutableModelDiff = {};
  const aInv: MutableEntityDiff<AnchorRecord, AnchorRecordDiff, AnchorRef> = {};
  const vInv: MutableEntityDiff<VertexRecord, VertexRecordDiff, VertexRef> = {};
  const eInv: MutableEntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
  const wInv: MutableEntityDiff<WireRecord, WireRecordDiff, WireRef> = {};
  const fInv: MutableEntityDiff<FaceRecord, FaceRecordDiff, FaceRef> = {};
  const sInv: MutableEntityDiff<ShellRecord, ShellRecordDiff, ShellRef> = {};
  const cInv: MutableEntityDiff<SolidRecord, SolidRecordDiff, SolidRef> = {};
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
      const eInvSync: MutableEntityDiff<EdgeRecord, EdgeRecordDiff, EdgeRef> = {};
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

/** @emoji 🧩️ Serializable context patch applied after pure box geometry actions (`set` keys merged; `del` removes top-level context keys). */
export interface ActionContextPatch {
  readonly set?: Record<string, unknown>;
  readonly del?: readonly string[];
}

/** @emoji 🧩️ Pure action output: model `diff` is the committed geometry; optional `data` is auxiliary; `patch` updates session context only. */
export interface ActionResult<TData = unknown> {
  readonly diff?: ModelDiff;
  readonly data?: TData;
  readonly patch?: ActionContextPatch;
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

// #region 🧪️Tests
import { ObjectRef, SelectionTarget, deletableObjectIdsFromSelection, deleteObjectsFromModel } from "../📐️geometry/🟦️.ts";

const __spatialCoreTestRuntime = import.meta.vitest ? await import("../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts") : null;
const __spatialCoreTestKernel = import.meta.vitest ? await import("../🧱️brepjs/🟦️.ts") : null;

if (import.meta.vitest) {
  __spatialCoreTestRuntime!.bootstrapCadModules();
  const { preciseSpatialKernelMath } = __spatialCoreTestKernel!;
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/cad-js/core model commit mesh", () => {
    it("appendCommittedMeshFaceToModel adds one mesh face from a triangle mesh", () => {
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      appendCommittedMeshFaceToModel(g, mesh, "t0", M);
      expect(Object.keys(g.faces).length).toBe(1);
      expect(g.revision).toBeGreaterThan(0);
    });
  });
  describe("@semio-tech/cad-js/core model diff", () => {
    it("applyModelDiff then inverse restores counts", () => {
      const g = new Model();
      const mesh: MeshTransfer = {
        position: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]),
        normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]),
        index: new Uint32Array([0, 1, 2]),
        edges: new Float32Array(0),
        faceGroups: [],
        edgeGroups: [],
        faceInfos: [],
        edgeInfos: [],
      };
      const d = M.meshFaceModelDiff(mesh, "x");
      const inv = applyModelDiff(g, d);
      expect(Object.keys(g.faces).length).toBe(1);
      applyModelDiff(g, inv);
      expect(Object.keys(g.faces).length).toBe(0);
    });

    it("boxModelDiff creates selectable boundary and volume records", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, solidRef("box-solid")));
      expect(Object.keys(g.vertices).length).toBe(8);
      expect(Object.keys(g.edges).length).toBe(12);
      expect(Object.keys(g.wires).length).toBe(6);
      expect(Object.keys(g.faces).length).toBe(6);
      expect(Object.keys(g.shells).length).toBe(1);
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deleteObjectsFromModel removes object rows but keeps geometry primitives", () => {
      const g = new Model();
      applyModelDiff(g, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-solid")));
      g.objects["box-a"] = { id: "box-a" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: "box-solid" } };
      g.objects["box-b"] = { id: "box-b" as ObjectRef, typology: "spatial.shape.primitive.box" as TypologyRef, primitives: { solid: "box-solid" } };
      const removed = deleteObjectsFromModel(g, ["box-a", "missing"]);
      expect(removed).toEqual(["box-a"]);
      expect(g.objects["box-a"]).toBeUndefined();
      expect(g.objects["box-b"]).toBeTruthy();
      expect(Object.keys(g.solids)).toEqual(["box-solid"]);
    });

    it("deletableObjectIdsFromSelection keeps only object targets", () => {
      const selection: SelectionTarget[] = [
        { kind: "object", id: "box-a", editable: true },
        { kind: "solid", id: "box-solid", editable: true },
        { kind: "object", id: "box-a", editable: true },
      ];
      expect(deletableObjectIdsFromSelection(selection)).toEqual(["box-a"]);
    });
  });
}
// #endregion 🧪️Tests
