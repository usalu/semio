// #region 🧲️Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭️ `@semio-tech/cad-js/brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM, kept ONLY as a vitest differential oracle (see `import.meta.vitest`-guarded exports below and `🎫️tickets/…/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/📓️w4a-spatial-kernel-first-party.md`). The production runtime kernel is `🧠️semio/🟦️.ts`; pure preview math moved to `🧮️preview/🟦️.ts`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import {
  box,
  bsplineApprox,
  circle,
  cone,
  curveEndPoint,
  curveStartPoint,
  curveLength,
  cut,
  cylinder,
  extrude,
  face,
  filledFace,
  healSolid,
  intersect,
  loft,
  sweep as sweepAlongSpine,
  thicken,
  translate,
  wire,
  getCurveType,
  getEdges,
  getFaces,
  getHashCode,
  getSurfaceType,
  initializeOwnedOpenCascade,
  isOk,
  isSolid,
  isValidSolid,
  verticesOfEdge,
  line,
  measureArea,
  measureDistance,
  measureLength,
  measureVolume,
  mesh,
  meshEdges,
  normalAt,
  offsetFace,
  fuseAll,
  sewShells,
  solidFromShell,
  sphere,
  threePointArc,
  toGroupedBufferGeometryData,
  toLineGeometryData,
  unwrap,
  vertex as brepVertex,
  wireLoop,
  ownedOpenCascadeWasmBundledUrl,
  resolveOwnedOpenCascadeWasmFileUrl,
  type OwnedBrepEdge as Edge,
  type OwnedBrepFace as Face,
  type OwnedBrepOrientedFace as OrientedFace,
  type OwnedBrepShape as Shape3D,
  type OwnedBrepSolid as ValidSolid,
  type OwnedBrepWire as Wire,
} from "../../../../🔌️plugins/📐️cad/⚙️engine/🧱️brepjs/🟦️.ts";
import { applyModelDiff, isEmptyModelDiff, type SpatialKernel, type ModelDiff, type EdgeRecordDiff } from "../🗺️spatial/🟦️.ts";
import { Model, ModelSpace, type ModelJson, defaultModelDefinitionId, type ModelSpaceJson } from "../📐️geometry/🟦️.ts";
import { emptyMeshTransfer, kernelGeometry, type EdgeCurve, type EdgeInfo, type FaceInfo, type MeshTransfer, type Vec3, solidRef } from "@semio-tech/s-3d-js";
import {
  PreciseSpatialKernelMath,
  aabbDifferencePieces,
  aabbIntersect,
  arcEndFromAngle,
  arcEndOnCircle,
  arcSamplePoints,
  boxModelDiff,
  circleFromCenterRadiusPoint,
  edgeCurveLength,
  ellipseSamplePoints,
  faceCentroid,
  fuseSolidsToExternalFaces,
  geom,
  meshFaceModelDiff,
  modelObjectAabb,
  nurbsCurveFromPoles,
  nurbsDisplaySamplePoints,
  vec3Add,
  vec3Cross,
  vec3Distance,
  vec3Dot,
  vec3Length,
  vec3Normalize,
  vec3Scale,
  vec3Sub,
  type EdgeRecord,
  type EdgeRef,
  type FaceRecord,
  type FaceRef,
  type MutableSolidRecord,
  type ShellRef,
  type SolidPrimitive,
  type SolidRecord,
  type SolidRef,
  type VertexRecord,
  type VertexRef,
  type WireRecord,
  type WireRef,
} from "../🧮️preview/🟦️.ts";
export { kernelGeometry };
// #endregion 🔌️Adapters

// #region 🧩️OpenCascade
const isBrepjsTestRun = import.meta.env.VITEST === true || import.meta.env.MODE === "test" || Boolean(import.meta.vitest);

const openCascadeWasmNeedsNodeResolve =
  (import.meta.env.VITEST || import.meta.env.MODE === "test") && (ownedOpenCascadeWasmBundledUrl.includes("@fs") || ownedOpenCascadeWasmBundledUrl.includes("node_modules/brepjs-opencascade"));

/** @emoji 📂️ Builds `locateFile` for OpenCascade: Vite asset URL in browser, `createRequire` on disk in Vitest. */
async function createOpenCascadeLocateFile(): Promise<(path: string) => string> {
  if (!openCascadeWasmNeedsNodeResolve) {
    return (path) => (path === "brepjs_single.wasm" ? ownedOpenCascadeWasmBundledUrl : path);
  }
  const wasmFile = await resolveOwnedOpenCascadeWasmFileUrl();
  return (path) => (path === "brepjs_single.wasm" ? wasmFile : path);
}
// #endregion 🧩️OpenCascade

// #region ♻️BrepjsScratch
const MESH_TRANSFER_CACHE_MAX = 64;

function vec3KeyQuantized(x: number, y: number, z: number, invTol: number): number {
  const ix = Math.round(x * invTol);
  const iy = Math.round(y * invTol);
  const iz = Math.round(z * invTol);
  return ((ix * 73856093) ^ (iy * 19349663) ^ (iz * 83492791)) >>> 0;
}

function extrudeDirection(direction: Vec3, distance: number): Vec3 {
  const len = Math.hypot(direction[0], direction[1], direction[2]);
  const dist = Math.abs(distance) || len || 1e-6;
  if (len > 1e-12) {
    const s = dist / len;
    return [direction[0] * s, direction[1] * s, direction[2] * s];
  } else {
    return [0, 0, dist];
  }
}

// #region 🗺️BrepEntityMaps
type BrepEntityMaps = { readonly faceByHash: ReadonlyMap<number, FaceRef>; readonly edgeByHash: ReadonlyMap<number, EdgeRef> };

function modelFaceIdsForSolid(model: Model, solid: SolidRecord): readonly FaceRef[] {
  const out: FaceRef[] = [];
  const seen = new Set<string>();
  for (const sid of solid.shellIds) {
    const sh = geom(model).shells[sid];
    if (!sh) continue;
    for (const fid of sh.faceIds) {
      const key = String(fid);
      if (seen.has(key)) continue;
      seen.add(key);
      out.push(fid);
    }
  }
  return out;
}

function modelEdgeIdsForSolid(model: Model, solid: SolidRecord): readonly EdgeRef[] {
  const out: EdgeRef[] = [];
  const seen = new Set<string>();
  for (const fid of modelFaceIdsForSolid(model, solid)) {
    const face = geom(model).faces[fid];
    if (!face) continue;
    for (const wid of face.wireIds) {
      const wire = geom(model).wires[wid];
      if (!wire) continue;
      for (const eid of wire.edgeIds) {
        const key = String(eid);
        if (seen.has(key)) continue;
        seen.add(key);
        out.push(eid);
      }
    }
  }
  return out;
}

function modelFaceNormal(model: Model, fid: FaceRef): Vec3 | null {
  const face = geom(model).faces[fid];
  if (!face) return null;
  if (face.surface?.kind === "plane") return vec3Normalize(face.surface.normal);
  return faceNormalFromPoints(derivedFacePoints(model, face));
}

function alignUnmatchedBrepFacesByNormal(model: Model, unmatchedBrepFaces: Face[], faceByHash: Map<number, FaceRef>, modelFaceIds: readonly FaceRef[]): void {
  const available = modelFaceIds.filter((fid) => ![...faceByHash.values()].some((v) => String(v) === String(fid)));
  for (const brepFace of [...unmatchedBrepFaces]) {
    let best: FaceRef | null = null;
    let bestDot = -1;
    let brepNormal: Vec3;
    try {
      brepNormal = vec3Normalize(normalAt(brepFace) as Vec3);
    } catch {
      continue;
    }
    for (const fid of available) {
      const mn = modelFaceNormal(model, fid);
      if (!mn) continue;
      const d = Math.abs(vec3Dot(brepNormal, mn));
      if (d > bestDot) {
        bestDot = d;
        best = fid;
      }
    }
    if (best && bestDot > 0.85) {
      faceByHash.set(getHashCode(brepFace), best);
      const ui = unmatchedBrepFaces.indexOf(brepFace);
      if (ui >= 0) unmatchedBrepFaces.splice(ui, 1);
      const ai = available.indexOf(best);
      if (ai >= 0) available.splice(ai, 1);
    }
  }
}

/** @emoji 🗺️ Maps brepjs `getHashCode` handles to spatial `FaceRef`/`EdgeRef`; brepjs has no geometry userData. */
function buildBrepEntityMaps(brepSolid: ValidSolid, context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord }): BrepEntityMaps {
  const faceByHash = new Map<number, FaceRef>();
  const edgeByHash = new Map<number, EdgeRef>();
  let brepFaces: Face[] = [];
  let brepEdges: Edge[] = [];
  try {
    brepFaces = getFaces(brepSolid);
  } catch {
    /* empty */
  }
  try {
    brepEdges = getEdges(brepSolid);
  } catch {
    /* empty */
  }

  const unmatchedFaces = [...brepFaces];
  if (context.model && context.solidRecord) {
    for (const fid of modelFaceIdsForSolid(context.model, context.solidRecord)) {
      const fr = geom(context.model).faces[fid];
      const wireId = fr?.wireIds[0];
      if (!wireId) continue;
      const oriented = geomWireToOrientedFace(context.model, wireId);
      if (!oriented) continue;
      const h = getHashCode(oriented);
      const idx = unmatchedFaces.findIndex((f) => getHashCode(f) === h);
      if (idx >= 0) {
        faceByHash.set(getHashCode(unmatchedFaces[idx]!), fid);
        unmatchedFaces.splice(idx, 1);
      }
    }
    const modelFaces = modelFaceIdsForSolid(context.model, context.solidRecord);
    alignUnmatchedBrepFacesByNormal(context.model, unmatchedFaces, faceByHash, modelFaces);
    const unmatchedEdges = [...brepEdges];
    for (const eid of modelEdgeIdsForSolid(context.model, context.solidRecord)) {
      const er = geom(context.model).edges[eid];
      if (!er) continue;
      const brepEdge = geomEdgeToBrepEdge(context.model, er);
      if (!brepEdge) continue;
      const h = getHashCode(brepEdge);
      const idx = unmatchedEdges.findIndex((e) => getHashCode(e) === h);
      if (idx >= 0) {
        edgeByHash.set(getHashCode(unmatchedEdges[idx]!), eid);
        unmatchedEdges.splice(idx, 1);
      }
    }
    let ei = 0;
    for (const edge of unmatchedEdges) {
      edgeByHash.set(getHashCode(edge), `${context.solidRef}-brep-e-${ei++}` as EdgeRef);
    }
  }
  let fi = 0;
  for (const face of unmatchedFaces) {
    faceByHash.set(getHashCode(face), `${context.solidRef}-brep-f-${fi++}` as FaceRef);
  }
  for (const face of brepFaces) {
    if (!faceByHash.has(getHashCode(face))) {
      faceByHash.set(getHashCode(face), `${context.solidRef}-brep-f-${fi++}` as FaceRef);
    }
  }
  if (edgeByHash.size === 0) {
    let ei = 0;
    for (const edge of brepEdges) {
      edgeByHash.set(getHashCode(edge), `${context.solidRef}-brep-e-${ei++}` as EdgeRef);
    }
  }
  return { faceByHash, edgeByHash };
}

function faceEntityId(maps: BrepEntityMaps, brepHash: number, solidRef: SolidRef): FaceRef {
  return maps.faceByHash.get(brepHash) ?? (`${solidRef}-brep-f-unknown` as FaceRef);
}

function edgeEntityId(maps: BrepEntityMaps, brepHash: number, solidRef: SolidRef): EdgeRef {
  return maps.edgeByHash.get(brepHash) ?? (`${solidRef}-brep-e-unknown` as EdgeRef);
}

function collectFaceInfos(brepSolid: ValidSolid, maps: BrepEntityMaps, solidRef: SolidRef): FaceInfo[] {
  let faces: Face[];
  try {
    faces = getFaces(brepSolid);
  } catch {
    return [];
  }
  const infos: FaceInfo[] = [];
  for (const face of faces) {
    try {
      const surfaceTypeResult = getSurfaceType(face);
      const surfaceType = isOk(surfaceTypeResult) ? String(surfaceTypeResult.value) : "OTHER_SURFACE";
      const normal = normalAt(face) as [number, number, number];
      const areaResult = measureArea(face);
      const area = isOk(areaResult) ? (areaResult.value as number) : Number.NaN;
      infos.push({
        entityId: faceEntityId(maps, getHashCode(face), solidRef),
        surfaceType,
        area,
        normal,
      });
    } catch {
      /* skip degenerate face */
    }
  }
  return infos;
}

function collectEdgeInfos(brepSolid: ValidSolid, maps: BrepEntityMaps, solidRef: SolidRef): EdgeInfo[] {
  let edges: Edge[];
  try {
    edges = getEdges(brepSolid);
  } catch {
    return [];
  }
  const infos: EdgeInfo[] = [];
  for (const edge of edges) {
    try {
      infos.push({
        entityId: edgeEntityId(maps, getHashCode(edge), solidRef),
        curveType: String(getCurveType(edge)),
        length: curveLength(edge),
      });
    } catch {
      /* skip degenerate edge */
    }
  }
  return infos;
}

