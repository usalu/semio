/** 🧊️ The solid topology of the brepjs kernel: extrusion distance, solids from model topology, the STEP vertex quantisation, wire edge order and the inline model space fixture.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { healSolid, getHashCode, isOk, verticesOfEdge, sewShells, solidFromShell, type OwnedBrepEdge as Edge, type OwnedBrepFace as Face, type OwnedBrepSolid as ValidSolid } from "@semio-tech/cad-engine-ts/brepjs";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type Vec3 } from "@semio-tech/framework-3d-js";
import { geom, type EdgeRef, type FaceRecord, type SolidPrimitive, type SolidRecord, type VertexRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { readVec3 } from "@semio-tech/s-spatial-kernel-ts/preview/preview-params";
import { vec3Dot, vec3Normalize, vec3Sub } from "@semio-tech/s-spatial-kernel-ts/preview/vec3";
import { modelFaceIdsForSolid } from "../🧱️entity-maps/🟦️.ts";
import { geomWireToOrientedFaceLoose } from "../🔗️wires-and-selection/🟦️.ts";

// #region 🔌️BrepModelBridge
export function extrusionDistanceFromParams(params: Record<string, unknown>): number {
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

/** ✅️ True when `cell` references at least one face through its shell graph. */
function solidRecordHasShellTopology(model: Model, cell: SolidRecord): boolean {
  return modelFaceIdsForSolid(model, cell).length > 0;
}

/** 🧊️ Builds one brep face from the model face's outer wire (live vertex positions). */
function brepFaceFromModelFaceRecord(model: Model, faceRec: FaceRecord): Face | null {
  const wireId = faceRec.wireIds[0];
  if (!wireId) return null;
  const oriented = geomWireToOrientedFaceLoose(model, wireId);
  return oriented;
}

/** 🧊️ Builds a brepjs `ValidSolid` by sewing closed shell faces from model topology. */
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

/** 🧊️ Brep for records with shell topology or analytic `SolidPrimitive` when no shell graph exists. */
export function deriveValidSolidFromRecordOrPrimitive(model: Model, cell: SolidRecord, primitiveFrom: (p: SolidPrimitive) => ValidSolid): ValidSolid | null {
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

/** 🗺️ `spatial.modelspace/v1` JSON with one object's primitives inlined as raw `materializeInlineObjectPrimitives` rows (pre-normalization). */
export interface InlineModelSpaceFixtureJson {
  readonly schema: "spatial.modelspace";
  readonly revision: number;
  readonly models: readonly { readonly id: string; readonly model: { readonly schema: "spatial.model"; readonly revision: number; readonly objects: readonly { readonly id: string; readonly typology: string; readonly primitives: readonly unknown[] }[] } }[];
}

/** 🧾️ Serializes one object and its solid closure as inline `spatial.modelspace/v1` fixture JSON. */
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
