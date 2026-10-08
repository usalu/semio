/** 🧬️ Puzzle3d diff schema — sparse typed delta: per-field entity patches and id-keyed collection deltas. */
import {
  parsePuzzle3dAttraction,
  parsePuzzle3dCompatSpecificity,
  parsePuzzle3dKindCatalogs,
  parsePuzzle3dKindCompatibility,
  parsePuzzle3dMeta,
  parsePuzzle3dObject,
  parsePuzzle3dObjectAnchor,
  parsePuzzle3dReference,
  parsePuzzle3dReferenceSource,
  parsePuzzle3dScale,
  parsePuzzle3dTargetVolume,
  parsePuzzle3dVortex,
  type Puzzle3dAttraction,
  type Puzzle3dCompatSpecificity,
  type Puzzle3dKindCatalogs,
  type Puzzle3dKindCompatibility,
  type Puzzle3dMeta,
  type Puzzle3dObject,
  type Puzzle3dObjectAnchor,
  type Puzzle3dReference,
  type Puzzle3dReferenceSource,
  type Puzzle3dScale,
  type Puzzle3dTargetVolume,
  type Puzzle3dVortex,
  type Puzzle3dArtifact,
} from "../🟦️.ts";

export interface Puzzle3dDiff {
  /** @state artifact */
  artifact?: Puzzle3dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  meta?: Puzzle3dMetaPatch;
  /** @state artifact */
  objects?: Puzzle3dObjectsDelta;
  /** @state artifact */
  attractions?: Puzzle3dAttractionsDelta;
  /** @state artifact */
  targetVolumes?: Puzzle3dTargetVolumesDelta;
  /** @state artifact */
  references?: Puzzle3dReferencesDelta;
}

export interface Puzzle3dKindCompatibilityKey { source: string; target: string; }
export interface Puzzle3dVortexPatch {
  vortexKind?: string | null;
  label?: string | null;
  position?: number[];
  direction?: number[] | null;
  radius?: number | null;
  hidden?: boolean;
  locked?: boolean;
}
export interface Puzzle3dObjectPatch {
  label?: string | null;
  objectKind?: string | null;
  anchor?: Puzzle3dObjectAnchor;
  origin?: number[];
  orientation?: number[] | null;
  scale?: Puzzle3dScale | null;
  meshUrl?: string | null;
  vortices?: Puzzle3dVorticesDelta;
  hidden?: boolean;
  locked?: boolean;
}
export interface Puzzle3dAttractionPatch {
  attracting?: string;
  attracted?: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}
