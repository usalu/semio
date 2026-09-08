/** 🔺️ SemioMeshDiff schema — real mirror of `🦀️.rs`. Collections are id-keyed
 * removed/modified/added triples (`engine::triples::NamedTripleDiff<K,D,T>`). */
import type { SemioMesh, SemioMaterial, SemioTexture, SemioPrimitive, SemioTopology, SemioPoint3, SemioUv, SemioRgba } from "../📸️snapshot/🟦️";

export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[]; }

export interface SemioPrimitiveDiff {
  topology?: SemioTopology;
  positions?: SemioPoint3[];
  normals?: SemioPoint3[];
  uvs?: SemioUv[];
  colors?: SemioRgba[];
  indices?: number[];
  /** tri-state: absent = unchanged, null = cleared, string = set */
  materialId?: string | null;
}

export interface SemioMeshItemDiff {
  primitives?: NamedTripleDiff<string, SemioPrimitiveDiff, SemioPrimitive>;
}

export interface SemioMaterialDiff {
  baseColor?: SemioRgba;
  metallic?: number;
  roughness?: number;
}

export interface SemioTextureDiff {
  mime?: string;
  bytes?: number[];
}

export interface SemioMeshDiff {
  meshes?: NamedTripleDiff<string, SemioMeshItemDiff, SemioMesh>;
  materials?: NamedTripleDiff<string, SemioMaterialDiff, SemioMaterial>;
  textures?: NamedTripleDiff<string, SemioTextureDiff, SemioTexture>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1MeshDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1MeshDiffGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1MeshDiffGuardRefusal(at, why);
};

type stdioSemioV1MeshDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1MeshDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1MeshDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1MeshDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1MeshDiffGuardReject(at, "value is not an object");
export const stdioSemioV1MeshDiffGuardArray = (value: unknown, at: string, bounds: stdioSemioV1MeshDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1MeshDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1MeshDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1MeshDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1MeshDiffGuardString = (value: unknown, at: string, bounds: stdioSemioV1MeshDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1MeshDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1MeshDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1MeshDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1MeshDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1MeshDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1MeshDiffGuardReject(at, "value is not a boolean"));
export const stdioSemioV1MeshDiffGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1MeshDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1MeshDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1MeshDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1MeshDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1MeshDiffGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1MeshDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1MeshDiffGuardNumber(value, at, bounds) : stdioSemioV1MeshDiffGuardReject(at, "value is not an integer");
export const stdioSemioV1MeshDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1MeshDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1MeshDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1MeshDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioMeshDiff(value: unknown, at = "$"): SemioMeshDiff {
  const row = stdioSemioV1MeshDiffGuardObject(value, at);
  return {
    meshes: row["meshes"] === undefined ? undefined : stdioSemioV1MeshDiffGuardObject(row["meshes"], `${at}.meshes`),
    materials: row["materials"] === undefined ? undefined : stdioSemioV1MeshDiffGuardObject(row["materials"], `${at}.materials`),
    textures: row["textures"] === undefined ? undefined : stdioSemioV1MeshDiffGuardObject(row["textures"], `${at}.textures`),
  };
}