/** @emoji 🖼️ Tessellates a solid to grouped buffers + B-Rep edge polylines (caller owns solid lifetime). */
function meshTransferFromBrep(brepSolid: ValidSolid, tolerance: number, context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord }, collectInspection = true): MeshTransfer {
  const maps = buildBrepEntityMaps(brepSolid, context);
  const shapeMesh = mesh(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
  const edgeMesh = meshEdges(brepSolid, { tolerance, cache: true, angularTolerance: 0.2 });
  const grouped = toGroupedBufferGeometryData(shapeMesh);
  const lineData = toLineGeometryData(edgeMesh);
  const transfer: MeshTransfer = {
    position: grouped.position,
    normal: grouped.normal,
    index: grouped.index,
    edges: lineData.position,
    faceGroups: collectInspection
      ? grouped.groups.map((g) => ({
          start: g.start,
          count: g.count,
          entityId: faceEntityId(maps, g.faceId, context.solidRef),
        }))
      : [],
    edgeGroups: collectInspection
      ? (edgeMesh.edgeGroups as { start: number; count: number; edgeId: number }[]).map((g) => ({
          start: g.start,
          count: g.count,
          entityId: edgeEntityId(maps, g.edgeId, context.solidRef),
        }))
      : [],
    faceInfos: collectInspection ? collectFaceInfos(brepSolid, maps, context.solidRef) : [],
    edgeInfos: collectInspection ? collectEdgeInfos(brepSolid, maps, context.solidRef) : [],
  };
  return transfer;
}
// #endregion 🗺️BrepEntityMaps

function meshTransferTransferables(mesh: MeshTransfer): Transferable[] {
  return [mesh.position.buffer, mesh.normal.buffer, mesh.index.buffer, mesh.edges.buffer];
}

function cloneMeshTransfer(mesh: MeshTransfer): MeshTransfer {
  return {
    position: new Float32Array(mesh.position),
    normal: new Float32Array(mesh.normal),
    index: new Uint32Array(mesh.index),
    edges: new Float32Array(mesh.edges),
    faceGroups: mesh.faceGroups.map((g) => ({ ...g, entityId: g.entityId })),
    edgeGroups: mesh.edgeGroups.map((g) => ({ ...g, entityId: g.entityId })),
    faceInfos: mesh.faceInfos.map((f) => ({ ...f, normal: [...f.normal] as [number, number, number] })),
    edgeInfos: mesh.edgeInfos.map((e) => ({ ...e })),
    color: mesh.color,
  };
}
// #endregion ♻️BrepjsScratch

// #region 🔌️BrepModelBridge
/** @emoji 🔗️ Builds a brepjs `Edge` from a model edge record. */
function geomEdgeToBrepEdge(model: Model, edge: EdgeRecord): Edge | null {
  const ids = edge.vertexIds;
  if (ids.length < 1) return null;
  const p0 = geom(model).vertices[String(ids[0])]?.position;
  const p1 = geom(model).vertices[String(ids[1] ?? ids[0])]?.position;
  if (!p0 || !p1) return null;
  const c = edge.curve;
  if (!c || c.kind === "line") return line(p0, p1);
  if (c.kind === "circle") {
    const e = circle(c.radius, { at: c.center, axis: c.normal });
    return e;
  }
  if (c.kind === "arc") {
    const mid = arcSamplePoints(c.center, p0, p1, 4)[2] ?? p1;
    return threePointArc(p0, mid, p1);
  }
  if (c.kind === "nurbs" && c.poles.length >= 2) {
    const fitPoints = c.through ? [...nurbsDisplaySamplePoints(c.poles, 16)] : [...c.poles];
    const r = bsplineApprox(fitPoints);
    if (isOk(r)) return r.value;
  }
  if (c.kind === "ellipse") {
    const samples = ellipseSamplePoints(c.center, c.normal, c.majorAxis, c.majorRadius, c.minorRadius, 32);
    const r = bsplineApprox([...(samples.length >= 2 ? samples : [p0, p1])]);
    if (isOk(r)) return r.value;
  }
  return line(p0, p1);
}

/** @emoji 🔗️ brepjs wire from a model wire (closed `wireLoop` or open `wire`). */
function geomWireToBrepWire(model: Model, wireId: WireRef): Wire | null {
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const loop = wireLoop(edges);
  if (isOk(loop)) return loop.value;
  const open = wire(edges);
  return isOk(open) ? open.value : null;
}

/** @emoji 🔗️ Closed planar brepjs face from a model wire (`wireLoop` + `face`). */
function geomWireToOrientedFace(model: Model, wireId: WireRef): OrientedFace | null {
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const cw = wireLoop(edges);
  if (!isOk(cw)) return null;
  const f = face(cw.value as Parameters<typeof face>[0]);
  return isOk(f) ? f.value : null;
}

/** @emoji 🔗️ Oriented face from a model wire; uses `filledFace` when planar `face` fails (deformed boxes). */
function geomWireToOrientedFaceLoose(model: Model, wireId: WireRef): OrientedFace | null {
  const planar = geomWireToOrientedFace(model, wireId);
  if (planar) return planar;
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const cw = wireLoop(edges);
  if (!isOk(cw)) return null;
  const filled = filledFace(cw.value as Parameters<typeof filledFace>[0]);
  return isOk(filled) ? filled.value : null;
}

/** @emoji 🔗️ Extrudes a model wire to a `ValidSolid` via brepjs (planar face or open-curve loft). */
function extrudeModelWire(model: Model, wireId: string, direction: Vec3, distance: number): ValidSolid | null {
  const wid = wireId as WireRef;
  const vec = extrudeDirection(direction, distance);
  const planar = geomWireToOrientedFace(model, wid) ?? geomWireToOrientedFaceLoose(model, wid);
  if (planar) {
    const solid = extrude(planar as Parameters<typeof extrude>[0], vec);
    return isOk(solid) ? (solid.value as ValidSolid) : null;
  }
  const profile = geomWireToBrepWire(model, wid);
  if (!profile) return null;
  const moved = translate(profile, vec);
  const lofted = loft([profile, moved], { ruled: true });
  if (!isOk(lofted)) return null;
  const shape = lofted.value as Shape3D;
  if (isSolid(shape)) {
    if (isValidSolid(shape)) return shape as ValidSolid;
    const healed = healSolid(shape);
    if (isOk(healed) && isValidSolid(healed.value)) return healed.value as ValidSolid;
    return shape as ValidSolid;
  }
  const thickened = thicken(shape as Parameters<typeof thicken>[0], 1e-3);
  if (isOk(thickened) && isSolid(thickened.value)) return thickened.value as ValidSolid;
  return null;
}

type SelectionPick = { readonly kind: string; readonly id: string };

function selectionPicksFromParams(params: Record<string, unknown>): SelectionPick[] {
  const raw = params.curves ?? params.targets ?? [];
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

/** @emoji 🧵️ Resolves wire ids from curve selection targets (`wire` or parent wire of `edge`). */
function wireIdsFromSelectionPicks(model: Model, picks: readonly SelectionPick[]): WireRef[] {
  const g = geom(model);
  const out: WireRef[] = [];
  const seen = new Set<string>();
  for (const pick of picks) {
    if (pick.kind === "wire") {
      if (g.wires[pick.id] && !seen.has(pick.id)) {
        seen.add(pick.id);
        out.push(pick.id as WireRef);
      }
      continue;
    }
    if (pick.kind !== "edge") continue;
    for (const wire of Object.values(g.wires)) {
      if (!wire.edgeIds.includes(pick.id as EdgeRef) || seen.has(wire.id)) continue;
      seen.add(wire.id);
      out.push(wire.id);
    }
  }
  return out;
}

function mergeSolidAdds(diffs: readonly ModelDiff[]): ModelDiff {
  const added = diffs.flatMap((d) => d.solids?.added ?? []);
  return added.length ? { solids: { added } } : {};
}

/** @emoji 🧩️ Parses raw `SelectionTarget[]`-shaped context data into `{kind,id}` picks (any field). */
function picksFromRaw(raw: unknown): SelectionPick[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

/** @emoji 🧱️ Resolves solid ids from raw `SelectionTarget[]`-shaped selection context (boolean operands). */
function solidRefsFromSelectionRaw(model: Model, raw: unknown): SolidRef[] {
  const g = geom(model);
  const out: SolidRef[] = [];
  const seen = new Set<string>();
  for (const pick of picksFromRaw(raw)) {
    if (pick.kind !== "solid" || !g.solids[pick.id] || seen.has(pick.id)) continue;
    seen.add(pick.id);
    out.push(pick.id as SolidRef);
  }
  return out;
}

/** @emoji 🧵️ Resolves wire ids from a raw `targets` context field that may be a flat array or `command.addSelection`'s keyed sub-object (e.g. `{rail:[...]}`, `{railA:[...],railB:[...]}`). */
function wireIdsFromKeyedRaw(model: Model, raw: unknown): WireRef[] {
  if (Array.isArray(raw)) return wireIdsFromSelectionPicks(model, picksFromRaw(raw));
  if (!raw || typeof raw !== "object") return [];
  const out: WireRef[] = [];
  const seen = new Set<string>();
  for (const value of Object.values(raw as Record<string, unknown>)) {
    for (const id of wireIdsFromSelectionPicks(model, picksFromRaw(value))) {
      if (seen.has(id)) continue;
      seen.add(id);
      out.push(id);
    }
  }
  return out;
}

/** @emoji 🧵️ Synthesizes a small circular cross-section wire when `surface.sweep{1,2}` receives no explicit profile curve. */
function defaultSweepProfileWire(model: Model, railWireId: WireRef): Wire | null {
  const g = geom(model);
  const rail = g.wires[railWireId];
  const firstEdgeId = rail?.edgeIds[0];
  const firstEdge = firstEdgeId ? g.edges[firstEdgeId] : undefined;
  if (!firstEdge) return null;
  const p0 = g.vertices[String(firstEdge.vertexIds[0])]?.position;
  if (!p0) return null;
  const p1 = g.vertices[String(firstEdge.vertexIds[1] ?? firstEdge.vertexIds[0])]?.position;
  const tangent = p1 && vec3Distance(p0, p1) > 1e-9 ? vec3Normalize(vec3Sub(p1, p0)) : ([0, 0, 1] as Vec3);
  let radius = 0.25;
  let maxD = 0;
  for (const eid of rail?.edgeIds ?? []) {
    const e = g.edges[eid];
    if (!e) continue;
    for (const vid of e.vertexIds) {
      const p = g.vertices[String(vid)]?.position;
      if (p) maxD = Math.max(maxD, vec3Distance(p0, p));
    }
  }
  if (maxD > 1e-9) radius = Math.max(maxD * 0.15, 0.05);
  const profileEdge = circle(radius, { at: p0, axis: tangent });
  const loopResult = wireLoop([profileEdge]);
  return isOk(loopResult) ? (loopResult.value as Wire) : null;
}

// #region ✂️EditTopologyOps
/** @emoji 🧷️ Parses a raw selection-target array (`{kind,id}[]`) from an `edit.*` command param. */
function picksFromValue(raw: unknown): SelectionPick[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

function freshEditRef(kind: string): string {
  return `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
}

/** @emoji 🧷️ Flattens picks (`edge` verbatim, `wire`/`face` expand to member edges) into concrete edge ids. */
function edgeIdsFromPicks(model: Model, picks: readonly SelectionPick[]): EdgeRef[] {
  const g = geom(model);
  const out: EdgeRef[] = [];
  for (const pick of picks) {
    if (pick.kind === "edge") {
      if (g.edges[pick.id]) out.push(pick.id as EdgeRef);
      continue;
    }
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (w) out.push(...w.edgeIds);
      continue;
    }
    if (pick.kind === "face") {
      const f = g.faces[pick.id];
      if (!f) continue;
      for (const wireId of f.wireIds) {
        const w = g.wires[wireId];
        if (w) out.push(...w.edgeIds);
      }
    }
  }
  return out;
}

type EdgeSegment = { readonly p0: Vec3; readonly p1: Vec3 };

function edgeSegment(model: Model, id: string): EdgeSegment | null {
  const g = geom(model);
  const e = g.edges[id];
  if (!e || e.vertexIds.length < 2) return null;
  const p0 = g.vertices[String(e.vertexIds[0])]?.position;
  const p1 = g.vertices[String(e.vertexIds[1])]?.position;
  if (!p0 || !p1) return null;
  return { p0, p1 };
}

function clampNumber(x: number, lo: number, hi: number): number {
  return x < lo ? lo : x > hi ? hi : x;
}

/** @emoji 📏️ Closest points between two 3D segments (Ericson's segment-segment algorithm); `t` is the parameter (0..1) on the first segment. */
function closestPointsOnSegments(p1: Vec3, p2: Vec3, q1: Vec3, q2: Vec3): { readonly onFirst: Vec3; readonly onSecond: Vec3; readonly t: number } {
  const d1 = vec3Sub(p2, p1);
  const d2 = vec3Sub(q2, q1);
  const r = vec3Sub(p1, q1);
  const a = vec3Dot(d1, d1);
  const e = vec3Dot(d2, d2);
  const f = vec3Dot(d2, r);
  const EPS = 1e-12;
  let s = 0;
  let t = 0;
  if (a <= EPS && e <= EPS) {
    s = 0;
    t = 0;
  } else if (a <= EPS) {
    s = 0;
    t = clampNumber(f / e, 0, 1);
  } else {
    const c = vec3Dot(d1, r);
    if (e <= EPS) {
      t = 0;
      s = clampNumber(-c / a, 0, 1);
    } else {
      const b = vec3Dot(d1, d2);
      const denom = a * e - b * b;
      s = denom !== 0 ? clampNumber((b * f - c * e) / denom, 0, 1) : 0;
      t = (b * s + f) / e;
      if (t < 0) {
        t = 0;
        s = clampNumber(-c / a, 0, 1);
      } else if (t > 1) {
        t = 1;
        s = clampNumber((b - c) / a, 0, 1);
      }
    }
  }
  return { onFirst: vec3Add(p1, vec3Scale(d1, s)), onSecond: vec3Add(q1, vec3Scale(d2, t)), t: s };
}

/** @emoji 🧷️ `edit.join`: merges the edges of the selected wires/edges/faces into one new wire (topological grouping, no geometric coincidence required). */
function editJoinDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const picks = picksFromValue(params.targets);
  const removedWires: WireRef[] = [];
  const edgeIds: EdgeRef[] = [];
  const seen = new Set<string>();
  const pushEdges = (ids: readonly EdgeRef[]) => {
    for (const id of ids) {
      if (seen.has(id)) continue;
      seen.add(id);
      edgeIds.push(id);
    }
  };
  for (const pick of picks) {
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (!w) continue;
      pushEdges(w.edgeIds);
      removedWires.push(pick.id as WireRef);
      continue;
    }
    if (pick.kind === "edge") {
      if (g.edges[pick.id]) pushEdges([pick.id as EdgeRef]);
      continue;
    }
    if (pick.kind === "face") {
      const f = g.faces[pick.id];
      if (!f) continue;
      for (const wireId of f.wireIds) {
        const w = g.wires[wireId];
        if (!w) continue;
        pushEdges(w.edgeIds);
        removedWires.push(wireId);
      }
    }
  }
  if (edgeIds.length < 2) return {};
  const joined: WireRecord = { id: freshEditRef("wire-join") as WireRef, edgeIds };
  return removedWires.length ? { wires: { added: [joined], removed: removedWires } } : { wires: { added: [joined] } };
}

/** @emoji 💥️ `edit.explode`: inverse of `edit.join` — decomposes each selected wire into one single-edge wire per member edge; shells/solids explode by dropping their container record. */
function editExplodeDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const picks = picksFromValue(params.targets);
  const addedWires: WireRecord[] = [];
  const removedWires: WireRef[] = [];
  const removedShells: ShellRef[] = [];
  const removedSolids: SolidRef[] = [];
  for (const pick of picks) {
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (!w || w.edgeIds.length < 2) continue;
      for (const edgeId of w.edgeIds) addedWires.push({ id: freshEditRef("wire-explode") as WireRef, edgeIds: [edgeId] });
      removedWires.push(pick.id as WireRef);
      continue;
    }
    if (pick.kind === "shell") {
      if (g.shells[pick.id]) removedShells.push(pick.id as ShellRef);
      continue;
    }
    if (pick.kind === "solid") {
      if (g.solids[pick.id]) removedSolids.push(pick.id as SolidRef);
    }
  }
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {};
  if (addedWires.length || removedWires.length) diff.wires = { ...(addedWires.length ? { added: addedWires } : {}), ...(removedWires.length ? { removed: removedWires } : {}) };
  if (removedShells.length) diff.shells = { removed: removedShells };
  if (removedSolids.length) diff.solids = { removed: removedSolids };
  return diff;
}

/** @emoji ✂️ Splits `edgeId` at parameter `s` (0..1 along its segment), rewiring the containing wire if any. New sub-edges default to straight lines (honest chord approximation for non-line curves). */
function splitEdgeAt(model: Model, edgeId: EdgeRef, seg: EdgeSegment, s: number): ModelDiff {
  const g = geom(model);
  const edge = g.edges[edgeId];
  if (!edge) return {};
  const v0 = edge.vertexIds[0]! as VertexRef;
  const v1 = edge.vertexIds[1]! as VertexRef;
  const splitPos = vec3Add(seg.p0, vec3Scale(vec3Sub(seg.p1, seg.p0), s));
  const vSplit: VertexRecord = { id: freshEditRef("v-split") as VertexRef, position: splitPos };
  const eA: EdgeRecord = { id: freshEditRef("e-split") as EdgeRef, vertexIds: [v0, vSplit.id] };
  const eB: EdgeRecord = { id: freshEditRef("e-split") as EdgeRef, vertexIds: [vSplit.id, v1] };
  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(edgeId));
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: { added: [vSplit] },
    edges: { added: [eA, eB], removed: [edgeId] },
  };
  if (wire) {
    const idx = wire.edgeIds.indexOf(edgeId);
    const newIds = [...wire.edgeIds.slice(0, idx), eA.id, eB.id, ...wire.edgeIds.slice(idx + 1)];
    diff.wires = { modified: [{ id: wire.id, edgeIds: newIds }] };
  }
  return diff;
}

/** @emoji ✂️ `edit.split`: cuts the split-object edge closest to the cutting reference into two edges at their closest-approach point. */
function editSplitDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const targetIds = edgeIdsFromPicks(model, picksFromValue(params.splitObjects));
  const cutterIds = edgeIdsFromPicks(model, picksFromValue(params.cutters));
  if (!targetIds.length || !cutterIds.length) return {};
  const cutterSeg = edgeSegment(model, cutterIds[0]!);
  if (!cutterSeg) return {};
  let bestId: EdgeRef | null = null;
  let bestSeg: EdgeSegment | null = null;
  let bestS = 0.5;
  let bestDist = Infinity;
  for (const id of targetIds) {
    const seg = edgeSegment(model, id);
    if (!seg) continue;
    const cp = closestPointsOnSegments(seg.p0, seg.p1, cutterSeg.p0, cutterSeg.p1);
    const dist = vec3Distance(cp.onFirst, cp.onSecond);
    if (dist < bestDist) {
      bestDist = dist;
      bestId = id;
      bestSeg = seg;
      bestS = clampNumber(cp.t, 0.15, 0.85);
    }
  }
  if (!bestId || !bestSeg) return {};
  return splitEdgeAt(model, bestId, bestSeg, bestS);
}

/** @emoji ✂️ `edit.trim`: trims the object edge closest to the cutting reference, discarding the shorter side of the closest-approach split point. */
function editTrimDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const cutterIds = edgeIdsFromPicks(model, picksFromValue(params.cutters));
  const targetIds = edgeIdsFromPicks(model, picksFromValue(params.trimmedObjects));
  if (!cutterIds.length || !targetIds.length) return {};
  let bestTargetId: EdgeRef | null = null;
  let bestSeg: EdgeSegment | null = null;
  let bestS = 0.5;
  let bestDist = Infinity;
  for (const targetId of targetIds) {
    const targetSeg = edgeSegment(model, targetId);
    if (!targetSeg) continue;
    for (const cutterId of cutterIds) {
      const cutterSeg = edgeSegment(model, cutterId);
      if (!cutterSeg) continue;
      const cp = closestPointsOnSegments(targetSeg.p0, targetSeg.p1, cutterSeg.p0, cutterSeg.p1);
      const dist = vec3Distance(cp.onFirst, cp.onSecond);
      if (dist < bestDist) {
        bestDist = dist;
        bestTargetId = targetId;
        bestSeg = targetSeg;
        bestS = clampNumber(cp.t, 0.15, 0.85);
      }
    }
  }
  if (!bestTargetId || !bestSeg) return {};
  const edge = g.edges[bestTargetId];
  if (!edge) return {};
  const v0 = edge.vertexIds[0]! as VertexRef;
  const v1 = edge.vertexIds[1]! as VertexRef;
  const splitPos = vec3Add(bestSeg.p0, vec3Scale(vec3Sub(bestSeg.p1, bestSeg.p0), bestS));
  const vSplit: VertexRecord = { id: freshEditRef("v-trim") as VertexRef, position: splitPos };
  const keepFirstHalf = bestS >= 0.5;
  const keptEdge: EdgeRecord = { id: freshEditRef("e-trim") as EdgeRef, vertexIds: keepFirstHalf ? [v0, vSplit.id] : [vSplit.id, v1] };
  const droppedVertexId = keepFirstHalf ? v1 : v0;
  const stillReferenced = Object.values(g.edges).some((e) => e.id !== bestTargetId && e.vertexIds.includes(droppedVertexId));
  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(bestTargetId!));
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: stillReferenced ? { added: [vSplit] } : { added: [vSplit], removed: [droppedVertexId] },
    edges: { added: [keptEdge], removed: [bestTargetId] },
  };
  if (wire) {
    const idx = wire.edgeIds.indexOf(bestTargetId);
    const newIds = [...wire.edgeIds.slice(0, idx), keptEdge.id, ...wire.edgeIds.slice(idx + 1)];
    diff.wires = { modified: [{ id: wire.id, edgeIds: newIds }] };
  }
  return diff;
}

function otherVertexOf(vertexIds: readonly VertexRef[], id: VertexRef): VertexRef {
  return (vertexIds[0] === id ? vertexIds[1] : vertexIds[0])! as VertexRef;
}

/** @emoji 📐️ Replaces the span between `startId` and `endId` in `wire.edgeIds` (inclusive ends, kept) with `replacement`, in whichever array order they appear. Returns `null` if either id is absent. */
function spliceWireSpan(wire: WireRecord, startId: EdgeRef, endId: EdgeRef, replacement: EdgeRef): readonly EdgeRef[] | null {
  const ids = wire.edgeIds;
  const i0 = ids.indexOf(startId);
  const i1 = ids.indexOf(endId);
  if (i0 === -1 || i1 === -1 || i0 === i1) return null;
  if (i0 < i1) return [...ids.slice(0, i0 + 1), replacement, ...ids.slice(i1)];
  return [...ids.slice(0, i1 + 1), replacement, ...ids.slice(i0)];
}

/** @emoji 📐️ Shared builder for `edit.chamfer` (straight connector) and `edit.fillet` (tangent-arc connector): bridges two curves at their shared vertex, their single connecting edge, or (failing that) their nearest endpoints — extending both lines to a virtual corner when no direct link exists. */
function cornerConnectorDiff(model: Model, edgeAId: EdgeRef, edgeBId: EdgeRef, style: "chamfer" | "fillet"): ModelDiff {
  const g = geom(model);
  const edgeA = g.edges[edgeAId];
  const edgeB = g.edges[edgeBId];
  if (!edgeA || !edgeB || edgeA.vertexIds.length < 2 || edgeB.vertexIds.length < 2 || edgeAId === edgeBId) return {};
  const aIds = edgeA.vertexIds as VertexRef[];
  const bIds = edgeB.vertexIds as VertexRef[];
  const posOf = (id: VertexRef): Vec3 | null => g.vertices[id]?.position ?? null;

  let nearA: VertexRef;
  let farA: VertexRef;
  let nearB: VertexRef;
  let farB: VertexRef;
  let bridgeId: EdgeRef | null = null;
  const shared = aIds.find((id) => bIds.includes(id));
  if (shared) {
    nearA = shared;
    farA = otherVertexOf(aIds, shared);
    nearB = shared;
    farB = otherVertexOf(bIds, shared);
  } else {
    let bridge: { a: VertexRef; b: VertexRef; id: EdgeRef } | null = null;
    outer: for (const a of aIds) {
      for (const b of bIds) {
        const cand = Object.values(g.edges).find((e) => e.id !== edgeAId && e.id !== edgeBId && e.vertexIds.includes(a) && e.vertexIds.includes(b));
        if (cand) {
          bridge = { a, b, id: cand.id };
          break outer;
        }
      }
    }
    if (bridge) {
      nearA = bridge.a;
      farA = otherVertexOf(aIds, bridge.a);
      nearB = bridge.b;
      farB = otherVertexOf(bIds, bridge.b);
      bridgeId = bridge.id;
    } else {
      let bestDist = Infinity;
      let bestA: VertexRef = aIds[0]!;
      let bestB: VertexRef = bIds[0]!;
      for (const a of aIds) {
        for (const b of bIds) {
          const pa = posOf(a);
          const pb = posOf(b);
          if (!pa || !pb) continue;
          const dist = vec3Distance(pa, pb);
          if (dist < bestDist) {
            bestDist = dist;
            bestA = a;
            bestB = b;
          }
        }
      }
      nearA = bestA;
      farA = otherVertexOf(aIds, bestA);
      nearB = bestB;
      farB = otherVertexOf(bIds, bestB);
    }
  }
  const pNearA = posOf(nearA);
  const pFarA = posOf(farA);
  const pNearB = posOf(nearB);
  const pFarB = posOf(farB);
  if (!pNearA || !pFarA || !pNearB || !pFarB) return {};

  const dirA = vec3Normalize(vec3Sub(pFarA, pNearA));
  const dirB = vec3Normalize(vec3Sub(pFarB, pNearB));
  let corner: Vec3;
  if (nearA === nearB) {
    corner = pNearA;
  } else {
    const big = 1e6;
    const cp = closestPointsOnSegments(vec3Sub(pNearA, vec3Scale(dirA, big)), vec3Add(pNearA, vec3Scale(dirA, big)), vec3Sub(pNearB, vec3Scale(dirB, big)), vec3Add(pNearB, vec3Scale(dirB, big)));
    corner = vec3Distance(cp.onFirst, cp.onSecond) < 1e-3 ? vec3Scale(vec3Add(cp.onFirst, cp.onSecond), 0.5) : vec3Scale(vec3Add(pNearA, pNearB), 0.5);
  }

  const lenA = vec3Distance(pFarA, pNearA);
  const lenB = vec3Distance(pFarB, pNearB);
  const d = Math.max(Math.min(lenA, lenB) * 0.3, 1e-6);
  const tA = clampNumber(vec3Dot(vec3Sub(vec3Add(corner, vec3Scale(dirA, d)), pNearA), dirA), lenA * 0.05, lenA * 0.95);
  const tB = clampNumber(vec3Dot(vec3Sub(vec3Add(corner, vec3Scale(dirB, d)), pNearB), dirB), lenB * 0.05, lenB * 0.95);
  const TA = vec3Add(pNearA, vec3Scale(dirA, tA));
  const TB = vec3Add(pNearB, vec3Scale(dirB, tB));

  const vTA: VertexRecord = { id: freshEditRef("v-corner") as VertexRef, position: TA };
  const vTB: VertexRecord = { id: freshEditRef("v-corner") as VertexRef, position: TB };

  let connectorCurve: EdgeCurve = { kind: "line" };
  if (style === "fillet") {
    const uA = vec3Normalize(vec3Sub(TA, corner));
    const uB = vec3Normalize(vec3Sub(TB, corner));
    const cosTheta = clampNumber(vec3Dot(uA, uB), -1, 1);
    const theta = Math.acos(cosTheta);
    if (theta > 1e-3 && theta < Math.PI - 1e-3) {
      const bis = vec3Normalize(vec3Add(uA, uB));
      const centerDist = d / Math.cos(theta / 2);
      connectorCurve = { kind: "arc", center: vec3Add(corner, vec3Scale(bis, centerDist)) };
    }
  }
  const connector: EdgeRecord = { id: freshEditRef("e-corner") as EdgeRef, vertexIds: [vTA.id, vTB.id], curve: connectorCurve };

  const modA: EdgeRecordDiff = { id: edgeAId, vertexIds: aIds.map((id) => (id === nearA ? vTA.id : id)) };
  const modB: EdgeRecordDiff = { id: edgeBId, vertexIds: bIds.map((id) => (id === nearB ? vTB.id : id)) };

  const stillUsed = (vid: VertexRef): boolean => Object.values(g.edges).some((e) => e.id !== edgeAId && e.id !== edgeBId && e.id !== bridgeId && e.vertexIds.includes(vid));
  const removedVertexIds = [...new Set<VertexRef>([nearA, nearB])].filter((vid) => !stillUsed(vid));

  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(edgeAId) && w.edgeIds.includes(edgeBId) && (!bridgeId || w.edgeIds.includes(bridgeId)));
  const splicedIds = wire ? spliceWireSpan(wire, edgeAId, edgeBId, connector.id) : null;
  const wireDiff = wire && splicedIds ? { modified: [{ id: wire.id, edgeIds: splicedIds }] } : undefined;

  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: removedVertexIds.length ? { added: [vTA, vTB], removed: removedVertexIds } : { added: [vTA, vTB] },
    edges: bridgeId ? { added: [connector], modified: [modA, modB], removed: [bridgeId] } : { added: [connector], modified: [modA, modB] },
  };
  if (wireDiff) diff.wires = wireDiff;
  return diff;
}
// #endregion ✂️EditTopologyOps

function extrusionDistanceFromParams(params: Record<string, unknown>): number {
  if (typeof params.distance === "number" && Number.isFinite(params.distance)) return Math.abs(params.distance);
  const origin = readVec3(params.origin) ?? readVec3(params.prevPoint) ?? ([0, 0, 0] as Vec3);
  let end = readVec3(params.cursor) ?? origin;
  const points = params.points;
  if (points && typeof points === "object" && !Array.isArray(points)) {
    end = readVec3((points as { distancePoint?: unknown }).distancePoint) ?? end;
  }
  const dir = vec3Normalize(readVec3(params.direction) ?? ([0, 0, 1] as Vec3));
  return Math.abs(vec3Dot(vec3Sub(end, origin), dir));
}

/** @emoji ✅️ True when `cell` references at least one face through its shell graph. */
function solidRecordHasShellTopology(model: Model, cell: SolidRecord): boolean {
  return modelFaceIdsForSolid(model, cell).length > 0;
}

/** @emoji 🧊️ Builds one brep face from the model face's outer wire (live vertex positions). */
function brepFaceFromModelFaceRecord(model: Model, faceRec: FaceRecord): Face | null {
  const wireId = faceRec.wireIds[0];
  if (!wireId) return null;
  const oriented = geomWireToOrientedFaceLoose(model, wireId);
  return oriented;
}

/** @emoji 🧊️ Builds a brepjs `ValidSolid` by sewing closed shell faces from model topology. */
function solidFromModelTopology(model: Model, cell: SolidRecord): ValidSolid | null {
  const faceIds = modelFaceIdsForSolid(model, cell);
  if (faceIds.length === 0) return null;
  const brepFaces: Face[] = [];
  for (const fid of faceIds) {
    const faceRec = geom(model).faces[fid];
    if (!faceRec) return null;
    const brepFace = brepFaceFromModelFaceRecord(model, faceRec);
    if (!brepFace) return null;
    brepFaces.push(brepFace);
  }
  const sewn = sewShells(brepFaces);
  if (isOk(sewn)) {
    const fromShell = solidFromShell(sewn.value);
    if (isOk(fromShell)) {
      const healed = healSolid(fromShell.value);
      return isOk(healed) ? healed.value : fromShell.value;
    }
  }
  return null;
}

/** @emoji 🧊️ Brep for records with shell topology or analytic `SolidPrimitive` when no shell graph exists. */
function deriveValidSolidFromRecordOrPrimitive(model: Model, cell: SolidRecord, primitiveFrom: (p: SolidPrimitive) => ValidSolid): ValidSolid | null {
  if (String(cell.id).startsWith("from_geometry-")) return null;
  if (solidRecordHasShellTopology(model, cell)) {
    const fromTopo = solidFromModelTopology(model, cell);
    if (fromTopo) return fromTopo;
  }
  if (cell.solid) return primitiveFrom(cell.solid);
  return null;
}
// #endregion 🔌️BrepModelBridge

// #region 🪜️StepBrepImport
const STEP_VERTEX_QUANT_INV = 1e6;

function stepVertexQuantKey(pos: Vec3): string {
  return `${Math.round(pos[0] * STEP_VERTEX_QUANT_INV)},${Math.round(pos[1] * STEP_VERTEX_QUANT_INV)},${Math.round(pos[2] * STEP_VERTEX_QUANT_INV)}`;
}

function orderWireEdgeIds(edges: readonly Edge[], edgeIdByHash: ReadonlyMap<number, EdgeRef>): EdgeRef[] {
  if (edges.length === 0) return [];
  const remaining = new Map<number, Edge>();
  for (const edge of edges) remaining.set(getHashCode(edge), edge);
  const ordered: EdgeRef[] = [];
  let current = edges[0]!;
  remaining.delete(getHashCode(current));
  const firstId = edgeIdByHash.get(getHashCode(current));
  if (!firstId) return [];
  ordered.push(firstId);
  let tailHash = getHashCode(verticesOfEdge(current)[1] ?? verticesOfEdge(current)[0]!);
  while (remaining.size > 0) {
    let nextEdge: Edge | null = null;
    let nextTailHash = tailHash;
    for (const edge of remaining.values()) {
      const verts = verticesOfEdge(edge);
      const headHash = getHashCode(verts[0]!);
      const endHash = getHashCode(verts[1] ?? verts[0]!);
      if (headHash === tailHash) {
        nextEdge = edge;
        nextTailHash = endHash;
        break;
      }
      if (endHash === tailHash) {
        nextEdge = edge;
        nextTailHash = headHash;
        break;
      }
    }
    if (!nextEdge) break;
    remaining.delete(getHashCode(nextEdge));
    const nextId = edgeIdByHash.get(getHashCode(nextEdge));
    if (!nextId) break;
    ordered.push(nextId);
    tailHash = nextTailHash;
  }
  if (ordered.length === edges.length) return ordered;
  const fallback: EdgeRef[] = [];
  for (const edge of edges) {
    const id = edgeIdByHash.get(getHashCode(edge));
    if (id) fallback.push(id);
  }
  return fallback;
}


/** @emoji 🗺️ `spatial.modelspace/v1` JSON with one object's primitives inlined as raw `materializeInlineObjectPrimitives` rows (pre-normalization). */
export interface InlineModelSpaceFixtureJson {
  readonly schema: "spatial.modelspace";
  readonly revision: number;
  readonly models: readonly { readonly id: string; readonly model: { readonly schema: "spatial.model"; readonly revision: number; readonly objects: readonly { readonly id: string; readonly typology: string; readonly primitives: readonly unknown[] }[] } }[];
}

/** @emoji 🧾️ Serializes one object and its solid closure as inline `spatial.modelspace/v1` fixture JSON. */
export function inlineModelSpaceFixtureJson(model: Model, modelId: string, objectId: string): InlineModelSpaceFixtureJson {
  const object = model.objects[objectId];
  if (!object) throw new Error(`missing object ${objectId}`);
  const solidId = object.primitives.solid;
  if (!solidId) throw new Error(`object ${objectId} has no solid primitive`);
  const solid = model.solids[solidId];
  const shellIds = solid?.shellIds ?? [];
  const faceIds = shellIds.flatMap((shellRef) => model.shells[shellRef]?.faceIds ?? []);
  const wireIds = faceIds.flatMap((faceRef) => model.faces[faceRef]?.wireIds ?? []);
  const edgeIds = wireIds.flatMap((wireRef) => model.wires[wireRef]?.edgeIds ?? []);
  const vertexIds = new Set<VertexRef>();
  for (const edgeRef of edgeIds) {
    const edge = model.edges[edgeRef];
    if (!edge) continue;
    for (const vertexRef of edge.vertexIds) vertexIds.add(vertexRef as VertexRef);
  }
  const primitives: unknown[] = [];
  for (const vertexRef of [...vertexIds].sort()) {
    const vertex = model.vertices[vertexRef]!;
    primitives.push({ kind: "vertex", id: vertexRef, position: vertex.position });
  }
  for (const edgeRef of [...edgeIds].sort()) {
    const edge = model.edges[edgeRef]!;
    const row: Record<string, unknown> = { kind: "edge", id: edgeRef, vertexIds: [...edge.vertexIds] };
    if (edge.curve) row.curve = edge.curve;
    primitives.push(row);
  }
  for (const wireRef of [...wireIds].sort()) {
    const wire = model.wires[wireRef]!;
    primitives.push({ kind: "wire", id: wireRef, edgeIds: [...wire.edgeIds] });
  }
  for (const faceRef of [...faceIds].sort()) {
    const faceRec = model.faces[faceRef]!;
    const row: Record<string, unknown> = { kind: "face", id: faceRef, wireIds: [...faceRec.wireIds] };
    if (faceRec.surface) row.surface = faceRec.surface;
    primitives.push(row);
  }
  for (const shellRef of shellIds) {
    const shell = model.shells[shellRef]!;
    primitives.push({ kind: "shell", id: shellRef, faceIds: [...shell.faceIds] });
  }
  primitives.push({ kind: "solid", slot: "solid", id: solidId, shellIds: [...shellIds] });
  return {
    schema: "spatial.modelspace",
    revision: 1,
    models: [
      {
        id: modelId,
        model: {
          schema: "spatial.model",
          revision: model.revision,
          objects: [{ id: objectId, typology: object.typology, primitives }],
        },
      },
    ],
  };
}
// #endregion 🪜️StepBrepImport

// #region 🔌️BrepjsWasmEngine
/** @emoji 🔌️ WASM-side engine: exact solids keyed by `SolidRef` (runs in worker or local fallback). */
class BrepjsWasmEngine {
  readonly operations: readonly string[] = ["solid.createBox", "wire.extrudeToSolid", "face.offset", "entity.tessellate", "measure.distance", "measure.area", "measure.volume"];

  private initPromise: Promise<void> | null = null;
  private seq = 0;
  private readonly solids = new Map<SolidRef, ValidSolid>();
  private solidsModelKey: string | null = null;

  /** @emoji 🧪️ Clears solids cache (vitest shared kernel). */
  resetDerivedPipeline(): void {
    this.solids.clear();
    this.meshCache.clear();
    this.solidsModelKey = null;
  }

  private modelDerivedKey(model: Model): string {
    const solids = (Object.keys(geom(model).solids) as SolidRef[]).sort().join(",");
    const vertexDigest = Object.values(geom(model).vertices)
      .map((v) => `${v.id}:${v.position.map((n) => n.toFixed(4)).join(",")}`)
      .sort()
      .join("|");
    return `${model.revision}:${solids}:${vertexDigest}`;
  }

  async ensureInit(): Promise<void> {
    if (!this.initPromise) {
      this.initPromise = createOpenCascadeLocateFile().then(initializeOwnedOpenCascade);
    }
    await this.initPromise;
  }

  private solidFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): ValidSolid {
    const ax = Math.min(input.cornerA[0], input.cornerB[0]);
    const ay = Math.min(input.cornerA[1], input.cornerB[1]);
    const bx = Math.max(input.cornerA[0], input.cornerB[0]);
    const by = Math.max(input.cornerA[1], input.cornerB[1]);
    const w = bx - ax;
    const d = by - ay;
    const h = input.height;
    const minZ = Math.min(input.cornerA[2], input.cornerB[2]);
    const cx = (ax + bx) / 2;
    const cy = (ay + by) / 2;
    return box(w, d, h, { at: [cx, cy, minZ + h / 2], centered: true });
  }

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    await this.ensureInit();
    const solid = this.solidFromCorners(input);
    const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
    this.solids.set(ref, solid);
    return ref;
  }

  async volume(solid: SolidRef): Promise<number> {
    await this.ensureInit();
    const s = this.solids.get(solid);
    if (!s) return 0;
    return unwrap(measureVolume(s));
  }

  private readonly meshCache = new Map<string, MeshTransfer>();

  private meshCacheKey(solid: SolidRef, tolerance: number, model?: Model): string {
    return model ? `${String(solid)}:${tolerance}:r${model.revision}` : `${String(solid)}:${tolerance}`;
  }

  disposeSolid(solid: SolidRef): void {
    const prefix = `${String(solid)}:`;
    for (const key of [...this.meshCache.keys()]) {
      if (key.startsWith(prefix)) this.meshCache.delete(key);
    }
    this.solids.delete(solid);
  }

  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    await this.ensureInit();
    if (model) await this.syncSolidsFromModel(model);
    const s = this.solids.get(solid);
    if (!s) return emptyMeshTransfer();
    const key = this.meshCacheKey(solid, tolerance, model);
    const cached = this.meshCache.get(key);
    if (cached) return cloneMeshTransfer(cached);
    const solidRecord = model ? geom(model).solids[solid] : undefined;
    const transfer = meshTransferFromBrep(s, tolerance, { solidRef: solid, model, solidRecord });
    if (this.meshCache.size >= MESH_TRANSFER_CACHE_MAX) {
      const first = this.meshCache.keys().next().value;
      if (first) this.meshCache.delete(first);
    }
    this.meshCache.set(key, cloneMeshTransfer(transfer));
    return transfer;
  }

  /** @emoji 🧊️ Brep for one shape source solid (topology or primitive; no fused-hull metadata). */
  private brepForShapeSourceSolid(model: Model, solidId: SolidRef): ValidSolid | null {
    const rec = geom(model).solids[solidId];
    if (!rec) return null;
    const cached = this.solids.get(rec.id);
    if (cached) return cached;
    return deriveValidSolidFromRecordOrPrimitive(model, rec, (p) => this.solidFromSolidPrimitive(p));
  }

  /** @emoji 🧊️ Boolean-union brep for energy hull rows tagged with `fuseSourceSolidIds` metadata. */
  private fusedHullBrepFromMetadata(model: Model, hullId: SolidRef): ValidSolid | null {
    const meta = model.metadata.get(String(hullId));
    const raw = meta?.fuseSourceSolidIds;
    if (!Array.isArray(raw) || raw.length === 0) return null;
    const shapes: ValidSolid[] = [];
    for (const sid of raw) {
      const brep = this.brepForShapeSourceSolid(model, sid as SolidRef);
      if (brep) shapes.push(brep);
    }
    if (shapes.length === 0) return null;
    if (shapes.length === 1) return shapes[0]!;
    const fused = fuseAll(shapes);
    return isOk(fused) ? fused.value : null;
  }

  /** @emoji 🧊️ Authoritative brep for a solid: fused hull metadata, shell topology, else analytic primitive. */
  solidForSolidRecord(model: Model, solid: SolidRecord): ValidSolid | null {
    const cached = this.solids.get(solid.id);
    if (cached) return cached;
    const fusedHull = this.fusedHullBrepFromMetadata(model, solid.id);
    if (fusedHull) {
      this.solids.set(solid.id, fusedHull);
      return fusedHull;
    }
    return deriveValidSolidFromRecordOrPrimitive(model, solid, (p) => this.solidFromSolidPrimitive(p));
  }

  /** @emoji 🧊️ Resolves solid refs to live `ValidSolid` breps for boolean operands (fused hull, shell topology, or primitive). */
  validSolidsFromRefs(model: Model, refs: readonly SolidRef[]): ValidSolid[] {
    const out: ValidSolid[] = [];
    for (const ref of refs) {
      const rec = geom(model).solids[ref];
      if (!rec) continue;
      const brep = this.solidForSolidRecord(model, rec);
      if (brep) out.push(brep);
    }
    return out;
  }

  async syncSolidsFromModel(model: Model): Promise<void> {
    await this.ensureInit();
    const modelKey = this.modelDerivedKey(model);
    if (this.solidsModelKey === modelKey && this.solids.size > 0) return;
    const kernelBreps = new Map<SolidRef, ValidSolid>();
    for (const cell of Object.values(geom(model).solids)) {
      if (cell.shellIds.length > 0) continue;
      const cached = this.solids.get(cell.id);
      if (cached) kernelBreps.set(cell.id, cached);
    }
    this.solids.clear();
    this.meshCache.clear();
    for (const cell of Object.values(geom(model).solids)) {
      const brep = this.solidForSolidRecord(model, cell);
      if (brep) this.solids.set(cell.id, brep);
    }
    for (const [id, brep] of kernelBreps) {
      if (!this.solids.has(id)) this.solids.set(id, brep);
    }
    this.solidsModelKey = modelKey;
  }

  /** @emoji 🧊️ Builds brepjs `ValidSolid` from `SolidPrimitive` (sphere/cylinder/cone/box). */
  solidFromSolidPrimitive(solid: SolidPrimitive): ValidSolid {
    if (solid.kind === "sphere") {
      return sphere(solid.radius, { at: [solid.center[0], solid.center[1], solid.center[2]] });
    }
    if (solid.kind === "cylinder") {
      const h = Math.max(solid.height, 1e-6);
      const ax = solid.axis;
      const axLen = Math.hypot(ax[0], ax[1], ax[2]);
      const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
      return cylinder(solid.radius, h, {
        at: solid.base,
        axis,
        centered: false,
      });
    }
    if (solid.kind === "cone") {
      const r2 = solid.radiusTop ?? 0;
      const h = Math.max(solid.height, 1e-6);
      const ax = solid.axis;
      const axLen = Math.hypot(ax[0], ax[1], ax[2]);
      const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
      return cone(solid.radius, r2, h, {
        at: solid.base,
        axis,
        centered: false,
      });
    }
    return this.solidFromCorners({ cornerA: solid.cornerA, cornerB: solid.cornerB, height: solid.height });
  }

  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    const nextId = (kind: string) => `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
    const asVec3 = (v: unknown, fallback: Vec3): Vec3 => (Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback);
    const poleList = (raw: unknown): Vec3[] => {
      if (!Array.isArray(raw)) return [];
      const out: Vec3[] = [];
      for (const p of raw) {
        if (!Array.isArray(p) || p.length < 3) continue;
        out.push([Number(p[0]), Number(p[1]), Number(p[2])]);
      }
      return out;
    };

    const createVertex = (pos: Vec3) => {
      const id = nextId("v");
      return { id: id as VertexRef, position: pos };
    };

    if (commandId === "curve.line") {
      const p0 = asVec3(params.p0, [0, 0, 0]);
      const p1 = asVec3(params.p1, [1, 0, 0]);
      const v0 = createVertex(p0);
      const v1 = createVertex(p1);
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v0.id, v1.id], curve: { kind: "line" as const } };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [v0, v1] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.polyline") {
      const pts = poleList(params.points);
      if (pts.length < 2) return { diff: {} };
      const verts = pts.map((p) => createVertex(p));
      const edges: EdgeRecord[] = [];
      for (let i = 0; i < verts.length - 1; i++) {
        edges.push({ id: nextId("e") as EdgeRef, vertexIds: [verts[i]!.id, verts[i + 1]!.id], curve: { kind: "line" } });
      }
      const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
      return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] } } };
    }
    if (commandId === "curve.circle") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const geom = circleFromCenterRadiusPoint(center, radiusPoint);
      if (!geom) return { diff: {} };
      const brepEdge = circle(geom.radius, { at: geom.center, axis: geom.normal });
      const v = createVertex(radiusPoint);
      const curve: EdgeCurve = { kind: "circle", center: geom.center, normal: geom.normal, radius: geom.radius };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v.id, v.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      void brepEdge;
      return { diff: { vertices: { added: [v] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.arc") {
      const center = asVec3(params.center, [0, 0, 0]);
      const start = params.start != null ? asVec3(params.start, [1, 0, 0]) : ([1, 0, 0] as Vec3);
      const endRaw = params.end != null ? asVec3(params.end, start) : null;
      const angle = typeof params.angle === "number" ? params.angle : null;
      let endPos: Vec3;
      if (endRaw) endPos = arcEndOnCircle(center, start, endRaw);
      else if (angle !== null) endPos = arcEndFromAngle(center, start, angle) ?? start;
      else endPos = start;
      const mid = arcSamplePoints(center, start, endPos, 4)[2] ?? endPos;
      const brepEdge = threePointArc(start, mid, endPos);
      const vStart = createVertex(curveStartPoint(brepEdge));
      const vEnd = createVertex(curveEndPoint(brepEdge));
      const curve: EdgeCurve = { kind: "arc", center };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.controlPointCurve" || commandId === "curve.interpolateCurve") {
      const poles = poleList(params.points);
      if (poles.length < 2) return { diff: {} };
      const through = commandId === "curve.interpolateCurve";
      const fitPoints = through ? [...nurbsDisplaySamplePoints(poles, 16)] : poles;
      const brepResult = bsplineApprox(fitPoints);
      if (!isOk(brepResult)) return { diff: {} };
      const curve = nurbsCurveFromPoles(poles, through);
      if (!curve) return { diff: {} };
      const vStart = createVertex(curveStartPoint(brepResult.value));
      const vEnd = createVertex(curveEndPoint(brepResult.value));
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "solid.sphere") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = params.radiusPoint != null ? asVec3(params.radiusPoint, center) : null;
      const radius = typeof params.radius === "number" ? params.radius : radiusPoint ? vec3Distance(center, radiusPoint) : 1;
      const solid: SolidPrimitive = { kind: "sphere", center, radius };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cylinder") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, base);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cone") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, [0, 0, 1] as Vec3);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    const fuseShapes = (shapes: readonly ValidSolid[]): ValidSolid | null => {
      if (shapes.length === 0) return null;
      if (shapes.length === 1) return shapes[0]!;
      const fused = fuseAll(shapes as ValidSolid[]);
      return isOk(fused) ? fused.value : null;
    };
    if (commandId === "solid.booleanUnion") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const refs = solidRefsFromSelectionRaw(model, params.targets);
      if (refs.length < 2) return { diff: {} };
      const shapes = this.validSolidsFromRefs(model, refs);
      if (shapes.length < 2) return { diff: {} };
      const fused = fuseAll(shapes);
      if (!isOk(fused)) return { diff: {} };
      const ref = kernelGeometry.solidRef(nextId("union"));
      this.solids.set(ref, fused.value);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: refs } } };
    }
    if (commandId === "solid.booleanDifference") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const baseRefs = solidRefsFromSelectionRaw(model, params.baseObjects);
      const toolRefs = solidRefsFromSelectionRaw(model, params.cutterObjects);
      if (!baseRefs.length || !toolRefs.length) return { diff: {} };
      const baseShape = fuseShapes(this.validSolidsFromRefs(model, baseRefs));
      const toolShapes = this.validSolidsFromRefs(model, toolRefs);
      if (!baseShape || !toolShapes.length) return { diff: {} };
      let result = baseShape;
      for (const tool of toolShapes) {
        const cutResult = cut(result, tool);
        if (!isOk(cutResult)) return { diff: {} };
        result = cutResult.value;
      }
      const ref = kernelGeometry.solidRef(nextId("diff"));
      this.solids.set(ref, result);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...baseRefs, ...toolRefs] } } };
    }
    if (commandId === "solid.booleanIntersection") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const firstRefs = solidRefsFromSelectionRaw(model, params.firstSet);
      const secondRefs = solidRefsFromSelectionRaw(model, params.secondSet);
      if (!firstRefs.length || !secondRefs.length) return { diff: {} };
      const firstShape = fuseShapes(this.validSolidsFromRefs(model, firstRefs));
      const secondShape = fuseShapes(this.validSolidsFromRefs(model, secondRefs));
      if (!firstShape || !secondShape) return { diff: {} };
      const isected = intersect(firstShape, secondShape);
      if (!isOk(isected)) return { diff: {} };
      const ref = kernelGeometry.solidRef(nextId("isect"));
      this.solids.set(ref, isected.value);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...firstRefs, ...secondRefs] } } };
    }
    if (commandId.startsWith("solid.")) {
      return { diff: {} };
    }
    if (commandId === "transform.mirror") {
      const v0 = createVertex([0, 0, 0]);
      return { diff: { vertices: { added: [v0] } } };
    }
    if (commandId.endsWith("From2PointsAndHeight")) {
      const p0 = asVec3(params.pointA, asVec3(params.p0, [0, 0, 0]));
      const p1 = asVec3(params.pointB, asVec3(params.p1, [1, 1, 0]));
      const height = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const cornerA: Vec3 = [Math.min(p0[0], p1[0]), Math.min(p0[1], p1[1]), Math.min(p0[2], p1[2])];
      const cornerB: Vec3 = [Math.max(p0[0], p1[0]), Math.max(p0[1], p1[1]), Math.max(p0[2], p1[2])];
      return this.createBoxFromCornersDiff({ cornerA, cornerB, height });
    }
    if (commandId.endsWith("FromCurveAndHeight")) {
      const wireId = String(params.wireId ?? "");
      const distance = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const model = params.model instanceof Model ? params.model : null;
      if (wireId && model) return this.extrudeWireDiff({ wireId, distance, direction: [0, 0, 1], model });
      return { diff: {} };
    }
    if (commandId.endsWith("FromSurface")) {
      const faceId = String(params.faceId ?? "");
      const model = params.model instanceof Model ? params.model : null;
      if (faceId && model) return this.offsetFacesDiff({ faceIds: [faceId], distance: 0.01, model });
      return { diff: {} };
    }
    if (commandId === "surface.extrudeCrv") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const distance = extrusionDistanceFromParams(params);
      if (!(distance > 1e-9)) return { diff: {} };
      const direction = asVec3(params.direction, [0, 0, 1]);
      const wireIds = wireIdsFromSelectionPicks(model, selectionPicksFromParams(params));
      if (!wireIds.length) return { diff: {} };
      const diffs: ModelDiff[] = [];
      for (const wireId of wireIds) {
        const row = await this.extrudeWireDiff({ wireId, distance, direction, model });
        if (!isEmptyModelDiff(row.diff)) diffs.push(row.diff);
      }
      return { diff: mergeSolidAdds(diffs) };
    }
    if (commandId === "surface.plane") {
      const cornerA = asVec3(params.cornerA, [0, 0, 0]);
      const cornerB = asVec3(params.cornerB, [1, 0, 0]);
      const along = vec3Sub(cornerB, cornerA);
      const side = vec3Length(along) > 1e-9 ? vec3Length(along) : 1;
      const dir = vec3Length(along) > 1e-9 ? vec3Normalize(along) : ([1, 0, 0] as Vec3);
      let perp = vec3Cross(dir, [0, 0, 1]);
      if (vec3Length(perp) < 1e-9) perp = vec3Cross(dir, [0, 1, 0]);
      perp = vec3Normalize(perp);
      const p0 = cornerA;
      const p1 = vec3Add(cornerA, vec3Scale(dir, side));
      const p2 = vec3Add(p1, vec3Scale(perp, side));
      const p3 = vec3Add(cornerA, vec3Scale(perp, side));
      const corners = [p0, p1, p2, p3];
      await this.ensureInit();
      const brepEdges = corners.map((p, i) => line(p, corners[(i + 1) % 4]!));
      const loopResult = wireLoop(brepEdges);
      if (!isOk(loopResult)) return { diff: {} };
      const faceResult = face(loopResult.value as Parameters<typeof face>[0]);
      if (!isOk(faceResult)) return { diff: {} };
      const verts = corners.map((p) => createVertex(p));
      const edges: EdgeRecord[] = verts.map((v, i) => ({ id: nextId("e") as EdgeRef, vertexIds: [v.id, verts[(i + 1) % 4]!.id], curve: { kind: "line" as const } }));
      const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
      const normal = vec3Normalize(vec3Cross(dir, perp));
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [w.id], surface: { kind: "plane", origin: p0, normal } };
      return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] }, faces: { added: [f] } } };
    }
    if (commandId === "surface.loft" || commandId === "surface.networkSrf") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const wireIds = wireIdsFromSelectionPicks(model, picksFromRaw(params.curves));
      if (!wireIds.length) return { diff: {} };
      await this.ensureInit();
      if (wireIds.length === 1) {
        const planar = geomWireToOrientedFaceLoose(model, wireIds[0]!);
        if (!planar) return { diff: {} };
        const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [wireIds[0]!] };
        return { diff: { faces: { added: [f] } } };
      }
      const brepWires: Wire[] = [];
      for (const wid of wireIds) {
        const bw = geomWireToBrepWire(model, wid);
        if (!bw) return { diff: {} };
        brepWires.push(bw);
      }
      const lofted = loft(brepWires as Parameters<typeof loft>[0], { ruled: true });
      if (!isOk(lofted)) return { diff: {} };
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [...wireIds] };
      return { diff: { faces: { added: [f] } } };
    }
    if (commandId === "surface.sweep1" || commandId === "surface.sweep2") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const railIds = wireIdsFromKeyedRaw(model, params.targets);
      if (!railIds.length) return { diff: {} };
      const sectionIds = wireIdsFromSelectionPicks(model, picksFromRaw(params.sections));
      await this.ensureInit();
      const railWire = geomWireToBrepWire(model, railIds[0]!);
      if (!railWire) return { diff: {} };
      const profileWire = sectionIds.length ? geomWireToBrepWire(model, sectionIds[0]!) : defaultSweepProfileWire(model, railIds[0]!);
      if (!profileWire) return { diff: {} };
      const swept = sweepAlongSpine(profileWire as Parameters<typeof sweepAlongSpine>[0], railWire as Parameters<typeof sweepAlongSpine>[1]);
      if (!isOk(swept)) return { diff: {} };
      const boundaryWireIds = sectionIds.length ? [railIds[0]!, sectionIds[0]!] : [railIds[0]!];
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: boundaryWireIds };
      return { diff: { faces: { added: [f] } } };
    }
    if (commandId === "edit.join") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editJoinDiff(model, params) };
    }
    if (commandId === "edit.explode") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editExplodeDiff(model, params) };
    }
    if (commandId === "edit.split") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editSplitDiff(model, params) };
    }
    if (commandId === "edit.trim") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editTrimDiff(model, params) };
    }
    if (commandId === "edit.chamfer" || commandId === "edit.fillet") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const keyed = params.targets as { readonly firstCurve?: unknown; readonly secondCurve?: unknown } | undefined;
      const idA = edgeIdsFromPicks(model, picksFromValue(keyed?.firstCurve))[0] ?? null;
      const idB = edgeIdsFromPicks(model, picksFromValue(keyed?.secondCurve))[0] ?? null;
      if (!idA || !idB) return { diff: {} };
      return { diff: cornerConnectorDiff(model, idA, idB, commandId === "edit.chamfer" ? "chamfer" : "fillet") };
    }

    return { diff: {} };
  }

  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = await this.createBoxFromCorners(input);
    const diff = boxModelDiff(input, solid);
    return { diff, solid };
  }

  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = await this.extrudeWire(input);
    return { diff: { solids: { added: [{ id: solid, shellIds: [] }] } }, solid };
  }

  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    await this.ensureInit();
    const fid = input.faceIds[0];
    if (!fid) return { diff: {} };
    const fr = geom(input.model).faces[fid];
    const wireId = fr?.wireIds[0];
    if (!wireId) return { diff: {} };
    const planar = geomWireToOrientedFace(input.model, wireId);
    if (!planar) return { diff: {} };
    const offset = offsetFace(planar, input.distance);
    if (!isOk(offset)) return { diff: {} };
    const offsetSolid = offset.value;
    if (!isSolid(offsetSolid) || !isValidSolid(offsetSolid)) return { diff: {} };
    const ref = kernelGeometry.solidRef(`brepjs-offset-${++this.seq}`);
    this.solids.set(ref, offsetSolid);
    return { diff: { solids: { added: [{ id: ref, shellIds: [] }] } } };
  }

  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    await this.ensureInit();
    const pa = geom(model).vertices[String(a)]?.position;
    const pb = geom(model).vertices[String(b)]?.position;
    if (!pa || !pb) return 0;
    return unwrap(measureDistance(brepVertex(pa), brepVertex(pb)));
  }

  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    await this.ensureInit();
    const ed = geom(model).edges[String(e)];
    if (!ed) return 0;
    const brepEdge = geomEdgeToBrepEdge(model, ed);
    if (brepEdge) return unwrap(measureLength(brepEdge));
    const ends: Vec3[] = [];
    for (const vid of ed.vertexIds) {
      const p = geom(model).vertices[String(vid)]?.position;
      if (p) ends.push(p);
    }
    if (ends.length < 2) return 0;
    return edgeCurveLength(ed.curve, [ends[0]!, ends[1]!]);
  }

  async faceArea(f: FaceRef, model: Model): Promise<number> {
    await this.ensureInit();
    const fr = geom(model).faces[String(f)];
    const wireId = fr?.wireIds[0];
    if (!wireId) return 0;
    const planar = geomWireToOrientedFace(model, wireId);
    if (planar) return unwrap(measureArea(planar));
    return 0;
  }

  async solidVolume(c: SolidRef): Promise<number> {
    return this.volume(c);
  }

  async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    const out = new Set<string>();
    const c = geom(model).solids[String(cell)];
    if (!c) return [];
    const faces = new Set<string>();
    for (const sid of c.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const f of sh.faceIds) faces.add(f);
    }
    for (const f of faces) {
      for (const [cid, cellRec] of Object.entries(geom(model).solids)) {
        if (cid === String(cell)) continue;
        for (const sid of cellRec.shellIds) {
          const sh = geom(model).shells[sid];
          if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
        }
      }
    }
    return [...out].map((id) => id as SolidRef);
  }

  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    const ca = geom(model).solids[String(a)];
    const cb = geom(model).solids[String(b)];
    if (!ca || !cb) return [];
    const fa = new Set<string>();
    const fb = new Set<string>();
    for (const sid of ca.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fa.add(fid);
    }
    for (const sid of cb.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fb.add(fid);
    }
    const xs: FaceRef[] = [];
    for (const x of fa) if (fb.has(x)) xs.push(x as FaceRef);
    return xs;
  }

  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    await this.ensureInit();
    const solid = extrudeModelWire(input.model, input.wireId, input.direction, input.distance);
    if (!solid) throw new Error(`Cannot extrude wire ${input.wireId}`);
    const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
    this.solids.set(ref, solid);
    return ref;
  }

  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    await this.offsetFacesDiff(input);
  }

}
// #endregion 🔌️BrepjsWasmEngine

