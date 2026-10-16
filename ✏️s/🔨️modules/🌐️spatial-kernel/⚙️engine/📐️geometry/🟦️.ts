// #region 🧲️Header
/** 🌐️ Shared spatial model vocabulary and operations. */
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { kernelGeometry } from "@semio-tech/framework-3d-js";

// #endregion 🧲️Header



// #region 📦️📐️geometry
// #region 🧮️Vec
// #endregion 🧮️Vec

// #region 🧱️kernelGeometry

//#region 📦️📐️geometry
//#region 🧱️Model
// #region 🧱️kernelGeometry

export type AnchorRef = kernelGeometry.AnchorRef;

export type VertexRef = kernelGeometry.VertexRef;

export type EdgeRef = kernelGeometry.EdgeRef;

export type WireRef = kernelGeometry.WireRef;

export type FaceRef = kernelGeometry.FaceRef;

export type ShellRef = kernelGeometry.ShellRef;

export type SolidRef = kernelGeometry.SolidRef;

type GeometryEntityKind = kernelGeometry.GeometryEntityKind;

export type EditableEntityKind = GeometryEntityKind;

/** 🧭️ Framework + brepjs sub-element selection kinds. */
export type ModelEntityKind = EditableEntityKind | "object" | "geometry" | "attribute";
// #endregion 🧱️kernelGeometry

export type VertexRecord = kernelGeometry.VertexRecord;

export type AnchorAttachment = kernelGeometry.AnchorAttachment;

export type AnchorRecord = kernelGeometry.AnchorRecord;

export type EdgeRecord = kernelGeometry.EdgeRecord;

export type WireRecord = kernelGeometry.WireRecord;

export type FaceSurface = kernelGeometry.FaceSurface;

export type FaceRecord = kernelGeometry.FaceRecord;

export type ShellRecord = kernelGeometry.ShellRecord;

export type SolidPrimitive = kernelGeometry.SolidPrimitive;

export type SolidRecord = kernelGeometry.SolidRecord;

export type KernelGeometryJson = kernelGeometry.KernelGeometryJson;

/** 🪪️ Opaque object id in a model. */
export type ObjectRef = string & { readonly __brand: "ObjectRef" };

/** 🪪️ Typology id referenced by objects and extension assets. */
export type TypologyRef = string & { readonly __brand: "TypologyRef" };

/** 📦️ Primitive refs owned by one object row. */
export type SpatialObjectPrimitives = Readonly<Record<string, string>>;

/** 📦️ Object instance row in a model (`typology` + kernel `primitives`). */
export interface SpatialObjectRecord {
  readonly id: ObjectRef;
  readonly typology: TypologyRef;
  readonly primitives: SpatialObjectPrimitives;
  readonly attributes?: Readonly<Record<string, unknown>>;
}

/** 🗺️ Serializable model (`spatial.model/v1`). */
export interface ModelJson {
  readonly schema: "spatial.model";
  readonly revision: number;
  readonly objects: readonly SpatialObjectRecord[];
  readonly geometry: KernelGeometryJson;
  readonly metadata?: readonly { readonly id: string; readonly fields: Readonly<Record<string, unknown>> }[];
}

export function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
  const o: Record<string, T> = {};
  for (const x of xs) o[x.id] = x;
  return o;
}

export function sortedRecordValues<T extends { id: string }>(bucket: Record<string, T>): T[] {
  return Object.keys(bucket)
    .sort()
    .map((k) => bucket[k]!);
}

export function sortedPrimitiveEntries(primitives: Record<string, unknown>): readonly (readonly [string, string])[] {
  return Object.entries(primitives)
    .filter((entry): entry is [string, string] => typeof entry[0] === "string" && typeof entry[1] === "string" && entry[0].length > 0)
    .sort(([left], [right]) => left.localeCompare(right));
}

export function normalizeSpatialObjectPrimitives(primitives: unknown): SpatialObjectPrimitives {
  return primitives && typeof primitives === "object" ? Object.fromEntries(sortedPrimitiveEntries(primitives as Record<string, unknown>)) : {};
}

export function readVec3FromUnknown(value: unknown): Vec3 | null {
  if (!Array.isArray(value) || value.length < 3) return null;
  const x = Number(value[0]);
  const y = Number(value[1]);
  const z = Number(value[2]);
  if (![x, y, z].every((n) => Number.isFinite(n))) return null;
  return [x, y, z];
}

