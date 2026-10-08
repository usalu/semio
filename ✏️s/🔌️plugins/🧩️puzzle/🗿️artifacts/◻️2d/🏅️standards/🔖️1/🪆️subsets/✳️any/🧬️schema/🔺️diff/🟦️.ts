/** 🧬️ Puzzle2d diff schema — sparse typed delta: per-field entity patches and id-keyed collection deltas. */
import {
  parsePuzzle2dCamera,
  parsePuzzle2dCompatSpecificity,
  parsePuzzle2dEdge,
  parsePuzzle2dHandle,
  parsePuzzle2dKindCatalogs,
  parsePuzzle2dKindCompatibility,
  parsePuzzle2dMeta,
  parsePuzzle2dNode,
  parsePuzzle2dNodeAnchor,
  parsePuzzle2dTargetRegion,
  type Puzzle2dCamera,
  type Puzzle2dCompatSpecificity,
  type Puzzle2dEdge,
  type Puzzle2dHandle,
  type Puzzle2dKindCatalogs,
  type Puzzle2dKindCompatibility,
  type Puzzle2dMeta,
  type Puzzle2dNode,
  type Puzzle2dNodeAnchor,
  type Puzzle2dTargetRegion,
  type Puzzle2dArtifact,
} from "../🟦️.ts";

export interface Puzzle2dDiff {
  /** @state artifact */
  artifact?: Puzzle2dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  camera?: Puzzle2dCamera;
  /** @state artifact */
  nodes?: Puzzle2dNodesDelta;
  /** @state artifact */
  edges?: Puzzle2dEdgesDelta;
  /** @state artifact */
  targetRegions?: Puzzle2dTargetRegionsDelta;
  /** @state artifact */
  meta?: Puzzle2dMetaPatch;
}

export interface Puzzle2dKindCompatibilityKey { source: string; target: string; }
export interface Puzzle2dHandlePatch {
  handleKind?: string | null;
  angle?: number;
  radius?: number | null;
  color?: string | null;
  iconKind?: string | null;
  scale?: number | null;
  visible?: boolean | null;
  locked?: boolean | null;
}
export interface Puzzle2dNodePatch {
  nodeKind?: string | null;
  shape?: string | null;
  x?: number;
  y?: number;
  radius?: number | null;
  width?: number | null;
  height?: number | null;
  text?: string | null;
  iconKind?: string | null;
  root?: boolean | null;
  scale?: number | null;
  visible?: boolean | null;
  locked?: boolean | null;
  anchor?: Puzzle2dNodeAnchor;
  handles?: Puzzle2dHandlesDelta;
}
export interface Puzzle2dEdgePatch {
  source?: string;
  target?: string;
  edgeKind?: string | null;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
  sourceTip?: string | null;
  targetTip?: string | null;
  visible?: boolean | null;
  locked?: boolean | null;
}
export interface Puzzle2dTargetRegionPatch {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  label?: string | null;
  hidden?: boolean;
  locked?: boolean;
}
export interface Puzzle2dKindCompatibilityPatch {
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle2dCompatSpecificity;
}
export interface Puzzle2dMetaPatch {
  manifestId?: string | null;
  kindCompatibility?: Puzzle2dKindCompatibilityDelta;
  kindCatalogs?: Puzzle2dKindCatalogs | null;
}
export interface Puzzle2dHandlesDelta { added: Puzzle2dHandle[]; removed: string[]; patched: Puzzle2dHandlePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dHandlePatchEntry { id: string; patch: Puzzle2dHandlePatch; }
export interface Puzzle2dNodesDelta { added: Puzzle2dNode[]; removed: string[]; patched: Puzzle2dNodePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dNodePatchEntry { id: string; patch: Puzzle2dNodePatch; }
export interface Puzzle2dEdgesDelta { added: Puzzle2dEdge[]; removed: string[]; patched: Puzzle2dEdgePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dEdgePatchEntry { id: string; patch: Puzzle2dEdgePatch; }
export interface Puzzle2dTargetRegionsDelta { added: Puzzle2dTargetRegion[]; removed: string[]; patched: Puzzle2dTargetRegionPatchEntry[]; reordered?: string[]; }
export interface Puzzle2dTargetRegionPatchEntry { id: string; patch: Puzzle2dTargetRegionPatch; }
export interface Puzzle2dKindCompatibilityDelta { added: Puzzle2dKindCompatibility[]; removed: Puzzle2dKindCompatibilityKey[]; patched: Puzzle2dKindCompatibilityPatchEntry[]; reordered?: Puzzle2dKindCompatibilityKey[]; }
export interface Puzzle2dKindCompatibilityPatchEntry { id: Puzzle2dKindCompatibilityKey; patch: Puzzle2dKindCompatibilityPatch; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class puzzlePuzzle2dDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const puzzlePuzzle2dDiffGuardReject = (at: string, why: string): never => {
  throw new puzzlePuzzle2dDiffGuardRefusal(at, why);
};

type puzzlePuzzle2dDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type puzzlePuzzle2dDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type puzzlePuzzle2dDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const puzzlePuzzle2dDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : puzzlePuzzle2dDiffGuardReject(at, "value is not an object");
export const puzzlePuzzle2dDiffGuardArray = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return puzzlePuzzle2dDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) puzzlePuzzle2dDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) puzzlePuzzle2dDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const puzzlePuzzle2dDiffGuardString = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return puzzlePuzzle2dDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) puzzlePuzzle2dDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) puzzlePuzzle2dDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) puzzlePuzzle2dDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const puzzlePuzzle2dDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : puzzlePuzzle2dDiffGuardReject(at, "value is not a boolean"));
export const puzzlePuzzle2dDiffGuardNumber = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return puzzlePuzzle2dDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) puzzlePuzzle2dDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) puzzlePuzzle2dDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const puzzlePuzzle2dDiffGuardInteger = (value: unknown, at: string, bounds: puzzlePuzzle2dDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? puzzlePuzzle2dDiffGuardNumber(value, at, bounds) : puzzlePuzzle2dDiffGuardReject(at, "value is not an integer");
export const puzzlePuzzle2dDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : puzzlePuzzle2dDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const puzzlePuzzle2dDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : puzzlePuzzle2dDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePuzzle2dKindCompatibilityKey(value: unknown, at = "$"): Puzzle2dKindCompatibilityKey {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    source: puzzlePuzzle2dDiffGuardString(row["source"], `${at}.source`),
    target: puzzlePuzzle2dDiffGuardString(row["target"], `${at}.target`),
  };
}