// #region 📨️WorkerProtocol
type BrepjsWorkerRequest = { readonly type: "init" } | { readonly type: "rpc"; readonly id: string; readonly method: string; readonly args: readonly unknown[] };

type BrepjsWorkerResponse =
  | { readonly type: "init-done" }
  | { readonly type: "init-error"; readonly error: string }
  | { readonly type: "rpc-result"; readonly id: string; readonly result: unknown }
  | { readonly type: "rpc-error"; readonly id: string; readonly error: string };

/** @emoji 📨️ Walks RPC payloads so nested `Model` / `ModelSpace` survive worker `postMessage`. */
function serializeWorkerValue(value: unknown): unknown {
  if (value instanceof Model) return { __modelJson: value.toJSON() };
  if (value instanceof ModelSpace) return { __modelSpaceJson: value.toJSON() };
  if (Array.isArray(value)) return value.map(serializeWorkerValue);
  if (!value || typeof value !== "object") return value;
  const row = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(row)) out[key] = serializeWorkerValue(entry);
  return out;
}

function serializeWorkerArg(arg: unknown): unknown {
  return serializeWorkerValue(arg);
}

/** @emoji 📨️ Restores `Model` / `ModelSpace` instances from worker RPC payloads. */
function deserializeWorkerValue(value: unknown): unknown {
  if (value && typeof value === "object" && "__modelJson" in value) {
    return Model.fromJSON((value as { readonly __modelJson: ModelJson }).__modelJson);
  }
  if (value && typeof value === "object" && "__modelSpaceJson" in value) {
    return ModelSpace.fromJSON((value as { readonly __modelSpaceJson: Parameters<typeof ModelSpace.fromJSON>[0] }).__modelSpaceJson);
  }
  if (Array.isArray(value)) return value.map(deserializeWorkerValue);
  if (!value || typeof value !== "object") return value;
  const row = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(row)) out[key] = deserializeWorkerValue(entry);
  return out;
}