/** #⃣ Stable FNV-1a digest for canonical geometry fingerprints. */
export function fnv1aHex(input: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < input.length; i++) {
    h ^= input.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return (h >>> 0).toString(16).padStart(8, "0");
}

/** #⃣ Opaque content hash for a hashed primitive (vertex position fingerprint). */
export type GeometryPrimitiveHash = string & { readonly __brand: "GeometryPrimitiveHash" };

/** #⃣ Quantizes a coordinate for stable hashing. */
export function quantizeCoord(value: number, decimals = 9): number {
  const factor = 10 ** decimals;
  return Math.round(value * factor) / factor;
}

/** #⃣ Hashes canonical primitive payload bytes. */
export function hashPrimitivePayload(kind: string, payload: string): GeometryPrimitiveHash {
  return `${kind[0]}:${fnv1aHex(payload)}` as GeometryPrimitiveHash;
}

/** #⃣ Hashes a vertex position (`cad/AGENTS.md` primitive hashing). */
export function hashVertexPosition(position: Vec3): GeometryPrimitiveHash {
  const q: Vec3 = [quantizeCoord(position[0]), quantizeCoord(position[1]), quantizeCoord(position[2])];
  return hashPrimitivePayload("vertex", `${q[0]},${q[1]},${q[2]}`);
}

/** #⃣ Hashes an edge by vertex ids and optional curve. */
export function hashEdgeRecord(edge: EdgeRecord, vertices: Record<string, VertexRecord>): GeometryPrimitiveHash {
  const positions = edge.vertexIds.map((vid) => {
    const p = vertices[vid]?.position ?? ([0, 0, 0] as Vec3);
    return p.map((c) => quantizeCoord(c)).join(",");
  });
  const curveKey = edge.curve ? JSON.stringify(edge.curve) : "line";
  return hashPrimitivePayload("edge", `${edge.vertexIds.join(",")}|${curveKey}|${positions.join(";")}`);
}

/** #⃣ Hashes a wire by sorted edge ids. */
export function hashWireRecord(wire: WireRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("wire", [...wire.edgeIds].sort().join(","));
}

/** #⃣ Hashes a face by sorted wire ids and surface kind. */
export function hashFaceRecord(face: FaceRecord): GeometryPrimitiveHash {
  const surfaceKey = face.surface ? JSON.stringify(face.surface) : "none";
  return hashPrimitivePayload("face", `${[...face.wireIds].sort().join(",")}|${surfaceKey}`);
}

/** #⃣ Hashes a shell by sorted face ids. */
export function hashShellRecord(shell: ShellRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("shell", [...shell.faceIds].sort().join(","));
}

/** #⃣ Hashes a solid by sorted shell ids and solid primitive. */
export function hashSolidRecord(solid: SolidRecord): GeometryPrimitiveHash {
  const primitiveKey = solid.solid ? JSON.stringify(solid.solid) : "none";
  return hashPrimitivePayload("solid", `${[...solid.shellIds].sort().join(",")}|${primitiveKey}`);
}

/** #⃣ Hashes an anchor position and attachment. */
export function hashAnchorRecord(anchor: AnchorRecord): GeometryPrimitiveHash {
  return hashPrimitivePayload("anchor", `${anchor.position.map((c) => quantizeCoord(c)).join(",")}|${JSON.stringify(anchor.attachment)}`);
}

/** #⃣ Per-primitive hashes for one model (`ModelSpace` geometry fingerprint). */
export type ModelPrimitiveHashes = Readonly<Partial<Record<KernelTopologyKind, Readonly<Record<string, GeometryPrimitiveHash>>>>>;

/** 🧱️ Standalone primitive kinds allowed on typology objects (`cad/AGENTS.md`). */
export type TypologyPrimitiveKind = "anchor" | "solid" | "surface" | "curve";

/** 🧬️ Kernel-private topology entity kinds (faces, wires, … are not standalone primitives). */
export type KernelTopologyKind = "anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "solid";

// #endregion 🧱️Model

// #endregion 📦️📐️geometry
//#endregion 🧱️kernelGeometry
//#endregion 📦️📐️geometry
