/** 🧬️ Puzzle5d snapshot schema — artifact-lane fields only. */

/** 🪪️ Composed-child handle — mirrors stdio's `s.stdio.semio.kit` cross-language convention. */
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

export interface Puzzle5dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  domain: string;
  /** @state artifact */
  label?: string;
  /** @state artifact */
  meta: Puzzle5dMeta;
  /** @state artifact @child kind=s.stdio.semio */
  kindCatalogs?: ArtifactChildHandle;
  /** @state artifact */
  kindCatalogsExtra?: Puzzle5dKindCatalogsExtra;
  /** @state artifact */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state artifact */
  parts: Puzzle5dPart[];
  /** @state artifact */
  fasteners: Puzzle5dFastener[];
}



/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🏷️ Part-kind attribute. */
export interface Puzzle5dAttribute {
  id?: string;
  key?: string;
  value?: string;
  definition?: string;
}

/** ✍️ Part-kind author. */
export interface Puzzle5dAuthor {
  id?: string;
  name?: string;
  email?: string;
  role?: string;
  rank?: number;
}

/** 🖼️ Part-kind representation. */
export interface Puzzle5dRepresentation {
  id?: string;
  name?: string;
  url?: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

/** 🌱️ Grip template on a part-kind. */
export interface Puzzle5dGripTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  gripKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

/** 🧱️ Part-kind catalog row. */
export interface Puzzle5dCatalogPartKind {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🔘️ Grip-kind catalog row. */
export interface Puzzle5dCatalogGripKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🔗️ Fastener-kind catalog row. */
export interface Puzzle5dCatalogFastenerKind {
  id: string;
  name?: string;
  label?: string;
}

/** 🧵️ Rope-kind catalog row. */
export interface Puzzle5dCatalogRopeKind {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Kind catalogs bundle — still the `replace-kind-catalogs` mutation payload shape; the snapshot
 * itself now carries the composed `kindCatalogs`/`kindCatalogsExtra` pair below instead. */
export interface Puzzle5dKindCatalogs {
  parts?: Puzzle5dCatalogPartKind[];
  grips?: Puzzle5dCatalogGripKind[];
  fasteners?: Puzzle5dCatalogFastenerKind[];
  ropes?: Puzzle5dCatalogRopeKind[];
}

/** 🧩️ Puzzle5d-owned overflow for one part-kind row — everything the composed `SemioKitType`
 * (`id`/`name`/`category`) cannot represent. */
export interface Puzzle5dCatalogPartKindExtra {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🧩️ Puzzle5d-owned overflow for one grip-kind row. */
export interface Puzzle5dCatalogGripKindExtra {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🧩️ Puzzle5d-owned overflow for one fastener-kind row. */
export interface Puzzle5dCatalogFastenerKindExtra {
  id: string;
  name?: string;
  label?: string;
}

/** 🧩️ Puzzle5d-owned overflow for one rope-kind row. */
export interface Puzzle5dCatalogRopeKindExtra {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Puzzle5d-owned overflow half of the kind-catalogs bundle, sibling to the composed
 * `kindCatalogs` child. */
export interface Puzzle5dKindCatalogsExtra {
  parts?: Puzzle5dCatalogPartKindExtra[];
  grips?: Puzzle5dCatalogGripKindExtra[];
  fasteners?: Puzzle5dCatalogFastenerKindExtra[];
  ropes?: Puzzle5dCatalogRopeKindExtra[];
}

/** 🔗️ Kind compatibility row. */
export interface Puzzle5dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle5dCompatSpecificity;
}

/** 📝️ Meta. */
export interface Puzzle5dMeta {
  description?: string;
}

/** 🧱️ Part. */
export interface Puzzle5dPart {
  id: string;
  partKind?: string;
  anchor?: Puzzle5dPartAnchor;
  "2d"?: Record<string, unknown>;
  "3d"?: Record<string, unknown>;
  grips?: Record<string, unknown>[];
}

/** 🔗️ Fastener with eight transform params. */
export interface Puzzle5dFastener {
  id: string;
  source: string;
  target: string;
  fastenerKind?: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dSnapshotGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dSnapshotGuardRefusal(at, why);
};

type puzzlePuzzle5dSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dSnapshotGuardReject(at, "value is not an object");
export const puzzlePuzzle5dSnapshotGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dSnapshotGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dSnapshotGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dSnapshotGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dSnapshotGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dSnapshotGuardNumber(value, at, bounds) : puzzlePuzzle5dSnapshotGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseArtifactChildHandle(value: unknown, at = "$"): ArtifactChildHandle {
  const row = puzzlePuzzle5dSnapshotGuardObject(value, at);
  return {
    childId: puzzlePuzzle5dSnapshotGuardString(row["childId"], `${at}.childId`),
    target: puzzlePuzzle5dSnapshotGuardString(row["target"], `${at}.target`),
  };
}

export function parsePuzzle5dKindCatalogsExtra(value: unknown, at = "$"): Puzzle5dKindCatalogsExtra {
  const row = puzzlePuzzle5dSnapshotGuardObject(value, at);
  return {
    parts: row["parts"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardArray(row["parts"], `${at}.parts`).map((item, index) => parsePuzzle5dCatalogPartKindExtra(item, `${at}.parts[${index}]`)),
    grips: row["grips"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardArray(row["grips"], `${at}.grips`).map((item, index) => parsePuzzle5dCatalogGripKindExtra(item, `${at}.grips[${index}]`)),
    fasteners: row["fasteners"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardArray(row["fasteners"], `${at}.fasteners`).map((item, index) => parsePuzzle5dCatalogFastenerKindExtra(item, `${at}.fasteners[${index}]`)),
    ropes: row["ropes"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardArray(row["ropes"], `${at}.ropes`).map((item, index) => parsePuzzle5dCatalogRopeKindExtra(item, `${at}.ropes[${index}]`)),
  };
}

export function parsePuzzle5dCatalogGripKindExtra(value: unknown, at = "$"): Puzzle5dCatalogGripKindExtra {
  const row = puzzlePuzzle5dSnapshotGuardObject(value, at);
  return {
    id: puzzlePuzzle5dSnapshotGuardString(row["id"], `${at}.id`),
    code: row["code"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["code"], `${at}.code`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["label"], `${at}.label`),
    order: row["order"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardInteger(row["order"], `${at}.order`),
    compatibleWith: row["compatibleWith"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardArray(row["compatibleWith"], `${at}.compatibleWith`).map((item, index) => puzzlePuzzle5dSnapshotGuardString(item, `${at}.compatibleWith[${index}]`)),
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["description"], `${at}.description`),
    icon: row["icon"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["icon"], `${at}.icon`),
    color: row["color"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["color"], `${at}.color`),
    defaultRopeKind: row["defaultRopeKind"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["defaultRopeKind"], `${at}.defaultRopeKind`),
  };
}

export function parsePuzzle5dCatalogFastenerKindExtra(value: unknown, at = "$"): Puzzle5dCatalogFastenerKindExtra {
  const row = puzzlePuzzle5dSnapshotGuardObject(value, at);
  return {
    id: puzzlePuzzle5dSnapshotGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["label"], `${at}.label`),
  };
}

export function parsePuzzle5dCatalogRopeKindExtra(value: unknown, at = "$"): Puzzle5dCatalogRopeKindExtra {
  const row = puzzlePuzzle5dSnapshotGuardObject(value, at);
  return {
    id: puzzlePuzzle5dSnapshotGuardString(row["id"], `${at}.id`),
    name: row["name"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["name"], `${at}.name`),
    label: row["label"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["label"], `${at}.label`),
    defaultFastenerKind: row["defaultFastenerKind"] === undefined ? undefined : puzzlePuzzle5dSnapshotGuardString(row["defaultFastenerKind"], `${at}.defaultFastenerKind`),
  };
}
