/** 🧊️ The in-memory spatial model: objects, geometry records, attribute store, hydration and primitive hashes. */
import type { EdgeCurve } from "@semio-tech/framework-3d-js";
import { AttributeTable, type SpatialEntityFlagKey, type SpatialEntityFlags } from "../🗺️paths/🟦️.ts";
import { defaultModelDefinitionId, listModelDefinitionManifests } from "../📥️definition-catalog/🟦️.ts";
import { type VertexRef, type EdgeRef, type WireRef, type FaceRef, type ShellRef, type SolidRef, type VertexRecord, type AnchorRecord, type EdgeRecord, type WireRecord, type FaceSurface, type FaceRecord, type ShellRecord, type SolidPrimitive, type SolidRecord, type KernelGeometryJson, type ObjectRef, type TypologyRef, type SpatialObjectRecord, type ModelJson, recordsById, sortedRecordValues, sortedPrimitiveEntries, normalizeSpatialObjectPrimitives, readVec3FromUnknown, type GeometryPrimitiveHash, hashVertexPosition, hashEdgeRecord, hashWireRecord, hashFaceRecord, hashShellRecord, hashSolidRecord, hashAnchorRecord, type ModelPrimitiveHashes, type TypologyPrimitiveKind, type KernelTopologyKind } from "../🟦️.ts";

function defaultBaseObjectTypology(): TypologyRef {
  const manifest = listModelDefinitionManifests().find((row) => row.id === defaultModelDefinitionId());
  return (manifest?.baseObjectTypology ?? "") as TypologyRef;
}

function normalizeSpatialObjectRecord(row: SpatialObjectRecord | Record<string, unknown>): SpatialObjectRecord {
  return {
    id: String(row.id) as ObjectRef,
    typology: String(row.typology ?? defaultBaseObjectTypology()) as TypologyRef,
    primitives: Array.isArray(row.primitives) ? {} : normalizeSpatialObjectPrimitives(row.primitives),
    ...(row.attributes && typeof row.attributes === "object" ? { attributes: row.attributes as Readonly<Record<string, unknown>> } : {}),
  };
}

/** 🧱️ Promotes inline `objects[].primitives[]` rows into kernel geometry tables and slot refs. */
export function materializeInlineObjectPrimitives(model: Model, rawObjects: readonly unknown[] = []): void {
  let changed = false;
  const rawById = new Map<string, Record<string, unknown>>();
  for (const entry of rawObjects) {
    if (!entry || typeof entry !== "object") continue;
    const row = entry as Record<string, unknown>;
    if (typeof row.id === "string") rawById.set(row.id, row);
  }
  for (const [objectId, object] of Object.entries(model.objects)) {
    const raw = rawById.get(objectId)?.primitives;
    if (!Array.isArray(raw)) continue;
    const slots: Record<string, string> = {};
    for (const entry of raw) {
      if (!entry || typeof entry !== "object") continue;
      const row = entry as Record<string, unknown>;
      const kind = typeof row.kind === "string" ? row.kind : "";
      const id = typeof row.id === "string" ? row.id : "";
      if (!kind || !id) continue;
      const slot = typeof row.slot === "string" && row.slot.length > 0 ? row.slot : kind;
      slots[slot] = id;
      if (kind === "vertex") {
        const position = readVec3FromUnknown(row.position);
        if (position) {
          model.vertices[id as VertexRef] = { id: id as VertexRef, position };
          changed = true;
        }
        continue;
      }
      if ((kind === "curve" || kind === "wire") && Array.isArray(row.edgeIds)) {
        model.wires[id as WireRef] = { id: id as WireRef, edgeIds: row.edgeIds.map(String) as EdgeRef[] };
        changed = true;
        continue;
      }
      if ((kind === "curve" || kind === "edge") && Array.isArray(row.vertexIds) && row.vertexIds.length >= 2) {
        model.edges[id as EdgeRef] = {
          id: id as EdgeRef,
          vertexIds: [String(row.vertexIds[0]), String(row.vertexIds[1])] as [VertexRef, VertexRef],
          ...(row.curve && typeof row.curve === "object" ? { curve: row.curve as EdgeCurve } : {}),
        };
        changed = true;
        continue;
      }
      if ((kind === "surface" || kind === "face") && Array.isArray(row.wireIds)) {
        model.faces[id as FaceRef] = {
          id: id as FaceRef,
          wireIds: row.wireIds.map(String) as WireRef[],
          ...(row.surface && typeof row.surface === "object" ? { surface: row.surface as FaceSurface } : {}),
        };
        changed = true;
        continue;
      }
      if (kind === "shell" && Array.isArray(row.faceIds)) {
        model.shells[id as ShellRef] = { id: id as ShellRef, faceIds: row.faceIds.map(String) as FaceRef[] };
        changed = true;
        continue;
      }
      if (kind === "solid") {
        model.solids[id as SolidRef] = {
          id: id as SolidRef,
          shellIds: Array.isArray(row.shellIds) ? (row.shellIds.map(String) as ShellRef[]) : [],
          ...(row.solid && typeof row.solid === "object" ? { solid: row.solid as SolidPrimitive } : {}),
        };
        changed = true;
      }
    }
    model.objects[objectId] = { ...object, primitives: normalizeSpatialObjectPrimitives(slots) };
    changed = true;
  }
  if (changed) model.bump();
}

