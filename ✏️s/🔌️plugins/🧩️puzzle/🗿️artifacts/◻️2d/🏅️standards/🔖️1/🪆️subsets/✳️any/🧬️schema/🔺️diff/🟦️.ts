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
export interface Puzzle2dHandlesDelta { removed: Puzzle2dHandleRemoval[]; inserted: Puzzle2dHandleInsertion[]; moved: Puzzle2dHandleRelocation[]; modified: Puzzle2dHandleModification[]; }
export interface Puzzle2dHandleRemoval { id: string; index: number; }
export interface Puzzle2dHandleInsertion { index: number; row: Puzzle2dHandle; }
export interface Puzzle2dHandleRelocation { id: string; from: number; to: number; }
export interface Puzzle2dHandleModification { id: string; patch: Puzzle2dHandlePatch; }
export interface Puzzle2dNodesDelta { removed: Puzzle2dNodeRemoval[]; inserted: Puzzle2dNodeInsertion[]; moved: Puzzle2dNodeRelocation[]; modified: Puzzle2dNodeModification[]; }
export interface Puzzle2dNodeRemoval { id: string; index: number; }
export interface Puzzle2dNodeInsertion { index: number; row: Puzzle2dNode; }
export interface Puzzle2dNodeRelocation { id: string; from: number; to: number; }
export interface Puzzle2dNodeModification { id: string; patch: Puzzle2dNodePatch; }
export interface Puzzle2dEdgesDelta { removed: Puzzle2dEdgeRemoval[]; inserted: Puzzle2dEdgeInsertion[]; moved: Puzzle2dEdgeRelocation[]; modified: Puzzle2dEdgeModification[]; }
export interface Puzzle2dEdgeRemoval { id: string; index: number; }
export interface Puzzle2dEdgeInsertion { index: number; row: Puzzle2dEdge; }
export interface Puzzle2dEdgeRelocation { id: string; from: number; to: number; }
export interface Puzzle2dEdgeModification { id: string; patch: Puzzle2dEdgePatch; }
export interface Puzzle2dTargetRegionsDelta { removed: Puzzle2dTargetRegionRemoval[]; inserted: Puzzle2dTargetRegionInsertion[]; moved: Puzzle2dTargetRegionRelocation[]; modified: Puzzle2dTargetRegionModification[]; }
export interface Puzzle2dTargetRegionRemoval { id: string; index: number; }
export interface Puzzle2dTargetRegionInsertion { index: number; row: Puzzle2dTargetRegion; }
export interface Puzzle2dTargetRegionRelocation { id: string; from: number; to: number; }
export interface Puzzle2dTargetRegionModification { id: string; patch: Puzzle2dTargetRegionPatch; }
export interface Puzzle2dKindCompatibilityDelta { removed: Puzzle2dKindCompatibilityRemoval[]; inserted: Puzzle2dKindCompatibilityInsertion[]; moved: Puzzle2dKindCompatibilityRelocation[]; modified: Puzzle2dKindCompatibilityModification[]; }
export interface Puzzle2dKindCompatibilityRemoval { id: Puzzle2dKindCompatibilityKey; index: number; }
export interface Puzzle2dKindCompatibilityInsertion { index: number; row: Puzzle2dKindCompatibility; }
export interface Puzzle2dKindCompatibilityRelocation { id: Puzzle2dKindCompatibilityKey; from: number; to: number; }
export interface Puzzle2dKindCompatibilityModification { id: Puzzle2dKindCompatibilityKey; patch: Puzzle2dKindCompatibilityPatch; }

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

export const puzzlePuzzle2dDiffGuardObject = (value: unknown, at: string, keys?: readonly string[]): Readonly<Record<string, unknown>> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return puzzlePuzzle2dDiffGuardReject(at, "value is not an object");
  if (keys && Object.keys(value).some((key) => !keys.includes(key))) return puzzlePuzzle2dDiffGuardReject(at, "unknown field");
  return value as Record<string, unknown>;
};
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

export function parsePuzzle2dHandleRemoval(value: unknown, at = "$"): Puzzle2dHandleRemoval {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle2dHandleInsertion(value: unknown, at = "$"): Puzzle2dHandleInsertion {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle2dHandle(row["row"], `${at}.row`),
  };
}