function deserializeWorkerArg(arg: unknown): unknown {
  return deserializeWorkerValue(arg);
}
// #endregion 📨️WorkerProtocol

// #region 🎬️BrepjsWorkerClient
/** @emoji 🎬️ Routes brepjs RPC to a dedicated worker or local `BrepjsWasmEngine` (vitest / no Worker). */
class BrepjsWorkerClient {
  private localEngine: BrepjsWasmEngine | null = null;
  private worker: Worker | null = null;
  private initPromise: Promise<void> | null = null;
  private readonly pending = new Map<string, { readonly resolve: (v: unknown) => void; readonly reject: (e: Error) => void }>();

  private localWasmEngine(): BrepjsWasmEngine {
    if (!this.localEngine) this.localEngine = new BrepjsWasmEngine();
    return this.localEngine;
  }

  constructor() {
    const workerDisabled = isBrepjsTestRun || openCascadeWasmNeedsNodeResolve;
    if (!workerDisabled && typeof Worker !== "undefined") {
      try {
        this.worker = new Worker(new URL("./index.ts", import.meta.url), { type: "module" });
        this.worker.onmessage = (event: MessageEvent<BrepjsWorkerResponse>) => this.onWorkerMessage(event.data);
      } catch {
        this.worker = null;
      }
    }
  }

