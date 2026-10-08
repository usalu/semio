/** 🧬️ Puzzle5d diff schema — sparse typed delta: per-field entity patches and id-keyed collection deltas. */
import {
  parsePuzzle5dCompatSpecificity,
  parsePuzzle5dFastener,
  parsePuzzle5dGrip,
  parsePuzzle5dGrip2d,
  parsePuzzle5dGrip3d,
  parsePuzzle5dKindCatalogsExtra,
  parsePuzzle5dKindCompatibility,
  parsePuzzle5dMeta,
  parsePuzzle5dPart,
  parsePuzzle5dPart2d,
  parsePuzzle5dPart3d,
  parsePuzzle5dPartAnchor,
  parsePuzzle5dScale,
  parsePuzzle5dTargetVolume,
  type Puzzle5dCompatSpecificity,
  type Puzzle5dFastener,
  type Puzzle5dGrip,
  type Puzzle5dGrip2d,
  type Puzzle5dGrip3d,
  type Puzzle5dKindCatalogsExtra,
  type Puzzle5dKindCompatibility,
  type Puzzle5dMeta,
  type Puzzle5dPart,
  type Puzzle5dPart2d,
  type Puzzle5dPart3d,
  type Puzzle5dPartAnchor,
  type Puzzle5dScale,
  type Puzzle5dTargetVolume,
  type Puzzle5dArtifact,
  type ArtifactChildHandle,
  parseArtifactChildHandle,
} from "../🟦️.ts";

export interface Puzzle5dDiff {
  /** @state artifact */
  artifact?: Puzzle5dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  domain?: string;
  /** @state artifact */
  label?: string | null;
  /** @state artifact */
  meta?: Puzzle5dMetaPatch;
  /** @state artifact */
  kindCatalogs?: ArtifactChildHandle | null;
  /** @state artifact */
  kindCatalogsExtra?: Puzzle5dKindCatalogsExtra | null;
  /** @state artifact */
  kindCompatibility?: Puzzle5dKindCompatibilityDelta;
  /** @state artifact */
  parts?: Puzzle5dPartsDelta;
  /** @state artifact */
  fasteners?: Puzzle5dFastenersDelta;
  /** @state artifact */
  targetVolumes?: Puzzle5dTargetVolumesDelta;
}

export interface Puzzle5dKindCompatibilityKey { source: string; target: string; }
export interface Puzzle5dPart2dPatch {
  x?: number;
  y?: number;
  shape?: string | null;
  radius?: number | null;
  width?: number | null;
  height?: number | null;
  text?: string | null;
  iconKind?: string | null;
  hidden?: boolean | null;
  locked?: boolean | null;
}
export interface Puzzle5dPart3dPatch {
  origin?: number[];
  meshUrl?: string | null;
  orientation?: number[] | null;
  scale?: Puzzle5dScale | null;
  label?: string | null;
}
export interface Puzzle5dGrip2dPatch {
  angle?: number;
  gripKind?: string | null;
  radius?: number | null;
}
export interface Puzzle5dGrip3dPatch {
  position?: number[];
  direction?: number[] | null;
  radius?: number | null;
  label?: string | null;
}
export interface Puzzle5dGripPatch {
  gripKind?: string | null;
  "2d"?: Puzzle5dGrip2dPatch;
  "3d"?: Puzzle5dGrip3dPatch;
}
export interface Puzzle5dPartPatch {
  partKind?: string | null;
  anchor?: Puzzle5dPartAnchor;
  "2d"?: Puzzle5dPart2dPatch;
  "3d"?: Puzzle5dPart3dPatch;
  grips?: Puzzle5dGripsDelta;
}
export interface Puzzle5dFastenerPatch {
  source?: string;
  target?: string;
  fastenerKind?: string | null;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}
