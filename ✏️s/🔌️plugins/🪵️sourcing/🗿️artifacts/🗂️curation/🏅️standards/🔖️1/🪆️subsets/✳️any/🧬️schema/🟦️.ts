/** 🧬️ Curation artifact schema — every field with its state class. */

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

export interface CuratedItem {
  objectId: string;
  count: number;
}

export interface CurationArtifact {
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog: ArtifactChildHandle;
  /** @state artifact */
  stockExtra: ObjectKindExtra[];
  /** @state artifact */
  curated: CuratedItem[];
  /** @state config */
  filters: Filters;
  /** @state config */
  /** @state config */
  contributionsJson: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sourcingCurationArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sourcingCurationArtifactGuardReject = (at: string, why: string): never => {
  throw new sourcingCurationArtifactGuardRefusal(at, why);
};

type sourcingCurationArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sourcingCurationArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sourcingCurationArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sourcingCurationArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sourcingCurationArtifactGuardReject(at, "value is not an object");
export const sourcingCurationArtifactGuardArray = (value: unknown, at: string, bounds: sourcingCurationArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sourcingCurationArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sourcingCurationArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sourcingCurationArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sourcingCurationArtifactGuardString = (value: unknown, at: string, bounds: sourcingCurationArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sourcingCurationArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sourcingCurationArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sourcingCurationArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sourcingCurationArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sourcingCurationArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sourcingCurationArtifactGuardReject(at, "value is not a boolean"));
export const sourcingCurationArtifactGuardNumber = (value: unknown, at: string, bounds: sourcingCurationArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sourcingCurationArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sourcingCurationArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sourcingCurationArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sourcingCurationArtifactGuardInteger = (value: unknown, at: string, bounds: sourcingCurationArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sourcingCurationArtifactGuardNumber(value, at, bounds) : sourcingCurationArtifactGuardReject(at, "value is not an integer");
export const sourcingCurationArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sourcingCurationArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sourcingCurationArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sourcingCurationArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCurationArtifact(value: unknown, at = "$"): CurationArtifact {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    catalog: sourcingCurationArtifactGuardObject(row["catalog"], `${at}.catalog`),
    stockExtra: sourcingCurationArtifactGuardArray(row["stockExtra"], `${at}.stockExtra`).map((item, index) => parseObjectKindExtra(item, `${at}.stockExtra[${index}]`)),
    curated: sourcingCurationArtifactGuardArray(row["curated"], `${at}.curated`).map((item, index) => parseCuratedItem(item, `${at}.curated[${index}]`)),
    filters: parseFilters(row["filters"], `${at}.filters`),
    contributionsJson: sourcingCurationArtifactGuardString(row["contributionsJson"], `${at}.contributionsJson`),
  };
}

export function parseObjectKind(value: unknown, at = "$"): ObjectKind {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    id: sourcingCurationArtifactGuardString(row["id"], `${at}.id`),
    name: sourcingCurationArtifactGuardString(row["name"], `${at}.name`),
    moduleId: sourcingCurationArtifactGuardString(row["moduleId"], `${at}.moduleId`),
    typologyPath: sourcingCurationArtifactGuardArray(row["typologyPath"], `${at}.typologyPath`).map((item, index) => sourcingCurationArtifactGuardString(item, `${at}.typologyPath[${index}]`)),
    availability: sourcingCurationArtifactGuardInteger(row["availability"], `${at}.availability`),
    geometry: parseGeometryRecipe(row["geometry"], `${at}.geometry`),
  };
}

export function parseObjectKindExtra(value: unknown, at = "$"): ObjectKindExtra {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    id: sourcingCurationArtifactGuardString(row["id"], `${at}.id`),
    name: sourcingCurationArtifactGuardString(row["name"], `${at}.name`),
    moduleId: sourcingCurationArtifactGuardString(row["moduleId"], `${at}.moduleId`),
    typologyPath: sourcingCurationArtifactGuardArray(row["typologyPath"], `${at}.typologyPath`).map((item, index) => sourcingCurationArtifactGuardString(item, `${at}.typologyPath[${index}]`)),
    availability: sourcingCurationArtifactGuardInteger(row["availability"], `${at}.availability`),
    geometry: parseGeometryRecipe(row["geometry"], `${at}.geometry`),
  };
}

export function parseCuratedItem(value: unknown, at = "$"): CuratedItem {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    objectId: sourcingCurationArtifactGuardString(row["objectId"], `${at}.objectId`),
    count: sourcingCurationArtifactGuardInteger(row["count"], `${at}.count`),
  };
}

export function parseSortDirection(value: unknown, at = "$"): SortDirection {
  return sourcingCurationArtifactGuardMember(value, `${at}`, ["asc", "desc"] as const);
}

export function parseTableSort(value: unknown, at = "$"): TableSort {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    columnId: sourcingCurationArtifactGuardString(row["columnId"], `${at}.columnId`),
    direction: parseSortDirection(row["direction"], `${at}.direction`),
  };
}

export function parseFilters(value: unknown, at = "$"): Filters {
  const row = sourcingCurationArtifactGuardObject(value, at);
  return {
    query: sourcingCurationArtifactGuardString(row["query"], `${at}.query`),
    moduleIds: sourcingCurationArtifactGuardArray(row["moduleIds"], `${at}.moduleIds`).map((item, index) => sourcingCurationArtifactGuardString(item, `${at}.moduleIds[${index}]`)),
    typologyPath: sourcingCurationArtifactGuardArray(row["typologyPath"], `${at}.typologyPath`).map((item, index) => sourcingCurationArtifactGuardString(item, `${at}.typologyPath[${index}]`)),
    minAvailability: sourcingCurationArtifactGuardInteger(row["minAvailability"], `${at}.minAvailability`),
    sort: row["sort"] === undefined ? undefined : parseTableSort(row["sort"], `${at}.sort`),
  };
}