  private onWorkerMessage(msg: BrepjsWorkerResponse): void {
    if (msg.type === "init-done") {
      const init = this.pending.get("init");
      init?.resolve(undefined);
      this.pending.delete("init");
      return;
    }
    if (msg.type === "init-error") {
      const init = this.pending.get("init");
      init?.reject(new Error(msg.error));
      this.pending.delete("init");
      return;
    }
    if (msg.type === "rpc-result") {
      const p = this.pending.get(msg.id);
      p?.resolve(msg.result);
      this.pending.delete(msg.id);
      return;
    }
    if (msg.type === "rpc-error") {
      const p = this.pending.get(msg.id);
      p?.reject(new Error(msg.error));
      this.pending.delete(msg.id);
    }
  }

  async ensureReady(): Promise<void> {
    if (!this.initPromise) this.initPromise = this.boot();
    await this.initPromise;
  }

  private async boot(): Promise<void> {
    if (!this.worker) {
      await this.localWasmEngine().ensureInit();
      return;
    }
    await new Promise<void>((resolve, reject) => {
      this.pending.set("init", { resolve: () => resolve(), reject });
      this.worker!.postMessage({ type: "init" } satisfies BrepjsWorkerRequest);
    });
  }

  async rpc<T>(method: string, args: readonly unknown[]): Promise<T> {
    await this.ensureReady();
    const serialized = args.map(serializeWorkerArg);
    if (!this.worker) {
      const hydrated = serialized.map(deserializeWorkerArg);
      const engine = this.localWasmEngine();
      const fn = (engine as unknown as Record<string, unknown>)[method];
      if (typeof fn !== "function") throw new Error(`brepjs rpc: unknown method ${method}`);
      return (await (fn as (...a: unknown[]) => unknown).apply(engine, hydrated)) as T;
    }
    return new Promise<T>((resolve, reject) => {
      const id = crypto.randomUUID();
      this.pending.set(id, {
        resolve: (v) => resolve(v as T),
        reject,
      });
      this.worker!.postMessage({ type: "rpc", id, method, args: serialized } satisfies BrepjsWorkerRequest);
    });
  }
}
// #endregion 🎬️BrepjsWorkerClient