export function objectPrimitiveEntries(object: SpatialObjectRecord): readonly (readonly [string, string])[] {
  return sortedPrimitiveEntries(object.primitives);
}

export function objectPrimitiveRefs(object: SpatialObjectRecord): readonly string[] {
  return objectPrimitiveEntries(object).map(([, primitiveRef]) => primitiveRef);
}

export function objectPrimaryPrimitiveRef(object: SpatialObjectRecord): string | null {
  return objectPrimitiveEntries(object)[0]?.[1] ?? null;
}

/** 🧱️ Mutable in-memory model: objects + kernel-private geometry + attribute store. */
export class Model {
  revision = 0;
  objects: Record<string, SpatialObjectRecord> = {};
  anchors: Record<string, AnchorRecord> = {};
  vertices: Record<string, VertexRecord> = {};
  edges: Record<string, EdgeRecord> = {};
  wires: Record<string, WireRecord> = {};
  faces: Record<string, FaceRecord> = {};
  shells: Record<string, ShellRecord> = {};
  solids: Record<string, SolidRecord> = {};
  readonly metadata: AttributeTable = new AttributeTable(() => this.bump());

  /** 🧭️ Serializes to `ModelJson` (stable id-sorted arrays). */
  toJSON(): ModelJson {
    const meta = this.metadata.toJSON();
    return {
      schema: "spatial.model",
      revision: this.revision,
      objects: sortedRecordValues(this.objects),
      geometry: {
        anchors: sortedRecordValues(this.anchors),
        vertices: sortedRecordValues(this.vertices),
        edges: sortedRecordValues(this.edges),
        wires: sortedRecordValues(this.wires),
        faces: sortedRecordValues(this.faces),
        shells: sortedRecordValues(this.shells),
        solids: sortedRecordValues(this.solids),
      },
      ...(meta.length > 0 ? { metadata: meta } : {}),
    };
  }

  /** 🧭️ Hydrates from `ModelJson`. */
  static fromJSON(j: ModelJson): Model {
    const g = new Model();
    g.revision = j.revision;
    const rawObjects = j.objects ?? [];
    g.objects = recordsById(rawObjects.map((row) => normalizeSpatialObjectRecord(row as SpatialObjectRecord | Record<string, unknown>)));
    const geo = j.geometry ?? (j as unknown as KernelGeometryJson);
    g.anchors = recordsById(geo.anchors ?? []);
    g.vertices = recordsById(geo.vertices ?? []);
    g.edges = recordsById(geo.edges ?? []);
    g.wires = recordsById(geo.wires ?? []);
    g.faces = recordsById(geo.faces ?? []);
    g.shells = recordsById(geo.shells ?? []);
    g.solids = recordsById(geo.solids ?? []);
    if (j.metadata?.length) g.metadata.loadSnapshot(j.metadata, false);
    materializeInlineObjectPrimitives(g, rawObjects);
    return g;
  }

  bump(): void {
    this.revision += 1;
  }

  /** 👁️ Reads persisted hide/lock flags for an entity id from metadata. */
  getEntityFlags(id: string): SpatialEntityFlags {
    return this.metadata.getEntityFlags(id);
  }