export function parsePuzzle2dHandlePatch(value: unknown, at = "$"): Puzzle2dHandlePatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    handleKind: row["handleKind"] === undefined ? undefined : row["handleKind"] === null ? null : puzzlePuzzle2dDiffGuardString(row["handleKind"], `${at}.handleKind`),
    angle: row["angle"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["angle"], `${at}.angle`),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["radius"], `${at}.radius`),
    color: row["color"] === undefined ? undefined : row["color"] === null ? null : puzzlePuzzle2dDiffGuardString(row["color"], `${at}.color`),
    iconKind: row["iconKind"] === undefined ? undefined : row["iconKind"] === null ? null : puzzlePuzzle2dDiffGuardString(row["iconKind"], `${at}.iconKind`),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["scale"], `${at}.scale`),
    visible: row["visible"] === undefined ? undefined : row["visible"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : row["locked"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle2dNodePatch(value: unknown, at = "$"): Puzzle2dNodePatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    nodeKind: row["nodeKind"] === undefined ? undefined : row["nodeKind"] === null ? null : puzzlePuzzle2dDiffGuardString(row["nodeKind"], `${at}.nodeKind`),
    shape: row["shape"] === undefined ? undefined : row["shape"] === null ? null : puzzlePuzzle2dDiffGuardString(row["shape"], `${at}.shape`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["y"], `${at}.y`),
    radius: row["radius"] === undefined ? undefined : row["radius"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["radius"], `${at}.radius`),
    width: row["width"] === undefined ? undefined : row["width"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : row["height"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["height"], `${at}.height`),
    text: row["text"] === undefined ? undefined : row["text"] === null ? null : puzzlePuzzle2dDiffGuardString(row["text"], `${at}.text`),
    iconKind: row["iconKind"] === undefined ? undefined : row["iconKind"] === null ? null : puzzlePuzzle2dDiffGuardString(row["iconKind"], `${at}.iconKind`),
    root: row["root"] === undefined ? undefined : row["root"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["root"], `${at}.root`),
    scale: row["scale"] === undefined ? undefined : row["scale"] === null ? null : puzzlePuzzle2dDiffGuardNumber(row["scale"], `${at}.scale`),
    visible: row["visible"] === undefined ? undefined : row["visible"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : row["locked"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["locked"], `${at}.locked`),
    anchor: row["anchor"] === undefined ? undefined : parsePuzzle2dNodeAnchor(row["anchor"], `${at}.anchor`),
    handles: row["handles"] === undefined ? undefined : parsePuzzle2dHandlesDelta(row["handles"], `${at}.handles`),
  };
}

export function parsePuzzle2dEdgePatch(value: unknown, at = "$"): Puzzle2dEdgePatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    source: row["source"] === undefined ? undefined : puzzlePuzzle2dDiffGuardString(row["source"], `${at}.source`),
    target: row["target"] === undefined ? undefined : puzzlePuzzle2dDiffGuardString(row["target"], `${at}.target`),
    edgeKind: row["edgeKind"] === undefined ? undefined : row["edgeKind"] === null ? null : puzzlePuzzle2dDiffGuardString(row["edgeKind"], `${at}.edgeKind`),
    gap: row["gap"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["gap"], `${at}.gap`),
    shift: row["shift"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["shift"], `${at}.shift`),
    rise: row["rise"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["rise"], `${at}.rise`),
    rotation: row["rotation"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["rotation"], `${at}.rotation`),
    turn: row["turn"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["turn"], `${at}.turn`),
    tilt: row["tilt"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["tilt"], `${at}.tilt`),
    x: row["x"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["y"], `${at}.y`),
    sourceTip: row["sourceTip"] === undefined ? undefined : row["sourceTip"] === null ? null : puzzlePuzzle2dDiffGuardString(row["sourceTip"], `${at}.sourceTip`),
    targetTip: row["targetTip"] === undefined ? undefined : row["targetTip"] === null ? null : puzzlePuzzle2dDiffGuardString(row["targetTip"], `${at}.targetTip`),
    visible: row["visible"] === undefined ? undefined : row["visible"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["visible"], `${at}.visible`),
    locked: row["locked"] === undefined ? undefined : row["locked"] === null ? null : puzzlePuzzle2dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle2dTargetRegionPatch(value: unknown, at = "$"): Puzzle2dTargetRegionPatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    x: row["x"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["y"], `${at}.y`),
    width: row["width"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : puzzlePuzzle2dDiffGuardNumber(row["height"], `${at}.height`),
    label: row["label"] === undefined ? undefined : row["label"] === null ? null : puzzlePuzzle2dDiffGuardString(row["label"], `${at}.label`),
    hidden: row["hidden"] === undefined ? undefined : puzzlePuzzle2dDiffGuardBoolean(row["hidden"], `${at}.hidden`),
    locked: row["locked"] === undefined ? undefined : puzzlePuzzle2dDiffGuardBoolean(row["locked"], `${at}.locked`),
  };
}

export function parsePuzzle2dKindCompatibilityPatch(value: unknown, at = "$"): Puzzle2dKindCompatibilityPatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    bidirectional: row["bidirectional"] === undefined ? undefined : puzzlePuzzle2dDiffGuardBoolean(row["bidirectional"], `${at}.bidirectional`),
    important: row["important"] === undefined ? undefined : puzzlePuzzle2dDiffGuardBoolean(row["important"], `${at}.important`),
    specificity: row["specificity"] === undefined ? undefined : parsePuzzle2dCompatSpecificity(row["specificity"], `${at}.specificity`),
  };
}