// #region 🔌️BrepjsKernel
/** @emoji 🔌️ `SpatialKernel` facade: preview math on main thread, WASM in worker via `BrepjsWorkerClient`. */
export class BrepjsKernel extends PreciseSpatialKernelMath implements SpatialKernel {
  readonly id: string = "brepjs-opencascade";
  private readonly wasm = new BrepjsWorkerClient();

  readonly operations: readonly string[] = ["solid.createBox", "wire.extrudeToSolid", "face.offset", "entity.tessellate", "measure.distance", "measure.area", "measure.volume"];

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    return this.wasm.rpc("createBoxFromCorners", [input]);
  }

  async volume(cell: SolidRef): Promise<number> {
    return this.wasm.rpc("volume", [cell]);
  }

  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    return this.wasm.rpc("tessellate", [solid, tolerance, model]);
  }

  /** @emoji 🧪️ Clears worker solids cache between vitest cases. */
  async resetDerivedPipelineForTest(): Promise<void> {
    return this.wasm.rpc("resetDerivedPipeline", []);
  }

  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    return this.wasm.rpc("executeCommandDiff", [commandId, params]);
  }

  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.wasm.rpc("createBoxFromCornersDiff", [input]);
  }

  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    return this.wasm.rpc("extrudeWireDiff", [input]);
  }

  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    return this.wasm.rpc("offsetFacesDiff", [input]);
  }

  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    return this.wasm.rpc("vertexDistance", [a, b, model]);
  }

  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    return this.wasm.rpc("edgeLength", [e, model]);
  }

  async faceArea(f: FaceRef, model: Model): Promise<number> {
    return this.wasm.rpc("faceArea", [f, model]);
  }

  async solidVolume(c: SolidRef): Promise<number> {
    return this.wasm.rpc("solidVolume", [c]);
  }

  async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    return this.wasm.rpc("adjacentSolids", [cell, model]);
  }

  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    return this.wasm.rpc("sharedFacesBetween", [a, b, model]);
  }

  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    return this.wasm.rpc("extrudeWire", [input]);
  }

  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    await this.wasm.rpc("offsetFaces", [input]);
  }

  disposeSolid(cell: SolidRef): void {
    void this.wasm.rpc("disposeSolid", [cell]);
  }

  async syncSolidsFromModel(model: Model): Promise<void> {
    await this.wasm.rpc("syncSolidsFromModel", [model]);
  }

  /** @emoji 🪜️ Exports a linked `ModelSpace` to AP242 STEP. */
  async exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
    return this.wasm.rpc("exportModelSpaceToStep", [space, modelSpaceId]);
  }

  /** @emoji 🪜️ Exports one `Model` to AP242 STEP. */
  async exportModelToStep(model: Model, modelId = "model"): Promise<string> {
    return this.wasm.rpc("exportModelToStep", [model, modelId]);
  }

  /** @emoji 🪜️ Imports AP242 STEP into a `ModelSpace`. */
  async importStepToModelSpace(stepText: string): Promise<ModelSpace> {
    return this.wasm.rpc("importStepToModelSpace", [stepText]);
  }

  /** @emoji 🪜️ Imports raw AP242 BREP STEP (no spatial UDA) into a `ModelSpace`. */
  async importStepBrepToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number }): Promise<ModelSpace> {
    return this.wasm.rpc("importStepBrepToModelSpace", [stepText, options]);
  }

  /** @emoji 🏗️ Imports AP242 BREP STEP with presentation layers into a building `ModelSpace`. */
  async importStepBimToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number; readonly modelDefinitionId?: string }): Promise<ModelSpace> {
    return this.wasm.rpc("importStepBimToModelSpace", [stepText, options]);
  }
}