export interface Puzzle5dTargetVolumePatch {
  origin?: number[];
  orientation?: number[] | null;
  scale?: Puzzle5dScale | null;
  hidden?: boolean;
  locked?: boolean;
}
export interface Puzzle5dKindCompatibilityPatch {
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle5dCompatSpecificity;
}
export interface Puzzle5dMetaPatch {
  description?: string;
}
export interface Puzzle5dGripsDelta { added: Puzzle5dGrip[]; removed: string[]; patched: Puzzle5dGripPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dGripPatchEntry { id: string; patch: Puzzle5dGripPatch; }
export interface Puzzle5dPartsDelta { added: Puzzle5dPart[]; removed: string[]; patched: Puzzle5dPartPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dPartPatchEntry { id: string; patch: Puzzle5dPartPatch; }
export interface Puzzle5dFastenersDelta { added: Puzzle5dFastener[]; removed: string[]; patched: Puzzle5dFastenerPatchEntry[]; reordered?: string[]; }
export interface Puzzle5dFastenerPatchEntry { id: string; patch: Puzzle5dFastenerPatch; }
export interface Puzzle5dTargetVolumesDelta { added: Puzzle5dTargetVolume[]; removed: string[]; patched: Puzzle5dTargetVolumePatchEntry[]; reordered?: string[]; }
export interface Puzzle5dTargetVolumePatchEntry { id: string; patch: Puzzle5dTargetVolumePatch; }
export interface Puzzle5dKindCompatibilityDelta { added: Puzzle5dKindCompatibility[]; removed: Puzzle5dKindCompatibilityKey[]; patched: Puzzle5dKindCompatibilityPatchEntry[]; reordered?: Puzzle5dKindCompatibilityKey[]; }
export interface Puzzle5dKindCompatibilityPatchEntry { id: Puzzle5dKindCompatibilityKey; patch: Puzzle5dKindCompatibilityPatch; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle5dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle5dDiffGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle5dDiffGuardRefusal(at, why);
};

type puzzlePuzzle5dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle5dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle5dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle5dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle5dDiffGuardReject(at, "value is not an object");
export const puzzlePuzzle5dDiffGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle5dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle5dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle5dDiffGuardString = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle5dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle5dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle5dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle5dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle5dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle5dDiffGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle5dDiffGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle5dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle5dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle5dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle5dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle5dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle5dDiffGuardNumber(value, at, bounds) : puzzlePuzzle5dDiffGuardReject(at, "value is not an integer");
export const puzzlePuzzle5dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle5dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle5dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle5dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle5dKindCompatibilityKey(value: unknown, at = "$"): Puzzle5dKindCompatibilityKey {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    source: puzzlePuzzle5dDiffGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle5dDiffGuardString(row["target"], `${at}.target`),
  };
}

export function parsePuzzle5dPart2dPatch(value: unknown, at = "$"): Puzzle5dPart2dPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    x: row["x"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["y"], `${at}.y`),
    shape: row["shape"] === undefined ? undefined : row["shape"] === null ? null : puzzlePuzzle5dDiffGuardString(row["shape"], `${at}.shape`),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle5dDiffGuardNumber(row["radius"], `${at}.radius`),
    width: row["width"] === undefined ? undefined : row["width"] === null ? null : puzzlePuzzle5dDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : row["height"] === null ? null : puzzlePuzzle5dDiffGuardNumber(row["height"], `${at}.height`),
    text: row["text"] === undefined ? undefined : row["text"] === null ? null : puzzlePuzzle5dDiffGuardString(row["text"], `${at}.text`),
    iconKind: row["iconKind"] === undefined ? undefined : row["iconKind"] === null ? null : puzzlePuzzle5dDiffGuardString(row["iconKind"], `${at}.iconKind`),
    hidden: row["hidden"] === undefined ? undefined : row["hidden"] === null ? null : puzzlePuzzle5dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : row["locked"] === null ? null : puzzlePuzzle5dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle5dPart3dPatch(value: unknown, at = "$"): Puzzle5dPart3dPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.origin[${index}]`)),
    meshUrl: row["meshUrl"] === undefined ? undefined : row["meshUrl"] === null ? null : puzzlePuzzle5dDiffGuardString(row["meshUrl"], `${at}.meshUrl`),
    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.orientation[${index}]`)),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : parsePuzzle5dScale(row["scale"], `${at}.scale`),
    label: row["label"] === undefined ? undefined : row["label"] === null ? null : puzzlePuzzle5dDiffGuardString(row["label"], `${at}.label`),
  };
}

export function parsePuzzle5dGrip2dPatch(value: unknown, at = "$"): Puzzle5dGrip2dPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    angle: row["angle"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["angle"], `${at}.angle`),
    gripKind: row["gripKind"] === undefined ? undefined : row["gripKind"] === null ? null : puzzlePuzzle5dDiffGuardString(row["gripKind"], `${at}.gripKind`),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle5dDiffGuardNumber(row["radius"], `${at}.radius`),
  };
}

export function parsePuzzle5dGrip3dPatch(value: unknown, at = "$"): Puzzle5dGrip3dPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    position: row["position"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["position"], `${at}.position`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.position[${index}]`)),
    direction: row["direction"] === undefined ? undefined : row["direction"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["direction"], `${at}.direction`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.direction[${index}]`)),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle5dDiffGuardNumber(row["radius"], `${at}.radius`),
    label: row["label"] === undefined ? undefined : row["label"] === null ? null : puzzlePuzzle5dDiffGuardString(row["label"], `${at}.label`),
  };
}

export function parsePuzzle5dGripPatch(value: unknown, at = "$"): Puzzle5dGripPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    gripKind: row["gripKind"] === undefined ? undefined : row["gripKind"] === null ? null : puzzlePuzzle5dDiffGuardString(row["gripKind"], `${at}.gripKind`),
    "2d": row["2d"] === undefined ? undefined : parsePuzzle5dGrip2dPatch(row["2d"], `${at}.2d`),
    "3d": row["3d"] === undefined ? undefined : parsePuzzle5dGrip3dPatch(row["3d"], `${at}.3d`),
  };
}