export interface Puzzle3dTargetVolumePatch {
  origin?: number[];
  orientation?: number[] | null;
  scale?: Puzzle3dScale | null;
  hidden?: boolean;
  locked?: boolean;
}
export interface Puzzle3dReferencePatch {
  source?: Puzzle3dReferenceSource;
  origin?: number[];
  widthWorld?: number;
  locked?: boolean;
  hidden?: boolean;
}
export interface Puzzle3dKindCompatibilityPatch {
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle3dCompatSpecificity;
}
export interface Puzzle3dMetaPatch {
  kindCatalogs?: Puzzle3dKindCatalogs | null;
  kindCompatibility?: Puzzle3dKindCompatibilityDelta;
}
export interface Puzzle3dVorticesDelta { removed: Puzzle3dVortexRemoval[]; inserted: Puzzle3dVortexInsertion[]; moved: Puzzle3dVortexRelocation[]; modified: Puzzle3dVortexModification[]; }
export interface Puzzle3dVortexRemoval { id: string; index: number; }
export interface Puzzle3dVortexInsertion { index: number; row: Puzzle3dVortex; }
export interface Puzzle3dVortexRelocation { id: string; from: number; to: number; }
export interface Puzzle3dVortexModification { id: string; patch: Puzzle3dVortexPatch; }
export interface Puzzle3dObjectsDelta { removed: Puzzle3dObjectRemoval[]; inserted: Puzzle3dObjectInsertion[]; moved: Puzzle3dObjectRelocation[]; modified: Puzzle3dObjectModification[]; }
export interface Puzzle3dObjectRemoval { id: string; index: number; }
export interface Puzzle3dObjectInsertion { index: number; row: Puzzle3dObject; }
export interface Puzzle3dObjectRelocation { id: string; from: number; to: number; }
export interface Puzzle3dObjectModification { id: string; patch: Puzzle3dObjectPatch; }
export interface Puzzle3dAttractionsDelta { removed: Puzzle3dAttractionRemoval[]; inserted: Puzzle3dAttractionInsertion[]; moved: Puzzle3dAttractionRelocation[]; modified: Puzzle3dAttractionModification[]; }
export interface Puzzle3dAttractionRemoval { id: string; index: number; }
export interface Puzzle3dAttractionInsertion { index: number; row: Puzzle3dAttraction; }
export interface Puzzle3dAttractionRelocation { id: string; from: number; to: number; }
export interface Puzzle3dAttractionModification { id: string; patch: Puzzle3dAttractionPatch; }
export interface Puzzle3dTargetVolumesDelta { removed: Puzzle3dTargetVolumeRemoval[]; inserted: Puzzle3dTargetVolumeInsertion[]; moved: Puzzle3dTargetVolumeRelocation[]; modified: Puzzle3dTargetVolumeModification[]; }
export interface Puzzle3dTargetVolumeRemoval { id: string; index: number; }
export interface Puzzle3dTargetVolumeInsertion { index: number; row: Puzzle3dTargetVolume; }
export interface Puzzle3dTargetVolumeRelocation { id: string; from: number; to: number; }
export interface Puzzle3dTargetVolumeModification { id: string; patch: Puzzle3dTargetVolumePatch; }
export interface Puzzle3dReferencesDelta { removed: Puzzle3dReferenceRemoval[]; inserted: Puzzle3dReferenceInsertion[]; moved: Puzzle3dReferenceRelocation[]; modified: Puzzle3dReferenceModification[]; }
export interface Puzzle3dReferenceRemoval { id: string; index: number; }
export interface Puzzle3dReferenceInsertion { index: number; row: Puzzle3dReference; }
export interface Puzzle3dReferenceRelocation { id: string; from: number; to: number; }
export interface Puzzle3dReferenceModification { id: string; patch: Puzzle3dReferencePatch; }
export interface Puzzle3dKindCompatibilityDelta { removed: Puzzle3dKindCompatibilityRemoval[]; inserted: Puzzle3dKindCompatibilityInsertion[]; moved: Puzzle3dKindCompatibilityRelocation[]; modified: Puzzle3dKindCompatibilityModification[]; }
export interface Puzzle3dKindCompatibilityRemoval { id: Puzzle3dKindCompatibilityKey; index: number; }
export interface Puzzle3dKindCompatibilityInsertion { index: number; row: Puzzle3dKindCompatibility; }
export interface Puzzle3dKindCompatibilityRelocation { id: Puzzle3dKindCompatibilityKey; from: number; to: number; }
export interface Puzzle3dKindCompatibilityModification { id: Puzzle3dKindCompatibilityKey; patch: Puzzle3dKindCompatibilityPatch; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle3dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle3dDiffGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle3dDiffGuardRefusal(at, why);
};

type puzzlePuzzle3dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle3dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle3dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle3dDiffGuardObject = (value: unknown, at: string, keys?: readonly string[]): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return puzzlePuzzle3dDiffGuardReject(at, "value is not an object");
  if (keys && Object.keys(value).some((key) => !keys.includes(key))) return puzzlePuzzle3dDiffGuardReject(at, "unknown field");
  return value as Record<string, unknown>;
};
export const puzzlePuzzle3dDiffGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle3dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle3dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle3dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle3dDiffGuardString = (value: unknown, at: string, bounds: puzzlePuzzle3dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle3dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle3dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle3dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle3dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle3dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle3dDiffGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle3dDiffGuardBinary64 = (value: unknown, at: string): Binary64 => {
  try { return parseBinary64(value); } catch { return puzzlePuzzle3dDiffGuardReject(at, "invalid binary64 word"); }
};
export const puzzlePuzzle3dDiffGuardInteger = (value: unknown, at: string): number =>
  typeof value === "number" && Number.isSafeInteger(value) ? value : puzzlePuzzle3dDiffGuardReject(at, "value is not an integer");
export const puzzlePuzzle3dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle3dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle3dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle3dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle3dKindCompatibilityKey(value: unknown, at = "$"): Puzzle3dKindCompatibilityKey {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    source: puzzlePuzzle3dDiffGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle3dDiffGuardString(row["target"], `${at}.target`),
  };
}