/** @emoji 🪜️ Exports `space` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelSpaceToStep(space: ModelSpace, modelSpaceId = "default"): Promise<string> {
  const kernel = new BrepjsKernel();
  return kernel.exportModelSpaceToStep(space, modelSpaceId);
}

/** @emoji 💾️ Exports `space` solids as merged OBJ via tessellation. */
export async function exportModelSpaceToObj(space: ModelSpace, deflection = 0.1): Promise<string> {
  const { meshTransferToObj, mergeMeshTransfers } = await import("@semio-tech/s-3d-js");
  const kernel = new BrepjsKernel();
  const meshes = [];
  for (const model of Object.values(space.models)) {
    for (const solid of Object.values(model.solids)) {
      const mesh = await kernel.tessellate(solid.id, deflection, model);
      if (mesh.position.length > 0 && mesh.index.length > 0) meshes.push(mesh);
    }
  }
  if (meshes.length === 0) return "# empty model space\n";
  return meshTransferToObj(mergeMeshTransfers(meshes));
}

/** @emoji 💾️ Exports `space` solids as merged GLB via tessellation. */
export async function exportModelSpaceToGlb(space: ModelSpace, deflection = 0.1): Promise<Uint8Array> {
  const { meshTransferToGlb, mergeMeshTransfers } = await import("@semio-tech/s-3d-js");
  const kernel = new BrepjsKernel();
  const meshes = [];
  for (const model of Object.values(space.models)) {
    for (const solid of Object.values(model.solids)) {
      const mesh = await kernel.tessellate(solid.id, deflection, model);
      if (mesh.position.length > 0 && mesh.index.length > 0) meshes.push(mesh);
    }
  }
  if (meshes.length === 0) return new Uint8Array([0x67, 0x6c, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00]);
  return meshTransferToGlb(mergeMeshTransfers(meshes));
}