export function parsePuzzle2dMetaPatch(value: unknown, at = "$"): Puzzle2dMetaPatch {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    manifestId: row["manifestId"] === undefined ? undefined : row["manifestId"] === null ? null : puzzlePuzzle2dDiffGuardString(row["manifestId"], `${at}.manifestId`),
    kindCompatibility: row["kindCompatibility"] === undefined ? undefined : parsePuzzle2dKindCompatibilityDelta(row["kindCompatibility"], `${at}.kindCompatibility`),
    kindCatalogs: row["kindCatalogs"] === undefined ? undefined : row["kindCatalogs"] === null ? null : parsePuzzle2dKindCatalogs(row["kindCatalogs"], `${at}.kindCatalogs`),
  };
}

export function parsePuzzle2dHandlePatchEntry(value: unknown, at = "$"): Puzzle2dHandlePatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dHandlePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dHandlesDelta(value: unknown, at = "$"): Puzzle2dHandlesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle2dHandle(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle2dHandlePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle2dNodePatchEntry(value: unknown, at = "$"): Puzzle2dNodePatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dNodePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dNodesDelta(value: unknown, at = "$"): Puzzle2dNodesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle2dNode(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle2dNodePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle2dEdgePatchEntry(value: unknown, at = "$"): Puzzle2dEdgePatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dEdgePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dEdgesDelta(value: unknown, at = "$"): Puzzle2dEdgesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle2dEdge(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle2dEdgePatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle2dTargetRegionPatchEntry(value: unknown, at = "$"): Puzzle2dTargetRegionPatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dTargetRegionPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dTargetRegionsDelta(value: unknown, at = "$"): Puzzle2dTargetRegionsDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle2dTargetRegion(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle2dTargetRegionPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.reordered[${index}]`)),
  };
}

export function parsePuzzle2dKindCompatibilityPatchEntry(value: unknown, at = "$"): Puzzle2dKindCompatibilityPatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: parsePuzzle2dKindCompatibilityKey(row["id"], `${at}.id`),
    patch: parsePuzzle2dKindCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dKindCompatibilityDelta(value: unknown, at = "$"): Puzzle2dKindCompatibilityDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    added: puzzlePuzzle2dDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePuzzle2dKindCompatibility(item, `${at}.added[${index}]`)),
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dKindCompatibilityKey(item, `${at}.removed[${index}]`)),
    patched: puzzlePuzzle2dDiffGuardArray(row["patched"], `${at}.patched`).map((item, index) => parsePuzzle2dKindCompatibilityPatchEntry(item, `${at}.patched[${index}]`)),
    reordered: row["reordered"] === undefined || row["reordered"] === null ? undefined : puzzlePuzzle2dDiffGuardArray(row["reordered"], `${at}.reordered`).map((item, index) => parsePuzzle2dKindCompatibilityKey(item, `${at}.reordered[${index}]`)),
  };
}