  /** 👁️ Sets one persisted hide/lock flag on an entity id. */
  setEntityFlag(id: string, flag: SpatialEntityFlagKey, value: boolean): void {
    this.metadata.setEntityFlag(id, flag, value);
  }
}

/** #⃣ Maps primitive tables on `model` to content hashes (every vertex and primitive). */
export function hashModelPrimitives(model: Model): ModelPrimitiveHashes {
  const out: Partial<Record<KernelTopologyKind, Record<string, GeometryPrimitiveHash>>> = {};
  const put = (kind: KernelTopologyKind, id: string, hash: GeometryPrimitiveHash): void => {
    out[kind] ??= {};
    out[kind]![id] = hash;
  };
  for (const [id, row] of Object.entries(model.anchors)) put("anchor", id, hashAnchorRecord(row));
  for (const [id, row] of Object.entries(model.vertices)) put("vertex", id, hashVertexPosition(row.position));
  for (const [id, row] of Object.entries(model.edges)) put("edge", id, hashEdgeRecord(row, model.vertices));
  for (const [id, row] of Object.entries(model.wires)) put("wire", id, hashWireRecord(row));
  for (const [id, row] of Object.entries(model.faces)) put("face", id, hashFaceRecord(row));
  for (const [id, row] of Object.entries(model.shells)) put("shell", id, hashShellRecord(row));
  for (const [id, row] of Object.entries(model.solids)) put("solid", id, hashSolidRecord(row));
  return out;
}

/** #⃣ Maps every model vertex id to its position hash. */
export function hashModelVertices(model: Model): Readonly<Record<string, GeometryPrimitiveHash>> {
  return hashModelPrimitives(model).vertex ?? {};
}

/** 🧾️ Parses `spatial.model/v1` JSON into a model or returns `null`. */
export function parseModelJson(raw: unknown): Model | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.model") return null;
  const geoKeys = ["anchors", "vertices", "edges", "wires", "faces", "shells", "solids"] as const;
  const geometry: Record<string, unknown> = r.geometry && typeof r.geometry === "object" ? { ...(r.geometry as Record<string, unknown>) } : {};
  if (!Array.isArray(geometry.solids) && Array.isArray((geometry as { cells?: unknown }).cells)) geometry.solids = (geometry as { cells: unknown[] }).cells;
  for (const k of geoKeys) {
    if (!Array.isArray(geometry[k]) && Array.isArray(r[k])) geometry[k] = r[k];
    if (!Array.isArray(geometry[k])) geometry[k] = [];
  }
  const json: ModelJson = {
    schema: "spatial.model",
    revision: typeof r.revision === "number" ? r.revision : 0,
    objects: Array.isArray(r.objects) ? (r.objects as SpatialObjectRecord[]) : [],
    geometry: {
      anchors: geometry.anchors as AnchorRecord[],
      vertices: geometry.vertices as VertexRecord[],
      edges: geometry.edges as EdgeRecord[],
      wires: geometry.wires as WireRecord[],
      faces: geometry.faces as FaceRecord[],
      shells: geometry.shells as ShellRecord[],
      solids: geometry.solids as SolidRecord[],
    },
  };
  return Model.fromJSON(json);
}

/** 🧭️ Resolves the kernel topology kind referenced by one primitive ref. */
export function resolveKernelTopologyKind(model: Model, primitiveRef: string): KernelTopologyKind | null {
  if (model.anchors[primitiveRef]) return "anchor";
  if (model.vertices[primitiveRef]) return "vertex";
  if (model.edges[primitiveRef]) return "edge";
  if (model.wires[primitiveRef]) return "wire";
  if (model.faces[primitiveRef]) return "face";
  if (model.shells[primitiveRef]) return "shell";
  if (model.solids[primitiveRef]) return "solid";
  return null;
}

/** 🧭️ Resolves the standalone primitive kind referenced by one primitive ref. */
export function resolveStandalonePrimitiveKind(model: Model, primitiveRef: string): TypologyPrimitiveKind | null {
  if (model.anchors[primitiveRef]) return "anchor";
  if (model.solids[primitiveRef]) return "solid";
  if (model.faces[primitiveRef]) return "surface";
  if (model.wires[primitiveRef] || model.edges[primitiveRef]) return "curve";
  return null;
}

/** 🧭️ Resolves the kernel topology kind referenced by one primitive ref. */
export const resolvePrimitiveRefKind = resolveKernelTopologyKind;