/** @emoji 📐️ Exports `space` solids as merged DWG bytes via tessellation, routed through the Rust DWG codec (flow-core wasm) rather than OpenCascade. */
export async function exportModelSpaceToDwg(space: ModelSpace, deflection = 0.1): Promise<Uint8Array> {
  const { mergeMeshTransfers, emptyMeshTransfer } = await import("@semio-tech/s-3d-js");
  const kernel = new BrepjsKernel();
  const meshes = [];
  for (const model of Object.values(space.models)) {
    for (const solid of Object.values(model.solids)) {
      const mesh = await kernel.tessellate(solid.id, deflection, model);
      if (mesh.position.length > 0 && mesh.index.length > 0) meshes.push(mesh);
    }
  }
  const flowCore = (await import("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/flow_core.js")) as { dwg_encode_mesh_json?: (meshJson: string) => string };
  if (typeof flowCore.dwg_encode_mesh_json !== "function") {
    throw new Error("dwg_encode_mesh_json export missing — rebuild flow/core wasm");
  }
  const merged = meshes.length > 0 ? mergeMeshTransfers(meshes) : emptyMeshTransfer();
  const meshJson = JSON.stringify({ positions: Array.from(merged.position), normals: Array.from(merged.normal), indices: Array.from(merged.index) });
  const raw = JSON.parse(flowCore.dwg_encode_mesh_json(meshJson)) as { dwg?: string; error?: string };
  if (raw.error) throw new Error(raw.error);
  if (typeof raw.dwg !== "string") throw new Error("dwg_encode_mesh_json missing payload");
  const binary = atob(raw.dwg);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

/** @emoji 🪜️ Exports `model` via a fresh `BrepjsKernel` (convenience). */
export async function exportModelToStep(model: Model, modelId = "model"): Promise<string> {
  const kernel = new BrepjsKernel();
  return kernel.exportModelToStep(model, modelId);
}

/** @emoji 🪜️ Imports STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepToModelSpace(stepText: string): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepToModelSpace(stepText);
}

/** @emoji 🪜️ Imports raw BREP STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepBrepToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number }): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepBrepToModelSpace(stepText, options);
}

/** @emoji 🏗️ Imports presentation-layer BIM STEP text via a fresh `BrepjsKernel` (convenience). */
export async function importStepBimToModelSpace(stepText: string, options?: { readonly prefix?: string; readonly lengthScale?: number; readonly modelDefinitionId?: string }): Promise<ModelSpace> {
  const kernel = new BrepjsKernel();
  return kernel.importStepBimToModelSpace(stepText, options);
}
// #endregion 🔌️BrepjsKernel

// #region 🌐️BrepjsWorkerEntry
function isBrepjsDedicatedWorker(): boolean {
  return typeof self !== "undefined" && self.constructor?.name === "DedicatedWorkerGlobalScope";
}

if (isBrepjsDedicatedWorker()) {
  let engine: BrepjsWasmEngine | null = null;
  self.addEventListener("message", (event: MessageEvent<BrepjsWorkerRequest>) => {
    const msg = event.data;
    if (msg.type === "init") {
      void (async () => {
        try {
          engine = new BrepjsWasmEngine();
          await engine.ensureInit();
          self.postMessage({ type: "init-done" } satisfies BrepjsWorkerResponse);
        } catch (e) {
          self.postMessage({
            type: "init-error",
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
      return;
    }
    if (msg.type === "rpc" && engine) {
      void (async () => {
        try {
          const args = msg.args.map(deserializeWorkerArg);
          const fn = (engine as unknown as Record<string, unknown>)[msg.method];
          if (typeof fn !== "function") throw new Error(`brepjs worker: unknown method ${msg.method}`);
          const result = await (fn as (...a: unknown[]) => unknown).apply(engine, args);
          const transfers = result && typeof result === "object" && "position" in result ? meshTransferTransferables(result as MeshTransfer) : undefined;
          if (transfers?.length) {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse, {
              transfer: transfers,
            });
          } else {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse);
          }
        } catch (e) {
          self.postMessage({
            type: "rpc-error",
            id: msg.id,
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
    }
  });
}
// #endregion 🌐️BrepjsWorkerEntry

// #region 🧪️Tests
if (import.meta.vitest) {
  const { beforeEach, describe, expect, it } = import.meta.vitest;
  const { bootstrapCadModules } = await import("../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  const { AEC_BUILDING_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building");
  const { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-energy");
  const { AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID } = await import("@semio-tech/cad-js-module-aec-building-structure");

  bootstrapCadModules();

  describe("@semio-tech/cad-js/brepjs", () => {
    const kernel = new BrepjsKernel();

    beforeEach(async () => {
      await kernel.resetDerivedPipelineForTest();
    });

    it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
      const cell = await kernel.createBoxFromCorners({
        cornerA: [0, 0, 0],
        cornerB: [2, 3, 0],
        height: 4,
      });
      const vol = await kernel.volume(cell);
      expect(vol).toBeCloseTo(24, 3);
    });

    it("tessellate returns non-empty mesh for a box", async () => {
      const cell = await kernel.createBoxFromCorners({
        cornerA: [0, 0, 0],
        cornerB: [1, 1, 0],
        height: 1,
      });
      const meshTransfer = await kernel.tessellate(cell, 1e-3);
      expect(meshTransfer.index.length).toBeGreaterThan(0);
      expect(meshTransfer.position.length).toBeGreaterThan(0);
      expect(meshTransfer.faceGroups.length).toBeGreaterThan(0);
    });

    it("syncSolidsFromModel rebuilds box volume after planar vertex move (not stale primitive)", async () => {
      const g = new Model();
      const solid = kernelGeometry.solidRef("moved-box");
      applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      await kernel.syncSolidsFromModel(g);
      expect(await kernel.volume(solid)).toBeCloseTo(1, 3);
      for (const [id, vert] of Object.entries(g.vertices)) {
        if (!id.includes("moved-box") || vert.position[2] < 0.5) continue;
        g.vertices[id as VertexRef] = { id: vert.id, position: [vert.position[0], vert.position[1], vert.position[2] + 1] };
      }
      g.bump();
      await kernel.syncSolidsFromModel(g);
      expect(await kernel.volume(solid)).toBeCloseTo(2, 2);
      const mesh = await kernel.tessellate(solid, 1e-3, g);
      expect(mesh.index.length).toBeGreaterThan(0);
    });

    it("extrudeWireDiff registers kernel brep for tessellation without meshFaceModelDiff shell", async () => {
      const g = new Model();
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const e0 = "e0" as EdgeRef;
      const w0 = "w0" as WireRef;
      applyModelDiff(g, {
        vertices: {
          added: [
            { id: v0, position: [0, 0, 0] },
            { id: v1, position: [1, 0, 0] },
          ],
        },
        edges: { added: [{ id: e0, vertexIds: [v0, v1] }] },
        wires: { added: [{ id: w0, edgeIds: [e0] }] },
      });
      const { diff, solid } = await kernel.extrudeWireDiff({ wireId: w0, distance: 2, direction: [0, 0, 1], model: g });
      applyModelDiff(g, diff);
      g.bump();
      const mesh = await kernel.tessellate(solid, 1e-3, g);
      expect(mesh.index.length).toBeGreaterThan(0);
      expect(Object.keys(g.faces).some((id) => id.startsWith("cm-"))).toBe(false);
    });

    it("extrudeWireDiff lofts open nurbs interpolate wires to solids", async () => {
      const g = new Model();
      const res = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model: g,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      applyModelDiff(g, res.diff);
      const wireId = res.diff.wires?.added?.[0]?.id;
      expect(wireId).toBeTruthy();
      const { diff, solid } = await kernel.extrudeWireDiff({
        wireId: String(wireId),
        distance: 1.2,
        direction: [0, 0, 1],
        model: g,
      });
      expect(diff.solids?.added?.length).toBe(1);
      applyModelDiff(g, diff);
      g.bump();
      const mesh = await kernel.tessellate(solid, 1e-3, g);
      expect(mesh.index.length).toBeGreaterThan(0);
    });

    it("executeCommandDiff surface.extrudeCrv extrudes selected wires along direction", async () => {
      const g = new Model();
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const e0 = "e0" as EdgeRef;
      const w0 = "w0" as WireRef;
      applyModelDiff(g, {
        vertices: {
          added: [
            { id: v0, position: [0, 0, 0] },
            { id: v1, position: [1, 0, 0] },
          ],
        },
        edges: { added: [{ id: e0, vertexIds: [v0, v1] }] },
        wires: { added: [{ id: w0, edgeIds: [e0] }] },
      });
      const res = await kernel.executeCommandDiff("surface.extrudeCrv", {
        model: g,
        curves: [{ kind: "wire", id: w0 }],
        direction: [0, 0, 1],
        distance: 1.5,
        origin: [0, 0, 0],
        cursor: [0, 0, 1.5],
      });
      expect(res.diff.solids?.added?.length).toBe(1);
    });

    it("syncSolidsFromModel follows sheared box shell (not axis-aligned primitive proxy)", async () => {
      const g = new Model();
      const solid = kernelGeometry.solidRef("sheared-box");
      applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      const corner = g.vertices["box-sheared-box-v111" as VertexRef];
      expect(corner).toBeDefined();
      g.vertices["box-sheared-box-v111" as VertexRef] = { id: corner!.id, position: [1.4, 1.2, 1] };
      g.bump();
      await kernel.syncSolidsFromModel(g);
      const vol = await kernel.volume(solid);
      expect(vol).toBeGreaterThan(1.05);
      expect(vol).toBeLessThan(1.35);
      const mesh = await kernel.tessellate(solid, 1e-3, g);
      expect(mesh.index.length).toBeGreaterThan(0);
    });

    it("tessellate maps brep faces to model FaceRef entityIds when model is provided", async () => {
      const g = new Model();
      const solid = kernelGeometry.solidRef("box-pick");
      applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      await kernel.syncSolidsFromModel(g);
      const meshTransfer = await kernel.tessellate(solid, 1e-3, g);
      expect(meshTransfer.faceInfos.length).toBeGreaterThan(0);
      const modelFaceIds = new Set(Object.keys(g.faces));
      for (const info of meshTransfer.faceInfos) {
        expect(typeof info.entityId).toBe("string");
        expect(modelFaceIds.has(String(info.entityId))).toBe(true);
      }
      for (const group of meshTransfer.faceGroups) {
        expect(modelFaceIds.has(String(group.entityId))).toBe(true);
      }
    });

    it("tessellate returns cached mesh with equal buffers for same solid and tolerance", async () => {
      const cell = await kernel.createBoxFromCorners({
        cornerA: [0, 0, 0],
        cornerB: [1, 1, 0],
        height: 1,
      });
      const tol = 1e-3;
      const a = await kernel.tessellate(cell, tol);
      const b = await kernel.tessellate(cell, tol);
      expect(a.index.length).toBe(b.index.length);
      expect(a.position.length).toBe(b.position.length);
      expect([...a.index]).toEqual([...b.index]);
    });

    it("disposeSolid clears tessellation cache for that solid", async () => {
      const cell = await kernel.createBoxFromCorners({
        cornerA: [0, 0, 0],
        cornerB: [1, 1, 0],
        height: 1,
      });
      const before = await kernel.tessellate(cell, 1e-3);
      kernel.disposeSolid(cell);
      const after = await kernel.tessellate(cell, 1e-3);
      expect(after.index.length).toBe(0);
      expect(before.index.length).toBeGreaterThan(0);
    });

    it("createBoxFromCornersDiff includes one face bucket", async () => {
      const r = await kernel.createBoxFromCornersDiff({
        cornerA: [0, 0, 0],
        cornerB: [1, 1, 0],
        height: 1,
      });
      expect(r.solid).toBeDefined();
      expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
      expect(await kernel.volume(r.solid)).toBeGreaterThan(0);
    });

    it("topology preview geometry resolves face centroid and fuse external faces", () => {
      const model = new Model();
      const west = solidRef("west");
      const east = solidRef("east");
      applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, west));
      applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 1], cornerB: [1, 1, 1], height: 1 }, east));
      const westTop = Object.keys(model.faces).find((id) => id.includes("face-top") && id.includes("west"))!;
      const face = model.faces[westTop as FaceRef]!;
      const centroid = faceCentroid(model, face);
      expect(centroid).not.toBeNull();
      expect(centroid![2]).toBeCloseTo(1, 3);
      const fused = fuseSolidsToExternalFaces(model, [west, east], {
        hullSolidId: "hull",
        contactPairs: [
          ["face-top", "face-bottom"],
          ["face-bottom", "face-top"],
        ],
        maxSeparation: 0.05,
      });
      const westTopFace = Object.keys(model.faces).find((id) => id.includes("west") && id.includes("face-top"))!;
      const eastBottomFace = Object.keys(model.faces).find((id) => id.includes("east") && id.includes("face-bottom"))!;
      expect(fused.externalFaces.map(String)).not.toContain(westTopFace);
      expect(fused.externalFaces.map(String)).not.toContain(eastBottomFace);
      expect(fused.externalFaces.some((id) => String(id).includes("west") && String(id).includes("face-bottom"))).toBe(true);
    });

    it("modelObjectAabb follows moved shell vertices when SolidPrimitive is stale", () => {
      const model = new Model();
      const cell = kernelGeometry.solidRef("box");
      applyModelDiff(model, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
      const rec = geom(model).solids[cell]! as MutableSolidRecord;
      rec.solid = { kind: "box", cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 };
      const before = modelObjectAabb(model, rec)!;
      let topId = Object.keys(geom(model).vertices)[0]!;
      let topZ = geom(model).vertices[topId]!.position[2];
      for (const [id, vert] of Object.entries(geom(model).vertices)) {
        if (vert.position[2] > topZ) {
          topZ = vert.position[2];
          topId = id;
        }
      }
      const top = geom(model).vertices[topId]!;
      geom(model).vertices[topId] = { id: top.id, position: [top.position[0], top.position[1], top.position[2] + 2] };
      const after = modelObjectAabb(model, rec)!;
      expect(after.max[2]).toBeGreaterThan(before.max[2] + 1);
    });

    it("vertexDistance matches graph positions", async () => {
      const g = new Model();
      const va = "va" as VertexRef;
      const vb = "vb" as VertexRef;
      g.vertices[va] = { id: va, position: [0, 0, 0] };
      g.vertices[vb] = { id: vb, position: [3, 4, 0] };
      expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
    });

    it("faceArea sums boundary wire triangles", async () => {
      const g = new Model();
      const fid = "f0" as FaceRef;
      const wid = "w0" as WireRef;
      const v0 = "v0" as VertexRef;
      const v1 = "v1" as VertexRef;
      const v2 = "v2" as VertexRef;
      const e0 = "e0" as EdgeRef;
      const e1 = "e1" as EdgeRef;
      const e2 = "e2" as EdgeRef;
      g.vertices[v0] = { id: v0, position: [0, 0, 0] };
      g.vertices[v1] = { id: v1, position: [1, 0, 0] };
      g.vertices[v2] = { id: v2, position: [0, 1, 0] };
      g.edges[e0] = { id: e0, vertexIds: [v0, v1] };
      g.edges[e1] = { id: e1, vertexIds: [v1, v2] };
      g.edges[e2] = { id: e2, vertexIds: [v2, v0] };
      g.wires[wid] = { id: wid, edgeIds: [e0, e1, e2] };
      g.faces[fid] = {
        id: fid,
        wireIds: [wid],
      };
      const a = await kernel.faceArea(fid, g);
      expect(a).toBeCloseTo(0.5, 5);
    });

    it("solidVolume matches volume", async () => {
      const cell = await kernel.createBoxFromCorners({
        cornerA: [0, 0, 0],
        cornerB: [1, 1, 0],
        height: 1,
      });
      expect(await kernel.solidVolume(cell)).toBeCloseTo(await kernel.volume(cell), 6);
    });

    it("syncSolidsFromModel fuses from_geometry hull metadata into one solid volume", async () => {
      const g = new Model();
      applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("west")));
      applyModelDiff(g, boxModelDiff({ cornerA: [3, 0, 0], cornerB: [4, 1, 0], height: 1 }, solidRef("east")));
      g.metadata.setField("from_geometry-hull", "fuseSourceSolidIds", ["west", "east"]);
      g.solids["from_geometry-hull" as SolidRef] = { id: "from_geometry-hull" as SolidRef, shellIds: [] };
      await kernel.syncSolidsFromModel(g);
      expect(await kernel.volume("from_geometry-hull" as SolidRef)).toBeCloseTo(2, 3);
    });

    it("adjacentSolids lists other solids sharing any face", async () => {
      const g = new Model();
      const f = "fs" as FaceRef;
      g.faces[f] = { id: f, wireIds: [] };
      const s0 = "s0" as ShellRef;
      const s1 = "s1" as ShellRef;
      g.shells[s0] = { id: s0, faceIds: [f] };
      g.shells[s1] = { id: s1, faceIds: [f] };
      g.solids["c0" as SolidRef] = { id: "c0" as SolidRef, shellIds: [s0] };
      g.solids["c1" as SolidRef] = { id: "c1" as SolidRef, shellIds: [s1] };
      const adj = await kernel.adjacentSolids("c0" as SolidRef, g);
      expect(adj.map(String).sort()).toEqual(["c1"]);
    });

    it("sharedFacesBetween returns shared face ids", async () => {
      const g = new Model();
      const f = "fx" as FaceRef;
      g.faces[f] = { id: f, wireIds: [] };
      const sa = "sa" as ShellRef;
      const sb = "sb" as ShellRef;
      g.shells[sa] = { id: sa, faceIds: [f] };
      g.shells[sb] = { id: sb, faceIds: [f] };
      g.solids["ca" as SolidRef] = { id: "ca" as SolidRef, shellIds: [sa] };
      g.solids["cb" as SolidRef] = { id: "cb" as SolidRef, shellIds: [sb] };
      const xs = await kernel.sharedFacesBetween("ca" as SolidRef, "cb" as SolidRef, g);
      expect(xs).toEqual([f]);
    });

    it("aabbDifferencePieces volume equals solid minus intersection overlap", () => {
      const cell = { min: [0, 0, 0] as Vec3, max: [2, 2, 2] as Vec3 };
      const other = { min: [1, 1, 0] as Vec3, max: [3, 3, 2] as Vec3 };
      const inter = aabbIntersect(cell, other)!;
      const pieces = aabbDifferencePieces(cell, [inter]);
      const pieceVol = pieces.reduce((acc, p) => acc + aabbVolume(p), 0);
      expect(pieceVol).toBeCloseTo(aabbVolume(cell) - aabbVolume(inter), 4);
    });

    it("executeCommandDiff curve.arc places end vertex on circle not off-circle pick", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [2, 0, 0],
        end: [0, 3, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
    });

    it("executeCommandDiff curve.arc creates one arc edge between start and end", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [2, 0, 0],
        end: [0, 2, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      const edges = res.diff.edges?.added ?? [];
      const wires = res.diff.wires?.added ?? [];
      expect(verts).toHaveLength(2);
      expect(edges).toHaveLength(1);
      expect(wires).toHaveLength(1);
      expect(verts[0]!.position).toEqual([2, 0, 0]);
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
      expect(verts[1]!.position[2]).toBeCloseTo(0, 5);
      expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
      expect(edges[0]!.vertexIds).toHaveLength(2);
    });

    it("executeCommandDiff curve.arc computes end from angle when end is missing", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", {
        center: [0, 0, 0],
        start: [1, 0, 0],
        angle: 90,
      });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts).toHaveLength(2);
      expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
      expect(verts[1]!.position[1]).toBeCloseTo(1, 5);
      expect(res.diff.edges?.added?.[0]?.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
    });

    it("executeCommandDiff curve.circle creates closed circle edge with circle metadata", async () => {
      const res = await kernel.executeCommandDiff("curve.circle", {
        center: [1, 2, 0],
        radiusPoint: [4, 2, 0],
      });
      const verts = res.diff.vertices?.added ?? [];
      const edges = res.diff.edges?.added ?? [];
      expect(verts).toHaveLength(1);
      expect(verts[0]!.position).toEqual([4, 2, 0]);
      expect(edges[0]!.curve).toEqual({ kind: "circle", center: [1, 2, 0], normal: [0, 0, 1], radius: 3 });
      expect(edges[0]!.vertexIds[0]).toBe(edges[0]!.vertexIds[1]);
    });

    it("executeCommandDiff solid.sphere stores SolidPrimitive and brepjs solid", async () => {
      const res = await kernel.executeCommandDiff("solid.sphere", {
        center: [0, 0, 0],
        radius: 2,
      });
      const solids = res.diff.solids?.added ?? [];
      expect(solids[0]!.solid).toEqual({ kind: "sphere", center: [0, 0, 0], radius: 2 });
      const vol = await kernel.volume(solids[0]!.id);
      expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 1);
    });

    it("executeCommandDiff curve.controlPointCurve creates nurbs edge", async () => {
      const res = await kernel.executeCommandDiff("curve.controlPointCurve", {
        points: [
          [0, 0, 0],
          [1, 2, 0],
          [3, 1, 0],
        ],
      });
      const edges = res.diff.edges?.added ?? [];
      expect(edges[0]!.curve?.kind).toBe("nurbs");
      if (edges[0]!.curve?.kind === "nurbs") {
        expect(edges[0]!.curve.poles).toHaveLength(3);
        expect(edges[0]!.curve.through).toBe(false);
      }
    });

    it("worker arg serialization roundtrips nested model in command params", () => {
      const g = new Model();
      const bag = serializeWorkerValue({
        model: g,
        points: [
          [0, 0, 0],
          [2, 1, 0],
        ],
      }) as Record<string, unknown>;
      expect(bag.model).toEqual(expect.objectContaining({ __modelJson: expect.objectContaining({ schema: "spatial.model" }) }));
      const restored = deserializeWorkerValue(bag) as { model: Model; points: readonly Vec3[] };
      expect(restored.model).toBeInstanceOf(Model);
      expect(restored.points).toHaveLength(2);
    });

    it("executeCommandDiff curve.interpolateCurve marks through-points nurbs", async () => {
      const g = new Model();
      const res = await kernel.executeCommandDiff("curve.interpolateCurve", {
        model: g,
        points: [
          [0, 0, 0],
          [2, 1, 0],
          [4, 0, 0],
        ],
      });
      const edges = res.diff.edges?.added ?? [];
      expect(edges[0]!.curve?.kind).toBe("nurbs");
      if (edges[0]!.curve?.kind === "nurbs") {
        expect(edges[0]!.curve.through).toBe(true);
        expect(edges[0]!.curve.poles).toHaveLength(3);
      }
      expect((res.diff.wires?.added ?? []).length).toBe(1);
    });

    it("executeCommandDiff typology constructFrom2PointsAndHeight builds a solid", async () => {
      const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", {
        pointA: [0, 0, 0],
        pointB: [4, 3, 0],
        height: 2.5,
      });
      expect((res.diff.solids?.added ?? []).length).toBeGreaterThanOrEqual(1);
    });

    it("concrete forest left play fixture roundtrips shape, building, energy, and structure models", async () => {
      const { readFile } = await import("node:fs/promises");
      const { resolve } = await import("node:path");
      const fixturePath = resolve(import.meta.dirname, "../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🔣️.json");
      const fixtureJson = JSON.parse(await readFile(fixturePath, "utf8")) as ModelSpaceJson;
      const space = ModelSpace.fromJSON(fixtureJson);
      const shape = space.models[defaultModelDefinitionId()]!;
      const building = space.models[AEC_BUILDING_MODEL_DEFINITION_ID]!;
      const energy = space.models[AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID]!;
      const structure = space.models[AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]!;
      expect(Object.keys(shape.objects)).toHaveLength(1);
      expect(Object.keys(building.objects)).toHaveLength(11);
      expect(Object.keys(energy.objects)).toHaveLength(1);
      expect(Object.keys(structure.objects)).toHaveLength(11);
      expect(Object.keys(geom(shape).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(building).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(energy).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(structure).vertices).length).toBeGreaterThan(0);
      expect(Object.keys(geom(structure).solids)).toHaveLength(0);
      expect(Object.keys(geom(energy).solids)).toHaveLength(0);
    });

  });
}
// #endregion 🧪️Tests