export function parsePuzzle5dPartPatch(value: unknown, at = "$"): Puzzle5dPartPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    partKind: row["partKind"] === undefined ? undefined : row["partKind"] === null ? null : puzzlePuzzle5dDiffGuardString(row["partKind"], `${at}.partKind`),
    anchor: row["anchor"] === undefined ? undefined : parsePuzzle5dPartAnchor(row["anchor"], `${at}.anchor`),
    "2d": row["2d"] === undefined ? undefined : parsePuzzle5dPart2dPatch(row["2d"], `${at}.2d`),
    "3d": row["3d"] === undefined ? undefined : parsePuzzle5dPart3dPatch(row["3d"], `${at}.3d`),
    grips: row["grips"] === undefined ? undefined : parsePuzzle5dGripsDelta(row["grips"], `${at}.grips`),
  };
}

export function parsePuzzle5dFastenerPatch(value: unknown, at = "$"): Puzzle5dFastenerPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    source: row["source"] === undefined ? undefined : puzzlePuzzle5dDiffGuardString(row["source"], `${at}.source`),
    target: row["target"] === undefined ? undefined : puzzlePuzzle5dDiffGuardString(row["target"], `${at}.target`),
    fastenerKind: row["fastenerKind"] === undefined ? undefined : row["fastenerKind"] === null ? null : puzzlePuzzle5dDiffGuardString(row["fastenerKind"], `${at}.fastenerKind`),
    gap: row["gap"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["gap"], `${at}.gap`),
    shift: row["shift"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["shift"], `${at}.shift`),
    rise: row["rise"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["rise"], `${at}.rise`),
    rotation: row["rotation"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["rotation"], `${at}.rotation`),
    turn: row["turn"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["turn"], `${at}.turn`),
    tilt: row["tilt"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["tilt"], `${at}.tilt`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle5dDiffGuardNumber(row["y"], `${at}.y`),
  };
}

export function parsePuzzle5dTargetVolumePatch(value: unknown, at = "$"): Puzzle5dTargetVolumePatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    origin: row["origin"] === undefined ? undefined : puzzlePuzzle5dDiffGuardArray(row["origin"], `${at}.origin`, { minItems: 3, maxItems: 3 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.origin[${index}]`)),
    orientation: row["orientation"] === undefined ? undefined : row["orientation"] === null ? null : puzzlePuzzle5dDiffGuardArray(row["orientation"], `${at}.orientation`, { minItems: 4, maxItems: 4 }).map((item, index) => puzzlePuzzle5dDiffGuardNumber(item, `${at}.orientation[${index}]`)),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : parsePuzzle5dScale(row["scale"], `${at}.scale`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle5dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle5dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle5dKindCompatibilityPatch(value: unknown, at = "$"): Puzzle5dKindCompatibilityPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle5dDiffGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle5dDiffGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : parsePuzzle5dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export function parsePuzzle5dMetaPatch(value: unknown, at = "$"): Puzzle5dMetaPatch {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    description: row["description"] === undefined ? undefined : puzzlePuzzle5dDiffGuardString(row["description"], `${at}.description`),
  };
}

export function parsePuzzle5dGripPatchEntry(value: unknown, at = "$"): Puzzle5dGripPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dGripPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dGripsDelta(value: unknown, at = "$"): Puzzle5dGripsDelta {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle5dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle5dGrip(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle5dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle5dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle5dGripPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle5dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle5dPartPatchEntry(value: unknown, at = "$"): Puzzle5dPartPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dPartPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dPartsDelta(value: unknown, at = "$"): Puzzle5dPartsDelta {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle5dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle5dPart(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle5dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle5dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle5dPartPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle5dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle5dFastenerPatchEntry(value: unknown, at = "$"): Puzzle5dFastenerPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dFastenerPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dFastenersDelta(value: unknown, at = "$"): Puzzle5dFastenersDelta {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle5dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle5dFastener(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle5dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle5dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle5dFastenerPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle5dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle5dTargetVolumePatchEntry(value: unknown, at = "$"): Puzzle5dTargetVolumePatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle5dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle5dTargetVolumePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dTargetVolumesDelta(value: unknown, at = "$"): Puzzle5dTargetVolumesDelta {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle5dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle5dTargetVolume(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle5dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle5dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle5dTargetVolumePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle5dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle5dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle5dKindCompatibilityPatchEntry(value: unknown, at = "$"): Puzzle5dKindCompatibilityPatchEntry {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    id: parsePuzzle5dKindCompatibilityKey(row["id"], `${at}.id`),
    patch: parsePuzzle5dKindCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle5dKindCompatibilityDelta(value: unknown, at = "$"): Puzzle5dKindCompatibilityDelta {
  const row = puzzlePuzzle5dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle5dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle5dKindCompatibility(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle5dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle5dKindCompatibilityKey(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle5dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle5dKindCompatibilityPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle5dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => parsePuzzle5dKindCompatibilityKey(item, `${at}.reordered[${index}]`)),
  };
}
