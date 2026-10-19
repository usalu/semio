/** 🧱️ The brep entity maps of the brepjs kernel: model face and edge ids, normal alignment, face and edge infos, and the mesh transfer with its cache.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { curveLength, getCurveType, getEdges, getFaces, getHashCode, getSurfaceType, isOk, measureArea, mesh, meshEdges, normalAt, toGroupedBufferGeometryData, toLineGeometryData, type OwnedBrepEdge as Edge, type OwnedBrepFace as Face, type OwnedBrepSolid as ValidSolid } from "@semio-tech/cad-engine-ts/brepjs";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type EdgeInfo, type FaceInfo, type MeshTransfer, type Vec3 } from "@semio-tech/framework-3d-js";
import { derivedFacePoints } from "@semio-tech/s-spatial-kernel-ts/preview/face-geometry";
import { faceNormalFromPoints } from "@semio-tech/s-spatial-kernel-ts/preview/anchor-placement";
import { geom, type EdgeRef, type FaceRef, type SolidRecord, type SolidRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { vec3Dot, vec3Normalize } from "@semio-tech/s-spatial-kernel-ts/preview/vec3";
import { geomEdgeToBrepEdge, geomWireToOrientedFace } from "../🔗️wires-and-selection/🟦️.ts";

// #region ♻️BrepjsScratch
// #region 🗺️BrepEntityMaps
type BrepEntityMaps = { readonly faceByHash: ReadonlyMap<number, FaceRef>; readonly edgeByHash: ReadonlyMap<number, EdgeRef> };

export function modelFaceIdsForSolid(model: Model, solid: SolidRecord): readonly FaceRef[] {
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

/** 🗺️ Maps brepjs `getHashCode` handles to spatial `FaceRef`/`EdgeRef`; brepjs has no geometry userData. */
function buildBrepEntityMaps(brepSolid: ValidSolid, context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord }): BrepEntityMaps {
  const faceByHash = new Map<number, FaceRef>();
  const edgeByHash = new Map<number, EdgeRef>();
  let brepFaces: Face[] = [];
  let brepEdges: Edge[] = [];
  try {
    brepFaces = getFaces(brepSolid);
  } catch {
  }
  try {
    brepEdges = getEdges(brepSolid);
  } catch {
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
    }
  }
  return infos;
}

/** 🖼️ Tessellates a solid to grouped buffers + B-Rep edge polylines (caller owns solid lifetime). */
export function meshTransferFromBrep(brepSolid: ValidSolid, tolerance: number, context: { readonly solidRef: SolidRef; readonly model?: Model; readonly solidRecord?: SolidRecord }, collectInspection = true): MeshTransfer {
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
export function meshTransferTransferables(mesh: MeshTransfer): Transferable[] {
  return [mesh.position.buffer, mesh.normal.buffer, mesh.index.buffer, mesh.edges.buffer];
}

export function cloneMeshTransfer(mesh: MeshTransfer): MeshTransfer {
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