export function parsePuzzle3dVortexPatch(value: unknown, at = "$"): Puzzle3dVortexPatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    vortexKind: row["vortexKind"] === undefined ? undefined : row["vortexKind"] === null ? null : puzzlePuzzle3dDiffGuardString(row["vortexKind"], `${at}.vortexKind`),
    label: row["label"] === undefined ? undefined : row["label"] === null ? null : puzzlePuzzle3dDiffGuardString(row["label"], `${at}.label`),
    position: row["position"] === undefined ? undefined : puzzlePuzzle3dDiffGuardArray(row["position"], `${at}.position`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.position}[${index}]`)),
    direction: row["direction"] === undefined ? undefined : row["direction"] === null ? null : puzzlePuzzle3dDiffGuardArray(row["direction"], `${at}.direction`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.direction}[${index}]`)),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle3dDiffGuardNumber(row["radius"], `${at}.radius`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle3dObjectPatch(value: unknown, at = "$"): Puzzle3dObjectPatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    label: row["label"] === undefined ? undefined : row["label"] === null ? null : puzzlePuzzle3dDiffGuardString(row["label"], `${at}.label`),
    objectKind: row["objectKind"] === undefined ? undefined : row["objectKind"] === null ? null : puzzlePuzzle3dDiffGuardString(row["objectKind"], `${at}.objectKind`),
    anchor: row["anchor"] === undefined ? undefined : parsePuzzle3dObjectAnchor(row["anchor"], `${at}.anchor`),
    origin: row["origin"] === undefined ? undefined : puzzlePuzzle3dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.origin}[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle3dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.orientation}[${index}]`)),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : parsePuzzle3dScale(row["scale"], `${at}.scale`),
    meshUrl: row["meshUrl"] === undefined ? undefined : row["meshUrl"] === null ? null : puzzlePuzzle3dDiffGuardString(row["meshUrl"], `${at}.meshUrl`),
    vortices: row["vortices"] === undefined ? undefined : parsePuzzle3dVorticesDelta(row["vortices"], `${at}.vortices`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle3dAttractionPatch(value: unknown, at = "$"): Puzzle3dAttractionPatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    attracting: row["attracting"] === undefined ? undefined : puzzlePuzzle3dDiffGuardString(row["attracting"], `${at}.attracting`),
    attracted: row["attracted"] === undefined ? undefined : puzzlePuzzle3dDiffGuardString(row["attracted"], `${at}.attracted`),
    gap: row["gap"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["gap"], `${at}.gap`),
    shift: row["shift"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["shift"], `${at}.shift`),
    rise: row["rise"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["rise"], `${at}.rise`),
    rotation: row["rotation"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["rotation"], `${at}.rotation`),
    turn: row["turn"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["turn"], `${at}.turn`),
    tilt: row["tilt"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["tilt"], `${at}.tilt`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePuzzle3dTargetVolumePatch(value: unknown, at = "$"): Puzzle3dTargetVolumePatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    origin: row["origin"] === undefined ? undefined : puzzlePuzzle3dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.origin}[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle3dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.orientation}[${index}]`)),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : parsePuzzle3dScale(row["scale"], `${at}.scale`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle3dReferencePatch(value: unknown, at = "$"): Puzzle3dReferencePatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    source: row["source"] === undefined ? undefined : parsePuzzle3dReferenceSource(row["source"], `${at}.source`),
    origin: row["origin"] === undefined ? undefined : puzzlePuzzle3dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle3dDiffGuardNumber(item, `${${at}.origin}[${index}]`)),
    widthWorld: row["widthWorld"] === undefined ? undefined : puzzlePuzzle3dDiffGuardNumber(row["widthWorld"], `${at}.widthWorld`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["locked"], `${at}.locked`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
  };
}

export function parsePuzzle3dKindCompatibilityPatch(value: unknown, at = "$"): Puzzle3dKindCompatibilityPatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle3dDiffGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : parsePuzzle3dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export function parsePuzzle3dMetaPatch(value: unknown, at = "$"): Puzzle3dMetaPatch {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    kindCatalogs: row["kindCatalogs"] === undefined ? undefined : row["kindCatalogs"] === null ? null : parsePuzzle3dKindCatalogs(row["kindCatalogs"], `${at}.kindCatalogs`),
    kindCompatibility: row["kindCompatibility"] === undefined ? undefined : parsePuzzle3dKindCompatibilityDelta(row["kindCompatibility"], `${at}.kindCompatibility`),
  };
}

