/** 🧬️ Curation diff schema — sparse field delta. */
import type { CurationArtifact } from "../🟦️.ts";

export type SortDirection = "asc" | "desc";

export interface TableSort {
  columnId: string;
  direction: SortDirection;
}

export interface Filters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: TableSort | null;
}

export type GeometryRecipe =
  | { kind: "box"; width: number; height: number; depth: number }
  | { kind: "frame"; width: number; height: number; depth: number; profile: number }
  | { kind: "slab"; width: number; depth: number; thickness: number }
  | { kind: "mesh"; positions: number[]; normals: number[]; indices: number[] };

export interface ObjectKind {
  id: string;
  name: string;
  moduleId: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

export interface CuratedItem {
  objectId: string;
  count: number;
}

export interface CurationStringList {
  values: string[];
}

/** 🧩️ Sourcing-owned overflow half of `ObjectKind` not representable in the composed
 * `s.stdio.semio.kit` subset's `SemioKitType` (id/name/category only). */
export interface ObjectKindExtra {
  id: string;
  name: string;
  moduleId: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

export interface CurationObjectKindExtraPatchEntry {
  id: string;
  extra: ObjectKindExtra;
}

export interface CurationStockExtraDelta {
  added?: ObjectKindExtra[];
  removed?: string[];
  patched?: CurationObjectKindExtraPatchEntry[];
  reordered?: string[];
}

export interface CurationCuratedPatchEntry {
  objectId: string;
  count?: number;
}

export interface CurationCuratedDelta {
  added?: CuratedItem[];
  removed?: string[];
  patched?: CurationCuratedPatchEntry[];
  reordered?: string[];
}

export interface CurationDiff {
  /** @state artifact */
  artifact?: CurationArtifact | null;
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog?: ArtifactChildHandle | null;
  /** @state artifact */
  stockExtra?: CurationStockExtraDelta | null;
  /** @state artifact */
  curated?: CurationCuratedDelta | null;
  /** @state config */
  filters?: Filters | null;
  /** @state config */
  /** @state config */
  contributionsJson?: string | null;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sourcingCurationDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sourcingCurationDiffGuardReject = (at: string, why: string): never => {
  throw new sourcingCurationDiffGuardRefusal(at, why);
};

type sourcingCurationDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sourcingCurationDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sourcingCurationDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sourcingCurationDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sourcingCurationDiffGuardReject(at, "value is not an object");
export const sourcingCurationDiffGuardArray = (value: unknown, at: string, bounds: sourcingCurationDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sourcingCurationDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sourcingCurationDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sourcingCurationDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sourcingCurationDiffGuardString = (value: unknown, at: string, bounds: sourcingCurationDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sourcingCurationDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sourcingCurationDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sourcingCurationDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sourcingCurationDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sourcingCurationDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sourcingCurationDiffGuardReject(at, "value is not a boolean"));
export const sourcingCurationDiffGuardNumber = (value: unknown, at: string, bounds: sourcingCurationDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sourcingCurationDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sourcingCurationDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sourcingCurationDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sourcingCurationDiffGuardInteger = (value: unknown, at: string, bounds: sourcingCurationDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sourcingCurationDiffGuardNumber(value, at, bounds) : sourcingCurationDiffGuardReject(at, "value is not an integer");
export const sourcingCurationDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sourcingCurationDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sourcingCurationDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sourcingCurationDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCurationStringList(value: unknown, at = "$"): CurationStringList {
  const row = sourcingCurationDiffGuardObject(value, at);
  return {
    values: sourcingCurationDiffGuardArray(row["values"], `${at}.values`).map((item, index) => sourcingCurationDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseCurationCuratedPatchEntry(value: unknown, at = "$"): CurationCuratedPatchEntry {
  const row = sourcingCurationDiffGuardObject(value, at);
  return {
    objectId: sourcingCurationDiffGuardString(row["objectId"], `${at}.objectId`),
    count: row["count"] === undefined ? undefined : sourcingCurationDiffGuardInteger(row["count"], `${at}.count`),
  };
}