export function parsePuzzle2dHandleRelocation(value: unknown, at = "$"): Puzzle2dHandleRelocation {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle2dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle2dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle2dHandleModification(value: unknown, at = "$"): Puzzle2dHandleModification {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dHandlePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dHandlesDelta(value: unknown, at = "$"): Puzzle2dHandlesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dHandleRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle2dHandleInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle2dHandleRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle2dHandleModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle2dNodeRemoval(value: unknown, at = "$"): Puzzle2dNodeRemoval {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle2dNodeInsertion(value: unknown, at = "$"): Puzzle2dNodeInsertion {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle2dNode(row["row"], `${at}.row`),
  };
}

export function parsePuzzle2dNodeRelocation(value: unknown, at = "$"): Puzzle2dNodeRelocation {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle2dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle2dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle2dNodeModification(value: unknown, at = "$"): Puzzle2dNodeModification {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dNodePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dNodesDelta(value: unknown, at = "$"): Puzzle2dNodesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dNodeRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle2dNodeInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle2dNodeRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle2dNodeModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle2dEdgeRemoval(value: unknown, at = "$"): Puzzle2dEdgeRemoval {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle2dEdgeInsertion(value: unknown, at = "$"): Puzzle2dEdgeInsertion {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle2dEdge(row["row"], `${at}.row`),
  };
}

export function parsePuzzle2dEdgeRelocation(value: unknown, at = "$"): Puzzle2dEdgeRelocation {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle2dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle2dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle2dEdgeModification(value: unknown, at = "$"): Puzzle2dEdgeModification {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dEdgePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dEdgesDelta(value: unknown, at = "$"): Puzzle2dEdgesDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dEdgeRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle2dEdgeInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle2dEdgeRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle2dEdgeModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle2dTargetRegionRemoval(value: unknown, at = "$"): Puzzle2dTargetRegionRemoval {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle2dTargetRegionInsertion(value: unknown, at = "$"): Puzzle2dTargetRegionInsertion {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle2dTargetRegion(row["row"], `${at}.row`),
  };
}

export function parsePuzzle2dTargetRegionRelocation(value: unknown, at = "$"): Puzzle2dTargetRegionRelocation {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    from: puzzlePuzzle2dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle2dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle2dTargetRegionModification(value: unknown, at = "$"): Puzzle2dTargetRegionModification {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dTargetRegionPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dTargetRegionsDelta(value: unknown, at = "$"): Puzzle2dTargetRegionsDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dTargetRegionRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle2dTargetRegionInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle2dTargetRegionRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle2dTargetRegionModification(item, `${at}.modified[${index}]`)),
  };
}

export function parsePuzzle2dKindCompatibilityRemoval(value: unknown, at = "$"): Puzzle2dKindCompatibilityRemoval {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: parsePuzzle2dKindCompatibilityKey(row["id"], `${at}.id`),
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
  };
}

export function parsePuzzle2dKindCompatibilityInsertion(value: unknown, at = "$"): Puzzle2dKindCompatibilityInsertion {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    index: puzzlePuzzle2dDiffGuardNumber(row["index"], `${at}.index`),
    row: parsePuzzle2dKindCompatibility(row["row"], `${at}.row`),
  };
}

export function parsePuzzle2dKindCompatibilityRelocation(value: unknown, at = "$"): Puzzle2dKindCompatibilityRelocation {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: parsePuzzle2dKindCompatibilityKey(row["id"], `${at}.id`),
    from: puzzlePuzzle2dDiffGuardNumber(row["from"], `${at}.from`),
    to: puzzlePuzzle2dDiffGuardNumber(row["to"], `${at}.to`),
  };
}

export function parsePuzzle2dKindCompatibilityModification(value: unknown, at = "$"): Puzzle2dKindCompatibilityModification {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: parsePuzzle2dKindCompatibilityKey(row["id"], `${at}.id`),
    patch: parsePuzzle2dKindCompatibilityPatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dKindCompatibilityDelta(value: unknown, at = "$"): Puzzle2dKindCompatibilityDelta {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    removed: puzzlePuzzle2dDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parsePuzzle2dKindCompatibilityRemoval(item, `${at}.removed[${index}]`)),
    inserted: puzzlePuzzle2dDiffGuardArray(row["inserted"], `${at}.inserted`).map((item, index) => parsePuzzle2dKindCompatibilityInsertion(item, `${at}.inserted[${index}]`)),
    moved: puzzlePuzzle2dDiffGuardArray(row["moved"], `${at}.moved`).map((item, index) => parsePuzzle2dKindCompatibilityRelocation(item, `${at}.moved[${index}]`)),
    modified: puzzlePuzzle2dDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePuzzle2dKindCompatibilityModification(item, `${at}.modified[${index}]`)),
  };
}