export function parsePuzzle3dVortexRemoval(value: unknown, at = "$"): Puzzle3dVortexRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dVortexInsertion(value: unknown, at = "$"): Puzzle3dVortexInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dVortex(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dVortexRelocation(value: unknown, at = "$"): Puzzle3dVortexRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dVortexModification(value: unknown, at = "$"): Puzzle3dVortexModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dVortexPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dVorticesDelta(value: unknown, at = "$"): Puzzle3dVorticesDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dVortexRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dVortexInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dVortexRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dVortexModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle3dObjectRemoval(value: unknown, at = "$"): Puzzle3dObjectRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dObjectInsertion(value: unknown, at = "$"): Puzzle3dObjectInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dObject(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dObjectRelocation(value: unknown, at = "$"): Puzzle3dObjectRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dObjectModification(value: unknown, at = "$"): Puzzle3dObjectModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dObjectPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dObjectsDelta(value: unknown, at = "$"): Puzzle3dObjectsDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dObjectRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dObjectInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dObjectRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dObjectModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle3dAttractionRemoval(value: unknown, at = "$"): Puzzle3dAttractionRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dAttractionInsertion(value: unknown, at = "$"): Puzzle3dAttractionInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dAttraction(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dAttractionRelocation(value: unknown, at = "$"): Puzzle3dAttractionRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dAttractionModification(value: unknown, at = "$"): Puzzle3dAttractionModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dAttractionPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dAttractionsDelta(value: unknown, at = "$"): Puzzle3dAttractionsDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dAttractionRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dAttractionInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dAttractionRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dAttractionModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle3dTargetVolumeRemoval(value: unknown, at = "$"): Puzzle3dTargetVolumeRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dTargetVolumeInsertion(value: unknown, at = "$"): Puzzle3dTargetVolumeInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dTargetVolume(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dTargetVolumeRelocation(value: unknown, at = "$"): Puzzle3dTargetVolumeRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dTargetVolumeModification(value: unknown, at = "$"): Puzzle3dTargetVolumeModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dTargetVolumePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dTargetVolumesDelta(value: unknown, at = "$"): Puzzle3dTargetVolumesDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dTargetVolumeRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dTargetVolumeInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dTargetVolumeRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dTargetVolumeModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle3dReferenceRemoval(value: unknown, at = "$"): Puzzle3dReferenceRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dReferenceInsertion(value: unknown, at = "$"): Puzzle3dReferenceInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dReference(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dReferenceRelocation(value: unknown, at = "$"): Puzzle3dReferenceRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dReferenceModification(value: unknown, at = "$"): Puzzle3dReferenceModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle3dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle3dReferencePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dReferencesDelta(value: unknown, at = "$"): Puzzle3dReferencesDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dReferenceRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dReferenceInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dReferenceRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dReferenceModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle3dKindCompatibilityRemoval(value: unknown, at = "$"): Puzzle3dKindCompatibilityRemoval {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: parsePuzzle3dKindCompatibilityKey(row["id"], `${at}.id`),
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle3dKindCompatibilityInsertion(value: unknown, at = "$"): Puzzle3dKindCompatibilityInsertion {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle3dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle3dKindCompatibility(row["row"], `${at}.row`),
  };
}

export function parsePuzzle3dKindCompatibilityRelocation(value: unknown, at = "$"): Puzzle3dKindCompatibilityRelocation {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: parsePuzzle3dKindCompatibilityKey(row["id"], `${at}.id`),
    from: puzzlePuzzle3dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle3dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle3dKindCompatibilityModification(value: unknown, at = "$"): Puzzle3dKindCompatibilityModification {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    id: parsePuzzle3dKindCompatibilityKey(row["id"], `${at}.id`),
    patch: parsePuzzle3dKindCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle3dKindCompatibilityDelta(value: unknown, at = "$"): Puzzle3dKindCompatibilityDelta {
  const row = puzzlePuzzle3dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle3dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle3dKindCompatibilityRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle3dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle3dKindCompatibilityInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle3dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle3dKindCompatibilityRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle3dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle3dKindCompatibilityModification(item, `${at}.modified[${index}]`)),
  };
}
