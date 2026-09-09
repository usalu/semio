/** 🧬️ Puzzle2d diff schema — sparse field delta. */

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
  meta?: Puzzle2dMeta;
}

export interface Puzzle2dStringList { values: string[]; }
export interface Puzzle2dNodesDelta { added: Puzzle2dNode[]; removed: string[]; patched: Puzzle2dNodePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dNodePatchEntry { id: string; patch: Puzzle2dNodePatch; }
export interface Puzzle2dNodePatch { replacement?: Puzzle2dNode; }
export interface Puzzle2dEdgesDelta { added: Puzzle2dEdge[]; removed: string[]; patched: Puzzle2dEdgePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dEdgePatchEntry { id: string; patch: Puzzle2dEdgePatch; }
export interface Puzzle2dEdgePatch { replacement?: Puzzle2dEdge; }
export interface Puzzle2dArtifact { schema: string; [key: string]: unknown; }

export type Puzzle2dNodeAnchor = "fixed" | "derived";

export interface Puzzle2dCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle2dHandle {
  id: string;
  handleKind?: string;
  angle: number;
  radius?: number;
  color?: string;
  iconKind?: string;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
}

export interface Puzzle2dNode {
  id: string;
  nodeKind?: string;
  shape?: string;
  x: number;
  y: number;
  radius?: number;
  width?: number;
  height?: number;
  text?: string;
  iconKind?: string;
  root?: boolean;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
  anchor: Puzzle2dNodeAnchor;
  handles: Puzzle2dHandle[];
}

export interface Puzzle2dEdge {
  id: string;
  source: string;
  target: string;
  edgeKind?: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  sourceTip?: string;
  targetTip?: string;
  visible?: boolean;
  locked?: boolean;
}

export type Puzzle2dCompatSpecificity = "general" | "node" | "edge" | "handle" | "wire" | "vortex";

export interface Puzzle2dKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle2dCompatSpecificity;
}

export interface Puzzle2dAttribute {
  id: string;
  key: string;
  value: string;
  definition?: string;
}

export interface Puzzle2dAuthor {
  id: string;
  name: string;
  email: string;
  role?: string;
  rank?: number;
}

export interface Puzzle2dRepresentation {
  id: string;
  name: string;
  url: string;
  mime: string;
  tags: string[];
  lod?: string;
  description: string;
}

export interface Puzzle2dHandleTemplate {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  handleKind?: string;
  angle: number;
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle2dCatalogNodeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  image: string;
  unit: string;
  abstract: boolean;
  baseKinds: string[];
  representations: Puzzle2dRepresentation[];
  handles: Puzzle2dHandleTemplate[];
  attributes: Puzzle2dAttribute[];
  authors: Puzzle2dAuthor[];
}

export interface Puzzle2dCatalogHandleKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith: string[];
  description: string;
  icon: string;
  color: string;
  defaultWireKind: string;
}

export interface Puzzle2dCatalogEdgeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
}

export interface Puzzle2dCatalogWireKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
  defaultEdgeKind: string;
}

export interface Puzzle2dKindCatalogs {
  nodes: Puzzle2dCatalogNodeKind[];
  handles: Puzzle2dCatalogHandleKind[];
  edges: Puzzle2dCatalogEdgeKind[];
  wires: Puzzle2dCatalogWireKind[];
}

export interface Puzzle2dMeta {
  manifestId?: string;
  kindCompatibility: Puzzle2dKindCompatibility[];
  kindCatalogs?: Puzzle2dKindCatalogs;
}

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

export function parsePuzzle2dStringList(value: unknown, at = "$"): Puzzle2dStringList {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    values: puzzlePuzzle2dDiffGuardArray(row["values"], `${at}.values`).map((item, index) => puzzlePuzzle2dDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parsePuzzle2dNodePatchEntry(value: unknown, at = "$"): Puzzle2dNodePatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dNodePatch(row["patch"], `${at}.patch`),
  };
}

export function parsePuzzle2dEdgePatchEntry(value: unknown, at = "$"): Puzzle2dEdgePatchEntry {
  const row = puzzlePuzzle2dDiffGuardObject(value, at);
  return {
    id: puzzlePuzzle2dDiffGuardString(row["id"], `${at}.id`),
    patch: parsePuzzle2dEdgePatch(row["patch"], `${at}.patch`),
  };
}
